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
//! `h = q·â` are FIRST harmonics in `θ` (the cross term `2ρ(C₀ − c)·(û
//! cos θ + v̂ sin θ)` is the only `θ`-dependence of `S`, since `|û cos θ
//! + v̂ sin θ| = 1`), so `F` is a trigonometric polynomial of degree
//! TWO. The tangent half-angle `t = tan(φ/2)` therefore turns
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
//! - **The circle lies ON the torus** (a rim, meridian or Villarceau
//!   circle): `F ≡ 0`, so the pole is on the torus at every anchor and
//!   the answer is `Uncertain` — which is what keeps an undeclared
//!   on-carrier circle away from every recording arm.
//! - **Parallel axes** (the circle's plane perpendicular to `â`, off
//!   the axis): `h` is constant and `F` is a quadratic in the first
//!   harmonic `S`. Nothing degenerates — the quartic factors into two
//!   real quadratics, which the ladder solves like any other.
//! - **A tangency** — the carrier grazing the tube, a double root — is
//!   the ladder's own `Uncertain`, anywhere on the carrier.

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
    let offset = (w0 - t_axis * w0.dot(t_axis)).norm();
    let coaxial = matches!(
        decide("bool_circle_torus_coaxial_tilt", Margin::of(tilt), band),
        Ok(Sign::Zero)
    ) && matches!(
        decide("bool_circle_torus_coaxial_offset", Margin::of(offset), band),
        Ok(Sign::Zero)
    );
    if coaxial {
        return Ok(CircleTorusRoots::Coaxial);
    }

    let mid = (t0 + t1) / two;
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
