---
id: probe-rows-refuse-nurbs-lane-not-supplied-at-the-lane-free-door
kind: issue
title: Seven probe-feature rows (m8_f67_r1_probes, span_meter_dim_twins) refuse NurbsLaneNotSupplied at the lane-free door, red on main and in the nightly's dev-probe sweep
status: open
opened: 2026-10-12
priority: P2
cost: E
refs: [lane-free-doors-skip-the-analytic-rung3-limbs]
---


Found by `pcert/spiric-reimport-tube` (PR 4846), whose `--all-features`
run listed exactly the same failures on its head and on main
`a1a1bcec43` (`cargo nextest run --profile ci -p geom-brep -p topo -p
sweep -p step-import --all-features`, failure lists diffed: identical).

## What

Seven `geom-brep` rows behind `#![cfg(feature = "probe")]` fail. Per-PR
CI never builds that feature. The nightly's `dev-probe` row runs both
modules: `scripts/gates/probe-suite-census.sh --executed` lists them
`plain`, and `scripts/k_probe_sweep.sh`'s `run_plain` runs them.

- `m8_f67_r1_probes::certify_poison_meter_escalates_invalid_at_the_meter`
  (`m8_f67_r1_probes.rs:219`) and
  `certify_backwards_span_stays_interval_not_forward` (`:231`): each
  expects its meter's or span's refusal and gets
  `Err(NurbsLaneNotSupplied)`.
- `span_meter_dim_twins::` (5 rows, `span_meter_dim_twins.rs:120`,
  `span_meter_sample`): "the span meter is gated exactly once per
  certification", 0 hits. The door refuses before the meter runs.

All of them certify an `Intersection` over a `Curve3::Nurbs` carrier
between two analytic surfaces through `EdgeCurve::certify`, the
lane-free door. Since `lane-free-doors-skip-the-analytic-rung3-limbs`
(closed 2026-10-12), that door refuses `NurbsLaneNotSupplied` for this
class before any meter or span decision. These rows were written
against the old admit-on-samples behaviour.

## Fix

Certify through `EdgeCurve::certify_via` with the scalar's `NurbsLane`,
where the scalar has one, so the meter and span decisions are reached.
Where it has none (`Probe` may not), assert the lane refusal and move
the meter's pin to a scalar that holds a lane. Check
`probe-suite-census.sh`'s floors after.
