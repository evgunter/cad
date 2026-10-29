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
//! - **Smooth** — tangent planes coincide at tolerance. This is
//!   UNSIGNED and therefore not a legality verdict: the legal π seam
//!   (coplanar splits, smooth profile joins, seams), the cusp (wedge
//!   0) and the knife slit (2π) all classify `Smooth`, because
//!   coincident tangent planes are all this predicate sees. Signing it
//!   is the material section below. The construction stores a
//!   conventional description (`MappedCurve`/`Seam`).
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
//! outward normal, its own `∇F` selected by the face's `Face::sense`
//! bit — and that is
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
//! `r = min(curvature arm of s₁, curvature arm of s₂, reach)`
//!
//! The curvature arms ([`crate::curvature_lever_arm`]) are the D4/D2
//! `1/κ_rel` reasoning in conservative per-surface form (min over the
//! two surfaces bounds 1/(κ₁ + κ₂) within a factor 2 — inside the
//! K-band's policy noise); planes contribute `+∞` (no curvature scale).
//! `reach` is the **faces' extent** (D4 ¶1's "face extent for
//! parallelism decisions"): how far the farther of the edge's two faces
//! runs from the edge, which `topo` derives from the body
//! (`topo::wedge_reach`) and every reader of an edge's wedge passes —
//! certification, tier 3's check 4 and the must-carry rule. A dihedral
//! is how far the two faces separate as they leave the edge, so it is
//! bounded by how far the faces run and how they curve, never by how
//! long the edge is: an edge's own extent would read a short arc
//! between large faces at dihedral δ as δ², a wedge the faces open
//! at δ. `sin θ` is used rather
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
//! zero — a zero/sub-ε reach, the cone-apex limit ρ → 0) maps **every**
//! angle into the coincidence band, so classifying there would return a
//! *definite* `Smooth` at what may be a true 90° corner — a definite
//! wrong answer, worse than no answer. The classifier therefore decides
//! the arm first (predicate `"dihedral_arm"`): definitely positive
//! proceeds; coincident-with-zero **escalates** with the margin the
//! funnel decided (with no displacement scale the wedge question is not
//! validly posed at this site); in-band or poisoned arms escalate
//! through the ordinary decide door. "Arm too small to say" is always
//! an escalation, never a classification, and every door ends it as
//! [`DIHEDRAL_ARM`], the arm's own decision, not the wedge's.
//!
//! # A seam with a spline face
//!
//! A spline face has no implicit form, so [`classify_dihedral`] reads
//! poison on it. [`classify_seam`] reads the same wedge, through the
//! same gate and predicates, with each face read by its kind
//! ([`SeamFace`]): an analytic face as above, a spline face at the
//! point's parameters on its own sheet, which the caller reads off the
//! edge's pcurve on that face (C4) and never off a projection foot,
//! since a foot can land on another sheet of the carrier. There the
//! normal is its chart's `S_u × S_v`, read only where the chart has
//! one ([`crate::regular_jet`], [`crate::chart_normal_at`]): within the
//! band of an irregular point or of a line the normal is not continuous
//! across, the seam escalates [`SeamEscalation::Crease`].
//!
//! Its curvature arm is bracketed, not read. Its upper end is the reach,
//! the fold's own ceiling. Its lower end is a certified lower bound on
//! the chart's length scale at the point ([`crate::NurbsLane::chart_arm`]),
//! whose slack has no bound: a certified curvature bound over-reads a
//! flat chart at uneven speed, a cell the bound refuses anywhere within
//! the arm stops it there, and curvature off the face's trim or near in
//! space but far along the sheet counts. A shorter arm shrinks the margin
//! toward Zero, so the seam reads smooth only where the wedge reads Zero
//! over the upper end, transverse only where it is decided past the band
//! over the lower end, and escalates at the arm between: the slack costs
//! an escalation, never a class. The two faces' arms and the reach are
//! folded in one place for both doors ([`fold_arms`]).

use core::ops::ControlFlow;

use crate::enters::LeverEscalation;
use geom::Surface;
use geom_core::k_stats::{Magnitude, NonzeroSign};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign};

use crate::implicit::{curvature_lever_arm, implicit_gradient, implicit_outward_normal};
use crate::recourse::{RefusedSide, SizedDecision, SizedPass};

/// Whether the folded lever arm at a point of an edge is positive: the
/// width the wedge between the edge's faces is metered over, the
/// shorter of the faces' extent and their radii of curvature. It
/// passes only on a definitely positive arm; a zero-band one is a size
/// a smaller tolerance decides. Every door that reads the dihedral's
/// [`crate::LeverRung::Arm`] ends it here, since the arm is a width and
/// the wedge an angle, save a door that refuses a nonzero wedge, which
/// withholds the offer. Its margin is the wedge the arm meters,
/// `sin θ · arm` (the arm's own where that wedge reads zero at the
/// tolerance deciding the arm), so the tolerance it offers decides the
/// arm and the wedge both. Its note names both ways the arm reaches no
/// width: the faces' extent (faces of none) and a face's radius (a
/// cone's point). Its refused side is one the lever reaches: a wedge below
/// zero needs a stored radius below zero, which a profile refuses at a
/// build and the at-rest representability check reports first.
pub const DIHEDRAL_ARM: SizedDecision = SizedDecision {
    lever: "move the geometry so that edge's faces clearly run farther and none curves tightly \
            there",
    size: "width or the gap its faces open",
    passes: SizedPass::Positive,
    side: RefusedSide::Lever,
    no_size: Some("faces of no extent or a cone's point leave no angle to measure"),
};

/// The clause [`DIHEDRAL_ARM`] decides, of an edge: every door that asks
/// it, or answers that the arm is not there, composes its sentence
/// around this one spelling, so the decision is told in one shape
/// (D4 ¶1 (iv)).
pub const DIHEDRAL_ARM_CLAUSE: &str = crate::dihedral_arm_clause!();

/// [`DIHEDRAL_ARM_CLAUSE`] as a literal, for `concat!`.
#[doc(hidden)]
#[macro_export]
macro_rules! dihedral_arm_clause {
    () => {
        "between faces wide enough, for how they curve, to measure their angle"
    };
}

/// The width [`DIHEDRAL_ARM`] decides, as a noun, for a door that names
/// the quantity rather than the question: the same word as the
/// decision's size and its clause's "faces wide enough".
pub const DIHEDRAL_ARM_NOUN: &str = "the width the faces' angle is measured across";

/// **The material pairing** ([`classify_material_pairing`]) as a
/// decision a refusal ends: it passes on either definite sign and
/// refuses in band and at zero. It is asked past a definitely-smooth
/// dihedral at the same point and arm, where the wedge `sin θ · arm` is
/// at most `ε`, so its margin `|cos θ| · arm` lies between
/// `√(arm² − ε²)` and the arm: within `ε²/arm` of the folded arm, the
/// size [`DIHEDRAL_ARM`] decides. A refused pairing is an arm too short
/// to read a side over, a length the user may intend, so both
/// band-decided arms end in the arm's lever with the tolerance the
/// margin gives (D4 ¶1 (i)); the door quotes the wedge where that
/// tolerance would leave the wedge undecided
/// ([`classify_material_pairing`]). A zero margin needs
/// `arm ≤ √2·ε` past an arm gate that decided `arm ≥ K·ε`, so a decided
/// Zero is reached only at `K < √2`.
pub const MATERIAL_PAIRING: SizedDecision = SizedDecision {
    lever: DIHEDRAL_ARM.lever,
    size: "length",
    passes: SizedPass::NonZero,
    side: RefusedSide::Lever,
    no_size: None,
};

/// The question [`MATERIAL_PAIRING`] decides, of an edge: every door that
/// ends its refusal names it in this one spelling (D4 ¶1 (iv)).
pub const MATERIAL_PAIRING_CLAUSE: &str = crate::material_pairing_clause!();

/// [`MATERIAL_PAIRING_CLAUSE`] as a literal, for `concat!`.
#[doc(hidden)]
#[macro_export]
macro_rules! material_pairing_clause {
    () => {
        "which side of an edge the material of its two smoothly meeting faces lies on"
    };
}

/// A definite dihedral classification (the indeterminate outcome is the
/// typed [`Indeterminate`] error — the sliver escalation, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralClass {
    /// Tangent planes definitely distinct: a genuine corner;
    /// `Intersection`'s transversality precondition holds here.
    Transverse,
    /// Tangent planes coincident at tolerance — and NOTHING more:
    /// this variant does not say the wedge is π, nor that the edge is
    /// legal. Tangent *planes* are all that is compared, so the seam
    /// (π), the cusp (0) and the knife slit (2π) all land here; which
    /// of the three it is takes the faces' material sides
    /// ([`MaterialWedge`], [`classify_material_pairing`]) and, for the
    /// two ends, the second-order jet.
    Smooth,
}

/// The dihedral and material-entry classifiers' funnel: delegates to
/// the unified recorder funnel [`geom_core::k_stats::decide`] (M2 PR 7),
/// which names the predicate for the margin-telemetry recorder,
/// classifies through the sanctioned [`Decide`] door, and tags any
/// escalation. It is not the crate's only decision site: other modules
/// (the curve-pair, pcurve-cache, SSI and props readers among them)
/// call the recorder funnel directly, which is the one door they all
/// share. The margin is a
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

/// [`decide`], keeping the reporting margin
/// ([`geom_core::k_stats::decide_reported`]) on every outcome: for a
/// refusal whose words read it — a sized decision's quoting the
/// tolerance that would decide it, and a residual's definite miss read
/// against the file's ε_in at the import door.
pub(crate) fn decide_reported<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Decided, Indeterminate> {
    geom_core::k_stats::decide_reported(name, margin, band)
}

/// The crate's **magnitude door**
/// ([`geom_core::k_stats::decide_magnitude`]): for a margin that is
/// nonnegative by construction, whose two signs are both verdicts.
pub(crate) fn decide_magnitude<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Magnitude, Indeterminate> {
    geom_core::k_stats::decide_magnitude(name, margin, band)
}

/// [`decide_magnitude`], keeping the reporting margin it decided on
/// ([`geom_core::k_stats::decide_magnitude_reported`]).
pub(crate) fn decide_magnitude_reported<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<(Magnitude, MarginDiag), Indeterminate> {
    geom_core::k_stats::decide_magnitude_reported(name, margin, band)
}

/// The crate's **collapsed-arm gate**, the same wrapper one door over
/// ([`geom_core::k_stats::decide_positive`]): for a predicate whose
/// question is only validly posed through a definitely-positive margin
/// — a folded lever arm, a metered span. The gate's refusal of a
/// definite non-positive sign is the funnel's escalation, recorded on
/// the open frame, so no caller here mints an [`Indeterminate`] the
/// verdict log cannot see.
pub(crate) fn decide_positive<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<(), Indeterminate> {
    geom_core::k_stats::decide_positive(name, margin, band)
}

/// The crate's **collapsed-discriminant gate**
/// ([`geom_core::k_stats::decide_nonzero`]): for a predicate that reads
/// a side off the margin's sign and has none to read at a definite
/// zero.
pub(crate) fn decide_nonzero<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<NonzeroSign, Indeterminate> {
    geom_core::k_stats::decide_nonzero(name, margin, band)
}

/// [`decide_nonzero`], keeping the reporting margin the passing sign was
/// decided on ([`geom_core::k_stats::decide_nonzero_reported`]).
pub(crate) fn decide_nonzero_reported<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<(NonzeroSign, MarginDiag), Indeterminate> {
    geom_core::k_stats::decide_nonzero_reported(name, margin, band)
}

/// **`dihedral_wedge`** — classifies the wedge between `s1` and `s2` at
/// the on-locus point `p` (module docs). Margin: `sin θ · r` in meters,
/// θ the tangent-plane angle from implicit gradients, `r` the folded
/// lever arm `min(curvature arms, reach)`; classified against the
/// run's linear band.
///
/// `reach` is the faces' extent (module docs): how far the farther of
/// the edge's two faces runs from it, never the edge's own extent.
///
/// # Errors
///
/// [`LeverEscalation`]: [`crate::LeverRung::Reading`], predicate
/// `"dihedral_wedge"` — the margin landed in the sliver band, or was
/// poisoned (off-locus garbage, a surface singularity such as the cone
/// apex, an unimplemented `Nurbs` kind); or [`crate::LeverRung::Arm`],
/// predicate `"dihedral_arm"` — the folded lever arm is collapsed
/// (coincident with zero, in-band, or poisoned), so no angle can
/// classify at this site (the collapsed-arm gate, module docs) —
/// escalate-never-guess, D4 ¶3.
pub fn classify_dihedral<T: Decide>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    p: Point3<T>,
    reach: T,
    band: Band,
) -> Result<DihedralClass, LeverEscalation> {
    wedge_decided(s1, s2, p, reach, band)
        .map(|(class, _)| class)
        .map_err(WedgeEscalation::into_lever)
}

/// **One face of a seam**, as [`classify_seam`] reads it at the seam's
/// point.
#[derive(Clone, Copy, Debug)]
pub enum SeamFace<'a, T: Real> {
    /// A face read off its carrier's implicit form at the point, as
    /// [`classify_dihedral`] reads both. A spline carrier has none, and is
    /// refused ([`SeamEscalation::SplineReadAsAnalytic`]).
    Analytic(&'a Surface<T>),
    /// A face on a spline chart, read at the point's chart parameters
    /// on the face's own sheet: the edge's pcurve on that face at the
    /// point's edge parameter (module docs).
    Spline {
        /// The face's chart.
        chart: &'a geom::NurbsSurface<T>,
        /// The point's parameters on it.
        uv: (T, T),
    },
}

/// Why [`classify_seam`] read no class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SeamEscalation {
    /// The arm gate or the wedge's own reading escalated, as
    /// [`classify_dihedral`]'s.
    Lever(LeverEscalation),
    /// A spline face's chart has no one normal at the point: it lies
    /// within the band of an irregular point of the chart or of a line
    /// its normal is not continuous across ([`crate::chart_normal_at`]).
    Crease(crate::CreaseReading),
    /// A spline carrier handed in as [`SeamFace::Analytic`], a caller's
    /// error: it has no implicit form to read, and no chart parameters
    /// were given to read its chart at.
    SplineReadAsAnalytic,
}

