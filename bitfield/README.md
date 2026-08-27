# bitfield

A Rust derive macro for wire-protocol bitfield enums.

> **Not ready for general use.** This crate was prototyped with heavy AI assistance and has not been hardened for production. It covers my own use-cases well enough to evaluate the model, but the API is unstable, the crate name is a placeholder, and I intend to develop it further on my own terms before considering any kind of release. Use at your own risk — or better yet, don't.

---

## What it does

Wire protocols define sub-byte fields whose bit patterns map to semantic values. Those mappings are often:

- **Non-injective** — multiple distinct patterns can mean the same thing
- **Partial** — some patterns are unallocated, reserved for future revisions
- **Multi-range** — a single variant may claim a mix of individual values and ranges
- **Growing** — unallocated patterns get claimed in later spec versions, and that should be a non-breaking change

Existing crates miss at least one of these requirements. `num_enum` has no width enforcement and no coverage checking. `bilge` is width-aware but only supports a single catch-all and tightly couples enums to its struct-packing model.

`bitfield` is built around three hard requirements: **exact field-width enforcement**, **multiple values and ranges per variant**, and **compile-time exhaustiveness checking**.

## Two crates

```
bitfield         support crate — BitsRepr, Bits<T>, UnmappedBits
bitfield-derive  proc-macro crate — #[derive(BitsEnum)]
```

Users depend on `bitfield` only. The derive re-export and the runtime types live there.

## Quick look

```rust
use bitfield::{Bits, BitsEnum};

#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(4, reserved = 0b1110..=0b1111)]
enum ProfileVersion {
    V1 = 0b0001,
    V2 = 0b0010,
    #[bits(alt = 0b0100 | 0b0101)]
    V3 = 0b0011,          // 0b0011–0b0101 all decode to V3
    #[bits(fallback)]
    Unknown = 0b0000,     // catches everything else
}

let raw: Bits<ProfileVersion> = Bits::new(0b0100);
assert_eq!(raw.get(), Ok(ProfileVersion::V3));
```

`Bits<T>` stores the raw pattern verbatim. Unmapped or reserved values are preserved and surfaced as `UnmappedBits` errors on `.get()`, not silently discarded.

## Status

This is a prototype I wrote to evaluate whether this model is actually useful for parsing real SCSI/MMC command structures. The core mechanics work. The API will change. I plan to continue developing this myself — polishing the error messages, firming up the attribute grammar, potentially renaming the crates, and validating the design against a larger corpus of real spec tables before I'd consider it something other people should reach for.

The design document is in [`PLAN.md`](PLAN.md) if you want to understand the reasoning behind the choices.
