---
id: thread-the-tolerance-through-the-prism-fixture-family
kind: issue
title: Link 2: thread tol through the prism fixture family, which witness-not-ambient forces before the move
status: open
opened: 2026-09-18
refs: [brick-has-two-constructions-and-two-homes]
---


## Finding

- **Where**: `crates/topo/tests/common/mod.rs` — the prism/cube fixture
  family, which calls `Tol::witness()` internally rather than taking a
  tolerance.
- **Importance**: medium — it gates link 3
- **Confidence**: sure, and the gate was **measured** rather than read
- **Raised by**: the `dup-brick-measure` lane (S-DUP), 2026-09-16

**Link 2 of `work/dup/brick-has-two-constructions-and-two-homes.md`.**
The family cannot move into `crates/topo/src/test_support_impl.rs`
until it stops reaching for an ambient tolerance, because
`scripts/gates/witness-not-ambient.sh` forbids `Tol::witness()` under
`crates/*/src` and exempts only `#[cfg(test)]` —
`gate_test_only_mounts`' `GATE_CFG_TEST_NOT_RE` explicitly excludes an
`any(...)` cfg from that narrowing, so
`test_support_impl.rs`'s
`#[cfg(any(debug_assertions, test, feature = "test-support"))]` mount
puts it squarely in the gate's production set.

**Established by planting one violation there, watching the gate fire
and name the line, then reverting and confirming green over 434 files.**
Method item 4 applied to a gate rather than a constant.

## Why it is worth doing on its own merits

A fixture should not reach for an ambient tolerance — that is what the
gate exists to say. And threading `tol: Tol` lands `prism_ops`'
signature on `sweep::test_support::brick`'s, which is convergent
evidence that the two are one door (the measurement on the brick-homes
row proved the bodies equal; matching signatures is the shape following
the fact).

Also in this unit: the file-level
`#![allow(unwrap_used, expect_used, panic)]` that is unremarkable in
`tests/` lands inside the library tree on the move, and needs a
decision rather than a copy.

## Count

The family called `Tol::witness()` **24 times** when measured on
2026-09-16; `crates/topo/tests/common/mod.rs` alone held 17. **Re-take
it** — this program's standing result is that no count survives contact
with the tree, and PR #2812 has since unified two builders into one,
which will have moved it.
