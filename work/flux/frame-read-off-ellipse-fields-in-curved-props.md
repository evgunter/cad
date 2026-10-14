---
id: frame-read-off-ellipse-fields-in-curved-props
kind: issue
title: props/curved.rs spells an ellipse's semi-axis frame from Curve3::Ellipse's fields instead of Conic::semi_axes
status: open
opened: 2026-10-14
---

Found by the review of PR 5058 (`cleave/carrier-cross-one-curve`),
which added `geom_brep::Conic::semi_axes` as the one spelling of a
conic's semi-axis frame `(major·u_ref, minor·v_ref)`.

`crates/geom-brep/src/props/curved.rs`, the ellipse arm near the
`let (a, b) = (u_ref * major, n.cross(u_ref) * minor);` line,
destructures `Curve3::Ellipse` field by field and re-spells that frame.
Reading `Conic::of(&e.carrier)` and `semi_axes()` there gives one home
for it, and drops the read of `Curve3::Ellipse`'s field names, which
PR 4861 renames.
