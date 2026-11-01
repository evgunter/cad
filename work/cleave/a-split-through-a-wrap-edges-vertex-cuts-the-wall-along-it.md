---
id: a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it
kind: issue
title: A split or boolean whose section loop meets a one-face wall's wrap edge at one vertex leaves that wall's piece as two faces on one surface across the wrap edge: a cut the author did not write
status: closed
opened: 2026-10-06
priority: P2
cost: M
branch: cleave/wrap-edge-split
closed: 2026-10-11
pr: 4722
---



Found by PATHS `one-segment-loop-revolves-and-lofts-to-one-wall` (the wrap-edge unit, #4175's implementation).

**What.** Extrude a one-segment circle (r = 1 about the z axis, 2 high, its strut on the meridian through its one vertex) and `topo::split` it by the plane through the bottom vertex with normal `(cos φ, sin φ, 1)` (`crates/sweep/tests/one_segment_loop.rs`, `a_split_through_the_seam_builds_as_the_two_arc_form_does`, "through the bottom seam vertex"). The section ellipse touches the strut only at that vertex, so the piece of wall above the plane is one connected region with the whole strut inside it: one face whose wrap edge is the strut. The split returns it as TWO faces on the one cylinder surface, with the strut between them.

**The boolean does the same.** A one-segment circle (r = 1) extruded 2 on a sketch plane tilted 0.4 or 0.9 rad about x, its vertex at z = 0, subtracted from and intersected with the half-space z ≤ 0, returns census (V, E, F) = (3, 5, 4) where the two-arc form returns (4, 7, 5); `split` at z = 0 returns the same (3, 5, 4). Volumes match the two-arc form's and every tier passes: the extra face pair is the same one the split leaves (review probe of #4226).

**Why it matters.** D1 now reads a wrap edge as a fact about one face ("a wall keeps the cuts its author wrote and the kernel adds none"), and tier 3 holds the two halves of a wrap edge to one face (`topo::validate`, check 2's adjacency arm). The split's extra face boundary is a cut nobody wrote. This unit keeps the result valid by re-describing a wrap edge the split or boolean has parted as an ordinary image in its chart (`Body::rest_parted_wrap_edges`, called from `splitting::finish::split_finish` after the section boundary pass; `boolean::ops::describe_edges`'s smooth arm), so volumes and tiers are right; the topology still carries the extra same-key face pair.

**Where to look.** Both paths:

- the split: `crates/topo/src/splitting/finish.rs` (`split_finish`) and the reduction that places the self-loop section edge at the vertex the wrap edge passes through;
- the boolean: `crates/topo/src/boolean/ops.rs` (`describe_edges`, whose smooth arm restates the parted wrap edge as an ordinary image in `s1`'s chart) and the section placement upstream of it.

In both the piece should come back as one face whose loop uses the strut twice.

## Built (branch cleave/wrap-edge-split)

**The premise is false; neither door has the defect.** No kernel
change. Measured on `origin/main` at `500361fc5f`, ε = 1e-9, 1e-6 and
1e-12.

**The split's two faces are forced.** The row's split pose ("through
the bottom seam vertex", normal `(cos φ, sin φ, 1)`) passes through the
vertex and the axis's midpoint, so it is tangent to the TOP rim at the
vertex's antipode `P`. The piece above, `1 − cos θ < z ≤ 2`, is pinched
shut at `P`: a disc, not an annulus, and the strut runs across it from
the section to the top rim. A face with both sides of a chord in it
has an interior in two pieces. Its census would be `(V, E, F) =
(3, 5, 3)` (V0, V1 and `P`; the strut, two section arcs, two top-rim
arcs), and tier 1's Euler–Poincaré check refuses odd `V − E + F`. The
split returns `(3, 5, 4)`: two faces on the cylinder, the strut
between them, described as an ordinary image. The two-arc form returns
the same census, because its second vertex IS `P`. The strut is the cut
the author wrote (D1: "where it sits is where the construction cut the
wall"), so the two faces are D1's answer, not a cut the kernel added.

**The boolean's census is one face.** The row's boolean poses (tilt
0.4 or 0.9 about x, vertex at z = 0, A − B with `z ≤ 0`) return
`(3, 5, 4)`, which is one wall face with the strut as its wrap edge
(both halves on it, the flag set). The piece above wraps round, since
the plane crosses the bottom cap and misses the top. The two-arc
`(4, 7, 5)` is that plus its second strut and vertex. The census
coincides with the split's, but the topology does not. Where a
boolean's piece IS a disc (tilt 1.3, or tilt ≥ π/4 at phase π/2), its
output stage glues the two wall faces into one maximal face and the
strut dies (DESIGN, "Maximal-faces precondition and the merge stage",
the sub-period re-merge). The two-arc form does the same.

**Rule, checked against closed forms.** With the plane's height over
the wall `z = g(θ)`, the piece that holds the strut is one face with
the strut as its wrap edge exactly where it goes round (`max g < 2`
through the bottom vertex), and two faces (split) or one merged face
with no strut (boolean) where it is a disc.
`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs` pins this.
Every result is held to all three tiers, the wall census, every edge's
wrap flag against its faces, and the closed-form volume
`∫∫ clamp(g, 0, 2) dA`:

- split, the cylinder: 2 vertex phases (both windings) × both vertices
  × 6 poses (round, pinched, disc, at three leans) × both normals ×
  offsets 0, ±0.6ε, ±0.9ε;
- split, the circle as a hole through a block: the crossing poses,
  offset 0;
- boolean: 3 phases × tilts ±0.4, ±π/4, ±0.9, ±1.3 × A − B, A ∩ B,
  A ∪ B, B − A × the same offsets.

At offset 0 every pose builds right on both doors, with two
exceptions. The boolean refuses `SingleSiteSectionLoop` where the plane
touches the near rim at the vertex and the piece goes round; the split
builds that pose and the two-arc form builds the boolean
(`work/secttail/a-plane-tangent-to-a-rim-at-its-wrap-edges-vertex-has-no-join-arm.md`).
And `VolumeUnmeasured` on some poses (`work/quad/`, evidence added).

At ±0.6ε and ±0.9ε, only the poses that touch the near rim at the
vertex (and some pinched ones) build, and they build right. The rest
refuse typed with a margin in the band, and the two-arc form refuses
them alike. The crossing of the rim lies `offset / sin(lean)` from the
vertex along it (1.54ε at tilt 0.4 for 0.6ε), and the On vertex lies
1.1–1.7ε from the section's carrier: these are real distances in the
band (D4). One class is the subdivision device rather than the
geometry, and it is one-segment-only in the split
(`work/cleave/a-wide-sectors-subdivision-ray-along-the-planes-trace-reads-in-the-band.md`).

**PR 4622 (spurs, slits).** `strip_spurs` reads section polygons only,
and a wrap edge lies in a section polygon only when the plane holds
the strut. On the solid, the plane is then tangent (no section) or
through the axis (the strut is a rectangle side). On a hole, the
tangent plane refuses `KnifeEdge` (two-arc alike), and through the axis
it builds. None of these reads as a spur, and the pins pass on main
with that PR in.

The class sweep (holes) also found
`work/cleave/a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident.md`
(two-arc alike).

## Closed (PR 4722, 2026-10-11)

No defect. The row's split pose leaves a pinched-disc piece, where one face carrying the strut twice would fail Euler–Poincaré, so the two faces are forced (D1: the strut is where the author cut). The row's boolean census was misread. The behaviour is pinned across phases, tilts, both normals and in-band offsets, against closed-form volumes.