/// **`classify_seam`** — [`classify_dihedral`] for a seam whose faces
/// may be spline faces (module docs, "A seam with a spline face"): the
/// wedge between `faces` at the seam's point `p`, over the same arm
/// gate and wedge decision, with each face's normal and curvature arm
/// read by its kind. `chart_arm` reads a certified lower bound on a
/// spline chart's arm at the seam's point over the reach, at this scalar
/// ([`crate::NurbsLane::chart_arm`] where the scalar certifies); where it
/// reads none, that lower end is poison.
///
/// A spline face's arm is bracketed rather than read: it lies between
/// that lower bound and the reach, the fold's own ceiling. The seam reads
/// smooth only where the wedge reads Zero over the fold's upper end, and
/// transverse only where it is decided past the band over its lower end;
/// between, the arm decides the class, and the seam escalates at the arm
/// (`"dihedral_arm"`, the wedge's margin over the lower end). So a lower
/// bound's slack can cost an escalation and never a class. On two
/// analytic faces the two ends are one arm, and it is
/// [`classify_dihedral`], bit for bit.
///
/// # Errors
///
/// [`SeamEscalation`]: the arm or the wedge undecided, or a spline
/// face's point on a crease or an irregular point of its chart, or a
/// spline carrier read as analytic.
pub fn classify_seam<T: Decide>(
    faces: [SeamFace<'_, T>; 2],
    p: Point3<T>,
    reach: T,
    chart_arm: impl Fn(&geom::NurbsSurface<T>, Point3<T>, T) -> Option<f64>,
    band: Band,
) -> Result<DihedralClass, SeamEscalation> {
    let [a, b] = [
        seam_face_reading(faces[0], p, reach, &chart_arm, band)?,
        seam_face_reading(faces[1], p, reach, &chart_arm, band)?,
    ];
    let class = wedge_of(a.normal, b.normal, fold_arms(a.hi, b.hi, reach), band)
        .map(|(class, _)| class)
        .map_err(|e| SeamEscalation::Lever(e.into_lever()))?;
    if class == DihedralClass::Smooth || !(a.bracketed || b.bracketed) {
        return Ok(class);
    }
    let sin_theta = a.normal.cross(b.normal).norm() / (a.normal.norm() * b.normal.norm());
    let lo = fold_arms(a.lo, b.lo, reach);
    decide_positive("dihedral_arm", Margin::levered(sin_theta, lo), band)
        .map(|()| DihedralClass::Transverse)
        .map_err(|diag| SeamEscalation::Lever(LeverEscalation::arm(diag)))
}

/// A seam face as [`classify_seam`] reads it: its normal at the seam's
/// point, unnormalized, and its arm's two ends, one value where the arm
/// is read rather than bracketed.
struct SeamFaceReading<T: Real> {
    normal: geom_core::Vec3<T>,
    lo: T,
    hi: T,
    bracketed: bool,
}

/// A seam face's [`SeamFaceReading`] (module docs).
fn seam_face_reading<T: Decide>(
    face: SeamFace<'_, T>,
    p: Point3<T>,
    reach: T,
    chart_arm: &impl Fn(&geom::NurbsSurface<T>, Point3<T>, T) -> Option<f64>,
    band: Band,
) -> Result<SeamFaceReading<T>, SeamEscalation> {
    let (chart, uv) = match face {
        SeamFace::Analytic(s) if s.spline_chart().is_some() => {
            return Err(SeamEscalation::SplineReadAsAnalytic);
        }
        SeamFace::Analytic(s) => {
            let arm = curvature_lever_arm(s, p);
            return Ok(SeamFaceReading {
                normal: implicit_gradient(s, p),
                lo: arm,
                hi: arm,
                bracketed: false,
            });
        }
        SeamFace::Spline { chart, uv } => (chart, uv),
    };
    // Where the chart's normal is defined is the chart's own question,
    // read over its control net's reach as the sector walk reads it; the
    // seam's reach is the arm's alone.
    let net_reach = crate::control_net_reach(chart.control(), p);
    let jet = crate::regular_jet(chart, uv, net_reach, band).map_err(SeamEscalation::Crease)?;
    let (normal, _) = crate::chart_normal_at(chart, true, &jet, uv, net_reach, band)
        .map_err(SeamEscalation::Crease)?;
    Ok(SeamFaceReading {
        normal: normal.vec(),
        lo: chart_arm(chart, p, reach).map_or_else(crate::implicit::poison, T::from_computed),
        hi: reach,
        bracketed: true,
    })
}

/// Why [`wedge_decided`] could not classify: a rung of the levered
/// reading, or no tangent plane to read an angle between.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum WedgeEscalation {
    /// The arm gate or the wedge's own reading escalated.
    Lever(LeverEscalation),
    /// The wedge's margin is invalid because a surface's implicit
    /// gradient is zero or undefined somewhere over the point's
    /// enclosure: the product of the two gradient magnitudes `sin θ`
    /// divides by is a readable value that reaches zero (a point on a
    /// cylinder's axis, a sphere's centre; a cone's apex is refused
    /// earlier, at the arm gate), so no tangent plane, and no
    /// angle, is defined there. Carries the wedge decision's own
    /// escalation, which is what [`classify_dihedral`] reports.
    NoTangentPlane(Indeterminate),
}

impl WedgeEscalation {
    /// The levered escalation [`classify_dihedral`] reports: a missing
    /// tangent plane is the wedge reading's own invalid margin.
    fn into_lever(self) -> LeverEscalation {
        match self {
            Self::Lever(e) => e,
            Self::NoTangentPlane(diag) => LeverEscalation::reading(diag),
        }
    }
}

/// [`classify_dihedral`], keeping the wedge decision's reporting
/// margin for a refusal that quotes it (certification's
/// `NotTransverse`), and naming a missing tangent plane as its own
/// cause ([`WedgeEscalation::NoTangentPlane`]).
pub(crate) fn wedge_decided<T: Decide>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    p: Point3<T>,
    reach: T,
    band: Band,
) -> Result<(DihedralClass, geom_core::MarginDiag), WedgeEscalation> {
    wedge_of(
        implicit_gradient(s1, p),
        implicit_gradient(s2, p),
        folded_lever_arm(s1, s2, p, reach),
        band,
    )
}

/// The wedge between two faces whose normals at the point are `n1` and
/// `n2`, of any length, levered over the folded arm `arm`: the arm gate,
/// then the wedge decision ([`wedge_decided`]'s reading, whatever read
/// the normals).
fn wedge_of<T: Decide>(
    n1: geom_core::Vec3<T>,
    n2: geom_core::Vec3<T>,
    arm: T,
    band: Band,
) -> Result<(DihedralClass, geom_core::MarginDiag), WedgeEscalation> {
    let magnitudes = n1.norm() * n2.norm();
    let sin_theta = n1.cross(n2).norm() / magnitudes;
    // The collapsed-arm gate (module docs): the wedge margin is only
    // meaningful through a definitely-positive arm.
    decide_positive("dihedral_arm", Margin::of(arm), band).map_err(|gate| {
        WedgeEscalation::Lever(at_wedge(LeverEscalation::arm(gate), arm, sin_theta, band))
    })?;
    let margin = Margin::levered(sin_theta, arm);
    // The cause of an invalid margin, read only once the decision has
    // refused, so no verdict can move. Past a certified arm, `sin θ` is
    // invalid only through the gradients: a poisoned one (no implicit
    // form, an off-locus point) poisons their magnitudes too; a
    // readable product of magnitudes means the quotient is undefined
    // because that product reaches zero, or a gradient is undefined
    // over the enclosure. Poison is a structure question
    // (`Real::is_poison`), not a decision on the value.
    let Decided { sign, margin } =
        decide_reported("dihedral_wedge", margin, band).map_err(|diag| {
            if diag.margin.is_invalid() && !magnitudes.is_poison() {
                WedgeEscalation::NoTangentPlane(diag)
            } else {
                WedgeEscalation::Lever(LeverEscalation::reading(diag))
            }
        })?;
    let class = match sign {
        Sign::Positive => DihedralClass::Transverse,
        Sign::Zero => DihedralClass::Smooth,
        // Unreachable for a true magnitude (sin θ ≥ 0, arm ≥ 0): a
        // negative product would require poisoned inputs, which the
        // classifier already surfaced as Indeterminate. Kept total and
        // conservative: a definitely-negative "magnitude" still means
        // "definitely not coincident".
        Sign::Negative => DihedralClass::Transverse,
    };
    Ok((class, margin))
}

/// **The arm gate's escalation, quoted at the wedge it meters**: the arm
/// is in band or decided zero, and the reading it meters is the wedge
/// `sin θ · arm`, no longer than the arm up to the in-band excess of the
/// gradients' quotient over one (which
/// [`LeverEscalation::quoting_reading`] guards). A tolerance that decides the
/// arm but leaves that wedge in band reads no class, so the escalation
/// carries the wedge's own margin, through the arm gate's funnel
/// (`"dihedral_arm_wedge"`), and the tolerance it offers decides both.
/// Where the wedge reads zero at the tolerance that decides the arm
/// ([`wedge_reads_zero_at_the_arm`]), it reads smooth at every tolerance
/// from there down to its own, so the arm binds and keeps its own margin:
/// an exactly zero wedge, and one that is only rounding on a tangent
/// seam, whose own value would offer a tolerance many decades below the
/// one that already decides the seam. An arm no smaller tolerance a run
/// admits decides positive — poisoned, decided negative, a zero on the
/// negative side, or one only a tolerance below `EPS_MIN` decides —
/// keeps its own too: there is no tolerance to quote
/// ([`LeverEscalation::re_quotes`]).
fn at_wedge<T: Decide>(
    escalation: LeverEscalation,
    arm: T,
    sin_theta: T,
    band: Band,
) -> LeverEscalation {
    let wedge = sin_theta * arm;
    // `arm / wedge` is finite unless the wedge is exactly zero, or poison
    // (a gradient the arm's decided zero leaves unread, at a cone's apex).
    if !escalation.re_quotes()
        || !geom_core::is_finite_length(arm / wedge)
        || wedge_reads_zero_at_the_arm(sin_theta, band)
    {
        return escalation;
    }
    match decide_positive("dihedral_arm_wedge", Margin::of(wedge.abs()), band) {
        Err(diag) => escalation.quoting_reading(diag),
        // Only where `sin θ`'s in-band excess over one carries the wedge
        // past the band: the arm keeps its own margin, whose tolerance
        // decides that wedge too.
        Ok(()) => escalation,
    }
}

/// **Whether the wedge reads zero at the arm's own tolerance**: the
/// tolerance that decides the arm, `arm/K`, is the one whose band
/// escalates at the arm, and in that band the wedge `sin θ · arm` reads
/// as `sin θ · escalate` does in `band` (both scaled by `escalate/arm`).
/// So the reading is the wedge levered over `band`'s own escalation
/// edge, decided in `band` (`"dihedral_wedge_at_arm"`): zero exactly
/// where `sin θ · arm ≤ arm/K`, with no threshold of its own.
fn wedge_reads_zero_at_the_arm<T: Decide>(sin_theta: T, band: Band) -> bool {
    matches!(
        decide_reported(
            "dihedral_wedge_at_arm",
            Margin::levered(sin_theta, T::from_f64(band.escalate())),
            band,
        ),
        Ok(Decided {
            sign: Sign::Zero,
            ..
        })
    )
}

/// **The folded lever arm** of a surface pair at `p` (module docs):
/// `min(curvature arm of s1, curvature arm of s2, reach)` — the
/// smallest scale over which an angular or curvature defect between
/// the two accumulates into a gap. One home for the fold, because
/// three sites must lever against the SAME arm or their margins are
/// not comparable: the first-order wedge ([`classify_dihedral`]), the
/// material pairing ([`classify_material_pairing`]) and the
/// second-order jet margin the tier-3 validator decides.
///
/// `crate::certify` reaches it directly and through
/// [`tangent_second_order`], the
/// smooth-join constructors through [`must_carry_over_edge`], which
/// composes it, and `topo::boolean::contact_verify` directly. The SSI
/// point decisions do not read it: their arm is
/// `crate::ssi::point_arm`'s, from the surfaces' largest principal
/// curvature, which this fold's [`curvature_lever_arm`] is not on a
/// cone or a torus. Nor does `contact_verify`'s
/// `contact_tangent_opposed`, the same C1 lemma as
/// [`classify_material_pairing`] between bodies: it levers its normals'
/// dot by the faces' extent, as `crate::certify`'s
/// `tangent_gap_orientation` does.
/// A new site levering against its own fold is a silent
/// non-comparability, so route new callers through this function.
pub fn folded_lever_arm<T: Real>(s1: &Surface<T>, s2: &Surface<T>, p: Point3<T>, reach: T) -> T {
    fold_arms(
        curvature_lever_arm(s1, p),
        curvature_lever_arm(s2, p),
        reach,
    )
}

/// **The fold** of two faces' arms and the faces' extent: the least of
/// the three. [`folded_lever_arm`] folds two analytic arms here and
/// [`classify_seam`] folds each face's by its kind. An arm a face cannot
/// state (a NURBS carrier's implicit one) is poison and poisons the fold.
fn fold_arms<T: Real>(arm1: T, arm2: T, reach: T) -> T {
    arm1.min(arm2).min(reach)
}

/// **`tangent_second_order`** — the must-carry rule's one metered
/// spelling (D2/OQ7): at a point `p` where the tangent planes already
/// coincide ([`DihedralClass::Smooth`]), do the two surfaces still
/// DETERMINE the locus one order up?
///
/// The margin is the sagitta the relative transverse normal curvature
/// subtends over the pair's folded lever arm — `|κ_rel|·arm²/2` in
/// meters, `κ_rel` from [`crate::tangent_jet`] along `tangent` and
/// `arm` from [`folded_lever_arm`] — classified against the run's
/// linear band:
///
/// - **`Positive`** — jet-determinate: the surfaces determine the
///   locus, so prefer-intrinsic demands the intrinsic
///   [`crate::EdgeDescription::TangentIntersection`] and a constructor
///   that stores less is storing a description tier 3 will refuse.
/// - **`Zero`/`Negative`** — under-determined (a G2 join, a
///   same-surface split, coplanar planes): the conventional
///   description is the honest one, BY THIS PREDICATE.
/// - **`Err`** — in-band: near-osculating geometry that is certifiable
///   as neither, escalated typed at the caller (D4 ¶3).
///
/// **Why one home.** The constructor that stores the description and
/// the tier-3 arm that demands it must read the SAME quantity under
/// the SAME predicate name, or the demanded set and the stored set are
/// two sets and every disagreement is a spurious
/// `DescriptionNotAdjacent`. Every smooth-join arm in the sweep verbs,
/// the boolean rebuild's smooth seams and the split's section boundary
/// route here through [`must_carry_over_edge`], which is where the
/// gate and the verdict policy live; it, the tier-3 validator's check 4
/// (`topo::validate`) and the boolean's shared-rim routing
/// (`topo::boolean::rim_wedge`) read their stations through the one
/// [`second_order_walk`]. `Intersection`-tangency certification folds
/// this reading into a walk of its own. One
/// hand-rolled sibling remains, issue 1439's work:
/// `topo::boolean::contact_verify`'s, which meters
/// `Margin::sagitta(|κ_rel| − drift, arm)` under its own predicate
/// (`"contact_tangent_second_order"`). A new site spelling its own is a
/// silent non-comparability.
///
/// The [`SecondOrder`] return carries the jet and the arm beside the
/// verdict, because the two callers that fold this into a longer walk
/// need the same numbers again a line later (the tangency certificate's
/// parallelism lever arm, the wedge's signed cusp side) and computing
/// them twice is how two spellings start.
pub fn tangent_second_order<T: Decide>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    p: Point3<T>,
    tangent: geom_core::Vec3<T>,
    reach: T,
    band: Band,
) -> SecondOrder<T> {
    let jet = crate::tangent::tangent_jet(s1, s2, p, tangent);
    let arm = folded_lever_arm(s1, s2, p, reach);
    let verdict = second_order_verdict(&jet, arm, band);
    SecondOrder { jet, arm, verdict }
}

/// The one `"tangent_second_order"` decision: the sagitta `|κ_rel|`
/// subtends over `arm`, classified against `band`.
fn second_order_verdict<T: Decide>(
    jet: &crate::TangentJet<T>,
    arm: T,
    band: Band,
) -> Result<Decided, Indeterminate> {
    decide_reported(
        "tangent_second_order",
        Margin::sagitta(jet.kappa_rel.abs(), arm),
        band,
    )
}

/// **The edge-level stations**: the certification schedule's interior
/// (`1..`[`crate::CERT_SAMPLES`]`-1`, through [`crate::sample_param`])
/// over `[t0, t1]`, as the carrier's point and tangent at each — the one
/// home of the schedule the must-carry rule's first-order gate,
/// [`second_order_walk`] and tier 3's check 4 read.
///
/// Kernel-internal: public only so `topo`'s tier 3 reads the same
/// stations, and hidden from the docs of the crates that re-export this
/// one.
#[doc(hidden)]
pub fn interior_stations<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
) -> impl Iterator<Item = (Point3<T>, geom_core::Vec3<T>)> + '_ {
    (1..crate::CERT_SAMPLES - 1).map(move |i| carrier.ders1(crate::sample_param(t0, t1, i)))
}

/// One station of [`second_order_walk`]: the contact point, and the
/// two quantities its second-order margin is read from.
/// Kernel-internal, as [`second_order_walk`] is.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct Station<T: Real> {
    /// The contact point the caller's schedule placed.
    pub p: Point3<T>,
    /// The pair's jet at `p` along the carrier tangent
    /// ([`crate::tangent_jet`]).
    pub jet: crate::TangentJet<T>,
    /// The folded lever arm at `p` ([`folded_lever_arm`]).
    pub arm: T,
}

/// A caller's per-station reads inside [`second_order_walk`]: one
/// before the station's second-order decision, one after a `Positive`
/// one. Either may stop the walk, which then answers
/// [`SecondOrderWalk::Stopped`] with the hook's break value. `()` is
/// the hook that reads nothing and never stops. Kernel-internal, as
/// [`second_order_walk`] is.
#[doc(hidden)]
pub trait StationHook<T: Real> {
    /// What a stopped walk carries back.
    type Break;
    /// Read at each station before its second-order decision.
    fn before_decision(&mut self, _station: &Station<T>) -> ControlFlow<Self::Break> {
        ControlFlow::Continue(())
    }
    /// Read at each station whose second-order decision was `Positive`.
    fn after_positive(&mut self, _station: &Station<T>) -> ControlFlow<Self::Break> {
        ControlFlow::Continue(())
    }
}

impl<T: Real> StationHook<T> for () {
    type Break = core::convert::Infallible;
}

/// [`second_order_walk`]'s answer. Kernel-internal, as the walk is.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecondOrderWalk<B> {
    /// Every station read `Positive`.
    Determinate,
    /// The first station not `Positive` read `Zero`/`Negative`.
    UnderDetermined,
    /// The first station not `Positive` was in band.
    InBand(Indeterminate),
    /// The hook stopped the walk.
    Stopped(B),
}

