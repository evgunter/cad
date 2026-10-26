---
id: join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge
kind: issue
title: the join's germ frame and ring run and the continuation scan answer a spline or spiric edge as a kernel invariant that blames the operand gate
status: closed
opened: 2026-10-03
priority: P0
cost: M
refs: [delete-the-boolean-operand-edge-gate]
closed: 2026-10-25
branch: splice/gate
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

## Moved to SPLICE (2026-10-25)

From ROOTS. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P2 → P0: these are the sites `delete-the-boolean-operand-edge-gate` (P0) waits on, so the P0 cannot land before this row.

## Closed

Landed on `splice/gate` with the gate's deletion. One variant,
`BooleanError::EdgeCarrierUnsupported { operand, edge, face, site }`,
where `site` is an `EdgeCarrierSite`. It replaces both
`CurvedEdgeUnsupported` and the sweep's `CrossingCarrierUnsupported`,
and is raised:

- by the germ frame (`join::germ_section_loops`, `GermFrame`), for a
  spiric or a spline, naming the edge's split root and the germ's face
  on its own solid;
- by the ring run (`join::ring_run_ccw`, `RingRun`), for a spline run
  edge, before any closing reads it — planar, wall chart or quadric;
- by both crossing arms (`PlanarCrossing`, `CurvedCrossing`).

Every site names the operand's own edge (its split pieces' root) and
the face as the site holds it, and states that face's owner
(`face_of`: the other operand at a crossing arm, the edge's own at the
join).

The chord joiner's run-edge reading (`chord_join::between_edge_is_section`)
is shared with the split, so it refuses its own
`SplitJoinError::RunEdgeCarrierUnsupported { face, edge }`, which the
boolean nests as `BooleanError::Join`.

The continuation scan's face-extent site no longer exists: D10 stage 4
retired the scan. Its successor, `glue::glued_pairs`, leaves a pair
whose extent does not read to the sweep, which refuses the spline edge
first (a spline claims no box).

Witnesses, each red with its fix reverted (measured by mutation):

- `boolean::join::join_carrier_site_rows::a_germ_along_a_curved_edge_refuses_at_the_germ_frame`;
- `…::a_ring_run_along_a_spline_edge_refuses_at_the_ring_run`;
- `chord_join::run_edge_carrier_rows::a_curved_run_edge_refuses_naming_itself`;
- the crossing arms' existing `boolean::reduce::planar_lane_carrier_rows`, which now pin the site too.
