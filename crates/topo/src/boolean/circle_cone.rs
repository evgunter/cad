//! **The circle × cone root door**: the certified crossings of a CIRCLE
//! carrier with the double cone `(A, â, α)`, beside the circle × torus
//! door ([`super::circle_torus`]) whose machinery it reuses.
//!
//! # The polynomial is a quartic
//!
//! With `q(θ) = C(θ) − A` for the carrier
//! `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, the double cone is the zero set of
//! `Q = |q|² cos²α − (q·â)²`. Both `|q|²` and `h = q·â` are FIRST
//! harmonics in `θ`, so `Q` is a trigonometric polynomial of degree two,
//! and the tangent half-angle turns it into a quartic — Bézout's
//! `2 × 2 = 4`, none lost at the circular points. `Q` is
//! `f·(ρ cos α + |h| sin α)` for the elevation `f` of
//! [`geom_brep::cone_elevation`], and the second factor is positive off
//! the apex, so `Q` and the surface's residual share their sign
//! everywhere but at the apex, which is what
//! [`half_angle_roots`] asks of a residual.
//!
//! # The machinery, and the cone's inputs to it
//!
//! [`half_angle_roots`] carries the anchor search, the pole decision on
//! the residual (`bool_circle_cone_pole`), the pole's conditioning, the
//! noise meter and the root slack; its docs state each certificate. The
//! cone supplies three things:
//!
//! - **The harmonics** of `Q`.
//! - **The lever**: the carrier's own `2ρ`. A torus caps it at its
//!   extent, because a circle larger than the torus meets it only where
//!   it passes through that extent. The double cone is unbounded, so a
//!   carrier meets it anywhere along its turn and its roots spread over
//!   its own size; the face's slant extent bounds where the FACE is,
//!   not where the carrier's roots are.
//! - **The noise meter's floor**, `f_per_metre`: `|Q| / |f|` is
//!   `ρ cos α + |h| sin α = |q|·sin(ψ + α)` with `ψ ∈ [0, π/2]` the
//!   point's angle from the axis, so it is at least `|q|·min(sin α,
//!   cos α)`, and `|q|` is at least the apex's distance from the
//!   carrier. The floor vanishes on a carrier through the apex, where
//!   the meter then refuses: that circle's `Q` has a double root at the
//!   apex, and the door holds either way.
//!
//! # The special poses, decided first and geometrically, in metres
//!
//! - **Coaxial** (the carrier's axis parallel to `â`, its centre on the
//!   cone's axis): `Q` is constant. [`coaxial_pose`] decides it, and it
//!   decides the constant elevation too, with the admitted tilt and
//!   offset charged: definitely off the carrier is clear, and a parallel
//!   ON the cone is the door. The circle rung reads it before any root
//!   arm, since the ladder would read the constant as a tangency.
//! - **Parallel axes** (the carrier's plane perpendicular to `â`, its
//!   centre off the axis): `h ≡ h₀`, and the plane meets the double cone
//!   in one parallel of radius `|h₀| tan α`, so the crossings are circle
//!   × circle in that plane, in closed form ([`parallel_axes_roots`]).
//!   The quartic would carry an exact complex pair here — `Q` is a first
//!   harmonic — which is the instrument trap the circle × torus lane
//!   measured on the lily, so the closed form is used instead.
//! - **A circle ON the cone** is only ever a parallel, which is coaxial:
//!   the planes that cut a right circular cone in a circle are the
//!   axis-normal ones.
//! - **A circle through the apex** has a double root there. The noise
//!   meter's floor refuses it, and the ladder would read the double root
//!   as `Uncertain` anyway.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::circle_torus::{
    HalfAngleFrame, HalfAngleRoots, HalfAngleRows, Harmonics, NOISE_ULPS, half_angle_roots,
};
use super::solid_contain::QuarticRows;
use crate::validate::decide;

