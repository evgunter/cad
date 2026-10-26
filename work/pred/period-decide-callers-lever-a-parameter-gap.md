---
id: period-decide-callers-lever-a-parameter-gap
kind: issue
title: Four Period::decide callers cross a parameter gap to metres through Margin::levered, not the rate pair
status: closed
opened: 2026-10-25
priority: P3
cost: E
branch: pred/turn-rule-follow-ups
pr: 5724
closed: 2026-10-26
---


Found sweeping for unit 2 of `period-headroom-margin-has-no-shared-home`
(branch `pred/period-turn-sites`), in the pass aimed at headrooms
formed through `Period` rather than spelled with `tau`.

`geom_core::period::Period::decide` takes the caller's margin door as a
closure over the gap `g` (a parameter difference, in radians on every
caller below). Four callers hand it `|g| Margin::levered(g, lever)`:

- `topo/src/offset_axial.rs` `keep_turn` (`offset_axial_keep_turn`);
- `topo/src/chord_join.rs`, the sphere pole branch
  (`split_sphere_pole_branch`);
- `geom-brep/src/pcurve_cache.rs`, the chart placement
  (`pcurve_chart_placement`);
- `geom-brep/src/props/curved.rs` `torus_lift` (`props_torus_lift_turns`).

A parameter gap is not the dimensionless `x` `Margin::levered`'s doc
admits: it says such a span "crosses through the rate pair
(`Margin::metered`, `Margin::metered_sup`; `[tolerance.margins]` clause
(iii))". Each lever is a speed (a floor, a radius), so the product is
bit-identical through `Margin::metered(g, InfSpeed::new(lever))`, and
the fix moves no verdict: the work is to choose, per site, whether the
lever is an inf (a mark decided either way admits a turn, so the
"definitely apart" door) or a sup, and to spell it so. This is the
branch-count question, not the headroom-to-one-turn fact unit 2
consolidated, so it was left out of that unit.

## Closed (PRED orchestrator, 2026-10-26, PR 5724)

Done in PR 5724, after a full single review (APPROVE-WITH-FIXES, nothing falsified on soundness) and its fix pass. The remaining cases are filed as `f64-turn-rule-measures-headroom-against-f64-tau`, `spiric-turn-defect-inf-stays-global` and `parameter-span-levered-sites-outside-period-decide`.
