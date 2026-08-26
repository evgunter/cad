//! **The tangency jet** (C7, M5 PR 9): the raw second-order data the
//! `TangentIntersection` certification schedule and the second-order
//! sector trilean consume.
//!
//! A tangential contact locus is exactly the place first-order data
//! ties: the two surfaces share a tangent plane along it, so every
//! first-order classifier (dihedral, transversality, sector ranking)
//! honestly returns "exactly on". The discriminating datum one order
//! up is the **relative transverse normal curvature**
//! `κ_rel = κ₁ − κ₂` along the shared-tangent-plane direction
//! transverse to the locus: it is the implicit-function-theorem
//! denominator of the jet system (D2's ratified observation — the
//! system includes the first-order equations, so reconstruction is
//! well-conditioned along a genuine tangency precisely when κ_rel is
//! bounded away from zero), and its collapse is the honest signature
//! of an under-determined locus (a G2 conventional join, an
//! osculating pair — F6 in-band, exempt-by-predicate at zero).
//!
//! Everything here is **raw total arithmetic** (poison in, poison
//! out); classification happens at the callers, through the crate's
//! single `decide` funnel, under the `tangent_*` predicate family —
//! the K funnel's second genuinely ill-conditioned crop (after
//! `solver_branch_margin`; K-REPORT's scope honesty predicted this
//! corpus, and the PR 14 K-snapshot will read it).

use geom::Curve3;
use geom::Surface;
use geom_core::{Point3, Real, Vec3};

use crate::implicit::{implicit_gradient, implicit_hessian_form};

/// The per-sample jet data of a candidate tangency between `s1` and
/// `s2` at `p`, along locus tangent `tangent`.
#[derive(Clone, Copy, Debug)]
pub struct TangentJet<T: Real> {
    /// `sin θ` of the angle between the two implicit gradients — the
    /// first-order (normal-parallelism) defect. ~0 on a genuine
    /// tangency.
    pub sin_theta: T,
    /// The relative transverse normal curvature `κ₁ − κ₂` (1/meters),
    /// both measured against the SAME shared normal, along the
    /// transverse direction `n̂ × τ̂`. Its magnitude bounded away from
    /// zero is the second-order margin; its collapse means the
    /// surfaces under-determine the locus.
    pub kappa_rel: T,
}

/// The jet of (`s1`, `s2`) at `p` along `tangent` (raw; docs above).
/// Fixed evaluation order (D9). Poison propagates from singular
/// gradients (cone apex/axis, torus axis) and `Nurbs` kinds.
pub fn tangent_jet<T: Real>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    p: Point3<T>,
    tangent: Vec3<T>,
) -> TangentJet<T> {
    let g1 = implicit_gradient(s1, p);
    let g2 = implicit_gradient(s2, p);
    let n1 = g1.norm();
    let n2 = g2.norm();
    let sin_theta = g1.cross(g2).norm() / (n1 * n2);
    // The shared normal (s1's side) and the transverse in-tangent-plane
    // direction d̂ = n̂ × τ̂.
    let n_hat = g1 / n1;
    let d = n_hat.cross(tangent.normalize());
    let d_hat = d / d.norm();
    // Normal curvatures along d̂, both signed against n̂: κᵢ =
    // d̂ᵀ(∇²Fᵢ)d̂ / (∇Fᵢ · n̂) — the denominator carries the sign when
    // the gradients are antiparallel, so κ_rel is orientation-honest.
    let k1 = implicit_hessian_form(s1, p, d_hat) / g1.dot(n_hat);
    let k2 = implicit_hessian_form(s2, p, d_hat) / g2.dot(n_hat);
    TangentJet {
        sin_theta,
        kappa_rel: k1 - k2,
    }
}

