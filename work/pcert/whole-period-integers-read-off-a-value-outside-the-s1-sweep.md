---
id: whole-period-integers-read-off-a-value-outside-the-s1-sweep
kind: issue
title: Whole-period integers read off a value by round, floor or periodic_branch, outside B2b's S1 sweep: three C4 violations, the rest renames no reader tells apart
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
- `crates/geom-brep/src/props/closure.rs:175`, `interval_loop_turns`.
  Each junction's turns are `(mid(δ)/τ).round()` at `f64`, through no
  band, then the junction's gap is decided with that integer. On a
  half-period mark the rounding picks a side silently where
  `Period::decide` escalates. Verdict-safe **only while the junction's
  lever is bounded away from zero**: a wrong integer leaves a gap of at
  least `π·lever`, which the junction refuses against its slack, so the
  cost is a refusal named as an open junction instead of an escalation.
  Today's levers are: `props::quad::loop_windings`, the cylinder's radius
  (and `1` on the free `v`); `props::spiric::wall_windings`, the torus's
  `R + r·cos v`, at least `R − r > 0` on a ring torus, and `r > 0`. A
  lever near zero (a pole, an apex) would leave the wrong integer's gap
  inside the slack, and then the rounding would decide a verdict. Its
  integer is then accumulated as `m[c] as i64` (`:191`), an `as` cast of
  a rounded period quotient: a shape the sweeps' patterns cannot see.
  Fix: decide the turns with `Period::decide` at the junction's lever.
- `crates/topo/src/shell.rs:3657`, the rim pairing's run mean:
  `tau * ((at - mean)/tau + 0.5).floor()`. The nesting decision reads
  the shifted run. Its doc says no tie arises for the concentric pairs
  `encloses` exists for, but the same code is asked of pairs not known
  to nest (both orders before a band seam, and `pair_rings`' two orders
  before it refuses), where two full-period runs' means can sit half a
  period apart. At that tie the floor lands the later run a whole period
  across the flat picture, moving its centroid, and `shell_rim_nesting`
  (`:3581`) decides on radii read from that placement, numerically: the
  site's own comment (`:3644`–`:3656`) says so, and that no reader takes
  it as an identity. Both hold: the tie moves what the numeric nesting
  decision reads, which is the C4 concern; no identity is read. Fix:
  decide the shift by the marks, escalating at the tie.

(`step-import`'s outerness `unwrap` is the same class, filed as
`work/foreign/step-import-unwrap-reads-a-whole-period-off-a-value.md`.)

## Fixed on main since filing

- `crates/geom-brep/src/props/curved.rs`, a torus rim's `v` lift, read
  by the Green form: it was `periodic_branch` (safe by slack, since every
  junction is decided closed first), and main now decides it by the
  marks, `torus_lift` (`:4518`, `props_torus_lift_turns`,
  `Period::decide` at the minor radius).

## Renames no reader tells apart

- `crates/topo/src/chord_join.rs:5337`: a vertex's azimuth lifted into
  the run's window by `periodic_branch` for a crossing parity. A vertex
  on the seam of a whole-period window lands on `hi`, and such a run
  carries its seam rows along both edges, so either side counts the same
  crossings. At `Sym` the crossings read numerically.
- `crates/topo/src/ring_path.rs:505`, `branch`: a parameter placed in
  `(mid − π, mid + π]`. Its doc states no reader reads the pick as an
  identity; it is compared only with span ends of another provenance.
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

## Outside the kernel's certification, noted

- `crates/geom/src/periodic.rs:35`, `periodic_window_may_hold`: the
  representative `phi_lo + period·⌈(lo − phi_hi)/period⌉`, a ceiling of
  a quotient at `f64`. It answers "may hold", and every doubtful case
  (NaN, a span of a period or more) answers `true`, so a wrong integer
  only widens what is kept.

At `Interval`, an offset of exactly `π` (a half period) escalates
through the projected lane's `off_period` at its seam rather than
deciding a side (B2b's delta review, N1). That is the marks doing what
this item asks of the sites above.

## Re-sweep owed

An `as i32` / `as i64` cast applied to a period quotient
(`(x / τ).round() as i64`, or a rounded `m` cast later, as at
`closure.rs:191`) forms the integer out of sight of every pattern above.
Re-sweep for it before closing this item.

## What the patterns could not match

The sweep for this item matched `.round()` / `.floor()` on a period
quotient, `periodic_branch`, `reduce_periodic(_centred)`,
`rem_euclid(τ)`, and `±= TAU`. The first pass missed `shell.rs:3650`, whose quotient carries an
added constant (`(x/τ + 0.5).floor()`), and `periodic.rs:35`'s
`.ceil()`; the second pass matched both. It still cannot see an
integer formed by an `as` cast or `trunc()`, a comparison loop spelled without `TAU`
(`while u > PI`), or a period that is not `τ` (a spline's knot length)
reduced by hand. A second set it matched and did not classify, all
`f64` and outside certification: `rem_euclid` in `boolean::rim_wedge`
(also a hand `floor`), `boolean::circle_torus`, `conic_quadric`,
`ellipse_torus`, `section_cert_rows`, `curve_pair::meet_oracle`, and
`step-import`'s `entities.rs:2070` and `normalize.rs:688`.
