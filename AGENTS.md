# ChromaDisc

ChromaDisc is written from scratch for reading optical discs. The long-term goal is to replace cdparanoia, and eventually EAC and XLD (see `README.md`).

## Workspace

- **`scsi3/`** builds CDBs and parses responses for SAM, SPC and MMC. It does no I/O. Read `scsi3/AGENTS.md` before changing it.
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

- **Formatting:** run `cargo fmt`. `rustfmt.toml` sets the edition, and the pre-commit hook in `devenv.nix` runs `cargo fmt` on every commit.
- **Checks:** `cargo nextest run -p scsi3`, `cargo clippy -p scsi3 --tests` and `cargo doc -p scsi3 --no-deps`.
  - Run tests with nextest (`devenv.nix` provides it), not `cargo test`.
  - nextest skips doctests. If a change adds or touches one, also run `cargo test -p scsi3 --doc`.
  - Some warnings already exist in files you aren't working on. Mention them rather than fixing them unasked.
- **Toolchain:** edition 2024, a recent stable rustc. `scsi3` sets no `rust-version`.