/// Span-wide certified bounds for the jet schedule between samples —
/// the C2.2 half of the certificate (metres), plus the tube's
/// curvature-drift bound. `None` when the (carrier kind, surface
/// kinds) triple is outside the certified lane (the two arms are
/// enumerated on [`tangent_certificate_lane`]: `Line` carriers on
/// plane/cylinder/sphere pairs, `Circle` carriers on those plus
/// torus). A `None` is a typed refusal at the caller, never a
/// fallback.
#[derive(Clone, Copy, Debug)]
pub struct TangentSpanBounds<T: Real> {
    /// Certified bound on the residual sag between adjacent schedule
    /// samples: `(Δt²/8)·sup|f″|` per surface, maxed — for a line in
    /// the certified lane both composites are QUADRATIC in t (plane:
    /// linear; sphere/cylinder: constant Hessian), so the interior of
    /// each span is bounded by its endpoints plus exactly this sag
    /// (the standard quadratic interpolation bound, exact in family).
    pub residual_sag: T,
    /// Certified bound on how far `|κ_rel|` can drift between
    /// adjacent samples: `Δstep · sup|dκ/dt|`, from the Gauss map's
    /// `1/arm` Lipschitz bound folded with the constant Hessians.
    pub kappa_drift: T,
}

/// **THE certified-lane predicate** (C12.1, one place): is this
/// (carrier kind, surface-kind pair) triple inside the jet
/// certificate's span-bound lane? It has THREE consumers, and naming
/// all three is what makes the together-by-construction claim checkable:
/// [`tangent_span_bounds`] (which refuses outside it), the tier-3
/// must-carry enforcement in `topo::validate`, and the declared-contact
/// verifier in `topo::boolean::contact_verify` (which gates on it before
/// taking any per-sample verdict). The demanded set, the certifiable set
/// and the verifiable set are therefore ONE set BY CONSTRUCTION — a
/// tangency the certificate cannot store is never demanded and never
/// accepted as a declared contact. Widening this predicate widens all
/// three at once, which is the property that makes a new surface row
/// (the cone row below) cost nothing downstream.
///
/// Two arms, both closed-form:
/// - **`Line` carriers on `Plane`/`Cylinder`/`Sphere` pairs** — the
///   class the C5 table's tangent arms mint (M5 PR 9), unchanged.
/// - **`Circle` carriers on `Plane`/`Cylinder`/`Sphere`/`Torus`/`Cone`
///   pairs** — the class the fillet trimlines mint: the corner ball's
///   contact circles with its edge cylinders, and every rim blend's
///   contact circles with its two supports, which are the elementary
///   surfaces of revolution in full. `Torus` and `Cone` enter the lane
///   HERE and only here: neither a torus nor a cone tangency along a
///   *line* is a configuration this kernel constructs, and the line
///   arm's bounds are left byte-for-byte unchanged so PR 9's
///   certificates do not move.
///
/// **Why a circle admits `Torus` when a line does not**: the
/// configurations this kernel MINTS on a circle carrier — a fillet
/// corner ball against its edge cylinders, a rim blend's torus
/// against its flat face and its pip sphere, a revolve's latitude
/// join — are all **coaxial**: the circle's axis is a common axis of
/// revolution of both surfaces and its centre lies on that axis. The
/// bounds below are written so that the coaxial configuration makes
/// every one of them exactly zero **by equivariance** (`κ_rel` and
/// the implicit residual are isometry invariants, and a coaxial
/// circle's motion is a symmetry flow of both surfaces), and so that
/// a configuration that misses coaxiality pays for the miss
/// continuously rather than through a gate.
///
/// **The scope of that claim, stated exactly (fix pass F3).** An
/// earlier draft of this comment asserted that circle tangency
/// between two distinct elementary surfaces of revolution FORCES the
/// coaxial configuration. That is false, and the reviewer's
/// counterexample is in `tests/review_pr12_meridian_probe.rs`: a
/// sphere centred on a torus's spine circle is tangent to the torus
/// along a whole MERIDIAN (minor) circle, whose axis is
/// perpendicular to the torus's, not parallel to it. The class is
/// real, it is jet-determinate, and it is INSIDE this lane.
///
/// What actually happens there is the honest outcome and not a hole:
/// the deviation measured below is large, so the span bounds are
/// large, and certification refuses LOUDLY
/// (`ResidualExceeded { TangentHull }`) instead of certifying
/// something it has not bounded. The residual risk is therefore
/// narrow and named: tier 3's must-carry could demand an intrinsic
/// description on such an edge while this arm cannot certify it —
/// **in-lane but uncertifiable**. No constructor in the kernel mints
/// that configuration today (the fillet arms produce coaxial contact
/// circles; revolve's joins are coaxial latitude circles), so the
/// class is latent; closing it needs either a meridian-aware bound or
/// a lane predicate that can see the configuration, and that is
/// recorded as a numbered deviation rather than papered over here.
pub fn tangent_certificate_lane<T: Real>(
    carrier: &Curve3<T>,
    s1: &Surface<T>,
    s2: &Surface<T>,
) -> bool {
    let straight = |s: &Surface<T>| {
        matches!(
            s,
            Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Sphere { .. }
        )
    };
    let round =
        |s: &Surface<T>| straight(s) || matches!(s, Surface::Torus { .. } | Surface::Cone { .. });
    match carrier {
        Curve3::Line { .. } => straight(s1) && straight(s2),
        Curve3::Circle { .. } => round(s1) && round(s2),
        _ => false,
    }
}

