//! **What a Boolean refusal says for the decision that raised it**
//! (D4 ¶1 (i)): each decision is a closed type set where the refusal is
//! raised or wrapped, so its sentence is an exhaustive match, composed
//! here.
//!
//! - [`Contradiction`]: which fact contradicted a declared face pair
//!   (`BooleanError::DeclarationContradicted`,
//!   `MergeCoplanarError::DeclarationContradicted`), set by the rung
//!   that decided it (`plane_eq`'s declared rung, `carrier_eq`'s kind
//!   and data rungs). The verdict is definite, so it ends in
//!   `contact::CONTRADICTION_RECOURSE`, with no tolerance arm.
//! - [`BooleanDecision`]: which decision `BooleanError::Escalated`
//!   escalated on, set at the site that wraps the escalation.
//!   [`BooleanDecision::render`] composes the whole sentence from the
//!   decision and its verdict: a coincidence between the two solids its
//!   own sentence and `COINCIDENCE_RECOURSE`; a decision on a size the
//!   user may intend its subject, its own lever and, on an in-band
//!   margin, the tolerance that decides it
//!   (`geom_brep::recourse::SizedDecision`); a residual or a kernel
//!   self-check its subject and the defect ending
//!   (`geom_brep::recourse::Unsized`). A decision with a decided
//!   refusal of its own ends it from the same table: the torus
//!   convention's (`BooleanError::DegenerateTorus`) reads
//!   [`TorusConvention::sized`], as its escalation does.

use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite, Unsized};
use geom_core::{COINCIDENCE_RECOURSE, Indeterminate, UNREADABLE_MARGIN_NOTE};

use super::plane_eq::PLANE_ORIENTATION;
pub use super::plane_eq::PlaneRung;
use crate::face_normal::NormalDecision;
pub use crate::sector_shape::SectorRung;
use crate::splitting::ConicRootFault;
pub use crate::splitting::CrossingDecision;
pub use geom_brep::TorusConvention;

/// Which fact contradicted a declared pair: the rung that found the
/// two carriers definitely distinct.
///
/// The façade's curated list carries neither this nor
/// [`BooleanDecision`]; why, and what would change that, is
/// `work/lib/boolean-decision-and-contradiction-are-rungs-under-boolean-error.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contradiction {
    /// The declared planes' normals are not parallel.
    PlanesNotParallel,
    /// The declared planes are parallel and offset.
    PlanesApart,
    /// The declared faces lie on different kinds of surface.
    KindsDiffer,
    /// The declared cylinders' axes are not parallel.
    CylinderAxesNotParallel,
    /// The declared cylinders' axes are parallel and offset.
    CylinderAxesApart,
    /// The declared cylinders' radii differ.
    CylinderRadiiDiffer,
    /// The declared spheres' centres differ.
    SphereCentresDiffer,
    /// The declared spheres' radii differ.
    SphereRadiiDiffer,
    /// The declared tori's axes are not parallel.
    TorusAxesNotParallel,
    /// The declared tori's centres differ.
    TorusCentresDiffer,
    /// The declared tori's major radii differ.
    TorusMajorRadiiDiffer,
    /// The declared tori's tube radii differ.
    TorusTubeRadiiDiffer,
}

impl Contradiction {
    /// The fact, as a clause with no colon or dash of its own.
    #[must_use]
    pub fn fact(self) -> &'static str {
        match self {
            Self::PlanesNotParallel => "the declared planes are not parallel",
            Self::PlanesApart => "the declared planes are parallel but apart",
            Self::KindsDiffer => "the declared faces are different kinds of surface",
            Self::CylinderAxesNotParallel => "the declared cylinders' axes are not parallel",
            Self::CylinderAxesApart => "the declared cylinders' axes are parallel but apart",
            Self::CylinderRadiiDiffer => "the declared cylinders' radii differ",
            Self::SphereCentresDiffer => "the declared spheres' centres differ",
            Self::SphereRadiiDiffer => "the declared spheres' radii differ",
            Self::TorusAxesNotParallel => "the declared tori's axes are not parallel",
            Self::TorusCentresDiffer => "the declared tori's centres differ",
            Self::TorusMajorRadiiDiffer => "the declared tori's major radii differ",
            Self::TorusTubeRadiiDiffer => "the declared tori's tube radii differ",
        }
    }

    /// Whether the fact is a separation a designed clearance could
    /// explain (a parallel offset, a centre offset, a radius
    /// difference), which `contact_verify::fit_steer` points at `Fit`.
    /// An angle or a kind no gap reconciles is not, and neither are the
    /// torus separations, which the steer's inventory does not name.
    #[must_use]
    pub(crate) fn fits_a_clearance(self) -> bool {
        match self {
            Self::PlanesApart
            | Self::CylinderAxesApart
            | Self::CylinderRadiiDiffer
            | Self::SphereCentresDiffer
            | Self::SphereRadiiDiffer => true,
            Self::PlanesNotParallel
            | Self::KindsDiffer
            | Self::CylinderAxesNotParallel
            | Self::TorusAxesNotParallel
            | Self::TorusCentresDiffer
            | Self::TorusMajorRadiiDiffer
            | Self::TorusTubeRadiiDiffer => false,
        }
    }
}

