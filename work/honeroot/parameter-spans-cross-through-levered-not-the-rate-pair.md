---
id: parameter-spans-cross-through-levered-not-the-rate-pair
kind: issue
title: Angular parameter spans cross to metres through Margin::levered, where clause (iii) puts every parameter span on the rate pair
status: open
opened: 2026-10-24
priority: P3
cost: M
---


## The rule

`docs/DESIGN.md`, the margin dimensional convention (`[tolerance.margins]`)
clause (iii): "Parameter-space values cross to model space only through
per-kind metric doors." A chart's azimuth `u`, a polar `v` or a curve's
angular parameter is a parameter-space value whether or not it is in
radians. So a span or gap in such a parameter crosses through the rate
pair (`Margin::metered` with an `InfSpeed`, `Margin::metered_sup` with a
`SupSpeed`), and `Margin::levered` is for dimensionless quantities that
are not parameters (a cosine, a ratio, an angle between two directions
or frames). CHART's orchestrator ruled this on PR 5655, and that PR
writes the rule into both doors' docs (`geom_core::predicate`) and moves
`pcurves::decide_joint`'s two channels onto the pair. The sites below
still lever a parameter span.

## Sites

From `grep -rnE 'Margin::levered\([^,]*(\bt[01]\b|t_old|tau\(\)|theta|\bspan\b|window\.|gap\b|excess|along|\bdu\b|u1|u0)' crates/*/src`.
Each one needs reading before it moves: some first arguments named
`gap` or `along` are dot products, which are not spans.

- `topo/src/boolean/`:
  - `ops.rs`: `bool_sphere_cut_span` (`t1 - t0`, `theta - end`,
    `hi_t - theta`, `along`), and the `τ − window` headroom;
  - `reduce.rs`: `τ − (t1 − t0)` by `radius`;
  - `solid_contain.rs`: five `τ − width`/`τ − (az.1 − az.0)` headrooms;
  - `contain.rs`: `τ − (az.1 − az.0)`;
  - `sectors.rs`: `span` by `arm.min(arc.p_arm)`;
  - `section_cert.rs`: `APERTURE`'s `gap`, to check.
- `topo/src/splitting/containment.rs`: `rows.span`'s `gap` by `lever` and
  by `speed`.
- `topo/src/chart_region.rs` (CHART): the seam gate's `meter` closure,
  `|g| levered(g, radius)`; `span − τ` by `radius`; `excess` by `radius`.
- `topo/src/offset_axial.rs`: `read − t_old`, `kept − (t1 − t0)`,
  `span` by `floor`.
- `topo/src/chord_join.rs`: two `along`s, to check.
- `geom-brep/src/props/curved.rs` (FLUX): `τ − w.total`, `du − τ`,
  `τ − arc.span`, `τ − dt`, and `gap` by `minor`.
- `sweep/src/revolve/mod.rs` and `tube.rs` (CARVE): `τ − t` by `r_max`;
  `span` and `τ − span` by `arm`.
- `profile/src/seg.rs`: `arc.sweep / τ` by a diameter, to check (it is
  a ratio).

Most of these files are CLEAVE's and HONE's, so the row is filed here.
Rows that land on other ground name their owner above, for the lane
that takes this to announce.

## Shape

Each site mints the rate it means: the exact radius as either member
(it is exact), a window bound as the member whose direction is safe for
its predicate. It then crosses through that member's door. One
operation each, so no margin's bits move. A site whose first argument
turns out not to be a parameter span stays on `levered` and says why
in one line.
