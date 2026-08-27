# Bitfield Enum Derive — Design & Implementation Plan

Status: design settled, implementation not started.
Scope of v1: **enums only**. No struct/packing support.

This document describes a standalone, general-purpose crate. SCSI appears
only as usage context — it is the driving use case and the source of the
examples, but nothing in the crate is SCSI-specific.

---

## 1. Problem

Wire protocols define sub-byte fields whose bit patterns map to semantic
values. The mapping is:

- **Non-injective** — several distinct patterns can mean the same thing
  (`0b0010` and `0b0100` both mean `ValueC`).
- **Partial** — some patterns are unallocated, set aside for future
  standardisation.
- **Multi-range** — a variant may claim a mix of single values, contiguous
  ranges, and non-contiguous alternatives.
- **Growing** — unallocated patterns get allocated in later spec revisions,
  and that should be a *non-breaking* change for every crate in the chain.

Existing crates each miss at least one requirement:

- `num_enum` — no width checking, single catch-all only, no coverage check.
- `bilge` — width-aware but no non-injective mappings beyond one fallback;
  couples enums to its integer-packed struct model.

The three hard requirements are: **exact width enforcement**, **multiple
values/ranges per variant**, and **compile-time exhaustiveness checking**.

---

## 2. Architecture

Two crates.

```
bits-derive     proc-macro crate. Zero runtime deps. Emits code only.
bits            facade + runtime. Re-exports the derive. Defines
                BitsRepr, Bits<T>, UnmappedBits. No field definitions,
                so it should never need a major bump.
```

`bits-derive` is pinned by `bits` at an exact version (`=x.y.z`) and is not
depended on directly by users. Generated code refers to `::bits::` paths;
use the `proc-macro-crate` crate to resolve the real name so renamed
dependencies still compile.

**Name is unresolved.** `bits` is almost certainly taken on crates.io. Check
before writing any paths; a global rename later is annoying but cheap if done
in week one.

---

## 3. Core model

### 3.1 Two types, not one

The enum is a **lossy semantic view**. The wire value lives separately.

```rust
Bits<T>   // stores T::Bits verbatim. Lossless. Round-trips exactly.
T         // the enum. Non-injective, possibly partial.
```

This is what makes everything else work:

- Round-tripping is exact even though the enum is non-injective, because
  `Bits<T>` never discards the pattern it was constructed from.
- Unmapped patterns need no enum representation — the raw value is still
  reachable through `Bits<T>`.
- Strict and lenient parsing come from one type: store `Bits<T>` in your
  structs, call `.get()` only where interpretation is actually required.

### 3.2 The crate takes no position on "reserved"

**"Reserved" is not a concept the macro models.** What reserved *means* is
domain-specific, and the crate deliberately supports both readings:

| Domain rule | Mechanism | Decode behaviour |
|---|---|---|
| "receipt of reserved values shall be reported as error" (SCSI) | leave the patterns uncovered | `from_bits` returns `None` |
| "ignore on read" / "preserve on modify" / pass through unknowns | `#[bits(fallback)]` on a `Reserved` variant | `from_bits` returns that variant |

Both are first-class. `reserved = ...` (§6.2) is an orthogonal *assertion*
about which patterns are uncovered, not a third behaviour.

**The two differ in how they handle spec growth**, and users should choose
deliberately:

- *Uncovered.* When a pattern is later allocated, an input that returned
  `None` now returns `Some(NewVariant)`. Strictly widening — no exhaustive
  match breaks, no existing `Some` arm changes meaning.
- *Fallback variant.* When a pattern is later allocated, it moves out of the
  fallback variant into a new one. Code doing `Reserved => log_unknown()`
  keeps compiling and silently stops firing. Not a compile error, and no
  signal anywhere.

Neither is wrong; the second is what some protocols require. Document the
trade-off in the crate docs so the choice is informed rather than accidental.

**Two related patterns worth naming in the docs**, using SCSI as the example:

- *Vendor-specific ranges* are normal variants. Permanently allocated,
  legitimately received, never subdivided by the spec body.
- *Obsolete values* are also normal variants. A value being obsolete reflects
  the newest revision of a standard; old devices still emit it, and a library
  that supports devices of all ages must still decode it. Do not suggest
  `#[deprecated]` for these — obsolete is a documentation fact, not an
  API-lifecycle one. A doc comment on the variant is the right tool.

### 3.3 No payload variants

