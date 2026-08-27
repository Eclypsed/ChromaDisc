# Problem Spec: SCSI MMC GET CONFIGURATION Response Parser

## Background

We need a Rust library that parses the response payload of the SCSI
Multimedia Commands (MMC) `GET CONFIGURATION` command, as issued to optical
disc drives (CD/DVD/BD). The response describes which optical-drive
features the device supports. This document specifies the format, the
failure modes that must be handled, and the architectural constraints the
implementation must follow — particularly around error-type design.

This is intended as a self-contained brief: no prior conversation context
is assumed.

## Format Description

The response is a byte stream with the following structure:

1. A response header (not detailed here beyond noting it contains a
   `current_profile` field) followed by:
2. A sequence of **feature descriptors**, packed back-to-back with no
   separators, filling the rest of the response.

### Feature descriptor layout

Each feature descriptor consists of:

- **4-byte header:**
  - Bytes 0–1: `feature_code` (u16, big-endian). Only a subset of the
    65,536 possible values are defined by the spec; drives may legally
    report codes unknown to any given parser (future/vendor-specific
    features).
  - Byte 2: packed bit flags, from most- to least-significant bit —
    - top 2 bits: reserved
    - next 4 bits: `version` (0–15)
    - next bit: `persistent`
    - final bit: `current`
  - Byte 3: `additional_length` — the length in bytes of the body that
    follows, *for this descriptor only*.
- **Body:** exactly `additional_length` bytes. Its internal structure
  depends on the combination of `feature_code` and `version`. Across all
  defined features and versions, there are on the order of **hundreds of
  distinct body layouts**, each specified independently in the MMC spec.

### Version semantics

Feature versions are designed to be backwards-compatible: a higher
version number for a given feature code generally extends rather than
replaces the layout of a lower version's body (e.g. by appending fields).
This isn't guaranteed to hold for every feature in the spec, and no
particular fallback strategy is assumed by this document, but it's worth
keeping in mind — it may be relevant when deciding how to treat an
unknown version of a feature whose lower versions *are* known.

### Framing guarantee

The critical structural property of this format: **you can always find the
start of the next descriptor from `additional_length`, even if you have no
idea how to interpret the current body.** Framing (finding descriptor
boundaries) is fully decoupled from content understanding (parsing a
specific body layout). The architecture must preserve and exploit this
property — see Design Constraints below.

## Failure Modes to Handle

The following situations must be explicitly designed for:

1. **Unknown feature code** — `feature_code` does not match any code the
   parser has a definition for at all (any version).
2. **Unknown version for a known feature code** — the feature code is
   recognized, but no parser is registered for the reported `version`.
3. **Length mismatch on a known feature/version** — the body parser for a
   *recognized* `(feature_code, version)` pair either runs out of bytes
   before finishing, or finishes without consuming all
   `additional_length` bytes (unaccounted trailing bytes).
4. **A caller-imposed length limit truncates a feature descriptor.** A
   caller of this library can explicitly limit how many bytes of the
   response they read/allocate (e.g. reading only the first N bytes off
   the device). This can truncate the response in one of two distinct
   ways, which must be distinguished:
   - **A feature descriptor is cut off mid-way** — the buffer ends before
     a complete header (4 bytes) is available, or before the full
     declared `additional_length` of body bytes is available. In this
     case only *that* descriptor is affected: it should be reported as an
     error, but every descriptor successfully parsed before it remains
     valid and should still be returned to the caller. This is a
     localized, recoverable failure, not a total one.
   - **The response header itself is cut off**, such that even the
     header's `current_profile` field can't be read. This is effectively
     a complete failure — there isn't a meaningful partial result to
     return, since nothing downstream of the header can be trusted to be
     present at all.

## Design Constraints

These constraints come from an explicit architectural discussion and
should be treated as requirements, not suggestions:

### 1. Errors must be narrow and specific to what can actually happen

The public error type(s) exposed by the library must not be a single
monolithic enum covering every failure mode any binary-parsing dependency
could theoretically produce (this is the default behavior of derive-macro
crates like `binrw`/`deku`/generated Kaitai Struct code, and is explicitly
what we're avoiding). Error types should be scoped to what is actually
reachable at each public entry point, mirroring the structure of the data
being parsed (e.g. a header-level error type separate from a
descriptor-level frame error type), rather than re-exporting or
transparently forwarding an underlying crate's error type.

### 2. Most of the failure modes above are *not* total errors

Given the framing guarantee, cases 1–3 (unknown code, unknown version,
length mismatch on a known body) must **not** cause the overall parse to
fail or abort. They should be represented as structured, inspectable
*data* about that one descriptor, so that one unrecognized or malformed
descriptor does not prevent the rest of the response from being parsed.

Case 4 requires the same "don't lose what's already valid" principle
applied at a different level: a descriptor cut off mid-way should produce
an error scoped to *that descriptor only*, without discarding the
descriptors already successfully parsed before it. Only a truncated
response *header* — where nothing downstream can be trusted to be
present — justifies a total, no-partial-result failure.

### 3. No information may be discarded in exceptional cases

Whenever a descriptor's body isn't fully understood or doesn't parse
cleanly (cases 1–3), the raw bytes of that descriptor's body must be
retained in the result (not just a summary or a discarded value), so
callers can log, inspect, or re-process it — e.g. against a newer version
of the feature table. Errors — whether for a truncated descriptor or a
truncated header — should retain precise byte offsets and counts
(expected vs. available), not just a generic message, and should retain
whatever partial bytes were actually available where practical.

### 4. Body parsing must be bounded and isolated per descriptor

Each body parser must be invoked against a sub-slice bounded to exactly
`additional_length` bytes — never given access to the remainder of the
buffer. Regardless of whether that parser succeeds, fails, or
under/over-consumes within its bounded slice, the outer cursor must
advance by exactly `4 + additional_length` before continuing to the next
descriptor. This is what makes it safe to keep parsing after an
unrecognized or malformed body.

## Non-Goals

- Serialization/writing support is out of scope for this spec (parsing
  only).
- Populating the full table of hundreds of known `(feature_code,
  version)` body layouts from the MMC spec is a large, separate task;
  this spec concerns the parsing architecture, not an exhaustive feature
  table. A small representative subset of known features is sufficient
  to validate the architecture.