/// The circle × cone lane's ladder rows.
const CIRCLE_CONE_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_cone_pole",
    conditioning: "bool_circle_cone_pole_conditioning",
    noise: "bool_circle_cone_noise",
    root_slack: "bool_circle_cone_root_slack",
    quartic: QuarticRows {
        disc: "bool_circle_cone_disc",
        shape: "bool_circle_cone_shape",
        depth: "bool_circle_cone_depth",
        odd: "bool_circle_cone_odd",
        split: "bool_circle_cone_split",
        split_lead: "bool_circle_cone_split_lead",
        count: "bool_circle_cone_count",
    },
};

/// What the certified circle × cone roots say about a whole carrier.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleConeRoots<T> {
    /// The carrier is coaxial with the cone: its residual is constant,
    /// and [`coaxial_pose`] is what decides it.
    Coaxial,
    /// A certified count of zero: the carrier misses the double cone.
    Miss,
    /// No certain count: a tangency, a carrier through the apex, a
    /// near-coaxial pose, a crossing whose bump is inside the band, or no
    /// anchor whose pole is definitely off the cone and well conditioned.
    Uncertain,
    /// A certified count (2 or 4) and the carrier parameters `θ` of those
    /// roots, unordered, in `thetas[..count]`, each within `π` of the
    /// arc's midpoint.
    Certified { count: usize, thetas: [T; 4] },
}

/// The carrier's pose against the cone's axis, read in metres: the tilt
/// of its axis times its radius (the amplitude of its height about
/// `â`), and its centre's offset from the cone's axis.
struct AxisPose<T: geom_core::Real> {
    /// `ρ·|n̂ × â|`.
    tilt: T,
    /// The centre's height above the apex along `â`.
    h0: T,
    /// The centre's offset from the axis, perpendicular to it, and its
    /// length.
    w_perp: Vec3<T>,
    offset: T,
}

fn axis_pose<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    apex: Point3<T>,
    c_axis: Vec3<T>,
) -> AxisPose<T> {
    let w0 = center - apex;
    let h0 = w0.dot(c_axis);
    let w_perp = w0 - c_axis * h0;
    AxisPose {
        tilt: radius * axis.cross(c_axis).norm(),
        h0,
        offset: w_perp.norm(),
        w_perp,
    }
}

/// What [`coaxial_pose`] decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Coaxial {
    /// Not a coaxial pose: the roots decide.
    No,
    /// Coaxial, and definitely off the carrier all the way round: the
    /// circle never meets the double cone.
    Clear,
    /// Coaxial and ON the carrier (a parallel of the cone), or a pose
    /// neither coaxial nor definitely not: the door.
    Door,
}

