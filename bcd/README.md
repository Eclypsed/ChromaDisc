# const-bcd

Packed Binary-Coded Decimal (BCD) values with const-generic storage.

`const-bcd` provides [`Bcd<BYTES>`], a `no_std`-friendly type for storing
unsigned integers in packed BCD format, where each byte holds two decimal
digits (one per nibble). The `BYTES` const parameter selects the storage size
at compile time and therefore the maximum representable value.

- `#![no_std]` compatible (disable the default `std` feature)
- No allocations
- All construction and conversion functions are `const fn`
- The [`bcd!`] macro is usable in `const` context

## Usage

Add `const-bcd` to your `Cargo.toml`:

```toml
[dependencies]
const-bcd = "0.1"
```

For a `no_std` build:

```toml
[dependencies]
const-bcd = { version = "0.1", default-features = false }
```

## Examples

```rust
use const_bcd::{Bcd, bcd};

// From a raw packed-BCD byte array
let n = Bcd::<2>::from_bcd_bytes([0x12, 0x34]).unwrap();
assert_eq!(n.to_string(), "1234");

// From a primitive integer (fallible)
let n = Bcd::<2>::try_from_u16(1234).unwrap();
assert_eq!(n.to_string(), "1234");

// Convenience macro (panics on overflow, usable in const context)
const N: Bcd<2> = bcd!(1234u16);
assert_eq!(N.to_string(), "1234");

// Convert back to a primitive
let value: u16 = n.try_into_u16().unwrap();
assert_eq!(value, 1234);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
