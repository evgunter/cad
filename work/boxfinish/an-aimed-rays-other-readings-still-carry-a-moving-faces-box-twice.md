---
id: an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice
kind: issue
title: A moving edge's line, the aim's step, the aimed face's advance, the loops' turn and a curved carrier's roots still carry a moving face's box twice
status: closed
opened: 2026-11-14
priority: P1
cost: M
branch: boxfinish/ray-readings
pr: 7480
closed: 2026-11-24
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

Each item says what it rests on. The measured ones were taken on the
brick of `side_reader::boxed_brick_rows` (every plane normal exact, the
top cap's height the one box, `Sym<Interval>`) and on the extrude
door's slab.

**1. A moving edge's own line (measured)** (the edge pass's gap,
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

**2. The aim's step into a moving face (measured)** (`side_reader::loop_aim`).
The aim is a point of the face's first edge stepped along
`normal × τ` by half the least clearance of the other edges
(`solid_contain::edge_clearance`). On a face whose every vertex shares
one boxed coordinate, `τ` and each clearance are differences of points
that share it, so the aim comes out wide in the face's own plane,
where the true aim does not move: top at `h = 1 ± 0.1`, the top cap's
aim is `x ∈ [0.184, 0.318]`, `z ∈ [0.822, 1.178]`; at `± 0.3`,
`x ∈ [0.140, 0.848]`, `z ∈ [0.433, 1.567]`.

**3. The aimed moving face's own advance (read from the code, not measured)**
(`ray_crossings`' plane arm, `t = (origin − aim)·n̂ / (d·n̂)`). The
plane's origin and the aim both lie on the moving face and share its
coordinate, so `t` is as wide as the box where the true advance is
zero. Read from the code; on the brick it is not what sets a ray aside
at the widths above, so its share is not separated.

**4. The turn of a face's loops about a crossing (read from the code, not measured)**
(`solid_contain::loop_turn`, `arc_turn`, the wall's image in
`wall_crosses_by_turn`): `atan2` over `a − p` and `b − p`, the face's
vertices less the ray's crossing of that face's plane, all sharing the
face's boxed coordinate. Read from the code, not measured apart.

**5. A curved carrier's roots, read about the line's nearest approach (read from the code and `About`'s own doc, not measured)**
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

## Re-banded P1 (2026-11-15, M4): it holds the switch's deletion

With the frames built (PR 6972) and the switch's box arm off, a slab
boxed wider than ± 0.215 of its depth still loses every aimed ray and
then the volume reading (`OrientationUndecided { reading: Volume {
Enclosure{-0.9699, 2.9699} }, aimed: 6, set_aside: 6 }` at depth
`[0.75, 1.25]`): 40 refusing finishes across the box studies, one
extra bisection level, and two red rows (m10_8's decision pin; m10_3's
`the_certification_width_is_no_longer_bounded_by_epsilon`, which
settles at a half-width of 0.215 where it wants about 1). The stored
normals are exact now; per ray the refusals are the caps'
`side_at_infinity_winding` (`{-3.827, 13.758}`, `{-1.412, 8.154}`,
raised by `turned_inside` / `crosses_by_turn`), `side_at_infinity_edge`
(`{0, 0.177}` to `{0, 1.227}`) and `side_at_infinity_advance`
(`{-1.199, 0.927}`). `delete-the-box-scalar-sweep-leaf-switch` is
parked on this row, so it takes that row's band.

## Closed

PR 7480 (2026-11-24). Over a box the side reader's in-plane turn is
read with no frame chosen (`solid_contain::planar_cross_dot`), the
aim's step reads the edge clearance in the face's plane
(`edge_clearance(.., across)`), and an aimed ray whose pair of
crossings a box cannot hold apart is set aside by the side reader
(the pairing rule itself, and point containment with it, are as they
were). The door's axis-aligned slab reads its side by its caps' rays at
depth 1 ± 0.25, and the bottom cap's ray to its real stop near 1.469.

What it did not reach, each with its own row: the walls still read
only to about ± 0.05
(`a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide`);
the aimed face's own crossing is as wide as the box
(`the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence`);
a cap whose normal has no zero component reads only to about ± 0.05,
the same on the merge base
(`a-moving-face-whose-normal-has-no-zero-component-reads-its-side-only-over-a-narrow-box`);
the curved and fallback readings; an aimed ray's own selected frame.
The first three stand before the switch's deletion in this row's place
(M10, taken without this PR, still saw every ray set aside past
± 0.215), so the first two are re-banded P1.
