---
id: props-torus-lift-and-gauss-bonnet-fan-pick-a-branch-by-floor
kind: issue
title: the props torus lift and the Gauss–Bonnet fan pick a whole-period branch by floor
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [periodic-branch-floor-remains-at-the-raw-picks, periodic-branch-floor-remains-at-three-sites]
---


Found by PCERT's `periodic-branch-floor-remains-at-the-raw-picks`, by
measurement. Filed here because both sites are FLUX's ground; PCERT
proposes, and changed neither.

Two sites in `crates/geom-brep/src/props/curved.rs` pick a whole-period
branch by `floor`. At `Sym` a `floor` is an opaque atom (it folds only
on a zero-form argument), so whatever reads the branch reads it
numerically.

Measured with a scratch probe over the topo, sweep, step-import, mesh,
editor-core, geom-brep and profile suites at default ε (2026-10-13).

## The torus lift: `turns = (at − angle).periodic_branch(τ)`

- **Reached:** f64 (153k calls), `Interval` (132k, point width ≤ 3.3e-12),
  `Dual`, `Sym<f64>` (24). Never over a parameter box.
- **No ties:** every gap is a whole number of turns, because
  `props_torus_lift_joint` decides it just before (a `require_zero` on
  the turn from the running level, levered at `minor`).

**Proposal: decide it as structure.** `geom_brep::whole_period_count`,
metered `Margin::levered(g, minor)`, as PCERT's three sites do. The
marks sit half a turn from a gap the joint has just decided is a whole
number of turns, so they always decide. `turns` is then a literal at
every scalar, and the rim levels read it as the constant it is. Rows at
`k = −1, 0, 1, 2` over a box, and a floor mutant.

## The Gauss–Bonnet fan: `turns = ⌊(gauss − fan)/4π + ½⌋`

- **Reached:**
  - f64: 583k calls, 142 within 1e-9 of a half-period tie (the
    `mesh12_saturated_span` and `mesh12_parse_vs_certification` rows);
  - `Interval`: 612k calls, **wide** in 442 tests, up to 101 sr. That is
    an enclosure property of the solid-angle sum, not a parameter box;
  - `Dual`, and `Sym<f64>` (17.5k).
- **What reads it:** only the numeric min/max blend that follows
  (`FAN_AGREEMENT_SR` / `FAN_REGION_SR`). No decision and no identity
  reads `turns`.

**Proposal: keep it, with the argument at the site.**
- Where `gauss − fan` straddles a half-period, the floor's enclosure spans
  both branches and the blend encloses the true value. Any decision
  there would fail, since the enclosure is wider than a period.
- At an f64 tie either branch blends to within the fan's agreement band.
- At `Sym` the blend is numeric whatever `turns` is.
