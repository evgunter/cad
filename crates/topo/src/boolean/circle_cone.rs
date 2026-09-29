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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]
mod tests {
    //! The door against an independent oracle: the cone's elevation
    //! `ρ cos α − |h| sin α` evaluated DIRECTLY at points of the carrier
    //! (no harmonics, no cancellation), sampled densely and each sign
    //! change bisected — simple roots only, which is the regime the door
    //! answers in. Each named pose reaches one of the lane's
    //! classifications, so a branch answered wrongly goes red on it.

    use super::*;
    use geom_core::{Bounds, Interval, Real, Tol};

    const S: f64 = core::f64::consts::FRAC_1_SQRT_2;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The default row's band, pinned: the rows that place a margin
    /// against the band's own thresholds are claims about THIS band.
    fn fixed_band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// The cone: apex at the origin, axis `+z`, half-angle `π/4`.
    fn cone<T: Real>() -> geom::Surface<T> {
        geom::Surface::Cone {
            apex: Point3::new(T::zero(), T::zero(), T::zero()),
            axis: Vec3::new(T::zero(), T::zero(), T::one()),
            half_angle: T::from_f64(core::f64::consts::FRAC_PI_4),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    /// A circle pose: centre, unit axis, radius, unit `u ⟂ axis`.
    #[derive(Clone, Copy, Debug)]
    struct Pose {
        c: [f64; 3],
        n: [f64; 3],
        rho: f64,
        u: [f64; 3],
    }

    fn v3<T: Real>(a: [f64; 3]) -> Vec3<T> {
        Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
    }

    fn point(pose: Pose, theta: f64) -> Point3<f64> {
        let (n, u) = (v3::<f64>(pose.n), v3::<f64>(pose.u));
        let v = n.cross(u);
        Point3::new(pose.c[0], pose.c[1], pose.c[2])
            + (u * theta.cos() + v * theta.sin()) * pose.rho
    }

    /// The oracle's function: the elevation, directly.
    fn elevation(p: Point3<f64>) -> f64 {
        p.x.hypot(p.y) * S - p.z.abs() * S
    }

    /// Roots of the elevation on `[t0, t1]`: sign changes on a grid of
    /// `n` cells, bisected.
    fn oracle_n(pose: Pose, t0: f64, t1: f64, n: u32) -> Vec<f64> {
        let f = |t: f64| elevation(point(pose, t));
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

    fn oracle(pose: Pose, t0: f64, t1: f64) -> Vec<f64> {
        oracle_n(pose, t0, t1, 20_000)
    }

    fn try_door_in(
        pose: Pose,
        t0: f64,
        t1: f64,
        band: Band,
    ) -> Result<CircleConeRoots<f64>, Indeterminate> {
        circle_cone_roots(
            Point3::new(pose.c[0], pose.c[1], pose.c[2]),
            v3(pose.n),
            pose.rho,
            v3(pose.u),
            t0,
            t1,
            &cone(),
            band,
        )
    }

    fn door(pose: Pose, t0: f64, t1: f64) -> CircleConeRoots<f64> {
        try_door_in(pose, t0, t1, band()).unwrap()
    }

    /// The door's roots that fall in `[t0, t1]`, sorted, with its count.
    fn in_arc(label: &str, got: CircleConeRoots<f64>, t0: f64, t1: f64) -> (usize, Vec<f64>) {
        let CircleConeRoots::Certified { count, thetas } = got else {
            panic!("{label}: expected a certified count, got {got:?}");
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
        let (count, ts) = in_arc(label, door(pose, t0, t1), t0, t1);
        let want = oracle(pose, t0, t1);
        assert_eq!(count, want_count, "{label}: certified count over the turn");
        assert_eq!(
            ts.len(),
            want.len(),
            "{label}: roots in the arc {ts:?} vs {want:?}"
        );
        for (a, b) in ts.iter().zip(&want) {
            assert!(
                (a - b).abs() * pose.rho < 1e-10,
                "{label}: root {a} vs oracle {b}"
            );
            assert!(
                elevation(point(pose, *a)).abs() < 1e-10,
                "{label}: on the cone at {a}"
            );
        }
    }

    /// A circle in a meridian plane, off the apex: it crosses both
    /// generators of the plane on both nappes — four roots, half of them
    /// on the mirror nappe, which a one-nappe reading would drop.
    #[test]
    fn a_meridian_circle_crosses_both_nappes_twice() {
        let pose = Pose {
            c: [0.1, 0.0, 0.05],
            n: [0.0, 1.0, 0.0],
            rho: 0.5,
            u: [1.0, 0.0, 0.0],
        };
        assert_eq!(oracle(pose, -3.1, 3.1).len(), 4, "the fixture");
        assert_matches_oracle("meridian", pose, -3.1, 3.1, 4);
        // An arc holding only the mirror nappe's crossings.
        let mirror: Vec<f64> = oracle(pose, -3.1, 3.1)
            .into_iter()
            .filter(|t| point(pose, *t).z < 0.0)
            .collect();
        assert_eq!(mirror.len(), 2, "two on the mirror nappe");
        assert_matches_oracle(
            "meridian, mirror arc",
            pose,
            mirror[0] - 0.1,
            mirror[1] + 0.1,
            4,
        );
    }

    /// A tilted, off-axis rim crossing one nappe twice: the generic pose
    /// and the ladder's Ferrari arm; then a short arc holding only the
    /// first crossing.
    #[test]
    fn a_tilted_rim_crossing_the_face_twice_matches_the_oracle() {
        let pose = Pose {
            c: [0.3, 0.1, 0.8],
            n: [0.0, S, S],
            rho: 0.4,
            u: [1.0, 0.0, 0.0],
        };
        let all = oracle(pose, -3.1, 3.1);
        assert_eq!(all.len(), 2, "the fixture: {all:?}");
        assert_matches_oracle("tilted", pose, -3.1, 3.1, 2);
        let (a, b) = (all[0] - 0.05, all[0] + 0.05);
        assert_matches_oracle("tilted short arc", pose, a, b, 2);
    }

    /// A tilted rim clear of the cone: a certified miss.
    #[test]
    fn a_clear_tilted_rim_is_a_miss() {
        let pose = Pose {
            c: [1.5, 0.2, 0.4],
            n: [0.0, S, S],
            rho: 0.3,
            u: [1.0, 0.0, 0.0],
        };
        assert!(oracle(pose, -3.1, 3.1).is_empty(), "the fixture");
        assert!(matches!(door(pose, -3.1, 3.1), CircleConeRoots::Miss));
    }

    /// **The coaxial pose**: inside the solid, outside it, on the mirror
    /// nappe's inside — clear; a parallel lying ON the cone — the door;
    /// an offset in the band's escalation gap — the door. The roots door
    /// answers `Coaxial` for all of them, whose constant residual only
    /// [`coaxial_pose`] reads.
    #[test]
    fn coaxial_rims_are_clear_and_a_parallel_on_the_cone_keeps_the_door() {
        let at = |x: f64, h: f64, rho: f64| Pose {
            c: [x, 0.0, h],
            n: [0.0, 0.0, 1.0],
            rho,
            u: [1.0, 0.0, 0.0],
        };
        for (label, pose, want) in [
            ("inside", at(0.0, 0.8, 0.3), Coaxial::Clear),
            ("outside", at(0.0, 0.5, 1.0), Coaxial::Clear),
            ("mirror inside", at(0.0, -0.5, 0.3), Coaxial::Clear),
            ("on the cone", at(0.0, 0.5, 0.5), Coaxial::Door),
            ("on the mirror nappe", at(0.0, -0.3, 0.3), Coaxial::Door),
            ("offset in the gap", at(5e-9, 0.8, 0.3), Coaxial::Door),
        ] {
            let got = coaxial_pose(
                Point3::new(pose.c[0], pose.c[1], pose.c[2]),
                v3(pose.n),
                pose.rho,
                &cone(),
                fixed_band(),
            );
            assert_eq!(got, Ok(want), "{label}");
            if pose.c[0] == 0.0 {
                assert!(
                    matches!(
                        try_door_in(pose, -3.0, 3.0, fixed_band()),
                        Ok(CircleConeRoots::Coaxial)
                    ),
                    "{label}: the roots door defers to the coaxial decision"
                );
            }
        }
    }

    /// **The parallel-axes pose, in closed form**: a rim in the plane
    /// `h = 0.35` (the parallel `ρ₀ = 0.35`), its distance from the axis
    /// sweeping `[0.2, 0.4]` — two roots; the same rim higher up, wholly
    /// inside — a miss; and the mirror nappe's image of both.
    #[test]
    fn a_parallel_axes_rim_crosses_the_parallel_in_closed_form() {
        for h in [0.35, -0.35] {
            let pose = Pose {
                c: [0.3, 0.0, h],
                n: [0.0, 0.0, 1.0],
                rho: 0.1,
                u: [1.0, 0.0, 0.0],
            };
            assert_matches_oracle(&format!("parallel at {h}"), pose, -3.1, 3.1, 2);
            let high = Pose {
                c: [0.3, 0.0, 2.0 * h],
                ..pose
            };
            assert!(oracle(high, -3.1, 3.1).is_empty(), "the fixture");
            assert!(
                matches!(door(high, -3.1, 3.1), CircleConeRoots::Miss),
                "a parallel rim inside the cone at {}",
                2.0 * h
            );
        }
    }

    /// **The lily trap, on the cone**: a parallel-axes rim a few
    /// millimetres short of the parallel is a definite miss at every
    /// band, and one a few millimetres past it crosses — where the
    /// quartic's exact complex pair `t = ±i` would read as a graze.
    #[test]
    fn a_parallel_rim_millimetres_from_the_parallel_is_decided() {
        for (rho, want) in [(0.195, 0), (0.205, 2)] {
            let pose = Pose {
                c: [0.3, 0.0, 0.5],
                n: [0.0, 0.0, 1.0],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            assert_eq!(oracle(pose, -3.1, 3.1).len(), want, "the fixture");
            for eps in [1e-9, 1e-6] {
                let got = try_door_in(pose, -3.1, 3.1, Band::new(eps, 10.0 * eps).unwrap());
                match (want, got) {
                    (0, Ok(CircleConeRoots::Miss)) => {}
                    (2, Ok(CircleConeRoots::Certified { count: 2, .. })) => {}
                    (_, got) => panic!("ρ {rho}, ε {eps}: expected {want} roots, got {got:?}"),
                }
            }
        }
    }

    /// **Tangency and the apex keep the door**: a parallel-axes rim
    /// touching the parallel from inside; a meridian-plane circle tangent
    /// to a generator; and circles through the apex, in an axis-normal
    /// plane and in a tilted one.
    #[test]
    fn tangent_rims_and_rims_through_the_apex_are_uncertain() {
        let r = 0.1;
        let apex_tilt = {
            // Centre `c`, radius `|c|`, axis `⟂ c`: through the origin.
            let c = [0.3, 0.0, 0.1];
            let k = (0.1_f64 * 0.1 + 0.25 + 0.09).sqrt();
            Pose {
                c,
                n: [-0.1 / k, 0.5 / k, 0.3 / k],
                rho: 0.1_f64.sqrt(),
                u: [0.3 / 0.1_f64.sqrt(), 0.0, 0.1 / 0.1_f64.sqrt()],
            }
        };
        for (label, pose) in [
            (
                "parallel, touching from inside",
                Pose {
                    c: [0.25, 0.0, 0.35],
                    n: [0.0, 0.0, 1.0],
                    rho: 0.1,
                    u: [1.0, 0.0, 0.0],
                },
            ),
            (
                "meridian, tangent to a generator",
                Pose {
                    c: [0.5 + r * S, 0.0, 0.5 - r * S],
                    n: [0.0, 1.0, 0.0],
                    rho: r,
                    u: [1.0, 0.0, 0.0],
                },
            ),
            (
                "axis-normal, through the apex",
                Pose {
                    c: [0.3, 0.0, 0.0],
                    n: [0.0, 0.0, 1.0],
                    rho: 0.3,
                    u: [1.0, 0.0, 0.0],
                },
            ),
            ("tilted, through the apex", apex_tilt),
        ] {
            let got = try_door_in(pose, -3.1, 3.1, band());
            assert!(
                matches!(got, Ok(CircleConeRoots::Uncertain) | Err(_)),
                "{label}: the door, got {got:?}"
            );
        }
    }

    /// **The pole on a root, or beside one, moves the anchor.** The
    /// meridian circle has four roots; the arc holds two of them, and
    /// its antipode — the first anchor's pole — sits `δ` from a third,
    /// down to exactly on it. A pole on a root gives the first anchor's
    /// quartic a vanishing leading coefficient, and one beside it a root
    /// near `t = ∞` whose monic coefficients blow up. The door must try
    /// another anchor and answer the oracle.
    #[test]
    fn a_pole_on_or_beside_a_root_moves_the_anchor() {
        let pose = Pose {
            c: [0.1, 0.0, 0.05],
            n: [0.0, 1.0, 0.0],
            rho: 0.5,
            u: [1.0, 0.0, 0.0],
        };
        let roots = oracle_n(pose, -3.1, 3.1, 200_000);
        assert_eq!(roots.len(), 4, "the fixture: {roots:?}");
        for delta in [0.0, -1e-3, -1e-4, 1e-5, 1e-6, -1e-7] {
            let mid = roots[2] + delta - core::f64::consts::PI;
            let (t0, t1) = (mid - 1.3, mid + 1.3);
            assert_eq!(
                oracle(pose, t0, t1).len(),
                2,
                "δ {delta}: the arc holds two"
            );
            assert_matches_oracle(&format!("pole δ {delta} from a root"), pose, t0, t1, 4);
        }
    }

    /// The interval lane: the certified enclosures contain the oracle's
    /// roots.
    #[test]
    fn the_interval_lane_encloses_the_oracle_roots() {
        for pose in [
            Pose {
                c: [0.1, 0.0, 0.05],
                n: [0.0, 1.0, 0.0],
                rho: 0.5,
                u: [1.0, 0.0, 0.0],
            },
            Pose {
                c: [0.3, 0.1, 0.8],
                n: [0.0, S, S],
                rho: 0.4,
                u: [1.0, 0.0, 0.0],
            },
        ] {
            let got = circle_cone_roots::<Interval>(
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
                &cone(),
                band(),
            )
            .unwrap();
            let CircleConeRoots::Certified { count, thetas } = got else {
                panic!("interval lane: expected a certified count, got {got:?}");
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

    /// A circle of radius `rho` in the meridian plane at azimuth `phi`,
    /// touching the upper nappe's generator at slant `10` from outside,
    /// offset by `depth` along the outward normal (`depth < 0` dips in):
    /// near the touch point `P` the generator is straight and the circle
    /// bends away, so its extremum there IS `depth`. `P` is `θ = 0`.
    fn grazing(rho: f64, phi: f64, depth: f64) -> Pose {
        let (sp, cp) = phi.sin_cos();
        let p = [10.0 * cp * S, 10.0 * sp * S, 10.0 * S];
        let normal = [cp * S, sp * S, -S];
        let k = rho + depth;
        Pose {
            c: [
                p[0] + k * normal[0],
                p[1] + k * normal[1],
                p[2] + k * normal[2],
            ],
            // `u = −normal`, `v` the generator's direction: `n = u × v`.
            n: [sp, -cp, 0.0],
            rho,
            u: [-normal[0], -normal[1], -normal[2]],
        }
    }

    /// **The ρ/scale sweep: no wrong certified answer at any radius.** At
    /// the default band, pinned (the depths are against it), circles
    /// grazing the face with an extremum depth of
    /// ±{1.6e-8, 1e-7, 1e-6, 1e-5} m, at radii from a millimetre to ten
    /// kilometres and four azimuths. Every answer must be the direct
    /// oracle's on the arc — a dip's two roots within the band's
    /// escalation threshold, or a clearance's none; `Uncertain` and an
    /// escalation are always allowed. `Q`'s terms grow as `ρ²` and its
    /// floor per metre of elevation as the apex's distance from the
    /// carrier (here about `10`). Measured, the door answers all 32 up to
    /// `ρ = 1`, 24 at 30 m and 7 at 300 m (the shallow grazes refuse on
    /// their root slack), and none from 3 km; with the noise charged as
    /// zero it certifies wrong answers from 3 km.
    #[test]
    fn no_wrong_certified_answer_across_circle_radii() {
        let radii = [1e-3, 1.0, 30.0, 300.0, 3000.0, 1e4];
        let mut answered = [0u32; 6];
        let mut wrong = Vec::new();
        for (i, rho) in radii.into_iter().enumerate() {
            let w = 3.0 * (2e-5 / rho).sqrt() + 1e-3 / rho.max(1.0);
            let (t0, t1) = (-w, 1.2 * w);
            for phi in [0.3_f64, 1.9, 3.4, 5.1] {
                for depth in [-1e-5, -1e-6, -1e-7, -1.6e-8, 1.6e-8, 1e-7, 1e-6, 1e-5] {
                    let pose = grazing(rho, phi, depth);
                    let truth = oracle_n(pose, t0, t1, 8_000);
                    assert_eq!(
                        truth.len(),
                        if depth < 0.0 { 2 } else { 0 },
                        "the fixture: ρ {rho}, φ {phi}, depth {depth}: {truth:?}"
                    );
                    let label = format!("ρ {rho}, φ {phi}, depth {depth}");
                    match try_door_in(pose, t0, t1, fixed_band()) {
                        Err(_) | Ok(CircleConeRoots::Uncertain) => {}
                        Ok(CircleConeRoots::Coaxial) => panic!("{label}: not coaxial"),
                        Ok(CircleConeRoots::Miss) => {
                            answered[i] += 1;
                            if !truth.is_empty() {
                                wrong.push(format!("{label}: a certified miss on a dip"));
                            }
                        }
                        Ok(CircleConeRoots::Certified { count, thetas }) => {
                            answered[i] += 1;
                            let mut got: Vec<f64> = thetas[..count]
                                .iter()
                                .copied()
                                .filter(|t| (t0..=t1).contains(t))
                                .collect();
                            got.sort_by(f64::total_cmp);
                            if got.len() != truth.len() {
                                wrong.push(format!("{label}: {got:?} vs {truth:?}"));
                                continue;
                            }
                            for (a, b) in got.iter().zip(&truth) {
                                if (a - b).abs() * rho >= 1e-8 {
                                    wrong.push(format!("{label}: root {a} vs {b}"));
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("answered per radius (of 32): {answered:?}");
        assert!(wrong.is_empty(), "wrong certified answers: {wrong:#?}");
        assert!(
            answered[..3].iter().all(|&a| a >= 24),
            "up to ρ = 30 every depth past the band answers: {answered:?}"
        );
    }

    /// The parallel-axes pose near a shallow crossing: centre offset
    /// `0.2` at height `0.4` (the parallel `ρ₀ = 0.4`), its far extreme
    /// `bump` metres of elevation outside the parallel, its axis tilted
    /// by `tilt` metres (about `x`, so the far extreme stays at `θ = 0`).
    fn shallow(bump: f64, tilt: f64) -> Pose {
        let (h0, e) = (0.4, 0.2);
        let rho = h0 + bump / S - e;
        let a = tilt / rho;
        Pose {
            c: [e, 0.0, h0],
            n: [0.0, -a.sin(), a.cos()],
            rho,
            u: [1.0, 0.0, 0.0],
        }
    }

    /// **The admitted tilt is charged against the extreme.** A parallel
    /// rim whose far extreme is 1.05e-8 m of elevation outside the
    /// parallel — just past the pinned band's escalation threshold — is
    /// admitted as parallel with a tilt of 9e-10 m, which moves the
    /// elevation by up to `2·tilt·(cos α + sin α)`. Charged, the margin
    /// falls back into the band: not certified.
    #[test]
    fn the_admitted_tilt_is_charged_against_the_extreme() {
        let pose = shallow(1.05e-8, 9e-10);
        let got = try_door_in(pose, -3.0, 3.0, fixed_band());
        assert!(
            matches!(
                &got,
                Ok(CircleConeRoots::Uncertain)
                    | Err(Indeterminate {
                        predicate: Some("bool_circle_cone_extreme"),
                        ..
                    })
            ),
            "the charged extreme is inside the band: {got:?}"
        );
    }

    /// **The admitted tilt moves the parallel arm's ROOTS.** A far
    /// extreme 5e-8 m outside the parallel is definite on its own, but
    /// the elevation's slope along the carrier at the roots is tiny, and
    /// a tilt of 9e-10 m moves them by micrometres of arc. Untilted, the
    /// door answers the oracle; tilted, it must refuse rather than hand a
    /// caller points that far off.
    #[test]
    fn the_admitted_tilt_moves_the_parallel_roots() {
        for tilt in [0.0, 9e-10] {
            let pose = shallow(5e-8, tilt);
            let got = try_door_in(pose, -0.5, 0.5, fixed_band());
            if tilt == 0.0 {
                let (count, ts) = in_arc("untilted", got.unwrap(), -0.5, 0.5);
                assert_eq!(count, 2, "untilted: the far extreme's pair");
                let want = oracle_n(pose, -0.5, 0.5, 200_000);
                assert_eq!(ts.len(), want.len(), "untilted: {ts:?} vs {want:?}");
                for (a, b) in ts.iter().zip(&want) {
                    assert!((a - b).abs() * pose.rho < 1e-8, "untilted: root {a} vs {b}");
                }
            } else {
                assert!(
                    matches!(got, Ok(CircleConeRoots::Uncertain)),
                    "tilt {tilt}: roots displaced past the band are not certified: {got:?}"
                );
            }
        }
    }
}
