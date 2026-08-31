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
//! # The material side (D1's tier-3 verdict table)
//!
//! [`DihedralClass`] compares tangent PLANES, so it is unsigned: the
//! cusp (wedge 0), the seam (π) and the knife slit (2π) all read
//! `Smooth`. Signing it takes the faces' material sides — each face's
//! outward normal, `Face::sense_sign · ∇F` — and that is
//! [`classify_material_pairing`]: aligned normals mean one material
//! side and the legal π seam; opposed normals mean the wedge is one of
//! the two ends. Which end is a SECOND-order fact, and
//! [`material_kappa_rel`] is the discriminant — the jet's κ_rel signed
//! into the plus face's outward frame. [`MaterialWedge`] carries the
//! table these two compose into, and the tier-3 validator is where the
//! declaration and the jet margin decide legality.
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
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign};

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
    let arm = folded_lever_arm(s1, s2, p, extent);
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

/// **The folded lever arm** of a surface pair at `p` (module docs):
/// `min(curvature arm of s1, curvature arm of s2, extent)` — the
/// smallest scale over which an angular or curvature defect between
/// the two accumulates into a gap. One home for the fold, because
/// three sites must lever against the SAME arm or their margins are
/// not comparable: the first-order wedge ([`classify_dihedral`]), the
/// material pairing ([`classify_material_pairing`]) and the
/// second-order jet margin the tier-3 validator decides.
pub fn folded_lever_arm<T: Real>(s1: &Surface<T>, s2: &Surface<T>, p: Point3<T>, extent: T) -> T {
    curvature_lever_arm(s1, p)
        .min(curvature_lever_arm(s2, p))
        .min(extent)
}

/// **The material wedge** an edge's two faces subtend at a sample —
/// D1's ratified tier-3 verdict table, in one enum.
///
/// The first-order classifier ([`DihedralClass`]) compares tangent
/// *planes* and is therefore unsigned: wedge 0, π and 2π all read
/// `Smooth`. The material verdict is that classification signed by the
/// two faces' outward normals (`Face::sense_sign · chart normal`), so
/// it distinguishes the three:
///
/// | verdict | wedge | legality |
/// |---|---|---|
/// | [`Transverse`](Self::Transverse) | ∈ (0, 2π) at the θ = ε/r margin | legal |
/// | [`Seam`](Self::Seam) | π | legal |
/// | [`Cusp`](Self::Cusp) | 0 | legal iff DECLARED `Tangent` and jet-determinate |
/// | [`Slit`](Self::Slit) | 2π | legal iff DECLARED `Tangent` and jet-determinate |
///
/// The two ends are one verdict under `revert`: reverting a body
/// negates every face's outward normal, which negates the material
/// κ_rel and maps `Cusp` ↔ `Slit` — so the pair is legal together or
/// not at all, exactly as the ruling states.
///
/// In-band κ_rel escalates and a collapsed κ_rel (osculation —
/// conformal contact, the lamina) is neither: it fails the
/// curve-locus condition, and no declaration cures it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialWedge {
    /// Wedge ∈ (0, 2π), bounded away from both ends: a genuine corner.
    Transverse,
    /// Wedge = π: the smooth seam — one material side, two faces.
    Seam,
    /// Wedge = 0: a cusp, material in the vanishing crescent between
    /// two tangent faces.
    Cusp,
    /// Wedge = 2π: a knife slit, the cusp's `revert` image — material
    /// everywhere but the vanishing crescent.
    Slit,
}

impl MaterialWedge {
    /// The verdict's name, for messages.
    pub fn name(self) -> &'static str {
        match self {
            Self::Transverse => "transverse",
            Self::Seam => "seam (wedge π)",
            Self::Cusp => "cusp (wedge 0)",
            Self::Slit => "slit (wedge 2π)",
        }
    }

    /// Whether this verdict is one of the two DECLARED-arm ends
    /// (wedge 0 or 2π) — the pair that needs a `Tangent` declaration
    /// and a jet-determinate contact to be legal at all.
    pub fn is_declared_arm(self) -> bool {
        matches!(self, Self::Cusp | Self::Slit)
    }
}

