//! **The circle × torus root door**: the certified crossings of a
//! CIRCLE carrier with a torus, beside the line × torus quartic
//! ([`super::solid_contain::line_torus_roots`]).
//!
//! # The polynomial is a quartic, not an octic
//!
//! With `q(θ) = C(θ) − c` for the carrier
//! `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, `v̂ = n̂ × û`, the torus
//! `(c, â, R, r)` is the zero set of
//! `F = (|q|² + R² − r²)² − 4R²(|q|² − (q·â)²)`. Both `S = |q|²` and
//! `h = q·â` are FIRST harmonics in `θ` (with `e(θ) = û cos θ +
//! v̂ sin θ`, the cross term `2ρ(C₀ − c)·e(θ)` is the only
//! `θ`-dependence of `S`, since `|e(θ)| = 1`), so `F` is a
//! trigonometric polynomial of degree TWO, and the shared half-angle
//! ladder ([`super::circle_roots::half_angle_roots`]) solves it — Bézout's
//! eight for a conic against a quartic surface loses four to the
//! circular points at infinity, through which both the circle and the
//! (bicircular) torus pass. At most four crossings per turn.
//!
//! The ladder's rows here are `bool_circle_torus_pole`, `_conditioning`,
//! `_noise` and the quartic's `bool_circle_torus_*`, and every in-band
//! sign escalates as [`BooleanDecision::ArcTorusRoots`]; the answer is
//! the certified subdivision's (`bool_circle_torus_sub_*`).
//!
//! # The noise meter's floor, and what it costs
//!
//! `F`'s harmonics are sums of terms as large as `(|C₀ − c|² + ρ²)²` —
//! `ρ⁴` for a large circle. `F = 2r·res·Q` with `Q ≥ R² − r²` on a ring
//! torus, so the floor on `|F|` per metre of residual the meter divides
//! by is `2r(R² − r²)`.
//!
//! What supports the ladder's premise here is measurement, not proof:
//! the delta review's fuzz — about 12k cases around the refusal
//! threshold, some 50k in all — found no wrong answer from the metered
//! door, and a maximum root error of 4.8e-10 m on the parallel arm.
//! That supports the premise at the poses it drew; it does not bound the
//! amplification, and the ladder's in-band answers are a known gap
//! (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//!
//! **What the noise costs.** Measured when the ladder answered: against
//! a torus `R = 1, r = 0.25` at the default band, grazing circles at
//! `ρ = 10` were answered and from `ρ = 30` every one refused — where
//! the unmetered door had certified misses on real dips and phantom
//! pairs on clearances from `ρ = 100`. The threshold scales as
//! `ρ⁴ ≲ 10ε·r·R²/(u·HARMONIC_NOISE_ULPS)`. The meter now only keeps the
//! ladder from running past it; the subdivision charges the same noise
//! to every Taylor term, so such a pose answers `Uncertain` there
//! (`a_large_circles_dip_below_its_own_noise_is_not_certified_away`).
//!
//! # The lever
//!
//! The ladder's lever is the length its roots spread over in
//! `τ = 2ρ·t`: the smaller of the carrier's own `2ρ` and the torus's
//! extent `R + r`. A circle smaller than the torus has its roots spread
//! over its own size (the torus's extent there made the margins scale
//! like `ρ¹⁰/(R + r)¹¹`, and small generic circles escalated), and one
//! larger than the torus meets it only where it passes through the
//! torus's extent. The biquadratic arm's accuracy is metered by the same
//! lever, and taking the smaller length keeps its dropped term within
//! the band on the spread the roots actually have.
//!
//! # The special poses
//!
//! - **Coaxial** (the carrier's axis parallel to `â` and its centre on
//!   the torus axis): `S` and `h` are constant, so `F` is constant and
//!   the quartic is `F·(1 + t²)²` — a repeated complex pair the ladder
//!   cannot answer. It is decided FIRST, geometrically
//!   (`bool_circle_torus_coaxial_tilt`, `_offset`, both metres), and the
//!   constant residual then decides it, its spread from the in-band
//!   offset and tilt charged (`bool_circle_torus_coaxial_residual`,
//!   [`super::circle_roots`], "A constant residual"): zero is
//!   [`CircleRoots::OnSurface`] (a rim circle of the torus), and
//!   definite a [`CircleRoots::Miss`].
//! - **Parallel axes** (the circle's plane perpendicular to `â`, off
//!   the axis — the lily's pose): `h` is constant, the plane meets the
//!   tube in two contour circles, and the crossings are circle ×
//!   contour intersections in closed form, decided on the surface's
//!   RESIDUAL at the carrier's two extremes, with the band's residual
//!   tilt charged ([`parallel_axes_roots`]). The quartic does not degenerate here,
//!   but its discriminant is the wrong instrument: it also measures the
//!   separation of the COMPLEX roots, so a carrier passing a few
//!   millimetres clear of a contour reads as a tangency at a coarse
//!   band (measured on the lily at ε = 1e-6: the arch's outer seam,
//!   8 mm inside the stem's outer contour).
//! - **The circle lies ON the torus** (a rim, meridian or Villarceau
//!   circle, none of them parallel-axes except the rims, which are
//!   coaxial): `F ≡ 0`, so the pole is on the torus at every anchor and
//!   the answer is `Uncertain`, which keeps a meridian or Villarceau
//!   circle from every recording arm. A coaxial rim is answered on the
//!   coaxial arm instead (`OnSurface`), and the reduction records it only
//!   under `reduce::lying_on`'s certificates.
//! - **A tangency** — the carrier grazing the tube, a double root — is a
//!   contour-reach margin in band on the parallel arm, and on the
//!   general arm a piece neither clear nor monotone down to the band's
//!   width, answered `Uncertain` (module docs of [`super::circle_roots`],
//!   "The half-angle ladder, and the subdivision that answers").

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::circle_roots::{
    CircleRoots, HalfAngleFrame, HalfAngleRows, Harmonics, SubdivisionRows,
    constant_residual_roots, half_angle_roots, rounding_charge,
};
use super::solid_contain::QuarticRows;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// The circle × torus lane's ladder rows.
const CIRCLE_TORUS_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_torus_pole",
    conditioning: "bool_circle_torus_pole_conditioning",
    noise: "bool_circle_torus_noise",
    quartic: QuarticRows {
        disc: "bool_circle_torus_disc",
        shape: "bool_circle_torus_shape",
        depth: "bool_circle_torus_depth",
        odd: "bool_circle_torus_odd",
        split: "bool_circle_torus_split",
        split_lead: "bool_circle_torus_split_lead",
    },
    verify: SubdivisionRows {
        clear: "bool_circle_torus_sub_clear",
        monotone: "bool_circle_torus_sub_monotone",
        side: "bool_circle_torus_sub_side",
        width: "bool_circle_torus_sub_width",
    },
    decision: BooleanDecision::ArcTorusRoots,
};

