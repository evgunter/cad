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
its bounds lies on the curved face (`vtxfac::partner_side`), so no row
is known to reach the refusal in `recl`. It is filed so the gap has an
owner, not because a model refuses there.

## The shape to give

A row that reaches it (a solid whose own vertex lies on its own curved
face, an edge of each solid along a ruling of that face through the
point), and either the second-order reading (the two carriers' relative
curvature, as `sectors::tangent_lump` reads a declared tangency) or a
pin that it refuses typed.