/// **The coaxial pose, and its constant elevation, decided in metres.**
///
/// The pose is coaxial when the tilt `ρ|n̂ × â|` and the centre's
/// offset from the axis are both in the zero band
/// (`bool_circle_cone_coaxial_tilt`, `bool_circle_cone_coaxial_offset`).
/// An offset in the escalation gap is neither pose, and is the door.
///
/// The elevation of the untilted coaxial circle is
/// `ρ cos α − |h₀| sin α`. The admitted pose moves every point's height
/// by at most the tilt, and its distance from the axis by at most
/// `offset + tilt²/ρ` (the tilted circle's projection is at least
/// `ρ√(1 − (tilt/ρ)²)` from its centre), so the true elevation is within
/// `cos α·(offset + tilt²/ρ) + sin α·tilt` of it all the way round.
/// `bool_circle_cone_coaxial_elevation` must put the elevation
/// definitely clear of zero with that charged, on either side, or the
/// circle may touch the carrier: the door.
///
/// # Errors
///
/// [`Indeterminate`] — an in-band elevation.
pub(super) fn coaxial_pose<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    cone: &geom::Surface<T>,
    band: Band,
) -> Result<Coaxial, Indeterminate> {
    let geom::Surface::Cone {
        apex,
        axis: c_axis,
        half_angle,
        ..
    } = *cone
    else {
        return Ok(Coaxial::No);
    };
    let pose = axis_pose(center, axis, radius, apex, c_axis);
    if !matches!(
        decide("bool_circle_cone_coaxial_tilt", Margin::of(pose.tilt), band),
        Ok(Sign::Zero)
    ) {
        return Ok(Coaxial::No);
    }
    match decide(
        "bool_circle_cone_coaxial_offset",
        Margin::of(pose.offset),
        band,
    ) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive) => return Ok(Coaxial::No),
        Ok(Sign::Negative) | Err(_) => return Ok(Coaxial::Door),
    }
    let (sin_a, cos_a) = half_angle.sin_cos();
    let elevation = radius * cos_a - pose.h0.abs() * sin_a;
    let charge = cos_a * (pose.offset + pose.tilt.powi(2) / radius) + sin_a * pose.tilt;
    let outside = decide(
        "bool_circle_cone_coaxial_elevation",
        Margin::of(elevation - charge),
        band,
    )?;
    let inside = decide(
        "bool_circle_cone_coaxial_elevation",
        Margin::of(elevation + charge),
        band,
    )?;
    Ok(if outside == Sign::Positive || inside == Sign::Negative {
        Coaxial::Clear
    } else {
        Coaxial::Door
    })
}

/// The certified crossings of the circle carrier
/// `center + radius·(u_ref cos θ + (axis × u_ref) sin θ)` with the
/// double cone `cone`, anchored so that the arc `[t0, t1]` avoids the
/// substitution's pole (module docs; [`half_angle_roots`]).
///
/// # Errors
///
/// [`Indeterminate`] — an in-band classifying sign in the ladder or in
/// the parallel-axes closed form. An in-band sign at the pose tests, at
/// a pole or at its conditioning is not an error: a near-coaxial pose is
/// `Uncertain`, a near-parallel one takes the quartic, and a pole tries
/// the next anchor.
#[allow(clippy::too_many_arguments)]
pub(super) fn circle_cone_roots<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    cone: &geom::Surface<T>,
    band: Band,
) -> Result<CircleConeRoots<T>, Indeterminate> {
    let geom::Surface::Cone {
        apex,
        axis: c_axis,
        half_angle,
        ..
    } = *cone
    else {
        return Ok(CircleConeRoots::Uncertain);
    };
    let two = T::from_f64(2.0);
    let v_ref = axis.cross(u_ref);
    let pose = axis_pose(center, axis, radius, apex, c_axis);
    let parallel = matches!(
        decide("bool_circle_cone_coaxial_tilt", Margin::of(pose.tilt), band),
        Ok(Sign::Zero)
    );
    if parallel {
        return match decide(
            "bool_circle_cone_coaxial_offset",
            Margin::of(pose.offset),
            band,
        ) {
            Ok(Sign::Zero) => Ok(CircleConeRoots::Coaxial),
            Ok(Sign::Positive) => parallel_axes_roots(
                &ParallelPose {
                    center,
                    radius,
                    u_ref,
                    v_ref,
                    pose,
                    t0,
                    t1,
                },
                cone,
                half_angle,
                band,
            ),
            Ok(Sign::Negative) | Err(_) => Ok(CircleConeRoots::Uncertain),
        };
    }

    // `Q` along the carrier as a degree-2 trigonometric polynomial about
    // `θ = 0`, from `|q|² = S₀ + S₁c cos θ + S₁s sin θ` and
    // `h = H₀ + H₁c cos θ + H₁s sin θ`, using
    // `(x cos θ + y sin θ)² = (x² + y²)/2 + (x² − y²)/2·cos 2θ + x y sin 2θ`.
    let half = T::from_f64(0.5);
    let (sin_a, cos_a) = half_angle.sin_cos();
    let cos2 = cos_a.powi(2);
    let w0 = center - apex;
    let s0 = w0.norm_squared() + radius.powi(2);
    let (s1c, s1s) = (two * radius * w0.dot(u_ref), two * radius * w0.dot(v_ref));
    let h0 = pose.h0;
    let (h1c, h1s) = (radius * u_ref.dot(c_axis), radius * v_ref.dot(c_axis));
    let harmonics = Harmonics {
        c0: cos2 * s0 - h0.powi(2) - (h1c.powi(2) + h1s.powi(2)) * half,
        c1: cos2 * s1c - two * h0 * h1c,
        s1: cos2 * s1s - two * h0 * h1s,
        c2: (h1s.powi(2) - h1c.powi(2)) * half,
        s2: T::zero() - h1c * h1s,
    };
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };
    // **The noise meter's inputs** (module docs): a bound on every term
    // the harmonics are built from, and the floor on `|Q|` per metre of
    // elevation over the carrier — the apex's distance from the carrier
    // circle, times `min(sin α, cos α)`.
    let h_abs = h0.abs() + h1c.abs() + h1s.abs();
    let terms = cos2 * (s0 + s1c.abs() + s1s.abs()) + h_abs.powi(2);
    let off_plane = w0.dot(axis);
    let in_plane = (w0 - axis * off_plane).norm();
    let apex_gap = (off_plane.powi(2) + (in_plane - radius).powi(2)).sqrt();
    let f_per_metre = apex_gap * sin_a.min(cos_a);
    Ok(
        match half_angle_roots(
            &harmonics,
            |theta| geom_brep::implicit_residual(cone, point_at(theta)),
            HalfAngleFrame {
                t0,
                t1,
                radius,
                lever: two * radius,
                noise: T::from_f64(NOISE_ULPS * f64::EPSILON * 0.5) * terms,
                f_per_metre,
            },
            &CIRCLE_CONE_ROWS,
            band,
        )? {
            HalfAngleRoots::Miss => CircleConeRoots::Miss,
            HalfAngleRoots::Uncertain => CircleConeRoots::Uncertain,
            HalfAngleRoots::Certified { count, thetas } => {
                CircleConeRoots::Certified { count, thetas }
            }
        },
    )
}

