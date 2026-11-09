---
id: link-near-clearance-divides-by-a-vanishing-bisector
kind: issue
title: link's Near clearance reads along a bisector that vanishes where the two ends are near-antipodal, a [simple.conditioned] defect to re-pose
status: open
opened: 2026-11-09
priority: P2
cost: M
---


Found by QML T2 (`onecopy/refusal-standing`), acting on PR 6637's third-delta note.

## What

`link::ArcReach::clears` (`crates/topo/src/link.rs`, the `Near` arm) reads whether the segment `q`–`w` clears the apex `p` as both ends' distances along `half = normalize(q̂ + ŵ)`, the bisector of their directions. Where `q̂` and `ŵ` are near-antipodal, `q̂ + ŵ` is a vanishing quantity, and normalizing it amplifies its rounding into the direction it reads along. The distances along that direction are then noise, and can be decided negative.

DESIGN `[simple.conditioned]` says such a formula is a defect, to be re-posed, never covered by K. It is not a refusal to name (orchestrator, 2026-11-09).

## Witnesses

A `debug_assert!` that no clearance reads negative (QML T2, reverted) fired on seven tests:

- `topo::all a_doubled_line_does_not_revert::the_unequal_pinch_grafted_as_a_cavity_refuses_its_wedges_at_tier_3_prime`, at both ε;
- `topo link::cleared_rows::a_keel_off_the_arcs_is_read_at_the_faces_reach_not_the_cleared_radius`, at ε 1e-12;
- `topo::all a_decided_overlap_decides_its_pair::a_crossing_touch_outranks_an_escalation_read_before_it`, at ε 1e-12;
- `topo::all coincident_line_wedges::a_pinwheel_beside_an_escalation_refuses_nothing_undecidable`, at ε 1e-12;
- `topo::all a_keel_on_the_floor_subtracts_as_a_slit::a_fate_corner_inside_an_edge_of_its_faces_division_builds` (slow set), at both ε;
- `topo::all a_keel_on_the_floor_subtracts_as_a_slit::a_partner_flush_beside_the_keel_unions_with_the_slit_at_tier_three` (slow set), at both ε.

To reproduce, put `debug_assert!(sign != Sign::Negative, ..)` in `clears`' `ends` closure and run those rows with `--profile ci` (debug).

## Re-posing

One candidate: read each end's distance along the wedge's own in-plane normal, or read the side directly, rather than along the bisector. That direction does not vanish with the angle between the ends.

## Done when

- The clearance is re-posed so that no reading direction vanishes with the configuration.
- The witnesses above pass with the debug assertion that no clearance reads negative, kept as a pin.
- `clears`' doc (which states the reachable negative today) is rewritten to match.
- `Unread::NoReference`'s "every arc decidedly meets the apex" arm is re-examined.
