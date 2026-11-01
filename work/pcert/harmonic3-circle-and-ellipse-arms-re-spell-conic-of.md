---
id: harmonic3-circle-and-ellipse-arms-re-spell-conic-of
kind: issue
title: pcurve_cache's Harmonic3 Circle and Ellipse arms re-spell geom_brep::Conic::of
status: open
opened: 2026-10-14
---

Found by the class sweep on `cleave/carrier-cross-one-curve` (conic
frames re-spelled per kind instead of read through
`geom_brep::Conic::of`).

`crates/geom-brep/src/pcurve_cache.rs`, the `Harmonic3` constructor
from a carrier: its Circle arm (`a: u_ref * radius`,
`b: axis.cross(u_ref) * radius`) and Ellipse arm (`u_ref * major`,
`axis.cross(u_ref) * minor`) are `Conic::of`'s two arms re-spelled. One
arm reading `Conic::of(&carrier)` (`a = u_ref·major`,
`b = v_ref()·minor`) covers both, and drops the read of
`Curve3::Ellipse`'s field names, which PR 4861 renames.

`Harmonic3`'s `a` and `b` are `Conic::semi_axis_vectors()`, added on the same
branch as the one spelling of that frame.
