---
id: the-tube-reads-an-invalid-margin-on-a-steinmetz-branch-clear-of-its-crossing
kind: issue
title: The rung-3 tube's one-arc check reads an invalid margin on a Steinmetz branch clear of its crossing
status: open
opened: 2026-10-08
priority: P3
cost: M
---


Found by `pcert/projected-image` (PR 4304), scoping the analytic rung-3
edge certificate to the edge's interval.

## Measured

The two unit cylinders about `y` and about `x` meet in two ellipses in
the planes `x = ±y`, crossing at `(0, 0, ±1)`. A rational quadratic arc
along one ellipse, `(cos θ, cos θ, sin θ)` over a stretch of `θ` that
stays clear of `π/2` (`steinmetz_arc(a, b)` with `b < 0` in
`crates/geom-brep/tests/analytic_rung3_certificate.rs:185`, or the
stretches `[0, 0.3]`, `[0.7, 1]` of `steinmetz_arc(0.4, 0.3)`), lies on
both cylinders exactly and is transverse everywhere. Its limbs 1–2
pass; the tube (`ssi::certify::certify_branch` with `Limbs::Tube`,
extent the control-net diameter, `edge_nurbs::analytic_rung3`) refuses
on every one of five such arcs at `f64`:

```
TubeNotOneArc { rungs: 18..20, cause: Undecided(Indeterminate {
  margin: MarginDiag(Invalid), predicate: Some("ssi_tube_one_arc") }) }
```

An invalid margin is an enclosure that went NaN, not a measured
verdict. The row
`analytic_rung3_certificate::the_tube_reads_only_the_edges_own_interval_of_the_carrier`
pins only that such a stretch does not read the crossing
(`TubeStraddles`).

## Open

Where the NaN comes from (the one-arc slice argument on a pair whose
axes are perpendicular, or the ladder's narrowest rung), and whether a
transverse stretch of this pair should certify.
