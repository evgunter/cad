//! D7 stage-1 NURBS surface recognition (ruling #256): every imported
//! NURBS surface is tested for promotion to an analytic kind, and
//! promotion fires iff the per-kind residual CERTIFIES at ε_in —
//! verified, never trusted (a file's `.PLANE_SURF.` annotation is not
//! consulted; the certificate is the only authority). A patch that
//! certifies nowhere stays NURBS silently — the M7-3-legal state:
//! recognition failing must never refuse a face that imports today.
//!
//! # Kind scope
//!
//! Plane and cylinder — the measured need of the wild corpus (dm1's 17
//! exact planes + 7 exact rational cylinders). Cone, sphere, and torus
//! recognition are unimplemented and stay-NURBS (banked; no dead
//! per-kind code here).
//!
//! # Certification, dual-track (D4 ¶2 shapes, ε_in budget)
//!
//! - **Plane, non-rational**: the closed-form control-net hull
//!   sup-bound — a B-spline point is a convex combination of its
//!   control points, so the surface's deviation from a plane is
//!   bounded by the worst CONTROL-POINT deviation. Whole-patch and
//!   total, no sampling.
//! - **Everything else** (rational planes, cylinders): the fixed
//!   [`geom_brep::CERT_SAMPLES`]² grid, sup of the closed-form
//!   linearized implicit residual ([`geom_brep::implicit_residual`]).
//!   The schedule is fixed — never data-dependent iteration (D9; the
//!   arc-rim gate's precedent).
//!
//! # Estimators (D9-clean: closed form, fixed evaluation order)
//!
//! - **Plane**: translate-to-origin Newell over the control net's
//!   boundary ring in index order (the `newell_plane` method, run on
//!   the net rather than a loop so the fit is certification-free
//!   here — certification is the residual bound above), oriented to
//!   the NURBS chart normal at the domain midpoint, `u_ref` from the
//!   normal's canonical orthonormal basis. Every step is a fixed-order
//!   sum or a closed-form map of one — no iteration, no
//!   data-dependent branching beyond the orientation sign.
//! - **Cylinder**: axis as the fixed-order sum of the net's v-column
//!   translations (an extruded patch's columns are translates along
//!   its axis); three fixed-parameter samples of the `v = v0` boundary
//!   projected ⊥ axis; the exact circumcenter solve for center and
//!   radius; `u_ref` toward the first sample (the patch's u-start
//!   azimuth — the seam, so a full-period patch's seam generator
//!   certifies as [`geom_brep::EdgeGeometry::Seam`]). Closed form
//!   throughout.
//!
//! Each estimator only PROPOSES; the certificate above decides. A
//! wrong estimate fails certification and the patch stays NURBS — an
//! estimator can therefore never make promotion incorrect, only
//! incomplete.
//!
//! # Conditioning (D7's typed ambiguity, its only stage-1 site)
//!
//! The cylinder estimator carries its own margin trilean: a patch
//! whose azimuth samples are collinear within ε_in determines no
//! finite radius (the near-flat patch), and one whose v-columns sum
//! below ε_in determines no axis. Either is
//! [`Recognition::IllConditioned`] — the estimator cannot answer at
//! the interpretation budget. The caller escalates that to the typed
//! `RecognitionAmbiguous` refusal ONLY where promotion was needed to
//! import at all (the multi-bound curved-face gate); an importable
//! face with an ill-conditioned estimator stays NURBS, exactly like a
//! refuted one.

use geom_core::{Point3, Vec3};
use geom_surfaces::{NurbsSurface, Surface};

pub(crate) use crate::PromotedKind;

/// The outcome of testing one NURBS patch for promotion.
#[derive(Debug)]
pub(crate) enum Recognition {
    /// A kind certified at ε_in.
    Promoted {
        /// The analytic surface. A plane's normal is aligned to the
        /// NURBS chart normal; a cylinder's chart normal is radially
        /// outward by construction, whatever the patch's orientation
        /// — the caller composes [`chart_flipped`] into the face's
        /// `same_sense` so orientation survives the promotion.
        surface: Surface<f64>,
        /// The certified residual sup (meters).
        residual: f64,
        /// Which kind certified.
        kind: PromotedKind,
    },
    /// No implemented kind certifies — the patch stays NURBS (the
    /// M7-3-legal state, silent).
    StaysNurbs,
    /// An estimator could not answer at ε_in (its own margin trilean).
    IllConditioned {
        /// The kind whose estimator declined.
        kind: PromotedKind,
        /// The margin that fell inside ε_in (meters).
        margin: f64,
    },
}

