---
id: unit-vector-witness-in-geom-core
kind: unit
title: The unit-vector witness moves to geom-core, minted by the decided-normalize ladder, and from_frame's consumers take it
status: open
opened: 2026-09-15
branch: scalar/unit-vector-witness
pr: 2646
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
