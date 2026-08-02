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

use geom_core::{Point3, Real, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

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
/// kinds) triple is outside the certified lane: **`Line` carriers on
/// `Plane`/`Cylinder`/`Sphere` pairs** — exactly the class the C5
/// table's tangent arms mint at M5. A `None` is a typed refusal at
/// the caller, never a fallback.
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
/// certificate's span-bound lane — `Line` carriers on
/// `Plane`/`Cylinder`/`Sphere` pairs, the class the C5 table's
/// tangent arms mint at M5? Consulted by [`tangent_span_bounds`]
/// (which refuses outside it) AND by the tier-3 must-carry
/// enforcement in `topo::validate` — the demanded set and the
/// certifiable set are the same set BY CONSTRUCTION (a tangency the
/// certificate cannot store is never demanded).
pub fn tangent_certificate_lane<T: Real>(
    carrier: &Curve3<T>,
    s1: &Surface<T>,
    s2: &Surface<T>,
) -> bool {
    let line = matches!(carrier, Curve3::Line { .. });
    let ok = |s: &Surface<T>| {
        matches!(
            s,
            Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Sphere { .. }
        )
    };
    line && ok(s1) && ok(s2)
}

/// The span bounds for `carrier` over `[t0, t1]` at the 9-sample
/// schedule (docs on [`TangentSpanBounds`]).
pub(crate) fn tangent_span_bounds<T: Real>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
) -> Option<TangentSpanBounds<T>> {
    if !tangent_certificate_lane(carrier, s1, s2) {
        return None;
    }
    let Curve3::Line { origin: _, dir } = *carrier else {
        return None;
    };
    let step = (t1 - t0) / T::from_f64(f64::from(crate::certify::CERT_SAMPLES - 1));
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
            Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) => None,
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
