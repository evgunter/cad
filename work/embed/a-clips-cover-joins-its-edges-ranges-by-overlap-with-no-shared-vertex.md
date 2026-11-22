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

`vacuous` does not share this: since the collinear-cover lane it joins a
line's edges only at a vertex they share by key (`joints`,
`run_through`), and its unit row
`edges_of_a_line_that_share_no_vertex_are_not_joined` pins that
overlapping ranges on two vertices are not joined there. The same
fixture's cell ABOVE the line (the clip reading) is not pinned either
way.

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
