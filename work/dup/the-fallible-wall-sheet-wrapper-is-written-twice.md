---
id: the-fallible-wall-sheet-wrapper-is-written-twice
kind: issue
title: try_wall_sheet is token-identical in two review-probe suites and wraps the now-shared door
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

`crates/topo/tests/r1_mate5_probe.rs` and
`crates/topo/tests/r2_probes.rs` each carry a `try_wall_sheet`:
a `std::panic::catch_unwind` around the cylinder-wall sheet builder
that turns a fixture the current ε cannot MINT into a `None`, so the
row stands down instead of reporting. The two are **token-identical**
after comment and whitespace normalisation (one md5), measured
2026-09-19 at merge base `5b4979ef2`; only their doc comments differ,
and those differ in substance — `r2_probes`' cites the tilt map its
own suite carries.

Both now wrap one door
(`topo::test_support::cyl_wall_sheet`, PR for
`the-cylindrical-patch-rim-builder-is-written-nine-times`), which is
what makes the remaining wrapper a duplication rather than two
builders that happen to share a shape. **This row exists because that
unit minted it** — method item 5, X4.

## Why it was not folded with the builder

The obvious home is `topo::test_support` beside the door. Two reasons
to decide that deliberately rather than by reflex:

- Nothing in `test_support_fixtures.rs` swallows a panic. A fixture
  door that returns `None` where its sibling doors panic is a posture
  change for the module, not a move.
- The stand-down is a per-suite JUDGEMENT — *this row's constants won
  the mint lottery at the default ε and would not be evidence at
  another* — and a shared wrapper states it once for suites that may
  not mean the same thing by it.

Neither reason survives measurement on its own; a unit here should
take both and decide, not assume.

## What a unit here owes

Re-take the count (two is the count at this merge base; the wrapper is
easy to copy again), then either give it the shared home with the
posture written down, or state why two suites keep it and delete the
duplicate doc sentence that is not true of both.
