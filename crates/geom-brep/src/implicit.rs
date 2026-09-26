//! Closed-form implicit residuals, gradients, and curvature lever arms
//! for the analytic surfaces — the evaluation substrate of D4 ¶2
//! certification and the dihedral predicate.
//!
//! Every analytic surface kind has a closed-form implicit function whose
//! zero set is the surface. This module exposes it in **dimensionally
//! honest, linearized** form (the M2 PR 1 review's contract): each
//! residual is a length in meters, agreeing with the true signed
//! distance to first order near the surface, so it classifies directly
//! against the run's linear ε — no squared-distance bands, no hidden
//! unit mismatches. The `(q² − r²)/2r` shape is used wherever the
//! natural form is a squared distance: it equals `(|q| − r)·(|q| + r)/2r
//! ≈ |q| − r` near the surface and is smooth through the center (no
//! `abs` kink for the Dual lane to trip on).
//!
//! **Gradients, not chart normals.** [`implicit_gradient`] is the
//! implicit function's spatial gradient — unit-magnitude on the surface,
//! defined off the chart entirely. This is the PR 1 reviewer's contract
//! honored structurally: nothing here ever calls `Surface::normal`, so
//! the cone-apex poison (a chart evaluation) is unreachable from the
//! certification layer. The gradient is still honestly poison exactly
//! where the *surface* is singular (the cone apex has no tangent plane;
//! the gradient's `w/ρ` normalization poisons on the axis) — that is the
//! correct answer, not a limitation.
//!
//! # Formulas (fixed evaluation orders, D9)
//!
//! With `q` the point relative to the surface's anchor, `h = q·axis` the
//! axial component, `w = q − axis·h` the radial component (computed as a
//! vector difference — never as `√(|q|² − h²)`, whose cancellation could
//! go negative and poison), and `ρ = |w|`:
//!
//! | kind | residual (meters) | gradient |
//! |---|---|---|
//! | plane | `(p − origin)·normal` | `normal` |
//! | sphere | `(\|p − c\|² − r²)/2r` | `(p − c)/r` |
//! | cylinder | `(ρ² − r²)/2r` | `w/r` |
//! | cone | `ρ·cos α − \|h\|·sin α` | `(w/ρ)·cos α − axis·copysign(sin α, h)` |
//! | torus | `((ρ − R)² + h² − r²)/2r` | `((ρ − R)·(w/ρ) + axis·h)/r` |
//!
//! The cone's `|h|` makes the residual vanish on **both** nappes (the
//! complete locus, per `geom`'s surface conventions); its residual is
//! the exact perpendicular distance to the generator line in the
//! meridian half-plane through the point. The `Nurbs` placeholder
//! yields poison throughout (representable ≠ implemented — the poison
//! fails certification loudly, D4 ¶2).
//!
//! # Curvature lever arms
//!
//! [`curvature_lever_arm`] is the local "feature scale" an angular
//! comparison at a point turns on (D4 ¶1: an angle means displacement
//! only through a lever arm): the smallest local radius of curvature of
//! the surface at the point. A plane has none — it contributes `+∞`,
//! which is the identity of the `min` lattice the dihedral predicate
//! folds arms with. The cone's arm is the radial distance ρ (→ 0 at the
//! apex: near the apex every feature is tiny and angular classification
//! honestly escalates).
//!
//! # Circle-carrier enclosures, and the one that is SAMPLED
//!
//! The residual of a whole circle or of one arc of it against a
//! surface is enclosed here, for the boolean lane's clearance rungs.
//! Two constructions live behind three doors, and the difference
//! matters to a reader:
//!
//! - **Closed-form**, for the plane, the sphere and the cylinder: the
//!   composed residual is a trigonometric polynomial of degree ≤ 2, so
//!   `circle_residual_harmonics`' `(c₀, A₁, A₂)` bounds both its range
//!   (exactly, for the first-harmonic kinds) and its second
//!   derivative. Nothing is sampled.
//! - **Sampled and CHARGED**, for the torus: the composed residual
//!   carries a `√` of a trigonometric polynomial and has no harmonic
//!   form at all, so [`circle_arc_residual_range`] walks
//!   [`ARC_RESIDUAL_SAMPLES`] sub-arcs and widens the sample hull by
//!   the chord-dip charge `f2·h²/8`. It is a certified enclosure, not
//!   an estimate — but it is a WIDER one, and it is what
//!   [`circle_residual_extremes`] falls back to for that kind.
//!
//! Both are outer bounds, so slack only ever sends a pair to the
//! typed frontier and never clears one that meets; the cost of the
//! second is `K + 1` residual evaluations per call.

use geom::Surface;
use geom_core::{Point3, Real, Vec3};

use crate::enters::OutwardNormal;

/// The scalar's poison value (NaN at `f64`, NaI at the interval scalar).
fn poison<T: Real>() -> T {
    T::from_f64(f64::NAN)
}

/// The all-poison vector.
fn poison_vec<T: Real>() -> Vec3<T> {
    let nan = poison::<T>();
    Vec3::new(nan, nan, nan)
}

/// The axial/radial decomposition `(h, w)` of `p` relative to an anchor
/// point and unit axis: `q = p − anchor`, `h = q·axis`,
/// `w = q − axis·h`. Shared by every axisymmetric form below (fixed
/// order, D9).
fn axial_radial<T: Real>(p: Point3<T>, anchor: Point3<T>, axis: Vec3<T>) -> (T, Vec3<T>) {
    let q = p - anchor;
    let h = q.dot(axis);
    let w = q - axis * h;
    (h, w)
}

/// The linearized implicit residual of `p` against `s`, in meters (the
/// module-doc table). Zero on the surface; agrees with the signed
/// distance to first order near it. Total: poison in, poison out;
/// [`Surface::Nurbs`] yields poison.
pub fn implicit_residual<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    let two = T::from_f64(2.0);
    match *s {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal),
        Surface::Sphere { center, radius, .. } => {
            let d2 = (p - center).norm_squared();
            (d2 - radius.powi(2)) / (two * radius)
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(p, origin, axis);
            (w.norm_squared() - radius.powi(2)) / (two * radius)
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (s_a, c_a) = half_angle.sin_cos();
            let (h, w) = axial_radial(p, apex, axis);
            w.norm() * c_a - h.abs() * s_a
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let d = rho - major_radius;
            // d and h straddle zero at legitimate on-locus points (the
            // tube's top/bottom circle, the equatorial plane): tight
            // squares via powi(2), not d·d/h·h — bit-identical at f64
            // and the dual value channel, honest [0, hi] enclosures at
            // the interval scalar (the norm_squared rationale, M2 PR 3
            // fix pass). minor_radius is positive conventional data —
            // its square is tight either way; powi(2) keeps the
            // square-discipline tripwire's scope clean.
            (d.powi(2) + h.powi(2) - minor_radius.powi(2)) / (two * minor_radius)
        }
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module. `Approx` joins it:
        // the fit is a spline, and an offset description has no
        // implicit form to inherit either.
        Surface::Nurbs(_) | Surface::Approx(_) => poison(),
    }
}

/// The spatial gradient of [`implicit_residual`] at `p` (the module-doc
/// table): unit-magnitude on the surface, so it is the surface's normal
/// direction there — computed entirely from the implicit form, never
/// from the chart (`Surface::normal` is deliberately unreachable from
/// this layer). Honest poison where the surface itself is singular
/// (cone apex / cone axis, torus axis) and for [`Surface::Nurbs`].
pub fn implicit_gradient<T: Real>(s: &Surface<T>, p: Point3<T>) -> Vec3<T> {
    match *s {
        Surface::Plane { normal, .. } => normal,
        Surface::Sphere { center, radius, .. } => (p - center) / radius,
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = axial_radial(p, origin, axis);
            w / radius
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (s_a, c_a) = half_angle.sin_cos();
            let (h, w) = axial_radial(p, apex, axis);
            // w/ρ poisons on the axis (0/0) — including the apex, where
            // no tangent plane exists (honest, per the module docs).
            let w_hat = w / w.norm();
            w_hat * c_a - axis * s_a.copysign(h)
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            (w_hat * (rho - major_radius) + axis * h) / minor_radius
        }
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module.
        // As `Nurbs`, and for the same reason one level in: an
        // approximating surface's stand-in IS a spline, so it has no
        // implicit form either — and its description has none to lend.
        Surface::Nurbs(_) | Surface::Approx(_) => poison_vec(),
    }
}

/// A face's OUTWARD normal at a point on a curved carrier: the
/// implicit gradient at `p`, normalized, folded through the face's
/// `sense` bit — the one home of that fold for a door handed a
/// carrier and the bit rather than a face (the contact verifier
/// reading two bodies' faces, the dihedral's material pairing, a blend
/// battery's supports).
///
/// INVARIANT, enforced by the caller and not here: `p` lies ON `s` (an
/// off-surface gradient is a direction of nothing, and this door reads
/// no residual), and `sense` is a `Face::sense` the caller resolved,
/// never a decided sign. A door that certifies the point onto the
/// chart first folds the RAW gradient it has just certified
/// unit-magnitude; this one normalizes, so the two readings can differ
/// in the last ulps on the same input and neither substitutes for the
/// other.
pub fn implicit_outward_normal<T: Real>(
    s: &Surface<T>,
    sense: bool,
    p: Point3<T>,
) -> OutwardNormal<T> {
    OutwardNormal::from_chart(implicit_gradient(s, p).normalize(), sense)
}

