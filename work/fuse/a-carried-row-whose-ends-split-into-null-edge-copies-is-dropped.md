---
id: a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped
kind: issue
title: "A carried v-v row whose ends the boolean splits into null-edge copies is dropped: remap_carried does not reach copies"
status: open
opened: 2026-10-03
priority: P1
cost: E
---

## What

The pinch (`union_with(q1, q3)` over z ∈ (0.5, 1.5), records carried)
against the prism along the diagonal x = y in
`two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`). The prism's
face in that plane holds the whole pinch line, so the pinch's top end
(0,0,1.5) lies inside it. `pinch ∖ prism` builds at its volume and
fails 3′: `UndeclaredContact` `VertexVertex` at (0,0,1.5), plus the
`EdgeEdgeOverlap` at (0,0,1). The other five ops pass. Pinned there as
it stands, at both notches the test runs.

## Cause (measured)

The carried top-end row (A 5v1, 15v1) is dropped by
`ops::remap_carried` (`crates/topo/src/boolean/ops.rs`):
`Descendants::live_vertex` resolves neither end. The result's
vertices at (0,0,1.5), 22v1 and 25v1, are the live members of
`desc.copies_of(A, 5v1)` and `desc.copies_of(A, 15v1)`: the null-edge
copies the face's section left. `remap_contacts` takes copies in for
the reduction's own v-v groups (it is how the lower end's row reaches
36v1 here), but `remap_carried` reads each carried end alone. The
result's `a_on_b`/`b_on_a` are empty, so this is not
`a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records`
(a pierce's vertex-on-face rows).

## Owed

Have `remap_carried` record a carried row between the live copies of
its two ends, as `remap_contacts` does for its groups, then flip the
pin to 3′ passing in all six ops.