/// The parallel-axes pose's data ([`parallel_axes_roots`]).
struct ParallelPose<T: geom_core::Real> {
    center: Point3<T>,
    radius: T,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    pose: AxisPose<T>,
    /// The arc: roots are reported within `π` of its midpoint.
    t0: T,
    t1: T,
}

/// **The parallel-axes pose, in closed form, decided on the elevation.**
/// A carrier whose plane is perpendicular to `â` lies at height `h₀`,
/// where the double cone is the one parallel `ρ = |h₀| tan α`. The
/// carrier's distance from the cone's axis sweeps
/// `[|e − ρ_c|, e + ρ_c]` (`e` its centre's offset, `ρ_c` its radius)
/// monotonically on each half-turn about `θ₀`, its farthest point, so
/// it crosses the parallel — twice, at
/// `θ₀ ± acos((ρ₀² − e² − ρ_c²)/(2ρ_c·e))` — exactly when the parallel
/// lies strictly between those extremes.
///
/// **Every classifying margin is the elevation, not an in-plane reach.**
/// The elevation along the carrier is `ρ cos α − |h₀| sin α`, monotone
/// in `ρ`, so the far extreme is definitely outside and the near one
/// definitely inside exactly when the parallel lies strictly between
/// them — and an in-plane reach `x` past the parallel is an elevation of
/// only `x cos α`, which a flat cone makes small. Each extreme is placed
/// by its own elevation (`bool_circle_cone_extreme`): far outside and
/// near inside is two roots, both on one side is a miss, and anything
/// else is `Uncertain`.
///
/// **The tilt the pose was admitted with is carried, not dropped.** The
/// carrier's height varies by up to `tilt` about `h₀`, and its distance
/// from the axis differs from the untilted closed form's by at most
/// `tilt`, so the elevation differs by at most `tilt·(cos α + sin α)`.
/// Doubled, as the circle × torus arm does (an interior point is moved
/// both by the tilt at its own position and by the tilt's shift of
/// where the extremes fall), every extreme's margin is charged that.
/// The roots are placed as if untilted, and the same charge over the
/// elevation's slope there — `cos α·ρ_c·e·sin(spread)/ρ₀` per radian —
/// bounds how far the true ones lie: `bool_circle_cone_root_slack`
/// holds that arc length, plus the `acos` argument's own rounding, to
/// the band.
fn parallel_axes_roots<T: Decide>(
    pose: &ParallelPose<T>,
    cone: &geom::Surface<T>,
    half_angle: T,
    band: Band,
) -> Result<CircleConeRoots<T>, Indeterminate> {
    let &ParallelPose {
        center,
        radius,
        u_ref,
        v_ref,
        pose:
            AxisPose {
                tilt,
                h0,
                w_perp,
                offset,
            },
        t0,
        t1,
    } = pose;
    let two = T::from_f64(2.0);
    let (sin_a, cos_a) = half_angle.sin_cos();
    let charge = two * tilt * (cos_a + sin_a);
    // `θ₀`: the carrier's farthest point from the cone's axis, along
    // `+w_perp`.
    let theta0 = w_perp.dot(v_ref).atan2(w_perp.dot(u_ref));
    let elevation = |theta: T| {
        let (s, c) = theta.sin_cos();
        geom_brep::implicit_residual(cone, center + u_ref * (radius * c) + v_ref * (radius * s))
    };
    let side = |theta: T| -> Result<Option<Sign>, Indeterminate> {
        let res = elevation(theta);
        if decide("bool_circle_cone_extreme", Margin::of(res - charge), band)? == Sign::Positive {
            return Ok(Some(Sign::Positive));
        }
        if decide("bool_circle_cone_extreme", Margin::of(res + charge), band)? == Sign::Negative {
            return Ok(Some(Sign::Negative));
        }
        Ok(None)
    };
    let (Some(far), Some(near)) = (side(theta0)?, side(theta0 + T::pi())?) else {
        return Ok(CircleConeRoots::Uncertain);
    };
    match (far, near) {
        (Sign::Positive, Sign::Negative) => {}
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
            return Ok(CircleConeRoots::Miss);
        }
        // The far extreme inside and the near one outside contradicts
        // the monotone elevation: no answer.
        _ => return Ok(CircleConeRoots::Uncertain),
    }
    let rho0 = h0.abs() * half_angle.tan();
    // Inside `[-1, 1]` by the placements above; the clamp is the rounding
    // guard, not a decision.
    let c = ((rho0.powi(2) - offset.powi(2) - radius.powi(2)) / (two * radius * offset))
        .max(T::zero() - T::one())
        .min(T::one());
    let spread = c.acos();
    let sin_spread = (T::one() - c.powi(2)).max(T::zero()).sqrt();
    let slope = cos_a * radius * offset * sin_spread / rho0;
    let rounding = T::from_f64(NOISE_ULPS * f64::EPSILON * 0.5)
        * (rho0.powi(2) + offset.powi(2) + radius.powi(2))
        / (two * radius * offset);
    let slack = radius * (charge / slope + rounding / sin_spread);
    match decide("bool_circle_cone_root_slack", Margin::of(slack), band) {
        Ok(Sign::Positive) | Err(_) => return Ok(CircleConeRoots::Uncertain),
        Ok(Sign::Zero | Sign::Negative) => {}
    }
    let mid = (t0 + t1) / two;
    let mut thetas = [T::zero(); 4];
    for (slot, theta) in thetas.iter_mut().zip([theta0 + spread, theta0 - spread]) {
        *slot = mid + (theta - mid).reduce_periodic_centred(T::tau());
    }
    Ok(CircleConeRoots::Certified { count: 2, thetas })
}
