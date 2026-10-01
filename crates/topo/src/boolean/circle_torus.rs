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
//! # The substitution, and where its pole goes, and why it must be well conditioned
//!
//! `φ = θ − θₐ` is measured from an anchor `θₐ`, and the half-angle map
//! covers every `θ` except its pole `θₐ + π`. The quartic's leading
//! coefficient is `F` at the pole. The arc's midpoint is tried first,
//! so the pole is the arc's antipode; then anchors a `τ/32` step at a
//! time either side of it, nearest first. A pole may land inside the
//! arc: the one parameter the map cannot reach is then certified not a
//! root by the first decision below, and the roots are reported within
//! `π` of the arc's midpoint whatever the anchor.
//!
//! **Non-zero is not enough, and the lane once assumed it was.** A pole
//! definitely off the torus makes the division that normalizes the
//! quartic a division by a non-zero — but a pole a milliradian from a
//! root the arc does not hold puts a root near `t = ∞`, the monic
//! coefficients grow like the inverse of that distance, and at `f64` the
//! ladder then certified wrong counts: in-arc roots dropped, phantoms
//! invented (review of PR 3375, executed at `δ` from 1e-3 to 1e-7).
//! So an anchor is used only when its pole passes TWO decisions:
//!
//! - `bool_circle_torus_pole` — the linearized residual at the pole is
//!   definitely non-zero (on a ring torus `R > r` it has `F`'s sign,
//!   since `F = 2r·res·((ρ + R)² + h² − r²)` and the second factor
//!   exceeds `R² − r² > 0`). This is what refuses a carrier lying ON the
//!   torus, whose `F` is identically zero and whose coefficients are
//!   rounding noise that no ratio can be read off.
//! - `bool_circle_torus_pole_conditioning` — `|F(pole)| ≥ κ·A` with
//!   `A = |c₀| + |(c₁, s₁)| + |(c₂, s₂)|`, which bounds `|F|` everywhere
//!   and `|F′|` by `2A`. Two consequences carry the ladder. Every root
//!   is at least `κ/2` radians from the pole (`F` cannot fall from
//!   `κA` to zero faster than `2A` per radian), so none is near `t = ∞`.
//!   And each coefficient of `F·(1 + t²)²` is at most `6A` in size, so
//!   the monic coefficients are at most `6/κ`: the ladder sees a
//!   quartic whose coefficients, roots and rounding are all bounded by
//!   a fixed multiple of the circle's own scale, whatever the pose. The
//!   margin is metered as the arc length `ρ·(|F(pole)| − κA)/A` that
//!   bound guarantees. `κ = 1/16`.
//!
//! **Some anchor always passes, unless `F ≡ 0`.** By Parseval,
//! `max|F|² ≥ mean F² = c₀² + (A₁² + A₂²)/2`, and by Cauchy–Schwarz
//! `A² = (c₀ + A₁ + A₂)² ≤ (1 + 2 + 2)(c₀² + A₁²/2 + A₂²/2)`, so `|F|`
//! reaches `A/√5 ≈ 0.45A` somewhere, and within `π/32` of there — where
//! some candidate lies — it is still at least `0.45A − 2A·π/32 ≈ 0.25A`,
//! above `κA`. So `Uncertain` from the anchor search means `F` is
//! (numerically) identically zero, or the band could not separate the
//! margins. The machinery
//! is the surface-generic [`half_angle_roots`]: any surface whose
//! implicit composed with a circle is a degree-2 trigonometric
//! polynomial (a cone's is too) can hand it its harmonics.
//!
//! # The noise meter
//!
//! `F`'s harmonics are sums of terms as large as `(|C₀ − c|² + ρ²)²` —
//! `ρ⁴` for a large circle — while the crossings live where `F` is
//! small. At `f64` their rounding is therefore an error in `F` of about
//! `u·T` (`T` the sum of the terms' magnitudes, [`NOISE_ULPS`] of them
//! charged), which is a residual error of `u·T / (2r(R² − r²))` metres
//! everywhere on the carrier (`F = 2r·res·Q` with `Q ≥ R² − r²`).
//! The door refuses when that error is DEFINITELY past the band's
//! escalation threshold (`bool_circle_torus_noise` deciding
//! `Positive`): the representation cannot resolve what the band asks
//! of it. The same error moves each root by `error/|F′|` radians; that
//! arc length is held to the same threshold
//! (`bool_circle_torus_root_slack`), or a caller's span and trim
//! decisions would be made on the wrong point.
//!
//! **What the meter covers, and what it does not.** It bounds ONE
//! stage: the evaluation of the harmonics from the geometry. It does
//! not certify "the count of the computed `F`": everything downstream —
//! the anchor rotation, the division by `F(pole)`, the monic `(2ρ)^k`
//! rescale, the depression and the discriminant — rounds again, and
//! those errors are covered only by the ladder's own band decisions,
//! as they are for the line lane. The conditioning guard bounds the
//! monic coefficients by `6/κ` in units of `2ρ`, which keeps that
//! amplification a fixed multiple of the circle's scale, but no bound
//! on it is computed here. So the premise is: **the downstream stages'
//! rounding stays within what the ladder's band margins absorb** — the
//! same premise every `f64` ladder in the kernel rests on. Under it,
//! the harmonics' error below the threshold changes the answer only at
//! an extremum within that threshold of zero, inside the gap where the
//! band declines to call a sign.
//!
//! What supports the premise is measurement, not proof: the delta
//! review's fuzz — about 12k cases around the refusal threshold, some
//! 50k in all — found no wrong answer from the metered door, and a
//! maximum root error of 4.8e-10 m on the parallel arm. That supports
//! the premise at the poses it drew; it does not bound the
//! amplification. The `Interval` lane needs no premise: every
//! stage there is an enclosure.
//!
//! **What that costs, measured.** Against a torus `R = 1, r = 0.25` at
//! the default band, grazing circles at `ρ = 10` are answered (the
//! shallowest grazes refuse on their root slack), and from `ρ = 30`
//! every one refuses — where the unmetered door certified misses on
//! real dips and phantom pairs on clearances from `ρ = 100`. The
//! threshold scales as `ρ⁴ ≲ 10ε·r·R²/(u·NOISE_ULPS)`.
//!
//! The `Interval` lane needs none of this to be sound: its coefficients
//! are enclosures of the true ones, and the ladder decides on the
//! enclosures. The meter runs there too, harmlessly.
//!
//! # Units, and the lever
//!
//! The root variable handed to the certified ladder is the LENGTH
//! `τ = 2ρ·t`, which is arc length to first order about the anchor. The
//! ladder's lever is the length its roots spread over, which is where
//! its margins become lengths (`Δ` over `lever¹¹` reads `δ²/lever` for
//! two roots `δ` apart among roots spread over `lever`). That is the
//! smaller of the carrier's own `2ρ` and the torus's extent `R + r`: a
//! circle smaller than the torus has its roots spread over its own
//! size (the torus's extent there made the margins scale like
//! `ρ¹⁰/(R + r)¹¹`, and small generic circles escalated), and one
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
//!   would read as a tangency. It is decided FIRST, geometrically
//!   (`bool_circle_torus_coaxial_tilt`, `_offset`, both metres), and
//!   answered [`CircleTorusRoots::Coaxial`]: the residual is constant
//!   along the carrier, and the caller decides what that means.
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
//!   the answer is `Uncertain` — which is what keeps an undeclared
//!   on-carrier circle away from every recording arm.
//! - **A tangency** — the carrier grazing the tube, a double root — is
//!   the ladder's own `Uncertain` (or a contour-reach margin in band),
//!   anywhere on the carrier.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::solid_contain::{QuarticRows, TorusRoots, depressed_quartic_roots};
use crate::validate::decide;

