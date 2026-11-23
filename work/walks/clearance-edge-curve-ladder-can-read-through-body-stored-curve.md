---
id: clearance-edge-curve-ladder-can-read-through-body-stored-curve
kind: issue
title: clearance's edge-to-certified-curve ladder could read through topo::Body::stored_curve
status: closed
opened: 2026-10-20
priority: P4
cost: E
closed: 2026-11-22
parent: edge-carrier-walks-and-tag-matches-outside-the-one-door
---


(Filed by the ENCL lane that added `topo::Body::stored_curve`, from its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`crates/editor-core/src/clearance.rs` (~2697) reads an edge's carrier and interval as `body.get_edge(ek)?` → `get_curve_geom(e.curve)?` → `.certified()?`, each miss a `None`. `topo::Body::stored_curve(ek).ok()?` is the same read, except that a dangling curve link panics there as a kernel bug instead of reading `None`. Whether this site should take that contract is this program's call.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.

- 2026-11-22 (WALKS, `walks/edge-view`) — closed by the edge view. The ladder this row names is no longer in `crates/editor-core/src/clearance.rs` at this base; the one production edge-to-curve ladder left in editor-core (`eval/measure.rs`'s edge carrier read, a stale key still `Carrier::Other`) and `coincide/replay.rs`'s now read through `topo::Body::edge` and `EdgeView::certified`, so a dangling curve link panics instead of reading `None`.
