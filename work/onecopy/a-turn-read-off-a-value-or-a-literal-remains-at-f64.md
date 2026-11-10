---
id: a-turn-read-off-a-value-or-a-literal-remains-at-f64
kind: issue
title: A whole turn is still spelled at f64 where a carrier's or a chart's period is owed, and three sites read a whole-turn integer off a value
status: open
opened: 2026-11-10
priority: P3
---

Filed from PR 6788's review, finding 5. These are the sites the `period-spelling` gate (`scripts/gates/period-spelling.sh`, table `period-spelling.tsv`) holds at class `owed`, plus the `T::tau()` sites and integer reads that the gate does not count.

## Owed: a turn that should read its carrier's or chart's period

These are `period-spelling.tsv`'s `owed` rows:
- geom-brep:
  - `pcurve.rs`'s ellipse pcurves (`span > TAU`), where the ellipse's period is owed;
  - `pcurve_cache.rs` `closed_form_grid`, `cover` and `dyadic` (`TAU / 2^d`), where the carrier's period is owed;
  - `pair_section/ruling.rs` `new`, `run`, `coverage` and its `FLOOR` const;
- step-import: `adopt.rs` `arc_rim_on_wall_boundary` and `entities.rs` `is_periodic_band`, both chart periods;
- mesh: `cert.rs` `abs_radial_max` and `trimmed.rs` `choose_pole_azimuths` and `harmonic_range`, the torus chart's `v` and the azimuth;
- sweep: `blend/surgery.rs` `past_misses`;
- editor-core: `names/emit_topo.rs` `on`, a `T::from_f64(TAU)` returned as a period;
- topo: `injective.rs`'s `TURN` const.

## A whole-turn integer read off a value (C4, finding F4)

C4 (`geom_core::period::Period`'s docs) says no whole-period integer is read off a value. These sites still do:
- `geom-brep/src/pair_section/winds.rs:317`, `winding`: `(total / TAU).round()`;
- `geom-brep/src/props/closure.rs:175`, `interval_loop_turns`: `(mid(delta) / TAU).round()`.

The period's doors are `Period::decide`, through its marks, and `Period::near`. Both sites move bits at their rounding edge if swapped, so each wants a measured change.

## `T::tau()` sites the gate does not count

`T::tau()` is the scalar's own τ, which `Period::angle()` reads, so the gate does not count it. These sites read it where a carrier or a chart is at hand:
- `geom/src/carrier_speed.rs:78`, `turn_defect`: `T::tau() − extent`. The periodic carrier's period is owed; `turn_defect_rate`'s caller holds the carrier.
- `geom-brep/src/certify.rs`'s other `T::tau()` reads.

## What was folded in PR 6788

- `implicit.rs:912`: a conic's arc range reads `geom::conic_period`.
- `implicit.rs:963`: a circle's curvature bound reads `circle.period()`.
- `topo/src/separating.rs:94`: a circle's cover window reads a circle's period.
- `certify.rs`'s `self_loop_circle_at`: its parameter end reads its circle's period, and its revolution `Period::angle()`.
- `slit.rs:1650`, `boolean/mod.rs:8111`, `pair_section.rs:1808`, `cover/readings.rs:96-98`, `half_angle.rs:822-823`, and chart_iso's `TAU` (retired).