v1 supports **fieldless variants only**.

Once `Bits<T>` retains the wire value, a `VendorSpecific(u4)` payload is a
second, unvalidated copy of information the wrapper already holds — and any
mismatch between them is a state the type system permits but the protocol
does not. Removing the payload eliminates the invariant rather than
defending it.

Consequences, all good:

- `to_bits` is a cast with no match: `self as u8`.
- No generated newtypes, no private-field construction guards.
- No `#[repr(u8)]` requirement — explicit discriminants on fieldless enums
  cast fine without one, and derives cannot add attributes to the item
  anyway.

Adding payload support later is purely additive. Do not build it now.

---

## 4. Conversion API — exact shapes

### 4.1 The trait

```rust
pub trait BitsRepr: Sized + Copy {
    /// Conversion boundary type: a primitive (`u8`, `u16`, …) or a
    /// wrapper such as `arbitrary_int::u4`.
    type Bits: Copy + Eq + fmt::Binary;

    /// Field width in bits. NOT necessarily the width of `Self::Bits`.
    const BITS: u32;

    /// Field name, used in error reporting and `Debug`.
    /// Defaults to the type name.
    const FIELD: &'static str;

    fn from_bits(bits: Self::Bits) -> Option<Self>;
    fn to_bits(self) -> Self::Bits;
}
```

`const BITS` is needed on day one. A future struct/packing layer must be able
to ask a field how wide it is to compute offsets, and adding a const to a
trait later without a default is breaking.

`fmt::Binary` on the associated type is what makes `Bits<T>`'s `Debug`
unconditional (§4.6). Every plausible repr implements it — the primitives do,
and so does `arbitrary_int::UInt`. Adding it now is free; adding it later is
breaking.

### 4.2 What the derive emits

Inherent const fns are the **primitive**. Trait methods are one-line forwards.
This split is forced by the language — trait methods cannot be const on
stable. If const trait impls land, `#[const_trait] BitsRepr` collapses it,
non-breakingly.

```rust
impl ExampleField {
    pub const fn from_bits(bits: u4) -> Option<Self> { /* match */ }
    pub const fn to_bits(self) -> u4 { u4::new(self as u8) }
}

impl BitsRepr for ExampleField { /* forwards */ }

impl From<ExampleField> for u4 { /* forwards to to_bits */ }
```

Plus, **only when coverage is total**:

```rust
impl From<u4> for ExampleField { /* from_bits(b).unwrap() — unreachable */ }
```

### 4.3 `from_bits` returns `Option`, permanently

`const fn from_bits(uN) -> Option<Self>` is fallible **forever**, including
after a mapping becomes total, at which point it simply never returns `None`.

Rationale: totality must not be a semver event. A protocol library may define
hundreds of these fields; if each closure forced a major bump the project
would be unshippable. With a fixed signature, closure is an additive `From`
impl and a changelog line.

This mirrors `char::from_u32` (const, fallible, inherent) alongside
`From<char> for u32`.

### 4.4 `TryFrom<uN> for T` is left permanently vacant

**Never implement it.** Rust's blanket `impl<T, U: Into<T>> TryFrom<U> for T`
means that writing `From<uN> for T` later would collide with a hand-written
`TryFrom`, and would silently change `Error` from your type to `Infallible`
for anyone who had one. Leaving the slot empty means the future `From` impl
collides with nothing and the blanket `TryFrom` it drags along is pure
addition.

Decode goes through the inherent const fn and `BitsRepr`. That is the whole
decode API.

### 4.5 Error enrichment lives in `Bits<T>`, not the derive

`from_bits(b)` fails and the caller already has `b`. The raw value in an
error is redundant at the call site; it only earns its place once the error
escapes that scope, and by then you want field context too — which
`from_bits` cannot supply.

So the derive stays generic (`Option`), and `Bits<T>` adds context:

```rust
pub fn get(self) -> Result<T, UnmappedBits<T::Bits>> {
    T::from_bits(self.0)
        .ok_or(UnmappedBits { raw: self.0, field: T::FIELD })
}
```

The error is named `UnmappedBits`, not `ReservedCode`: an unmapped pattern is
only "reserved" in domains that say so (§3.2). Note that when the enum has a
`#[bits(fallback)]` variant, `get()` can never fail — that is expected, not a
defect.

### 4.6 `Bits<T>` implementation notes

```rust
pub struct Bits<T: BitsRepr>(T::Bits);
```

