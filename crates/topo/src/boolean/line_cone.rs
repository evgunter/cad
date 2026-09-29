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
//!   and `k` [`NOISE_ULPS`] half-ulps. `bool_line_cone_noise` refuses
//!   when `δΔ` over the same lever is definitely past the escalation
//!   threshold. Otherwise it is below it, and a definite `Δ` is past it,
//!   so `|Δ| > δΔ` and the computed sign is the true one: a certified
//!   `Miss` has no real root and a certified pair is real.
//! - **Each root's position.** The coefficients' error moves `G(t)` by
//!   at most `e(t) = e_A t² + 2e_B|t| + e_C`, and at a root
//!   `|G′| = 2√Δ`, so the true root lies within `e(t)/(2√Δ)` metres of
//!   the computed one; the root formula's own rounding adds
//!   `k·((|B| + √Δ)/|A| + |t|)`, the cancellation of `−B ± √Δ` over a
//!   small `A`. `bool_line_cone_root_slack` holds their sum to the band,
//!   or the span and trim decisions the caller makes on the root would
//!   be made on the wrong point.
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
/// face's slant extent `lever` (module docs).
///
/// # Errors
///
/// [`Indeterminate`] — the quadratic's own in-band lead or discriminant.
/// An in-band meter is not an error: it is below the escalation
/// threshold, which is all the count argument needs.
#[allow(clippy::too_many_arguments)]
pub(super) fn line_cone_edge_roots<T: Decide>(
    origin: Point3<T>,
    dir: Vec3<T>,
    apex: Point3<T>,
    axis: Vec3<T>,
    half_angle: T,
    lever: T,
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
    match decide(
        "bool_line_cone_noise",
        Margin::over_lever(disc_noise, lever),
        band,
    ) {
        Ok(Sign::Positive) => return Ok(EdgeConeRoots::Door),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
    }
    let ts = match line_cone_roots(origin, dir, apex, axis, half_angle, lever, band)? {
        ConeRoots::Two(ts) => ts,
        ConeRoots::Miss => return Ok(EdgeConeRoots::Miss),
        ConeRoots::GeneratorParallel | ConeRoots::Tangent => return Ok(EdgeConeRoots::Door),
    };
    let root = (b2.powi(2) - a2 * c2).max(T::zero()).sqrt();
    for t in ts {
        let moved = (e_a * t.powi(2) + two * e_b * t.abs() + e_c) / (two * root);
        let rounding = k * ((b2.abs() + root) / a2.abs() + t.abs());
        match decide(
            "bool_line_cone_root_slack",
            Margin::of(moved + rounding),
            band,
        ) {
            Ok(Sign::Positive) => return Ok(EdgeConeRoots::Door),
            Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
        }
    }
    Ok(EdgeConeRoots::Two(ts))
}