/// Which decision a Boolean escalation came from, set at the site that
/// wraps the escalation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(strum::EnumDiscriminants))]
#[cfg_attr(
    test,
    strum_discriminants(name(BooleanDecisionKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum BooleanDecision {
    /// Whether parts of the two solids coincide: the plane and carrier
    /// identity rungs, the tangent locus and its verification, the
    /// shared rim, the sections and joins, an edge or a vertex against a
    /// face, the sector and edge-edge classification, and the sphere
    /// lanes.
    Coincidence,
    /// Whether two planes face the same way or opposite ways, at a
    /// cross-operand door, which both definite signs answer and no
    /// declaration changes ([`PlaneRung::Orientation`]).
    PlaneOrientation,
    /// Whether a declared pair's planes are parallel. The declared rung
    /// bridges an in-band margin, so what escalates is a norm it could
    /// not read ([`PlaneRung::Parallel`]).
    DeclaredParallel,
    /// Whether two neighbouring faces of one operand lie on one plane,
    /// as the maximal-faces gate asks it of a rung of the plane ladder
    /// (F7). Declarations name pairs across the operands, so none
    /// settles it.
    Neighbours(PlaneRung),
    /// A corner's own shape (`sector_shape`'s rungs).
    Corner(SectorRung),
    /// Whether a pierce point lies on the curved face it pierces, so
    /// the face's normal can be read there. The point is a vertex of
    /// the piercing solid that the contact sweep already placed on that
    /// face, so this is a residual re-asking a decision taken upstream.
    PierceOnFace,
    /// A half of a pierced torus's ring convention.
    Torus(TorusConvention),
    /// Whether a point lies inside a face, on its boundary, or outside
    /// it (`ContainError::Escalated`), from any rung of the walk.
    Containment,
    /// Where a crossing lands along its edge.
    Crossing(CrossingDecision),
    /// Whether a vertex of one solid coincides with a vertex of the
    /// other, asked of an edge end on a curved face: a coincidence both
    /// verdicts of which pass, and which no face-pair declaration names.
    VertexOnVertex,
    /// Whether a split point lies on the circle it was placed on: a
    /// residual on a point the kernel placed.
    SplitPointOnCircle,
    /// Whether an arc stays short of a full turn.
    ArcSpan,
    /// Whether the result's volume agrees with its operands': the
    /// kernel checking its own result.
    VolumeBackstop,
}

/// How one decision's escalation ends.
enum Ending {
    /// A coincidence between the two solids: the declaration, the
    /// geometry and the tolerance (`COINCIDENCE_RECOURSE`), in the
    /// coincidence's own sentence.
    Coincidence,
    /// A decision on a size the user may intend.
    Sized(SizedDecision),
    /// A family of decisions the escalation does not tell apart: the
    /// family's lever alone, since no one member's margin gives a
    /// tolerance to tighten below.
    Lever(&'static str),
    /// A decision with no size the user chose.
    Unsized(Unsized),
}

/// Which door compared two planes: what a plane rung's escalation asks
/// there, and which move reaches a pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlaneDoor {
    /// A face of each operand, no declaration naming the pair.
    Undeclared,
    /// A face of each operand, the pair declared.
    Declared,
    /// Two neighbouring faces of one operand (F7).
    Neighbours,
}

impl PlaneDoor {
    /// The cross-operand door a pair meets, by whether it is declared.
    pub(crate) const fn of(declared: bool) -> Self {
        if declared {
            Self::Declared
        } else {
            Self::Undeclared
        }
    }
}

/// The maximal-faces gate's lever: the one move that takes two
/// neighbouring faces off the question, whichever rung asked it.
const NEIGHBOUR_LEVER: &str = "merge the two faces into one first (merge_coplanar_faces), or \
                               tilt one so they meet at a clear angle along an edge clearly \
                               longer than the tolerance";

/// A corner's own shape.
const CORNER_LEVER: &str = "reshape that corner so its edges are clearly longer than the tolerance and clearly not in line";

/// A sized decision's table row at a build, where the stored arm is
/// never read.
const fn sized(lever: &'static str, size: &'static str, passes: SizedPass) -> Ending {
    Ending::Sized(SizedDecision {
        lever,
        size,
        passes,
        stored: StoredDefinite::Lever,
        at_zero: None,
    })
}

impl BooleanDecision {
    /// The decision a pierce point's face normal escalated on.
    pub(crate) const fn of_normal(decision: NormalDecision) -> Self {
        match decision {
            NormalDecision::Torus(half) => Self::Torus(half),
            NormalDecision::OnSurface => Self::PierceOnFace,
        }
    }

    /// The decision a plane-identity rung escalated on at `door`:
    /// across the operands, in-band parallelism is a coincidence a
    /// declaration would bridge unless the pair is declared already, and
    /// orientation a decision of its own; between neighbours of one
    /// operand, both rungs ask the maximal-faces question.
    pub(crate) const fn of_plane_rung(rung: PlaneRung, door: PlaneDoor) -> Self {
        match (door, rung) {
            (PlaneDoor::Undeclared, PlaneRung::Parallel) => Self::Coincidence,
            (PlaneDoor::Declared, PlaneRung::Parallel) => Self::DeclaredParallel,
            (PlaneDoor::Undeclared | PlaneDoor::Declared, PlaneRung::Orientation) => {
                Self::PlaneOrientation
            }
            (PlaneDoor::Neighbours, rung) => Self::Neighbours(rung),
        }
    }

    /// The decision a conic root lane's escalation came from: a
    /// crossing decision as the fault routes it, and otherwise a
    /// coincidence between the plane and the conic.
    pub(crate) fn of_conic_root(fault: ConicRootFault) -> Self {
        fault.decision().map_or(Self::Coincidence, Self::Crossing)
    }

    /// What the decision decides, as a clause with no colon or dash of
    /// its own; the coincidence has its own sentence and no subject.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Coincidence => "whether parts of the two solids coincide",
            Self::PlaneOrientation => PlaneRung::Orientation.subject(),
            Self::DeclaredParallel => PlaneRung::Parallel.subject(),
            Self::Neighbours(_) => "whether two neighbouring faces of one operand lie on one plane",
            Self::Corner(rung) => rung.subject(),
            Self::PierceOnFace => {
                "whether a point lies on a curved face, so the face's normal can be read there"
            }
            Self::Torus(half) => half.subject(),
            Self::Containment => {
                "whether a point lies inside a face, on its boundary, or outside it"
            }
            Self::Crossing(decision) => decision.subject(),
            Self::VertexOnVertex => "whether a vertex of one solid coincides with one of the other",
            Self::SplitPointOnCircle => "whether a split point lies on the circle it was placed on",
            Self::ArcSpan => "whether an arc stays short of a full turn",
            Self::VolumeBackstop => "whether the result's volume agrees with its operands'",
        }
    }

    /// How the decision's escalation ends: from what it passes on.
    fn ending(self) -> Ending {
        match self {
            Self::Coincidence => Ending::Coincidence,
            Self::PlaneOrientation => Ending::Sized(PLANE_ORIENTATION),
            // A poisoned description, which the merge's declared rung
            // ends the same way (`MergeDecision::DeclaredPlanes`).
            Self::DeclaredParallel => Ending::Unsized(Unsized::Defect),
            // The margin is the normals' sine over the shared edge's
            // chord, and a definitely positive one (a clear angle)
            // passes.
            Self::Neighbours(PlaneRung::Parallel) => sized(
                NEIGHBOUR_LEVER,
                "bend between the two faces over their shared edge's chord",
                SizedPass::Positive,
            ),
            // Asked only once the angle read flat over the chord; either
            // definite orientation then leaves the faces coplanar, which
            // the gate refuses, so no sign of this margin passes and no
            // tolerance decides it passing.
            Self::Neighbours(PlaneRung::Orientation) => Ending::Lever(NEIGHBOUR_LEVER),
            // The arm passes on a positive length.
            Self::Corner(SectorRung::Arm) => {
                sized(CORNER_LEVER, "edge length", SizedPass::Positive)
            }
            // The margin is cos θ levered by the shorter edge: that
            // edge's projection onto the other, a length. A straight
            // corner passes on a negative one (`sector_shape` gates it with
            // `decide_negative`); a sector bounded twice by
            // one edge passes on any definite one.
            Self::Corner(SectorRung::Straight { full_circle }) => sized(
                CORNER_LEVER,
                "projection of one of the corner's edges onto the other",
                if full_circle {
                    SizedPass::AnySign
                } else {
                    SizedPass::Negative
                },
            ),
            // Each passes only at zero, and its definite sibling (a
            // point definitely off) is a broken classification
            // invariant.
            Self::PierceOnFace | Self::SplitPointOnCircle => Ending::Unsized(Unsized::Defect),
            Self::Torus(half) => Ending::Sized(half.sized()),
            // The escalation does not carry which rung of the walk
            // refused, and the rungs pass on different sets (the carrier
            // rung is a residual where the caller placed the point on
            // the surface; the period rung refuses a negative margin),
            // so no one margin gives a tolerance to tighten below.
            Self::Containment => Ending::Lever(
                "move the parts so they meet clearly inside or clearly outside that face's \
                 boundary",
            ),
            Self::Crossing(decision) => Ending::Sized(decision.sized()),
            // Both definite verdicts pass (the vertices meet, or lie
            // apart); a negative distance is not a verdict.
            Self::VertexOnVertex => sized(
                "move the parts so their vertices clearly meet or lie clearly apart",
                "distance between the vertices",
                SizedPass::NonNegative,
            ),
            // The edge's certification decided this same margin (its
            // headroom to a full turn, metred at the radius) at this
            // band, so neither arm is one the user reaches: an arc
            // certified short of a full turn that now reads longer, or
            // undecided, is the kernel's. It ends as its definite
            // sibling, a broken classification invariant, does.
            Self::ArcSpan => Ending::Unsized(Unsized::Defect),
            Self::VolumeBackstop => Ending::Unsized(Unsized::Defect),
        }
    }

    /// The whole sentence an escalation of this decision renders, at the
    /// Boolean that built the geometry: the coincidence's own, or the
    /// subject, the payload and the one ending the verdict gives.
    #[must_use]
    pub(crate) fn render(self, diag: &Indeterminate) -> String {
        let arm = RefusedArm::Undecided(diag);
        let ending = match self.ending() {
            Ending::Coincidence => {
                return format!(
                    "parts of the two solids are too close to call at this tolerance ({}), \
                     and the Boolean never snaps them together. Recourse: \
                     {COINCIDENCE_RECOURSE}",
                    diag.payload()
                );
            }
            Ending::Sized(decision) => decision.recourse(arm, Reading::Build),
            Ending::Lever(lever) if diag.margin.is_invalid() => {
                format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}")
            }
            Ending::Lever(lever) => format!("Recourse: {lever}"),
            Ending::Unsized(decision) => decision.recourse(arm, Reading::Build),
        };
        format!(
            "{} is undecided: {}. {ending}",
            self.subject(),
            diag.payload()
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::boolean::{BooleanError, CarrierDesc, CarrierEqError, Operand, PlaneIdentity};
    use crate::entity::{EdgeKey, VertexKey};
    use crate::euler::EulerOpError;
    use crate::merge_faces::MergeCoplanarError;
    use crate::sector_shape::SectorRungKind;
    use crate::splitting::SplitReduceError;
    use geom_core::{Band, KERNEL_DEFECT_ENDING, MarginDiag, Point3, Tol, Vec3};
    use strum::IntoEnumIterator as _;
    use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band forms")
    }

    fn diag_of(margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: band(),
            predicate: Some("routing_probe"),
            terminal_sliver: false,
        }
    }

    /// The band's multiplier `K`, which turns a margin into the
    /// tolerance a smaller one than which decides it.
    fn k() -> f64 {
        band().escalate() / band().zero()
    }

    /// The point margin `diag` carries, for this row's arithmetic.
    fn point_margin(diag: &Indeterminate) -> f64 {
        diag.margin
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a point margin")
    }

    /// The tolerance an ending offers to tighten below, where it offers
    /// one; `Some(None)` for an offer that names no value.
    fn offered_below(text: &str) -> Option<Option<f64>> {
        let (_, tail) = text.split_once("tighten the tolerance")?;
        Some(
            tail.strip_prefix(" below ")
                .and_then(|v| v.strip_suffix(" m"))
                .and_then(|v| v.parse::<f64>().ok()),
        )
    }

    /// The refusal-shape guard's three checks on one rendered text:
    /// exactly one recourse, a subject for every escalation payload,
    /// and no stage prefix other than the `filed` ones.
    fn short_of_the_guard(text: &str, filed: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        match recourse_markers(text) {
            1 => {}
            n => out.push(format!("{n} recourses, not one")),
        }
        for clause in subjectless_escalations(text) {
            out.push(format!("an escalation with no subject ({clause:?})"));
        }
        for prefix in stage_prefixes(text, &[]) {
            if !filed.iter().any(|f| prefix.starts_with(f)) {
                out.push(format!("the stage prefix {prefix:?}"));
            }
        }
        out
    }

    const NEIGHBOURS: &str = "whether two neighbouring faces of one operand lie on one plane";
    const NEIGHBOUR_ENDING: &str = "Recourse: merge the two faces into one first \
                                    (merge_coplanar_faces), or tilt one so they meet at a clear \
                                    angle along an edge clearly longer than the tolerance";

    const TUBE_LEVER: &str =
        "Recourse: reshape the torus so its tube is clearly thicker than the tolerance";
    const RING_LEVER: &str = "Recourse: make the tube radius clearly smaller than the ring radius";

    const CROSSING_LEVER_ENDING: &str =
        "Recourse: move the geometry so the crossing lands clearly away from the edge's ends";

    /// The split door's own clause, a stage for a subject, filed with
    /// its owner: `work/hone/reach-refusals-short-of-the-shape-guard.md`.
    const SPLIT_DOOR_FILED: &str = "inserting the plane crossing on edge";

    /// **`split_edge`'s in-band interiority reads whole at every door
    /// that forwards it**, on a real raise: the split and the Boolean
    /// here (the blend's door is `sweep`'s, and its row is there). The
    /// subject is the decision; the one recourse is the lever every
    /// splitting door has and, the decision passing on a positive
    /// margin, the tolerance below which that margin is decided
    /// passing. No door is offered a declaration.
    #[test]
    fn the_split_param_escalation_reads_whole_at_every_splitting_door() {
        let raise = || {
            let cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
            let mut body = cube.body;
            let edge = cube.mevs[0].edge;
            let b = band();
            let err = body
                .split_edge(edge, (b.zero() + b.escalate()) * 0.5, Tol::witness())
                .unwrap_err();
            let EulerOpError::SplitParamEscalated { diag, .. } = err else {
                panic!("the band-midpoint split escalates: {err:?}");
            };
            (err, point_margin(&diag))
        };
        let edge = EdgeKey::default();
        let (operator, margin) = raise();
        assert!(margin > 0.0, "the midpoint margin is positive: {margin:e}");
        let rendered = [
            ("the operator", operator.to_string()),
            (
                "the split",
                SplitReduceError::CrossingInsertion {
                    edge,
                    endpoints: (VertexKey::default(), VertexKey::default()),
                    source: raise().0,
                }
                .to_string(),
            ),
            (
                "the Boolean",
                BooleanError::CrossingInsertion {
                    operand: Operand::A,
                    edge,
                    source: raise().0,
                }
                .to_string(),
            ),
        ];
        for (door, text) in rendered {
            let problems = short_of_the_guard(&text, &[SPLIT_DOOR_FILED]);
            assert!(problems.is_empty(), "{door}: {problems:?}: {text}");
            assert!(
                text.contains(
                    "whether a crossing lands strictly inside its edge is undecided: margin "
                ) && text.contains(CROSSING_LEVER_ENDING)
                    && !text.contains("declare"),
                "{door} states the decision and the lever it has, and no declaration: {text}"
            );
            assert_eq!(
                offered_below(&text),
                Some(Some(margin / k())),
                "{door} offers the tolerance the margin gives: {text}"
            );
        }
    }

    /// **Every decision, by construction.** The top-level variants come
    /// from the compiler (`BooleanDecisionKind::iter`), and each kind's
    /// concrete decisions from a match that must name every kind, over
    /// the nested rungs' own compiler-derived lists: a new decision, or
    /// a new rung under one, is a compile error or a new row here, never
    /// a silent pass.
    fn every_decision() -> Vec<BooleanDecision> {
        BooleanDecisionKind::iter()
            .flat_map(|kind| match kind {
                BooleanDecisionKind::Coincidence => vec![BooleanDecision::Coincidence],
                BooleanDecisionKind::PlaneOrientation => vec![BooleanDecision::PlaneOrientation],
                BooleanDecisionKind::DeclaredParallel => vec![BooleanDecision::DeclaredParallel],
                BooleanDecisionKind::Neighbours => {
                    PlaneRung::iter().map(BooleanDecision::Neighbours).collect()
                }
                BooleanDecisionKind::Corner => SectorRungKind::iter()
                    .flat_map(|rung| match rung {
                        SectorRungKind::Arm => vec![SectorRung::Arm],
                        SectorRungKind::Straight => [false, true]
                            .map(|full_circle| SectorRung::Straight { full_circle })
                            .to_vec(),
                    })
                    .map(BooleanDecision::Corner)
                    .collect(),
                BooleanDecisionKind::PierceOnFace => vec![BooleanDecision::PierceOnFace],
                BooleanDecisionKind::Torus => TorusConvention::iter()
                    .map(BooleanDecision::Torus)
                    .collect(),
                BooleanDecisionKind::Containment => vec![BooleanDecision::Containment],
                BooleanDecisionKind::Crossing => CrossingDecision::iter()
                    .map(BooleanDecision::Crossing)
                    .collect(),
                BooleanDecisionKind::VertexOnVertex => vec![BooleanDecision::VertexOnVertex],
                BooleanDecisionKind::SplitPointOnCircle => {
                    vec![BooleanDecision::SplitPointOnCircle]
                }
                BooleanDecisionKind::ArcSpan => vec![BooleanDecision::ArcSpan],
                BooleanDecisionKind::VolumeBackstop => vec![BooleanDecision::VolumeBackstop],
            })
            .collect()
    }

    /// How a decision's escalation must end, written independently of
    /// the table.
    enum Ending {
        /// The coincidence's own sentence.
        Coincidence,
        /// The lever, and the sign of the margins a smaller tolerance
        /// decides passing: `Some(true)` the positive ones, `Some(false)`
        /// the negative, `None` either.
        Sized(&'static str, Option<bool>),
        /// The lever alone, on every margin.
        Lever(&'static str),
        /// The defect ending.
        Defect,
    }

    /// Each decision's subject and ending, as literals: an independent
    /// statement of the words `subject` and `ending` must produce.
    fn want(decision: BooleanDecision) -> (&'static str, Ending) {
        const CORNER: &str = "Recourse: reshape that corner so its edges are clearly longer than \
                              the tolerance and clearly not in line";
        const STRAIGHT: &str = "whether a corner is straight or folds back on itself";
        match decision {
            BooleanDecision::Coincidence => (
                "whether parts of the two solids coincide",
                Ending::Coincidence,
            ),
            BooleanDecision::Corner(SectorRung::Arm) => (
                "whether a corner's edges are long enough to measure its angle over",
                Ending::Sized(CORNER, Some(true)),
            ),
            BooleanDecision::Corner(SectorRung::Straight { full_circle: false }) => {
                (STRAIGHT, Ending::Sized(CORNER, Some(false)))
            }
            BooleanDecision::Corner(SectorRung::Straight { full_circle: true }) => {
                (STRAIGHT, Ending::Sized(CORNER, None))
            }
            BooleanDecision::PierceOnFace => (
                "whether a point lies on a curved face, so the face's normal can be read there",
                Ending::Defect,
            ),
            BooleanDecision::PlaneOrientation => (
                "whether the two planes face the same way or opposite ways",
                Ending::Sized(
                    "Recourse: make the edges at the corner where the two faces meet clearly \
                     longer than the tolerance",
                    None,
                ),
            ),
            BooleanDecision::DeclaredParallel => {
                ("whether the two planes are parallel", Ending::Defect)
            }
            BooleanDecision::Neighbours(PlaneRung::Parallel) => {
                (NEIGHBOURS, Ending::Sized(NEIGHBOUR_ENDING, Some(true)))
            }
            BooleanDecision::Neighbours(PlaneRung::Orientation) => {
                (NEIGHBOURS, Ending::Lever(NEIGHBOUR_ENDING))
            }
            BooleanDecision::Torus(TorusConvention::Tube) => (
                "whether a torus's tube radius is positive",
                Ending::Sized(TUBE_LEVER, Some(true)),
            ),
            BooleanDecision::Torus(TorusConvention::Ring) => (
                "whether a torus's tube radius is smaller than its ring radius",
                Ending::Sized(RING_LEVER, Some(true)),
            ),
            BooleanDecision::Containment => (
                "whether a point lies inside a face, on its boundary, or outside it",
                Ending::Lever(
                    "Recourse: move the parts so they meet clearly inside or clearly outside \
                     that face's boundary",
                ),
            ),
            BooleanDecision::Crossing(CrossingDecision::OnEdge) => (
                "whether a crossing lands strictly inside its edge",
                Ending::Sized(CROSSING_LEVER_ENDING, None),
            ),
            BooleanDecision::Crossing(CrossingDecision::Order) => (
                "which of two crossings on an edge comes first",
                Ending::Sized(
                    "Recourse: move the geometry so the two crossings on that edge lie clearly \
                     apart",
                    None,
                ),
            ),
            BooleanDecision::VertexOnVertex => (
                "whether a vertex of one solid coincides with one of the other",
                Ending::Sized(
                    "Recourse: move the parts so their vertices clearly meet or lie clearly \
                     apart",
                    Some(true),
                ),
            ),
            BooleanDecision::SplitPointOnCircle => (
                "whether a split point lies on the circle it was placed on",
                Ending::Defect,
            ),
            BooleanDecision::ArcSpan => {
                ("whether an arc stays short of a full turn", Ending::Defect)
            }
            BooleanDecision::VolumeBackstop => (
                "whether the result's volume agrees with its operands'",
                Ending::Defect,
            ),
        }
    }

    /// The tolerance a smaller one than which decides `margin` on the
    /// side(s) `passing` names: a point margin at `|m|/K`, an enclosure
    /// with both ends on one such side at its nearer end's.
    fn expected_offer(margin: MarginDiag, passing: Option<bool>) -> Option<f64> {
        let on = |v: f64| v != 0.0 && passing.is_none_or(|positive| (v > 0.0) == positive);
        match margin.diagnostic_f64_for_error_text() {
            geom_core::ErrorTextReading::Value(m) => on(m).then(|| m.abs() / k()),
            geom_core::ErrorTextReading::Enclosure { lo, hi } => {
                (on(lo) && on(hi) && (lo > 0.0) == (hi > 0.0)).then(|| lo.abs().min(hi.abs()) / k())
            }
            geom_core::ErrorTextReading::Invalid => None,
        }
    }

    /// **`BooleanError::Escalated` ends as its decision and verdict
    /// give**, for every decision, on in-band margins of each sign (a
    /// point and an enclosure), an enclosure across zero, a signed zero,
    /// and an `INVALID` margin (the enclosure and zero rows are the
    /// review's `probe_c7_shape_guard_enclosures`):
    ///
    /// - every sentence opens on its decision's own subject, written
    ///   here as a literal, and passes the refusal-shape guard;
    /// - a coincidence composes the coincidence sentence, and only it
    ///   offers a declaration;
    /// - a decision on a size names its lever and, on an in-band margin
    ///   on a side it passes on, the tolerance below which that margin is
    ///   decided passing; elsewhere no tolerance, and an `INVALID` margin
    ///   adds the unreadable-margin note;
    /// - a family the escalation does not tell apart names its lever
    ///   alone;
    /// - a residual or a kernel self-check ends as a defect and never
    ///   names the tolerance.
    #[test]
    fn every_escalation_ends_as_its_decision_and_verdict_give() {
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let mid = (z + e) / 2.0;
        for decision in every_decision() {
            for margin in [
                MarginDiag::value(mid),
                MarginDiag::value(-mid),
                MarginDiag::enclosure(2.0 * z, 0.5 * e),
                MarginDiag::enclosure(-0.5 * e, -2.0 * z),
                MarginDiag::enclosure(-2.0 * z, 3.0 * z),
                MarginDiag::value(0.0),
                MarginDiag::value(-0.0),
                MarginDiag::INVALID,
            ] {
                let diag = diag_of(margin);
                let text = BooleanError::Escalated { decision, diag }.to_string();
                let label = format!("{decision:?} at {margin}");
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
                assert!(
                    !text.contains("routing_probe"),
                    "{label}: the routing name stays out: {text}"
                );
                assert_eq!(
                    text.contains("declare"),
                    decision == BooleanDecision::Coincidence,
                    "{label}: only the coincidence offers a declaration: {text}"
                );
                let (subject, ending) = want(decision);
                let head = format!("{subject} is undecided: {}. ", diag.payload());
                let tail = text.strip_prefix(&head);
                match ending {
                    Ending::Coincidence => assert_eq!(
                        text,
                        format!(
                            "parts of the two solids are too close to call at this tolerance \
                             ({}), and the Boolean never snaps them together. Recourse: \
                             declare the coincidence, move the geometry, or lower the tolerance",
                            diag.payload()
                        ),
                        "{label}"
                    ),
                    Ending::Sized(lever, passing) => {
                        assert!(
                            tail.is_some_and(|t| t.starts_with(lever)),
                            "{label}: its subject, then its lever: {text}"
                        );
                        assert_eq!(
                            text.ends_with(UNREADABLE_MARGIN_NOTE),
                            margin.is_invalid(),
                            "{label}: {text}"
                        );
                        assert_eq!(
                            offered_below(&text),
                            expected_offer(margin, passing).map(Some),
                            "{label}: {text}"
                        );
                    }
                    Ending::Lever(lever) => {
                        let want = if margin.is_invalid() {
                            format!("{lever}; {UNREADABLE_MARGIN_NOTE}")
                        } else {
                            lever.to_owned()
                        };
                        assert_eq!(tail, Some(want.as_str()), "{label}: {text}");
                    }
                    Ending::Defect => assert_eq!(
                        tail,
                        Some(KERNEL_DEFECT_ENDING),
                        "{label}: its subject, then the defect ending: {text}"
                    ),
                }
            }
        }
    }

    /// **A containment escalation on a residual rung names no
    /// tolerance**, on a real raise: a point off a cylinder wall by an
    /// in-band distance escalates the carrier rung, which passes only
    /// on the surface where the crossing layer placed the point there,
    /// so its margin is a miss, not a size. The containment family ends
    /// on its lever alone. (The review's
    /// `probe_c1_containment_residual_rung_constructed` rendered "tighten
    /// the tolerance below …" here.)
    #[test]
    fn a_containment_escalation_on_a_residual_rung_names_its_lever_alone() {
        use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
        let tol = Tol::witness();
        let b = band();
        let mut body: crate::body::Body<f64> = brick((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), tol);
        let wall = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            None,
            (0.5, 2.0),
            (0.0, 1.0),
            tol,
        );
        let r = 1.0 + (b.zero() + b.escalate()) / 2.0;
        let p = Point3::new(r * 1.2_f64.cos(), r * 1.2_f64.sin(), 0.5);
        let diag = match crate::boolean::contain::curved_face_placement(&body, wall, p, b) {
            Err(crate::boolean::ContainError::Escalated(diag)) => diag,
            other => panic!("an in-band point off the wall escalates: {other:?}"),
        };
        assert_eq!(diag.predicate, Some("bool_curved_contain_carrier"));
        let text = BooleanError::Escalated {
            decision: BooleanDecision::Containment,
            diag,
        }
        .to_string();
        assert!(
            text.ends_with(
                "Recourse: move the parts so they meet clearly inside or clearly outside that \
                 face's boundary"
            ) && !text.contains("tolerance below"),
            "{text}"
        );
    }

    /// **An ellipse carrier's escalation reads by door**: the section
    /// through a curved face whose kind the carrier constructor could not
    /// decide says which question it hinged on, offers the split the
    /// geometry and the tolerance alone and the Boolean its declaration
    /// too, and offers neither a carrier to construct.
    #[test]
    fn an_ellipse_carrier_escalation_is_routed_by_door() {
        let b = band();
        let rows = [
            (
                "ellipse_axes_distinct",
                "whether the curve is a circle or an ellipse",
            ),
            (
                "ellipse_minor_positive",
                "whether the curve's minor semi-axis is positive",
            ),
        ];
        for (predicate, subject) in rows {
            let diag = Indeterminate {
                predicate: Some(predicate),
                ..diag_of(MarginDiag::value((b.zero() + b.escalate()) / 2.0))
            };
            let join = || crate::SplitJoinError::Section {
                face: crate::entity::FaceKey::default(),
                source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(diag)),
            };
            let split = crate::SplitError::Join(join()).to_string();
            let boolean = BooleanError::Join(join()).to_string();
            for text in [&split, &boolean] {
                let problems = short_of_the_guard(text, &[]);
                assert!(problems.is_empty(), "{predicate}: {problems:?}: {text}");
                assert!(
                    text.contains(&format!(
                        "{subject} is undecided for the section through a curved face: {}. ",
                        diag.payload()
                    )) && !text.contains("Circle carrier"),
                    "{predicate}: {text}"
                );
            }
            assert!(
                split.ends_with("Recourse: move the geometry, or lower the tolerance")
                    && !split.contains("declare"),
                "{predicate}: the split takes no declaration: {split}"
            );
            assert!(
                boolean.ends_with(&format!("Recourse: {}", geom_core::COINCIDENCE_RECOURSE)),
                "{predicate}: the Boolean takes one: {boolean}"
            );
        }
    }

    /// **The conic root lane ends alike at the split and the Boolean**,
    /// rung by rung, through the one routing ([`ConicRootFault::decision`]):
    /// a crossing rung renders the same sentence at both doors, its
    /// subject and its decision's ending; a rung that asks whether the
    /// plane coincides with the conic states its own subject at the split,
    /// with the split's levers and no declaration, and is the
    /// coincidence at the Boolean.
    #[test]
    fn the_conic_root_lane_ends_alike_at_the_split_and_the_boolean() {
        let b = band();
        let diag = diag_of(MarginDiag::value((b.zero() + b.escalate()) / 2.0));
        let faults = [
            (
                ConicRootFault::PlaneParallel(diag),
                "whether a curved edge's plane is parallel to the plane that cuts it",
            ),
            (
                ConicRootFault::BellyGraze(diag),
                "whether a plane cuts a curved edge, grazes it or misses it",
            ),
            (
                ConicRootFault::CrossingInterior(diag),
                "whether a crossing lands strictly inside its edge",
            ),
            (
                ConicRootFault::RootOrder(diag),
                "which of two crossings on an edge comes first",
            ),
        ];
        for (fault, subject) in faults {
            let split = SplitReduceError::CrossingEscalated {
                edge: EdgeKey::default(),
                fault,
            }
            .to_string();
            let boolean = BooleanError::Escalated {
                decision: BooleanDecision::of_conic_root(fault),
                diag,
            }
            .to_string();
            for text in [&split, &boolean] {
                let problems = short_of_the_guard(text, &[]);
                assert!(problems.is_empty(), "{fault:?}: {problems:?}: {text}");
            }
            assert!(
                split.starts_with(&format!("{subject} is undecided: {}. ", diag.payload())),
                "{fault:?}: {split}"
            );
            match fault.decision() {
                Some(_) => assert_eq!(split, boolean, "{fault:?}"),
                None => {
                    assert!(
                        split.ends_with(
                            "Recourse: move the split plane or the geometry, or lower the \
                             tolerance"
                        ) && !split.contains("declare"),
                        "{fault:?}: {split}"
                    );
                    assert!(
                        boolean.contains("declare the coincidence"),
                        "{fault:?}: {boolean}"
                    );
                }
            }
        }
    }

    /// The recourse of a contradicted declaration, as `contact` states
    /// it for every contradiction.
    const CONTRADICTED: &str =
        "Recourse: correct or remove the declaration, or move the geometry so it holds";

    /// Each contradiction's clause, and the predicate whose definite
    /// verdict raises it, written independently of the enum.
    const FACTS: &[(Contradiction, &str, &str)] = &[
        (
            Contradiction::PlanesNotParallel,
            "bool_plane_parallel",
            "the declared planes are not parallel",
        ),
        (
            Contradiction::PlanesApart,
            "bool_plane_offset",
            "the declared planes are parallel but apart",
        ),
        (
            Contradiction::KindsDiffer,
            "carrier_kind",
            "the declared faces are different kinds of surface",
        ),
        (
            Contradiction::SphereCentresDiffer,
            "carrier_sphere_center",
            "the declared spheres' centres differ",
        ),
        (
            Contradiction::SphereRadiiDiffer,
            "carrier_sphere_radius",
            "the declared spheres' radii differ",
        ),
        (
            Contradiction::CylinderAxesNotParallel,
            "carrier_cyl_axis_parallel",
            "the declared cylinders' axes are not parallel",
        ),
        (
            Contradiction::CylinderAxesApart,
            "carrier_cyl_axis_offset",
            "the declared cylinders' axes are parallel but apart",
        ),
        (
            Contradiction::CylinderRadiiDiffer,
            "carrier_cyl_radius",
            "the declared cylinders' radii differ",
        ),
        (
            Contradiction::TorusAxesNotParallel,
            "carrier_torus_axis_parallel",
            "the declared tori's axes are not parallel",
        ),
        (
            Contradiction::TorusCentresDiffer,
            "carrier_torus_center",
            "the declared tori's centres differ",
        ),
        (
            Contradiction::TorusMajorRadiiDiffer,
            "carrier_torus_major_radius",
            "the declared tori's major radii differ",
        ),
        (
            Contradiction::TorusTubeRadiiDiffer,
            "carrier_torus_minor_radius",
            "the declared tori's tube radii differ",
        ),
    ];

    /// A declared pair of `c1` and `c2`, verified by the real rung
    /// (`carrier_eq`, which `recl` and `vtxfac` call and `plane_eq`
    /// serves for planes): the contradiction it raises.
    fn contradicted(c1: CarrierDesc<f64>, c2: CarrierDesc<f64>) -> (Contradiction, Indeterminate) {
        let id = PlaneIdentity {
            s1: None,
            s2: None,
            declared: true,
        };
        match crate::boolean::carrier_eq(&c1, &c2, id, 1.0, band()) {
            Err(CarrierEqError::Contradicted { fact, diag }) => (fact, diag),
            other => panic!("a declared pair this far apart contradicts: {other:?}"),
        }
    }

    /// **A contradicted declaration names the fact that contradicted
    /// it**, one clause per rung, on the verdict the real rung raises
    /// for a pair built to trip that rung alone — two cylinders of
    /// different radii among them, the non-planar pair `recl` raises.
    /// The verdict is definite: no payload, no declaration offered, no
    /// tolerance, one recourse.
    #[test]
    fn every_contradiction_names_the_fact_that_contradicted() {
        let p = Point3::new;
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let o = p(0.0, 0.0, 0.0);
        let plane = |origin, normal| CarrierDesc::Plane { origin, normal };
        let sphere = |center, radius| CarrierDesc::Sphere {
            center,
            radius,
            outward: true,
        };
        let cylinder = |origin, axis, radius| CarrierDesc::Cylinder {
            origin,
            axis,
            radius,
            outward: true,
        };
        let torus = |center, axis, major_radius, minor_radius| CarrierDesc::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            outward: true,
        };
        let pairs = [
            (plane(o, z), plane(o, x)),
            (plane(o, z), plane(p(0.0, 0.0, 1.0), z)),
            (plane(o, z), sphere(o, 1.0)),
            (sphere(o, 1.0), sphere(p(1.0, 0.0, 0.0), 1.0)),
            (sphere(o, 1.0), sphere(o, 2.0)),
            (cylinder(o, z, 1.0), cylinder(o, x, 1.0)),
            (cylinder(o, z, 1.0), cylinder(p(1.0, 0.0, 0.0), z, 1.0)),
            (cylinder(o, z, 1.0), cylinder(o, z, 2.0)),
            (torus(o, z, 2.0, 0.5), torus(o, x, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(p(0.0, 0.0, 1.0), z, 2.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 3.0, 0.5)),
            (torus(o, z, 2.0, 0.5), torus(o, z, 2.0, 0.25)),
        ];
        assert_eq!(pairs.len(), FACTS.len());
        for ((c1, c2), &(want, name, clause)) in pairs.into_iter().zip(FACTS) {
            let (fact, diag) = contradicted(c1, c2);
            assert_eq!(
                (fact, diag.predicate),
                (want, Some(name)),
                "the pair trips {name}"
            );
            let planar = matches!(
                fact,
                Contradiction::PlanesNotParallel | Contradiction::PlanesApart
            );
            let mut texts = vec![(
                BooleanError::DeclarationContradicted { fact }.to_string(),
                "the Boolean",
            )];
            if planar {
                texts.push((
                    MergeCoplanarError::DeclarationContradicted { fact }.to_string(),
                    "the merge",
                ));
            }
            for (text, who) in texts {
                assert_eq!(
                    text,
                    format!(
                        "a declared coincidence contradicts the geometry: {clause}, and {who} \
                         never glues a lie. {CONTRADICTED}"
                    ),
                    "{who}, {name}"
                );
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{who}, {name}: {problems:?}: {text}");
                assert!(
                    !text.contains("tolerance"),
                    "{who}, {name}: a definite verdict names no tolerance: {text}"
                );
            }
        }
    }

    /// **The merge meets a contradicted declaration on a real raise**:
    /// two faces of a brick that meet at an edge, declared one surface.
    #[test]
    fn a_declared_pair_of_meeting_faces_is_contradicted_at_the_merge() {
        let tol = Tol::witness();
        let mut body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let normal = |b: &crate::body::Body<f64>, s| match b.get_surface(s) {
            Some(&crate::Surface::Plane { normal, .. }) => normal,
            other => panic!("a brick face is a plane: {other:?}"),
        };
        let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
        let first = surfaces[0];
        let meeting = *surfaces
            .iter()
            .find(|&&s| normal(&body, s).dot(normal(&body, first)).abs() < 0.5)
            .expect("a brick face meets four others");
        let err = body
            .merge_coplanar_faces_declared(&[(first, meeting)], tol)
            .expect_err("two meeting faces are not one plane");
        assert!(
            matches!(
                err,
                MergeCoplanarError::DeclarationContradicted {
                    fact: Contradiction::PlanesNotParallel
                }
            ),
            "the declared rung contradicts on parallelism: {err:?}"
        );
        let text = err.to_string();
        assert_eq!(
            text,
            format!(
                "a declared coincidence contradicts the geometry: the declared planes are not \
                 parallel, and the merge never glues a lie. {CONTRADICTED}"
            )
        );
        let wrapped = BooleanError::Merge(err).to_string();
        let problems = short_of_the_guard(&wrapped, &[]);
        assert!(problems.is_empty(), "{problems:?}: {wrapped}");
    }

    /// A one-face skeletal body whose face carries `surface`.
    fn face_on(surface: crate::Surface<f64>) -> (crate::body::Body<f64>, crate::entity::FaceKey) {
        let st = crate::fixtures::mvfs_state();
        let mut body = st.body;
        body.set_face_surface(
            st.face,
            crate::euler::FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .expect("a skeletal face takes any surface");
        (body, st.face)
    }

    /// **A pierced torus tells one story per half of its ring
    /// convention, on every arm** (D4 ¶1 (iv)), each a real raise: the
    /// pierce point's normal door refuses a torus face and the Boolean
    /// routes that refusal as `vtxfac` does. In band, decided at zero
    /// (with a margin and exactly on), and definitely negative, each
    /// half names its own lever and passes the refusal-shape guard;
    /// an arm with a positive margin offers the tolerance the margin
    /// gives; no arm offers a declaration or calls the torus a pairing
    /// not supported yet. (No offer on a nonpositive margin is not
    /// asserted: `Refused::Negative` is sign-certain and a zero margin
    /// leaves no size, so no single edit makes one offer.)
    #[test]
    fn a_pierced_torus_tells_one_story_per_convention_half() {
        use crate::face_normal::face_outward_normal_at;
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let mid = (z + e) / 2.0;
        // (half, R, r, the half's margin, whether the arm is in band).
        let rows = [
            (TorusConvention::Tube, 0.75, mid, mid, true),
            (TorusConvention::Tube, 0.75, 0.5 * z, 0.5 * z, false),
            (TorusConvention::Tube, 0.75, -0.3, -0.3, false),
            (TorusConvention::Ring, 0.5 + mid, 0.5, 0.5 + mid - 0.5, true),
            (
                TorusConvention::Ring,
                0.5 + 0.5 * z,
                0.5,
                0.5 + 0.5 * z - 0.5,
                false,
            ),
            (TorusConvention::Ring, 0.5, 0.5, 0.0, false),
            (TorusConvention::Ring, 0.4, 0.5, 0.4 - 0.5, false),
        ];
        for (half, big_r, r, margin, in_band) in rows {
            let label = format!("{half:?} at R = {big_r}, r = {r}");
            let (body, face) = face_on(crate::Surface::Torus {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: big_r,
                minor_radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            });
            let refusal = face_outward_normal_at(&body, face, Point3::new(big_r + r, 0.0, 0.0), b)
                .expect_err("a torus outside the ring convention has no normal");
            let err = BooleanError::of_pierced_normal(refusal, Operand::B, face);
            match (&err, in_band) {
                (BooleanError::Escalated { decision, .. }, true) => {
                    assert_eq!(*decision, BooleanDecision::Torus(half), "{label}");
                }
                (BooleanError::DegenerateTorus { convention, .. }, false) => {
                    assert_eq!(*convention, half, "{label}");
                }
                _ => panic!("{label}: the arm's own refusal: {err:?}"),
            }
            let text = err.to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
            let lever = match half {
                TorusConvention::Tube => TUBE_LEVER,
                TorusConvention::Ring => RING_LEVER,
            };
            assert!(
                text.contains(lever) && !text.contains("declare") && !text.contains("supported"),
                "{label}: the half's one lever, no declaration, no gap: {text}"
            );
            if margin > 0.0 {
                assert_eq!(
                    offered_below(&text),
                    Some(Some(margin / k())),
                    "{label}: the tolerance a positive margin gives: {text}"
                );
            }
        }
    }

    /// Two planes through `z = 1`, the second facing `sign` times the
    /// first's normal.
    fn planes(
        sign: f64,
    ) -> (
        crate::boolean::PlaneDesc<f64>,
        crate::boolean::PlaneDesc<f64>,
    ) {
        let plane = |n: f64| crate::boolean::PlaneDesc {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(0.0, 0.0, n),
        };
        (plane(1.0), plane(sign))
    }

    const DECLARED: PlaneIdentity<'static> = PlaneIdentity {
        s1: None,
        s2: None,
        declared: true,
    };

    /// **At the Boolean's cross-operand doors a plane pair's orientation
    /// refusal names its own decision, the move that reaches a pass, and
    /// no declaration** (D4 ¶1 (i)), each a real raise: two coincident
    /// planes, facing the same way and opposite ways, compared at an arm
    /// that puts the orientation margin (the normals' cosine at the arm)
    /// in the zero band and in the ambiguity band, by the declared rung
    /// and by the undeclared ladder. The zero verdict carries the margin
    /// the rung decided. Both definite signs pass there, so either
    /// sign's margin offers the tolerance it gives; the lever lengthens
    /// the arm, the one thing an undecided margin measures.
    #[test]
    fn the_boolean_orientation_refusal_names_the_arm_and_offers_no_declaration() {
        use crate::boolean::{PlaneDoor, PlaneEqError, PlaneRung, oriented_plane_eq};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        for (arm, zero) in [(0.5 * z, true), ((z + e) / 2.0, false)] {
            for sign in [1.0, -1.0] {
                let (p1, p2) = planes(sign);
                for id in [DECLARED, PlaneIdentity::NONE] {
                    let label = format!("arm {arm:e}, facing {sign}, declared {}", id.declared);
                    let err = oriented_plane_eq(&p1, &p2, id, arm, b)
                        .expect_err("an orientation margin this small refuses");
                    let PlaneEqError::Escalated {
                        rung: PlaneRung::Orientation,
                        diag,
                    } = err
                    else {
                        panic!("{label}: the orientation rung refuses: {err:?}");
                    };
                    assert_eq!(
                        point_margin(&diag),
                        sign * arm,
                        "{label}: the margin the rung decided rides the payload"
                    );
                    let text = BooleanError::plane_identity(
                        PlaneRung::Orientation,
                        PlaneDoor::of(id.declared),
                        diag,
                    )
                    .to_string();
                    let problems = short_of_the_guard(&text, &[]);
                    assert!(problems.is_empty(), "{label}: {problems:?}: {text}");
                    assert!(
                        text.starts_with(
                            "whether the two planes face the same way or opposite ways is \
                             undecided: "
                        ) && text.contains(
                            "Recourse: make the edges at the corner where the two faces meet \
                             clearly longer than the tolerance"
                        ) && text.contains(if zero {
                            "lies within the zero band"
                        } else {
                            "lies inside the ambiguity band"
                        }) && !text.contains("declare"),
                        "{label}: {text}"
                    );
                    assert_eq!(
                        offered_below(&text),
                        Some(Some(arm / k())),
                        "{label}: {text}"
                    );
                }
            }
        }
    }

    /// **The merge's declared pair tells one story across its in-band
    /// and definite arms** (D4 ¶1 (iv)), and it is the merge's own: only
    /// a pair facing the same way glues, so the decision passes on a
    /// positive margin alone, where the Boolean's passes on either sign.
    /// Real raises of the declared rung, routed as the merge routes them
    /// (`declared_pair_verdict`): coincident planes facing the same way
    /// (positive margins) and opposite ways (negative), at a shared-edge
    /// chord in the zero band, in the ambiguity band, and definite. The
    /// same-facing definite pair glues; every other arm names the one
    /// lever toward that pass, no declaration, no stage label and no
    /// face key; a positive margin offers the tolerance it gives, and a
    /// negative margin or the definite opposite arm offers none, since
    /// no smaller tolerance turns opposite faces into same-facing ones.
    /// The Boolean's merge stage wraps the sentence behind its own
    /// label, compared as text rather than through the shape guard,
    /// which does not see that label
    /// (`work/tint/the-shape-guard-misses-the-boolean-merge-stage-label.md`).
    #[test]
    fn the_merge_orientation_tells_one_story_across_its_arms() {
        use crate::boolean::oriented_plane_eq;
        use crate::entity::FaceKey;
        use crate::merge_faces::declared_pair_verdict;
        const LEVER: &str = "Recourse: turn one of the two faces so both clearly face the same \
                             way, across a shared edge whose ends lie clearly apart";
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let (f1, f2) = (FaceKey::default(), FaceKey::default());
        for (chord, definite) in [(0.5 * z, false), ((z + e) / 2.0, false), (1.0, true)] {
            for sign in [1.0, -1.0] {
                let label = format!("chord {chord:e}, facing {sign}");
                let (p1, p2) = planes(sign);
                let verdict =
                    declared_pair_verdict(oriented_plane_eq(&p1, &p2, DECLARED, chord, b), f1, f2);
                let err = match verdict {
                    Ok(glued) => {
                        assert!(
                            definite && sign > 0.0 && glued,
                            "{label}: only a definite same-facing pair glues"
                        );
                        continue;
                    }
                    Err(err) => err,
                };
                assert_eq!(
                    matches!(err, MergeCoplanarError::DeclaredOppositeOrientation { .. }),
                    definite,
                    "{label}: the definite opposite pair is the decision's sign-certain arm: \
                     {err:?}"
                );
                let text = err.to_string();
                assert_eq!(recourse_markers(&text), 1, "{label}: {text}");
                assert!(
                    subjectless_escalations(&text).is_empty()
                        && stage_prefixes(&text, &[]).is_empty(),
                    "{label}: {text}"
                );
                let head = if definite {
                    "the two declared faces face opposite ways across the edge they share. "
                } else {
                    "whether the two declared faces face the same way across the edge they \
                     share is undecided: "
                };
                assert!(
                    text.starts_with(head)
                        && text.contains(LEVER)
                        && !text.contains("declare the")
                        && !text.contains("merge_coplanar_faces")
                        && !text.contains("FaceKey"),
                    "{label}: {text}"
                );
                let offer = (!definite && sign > 0.0).then(|| chord / k());
                assert_eq!(offered_below(&text), offer.map(Some), "{label}: {text}");
                assert_eq!(
                    BooleanError::Merge(err).to_string(),
                    format!("coplanar-merge output stage refused: {text}"),
                    "{label}: the Boolean's merge stage forwards the sentence"
                );
            }
        }
    }

    /// **The maximal-faces gate ends a near-flat pair of neighbours in
    /// its own lever, on a real raise** (F7): a brick's top face split
    /// along its diagonal, one half re-described on a plane bent about
    /// that diagonal by an angle whose sine over the diagonal lands in
    /// the ambiguity band. The gate's parallelism rung escalates, and
    /// the refusal names the gate's decision and lever with the
    /// tolerance its margin gives, not the declare menu, which no
    /// declaration between two faces of one operand could settle.
    #[test]
    fn the_maximal_faces_gate_ends_near_flat_neighbours_in_its_own_lever() {
        let (tol, b) = (Tol::witness(), band());
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let prism = crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, tol);
        let mut body = prism.body;
        let outer = body.get_face(prism.top_face).expect("the top face").outer;
        let crate::LoopBoundary::Cycle { first } =
            body.get_loop(outer).expect("its outer loop").boundary
        else {
            panic!("the top face's outer loop is a cycle");
        };
        let cycle = body.loop_cycle(first).expect("the top loop walks");
        let at = |body: &crate::body::Body<f64>, he| {
            let v = body.get_half_edge(he).expect("a live half-edge").start;
            *body
                .get_point(body.get_vertex(v).expect("a live vertex").point)
                .expect("a live point")
        };
        let (p0, p1) = (at(&body, cycle[0]), at(&body, cycle[2]));
        let half = body
            .mef_chord(
                crate::euler::MefSite::Chords {
                    he1: cycle[0],
                    he2: cycle[2],
                },
                tol,
            )
            .expect("the diagonal splits the top face");
        let diagonal = (p1 - p0).norm();
        let along = (p1 - p0) * (1.0 / diagonal);
        let theta = (b.zero() + b.escalate()) / 2.0 / diagonal;
        let up = Vec3::new(0.0, 0.0, 1.0);
        // Lifts both refusals: the bent plane is the gate's near-flat input; the half's edges are not the row.
        body.set_face_surface_stranding_for_tests(
            half.face,
            crate::euler::FaceSurface::New {
                surface: crate::Surface::Plane {
                    origin: p0,
                    normal: up * theta.cos() + along.cross(up) * theta.sin(),
                    u_ref: along,
                },
                sense: true,
            },
        )
        .expect("a face takes a plane through its diagonal");
        let err = super::super::reduce::gate_maximal_faces(&body, Operand::A, b)
            .expect_err("the bent neighbours are too flat to call");
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the gate escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::Neighbours(PlaneRung::Parallel));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        let problems = short_of_the_guard(&text, &[]);
        assert!(problems.is_empty(), "{problems:?}: {text}");
        assert!(
            text.starts_with(NEIGHBOURS)
                && text.contains(NEIGHBOUR_ENDING)
                && !text.contains("declare"),
            "{text}"
        );
        assert_eq!(
            offered_below(&text),
            Some(Some(point_margin(&diag) / k())),
            "{text}"
        );
    }

    /// **A declared pair's parallelism, and the maximal-faces gate's
    /// rungs, each end where their own door can reach** (D4 ¶1 (i)).
    /// At a declared door the parallelism rung bridges an in-band
    /// margin, so its one escalation is a norm it could not read
    /// (`plane_eq::unreadable_norm`, the rung's own raise): the merge and
    /// the Boolean both end it as a defect, with no declaration. The
    /// maximal-faces gate compares two faces of one operand, which no
    /// declaration names: a real in-band parallelism raise there (two
    /// neighbours bent by an in-band angle over a unit chord) and a
    /// real orientation raise (coincident neighbours over a chord in
    /// the band) end in the gate's lever, the former with the tolerance
    /// its margin gives and the latter with none, since either
    /// orientation leaves the faces coplanar.
    #[test]
    fn declared_parallelism_and_the_neighbour_gate_end_where_their_door_reaches() {
        use crate::boolean::plane_eq::unreadable_norm;
        use crate::boolean::{PlaneDesc, PlaneDoor, PlaneEqError, PlaneRung, oriented_plane_eq};
        let b = band();
        let (z, e) = (b.zero(), b.escalate());
        let PlaneEqError::Escalated {
            rung: PlaneRung::Parallel,
            diag,
        } = unreadable_norm(b)
        else {
            panic!("the unreadable norm is the parallelism rung's");
        };
        let boolean = BooleanError::plane_identity(PlaneRung::Parallel, PlaneDoor::Declared, diag)
            .to_string();
        let merge = MergeCoplanarError::of_declared_refusal(unreadable_norm(b)).to_string();
        for text in [&boolean, &merge] {
            let problems = short_of_the_guard(text, &[]);
            assert!(problems.is_empty(), "{problems:?}: {text}");
            assert_eq!(
                text.strip_prefix("whether the two planes are parallel is undecided: ")
                    .and_then(|t| t.split_once(". "))
                    .map(|(_, ending)| ending),
                Some(KERNEL_DEFECT_ENDING),
                "a declared door's unreadable norm is a defect, at both: {text}"
            );
        }
        const GATE: &str = "Recourse: merge the two faces into one first (merge_coplanar_faces), \
                            or tilt one so they meet at a clear angle along an edge clearly \
                            longer than the tolerance";
        let theta = (z + e) / 2.0;
        let (flat, _) = planes(1.0);
        let bent = PlaneDesc {
            origin: Point3::new(0.0, 0.0, 1.0),
            normal: Vec3::new(theta.sin(), 0.0, theta.cos()),
        };
        for (p2, chord, rung) in [
            (bent, 1.0, PlaneRung::Parallel),
            (flat, (z + e) / 2.0, PlaneRung::Orientation),
        ] {
            let err = oriented_plane_eq(&flat, &p2, PlaneIdentity::NONE, chord, b)
                .expect_err("the gate's rung refuses");
            let PlaneEqError::Escalated { rung: got, diag } = err else {
                panic!("{rung:?}: an escalation: {err:?}");
            };
            assert_eq!(got, rung);
            let text = BooleanError::plane_identity(rung, PlaneDoor::Neighbours, diag).to_string();
            let problems = short_of_the_guard(&text, &[]);
            assert!(problems.is_empty(), "{rung:?}: {problems:?}: {text}");
            assert!(
                text.starts_with(
                    "whether two neighbouring faces of one operand lie on one plane is \
                     undecided: "
                ) && text.contains(GATE)
                    && !text.contains("declare"),
                "{rung:?}: {text}"
            );
            let offer = (rung == PlaneRung::Parallel).then(|| point_margin(&diag) / k());
            assert_eq!(offered_below(&text), offer.map(Some), "{rung:?}: {text}");
        }
    }
}
