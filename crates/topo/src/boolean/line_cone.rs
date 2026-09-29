//! **The line × cone root door for an edge span**: the ray lane's
//! quadratic ([`super::solid_contain::line_cone_roots`]) with an `f64`
//! noise meter in front of its count and behind its roots, the shape
//! [`super::circle_torus::half_angle_roots`] carries for the circle.
//!
//! # Why a quadratic needs a meter
//!
//! The coefficients `A = (d·â)² − cos²α`, `B` and `C` of
//! `G(t) = A t² + 2B t + C` are built from `w = q − apex`, so `C` is a
//! difference of terms as large as `|w|²`: an edge whose carrier origin
//! is far from the apex carries rounding of `u·|w|²` in `C` while its
//! crossings live where `G` is small. Two decisions rest on the computed
//! coefficients, and each is metered:
//!
//! - **The count** is the sign of `Δ = B² − AC`, decided over the lever
//!   (`bool_ray_cone_disc`). `Δ`'s evaluation error is bounded by
//!   `δΔ = 2|B|·e_B + |A|·e_C + |C|·e_A + k·(B² + |AC|)`, with `e_X = k·T_X`
//!   the error of coefficient `X` from the magnitudes `T_X` of its terms
//!   and `k` [`NOISE_ULPS`] half-ulps. `bool_line_cone_noise` decides
//!   the discriminant again with that error charged, `(|Δ| − δΔ)` over
//!   the same lever, and refuses unless it is still definite: then
//!   `|Δ| > δΔ`, and the computed sign is the true one — a certified
//!   `Miss` has no real root and a certified pair is real. The charge is
//!   against `Δ` itself, not the band: a grazing line whose origin is a
//!   kilometre off has noise far above the band and a discriminant far
//!   above its noise.
//! - **Each root's position.** The coefficients' error moves `G(t)` by
//!   at most `e(t) = e_A t² + 2e_B|t| + e_C`, and at a root
//!   `|G′| = 2√Δ`, so the true root lies within `e(t)/(2√Δ)` metres of
//!   the computed one; the root formula's own rounding adds
//!   `k·((|B| + √Δ)/|A| + |t|)`, the cancellation of `−B ± √Δ` over a
//!   small `A`. `bool_line_cone_root_slack` holds their sum to the band,
//!   or the span and trim decisions the caller makes on the root would
//!   be made on the wrong point — unless the root is farther from the
//!   caller's span than its slack (`bool_line_cone_root_off_span`), where
//!   no decision reads its position: a line nearly parallel to a
//!   generator has a far root at `2|B|/|A|` metres, whose rounding is
//!   metres too.
//!
//! Like the circle lane's meter it is a ROUNDING estimate, the `f64`
//! lane's contract; the `Interval` lane's coefficients enclose the true
//! ones and the ladder decides on the enclosures, so the meter runs
//! there harmlessly.
//!
//! # What keeps the door
//!
//! A line parallel to a generator (the quadratic degenerates), a
//! tangency (a graze along a generator), and every line through the
//! apex (a double root there, and no tangent plane): all three are
//! `Door`, never a crossing and never a miss.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::circle_torus::NOISE_ULPS;
use super::solid_contain::{ConeRoots, line_cone_roots};
use crate::validate::decide;

/// What the metered line × cone roots say about the line.
#[derive(Debug, Clone, Copy)]
pub(super) enum EdgeConeRoots<T> {
    /// No certified answer: a generator-parallel line, a tangency, a line
    /// through the apex, or a count or root the `f64` noise could move.
    Door,
    /// A certified miss: the line never meets the double cone.
    Miss,
    /// Two certified roots of the line's own parameter, unordered.
    Two([T; 2]),
}