**Do not use `#[derive]` on this.** `#[derive(Clone)]` generates
`impl<T: BitsRepr + Clone> Clone`, bounding `T` — which is never stored.
Write `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Debug` by hand, bounding
`T::Bits`.

```rust
impl<T: BitsRepr> Clone for Bits<T> { fn clone(&self) -> Self { *self } }
impl<T: BitsRepr> Copy for Bits<T> {}
impl<T: BitsRepr> PartialEq for Bits<T> {
    fn eq(&self, o: &Self) -> bool { self.0 == o.0 }
}
impl<T: BitsRepr> Eq for Bits<T> {}

impl<T: BitsRepr> Bits<T> {
    pub const fn new(bits: T::Bits) -> Self { Self(bits) }
    pub fn bits(self) -> T::Bits { self.0 }
    pub fn get(self) -> Result<T, UnmappedBits<T::Bits>> { /* §4.5 */ }
}

impl<T: BitsRepr> From<T> for Bits<T> {
    fn from(v: T) -> Self { Self(v.to_bits()) }
}
```

**`Debug` shows the bits, nothing else.** It must not call `get()`. `Bits<T>`
is the storage type; its `Debug` reports what is stored. A user who wants the
interpretation calls `get()` and uses the enum's own `Debug`. Zero-pad to the
field width rather than a fixed width:

```rust
impl<T: BitsRepr> fmt::Debug for Bits<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ExampleField(0b0010)
        write!(f, "{}({:#0w$b})", T::FIELD, self.0, w = T::BITS as usize + 2)
    }
}
```

(`+ 2` accounts for the `0b` prefix that `#` adds.)

This is not in tension with §8.1's "emit no `Debug`". That rule is about what
the **derive** generates for **user enums**. `Bits<T>` is a concrete type in
the support crate, and its `Debug` must be hand-written regardless because
the bounds are wrong for `#[derive]`.

**Document the equality asymmetry:** `PartialEq` on `Bits<T>` is bit
equality, `get()` gives semantic equality. `Bits::new(0b0010) !=
Bits::new(0b0100)` but their `get()`s are equal. Correct, but surprising.

---

## 5. Attribute syntax

One namespaced helper attribute, `#[bits(...)]`, at both enum and variant
level. A single namespace avoids collisions in user crates (this is why
serde uses one `#[serde(...)]`).

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug, BitsEnum)]
#[bits(4, repr = u4, reserved = 0b1110..=0b1111)]
#[non_exhaustive]
pub enum ExampleField {
    ValueA = 0b0000,
    ValueB = 0b0001,
    #[bits(alt = 0b0100)]
    ValueC = 0b0010,
    ValueD = 0b0011,
    #[bits(alt = 0b0110 | 0b1000..=0b1010)]
    ValueE = 0b0101,
    ValueF = 0b0111,
    #[bits(alt = 0b1100..=0b1101)]
    VendorSpecific = 0b1011,
}
```

### Enum-level keys

| Key | Meaning |
|---|---|
| *(first positional)* | field width in bits. Required. |
| `repr = <ty>` | conversion boundary type. Default: smallest primitive that fits. |
| `reserved = <patterns>` | optional assertion; see §6.2. |
| `field = "…"` | optional override for `BitsRepr::FIELD`. Default: type name. |

### Variant-level keys

| Key | Meaning |
|---|---|
| `alt = <patterns>` | additional patterns claimed by this variant. |
| `fallback` | this variant absorbs everything unclaimed. See §6.3. |

The discriminant (`= 0b0010`) is the **canonical encoding** — what `to_bits`
emits. `alt` patterns decode to the variant but are not re-emitted.

### `alt` accepts match patterns, not a bespoke grammar

Whatever the user writes is spliced into a generated `match`, so the
**syntax** accepted is Rust's pattern grammar. Parse with `syn::Pat`
(`Pat::parse_multi_with_leading_vert`), not a hand-rolled range parser. That
gets `|` alternation, `..=`, exclusive `..` (stable in patterns since 1.80),
open-ended ranges, and anything else the language adds later, for free.

The generated arm is:

```
<discriminant> | <alt patterns> => Some(Self::Variant),
```

which is why the discriminant needs no special handling and a variant without
`alt` still works.

**Semantic restriction:** coverage analysis must reduce each pattern to a set
of integer intervals. Accept integer literals, ranges, `|` alternation, and
nested combinations of those. Reject anything that cannot be reduced —
bindings, paths, `_`, guards, `@` — with a clear error pointing at the
offending sub-pattern and naming `#[bits(fallback)]` as the way to express a
catch-all. Syntax is Rust's; semantics are the macro's.