/// The **material pairing** at a tangency sample: which way the two
/// faces' material sides face each other across their shared tangent
/// plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialPairing {
    /// Outward normals definitely aligned: one material side, so the
    /// two faces continue one another — the legal π seam.
    Aligned,
    /// Outward normals definitely opposed: the material sides face
    /// each other and the wedge is one of the two ends, 0 or 2π.
    Opposed,
}

/// **`material_wedge_side`** — the material pairing of two faces at an
/// on-locus point where the tangent planes already classified
/// [`DihedralClass::Smooth`].
///
/// `sense_plus`/`sense_minus` are the faces' outward-normal signs
/// (`Face::sense_sign`), so `sense · ∇F` is each face's outward normal
/// and `n̂₊ · n̂₋` is the sign the wedge turns on: aligned ⇒ π,
/// opposed ⇒ 0 or 2π. This is the C1 lemma the declared-contact
/// verifier already decides between bodies (`contact_tangent_opposed`),
/// read edge-locally between two faces of ONE body — same construction,
/// same margin: the dot of unit normals levered by the folded arm
/// ([`folded_lever_arm`]), a displacement in meters.
///
/// # Errors
///
/// [`Indeterminate`]: predicate `"material_wedge_side"` — the margin
/// landed in the band or was poisoned, or (as
/// [`geom_core::MarginDiag::Invalid`]) classified `Zero`, which on a
/// definitely-smooth sample means the two encodings contradict each
/// other: unit normals whose tangent planes coincide cannot be
/// perpendicular, so the pairing question is not validly posed at this
/// site — the collapsed-arm gate's posture, one order over.
pub fn classify_material_pairing<T: Decide>(
    s_plus: &Surface<T>,
    sense_plus: T,
    s_minus: &Surface<T>,
    sense_minus: T,
    p: Point3<T>,
    arm: T,
    band: Band,
) -> Result<MaterialPairing, Indeterminate> {
    let n_plus = implicit_gradient(s_plus, p).normalize() * sense_plus;
    let n_minus = implicit_gradient(s_minus, p).normalize() * sense_minus;
    match decide(
        "material_wedge_side",
        Margin::levered(n_plus.dot(n_minus), arm),
        band,
    )? {
        Sign::Positive => Ok(MaterialPairing::Aligned),
        Sign::Negative => Ok(MaterialPairing::Opposed),
        Sign::Zero => Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("material_wedge_side"),
        }),
    }
}

