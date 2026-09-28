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
//! trigonometric polynomial of degree TWO. The tangent half-angle `t = tan(φ/2)` therefore turns
//! `F·(1 + t²)²` into a polynomial of degree FOUR in `t` — Bézout's
//! eight for a conic against a quartic surface loses four to the
//! circular points at infinity, through which both the circle and the
//! (bicircular) torus pass. At most four crossings per turn.
//!
//! # The substitution, and where its pole goes
//!
//! `φ = θ − θₐ` is measured from an anchor `θₐ`, and the half-angle map
//! covers every `θ` except its pole `θₐ + π`. The anchor is the arc's
//! midpoint, so the pole sits on the part of the carrier the arc does
//! NOT occupy (for a full turn it is the edge's own vertex). The
//! quartic's leading coefficient is `F` at the pole, so the pole is
//! required to be DEFINITELY off the torus — decided on the residual
//! there (`bool_circle_torus_pole`; on a ring torus `R > r`, `F` and
//! the linearized residual share their sign, since
//! `F = 2r·res·((ρ + R)² + h² − r²)` and the second factor exceeds
//! `R² − r² > 0`). That makes the division that normalizes the quartic
//! a division by a certified non-zero, and it certifies that the one
//! parameter the map cannot reach is not a root. A pole on the torus is
//! tried again at two other anchors still off the arc; if none is
//! definite the count is `Uncertain`.
//!
//! # Units, and the lever
//!
//! The root variable handed to the certified ladder is the LENGTH
//! `τ = 2ρ·t`, which is arc length to first order about the anchor, and
//! the ladder is metered with the torus's own extent `R + r` as its
//! lever — the line lane's choice, for the same reason: the crossings
//! that matter lie where the carrier passes through the torus's
//! extent. Roots far round the carrier (near the pole) have large `τ`,
//! which only enlarges the discriminant's margin.
//!
//! # The special poses
//!
//! - **Coaxial** (the carrier's axis parallel to `â` and its centre on
//!   the torus axis): `S` and `h` are constant, so `F` is constant and
//!   the quartic is `F·(1 + t²)²` — a repeated complex pair the ladder
//!   would read as a tangency. It is decided FIRST, geometrically
//!   (`bool_circle_torus_coaxial_tilt`, `_offset`, both metres), and
//!   answered [`CircleTorusRoots::Coaxial`]: the residual is constant
//!   along the carrier, and the caller decides what that means.
//! - **Parallel axes** (the circle's plane perpendicular to `â`, off
//!   the axis — the lily's pose): `h` is constant, the plane meets the
//!   tube in two contour circles, and the crossings are circle ×
//!   contour intersections in closed form, decided on LENGTHS
//!   ([`parallel_axes_roots`]). The quartic does not degenerate here,
//!   but its discriminant is the wrong instrument: it also measures the
//!   separation of the COMPLEX roots, so a carrier passing a few
//!   millimetres clear of a contour reads as a tangency at a coarse
//!   band (measured on the lily at ε = 1e-6: the arch's outer seam,
//!   8 mm inside the stem's outer contour).
//! - **The circle lies ON the torus** (a rim, meridian or Villarceau
//!   circle, none of them parallel-axes except the rims, which are
//!   coaxial): `F ≡ 0`, so the pole is on the torus at every anchor and
//!   the answer is `Uncertain` — which is what keeps an undeclared
//!   on-carrier circle away from every recording arm.
//! - **A tangency** — the carrier grazing the tube, a double root — is
//!   the ladder's own `Uncertain` (or a contour-reach margin in band),
//!   anywhere on the carrier.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::solid_contain::{QuarticRows, TorusRoots, depressed_quartic_roots};
use crate::validate::decide;

/// The circle × torus lane's ladder rows.
const CIRCLE_TORUS_ROWS: QuarticRows = QuarticRows {
    disc: "bool_circle_torus_disc",
    shape: "bool_circle_torus_shape",
    depth: "bool_circle_torus_depth",
    odd: "bool_circle_torus_odd",
    split: "bool_circle_torus_split",
    split_lead: "bool_circle_torus_split_lead",
    count: "bool_circle_torus_count",
};

/// What the certified circle × torus roots say about a whole carrier.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleTorusRoots<T> {
    /// The carrier is coaxial with the torus: its residual is constant.
    Coaxial,
    /// A certified count of zero: the carrier misses the torus.
    Miss,
    /// No certain count — a tangency, a carrier on the torus, or a pole
    /// that no anchor could put definitely off it.
    Uncertain,
    /// A certified count (2 or 4) and the carrier parameters `θ` of
    /// those roots, unordered, in `thetas[..count]`. Every `θ` lies in
    /// `(θₐ − π, θₐ + π)` for the anchor used, which contains the arc
    /// `[t₀, t₁]` the caller passed.
    Certified { count: usize, thetas: [T; 4] },
}

