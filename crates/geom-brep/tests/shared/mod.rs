//! Helpers that more than one suite of this crate's `all` binary uses.
//!
//! Declared ONCE in `tests/all.rs` as `mod shared;`, so the code below
//! is parsed, resolved, type-checked and codegen'd once for the whole
//! binary; a suite that wants a piece of it says
//! `use crate::shared::<module>;`. Nothing here is a suite — the
//! aggregation guard's directory walk skips a directory carrying a
//! `mod.rs`, which is why this is a directory and not a flat
//! `tests/shared.rs`.
//!
//! **The bar for putting something here is that two suites were
//! building the same thing, not that two suites need a thing of the
//! same shape.** Several probe suites in this crate exist to check the
//! kernel against a truth derived without it, and two such derivations
//! sharing one routine would be one derivation wearing two names. Each
//! module below says which of its neighbours it deliberately did not
//! absorb, and every suite that keeps its own copy says why at the
//! copy.

pub(crate) mod fixture;
pub(crate) mod patch;
pub(crate) mod sample;
