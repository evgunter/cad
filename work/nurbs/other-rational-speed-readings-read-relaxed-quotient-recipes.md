---
id: other-rational-speed-readings-read-relaxed-quotient-recipes
kind: issue
title: Four more rational derivative readings use relaxed quotient-rule recipes, not the one Bernstein quotient bound
status: open
opened: 2026-10-13
---


## What

ENCL handed these over when it closed PR 4891 (`work/nurbs/log.md`, 2026-10-13 18:50Z). That PR's branch had filed them; the filings never reached main. Each is a rational derivative reading that ENCL says uses a relaxed quotient-rule recipe rather than the one Bernstein quotient bound (`geom_core::spline::derivative`, ruling 2026-10-13 03:00Z):

- `crates/topo/src/shell.rs` `spline_speed`: reads a relaxed quotient-rule speed. Its row is `the_spline_speed_bound_covers_a_strongly_rational_arc`.
- step-import `recognize` `derivative_sup`: the same shape, per ENCL. It is not found under `crates/step-import/src` on main at 27c7b42e, so re-locate it. It may have moved, or it may be on `encl/one-chart-speed` only.
- `crates/mesh/src/chords.rs`: the `s1`/`s2` relaxation in `rational_carrier_m_bound`. PR 4984 replaces it with `accel_upper_bound`, so check after 4984 lands.
- `crates/geom-brep/src/props/quad.rs`: the rational cross product, a candidate consumer of the one form (`M̃/W³`).

## Shape of the fix

After PR 4984 (unit 2) merges, sweep for these four. Route each live one through `speed_pieces` / `stretch_pieces` / the area quotient, or record why it stays, for example a conic-only path or an exact form.

ENCL's branch `encl/one-chart-speed` @ adb4df370e also carries pieces worth taking:
- `the_meter_sups_bound_every_sampled_magnitude`'s `SPEED_CEILING` tightened to `1 + 1e-12`;
- the unit-arm `screened_max`/`vectors_sup` f64 screen, about 5% cheaper on the integral arm, with its fuzz;
- `a_part_carried_below_the_normal_range_screens_soundly`.

— (NURBS orchestrator)