/// The circle × torus lane's ladder rows.
const CIRCLE_TORUS_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_torus_pole",
    conditioning: "bool_circle_torus_pole_conditioning",
    noise: "bool_circle_torus_noise",
    root_slack: "bool_circle_torus_root_slack",
    quartic: QuarticRows {
        disc: "bool_circle_torus_disc",
        shape: "bool_circle_torus_shape",
        depth: "bool_circle_torus_depth",
        odd: "bool_circle_torus_odd",
        split: "bool_circle_torus_split",
        split_lead: "bool_circle_torus_split_lead",
    },
};

/// What the certified circle × torus roots say about a whole carrier.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleTorusRoots<T> {
    /// The carrier is coaxial with the torus: its residual is constant.
    Coaxial,
    /// A certified count of zero: the carrier misses the torus.
    Miss,
    /// No certain count — a tangency, a carrier on the torus, a crossing
    /// whose bump is inside the band, or no anchor whose pole is
    /// definitely off the torus AND well conditioned.
    Uncertain,
    /// A certified count (2 or 4) and the carrier parameters `θ` of
    /// those roots, unordered, in `thetas[..count]`. Every `θ` lies
    /// within `π` of the arc's midpoint, so it compares with the arc
    /// `[t₀, t₁]` the caller passed without wrapping.
    Certified { count: usize, thetas: [T; 4] },
    /// The quartic's constructed roots disagree in number with its
    /// certified count ([`TorusRoots::CountDisagrees`]).
    CountDisagrees,
}

