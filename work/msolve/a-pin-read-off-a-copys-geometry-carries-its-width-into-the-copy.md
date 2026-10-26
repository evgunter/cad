---
id: a-pin-read-off-a-copys-geometry-carries-its-width-into-the-copy
kind: issue
title: A pin read off a copy's geometry carries the geometry's interval width into the copy, so a Transform over it escalates at ε = 1e-12
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by PR 5267 (s3 C), after the named interim's world pin. The orchestrator asked for this class to be filed apart from the world pin's retirement (`a-copy-standing-where-its-body-stands-has-no-d10-spelling`).

**The class.** `pin_at` and `place_at` read a copy's pose off its own geometry (`crates/editor-core/src/mate/pin.rs`). In the interval lane the solved map then carries that geometry's enclosure, which is a few ε wide. A placement that moves the copy by such a map, or a `Transform` over the moved copy, re-certifies its edges against that width. At ε = 1e-12 the width reaches the band, and the run escalates. Before C, a world placement at a pose was literal numbers read off no geometry.

**Rows that still read a chain after the interim** (each holds the escalation at ε = 1e-12 alone, by `p2_gauges::encloses_or_escalates`, and encloses at every other ε):
- `p2_gauge_offsets_and_spaces::a_seat_under_a_parametric_world_pin_poses_as_composed`: the base instance is pinned by `pin_at` at a pose off the identity with a lift appended (`pin_lifted`). Depending on the lift value, the instance or the `Transform` over it escalates (`SurfaceResidual`, an enclosure ±1.08e-12).
- `p2_gauge_poses_and_doors::a_seat_under_a_two_parameter_world_pin_poses_as_composed`: the same, at the `Transform` over the seated top (`EndpointEnd`, [0, 1.01e-12]).
- `msolve14_run_scalar`'s `INTERVAL_ESCALATIONS`: `pinned-chain` at 1e-12 (`carrier_endpoint_start`).

**Measured, not re-baselined:** at the default ε and at 1e-6 every such row encloses.

**Done when** a pin at a pose reads no geometry it can avoid (D: frames enter only at placement, a root's start cap reads nothing), or the interval solve carries the shared enclosure through the map without widening; the rows above then enclose at 1e-12 and drop their escalation clause.
