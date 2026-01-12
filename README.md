# ChromaDisc

Welcome to ChromaDisc, a project I am pursuing with the following goals:

- Learn about the various CD-\* format specifications as defined in the [Rainbow Books](https://en.wikipedia.org/wiki/Rainbow_Books)
- Learn Rust. I already love rust as a language, but I know very little about low-level systems development with rust.
- Develop a replacement for `cdparanoia` that properly implements error correction, gap-detection, and HTOA (Hidden track one audio) support. Eventually I would like ChromaDisc to be robust enough to serve as an open source replacement for Exact Audio Copy and X Lossless Decoder.

**Current Objective:** Create a complete drive profiler to identify all relevant information about the drive being used (Vendor/Product info, features, etc.)

## libcdio

After a not-so-thorough review I have determined that I will not utilize the rust bindings for `libcdio` in this project. In short, after an intitial inverstigation I found a few confusing/questionable design choices (libcdio's own GNU manual documents admitted as such in a few places) and decided writing my own library would be better both for development and my own learning, rather than trying to shoe-horn in an existing C library. However, it will be a great reference and point of comparison.
