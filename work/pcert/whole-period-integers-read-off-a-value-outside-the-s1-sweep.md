---
id: whole-period-integers-read-off-a-value-outside-the-s1-sweep
kind: issue
title: Whole-period integers read off a value by round, floor or periodic_branch, outside B2b's S1 sweep: two C4 violations, the rest renames no reader tells apart
status: open
opened: 2026-10-19
priority: P3
cost: E
---

Found by both reviews of PCERT B2b (PR 5343, F4): B2b's S1 sweep
matched "k periods as a length" (`k * period`, `f64::from(k)`) and so
missed the reads that take a whole-period integer off a value by
rounding, flooring or `Real::periodic_branch`. C4 says no whole-period
integer is read off a value: the kernel decides one by the half-period
marks (`geom_brep::Period::decide`) and makes it a length only through
`Period::times`. Each site below is classified as a **C4 violation** (a
whole period of a chart quantity a certificate or a stored row reads,
taken by rounding through no band) or a **rename no reader tells apart**
(a branch picked inside a window where no stored row, verdict or
certificate depends on which whole period was taken). No code changes
with this item.

## C4 violations

- `crates/geom-brep/src/pcurve.rs:267`, `ellipse_pcurve_on_cylinder`.
  The fit's first sample keeps `atan2`'s principal branch, not the
  chart's principal window (`Placement`), and each later sample takes
  `((prev − u)/τ).round()` periods at `f64`. The image it fits is
  placed by `atan2`, not by C4. Its only callers are
  `tests/pcurve_conic.rs`; no mint reaches it today. Fix: place the
  first sample through `Placement::over`, and decide each later step by
  the marks, or retire the function.
- `crates/geom-brep/src/props/closure.rs:166`, `interval_loop_turns`.
  Each junction's turns are `(mid(δ)/τ).round()` at `f64`, through no
  band, then the junction's gap is decided with that integer. On a
  half-period mark the rounding picks a side silently where
  `Period::decide` escalates. Verdict-safe: a wrong integer leaves a
  gap of at least `π·lever`, which the junction refuses, so the cost is
  a refusal named as an open junction instead of an escalation. Fix:
  decide the turns with `Period::decide` at the junction's lever.

(`step-import`'s outerness `unwrap` is the same class, filed as
`work/exch/step-import-unwrap-reads-a-whole-period-off-a-value.md`.)

## Renames no reader tells apart

- `crates/topo/src/chord_join.rs:5337`: a vertex's azimuth lifted into
  the run's window by `periodic_branch` for a crossing parity. A vertex
  on the seam of a whole-period window lands on `hi`, and such a run
  carries its seam rows along both edges, so either side counts the same
  crossings. At `Sym` the crossings read numerically.
- `crates/topo/src/ring_path.rs:505`, `branch`: a parameter placed in
  `(mid − π, mid + π]`. Its doc states no reader reads the pick as an
  identity; it is compared only with span ends of another provenance.
- `crates/geom-brep/src/props/curved.rs:4318`: a torus rim's `v` follows
  the running `v` by `periodic_branch`. Every junction on the path is
  already decided closed within its slack, so the running value sits a
  slack from the rim's level, never near a half-period mark.
- `crates/topo/src/chart_iso.rs:70`, `unwrap_near`, and its readers
  (`mesh::walk`, `topo::coherence::wrapped`): tessellation's chart
  readings and a coherence measurement, at `f64`, certifying nothing.
- The `reduce_periodic` / `reduce_periodic_centred` readers reduce an
  angle difference into one period: arc extents (`profile::sugar`,
  `chord_join`'s arc ends, `shell`'s split, `boolean::boxes`,
  `circle_roots`), and relative angles in `splitting::classify`,
  `curve_pair` (`pose`, `span`, `pose/window`), `half_angle` and
  `projected`'s angle gap. None stores the integer. Their own question
  is the reduction's seam (an extent of `0` against `τ`), which is not
  C4's and is not judged here.

## What the patterns could not match

The sweep for this item matched `.round()` / `.floor()` on a period
quotient, `periodic_branch`, `reduce_periodic(_centred)`,
`rem_euclid(τ)`, and `±= TAU`. It cannot see an integer formed by an
`as` cast or `trunc()`, a comparison loop spelled without `TAU`
(`while u > PI`), or a period that is not `τ` (a spline's knot length)
reduced by hand. A second set it matched and did not classify, all
`f64` and outside certification: `rem_euclid` in `boolean::rim_wedge`
(also a hand `floor`), `boolean::circle_torus`, `conic_quadric`,
`ellipse_torus`, `section_cert_rows`, `curve_pair::meet_oracle`, and
`step-import`'s `entities.rs:2070` and `normalize.rs:688`.
