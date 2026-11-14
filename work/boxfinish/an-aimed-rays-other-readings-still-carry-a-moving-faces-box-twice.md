---
id: an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice
kind: issue
title: The aim's step, the loops' turn and a curved carrier's roots still read a moving face through differences that carry its box twice
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by the aimed-ray unit's sweep
(`the-side-reader-cannot-read-a-solids-orientation-over-a-box`,
branch `boxfinish/aimed-ray`). That unit made the ray its line through
its aim (`side_reader::aimed_ray`, `geom::Ray::starting`) and stopped
the edge pass reading the ray's direction back off two ends
(`census::line_passes`). Three readings of the same shape are left, a
value built from two quantities that share a boxed term, so the term's
width enters twice where the true value holds none of it. None is
behind a door refusal by itself; each narrows the width at which a
ray reads.

**1. The aim's step into a moving face** (`side_reader::loop_aim`).
The aim is a point of the face's first edge stepped along
`normal × τ` by half the least clearance of the other edges
(`solid_contain::edge_clearance`). On a face whose every vertex shares
one boxed coordinate (a slab's top cap at height `h`), `τ` and each
clearance are differences of points that share `h`, so the aim comes
out wide in the face's own plane, where the true aim does not move.
Measured (the boxed brick of
`side_reader::aimed_ray_tests`, every plane normal exact,
`Sym<Interval>`): top at `h = 1 ± 0.1`, the top cap's aim is
`x ∈ [0.184, 0.318]`, `z ∈ [0.822, 1.178]`; at `± 0.3`,
`x ∈ [0.140, 0.848]`, `z ∈ [0.433, 1.567]`. The cap itself spans
`z ∈ [0.7, 1.3]` there.

**2. The turn of a face's loops about a crossing**
(`solid_contain::loop_turn`, `arc_turn`, and the wall's image in
`wall_crosses_by_turn`). The turn sums `atan2` over `a − p` and
`b − p`, `a` and `b` the face's vertices and `p` the ray's crossing of
that face's plane: on a moving face all three share the face's boxed
coordinate. Read from the code, not measured apart from the widths
around it: on the same brick at `± 0.3` the bottom cap's ray is set
aside on `side_at_infinity_winding` at the top cap,
`Enclosure { -3.57, 14.48 }`, but its exit there also comes within
0.02 of the cap's edge for some members, so how much of that is this
term is not separated.

**3. A curved carrier's roots, read about the line's nearest approach**
(`solid_contain::carrier_roots`, `About::Nearest`). `About`'s own doc
says what it costs at `Interval`: the shift is an enclosure, and the
moved offset and the root each carry its width. The reading exists
for a ray started far from the body. An aimed ray's origin is now its
aim, at the body, where `About::Start` is the reading the doc
recommends; point containment's rays still start wherever the query
is. Unmeasured: no curved body was read over a box in the unit.

**The test that says it is closed.** The boxed brick reads `Out` by
its rays at a top-cap box wider than `± 0.2` and `In` wider than
`± 0.1` (the widths `a_boxed_brick_reads_by_its_rays_the_side_its_members_have`
pins today), with the aim's enclosure no wider in the face's plane
than its inputs make it; and a cylinder whose height carries a box
reads by a ray aimed at its wall at a width where `About::Nearest`
sets it aside.

Item 1 changes how an aim is computed, not where it lands at `f64`;
`work/sideties/` holds the rows about where aims land, and
`side_reader.rs` is claimed by both programs.