/// **The second-order walk** — [`tangent_second_order`]'s reading at
/// each of `stations` (a point on the contact and the tangent there),
/// in order, where the first station not `Positive` decides.
///
/// The one home of the walk's decision, which the must-carry rule
/// ([`must_carry_over_edge`]), tier 3's check 4 (`topo::validate`) and
/// the boolean's shared-rim routing (`topo::boolean::rim_wedge`) all
/// ask: the constructor that stores a description and the validators
/// that demand it read one walk, so the demanded set and the stored set
/// are one set. The caller owns the schedule — an edge passes
/// [`interior_stations`], a closed rim its every uniform phase — and a
/// caller with reads of its own at each station (the material pairing
/// and cusp side) takes them through `hook`, not a second loop.
///
/// The walk assumes the caller has established the contact as smooth
/// first-order at every station; it does not gate.
///
/// Kernel-internal: public only for `topo`, and hidden from the docs of
/// the crates that re-export this one.
#[doc(hidden)]
pub fn second_order_walk<T: Decide, H: StationHook<T>>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    stations: impl IntoIterator<Item = (Point3<T>, geom_core::Vec3<T>)>,
    reach: T,
    band: Band,
    hook: &mut H,
) -> SecondOrderWalk<H::Break> {
    for (p, tau) in stations {
        let station = Station {
            p,
            jet: crate::tangent::tangent_jet(s1, s2, p, tau),
            arm: folded_lever_arm(s1, s2, p, reach),
        };
        if let ControlFlow::Break(b) = hook.before_decision(&station) {
            return SecondOrderWalk::Stopped(b);
        }
        match second_order_verdict(&station.jet, station.arm, band) {
            Ok(Decided {
                sign: Sign::Positive,
                ..
            }) => {}
            Ok(_) => return SecondOrderWalk::UnderDetermined,
            Err(source) => return SecondOrderWalk::InBand(source),
        }
        if let ControlFlow::Break(b) = hook.after_positive(&station) {
            return SecondOrderWalk::Stopped(b);
        }
    }
    // No station at all reads nothing, the vacuous determinacy the
    // empty walk has always answered.
    SecondOrderWalk::Determinate
}

/// [`tangent_second_order`]'s reading: the verdict, and the two
/// quantities it was read from.
#[derive(Clone, Copy, Debug)]
pub struct SecondOrder<T: geom_core::Real> {
    /// The pair's jet at the sample ([`crate::tangent_jet`]).
    pub jet: crate::TangentJet<T>,
    /// The folded lever arm the sagitta was subtended over
    /// ([`folded_lever_arm`]).
    pub arm: T,
    /// The classified sagitta — `Positive` jet-determinate,
    /// `Zero`/`Negative` under-determined, `Err` in-band (predicate
    /// `"tangent_second_order"`) — with its reporting margin.
    pub verdict: Result<Decided, Indeterminate>,
}

/// **The must-carry rule over an EDGE** — [`tangent_second_order`]'s
/// contract asked of a whole smooth join rather than one point, and
/// the one place a constructor decides what description such a join
/// carries.
///
/// The answer is typed: the metered predicate's own three ways, plus
/// the first-order refutation its premise needs:
///
/// - **[`MustCarryVerdict::JetDeterminate`]** — every station read
///   `Positive` and the pair is inside
///   [`crate::tangent_certificate_lane`]: the surfaces determine the
///   locus along the whole edge and the certificate can store it, so
///   prefer-intrinsic (D2/OQ7) demands the intrinsic
///   [`crate::EdgeDescription::TangentIntersection`].
/// - **[`MustCarryVerdict::UnderDetermined`]** — every station read
///   was smooth first-order and definite second-order, and no
///   intrinsic tangency is demanded: a station's second-order
///   separation read `Zero`/`Negative`, or the pair is outside the
///   certificate's lane and cannot store one (see the variant). The
///   conventional description is the honest one either way.
/// - **[`MustCarryVerdict::InBand`]** — a station was certifiable as
///   neither, carrying that station's escalation and the reading that
///   raised it ([`MustCarryEscalation`]): the caller refuses TYPED
///   (D4 ¶3). An in-band verdict is never silently either side, so no
///   caller may fold it into "conventional".
/// - **[`MustCarryVerdict::Transverse`]** — every station's tangent
///   planes are definitely distinct: the join is a corner, not a smooth
///   join, and the rule has no description to choose for it.
/// - **[`MustCarryVerdict::Mixed`]** — some stations read definitely
///   transverse and others definitely smooth, and none is in band. The
///   dihedral is continuous along the edge, so two neighbouring
///   stations that disagree bracket an in-band point the schedule did
///   not sample (D2): the edge is refused typed, naming those two
///   stations ([`MixedStations`]).
///
/// **Each station is gated first-order before it is metered
/// second-order.** [`tangent_second_order`]'s transverse direction
/// `n̂₁ × τ̂` is tangent to both surfaces only where their normals are
/// parallel, so off a smooth join its reading depends on the argument
/// order. Each station therefore asks [`classify_dihedral`] first —
/// symmetric in its two surfaces, levered against the same
/// [`folded_lever_arm`] — and only a `Smooth` station reaches the jet.
/// The arm is classified before any angle, so over a reach inside
/// the band's escalation threshold a genuine tangency answers `InBand`
/// under `"dihedral_arm"`, not `UnderDetermined`.
///
/// **The verdict is the whole answer, and the only number that rides
/// with it is the DECIDING station's.** A reading taken beside the
/// verdict would be the first station's, which on a refusal is the one
/// station that did NOT fail — a caller reporting its margin as the
/// cause would report a margin that passed. The station that decided
/// is the one a caller has to name, and it is already inside
/// [`MustCarryVerdict::InBand`]'s [`MustCarryEscalation`]: which
/// reading raised it (the first-order rung, or the second-order
/// sagitta), its margin, its band and the predicate it was classified
/// under. [`MustCarryVerdict::Mixed`] names the two stations that
/// bracket its unsampled in-band point. The three other verdicts carry
/// no cause, and a caller that wants the jet re-reads it at the station
/// it cares about through [`tangent_second_order`].
///
/// **The lane gates the DEMAND, and only that.**
/// [`crate::tangent_certificate_lane`] says whether the jet
/// certificate can store an intrinsic tangency for this carrier over
/// this pair, so an out-of-lane edge whose stations all read
/// `Positive` answers `UnderDetermined`, not `JetDeterminate`. Both
/// readings are every pair's: tier 3 refuses an in-band sagitta on
/// every definitely-smooth edge, lane or not, so an in-band station
/// cannot be answered conventionally whatever the certificate could
/// store. An out-of-lane pair answers `UnderDetermined` only once every
/// station read is `Smooth` and definite second-order.
///
/// **A `Nurbs` or `Approx` surface answers `InBand` at the first
/// station, whatever its geometry.** Neither kind has an implicit
/// form, so [`implicit_gradient`] returns poison and
/// [`classify_dihedral`] escalates with an invalid margin — a
/// genuinely smooth join included. That refusal means "kind not
/// implemented", not ill-conditioned geometry, and the recourse an
/// [`Indeterminate`] renders for an invalid margin (check the inputs,
/// then the coincidence levers) names no lever that reaches it.
///
/// **The stations are the certification schedule's interior**
/// ([`interior_stations`]), read in two passes, in tier 3's order.
/// Every station is classified first-order before any is metered
/// second-order: an in-band station anywhere answers `InBand`, else a
/// transverse one anywhere answers `Transverse` (every station) or
/// `Mixed` (some); only an edge smooth at every station descends into
/// [`second_order_walk`], where the first station not `Positive`
/// decides. The tier-3 must-carry arm re-asks
/// this question of the stored description through the same walk, and
/// that is what keeps the demanded set and the stored set ONE set: a
/// constructor reading a coarser schedule can store a description tier
/// 3 then refuses, and one reading a finer schedule can refuse what
/// tier 3 would have accepted. The order is part of that: tier 3
/// escalates at any first-order in-band station, so a walk that
/// answered `Transverse` or `Mixed` from an earlier station would
/// refuse an edge in other words than tier 3 does.
///
/// **Why the extra stations never disagree on the joins this kernel
/// mints**, stated because it is an argument and not a licence to read
/// fewer: an extruded wall is ruled in the sweep direction and the
/// strut is one of its rulings, along which both surfaces' Hessians
/// and the transverse direction are constant — so `κ_rel` is constant
/// along it; an extruded cap rim that reads smooth is a line between
/// two planes, whose normals are constant along it; and a revolve's
/// latitude join carries a circle coaxial with both surfaces of
/// revolution, along which the configuration is carried by a symmetry
/// flow of both — so `κ_rel` is constant there too. Each fact holds
/// for the pairs ONE caller mints, while this walk must hold for every
/// pair it is handed, so none licenses reading one station.
///
/// **The one home of the EDGE-level rule** — gate and verdict policy,
/// over the stations [`interior_stations`] owns and the walk
/// [`second_order_walk`] owns — as [`folded_lever_arm`] is of the fold
/// and [`tangent_second_order`] of the metered margin. A constructor
/// spelling its own is a second verdict policy, and the in-band case
/// is where such spellings have disagreed.
pub fn must_carry_over_edge<T: Decide>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    reach: T,
    band: Band,
) -> MustCarryVerdict {
    let mut transverse = Vec::with_capacity(crate::CERT_SAMPLES as usize - 2);
    for (p, _) in interior_stations(carrier, t0, t1) {
        match classify_dihedral(s1, s2, p, reach, band) {
            Ok(DihedralClass::Smooth) => transverse.push(false),
            Ok(DihedralClass::Transverse) => transverse.push(true),
            Err(escalation) => {
                return MustCarryVerdict::InBand(MustCarryEscalation::FirstOrder(escalation));
            }
        }
    }
    if let Some(verdict) = MixedStations::corner_verdict(&transverse) {
        return verdict;
    }
    match second_order_walk(
        s1,
        s2,
        interior_stations(carrier, t0, t1),
        reach,
        band,
        &mut (),
    ) {
        SecondOrderWalk::Determinate
            if crate::tangent::tangent_certificate_lane(carrier, s1, s2) =>
        {
            MustCarryVerdict::JetDeterminate
        }
        SecondOrderWalk::Determinate | SecondOrderWalk::UnderDetermined => {
            MustCarryVerdict::UnderDetermined
        }
        SecondOrderWalk::InBand(source) => {
            MustCarryVerdict::InBand(MustCarryEscalation::SecondOrder(source))
        }
        SecondOrderWalk::Stopped(never) => match never {},
    }
}

/// **Two neighbouring stations that disagree** — one read definitely
/// transverse, the other definitely smooth — named by their indices in
/// the certification schedule (`1..`[`crate::CERT_SAMPLES`]`-1`, the
/// `sample` a certification refusal names). The dihedral is
/// continuous along the edge, so between them it passes through the
/// ambiguous band at a point no station read (D2): the edge is
/// certifiable as neither a corner nor a smooth join.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MixedStations {
    /// The station that read the faces meeting at a corner.
    pub transverse: u32,
    /// Its neighbour, which read them meeting smoothly.
    pub smooth: u32,
}

impl MixedStations {
    /// The corner verdict over the first-order readings of the
    /// interior stations in schedule order (`true` = transverse):
    /// `None` when no station read a corner, [`MustCarryVerdict::Transverse`]
    /// when every one did, else [`MustCarryVerdict::Mixed`] naming the
    /// first neighbouring pair that disagrees.
    fn corner_verdict(transverse: &[bool]) -> Option<MustCarryVerdict> {
        if !transverse.contains(&true) {
            return None;
        }
        Some(
            Self::bracketing(transverse)
                .map_or(MustCarryVerdict::Transverse, MustCarryVerdict::Mixed),
        )
    }

    /// The first neighbouring pair of interior stations whose
    /// first-order readings disagree, over the readings in schedule
    /// order (`true` = transverse, interior station `k` being schedule
    /// sample `k + 1`); `None` where they all agree. The one home of the
    /// bracket the must-carry rule and tier 3 name.
    #[must_use]
    pub fn bracketing(transverse: &[bool]) -> Option<Self> {
        let k = transverse.windows(2).position(|w| w[0] != w[1])?;
        let sample = |i: usize| {
            u32::try_from(i + 1).unwrap_or_else(|_| {
                unreachable!("an interior station indexes a schedule of `CERT_SAMPLES` samples")
            })
        };
        let (a, b) = (sample(k), sample(k + 1));
        Some(if transverse[k] {
            Self {
                transverse: a,
                smooth: b,
            }
        } else {
            Self {
                transverse: b,
                smooth: a,
            }
        })
    }
}

impl core::fmt::Display for MixedStations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the faces meet at a corner at sample {} and smoothly at sample {}, so they \
             turn tangent between them, too near to classify",
            self.transverse, self.smooth
        )
    }
}

/// What a join entered as definitely smooth stores, by the must-carry
/// rule's verdict ([`MustCarryVerdict::description`]).
#[derive(Clone, Debug)]
pub enum MustCarryDescription<T: Real> {
    /// The intrinsic [`crate::EdgeDescriptionSpec::TangentIntersection`]
    /// over the pair the rule was asked about, in that order; the caller
    /// stores it on its carrier.
    Intrinsic(crate::EdgeDescriptionSpec<T>),
    /// The conventional description: a locus the pair under-determines
    /// ([`crate::EdgeDescriptionSpec::OnPair`]). Whether the stored
    /// carrier is restated or the exact carrier stated is the caller's:
    /// a sweep-minted edge already stores its locus and restates it,
    /// while a blend contact edge stores a chord scaffold and states the
    /// locus fresh ([`MustCarryDescription::spec`]).
    Conventional,
}

impl<T: Real> MustCarryDescription<T> {
    /// The description stated fresh: the intrinsic tangency, or the
    /// conventional one where the edge rests (`rest`, which the body's
    /// one home decides: `topo::rest_locus`).
    #[must_use]
    pub fn spec(self, rest: RestLocus) -> crate::EdgeDescriptionSpec<T> {
        match self {
            Self::Intrinsic(description) => description,
            Self::Conventional => rest.spec(),
        }
    }
}

/// **Where a conventional edge between two faces rests** (D2): in the
/// chart of the one surface both faces lie on — one record, or two
/// records of one carrier, whose incidence on both is incidence on one
/// and so determines nothing — or as the pair of two distinct surfaces
/// that under-determine it. Decided in one home, `topo`'s
/// `topo::rest_locus`, which reads coincidence by the carrier-equality
/// ladder; every door states what that home decides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestLocus {
    /// The one surface's chart.
    Chart(crate::SurfaceKey),
    /// The pair of two distinct surfaces.
    Pair(crate::SurfacePair),
}

impl RestLocus {
    /// The conventional description resting there, derived from the
    /// carrier: an image the door derives, or the pair.
    #[must_use]
    pub fn spec<T: Real>(self) -> crate::EdgeDescriptionSpec<T> {
        match self {
            Self::Chart(surface) => crate::EdgeDescriptionSpec::chart(surface),
            Self::Pair(pair) => crate::EdgeDescriptionSpec::on_pair(pair),
        }
    }
}

/// A join entered as definitely smooth, as the must-carry rule is asked
/// about it ([`must_carry_description`]): its two surfaces with their
/// keys, in either order, and the edge between them — its carrier, its
/// parameter interval and its witness — with the faces' extent the
/// wedge is levered at ([`folded_lever_arm`]).
#[derive(Clone, Copy)]
pub struct SmoothJoin<'a, T: Real> {
    /// The two surfaces, each with its key.
    pub surfaces: [(crate::SurfaceKey, &'a Surface<T>); 2],
    /// The edge's carrier.
    pub carrier: &'a geom::Curve3<T>,
    /// The edge's parameter interval on the carrier.
    pub interval: (T, T),
    /// How far the farther of the edge's two faces runs from it
    /// (`topo::wedge_reach`), never the edge's own extent.
    pub reach: T,
    /// The witness an intrinsic description stores.
    pub witness: Point3<T>,
}

/// **What a definitely-smooth join stores**, or the caller's typed
/// refusal: the surfaces put in their pair's key order
/// ([`crate::SurfacePair::sorted`]), the rule over the edge
/// ([`must_carry_over_edge`]), and its verdict mapped to a description
/// ([`MustCarryVerdict::description`]): the conventional one is the
/// pair itself ([`crate::EdgeDescriptionSpec::OnPair`]), so no caller
/// chooses a chart.
///
/// Each refusal is built by the caller, from the reading that raised it
/// ([`DihedralReading::of_must_carry`]) and its diagnostics: an in-band
/// first-order station (the dihedral, by the rung that raised it)
/// through `first_order`, an in-band second-order station (the bend)
/// through `second_order`, a refuted smooth premise through `refuted`
/// — with the two bracketing stations where the edge is mixed
/// ([`MixedStations`]), `None` where every station read a corner.
///
/// # Errors
///
/// What `first_order`, `second_order` or `refuted` builds, on an
/// in-band, transverse or mixed verdict.
pub fn must_carry_description<T: Decide, E>(
    join: SmoothJoin<'_, T>,
    band: Band,
    first_order: impl FnOnce(DihedralReading, Indeterminate) -> E,
    second_order: impl FnOnce(DihedralReading, Indeterminate) -> E,
    refuted: impl FnOnce(Option<MixedStations>) -> E,
) -> Result<MustCarryDescription<T>, E> {
    let [(ka, a), (kb, b)] = join.surfaces;
    let (pair, [sa, sb]) = crate::SurfacePair::sorted(ka, a, kb, b);
    let (t0, t1) = join.interval;
    match must_carry_over_edge(sa, sb, join.carrier, t0, t1, join.reach, band)
        .description(pair, join.witness)
    {
        Ok(description) => Ok(description),
        Err(MustCarryRefusal::InBand(escalation)) => {
            let (reading, diag) = DihedralReading::of_must_carry(escalation);
            Err(match escalation {
                MustCarryEscalation::FirstOrder(_) => first_order(reading, diag),
                MustCarryEscalation::SecondOrder(_) => second_order(reading, diag),
            })
        }
        Err(MustCarryRefusal::Refuted) => Err(refuted(None)),
        Err(MustCarryRefusal::Mixed(stations)) => Err(refuted(Some(stations))),
    }
}

/// Why a conventional pair re-asked the rule ([`reask_pair`]) has no
/// description.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PairRefusal {
    /// A station certifiable as neither, with its escalation.
    InBand(MustCarryEscalation),
    /// Neighbouring stations that disagree ([`MixedStations`]).
    Mixed(MixedStations),
}

