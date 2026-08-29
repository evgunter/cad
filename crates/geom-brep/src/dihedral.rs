//! The dihedral classification predicate — the material wedge-angle
//! predicate's first arrival (tier 3's second half, D1's ratified tier
//! list; the classifier for construction, M2-PLAN PR 3).
//!
//! Given the two surfaces meeting at an edge and a point on the locus,
//! [`classify_dihedral`] classifies the wedge:
//!
//! - **Transverse** — tangent planes definitely distinct: the edge is a
//!   genuine corner, and an [`crate::EdgeDescription::Intersection`]
//!   description is well-conditioned there (the margin below *is* D2's
//!   transversality margin).
//! - **Smooth** — tangent planes coincide at tolerance: the legal
//!   π-wedge seam case (coplanar splits, smooth profile joins, seams).
//!   The construction stores a conventional description
//!   (`MappedCurve`/`Seam`).
//! - **Sliver** — the in-band remainder, surfaced as the typed
//!   [`Indeterminate`] escalation (D4 ¶3): near-tangent geometry is
//!   certifiable as neither intrinsic nor conventional, and a
//!   conventional description is not an escape hatch from
//!   ill-conditioned geometry (D2's ratified text).
//!
//! # The margin and its lever arm (D4 ¶1)
//!
//! Angles mean nothing without a lever arm, so the classified margin is
//! a **displacement in meters**: `sin θ · r`, where θ is the angle
//! between the tangent planes (via implicit-form gradients — never chart
//! normals, so the cone-apex chart poison is unreachable; exactly *at* a
//! surface singularity the gradient is honestly poison and the
//! classification escalates) and `r` is the honest lever arm — the
//! smallest scale over which the angular defect accumulates into a gap:
//!
//! `r = min(curvature arm of s₁, curvature arm of s₂, extent)`
//!
//! The curvature arms ([`crate::curvature_lever_arm`]) are the D4/D2
//! `1/κ_rel` reasoning in conservative per-surface form (min over the
//! two surfaces bounds 1/(κ₁ + κ₂) within a factor 2 — inside the
//! K-band's policy noise); planes contribute `+∞` (no curvature scale).
//! `extent` is the caller-named feature extent for the curvature-free
//! case (D4 ¶1's "face extent for parallelism decisions"): callers pass
//! the local feature scale the decision turns on — this crate's
//! certification and `topo`'s tier-3 validator pass the **edge's honest
//! spatial extent** ([`crate::edge_extent`]: the chord for open edges,
//! the carrier-derived point-set diameter for closed/near-closed circle
//! carriers, whose chord dishonestly collapses — see that function's
//! derivation). `sin θ` is used rather
//! than θ itself: identical to first order where the band lives, exact
//! at the π-coincidence case (antiparallel normals also classify
//! Smooth — tangent *planes*, not oriented normals, are compared), and
//! computable branch-free as `|n₁ × n₂|/(|n₁||n₂|)`.
//!
//! The margin is classified against the run's **linear** band (ε, K·ε):
//! multiplying the angle through its arm in `T` is the profile crate's
//! ratified pattern — no second band, no f64 extraction from `T`.
//!
//! # The collapsed-arm gate (M2 PR 3 fix pass)
//!
//! The margin `sin θ · r` decides the *wedge* only when the arm `r` is
//! itself definitely positive. A collapsed arm (`r` coincident with
//! zero — a zero/sub-ε extent, the cone-apex limit ρ → 0) maps **every**
//! angle into the coincidence band, so classifying there would return a
//! *definite* `Smooth` at what may be a true 90° corner — a definite
//! wrong answer, worse than no answer. The classifier therefore decides
//! the arm first (predicate `"dihedral_arm"`): definitely positive
//! proceeds; coincident-with-zero **escalates** with
//! [`geom_core::MarginDiag::Invalid`] (with no displacement scale the
//! wedge question is not validly posed at this site — the same honest
//! refusal as the poison gradient exactly *at* the apex); in-band or
//! poisoned arms escalate through the ordinary decide door. "Arm too
//! small to say" is always an escalation, never a classification.

use geom::Surface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign};

use crate::implicit::{curvature_lever_arm, implicit_gradient};

/// A definite dihedral classification (the indeterminate outcome is the
/// typed [`Indeterminate`] error — the sliver escalation, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralClass {
    /// Tangent planes definitely distinct: a genuine corner;
    /// `Intersection`'s transversality precondition holds here.
    Transverse,
    /// Tangent planes coincident at tolerance: the legal smooth/seam
    /// case (wedge = π).
    Smooth,
}

/// The one classification funnel of this crate: delegates to the
/// unified recorder funnel [`geom_core::k_stats::decide`] (M2 PR 7),
/// which names the predicate for the margin-telemetry recorder,
/// classifies through the sanctioned [`Decide`] door, and tags any
/// escalation. Kept as the crate-local wrapper so this remains the
/// crate's single greppable decision site. The margin is a
/// [`Margin<T>`] by signature (D4's margin dimensional convention,
/// clause (i)): each call site states its dimensional argument by the
/// construction door it chooses; the per-site arguments are the rows of
/// `docs/predicate-dimension-audit.md`.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