/// The certified, metered roots of the line `origin + t·dir` (`dir`
/// unit) against the double cone `(apex, axis, half_angle)`, over the
/// face's slant extent `lever`, for a caller that reads them against the
/// span `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`Indeterminate`] — the quadratic's own in-band lead or discriminant.
/// An in-band meter is not an error: the door refuses, since the count
/// argument needs the charged discriminant definite.
#[allow(clippy::too_many_arguments)]
pub(super) fn line_cone_edge_roots<T: Decide>(
    origin: Point3<T>,
    dir: Vec3<T>,
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    lever: T,
    (t0, t1): (T, T),
    band: Band,
) -> Result<EdgeConeRoots<T>, Indeterminate> {
    // The coefficients as [`line_cone_roots`] builds them, and the
    // magnitudes of the terms each is a sum of.
    let (_, cos_a) = half_angle.sin_cos();
    let cos2 = cos_a.powi(2);
    let w0 = origin - apex;
    let da = dir.dot(axis);
    let wa = w0.dot(axis);
    let wd = w0.dot(dir);
    let a2 = da.powi(2) - cos2;
    let b2 = da * wa - wd * cos2;
    let c2 = wa.powi(2) - w0.norm_squared() * cos2;
    let k = T::from_f64(NOISE_ULPS * f64::EPSILON * 0.5);
    let e_a = k * (da.powi(2) + cos2);
    let e_b = k * ((da * wa).abs() + wd.abs() * cos2);
    let e_c = k * (wa.powi(2) + w0.norm_squared() * cos2);
    let two = T::from_f64(2.0);
    let disc_noise =
        two * b2.abs() * e_b + a2.abs() * e_c + c2.abs() * e_a + k * (b2.powi(2) + (a2 * c2).abs());
    let answer = match line_cone_roots(origin, dir, apex, axis, half_angle, lever, band)? {
        ConeRoots::Two(ts) => EdgeConeRoots::Two(ts),
        ConeRoots::Miss => EdgeConeRoots::Miss,
        ConeRoots::GeneratorParallel | ConeRoots::Tangent => return Ok(EdgeConeRoots::Door),
    };
    let disc = b2.powi(2) - a2 * c2;
    match decide(
        "bool_line_cone_noise",
        Margin::over_lever(disc.abs() - disc_noise, lever),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) | Err(_) => return Ok(EdgeConeRoots::Door),
    }
    let EdgeConeRoots::Two(ts) = answer else {
        return Ok(answer);
    };
    let root = disc.max(T::zero()).sqrt();
    for t in ts {
        let moved = (e_a * t.powi(2) + two * e_b * t.abs() + e_c) / (two * root);
        let rounding = k * ((b2.abs() + root) / a2.abs() + t.abs());
        let slack = moved + rounding;
        match decide("bool_line_cone_root_slack", Margin::of(slack), band) {
            Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
            // Uncertain past the band: harmless only where the span
            // decisions cannot reach it.
            Ok(Sign::Positive) => match decide(
                "bool_line_cone_root_off_span",
                Margin::of((t0 - t).max(t - t1) - slack),
                band,
            ) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero | Sign::Negative) | Err(_) => return Ok(EdgeConeRoots::Door),
            },
        }
    }
    Ok(EdgeConeRoots::Two(ts))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]
mod tests {
    //! The door against an oracle that does not share its conditioning:
    //! the quadratic solved from the line's point NEAREST the apex, by
    //! the cancellation-free formula, and the elevation evaluated
    //! directly at each root.

    use super::*;
    use geom_core::Tol;

    /// The face's slant extent the rows lever by.
    const LEVER: f64 = 1.5;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The default row's band, pinned: the grazing depths are chosen
    /// against its thresholds.
    fn fixed_band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// The cone: apex at the origin, axis `+z`, half-angle `π/4`.
    fn door(o: [f64; 3], d: [f64; 3], band: Band) -> EdgeConeRoots<f64> {
        try_door(o, d, band).unwrap()
    }

    /// [`door`], with an escalation — the band declining to call a
    /// margin, which the edge lane raises as a refusal — read as `Door`.
    fn refusing_door(o: [f64; 3], d: [f64; 3], band: Band) -> EdgeConeRoots<f64> {
        try_door(o, d, band).unwrap_or(EdgeConeRoots::Door)
    }

    fn try_door(o: [f64; 3], d: [f64; 3], band: Band) -> Result<EdgeConeRoots<f64>, Indeterminate> {
        try_door_over(o, d, (-5.0, 5.0), band)
    }

    /// The door, read against the span `[t0, t1]`.
    fn try_door_over(
        o: [f64; 3],
        d: [f64; 3],
        span: (f64, f64),
        band: Band,
    ) -> Result<EdgeConeRoots<f64>, Indeterminate> {
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        line_cone_edge_roots(
            Point3::new(o[0], o[1], o[2]),
            Vec3::new(d[0] / n, d[1] / n, d[2] / n),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            core::f64::consts::FRAC_PI_4,
            LEVER,
            span,
            band,
        )
    }

    fn elevation(p: [f64; 3]) -> f64 {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        p[0].hypot(p[1]) * s - p[2].abs() * s
    }

