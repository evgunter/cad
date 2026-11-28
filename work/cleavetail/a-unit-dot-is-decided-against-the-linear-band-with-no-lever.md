---
id: a-unit-dot-is-decided-against-the-linear-band-with-no-lever
kind: issue
title: A dot of two unit vectors is decided by Margin::of against the linear band, with no lever, at three sites
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by the LATHE one-site lane's dual review (PR 7476, 2026-11-23),
whose fix of `boolean/zip.rs` `one_vertex_sense` (a raw tangent dot
decided against the linear band, misdeciding at millimetre scale under
ε 1e-6) named the class. Not measured misdeciding at any site below.

## What

`Margin::of` admits a quantity that IS a length (`geom_core`
`predicate.rs`, `Margin::of`'s doc). A dot of two unit vectors is a
cosine, dimensionless; deciding it against the linear band compares an
angle with a length, so the angle tolerance it implies is ε radians at
every scale, not ε over the geometry's reach. The door for it is
`Margin::levered(cosine, arm)`.

## Sites (each to be judged)

- `crates/topo/src/boolean/ops.rs` `apply_cut_ins`'s recut,
  `bool_sphere_recut_seam`: `Margin::of(a.get().dot(seam))`. The flip it
  decides picks between the seam's two half-planes, so a misread near
  zero may only pick the other, equivalent one; judge whether either
  choice is sound there. (ops.rs: this slate and the other topo boolean
  programs.)
- `crates/topo/src/rim_wedge.rs` `curve_sense`,
  `seam_rim_axis_sense`: `Margin::of(axis.dot(rim.axis))`, two axes
  that are parallel or antiparallel, so it reads ±1 and misdecides only
  for a band near 1. (rim_wedge.rs: germ, splice.)
- `crates/topo/src/shell.rs` `shell_walls_antiparallel_cosine`:
  `Margin::of(-(a.normal.dot(b.normal)) - 1)`, decided Zero: walls
  antiparallel within ε in cosine, i.e. within about √(2ε) radians
  whatever the walls' extent. The likeliest to misdecide, at a large or
  a small shell. (shell.rs: shell.)

## Sweep that found them

`Margin::of\(.*\.dot\(` over `crates/topo/src`, read by hand for unit
operands; blind spot: a dot bound to a name first and passed later.