/// The certified crossings of the circle carrier
/// `center + radius·(u_ref cos θ + (axis × u_ref) sin θ)` with the torus
/// `(t_center, t_axis, major, minor)`, anchored so that the arc
/// `[t0, t1]` avoids the substitution's pole (module docs).
///
/// # Errors
///
/// [`Indeterminate`] — an in-band classifying sign in the ladder, or its
/// count cross-check. An in-band sign at the coaxial test, at a pole or
/// at its conditioning is not an error: the coaxial test falls through
/// (a near-coaxial carrier is `Uncertain`), and a pole tries the next
/// anchor.
#[allow(clippy::too_many_arguments)]
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
            Ok(Sign::Zero) => Ok(CircleTorusRoots::Coaxial),
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
            ),
            // A negative length is not an answer; an in-band one is a
            // near-coaxial pose neither arm can stand behind.
            Ok(Sign::Negative) | Err(_) => Ok(CircleTorusRoots::Uncertain),
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
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };
    // The ladder's lever is the scale the roots spread over in the
    // arc-length variable `τ = 2ρ·t`: the carrier's own `2ρ` for a
    // circle smaller than the torus, the torus's extent for a larger
    // one, whose crossings lie where it passes through that extent.
    let lever = (two * radius).min(major_radius + minor_radius);
    // **The noise meter's inputs** (module docs, "The noise meter"): a
    // bound on every term the harmonics are built from, and the torus's
    // floor on `|F|` per metre of residual.
    let s_abs = s_k.abs() + s1c.abs() + s1s.abs();
    let h_abs = h0.abs() + h1c.abs() + h1s.abs();
    let terms = s_abs.powi(2) + four_rr * (s0.abs() + s1c.abs() + s1s.abs() + h_abs.powi(2));
    let f_per_metre = two * minor_radius * (rr - minor_radius.powi(2));
    Ok(
        match half_angle_roots(
            &harmonics,
            |theta| geom_brep::implicit_residual(torus, point_at(theta)),
            HalfAngleFrame {
                t0,
                t1,
                radius,
                lever,
                noise: rounding_charge(terms),
                f_per_metre,
            },
            &CIRCLE_TORUS_ROWS,
            band,
        )? {
            HalfAngleRoots::Miss => CircleTorusRoots::Miss,
            HalfAngleRoots::Uncertain => CircleTorusRoots::Uncertain,
            HalfAngleRoots::Certified { count, thetas } => {
                CircleTorusRoots::Certified { count, thetas }
            }
            HalfAngleRoots::CountDisagrees => CircleTorusRoots::CountDisagrees,
        },
    )
}

/// A degree-2 trigonometric polynomial
/// `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`.
pub(super) struct Harmonics<T> {
    pub(super) c0: T,
    pub(super) c1: T,
    pub(super) s1: T,
    pub(super) c2: T,
    pub(super) s2: T,
}

/// The predicate rows one caller of [`half_angle_roots`] meters under.
pub(super) struct HalfAngleRows {
    /// The pole is definitely off the surface (a residual, metres).
    pub(super) pole: &'static str,
    /// The pole is well conditioned (module docs; metres).
    pub(super) conditioning: &'static str,
    /// The harmonics' rounding, as a residual, is inside the band (the
    /// noise meter; metres).
    pub(super) noise: &'static str,
    /// Each root's position uncertainty, as arc length, is inside the
    /// band (metres).
    pub(super) root_slack: &'static str,
    /// The quartic ladder's own rows.
    pub(super) quartic: QuarticRows,
}