    /// The oracle's roots of the line through `p` along unit `d`, as
    /// parameters from `p`: `G = ((p + t d)·ẑ)² − |p + t d|²/2` solved
    /// by `t = C/q`, `q = −(B + sign(B)√Δ)`, which never cancels.
    fn oracle(p: [f64; 3], d: [f64; 3]) -> Vec<f64> {
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let a = d[2] * d[2] - 0.5;
        let b = d[2] * p[2] - 0.5 * dot(p, d);
        let c = p[2] * p[2] - 0.5 * dot(p, p);
        let disc = b * b - a * c;
        if disc < 0.0 {
            return Vec::new();
        }
        let q = -(b + disc.sqrt().copysign(b));
        let mut ts = vec![q / a, c / q];
        ts.sort_by(f64::total_cmp);
        ts
    }

    fn unit(d: [f64; 3]) -> [f64; 3] {
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        [d[0] / n, d[1] / n, d[2] / n]
    }

    fn sorted(ts: [f64; 2]) -> [f64; 2] {
        if ts[0] <= ts[1] { ts } else { [ts[1], ts[0]] }
    }

    /// The three root classes: a line inside the aperture meets each
    /// nappe once, one outside it meets one nappe twice, and one outside
    /// it clear of the cone misses.
    #[test]
    fn the_three_generic_classes_match_the_oracle() {
        for (label, p, d, want) in [
            ("inside the aperture", [0.1, 0.05, 0.0], [0.1, 0.2, 1.0], 2),
            ("outside, one nappe", [-2.0, 0.3, 1.0], [1.0, 0.1, 0.2], 2),
            ("outside, a miss", [3.0, -1.0, 0.1], [0.1, 1.0, 0.05], 0),
        ] {
            let d = unit(d);
            let truth = oracle(p, d);
            assert_eq!(truth.len(), want, "{label}: the fixture");
            match door(p, d, band()) {
                EdgeConeRoots::Two(ts) => {
                    assert_eq!(want, 2, "{label}: a certified pair");
                    for (t, w) in sorted(ts).iter().zip(&truth) {
                        assert!((t - w).abs() < 1e-12, "{label}: root {t} vs {w}");
                        let q = [p[0] + t * d[0], p[1] + t * d[1], p[2] + t * d[2]];
                        assert!(elevation(q).abs() < 1e-12, "{label}: on the cone");
                    }
                    let h: Vec<f64> = ts.iter().map(|t| p[2] + t * d[2]).collect();
                    let opposite = h[0] * h[1] < 0.0;
                    assert_eq!(
                        opposite,
                        label == "inside the aperture",
                        "{label}: the nappes {h:?}"
                    );
                }
                EdgeConeRoots::Miss => assert_eq!(want, 0, "{label}: a certified miss"),
                EdgeConeRoots::Door => panic!("{label}: answered"),
            }
        }
    }

    /// **A line parallel to a generator keeps the door** (Q3): the
    /// quadratic's leading coefficient vanishes, and its one root is not
    /// taken — never a miss, which the same-side arm would read as no
    /// event.
    #[test]
    fn a_generator_parallel_line_keeps_the_door() {
        let g = unit([1.0, 0.0, 1.0]);
        for p in [[-0.2, 0.0, 0.5], [0.3, 0.1, -0.4], [2.0, -1.0, 0.0]] {
            assert!(
                matches!(door(p, g, band()), EdgeConeRoots::Door),
                "{p:?} along a generator"
            );
        }
    }

    /// **Every line through the apex has a double root there**, and the
    /// apex has no tangent plane: the door, never a miss or a pair —
    /// inside the aperture, outside it, and along the axis.
    #[test]
    fn every_line_through_the_apex_keeps_the_door() {
        for d in [[0.0, 0.0, 1.0], [0.3, 0.2, 1.0], [1.0, 0.4, 0.2]] {
            let d = unit(d);
            let p = [-0.7 * d[0], -0.7 * d[1], -0.7 * d[2]];
            assert!(
                matches!(door(p, d, band()), EdgeConeRoots::Door),
                "through the apex along {d:?}"
            );
        }
    }

    /// A line through `P + depth·n̂` along the cone's azimuthal tangent
    /// at `P`, `P` at slant `1` on the upper nappe at azimuth `phi`: the
    /// surface curves away from it, so it misses for `depth > 0` and dips
    /// through for `depth < 0`. Returned with its origin moved `far`
    /// back along the line, and the oracle's roots from the dip point.
    #[allow(clippy::type_complexity)]
    fn grazing(phi: f64, depth: f64, far: f64) -> ([f64; 3], [f64; 3], Vec<f64>) {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let (sp, cp) = phi.sin_cos();
        let n = [cp * s, sp * s, -s];
        let p = [
            cp * s + depth * n[0],
            sp * s + depth * n[1],
            s + depth * n[2],
        ];
        let d = [-sp, cp, 0.0];
        let o = [p[0] - far * d[0], p[1] - far * d[1], p[2] - far * d[2]];
        let truth = oracle(p, d).into_iter().map(|t| t + far).collect();
        (o, d, truth)
    }