/// Tests `patch` for promotion at the interpretation budget `eps_in`
/// (module docs: kinds, estimators, certificates).
///
/// Selection is the fixed kind-preference order **Plane > Cylinder**.
/// A patch that certifies as BOTH is canonicalization, not ambiguity:
/// both analytic surfaces agree with the patch — hence with each
/// other — within 2·ε_in everywhere on it, so either answer is a
/// correct reading of the file and the fixed order merely picks the
/// canonical one (D7's typed ambiguity is reserved for the estimator's
/// own conditioning, where no answer exists at the budget).
pub(crate) fn recognize(patch: &NurbsSurface<f64>, eps_in: f64) -> Recognition {
    if let Some((surface, residual)) = try_plane(patch, eps_in) {
        return Recognition::Promoted {
            surface,
            residual,
            kind: PromotedKind::Plane,
        };
    }
    match try_cylinder(patch, eps_in) {
        Ok(Some((surface, residual))) => Recognition::Promoted {
            surface,
            residual,
            kind: PromotedKind::Cylinder,
        },
        Ok(None) => Recognition::StaysNurbs,
        Err(margin) => Recognition::IllConditioned {
            kind: PromotedKind::Cylinder,
            margin,
        },
    }
}

/// The NURBS chart normal (unnormalized `∂u × ∂v`) at the domain
/// midpoint — the orientation reference every promoted chart is
/// aligned to, so the face's `same_sense` keeps its meaning across
/// the promotion.
fn chart_normal_mid(patch: &NurbsSurface<f64>) -> Vec3<f64> {
    let (u0, u1) = patch.knots_u().domain();
    let (v0, v1) = patch.knots_v().domain();
    let jet = patch.ders((u0 + u1) / 2.0, (v0 + v1) / 2.0);
    jet.du.cross(jet.dv)
}

/// The plane candidate: Newell over the control net's boundary ring,
/// certified by the control-net hull sup-bound (non-rational) or the
/// fixed sampled-grid sup (rational) — module docs.
fn try_plane(patch: &NurbsSurface<f64>, eps_in: f64) -> Option<(Surface<f64>, f64)> {
    let (nu, nv) = patch.control_counts();
    let control = patch.control();
    // Centroid over the whole net (fixed left-to-right order).
    let n = control.len() as f64;
    let mut sum = Vec3::zero();
    for p in control {
        sum = sum + (*p - Point3::origin());
    }
    let origin = Point3::origin() + sum / n;
    // Newell cross-sum over the net's boundary ring: row v=0 forward,
    // column u=nu−1 up, row v=nv−1 backward, column u=0 down — one
    // fixed cycle in index order. (For a 2×2 net this is the quad's
    // own cycle.)
    let at = |iu: usize, iv: usize| control[iu * nv + iv];
    let mut ring: Vec<Point3<f64>> = Vec::with_capacity(2 * (nu + nv));
    for iu in 0..nu {
        ring.push(at(iu, 0));
    }
    for iv in 1..nv {
        ring.push(at(nu - 1, iv));
    }
    for iu in (0..nu - 1).rev() {
        ring.push(at(iu, nv - 1));
    }
    for iv in (1..nv - 1).rev() {
        ring.push(at(0, iv));
    }
    let mut normal_sum = Vec3::zero();
    for (i, p) in ring.iter().enumerate() {
        let next = ring[(i + 1) % ring.len()];
        normal_sum = normal_sum + (*p - origin).cross(next - origin);
    }
    let mut normal = normal_sum.normalize();
    // Orient to the chart normal (module docs). A degenerate net — a
    // poison normal (`normalize` of ~0 is all-NaN), or one orthogonal
    // to the chart's — determines no oriented plane: refuted here,
    // stays NURBS.
    let align = normal.dot(chart_normal_mid(patch));
    if !(align.is_finite() && align != 0.0) {
        return None;
    }
    if align < 0.0 {
        normal = -normal;
    }
    let (u_ref, _) = normal.orthonormal_basis();
    let plane = Surface::Plane {
        origin,
        normal,
        u_ref,
    };
    // Non-rational: the hull sup-bound over the WHOLE net. `S(u,v)`
    // is a convex combination of control points (basis functions
    // nonnegative, partition of unity), so
    // `sup |dist(S, plane)| ≤ max_i |dist(P_i, plane)|` — a total,
    // whole-patch certificate. Rational: the fixed sampled grid.
    let residual = if patch.weights().iter().all(|&w| w == 1.0) {
        let mut worst = 0.0f64;
        for p in control {
            worst = worst.max((*p - origin).dot(normal).abs());
        }
        worst
    } else {
        sampled_residual_sup(patch, &plane)
    };
    (residual <= eps_in).then_some((plane, residual))
}