**Helper attribute syntax is public API.** New keys can be added as minor
bumps; existing keys can never change meaning. Pick spellings deliberately.

### Derives, not an attribute macro

Use `#[derive]`. An attribute macro consumes and re-emits the item, so
rust-analyzer and rustdoc see expansion output instead of the real enum —
go-to-definition, hover, and rendered docs all degrade. Everything needed
fits in helper attributes, and discriminants on fieldless variants are valid
Rust, so there is nothing to rewrite.

An attribute macro expanding to the derive list is acceptable *optional
sugar* later, off the same implementation. Not v1.

---

## 6. Compile-time checks

### 6.1 Coverage — the core feature

Reduce every declared pattern to intervals, sort by start, scan adjacent
pairs:

- overlap: `next.start <= cur.end` → error, naming both variants
- gap: `next.start > cur.end + 1` → error, naming the uncovered patterns

**Do not materialise a `Vec<Option<VariantId>>` of length 2^N.** Real specs
define 16-bit code spaces; 65536-entry vectors across hundreds of derive
invocations is noticeable compile time. The scan is O(k log k) in declared
ranges and produces identical diagnostics.

Coverage is computed over `0 .. 2^width`, **not** over the repr's range.

Correctness of this logic is the macro crate's responsibility and belongs in
the macro crate's own test suite (§11, Phase 3) — unit tests over the
interval reducer plus property tests comparing it against a brute-force
`0..2^N` oracle at small widths. It must **not** be re-verified in every user
enum at compile time (§10).

### 6.2 `reserved` is an assertion, not a declaration of intent

`From<uN> for T` emission is decided by **computed** coverage, never by the
presence of `reserved`. Coverage is a fact; the macro computes it.

`reserved = ...` is optional and, when present, is asserted to equal the
computed complement **exactly**. Its job is drift detection:

> Declare `0x00..=0x14`, `0x1E`, `0x1F`, intending `0x15..=0x1D`
> unallocated. Typo one bound and leave `0x15` uncovered. With
> `reserved = 0x15..=0x1D`, the macro names the uncovered code. Without it,
> the macro shrugs — gaps are expected and it cannot know which ones were
> meant. Coverage silently drops, and if the field was going to become total,
> the `From` impl silently never appears.

Optional keeps the crate pleasant for prototyping. Recommend it in the docs
for any field transcribed from a published table.

### 6.3 `fallback`

`#[bits(fallback)]` on one fieldless variant makes the mapping total by
construction and therefore **disables gap detection**. Document that cost
explicitly.

**Incompatible with `reserved`** — a fallback makes the complement
unreachable, so asserting its contents is meaningless. Error on both.

Overlap detection still applies to the explicitly declared variants.

### 6.4 Width enforcement

Check every literal against `2^width` **during analysis**, with a
span-accurate error:

```
error: `0b10000` does not fit in a 4-bit field
  --> src/lib.rs:12:20
```

Do not delegate this to const-eval by relying on `arbitrary_int`'s `new()`
panicking. That only works for arbitrary_int reprs, produces a const-eval
panic message instead of a pointed diagnostic, and points at generated code
rather than the user's literal.

The same check applies to `reserved` bounds and to both ends of every range.
Parse literals to `u128` for comparison; reject widths above 128 outright.

### 6.5 No `#[non_exhaustive]` check

The macro does **not** validate that `#[non_exhaustive]` is present alongside
`reserved`. Declaring types appropriately is the user's job; a proc macro
erroring on a merely unusual declaration is overreach.

Worth one paragraph in the crate docs, though: leaving patterns uncovered
means variants may be added later, which is the situation `#[non_exhaustive]`
exists for. Adding it is breaking; removing it is not — so it is the free
option.

---

## 7. The `repr` adapter — no arbitrary_int dependency

If the derive emitted `impl From<T> for u4` unconditionally, arbitrary_int
would become a **public** dependency of every crate using the macro, and any
major bump of it would propagate to all of them.

Instead, the derive never depends on arbitrary_int. It carries two
token-level adapters chosen from the `repr` the user wrote:

```
repr = u8   →  to_prim(e) = e            from_prim(e) = e
repr = u4   →  to_prim(e) = e.value()    from_prim(e) = u4::new(e)
```

