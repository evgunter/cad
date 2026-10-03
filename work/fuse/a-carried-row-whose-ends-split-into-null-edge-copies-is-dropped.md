---
id: a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped
kind: issue
title: A carried v-v row whose ends the boolean splits into null-edge copies is dropped: remap_carried does not reach copies
status: closed
opened: 2026-10-03
priority: P1
cost: E
closed: 2026-10-03
pr: 3955
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

## A second witness

`y ∖ cube` in `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`
(`lens_in_a_lens`, same test file), with `y` built both ways. `y` is
a block less a lens, with a smaller lens put back. They touch along a
pinch line in the cube's bottom face whose far end, (0.5, 0.15, 0),
lies inside that face. 3′ fails `UndeclaredContact` `VertexVertex`
there, plus `EdgeEdgeOverlap` along the line. The other five ops pass.
Pinned there as it stands.

Measured on PR 3950's first head, where two lenses tied at both rays
still built: `remap_carried` resolved neither end of the carried rows
at the two far ends. The undeclared pairs were exactly those ends'
copies in `desc.copies_of`.

## Owed

Have `remap_carried` record a carried row between the live copies of
its two ends, as `remap_contacts` does for its groups, then flip the
pins to 3′ passing in all six ops.

## Closed (FUSE, PR 3955, 2026-10-03)

`remap_contacts` and `remap_carried` became one substitution door
(`carry` in `crates/topo/src/boolean/ops.rs`): carried and discovered
v-v rows are one group per shared vertex end, and every end reaches its
null-edge copies. Both pins in
`crates/topo/tests/union_flush_onto_edge_contact.rs` pass 3′ in all six
ops.
