---
id: clearance-edge-curve-ladder-can-read-through-body-stored-curve
kind: issue
title: clearance's edge-to-certified-curve ladder could read through topo::Body::stored_curve
status: open
opened: 2026-10-20
priority: P4
cost: E
---


(Filed by the ENCL lane that added `topo::Body::stored_curve`, from its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`crates/editor-core/src/clearance.rs` (~2697) reads an edge's carrier and interval as `body.get_edge(ek)?` → `get_curve_geom(e.curve)?` → `.certified()?`, each miss a `None`. `topo::Body::stored_curve(ek).ok()?` is the same read, except that a dangling curve link panics there as a kernel bug instead of reading `None`. Whether this site should take that contract is this program's call.