Emit `match to_prim(bits) { … }` and wrap results with `from_prim`. Both
arbitrary_int methods are `const fn`, so the const path survives. Treat known
primitive names as identity and anything else as a wrapper.

**Do not make this a cargo feature.** Features unify across the dependency
graph, so an unrelated crate enabling `arbitrary-int` would silently change
what your code compiles to. A feature may gate whether the `bits` support
crate carries arbitrary_int impls at all; the repr choice must live in the
attribute.

### Repr width determines whether totality is reachable

With `repr = u8` on a 4-bit field, `from_bits(0xFF)` must return `None`, so
the mapping over `u8` is never total and `From<u8> for T` can **never** be
emitted — even after every in-range pattern is allocated. The totality
transition of §4.3 only exists when repr width == field width.

For sub-byte fields in a protocol library that expects growth, an exact-width
repr is therefore not just documentation: it is what makes the additive
`From` path reachable at all. Say so in the docs.

---

## 8. Semver contract

### 8.1 Everything the derive emits is transitively public API

When the derive emits `impl From<T> for u4`, that impl is part of the
*user's* crate's API, and their users depend on it. Withdrawing it in v2
breaks crates two hops away.

**Emit the minimum at 1.0.** Only:

- inherent `from_bits` / `to_bits`
- `impl BitsRepr for T`
- `impl From<T> for uN`
- `impl From<uN> for T` when total

No `Debug`, no `Display`, no serde, no `Hash`. Each is a minor bump later.
None can be taken back.

### 8.2 The growth story is the crate's main selling point

Document it explicitly, because it is the reason for §4.3 and §4.4:

1. A field starts partial. `from_bits` returns `Option`; no `From<uN>`.
2. Patterns get allocated over successive spec revisions. Each is a new
   variant and a widening of `from_bits` — **minor bump**.
3. The last pattern is allocated. `From<uN> for T` appears, and with it the
   blanket `TryFrom` — **minor bump**. `from_bits` keeps its signature.

Nothing in that sequence is breaking.

**Minor gotcha:** auto-emitting `From<uN> for T` at step 3 can break
downstream *inference* (a bare `.into()` that was unambiguous becoming
ambiguous). Rust classifies this as possibly-breaking rather than breaking
and it is rare, but it is the one way the transition is not perfectly silent.

### 8.3 The residual hazard, and why it is not fixable

When a previously unmapped pattern is allocated, downstream code shaped like
`Err(_) => log_unknown()` or `_ => unsupported()` keeps compiling and
silently stops firing. `#[non_exhaustive]` does not catch it — by mandating a
wildcard arm at every match site, it is the mechanism that *creates* the
absorption path.

This is intrinsic to widening a mapping. Do not engineer around it. Put it in
the crate docs as guidance for downstream maintainers: when a pattern becomes
allocated, name it explicitly in the changelog. The type system cannot warn;
release notes can.

---

## 9. Codegen details easy to miss

- **`#[allow(deprecated)]` on generated impls.** The crate does not recommend
  `#[deprecated]` for obsolete protocol values (§3.2), but users may mark
  variants deprecated for their own reasons, and the `deprecated` lint *does*
  fire on same-crate uses — including generated match arms. Emitting the
  allow costs nothing and prevents warnings the user cannot silence.
- **Doc comments on emitted items** listing each variant's code ranges. For a
  spec library this is a large share of the value, and docs are not
  semver-constrained.
- **`proc-macro-crate`** for path resolution, so renamed dependencies work.
- **Preserve spans on every parsed literal.** Diagnostics depend on it, and
  it is very hard to retrofit.

---

## 10. Out of scope for v1 — explicitly rejected

Do not add these without revisiting the reasoning above.

