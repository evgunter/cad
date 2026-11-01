---
id: naming-edge-curve-ladders-can-read-through-body-stored-curve
kind: issue
title: emit_topo's edge-to-certified-curve ladders could read through topo::Body::stored_curve
status: closed
opened: 2026-10-20
closed: 2026-10-20
---


(Filed by the ENCL lane that added `topo::Body::stored_curve`, from the review of its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`. `crates/editor-core/src/names/emit_topo.rs` is shared by borders, refs and wire.)

## What

`topo::Body::stored_curve` (`crates/topo/src/body.rs`) reads a stored edge's certified carrier and interval:
- a stale key is `BadArgument::Stale`;
- a null edge is `NullScaffoldCurve`;
- a live edge's dangling curve link panics, because only a kernel bug reaches it.

`crates/editor-core/src/names/emit_topo.rs` spells the same ladder three times:
- `certified_curve` (~4054): every miss is one `NamingError::Emission { what: "a curved edge has no certified curve" }`.
- `carrier_arc` (~4442): the same chain inline, with the same text.
- `crossed_curve` (~4625): each miss gets its own `Emission` text, and a dangling curve link is "a crossed edge's carrier is dangling".

Routing each site is `stored_curve(e)` with a `map_err`. A dangling curve link would then panic instead of coming back as an `Emission` error. That trade, and whether the three collapse into one helper, is this program's call.

Closed by PR 5503 (2026-10-20): `emit_topo`'s five edge-to-curve reads are one `#[track_caller] certified_curve` over `Body::stored_curve`; the FULL review found the behaviour identical.