/// Where [`half_angle_roots`] works: the arc `[t0, t1]` of a circle of
/// `radius`, the ladder's `lever` (a length), and the noise meter's two
/// inputs: `noise`, a bound on the `f64` evaluation error of `F` from
/// its harmonics (in `F`'s units), and `f_per_metre`, a floor on
/// `|F| / |residual|` over the surface's neighbourhood, which turns it
/// into metres.
pub(super) struct HalfAngleFrame<T> {
    pub(super) t0: T,
    pub(super) t1: T,
    pub(super) radius: T,
    pub(super) lever: T,
    pub(super) noise: T,
    pub(super) f_per_metre: T,
}

/// How many units in the last place of the term bound the harmonics'
/// evaluation error is charged (module docs, "The noise meter"). Each
/// harmonic is a short chain from the inputs — a squared norm, a
/// product, a sum of four terms — whose every rounding is half an ulp
/// of a quantity the term bound dominates; sixteen is that chain's
/// count with room. It is a ROUNDING estimate, the `f64` lane's
/// contract, not an enclosure: the `Interval` lane carries the
/// enclosure itself through every coefficient and the ladder decides on
/// it, so it needs no meter to be sound. The circle × sphere door
/// ([`super::circle_sphere`]) charges its first harmonic the same count:
/// its chain is shorter, so the count holds there with more room.
pub(super) const NOISE_ULPS: f64 = 16.0;

/// The rounding charged against a term bound `terms`: [`NOISE_ULPS`]
/// half-ulps of it — the meters' one spelling of the charge.
pub(super) fn rounding_charge<T: geom_core::Real>(terms: T) -> T {
    T::from_f64(NOISE_ULPS * f64::EPSILON * 0.5) * terms
}

/// What [`half_angle_roots`] certifies.
pub(super) enum HalfAngleRoots<T> {
    Miss,
    Uncertain,
    Certified { count: usize, thetas: [T; 4] },
    CountDisagrees,
}

/// The conditioning floor `κ` (module docs): the pole's `|F|` must be at
/// least this share of `F`'s amplitude bound.
const POLE_CONDITIONING: f64 = 1.0 / 16.0;

/// How many evenly spaced poles round the carrier the anchor search may
/// try (module docs: 32 is what makes some candidate always pass
/// [`POLE_CONDITIONING`]).
const POLE_CANDIDATES: u32 = 32;

