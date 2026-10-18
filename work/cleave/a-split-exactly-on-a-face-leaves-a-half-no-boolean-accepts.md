---
id: a-split-exactly-on-a-face-leaves-a-half-no-boolean-accepts
kind: issue
title: A split whose plane lies on an operand face or edge leaves coplanar section faces unmerged, so the half is refused as the input to every later boolean, with no editor merge door
status: open
opened: 2026-10-18
priority: P1
cost: M
---


Filed by the WIRE orchestrator from FORK-INPLANE's designers
(2026-10-18, measured on private copies of `origin/main`).

## What

`crates/editor-core/tests/split_pinch_keeps.rs`'s notched, pentagon and
revolved fixtures, split at `y = 1` (the plane on the notch tip / apex),
leave the lower half with coplanar section faces (and, notched, the
floor beside a section face) unmerged: `topo::splitting::finish`'s
module docs say coplanar artifacts are deliberately not auto-merged.
Cutting a small box hole from that half, well away from the plane,
refuses `Boolean(NonMaximalFaces)` naming the floor edge at `x = 7`,
the pentagon tip edge and the revolved apex circle. The upper halves
accept the same cut, and the same lower half from a plane at `y = 1.3`
is accepted by `Union` and `Subtract`. No editor door calls
`merge_coplanar_faces`, so a user has no recourse.

`Body::merge_coplanar_faces` on the lower halves: notched 9/21/14
faces/edges/vertices to 6/12/8, pentagon 7/15/10 to 6/12/8, both
passing tier 3 at rest, both the box the plane just below the tip
gives.

## What it needs

The split finishes each half with maximal faces (DESIGN's "Maximal
faces and maximal edges"), recording the merge's groups and joins for
naming. The naming side is WIRE's
`a-split-names-an-operand-edge-in-its-plane-as-a-section-chord`
(FORK-INPLANE), which recommends this; it is the split's instance of
`work/issues/tier-3-does-not-check-maximal-faces.md`. The unit that
builds it is likely one cross-territory WIRE unit (naming plus
`splitting/finish.rs`); CLEAVE's orchestrator should say if a live
CLEAVE lane is in that file.

Signed (WIRE orchestrator).
