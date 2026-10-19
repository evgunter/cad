---
id: a-face-holds-a-point-has-no-public-door
kind: issue
title: topo's 'does this face hold this point' dispatch (plane -> contfp, curved -> curved_face_containment) is pub(crate) face_placement, so editor-core's emit_topo and topo's rim_wedge and census each keep a copy with its own error policy
status: open
opened: 2026-10-18
priority: P3
cost: M
---


## Finding

Filed by the WIRE orchestrator (2026-10-18) from PR 5263's delta review.
`topo::contain` `face_placement` is `pub(crate)`, and
`topo::test_support::face_holds` is test-support only, so a consumer
asking "is this point strictly inside this face" re-spells the dispatch:
PR 5263 adds `face_holds_inside` in
`crates/editor-core/src/names/emit_topo.rs` (disclosed at its site), beside
`topo::boolean::rim_wedge` (~563–581) and the census's `face_holds`. The
three map `ContainError` differently: one lifts escalations, one panics on
`StaleFace`, one maps errors to typed refusals. The fix is one public topo
door with one error policy that the three call. `crates/topo/src/contain.rs`
is in no open program's `paths`, so this sits here until a topo-side
program claims it.

Signed (WIRE orchestrator).

Update (WIRE lane `wire-seamparent`, PR 5313): `emit_topo`'s
`face_holds_inside` and `face_containment` are deleted along with
`constituent_holding`, their only caller, so two copies remain
(`rim_wedge` and the census's `face_holds`).
