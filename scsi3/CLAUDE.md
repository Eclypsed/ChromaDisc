# scsi3 design guide

This guide covers how scsi3 is meant to be used and the reasoning behind its design. Before adding a command or structure, look at the most up-to-date examples and follow the same patterns. Right now those are `scsi_mmc/features/` and `scsi_mmc/commands/get_configuration.rs`.

The crate is fragmented at the moment. Some modules predate the patterns below and haven't been brought up to date yet, so check a module against this guide before copying from it. Known bugs in that older code:
- The `parse` functions for the PMA, Raw TOC and CD-TEXT formats of READ TOC/PMA/ATIP cut the response at `toc_data_length - 2`. They should cut at `+ 2`, so they stop 4 bytes short, and the subtraction panics in debug builds when TOC Data Length is under 2.

## What scsi3 is

- **A strict implementation of the SCSI-3 standards that optical drives use:** SAM (`core`), SPC (`spc`) and MMC (`mmc`).
  - Each directory (`scsi_core`, `scsi_spc`, `scsi_mmc`) is named after the crate it would become if scsi3 were split up.
  - `docs/references.md` lists the standard each one follows.
- **Commands and responses only, no I/O.** scsi3 builds CDBs and parses responses. The transport owns the buffers, their sizes and the residual counts.
- **It models what the spec says, not what particular drives do.** Drive quirks are the application's job (see `docs/quirks.md`). That's one reason responses are references into the caller's buffer: the application can inspect or fix the exact bytes the drive sent.
- **It shows the caller what's there rather than guessing.** The library can't tell why data disagrees with itself. The drive may be misbehaving, the response may have been cut short, or the caller may have passed the wrong bytes. So it returns what's present, keeps the evidence visible, and saves errors for responses it can't read at all.
- **Restrictions:** `#![no_std]` and `#![forbid(unsafe_code)]`. `alloc` is linked, but the goal is to stop needing it.
- **The public API follows the spec's structure and names.**
  - Types and accessors are named after the spec's tables and fields.
  - The spec's abbreviations go in `#[doc(alias = "...")]`.
  - Parsing machinery that has no counterpart in the spec stays private.

## Zero-copy parsing

- **Responses are zerocopy casts over the caller's buffer.** They are `#[repr(C)]` structs deriving `FromBytes`, `KnownLayout`, `Immutable` and `Unaligned`.
  - `parse` reads a length field and casts, so parsing is O(1). Accessors decode the bits when called.
  - Nothing is copied, so a reference aliases the buffer, and a write through a `*_mut` accessor shows up everywhere. The caller decides when to copy.
- **Layouts keep every byte of the spec's table**, reserved bytes included (`_reserved`, `_bytes9_11`), so `size_of` matches the spec.
- **Multi-byte fields are `U16`/`U32<BigEndian>`**, or newtypes over them. Every type is `Unaligned`, so the `CastError::Alignment` match arms are `unreachable!`.
- **Value newtypes follow one shape** (e.g. `ProfileNumber` in `scsi_mmc/profiles.rs`):
  - They are `#[repr(transparent)]` over the big-endian integer.
  - They have const `from_u16`/`into_u16` methods, not derive_more `From`/`Into`, which aren't const.
  - Values the spec names are associated consts.
  - Debug prints hex with the spec's `h` suffix, e.g. `ProfileNumber(0008h)`.
- **Layout fields are private.** They are read through `#[inline] pub const fn` accessors, with the `bit!` macro for single bits (`features/mod.rs`).

## Commands

- **Traits:** a command is a struct holding the CDB inputs. It implements `Command<OpCode<N>>` (`as_cdb`) and `ReadCommand` (`response_len`, `parse`) from `scsi_core/mod.rs`.
- **Generic or not:**
  - If a CDB field changes the response layout, make the command generic over the response, using sealed traits.
  - If it doesn't, keep the command non-generic with a private enum and named constructors (`GetConfiguration::{all, current, single}`).
- **`parse` returns `(&Response, &[u8])`:** the response cut to its own length field, plus every byte of `buf` after it. Nothing is discarded.

## Spec revisions

- **The last revision that describes a version defines it.**
  - The draft revisions (e.g. mmcNrNNx) are working drafts. Some of them changed a version's layout without bumping the version number.
  - Model the latest description, and leave a `// NOTE:` about the earlier shape (`features/formattable.rs`).
- **Fields that a later revision made obsolete:**
  - Keep the explicit fields from the current spec.
  - Offer the old reading through an extra accessor (`features/bd_read.rs`).
- **Say what you can't verify.** The user doesn't have every draft revision. `docs/notes.md` links the archived T10 drafts.
- **Comment tags:**
  - `// NOTE:` for spec history and discrepancies.
  - `// TODO:` for work left to do.
  - `// CONFIRM:` for behavior to check on real hardware.

## Trait, inline and derive conventions

- **Trait bounds:** a trait requires only what the library itself needs. Generic code adds its own bounds (`<F: Feature + fmt::Debug>`). Registries and marker traits are sealed.
- **Constants:** use associated consts, not methods. A method implies the value isn't constant.
- **`#[inline]`:** put it on non-generic public accessors, so they can inline across crates. Leave it off generic functions, `Debug::fmt` and private functions.
- **Derives:**
  - Sized layouts: `Clone, Copy, PartialEq, Eq, Hash` plus Debug.
  - DSTs: `PartialEq, Eq, Hash` plus Debug.
  - No `Default`, since `FromZeros::new_zeroed` exists. No `Ord` on layouts.
  - Equality and hashing compare all the bytes, reserved ones included.
- **Debug:**
  - It prints the decoded values the accessors return, not the raw fields (`debug_fields!` in `features/mod.rs`).
  - Write list formatting inline with `fmt::from_fn`, not through a helper type.
