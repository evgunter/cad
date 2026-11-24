---
id: a-convex-corner-across-a-shared-edge-is-read-at-the-rays-sine-and-in-smaller-cells
kind: issue
title: A convex two-line corner is read at the ray's sine across a shared edge, and only in smaller cells: two narrowings of the pieces against the quadrant
status: open
opened: 2026-11-24
priority: P2
cost: M
---


## Finding

From `embed/corner-pieces` (PR 7627) and its two reviews, 2026-11-24.
The unit reads every corner of two line edges on a plane face as two
pieces cut on the ray into the face, convex corners included (the
orchestrator's ruling: one reading, no lean gate). For a convex corner
that is narrower than the one quadrant it replaced, in two ways. Both
refuse more and certify nothing new; no verdict of the suite or the
tour moved.

**The sine across a shared edge.** The piece on a shared edge is the
cone of that edge and the ray, so under the partner's carrier it
departs along the ray: its sine is `sin δ · sin(θ/2)`, `δ` the dihedral
and `θ` the corner's angle, where the quadrant departed along the
corner's other edge at `sin δ · sin θ`. The review measured the least
certified dihedral rising by a factor of 1.16 to 1.41 at every corner
angle it scanned. At a right corner between two quadrants sharing an
edge, rims 4 long, the head certifies from `δ` = 3.2e-9
(`injective::presentation_rows::a_thin_wedge_on_a_convex_corners_edge_is_certified_from_the_rays_sine`
pins it between 2.8e-9 and 3.6e-9). The review found no valid body in
the window: a wedge under about 1.4e-8 thick at unit reach. At a vertex
shared alone the design's sentence holds (each piece's cone lies in
its face's, and the parting plane is chosen for the piece pair).

**Coarse cells.** A piece's reach along its edge is the cell's reach
measured parallel to the ray, about twice what the quadrant read along
the edge at a right corner, and unbounded toward a slit (a piece's
frame degenerates as `sin(θ/2)` falls). So a coarse cell at a convex
corner is refused `Local` where the quadrant presented it, and the pair
splits once more. The review's counts of `Local` readings on four suite
rows: 0 to 8, 0 to 16, 310 to 1,412, 0 to 39; one receipt moved by two
(the stepped ring's). The tour's walk moved 26 receipts, total splits
down 1%.

## What is needed

Decide whether a convex corner keeps the one-quadrant reading where
it is certified strictly convex (a lean decided `Positive` at
`SELF_INTERSECTION_CORNER`, the pieces otherwise), which restores both
at the cost of the gate this unit removed, or whether the pieces' frame
is chosen so a convex corner's piece departs along the other edge (the
quadrant cut on the ray, each piece a triangle's cone but with its
second axis the other edge), or neither: the window holds no body
found. Measure first on the census sets and the tour, cells refused and
receipts, with and without.
