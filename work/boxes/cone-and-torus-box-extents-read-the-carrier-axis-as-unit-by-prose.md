---
id: cone-and-torus-box-extents-read-the-carrier-axis-as-unit-by-prose
kind: issue
title: The cone and torus box extents read the carrier's axis (and the torus's u_ref) as unit by prose; the cylinder slab decides it at the read
status: open
opened: 2026-10-02
priority: P3
cost: E
---

## Finding

The class sweep of TQUERY's
`split-plane-normal-and-slab-axis-carry-unitness-as-prose`. The
cylinder slab now reads a decided axis: `face_box_rule`
(`crates/topo/src/boolean/boxes.rs`) decides a cylinder carrier's axis
length under `bool_box_cylinder_axis` and hands `slab_extent` a
`UnitSpanBox`, minted only from a `geom_core::UnitVec3`. Its siblings
in the same file still read the carrier's bare field as unit:

- `cone_frustum_extent` — `h` is a length along the axis and
  `perp_room` bounds a coordinate of a unit vector perpendicular to a
  unit axis; `FaceBoxRule::ConeSlab { axis: Vec3<T> }`.
- `torus_extent` and `torus_window_extent` — the widening is
  `(R + r)·√(1 − axis_i²) + r·|axis_i|`, which under-claims for
  `|axis| > 1`; the window extent also reads `u_ref` and
  `axis × u_ref` as a unit orthonormal pair.
  `FaceBoxRule::TorusWindow { axis, u_ref: Vec3<T> }`.
- `edge_axial_span`'s `AxialCarrier::Conic { u_ref, v_ref }` and
  `conic_extent` read an edge carrier's reference directions as unit.

## What taking it would look like

The cylinder's spelling: decide each axis in `face_box_rule` (the band
is already in scope there), carry `UnitVec3` in the rule's arm, and
pass `UnitSpanBox` into the extent. The torus's `u_ref` wants an
orthonormal pair, not two independent decisions — an
`OrthoFrame`-shaped witness, which is the larger step. The cone lane
is fenced in `FaceBoxRule::ConeSlab`'s own docs as another unit's
jurisdiction; take it with that lane.
