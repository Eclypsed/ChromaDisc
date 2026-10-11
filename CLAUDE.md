# ChromaDisc

ChromaDisc is written from scratch for reading optical discs. The long-term goal is to replace cdparanoia, and eventually EAC and XLD (see `README.md`).

## Workspace

- **`scsi3/`** builds CDBs and parses responses for SAM, SPC and MMC. It does no I/O. Read `scsi3/CLAUDE.md` before changing it.
- **`chromadisc/`** is the application. For now it's a testing ground for scsi3 against a real drive, and it changes a lot.
- **Support crates:**
  - `bcd/` (published as `const-bcd`).
  - `bitfield/` and `bitfield-derive/`: a prototype derive for sub-byte wire enums.
  - `mmc-errors/`: a derive that maps sense key, ASC and ASCQ to enum variants.
- **`docs/`:**
  - `references.md`: which standards the code follows.
  - `quirks.md`: observed ways the test drive departs from the spec.
  - `notes.md`: links to sources, including archived T10 drafts.

## Conventions

- **Formatting:** format only the files you touched, with `rustfmt --edition 2024 --config style_edition=2021 <file>`.
  - Never run `cargo fmt`. It uses the 2024 style and would reorder imports in untouched files.
  - Diff each file after formatting, and restore any line the user formatted by hand.
- **Checks:** `cargo test -p scsi3`, `cargo clippy -p scsi3 --tests` and `cargo doc -p scsi3 --no-deps`.
  - Some warnings already exist in files you aren't working on. Mention them rather than fixing them unasked.
- **Toolchain:** edition 2024, a recent stable rustc. `scsi3` sets no `rust-version`.
