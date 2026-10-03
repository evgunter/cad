---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: SharedVertexCrossings' tie, interleave and dangling-edge arms are reached by no witness
status: open
opened: 2026-10-03
priority: P2
cost: M
---


## What

After `fuse/shared-vertex-crossings`, `BooleanError::SharedVertexCrossings`
is raised by `insert::reconcile_shared` (`crates/topo/src/boolean/insert.rs`)
in four arms. Only one has a witness: two crossing pairs that share both
their vertices (`two-pinches-crossing-on-one-line-refuse-their-union`).
These three are reached by none:

- a null edge both of whose runs hold another pair's cut, so the
  pairs' cuts interleave round the shared vertex;
- a cut the corner cannot order against a germ (`bool_shared_cut_order`
  zero or in band), so two pairs' cuts tie in one corner;
- another pair's cut between a dangling null edge's two germs.

The pinch witnesses all put the pinch line along an edge of the shared
vertex, where `recl::Reversed` keeps the cuts apart. The case the arms
were written against is a pinch line through the interior of a face
corner of the shared vertex: both pairs' germs then lie along one
direction in one corner, a tie. No witness was built. It needs a
cutter whose vertex has a face containing the pinch line inside its
corner, such as a pyramid with its apex on the line, and none of the
prism fixtures gives one.

## Owed

Build that witness. If it ties, decide the order the pinch's pieces
give round the line and build the arm, as `recl::Reversed` does along
an edge. Probe the interleave and dangling-edge arms the same way, or
show they cannot be reached and make them invariants.