/// The cylinder candidate (module docs). `Err(margin)` is the
/// estimator's own conditioning refusal; `Ok(None)` a refuted
/// certificate.
#[allow(clippy::type_complexity)]
fn try_cylinder(
    patch: &NurbsSurface<f64>,
    eps_in: f64,
) -> Result<Option<(Surface<f64>, f64)>, f64> {
    let (nu, nv) = patch.control_counts();
    let control = patch.control();
    let at = |iu: usize, iv: usize| control[iu * nv + iv];
    // Axis: the fixed-order sum of each u-row's v-span translation.
    // For an extruded patch every row's `last − first` is the same
    // vector (length × axis); summing is the D9-clean mean.
    let mut axis_sum = Vec3::zero();
    for iu in 0..nu {
        axis_sum = axis_sum + (at(iu, nv - 1) - at(iu, 0));
    }
    let axis_norm = axis_sum.norm();
    if !axis_norm.is_finite() {
        return Ok(None);
    }
    // Divided by nu it is the mean v-span — a patch length. Below
    // ε_in no axis is determined at the budget.
    if axis_norm / (nu as f64) <= eps_in {
        return Err(axis_norm / (nu as f64));
    }
    let axis = axis_sum / axis_norm;
    // Three fixed samples of the v-start boundary at u-fractions
    // 0, 3/8, 6/8 of the domain — distinct azimuths even on a
    // full-period patch (where u-start and u-end coincide).
    let (u0, u1) = patch.knots_u().domain();
    let (v0, _) = patch.knots_v().domain();
    let sample = |f: f64| patch.eval(u0 + (u1 - u0) * f, v0);
    let p0 = sample(0.0);
    let pa = sample(3.0 / 8.0);
    let pb = sample(6.0 / 8.0);
    // Project ⊥ axis into the plane through p0.
    let proj = |p: Point3<f64>| p - axis * (p - p0).dot(axis);
    let (q0, qa, qb) = (proj(p0), proj(pa), proj(pb));
    // Conditioning margin: the sagitta of the middle sample over the
    // chord — collinear-at-ε_in samples determine no finite radius
    // (the near-flat patch; D7's ambiguity class).
    let chord = qb - q0;
    let chord_norm = chord.norm();
    if !(chord_norm.is_finite() && chord_norm > 0.0) {
        return Ok(None);
    }
    let chord_dir = chord / chord_norm;
    let off = (qa - q0) - chord_dir * (qa - q0).dot(chord_dir);
    let sagitta = off.norm();
    if !sagitta.is_finite() {
        return Ok(None);
    }
    if sagitta <= eps_in {
        return Err(sagitta);
    }
    // Exact circumcenter of (q0, qa, qb) in their common plane ⊥
    // axis: the standard closed form
    // `c = q0 + (|b|²·(a×b)×a + |a|²·b×(a×b)) / (2·|a×b|²)` with
    // `a = qa − q0`, `b = qb − q0`.
    let a = qa - q0;
    let b = qb - q0;
    let axb = a.cross(b);
    let denom = 2.0 * axb.norm_squared();
    let center = q0 + (axb.cross(a) * b.norm_squared() + b.cross(axb) * a.norm_squared()) / denom;
    let radial = q0 - center;
    let radius = radial.norm();
    if !(radius.is_finite() && radius > 0.0) {
        return Ok(None);
    }
    let cylinder = Surface::Cylinder {
        origin: center,
        axis,
        radius,
        u_ref: radial / radius,
    };
    let residual = sampled_residual_sup(patch, &cylinder);
    // Orientation: the cylinder chart's normal is radially outward by
    // construction, whatever the patch's chart orientation — the
    // caller compares chart normals and composes `same_sense`.
    Ok((residual <= eps_in).then_some((cylinder, residual)))
}

/// The sup of the closed-form implicit residual over the fixed
/// [`geom_brep::CERT_SAMPLES`]² grid (module docs: the rational /
/// no-closed-form certification track). Poison anywhere makes the
/// sup NaN, which certifies nothing (D4 ¶2 totality).
fn sampled_residual_sup(patch: &NurbsSurface<f64>, candidate: &Surface<f64>) -> f64 {
    let (u0, u1) = patch.knots_u().domain();
    let (v0, v1) = patch.knots_v().domain();
    let n = geom_brep::CERT_SAMPLES;
    let mut worst = 0.0f64;
    for i in 0..n {
        let fu = f64::from(i) / f64::from(n - 1);
        for j in 0..n {
            let fv = f64::from(j) / f64::from(n - 1);
            let p = patch.eval(u0 + (u1 - u0) * fu, v0 + (v1 - v0) * fv);
            let r = geom_brep::implicit_residual(candidate, p).abs();
            // `max` with NaN-propagation: a poison residual must not
            // be masked by an earlier finite one.
            worst = if r.is_nan() { f64::NAN } else { worst.max(r) };
            if worst.is_nan() {
                return worst;
            }
        }
    }
    worst
}

/// Whether the promoted chart's normal OPPOSES the NURBS chart's at
/// the domain midpoint — the `same_sense` composition bit. Computed
/// against the promoted surface's own chart normal via the implicit
/// gradient (plane: the stored normal; cylinder: radially outward),
/// which equals the chart normal for both promoted kinds.
pub(crate) fn chart_flipped(patch: &NurbsSurface<f64>, promoted: &Surface<f64>) -> bool {
    let (u0, u1) = patch.knots_u().domain();
    let (v0, v1) = patch.knots_v().domain();
    let jet = patch.ders((u0 + u1) / 2.0, (v0 + v1) / 2.0);
    let nurbs_normal = jet.du.cross(jet.dv);
    let grad = geom_brep::implicit_gradient(promoted, jet.point);
    nurbs_normal.dot(grad) < 0.0
}
