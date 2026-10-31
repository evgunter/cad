---
id: mesh-edge-curve-ladders-can-read-through-body-stored-curve
kind: issue
title: mesh's edge-to-certified-curve ladders could read through topo::Body::stored_curve
status: open
opened: 2026-10-20
---


(Filed by the ENCL lane that added `topo::Body::stored_curve`, from its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`topo::Body::stored_curve` (`crates/topo/src/body.rs`) reads a stored edge's certified carrier and interval: a stale key is `BadArgument::Stale`, a null edge `NullScaffoldCurve`, and a live edge's dangling curve link panics as a kernel bug. mesh spells the same ladder itself:

- `walk.rs` (~225) and `memo.rs` (~773): `get_edge` → `MissingEntity { what: "edge" }`, `get_curve_geom` → `MissingEntity { what: "edge curve" }`, `.certified()` → `NullScaffoldEdge`.
- `trimmed.rs` (~160, ~947): one `MissingEntity { what: "edge curve" }` for every miss, a null edge included.
- `chords.rs` (~101-106): the same `MissingEntity { what: "edge curve" }` / `NullScaffoldEdge` pair, over `body.edges()`.

Routing them is a `map_err` from `EulerOpError::NullScaffoldCurve` to `NullScaffoldEdge` and from `BadArgument::Stale` to `MissingEntity`. It changes one thing: a dangling curve link would panic rather than come back as `MissingEntity { what: "edge curve" }`, and `trimmed.rs` would tell a null edge apart. Those are this program's calls.

## Moved to TESSWORD (tessellation refusal words, mesh docs and contract hygiene) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.