/// The local curvature lever arm of `s` at `p` (module docs): the
/// chart's own length scale, `f64::MAX` for a plane (the practical
/// `min` identity — see the module docs for why not `+∞`), poison for
/// [`Surface::Nurbs`].
///
/// Per kind: sphere/cylinder — the radius; cone — the radial distance
/// ρ of `p` from the axis; torus — the tube radius `r`. It is the scale
/// the implicit forms are normalized by (`|∇F| − 1` is the elevation
/// over it for all four), which is how its readers use it: to meter a
/// dimensionless residual or a relative curvature as a length at the
/// chart's own scale (`face_normal`'s on-chart certificate, the
/// dihedral fold, the contact and SSI jet checks).
///
/// **It is NOT a bound on the curvature on a fat torus.** On a ring
/// with `R < 2r` the inner equator bends at `1/(R − r)`, harder than
/// the tube's `1/r`. A reader that charges a SAGITTA against the
/// tightest bend wants [`min_radius_of_curvature`] instead, and the two
/// must not be swapped: a smaller value here LOOSENS the readers above
/// (their margins shrink toward Zero), while a smaller radius of
/// curvature tightens a sagitta charge.
pub fn curvature_lever_arm<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    match *s {
        Surface::Plane { .. } => T::from_f64(f64::MAX),
        Surface::Sphere { radius, .. } | Surface::Cylinder { radius, .. } => radius,
        Surface::Cone { apex, axis, .. } => {
            let (_, w) = axial_radial(p, apex, axis);
            w.norm()
        }
        Surface::Torus { minor_radius, .. } => minor_radius,
        // STAYS poison after M5 PR 3 gave the variant a payload: a NURBS
        // carrier has no implicit form — foot-point machinery (C2.1,
        // M5 PR 4) owns that story, not this module.
        // As `Nurbs`: the stand-in is a spline, so no implicit form —
        // and the offset description has no closed lever arm to lend.
        Surface::Nurbs(_) | Surface::Approx(_) => poison(),
    }
}

/// **The smallest radius of curvature of `s`**, for a reader that
/// charges a sagitta against the tightest bend — a lower bound on every
/// normal curvature radius, `f64::MAX` for a plane, poison for
/// [`Surface::Nurbs`].
///
/// Per kind: sphere/cylinder — the radius; cone — the radial distance
/// ρ of `p` (a conservative bound on the osculating radius ρ/cos α);
/// torus — `min(r, R − r)`, the smallest principal radius anywhere on
/// the ring. The tube radius `r` is one principal radius everywhere;
/// the other is `r·ρ/|ρ − R|`, which is at least `ρ` and reaches `R − r`
/// on the inner equator, so a fat ring (`R < 2r`) curves hardest ALONG
/// its inner equator. The bound is global rather than local to `p`, the
/// conservative direction: a sagitta is charged over an arm that leaves
/// `p`. A spindle or horn torus (`R ≤ r`) has a curvature singularity on
/// the axis and gets ZERO, so every charge read from it refuses.
pub fn min_radius_of_curvature<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    match *s {
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => minor_radius.min((major_radius - minor_radius).max(T::zero())),
        _ => curvature_lever_arm(s, p),
    }
}

/// The quadratic form `dᵀ (∇²F) d` of [`implicit_residual`]'s Hessian
/// at `p`, along direction `d` (NOT normalized — the form is
/// homogeneous of degree 2 in `d`). With `d` a unit surface tangent,
/// `dᵀ∇²F d / |∇F|` is the surface's **normal curvature** along `d`
/// (signed against the outward implicit gradient) — the second-order
/// jet datum C7's tangency schedule and the second-order sector
/// trilean consume (M5 PR 9).
///
/// Derived per kind from the module-doc forms (fixed order, D9);
/// squares of possibly-zero components go through `powi(2)` (the
/// interval-square rule). Honest poison at surface singularities
/// (cone axis, torus axis) and for [`Surface::Nurbs`].
pub fn implicit_hessian_form<T: Real>(s: &Surface<T>, p: Point3<T>, d: Vec3<T>) -> T {
    match *s {
        // F = (p − o)·n̂: linear, Hessian 0.
        Surface::Plane { .. } => T::zero(),
        // F = (|q|² − r²)/2r: ∇²F = I/r.
        Surface::Sphere { radius, .. } => d.norm_squared() / radius,
        // F = (|w|² − r²)/2r, w = q − a(q·a): ∇²F = (I − aaᵀ)/r.
        Surface::Cylinder { axis, radius, .. } => {
            let d_ax = d.dot(axis);
            (d.norm_squared() - d_ax.powi(2)) / radius
        }
        // F = |w|·cos α − |h|·sin α: away from h = 0 the |h| term is
        // linear; ∇²(|w|) = (I − aaᵀ − ŵŵᵀ)/ρ. Poison on the axis
        // (ρ = 0), as the gradient already is.
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (_, c_a) = half_angle.sin_cos();
            let (_, w) = axial_radial(p, apex, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            let d_ax = d.dot(axis);
            let d_w = d.dot(w_hat);
            (d.norm_squared() - d_ax.powi(2) - d_w.powi(2)) * c_a / rho
        }
        // F = ((ρ − R)² + h² − r²)/2r: ∇²F = ((I − aaᵀ − ŵŵᵀ)·(ρ − R)/ρ
        // + ŵŵᵀ + aaᵀ)/r. Poison on the axis (ρ = 0).
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (_, w) = axial_radial(p, center, axis);
            let rho = w.norm();
            let w_hat = w / rho;
            let d_ax = d.dot(axis);
            let d_w = d.dot(w_hat);
            let d_perp2 = d.norm_squared() - d_ax.powi(2) - d_w.powi(2);
            (d_perp2 * (rho - major_radius) / rho + d_w.powi(2) + d_ax.powi(2)) / minor_radius
        }
        // As `implicit_residual`: no implicit form, so no Hessian of
        // one — for the spline fit or the description behind it.
        Surface::Nurbs(_) | Surface::Approx(_) => poison(),
    }
}

/// The **largest normal-curvature magnitude** of `s` at `p` over its
/// tangent plane (1/meters) — the direction-free second-order datum
/// the C12.2 tangent-contact descent classifies against (M5 PR 9):
/// zero iff the surface osculates its tangent plane (locally
/// plane-like — the under-determined case), definitely positive iff
/// it bends off it in SOME direction.
///
/// Branch-free (no basis choice — D9/equivariance): the Hessian is
/// assembled from six [`implicit_hessian_form`] evaluations by
/// polarization; the tangent-plane restriction's eigen extremum comes
/// from the two invariants `tr_r = tr H − n̂ᵀHn̂` and
/// `det_r = n̂ᵀ adj(H) n̂` (the adjugate identity), giving
/// `|λ|max = |tr_r/2| + √((tr_r/2)² − det_r)`, divided by `|∇F|`.
/// Poison in, poison out (singular points, `Nurbs`).
pub fn implicit_max_normal_curvature<T: Real>(s: &Surface<T>, p: Point3<T>) -> T {
    let g = implicit_gradient(s, p);
    let n_hat = g / g.norm();
    let ex = Vec3::new(T::one(), T::zero(), T::zero());
    let ey = Vec3::new(T::zero(), T::one(), T::zero());
    let ez = Vec3::new(T::zero(), T::zero(), T::one());
    let hxx = implicit_hessian_form(s, p, ex);
    let hyy = implicit_hessian_form(s, p, ey);
    let hzz = implicit_hessian_form(s, p, ez);
    let two = T::from_f64(2.0);
    let hxy = (implicit_hessian_form(s, p, ex + ey) - hxx - hyy) / two;
    let hyz = (implicit_hessian_form(s, p, ey + ez) - hyy - hzz) / two;
    let hxz = (implicit_hessian_form(s, p, ex + ez) - hxx - hzz) / two;
    // Restricted trace: tr H − n̂ᵀHn̂.
    let n_form = implicit_hessian_form(s, p, n_hat);
    let tr_r = hxx + hyy + hzz - n_form;
    // Restricted determinant: n̂ᵀ adj(H) n̂ (cofactors, fixed order).
    let adj_xx = hyy * hzz - hyz.powi(2);
    let adj_yy = hxx * hzz - hxz.powi(2);
    let adj_zz = hxx * hyy - hxy.powi(2);
    let adj_xy = hxz * hyz - hxy * hzz;
    let adj_yz = hxy * hxz - hyz * hxx;
    let adj_xz = hxy * hyz - hyy * hxz;
    let det_r = adj_xx * n_hat.x.powi(2)
        + adj_yy * n_hat.y.powi(2)
        + adj_zz * n_hat.z.powi(2)
        + two
            * (adj_xy * n_hat.x * n_hat.y
                + adj_yz * n_hat.y * n_hat.z
                + adj_xz * n_hat.x * n_hat.z);
    let half_tr = tr_r / two;
    // (tr/2)² − det ≥ 0 in ℝ (a real symmetric restriction); the
    // clamp guards rounding, powi keeps the interval square tight.
    let disc = (half_tr.powi(2) - det_r).max(T::zero());
    (half_tr.abs() + disc.sqrt()) / g.norm()
}

