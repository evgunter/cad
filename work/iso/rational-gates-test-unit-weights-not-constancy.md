---
id: rational-gates-test-unit-weights-not-constancy
kind: issue
title: The cap class's 'is this rational' gate tests weights == 1.0 where any constant weight vector is polynomial - over-strict by TRIM-1's own insight
status: closed
opened: 2026-09-06
refs: [2095]
priority: P1
cost: M
branch: iso/one-polynomial-weight-test
pr: 5635
closed: 2026-10-24
---


## What

TRIM-1 (PR #2095) established that a constant weight vector cancels out
of the rational basis, so a row wrapped with any constant is the same
curve as its polynomial wrap. The cap class's gate in `pcurve_cache.rs`
still tests `any(|w| *w != 1.0)` and refuses a constant-weight carrier
as "rational". Found by the dual (R2 S1); the same shape sits in
`props/loop_area.rs`, `props/quad.rs`, `patch_bound.rs`, `mesh/chords.rs`
and `mesh/nurbs_cert.rs` (other programs' fences — reported on the away
channel, not filed here); `step-export/writer.rs`'s `== 1.0` is a
file-format arm and correct as is.

## Fix

The gate asks "constant", with the constant folded out; one row on a
constant-`2.0` carrier that certifies. E.

## Home

TRIM — `pcurve_cache.rs`.

## More sites (NURBS sweep, 2026-10-12)

NURBS's rational-speed-meter PR fixed this shape in
`NurbsCurve3::speed_lower_bound` (`geom/src/curves/nurbs.rs`), which now
takes the integral arm on any equal weights. Its sweep of
`(!=|==) 1.0` and `to_bits() != 1.0f64.to_bits()` over `crates/*/src`
found the sites above plus these, each sound but refusing or looser on
a uniform-`w ≠ 1` net, none wrong:

- `geom-brep/src/offset_fit.rs` `NetForm::of` and `Composite::build`,
  through `patch_bound::is_rational`: the homogeneous form, only wider.
- `geom-brep/src/pcurve_cache.rs` arc-rim pattern: requires end weight
  `1.0`, so a scaled arc `c, c·w, c` is refused (compare `w₁/w₀`).
- `topo/src/offset_derive.rs` `polynomial_speed`: `None`, and
  `plane_spline_root` refuses "rational spline edge".
- `step-import/src/recognize_curve.rs` line promotion and
  `step-import/src/adopt.rs` `line_column_match`: bitwise refusals;
  `a_rational_straight_carrier_refuses_the_map_certificate` expects a
  `[2, 2, 2]` carrier to refuse.
- `sweep/src/skin.rs` the `integral` flag: a uniform-`w` input skins in
  the homogeneous lane, so its output weights need not come out
  bitwise constant.
- `mesh/src/nurbs_cert.rs`'s `rational_uniform_weight_bilinear_dust`
  pins a uniform-0.5 net on the rational arm; it moves when
  `patch_bound::is_rational` does.

Correct constancy tests to copy: `pcurve_cache.rs` `constant_weights`,
`edge_nurbs.rs`'s bitwise check, `mesh/src/chords.rs`
`general_uv_speeds`.

## `offset_derive.rs` answers rational carriers now (NURBS, 2026-10-12)

`nurbs/one-rational-speed-bound` routes `offset_derive.rs`'s spline
speed through `NurbsCurve3::speed_upper_bound`, which bounds any
positive-weight net (now the quotient bound, PR 4954), so `plane_spline_root` and
the level-row section no longer refuse a rational or constant-weight
carrier. That bullet above is closed by it.

## `skin.rs` closed (2026-10-23, PR 5629)

`skin_on` decides its lane per control row on constancy: a row whose
weight is the same `c` in every section skins in Cartesian ℝ³ with
every output weight exactly `c`, `1.0` included. The `sweep/src/skin.rs`
bullet above is closed by it; the other sites remain.

## Closed (2026-10-24, PR 5635)

"Is this NURBS polynomial" has one home: `geom_core::spline::constant_weight`,
which checks that every weight is bitwise equal to the first. `Weights::constant`
and the curve and surface `constant_weight()` methods delegate to it. Every site
listed above now routes through that home, and each site's polynomial arm is
pinned bitwise against the unit-weight answer at c ∈ {2, 3, 0.1}.

- **Arc-rim pattern:** it reads the ratio `w₁/w₀`.
- **`step-export`'s writer:** unit-only, by the file format, as noted at the site.
- **Creased nets:** a creased constant-c net now refuses as a unit one does.
  The integral arm's crease gap is recorded on
  `work/tess/loft-of-three-or-more-sections-at-degree-one-is-unmeshable.md`.

The review also surfaced a NaN composite in `try_line` that read as a zero
residual at c = 1e-300. That site now refuses. Its source is
`work/nurbs/implicit-composite-refuses-a-net-whose-weights-sit-near-the-bottom-of-f64.md`.