/// An in-band sign of this door's own rows, escalated as its decision.
fn escalated(diag: Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::ArcTorusRoots,
        diag,
    }
}

/// The certified crossings of the `carrier` circle with the `torus`,
/// anchored so that the arc `[t0, t1]` avoids the substitution's pole
/// (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not a
/// circle or `torus` not a torus — the caller dispatched on those kinds.
/// An escalation as [`BooleanDecision::ArcTorusRoots`] for an in-band
/// classifying sign: a coaxial carrier's constant residual, a rung of the
/// parallel arm, or one of the ladder's. An in-band sign at the coaxial
/// tilt is not an error: the pose takes the ladder.
pub(super) fn circle_torus_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    torus: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let (
        &geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        },
        &geom::Surface::Torus {
            center: t_center,
            axis: t_axis,
            major_radius,
            minor_radius,
            ..
        },
    ) = (carrier, torus)
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "the circle × torus root door was handed a carrier that is not a circle \
                   or a surface that is not a torus",
        });
    };
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let v_ref = axis.cross(u_ref);
    let w0 = center - t_center;
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };

    // The parallel-axes poses, decided on the geometry in metres: the
    // tilt of the carrier's axis times its radius is the amplitude of
    // the carrier's height about the torus axis, and the carrier
    // centre's distance from that axis separates coaxial from merely
    // parallel.
    let tilt = radius * axis.cross(t_axis).norm();
    let h0 = w0.dot(t_axis);
    let w_perp = w0 - t_axis * h0;
    let offset = w_perp.norm();
    let parallel = matches!(
        decide("bool_circle_torus_coaxial_tilt", Margin::of(tilt), band),
        Ok(Sign::Zero)
    );
    if parallel {
        return match decide("bool_circle_torus_coaxial_offset", Margin::of(offset), band) {
            // Coaxial: the residual is one value along the carrier, to
            // within its spread about the reading at `θ = 0`. Every
            // carrier point lies within `δ = offset + √2·tilt` of the
            // coaxial circle of the same radius (a translation by the
            // offset, then a turn of at most a right angle, whose chord
            // `2ρ·sin(α/2)` is at most `√2·ρ·sin α`), on which the
            // residual is constant. The residual is `(g² − r²)/2r` in
            // the distance `g` from the core circle, which is
            // 1-Lipschitz, so between two points `d` apart with `g` at
            // most `G` it moves at most `d·G/r`; every point in play has
            // `g ≤ g₀ + 2δ`. The reading is within `δ·G/r` of the
            // coaxial value and so is every carrier point: the spread is
            // twice that.
            Ok(Sign::Zero) => {
                let constant = geom_brep::implicit_residual(torus, point_at(T::zero()));
                let delta = offset + T::from_f64(core::f64::consts::SQRT_2) * tilt;
                let g0 = (minor_radius.powi(2) + two * minor_radius * constant)
                    .max(T::zero())
                    .sqrt();
                let spread = two * delta * (g0 + two * delta) / minor_radius;
                constant_residual_roots(
                    constant,
                    spread,
                    "bool_circle_torus_coaxial_residual",
                    band,
                )
                .map_err(escalated)
            }
            Ok(Sign::Positive) => parallel_axes_roots(
                &ParallelPose {
                    center,
                    radius,
                    u_ref,
                    v_ref,
                    w_perp,
                    offset,
                    h0,
                    tilt,
                    t0,
                    t1,
                },
                torus,
                major_radius,
                minor_radius,
                band,
            )
            .map_err(escalated),
            // A negative length is not an answer; an in-band one is a
            // near-coaxial pose neither arm can stand behind.
            Ok(Sign::Negative) | Err(_) => Ok(CircleRoots::Uncertain),
        };
    }

    // `F` along the carrier as a degree-2 trigonometric polynomial about
    // `θ = 0`, from `S = S₀ + S₁c cos θ + S₁s sin θ` and
    // `h = H₀ + H₁c cos θ + H₁s sin θ` (module docs), using
    // `(x cos θ + y sin θ)² = (x² + y²)/2 + (x² − y²)/2·cos 2θ + x y sin 2θ`.
    let half = T::from_f64(0.5);
    let rr = major_radius.powi(2);
    let four_rr = four * rr;
    let s_k = w0.norm_squared() + radius.powi(2) + rr - minor_radius.powi(2);
    let s0 = w0.norm_squared() + radius.powi(2);
    let (s1c, s1s) = (two * radius * w0.dot(u_ref), two * radius * w0.dot(v_ref));
    let (h1c, h1s) = (radius * u_ref.dot(t_axis), radius * v_ref.dot(t_axis));
    let harmonics = Harmonics {
        c0: s_k.powi(2) + (s1c.powi(2) + s1s.powi(2)) * half
            - four_rr * (s0 - h0.powi(2) - (h1c.powi(2) + h1s.powi(2)) * half),
        c1: two * s_k * s1c - four_rr * (s1c - two * h0 * h1c),
        s1: two * s_k * s1s - four_rr * (s1s - two * h0 * h1s),
        c2: (s1c.powi(2) - s1s.powi(2)) * half + four_rr * (h1c.powi(2) - h1s.powi(2)) * half,
        s2: s1c * s1s + four_rr * h1c * h1s,
    };
    // The lever (module docs, "The lever").
    let lever = (two * radius).min(major_radius + minor_radius);
    // The noise meter's inputs: a bound on every term the harmonics are
    // built from, and the torus's floor on `|F|` per metre of residual.
    let s_abs = s_k.abs() + s1c.abs() + s1s.abs();
    let h_abs = h0.abs() + h1c.abs() + h1s.abs();
    let terms = s_abs.powi(2) + four_rr * (s0.abs() + s1c.abs() + s1s.abs() + h_abs.powi(2));
    let f_per_metre = two * minor_radius * (rr - minor_radius.powi(2));
    half_angle_roots(
        &harmonics,
        |theta| geom_brep::implicit_residual(torus, point_at(theta)),
        HalfAngleFrame {
            t0,
            t1,
            speed_lo: radius,
            speed_hi: radius,
            lever,
            noise: rounding_charge(terms),
            f_per_metre,
        },
        &CIRCLE_TORUS_ROWS,
        band,
    )
}

/// The parallel-axes pose's data ([`parallel_axes_roots`]).
struct ParallelPose<T: geom_core::Real> {
    center: Point3<T>,
    radius: T,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    /// The carrier centre's offset from the torus axis, perpendicular
    /// to it, and its length.
    w_perp: Vec3<T>,
    offset: T,
    /// The carrier centre's height along the torus axis.
    h0: T,
    /// The amplitude of the carrier's height about `h0` (in band here).
    tilt: T,
    /// The arc: roots are reported within `π` of its midpoint.
    t0: T,
    t1: T,
}

/// Where one extreme of the carrier's distance from the torus axis sits
/// among the plane's contour circles `ρ− < ρ+`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Band3 {
    /// `ρ < ρ−`: in the hole.
    Hole,
    /// `ρ− < ρ < ρ+`: inside the tube.
    Tube,
    /// `ρ > ρ+`: beyond the tube.
    Beyond,
}

