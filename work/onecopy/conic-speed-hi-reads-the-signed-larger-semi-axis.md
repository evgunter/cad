---
id: conic-speed-hi-reads-the-signed-larger-semi-axis
kind: issue
title: Conic::speed_hi and speed_lo read the signed semi-axes, so a circle or an unchecked ellipse stored with a negative one under-reads its speed
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Filed by PCERT (`pcert/ellipse-general-lane`, the ellipse's general
lane).

## What

`geom::Conic::speed_hi` documents itself as "the greatest speed `|P′|`
over the carrier, and a bound on `|P″|`: the larger semi-axis", and
`speed_lo` as the least speed. Both read the stored semi-axes SIGNED
(`crates/geom/src/curves/conic.rs`): a `Circle`'s are its `radius`, and
an `Ellipse`'s `semi_axes_by_size()` is `max`/`min` of `semi_u`,
`semi_v`. Neither `Curve3::circle` nor `Ellipse::new` refuses a negative
value, and the repo already treats a negatively stored semi-axis as a
real input at rest (`work/restfront/an-ellipse-stored-minor-over-major-passes-tier-3.md`,
closed: "the readers that took a stored semi-axis as ordered or signed
now read" magnitudes). So an ellipse stored `(−10, 1e-3)` reads
`speed_hi = 1e-3`, and a circle of radius `−r` reads `−r`.

`git grep -n '\.speed_hi()' crates` lists 59 production readers. Each
that charges rounding or bounds a derivative at that reach under-reads
such a carrier. In `crates/geom-brep/src/implicit.rs`:
`sample_rounding` (`let a = conic.speed_hi()`), `torus_curvature_bound`
(`a` bounds `|C′|`, `|C″|`), and `conic_residual_harmonics`' `noise`
(`rounding_charge(d.norm() + conic.speed_hi())`).

The projected lane's charge no longer reads it: `implicit::semi_reach`
reads `max(|a|, |b|)` for `quadric_harmonics`' `terms`,
`conic_cone_harmonics`' reach and `conic_axis_distance_floor`, and the
`semi-reach-signed` mutant there is red on
`ellipse_lane_rows::an_ellipses_coefficients_stay_within_their_charge_on_every_chart`
(the row "a negative semi-axis a hundred times the chart").

## Fix

Read magnitudes in `speed_hi`, `speed_lo` and `speed_at` (bit-identical
for every carrier a door builds, whose semi-axes are positive), then
retire `semi_reach` into it.
