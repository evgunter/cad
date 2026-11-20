---
id: the-half-edge-of-a-faces-loop-leaving-a-vertex-is-hand-walked-in-nine-places
kind: issue
title: the half-edge of a face's loop leaving a vertex is hand-walked in nine places
status: open
opened: 2026-11-20
priority: P4
cost: E
---


## What

The question "which half-edge of `face`'s loops leaves vertex `v`" is
answered by a hand-written walk (the face's outer loop's `Cycle { first }`,
`loop_cycle`, `find` on `start == v`) in these places:

- `crates/topo/src/test_support_fixtures.rs`: two `leaving` closures
  (`kill_under_a_null_strut` and the fixture before it).
- `crates/topo/src/row_walk_proofs.rs`: one `leaving` closure.
- `crates/topo/src/null.rs` tests: the `leaving` fn and three closures.
- `crates/topo/src/merge_faces.rs` tests: one `leaving` fn.
- `crates/topo/src/shell.rs`: two `leaving` closures.
- `crates/topo/src/boolean/ops.rs` tests: one `leaving` closure.

The kernel already reads the same thing through the face view:
`crates/topo/src/boolean/slit.rs`'s `loop_leaving` filters
`proven_face(face).edges()` on `ends[0] == v`. `sphere_region.rs`'s
tests `leaving` (`charttail/sphere-scaffolding`, PR 7169) uses that
spelling.

## Owed

One home: a `FaceView` method (for example `leaving(v) -> Vec<HalfEdgeKey>`,
one per pass of a loop through `v`), with `slit.rs`'s `loop_leaving` and
each site above reading it. Each site that walks only the outer loop
states whether a ring passing through `v` could change its answer. A
site that cannot be moved says why.

Filed from the review of PR 7169.