/// **The parallel-axes pose, in closed form, decided on normal
/// distance.** A carrier whose axis is parallel to the torus's lies in a
/// plane at height `h₀`, which meets the tube in the two contour circles
/// `ρ± = R ± √(r² − h₀²)` about the torus axis. The carrier's distance
/// from that axis sweeps `[ρmin, ρmax] = [|ρc − d|, ρc + d]`
/// monotonically on each half-turn about `θ₀` (its farthest point), so
/// it crosses a contour — twice, at `θ₀ ± acos((ρ±² − d² − ρc²)/(2ρc·d))`
/// — exactly when that contour lies strictly between `ρmin` and
/// `ρmax`.
///
/// **Every classifying margin is the surface's RESIDUAL, not an in-plane
/// reach.** Where the contour is crossed, the residual along the carrier
/// changes sign, and the bump it makes between the two roots is the
/// residual at the extreme point, `C(θ₀)` or `C(θ₀ + π)`. Near the top
/// or bottom of the tube the surface is nearly parallel to the plane, so
/// an in-plane reach of `x` past a contour is a normal distance of only
/// `x·√(r² − h₀²)/r`: deciding on the reach certified crossings whose
/// bump the band cannot see. Each extreme is therefore placed by its own
/// residual (`bool_circle_torus_contour_residual`), and a bump in band
/// is `Uncertain`.
///
/// **The tilt the pose was admitted with is carried, not dropped.** The
/// pose is parallel only to within the band: the carrier's height
/// varies by up to `tilt` about `h₀`. The residual `((ρ−R)² + h² −
/// r²)/2r` then differs from the untilted one by at most
/// `tilt·(2|h₀| + tilt)/2r` in `h`, plus the in-plane shortening of a
/// tilted circle, at most `tilt²/ρc` in `ρ` at a slope of at most
/// `(ρmax + R)/r` — and every residual margin is charged that much.
///
/// The remaining decisions are the plane's depth into the tube,
/// `(r² − h₀²)/2r` — the residual of the tube's centre circle at that
/// height, negated (`bool_circle_torus_plane_height`) — and which side
/// of the tube's centre circle an extreme outside the tube lies
/// (`bool_circle_torus_contour_side`, `ρ − R`, which is at least
/// `√(r² − h₀²)` from zero there).
fn parallel_axes_roots<T: Decide>(
    pose: &ParallelPose<T>,
    torus: &geom::Surface<T>,
    major_radius: T,
    minor_radius: T,
    band: Band,
) -> Result<CircleRoots<T>, Indeterminate> {
    let &ParallelPose {
        center,
        radius,
        u_ref,
        v_ref,
        w_perp,
        offset,
        h0,
        tilt,
        t0,
        t1,
    } = pose;
    let two = T::from_f64(2.0);
    let charge_h = tilt * (two * h0.abs() + tilt) / (two * minor_radius);
    let depth = (minor_radius.powi(2) - h0.powi(2)) / (two * minor_radius);
    match decide(
        "bool_circle_torus_plane_height",
        Margin::of(depth - charge_h),
        band,
    )? {
        Sign::Positive => {}
        // Definitely past the top or bottom of the tube, even charged.
        Sign::Negative
            if matches!(
                decide(
                    "bool_circle_torus_plane_height",
                    Margin::of(depth + charge_h),
                    band
                )?,
                Sign::Negative
            ) =>
        {
            return Ok(CircleRoots::Miss);
        }
        _ => return Ok(CircleRoots::Uncertain),
    }
    let rho_max = radius + offset;
    let rho_min = (radius - offset).abs();
    // Doubled: the bound above is for a point whose height and in-plane
    // radius the tilt moves once; an interior point of the arc between
    // the two extremes is moved by both the tilt of its own position and
    // the tilt's shift of where the extremes fall, which the doubling
    // covers.
    let charge = two * (charge_h + tilt.powi(2) / radius * (rho_max + major_radius) / minor_radius);
    // `θ₀`: the carrier's farthest point from the torus axis, along
    // `+w_perp`.
    let theta0 = w_perp.dot(v_ref).atan2(w_perp.dot(u_ref));
    let place = |theta: T, rho: T| -> Result<Option<Band3>, Indeterminate> {
        let (s, c) = theta.sin_cos();
        let res = geom_brep::implicit_residual(
            torus,
            center + u_ref * (radius * c) + v_ref * (radius * s),
        );
        let inside = decide(
            "bool_circle_torus_contour_residual",
            Margin::of(res + charge),
            band,
        )?;
        if inside == Sign::Negative {
            return Ok(Some(Band3::Tube));
        }
        let outside = decide(
            "bool_circle_torus_contour_residual",
            Margin::of(res - charge),
            band,
        )?;
        if outside != Sign::Positive {
            return Ok(None);
        }
        Ok(
            match decide(
                "bool_circle_torus_contour_side",
                Margin::of(rho - major_radius),
                band,
            )? {
                Sign::Positive => Some(Band3::Beyond),
                Sign::Negative => Some(Band3::Hole),
                Sign::Zero => None,
            },
        )
    };
    let (Some(far), Some(near)) = (place(theta0, rho_max)?, place(theta0 + T::pi(), rho_min)?)
    else {
        return Ok(CircleRoots::Uncertain);
    };
    let half = ((minor_radius.powi(2) - h0.powi(2)).max(T::zero())).sqrt();
    // Contour ρ− lies between the extremes iff the near one is in the
    // hole and the far one is not; ρ+ iff the far one is beyond and the
    // near one is not.
    let crossed = [
        (
            major_radius - half,
            near == Band3::Hole && far != Band3::Hole,
        ),
        (
            major_radius + half,
            far == Band3::Beyond && near != Band3::Beyond,
        ),
    ];
    let mid = (t0 + t1) / two;
    let mut thetas = [T::zero(); 4];
    let mut count = 0usize;
    for (contour, hit) in crossed {
        if !hit {
            continue;
        }
        // Inside `[-1, 1]` by the placements above; the clamp is the
        // rounding guard, not a decision.
        let c = ((contour.powi(2) - offset.powi(2) - radius.powi(2)) / (two * radius * offset))
            .max(T::zero() - T::one())
            .min(T::one());
        let spread = c.acos();
        // **Where the roots are, not just that they are.** The roots are
        // placed as if the carrier were untilted; the tilt moves the
        // residual by up to `charge`, and the residual's slope along the
        // carrier at a contour crossing is
        // `(|ρ± − R|/r)·(ρc·d·sin spread/ρ±)` per radian, so the true
        // roots lie within `charge/slope` radians of these — plus the
        // `acos` argument's own rounding, `δc/sin spread`. That arc
        // length must be inside the band, or a caller's span and trim
        // decisions are on the wrong point.
        let sin_spread = (T::one() - c.powi(2)).max(T::zero()).sqrt();
        let slope = half / minor_radius * radius * offset * sin_spread / contour;
        let rounding = rounding_charge(contour.powi(2) + offset.powi(2) + radius.powi(2))
            / (two * radius * offset);
        let slack = radius * (charge / slope + rounding / sin_spread);
        match decide("bool_circle_torus_root_slack", Margin::of(slack), band) {
            Ok(Sign::Positive) => return Ok(CircleRoots::Uncertain),
            // A NaN slack (a zero slope, which the placements above have
            // already refused as a graze) is `Err` and refuses too.
            Ok(Sign::Zero | Sign::Negative) => {}
            Err(_) => return Ok(CircleRoots::Uncertain),
        }
        for theta in [theta0 + spread, theta0 - spread] {
            thetas[count] = mid + (theta - mid).reduce_periodic_centred(T::tau());
            count += 1;
        }
    }
    Ok(if count == 0 {
        CircleRoots::Miss
    } else {
        CircleRoots::Certified { count, thetas }
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]
mod tests {
    //! The door against an independent oracle: `F` sampled densely over
    //! the whole turn and each sign change bisected — simple roots only,
    //! which is the regime the door answers in. Each named pose is one
    //! of the lane's classifications, so a branch answered wrongly goes
    //! red on the pose that reaches it.

    use super::*;
    use geom_core::{Bounds, Interval, Real, Tol};

    /// The door on a circle given by its parts.
    #[allow(clippy::too_many_arguments)]
    fn roots_of<T: Decide>(
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        t0: T,
        t1: T,
        torus: &geom::Surface<T>,
        band: Band,
    ) -> Result<CircleRoots<T>, BooleanError> {
        let carrier = geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        circle_torus_roots(&carrier, t0, t1, torus, band)
    }

    const R: f64 = 1.0;
    const RT: f64 = 0.25;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The default row's band, pinned: the rows that place a margin
    /// against the band's own thresholds (a bump of 5.6e-10 m, a tilt of
    /// 9e-10 m) are claims about THIS band, and at a finer ε the same
    /// geometry is honestly decidable.
    fn fixed_band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn torus<T: geom_core::Real>() -> geom::Surface<T> {
        geom::Surface::Torus {
            center: Point3::new(T::zero(), T::zero(), T::zero()),
            axis: Vec3::new(T::zero(), T::zero(), T::one()),
            major_radius: T::from_f64(R),
            minor_radius: T::from_f64(RT),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    /// A circle pose: centre, unit axis, radius, unit `u_ref ⟂ axis`.
    #[derive(Clone, Copy)]
    struct Pose {
        c: [f64; 3],
        n: [f64; 3],
        rho: f64,
        u: [f64; 3],
    }

    fn v3<T: geom_core::Real>(a: [f64; 3]) -> Vec3<T> {
        Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
    }

    fn point(pose: Pose, theta: f64) -> Point3<f64> {
        let (n, u) = (v3::<f64>(pose.n), v3::<f64>(pose.u));
        let v = n.cross(u);
        Point3::from_array(pose.c) + (u * theta.cos() + v * theta.sin()) * pose.rho
    }

    /// The torus's own implicit `F` at a point — the oracle's function.
    fn big_f(p: Point3<f64>) -> f64 {
        let s = p.x * p.x + p.y * p.y + p.z * p.z;
        (s + R * R - RT * RT).powi(2) - 4.0 * R * R * (s - p.z * p.z)
    }

    /// Independent roots over `[t0, t1]`: sign changes on a fine grid,
    /// bisected.
    fn oracle(pose: Pose, t0: f64, t1: f64) -> Vec<f64> {
        let n = 20_000;
        let f = |t: f64| big_f(point(pose, t));
        let mut out = Vec::new();
        for i in 0..n {
            let (mut a, mut b) = (
                t0 + (t1 - t0) * f64::from(i) / f64::from(n),
                t0 + (t1 - t0) * f64::from(i + 1) / f64::from(n),
            );
            // A root on a grid point is counted in the cell it ends.
            if !(f(a) * f(b) < 0.0 || f(b) == 0.0) {
                continue;
            }
            for _ in 0..80 {
                let m = 0.5 * (a + b);
                if f(a) * f(m) <= 0.0 {
                    b = m;
                } else {
                    a = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        out
    }

    fn door(pose: Pose, t0: f64, t1: f64) -> CircleRoots<f64> {
        roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            t0,
            t1,
            &torus(),
            band(),
        )
        .unwrap()
    }

    /// The door's roots that fall in `[t0, t1]`, sorted.
    fn in_arc(pose: Pose, t0: f64, t1: f64) -> (usize, Vec<f64>) {
        let CircleRoots::Certified { count, thetas } = door(pose, t0, t1) else {
            panic!("expected a certified count");
        };
        let mut ts: Vec<f64> = thetas[..count]
            .iter()
            .copied()
            .filter(|t| (t0..=t1).contains(t))
            .collect();
        ts.sort_by(f64::total_cmp);
        (count, ts)
    }

    fn assert_matches_oracle(label: &str, pose: Pose, t0: f64, t1: f64, want_count: usize) {
        let (count, ts) = in_arc(pose, t0, t1);
        let want = oracle(pose, t0, t1);
        assert_eq!(count, want_count, "{label}: certified count over the turn");
        assert_eq!(
            ts.len(),
            want.len(),
            "{label}: roots in the arc {ts:?} vs {want:?}"
        );
        for (a, b) in ts.iter().zip(&want) {
            assert!((a - b).abs() < 1e-9, "{label}: root {a} vs oracle {b}");
            assert!(big_f(point(pose, *a)).abs() < 1e-9, "{label}: F at {a}");
        }
    }

    /// Midplane, off-axis: the carrier crosses the OUTER equator twice
    /// and the INNER equator twice — the inner-equator region, four
    /// roots, which a two-root reading would halve.
    #[test]
    fn a_midplane_circle_through_both_equators_has_four_roots() {
        let pose = Pose {
            c: [1.0, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 1.0,
            u: [1.0, 0.0, 0.0],
        };
        assert_matches_oracle("full turn", pose, -3.0, 3.0, 4);
        // An arc holding only an inner-equator crossing (`ρ = 2|cos θ/2|`
        // meets `R − r` at `θ ≈ 2.37`, the outer equator at `≈ 1.79`).
        let (_, ts) = in_arc(pose, 2.0, 3.0);
        assert_eq!(ts.len(), 1, "one inner-equator root on the arc");
        for t in &ts {
            let p = point(pose, *t);
            assert!(
                ((p.x * p.x + p.y * p.y).sqrt() - (R - RT)).abs() < 1e-9,
                "an inner-equator root at {t}"
            );
        }
    }

    /// A tilted, off-centre carrier: the generic pose, two roots on the
    /// arc and the ladder's Ferrari arm (no symmetry sets `q̂` to zero).
    #[test]
    fn a_tilted_arc_crossing_the_tube_twice_matches_the_oracle() {
        let s = 0.5_f64.sqrt();
        let pose = Pose {
            c: [1.3, 0.1, 0.05],
            n: [0.0, s, s],
            rho: 0.4,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -3.1, 3.1);
        assert!(!all.is_empty(), "the pose must cross the tube");
        assert_matches_oracle("tilted", pose, -3.1, 3.1, all.len());
        // A short arc holding exactly the FIRST crossing: the door
        // reports the whole turn's count, the arc's filter keeps one.
        let (a, b) = (all[0] - 0.05, all[0] + 0.05);
        assert_matches_oracle("tilted short arc", pose, a, b, all.len());
    }

    /// A circle in a meridian plane, crossing the tube's cross-section
    /// twice: the parallel-to-axis family, with its `h` a pure harmonic.
    #[test]
    fn a_meridian_plane_circle_crosses_the_tube_twice() {
        let pose = Pose {
            c: [1.4, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: 0.3,
            u: [1.0, 0.0, 0.0],
        };
        assert_matches_oracle("meridian", pose, -3.0, 3.0, 2);
    }

    /// Tangent to the outer equator from outside: a double root, which
    /// the ladder must refuse rather than read as two crossings or none.
    #[test]
    fn a_tangent_circle_is_uncertain() {
        let pose = Pose {
            c: [2.0, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 2.0 - (R + RT),
            u: [1.0, 0.0, 0.0],
        };
        assert!(
            matches!(door(pose, 2.0, 4.0), CircleRoots::Uncertain),
            "a graze is not a certified count"
        );
    }

    /// **A near-coaxial carrier whose in-band offset and tilt carry its
    /// residual across zero is never a miss**, at any admissible `K`.
    /// The circle is coaxial to within `0.99·zero` of offset and of tilt,
    /// at a tube distance `δ = −8.2e-10` inside, and `θ = 0` is turned to
    /// where its residual is lowest, past the escalation threshold at
    /// `K = 1.5`; elsewhere the residual is positive, so the carrier
    /// crosses the tube. Escalating or answering `OnSurface` or
    /// `Uncertain` is sound; `Miss` is not.
    #[test]
    fn a_near_coaxial_carrier_spanning_zero_is_not_a_miss() {
        let zero = 1e-9;
        let delta = -8.2e-10;
        let (offset, tilt) = (0.99 * zero, 0.99 * zero);
        let (phi, psi) = (3.338_f64, 10f64.to_radians());
        let h0 = (RT + delta) / 2f64.sqrt();
        let rho = R + h0;
        let c = Point3::new(offset * phi.cos(), offset * phi.sin(), h0);
        let beta = tilt / rho;
        let n = Vec3::new(beta.sin() * psi.cos(), beta.sin() * psi.sin(), beta.cos());
        let u0 = n.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let v0 = n.cross(u0);
        let surface = torus::<f64>();
        let at = |u: Vec3<f64>, t: f64| {
            let v = n.cross(u);
            geom_brep::implicit_residual(&surface, c + (u * t.cos() + v * t.sin()) * rho)
        };
        let samples: Vec<f64> = (0..3600)
            .map(|i| f64::from(i) * core::f64::consts::PI / 1800.0)
            .collect();
        let t_min = samples
            .iter()
            .copied()
            .min_by(|a, b| at(u0, *a).total_cmp(&at(u0, *b)))
            .unwrap();
        let u = u0 * t_min.cos() + v0 * t_min.sin();
        let high = samples.iter().map(|t| at(u, *t)).fold(f64::MIN, f64::max);
        assert!(
            at(u, 0.0) < -1.5 * zero && high > 0.0,
            "the pose's premise: read at θ = 0 past 1.5·zero, positive elsewhere: \
             {} .. {high}",
            at(u, 0.0)
        );
        for k in [1.5, 1.2, 2.0, 2.5, 3.0, 10.0] {
            let got = roots_of(
                c,
                n,
                rho,
                u,
                0.0,
                1.0,
                &surface,
                Band::new(zero, k * zero).unwrap(),
            );
            assert!(
                !matches!(got, Ok(CircleRoots::Miss)),
                "K = {k}: a carrier crossing the tube certified a miss"
            );
        }
    }

    /// Coaxial with the torus: constant residual, answered before the
    /// quartic (whose complex double pair it cannot answer) by that
    /// residual's sign: a carrier clear of the tube is a miss, and one
    /// lying ON it (the outer equator) is on the surface.
    #[test]
    fn a_coaxial_circle_is_decided_by_its_constant_residual() {
        for (z, rho, on) in [(0.1, 3.0, false), (0.0, R + RT, true)] {
            let pose = Pose {
                c: [0.0, 0.0, z],
                n: [0.0, 0.0, 1.0],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            let got = door(pose, 0.0, 1.0);
            assert!(
                if on {
                    matches!(got, CircleRoots::OnSurface)
                } else {
                    matches!(got, CircleRoots::Miss)
                },
                "coaxial at z {z}, radius {rho}: {got:?}"
            );
        }
    }

    /// A carrier ON the torus but not coaxial (a meridian circle of the
    /// tube): `F ≡ 0`, so no pole is definite and the door refuses,
    /// which keeps a non-coaxial on-carrier circle from any recording
    /// arm.
    #[test]
    fn a_circle_lying_on_the_torus_is_uncertain() {
        let pose = Pose {
            c: [R, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: RT,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, 0.0, 1.0), CircleRoots::Uncertain));
    }

    /// Clear of the torus: a certified zero count.
    #[test]
    fn a_clear_circle_is_a_miss() {
        let pose = Pose {
            c: [4.0, 0.0, 0.3],
            n: [0.3_f64.sin(), 0.0, 0.3_f64.cos()],
            rho: 1.0,
            u: [0.3_f64.cos(), 0.0, -(0.3_f64.sin())],
        };
        assert!(matches!(door(pose, 0.0, 1.0), CircleRoots::Miss));
    }

    /// The pole lands ON the torus at the arc's antipode: the first
    /// anchor is refused and a shifted one answers, with the same roots.
    /// A tilted pose, so the quartic arm (not the parallel one) answers.
    #[test]
    fn a_pole_on_the_torus_moves_the_anchor() {
        let s = 0.5_f64.sqrt();
        let pose = Pose {
            c: [1.3, 0.1, 0.05],
            n: [0.0, s, s],
            rho: 0.4,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -3.1, 3.1);
        let mid = all[0] - core::f64::consts::PI;
        assert_matches_oracle("pole on the torus", pose, mid - 0.5, mid + 0.5, all.len());
    }

    /// A meridian-plane circle touching the tube's cross-section from
    /// outside: a tangency on the QUARTIC arm (the parallel arm's own
    /// graze is `a_tangent_circle_is_uncertain`).
    #[test]
    fn a_tangent_circle_off_the_parallel_pose_is_uncertain() {
        let pose = Pose {
            c: [R + 2.0 * RT, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: RT,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, 2.0, 4.0), CircleRoots::Uncertain));
    }

    /// The parallel pose passing a few millimetres INSIDE a contour's
    /// reach (the lily's arch seam): a definite miss of that contour at
    /// every band, where the quartic's discriminant would read the
    /// near-double complex pair as a graze.
    #[test]
    fn a_parallel_circle_just_short_of_a_contour_is_decided() {
        // Reaches ρ ≤ 0.5 + 0.742 = 1.242 < R + r = 1.25 (8 mm short),
        // and crosses the inner contour twice.
        let pose = Pose {
            c: [0.5, 0.0, 0.0],
            n: [0.0, 0.0, 1.0],
            rho: 0.742,
            u: [1.0, 0.0, 0.0],
        };
        for eps in [1e-9, 1e-6] {
            let got = roots_of(
                Point3::new(0.5, 0.0, 0.0),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -3.0,
                3.0,
                &torus(),
                Band::new(eps, 10.0 * eps).unwrap(),
            )
            .unwrap();
            let CircleRoots::Certified { count, .. } = got else {
                panic!("ε {eps}: a certified count, got {got:?}");
            };
            assert_eq!(count, 2, "ε {eps}: the inner contour only");
        }
        assert_matches_oracle("just short", pose, -3.0, 3.0, 2);
    }

    /// The interval lane: the same poses, and every certified root
    /// enclosure contains the oracle's root.
    #[test]
    fn the_interval_lane_encloses_the_oracle_roots() {
        let s = 0.5_f64.sqrt();
        for pose in [
            Pose {
                c: [1.0, 0.0, 0.0],
                n: [0.0, 0.0, 1.0],
                rho: 1.0,
                u: [1.0, 0.0, 0.0],
            },
            Pose {
                c: [1.3, 0.1, 0.05],
                n: [0.0, s, s],
                rho: 0.4,
                u: [1.0, 0.0, 0.0],
            },
        ] {
            let got = roots_of::<Interval>(
                Point3::new(
                    Interval::from_f64(pose.c[0]),
                    Interval::from_f64(pose.c[1]),
                    Interval::from_f64(pose.c[2]),
                ),
                v3(pose.n),
                Interval::from_f64(pose.rho),
                v3(pose.u),
                Interval::from_f64(-3.1),
                Interval::from_f64(3.1),
                &torus(),
                band(),
            )
            .unwrap();
            let CircleRoots::Certified { count, thetas } = got else {
                panic!("interval lane: expected a certified count");
            };
            let want = oracle(pose, -3.1, 3.1);
            assert_eq!(count, want.len(), "interval count");
            for w in want {
                assert!(
                    thetas[..count]
                        .iter()
                        .any(|t| t.lo() - 1e-12 <= w && w <= t.hi() + 1e-12),
                    "oracle root {w} in no enclosure {thetas:?}"
                );
            }
        }
    }

    /// **The pole next to a root off the arc** (review of PR 3375,
    /// executed). The arc's antipode — the first anchor's pole — sits
    /// `δ` from a root of the carrier that the arc does not hold, so
    /// `F(pole)` is non-zero but tiny and the monic quartic's
    /// coefficients blow up. Unconditioned, the ladder certified 2 roots
    /// with none in the arc where the arc holds 2. The door must answer
    /// the oracle's roots or refuse, never a wrong set; with the
    /// conditioning guard it moves the anchor and answers.
    #[test]
    fn a_pole_beside_an_off_arc_root_is_moved_not_trusted() {
        let n = {
            let k = (0.3_f64.powi(2) + 1.0).sqrt();
            [0.0, 0.3 / k, 1.0 / k]
        };
        let pose = Pose {
            c: [0.9, 0.0, 0.1],
            n,
            rho: 0.5,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -core::f64::consts::PI, core::f64::consts::PI);
        let root = *all
            .iter()
            .min_by(|a, b| (*a + 1.758).abs().total_cmp(&(*b + 1.758).abs()))
            .unwrap();
        assert!((root + 1.758).abs() < 1e-3, "the named root: {all:?}");
        for delta in [-1e-3, -1e-4, 1e-5, 1e-6, -1e-7] {
            let mid = root + delta - core::f64::consts::PI;
            let (t0, t1) = (mid - 1.0, mid + 1.0);
            assert_matches_oracle(&format!("δ = {delta}"), pose, t0, t1, all.len());
        }
    }

    /// **The parallel arm near the tube's top** (review of PR 3375,
    /// executed). The carrier's plane sits 1e-4 below the top circle
    /// and its far side reaches 2e-8 m past the outer contour IN THE
    /// PLANE — but near the top the surface is nearly flat, so the
    /// normal distance of that bump is far inside the band. The outer
    /// pair is not certifiable, and the in-plane arm certified four
    /// roots on it (the reviewer's oracle, on its pose, found two; this
    /// pose's `f64` oracle happens to resolve all four, which is not a
    /// certificate). At zero tilt and at a tilt of 9e-10 m, the door must
    /// refuse — `Uncertain`, or an escalation.
    #[test]
    fn a_parallel_bump_past_a_contour_inside_the_band_is_uncertain() {
        let h = RT - 1e-4;
        let outer = R + (RT * RT - h * h).sqrt();
        let d = 0.5;
        let rho = outer + 2e-8 - d;
        for tilt in [0.0, 9e-10] {
            let a = tilt / rho;
            let pose = Pose {
                c: [d, 0.0, h],
                n: [0.0, -a.sin(), a.cos()],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            // An in-band bump is a refusal either way: `Uncertain`, or
            // the band's own escalation when the margin lands in its gap.
            let got = roots_of(
                Point3::from_array(pose.c),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -3.0,
                3.0,
                &torus(),
                fixed_band(),
            );
            assert!(
                matches!(
                    &got,
                    Ok(CircleRoots::Uncertain)
                        | Err(BooleanError::Escalated {
                            decision: BooleanDecision::ArcTorusRoots,
                            diag: Indeterminate {
                                predicate: Some("bool_circle_torus_contour_residual"),
                                ..
                            },
                        })
                ),
                "tilt {tilt}: the in-band bump is not two crossings, got {got:?}"
            );
        }
    }
    /// **Circles lying ON the torus refuse, whatever their pose.** `F`
    /// is identically zero along them, so its coefficients are rounding
    /// noise and the conditioning ratio can read anything; the pole's
    /// own residual decision is what refuses them. Meridian circles at
    /// several azimuths and a Villarceau circle.
    #[test]
    fn circles_lying_on_the_torus_are_uncertain() {
        let mut poses = Vec::new();
        let tilt = (RT / R).asin();
        for k in 0..24 {
            let az = 0.261_799 * f64::from(k) + 0.1;
            let (s, c) = az.sin_cos();
            // A meridian circle of the tube.
            poses.push(Pose {
                c: [R * c, R * s, 0.0],
                n: [-s, c, 0.0],
                rho: RT,
                u: [c, s, 0.0],
            });
            // A Villarceau circle: radius `R`, centred `r` along the
            // midplane direction `az`, in the bitangent plane through
            // that direction tilted `asin(r/R)` off the midplane.
            let perp = [-s, c, 0.0];
            poses.push(Pose {
                c: [RT * c, RT * s, 0.0],
                n: [-tilt.sin() * perp[0], -tilt.sin() * perp[1], tilt.cos()],
                rho: R,
                u: [c, s, 0.0],
            });
        }
        for (i, pose) in poses.into_iter().enumerate() {
            for (t0, t1) in [(0.0, 1.0), (2.0, 4.5)] {
                assert!(
                    matches!(door(pose, t0, t1), CircleRoots::Uncertain),
                    "pose {i}, arc [{t0}, {t1}]: an on-torus circle is not a root set"
                );
            }
        }
    }

    /// **A nearly full arc whose antipode is ill conditioned.** The pole
    /// then moves INTO the arc, and roots near the far end of the arc
    /// come out of the half-angle map a whole turn away from it; they
    /// must be reported within `π` of the arc's midpoint or they are
    /// silently dropped from the arc.
    #[test]
    fn roots_are_reported_within_half_a_turn_of_the_arc() {
        // Two roots 0.04 rad apart (a shallow dip into the tube): with
        // the antipode beside one of them, the other sits just inside
        // the arc's far end, where the moved anchor's map puts it a
        // whole turn away.
        let n = {
            let k = (0.3_f64.powi(2) + 1.0).sqrt();
            [0.0, 0.3 / k, 1.0 / k]
        };
        let pose = Pose {
            c: [1.44, 0.0, 0.1],
            n,
            rho: 0.8,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -core::f64::consts::PI, core::f64::consts::PI);
        for &star in &all {
            for delta in [1e-4, -1e-4] {
                let mid = star + delta - core::f64::consts::PI;
                assert_matches_oracle(
                    &format!("antipode beside {star}, δ = {delta}"),
                    pose,
                    mid - 3.1,
                    mid + 3.1,
                    all.len(),
                );
            }
        }
    }

    /// **A parallel plane above the tube is a miss**, even where the
    /// carrier's projection spans the tube's centre circle: the plane's
    /// depth decides it before any contour does.
    #[test]
    fn a_parallel_circle_above_the_tube_is_a_miss() {
        let pose = Pose {
            c: [0.5, 0.0, RT + 0.05],
            n: [0.0, 0.0, 1.0],
            rho: 1.0,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, -3.0, 3.0), CircleRoots::Miss));
    }

    /// **The tilt a parallel pose was admitted with is charged.** The far
    /// point's residual is 1.05e-8 m — past the band's escalation
    /// threshold on its own — but the carrier is tilted 9e-10 m, and the
    /// charged margin falls back into the band: the crossing pair is not
    /// certified (a refusal: `Uncertain`, or the band's escalation).
    #[test]
    fn the_admitted_tilt_is_charged_against_the_bump() {
        let h = RT - 1e-4;
        let target = 1.05e-8;
        let rho_max = R + (RT * RT - h * h + 2.0 * RT * target).sqrt();
        let d = 0.5;
        let rho = rho_max - d;
        let a = 9e-10 / rho;
        let pose = Pose {
            c: [d, 0.0, h],
            n: [0.0, -a.sin(), a.cos()],
            rho,
            u: [1.0, 0.0, 0.0],
        };
        let got = roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            -3.0,
            3.0,
            &torus(),
            fixed_band(),
        );
        assert!(
            matches!(
                &got,
                Ok(CircleRoots::Uncertain)
                    | Err(BooleanError::Escalated {
                        decision: BooleanDecision::ArcTorusRoots,
                        diag: Indeterminate {
                            predicate: Some("bool_circle_torus_contour_residual"),
                            ..
                        },
                    })
            ),
            "the charged bump is inside the band: {got:?}"
        );
    }
    /// The linearized residual `(d² − r²)/2r` evaluated DIRECTLY at a
    /// point — no harmonics, no cancellation of `ρ⁴`-sized terms, so it
    /// is accurate to the point's own rounding (`~|p|·u`). The large-
    /// circle oracle's function.
    fn direct_res(p: Point3<f64>) -> f64 {
        let rho = (p.x * p.x + p.y * p.y).sqrt();
        ((rho - R).powi(2) + p.z * p.z - RT * RT) / (2.0 * RT)
    }

    /// Roots of the direct residual on `[t0, t1]`, bisected on a grid of
    /// `n` cells.
    fn direct_roots(pose: Pose, t0: f64, t1: f64, n: u32) -> Vec<f64> {
        let f = |t: f64| direct_res(point(pose, t));
        let mut out = Vec::new();
        for i in 0..n {
            let (mut a, mut b) = (
                t0 + (t1 - t0) * f64::from(i) / f64::from(n),
                t0 + (t1 - t0) * f64::from(i + 1) / f64::from(n),
            );
            if f(a) * f(b) >= 0.0 {
                continue;
            }
            for _ in 0..100 {
                let m = 0.5 * (a + b);
                if f(a) * f(m) <= 0.0 {
                    b = m;
                } else {
                    a = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        out
    }

    /// **A large circle's dip, certified away** (delta review of PR 3375,
    /// executed): `ρ = 100` dipping 9.8e-8 m into the tube at `θ = 0`.
    /// `F`'s coefficients are `~ρ⁴`, so its `f64` rounding is about
    /// `ρ⁴·u` — a residual noise near 1e-7 m, above the dip — and the
    /// unmetered door certified `Miss`. The noise meter must refuse
    /// instead (or answer the truth).
    #[test]
    fn a_large_circles_dip_below_its_own_noise_is_not_certified_away() {
        let pose = Pose {
            c: [
                -32.093_101_916_043_01,
                53.385_951_445_980_05,
                -77.850_260_585_465_68,
            ],
            n: [
                -0.899_931_609_984_986_3,
                -0.427_790_360_567_627_76,
                0.084_371_231_798_816_28,
            ],
            rho: 100.0,
            u: [
                0.328_284_194_818_169_7,
                -0.537_396_029_460_319_1,
                0.776_810_784_524_05,
            ],
        };
        let dip = direct_roots(pose, -0.01, 0.01, 20_000);
        assert_eq!(
            dip.len(),
            2,
            "the pose dips into the tube at θ = 0: {dip:?}"
        );
        // The dip is chosen against the default band, pinned: at a band
        // wider than 9.8e-8 m it is a graze the band calls touching.
        let got = roots_of(
            Point3::from_array(pose.c),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            0.5017,
            3.7658,
            &torus(),
            fixed_band(),
        )
        .unwrap();
        assert!(
            !matches!(got, CircleRoots::Miss),
            "a real dip is not a certified miss: {got:?}"
        );
        if let CircleRoots::Certified { count, .. } = got {
            assert_eq!(count, 2, "the whole turn holds the dip's two roots");
        }
        // The interval lane encloses the rounding the meter estimates,
        // so it never certifies the miss either.
        let got = roots_of::<Interval>(
            Point3::new(
                Interval::from_f64(pose.c[0]),
                Interval::from_f64(pose.c[1]),
                Interval::from_f64(pose.c[2]),
            ),
            v3(pose.n),
            Interval::from_f64(pose.rho),
            v3(pose.u),
            Interval::from_f64(0.5017),
            Interval::from_f64(3.7658),
            &torus(),
            fixed_band(),
        );
        assert!(
            !matches!(got, Ok(CircleRoots::Miss)),
            "interval lane: a real dip is not a certified miss: {got:?}"
        );
    }

    /// A circle of radius `rho` that touches the torus's outer equator at
    /// `P = (R + r)·(cos α, sin α, 0)` from outside, offset by `depth`
    /// along the outward normal (`depth < 0` dips into the tube), in the
    /// plane of the normal and the tube's vertical tangent — so near `P`
    /// it bends away from the torus and its extremum there IS `depth`.
    /// `P` is the carrier's `θ = 0`.
    fn grazing(rho: f64, alpha: f64, depth: f64) -> Pose {
        let (s, c) = alpha.sin_cos();
        let p = [(R + RT) * c, (R + RT) * s, 0.0];
        let k = rho + depth;
        // `u = −normal` (so `C(0) = P + depth·normal`), `v = +z` (the
        // tube's vertical tangent at `P`), `n = u × v`.
        Pose {
            c: [p[0] + k * c, p[1] + k * s, p[2]],
            n: [-s, c, 0.0],
            rho,
            u: [-c, -s, 0.0],
        }
    }

    /// **The ρ-sweep: no wrong certified answer at any radius** (delta
    /// review of PR 3375). At the default band, pinned (the depths are
    /// chosen against it: the shallowest just past its escalation
    /// threshold), circles grazing the tube with an extremum depth of
    /// ±{1.6e-8, 1e-7, 1e-6, 1e-5} m, at radii 10, 30, 100 and
    /// 300 m and four azimuths. Every answer the door gives must match
    /// the direct-residual oracle on the arc — a dip's two roots, or a
    /// clearance's none; `Uncertain` is always allowed. Unmetered, the
    /// door certified misses on dips and phantom pairs on clearances from
    /// `ρ = 100` up.
    #[test]
    fn no_wrong_certified_answer_across_circle_radii() {
        let mut answered = [0u32; 4];
        for (i, rho) in [10.0_f64, 30.0, 100.0, 300.0].into_iter().enumerate() {
            for alpha in [0.3_f64, 1.9, 3.4, 5.1] {
                for depth in [-1e-5, -1e-6, -1e-7, -1.6e-8, 1.6e-8, 1e-7, 1e-6, 1e-5] {
                    let pose = grazing(rho, alpha, depth);
                    let (t0, t1) = (-0.4, 0.6);
                    let truth = direct_roots(pose, t0, t1, 200_000);
                    assert_eq!(
                        truth.len(),
                        if depth < 0.0 { 2 } else { 0 },
                        "the fixture: ρ {rho}, α {alpha}, depth {depth}: {truth:?}"
                    );
                    let label = format!("ρ {rho}, α {alpha}, depth {depth}");
                    let got = roots_of(
                        Point3::from_array(pose.c),
                        v3(pose.n),
                        pose.rho,
                        v3(pose.u),
                        t0,
                        t1,
                        &torus(),
                        fixed_band(),
                    )
                    .unwrap();
                    match got {
                        CircleRoots::Uncertain => {}
                        CircleRoots::OnSurface => panic!("{label}: not on the torus"),
                        CircleRoots::CountDisagrees => panic!("{label}: counts disagree"),
                        CircleRoots::Miss => {
                            assert!(truth.is_empty(), "{label}: a certified miss on a dip");
                            answered[i] += 1;
                        }
                        CircleRoots::Certified { count, thetas } => {
                            let mut got: Vec<f64> = thetas[..count]
                                .iter()
                                .copied()
                                .filter(|t| (t0..=t1).contains(t))
                                .collect();
                            got.sort_by(f64::total_cmp);
                            assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
                            for (a, b) in got.iter().zip(&truth) {
                                // A certified root must be the truth's
                                // point to within the band's escalation
                                // threshold (the fixture's band): span and
                                // trim decide on it.
                                assert!(
                                    (a - b).abs() * rho < 1e-8,
                                    "{label}: root {a} vs {b}, {} m apart",
                                    (a - b).abs() * rho
                                );
                            }
                            answered[i] += 1;
                        }
                    }
                }
            }
        }
        println!("answered per radius (of 32): {answered:?}");
        assert!(
            answered[0] >= 16,
            "at ρ = 10 the door still answers: {answered:?}"
        );
    }
    /// **The admitted tilt moves the parallel arm's ROOTS, not just its
    /// margins** (delta review of PR 3375). Near the tube's top the
    /// carrier crosses the outer contour shallowly — a bump of 5e-8 m,
    /// definite on its own — so the residual's slope along the carrier at
    /// the roots is tiny, and a tilt of 9e-10 m moves them by tens of
    /// micrometres of arc. Untilted, the door answers the oracle; tilted,
    /// it must refuse rather than hand a caller points that far off.
    #[test]
    fn the_admitted_tilt_moves_the_parallel_roots() {
        let h = RT - 1e-4;
        let half = (RT * RT - h * h).sqrt();
        let x = 5e-8 * RT / half;
        let d = 0.5;
        let rho = R + half + x - d;
        for tilt in [0.0, 9e-10] {
            let a = tilt / rho;
            let pose = Pose {
                c: [d, 0.0, h],
                n: [0.0, -a.sin(), a.cos()],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            let got = roots_of(
                Point3::from_array(pose.c),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -0.5,
                0.5,
                &torus(),
                fixed_band(),
            );
            if tilt == 0.0 {
                let Ok(CircleRoots::Certified { count, thetas }) = got else {
                    panic!("untilted: a certified count, got {got:?}");
                };
                assert_eq!(count, 4, "untilted: both contours");
                let mut ts: Vec<f64> = thetas[..count]
                    .iter()
                    .copied()
                    .filter(|t| (-0.5..=0.5).contains(t))
                    .collect();
                ts.sort_by(f64::total_cmp);
                let want = direct_roots(pose, -0.5, 0.5, 200_000);
                assert_eq!(ts.len(), want.len(), "untilted: {ts:?} vs {want:?}");
                for (a, b) in ts.iter().zip(&want) {
                    assert!((a - b).abs() * rho < 1e-8, "untilted: root {a} vs {b}");
                }
            } else {
                assert!(
                    matches!(got, Ok(CircleRoots::Uncertain)),
                    "tilt {tilt}: roots displaced past the band are not certified: {got:?}"
                );
            }
        }
    }
}