    /// **A far origin does not certify rounding noise.** The quadratic's
    /// coefficients carry `|w|²`-sized terms for a carrier origin `|w|`
    /// from the apex, and grazing lines put the answer in their rounding:
    /// at the default band, pinned (the depths are against it), every
    /// answer the door gives at origins up to `1e6` m back along the line
    /// must be the oracle's — a dip's two roots within the band's
    /// escalation threshold, or a clearance's miss. The door may refuse,
    /// and with its origin at the dip it must answer. Measured, it
    /// answers 16, 12 and 8 of the 32 at `1e2`, `1e3` and `1e4` m, and
    /// none from `1e5` m.
    #[test]
    fn a_far_origin_does_not_certify_noise() {
        let mut answered = [0u32; 6];
        let mut wrong = Vec::new();
        for (i, far) in [0.0, 1e2, 1e3, 1e4, 1e5, 1e6].into_iter().enumerate() {
            for phi in [0.3_f64, 1.9, 3.4, 5.1] {
                for depth in [-1e-5, -1e-6, -1e-7, -1.6e-8, 1.6e-8, 1e-7, 1e-6, 1e-5] {
                    let (o, d, truth) = grazing(phi, depth, far);
                    assert_eq!(truth.len(), if depth < 0.0 { 2 } else { 0 }, "fixture");
                    let label = format!("far {far}, φ {phi}, depth {depth}");
                    // The span a metre either side of the dip.
                    match try_door_over(o, d, (far - 1.0, far + 1.0), fixed_band())
                        .unwrap_or(EdgeConeRoots::Door)
                    {
                        EdgeConeRoots::Door => {}
                        EdgeConeRoots::Miss => {
                            answered[i] += 1;
                            if !truth.is_empty() {
                                wrong.push(format!("{label}: a certified miss on a dip"));
                            }
                        }
                        EdgeConeRoots::Two(ts) => {
                            answered[i] += 1;
                            if truth.len() != 2 {
                                wrong.push(format!("{label}: a phantom pair {ts:?}"));
                                continue;
                            }
                            for (t, w) in sorted(ts).iter().zip(&truth) {
                                if (t - w).abs() >= 1e-8 {
                                    wrong.push(format!("{label}: root {t} vs {w}"));
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("answered per origin distance (of 32): {answered:?}");
        assert!(wrong.is_empty(), "wrong certified answers: {wrong:#?}");
        // At the dip point every depth past the band answers; the two
        // `1.6e-8` grazes per azimuth put the discriminant in the band's
        // escalation gap.
        assert_eq!(
            answered[0], 24,
            "at the dip point the door answers: {answered:?}"
        );
    }

    /// **A root the formula cancels is not certified at the wrong point.**
    /// Nearly parallel to a generator, `A` is small but definite, and the
    /// near root `(−B + √Δ)/A` subtracts two numbers of size `|B|`: over
    /// `|A|` their rounding is a displacement of `u·|B|/|A|`. At these
    /// two poses (found by an exact-arithmetic search) the formula's near
    /// root is 2.3e-7 m and 2.2e-7 m off the true one. The door must give
    /// the oracle's near root or refuse; the far root, at `2|B|/|A|`
    /// metres, is past what an `f64` oracle can place.
    #[test]
    fn a_cancelled_near_root_is_not_certified_off_the_oracle() {
        for (tilt, p, phi) in [
            (
                1.050_106_003_786_270_5e-8,
                [
                    21.129_932_133_410_57,
                    -24.060_937_591_525_533,
                    28.939_456_118_774_79,
                ],
                1.502_617_736_960_703_2_f64,
            ),
            (
                1.164_706_284_198_742_8e-8,
                [
                    -9.104_083_400_321_045,
                    -26.442_039_833_415_02,
                    39.847_962_645_213_03,
                ],
                0.058_364_330_640_065_167_f64,
            ),
        ] {
            let (sp, cp) = phi.sin_cos();
            let d = unit([cp * (1.0 - tilt), sp * (1.0 - tilt), 1.0 + tilt]);
            let near = oracle(p, d)
                .into_iter()
                .min_by(|a, b| a.abs().total_cmp(&b.abs()))
                .unwrap();
            if let EdgeConeRoots::Two(ts) = refusing_door(p, d, fixed_band()) {
                let got = if ts[0].abs() < ts[1].abs() {
                    ts[0]
                } else {
                    ts[1]
                };
                assert!(
                    (got - near).abs() < 1e-8,
                    "tilt {tilt}: near root {got} vs {near}"
                );
            }
        }
    }
}
