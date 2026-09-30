---
id: deep-snapshot-does-not-walk-the-body-side-tables
kind: issue
title: fixtures::deep_snapshot walks the ten arenas and live provenance but none of the body's side tables, so an atomicity row cannot see a side-table write
status: open
opened: 2026-09-30
priority: P2
---



## What

Found while unifying the deep snapshot
(`deep-snapshot-is-written-three-times`). `fixtures::deep_snapshot`
(`crates/topo/src/fixtures.rs`) is the crate's in-lib "body unchanged"
observation. `assert_err_deep_unchanged` and every atomicity row in
`src/` compare by it. It walks the ten arenas, plus the seven provenance
maps through `Body::provenance` on live keys. It reads none of the other
`SecondaryMap`s on `Body` (`crates/topo/src/body.rs`, `struct Body`):

- `pcurves` (per-half-edge chart images and certificates),
- `null_faces`,
- `point_origins`, `curve_origins`, `surface_origins`,
- `surface_field_sources`, `surface_axis_sources`.

Because provenance is read through live keys, the snapshot also cannot
see a record left behind for a dead key. That is the validator's job (its
`Leaked*` rules), not a before/after comparison's.

So an operator that fails after writing a pcurve row, a null-face record
or an origin passes every `assert_err_deep_unchanged` row in the crate.
The pcurve-row doors are where this is live today: several open rows on
this slate concern Euler ops that move or drop pcurve rows.

Whole-body `Debug` (`format!("{body:?}")`, used by `merge_faces.rs` and
`revert.rs` tests and by several integration crates) reads every field.
The side tables are what separate it from the snapshot.

## Shape to consider

Extend `deep_snapshot` to walk each side table, one line per row keyed
like the arena lines. Extend its own row the same way: add a row to
each table on a clone and require the snapshot to move. The snapshot's
doc comment names the gap, and should stop naming it once the gap is
closed.
