---
id: rational-gates-test-unit-weights-not-constancy
kind: issue
title: The cap class's 'is this rational' gate tests weights == 1.0 where any constant weight vector is polynomial - over-strict by TRIM-1's own insight
status: open
opened: 2026-09-06
refs: [2095]
priority: P1
cost: E
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
