---
id: declared-opposite-orientation-refusal-is-unreached-by-any-row
kind: issue
title: merge_faces's DeclaredOppositeOrientation refusal is reached by no row: dropping the sense from the declared-pair rung's normals survives the suites
status: open
opened: 2026-09-15
---


## Finding

`crates/topo/src/merge_faces.rs`, `planes_declared_equal`'s declared-pair
rung: the two `PlaneDesc` normals are the faces' OUTWARD normals (the
sense folded in through `face_normal::plane_outward_normal`), and the
`oriented_plane_eq` verdict `SameOpposite` is what fires
`MergeCoplanarError::DeclaredOppositeOrientation` — "an opposite-sense
pair on one plane lands there by construction" (the comment at the
rung). No row in the tree names that variant
(`grep -rn DeclaredOppositeOrientation crates` finds only the
definition, the rung and its docs), and a mutant that makes the
planar door ignore the bit (`plane_outward_normal` folding `true` for
every face) survives `cargo nextest run -p topo -p sweep -p mesh -p
editor-core` at this rung: the six rows it does red are check 6's
(`tier_three_refuses_a_hand_flipped_face_sense` ×2,
`tier_three_refusal_is_surgical`), check 9's
(`check_9_refuses_a_ring_that_lies_outside_its_outer_loop`) and
`merged_outline_ring`'s (`verbs_1031b_arcwind` ×2). So a declared
coplanar pair whose two faces carry opposite senses is not exercised
by any row: the refusal is documented, typed and unreached.

What is owed is one row that declares such a pair (two coplanar faces
of one body, one of them reversed through `Body::set_face_sense`, the
pair declared through the merge door's `DeclaredCtx`) and asserts the
variant — the same shape as PR 2649's
`the_rim_routing_reads_the_second_faces_sense_and_not_the_firsts`. It
would also be the row that reds when the rung's normals lose the
sense.

Found by SENSE-FOLD (SCALAR, 2026-09-15), from the anti-vacuity mutant
over the folded planar door; `merge_faces.rs` is TOPO's by
`work/topo/program.md`.
