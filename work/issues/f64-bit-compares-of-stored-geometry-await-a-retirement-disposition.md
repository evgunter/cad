---
id: f64-bit-compares-of-stored-geometry-await-a-retirement-disposition
kind: issue
title: Three production f64 to_bits() compares decide geometric identity, outside the Bounds-shaped sweep and the consumer gate
status: open
opened: 2026-10-02
---

## What

TQUERY's rim repair swept production code for `to_bits()` reads of
`Bounds::lo()`/`hi()` that decide coincidence (the `rim_of` shape Ev
ruled a retired coincidence check on PR 3156). That pattern cannot
see a bit compare of plain `f64` stored values, so a second pass
looked at every production `to_bits()` outside `#[cfg(test)]`
modules. Three compare stored geometry and decide identity from the
bits; whether each is a coincidence check under `docs/DESIGN.md`'s
retirement ("Production bit-identity coincidence checking is
RETIRED") has not been decided:

- `crates/mesh/src/planar.rs` `same_position` — a point is the far
  point when its three coordinates are bit-equal (its doc argues the
  signed-zero case).
- `crates/step-import/src/adopt.rs` `bitwise_iso_match` — a parsed
  NURBS carrier IS a wall's boundary iso-curve when degree, knots,
  control points and weights are bit-equal (the IsoCurve rung).
- `crates/topo/src/seqgen.rs` `chord_of` — a merged member whose start
  and end are bit-equal is re-described as a self-loop circle.

`scripts/gates/bit-identity-consumer.sh` sees none of them (it matches
`bit_identity::|repr_bits|eq_bits`), and
`work/guard/the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds.md`
widens it to `Bounds` accessors only.

## Not these

The rest of the second pass is hashing and memo keys
(`mesh/src/memo.rs`, `bvh/src/aabb.rs`, `geom-core/src/bit_identity.rs`,
`Interval::repr_bits`), exactness tests of a single value against a
constant (`step-import` weights `== 1.0`), STEP parse-record dedupe of
unit factors (`step-import/src/entities.rs`), diagnostics that report
ulps (`profile/src/lift.rs`), and the test fixtures in
`topo/src/fixtures.rs`. The ~45 hits in `editor-core`, `viewer`,
`pncad-py` and `demos/tour` were classed by file (document values,
quantity formatting, memo keys) and not read one by one — the open
gap of this sweep.

## The shape to give

Per site: a disposition — not a coincidence check (say why in its
doc), or routed through `bit_identity::eq_bits`, allowlisted with a
retirement-scheduled note, or replaced by a structural or recorded
fact as `rim_of` was.
