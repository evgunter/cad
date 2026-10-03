---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: SharedVertexCrossings' tie arm refuses a pinch line lying in a face of the shared corner, and its interleave and dangling-edge arms are reached by no witness
status: dispatched
opened: 2026-10-03
priority: P0
cost: M
branch: fuse/shared-vertex-tie
---


## What

After `fuse/shared-vertex-crossings`, `insert::reconcile_shared` and
`insert::strut_anchor` (`crates/topo/src/boolean/insert.rs`) raise
`BooleanError::SharedVertexCrossings` in five arms. Two have
witnesses:

- **Both vertices shared**: `two-pinches-crossing-on-one-line-refuse-their-union`.
- **Two pairs' cuts that tie in one corner.** Witness: the pinch
  (`union_with(q1, q3)`, along the z-axis) against the 80°–190° wedge
  over z ∈ (0.5, 1), sheared backwards along its 80° face by half a
  unit per unit of height: (x, y, z) → (x − 0.5·cos 80°·(z − 0.5),
  y − 0.5·sin 80°·(z − 0.5), z), through `prism_ops`' map. The pinch
  line runs flat inside that face from the shared corner, so both
  pairs' cuts lie along it in one corner (`bool_shared_cut_order`
  reads Zero), and all six ops refuse. Pinned by
  `a_pinch_line_flat_in_the_shared_corners_face_refuses_typed`
  (`crates/topo/tests/union_flush_onto_edge_contact.rs`). **P0** on the
  parent row's ruling (FUSE orchestrator, 2026-10-02): the pinch is a
  Boolean output, and the next Boolean refuses it.

Three are reached by none:

- a null edge both of whose runs hold another pair's cut (interleaved
  cuts);
- another pair's cut between a dangling null edge's two germs;
- two dangling null edges at one germ direction in one corner
  (`strut_anchor`; `reconcile_shared` reads such a pair as held first).

The witnesses so far either keep the cuts apart (the pinch line along
an edge, which `recl::Reversed` handles) or tie them. An interleave
needs pieces whose regions about the shared vertex alternate, which
disjoint pieces of one operand cannot do in a convex corner.

## Owed

For the tie: decide the order the pinch's pieces give about the line,
read off the pieces' own faces, and build the arm, as
`recl::Reversed` does along an edge. Flip the pinned test to a build in
all six ops at volumes checked outside the kernel, with 3′ passing.
For the three unreached arms: show they cannot be reached and make
them invariants, or build a witness.
