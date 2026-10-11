---
id: a-moved-cap-on-a-frustum-wedge-reaches-props-du-consistent
kind: issue
title: a frustum wedge whose axial cap is offset measures no volume: its cone wall's two rims disagree on their span at props_du_consistent
status: open
opened: 2026-10-11
priority: P2
cost: M
refs: [pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span]
---

## What

`replace_face_offset` on an axial cap of a quarter frustum wedge (the
cone `tan α = 1/64` about `+y`, apex at the origin, `y ∈ [1/256, 3/8]`,
revolved by `π/2`) moves the cap by `±0.9·ε` and returns `Ok`, with
every edge planned (the wall edge keeps its generator). The moved
body then fails tier 3:
`VolumeUncomputable { source: Face { NotIsoRectangle { what:
"props_du_consistent" } } }`, on the cone wall, at ε unset, `1e-6` and
`1e-12`, for both caps and both signs (eight bodies per ε).
`mass_properties` refuses the same way.

The row that builds it is
`a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge` in
`crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`. It checks the
plans and the planned wall edge against the closed-form section, and
does not read tier 3, citing this item.

## Why, read off the geometry

The moved cap meets each rim circle `d/r` in azimuth past where it met
it before (`r = y·tan α`): `d/r₀` at the small rim (`r₀ ≈ 6.1e-5`) and
`d/r₁` at the large one (`r₁ ≈ 5.9e-3`). So the wall face's two rim arcs no longer span the same
azimuth, though every vertex is within ε of where it belongs.
`du_of_rims` (`crates/geom-brep/src/props/curved.rs`) requires the
rim groups' span sums equal, `props_du_consistent` levered at
`arms.azimuth`, and the difference `d·(1/r₀ − 1/r₁)` levered there is
definite.

`du_of_rims`'s own doc records `props_du_consistent`'s reachability as
"unsure", with no loop constructed whose groups disagree. This is one:
a cone wall whose rims are cut by a plane through the axis moved off
it. The cylinder's rims share a radius, so the same move leaves their
spans equal; on a cone they differ by `d·(1/r₀ − 1/r₁)`.

Nothing here is specific to the apex: the move is `±0.9·ε` because the
row is the plane×cone apex arm's, and the wall edge is the generator
that arm serves. Before PR 4592 main served the same generator here,
through the same construction, so by reading the same refusal stood
there; it was not run.

## The shape of a fix

The face's span is one number only where its rims' spans agree, and a
cone face trimmed by a plane that holds no generator has rims whose
spans differ by the band-sized amount above. Either the volume lane
measures such a face (the rims' spans as two numbers, the wall as the
ruled region between two arcs joined by straight-to-band edges), or the
offset door re-derives the rim arcs' ends so they sit on the served
generator; which one is the flux program's call.
