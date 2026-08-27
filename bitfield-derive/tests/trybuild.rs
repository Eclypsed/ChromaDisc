//! UI test harness. See `tests/ui/`.
//!
//! The stderr fixtures are the actual product (PLAN §11 phase 5). Regenerate
//! with `TRYBUILD=overwrite cargo test -p bitfield-derive --test trybuild`
//! after reviewing each message.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
