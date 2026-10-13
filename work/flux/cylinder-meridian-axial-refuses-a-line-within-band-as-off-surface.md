---
id: cylinder-meridian-axial-refuses-a-line-within-band-as-off-surface
kind: issue
title: The cylinder iso parse refuses as OffSurface a line within band of the cylinder that is not a ruling
status: open
opened: 2026-10-13
priority: P3
cost: E
---

Found by the FLUX iso-parse-incidence unit's sweep (`flux/iso-parse-incidence`),
which moved the rim half of the same class: the iso parses decided a
circle's deviation from the iso rim as its incidence on the surface.

The line half remains. `cylinder_boundary`
(`crates/geom-brep/src/props/curved.rs`) decides `props_meridian_axial`
at `‖dir × â‖·L` under `Premise::OnSurface`. That is the line's
deviation from the ruling through its start, not its distance from the
cylinder: a line of length `L` that starts on a radius-`R` cylinder and
tips by `θ` in the tangent plane leaves it by about `(L·θ)²/(2R)`, so
it lies within band and is not a ruling, and refuses `OffSurface`.

The cone's `require_cone_generator` has the same shape:
`props_meridian_generator` meters the direction's departure from the
half-angle levered by the reach, and `props_meridian_apex` the line's
offset from the apex, and a line tipped tangentially to the cone leaves
it to second order in the same way.

Fix shape, as for the rims: decide the line's true distance from the
surface as `OffSurface`, then its being a ruling or generator under
`Premise::Inventory` (`NotIsoRectangle`). Rows: a tangentially tipped
line within band on each, and a radially tipped one refusing
`OffSurface`.

