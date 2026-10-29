---
id: the-c0-knot-split-refuses-what-it-cannot-place
kind: issue
title: the import's C⁰-knot split refuses a face whose knot line it cannot place, and an edge knot whose vertex the closing join would take back
status: open
opened: 2026-10-29
priority: P1
cost: H
design: true
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, tier-3-refuses-a-carrier-with-an-interior-c0-knot]
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

`crates/step-import/src/knot_split.rs` splits a face chart along a crease knot's iso line only where the line crosses the whole chart and parts the face's single loop at two boundary points, one chain on each sub-patch (or, on a chart closed across the split direction, one walk of the closed boundary on each). Everything else refuses `StepImportError::C0KnotUnsplit`, naming the face and the knot. Each refusal below is a shape the door owes a split for and does not yet make:

- **A face with rings.**
- **A trim that crosses the knot line more than twice**, or touches it away from the chart's two boundary rows: the line then parts the face into several pieces, or into pieces whose loops need outer-loop inference. Placing it needs the line's crossings with the trim, a curve-surface inversion the door does not have (`chart::uv_of` has no NURBS arm).
- **A face lying on one side of the knot line**, along it or clear of it. Its cell holds no crease strictly inside, yet its chart still carries the knot. Whether the door should restrict the chart to the sub-patch the face covers, a re-mint with no topology change, turns on how `tier-3-refuses-a-carrier-with-an-interior-c0-knot` reads "strictly inside its cell": over the chart's whole knot vector, or over the face's domain. That is the design question this row carries.
- **An edge knot whose vertex nothing else meets** (`fixtures/c0-knot/seam_edge.step`). The import ends with the join, and the join reads a valence-2 vertex between two edges on one surface pair as one edge cut for no reason (`topo::boolean::edge_join`, `joinable`). On a plane × spline pair it refuses `JoinUndecided` on a NaN margin; past that, `joined_spec` cannot run a spline sub-arc on through its own domain end. Tier 3's check 11 reads the same predicate at rest. DESIGN [topology] says the kernel adds a cut where a carrier is not C¹ by its knot structure, and maximal edges say the join removes a vertex nothing else meets. Which yields needs a ruling: the join keeps a C⁰-knot vertex (a `joinable` change in topo, check 11 with it), or the join re-derives one C¹ carrier over the joined locus (spline-carrier joining is unbuilt).

Until each lands, the file refuses typed, where on the merge base it imported with the crease inside a cell (and the mesher refused it).
