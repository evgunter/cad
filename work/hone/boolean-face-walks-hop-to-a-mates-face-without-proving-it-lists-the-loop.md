---
id: boolean-face-walks-hop-to-a-mates-face-without-proving-it-lists-the-loop
kind: issue
title: the REST lane's patch flood and BFS, and surface_group's closure test, read a mate loop's face without proving the face lists the loop
status: open
opened: 2026-10-01
priority: P3
---


## What

Found by the receipt (§5) of PR 3669, which closed the same hop in
`Body::movefac` (`crates/topo/src/movefac.rs`, the labelling).

Three face walks in the boolean lane step from a member half-edge to
its mate through `Body::mate`, read the mate's `parent_loop`, and take
that loop's `face` as the face across the edge. None proves that the
face lists the loop (its `outer` or one of its `rings`), nor that the
mate's own `edge` is the member's (`Body::proven_mate`'s
`NotSameEdge`):

- **`boolean::rest::patch_faces`** (`crates/topo/src/boolean/rest.rs`):
  the region flood (`assigned`, `region`, `queue`). A mate loop torn to
  name another face adds that face to the region, so the patch can
  join faces no loop of theirs connects.
- **`boolean::rest::bfs_order`** (same file): the glue order over the
  patch (`in_patch`, `visited`, `queue`). The same hop can order a face
  through an edge it does not border, or leave a patch face unvisited.
- **`boolean::surface_group::unmated_boundary`**
  (`crates/topo/src/boolean/surface_group.rs`), through
  `Body::face_of_half_edge`: a member's boundary half whose mate's loop
  names another member reads as mated within the group, so the group
  can read as closed when one of its edges borders a face outside it.

`boolean::ops::gate` runs tiers 1 and 2 on the finished result. The
REST walks read the lane's reduced operands (`red.a`, `red.b` in
`try_rest_union`); the sweep that found them did not trace every
door's entry for a tier-1 check of the operands before them, so how a
torn record reaches them is not established here. The D2 addendum's
row 1 asks the proof of the plan's own reads either way.

## Measured

Not probed. The shape is the one PR 3669 witnessed in `movefac` on
`fixtures::detached_digons(1)`: the digon's second face's outer loop
torn to name the seed face, and the shell torn to stop listing the
second face. At that PR's base `movefac` returned `Ok` with face
counts `[2, 1]`, joining the seed face's component to the digon's
through a loop the seed face does not list
(`movefac::tests::movefac_refuses_a_mates_loop_its_face_does_not_list`).

## Fix shape

Hop through `Body::proven_mate` and prove that the face the mate's loop
names lists the loop, answering the lane's corrupt-input refusal
(`desync` in `rest`, the `Err(face)` arm `unmated_boundary` already
returns) at the hop.