/// **A conventional pair re-asked the must-carry rule** after a door
/// changed its edge: its span (a split, a join, a re-anchored end), its
/// faces' extent `reach`, or its surfaces (an offset). The verdict reads
/// the stations over the span and the wedge at `reach` as well as the
/// pair, so an `OnPair` stays one only where the rule still reads the
/// join under-determined over the edge as it now stands. A corner
/// is the pair's `Intersection` and a determinate tangency its
/// `TangentIntersection`, each witnessed at the carrier's middle and
/// derived, so a declaration does not ride either; under-determined
/// keeps the pair with `declared`.
///
/// # Errors
///
/// [`PairRefusal`] on an in-band or mixed verdict.
pub fn reask_pair<T: Decide>(
    surfaces: [(crate::SurfaceKey, &Surface<T>); 2],
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    reach: T,
    declared: Option<crate::mapped::MappedCurve<T>>,
    band: Band,
) -> Result<crate::EdgeDescriptionSpec<T>, PairRefusal> {
    let [(ka, a), (kb, b)] = surfaces;
    let (pair, [sa, sb]) = crate::SurfacePair::sorted(ka, a, kb, b);
    let witness = carrier.mid_point(t0, t1);
    match must_carry_over_edge(sa, sb, carrier, t0, t1, reach, band) {
        MustCarryVerdict::Transverse => Ok(crate::EdgeDescriptionSpec::Intersection {
            pair,
            selector: crate::CurveSelector::Witness(witness),
        }),
        MustCarryVerdict::JetDeterminate => {
            Ok(crate::EdgeDescriptionSpec::TangentIntersection { pair, witness })
        }
        MustCarryVerdict::UnderDetermined => {
            Ok(crate::EdgeDescriptionSpec::OnPair { pair, declared })
        }
        MustCarryVerdict::Mixed(stations) => Err(PairRefusal::Mixed(stations)),
        MustCarryVerdict::InBand(escalation) => Err(PairRefusal::InBand(escalation)),
    }
}

/// **A chart image a door kept, re-asked the must-carry rule** over
/// the edge as the door leaves it: its extent (a split, a join, a
/// re-anchored end) or its surfaces (an offset). `surfaces` are the
/// edge's two sides after the door, each with its key, and `reach` the
/// wedge's lever ([`folded_lever_arm`]: the faces' extent).
///
/// A chart image is a conventional form: it claims the two surfaces
/// under-determine the locus, which the rule reads over the edge and
/// its faces' reach as well as its pair. So the door asks again, as it does an
/// `OnPair` ([`reask_pair`]): `Some` is the intrinsic description the
/// rule now demands — the pair's `Intersection` at a corner, its
/// `TangentIntersection` at a determinate tangency, each derived, so a
/// declaration does not ride either — and `None` keeps the image, where
/// the rule still reads the join under-determined or the edge falls
/// outside it (one surface on both sides, or a spline face beside it).
/// The caller leaves a wrap image alone: it bounds one face.
///
/// # Errors
///
/// [`PairRefusal`] on an in-band or mixed verdict.
pub fn reask_chart<T: Decide>(
    surfaces: [(crate::SurfaceKey, &Surface<T>); 2],
    carrier: &geom::Curve3<T>,
    interval: (T, T),
    reach: T,
    band: Band,
) -> Result<Option<crate::EdgeDescriptionSpec<T>>, PairRefusal> {
    let [(ka, a), (kb, b)] = surfaces;
    if ka == kb || a.spline_chart().is_some() || b.spline_chart().is_some() {
        return Ok(None);
    }
    Ok(
        match reask_pair(surfaces, carrier, interval, reach, None, band)? {
            crate::EdgeDescriptionSpec::OnPair { .. } => None,
            intrinsic => Some(intrinsic),
        },
    )
}

/// Why a join entered as definitely smooth gets no description:
/// [`must_carry_description`] routes it to its caller's typed refusals.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MustCarryRefusal {
    /// [`MustCarryVerdict::InBand`]: certifiable as neither, refused
    /// typed with the deciding station's escalation (D4 ¶3) — never
    /// folded into either description.
    InBand(MustCarryEscalation),
    /// [`MustCarryVerdict::Transverse`]: every station read the join a
    /// corner, refuting the smooth premise the caller entered on.
    Refuted,
    /// [`MustCarryVerdict::Mixed`]: the stations disagree, bracketing an
    /// unsampled in-band point (D2), refused typed with the two.
    Mixed(MixedStations),
}

impl MustCarryVerdict {
    /// The one mapping from the verdict to what the join stores:
    /// jet-determinate ⇒ the intrinsic `TangentIntersection { pair,
    /// witness }`, under-determined ⇒ conventional, in-band, transverse
    /// and mixed ⇒ a typed refusal. `pair` names the surfaces
    /// [`must_carry_over_edge`] was asked about.
    ///
    /// # Errors
    ///
    /// [`MustCarryRefusal`] on an in-band, transverse or mixed verdict.
    pub fn description<T: Real>(
        self,
        pair: crate::SurfacePair,
        witness: Point3<T>,
    ) -> Result<MustCarryDescription<T>, MustCarryRefusal> {
        match self {
            Self::JetDeterminate => Ok(MustCarryDescription::Intrinsic(
                crate::EdgeDescriptionSpec::TangentIntersection { pair, witness },
            )),
            Self::UnderDetermined => Ok(MustCarryDescription::Conventional),
            Self::InBand(source) => Err(MustCarryRefusal::InBand(source)),
            Self::Transverse => Err(MustCarryRefusal::Refuted),
            Self::Mixed(stations) => Err(MustCarryRefusal::Mixed(stations)),
        }
    }
}

/// [`must_carry_over_edge`]'s verdict.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MustCarryVerdict {
    /// Every interior station of the certification schedule read a
    /// definitely-positive second-order separation and the pair is
    /// inside [`crate::tangent_certificate_lane`]: the intrinsic
    /// description is demanded.
    JetDeterminate,
    /// The join is smooth first-order at every station read, and no
    /// intrinsic tangency is demanded, for one of two reasons.
    ///
    /// - A station's second-order separation was definitely
    ///   `Zero`/`Negative` (a G2 join, a same-surface split, coplanar
    ///   planes): the conventional description is the honest one, by
    ///   this predicate.
    /// - The pair is outside [`crate::tangent_certificate_lane`]: every
    ///   interior station read `Smooth` first-order and `Positive`
    ///   second-order, and the certificate cannot store an intrinsic
    ///   tangency there. A transverse or in-band station out of lane,
    ///   at either order, answers `Transverse` or `InBand`, exactly as
    ///   in lane.
    UnderDetermined,
    /// A station was in-band: near-osculating geometry, certifiable as
    /// neither, carrying that station's escalation for the caller to
    /// refuse typed, tagged with the reading that raised it.
    InBand(MustCarryEscalation),
    /// Every station's tangent planes are definitely distinct
    /// ([`DihedralClass::Transverse`]): the join is not first-order
    /// smooth, so the second-order question was never posed and the
    /// rule has no description to choose. A caller whose premise was a
    /// smooth join has had that premise refuted:
    /// [`MustCarryVerdict::description`] refuses it
    /// ([`MustCarryRefusal::Refuted`]).
    Transverse,
    /// Some stations read definitely transverse and others definitely
    /// smooth, none in band: the two bracketing stations
    /// ([`MixedStations`]) enclose an in-band point the schedule did not
    /// sample, so the edge is certifiable as neither (D2) and every
    /// caller refuses it typed, tier 3 included.
    Mixed(MixedStations),
}

/// Which of a station's two readings escalated a
/// [`MustCarryVerdict::InBand`], so a caller routes each to the decision
/// it asks rather than telling them apart by predicate name.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MustCarryEscalation {
    /// The first-order dihedral ([`classify_dihedral`],
    /// `"dihedral_arm"` or `"dihedral_wedge"`), with the rung that
    /// raised it.
    FirstOrder(LeverEscalation),
    /// The second-order sagitta ([`tangent_second_order`],
    /// `"tangent_second_order"`).
    SecondOrder(Indeterminate),
}

impl MustCarryEscalation {
    /// The deciding station's escalation, whichever reading raised it.
    #[must_use]
    pub const fn diag(self) -> Indeterminate {
        match self {
            Self::FirstOrder(escalation) => escalation.diag(),
            Self::SecondOrder(diag) => diag,
        }
    }
}

/// The reading of the two faces at an edge that could not decide: the
/// first-order dihedral by the rung that raised it, or the second-order
/// bend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralReading {
    /// The first-order dihedral: the angle between the faces, metered
    /// over the edge, at the rung that could not decide.
    Lever(crate::LeverRung),
    /// The second-order bend of two faces that meet smoothly.
    Bend,
}

impl DihedralReading {
    /// A first-order dihedral escalation ([`classify_dihedral`])
    /// as the rung that raised it, with its diagnostics.
    #[must_use]
    pub const fn of_lever(escalation: crate::LeverEscalation) -> (Self, Indeterminate) {
        (Self::Lever(escalation.rung()), escalation.diag())
    }

    /// A must-carry station's escalation
    /// ([`crate::MustCarryEscalation`]) as the reading that raised
    /// it — the first-order dihedral by rung, or the second-order bend —
    /// with its diagnostics.
    #[must_use]
    pub const fn of_must_carry(escalation: crate::MustCarryEscalation) -> (Self, Indeterminate) {
        match escalation {
            crate::MustCarryEscalation::FirstOrder(lever) => Self::of_lever(lever),
            crate::MustCarryEscalation::SecondOrder(diag) => (Self::Bend, diag),
        }
    }
}

/// **The material wedge** an edge's two faces subtend at a sample —
/// D1's ratified tier-3 verdict table, in one enum.
///
/// The first-order classifier ([`DihedralClass`]) compares tangent
/// *planes* and is therefore unsigned: wedge 0, π and 2π all read
/// `Smooth`. The material verdict is that classification signed by the
/// two faces' outward normals (each face's chart normal selected by
/// its `Face::sense` bit), so
/// it distinguishes the three:
///
/// | verdict | wedge | legality |
/// |---|---|---|
/// | [`Transverse`](Self::Transverse) | ∈ (0, 2π) at the θ = ε/r margin | legal |
/// | [`Seam`](Self::Seam) | π | legal |
/// | [`Cusp`](Self::Cusp) | 0 | legal iff jet-determinate |
/// | [`Slit`](Self::Slit) | 2π | legal iff jet-determinate |
///
/// The two ends are one verdict under `revert`: reverting a body
/// negates every face's outward normal, which negates the material
/// κ_rel and maps `Cusp` ↔ `Slit` — so the pair is legal together or
/// not at all, exactly as the ruling states.
///
/// In-band κ_rel escalates and a collapsed κ_rel (osculation —
/// conformal contact, the lamina) is neither: it fails the
/// jet-determinacy condition, and no contact declaration cures it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialWedge {
    /// Wedge ∈ (0, 2π), bounded away from both ends: a genuine corner.
    Transverse,
    /// Wedge = π: the smooth seam — one material side, two faces.
    ///
    /// **Not `topo::ContactMark::Seam`**, which is a statement about an
    /// edge's DESCRIPTION (a chart seam, exempt by kind) rather than a
    /// material verdict. The two names sit four lines apart in tier
    /// 3's check 4; each is load-bearing in its own vocabulary, so the
    /// collision is named rather than renamed.
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
/// `sense_plus`/`sense_minus` are the faces' `Face::sense` BITS, not
/// signs to multiply by: each selects whether the face's own implicit
/// gradient already points out of the material or must be negated, so
/// the door mints each outward normal ITSELF — through
/// [`implicit_outward_normal`], the one home of that fold on a curved
/// carrier — and `n̂₊ · n̂₋` is the sign the wedge turns on: aligned ⇒ π,
/// opposed ⇒ 0 or 2π. The parameter is the bit rather than an
/// [`crate::enters::OutwardNormal`] because the normals are this door's own to
/// compute, and rather than a `T` ±1 for the reason
/// [`crate::enters::OutwardNormal::from_chart`]'s doc gives.
/// This is the C1 lemma the declared-contact
/// verifier already decides between bodies (`contact_tangent_opposed`),
/// read edge-locally between two faces of ONE body — the same
/// construction, the dot of unit normals as a displacement in meters,
/// levered here by the folded arm ([`folded_lever_arm`]) and there by
/// the faces' extent.
///
/// # Errors
///
/// [`Indeterminate`]: predicate `"material_wedge_side"` — the margin
/// landed in the band or was poisoned, or classified `Zero`: normals
/// that name no side. That rejection carries the decided margin, as
/// every gate's does. On a definitely-smooth sample the margin is the
/// folded arm, so either refusal is an arm too short to read a side
/// over, ended by [`MATERIAL_PAIRING`]. Where the tolerance it would
/// offer leaves the smooth verdict's wedge in band, the refusal quotes
/// the wedge instead, under `"material_pairing_wedge"`
/// (`pairing_at_wedge`).
pub fn classify_material_pairing<T: Decide>(
    s_plus: &Surface<T>,
    sense_plus: bool,
    s_minus: &Surface<T>,
    sense_minus: bool,
    p: Point3<T>,
    arm: T,
    band: Band,
) -> Result<MaterialPairing, Indeterminate> {
    let (pairing, cos_theta, sin_theta) = pairing_reading(
        "material_wedge_side",
        s_plus,
        sense_plus,
        s_minus,
        sense_minus,
        p,
        arm,
        band,
    );
    pairing.map_err(|refusal| pairing_at_wedge(refusal, cos_theta, sin_theta, arm, band))
}

/// The pairing's construction and margin, decided as `name`, for a
/// caller asking it about a pair of planes it reached by no dihedral (an
/// on-plane face against a candidate plane), which keeps its own
/// population in the K report instead of joining `material_wedge_side`'s.
/// No wedge was read there, so a refusal keeps the pairing's own margin.
///
/// # Errors
///
/// [`Indeterminate`] under `name`: in band, poisoned, or decided zero.
#[allow(clippy::too_many_arguments)] // `classify_material_pairing`'s signature plus the name
pub fn classify_material_pairing_as<T: Decide>(
    name: &'static str,
    s_plus: &Surface<T>,
    sense_plus: bool,
    s_minus: &Surface<T>,
    sense_minus: bool,
    p: Point3<T>,
    arm: T,
    band: Band,
) -> Result<MaterialPairing, Indeterminate> {
    pairing_reading(name, s_plus, sense_plus, s_minus, sense_minus, p, arm, band).0
}

/// The pairing decided as `name`, with the `cos θ` it decided on and the
/// `sin θ` of the same normals.
#[allow(clippy::too_many_arguments)]
fn pairing_reading<T: Decide>(
    name: &'static str,
    s_plus: &Surface<T>,
    sense_plus: bool,
    s_minus: &Surface<T>,
    sense_minus: bool,
    p: Point3<T>,
    arm: T,
    band: Band,
) -> (Result<MaterialPairing, Indeterminate>, T, T) {
    let n_plus = implicit_outward_normal(s_plus, sense_plus, p).vec();
    let n_minus = implicit_outward_normal(s_minus, sense_minus, p).vec();
    let cos_theta = n_plus.dot(n_minus);
    let pairing = match decide_nonzero(name, Margin::levered(cos_theta, arm), band) {
        Ok(NonzeroSign::Positive) => Ok(MaterialPairing::Aligned),
        Ok(NonzeroSign::Negative) => Ok(MaterialPairing::Opposed),
        Err(refusal) => Err(refusal),
    };
    (pairing, cos_theta, n_plus.cross(n_minus).norm())
}

