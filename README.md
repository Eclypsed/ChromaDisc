# ChromaDisc

Welcome to ChromaDisc, a project I am pursuing with the following goals:

- Learn about the various CD-\* format specifications as defined in the [Rainbow Books](https://en.wikipedia.org/wiki/Rainbow_Books)
- Learn Rust. I already love rust as a language, but I know very little about low-level systems development with rust.
- Develop a replacement for `cdparanoia` that properly implements error correction, gap-detection, and HTOA (Hidden track one audio) support. Eventually I would like ChromaDisc to be robust enough to serve as an open source replacement for Exact Audio Copy and X Lossless Decoder.

**Current Objective:** Achieve basic communication with optical drives using rust bindings for `libcdio` (NOT `libcdio-paranoia`)

> Note: As I develop this project I intend to evaluate the efficacy of `libcdio` for the goals of this project. If needed I may write my own replacement library from scratch.
