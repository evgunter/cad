---
id: a-clips-cover-joins-its-edges-ranges-by-overlap-with-no-shared-vertex
kind: issue
title: sides_are_the_faces' cover joins a clip's edges wherever their stored ranges touch or overlap, with no shared vertex, on the outer hulls of their ends
status: open
opened: 2026-11-21
priority: P3
cost: E
---


## Finding

From the `embed/collinear-cover` lane (2026-11-21), by reading, not by a
failing body.

`crates/topo/src/injective.rs`, `sides_are_the_faces`, reads a clip's
side as the face's own where `covered` holds of the ranges `run_on`
returns: every edge of the merged clip (`edge_ranges`, each range the
outer hull of its two stored ends, `along`), run on past its corners.
`covered` sorts the ranges and joins any two that touch or overlap
(`if a > reach { break }`). A merged clip's edges are the edges on one
side of the cell whose lines hold a common value (`present_chart`'s
merge); nothing asks that two of them share a vertex.

So two edges of one line that END AT TWO VERTICES, their stored ends
within each other's enclosures, are read as covering the side between
them. If the face comes through between those two vertices (a slot no
wider than the two ends' enclosures), the rectangle is taken for the
face's part of the cell where it is not. At `f64` an end is a point
(`the-lift-reads-a-rows-ends-as-points-at-f64-so-a-joints-two-rows-read-a-rounding-apart`),
so the two ends must be equal bit for bit; at `Interval` they overlap
wherever their enclosures do. Latent: no body is known to reach it, and
the gap it would bridge is at most the enclosures' width.

**Can it cover a side the boundary does not run along?** Only between
two vertices whose stored ends touch or overlap. Measured on a unit
fixture (two edges of the line `v = 0.5`, the face above, over
`[0.2, 0.52]` and `[0.48, 0.8]`, ending at two vertices; the cell
`u ∈ (0.45, 0.55)`, `v ∈ (0.5, 0.6)`): `present_chart` reads it
`Of`, the line its side, where neither edge covers the side alone and
no vertex joins them. No fixture of a finished body was built, and
none is expected at `f64`: two edges of one line with the face on the
same side whose ranges overlap run the boundary twice over the
overlap, and two that touch at two vertices put two vertices at one
point, neither of which a body at rest holds. Another loop, a ring or
a seam's other use does not reach it either: a seam is no clip, and a
ring's or second loop's edge on the same line and side is joined only
through the same touch. At `Interval` the touch is as wide as the two
ends' enclosures, which is the latent part, and why this is P3.

`vacuous` does not share this: since the collinear-cover lane it joins a
line's edges only at a vertex they share by key (`joints`,
`run_through`), and its unit row
`edges_of_a_line_that_share_no_vertex_are_not_joined` pins that
overlapping ranges on two vertices are not joined there. The same
row pins the cell ABOVE the line as it reads (`Of`, for ends that
overlap and for ends that are equal, at two vertices): the two cover
rules disagree on one line, by key below it and by overlap above
(the review's probe R4). The pin goes red the day this row is fixed.

## What it calls for

Read a clip's cover as `vacuous` reads its line's: each edge alone, or
the edges joined at shared vertices (`joints`), never two ranges that
merely overlap. That narrows what `sides_are_the_faces` admits, so it
needs the measurement first: which holding bodies of
`self_intersection_proves_injectivity` and the tour walk rely on an
overlap with no shared vertex (a print in `covered` of the ranges that
joined, against `joints`' pairs). The cover reading is ground the
sliver and touch concessions rest on, so the change is a question for
the orchestrator before it is a diff.