/// The subdivision count [`circle_arc_residual_range`] uses, and with
/// it the **resolution law** of every circle-versus-surface clearance
/// the boolean lane decides: over an arc of angular span `Δθ` whose
/// composed residual has second-derivative bound `f2`, the enclosure
/// is the sample hull widened by the chord-dip charge
/// `f2·(Δθ/K)²/8`, so an arc whose true clearance exceeds that charge
/// plus the run's definite threshold clears. The converse does not
/// hold and is not claimed: the sample hull is itself an inner bound
/// on the true range, so an arc can fail to clear for want of a
/// sample rather than for want of room. "The run's definite
/// threshold" is `Band::escalate` — `K_band·ε`, not `ε`, since a
/// margin inside the ambiguity band escalates rather than deciding.
///
/// The charge falls as `K⁻²`. The cost is `K + 1` residual
/// evaluations per call and TWO calls per examined circle ×
/// curved-face pair — the reduction reads the whole carrier and the
/// edge's own arc — so **514 evaluations** at this constant.
///
/// `K = 256` is the ratified value (`docs/CURVED-TORUS-SPEC.md`
/// §PR-2, ruling 3). It is set by the tightest measured consumer: the
/// lily stem's 22° outer-equator seam against the arch's torus
/// carrier clears by 8.6 mm with an arc-scoped `f2` of ~1.4e3 m/rad²,
/// a charge of 0.39 mm — and still resolves against the much looser
/// full-carrier `f2` of 8.36e3, whose charge is 2.3 mm. An adaptive
/// count is not offered until a consumer needs a finer arc than this
/// law admits.
pub const ARC_RESIDUAL_SAMPLES: usize = 256;

/// **The chord-dip charge, spelled once for this crate**: how far a
/// C² function of second-derivative bound `f2` can leave the chord of
/// a sub-interval of width `step` — `f2·step²/8`.
///
/// `topo`'s `boolean::boxes::subdivision_charge` is the same quantity
/// in the crate above, and cannot be depended on from here; the two
/// spellings are one class, filed as
/// `work/curved/the-chord-dip-charge-has-two-homes.md`.
pub(crate) fn chord_dip_charge<T: Real>(f2: T, step: T) -> T {
    f2 * step.powi(2) * T::from_f64(0.125)
}

/// The parameter of sample `k` of the arc schedule, **spelled once**.
///
/// `K + 1` parameters across `[t₀, t₁]` in the LERP form, so that
/// `sample(0)` is exactly `t₀` and `sample(K)` is exactly `t₁`. That
/// exactness is load-bearing, not tidiness: the chord-dip argument
/// charges each of the `K` sub-intervals against BOTH of its ends, so
/// a schedule that stops one parameter short leaves the last
/// sub-interval uncharged and the enclosure unsound — a defect no
/// fixture row can see, since it hides wherever the residual's
/// extreme sits in the final cell. Pinned by
/// `the_sample_schedule_reaches_both_ends_of_the_arc`.
fn arc_sample<T: Real>(t0: T, t1: T, k: usize) -> T {
    let f = T::from_f64(k as f64) / T::from_f64(ARC_RESIDUAL_SAMPLES as f64);
    t0 * (T::one() - f) + t1 * f
}

/// One walk of the schedule, carrying everything BOTH arc readers
/// need: the residual's sample hull, and — for the torus, whose
/// curvature bound is arc-scoped — the radial and axial ranges over
/// the same samples.
///
/// One walk rather than two. The bound and the enclosure are read off
/// the same parameters by construction, so they cannot disagree about
/// which arc they describe, and a torus pair pays `K + 1` residual
/// evaluations instead of `2(K + 1)`.
struct ArcScan<T> {
    /// `(min, max)` of [`implicit_residual`] over the samples.
    residual: (T, T),
    /// `(min, max)` of `|w|` about the torus axis; `(0, 0)` for every
    /// other kind, which reads neither field.
    rho: (T, T),
    /// `max |q·n|` about the torus axis.
    h_abs_hi: T,
}

fn scan_arc<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
) -> ArcScan<T> {
    let v = axis.cross(u_ref);
    let hub = match *s {
        Surface::Torus {
            center: tc,
            axis: tn,
            ..
        } => Some((tc, tn)),
        _ => None,
    };
    let at = |k: usize| {
        let (sin, cos) = arc_sample(t0, t1, k).sin_cos();
        let p = center + u_ref * (radius * cos) + v * (radius * sin);
        let (rho, h_abs) = hub.map_or((T::zero(), T::zero()), |(tc, tn)| {
            let (h, w) = axial_radial(p, tc, tn);
            (w.norm(), h.abs())
        });
        (implicit_residual(s, p), rho, h_abs)
    };
    let (r0, rho0, h0) = at(0);
    let mut scan = ArcScan {
        residual: (r0, r0),
        rho: (rho0, rho0),
        h_abs_hi: h0,
    };
    for k in 1..=ARC_RESIDUAL_SAMPLES {
        let (r, rho, h_abs) = at(k);
        scan.residual = (scan.residual.0.min(r), scan.residual.1.max(r));
        scan.rho = (scan.rho.0.min(rho), scan.rho.1.max(rho));
        scan.h_abs_hi = scan.h_abs_hi.max(h_abs);
    }
    scan
}

/// A conservative enclosure of [`implicit_residual`] over an **arc**
/// `θ ∈ [t₀, t₁]` of the circle carrier
/// `C(θ) = center + radius·(û·cosθ + v̂·sinθ)`, `v̂ = axis × û`, by
/// certified subdivision.
///
/// The residual is sampled at [`ARC_RESIDUAL_SAMPLES`] + 1 parameters
/// spanning the arc and the sample hull is widened by the chord-dip
/// charge of one sub-arc: a C² function leaves the chord of a
/// sub-interval of width `h` by at most `max|F″|·h²/8`, and `f2`
/// encloses `|F″|` over this arc. It is `arc_extent`'s doctrine in
/// residual space, and the consumer's two-endpoint chord dip is its
/// `K = 1` instance.
///
/// **Precondition on the frame, unchecked:** `axis` and `u_ref` must
/// be unit and mutually orthogonal, as every `Curve3::Circle` minted
/// in this tree is. Nothing here normalizes them, and nothing can
/// afford to: the carrier's own `radius` is the `ρ_c` every term of
/// the curvature bound is stated at, so a non-unit `u_ref` rescales
/// the sampled curve without rescaling the bound and the enclosure
/// stops enclosing. A caller synthesising a frame owes the
/// normalization.
///
/// Returns `(lo, hi)` in METERS (the residual's own linearized units),
/// or `None` for the kinds with no curvature bound at all (cone,
/// NURBS, `Approx`) — the caller keeps its frontier door there.
///
/// Total arithmetic: poison in, poison out. An arc whose `f2` is
/// infinite — a circle that may reach a torus's axis, where the
/// residual has a kink and no finite `|F″|` exists — returns the
/// infinite enclosure by the arithmetic, with no branch to get wrong.
#[must_use]
pub fn circle_arc_residual_range<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
) -> Option<(T, T)> {
    let scan = scan_arc(s, center, axis, radius, u_ref, t0, t1);
    let f2 = match circle_residual_harmonics(s, center, axis, radius, u_ref) {
        Some((_, a1, a2)) => a1 + T::from_f64(4.0) * a2,
        None => torus_curvature_bound(s, axis, radius, u_ref, t0, t1, &scan)?,
    };
    let step = (t1 - t0) / T::from_f64(ARC_RESIDUAL_SAMPLES as f64);
    let charge = chord_dip_charge(f2, step);
    Some((scan.residual.0 - charge, scan.residual.1 + charge))
}

/// A conservative enclosure of [`implicit_residual`] over an ENTIRE
/// circle carrier `C(θ) = center + radius·(û·cosθ + v̂·sinθ)`,
/// `v̂ = axis × û` — the M6 door-A rider's algebra, shared with
/// `tangent.rs`'s circle arm: against a **sphere** the composed
/// squared distance is an EXACT first harmonic in θ; against a
/// **cylinder** the squared axis distance is a degree-≤2
/// trigonometric polynomial whose harmonic amplitudes bound its
/// range. Both enclose (sphere tightly, cylinder conservatively —
/// slack only ever widens the returned range, which sends more pairs
/// to the typed frontier, never fewer).
///
/// A **torus** has no harmonic form — the composed residual carries a
/// `√` of a trigonometric polynomial — so the whole turn is enclosed
/// by [`circle_arc_residual_range`] over `[0, τ]` instead. That
/// enclosure is a sampled one and is looser than a closed form would
/// be, in the direction that refuses rather than clears. The two
/// answers are therefore not the same KIND of answer behind one name,
/// and a caller that needs to know which it got must ask the surface
/// (Q7's class; the doc says it here rather than splitting the door).
///
/// The frame precondition of [`circle_arc_residual_range`] binds here
/// too: `axis` and `u_ref` unit and mutually orthogonal, unchecked.
///
/// Returns `(lo, hi)` in METERS (the residual's own linearized
/// units), or `None` for kinds with neither form (cone, NURBS,
/// `Approx`) — the caller keeps its frontier door there. Total
/// arithmetic: poison in, poison out.
#[must_use]
pub fn circle_residual_extremes<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
) -> Option<(T, T)> {
    if let Some((c0, a1, a2)) = circle_residual_harmonics(s, center, axis, radius, u_ref) {
        return Some((c0 - a1 - a2, c0 + a1 + a2));
    }
    circle_arc_residual_range(s, center, axis, radius, u_ref, T::zero(), T::tau())
}