/// **The pairing's refusal, quoted at the wedge where its tolerance would
/// not decide the wedge.** At a tolerance below `|cos θ|·arm/K` the
/// pairing decides, but the same smaller tolerance re-reads the wedge
/// `sin θ · arm` the smooth verdict was read on, and where that lands in
/// band the edge refuses on its dihedral instead. So where the wedge does
/// not read zero at the pairing's offered tolerance
/// ([`wedge_reads_zero_at_the_offer`]), the refusal carries the wedge's
/// own margin through its funnel (`"material_pairing_wedge"`), and the
/// tolerance it offers decides the wedge positive: the edge reads as a
/// crease, and the pairing is not asked. A refusal that offers no
/// tolerance keeps its margin, as does a wedge decided positive, whose
/// crease every smaller tolerance keeps.
fn pairing_at_wedge<T: Decide>(
    refusal: Indeterminate,
    cos_theta: T,
    sin_theta: T,
    arm: T,
    band: Band,
) -> Indeterminate {
    if !refusal.offers_tolerance() || wedge_reads_zero_at_the_offer(sin_theta, cos_theta, band) {
        return refusal;
    }
    match decide_positive("material_pairing_wedge", Margin::of(sin_theta * arm), band) {
        Err(wedge) => wedge,
        Ok(()) => refusal,
    }
}

/// **Whether the wedge reads zero at the pairing's offered tolerance**,
/// `|cos θ|·arm/K`: in the band that escalates at the pairing's margin
/// the wedge reads as `tan θ · escalate` does in `band`, so the reading
/// is `|tan θ|` levered over `band`'s own escalation edge, decided in
/// `band` (`"material_pairing_offer_wedge"`).
fn wedge_reads_zero_at_the_offer<T: Decide>(sin_theta: T, cos_theta: T, band: Band) -> bool {
    matches!(
        decide_reported(
            "material_pairing_offer_wedge",
            Margin::levered(sin_theta / cos_theta.abs(), T::from_f64(band.escalate())),
            band,
        ),
        Ok(Decided {
            sign: Sign::Zero,
            ..
        })
    )
}

