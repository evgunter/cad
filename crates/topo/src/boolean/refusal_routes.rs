//! **What a Boolean refusal says for the decision that raised it**
//! (D4 ¶1 (i)): each decision is a closed type set where the refusal is
//! raised or wrapped, so its sentence is an exhaustive match.
//!
//! - [`Contradiction`]: which fact contradicted a declared face pair
//!   (`BooleanError::DeclarationContradicted`,
//!   `MergeCoplanarError::DeclarationContradicted`), set by the rung
//!   that decided it (`plane_eq`'s declared rung, `carrier_eq`'s kind
//!   and data rungs). The verdict is definite, so its one lever is the
//!   declaration or the geometry, with no tolerance arm.
//! - [`BooleanDecision`]: which decision `BooleanError::Escalated`
//!   escalated on, set at the site that wraps the escalation. The
//!   ending follows from the decision and its verdict: a coincidence
//!   between the two solids composes `COINCIDENCE_RECOURSE`; a decision
//!   on a size the user may intend ends in its own lever, and on an
//!   in-band margin the tolerance that decides it
//!   (`geom_brep::recourse::SizedDecision`); a residual or a kernel
//!   self-check ends as a defect (`geom_brep::recourse::Unsized`).

use geom_brep::recourse::{Reading, RefusedArm, SizedDecision, SizedPass, StoredDefinite, Unsized};
use geom_core::Indeterminate;

pub use crate::sector_shape::SectorRung;

/// The one lever a contradicted declaration leaves: the declaration
/// is wrong, or the geometry is.
pub(crate) const CONTRADICTION_RECOURSE: &str =
    "Recourse: fix the declaration or move the geometry";

/// Which fact contradicted a declared pair: the rung that found the
/// two carriers definitely distinct.
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
pub enum BooleanDecision {
    /// Whether parts of the two solids coincide: the plane and carrier
    /// identity rungs, the tangent locus and its verification, the
    /// shared rim, the sections and joins, an edge or a vertex against a
    /// face, the sector and edge-edge classification, and the sphere
    /// lanes.
    Coincidence,
    /// A corner's own shape (`sector_shape`'s rungs).
    Corner(SectorRung),
    /// Whether a pierce point lies on the curved face it pierces, so
    /// the face's normal can be read there: a residual on a point the
    /// kernel computed.
    PierceOnFace,
    /// Whether a pierced torus's tube radius is positive.
    TorusTube,
    /// Whether a pierced torus's tube stays clear of its axis.
    TorusRing,
    /// Whether a point lies inside a face, on its boundary, or outside
    /// it (`ContainError::Escalated`).
    Containment,
    /// Whether a crossing the Boolean found on an edge lands inside it:
    /// every definite answer passes (inside, at an end, outside).
    CrossingOnEdge,
    /// Which of two crossings on an edge comes first.
    CrossingOrder,
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
    /// geometry and the tolerance (`COINCIDENCE_RECOURSE`).
    Coincidence,
    /// A decision on a size the user may intend.
    Sized(SizedDecision),
    /// A decision with no size the user chose.
    Unsized(Unsized),
}

/// Where a crossing lands along its edge, in words.
pub(crate) const CROSSING_INTERIOR: &str = "whether a crossing lands strictly inside its edge";

/// The lever every door that splits an edge at a crossing has (the
/// split, the blend, the Boolean): the geometry.
const CROSSING_LEVER: &str =
    "move the geometry so the crossing lands clearly away from the edge's ends";

/// What a crossing's interiority margin measures.
const CROSSING_SIZE: &str = "distance from the edge's end";