/// A quadratic in `t`, low coefficient first.
type Quad<T> = [T; 3];

fn quad_mul<T: geom_core::Real>(a: Quad<T>, b: Quad<T>) -> [T; 5] {
    [
        a[0] * b[0],
        a[0] * b[1] + a[1] * b[0],
        a[0] * b[2] + a[1] * b[1] + a[2] * b[0],
        a[1] * b[2] + a[2] * b[1],
        a[2] * b[2],
    ]
}

/// The certified crossings of the circle carrier
/// `center + radius·(u_ref cos θ + (axis × u_ref) sin θ)` with the torus
/// `(t_center, t_axis, major, minor)`, anchored so that the arc
/// `[t0, t1]` avoids the substitution's pole (module docs).
///
/// # Errors
///
/// [`Indeterminate`] — an in-band classifying sign in the ladder, or its
/// count cross-check. An in-band sign at the coaxial test or at a pole
/// is not an error: the former falls through to the general quartic
/// (which answers `Uncertain` where a near-coaxial carrier makes it
/// so) and the latter tries the next anchor.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)] // the quartic's textbook names
pub(super) fn circle_torus_roots<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    torus: &geom::Surface<T>,
    band: Band,
) -> Result<CircleTorusRoots<T>, Indeterminate> {
    let geom::Surface::Torus {
        center: t_center,
        axis: t_axis,
        major_radius,
        minor_radius,
        ..
    } = *torus
    else {
        return Ok(CircleTorusRoots::Uncertain);
    };
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let v_ref = axis.cross(u_ref);
    let w0 = center - t_center;

    // The coaxial pose, decided on the geometry in metres: the tilt of
    // the carrier's axis times its radius (how far the circle leaves a
    // plane perpendicular to `â`), and the carrier centre's distance
    // from the torus axis.
    let tilt = radius * axis.cross(t_axis).norm();
    let h0 = w0.dot(t_axis);
    let w_perp = w0 - t_axis * h0;
    let offset = w_perp.norm();
    let parallel = matches!(
        decide("bool_circle_torus_coaxial_tilt", Margin::of(tilt), band),
        Ok(Sign::Zero)
    );
    let mid = (t0 + t1) / two;
    if parallel {
        return match decide("bool_circle_torus_coaxial_offset", Margin::of(offset), band) {
            Ok(Sign::Zero) => Ok(CircleTorusRoots::Coaxial),
            Ok(Sign::Positive) => parallel_axes_roots(
                ParallelPose {
                    radius,
                    u_ref,
                    v_ref,
                    w_perp,
                    offset,
                    h0,
                    major_radius,
                    minor_radius,
                    mid,
                },
                band,
            ),
            // A negative length is not an answer; an in-band one is a
            // near-coaxial pose neither arm can stand behind.
            Ok(Sign::Negative) | Err(_) => Ok(CircleTorusRoots::Uncertain),
        };
    }

    // The pole's room off the arc: a quarter of the carrier the arc
    // does not occupy either side of the antipode. Zero for a full turn,
    // where every candidate is the same one.
    let room = (T::tau() - (t1 - t0)) / four;
    let rr = major_radius.powi(2);
    let k = rr - minor_radius.powi(2);
    let ext = major_radius + minor_radius;
    let lever_t = two * radius;
    for shift in [T::zero(), room, T::zero() - room] {
        let anchor = mid + shift;
        let (sa, ca) = anchor.sin_cos();
        let u = u_ref * ca + v_ref * sa;
        let v = v_ref * ca - u_ref * sa;
        // The pole `anchor + π` is the point `center − radius·u`.
        let pole = center - u * radius;
        match decide(
            "bool_circle_torus_pole",
            Margin::of(geom_brep::implicit_residual(torus, pole)),
            band,
        ) {
            Ok(Sign::Positive | Sign::Negative) => {}
            Ok(Sign::Zero) | Err(_) => continue,
        }
        // `S·(1 + t²)` and `h·(1 + t²)` as quadratics in `t`, from
        // `S = S₀ + S₁ cos φ + S₂ sin φ`, `h = H₀ + H₁ cos φ + H₂ sin φ`.
        let s0 = w0.norm_squared() + radius.powi(2);
        let s1 = two * radius * w0.dot(u);
        let s2 = two * radius * w0.dot(v);
        let h0 = w0.dot(t_axis);
        let h1 = radius * u.dot(t_axis);
        let h2 = radius * v.dot(t_axis);
        let sigma: Quad<T> = [s0 + s1, two * s2, s0 - s1];
        let eta: Quad<T> = [h0 + h1, two * h2, h0 - h1];
        let d: Quad<T> = [T::one(), T::zero(), T::one()];
        let sk: Quad<T> = [sigma[0] + k, sigma[1], sigma[2] + k];
        // `F·(1 + t²)² = (σ + K·D)² − 4R²(σ·D − η²)`.
        let sq = quad_mul(sk, sk);
        let sd = quad_mul(sigma, d);
        let ee = quad_mul(eta, eta);
        let four_rr = four * rr;
        let a: [T; 5] = core::array::from_fn(|i| sq[i] - four_rr * (sd[i] - ee[i]));
        // Monic in `τ = L·t` (a length): `τ⁴ + B τ³ + C τ² + D τ + E`,
        // with the leading coefficient the certified non-zero `F(pole)`.
        let lead = a[4];
        let b3 = lever_t * a[3] / lead;
        let c2 = lever_t.powi(2) * a[2] / lead;
        let d1 = lever_t.powi(3) * a[1] / lead;
        let e0 = lever_t.powi(4) * a[0] / lead;
        // Depress by `τ = y − B/4`.
        let shift_y = b3 / four;
        let p = c2 - T::from_f64(3.0) * b3.powi(2) / T::from_f64(8.0);
        let q = d1 - b3 * c2 / two + b3.powi(3) / T::from_f64(8.0);
        let s = e0 - b3 * d1 / four + b3.powi(2) * c2 / T::from_f64(16.0)
            - T::from_f64(3.0) * b3.powi(4) / T::from_f64(256.0);
        return Ok(
            match depressed_quartic_roots(p, q, s, ext, &CIRCLE_TORUS_ROWS, band)? {
                TorusRoots::Miss => CircleTorusRoots::Miss,
                TorusRoots::Uncertain => CircleTorusRoots::Uncertain,
                TorusRoots::Certified { count, ts: ys } => {
                    let mut thetas = [T::zero(); 4];
                    for (theta, y) in thetas.iter_mut().zip(ys).take(count) {
                        let tau = y - shift_y;
                        *theta = anchor + two * (tau / lever_t).atan();
                    }
                    CircleTorusRoots::Certified { count, thetas }
                }
            },
        );
    }
    Ok(CircleTorusRoots::Uncertain)
}

