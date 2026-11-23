---
id: step-export-certified-carrier-can-read-through-body-stored-curve
kind: issue
title: step-export's certified_carrier could read through topo::Body::stored_curve
status: closed
opened: 2026-10-20
priority: P4
cost: E
closed: 2026-11-22
parent: edge-carrier-walks-and-tag-matches-outside-the-one-door
---


(Filed by the ENCL lane that added `topo::Body::stored_curve`, from the review of its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`crates/step-export/src/writer.rs` `certified_carrier` (~58) is keyed by edge. It takes `(edge_key, &Edge)`, reads `get_curve_geom(edge.curve)` and matches on `CurveGeom`:
- a miss is `StepExportError::Corrupt { what: "edge curve key does not resolve" }`;
- a null edge is `NullScaffoldEdge { edge }`.

`topo::Body::stored_curve(edge_key)` is the same read. Under it, a null edge maps to `NullScaffoldEdge`, and a dangling curve link panics as a kernel bug instead of coming back as `Corrupt`. Whether the exporter takes that contract is this program's call. The curve-keyed read at ~499 is a different read and is out of scope here.

- 2026-11-02 — moved from REIMPORT to EXPORT by `git mv` when REIMPORT merged into EXPORT, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and REIMPORT had fallen far under it).

- 2026-11-22 (WALKS, `walks/edge-view`) — closed by the edge view: `writer.rs` resolves the edge once (`Body::edge`, its miss still `Corrupt`) and reads both the carrier and the interval through `certified_curve(EdgeView)`; a null edge is `NullScaffoldEdge`, a dangling curve link panics. The separate `edge_params` read (the "curve-keyed read" above, which was keyed by edge) is gone with it.