/// **`dihedral_wedge`** — classifies the wedge between `s1` and `s2` at
/// the on-locus point `p` (module docs). Margin: `sin θ · r` in meters,
/// θ the tangent-plane angle from implicit gradients, `r` the folded
/// lever arm `min(curvature arms, extent)`; classified against the
/// run's linear band.
///
/// `extent` is the caller-named arm for the curvature-free case —
/// certification and the tier-3 validator pass the edge's honest
/// extent from [`crate::edge_extent`] (chord for open edges, carrier
/// diameter at closure for circle carriers).
///
/// # Errors
///
/// [`Indeterminate`]: predicate `"dihedral_wedge"` — the margin landed
/// in the sliver band, or was poisoned (off-locus garbage, a surface
/// singularity such as the cone apex, an unimplemented `Nurbs` kind);
/// or predicate `"dihedral_arm"` — the folded lever arm is collapsed
/// (coincident with zero, in-band, or poisoned), so no angle can
/// classify at this site (the collapsed-arm gate, module docs) —
/// escalate-never-guess, D4 ¶3.
pub fn classify_dihedral<T: Decide>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    p: Point3<T>,
    extent: T,
    band: Band,
) -> Result<DihedralClass, Indeterminate> {
    let n1 = implicit_gradient(s1, p);
    let n2 = implicit_gradient(s2, p);
    let sin_theta = n1.cross(n2).norm() / (n1.norm() * n2.norm());
    let arm = curvature_lever_arm(s1, p)
        .min(curvature_lever_arm(s2, p))
        .min(extent);
    // The collapsed-arm gate (module docs): the wedge margin is only
    // meaningful through a definitely-positive arm. A Zero (or, for a
    // true magnitude, unreachable Negative) arm escalates as Invalid —
    // "the question was never validly posed here" — and an in-band or
    // poisoned arm escalates through `decide` itself via `?`.
    match decide("dihedral_arm", Margin::of(arm), band)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some("dihedral_arm"),
            });
        }
    }
    let margin = Margin::levered(sin_theta, arm);
    Ok(match decide("dihedral_wedge", margin, band)? {
        Sign::Positive => DihedralClass::Transverse,
        Sign::Zero => DihedralClass::Smooth,
        // Unreachable for a true magnitude (sin θ ≥ 0, arm ≥ 0): a
        // negative product would require poisoned inputs, which the
        // classifier already surfaced as Indeterminate. Kept total and
        // conservative: a definitely-negative "magnitude" still means
        // "definitely not coincident".
        Sign::Negative => DihedralClass::Transverse,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn eps() -> f64 {
        Tol::witness().get().eps
    }

    fn plane(normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal,
            u_ref,
        }
    }

    #[test]
    fn perpendicular_planes_are_transverse() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(Vec3::unit_x(), Vec3::unit_y());
        let c = classify_dihedral(&s1, &s2, Point3::origin(), 1.0, band()).unwrap();
        assert_eq!(c, DihedralClass::Transverse);
    }

    #[test]
    fn identical_and_antiparallel_planes_are_smooth() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(Vec3::unit_z(), Vec3::unit_y());
        let c = classify_dihedral(&s1, &s2, Point3::origin(), 1.0, band()).unwrap();
        assert_eq!(c, DihedralClass::Smooth);
        // Tangent PLANES are compared: the sign of the normal is a
        // chart artifact, and the antiparallel pair is smooth too.
        let s3 = plane(-Vec3::unit_z(), Vec3::unit_x());
        let c = classify_dihedral(&s1, &s3, Point3::origin(), 1.0, band()).unwrap();
        assert_eq!(c, DihedralClass::Smooth);
    }

    /// The ε-parameterized sliver: an angle chosen from the RUN's ε so
    /// the displacement margin lands strictly inside (ε, K·ε) at arm 1,
    /// at every CI ε row.
    #[test]
    fn near_tangent_planes_escalate() {
        let theta = 3.0 * eps(); // sin θ ≈ θ; margin ≈ 3ε ∈ (ε, 10ε)
        let n = Vec3::new(theta.sin(), 0.0, theta.cos());
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(n, Vec3::unit_y());
        let err = classify_dihedral(&s1, &s2, Point3::origin(), 1.0, band()).unwrap_err();
        assert_eq!(err.predicate, Some("dihedral_wedge"));
    }

    /// Cylinder tangent to a plane: exactly tangent classifies smooth at
    /// the contact line; tilting the plane by a definite angle flips it
    /// to transverse — the curvature arm (the radius) is what scales the
    /// band.
    #[test]
    fn plane_cylinder_tangency_and_corner() {
        let r = 2.0;
        let cyl = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, r),
            axis: Vec3::unit_y(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        // The plane z = 0 touches the cylinder along the x…no: along
        // the line through the origin parallel to y (lowest generator).
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let p = Point3::origin();
        let c = classify_dihedral(&s1, &cyl, p, 10.0, band()).unwrap();
        assert_eq!(c, DihedralClass::Smooth);
        // A definitely-tilted plane through the same line: transverse.
        let tilt = std::f64::consts::FRAC_PI_4;
        let s2 = plane(Vec3::new(tilt.sin(), 0.0, tilt.cos()), Vec3::unit_y());
        let c = classify_dihedral(&s2, &cyl, p, 10.0, band()).unwrap();
        assert_eq!(c, DihedralClass::Transverse);
    }

    /// At the cone apex the gradient is poison and the classification
    /// escalates as Invalid — never a guess, never a chart normal.
    #[test]
    fn cone_apex_escalates_as_poison() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_6,
            u_ref: Vec3::unit_x(),
        };
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let err = classify_dihedral(&cone, &s1, Point3::origin(), 1.0, band()).unwrap_err();
        assert_eq!(err.margin, geom_core::MarginDiag::Invalid);
    }

    /// Nurbs (representable-unimplemented) escalates as poison too.
    #[test]
    fn nurbs_escalates() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let err = classify_dihedral(
            &s1,
            &Surface::nurbs_placeholder(),
            Point3::origin(),
            1.0,
            band(),
        )
        .unwrap_err();
        assert_eq!(err.margin, geom_core::MarginDiag::Invalid);
    }
}
