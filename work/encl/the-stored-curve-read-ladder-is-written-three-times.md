---
id: the-stored-curve-read-ladder-is-written-three-times
kind: issue
title: sweep and topo each read a stored edge's carrier and interval through their own get_edge/get_curve_geom/certified ladder
status: closed
closed: 2026-10-20
pr: 5467
opened: 2026-10-20
priority: P4
cost: E
branch: encl/stored-curve-read
---


(Filed by the ENCL orchestrator from the PR 5418 review.)

## What

One idiom, "look up the edge, get its curve, take `.certified()`, read the carrier and `params()`, `unreachable!` on a scaffold", is written in three homes:

- topo `Body::restatement` (`crates/topo/src/attach.rs`, ~1418), `pub(crate)`. The restatement door reads through it.
- sweep `stored_carrier` (`crates/sweep/src/extrude.rs`, ~1539-1556), new in PR 5418. It collapsed extrude's two copies and `describe_wrap_edge`'s.
- sweep `edge_data` (`crates/sweep/src/revolve/upgrade.rs`, ~36-60).

The copies carry the same `unreachable!` texts. Sweep the class with `blend/surgery.rs` (~2528) and `blend/reach.rs` (~1785).

Also: `describe_face_rim_at_rest` (`crates/sweep/src/swept.rs`, ~1443-1452) builds a full `restated_spec()`, cloning the carrier, only to keep `.description`. The door then re-reads the same curve.

## Repair shape

topo exposes one read of a stored edge's certified carrier and interval (`pub`, typed on a stale key). sweep and blend read through it, and the private copies go away.
