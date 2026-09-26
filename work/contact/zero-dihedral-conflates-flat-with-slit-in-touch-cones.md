---
id: zero-dihedral-conflates-flat-with-slit-in-touch-cones
kind: issue
title: The census's touch cones read a Zero dihedral as flat, conflating a 180° seam with a folded 0°/360° slit that classify_material_pairing tells apart
status: open
opened: 2026-09-26
priority: P3
cost: E
---


Filed by CONTACT-1's dual-review fix pass (R1). In
`crates/topo/src/census.rs` the touch cones decide an edge's dihedral
as the sign of one dot (`census_touch_dihedral`, decided in
`Cone::fan` for the fans `Cone::wedge` and `Cone::vertex` build); `Zero` is read as a flat 180° seam (`RayKind::Edge`,
`ConeShape::Flat`). A folded edge — 0° or 360°, the two faces lying on
each other — reads `Zero` too. `geom_brep::classify_material_pairing`
makes exactly this distinction (aligned against opposed senses). The
fix pass made the decided-but-unsettling readings this produces
refuse typed (`Undecided::TouchDegenerate`) rather than as in band
when no other candidate certifies a rest; whether a slit read as flat
can reach a wrong rest through another candidate is not measured, and
a slit is not NAMED as one. Reachable at a vertex fan through an intra-solid declared patch
(a body certified with two of its faces lying on each other). No row
exercises it yet. Difficulty E.