/// A bound on `|d²F/dθ²|` for [`implicit_residual`] composed with the
/// same circle carrier over the WHOLE turn — the curvature term an
/// ARC-SCOPED clearance needs. [`circle_arc_residual_range`] uses the
/// arc-scoped form of the same bound.
///
/// Against a plane/sphere/cylinder the composed residual is
/// `c₀ + A₁cos(θ−φ₁) + A₂cos(2θ−φ₂)` (the second harmonic present
/// only against a cylinder), so differentiating twice multiplies the
/// harmonics by `1` and `4`: `|F″| ≤ A₁ + 4A₂`. Against a torus the
/// bound is the certified one of `docs/CURVED-TORUS-SPEC.md` §PR-2
/// (see `torus_curvature_bound`). With it, a smooth function's dip
/// below its ENDPOINT CHORD is at most `|F″|max·Δθ²/8` — the same
/// total-arithmetic bound the line row uses along a segment, and the
/// reason an arc can clear where the whole circle it rides cannot.
///
/// The frame precondition of [`circle_arc_residual_range`] binds here
/// too: `axis` and `u_ref` unit and mutually orthogonal, unchecked.
///
/// `None` for the kinds with no bound at all (cone, NURBS, `Approx`).
/// Total arithmetic: poison in, poison out.
#[must_use]
pub fn circle_residual_curvature_bound<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
) -> Option<T> {
    arc_curvature_bound(s, center, axis, radius, u_ref, T::zero(), T::tau())
}

/// [`circle_residual_curvature_bound`]'s arc-scoped form: an
/// enclosure of `|F″|` over `θ ∈ [t₀, t₁]` only.
///
/// The harmonic kinds ignore the arc — their bound is a property of
/// the carrier, and restricting it would need the harmonic phases —
/// so only the torus walks the schedule here.
fn arc_curvature_bound<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
) -> Option<T> {
    if let Some((_, a1, a2)) = circle_residual_harmonics(s, center, axis, radius, u_ref) {
        return Some(a1 + T::from_f64(4.0) * a2);
    }
    let scan = scan_arc(s, center, axis, radius, u_ref, t0, t1);
    torus_curvature_bound(s, axis, radius, u_ref, t0, t1, &scan)
}

/// The **torus** arm of the curvature bound, over the arc the scan
/// walked. `None` for every other kind.
///
/// This is where the arc earns its keep. With `q = C(θ) − c`,
/// `h = q·n`, `w = q − h·n`, `ρ = |w|` and `d² = (ρ − R)² + h²`, the
/// residual is `(d² − r²)/2r`, so
/// `(d²)″ = 2[(ρ′)² + (ρ−R)ρ″] + 2[(h′)² + h·h″]` and
///
/// ```text
/// |(d²)″| ≤ 2ρ_c² + 2·D_max·(ρ_c + 2ρ_c²/ρ_min) + 2a_h² + 2·H_max·a_h
/// ```
///
/// Every term is certified: `|w′|, |w″| ≤ ρ_c` because a
/// perpendicular projection is a contraction of `C′`, whose length is
/// the circle's radius `ρ_c`; `ρ′ = w·w′/ρ` gives `|ρ′| ≤ ρ_c` and
/// `ρ″ = (|w′|² + w·w″)/ρ − (w·w′)²/ρ³` gives
/// `|ρ″| ≤ ρ_c + 2ρ_c²/ρ_min`; `h` is an EXACT first harmonic of
/// amplitude `a_h`, so `|h′|, |h″| ≤ a_h` on any arc at all.
///
/// `D_max` is TWO-SIDED (`max` over both ends of the `ρ` range, not
/// the far end alone) because `|ρ − R|` is largest at whichever end
/// stands further from the spine, and on a circle that crosses the
/// spine radius that is the INNER end. `H_max` is the range of `|h|`,
/// not of `h`: the product `h·h″` is bounded by `|h|·|h″|`, and a
/// signed range whose maximum is negative would bound it by a
/// negative number.
///
/// `ρ_min`, `ρ_max` and `H_max` come from the scan's samples widened
/// by the Lipschitz charge `ρ_c·h/2`, since every parameter of the arc
/// is within half a step of a sample and both `ρ` and `h` are
/// `ρ_c`-Lipschitz. Scoping them to the arc is what makes the bound
/// usable: on the lily's stem seam the full-carrier `D_max` is 7.86 m
/// against the arc's 0.95 m, an eight-fold difference in the dominant
/// term.
///
/// `ρ_min = 0` — an arc that may reach the axis — divides by zero and
/// yields an infinite (interval lane: poisoned) bound. That is the
/// honest answer: the residual has a kink on the axis. No branch
/// tests for it.
fn torus_curvature_bound<T: Real>(
    s: &Surface<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    scan: &ArcScan<T>,
) -> Option<T> {
    let Surface::Torus {
        axis: tn,
        major_radius,
        minor_radius,
        ..
    } = *s
    else {
        return None;
    };
    let two = T::from_f64(2.0);
    let v = axis.cross(u_ref);
    let a_h = radius * (u_ref.dot(tn).powi(2) + v.dot(tn).powi(2)).sqrt();
    let step = (t1 - t0) / T::from_f64(ARC_RESIDUAL_SAMPLES as f64);
    let lipschitz = radius * step.abs() / two;
    // A radius is never negative, so the clamp is the honest floor
    // rather than a guard — and clamping to zero is what hands the
    // through-axis case its infinity.
    let rho_min = (scan.rho.0 - lipschitz).max(T::zero());
    let rho_max = scan.rho.1 + lipschitz;
    let h_max = scan.h_abs_hi + lipschitz;
    let d_max = (rho_min - major_radius)
        .abs()
        .max((rho_max - major_radius).abs());
    let rho_second = radius + two * radius.powi(2) / rho_min;
    let d2_second =
        two * radius.powi(2) + two * d_max * rho_second + two * a_h.powi(2) + two * h_max * a_h;
    Some(d2_second / (two * minor_radius))
}

/// The composed residual's harmonic decomposition in RESIDUAL units:
/// `(c₀, A₁, A₂)` of `c₀ + A₁cos(θ−φ₁) + A₂cos(2θ−φ₂)`. The whole-turn
/// range and the curvature bound are both views of this one algebra
/// where it exists, so they cannot drift apart.
///
/// It exists for the plane, the sphere and the cylinder and for no
/// other kind. The **torus** answers `None` here and is served by
/// `arc_curvature_bound`'s own arm instead: the composed residual
/// carries `√` of a trigonometric polynomial (the distance to the
/// spine circle), which is not a trigonometric polynomial and has no
/// harmonic triple to report. Answering one would be a lie, not a
/// widening.
fn circle_residual_harmonics<T: Real>(
    s: &Surface<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
) -> Option<(T, T, T)> {
    let two = T::from_f64(2.0);
    let u = u_ref;
    let v = axis.cross(u_ref);
    let amp = |a: T, b: T| (a.powi(2) + b.powi(2)).sqrt();
    match *s {
        Surface::Plane { origin, normal, .. } => {
            let c0 = (center - origin).dot(normal);
            let a1 = radius * amp(u.dot(normal), v.dot(normal));
            Some((c0, a1, T::zero()))
        }
        Surface::Sphere {
            center: sc,
            radius: r,
            ..
        } => {
            // |C(θ) − sc|² = |e|² + R_c² + 2R_c(e·û cosθ + e·v̂ sinθ):
            // û ⊥ v̂ unit makes the θ-dependence a pure first
            // harmonic, so the range below is EXACT.
            let e = center - sc;
            let c0 = e.norm_squared() + radius.powi(2);
            let a1 = two * radius * amp(e.dot(u), e.dot(v));
            Some(((c0 - r.powi(2)) / (two * r), a1 / (two * r), T::zero()))
        }
        Surface::Cylinder {
            origin,
            axis: a,
            radius: r,
            ..
        } => {
            // The radial part w(θ) = perp(e) + R_c(perp(û)cosθ +
            // perp(v̂)sinθ) has |w|² of trigonometric degree ≤ 2; its
            // constant term and harmonic amplitudes are exact, and
            // |A₁ cos + B₁ sin| + |second harmonic| bounds the swing.
            // Named binding so the interval-square tripwire's grep does
            // not false-positive on `a * a.dot(x)` (vector × projection
            // coefficient, not a scalar square) — the blend.rs precedent.
            let perp = |x: Vec3<T>| {
                let along = a.dot(x);
                x - a * along
            };
            let e = perp(center - origin);
            let up = perp(u);
            let vp = perp(v);
            let c0 =
                e.norm_squared() + radius.powi(2) * (up.norm_squared() + vp.norm_squared()) / two;
            let a1 = two * radius * amp(e.dot(up), e.dot(vp));
            let a2 =
                radius.powi(2) * amp((up.norm_squared() - vp.norm_squared()) / two, up.dot(vp));
            Some(((c0 - r.powi(2)) / (two * r), a1 / (two * r), a2 / (two * r)))
        }
        // `Approx` joins the no-closed-form group: the fit is a spline
        // and the description's offset locus has no harmonic residual.
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            None
        }
    }
}

