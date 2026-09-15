---
id: unit-vector-witness-in-geom-core
kind: unit
title: The unit-vector witness moves to geom-core, minted by the decided-normalize ladder, and from_frame's consumers take it
status: closed
opened: 2026-09-15
branch: scalar/unit-vector-witness
pr: 2646
closed: 2026-09-15
---


## What

The ruling on `unit-vector-invariants-carried-as-prose` (PR 2457),
first unit. `topo::query::UnitVec3<T>` moves to `geom-core` (no
re-export from `topo`; imports repointed), private Cartesian field,
one normalizing mint (`new(v, site, band)`: decide the length under
the band at the caller's funnel-site name, divide) plus exact
negation; `frame.rs`'s ladders mint through that constructor under
their own funnel names. The `sin_cos` mint the ruling also names has
no customer in the tree (`path_start_frame`'s in-plane axes are cross
products) and lands with its first customer rather than as a public
door nobody calls.
`Vec3::orthonormal_basis` and `frame_from_unit_aim` take it. What it
means at every scalar: produced by a normalize whose length decided
positive under the band — an enclosure of a unit vector at `Interval`,
a unit value channel at `Dual`. The geom carrier fields (`Line.dir`,
`Plane.normal`, the axes) stay under `geom/src/lib.rs`'s at-rest rule
and are NOT touched; their row is filed separately. PROPS' ground
(`geom-core/src/linalg/*`) and TOPO's (`query.rs`), announced. Full v6
dual.

## Closed (2026-09-15) — PR 2646

`UnitVec3<T>`, `UnitVec3Error` and `decide_unit_direction` live in
`crates/geom-core/src/linalg/unit_vec.rs` (private field, `Copy`,
`Debug`); one normalizing mint `new(v, site, band)` plus exact `Neg`;
no `sin_cos` mint (no customer), no "already unit" constructor
(compile-fail doctests pin the field and the witness door). The three
`frame.rs` ladders mint through `new` under their own funnel names and
map refusals through one total `refused_direction`; `after_decided_length`
and `decided_unit` are gone, so nothing in `linalg` can mint an
undecided witness. Consumers: `frame_from_unit_aim(aim)`,
`UnitVec3::orthonormal_basis`, `viewer/datums.rs` `basis`/`plane_segments`/
`axis_segments`, WIRE's `wire.rs` `unit()` returns it and `transform_map`
and the stepped-rule operands carry it. `topo` does not re-export;
every importer names `geom_core`. Bit-identity held over the four
suites (3682 rows) and the interval feature. The bare
`Vec3::orthonormal_basis` stays (its two callers make no decision —
filed, not retired). Reviews: dual, both APPROVE WITH FIXES; R2's three
MAJORs were claim-class (the PR body's sweep freshness and two doc
sentences), corrected, not code; thirteen fix-pass items taken. Rows
filed: VIEW, PROPS ×2, BOOL; widened CURVED, issues, EXCH; one TRIM row
withdrawn as a false positive.
