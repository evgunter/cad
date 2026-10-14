---
id: join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge
kind: issue
title: the join's germ frame and ring run and the continuation scan answer a spline or spiric edge as a kernel invariant that blames the operand gate
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [delete-the-boolean-operand-edge-gate]
---


Filed by `planar-crossing-lane-reads-a-curved-carrier-as-a-line` (PR
3984; both dual reviews, r1 MINOR-3 and r2 MINOR-1).

## Finding

Four sites behind the boolean sweep have no row for a spiric or spline
(NURBS) operand edge and answer one as a kernel invariant whose text
blames the operand gate for letting it through:

- `crates/topo/src/boolean/join.rs` `germ_section_frame` (on-edge arm,
  :1036): `JoinDesync("an OnEdge germ's edge is neither a line nor a
  conic (the operand gates refuse the kinds)")`;
- `crates/topo/src/boolean/join.rs` `ring_run_ccw` (:1902):
  `SectionInvariant("the ring lane reached a spiric or spline run edge
  (the operand gates refuse the kinds)")`;
- `crates/topo/src/chord_join.rs` the run-edge reading (:2430):
  `SectionInvariant("a join lane reached a spiric or spline run edge
  (the operand gates refuse the kinds)")`;
- `crates/topo/src/boolean/reduce.rs` `refuse_undeclared_continuations`
  (the `PairUnread::Extent` arm): `ClassificationInvariant` for a face
  a spline edge bounds, which has no box.

None reads a curve as a line: all refuse. But none is typed as a
carrier refusal or names the edge, so the day the gate narrows each
would mislabel an input the kernel cannot read as a broken kernel.
Reviewer r1's P5 shows a spiric whose box clears the other operand
passes both sweep directions silently, so a far spiric edge on a cut
face's run reaches the ring lane; the germ frame is unreachable (a
coincident edge's box always overlaps, so the sweep refuses first).

Measured on PR 3984's backed-out attempt (c4f3840bd): with the gate
deleted, the M7-8 cube's disjoint union reaches the continuation
scan's `ClassificationInvariant`; the germ frame and ring run answer as
above when driven directly.

## Fix

Type each at its own site, naming the edge and the face (the attempt's
`EdgeCarrierSite::{GermFrame, RingRun, FaceExtent}`), each with a row
that reds without it. Part of `delete-the-boolean-operand-edge-gate`.