/// **The certified real roots of a degree-2 trigonometric polynomial
/// `F(θ)` along a circle, by the tangent half-angle** — a quartic in
/// `t = tan((θ − θₐ)/2)`, solved by the ray lane's certified ladder.
/// General over the surface: `residual` is the surface's linearized
/// residual (metres) along the carrier, which must share `F`'s sign.
///
/// An anchor `θₐ` is USED only when its pole `θₐ + π` passes two
/// decisions (module docs, "The substitution, and where its pole goes,
/// and why it must be well conditioned"): the residual there is
/// definitely non-zero, and `|F(pole)|` clears `κ·A` with
/// `A = |c₀| + |(c₁, s₁)| + |(c₂, s₂)|`, metered as the arc length
/// `ρ·(|F(pole)| − κA)/A` that bound guarantees between the pole and
/// every root. The antipode of the arc is tried first and poles off the
/// arc are preferred, but a pole may land inside it (certified not a
/// root); if no candidate passes the answer is `Uncertain`.
///
/// Before any of that, the noise meter (module docs, "The noise meter")
/// must put the harmonics' rounding inside the band, and after it each
/// root's position uncertainty must be inside the band too.
pub(super) fn half_angle_roots<T: Decide>(
    f: &Harmonics<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    band: Band,
) -> Result<HalfAngleRoots<T>, Indeterminate> {
    let HalfAngleFrame {
        t0,
        t1,
        radius,
        lever,
        noise,
        f_per_metre,
    } = frame;
    // **The noise meter** (module docs): it bounds the harmonics'
    // evaluation error, `noise`, a residual error of up to
    // `noise / f_per_metre` metres everywhere on the carrier, and refuses
    // when that is definitely past the band's escalation threshold. The
    // stages after it (rotation, pole division, rescale, depression,
    // discriminant) round again; those are left to the ladder's own band
    // decisions, which is the premise the module docs state.
    match decide(rows.noise, Margin::of(noise / f_per_metre), band) {
        Ok(Sign::Positive) => return Ok(HalfAngleRoots::Uncertain),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
    }
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let six = T::from_f64(6.0);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let amplitude = f.c0.abs() + hypot(f.c1, f.s1) + hypot(f.c2, f.s2);
    let mid = (t0 + t1) / two;
    let scale = two * radius;
    // The candidate anchors: the arc's midpoint first (its pole is the
    // antipode), then every `τ/32` step either side of it, nearest
    // first — so a pole off the arc is preferred and a pole inside it
    // (certified not a root by the residual decision) is used only when
    // none off it is well conditioned. Thirty-two steps are what make
    // the list complete: some candidate always passes the conditioning
    // floor unless `F` is identically zero (module docs).
    let step = T::tau() / T::from_f64(f64::from(POLE_CANDIDATES));
    let shifts = (0..=POLE_CANDIDATES / 2).flat_map(|k| {
        let k = T::from_f64(f64::from(k));
        [step * k, T::zero() - step * k]
    });
    // `k = 0` yields the zero shift twice; the second copy is dropped.
    for shift in shifts.skip(1) {
        let anchor = mid + shift;
        match decide(rows.pole, Margin::of(residual(anchor + T::pi())), band) {
            Ok(Sign::Positive | Sign::Negative) => {}
            Ok(Sign::Zero) | Err(_) => continue,
        }
        // The harmonics about the anchor: the first rotates by `θₐ`, the
        // second by `2θₐ`.
        let (sa, ca) = anchor.sin_cos();
        let (s2a, c2a) = (two * anchor).sin_cos();
        let c1 = f.c1 * ca + f.s1 * sa;
        let s1 = f.s1 * ca - f.c1 * sa;
        let c2 = f.c2 * c2a + f.s2 * s2a;
        let s2 = f.s2 * c2a - f.c2 * s2a;
        // `F·(1 + t²)²` in `t = tan(φ/2)`, low coefficient first.
        let a = [
            f.c0 + c1 + c2,
            two * s1 + four * s2,
            two * f.c0 - six * c2,
            two * s1 - four * s2,
            f.c0 - c1 + c2,
        ];
        let lead = a[4];
        match decide(
            rows.conditioning,
            Margin::over_lever(
                radius * (lead.abs() - T::from_f64(POLE_CONDITIONING) * amplitude),
                amplitude,
            ),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => continue,
        }
        // Monic in `τ = 2ρ·t` (a length): `τ⁴ + B τ³ + C τ² + D τ + E`.
        let b3 = scale * a[3] / lead;
        let c2m = scale.powi(2) * a[2] / lead;
        let d1 = scale.powi(3) * a[1] / lead;
        let e0 = scale.powi(4) * a[0] / lead;
        // Depress by `τ = y − B/4`.
        let shift_y = b3 / four;
        let p = c2m - T::from_f64(3.0) * b3.powi(2) / T::from_f64(8.0);
        let q = d1 - b3 * c2m / two + b3.powi(3) / T::from_f64(8.0);
        let s = e0 - b3 * d1 / four + b3.powi(2) * c2m / T::from_f64(16.0)
            - T::from_f64(3.0) * b3.powi(4) / T::from_f64(256.0);
        return Ok(
            match depressed_quartic_roots(p, q, s, lever, &rows.quartic, band)? {
                TorusRoots::Miss => HalfAngleRoots::Miss,
                TorusRoots::Uncertain => HalfAngleRoots::Uncertain,
                TorusRoots::CountDisagrees => HalfAngleRoots::CountDisagrees,
                TorusRoots::Certified { count, ts: ys } => {
                    let mut thetas = [T::zero(); 4];
                    for (theta, y) in thetas.iter_mut().zip(ys).take(count) {
                        // Reported within `π` of the arc's midpoint, so a
                        // caller compares it with `[t0, t1]` directly.
                        let raw = anchor + two * ((y - shift_y) / scale).atan();
                        *theta = mid + (raw - mid).reduce_periodic_centred(T::tau());
                        // **Where the root is, not just that it is.** The
                        // true root lies within `noise / |F′|` radians of
                        // the computed one; that arc length must be inside
                        // the band, or the span and trim decisions a
                        // caller makes on the point are on the wrong point.
                        let (s1t, c1t) = theta.sin_cos();
                        let (s2t, c2t) = (two * *theta).sin_cos();
                        let slope = f.s1 * c1t - f.c1 * s1t + two * (f.s2 * c2t - f.c2 * s2t);
                        match decide(
                            rows.root_slack,
                            Margin::of(radius * noise / slope.abs()),
                            band,
                        ) {
                            Ok(Sign::Positive) => return Ok(HalfAngleRoots::Uncertain),
                            Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
                        }
                    }
                    HalfAngleRoots::Certified { count, thetas }
                }
            },
        );
    }
    Ok(HalfAngleRoots::Uncertain)
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
) -> Result<CircleTorusRoots<T>, Indeterminate> {
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
            return Ok(CircleTorusRoots::Miss);
        }
        _ => return Ok(CircleTorusRoots::Uncertain),
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
        return Ok(CircleTorusRoots::Uncertain);
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
            Ok(Sign::Positive) => return Ok(CircleTorusRoots::Uncertain),
            // A NaN slack (a zero slope, which the placements above have
            // already refused as a graze) is `Err` and refuses too.
            Ok(Sign::Zero | Sign::Negative) => {}
            Err(_) => return Ok(CircleTorusRoots::Uncertain),
        }
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
            let got = circle_torus_roots(
                Point3::new(pose.c[0], pose.c[1], pose.c[2]),
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
                    Ok(CircleTorusRoots::Uncertain)
                        | Err(Indeterminate {
                            predicate: Some("bool_circle_torus_contour_residual"),
                            ..
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
                    matches!(door(pose, t0, t1), CircleTorusRoots::Uncertain),
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
        assert!(matches!(door(pose, -3.0, 3.0), CircleTorusRoots::Miss));
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
        let got = circle_torus_roots(
            Point3::new(pose.c[0], pose.c[1], pose.c[2]),
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
                Ok(CircleTorusRoots::Uncertain)
                    | Err(Indeterminate {
                        predicate: Some("bool_circle_torus_contour_residual"),
                        ..
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
        let got = circle_torus_roots(
            Point3::new(pose.c[0], pose.c[1], pose.c[2]),
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
            !matches!(got, CircleTorusRoots::Miss),
            "a real dip is not a certified miss: {got:?}"
        );
        if let CircleTorusRoots::Certified { count, .. } = got {
            assert_eq!(count, 2, "the whole turn holds the dip's two roots");
        }
        // The interval lane encloses the rounding the meter estimates,
        // so it never certifies the miss either.
        let got = circle_torus_roots::<Interval>(
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
            !matches!(got, Ok(CircleTorusRoots::Miss)),
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
                    let got = circle_torus_roots(
                        Point3::new(pose.c[0], pose.c[1], pose.c[2]),
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
                        CircleTorusRoots::Uncertain => {}
                        CircleTorusRoots::Coaxial => panic!("{label}: not coaxial"),
                        CircleTorusRoots::CountDisagrees => panic!("{label}: counts disagree"),
                        CircleTorusRoots::Miss => {
                            assert!(truth.is_empty(), "{label}: a certified miss on a dip");
                            answered[i] += 1;
                        }
                        CircleTorusRoots::Certified { count, thetas } => {
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
            let got = circle_torus_roots(
                Point3::new(pose.c[0], pose.c[1], pose.c[2]),
                v3(pose.n),
                pose.rho,
                v3(pose.u),
                -0.5,
                0.5,
                &torus(),
                fixed_band(),
            );
            if tilt == 0.0 {
                let Ok(CircleTorusRoots::Certified { count, thetas }) = got else {
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
                    matches!(got, Ok(CircleTorusRoots::Uncertain)),
                    "tilt {tilt}: roots displaced past the band are not certified: {got:?}"
                );
            }
        }
    }
}
