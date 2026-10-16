---
id: a-line-held-by-a-curved-touched-face-refuses-at-the-edge-edge-event
kind: issue
title: A common line held by a curved face a vertex touches refuses at the edge-edge event instead of reading the face's tangent half-plane
status: open
opened: 2026-10-16
priority: P3
cost: E
---


Filed by the lane that closed
`an-edge-along-a-partners-ray-through-a-touch-has-no-record`.

## What

At a vertex pair whose vertex of one solid touches a face of the
other's (`vtxfac`'s pierce read again), the edge-edge event along a
common line through the point reads the face's solid as its wedge and,
where the face holds the line, the half-turn under the face too
(`crates/topo/src/boolean/recl.rs`, `HeldFace::holds` and
`resolve_edge_edge`'s membership). A curved face holding the line to
first order refuses there, `PartnerAlongFace { along: Curved }`: which
side of the face the line's neighbourhood takes is second order, and no
reading of it exists at that site.

The partner beside the touch refuses the same way first wherever one of
its bounds lies on the curved face (`vtxfac::partner_side`). The
refusal in `recl` is reachable in principle where only the other solid
has an edge on the line and the face's solid has no bound on the face
there: a convex cylinder tangent along a ruling that the other solid's
edge lies on. No row builds it yet.

The same gap has a second site. A curved face among a solid's own
sectors holding the line beside its edge is not folded into the edge's
wedge as a plane one is (`recl::Folded`); the contact-line arm meets its
half-turn on its own, blind to the edge, as it read every face before.
That is the misreading the plane case retired, in curved form. Where
the touch's other end reads the same face as a held one, that end
refuses (`Curved`), so the two ends never disagree silently; but the
curved half-turn is still read without the edge.

## The shape to give

A row that reaches each site (a solid whose own vertex lies on its own
curved face, an edge of each solid along a ruling of that face through
the point; and a curved face beside an edge among a vertex's own
sectors), and either the second-order reading (the two carriers' relative
curvature, as `sectors::tangent_lump` reads a declared tangency) or a
pin that it refuses typed.

