---
id: two-pinches-crossing-on-one-line-refuse-their-union
kind: issue
title: Two pinches crossing on one line refuse their union: two crossing pairs share both their vertices
status: open
opened: 2026-10-03
priority: P0
cost: H
---


## What

Two pinches on one axis in complementary quadrants, overlapping in z:
`pinch_a = brick((0,1),(0,1),(0.5,1.5)) ∪ brick((-1,0),(-1,0),(0.5,1.5))`
and `pinch_b = brick((-1,0),(0,1),(1,2)) ∪ brick((0,1),(-1,0),(1,2))`,
each a Boolean output with its own records carried. Their union fills
all four quadrants over z ∈ (1, 1.5). At each end of the overlap each
pinch holds two vertices, and all four vertex-vertex pairs between them
cross, so two crossing pairs share both their vertices. The union
refuses `SharedVertexCrossings` in both operand orders
(`insert::reconcile_shared`, the guard on crossing pairs that share
both vertices, `crates/topo/src/boolean/insert.rs`). The subtract
builds. Pinned by `two_pinches_crossing_on_one_line_refuse_their_union_typed`
in `crates/topo/tests/union_flush_onto_edge_contact.rs`, the second
witness of `a-vertex-crossing-both-sides-of-a-pinch-refuses-shared-vertex-crossings`,
which `fuse/shared-vertex-crossings` built for the one-sided case only.

**P0** on that row's ruling (FUSE orchestrator, 2026-10-02): both
operands are Boolean outputs, and the next Boolean refuses them.

## What was measured

With the guard off, the null edges insert and the join completes,
but the finish cannot place the result. At (0,0,1) the union holds one
vertex whose link is a single cycle passing the −z direction twice:
the two pinch edges below it end there, coincident. The seam
correspondence then gives one A vertex both B partners. Before
`fuse/shared-vertex-crossings` admitted that, it refused `JoinDesync`
("conflicting seam vertex correspondence"). Admitted, the zip fuses a
vertex onto an edge's other end and refuses `Euler(SelfLoopEdge)`.

## Owed

Decide what the result is at rest: one vertex with two coincident
edges ending at it, or two vertices there with a v-v record between
them. Then build the zip arm that produces it, and flip the pinned
test to a build at volume 4 that passes 3′ with both records carried.

## Finding: neither reading is representable (`fuse/two-pinches-one-line`)

The union's stretch of pinch line below the overlap, z ∈ (0.5, 1),
has the bricks' convex corners at its lower end and the empty
corners' concave ones at (0,0,1). Along the stretch the four faces
x = 0 and y = 0 pair into two coincident edges, and only two pairings
orient: each brick's own two faces, or the two faces bounding one
empty corner. Two vertices at (0,0,0.5) need the first pairing, since
each brick's corner is bounded by its own faces. Two vertices at
(0,0,1) need the second, since each empty corner there is bounded by
one face of each brick. With the first pairing, the orbit at (0,0,1)
runs from one pinch edge through the empty corner's top face to the
other, so both edges end at one vertex whose orbit passes −z twice:
the measured result above. A vertex on the stretch that switches
pairings has the same orbit. That vertex passes tier 1 (its orbit is
one cycle), but it is a shared-entity wedge fan, which D1's tier 3′
holds unrepresentable and a typed error (`docs/DESIGN.md`,
"Representability boundary", ~352–358). The stretch above the
overlap is the mirror case at (0,0,1.5).

So refusing is the contract's answer for the union, and no zip arm
is owed. Its subtract (volume 2, passing 3′) and empty intersect are
pinned beside it. The end-to-end variant (overlap a single point)
builds in every op and passes 3′, with four vertices at (0,0,1):
`two_pinches_meeting_end_to_end_build_in_every_op`.

The refusal is now permanent and named: the both-shared arm of
`insert::reconcile_shared` refuses `BooleanError::NonManifoldResult`
(pncad-py tag `non_manifold_result`, no offer), naming A's vertex at
the point and two of B's, with the recourse to keep the pinch lines
from overlapping along a length. Every other `SharedVertexCrossings`
arm keeps its kind.