/// The span bounds for `carrier` over `[t0, t1]` at the 9-sample
/// schedule (docs on [`TangentSpanBounds`]).
pub fn tangent_span_bounds<T: Real>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
) -> Option<TangentSpanBounds<T>> {
    if !tangent_certificate_lane(carrier, s1, s2) {
        return None;
    }
    let step = (t1 - t0) / T::from_f64(f64::from(crate::certify::CERT_SAMPLES - 1));
    if let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *carrier
    {
        return circle_span_bounds(s1, s2, center, axis, radius, u_ref, step);
    }
    let Curve3::Line { origin: _, dir } = *carrier else {
        return None;
    };
    // Per surface: (sup |f″| along the segment, sup |dκ/dt|).
    //
    // The drift factor 8·|dir|/r² per curved surface: κ = d̂ᵀHd̂/|∇F|
    // with H CONSTANT in the lane (|H| ≤ 1/r); the transverse
    // direction turns no faster than the Gauss map, |dd̂/dt| ≤
    // |dir|/arm, and |∇F| = ρ/r stays within a factor 2 of 1 for any
    // point the residual schedule can accept (ε ≪ r), so arm ≥ r/2.
    // Product+quotient rule: 2·(1/r)·(2|dir|/r) for the numerator's
    // turn plus (1/(r/2))·(4|dir|/r) for the denominator's drift —
    // ≤ 8|dir|/r² total, taken per surface and summed by the caller.
    let bounds_of = |s: &Surface<T>| -> Option<(T, T)> {
        let eight = T::from_f64(8.0);
        match *s {
            Surface::Plane { .. } => Some((T::zero(), T::zero())),
            Surface::Sphere { radius, .. } => {
                let f2 = dir.norm_squared() / radius;
                let drift = eight * dir.norm() / radius.powi(2);
                Some((f2, drift))
            }
            Surface::Cylinder { axis, radius, .. } => {
                let d_ax = dir.dot(axis);
                let d_perp2 = dir.norm_squared() - d_ax.powi(2);
                let f2 = d_perp2 / radius;
                // Only the axis-TRANSVERSE component turns the Gauss
                // map or moves ρ: a ruling (the C5 tangent-arm mint)
                // has exactly zero drift, and the bound says so.
                let drift = eight * d_perp2.max(T::zero()).sqrt() / radius.powi(2);
                Some((f2, drift))
            }
            // No closed second-fundamental-form bound for a spline
            // stand-in, so no tangent span bound — `Approx` declines
            // with the rest, and the caller keeps its own door.
            Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
                None
            }
        }
    };
    let (f2a, da) = bounds_of(s1)?;
    let (f2b, db) = bounds_of(s2)?;
    let eighth = T::from_f64(0.125);
    Some(TangentSpanBounds {
        residual_sag: step.powi(2) * eighth * f2a.max(f2b),
        kappa_drift: step * (da + db),
    })
}

