---
id: perpendicular-part-over-a-small-norm-outside-section-cert
kind: issue
title: Four perpendicular parts divided by a small norm outside section_cert (join, offset_axial, props)
status: open
opened: 2026-10-06
---


A vector's part square to a unit axis spelled `w − a·(a·w)` (or
`c − foot` with `foot = o + a·(c − o)·a`) keeps a component along `a`
of `w`'s rounding, `~u·|w|`. Divided by its own norm `e`, that is a
direction tilted `u·|w|/e` toward the axis, and `e` is small exactly
when the point is near the axis or the origin stands far along it.
`topo::boolean::section_cert` routes every copy through `square_to`
(`a × (w × a)`, square to `a` to its own rounding); PR #4159 found and
fixed the last one there (`sphere_cylinder`). These four, outside that
file and its owners' ground, are the same shape and are not fixed. None
is measured; each is read from the code.

- `crates/topo/src/boolean/join.rs` `parallel_radical_plane` (~1580):
  `w = delta − a1·(delta·a1)` goes to `UnitVec3::new` as the radical
  plane's normal with `|w|` decided only above the band. Two parallel
  cylinders storing their origins at different stations (large
  `|delta|`) and nearly coaxial (small `|w|`) tilt the normal along the
  axis by `u·|delta|/|w|`; a tilted plane cuts a wall in a conic, which
  ends in `JoinDesync`.
- ~~`crates/topo/src/boolean/join.rs` `cs_transverse_frame` (~2050)~~:
  retired 2026-10-27 (SECT, `the-section-of-a-surface-pair-is-one-object`);
  its successor, `geom_brep::CylinderSpherePose::decide`, takes the
  centre's part square to the axis as `a × (w × a)`. The finding as it
  stood:
  `off = center − foot`, the frame axis `off / d` with `d` decided only
  above the band. `foot` is a rounded point, so the error is
  `u·max(|center|, |foot|)/d` in every direction: a sphere centre near
  the cylinder's axis, or a model far from the origin. The fix is
  `square_to(center − origin, axis)`, not a re-projection of `off`.
- `crates/topo/src/offset_axial.rs` `solve_corner` (~1162, read at
  ~1441, ~1490, ~1494): `e_old = radial_old / rho_old` with `rho_old`
  decided only above the band; `frame.place(rho, h, e_old)` lands the
  new corner off its station by `u·|here − origin|·rho/rho_old`. Mild:
  it needs a near-axis corner moved far out (a corner near a cone apex).
- `crates/geom-brep/src/props/curved.rs` `side_on_meridian` (~2744,
  ~2754): `out = radial / |radial|` with `|radial|` decided only above
  the band; the `(p − center)·out` half-plane decision carries
  `u·r²/|radial|` when the loop passes near the pole. Mild: a cross
  check, so a wrong sign refuses the face.

Owners: `join` (the first two), `offset`/`shell` (the third), `flux`
(the fourth). The fix in each is `square_to`'s spelling or a local
copy of it, with a row that measures the point or plane against its
carriers near the grazing pose and is red without it.

## 2026-11-02: a cited site retired (SECTARMS stage 3)

Branch `sectarms/cylinder-join-reads-value` retires `topo::boolean::join`'s
`cylinder_pair_axes`, `cylinder_pair_frame`, `crossing_cylinder_reading`
and `parallel_radical_plane`. The join now reads two cylinders on the
section value (`join::cylinder_read`), and the parallel lane takes its
plane from `geom_brep::CylinderCylinderPose::rulings_plane`. The hit
above that cites one of those sites no longer exists. The rest of the
row stands as filed.

## 2026-11-02: the rulings plane retired too (SECTARMS stage 3b-i)

Branch `sectarms/cylinder-carried-closed-form` retires the parallel
lane's plane: `geom_brep::CylinderCylinderPose::rulings_plane` and
`topo::boolean::join`'s `GermLane::Rulings`, `rulings_are_straight` and
`rulings_normal` are gone. A parallel cylinder pair now rides its
carried rulings (`chord_join::carried_chord_spec`'s closed-form arm).
The note above that names the rulings plane no longer points at live
code. The rest of the row stands as filed.

Moved from `work/issues/` to `work/honeone/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.