/// The seam frame of an axisymmetric surface: `(w, u_ref, v_ref)` with
/// `w` the radial component of `p` relative to the surface's own
/// anchor/axis and `v_ref = axis × u_ref` — the pieces the
/// a seam chart image residuals are built from. `None` for
/// the plane (not periodic — a seam description on it is malformed) and
/// for [`Surface::Nurbs`] (unimplemented).
pub(crate) fn seam_frame<T: Real>(
    s: &Surface<T>,
    p: Point3<T>,
) -> Option<(Vec3<T>, Vec3<T>, Vec3<T>)> {
    let (anchor, axis, u_ref) = match *s {
        // Nurbs: no implicit/seam form (C2.1 foot points, M5 PR 4).
        // Approx: neither — its stand-in is a spline, and an offset
        // description carries no axis to hang a seam frame on.
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => return None,
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => (origin, axis, u_ref),
        Surface::Cone {
            apex, axis, u_ref, ..
        } => (apex, axis, u_ref),
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
        Surface::Torus {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
    };
    let (_, w) = axial_radial(p, anchor, axis);
    Some((w, u_ref, axis.cross(u_ref)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::FRAC_PI_6;

    use proptest::prelude::*;

    use super::*;

    /// **The torus's smallest radius of curvature bounds every bend, on
    /// a fat ring too.** It is read as a radius a sagitta is charged
    /// against, so it must not exceed `1/|κ|` for any normal curvature
    /// `κ` the surface has. On a fat ring (`R = 0.8`, `r = 0.5` — the
    /// dumbbell waist) the inner equator bends at `1/(R − r) = 1/0.3`
    /// along the parallel, harder than the tube's `1/r = 1/0.5` across
    /// it. The normal curvature is read off the residual's own Hessian
    /// (`dᵀ∇²F d / |∇F|`, `|∇F| = 1` on the surface), so the row does
    /// not restate the lever's formula.
    #[test]
    fn the_torus_min_radius_of_curvature_bounds_every_normal_curvature() {
        for (big_r, r) in [(0.8, 0.5), (2.0, 0.5), (1.2, 0.3)] {
            let s = Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 1.0, 0.0),
                major_radius: big_r,
                minor_radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            // Around the tube at 24 minor angles, both principal
            // directions: along the parallel (z) and across the tube.
            for k in 0..24 {
                let v = f64::from(k) * core::f64::consts::TAU / 24.0;
                let (sv, cv) = v.sin_cos();
                let p = Point3::new(big_r + r * cv, r * sv, 0.0);
                let across = Vec3::new(-sv, cv, 0.0);
                let along = Vec3::new(0.0, 0.0, 1.0);
                let lever = min_radius_of_curvature(&s, p);
                for d in [along, across] {
                    let kappa = implicit_hessian_form(&s, p, d).abs();
                    assert!(
                        lever * kappa <= 1.0 + 1e-12,
                        "R = {big_r}, r = {r}, v = {v}: lever {lever} exceeds the radius \
                         of curvature 1/{kappa} along {d:?}"
                    );
                }
            }
        }
    }

    /// **The chart length scale stays the tube radius, and the two
    /// quantities stay apart.** `curvature_lever_arm` is what turns
    /// `|∇F| − 1` into metres (`face_normal`'s on-chart certificate), so
    /// on a torus it must be exactly `r`: a point `δ` off the tube reads
    /// `|∇F| − 1 = δ/r`, and levering by anything smaller would accept
    /// points further off. `min_radius_of_curvature` is the sagitta
    /// reader's quantity; on a horn or spindle torus it is ZERO, so
    /// every charge read from it refuses.
    #[test]
    fn the_torus_chart_scale_and_its_curvature_bound_are_different_quantities() {
        let torus = |big_r: f64, r: f64| Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let fat = torus(0.8, 0.5);
        for (v, delta) in [(0.4_f64, 1e-3), (2.9, -2e-3), (3.1, 5e-4)] {
            let (sv, cv) = v.sin_cos();
            let p = Point3::new(0.8 + (0.5 + delta) * cv, (0.5 + delta) * sv, 0.0);
            let lever = curvature_lever_arm(&fat, p);
            assert_eq!(lever, 0.5, "the chart scale is the tube radius");
            let reading = implicit_gradient(&fat, p).norm() - 1.0;
            assert!(
                (reading * lever - delta).abs() < 1e-12,
                "|∇F| − 1 levered by the chart scale is the elevation: {reading} · {lever} vs {delta}"
            );
        }
        assert!((min_radius_of_curvature(&fat, Point3::new(0.3, 0.0, 0.0)) - 0.3).abs() < 1e-15);
        for (big_r, r) in [(0.5, 0.5), (0.3, 0.5)] {
            assert_eq!(
                min_radius_of_curvature(&torus(big_r, r), Point3::new(big_r + r, 0.0, 0.0)),
                0.0,
                "R = {big_r}, r = {r}: a horn or spindle torus has no curvature bound"
            );
        }
    }

    /// The exactly orthonormal tilted frame from PR 1's fixtures
    /// (integer Pythagorean triple over 3).
    fn t_axis() -> Vec3<f64> {
        Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
    }

    fn t_uref() -> Vec3<f64> {
        Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
    }

    fn t_center() -> Point3<f64> {
        Point3::new(-0.5, 4.0, 1.25)
    }

    fn all_curved() -> Vec<Surface<f64>> {
        vec![
            Surface::Cylinder {
                origin: t_center(),
                axis: t_axis(),
                radius: 2.5,
                u_ref: t_uref(),
            },
            Surface::Cone {
                apex: t_center(),
                axis: t_axis(),
                half_angle: FRAC_PI_6,
                u_ref: t_uref(),
            },
            Surface::Sphere {
                center: t_center(),
                radius: 2.5,
                axis: t_axis(),
                u_ref: t_uref(),
            },
            Surface::Torus {
                center: t_center(),
                axis: t_axis(),
                major_radius: 3.0,
                minor_radius: 1.25,
                u_ref: t_uref(),
            },
        ]
    }

    proptest! {
        /// On-surface points have residual ~0 and unit-magnitude
        /// gradient, for every kind, at chart-generated samples (away
        /// from chart singularities).
        #[test]
        fn residual_zero_and_gradient_unit_on_surface(
            u in -3.0..3.0f64,
            v in 0.25..1.5f64,
        ) {
            for s in all_curved() {
                let p = s.eval(u, v);
                let r = implicit_residual(&s, p);
                prop_assert!(r.abs() <= 1e-12, "{s:?}: residual {r}");
                let g = implicit_gradient(&s, p);
                prop_assert!((g.norm() - 1.0).abs() <= 1e-12, "{s:?}: |grad| {}", g.norm());
            }
            let plane = Surface::Plane { origin: t_center(), normal: t_axis(), u_ref: t_uref() };
            let p = plane.eval(u, v);
            prop_assert!(implicit_residual(&plane, p).abs() <= 1e-12);
        }

        /// The residual agrees with the true signed distance to first
        /// order: stepping δ along the gradient from an on-surface point
        /// changes the residual by ≈ δ.
        #[test]
        fn residual_is_first_order_distance(
            u in -3.0..3.0f64,
            v in 0.5..1.5f64,
            delta in -1e-4..1e-4f64,
        ) {
            for s in all_curved() {
                let p0 = s.eval(u, v);
                let g = implicit_gradient(&s, p0);
                let p = p0 + g * delta;
                let r = implicit_residual(&s, p);
                prop_assert!(
                    (r - delta).abs() <= 1e-7 * (1.0 + delta.abs()),
                    "{s:?}: residual {r} vs step {delta}"
                );
            }
        }

        /// The implicit gradient matches the chart normal (up to sign
        /// conventions they are the same direction) at regular points.
        #[test]
        fn gradient_matches_chart_normal(u in -3.0..3.0f64, v in 0.5..1.5f64) {
            for s in all_curved() {
                let p = s.eval(u, v);
                let g = implicit_gradient(&s, p);
                let n = s.normal(u, v);
                // Both unit; chart orientation for these variants is the
                // outward one, matching the gradient of "inside < 0".
                prop_assert!(g.cross(n).norm() <= 1e-10, "{s:?}");
                prop_assert!(g.dot(n) > 0.0, "{s:?}: sign flip");
            }
        }
    }

    #[test]
    fn cone_residual_covers_both_nappes_and_apex() {
        let cone = Surface::Cone {
            apex: t_center(),
            axis: t_axis(),
            half_angle: FRAC_PI_6,
            u_ref: t_uref(),
        };
        // Mirror nappe (v < 0) is on the locus too.
        let p = cone.eval(1.0, -0.75);
        assert!(implicit_residual(&cone, p).abs() <= 1e-13);
        // The apex is on the locus (residual 0, no poison)...
        assert_eq!(implicit_residual(&cone, t_center()), 0.0);
        // ...but has no tangent plane: the gradient is honest poison.
        let g = implicit_gradient(&cone, t_center());
        assert!(g.x.is_nan() && g.y.is_nan() && g.z.is_nan());
        // And the curvature arm collapses to zero at the apex.
        assert_eq!(curvature_lever_arm(&cone, t_center()), 0.0);
    }

    #[test]
    fn plane_arm_is_max_finite_and_nurbs_poisons() {
        let plane: Surface<f64> = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        // The practical min identity — finite so the interval lane's
        // enclosure stays well-formed (module docs).
        assert_eq!(curvature_lever_arm(&plane, Point3::origin()), f64::MAX);
        let n: Surface<f64> = Surface::nurbs_placeholder();
        assert!(implicit_residual(&n, Point3::origin()).is_nan());
        assert!(implicit_gradient(&n, Point3::origin()).x.is_nan());
        assert!(curvature_lever_arm(&n, Point3::origin()).is_nan());
    }

    #[test]
    fn off_surface_residuals_report_metric_distance() {
        // A unit-ish sphere: a point 0.1 outside reports ≈ +0.1 (the
        // linearized form is (d² − r²)/2r = (d − r)(d + r)/2r).
        let s = Surface::Sphere {
            center: Point3::origin(),
            radius: 2.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let p = Point3::new(2.1, 0.0, 0.0);
        let r = implicit_residual(&s, p);
        assert!((r - 0.1025).abs() < 1e-12); // (2.1² − 4)/4 exactly
        // Inside is negative.
        assert!(implicit_residual(&s, Point3::new(1.9, 0.0, 0.0)) < 0.0);
    }

    /// The M6 rider's algebra: the returned range ENCLOSES every
    /// sampled residual over the circle (all three closed-form
    /// kinds), and the sphere arm — an exact first harmonic — is
    /// TIGHT: dense sampling attains both ends to rounding.
    #[test]
    fn circle_residual_extremes_enclose_and_the_sphere_arm_is_tight() {
        let center = Point3::new(0.4, -0.2, 0.7);
        let axis = Vec3::new(1.0, 2.0, 2.0).normalize();
        let u_ref = axis.cross(Vec3::unit_z()).normalize();
        let radius = 0.8;
        let eval = |t: f64| {
            let v = axis.cross(u_ref);
            center + (u_ref * t.cos() + v * t.sin()) * radius
        };
        let kinds: Vec<Surface<f64>> = vec![
            Surface::Plane {
                origin: Point3::new(0.1, 0.0, 0.0),
                normal: Vec3::new(0.2, -1.0, 0.4).normalize(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Sphere {
                center: Point3::new(1.5, 0.3, -0.2),
                radius: 0.9,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Cylinder {
                origin: Point3::new(-0.5, 1.0, 0.2),
                axis: Vec3::new(0.3, 0.1, 1.0).normalize(),
                radius: 0.6,
                u_ref: Vec3::unit_x(),
            },
        ];
        for s in &kinds {
            let (lo, hi) =
                circle_residual_extremes(s, center, axis, radius, u_ref).expect("closed form");
            let mut seen_lo = f64::INFINITY;
            let mut seen_hi = f64::NEG_INFINITY;
            for i in 0..4096 {
                let t = core::f64::consts::TAU * f64::from(i) / 4096.0;
                let r = implicit_residual(s, eval(t));
                assert!(
                    lo - 1e-12 <= r && r <= hi + 1e-12,
                    "sample {r} escapes [{lo}, {hi}] on {s:?}"
                );
                seen_lo = seen_lo.min(r);
                seen_hi = seen_hi.max(r);
            }
            if matches!(s, Surface::Sphere { .. } | Surface::Plane { .. }) {
                // First-harmonic arms are EXACT: sampling attains the
                // bounds to discretization error.
                assert!((seen_lo - lo).abs() < 1e-5 && (seen_hi - hi).abs() < 1e-5);
            }
        }
        // **The torus answers now, and it answers by SUBDIVISION.**
        // There is still no harmonic form — the pin is that the arc
        // door stands in for one over the whole turn, and that what
        // it returns encloses.
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let (lo, hi) =
            circle_residual_extremes(&torus, center, axis, radius, u_ref).expect("the torus arm");
        let mut seen_lo = f64::INFINITY;
        let mut seen_hi = f64::NEG_INFINITY;
        for i in 0..4096 {
            let r = implicit_residual(&torus, eval(core::f64::consts::TAU * f64::from(i) / 4096.0));
            assert!(
                lo <= r && r <= hi,
                "torus sample {r} escapes [{lo}, {hi}] — the subdivision is unsound"
            );
            seen_lo = seen_lo.min(r);
            seen_hi = seen_hi.max(r);
        }
        // And it is not the whole world: the charge is small against
        // the range it widens.
        assert!(
            seen_lo - lo < 0.1 * (seen_hi - seen_lo) && hi - seen_hi < 0.1 * (seen_hi - seen_lo),
            "the charge dominates the range it widens: [{lo}, {hi}] around              [{seen_lo}, {seen_hi}]"
        );
        // The cone keeps the frontier door: no harmonic form and no
        // curvature bound either.
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::unit_x(),
        };
        assert!(circle_residual_extremes(&cone, center, axis, radius, u_ref).is_none());
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod arc_clearance_tests {
    use core::f64::consts::{PI, TAU};

    use test_utils::fuzz;

    use super::*;

    /// One (surface, circle) pair the rows below sample.
    struct Case {
        name: &'static str,
        surface: Surface<f64>,
        center: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        u_ref: Vec3<f64>,
    }

    fn case(
        name: &'static str,
        surface: Surface<f64>,
        center: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        u_ref: Vec3<f64>,
    ) -> Case {
        Case {
            name,
            surface,
            center,
            axis,
            radius,
            u_ref,
        }
    }

    /// A spread of (surface, circle) pairs whose composed residual
    /// exercises BOTH harmonics: the cylinder rows tilt the circle
    /// against the wall axis, which is the only way `A₂` is nonzero,
    /// and the plane/sphere rows pin the first-harmonic-only arms.
    fn cases() -> Vec<Case> {
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        // A circle whose axis is tilted 45° from the wall's: the
        // composed squared distance then has a genuine second harmonic.
        let tilt = Vec3::new(0.0, 1.0, 1.0).normalize();
        let tilt_u = Vec3::new(1.0, 0.0, 0.0);
        let steep = Vec3::new(0.0, 3.0, 1.0).normalize();
        vec![
            case(
                "cylinder, tilted circle (A2 large)",
                Surface::Cylinder {
                    origin: Point3::new(-1.0, 0.0, 0.0),
                    axis: z,
                    radius: 1.6,
                    u_ref: x,
                },
                Point3::new(0.0, 0.0, 0.0),
                tilt,
                1.0,
                tilt_u,
            ),
            case(
                "cylinder, steeply tilted circle",
                Surface::Cylinder {
                    origin: Point3::new(0.4, -0.3, 0.0),
                    axis: z,
                    radius: 0.9,
                    u_ref: x,
                },
                Point3::new(0.2, 0.1, 0.0),
                steep,
                1.3,
                tilt_u,
            ),
            case(
                "cylinder, coaxial circle (A2 = 0)",
                Surface::Cylinder {
                    origin: Point3::new(-1.0, 0.0, 0.0),
                    axis: z,
                    radius: 1.6,
                    u_ref: x,
                },
                Point3::new(0.0, 0.0, 0.0),
                z,
                1.0,
                x,
            ),
            case(
                "sphere",
                Surface::Sphere {
                    center: Point3::new(-1.0, 0.0, 0.0),
                    radius: 1.2,
                    axis: z,
                    u_ref: x,
                },
                Point3::new(0.0, 0.0, 0.0),
                z,
                1.0,
                x,
            ),
            case(
                "plane",
                Surface::Plane {
                    origin: Point3::new(0.0, 0.0, 0.1),
                    normal: Vec3::new(0.3, 0.0, 1.0).normalize(),
                    u_ref: x,
                },
                Point3::new(0.0, 0.0, 0.0),
                z,
                1.0,
                x,
            ),
            // The torus arm's three shapes. It has no harmonic
            // triple, so its bound comes from the certified
            // `|(d²)″|` of `docs/CURVED-TORUS-SPEC.md` §PR-2 — and
            // the three terms that bound differ in which of these
            // reaches them: the coplanar case has `a_h = 0` and
            // exercises the radial channel alone, the tilted case
            // turns on the axial channel (`a_h ≠ 0`, the
            // n-projection terms), and the near-axis case drives
            // `2ρ_c²/ρ_min` — the term the through-axis row below
            // takes to infinity.
            case(
                "torus, coplanar offset circle",
                Surface::Torus {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: z,
                    major_radius: 1.0,
                    minor_radius: 0.25,
                    u_ref: x,
                },
                Point3::new(1.7, 0.0, 0.0),
                z,
                0.8,
                x,
            ),
            case(
                "torus, tilted circle (a_h large)",
                Surface::Torus {
                    center: Point3::new(0.1, -0.2, 0.0),
                    axis: z,
                    major_radius: 1.2,
                    minor_radius: 0.3,
                    u_ref: x,
                },
                Point3::new(0.3, 0.4, 0.2),
                tilt,
                1.1,
                tilt_u,
            ),
            case(
                "torus, near-axis circle",
                Surface::Torus {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: z,
                    major_radius: 0.9,
                    minor_radius: 0.2,
                    u_ref: x,
                },
                Point3::new(0.66, 0.0, 0.35),
                steep,
                0.6,
                tilt_u,
            ),
        ]
    }

    fn residual_at(
        s: &Surface<f64>,
        c: Point3<f64>,
        axis: Vec3<f64>,
        r: f64,
        u: Vec3<f64>,
        theta: f64,
    ) -> f64 {
        let v = axis.cross(u);
        implicit_residual(s, c + u * (r * theta.cos()) + v * (r * theta.sin()))
    }

    /// **The curvature bound must ENCLOSE.** `|F″| ≤ A₁ + 4A₂` is what
    /// licenses the arc's chord-dip clearance, and it is the one
    /// constant in that computation whose only effect is to WIDEN
    /// acceptance — too small and the boolean's circle row clears an
    /// arc that meets the face.
    ///
    /// Sampled second differences are the oracle. **Planted-corruption
    /// check**: replacing the `4.0` in
    /// [`circle_residual_curvature_bound`] with `2.0` reds this row on
    /// both tilted-cylinder cases (their `A₂` is the dominant term);
    /// the coaxial, sphere and plane cases have `A₂ = 0` and cannot
    /// see that constant at all, which is why the tilted rows are here.
    /// The torus arm's terms are covered the same way and by the same
    /// argument — each of §PR-2's four terms dominates in one of the
    /// three torus cases, so dropping the `D_max·ρ″` product reds the
    /// near-axis case and flipping the sign of either n-projection
    /// term reds the tilted one.
    #[test]
    fn the_curvature_bound_encloses_the_sampled_second_derivative() {
        const N: usize = 4096;
        for Case {
            name,
            surface: s,
            center: c,
            axis,
            radius: r,
            u_ref: u,
        } in cases()
        {
            let bound = circle_residual_curvature_bound(&s, c, axis, r, u).expect("closed form");
            let h = TAU / N as f64;
            let mut worst: f64 = 0.0;
            for k in 0..N {
                let t = k as f64 * h;
                let f2 = (residual_at(&s, c, axis, r, u, t + h)
                    - 2.0 * residual_at(&s, c, axis, r, u, t)
                    + residual_at(&s, c, axis, r, u, t - h))
                    / (h * h);
                worst = worst.max(f2.abs());
            }
            assert!(
                bound >= worst - 1e-6,
                "{name}: |F''| bound {bound} under-encloses the sampled {worst}"
            );
        }
    }

    /// **The enclosure's lower end must LOWER-bound the arc.** This is
    /// the boolean circle row's accepting computation, read through
    /// the door the row actually calls: an arc clears when
    /// [`circle_arc_residual_range`]'s `lo` is positive, so `lo` must
    /// never exceed the arc's true minimum. Every span from a sliver
    /// to a full turn, on every case.
    ///
    /// It used to spell the deleted two-endpoint chord dip by hand —
    /// arithmetic no shipped code computes any more, so a row over it
    /// was testing a formula rather than a door.
    ///
    /// The same `4.0 → 2.0` corruption reds this row, and it is the
    /// row that names the CONSEQUENCE: the bound would claim clearance
    /// the arc does not have.
    #[test]
    fn the_enclosure_never_exceeds_the_arcs_true_minimum() {
        const N: usize = 512;
        for Case {
            name,
            surface: s,
            center: c,
            axis,
            radius: r,
            u_ref: u,
        } in cases()
        {
            for span_steps in 1..=16 {
                let span = TAU * f64::from(span_steps) / 16.0;
                for start_steps in 0..8 {
                    let t0 = TAU * f64::from(start_steps) / 8.0;
                    let t1 = t0 + span;
                    let (claimed, top) = circle_arc_residual_range(&s, c, axis, r, u, t0, t1)
                        .expect("every case here has a curvature bound");
                    let mut truth = f64::INFINITY;
                    let mut peak = f64::NEG_INFINITY;
                    for k in 0..=N {
                        let t = t0 + span * k as f64 / N as f64;
                        truth = truth.min(residual_at(&s, c, axis, r, u, t));
                        peak = peak.max(residual_at(&s, c, axis, r, u, t));
                    }
                    assert!(
                        claimed <= truth + 1e-9 && top >= peak - 1e-9,
                        "{name}: span {span} from {t0}: the enclosure [{claimed}, {top}] \
                         does not hold the arc's [{truth}, {peak}]"
                    );
                }
            }
        }
    }

    /// **The accepting direction EXISTS, and these are its numbers.**
    /// A row that only checked soundness would stay green if the arc
    /// bound were deleted, so this pins a configuration where the arc
    /// decides and the carrier cannot: a unit circle about the origin
    /// against a wall of radius 1.6 about `(−1, 0)`. The carrier dives
    /// 0.8 m inside that wall near `θ = π`, so the whole-circle
    /// enclosure straddles and refuses; the 60° arc from `θ = 0` stands
    /// clear, and the chord-dip bound proves it.
    #[test]
    fn a_short_arc_clears_a_carrier_that_straddles() {
        let s = Surface::Cylinder {
            origin: Point3::new(-1.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.6,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (c, axis, r, u) = (
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (lo, hi) = circle_residual_extremes(&s, c, axis, r, u).expect("closed form");
        let carrier_margin = lo.max(-hi);
        assert!(
            carrier_margin < 0.0,
            "the carrier must straddle for this row to mean anything: {carrier_margin}"
        );
        let (t0, t1) = (0.0, PI / 3.0);
        // Through the door, not through the deleted two-endpoint fold:
        // this is the arithmetic `reduce.rs` runs.
        let (arc_lo, arc_hi) =
            circle_arc_residual_range(&s, c, axis, r, u, t0, t1).expect("closed form");
        let arc_margin = arc_lo.max(-arc_hi);
        assert!(
            arc_margin > 1e-3,
            "the arc must clear DEFINITELY, well outside any band: {arc_margin}"
        );
        // And the arc really is clear — the bound is not clearing a
        // meeting.
        for k in 0..=512 {
            let t = t0 + (t1 - t0) * f64::from(k) / 512.0;
            assert!(residual_at(&s, c, axis, r, u, t) > 0.0);
        }
    }

    /// **A circle that may reach the torus's axis has NO finite
    /// curvature bound, and the arithmetic says so.** `ρ_min = 0`
    /// puts a zero in the denominator of `2ρ_c²/ρ_min`; the residual
    /// really does have a kink on the axis (the chart's `|w|` is not
    /// differentiable there), so an infinite enclosure is the honest
    /// answer and not a limitation. No branch tests for it — this row
    /// exists because a branch is exactly what a later reader would
    /// be tempted to add.
    ///
    /// The consumer's reading of the infinity is the point: the
    /// margin `lo.max(-hi)` is `−∞`, which is a definite NEGATIVE and
    /// takes the typed frontier door, never a clearance.
    #[test]
    fn a_through_axis_circle_encloses_infinitely_rather_than_branching() {
        let s = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 1.0,
            minor_radius: 0.25,
            u_ref: Vec3::unit_x(),
        };
        // A circle in the ring plane whose centre stands exactly its
        // own radius from the axis: it MEETS the axis, at `t = π`.
        let center = Point3::new(0.5, 0.0, 0.0);
        let (lo, hi) =
            circle_arc_residual_range(&s, center, Vec3::unit_z(), 0.5, Vec3::unit_x(), 0.0, TAU)
                .expect("the torus arm answers, infinitely");
        assert!(
            lo == f64::NEG_INFINITY && hi == f64::INFINITY,
            "the through-axis enclosure must be the whole line: [{lo}, {hi}]"
        );
        assert!(
            lo.max(-hi) == f64::NEG_INFINITY,
            "and the consumer's margin must be a definite negative"
        );
        let bound =
            circle_residual_curvature_bound(&s, center, Vec3::unit_z(), 0.5, Vec3::unit_x())
                .expect("the torus arm answers");
        assert!(bound.is_infinite(), "the bound itself is infinite: {bound}");
    }

    /// The same circle at the **interval** scalar: the division by an
    /// enclosure that contains zero poisons rather than overflowing,
    /// and a poisoned margin is what the boolean lane escalates on.
    #[test]
    fn a_through_axis_circle_poisons_the_interval_lane() {
        use geom_core::Interval;
        let i = Interval::from_f64;
        let s = Surface::Torus {
            center: Point3::new(i(0.0), i(0.0), i(0.0)),
            axis: Vec3::new(i(0.0), i(0.0), i(1.0)),
            major_radius: i(1.0),
            minor_radius: i(0.25),
            u_ref: Vec3::new(i(1.0), i(0.0), i(0.0)),
        };
        let center = Point3::new(i(0.5), i(0.0), i(0.0));
        let (lo, hi) = circle_arc_residual_range(
            &s,
            center,
            Vec3::new(i(0.0), i(0.0), i(1.0)),
            i(0.5),
            Vec3::new(i(1.0), i(0.0), i(0.0)),
            i(0.0),
            Interval::tau(),
        )
        .expect("the torus arm answers");
        // The claim is POISON, specifically. A finite
        // `[−charge, +charge]` also satisfies "never certifies a
        // clearance", so a weaker assertion here could not tell the
        // arithmetic's honest refusal from a branch that clamped
        // `ρ_min` off zero and returned a finite bound.
        let margin = lo.max(-hi);
        assert!(
            lo.is_poison() && hi.is_poison() && margin.is_poison(),
            "the interval lane must POISON on the axis, not enclose: [{lo:?}, {hi:?}]"
        );
        let bound = circle_residual_curvature_bound(
            &s,
            center,
            Vec3::new(i(0.0), i(0.0), i(1.0)),
            i(0.5),
            Vec3::new(i(1.0), i(0.0), i(0.0)),
        )
        .expect("the torus arm answers");
        assert!(
            bound.is_poison(),
            "and so must the bound behind it: {bound:?}"
        );
    }

    /// **The schedule reaches BOTH ends of the arc, exactly.** The
    /// chord-dip argument charges each of the `K` sub-intervals
    /// against both of its ends; a schedule that stops one parameter
    /// short leaves the last sub-interval uncharged, and the
    /// enclosure is then unsound wherever the residual's extreme sits
    /// in that cell — which no fixture row can be relied on to catch.
    ///
    /// Both ends are pinned to EXACT equality, which the lerp form
    /// gives and `t₀ + step·k` does not.
    #[test]
    fn the_sample_schedule_reaches_both_ends_of_the_arc() {
        for (t0, t1) in [
            (0.0, TAU),
            (0.0, 0.383_972_435_438_75),
            (-0.5, 0.5),
            (2.9, 3.4),
            (1.0, 1.000_001),
            (TAU, 0.0),
        ] {
            assert!(
                arc_sample(t0, t1, 0) == t0,
                "sample 0 must be exactly t0: {} vs {t0}",
                arc_sample(t0, t1, 0)
            );
            assert!(
                arc_sample(t0, t1, ARC_RESIDUAL_SAMPLES) == t1,
                "sample K must be exactly t1: {} vs {t1}",
                arc_sample(t0, t1, ARC_RESIDUAL_SAMPLES)
            );
            // And the interior is monotone and inside, so the `K`
            // sub-intervals really tile the arc.
            let mut prev = t0;
            for k in 1..=ARC_RESIDUAL_SAMPLES {
                let t = arc_sample(t0, t1, k);
                assert!(
                    (t - prev) * (t1 - t0) >= 0.0,
                    "the schedule must advance toward t1: {prev} -> {t}"
                );
                prev = t;
            }
        }
    }

    /// **The ARC-SCOPED bound, against a dense analytic oracle, on
    /// random configurations.** Adopted from the v6 dual on
    /// `f0f46ebb5` — TARM-R1's probe P1 and TARM-R2's probe C1, which
    /// converged on the same gap: every shipped bound row called the
    /// `[0, τ]` form, so the arc-scoped `f2` — this unit's novelty —
    /// had no direct soundness row at all, and three planted defects
    /// (a signed `h` range, a one-sided `D_max`, a schedule one
    /// parameter short) passed the whole suite.
    ///
    /// The oracle is the closed second derivative of the composed
    /// residual, `(d²)″/2r` with
    /// `(d²)″ = 2[(ρ′)² + (ρ−R)ρ″] + 2[(h′)² + h·h″]` evaluated from
    /// `C′` and `C″` directly — not the bound's own algebra, so a term
    /// dropped or sign-flipped in the bound cannot hide in it.
    #[test]
    fn the_arc_scoped_bound_never_falls_below_a_dense_second_derivative() {
        /// The composed residual's exact `F″` at one parameter.
        fn g2(c: &TorusArcCase, t: f64) -> f64 {
            let Surface::Torus {
                center: tc,
                axis: tn,
                major_radius,
                minor_radius,
                ..
            } = c.surface
            else {
                unreachable!("torus fixture")
            };
            let v = c.axis.cross(c.u_ref);
            let (st, ct) = t.sin_cos();
            let p = c.center + c.u_ref * (c.radius * ct) + v * (c.radius * st);
            let c1 = c.u_ref * (-c.radius * st) + v * (c.radius * ct);
            let c2 = c.u_ref * (-c.radius * ct) + v * (-c.radius * st);
            let q = p - tc;
            let (h, h1, h2) = (q.dot(tn), c1.dot(tn), c2.dot(tn));
            let (w, w1, w2) = (q - tn * h, c1 - tn * h1, c2 - tn * h2);
            let rho = w.norm();
            let rho1 = w.dot(w1) / rho;
            let rho2 = (w1.dot(w1) + w.dot(w2)) / rho - w.dot(w1).powi(2) / rho.powi(3);
            let d2pp = 2.0 * (rho1 * rho1 + (rho - major_radius) * rho2) + 2.0 * (h1 * h1 + h * h2);
            d2pp / (2.0 * minor_radius)
        }

        let mut rng = fuzz::start("implicit::arc_scoped_bound_vs_dense_second_derivative");
        let cases = fuzz::scaled(200);
        let dense = fuzz::scaled(600);
        let (mut min_ratio, mut infinite, mut checked) = (f64::INFINITY, 0u32, 0u32);
        for i in 0..cases {
            let c = random_torus_arc(&mut rng, i);
            let f2 =
                arc_curvature_bound(&c.surface, c.center, c.axis, c.radius, c.u_ref, c.t0, c.t1)
                    .expect("the torus arm answers");
            if !f2.is_finite() {
                infinite += 1;
                continue;
            }
            let mut worst = 0.0f64;
            for k in 0..=dense {
                let t = c.t0 + (c.t1 - c.t0) * k as f64 / dense as f64;
                worst = worst.max(g2(&c, t).abs());
            }
            assert!(
                f2 >= worst * (1.0 - 1e-9),
                "case {i}: the arc-scoped bound {f2} falls below the dense \
                 |F''| {worst} — {}",
                fuzz::replay()
            );
            checked += 1;
            if worst > 0.0 {
                min_ratio = min_ratio.min(f2 / worst);
            }
        }
        println!(
            "arc-scoped bound: {checked} finite configurations, {infinite} \
             through-axis (infinite by arithmetic); tightest ratio {min_ratio:.4}"
        );
        // The bound must also be USABLE, not merely sound: a bound
        // orders of magnitude over the truth everywhere would pass the
        // assertion above and resolve nothing.
        assert!(
            min_ratio < 1e4,
            "the bound is sound but useless at its tightest: {min_ratio}"
        );
    }

    /// One random torus, circle and arc — the configuration family the
    /// two dense-oracle rows share.
    struct TorusArcCase {
        surface: Surface<f64>,
        center: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        u_ref: Vec3<f64>,
        t0: f64,
        t1: f64,
    }

    fn random_dir(rng: &mut fuzz::Rng) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            if v.norm() > 0.2 && v.norm() < 1.0 {
                return v.normalize();
            }
        }
    }

    fn random_perp(rng: &mut fuzz::Rng, a: Vec3<f64>) -> Vec3<f64> {
        loop {
            let d = random_dir(rng);
            let p = d - a * a.dot(d);
            if p.norm() > 0.2 {
                return p.normalize();
            }
        }
    }

    /// Spans are log-uniform over three decades and centres are drawn
    /// near the torus a quarter of the time, so the family reaches
    /// slivers, whole turns and near-axis circles rather than only
    /// comfortable ones (TARM-R1's `rand_cfg`).
    fn random_torus_arc(rng: &mut fuzz::Rng, i: usize) -> TorusArcCase {
        let big_r = rng.range(0.5, 3.0);
        let tn = random_dir(rng);
        let tc = Point3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let minor = big_r * rng.range(0.05, 0.6);
        let t_u = random_perp(rng, tn);
        let surface = Surface::Torus {
            center: tc,
            axis: tn,
            major_radius: big_r,
            minor_radius: minor,
            u_ref: t_u,
        };
        let t0 = rng.range(0.0, TAU);
        let span = 10f64.powf(rng.range(-3.0, TAU.log10()));
        // One case in eight is a RING-PLANE circle that reaches from
        // near the axis out past the spine, so the radial range
        // straddles `R` with its far-from-spine end at the INNER side.
        // That is the only family where a one-sided `D_max` (the far
        // end alone) under-bounds, and it is a defect no comfortable
        // configuration can see.
        if i % 8 == 1 {
            let e = big_r * rng.range(0.5, 0.7);
            return TorusArcCase {
                surface,
                center: tc + t_u * e,
                axis: tn,
                radius: big_r * rng.range(0.4, 0.5),
                u_ref: t_u,
                t0,
                t1: t0 + span,
            };
        }
        let spread = if i.is_multiple_of(4) { 0.5 } else { 4.0 };
        let axis = random_dir(rng);
        TorusArcCase {
            surface,
            center: tc
                + Vec3::new(
                    rng.range(-spread, spread),
                    rng.range(-spread, spread),
                    rng.range(-spread, spread),
                ),
            axis,
            radius: rng.range(0.05, 4.0),
            u_ref: random_perp(rng, axis),
            t0,
            t1: t0 + span,
        }
    }
}