/// `Body::split_edge`'s interiority decision: it passes only on a
/// crossing definitely inside its edge.
pub(crate) const SPLIT_PARAM_INTERIOR: SizedDecision = SizedDecision {
    lever: CROSSING_LEVER,
    size: CROSSING_SIZE,
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// A corner's own shape.
const CORNER_LEVER: &str = "reshape that corner so its edges are clearly longer than the tolerance and clearly not in line";

/// Whether a point lies inside a face.
const CONTAINMENT_LEVER: &str =
    "move the parts so they meet clearly inside or clearly outside that face's boundary";

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
    /// What the decision decides, as a clause with no colon or dash of
    /// its own; the coincidence has its own sentence and no subject.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Coincidence => "whether parts of the two solids coincide",
            Self::Corner(rung) => rung.subject(),
            Self::PierceOnFace => {
                "whether a point lies on a curved face, so the face's normal can be read there"
            }
            Self::TorusTube => "whether a torus's tube radius is positive",
            Self::TorusRing => "whether a torus's tube stays clear of its axis",
            Self::Containment => {
                "whether a point lies inside a face, on its boundary, or outside it"
            }
            Self::CrossingOnEdge => CROSSING_INTERIOR,
            Self::CrossingOrder => "which of two crossings on an edge comes first",
            Self::SplitPointOnCircle => "whether a split point lies on the circle it was placed on",
            Self::ArcSpan => "whether an arc stays short of a full turn",
            Self::VolumeBackstop => "whether the result's volume agrees with its operands'",
        }
    }

    /// How the decision's escalation ends: from what it passes on.
    fn ending(self) -> Ending {
        match self {
            Self::Coincidence => Ending::Coincidence,
            // The arm passes on a positive length.
            Self::Corner(SectorRung::Arm) => {
                sized(CORNER_LEVER, "edge length", SizedPass::Positive)
            }
            // A straight corner passes on a negative cosine; a sector
            // bounded twice by one edge passes on any definite one.
            Self::Corner(SectorRung::Straight { full_circle }) => sized(
                CORNER_LEVER,
                "angle",
                if full_circle {
                    SizedPass::NonZero
                } else {
                    SizedPass::Negative
                },
            ),
            // It passes only at zero, and its definite sibling (a point
            // definitely off the face) is a broken classification
            // invariant.
            Self::PierceOnFace | Self::SplitPointOnCircle => Ending::Unsized(Unsized::Defect),
            Self::TorusTube => sized(
                "reshape the torus so its tube is clearly thicker than the tolerance",
                "tube radius",
                SizedPass::Positive,
            ),
            Self::TorusRing => sized(
                "reshape the torus so its tube stays clearly off its axis",
                "clearance between the tube and the axis",
                SizedPass::Positive,
            ),
            Self::Containment => sized(
                CONTAINMENT_LEVER,
                "distance from the face's boundary",
                SizedPass::NonZero,
            ),
            Self::CrossingOnEdge => sized(CROSSING_LEVER, CROSSING_SIZE, SizedPass::NonZero),
            Self::CrossingOrder => sized(
                "move the geometry so the two crossings on that edge lie clearly apart",
                "distance between the crossings",
                SizedPass::NonZero,
            ),
            // A span of at most one turn passes; a longer one is a broken
            // classification invariant.
            Self::ArcSpan => sized(
                "reshape the arc so it clearly stays short of a full turn",
                "arc",
                SizedPass::NonNegative,
            ),
            Self::VolumeBackstop => Ending::Unsized(Unsized::Defect),
        }
    }

    /// The one ending this decision's escalation `diag` carries, at the
    /// Boolean that built the geometry, where the decision has no
    /// sentence of its own; `None` for the coincidence, whose sentence
    /// composes `COINCIDENCE_RECOURSE`.
    #[must_use]
    pub(crate) fn ending_of(self, diag: &Indeterminate) -> Option<String> {
        let arm = RefusedArm::Undecided(diag);
        match self.ending() {
            Ending::Coincidence => None,
            Ending::Sized(decision) => Some(decision.recourse(arm, Reading::Build)),
            Ending::Unsized(decision) => Some(decision.recourse(arm, Reading::Build)),
        }
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
    use crate::splitting::SplitReduceError;
    use geom_core::{
        Band, COINCIDENCE_RECOURSE, KERNEL_DEFECT_ENDING, MarginDiag, Point3, Tol,
        UNREADABLE_MARGIN_NOTE, Vec3,
    };
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

    const CROSSING_LEVER_ENDING: &str =
        "Recourse: move the geometry so the crossing lands clearly away from the edge's ends";

    /// The split door's own clause, a stage for a subject, filed with
    /// its owner: `work/reach/reach-refusals-short-of-the-shape-guard.md`.
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

    /// Every decision, spelled once per variant: `Corner` once per rung
    /// and pass set.
    const DECISIONS: &[BooleanDecision] = &[
        BooleanDecision::Coincidence,
        BooleanDecision::Corner(SectorRung::Arm),
        BooleanDecision::Corner(SectorRung::Straight { full_circle: false }),
        BooleanDecision::Corner(SectorRung::Straight { full_circle: true }),
        BooleanDecision::PierceOnFace,
        BooleanDecision::TorusTube,
        BooleanDecision::TorusRing,
        BooleanDecision::Containment,
        BooleanDecision::CrossingOnEdge,
        BooleanDecision::CrossingOrder,
        BooleanDecision::SplitPointOnCircle,
        BooleanDecision::ArcSpan,
        BooleanDecision::VolumeBackstop,
    ];

    /// How a decision's escalation must end, written independently of
    /// the table: the coincidence sentence; a lever and the sign of the
    /// margins a smaller tolerance decides passing (`Some(true)` the
    /// positive ones, `Some(false)` the negative, `None` either); or the
    /// defect ending. The match is exhaustive, so a new decision is a
    /// compile error here until its ending is written down.
    enum Want {
        Coincidence,
        Sized(&'static str, Option<bool>),
        Defect,
    }

    fn want(decision: BooleanDecision) -> Want {
        const CORNER: &str = "Recourse: reshape that corner so its edges are clearly longer than \
                              the tolerance and clearly not in line";
        const CONTAIN: &str = "Recourse: move the parts so they meet clearly inside or clearly \
                               outside that face's boundary";
        match decision {
            BooleanDecision::Coincidence => Want::Coincidence,
            BooleanDecision::Corner(SectorRung::Arm) => Want::Sized(CORNER, Some(true)),
            BooleanDecision::Corner(SectorRung::Straight { full_circle: false }) => {
                Want::Sized(CORNER, Some(false))
            }
            BooleanDecision::Corner(SectorRung::Straight { full_circle: true }) => {
                Want::Sized(CORNER, None)
            }
            BooleanDecision::PierceOnFace
            | BooleanDecision::SplitPointOnCircle
            | BooleanDecision::VolumeBackstop => Want::Defect,
            BooleanDecision::TorusTube => Want::Sized(
                "Recourse: reshape the torus so its tube is clearly thicker than the tolerance",
                Some(true),
            ),
            BooleanDecision::TorusRing => Want::Sized(
                "Recourse: reshape the torus so its tube stays clearly off its axis",
                Some(true),
            ),
            BooleanDecision::Containment => Want::Sized(CONTAIN, None),
            BooleanDecision::CrossingOnEdge => Want::Sized(CROSSING_LEVER_ENDING, None),
            BooleanDecision::CrossingOrder => Want::Sized(
                "Recourse: move the geometry so the two crossings on that edge lie clearly apart",
                None,
            ),
            BooleanDecision::ArcSpan => Want::Sized(
                "Recourse: reshape the arc so it clearly stays short of a full turn",
                Some(true),
            ),
        }
    }

    /// **`BooleanError::Escalated` ends as its decision and verdict
    /// give**, for every decision, on an in-band margin of each sign and
    /// on an `INVALID` one:
    ///
    /// - a coincidence composes the coincidence sentence, and only it
    ///   offers a declaration;
    /// - a decision on a size names its lever and, on an in-band margin
    ///   on a side it passes on, the tolerance `|m|/K` below which that
    ///   margin is decided passing; on the other side, or on an
    ///   `INVALID` margin, no tolerance, and an `INVALID` one adds the
    ///   unreadable-margin note;
    /// - a residual or a kernel self-check ends as a defect and never
    ///   names the tolerance.
    #[test]
    fn every_escalation_ends_as_its_decision_and_verdict_give() {
        let b = band();
        let mid = (b.zero() + b.escalate()) / 2.0;
        for &decision in DECISIONS {
            for margin in [
                MarginDiag::value(mid),
                MarginDiag::value(-mid),
                MarginDiag::INVALID,
            ] {
                let diag = diag_of(margin);
                let text = BooleanError::Escalated { decision, diag }.to_string();
                let problems = short_of_the_guard(&text, &[]);
                assert!(problems.is_empty(), "{decision:?}: {problems:?}: {text}");
                assert!(
                    !text.contains("routing_probe"),
                    "{decision:?}: the routing name stays out: {text}"
                );
                assert_eq!(
                    text.contains("declare"),
                    decision == BooleanDecision::Coincidence,
                    "{decision:?}: only the coincidence offers a declaration: {text}"
                );
                let label = format!("{decision:?} at {margin}");
                match want(decision) {
                    Want::Coincidence => assert_eq!(
                        text,
                        format!(
                            "parts of the two solids are too close to call at this tolerance \
                             ({}), and the Boolean never snaps them together. Recourse: \
                             {COINCIDENCE_RECOURSE}",
                            diag.payload()
                        ),
                        "{label}"
                    ),
                    Want::Sized(lever, passing_sign) => {
                        let head =
                            format!("{} is undecided: {}. ", decision.subject(), diag.payload());
                        assert!(
                            text.starts_with(&head) && text[head.len()..].starts_with(lever),
                            "{label}: its subject, then its lever: {text}"
                        );
                        let offer = if margin.is_invalid() {
                            assert!(text.ends_with(UNREADABLE_MARGIN_NOTE), "{label}: {text}");
                            None
                        } else {
                            let m = point_margin(&diag);
                            passing_sign
                                .is_none_or(|positive| (m > 0.0) == positive)
                                .then(|| Some(m.abs() / k()))
                        };
                        assert_eq!(offered_below(&text), offer, "{label}: {text}");
                    }
                    Want::Defect => assert!(
                        text.ends_with(KERNEL_DEFECT_ENDING) && offered_below(&text).is_none(),
                        "{label}: the defect ending, and no tolerance: {text}"
                    ),
                }
            }
        }
    }

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
                         never glues a lie. Recourse: fix the declaration or move the geometry"
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
            "a declared coincidence contradicts the geometry: the declared planes are not \
             parallel, and the merge never glues a lie. Recourse: fix the declaration or move \
             the geometry"
        );
        let wrapped = BooleanError::Merge(err).to_string();
        let problems = short_of_the_guard(&wrapped, &[]);
        assert!(problems.is_empty(), "{problems:?}: {wrapped}");
    }
}