/// **The cusp/slit discriminant**: the jet's relative transverse
/// curvature ([`crate::TangentJet::kappa_rel`]) signed into the PLUS
/// face's OUTWARD frame.
///
/// `kappa_rel` is `κ₊ − κ₋`, both measured against the plus surface's
/// implicit gradient `n̂`. Two sign steps carry it into the material
/// frame, and both are exact negations, so the magnitude — and with
/// it the jet-determinacy verdict — is untouched:
///
/// 1. **The implicit convention.** Expanding `F = 0` in the frame
///    `(d̂, n̂)` gives `|∇F|·t + ½u²·d̂ᵀ∇²F d̂ = 0`, so a surface's HEIGHT
///    coefficient over its tangent plane along `+n̂` is `−κ`: a
///    positive jet curvature is a surface bending away from `n̂`.
/// 2. **The material side.** The plus face's outward normal is `n̂`
///    where the face's `Face::sense` bit is set and `−n̂` where it is
///    not, so measuring the heights along it flips them again on a
///    reversed face. `sense_plus` is that BIT, not a sign to multiply
///    by — [`crate::enters::OutwardNormal::from_chart`]'s doc for why. Selecting the
///    negation is EXACT where multiplying by a `±1` is not: `-x` flips
///    the sign of every value including a signed zero and a `NaN`, and
///    at `Interval` it is the exact reflection where a `[-1, -1]`
///    product pads by an ulp at tiny magnitudes. The magnitude, and
///    with it the jet-determinacy verdict, is untouched either way.
///
/// The result is `h₊ − h₋`, the two surfaces' height coefficients over
/// their shared tangent plane along the plus face's OUTWARD normal.
/// Positive ⇒ the plus sheet sits above the minus sheet on the side
/// its own material is NOT, so the material is the vanishing crescent
/// between them ([`MaterialWedge::Cusp`]); negative ⇒ that crescent is
/// the void and the material is everything else
/// ([`MaterialWedge::Slit`]).
pub fn material_kappa_rel<T: Real>(kappa_rel: T, sense_plus: bool) -> T {
    if sense_plus { -kappa_rel } else { kappa_rel }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::{Point3, Vec3};

    use super::*;
    use crate::LeverRung;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn eps() -> f64 {
        Tol::witness().get().eps
    }

    /// An in-band extent escalates the arm gate, named as the arm,
    /// before any angle is read (the reading's row is
    /// `near_tangent_planes_escalate`).
    #[test]
    fn an_in_band_arm_escalates_as_the_arm() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let floor = plane(Vec3::unit_z(), Vec3::unit_x());
        let wall = plane(Vec3::unit_x(), Vec3::unit_y());
        let err = classify_dihedral(&floor, &wall, Point3::origin(), mid, b).unwrap_err();
        assert_eq!(
            (err.rung(), err.diag().predicate),
            (LeverRung::Arm, Some("dihedral_arm_wedge"))
        );
    }

    /// **The arm's own verdict is the gate's, not the wedge it quotes**
    /// (`LeverEscalation::collapsed_arm`): an in-band arm whose wedge
    /// reads zero escalates quoting the wedge's decided zero, a tagged
    /// rejection, yet the arm itself was not decided, so it is not
    /// collapsed; an arm decided zero (a sub-tolerance extent, a cone's
    /// apex) is, whichever margin it quotes.
    #[test]
    fn the_arm_is_collapsed_only_where_its_own_gate_decided_it() {
        let b = band();
        let floor = plane(Vec3::unit_z(), Vec3::unit_x());
        let arm = (b.zero() + b.escalate()) / 2.0;
        // A wedge that reads zero over the in-band arm, but not at the
        // arm's own tolerance (`wedge_reads_zero_at_the_arm`).
        let sin = (b.zero() / b.escalate() + b.zero() / arm) / 2.0;
        let leaning = plane(
            Vec3::new(0.0, sin, (1.0 - sin * sin).sqrt()),
            Vec3::unit_x(),
        );
        let undecided = classify_dihedral(&floor, &leaning, Point3::origin(), arm, b).unwrap_err();
        assert_eq!(undecided.rung(), LeverRung::Arm);
        assert_eq!(
            undecided.diag().margin.rejected_sign(),
            Some(Sign::Zero),
            "the quoted wedge is a decided zero: {undecided:?}"
        );
        assert_eq!(undecided.collapsed_arm(), None, "{undecided:?}");
        let wall = plane(Vec3::unit_x(), Vec3::unit_y());
        let collapsed =
            classify_dihedral(&floor, &wall, Point3::origin(), 0.5 * b.zero(), b).unwrap_err();
        assert!(
            matches!(
                collapsed.collapsed_arm(),
                Some(crate::recourse::Refused::Zero(_))
            ),
            "{collapsed:?}"
        );
    }

    /// **A gate's rejection ends by its reading on either arm**
    /// (`SizedDecision::recourse`): a reader that routes
    /// the arm gate's rejection as `RefusedArm::Undecided` ends it as the
    /// Zero arm does, at a build, at rest and at the import door. A
    /// reading that shows no size — a read or proved zero, or an untagged
    /// in-band margin on the refused side — takes the note at a build and
    /// at rest; a positive rounding whose tolerance lies below `EPS_MIN`
    /// takes the floor's sentence instead, at the import door too; a
    /// rejected zero a smaller tolerance decides passing is offered that
    /// tolerance; a rejected Negative, wholly on the refused side, takes
    /// the note as an in-band one does, at every door, on the
    /// sign-certain arm alike, since the lever reaches the refused side;
    /// and at the import door no reading within ε_in takes the note.
    #[test]
    fn a_gates_rejection_ends_by_its_reading_on_either_arm() {
        use geom_core::sym::with_session;
        use geom_core::{FileCoincidence, ParamSymbol, Sym, SymBudget};

        use crate::recourse::{ReadAt, Reading, Refused, RefusedArm};

        let b = Band::new(1e-9, 1e-8).unwrap();
        let reject = |margin| decide_positive("dihedral_arm", margin, b).unwrap_err();
        let read = reject(Margin::of(0.0));
        // `0.1·3/3 − 0.1` rounds to a positive reading in the zero band,
        // 1.39e-17 m, whose `|m|/K` lies below `EPS_MIN`.
        let rounded = reject(Margin::of(0.1_f64 * 3.0 / 3.0 - 0.1));
        let small = reject(Margin::of(5e-10));
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let (proved, _) = with_session(budget, || {
            let x = Sym::param(ParamSymbol::new(0xa7), 0.1_f64);
            let three = Sym::from_f64(3.0);
            decide_positive("dihedral_arm", Margin::of(x * three / three - x), b).unwrap_err()
        });
        assert_eq!(
            (proved.margin.rejected_sign(), proved.margin.proven()),
            (Some(Sign::Zero), true),
            "the proved zero is a gate's rejection: {proved:?}"
        );
        assert_eq!(
            proved.margin.diagnostic_f64_for_error_text(),
            rounded.margin.diagnostic_f64_for_error_text(),
            "the proof is the only difference from the rounded reading"
        );
        let in_band = Indeterminate {
            margin: MarginDiag::value(-5e-9),
            band: b,
            predicate: Some("dihedral_arm"),
            terminal_sliver: false,
        };
        let negative = reject(Margin::of(-1e-3));
        assert_eq!(negative.margin.rejected_sign(), Some(Sign::Negative));

        let lever = "Recourse: move the geometry so that edge's faces clearly run farther and none \
                     curves tightly there";
        let noted =
            format!("{lever}; faces of no extent or a cone's point leave no angle to measure");
        let untold = "This width or the gap its faces open is within the file's declared \
                        coincidence distance ε_in = 1e-8 m, so the file cannot tell it from \
                        zero. ";
        let offer = format!(
            "{lever}, or, if this width or the gap its faces open is intended, tighten the \
             tolerance below 5e-11 m"
        );
        let floor = format!(
            "{lever}; {}",
            geom_core::floor_note("width or the gap its faces open")
        );
        let file = FileCoincidence::new(1e-8);
        let rows: [(&str, &Indeterminate, [String; 3]); 6] = [
            (
                "a read zero",
                &read,
                [noted.clone(), noted.clone(), format!("{untold}{lever}")],
            ),
            (
                "a proved zero",
                &proved,
                [noted.clone(), noted.clone(), format!("{untold}{lever}")],
            ),
            (
                "a rounded zero",
                &rounded,
                [floor.clone(), floor.clone(), format!("{untold}{floor}")],
            ),
            (
                "a small size",
                &small,
                [
                    offer.clone(),
                    offer,
                    format!(
                        "{untold}{lever}, or, if this width or the gap its faces open is \
                         intended, re-export the file with its uncertainty declared below \
                         5e-10 m and tighten the tolerance below 5e-11 m"
                    ),
                ],
            ),
            (
                "an untagged in-band margin on the refused side",
                &in_band,
                [noted.clone(), noted.clone(), format!("{untold}{lever}")],
            ),
            (
                "a rejected Negative",
                &negative,
                [noted.clone(), noted.clone(), noted.clone()],
            ),
        ];
        for (what, cause, want) in rows {
            let doors = [
                ReadAt::Run(Reading::Build),
                ReadAt::Run(Reading::AtRest),
                ReadAt::File(file),
            ];
            for (at, want) in doors.into_iter().zip(want) {
                let got = DIHEDRAL_ARM.recourse(RefusedArm::Undecided(cause), at);
                assert_eq!(got, want, "{what}, undecided arm, {at:?}");
                if let Some(zero @ Refused::Zero(_)) = Refused::rejected(cause) {
                    assert_eq!(
                        DIHEDRAL_ARM.recourse(zero.arm(), at),
                        got,
                        "{what}: the Zero arm and the undecided arm agree at {at:?}"
                    );
                }
            }
        }
        // The same negative wedge decided on the sign-certain arm ends as
        // the undecided route does at every door.
        let definite = Refused::Negative {
            margin: negative.margin,
        };
        for at in [
            ReadAt::Run(Reading::Build),
            ReadAt::Run(Reading::AtRest),
            ReadAt::File(file),
        ] {
            assert_eq!(
                DIHEDRAL_ARM.recourse(definite.arm(), at),
                DIHEDRAL_ARM.recourse(RefusedArm::Undecided(&negative), at),
                "a definite Negative: the sign-certain arm and the undecided arm agree at {at:?}"
            );
        }
    }

    /// **The arm's escalation quotes the wedge it meters** (`at_wedge`):
    /// two planes at an angle θ over an in-band extent escalate the arm
    /// with the margin `sin θ · extent`, the value whose tolerance decides
    /// the arm and the wedge both (the coincfr4 review's MINOR-1: at one
    /// radian the arm's own offer met the wedge in band); in the zero
    /// band alike; and two coincident planes, whose wedge is exactly zero
    /// at every tolerance that decides the arm, keep the arm's own.
    #[test]
    fn an_arm_escalation_quotes_the_wedge_it_meters() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        let floor = plane(Vec3::unit_z(), Vec3::unit_x());
        let theta = 1.0_f64;
        let leaning = plane(Vec3::new(0.0, theta.sin(), theta.cos()), Vec3::unit_x());
        for extent in [mid, 0.5 * b.zero()] {
            let err = classify_dihedral(&floor, &leaning, Point3::origin(), extent, b).unwrap_err();
            let geom_core::ErrorTextReading::Value(m) =
                err.diag().margin.diagnostic_f64_for_error_text()
            else {
                panic!("a point margin: {err:?}");
            };
            assert_eq!(err.rung(), LeverRung::Arm);
            if b.offer_deciding(extent).is_none() {
                // The arm offers no tolerance (D4 ¶4), so there is none
                // to decide the wedge by: the arm keeps its own margin.
                assert_eq!(err.diag().predicate, Some("dihedral_arm"), "{extent:e}");
                assert_eq!(m, extent, "{extent:e}");
                continue;
            }
            assert_eq!(err.diag().predicate, Some("dihedral_arm_wedge"));
            assert!((m - theta.sin() * extent).abs() <= 1e-6 * m, "{m:e}");
        }
        let err = classify_dihedral(&floor, &floor, Point3::origin(), mid, b).unwrap_err();
        assert_eq!(
            (err.rung(), err.diag().predicate),
            (LeverRung::Arm, Some("dihedral_arm"))
        );
        assert_eq!(
            err.diag().margin.diagnostic_f64_for_error_text(),
            geom_core::ErrorTextReading::Value(mid)
        );
    }

    /// The band `b`'s `K` puts at the tolerance `0.9 ×` what `margin`
    /// offers: the first rung below the offer, as the offer rows run it.
    fn just_below_the_offer(b: Band, margin: f64) -> Band {
        let eps = 0.9 * b.tolerance_deciding(margin);
        Band::new(eps, eps * (b.escalate() / b.zero())).unwrap()
    }

    /// **A wedge that reads zero at the arm's own tolerance leaves the arm
    /// binding** (`wedge_reads_zero_at_the_arm`; the coincv5 review's
    /// NF-1, whose poses these are): over an in-band arm, a tangent
    /// cylinder and plane turned generically (`sin θ` is rounding, about
    /// `1.5e-16`), and plane pairs at 3e-3, 1e-7 and 1e-9 rad, quote the
    /// arm, and a tolerance just below its offer classifies them smooth.
    /// Quoting the wedge offered `1.05e-25 m` on the tangent pair at
    /// `ε = 1e-9`. Past `sin θ = 1/K` the wedge is quoted, and just below
    /// its offer it classifies transverse.
    #[test]
    fn a_wedge_in_the_zero_band_at_the_arms_tolerance_leaves_the_arm_binding() {
        let b = band();
        let extent = 7.0 * eps();
        let quoted = |s1: &Surface<f64>, s2: &Surface<f64>, p: Point3<f64>| {
            let err = classify_dihedral(s1, s2, p, extent, b).unwrap_err();
            let geom_core::ErrorTextReading::Value(m) =
                err.diag().margin.diagnostic_f64_for_error_text()
            else {
                panic!("a point margin: {err:?}");
            };
            assert_eq!(err.rung(), LeverRung::Arm, "{err:?}");
            (err.diag().predicate, m)
        };
        // The coincv5 review's tangent pose (`seam_tangent_noise`).
        let k = Vec3::new(0.37, -0.81, 0.45).normalize();
        let a = 0.913_f64;
        let r =
            |v: Vec3<f64>| v * a.cos() + k.cross(v) * a.sin() + k * (k.dot(v) * (1.0 - a.cos()));
        let c0 = Point3::new(0.4, -1.3, 2.2);
        let cyl = Surface::Cylinder {
            origin: c0 + r(Vec3::new(0.0, 1.0, 0.0)),
            axis: r(Vec3::unit_z()),
            radius: 1.0,
            u_ref: r(Vec3::unit_x()),
        };
        let pl = Surface::Plane {
            origin: c0,
            normal: r(Vec3::new(0.0, -1.0, 0.0)),
            u_ref: r(Vec3::unit_x()),
        };
        let p = c0 + r(Vec3::new(0.0, 0.0, 0.3));
        assert_eq!(quoted(&cyl, &pl, p), (Some("dihedral_arm"), extent));
        assert_eq!(
            classify_dihedral(&cyl, &pl, p, extent, just_below_the_offer(b, extent)).unwrap(),
            DihedralClass::Smooth
        );
        let floor = plane(Vec3::unit_z(), Vec3::unit_x());
        let at = |theta: f64| plane(Vec3::new(0.0, theta.sin(), theta.cos()), Vec3::unit_x());
        for theta in [3e-3, 1e-7, 1e-9] {
            let o = Point3::origin();
            assert_eq!(
                quoted(&floor, &at(theta), o),
                (Some("dihedral_arm"), extent),
                "{theta:e}"
            );
            let below = just_below_the_offer(b, extent);
            assert_eq!(
                classify_dihedral(&floor, &at(theta), o, extent, below).unwrap(),
                DihedralClass::Smooth,
                "{theta:e}"
            );
        }
        // Past 1/K, the wedge binds: just below the arm's offer it would
        // still be in band.
        let theta = 0.15_f64;
        let (predicate, m) = quoted(&floor, &at(theta), Point3::origin());
        assert_eq!(predicate, Some("dihedral_arm_wedge"));
        assert!((m - theta.sin() * extent).abs() <= 1e-6 * m, "{m:e}");
        let o = Point3::origin();
        assert!(
            classify_dihedral(
                &floor,
                &at(theta),
                o,
                extent,
                just_below_the_offer(b, extent)
            )
            .is_err()
        );
        assert_eq!(
            classify_dihedral(&floor, &at(theta), o, extent, just_below_the_offer(b, m)).unwrap(),
            DihedralClass::Transverse
        );
    }

    /// The one verdict-to-description mapping every smooth-join caller
    /// stores by: each verdict lands on its own answer, and the
    /// intrinsic one names the pair the rule was asked about.
    #[test]
    fn must_carry_description_maps_each_verdict() {
        let mut keys = slotmap::SlotMap::<crate::SurfaceKey, ()>::with_key();
        let (a, b) = (keys.insert(()), keys.insert(()));
        let pair = crate::SurfacePair::new(a, b);
        let witness = Point3::new(1.0, 2.0, 3.0);
        match MustCarryVerdict::JetDeterminate.description(pair, witness) {
            Ok(MustCarryDescription::Intrinsic(
                crate::EdgeDescriptionSpec::TangentIntersection {
                    pair: got,
                    witness: w,
                },
            )) => assert!(got == pair && w.distance(witness) == 0.0, "{got:?} {w:?}"),
            other => panic!("jet-determinate must be the intrinsic tangency: {other:?}"),
        }
        assert!(matches!(
            MustCarryVerdict::UnderDetermined.description::<f64>(pair, witness),
            Ok(MustCarryDescription::Conventional)
        ));
        let source = MustCarryEscalation::SecondOrder(Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: band(),
            predicate: Some("tangent_second_order"),
            terminal_sliver: false,
        });
        assert!(matches!(
            MustCarryVerdict::InBand(source).description::<f64>(pair, witness),
            Err(MustCarryRefusal::InBand(s)) if s == source
        ));
        assert!(matches!(
            MustCarryVerdict::Transverse.description::<f64>(pair, witness),
            Err(MustCarryRefusal::Refuted)
        ));
        let stations = MixedStations {
            transverse: 3,
            smooth: 4,
        };
        assert!(matches!(
            MustCarryVerdict::Mixed(stations).description::<f64>(pair, witness),
            Err(MustCarryRefusal::Mixed(s)) if s == stations
        ));
    }

    /// **The corner verdict over a station row**, as an enumeration: no
    /// corner reads no verdict, every corner reads `Transverse`, and a
    /// row that changes names the first neighbouring pair that
    /// disagrees, as schedule samples (interior station `k` is sample
    /// `k + 1`), each in its role whichever way the row turns.
    #[test]
    fn mixed_stations_name_the_first_disagreeing_neighbours() {
        let (t, s) = (true, false);
        let mixed = |transverse, smooth| {
            Some(MustCarryVerdict::Mixed(MixedStations {
                transverse,
                smooth,
            }))
        };
        let rows: [(&[bool], Option<MustCarryVerdict>); 6] = [
            (&[s, s, s, s], None),
            (&[t, t, t, t], Some(MustCarryVerdict::Transverse)),
            (&[t, s, s, s], mixed(1, 2)),
            (&[s, s, s, t], mixed(4, 3)),
            (&[s, t, s, t], mixed(2, 1)),
            (&[t, t, s, t], mixed(2, 3)),
        ];
        for (row, want) in rows {
            assert_eq!(MixedStations::corner_verdict(row), want, "{row:?}");
        }
        let text = MixedStations {
            transverse: 2,
            smooth: 3,
        }
        .to_string();
        assert!(
            text.contains("sample 2")
                && text.contains("sample 3")
                && text.ends_with("too near to classify"),
            "a mixed reading names both stations, and leaves the recourse to its decision: {text}"
        );
    }

    /// **The shared chain answers each verdict with the caller's own
    /// pieces, and the same answer in either surface order**: an
    /// under-determined join is conventional, naming no chart; a
    /// jet-determinate one is the
    /// tangency over the pair in key order; and each refusal comes from
    /// the constructor for its own reading — an in-band dihedral from
    /// the first-order one, with the rung that raised it, an in-band
    /// sagitta from the second-order one as the bend, a corner from the
    /// refuted one. Every answer, diagnostics included, is the same
    /// whichever surface the caller lists first, which only the sort
    /// makes so: each order reads the transverse curvature along its
    /// first surface's normal, and on the sphere in its sleeve the two
    /// readings differ in their last bit.
    #[test]
    fn must_carry_description_answers_with_the_callers_constructors() {
        #[derive(Debug, PartialEq)]
        enum Refused {
            FirstOrder(DihedralReading, Indeterminate),
            SecondOrder(DihedralReading, Indeterminate),
            Refuted(Option<MixedStations>),
        }
        let mut keys = slotmap::SlotMap::<crate::SurfaceKey, ()>::with_key();
        let (a, b) = (keys.insert(()), keys.insert(()));
        let ask = |sa: &Surface<f64>, sb: &Surface<f64>, carrier, interval: (f64, f64)| {
            let (t0, t1) = interval;
            let answers = [[(a, sa), (b, sb)], [(b, sb), (a, sa)]].map(|surfaces| {
                let answer = must_carry_description(
                    SmoothJoin {
                        surfaces,
                        carrier,
                        interval,
                        // Faces that reach no farther than the edge.
                        reach: carrier.eval(t0).distance(carrier.eval(t1)),
                        witness: carrier.mid_point(t0, t1),
                    },
                    band(),
                    Refused::FirstOrder,
                    Refused::SecondOrder,
                    Refused::Refuted,
                );
                format!("{answer:?}")
            });
            assert_eq!(
                answers[0], answers[1],
                "the two surface orders answer alike"
            );
            let [answer, _] = answers;
            answer
        };
        let along = |dir| geom::Curve3::Line {
            origin: Point3::origin(),
            dir,
        };
        let floor = plane(Vec3::unit_z(), Vec3::unit_x());
        let (x_axis, y_axis) = (along(Vec3::unit_x()), along(Vec3::unit_y()));

        let coplanar = plane(Vec3::unit_z(), Vec3::unit_y());
        assert_eq!(
            ask(&floor, &coplanar, &x_axis, (0.0, 1.0)),
            format!(
                "{:?}",
                Ok::<_, Refused>(MustCarryDescription::<f64>::Conventional)
            ),
            "coplanar planes are conventional"
        );

        let cylinder = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 2.0),
            axis: Vec3::unit_y(),
            radius: 2.0,
            u_ref: Vec3::unit_x(),
        };
        let pair = crate::SurfacePair::new(a, b);
        assert!(
            ask(&floor, &cylinder, &y_axis, (0.0, 10.0)).starts_with(&format!(
                "Ok(Intrinsic(TangentIntersection {{ pair: {pair:?}"
            )),
            "a plane on a cylinder's generator is the intrinsic tangency over the pair"
        );

        let theta = 3.0 * eps();
        let near = plane(Vec3::new(theta.sin(), 0.0, theta.cos()), Vec3::unit_y());
        assert!(
            ask(&floor, &near, &y_axis, (0.0, 1.0)).starts_with("Err(FirstOrder(Lever(Reading), "),
            "an in-band dihedral is the first-order constructor's, by its rung"
        );

        let (sin, cos) = std::f64::consts::FRAC_PI_4.sin_cos();
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_4,
            u_ref: Vec3::unit_x(),
        };
        let ruling = Vec3::new(sin, 0.0, cos);
        let tangent = plane(Vec3::new(cos, 0.0, -sin), ruling);
        let len = (2.0 * (band().zero() * band().escalate()).sqrt() / 0.01).sqrt();
        let generator = along(ruling);
        let bend = ask(&cone, &tangent, &generator, (100.0, 100.0 + len));
        assert!(
            bend.starts_with("Err(SecondOrder(Bend, ") && bend.contains("tangent_second_order"),
            "an in-band sagitta is the second-order constructor's, as the bend: {bend}"
        );

        // A sphere in a cylinder of its own radius, touching along the
        // equator, on a tilted axis: the two orders read `κ_rel` apart in
        // its last bit at some stations, and this interval's deciding
        // station is one of them.
        let axis = Vec3::new(0.3, 0.7, 0.2).normalize();
        let u_ref = Vec3::new(1.0, -0.2, -0.3);
        let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
        let (center, radius) = (Point3::new(0.37, -1.21, 2.9), 1.7);
        let sleeve = Surface::Cylinder {
            origin: center,
            axis,
            radius,
            u_ref,
        };
        let ball = Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        };
        let equator = geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        let turn = (2.0 * (band().zero() * band().escalate()).sqrt() * radius).sqrt() / radius;
        let bend = ask(&sleeve, &ball, &equator, (0.15, 0.15 + turn));
        assert!(
            bend.starts_with("Err(SecondOrder(Bend, "),
            "the equator's sagitta is in band: {bend}"
        );

        let wall = plane(Vec3::unit_x(), Vec3::unit_y());
        assert_eq!(
            ask(&floor, &wall, &y_axis, (0.0, 1.0)),
            format!(
                "{:?}",
                Err::<MustCarryDescription<f64>, _>(Refused::Refuted(None))
            ),
            "a corner is the refuted constructor's"
        );
    }

    /// **The lane gates the demand, never the in-band refusal.** The
    /// plane tangent to a 45° cone (apex at the origin, axis `z`) along
    /// the ruling `(sin 45°, 0, cos 45°)`, over `[100, 100 + L]`: a
    /// `Line` carrier on a cone, which
    /// [`crate::tangent_certificate_lane`] refuses. `κ_rel` is the
    /// cone's transverse curvature `1/t ≈ 0.01` and the folded arm is
    /// the extent `L`, so `L` sets the sagitta `κ_rel·L²/2`: at the
    /// band's geometric mean every station is in band, and tier 3
    /// refuses that edge `SliverDihedral` lane or not, so the rule
    /// answers `InBand`; at ten times the escalation edge every station
    /// reads `Positive` and the lane leaves nothing to demand, and at a
    /// hundredth of the zero edge the first station reads `Zero`.
    #[test]
    fn an_out_of_lane_in_band_sagitta_escalates_and_a_definite_one_is_under_determined() {
        let b = band();
        let (s, c) = std::f64::consts::FRAC_PI_4.sin_cos();
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_4,
            u_ref: Vec3::unit_x(),
        };
        let ruling = Vec3::new(s, 0.0, c);
        let tangent = plane(Vec3::new(c, 0.0, -s), ruling);
        let carrier = geom::Curve3::Line {
            origin: Point3::origin(),
            dir: ruling,
        };
        assert!(
            !crate::tangent_certificate_lane(&carrier, &cone, &tangent),
            "the witness is out of lane"
        );
        let length_at = |sagitta: f64| (2.0 * sagitta / 0.01).sqrt();
        let verdicts = |len: f64| {
            [(&cone, &tangent), (&tangent, &cone)]
                .map(|(a, z)| must_carry_over_edge(a, z, &carrier, 100.0, 100.0 + len, len, b))
        };
        let mean = (b.zero() * b.escalate()).sqrt();
        for (order, verdict) in verdicts(length_at(mean)).into_iter().enumerate() {
            assert!(
                matches!(
                    verdict,
                    MustCarryVerdict::InBand(MustCarryEscalation::SecondOrder(diag))
                        if diag.predicate == Some("tangent_second_order")
                ),
                "order {order}: an in-band sagitta out of lane escalates, got {verdict:?}"
            );
        }
        for (label, sagitta) in [
            ("positive", 10.0 * b.escalate()),
            ("zero", b.zero() / 100.0),
        ] {
            for (order, verdict) in verdicts(length_at(sagitta)).into_iter().enumerate() {
                assert_eq!(
                    verdict,
                    MustCarryVerdict::UnderDetermined,
                    "order {order}: a definitely {label} sagitta out of lane demands nothing"
                );
            }
        }
    }

    /// A scripted [`StationHook`]: logs the index of every station it
    /// is shown, before and after the decision, and breaks where told.
    #[derive(Default)]
    struct Script {
        break_before: Option<usize>,
        break_after: Option<usize>,
        next: usize,
        before: Vec<usize>,
        after: Vec<usize>,
    }

    impl<T: Real> StationHook<T> for Script {
        type Break = usize;

        fn before_decision(&mut self, _: &Station<T>) -> ControlFlow<usize> {
            let i = self.next;
            self.next += 1;
            self.before.push(i);
            match self.break_before {
                Some(k) if k == i => ControlFlow::Break(i),
                _ => ControlFlow::Continue(()),
            }
        }

        fn after_positive(&mut self, _: &Station<T>) -> ControlFlow<usize> {
            let i = self.next - 1;
            self.after.push(i);
            match self.break_after {
                Some(k) if k == i => ControlFlow::Break(i),
                _ => ControlFlow::Continue(()),
            }
        }
    }

    /// Two cylinders of radii 1 and `r2` kissing along the y axis, and
    /// the unit stretch of that axis: `r2 = 2` reads `Positive` at every
    /// station (sagitta 1/4 over the unit arm), `r2 = 1` osculates.
    fn walk<T: Decide>(r2: f64, script: &mut Script) -> SecondOrderWalk<usize> {
        let f = T::from_f64;
        let cylinder = |radius: f64| Surface::Cylinder {
            origin: Point3::new(f(0.0), f(0.0), f(radius)),
            axis: Vec3::new(f(0.0), f(1.0), f(0.0)),
            radius: f(radius),
            u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
        };
        let axis = geom::Curve3::Line {
            origin: Point3::new(f(0.0), f(0.0), f(0.0)),
            dir: Vec3::new(f(0.0), f(1.0), f(0.0)),
        };
        second_order_walk(
            &cylinder(1.0),
            &cylinder(r2),
            interior_stations(&axis, f(0.0), f(1.0)),
            f(1.0),
            band(),
            script,
        )
    }

    /// **The walk's stop paths.** A break before a station's decision
    /// answers `Stopped` and nothing after it is read; a break after a
    /// `Positive` decision likewise; `after_positive` is never shown a
    /// station that did not read `Positive`.
    #[test]
    fn the_walk_stops_where_its_hook_breaks_and_reads_on_only_past_positive() {
        let interior = usize::try_from(crate::CERT_SAMPLES - 2).expect("a small count");
        let all: Vec<usize> = (0..interior).collect();

        let mut s = Script::default();
        assert!(matches!(
            walk::<f64>(2.0, &mut s),
            SecondOrderWalk::Determinate
        ));
        assert_eq!((&s.before, &s.after), (&all, &all), "an unbroken walk");

        let mut s = Script {
            break_before: Some(3),
            ..Script::default()
        };
        assert_eq!(walk::<f64>(2.0, &mut s), SecondOrderWalk::Stopped(3));
        assert_eq!(
            (s.before, s.after),
            (vec![0, 1, 2, 3], vec![0, 1, 2]),
            "a break before station 3's decision: station 3 is not decided, and none after it is read"
        );

        let mut s = Script {
            break_after: Some(2),
            ..Script::default()
        };
        assert_eq!(walk::<f64>(2.0, &mut s), SecondOrderWalk::Stopped(2));
        assert_eq!(
            (s.before, s.after),
            (vec![0, 1, 2], vec![0, 1, 2]),
            "a break after station 2's decision: none after it is read"
        );

        let mut s = Script::default();
        assert_eq!(walk::<f64>(1.0, &mut s), SecondOrderWalk::UnderDetermined);
        assert_eq!(
            (s.before, s.after),
            (vec![0], vec![]),
            "a zero-side station decides the walk, and is never shown to after_positive"
        );
    }

    /// **The walk's stop paths, metered**: a break before station k's
    /// decision spends k `tangent_second_order` samples, not k + 1; a
    /// break after it spends k + 1.
    #[cfg(feature = "probe")]
    #[test]
    fn a_stopped_walk_meters_only_the_stations_it_decided() {
        use geom_core::k_stats::{self, Probe};
        let metered = |script: &mut Script| {
            k_stats::start_recording();
            let answer = walk::<Probe>(2.0, script);
            let n = k_stats::take_samples()
                .iter()
                .filter(|s| s.predicate == "tangent_second_order")
                .count();
            (answer, n)
        };
        let interior = usize::try_from(crate::CERT_SAMPLES - 2).expect("a small count");
        let (answer, n) = metered(&mut Script::default());
        assert!(
            matches!(answer, SecondOrderWalk::Determinate) && n == interior,
            "an unbroken walk decides every station"
        );
        assert_eq!(
            metered(&mut Script {
                break_before: Some(3),
                ..Script::default()
            }),
            (SecondOrderWalk::Stopped(3), 3),
            "a break before station 3 decides stations 0..3 only"
        );
        assert_eq!(
            metered(&mut Script {
                break_after: Some(2),
                ..Script::default()
            }),
            (SecondOrderWalk::Stopped(2), 3),
            "a break after station 2 decides stations 0..=2 only"
        );
    }

    fn plane(normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal,
            u_ref,
        }
    }

    /// The fold propagates poison from either arm and from the extent:
    /// a NURBS operand folds to poison, never to the plane's arm or the
    /// extent that would otherwise win.
    #[test]
    fn the_folded_arm_is_poison_when_any_operand_is() {
        let p = plane(Vec3::unit_z(), Vec3::unit_x());
        let n = Surface::nurbs_placeholder();
        let o = Point3::origin();
        assert!(folded_lever_arm(&p, &n, o, 1.0).is_nan());
        assert!(folded_lever_arm(&n, &p, o, 1.0).is_nan());
        assert!(folded_lever_arm(&p, &p, o, f64::NAN).is_nan());
        assert_eq!(folded_lever_arm(&p, &p, o, 1.0), 1.0);
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
        assert_eq!(
            (err.rung(), err.diag().predicate),
            (LeverRung::Reading, Some("dihedral_wedge"))
        );
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

    /// At the cone apex the radius of curvature, and with it the folded
    /// lever arm, is zero: the arm gate escalates with that decided zero
    /// before the poisoned gradient is read — never a guess, never a
    /// chart normal.
    #[test]
    fn cone_apex_escalates_at_the_arm_gate() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_6,
            u_ref: Vec3::unit_x(),
        };
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let err = classify_dihedral(&cone, &s1, Point3::origin(), 1.0, band()).unwrap_err();
        assert_eq!(
            (
                err.rung(),
                err.diag().predicate,
                err.diag().margin.diagnostic_f64_for_error_text(),
                err.diag().margin.rejected_sign()
            ),
            (
                crate::LeverRung::Arm,
                Some("dihedral_arm"),
                geom_core::ErrorTextReading::Value(0.0),
                Some(Sign::Zero)
            )
        );
    }

    /// **A cone's apex at the import door ends in the door's sentence and
    /// the lever, without the arm's zero note** — the accepted cost of
    /// one message below ε_in: the door cannot tell an apex the file
    /// states exactly from a rounded zero, so the note that names the
    /// apex rides only away from the door. The apex arm reads exactly
    /// 0.0 on the `f64` lane; both routes a reader takes end alike. Red
    /// if the note returns at the door or leaves the at-rest ending.
    #[test]
    fn a_cone_apex_at_the_import_door_keeps_its_lever_and_loses_its_note() {
        use geom_core::FileCoincidence;

        use crate::recourse::{Reading, RefusedArm};

        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: std::f64::consts::FRAC_PI_6,
            u_ref: Vec3::unit_x(),
        };
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let err = classify_dihedral(&cone, &s1, Point3::origin(), 1.0, band()).unwrap_err();
        let diag = err.diag();
        let geom_core::ErrorTextReading::Value(m) = diag.margin.diagnostic_f64_for_error_text()
        else {
            panic!("the apex arm is a point reading: {err:?}");
        };
        assert_eq!(
            m.to_bits(),
            0.0_f64.to_bits(),
            "the apex arm reads exactly +0.0"
        );
        let collapsed = err.collapsed_arm().expect("the apex's arm is decided zero");
        let note = "a cone's point leave no angle to measure";
        let lever = "Recourse: move the geometry so that edge's faces clearly run farther and none \
                     curves tightly there";
        let file = FileCoincidence::new(1e-6);
        for arm in [RefusedArm::Undecided(&diag), collapsed.arm()] {
            let at_rest = DIHEDRAL_ARM.recourse(arm, Reading::AtRest);
            assert!(
                at_rest.starts_with(lever) && at_rest.ends_with(note),
                "{arm:?} at rest: {at_rest}"
            );
            assert_eq!(
                DIHEDRAL_ARM.recourse(arm, file),
                format!(
                    "This width or the gap its faces open is within the file's declared \
                     coincidence distance ε_in = 1e-6 m, so the file cannot tell it from \
                     zero. {lever}"
                ),
                "{arm:?} at the import door"
            );
        }
    }

    /// Two coplanar faces on one tangent plane: material sides agree
    /// (both senses set on the same normal) ⇒ the legal π seam; flip
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
            classify_material_pairing(&s1, true, &s2, true, p, 1.0, band()).unwrap(),
            MaterialPairing::Aligned
        );
        assert_eq!(
            classify_material_pairing(&s1, true, &s2, false, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        // Antiparallel STORED normals with agreeing senses are the
        // same aligned pairing read through the other encoding: a
        // plane carries its reversal in the datum, a curved chart in
        // the sense bit, and the pairing sees only the product.
        let s3 = plane(-Vec3::unit_z(), Vec3::unit_x());
        assert_eq!(
            classify_material_pairing(&s1, true, &s3, false, p, 1.0, band()).unwrap(),
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
        // inside (outward normal points INTO the inner cylinder, so
        // its sense bit is clear against the outward-pointing implicit
        // gradient), the outer from outside (sense bit set).
        assert_eq!(
            classify_material_pairing(&inner, false, &outer, true, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        let jet = crate::tangent_jet(&inner, &outer, p, Vec3::unit_y());
        // Plus = the inner cylinder, whose outward normal (sense bit
        // clear) points down: the crescent is the material ⇒ cusp.
        assert!(
            material_kappa_rel(jet.kappa_rel, false) > 0.0,
            "kappa_rel {} in the material frame",
            material_kappa_rel(jet.kappa_rel, false)
        );
        // Revert: every outward normal negates, the crescent becomes
        // the void, and the same edge is the slit.
        assert!(material_kappa_rel(jet.kappa_rel, true) < 0.0);
    }

    /// Osculation: one surface against a coincident copy of itself.
    /// The pairing is opposed (a zero-thickness sheet), and the
    /// discriminant collapses exactly — no side to pick, which is the
    /// lamina the material arm refuses.
    #[test]
    fn coincident_surfaces_osculate_with_no_side() {
        let s1 = plane(Vec3::unit_z(), Vec3::unit_x());
        let s2 = plane(Vec3::unit_z(), Vec3::unit_y());
        let p = Point3::origin();
        assert_eq!(
            classify_material_pairing(&s1, true, &s2, false, p, 1.0, band()).unwrap(),
            MaterialPairing::Opposed
        );
        let jet = crate::tangent_jet(&s1, &s2, p, Vec3::unit_y());
        assert_eq!(material_kappa_rel(jet.kappa_rel, true), 0.0);
        // **The bit SELECTS a negation; it does not scale by a `±1`**,
        // and a zero is where the two spellings part: `-(κ · ±1.0)`
        // answers `-0.0` on the clear bit and `+0.0` on the set one,
        // the opposite of both rows below. No verdict reads a zero's
        // sign — this collapse is `Zero` either way — so what these
        // pin is the exactness the door's contract claims, on the one
        // input where an inexact spelling is visible.
        assert_eq!(
            material_kappa_rel(0.0_f64, false).to_bits(),
            0.0_f64.to_bits(),
            "the clear bit returns its argument unchanged, sign bit included"
        );
        assert_eq!(
            material_kappa_rel(0.0_f64, true).to_bits(),
            (-0.0_f64).to_bits(),
            "the set bit negates, and negating +0.0 gives -0.0"
        );
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
        let err = classify_material_pairing(&cone, true, &s1, true, Point3::origin(), 1.0, band())
            .unwrap_err();
        assert_eq!(err.margin, geom_core::MarginDiag::INVALID);
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
        assert_eq!(err.diag().margin, geom_core::MarginDiag::INVALID);
    }

    /// **A cylinder's gradient enclosure that reaches zero leaves no
    /// tangent plane, and the wedge says so.** The point and the
    /// cylinder's axis are enclosed apart, each `±δ` across the radial
    /// direction, as two images of one widened rigid map are: the radial
    /// vector `p − origin` is then enclosed `r ± 2δ` wide, and once
    /// `2δ ≥ r` the enclosures admit a point on the axis, where the
    /// gradient vanishes. `sin θ` divides by the gradients' magnitudes,
    /// whose enclosure then reaches zero, so the margin is undefined
    /// (`Trv`): [`WedgeEscalation::NoTangentPlane`], which
    /// [`classify_dihedral`] still reports as the wedge reading's
    /// invalid margin. Below half the radius the same pair is a definite
    /// corner; a poisoned gradient stays the reading's own invalid margin.
    #[test]
    fn a_cylinder_gradient_reaching_zero_leaves_no_tangent_plane() {
        use geom_core::Interval;
        let i = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let pt = |x: f64, y: Interval| Point3::new(Interval::from_f64(x), y, Interval::zero());
        let unit = |x: f64, y: f64, z: f64| {
            Vec3::new(
                Interval::from_f64(x),
                Interval::from_f64(y),
                Interval::from_f64(z),
            )
        };
        let r = 1.0;
        let cap = Surface::Plane {
            origin: pt(0.0, Interval::zero()),
            normal: unit(0.0, 0.0, -1.0),
            u_ref: unit(1.0, 0.0, 0.0),
        };
        let cyl = |delta: f64| Surface::Cylinder {
            origin: pt(0.0, i(-delta, delta)),
            axis: unit(0.0, 0.0, 1.0),
            radius: Interval::from_f64(r),
            u_ref: unit(1.0, 0.0, 0.0),
        };
        let p = |delta: f64| pt(0.0, i(r - delta, r + delta));
        let extent = Interval::from_f64(2.0 * r);
        let wedge = |delta: f64| wedge_decided(&cap, &cyl(delta), p(delta), extent, band());
        assert_eq!(
            wedge(0.49 * r).unwrap().0,
            DihedralClass::Transverse,
            "2δ = 0.98 r: the radial enclosure excludes zero"
        );
        let Err(WedgeEscalation::NoTangentPlane(diag)) = wedge(0.51 * r) else {
            panic!(
                "2δ = 1.02 r: the radial enclosure reaches zero, so the tangent plane is \
                 undefined: {:?}",
                wedge(0.51 * r)
            );
        };
        assert_eq!(diag.predicate, Some("dihedral_wedge"), "{diag:?}");
        assert_eq!(diag.margin, geom_core::MarginDiag::INVALID, "{diag:?}");
        let public =
            classify_dihedral(&cap, &cyl(0.51 * r), p(0.51 * r), extent, band()).unwrap_err();
        assert_eq!(
            public,
            LeverEscalation::reading(diag),
            "classify_dihedral reports the same escalation it always did"
        );
        let off_locus = wedge_decided(
            &plane(Vec3::unit_z(), Vec3::unit_x()),
            &Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            Point3::new(f64::NAN, 1.0, 0.0),
            1.0,
            band(),
        );
        assert!(
            matches!(off_locus, Err(WedgeEscalation::Lever(e)) if e.rung() == LeverRung::Reading),
            "a poisoned point poisons the gradient, which is not a missing tangent plane: \
             {off_locus:?}"
        );
    }

    /// **`reask_chart` asks only where the rule applies.** Two planes at
    /// a right angle along the `x` axis: as two keys the rule reads a
    /// corner and demands the section; as one key, or with one side a
    /// spline face (the same plane as a bilinear net), the edge falls
    /// outside the rule and the image is kept unasked.
    #[test]
    fn reask_chart_asks_only_where_the_rule_applies() {
        use geom::NurbsSurface;
        use geom_core::spline::KnotVector;
        let band = Band::linear(Tol::witness()).unwrap();
        let mut keys: slotmap::SlotMap<crate::SurfaceKey, ()> = slotmap::SlotMap::with_key();
        let (ka, kb) = (keys.insert(()), keys.insert(()));
        let floor = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let wall = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
        };
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let spline_wall = Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(
                kv.clone(),
                kv,
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, 0.0, 1.0),
                    Point3::new(1.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 1.0),
                ],
                vec![1.0; 4],
            )
            .unwrap(),
        ));
        let edge = geom::Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        let ask = |a: (crate::SurfaceKey, &Surface<f64>), b| {
            reask_chart([a, b], &edge, (0.0, 1.0), 1.0, band).unwrap()
        };
        assert!(
            matches!(
                ask((ka, &floor), (kb, &wall)),
                Some(crate::EdgeDescriptionSpec::Intersection { .. })
            ),
            "two keys at a corner: the section"
        );
        assert!(ask((ka, &floor), (ka, &wall)).is_none(), "one key: kept");
        assert!(
            ask((ka, &floor), (kb, &spline_wall)).is_none(),
            "beside a spline face: kept"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod seam_rows {
    use super::*;
    use crate::NurbsLane;
    use geom::NurbsSurface;
    use geom_core::Tol;
    use geom_core::Vec3;
    use geom_core::spline::KnotVector;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The band's geometric middle: a length no row's band decides.
    fn in_band() -> f64 {
        let b = band();
        (b.zero() * b.escalate()).sqrt()
    }

    fn chart_arm(chart: &NurbsSurface<f64>, p: Point3<f64>, reach: f64) -> Option<f64> {
        NurbsLane::certified().chart_arm(chart, p, reach)
    }

    /// The certified curvature bound over every cell of `chart`: the
    /// whole-chart reading the arm once was, which the far-bend rows choose
    /// their angles against, so that reading would read them smooth.
    fn whole_chart_curvature(chart: &NurbsSurface<f64>) -> f64 {
        let splits = crate::patch_bound::RATIONAL_CERT_SPLITS;
        let cells = crate::patch_bound::patch_cells_refined(chart, splits).unwrap();
        let (lo, hi) = crate::offset_meters::patch_curvature(&cells);
        (-lo).max(hi)
    }

    /// A chart ruled along the `x` axis, `u ∈ [0, 1]` running `x` from
    /// −1 to 1, through the `(y, z)` profile with its weights on `kv`.
    fn ruled(kv: KnotVector, profile: &[(f64, f64)], weights: &[f64]) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [-1.0, 1.0]
            .into_iter()
            .flat_map(|x| profile.iter().map(move |&(y, z)| Point3::new(x, y, z)))
            .collect();
        let w = [weights, weights].concat();
        NurbsSurface::new(ku, kv, control, w).unwrap()
    }

    fn integral(kv: KnotVector, profile: &[(f64, f64)]) -> NurbsSurface<f64> {
        ruled(kv, profile, &vec![1.0; profile.len()])
    }

    fn linear() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
    }

    fn quadratic() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
    }

    /// The floor strip `z = 0`, `y ∈ [0, 1]`, as a spline chart.
    fn floor() -> NurbsSurface<f64> {
        integral(linear(), &[(0.0, 0.0), (1.0, 0.0)])
    }

    /// A spline face at `(½, v)`, on the seam `x = 0` of every chart here.
    fn at(chart: &NurbsSurface<f64>, v: f64) -> SeamFace<'_, f64> {
        SeamFace::Spline {
            chart,
            uv: (0.5, v),
        }
    }

    const ORIGIN: Point3<f64> = Point3::new(0.0, 0.0, 0.0);

    fn seam(a: SeamFace<'_, f64>, b: SeamFace<'_, f64>) -> Result<DihedralClass, SeamEscalation> {
        seam_over(a, b, 1.0)
    }

    fn seam_over(
        a: SeamFace<'_, f64>,
        b: SeamFace<'_, f64>,
        reach: f64,
    ) -> Result<DihedralClass, SeamEscalation> {
        classify_seam([a, b], ORIGIN, reach, chart_arm, band())
    }

    /// A plane through the seam point with normal `n`.
    fn plane(n: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin: ORIGIN,
            normal: n.normalize(),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// A wall leaving the seam along `(c, s)` in the `(y, z)` plane, its
    /// first rows collinear out to `flat` and a tight bend past them: flat
    /// out to about `flat`, bending hard beyond.
    fn bent(flat: f64, c: f64, s: f64) -> NurbsSurface<f64> {
        let rot = |(y, z): (f64, f64)| (c * y - s * z, s * y + c * z);
        let rows = [
            (0.0, 0.0),
            (flat / 3.0, 0.0),
            (2.0 * flat / 3.0, 0.0),
            (flat, 0.0),
            (flat + 0.005, 0.005),
        ];
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        integral(kv, &rows.map(rot))
    }

    fn arm_refused(got: Result<DihedralClass, SeamEscalation>) -> bool {
        matches!(got, Err(SeamEscalation::Lever(e))
            if e.diag().predicate == Some("dihedral_arm") && e.diag().margin.is_invalid())
    }

    /// Two spline walls meeting along the `x` axis at a right angle read
    /// transverse, and a wall leaving the floor tangent to it reads
    /// smooth: each normal is its chart's at the seam, and the arm is the
    /// charts' certified curvature, so both decide.
    #[test]
    fn two_spline_walls_read_transverse_or_smooth() {
        let floor = floor();
        let rising = integral(quadratic(), &[(0.0, 0.0), (0.0, 0.5), (-0.2, 1.0)]);
        assert_eq!(
            seam(at(&rising, 0.0), at(&floor, 0.0)),
            Ok(DihedralClass::Transverse),
            "a wall rising off the floor"
        );
        let tangent = integral(quadratic(), &[(0.0, 0.0), (-0.5, 0.0), (-1.0, 0.5)]);
        assert_eq!(
            seam(at(&tangent, 0.0), at(&floor, 0.0)),
            Ok(DihedralClass::Smooth),
            "a wall leaving the floor along it"
        );
    }

    /// A rational chart on one side: a quarter cylinder of radius one
    /// leaving the seam along `+y`, smooth against the floor and
    /// transverse against the wall `y = 0`.
    #[test]
    fn a_rational_chart_reads_its_seam() {
        let h = core::f64::consts::FRAC_1_SQRT_2;
        let quarter = ruled(
            quadratic(),
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
            &[1.0, h, 1.0],
        );
        let wall = integral(linear(), &[(0.0, 0.0), (0.0, 1.0)]);
        assert_eq!(
            seam(at(&quarter, 0.0), at(&wall, 0.0)),
            Ok(DihedralClass::Transverse),
            "against the wall"
        );
        assert_eq!(
            seam(at(&quarter, 0.0), at(&floor(), 0.0)),
            Ok(DihedralClass::Smooth),
            "against the floor"
        );
    }

    /// A seam point on a C⁰ crease of a chart, or on a point where its
    /// `S_u × S_v` vanishes, reads no normal there and escalates as a
    /// crease, whatever the other face.
    #[test]
    fn a_seam_on_a_crease_or_a_pinch_escalates() {
        let floor = floor();
        // Folded along the `x` axis at the degree-1 knot `v = ½`.
        let creased = integral(
            KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (1.0, 1.0)],
        );
        // The same fold with no knot crease: the middle rows coincide,
        // so `S_v` vanishes on it.
        let pinched = integral(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            &[(-1.0, 1.0), (0.0, 0.0), (0.0, 0.0), (1.0, 1.0)],
        );
        for (what, chart) in [("a crease", &creased), ("a pinch", &pinched)] {
            let got = seam(at(chart, 0.5), at(&floor, 0.0));
            assert!(
                matches!(got, Err(SeamEscalation::Crease(_))),
                "{what}: {got:?}"
            );
        }
    }

    /// **No curvature is read across a crease within the reach**: the
    /// crease of a rational degree-2 chart (an interior knot of
    /// multiplicity two) a unit from the seam bounds the cells the unit
    /// reach covers, so the arm is poison and escalates; over half that
    /// reach the crease is past every cell read, and the seam decides.
    #[test]
    fn a_crease_within_the_reach_reads_no_arm() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let creased = ruled(
            kv,
            &[(0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.5, 1.5), (1.0, 2.0)],
            &[1.0, 0.8, 1.0, 0.8, 1.0],
        );
        assert_eq!(
            chart_arm(&creased, ORIGIN, 1.0),
            None,
            "the arm with the crease in reach"
        );
        let got = seam(at(&creased, 0.0), at(&floor(), 0.0));
        assert!(arm_refused(got), "the arm: {got:?}");
        assert_eq!(
            seam_over(at(&creased, 0.0), at(&floor(), 0.0), 0.5),
            Ok(DihedralClass::Transverse),
            "the crease past the reach"
        );
    }

    /// **The arm's lower end charges a bend it reaches, and no bend past
    /// it**: a wall flat for 0.05 m that bends hard beyond reads a lower
    /// arm stopping at the bend over a unit reach, so an angle whose sine
    /// over that arm lies in the band, transverse over the reach, cannot be
    /// decided and escalates at the arm; over a reach of 0.02, short of the
    /// bend, the lower arm is the reach.
    #[test]
    fn a_bend_within_the_arm_is_charged_and_one_past_it_is_not() {
        let flat = 0.05;
        let wall = bent(flat, 1.0, 0.0);
        let arm = chart_arm(&wall, ORIGIN, 1.0).expect("an arm");
        assert!(arm <= flat, "the arm stops at the bend: {arm}");
        assert_eq!(
            chart_arm(&wall, ORIGIN, 0.02),
            Some(0.02),
            "short of the bend, the reach"
        );
        let sin = in_band() / arm;
        let got = seam(
            at(&bent(flat, (1.0 - sin * sin).sqrt(), sin), 0.0),
            at(&floor(), 0.0),
        );
        assert!(
            matches!(got, Err(SeamEscalation::Lever(e))
                if e.rung() == crate::LeverRung::Arm
                    && e.diag().predicate == Some("dihedral_arm")),
            "in band over the lower arm, transverse over the reach: {got:?}"
        );
    }

    /// **Smooth stays reachable**: a wall leaving the floor along it and
    /// bending hard 0.05 m out has a short lower arm, and its seam still
    /// reads smooth, Zero over the reach.
    #[test]
    fn a_tangent_seam_with_a_short_lower_arm_reads_smooth() {
        let wall = bent(0.05, 1.0, 0.0);
        assert!(
            chart_arm(&wall, ORIGIN, 1.0).expect("an arm") <= 0.05,
            "the lower arm is short"
        );
        assert_eq!(
            seam(at(&wall, 0.0), at(&floor(), 0.0)),
            Ok(DihedralClass::Smooth)
        );
    }

    /// **A bend past the reach does not shrink the arm** (review r1): the
    /// floor and a wall planar over every point within the reach of 0.1,
    /// bent only 0.4 m out, read as two planes at that angle read, at an
    /// angle whose sine over the whole chart's curvature radius is under
    /// the band.
    #[test]
    fn a_bend_past_the_reach_reads_as_the_planes_do() {
        let reach = 0.1;
        let kappa = whole_chart_curvature(&bent(1.2, 1.0, 0.0));
        let b = band();
        let sin = 0.5 * b.zero() * kappa;
        assert!(
            sin * reach > 2.0 * b.escalate(),
            "κ̄ {kappa} too small for the row"
        );
        let planes = classify_dihedral(
            &plane(Vec3::new(0.0, 0.0, 1.0)),
            &plane(Vec3::new(0.0, -sin, (1.0 - sin * sin).sqrt())),
            ORIGIN,
            reach,
            b,
        );
        assert_eq!(planes, Ok(DihedralClass::Transverse), "the two planes");
        let wall = bent(1.2, (1.0 - sin * sin).sqrt(), sin);
        assert_eq!(
            seam_over(at(&wall, 0.0), at(&floor(), 0.0), reach),
            Ok(DihedralClass::Transverse),
            "the spline seam, κ̄ {kappa}, sin θ {sin}"
        );
    }

    /// **A bend within the reach but past the arm is not charged**
    /// (review r2): a wall leaving the floor flat for three quarters of a
    /// meter, bending hard only at its far end inside the unit reach,
    /// stands hundreds of bands off the floor across its flat run and reads
    /// transverse, as a plane at its angle does. The whole chart's
    /// curvature radius would read the corner smooth.
    #[test]
    fn a_far_bend_within_the_reach_reads_a_corner_transverse() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let wall_at = |c: f64, s: f64| {
            let rot = |(y, z): (f64, f64)| (c * y - s * z, s * y + c * z);
            let rows = [(0.0, 0.0), (0.5, 0.0), (1.0, 0.0), (1.0, 0.01)];
            integral(kv.clone(), &rows.map(rot))
        };
        let kappa = whole_chart_curvature(&wall_at(1.0, 0.0));
        let sin = 0.5 * band().zero() * kappa;
        let (c, s) = ((1.0 - sin * sin).sqrt(), sin);
        let wall = wall_at(c, s);
        let out = wall.ders(0.5, 0.5).point;
        assert!(
            out.z > 100.0 * band().escalate(),
            "the wall stands {} off the floor at {out:?}; κ̄ {kappa}, sin {sin}",
            out.z
        );
        assert_eq!(
            classify_dihedral(
                &plane(Vec3::new(0.0, -s, c)),
                &plane(Vec3::new(0.0, 0.0, 1.0)),
                ORIGIN,
                1.0,
                band()
            ),
            Ok(DihedralClass::Transverse),
            "the same angle between planes"
        );
        assert_eq!(
            seam(at(&wall, 0.0), at(&floor(), 0.0)),
            Ok(DihedralClass::Transverse),
            "the spline wall at that angle (κ̄ {kappa}, sin {sin})"
        );
    }

    /// **The reading is the sheet its parameters name**: a chart whose
    /// profile leaves the seam along `+z` and comes back through it along
    /// `+y` passes through the seam point twice, on two sheets. Against
    /// the floor, the first is transverse and the second smooth; a foot
    /// of the point would land on one of the two whichever was asked.
    #[test]
    fn the_seam_reads_the_sheet_its_parameters_name() {
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
        let hairpin = integral(
            kv,
            &[
                (0.0, 0.0),
                (0.0, 0.5),
                (-0.5, 0.0),
                (0.5, 0.0),
                (1.0, -0.5),
                (1.0, -1.0),
            ],
        );
        let floor = floor();
        assert!(
            (hairpin.ders(0.5, 0.5).point - ORIGIN).norm() < in_band(),
            "the second pass is through the seam point"
        );
        assert_eq!(
            seam(at(&hairpin, 0.0), at(&floor, 0.0)),
            Ok(DihedralClass::Transverse),
            "the sheet leaving along +z"
        );
        assert_eq!(
            seam(at(&hairpin, 0.5), at(&floor, 0.0)),
            Ok(DihedralClass::Smooth),
            "the sheet crossing along +y"
        );
    }

    /// A general bilinear chart over the rows `rows[i_u][i_v]`.
    fn general(rows: &[Vec<Point3<f64>>]) -> NurbsSurface<f64> {
        let control: Vec<Point3<f64>> = rows.iter().flatten().copied().collect();
        let n = control.len();
        NurbsSurface::new(linear(), linear(), control, vec![1.0; n]).unwrap()
    }

    /// **A planar wall at uneven speed never reads smooth** (re-review): an
    /// exactly planar cubic wall, its control rows collinear and bunched
    /// toward the seam, tilted 0.3 rad, meets a plane 100ε off it. Two
    /// planes at that angle read transverse over the unit reach; the
    /// chart's certified curvature over-reads the plane at uneven speed, so
    /// its lower arm is short, and the seam may escalate, never read
    /// smooth.
    #[test]
    fn a_flat_wall_with_uneven_speed_never_reads_smooth() {
        let cubic = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (s, c) = 0.3_f64.sin_cos();
        let rows = [(0.0, 0.0), (0.01, 0.0), (0.02, 0.0), (1.0, 0.0)];
        let wall = integral(cubic, &rows.map(|(y, z)| (c * y - s * z, s * y + c * z)));
        let (sd, cd) = (100.0 * band().zero()).sin_cos();
        let other = plane(Vec3::new(0.0, -(s * cd + c * sd), c * cd - s * sd));
        assert_eq!(
            classify_dihedral(&plane(Vec3::new(0.0, -s, c)), &other, ORIGIN, 1.0, band()),
            Ok(DihedralClass::Transverse),
            "the two planes"
        );
        let got = seam(at(&wall, 0.0), SeamFace::Analytic(&other));
        assert_ne!(got, Ok(DihedralClass::Smooth), "the planar spline wall");
    }

    /// **A flat wall beside a pole never reads smooth** (re-review): a
    /// planar triangle whose far edge collapses to an apex 0.02 m from the
    /// seam, against a plane 50ε off it. The cells at the apex read no
    /// curvature bound, which stops the lower arm there; the seam may
    /// escalate, never read smooth.
    #[test]
    fn a_flat_triangle_near_its_apex_never_reads_smooth() {
        let apex = Point3::new(0.0, 0.02, 0.0);
        let wall = general(&[
            vec![Point3::new(-1.0, 0.0, 0.0), apex],
            vec![Point3::new(1.0, 0.0, 0.0), apex],
        ]);
        let delta = 50.0 * band().zero();
        let other = plane(Vec3::new(0.0, -delta.sin(), delta.cos()));
        assert_eq!(
            classify_dihedral(
                &plane(Vec3::new(0.0, 0.0, 1.0)),
                &other,
                ORIGIN,
                1.0,
                band()
            ),
            Ok(DihedralClass::Transverse),
            "the two planes"
        );
        let got = seam(at(&wall, 0.0), SeamFace::Analytic(&other));
        assert_ne!(got, Ok(DihedralClass::Smooth), "the flat triangle");
    }

    /// Two analytic faces read as [`classify_dihedral`] reads them, the
    /// in-band escalation included.
    #[test]
    fn two_analytic_faces_read_as_classify_dihedral() {
        // A cylinder of radius ρ through the seam point, its normal there
        // `θ` off the floor's: its arm ρ binds the fold.
        let cylinder = |rho: f64, sin: f64| {
            let n = Vec3::new(0.0, -sin, (1.0 - sin * sin).sqrt());
            Surface::Cylinder {
                origin: ORIGIN - n * rho,
                axis: Vec3::new(1.0, 0.0, 0.0),
                radius: rho,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            }
        };
        let rho = 0.01;
        let floor = plane(Vec3::new(0.0, 0.0, 1.0));
        let others = [
            ("transverse", plane(Vec3::new(0.0, 1.0, 1.0))),
            ("in band", plane(Vec3::new(0.0, in_band(), 1.0))),
            ("tangent", plane(Vec3::new(0.0, 0.0, 1.0))),
            ("opposite sense", plane(Vec3::new(0.0, 0.0, -1.0))),
            ("a unit cylinder", cylinder(1.0, 0.5)),
            ("its own arm in band", cylinder(rho, in_band() / rho)),
        ];
        for (what, other) in &others {
            for (a, b) in [(&floor, other), (other, &floor)] {
                assert_eq!(
                    classify_seam(
                        [SeamFace::Analytic(a), SeamFace::Analytic(b)],
                        ORIGIN,
                        1.0,
                        chart_arm,
                        band()
                    ),
                    classify_dihedral(a, b, ORIGIN, 1.0, band()).map_err(SeamEscalation::Lever),
                    "{what}, {a:?} against {b:?}"
                );
            }
        }
        // The cylinder's arm decides the row: over the unit reach alone its
        // angle would read transverse.
        assert!(
            in_band() / rho > band().escalate(),
            "the row's angle is past the band over the reach"
        );
    }

    /// A spline carrier handed in as analytic is refused, not read as
    /// poison.
    #[test]
    fn a_spline_carrier_read_as_analytic_is_refused() {
        let wall = Surface::Nurbs(std::sync::Arc::new(floor()));
        let flat = plane(Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(
            classify_seam(
                [SeamFace::Analytic(&wall), SeamFace::Analytic(&flat)],
                ORIGIN,
                1.0,
                chart_arm,
                band()
            ),
            Err(SeamEscalation::SplineReadAsAnalytic)
        );
    }
}
