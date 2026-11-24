//! **`radius_gap` is the difference `norm − radius` at every scalar's
//! own accuracy** ([`geom_core::radius_gap`]): at `f64` it is the root
//! form's reading, exactly zero where the radius is the measured norm
//! and never a rounding of the squares; at `Interval` it encloses the
//! difference, wider than the direct subtraction by no more than the
//! box's relative width on `norm + radius`.
#![allow(clippy::unwrap_used, clippy::float_cmp)]

use geom_core::{Bounds, Interval, Real, Vec2};

/// At `f64`, on offsets whose length is the radius to a few roundings,
/// from the unit scale to `1e9`: the gap is `norm − radius` to two
/// roundings of that difference, and exactly zero where they are equal.
///
/// Red with the gap spelled `(norm_sq − radius²)/(norm + radius)`: at a
/// radius of `4e6` a rounding of the squares over twice the radius is
/// `4e-10`, where the difference is exact.
#[test]
fn at_f64_the_gap_is_the_root_forms_difference() {
    let mut rng = test_utils::fuzz::Rng::from_seed(0x5eed_09a9);
    for scale in [1.0, 4.0e6, 8.6e7, 1.0e9] {
        for _ in 0..2000 {
            let angle = rng.range(0.0, core::f64::consts::TAU);
            let radius = scale * rng.range(0.5, 2.0);
            let w = Vec2::new(radius * angle.cos(), radius * angle.sin());
            let norm = w.norm();
            for ulps in [0.0, 1.0, -1.0, 3.0, -7.0] {
                let r = radius + ulps * f64::EPSILON * radius;
                let (gap, direct) = (w.radius_gap(r), norm - r);
                assert!(
                    (gap - direct).abs() <= 2.0 * f64::EPSILON * direct.abs(),
                    "radius {r:e}: gap {gap:e} against {direct:e}"
                );
            }
            assert_eq!(
                w.radius_gap(norm),
                0.0,
                "the measured norm is on its own circle"
            );
        }
    }
}

/// At `Interval`, the gap of an offset built at a boxed radius along a
/// direction that is a unit to rounding encloses zero, and is wider
/// than the direct difference by the factor `(1 + ρ)/(1 − ρ)` at most,
/// `ρ` the relative half-width of `norm + radius`: the price of the
/// product form, which the symbolic tier reads as the zero polynomial.
#[test]
fn at_interval_the_gap_is_within_the_boxs_relative_width_of_the_difference() {
    let (s, c) = 0.4_f64.sin_cos();
    for r0 in [0.3, 4.0e6] {
        for rel in [0.33, 0.1, 1e-2, 1e-3, 1e-6, 1e-9] {
            let r = Interval::from_bounds(r0 * (1.0 - rel), r0 * (1.0 + rel));
            let w = Vec2::new(Interval::from_f64(c) * r, Interval::from_f64(s) * r);
            let (gap, direct) = (w.radius_gap(r), w.norm() - r);
            assert!(gap.lo() <= 0.0 && gap.hi() >= 0.0, "encloses zero: {gap:?}");
            let sum = w.norm() + r;
            let rho = (sum.hi() - sum.lo()) / (sum.hi() + sum.lo());
            let factor = (1.0 + rho) / (1.0 - rho);
            let (wide, tight) = (gap.hi() - gap.lo(), direct.hi() - direct.lo());
            assert!(
                wide <= tight * factor * (1.0 + 1e-9),
                "r0 {r0:e} ±{rel:e}: {wide:e} against {tight:e} × {factor}"
            );
        }
    }
}
