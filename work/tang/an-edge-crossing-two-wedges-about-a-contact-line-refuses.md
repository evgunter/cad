---
id: an-edge-crossing-two-wedges-about-a-contact-line-refuses
kind: issue
title: An edge crossing two wedges about a contact line refuses: two edge-edge germs on one ray at one vertex
status: open
opened: 2026-10-08
priority: P2
cost: M
---

## What

Found while closing `three-solids-touching-along-one-line-refuse-their-union`.

An operand whose vertex holds a contact line (two coincident edges on
one ray, each bounding a wedge of its material) meets the other
operand's edge along that ray. `boolean/recl.rs` `recl_edges` resolves
each edge of one solid against each edge of the other by the edge-edge
rule (`resolve_edge_edge`), and refuses `ClassificationInvariant {
"an edge crosses two wedges about a contact line on one ray (unbuilt)" }`
when two of those pairs cross: the edge's wedge overlaps both of the
other solid's wedges, two germs at one vertex along one ray.

Row: `crates/topo/tests/three_solids_on_one_line.rs`,
`a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed`,
"both wedges crossed": the plate, prisms over 0°–50° and 120°–170°,
and a third over 30°–140° (z 0.44–1.81) bridging the gap. Measured on
this branch: the 6 orders that fold the bridging prism last refuse
there; the other 18 build sound ([22, 55, 36], closed-form volume,
3/3′, the analytic oracle). The row asserts the refusal and fails when
it builds.

The two germs run along one edge of the result between two crossing
pairs of one vertex, which is the shape `Reversed` already folds Out
for a pinch's two vertices; here the two pairs share one vertex, and
`place_germ` holds one germ per sector pair. Whether the two germs
can each go on their own flanking record, both folded Out, is the
question.

## The coincidence door

As the parent row: the bridging edge lying on the contact line's ray is
`bool_ee_collinear`'s margined verdict, read from values; no
declaration is consulted.