/// The parallel-axes pose's data ([`parallel_axes_roots`]).
struct ParallelPose<T: geom_core::Real> {
    radius: T,
    u_ref: Vec3<T>,
    v_ref: Vec3<T>,
    /// The carrier centre's offset from the torus axis, perpendicular
    /// to it (in the carrier's plane), and its length.
    w_perp: Vec3<T>,
    offset: T,
    /// The carrier plane's height along the torus axis.
    h0: T,
    major_radius: T,
    minor_radius: T,
    /// The arc's midpoint: roots are reported within `π` of it.
    mid: T,
}

/// **The parallel-axes pose, in closed form and in metres.** A carrier
/// whose axis is parallel to the torus's lies in a plane at constant
/// height `h₀`, which meets the tube in the two CONTOUR circles
/// `ρ± = R ± √(r² − h₀²)` about the torus axis. The carrier's distance
/// from that axis is `ρ(θ)² = d² + ρc² + 2ρc·d·cos(θ − θ₀)`, which
/// sweeps `[|ρc − d|, ρc + d]`, so the carrier crosses a contour
/// exactly when `|ρc − d| < ρ± < ρc + d` — two roots at
/// `θ₀ ± acos((ρ±² − d² − ρc²)/(2ρc·d))`.
///
/// Every decision is a LENGTH, and that is why the pose has its own arm
/// rather than the quartic's: the quartic's discriminant also measures
/// the COMPLEX roots' separation, so a carrier that passes a few
/// millimetres clear of a contour (a near-double complex pair, no
/// crossing at all) reads as a tangency once the band is coarse. Here
/// that pose is the definite length `ρc + d − ρ±`, and only a real
/// graze — a margin in band — is `Uncertain`:
///
/// - `r² − h₀²` over `r` (`bool_circle_torus_plane_height`): negative,
///   the plane misses the tube; in band, it touches the tube's top or
///   bottom circle; positive, two contours;
/// - per contour, `ρ± − (ρc − d)`, `ρ± − (d − ρc)` and `ρc + d − ρ±`
///   (`bool_circle_torus_contour_reach`): all positive, two crossings;
///   any negative, none; otherwise a graze.
fn parallel_axes_roots<T: Decide>(
    pose: ParallelPose<T>,
    band: Band,
) -> Result<CircleTorusRoots<T>, Indeterminate> {
    let ParallelPose {
        radius,
        u_ref,
        v_ref,
        w_perp,
        offset,
        h0,
        major_radius,
        minor_radius,
        mid,
    } = pose;
    let two = T::from_f64(2.0);
    let depth = minor_radius.powi(2) - h0.powi(2);
    match decide(
        "bool_circle_torus_plane_height",
        Margin::over_lever(depth, minor_radius),
        band,
    )? {
        Sign::Negative => return Ok(CircleTorusRoots::Miss),
        Sign::Zero => return Ok(CircleTorusRoots::Uncertain),
        Sign::Positive => {}
    }
    let half = depth.max(T::zero()).sqrt();
    // The direction of the torus axis's foot, seen from the carrier
    // centre, is `−w_perp`; `θ₀` is where the carrier is FARTHEST from
    // the axis, along `+w_perp`.
    let theta0 = w_perp.dot(v_ref).atan2(w_perp.dot(u_ref));
    let mut thetas = [T::zero(); 4];
    let mut count = 0usize;
    for contour in [major_radius + half, major_radius - half] {
        let mut signs = [Sign::Zero; 3];
        for (slot, reach) in signs.iter_mut().zip([
            contour - (radius - offset),
            contour - (offset - radius),
            radius + offset - contour,
        ]) {
            *slot = decide("bool_circle_torus_contour_reach", Margin::of(reach), band)?;
        }
        if signs.contains(&Sign::Negative) {
            continue;
        }
        if signs.contains(&Sign::Zero) {
            return Ok(CircleTorusRoots::Uncertain);
        }
        // Inside `[-1, 1]` by the three decisions above; the clamp is
        // the rounding guard, not a decision.
        let c = ((contour.powi(2) - offset.powi(2) - radius.powi(2)) / (two * radius * offset))
            .max(T::zero() - T::one())
            .min(T::one());
        let spread = c.acos();
        for theta in [theta0 + spread, theta0 - spread] {
            thetas[count] = mid + (theta - mid).reduce_periodic_centred(T::tau());
            count += 1;
        }
    }
    Ok(if count == 0 {
        CircleTorusRoots::Miss
    } else {
        CircleTorusRoots::Certified { count, thetas }
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

    const R: f64 = 1.0;
    const RT: f64 = 0.25;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
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
        Point3::new(pose.c[0], pose.c[1], pose.c[2])
            + (u * theta.cos() + v * theta.sin()) * pose.rho
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

    fn door(pose: Pose, t0: f64, t1: f64) -> CircleTorusRoots<f64> {
        circle_torus_roots(
            Point3::new(pose.c[0], pose.c[1], pose.c[2]),
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
        let CircleTorusRoots::Certified { count, thetas } = door(pose, t0, t1) else {
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
            matches!(door(pose, 2.0, 4.0), CircleTorusRoots::Uncertain),
            "a graze is not a certified count"
        );
    }

    /// Coaxial with the torus: constant residual, answered before the
    /// quartic (whose complex double pair would read as a tangency).
    /// Both a carrier clear of the tube and one lying ON it (the outer
    /// equator) answer `Coaxial`.
    #[test]
    fn a_coaxial_circle_is_answered_coaxial() {
        for (z, rho) in [(0.1, 3.0), (0.0, R + RT)] {
            let pose = Pose {
                c: [0.0, 0.0, z],
                n: [0.0, 0.0, 1.0],
                rho,
                u: [1.0, 0.0, 0.0],
            };
            assert!(
                matches!(door(pose, 0.0, 1.0), CircleTorusRoots::Coaxial),
                "coaxial at z {z}, radius {rho}"
            );
        }
    }

    /// A carrier ON the torus but not coaxial (a meridian circle of the
    /// tube): `F ≡ 0`, so no pole is definite and the door refuses —
    /// what keeps an on-carrier circle from any recording arm.
    #[test]
    fn a_circle_lying_on_the_torus_is_uncertain() {
        let pose = Pose {
            c: [R, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            rho: RT,
            u: [1.0, 0.0, 0.0],
        };
        assert!(matches!(door(pose, 0.0, 1.0), CircleTorusRoots::Uncertain));
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
        assert!(matches!(door(pose, 0.0, 1.0), CircleTorusRoots::Miss));
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
        assert!(matches!(door(pose, 2.0, 4.0), CircleTorusRoots::Uncertain));
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
            let got = circle_torus_roots(
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
            let CircleTorusRoots::Certified { count, .. } = got else {
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
            let got = circle_torus_roots::<Interval>(
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
            let CircleTorusRoots::Certified { count, thetas } = got else {
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
}
