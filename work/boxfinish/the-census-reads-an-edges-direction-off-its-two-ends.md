---
id: the-census-reads-an-edges-direction-off-its-two-ends
kind: issue
title: The census reads an edge's direction off its two vertices where the ray's pass reads its stored line
status: open
opened: 2026-11-15
priority: P2
cost: M
refs: [6958]
---

Left by the aimed-ray unit (PR 6958), disclosed in its body and raised by
its second review.

**What is there.** One file holds an edge's direction in two forms.
The census's exact sweeps read `EdgeGeo` (`crates/topo/src/census.rs:348`),
whose direction is the difference of the edge's two vertices
(`line_edges`, `census.rs:2377`). The ray's edge pass (`line_passes`,
`census.rs:11493`) reads the edge's stored line (`edge_line_span`,
`census.rs:2354`), since PR 6958.

**Why it was left.** The PR's author: a direction taken from the carrier
with ends taken from the vertices disagrees with itself by the body's
tolerance, and the census's sweeps read from both ends and intersect.
Not measured.

**Why it matters to this track.** Over a parameter box a vertex
difference carries the box twice, where a stored line's direction does
not once the sweep doors store the direction they were handed
(`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`).
Until then the two agree: the door mints an edge's line from its two
boxed ends.

**What closes it.** After the doors store exact edge directions: measure
which census readings over a boxed door body are wide only because
`EdgeGeo` reads two ends, and either read the stored line there or say
why the two-end reading is the right one for the exact sweeps.
