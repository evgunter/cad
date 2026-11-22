---
id: the-planner-walks-a-closed-edge-pose-the-pierce-refuses
kind: issue
title: The vertex-vertex planner walks a closed edge's lone-vertex pose that the pierce refuses: two policies for one pose
status: open
opened: 2026-11-16
priority: P2
cost: M
---

Found by the review of PR 7008 (2026-11-16).

At a closed edge's lone vertex with a germ along it
(`StrutFacing::ClosedEdge`) the edge key names no half, and the two
callers do different things:

- The vertex–vertex planner (`crates/topo/src/boolean/insert.rs:2049`)
  gets `None` from `strut_faces_first` and walks the germs
  (`walk_faces_first`).
- The pierce (`crates/topo/src/boolean/vtxfac.rs:1392`) asks
  `strut_at_closed_edge` and refuses `CurvedBooleanUnsupported`.

That is two policies for one pose. Neither is checked: no row reaches the
pose (`a-strut-pose-row-with-an-oracle-independent-of-the-tip`).

**The reviewer's doubt, at a wrap tip.** `strut_facing` answers
`PlusFirst` at a wrap edge's tip whenever a germ runs along the wrap edge
(`insert.rs:2365`), on the argument that a germ along it is met first
beside the arrival and last beside the departure. But where the run holds
the tip's one real half, `run_site` gives the tip `TipSide::Other`
(`crates/topo/src/star.rs:403`), so the run's start germ may lie beside
the departure instead. Base (before PR 7008) refused there and head
builds. No row reaches the pose either way.

## Done when

The pose has one policy, read by both callers, and the wrap-tip facing is
checked against a row that reaches it.
