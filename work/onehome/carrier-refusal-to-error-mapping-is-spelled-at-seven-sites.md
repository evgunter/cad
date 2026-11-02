---
id: carrier-refusal-to-error-mapping-is-spelled-at-seven-sites
kind: issue
title: The CarrierRefusal-to-error mapping (escalation vs a spline's walk) is hand-kept at about seven topo sites beside curve_pair::carrier_fault
status: open
opened: 2026-11-02
priority: P1
cost: E
refs: [6327]
---

Found in the review of ONEHOME's PR 6327, which added one more copy (`reduce::circle_refusal`).

`geom::CarrierRefusal` has four arms:
- `Escalated` and `Straddle` are the band's escalation;
- `Exhausted` and `Unbounded` mean a spline's piece walk could not run.

Every consumer has to sort them into its own error. That sort is spelled by hand at:
- `geom_brep::curve_pair::carrier_fault` (`curve_pair.rs`), the meeting reader's;
- `boolean/reduce.rs` `circle_refusal` (PR 6327);
- `boolean/slit.rs` `on_span`;
- `census.rs` `on_edge_interior_reading`;
- `census/curved.rs` `on_interior_at`, which splits `Exhausted` from `Unbounded`;
- `chord_join.rs` `carried_chord_spec`;
- `splitting/containment.rs` `ConicArc::hit` (`hit`).

The sites already disagree on what the walk arms are:
- `circle_refusal` and `carrier_cross` call them a kernel fault on a closed-form carrier;
- `slit` reads them as "outside";
- `census/curved` types each one separately.

Some of that may be legitimate per caller. Whether a refusal is an escalation or a walk that could not run is one fact about the refusal, though, so it belongs on the type as a method: `CarrierRefusal::escalation() -> Option<Indeterminate>` or a two-variant split. With that, no consumer re-spells the four-arm match.

Sweep the class first: `CarrierRefusal::(Exhausted|Unbounded)` across `crates/`. The row's list is from that grep at `b84804fa`.