| Rejected | Why |
|---|---|
| `impl TryFrom<uN> for T` | permanently reserved slot; blanket impl collision (§4.4) |
| Payload variants (`VendorSpecific(u4)`) | duplicate state, constructible invariants (§3.3) |
| A built-in `Reserved` variant or reserved-specific behaviour | domain-dependent; two mechanisms already cover it (§3.2) |
| `from_bits` returning a rich error type | ties the generic derive to one domain (§4.5) |
| Signature change when a mapping becomes total | breaks the growth story for every field (§4.3) |
| Emitting a `const _: () = { … }` coverage loop per enum | verifying the macro's own logic in user code; O(2^N) compile cost. Belongs in the macro crate's tests (§6.1) |
| Relying on `uN::new()` const panics for width checks | poor diagnostics, arbitrary_int-only (§6.4) |
| `Debug` on `Bits<T>` that calls `get()` | storage type reports storage; interpretation is `get()`'s job (§4.6) |
| Recommending `#[deprecated]` for obsolete values | obsolete is a spec fact, not an API-lifecycle one (§3.2) |
| Separate `FromBits` / `TryFromBits` derives | fallibility is declaration-determined, not derive-determined |
| An `exhaustive` marker key | redundant with computed coverage; collides visually with `#[non_exhaustive]` |
| Attribute macro instead of derives | destroys IDE and rustdoc fidelity (§5) |
| `Vec<Option<VariantId>>` coverage | O(2^N); 16-bit fields exist (§6.1) |
| Cargo feature selecting repr | feature unification changes codegen silently (§7) |
| Struct / packing support | separate concern; see §12 |

---

## 11. Implementation phases

**Phase 1 — support crate (`bits`)**
`BitsRepr`, `Bits<T>` with manual impls, `UnmappedBits`, `Debug`.
Hand-write one field's impls to validate the shape before any codegen exists.

**Phase 2 — parser**
`#[bits(...)]` at enum and variant level. `syn::Pat` for `alt` and
`reserved`; `repr`, width, `field`, `fallback`. Preserve spans everywhere.
Factor into one `parse_bits_spec(&DeriveInput)` shared by all downstream
logic.

**Phase 3 — analysis**
Pattern → interval reduction, with a clear error for irreducible patterns.
Range sort + adjacent scan. Overlap, gap, width, `reserved` assertion,
`fallback` interaction. Error messages carrying the offending pattern and
both variant names.

This is the phase to test hardest: unit tests on the reducer, plus property
tests at widths 1–8 comparing the scan against a brute-force `0..2^N` oracle.
That oracle exists here and nowhere else.

**Phase 4 — codegen**
Inherent const fns, `BitsRepr`, `From<T> for uN`, conditional
`From<uN> for T`, repr adapters, doc comments, `#[allow(deprecated)]`.

**Phase 5 — trybuild**
Compile-fail tests for every diagnostic. **The error messages are the actual
product** and are the hardest thing to retrofit once the parser's shape is
fixed. Cover: gap, overlap, `reserved` mismatch, out-of-width literal,
`fallback` + `reserved`, irreducible `alt` pattern, payload variant present,
missing width, width > 128.

**Phase 6 — validation at scale**
Transcribe a few hundred real code-value tables from published standards.
This will tell you more about the attribute syntax than any amount of further
design. Watch for: tables that do not fit the model, compile-time
regressions, and whether the absence of payload variants actually hurts.

---

## 12. Why enums only

The crate deliberately does not take a position on how fields are packed into
a buffer. Its conversion boundary is a single integer, with bit extraction
left to the caller, so it composes equally with an integer-backed struct
macro, a byte-array-backed one, or hand-written parsing.

That orthogonality is what makes enum-only v1 a complete deliverable rather
than a partial one, and it is the main thing that distinguishes this crate
from `bilge` and `bitbybit`, whose enum support is entangled with their
struct models.

One applicability note worth putting in the docs: this macro is for fields
with a modest number of semantically distinct values. A sparse 16-bit table
with several hundred entries that nobody matches on exhaustively is better
served by a newtype over `u16` with a `name()` lookup against a sorted static
table.

---

## 13. Open questions

1. **Crate name.** `bits` is likely taken. Resolve before writing paths.
2. **Derive name and count.** Single `BitsEnum` emitting everything, versus
   separate encode/decode derives. Recommendation: **single derive**. The
   original argument for splitting was that encode and decode need different
   inputs and have different failure modes, but with payload variants gone
   `to_bits` is a cast needing nothing, and fallibility is determined by the
   declaration rather than by which derive was chosen. A third derive for
   `BitsRepr` (needed if split, since two derives cannot both emit the same
   trait impl) is pure clutter.
3. **`FIELD` default.** Type name is the obvious default, but nested or
   generic contexts may want the full path.
4. **`alt` spelling.** `alt` vs `alternatives` vs `values`. Locked once
   published (§5).
5. **MSRV.** Exclusive range patterns in `alt` require 1.80 if supported.
   Establish a floor before 1.0 and put it in CI.