/// The component of `x` orthogonal to the unit direction `a`.
fn perp<T: Real>(x: Vec3<T>, a: Vec3<T>) -> Vec3<T> {
    x - a * x.dot(a)
}

/// `√(p² + q²)` — the amplitude of the harmonic `p·cos t + q·sin t`.
fn amp<T: Real>(p: T, q: T) -> T {
    (p.powi(2) + q.powi(2)).sqrt()
}

/// **`sup |(√g)″|`** for a degree-≤2 trigonometric `g` given its
/// harmonic amplitudes `(A₁, A₂)` and a floor `√g ≥ g_lo > 0`:
/// `|g″|/(2√g) + |g′|²/(4·g^{3/2})` with `|g′| ≤ A₁ + 2A₂` and
/// `|g″| ≤ A₁ + 4A₂`.
///
/// The torus arm and the cone arm both bound a `√g` term of exactly
/// this shape — the torus's tube distance and the cone's distance from
/// the axis — and they differ ONLY in the floor they can justify (the
/// tube radius `(R − r)/2` there, the carrier's own radial minimum
/// here). Written once so the two cannot drift; the floors stay at
/// their sites, where the argument for each lives.
fn sqrt_second_derivative_bound<T: Real>(a1: T, a2: T, g_lo: T) -> T {
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let dg = a1 + two * a2;
    let ddg = a1 + four * a2;
    ddg / (two * g_lo) + dg.powi(2) / (four * g_lo.powi(3))
}

