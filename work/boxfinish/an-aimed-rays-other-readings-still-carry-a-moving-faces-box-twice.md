---
id: an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice
kind: issue
title: A moving edge's line, the aim's step, the aimed face's advance, the loops' turn and a curved carrier's roots still carry a moving face's box twice
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Found by the aimed-ray unit's sweep
(`the-side-reader-cannot-read-a-solids-orientation-over-a-box`,
branch `boxfinish/aimed-ray`, PR 6958). That unit made the ray its line
through its aim (`side_reader::aimed_ray`, `geom::Ray::starting`),
stopped the edge pass reading the ray's direction back off two ends,
reads each line edge there off its stored line (`census::line_passes`,
`on_its_line`), and sets a ray aside where its gap from an edge is
undecided inside the edge's span (`solid_contain::edge_meets`). What is
left are readings of one shape: a value built from two quantities that
share a boxed term, so the term's width enters twice where the true
value holds none of it. None is a wrong answer; each narrows the width
at which a ray reads, and since an undecided edge gap now sets the ray
aside, the first is what stops most rays.

Measured on the brick of `side_reader::boxed_brick_rows` (every plane
normal exact, the top cap's height the one box, `Sym<Interval>`) and on
the extrude door's slab.

**1. A moving edge's own line** (the edge pass's gap,
`side_at_infinity_edge`). A top edge's stored line was minted from its
two ends, which share the cap's height `h`, so its direction's `z` is
`(h − h)/len`: on the door's slab at `h = 1 ± 0.05` every top edge's
carrier direction is `([0.995, 1], 0, [−0.1, 0.1])`, and each upright
edge's is `(0, 0, [0.905, 1.105])`. Reading the edge off its carrier
gives back nothing there, since the carrier is as wide as the chord:
the mint is the door's
(`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`;
the fixture's `EdgeCurveSpec::line_between` does the same). With it,
the gap between a ray and a top edge it passes 0.4 away encloses zero:
on the brick at `± 0.05` the four wall rays are set aside on
`side_at_infinity_edge` at the top cap, `Enclosure { 0, 0.637 }`, and
the two cap rays read; at `± 0.1` one of six reads; at `± 0.025` all
six. On the door's slab at `± 0.025` one ray reads (the bottom cap's).

**2. The aim's step into a moving face** (`side_reader::loop_aim`).
The aim is a point of the face's first edge stepped along
`normal × τ` by half the least clearance of the other edges
(`solid_contain::edge_clearance`). On a face whose every vertex shares
one boxed coordinate, `τ` and each clearance are differences of points
that share it, so the aim comes out wide in the face's own plane,
where the true aim does not move: top at `h = 1 ± 0.1`, the top cap's
aim is `x ∈ [0.184, 0.318]`, `z ∈ [0.822, 1.178]`; at `± 0.3`,
`x ∈ [0.140, 0.848]`, `z ∈ [0.433, 1.567]`.

**3. The aimed moving face's own advance**
(`ray_crossings`' plane arm, `t = (origin − aim)·n̂ / (d·n̂)`). The
plane's origin and the aim both lie on the moving face and share its
coordinate, so `t` is as wide as the box where the true advance is
zero. Read from the code; on the brick it is not what sets a ray aside
at the widths above, so its share is not separated.

**4. The turn of a face's loops about a crossing**
(`solid_contain::loop_turn`, `arc_turn`, the wall's image in
`wall_crosses_by_turn`): `atan2` over `a − p` and `b − p`, the face's
vertices less the ray's crossing of that face's plane, all sharing the
face's boxed coordinate. Read from the code, not measured apart.

**5. A curved carrier's roots, read about the line's nearest approach**
(`solid_contain::carrier_roots`, `About::Nearest`). `About`'s own doc
says what it costs at `Interval`: the shift is an enclosure, and the
moved offset and the root each carry its width. An aimed ray's origin
is now its aim, at the body, where `About::Start` is the reading that
doc recommends. Unmeasured.

**The test that says it is closed.** The boxed brick reads its side by
every face's own ray at a top-cap box wider than `± 0.025` (what
`a_boxed_brick_reads_the_side_its_members_have` pins today), with a
moving edge's line and the aim's enclosure no wider than their inputs
make them; and the door's slab reads by its first aimed ray, not only
its second.

Item 2 changes how an aim is computed, not where it lands at `f64`;
`work/sideties/` holds the rows about where aims land, and
`side_reader.rs` is claimed by both programs.
