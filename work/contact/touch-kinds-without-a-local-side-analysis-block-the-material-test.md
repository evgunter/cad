---
id: touch-kinds-without-a-local-side-analysis-block-the-material-test
kind: issue
title: VertexVertex, VertexOnEdge, EdgeEdgeOverlap and ConformalPatch touches between two solids block the census's material test because only the vertex-on-face and edge-in-face kinds have a local side analysis
status: closed
opened: 2026-09-16
refs: [2767, 750]
priority: P0
cost: H
parent: CONTACT-1
closed: 2026-09-26
---

Filed by the S-BOOL orchestrator at BOOL-4's merge (PR 2767) on
CURVED's slate (`crates/topo/src/census.rs`). BOOL-4's material test
admits a touch between the two solids only when a local side analysis
says it is a one-sided REST: for a vertex on a face, every incident
edge leaves the face's plane on one side; for an edge lying in a face,
both adjacent faces lie on one side. A transverse crossing whose
crossing points are lower-dimensional features degrades to a touch in
the exact sweeps' classification, which is why the analysis exists.
The other touch kinds — a vertex on a vertex, a vertex on an edge, two
overlapping edges, a conformal patch — need the dihedral wedge at the
feature to say whether the touch is one-sided, and BOOL-4 BLOCKS the
material test on them as a stated narrowing (the pair refuses typed
under the precondition wording). A part resting on a container's edge
or corner, or two parts meeting along an edge, therefore cannot
certify through the material test today. The fix is the wedge
analysis for each kind, one home, decided under the run band.
Measured, not acted on; difficulty M.

## Closed

By CONTACT-1 (PR 3253). One local material-cone analysis now decides
every planar touch kind, for undeclared findings and for declared vv
and vf records alike. The kinds are vertex–vertex, vertex on edge,
collinear edge overlap, vertex on face and edge in face.

What it still refuses has a row each on `work/contact/`:
- a saddle corner no plane separates;
- the direction-lever gap
  (`touch-cone-readings-are-levered-directions-not-face-distances`).

A conformal patch is always curved, so it stays `TouchUnreadable`.
