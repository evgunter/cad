---
id: a-cone-wall-whose-rims-span-different-azimuths-refuses-containment
kind: issue
title: point_in_solid refuses PartialConeFace on a frustum wedge whose axial cap was offset outward: the wall's two rims span different azimuths
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-24
---


## What

`replace_face_offset` on an axial cap of a quarter frustum wedge (the
cone `tan α = 1/64` about `+y`, apex at the origin, `y ∈ [1/256, 3/8]`,
revolved by `π/2`) moves the cap by `±0.9·ε` and returns a right body:
its volume is `V + d·A` within `0.083·|d|·A`, and it passes tier 3, at
ε unset, `1e-6` and `1e-12`, both caps, both signs.

Moved **outward** (`d > 0`), `point_in_solid` refuses `PartialConeFace`
on the cone wall at 50 of 216 grid points at the default ε and 60 at
`1e-6`, interior points among them (`(0.003, 0.3, −0.001)` and
`(0, 0.2, 0.003)` both refuse); 0 at `1e-12`. Moved inward, and on the
unmoved wedge, every point answers. Every point it does answer agrees
with the wedge's membership.

The row is `a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`
in `crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`, which
tolerates those grid refusals through `solid_truth::assert_body_is`.
This item's fix shows there as more answered points.

## Why, by reading

The moved cap meets each rim circle `d/r` in azimuth past where it met
it before (`r = y·tan α`): `d/r₀` at the small rim (`r₀ ≈ 6.1e-5`),
`d/r₁` at the large one (`r₁ ≈ 5.9e-3`). So the wall face's two rim
arcs no longer span the same azimuth, though every vertex is within ε
of where it belongs. The face's chart image is then not its window's
rectangle, and `cone_trimmed_window` (`crates/topo/src/boolean/solid_contain.rs`)
refuses `PartialConeFace` for a boundary that is not its window's
rectangle (`chart_polygon_box` answering `None`). This is read, not
traced; the branch that refuses has not been instrumented. Moved
inward, the azimuth difference has the other sign, and the box check
evidently passes.

The volume lane had the same geometry (`du_of_rims`,
`props_du_consistent`) and now measures this body.

## The shape of a fix

Either read a cone face whose rims' spans differ by a band-sized
amount as its window (the difference is `d·(1/r₀ − 1/r₁)` in azimuth,
`≤ |d|` in length at each rim), or let the offset door land the rim
arcs' ends on the served generator.

## Closed by CHART (2026-10-24)

On branch `chart/ringed-curved-doors`
(`curved-face-containment-answers-none-on-a-ringed-face`), a cone face is
read from its boundary edges (`topo::boolean::ruled_region`), whatever
azimuths its rims span. `a_cap_moved_off_a_frustum_wedge_apex_plans_its_wall_edge`
now allows no refusal, and every grid point answers its truth at ε
`1e-9`, `1e-6` and `1e-12`. `solid_truth`'s `PartialConeFace` allowance is
retired with it: `a_ring_on_a_cone_face`, `reach_cone_root_lane`,
`cone_join_lane` and `reach_cone_split` answer every point.