/// **The cusp/slit discriminant**: the jet's relative transverse
/// curvature ([`crate::TangentJet::kappa_rel`]) signed into the PLUS
/// face's OUTWARD frame.
///
/// `kappa_rel` is `κ₊ − κ₋`, both measured against the plus surface's
/// implicit gradient `n̂`. Two sign steps carry it into the material
/// frame, and both are exact (a negation and a ±1 product), so the
/// magnitude — and with it the jet-determinacy verdict — is untouched:
///
/// 1. **The implicit convention.** Expanding `F = 0` in the frame
///    `(d̂, n̂)` gives `|∇F|·t + ½u²·d̂ᵀ∇²F d̂ = 0`, so a surface's HEIGHT
///    coefficient over its tangent plane along `+n̂` is `−κ`: a
///    positive jet curvature is a surface bending away from `n̂`.
/// 2. **The material side.** The plus face's outward normal is
///    `sense_plus · n̂`, so measuring the heights along it flips them
///    again when `sense_plus` is `−1`.
///
/// The result is `h₊ − h₋`, the two surfaces' height coefficients over
/// their shared tangent plane along the plus face's OUTWARD normal.
/// Positive ⇒ the plus sheet sits above the minus sheet on the side
/// its own material is NOT, so the material is the vanishing crescent
/// between them ([`MaterialWedge::Cusp`]); negative ⇒ that crescent is
/// the void and the material is everything else
/// ([`MaterialWedge::Slit`]).
pub fn material_kappa_rel<T: Real>(kappa_rel: T, sense_plus: T) -> T {
    -(kappa_rel * sense_plus)
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

    /// Two coplanar faces on one tangent plane: material sides agree
    /// (both senses +1 on the same normal) ⇒ the legal π seam; flip
    /// one face's sense and the same geometry is the 0/2π pair. The
    /// FIRST-order data is identical in both rows — that is the whole
    /// content of "the dihedral classification is unsigned".
    #[test]
    fn coplanar_faces_pair_by_their_senses_not_their_geometry() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(Vec3::unit_z(), Vec3::unit_y());
        let p = Point3::origin();
        assert_eq!(
            classify_dihedral(&s1, &s2, p, 1.0, band()).unwrap(),
            DihedralClass::Smooth
        );
        assert_eq!(
            classify_material_pairing(&s1, 1.0, &s2, 1.0, p, 1.0, band()).unwrap(),
            MaterialPairing::Aligned
        );
        assert_eq!(
            classify_material_pairing(&s1, 1.0, &s2, -1.0, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        // Antiparallel STORED normals with agreeing senses are the
        // same aligned pairing read through the other encoding: a
        // plane carries its reversal in the datum, a curved chart in
        // the sense bit, and the pairing sees only the product.
        let s3 = plane(-Vec3::unit_z(), Vec3::unit_x());
        assert_eq!(
            classify_material_pairing(&s1, 1.0, &s3, -1.0, p, 1.0, band()).unwrap(),
            MaterialPairing::Aligned
        );
    }

    /// The cusp/slit discriminant on the ruling's own figure: two
    /// kissing cylinders (radii 1 and 2, internally tangent at the
    /// origin along the y axis). The material crescent between them is
    /// the cusp; negating the plus face's material side — what
    /// `revert` does to every face at once — is the slit.
    #[test]
    fn kissing_cylinders_discriminate_cusp_from_slit() {
        let inner = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::unit_y(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let outer = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::unit_y(),
            radius: 2.0,
            u_ref: Vec3::unit_x(),
        };
        let p = Point3::origin();
        assert_eq!(
            classify_dihedral(&inner, &outer, p, 1.0, band()).unwrap(),
            DihedralClass::Smooth
        );
        // Material between them: the inner cylinder bounds it from
        // inside (outward normal points INTO the inner cylinder, i.e.
        // sense −1 against the outward-pointing implicit gradient),
        // the outer from outside (sense +1).
        assert_eq!(
            classify_material_pairing(&inner, -1.0, &outer, 1.0, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        let jet = crate::tangent_jet(&inner, &outer, p, Vec3::unit_y());
        // Plus = the inner cylinder, whose outward normal (sense −1)
        // points down: the crescent is the material ⇒ cusp.
        assert!(
            material_kappa_rel(jet.kappa_rel, -1.0) > 0.0,
            "kappa_rel {} in the material frame",
            material_kappa_rel(jet.kappa_rel, -1.0)
        );
        // Revert: every outward normal negates, the crescent becomes
        // the void, and the same edge is the slit.
        assert!(material_kappa_rel(jet.kappa_rel, 1.0) < 0.0);
    }

    /// Osculation: one surface against a coincident copy of itself.
    /// The pairing is opposed (a zero-thickness sheet), and the
    /// discriminant collapses exactly — no side to pick, which is the
    /// lamina the declared arm refuses.
    #[test]
    fn coincident_surfaces_osculate_with_no_side() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(Vec3::unit_z(), Vec3::unit_y());
        let p = Point3::origin();
        assert_eq!(
            classify_material_pairing(&s1, 1.0, &s2, -1.0, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        let jet = crate::tangent_jet(&s1, &s2, p, Vec3::unit_y());
        assert_eq!(material_kappa_rel(jet.kappa_rel, 1.0), 0.0);
    }

    /// The pairing escalates on a poisoned gradient exactly as the
    /// first-order classifier does — the cone apex, again.
    #[test]
    fn material_pairing_escalates_on_poison() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_6,
            u_ref: Vec3::unit_x(),
        };
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let err = classify_material_pairing(&cone, 1.0, &s1, 1.0, Point3::origin(), 1.0, band())
            .unwrap_err();
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