/// The **circle arm** of [`tangent_span_bounds`] (docs on
/// [`tangent_certificate_lane`]), for the carrier
/// `C(t) = c₀ + (û·cos t + v̂·sin t)·R_c`, `v̂ = axis × û`.
///
/// Both bounds are derived so that the **coaxial** configuration —
/// the only one in which two distinct elementary surfaces of
/// revolution are tangent along a whole circle — makes them exactly
/// zero, and a miss pays continuously. No gate, hence no
/// demanded-but-not-certifiable hole.
///
/// **`residual_sag`**: for `Plane`/`Sphere`/`Cylinder` the composite
/// `f∘C` is a trigonometric polynomial of degree ≤ 2 (the implicit
/// residuals are quadratic in the point and `C` is a first harmonic),
/// so `sup|(f∘C)″| ≤ A₁ + 4·A₂` with `A_k` the exact harmonic
/// amplitudes computed below; the standard `(Δt²/8)·sup|f″|`
/// quadratic-interpolation sag then applies exactly as on the line
/// arm. For `Torus` the residual splits as
/// `(|p−c|² + R² − r²)/(2r) − (R/r)·|w|`: the first summand is
/// quadratic (same treatment), the second is `√g` with `g = |w|²`
/// again degree ≤ 2, bounded by [`sqrt_second_derivative_bound`] at
/// the floor `√g_min = (R − r)/2` — the ring-torus tube floor
/// `|w| ≥ R − r` halved by the module's standing `ε ≪ r` allowance
/// (the same allowance the line arm's `arm ≥ r/2` step already
/// takes). The cone arm reuses that bound at its own floor.
///
/// **`kappa_drift`**: `κ_rel` is an isometry invariant, so it is
/// CONSTANT along any carrier motion that is a symmetry flow of both
/// surfaces (the equivariance principle, applied rather than
/// assumed). Per surface we measure `dev`, the sup over the span of
/// how far the carrier's velocity departs from that surface's
/// Killing field — rotation about its axis (through its axis point)
/// for sphere/cylinder/torus, plus free translation along the axis
/// for a cylinder, everything for a plane — and reuse the line arm's
/// `8·(motion)/r²` accounting with `dev` in place of the motion.
/// Coaxial ⇒ `dev = 0` ⇒ zero drift, EXACTLY.
#[allow(clippy::too_many_arguments)]
fn circle_span_bounds<T: Real>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    step: T,
) -> Option<TangentSpanBounds<T>> {
    let u = u_ref;
    let v = axis.cross(u_ref);
    let rc = radius;
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let eight = T::from_f64(8.0);
    // The degree-≤2 harmonic amplitudes of `|w(t)|²` for the radial
    // part `w` about the axis `a` through `o` (shared by the cylinder
    // and torus arms): `(A₁, A₂)`.
    let radial_harmonics = |a: Vec3<T>, o: Point3<T>| -> (T, T) {
        let e = perp(center - o, a);
        let up = perp(u, a);
        let vp = perp(v, a);
        let a1 = two * rc * amp(e.dot(up), e.dot(vp));
        let a2 = rc.powi(2) * amp((up.norm_squared() - vp.norm_squared()) / two, up.dot(vp));
        (a1, a2)
    };
    // The Killing-field deviation for rotation about `a` through `o`,
    // optionally quotienting the free translation along `a`.
    //
    // **Both angular directions** (M5 PR 12 fix pass): rotation about
    // an axis is a one-parameter group, so `−a·(p − o)` is as much a
    // Killing field of the surface as `+a·(p − o)` is — a carrier
    // circle traversed CLOCKWISE about a cylinder's stored axis is the
    // same coaxial configuration as one traversed counterclockwise,
    // and only the sign of the stored `axis` field differs. Measuring
    // against `+a` alone made that half of the configurations pay
    // `dev = r_c` (the clamp) for nothing — which is exactly the die's
    // corner arcs, half of whose `he_plus` directions point the other
    // way around their blend cylinder. The bound is the MINIMUM over
    // the two group directions: both are valid bounds, so taking the
    // smaller only tightens the certificate, and the coaxial
    // configuration now yields EXACTLY zero either way round.
    let dev_rot = |a: Vec3<T>, o: Point3<T>, slide: bool| -> T {
        let fix = |x: Vec3<T>| if slide { perp(x, a) } else { x };
        let dev = |s: T| -> T {
            let d0 = fix(a.cross(center - o) * s);
            let d1 = fix(v - a.cross(u) * s);
            let d2 = fix(u + a.cross(v) * s);
            d0.norm() + rc * (d1.norm() + d2.norm())
        };
        dev(T::one()).min(dev(-T::one())).min(rc)
    };
    let bounds_of = |s: &Surface<T>| -> Option<(T, T)> {
        match *s {
            // A plane's Hessian is zero: it contributes no curvature
            // drift at all, and its residual is the exact first
            // harmonic `R_c·|P_uv(n)|·(…)`.
            Surface::Plane { normal, .. } => {
                Some((rc * amp(u.dot(normal), v.dot(normal)), T::zero()))
            }
            Surface::Sphere {
                center: sc,
                radius: r,
                ..
            } => {
                let e = center - sc;
                let f2 = rc * amp(u.dot(e), v.dot(e)) / r;
                // A sphere is invariant under EVERY rotation about its
                // centre, so the honest Killing field is the one about
                // the CIRCLE's axis through the sphere centre: the
                // frame terms cancel identically (`v̂ = a × û`), and
                // `dev` collapses to the distance from the sphere
                // centre to the circle's axis line — exactly the
                // coaxiality defect.
                let drift = eight * dev_rot(axis, sc, false) / r.powi(2);
                Some((f2, drift))
            }
            Surface::Cylinder {
                origin,
                axis: a,
                radius: r,
                ..
            } => {
                let (a1, a2) = radial_harmonics(a, origin);
                let f2 = (a1 + four * a2) / (two * r);
                let drift = eight * dev_rot(a, origin, true) / r.powi(2);
                Some((f2, drift))
            }
            Surface::Torus {
                center: tc,
                axis: a,
                major_radius,
                minor_radius,
                ..
            } => {
                let (g1, g2) = radial_harmonics(a, tc);
                // The tube floor `|w| ≥ R − r`, halved by the standing
                // `ε ≪ r` allowance. Clamped at zero so a NON-ring
                // payload (`R ≤ r`, degenerate data the variant's
                // convention excludes) yields an infinite bound that
                // fails the caller's residual check loudly, never a
                // negative one that would silently under-bound.
                let g_lo = ((major_radius - minor_radius) / two).max(T::zero());
                let sqrt_part = sqrt_second_derivative_bound(g1, g2, g_lo);
                let e = center - tc;
                let f2 = rc * amp(u.dot(e), v.dot(e)) / minor_radius
                    + major_radius * sqrt_part / minor_radius;
                // The torus Hessian is not bounded by 1/r_minor alone:
                // the major-direction term adds `|d|/|w| ≤ r/(R − r)`,
                // so the effective radius is `r·(R − r)/R`.
                let r_eff = minor_radius * (major_radius - minor_radius) / major_radius;
                let drift = eight * dev_rot(a, tc, false) / r_eff.powi(2);
                Some((f2, drift))
            }
            // `F = |w|·cos α − |h|·sin α`. The radial half is the
            // torus's `√g` bound, through the same helper, with the tube
            // floor replaced by the carrier's OWN radial floor
            // `√(g₀ − A₁ − A₂)` — the harmonic decomposition's exact
            // minimum bound, which on a coaxial carrier is `R_c`. The
            // axial half is linear in the point away from the apex
            // plane, so its second derivative is `|h″| ≤ A_h`, the
            // first harmonic's amplitude — EXACTLY zero when the
            // carrier's plane is normal to the cone's axis, which is
            // what coaxial means here.
            //
            // **The presumption, stated:** `|h|` has a kink where the
            // carrier crosses the apex plane, and the sag argument
            // above needs `F` twice differentiable across each step.
            // A coaxial carrier makes `h` CONSTANT (`A_h = 0`), so no
            // crossing is possible in any configuration this kernel
            // mints; a non-coaxial one that does cross is bounded
            // optimistically here, which is the same narrow, named
            // residual risk this module's header records for the
            // meridian-circle class rather than papering over.
            Surface::Cone {
                apex,
                axis: a,
                half_angle,
                ..
            } => {
                let (s_a, c_a) = half_angle.sin_cos();
                let (g1, g2) = radial_harmonics(a, apex);
                let e = perp(center - apex, a);
                let up = perp(u, a);
                let vp = perp(v, a);
                let g0 =
                    e.norm_squared() + rc.powi(2) * (up.norm_squared() + vp.norm_squared()) / two;
                // Clamped at zero so a carrier that can reach the cone's
                // AXIS yields an infinite bound the caller's residual
                // check refuses loudly, never a negative one that would
                // silently under-bound.
                let g_lo = (g0 - g1 - g2).max(T::zero()).sqrt();
                let sqrt_part = sqrt_second_derivative_bound(g1, g2, g_lo);
                let h_amp = rc * amp(u.dot(a), v.dot(a));
                let f2 = c_a * sqrt_part + s_a * h_amp;
                // A cone is invariant under rotation about its axis
                // through its APEX and under nothing else — no free
                // translation, so the slide quotient is off. Its
                // curvature lever arm is the distance from the axis,
                // floored by the same `g_lo`.
                let drift = eight * dev_rot(a, apex, false) / g_lo.powi(2);
                Some((f2, drift))
            }
            // As the first-order bound above: no closed form for the
            // spline stand-in or the description behind it.
            Surface::Nurbs(_) | Surface::Approx(_) => None,
        }
    };
    let (f2a, da) = bounds_of(s1)?;
    let (f2b, db) = bounds_of(s2)?;
    Some(TangentSpanBounds {
        residual_sag: step.powi(2) * T::from_f64(0.125) * f2a.max(f2b),
        kappa_drift: step * (da + db),
    })
}
