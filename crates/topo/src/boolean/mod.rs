//! Booleans, part 1 (M3 PR 4): **reduction + classification across two
//! bodies** — ch. 15 §§15.4–15.6 re-derived under our conventions, with
//! the TOG 1986 second witness supplying the unprinted on-edge
//! machinery (Tables II/III). Scope is BINDING (M3-PLAN PR item 4):
//! reduction sweep, the three ON-sets as contact records
//! (F1/F2), vertex-vertex sector classification, vertex-on-face
//! classification with the ring insertion, on-edge machinery, and
//! paired null-edge insertion with explicit cross-body correspondence
//! keys (F9/F12). [`boolean_reduce`] itself stops there: both operands
//! are functionally untouched and the annotated clones come back in
//! [`BooleanReduction`]. Joining, result generation and the
//! containment fallback are the module's too — they arrived with PR 5
//! and after, and [`BooleanError`] carries their refusals — so a
//! reader deciding what belongs in this file should read the pipeline
//! below, not this paragraph, as its boundary.
//!
//! Pipeline of [`boolean_reduce`]:
//!
//! 1. **Gates**: per-arm since M5 PR 9 (C12.1 —
//!    [`BooleanError::CurvedBooleanUnsupported`] retires per C5 table
//!    arm; every kind but `Approx` passes, `Nurbs` under ∪ only);
//!    each operand a closed solid by tier 2's own verdict
//!    ([`crate::validate_closed`]; no scaffolding operands); **maximal
//!    faces (F7)** via the
//!    coincidence ladder — adjacent faces sharing a surface key or with
//!    bit-equal oriented planes ([`plane_eq`]) refuse as
//!    [`BooleanError::NonMaximalFaces`]; *numeric* coplanarity never
//!    triggers the precondition (it is not coincidence; if it bites, it
//!    bites later as a typed escalation — the ladder's honest shape).
//! 2. **Reduction sweep** (`reduce`): edge×face, BOTH directions,
//!    candidate generation through the `bvh` tree since M5 PR 8 (the
//!    tree prunes, predicates decide — `reduce` module docs; the
//!    brute-force scan survives as `SweepStrategy::Idealized` under
//!    the differential suite, behind the `sweep-testing` feature, so
//!    the name is not a link), with `contfv`/`contfp` as typed
//!    trilean case codes. Proper
//!    crossings insert vertices via the certified `split_edge` lane;
//!    edge-on-edge crossings are discovered as edge-face events landing
//!    ON an edge (both edges split → a v-v pair); coplanar edge-face
//!    pairs are skipped — their edge-edge events are caught when the
//!    edge meets the face's noncoplanar NEIGHBOR faces (tested).
//!    Every contact the sweep discovers or creates is emitted as a
//!    **contact record** ([`ContactRecords`]), citing the coincidence
//!    that decided it; nothing is ever scanned-for after the fact.
//! 3. **Classification** (`sectors`/`recl`/`tables`/`vtxfac`): v-v
//!    pairs via the all-pairs sector intersection search (Programs
//!    15.7–15.9 re-derived), on-sector reclassification (15.10 in
//!    full), on-edge reclassification (TOG Tables II/III as typed
//!    decision tables); v-on-f pairs via the ch. 14 classifier deltas
//!    (plane := the pierced face's plane, OUT/IN from
//!    [`geom_brep::enters_material`] — never 15.7's printed labels)
//!    plus the null-edge **ring insertion** into the pierced face.
//! 4. **Paired null-edge insertion** (`insert`): per consecutive
//!    surviving crossing-record pair, one null edge in each solid, F9
//!    attributes ([`crate::null::NullEdge`], below ≙ IN, above ≙ OUT)
//!    and explicit A↔B correspondence keys as data
//!    ([`NullEdgePairRecord`]) — never correlated array order
//!    (`ssortnulledges` is engineered out). The 15.11
//!    consecutive-pairing invariant is guarded at runtime (no two pairs
//!    may cross in either neighborhood, and where one nests another in
//!    B's the inner mints after it, at the copy of the innermost fan
//!    that holds it) and stressed by the four-, six- and eight-crossing
//!    fixtures (F12).
//!
//! # The 15.7 sign resolution (F3)
//!
//! Program 15.7 prints `IN = +1` with `s = comp(dot(feq, ref))` — i.e.
//! positive dot ⇒ IN, coherent only for an INWARD feq. TOG §2/§6.1
//! fixes outward normals (our ratified convention), so the printed
//! labeling is the suspect side. We derive from `enters_material`:
//! `dot(dir, n_outward) < 0 ⇒ Enters ⇒ IN`; positive ⇒ OUT. Mirror
//! tests pin both directions on brick fixtures.

mod arcs;
pub(crate) mod boxes;
mod carrier_cross;
pub mod carrier_eq;
pub(crate) mod carrier_pair;
mod carrier_touch;
mod circle_roots;
mod circle_torus;
pub(crate) mod combine;
#[cfg(test)]
mod cone_frame_rows;
mod conic_quadric;
pub mod contact_verify;
// The conic rows' shared test oracles (test builds only).
#[cfg(test)]
mod conic_oracle;
mod contain;
mod discard;
#[cfg(feature = "door-tier3-meter")]
mod door_meter;
pub(crate) mod edge_join;
mod ellipse_torus;
// The variant roster the sample-coverage row reads (test builds only).
#[cfg(test)]
pub(crate) use contain::ContainErrorKind;
mod fate;
mod finish;
mod fragments;
mod glue;
#[cfg(test)]
mod glued_pair_rows;
pub(crate) mod insert;
mod join;
mod nary;
mod node;
mod ops;
pub(crate) use ops::{AtRestRefusal, at_rest_refusal};
pub(crate) use ops::{GluedSurfacePair, GluedSurfaces};
pub(crate) mod section_cert;
mod section_table;
mod spline_known;
#[cfg(any(test, feature = "test-support"))]
pub(crate) use ops::{ChartCache, section_report};
pub(crate) use ops::{boundary_edges, describe_edges};
#[cfg(any(test, feature = "test-support"))]
pub(crate) use ops::{
    no_crossings_certificates, no_crossings_section_report, no_event_section_report,
};
#[cfg(test)]
mod node_probe_rows;
pub mod plane_eq;
#[cfg(test)]
mod r2_probes;
pub(crate) mod recl;
pub(crate) mod records;
pub(crate) mod reduce;
pub(crate) mod refusal_routes;
pub(crate) use refusal_routes::PlaneDoor;
pub use refusal_routes::{
    BooleanDecision, Coincide, Contradiction, CrossingDecision, DeclarationRead, LeverArm,
    PlaneRung, SectionRadius, SectorRung, SelfCheck, Settling, SphereQuestion, TorusConvention,
    WallRung,
};
pub(crate) mod region;
mod rim_wedge;
pub(crate) mod ruled_region;
pub(crate) mod sectors;
pub(crate) mod separating;
mod shell_witness;
mod slit;
pub use slit::EdgeInFaceGap;
pub mod solid_contain;
pub(crate) mod sphere_region;
pub(crate) mod spline_region;
mod surface_group;
pub mod tables;
#[cfg(test)]
mod torn_hop_rows;
#[cfg(test)]
mod torus_frame_rows;
pub(crate) mod torus_region;
pub mod trace;
pub mod voids;
pub(crate) mod vtxfac;
pub(crate) mod zip;

use geom_core::{
    Band, BandError, Bounds, Decide, Indeterminate, KERNEL_DEFECT_ENDING, Margin, MarginDiag,
    NOT_YET_ENDING, Point3, Real, Sign, Tol,
};

use crate::body::Body;
use crate::chord_join::SplitJoinError;
use crate::coincidence::{Cited, Cites};
use crate::contact::GlueClass;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, ShellKey, VertexKey};
use crate::euler::EulerOpError;
use crate::live::Proven;
use crate::merge_faces::MergeCoplanarError;
use crate::validate::ValidationError;

pub use carrier_eq::{
    CarrierDesc, CarrierEqError, CarrierRelation, CoincidenceMeasure, ConsumedExtent, carrier_eq,
};
pub use contain::{
    ArcEndGaps, CONTAINMENT_RAISED, ContainDecision, ContainError, FaceContainment, contfp,
    curved_face_containment, placement_ending, placement_lever, placement_subject,
    unnamed_placement_ending,
};
// Crate-internal: tier 3's check 9 decides two whole-circle loops
// against each other (its contact arm 4) on the same loop
// classification this module's own walk dispatches on, and the census
// places a record's point in its face through the boolean's own read.
pub(crate) use contain::{
    CurvedPlacement, PlacementError, curved_face_placement, cycle_side, driver_face_stale,
    face_placement, loop_circle,
};
pub use discard::DiscardRow;
pub use fate::{SubstitutedFace, UncutFace};
// Crate-internal: tier 3's check 11 reads every vertex through the
// join's own predicate.
pub(crate) use edge_join::joinable_at_rest;
pub use edge_join::{
    EdgeJoin, JOIN_LEVER, JOIN_SUBJECT, JoinReading, JoinRefusal, JoinUndecided,
    is_conventional_vertex, join_covers, joinable_vertices, joined_cell, joined_edge,
};
pub use join::CompletedPolygonPair;
pub use nary::{
    Apart, FaceDescent, JudgedRow, MemberFace, MemberInputs, MemberLineage, NaryEnd, NaryJudgement,
    NaryMember, NaryOp, NaryRefusal, NaryRun, NaryStep, nary_boolean, nary_boolean_apart,
};
pub use ops::{
    BooleanBody, BooleanNaming, BooleanResult, BooleanResultKind, OperandKeys, Recharts,
    SettleError, Settler, boolean_op_with, intersect, subtract, union,
};
#[cfg(feature = "sweep-testing")]
pub use ops::{StagePoints, arm_output_stage_points, take_output_stage_points};
pub use plane_eq::{PlaneDesc, PlaneEqError, PlaneRelation, oriented_plane_eq};
pub(crate) use records::incident;
pub use records::{CarryRefusal, Contact, ContactKind, ContactPairError, ContactRecords};
#[cfg(feature = "sweep-testing")]
pub use reduce::PlantedDegradation;
pub use reduce::{SweepStrategy, SweepTrace};
pub use shell_witness::ShellOrientation;
pub use zip::Fusions;
#[cfg(feature = "sweep-testing")]
pub use zip::{SharedClass, swap_split_roles, take_shared_points};
// LIB-SEL2 (SELECT-DESIGN §3b; #304 review MINOR-1): THE flush-pair
// verify door — descriptions, oriented sources and the verification
// arm in one function, shared by verify-at-use and the detector's
// candidate-generation mode BY CONSTRUCTION.
pub use crate::side_reader::AimedPair;
pub use carrier_pair::{
    PairFace, PairUnread, carrier_pair_decided, carrier_pair_relation, face_carrier,
    flush_pair_relation, rest_locus,
};
pub use contact_verify::{rest_pair_verdict, tangent_pair_relation};
pub use solid_contain::{
    PointInSolidError, SolidContainment, SolidFaces, point_in_solid, point_in_solid_faces,
    point_in_solid_of,
};
pub use voids::{
    VoidContainment, VoidEvidence, VoidInsertError, VoidInserted, insert_void, insert_voids,
};

/// What one of this crate's decisions decides, in the words a flip
/// report states in place of the predicate's name (which is routing,
/// kept to `Debug`): a clause with no colon or dash of its own. `None`
/// for a predicate this crate has no words for yet, or does not own.
///
/// A flip report has only the name, so this lookup is by name; a
/// refusal carries its decision and reads the words there
/// ([`BooleanDecision::subject`]). Each row is the words of the one
/// decision every raise of that name is routed to, read from that
/// decision's closed type where it has one. A name raised under two
/// decisions has no words here, since neither decision's words are
/// true of it: `bool_contact_vertex` (the contact sweep's
/// [`BooleanDecision::VertexOnVertex`], and containment's boundary
/// pre-pass), `bool_contact_arc` (the split point on its circle, and
/// the same pre-pass) and `bool_sector_crescent` (the sectors'
/// coincidence where its crescent reads in band, and
/// [`BooleanDecision::ConicBend`] where a bound's bend carries no
/// bound).
#[must_use]
pub fn decision_words(predicate: &str) -> Option<&'static str> {
    if let Some(words) = crate::sector_shape::rung_words(predicate) {
        return Some(words);
    }
    if let Some(reading) = refusal_routes::PARALLELISMS
        .iter()
        .find(|reading| reading.predicate == predicate)
    {
        return Some(reading.coincide.subject());
    }
    Some(match predicate {
        "bool_point_in_solid_plane" => "which side of a face's plane a point lies on",
        "bool_point_in_solid_sphere" => "how far a point lies off a sphere face's carrier",
        "bool_point_in_solid_beside" => "whether a face lies to one side of a ray along its plane",
        "bool_point_in_solid_clearance" => {
            "how far a point lies off the carrier of a face a ray runs along"
        }
        // The coincidences these names decide.
        "bool_vertex_face_side" => Coincide::VertexOnFace.subject(),
        reduce::factored_crossing::FACTORED_PLANE_SIDE => Coincide::VertexOnCurvedFace.subject(),
        crate::vertex_section::VERTEX_SECTION_SIDE => crate::vertex_section::VERTEX_SECTION_SUBJECT,
        "bool_held_carrier_datum" => {
            "whether a face lies on the carrier of a face its pierce point's vertex bounds"
        }
        "bool_conic_face_plane_offset" => Coincide::EdgeOnPlane.subject(),
        "bool_line_cylinder_clearance"
        | "bool_line_face_carrier_offset"
        | slit::RIDER_ON_CURVED => Coincide::EdgeOnCurvedFace.subject(),
        slit::RIDER_ON_PLANE => Coincide::EdgeOnPlane.subject(),
        "bool_sector_within"
        | "bool_flank_offset"
        | "bool_wedge_reflex"
        | "bool_cone_arc"
        | "bool_cone_arc_span"
        | "bool_cone_within"
        | "bool_cone_facing"
        | "bool_cone_apart"
        | "bool_rider_wall_side"
        | slit::RIDER_WALL_BEND_ACROSS
        | "bool_rider_wall_coplanar"
        | "bool_touch_sheets_part"
        | "bool_sector_runs_into_bend"
        | "bool_fate_corner"
        | "bool_fate_corner_straddle"
        | "bool_fate_corner_bend"
        | "bool_fate_corner_bend_straddle"
        | "bool_rim_face_opposed" => Coincide::Sectors.subject(),
        "bool_cone_pointed" => "whether a corner's link leans to one side of its vertex",
        // The census's edge-edge reading, which the result gate takes of
        // the result and ∖ of an operand part it reverts unjoined
        // (`census::collinear_overlaps`).
        "pm_census_ee_parallel" => "whether two straight edges of one solid are parallel",
        "pm_census_ee_line_gap" => "whether two straight edges of one solid lie on one line",
        "pm_census_span_order" => "the order of two points along a straight edge",
        "pm_census_ee_overlap" => "whether two edges on one line overlap along a length",
        "pm_census_span_precision" => {
            "whether two edges on one line are short enough to read their ends uncharged at this \
             tolerance"
        }
        crate::census::SPAN_ORDER_CHARGED
        | crate::census::EE_OVERLAP_CHARGED
        | crate::census::SPAN_ROUNDING => BooleanDecision::LineSpanPrecision.subject(),
        slit::PARTING_ORDER => BooleanDecision::PartingOrder.subject(),
        // The line's wedges the result gate reads (`census::line_wedges`),
        // tier 3's pinch-corner reading under names of its own.
        crate::census::LINE_WEDGE_AFTER_LEAVING
        | crate::census::LINE_WEDGE_BEFORE_ARRIVING
        | crate::census::LINE_WEDGE_TURN
        | crate::census::LINE_WEDGE_STRAIGHT => BooleanDecision::LineWedges.subject(),
        "bool_plane_parallel" => PlaneRung::Parallel.subject(),
        "bool_plane_orient" => PlaneRung::Orientation.subject(),
        "carrier_cyl_axis_parallel" => refusal_routes::CYLINDER_AXES_PARALLEL,
        crate::query::POSE_UNIT_NORM
        | join::BOOL_GERM_PLANE_NORMAL
        | boxes::BOX_CYLINDER_AXIS
        | boxes::BOX_SPHERE_AXIS
        | boxes::BOX_SPHERE_SEAM_UNIT
        | separating::PAIR_AXIS
        | separating::PAIR_NORMAL
        | solid_contain::POINT_AXIS => geom_core::DIRECTION_LENGTH_SUBJECT,
        boxes::BOX_SPHERE_SEAM => "whether a sphere's seam direction is square to its polar axis",
        // `geom_core`'s lever on a dimensionless direction, which the
        // narrow phase's normals and the cylinder box's axis read.
        geom_core::UNIT_DIRECTION_ARM => {
            "whether the reach a direction is read over has any length"
        }
        separating::PAIR_GAP => {
            "how far apart two faces lie along a direction that turns with them"
        }
        "bool_pierce_normal_on_chart" | "bool_pierce_normal_on_cone" => {
            BooleanDecision::PierceOnFace.subject()
        }
        "cone_off_axis" => "whether a point of a cone face stands clear of the cone's axis",
        "cone_at_apex" => "whether a point of a cone face near its axis is the cone's apex",
        "foot_off_locus" => {
            "whether a point stands clear of the locus where a face's carrier has no one \
             nearest point: a cylinder's axis, a sphere's centre, or a cone's apex plane"
        }
        // `geom`'s torus convention, which the pierce point's normal
        // reads before it differentiates the torus.
        "torus_tube_positive" => TorusConvention::Tube.subject(),
        "ring_torus_convention" => TorusConvention::Ring.subject(),
        "split_edge_param_interior" | "split_conic_crossing_root" | "bool_wall_root_in_span" => {
            CrossingDecision::OnEdge.subject()
        }
        "split_conic_root_order" | "spline_composite_roots_apart" | "spline_face_roots_apart" => {
            CrossingDecision::Order.subject()
        }
        "spline_composite_root_at_end"
        | "spline_composite_root_at_crease"
        | "spline_face_root_at_end" => CrossingDecision::OnEdge.subject(),
        "spline_composite_root_on" => {
            "whether a crossing found along a spline edge lies on the face"
        }
        "spline_face_root_on" => "whether a meeting found on a spline face lies on it",
        "bool_point_past_every_reach" => "whether a point stands past every face of the solid",
        // The point-on-span reading's two halves, which the on-carrier
        // crossing and the boundary pass read: `geom`'s one-carrier
        // reading and `geom_brep::curve_pair`'s span reading.
        "curve_pair_span_end_chord"
        | "curve_pair_span_end_side"
        | "curve_pair_span_end_straddle" => CrossingDecision::OnEdge.subject(),
        "curve_carrier_gap"
        | "curve_carrier_clear"
        | "curve_carrier_straddle"
        | "curve_carrier_piece_clear"
        | "curve_carrier_piece" => "whether a point lies on an edge's carrier",
        // Every row of `geom_brep::curve_pair::pose`, by its prefix.
        p if p.starts_with("curve_pair_pose_") => "how two edges' carriers meet",
        "curve_pair_meet_same" | "curve_pair_meet_stretch" => "how two edges' carriers meet",
        "bool_face_disc_carrier"
        | "bool_contact_arc_end_vertex"
        | "bool_curved_contain_carrier"
        | "bool_wall_outline_reach"
        | "bool_wall_turn_true"
        | "bool_wrap_rim"
        | "bool_sphere_iso_meridian"
        | "bool_sphere_iso_rim"
        | "bool_sphere_trim_antipode"
        | "bool_sphere_trim_latitude"
        | "bool_sphere_trim_period"
        | "bool_sphere_trim_pole_end"
        | "bool_sphere_trim_pole_interior"
        | "bool_sphere_region_arm"
        | "bool_sphere_region_span"
        | "bool_sphere_region_at"
        | "bool_sphere_region_order"
        | "bool_sphere_region_cross"
        | "bool_sphere_region_roots_noise"
        | "bool_sphere_region_roots_coaxial"
        | "bool_sphere_region_roots_extreme"
        | "bool_sphere_region_roots_slack"
        | "bool_sphere_region_arc_span"
        | "bool_sphere_region_arc_straddle"
        | "bool_torus_region_component"
        | "bool_torus_region_family"
        | "bool_torus_region_span"
        | "bool_torus_region_at"
        | "bool_torus_region_corner"
        | "bool_torus_region_order"
        | "bool_torus_region_cross"
        | "bool_torus_region_roots_noise"
        | "bool_torus_region_roots_coaxial"
        | "bool_torus_region_roots_extreme"
        | "bool_torus_region_roots_slack"
        | "bool_torus_region_arc_span"
        | "bool_torus_region_arc_straddle"
        | "bool_torus_region_spiric_end"
        | "bool_torus_region_spiric_clear"
        | "bool_torus_region_spiric_on"
        | "bool_torus_region_spiric_leaf"
        | "bool_torus_frame_radius"
        | "bool_ruled_region_frame"
        | "bool_ruled_region_nappe"
        | "bool_ruled_region_component"
        | "bool_ruled_region_collinear"
        | "bool_ruled_region_span"
        | "bool_ruled_region_at"
        | "bool_ruled_region_end"
        | "bool_ruled_region_corner"
        | "bool_ruled_region_order"
        | "bool_ruled_region_cross"
        | "bool_ruled_region_roots_noise"
        | "bool_ruled_region_roots_coaxial"
        | "bool_ruled_region_roots_extreme"
        | "bool_ruled_region_roots_slack"
        | "bool_ruled_region_arc_span"
        | "bool_ruled_region_arc_straddle"
        | "bool_ruled_region_edge_length"
        | "bool_ruled_region_edge"
        | "bool_ray_cone_nappe"
        | "bool_cone_partial_reach"
        | "point_in_loop_segment"
        | "point_in_loop_boundary"
        | "point_in_loop_side"
        | "point_in_loop_advance"
        | "point_in_loop_arm"
        | "point_in_arc_loop_segment"
        | "point_in_arc_loop_boundary"
        | "point_in_arc_loop_side"
        | "point_in_arc_loop_advance"
        | "point_in_arc_loop_arm"
        | "point_in_arc_loop_reach"
        | "point_in_arc_loop_conic_span"
        | "point_in_arc_loop_conic_end"
        | "point_in_arc_loop_conic_straddle"
        | "point_in_arc_loop_conic_window"
        | "point_in_arc_loop_conic_disc"
        | "point_in_arc_loop_conic_advance"
        | "point_in_arc_loop_spiric_end"
        | "point_in_arc_loop_spiric_clear"
        | "point_in_arc_loop_spiric_on"
        | "point_in_arc_loop_spiric_leaf"
        | "point_in_arc_loop_spiric_side"
        | "point_in_arc_loop_spiric_turn"
        | "point_in_arc_loop_spiric_advance"
        | "point_in_arc_loop_spline_end"
        | "point_in_arc_loop_spline_clear"
        | "point_in_arc_loop_spline_on"
        | "point_in_arc_loop_spline_leaf"
        | "point_in_arc_loop_spline_side"
        | "point_in_arc_loop_spline_turn"
        | "point_in_arc_loop_spline_advance" => BooleanDecision::CONTAINMENT_UNNAMED.subject(),
        spline_region::CARRIER => "how far a point lies off a spline face's carrier",
        spline_region::ARM | spline_region::TRIM => {
            "whether a point of a spline face's carrier lies clear of the face's trim"
        }
        _ => return None,
    })
}

/// Which regularized boolean is being computed — threaded through the
/// classifier because on-case lumping (Eq. 15.3) is op-dependent.
///
/// `Hash`/`Ord` are derived because this is also the DOCUMENT layer's
/// operation (re-exported, never re-minted), where a node's fields are
/// keyed and ordered. Ordering is declaration order and carries no
/// meaning — nothing may read it as a ranking of the operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BooleanOp {
    /// A ∪* B.
    Union,
    /// A ∩* B.
    Intersect,
    /// A ∖* B.
    Subtract,
}

impl BooleanOp {
    /// **Every operation this enum names**, in declaration order — the
    /// one enumeration, owned where the exhaustive matches live.
    ///
    /// A list cannot be derived from a match in safe Rust, so SOMEONE
    /// writes it by hand; the only question is where. Written here, it
    /// sits in the crate whose exhaustive matches over `BooleanOp`
    /// (`finish::kept_side`, `tables::lump_keeps_one`) fail to compile on
    /// a fourth operation — so the author adding one is already in this
    /// module with the list in front of them, and the
    /// `all_is_every_operation` census below puts a second visit right
    /// beside it. **Neither forces the edit**: what they force is that
    /// the author is here and has to decide, and the census's own doc
    /// measures how far short of forcing it stops. A copy in a
    /// downstream crate gets not even that. The enum is closed, so a
    /// consumer's own exhaustive match does fence THAT consumer; but
    /// nothing ties an array literal to a variant list, so a downstream
    /// list of three stays three with no error anywhere and no author
    /// standing over it.
    ///
    /// So this is the list downstream reads instead of writing its own
    /// — `crates/editor-core`'s wire table and the viewer's operation
    /// buttons both iterate it — and the ordering caveat on the type
    /// holds for it too: it is declaration order, and a consumer that
    /// renders it renders an arbitrary order, not a ranked one.
    pub const ALL: &'static [BooleanOp] =
        &[BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];
}

/// Which operand a key belongs to (keys are body-lineage-scoped;
/// cross-body records must say which arena they index — F9).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operand {
    /// The first operand (left of the op).
    A,
    /// The second operand.
    B,
}

/// A trilean side code against the *other* solid's boundary — the
/// boolean analogue of `PlaneSide`, derived from `enters_material`
/// (module docs): `Enters ⇒ In`, `Exits ⇒ Out`, `Tangent ⇒ On`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SideCode {
    /// Definitely inside the other solid's material.
    In,
    /// Tangent/coincident (resolved away by reclassification).
    On,
    /// Definitely outside.
    Out,
}

impl SideCode {
    /// The transition partner (In ↔ Out); `On` has none.
    pub fn opposite(self) -> Self {
        match self {
            Self::In => Self::Out,
            Self::Out => Self::In,
            Self::On => Self::On,
        }
    }
}

/// **One operand edge piece's side of the other operand, beside a
/// vertex the classification read**: the piece of operand `operand`'s
/// edge `edge` that starts or ends at `vertex`, and which side of the
/// other operand's boundary it lies on just beside it.
///
/// Two passes write rows, and only the first records a decision the
/// classification made:
/// - The vertex-on-face pass records every edge at the piercing vertex,
///   read against the pierced face: the side codes that pass decided
///   and acted on, recorded as it read them (D5's birth row), before
///   any reclassification lumps them.
/// - The vertex-vertex pass records every edge at each vertex in a
///   pair, read through one cone of the other operand at the vertex's
///   point: every partner's faces, and the face the vertex touches where
///   it touches one, its pierce's side codes standing for the edges'
///   sides of that face (`vtxfac::cone_classes`). The classification
///   itself decides sector pairs, not edges, so these rows are a
///   measurement of their own, taken beside it from the same sectors,
///   not a record of what it decided. A vertex whose edges the cone
///   does not read records none: in one pair alone it names its partner
///   ([`UnreadPartner`]); touching a face, or beside several partners,
///   the boolean refuses ([`BooleanError::VertexReadTwice`]).
///
/// `In` and `On` both lie in the other operand's closed body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgePieceClass {
    /// The operand whose edge the piece is.
    pub operand: Operand,
    /// The vertex, in that operand's CLONE keys (as
    /// [`BooleanNaming::reduction_contacts`]).
    pub vertex: VertexKey,
    /// The operand edge the piece lies on, in the operand's own keys:
    /// the reduction's splits read back to the edge they divided.
    pub edge: EdgeKey,
    /// Whether the piece starts at `vertex` as the edge is stored, so it
    /// lies after the vertex along the edge; `false` where it ends there.
    pub starts: bool,
    /// The piece's side of the other operand's boundary.
    pub class: SideCode,
}

/// **A vertex in one pair whose partner the classification read
/// nothing of** (`vtxfac::cone_classes`): the vertex publishes no
/// [`EdgePieceClass`], and the result is built all the same. Its edges
/// meet the other operand only at the point, through its partner's
/// neighbourhood, which the cone at the point reads none of.
/// A reader that needs a class there reads why from here rather than
/// finding a row missing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnreadPartner {
    /// The operand whose vertex it is.
    pub operand: Operand,
    /// The vertex, in that operand's CLONE keys (as [`EdgePieceClass`]),
    /// each null-edge copy of it listed too.
    pub vertex: VertexKey,
    /// Its partner, the other operand's vertex, in that operand's clone
    /// keys.
    pub partner: VertexKey,
    /// What the partner's neighbourhood is.
    pub neighbourhood: UnreadNeighbourhood,
}

/// Why an [`UnreadPartner`]'s partner read nothing, as the reading of
/// the other solid's cone at the point found it (`vtxfac::cone_classes`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnreadNeighbourhood {
    /// The partner's faces, as one cone at the vertex's point, do not
    /// pair up at their bounds on decided readings, or some edge of the
    /// vertex reads in band of it (`link::Cone::reads_tied`'s `None`).
    ConeUnread,
}

/// A coincident vertex pair in the reduction's working set (`sonvv`):
/// vertex `a` of operand A and vertex `b` of operand B, each in its
/// operand clone's keys ([`ReductionContacts`]). Carried within one
/// operand ([`OperandRecords`]), both are that operand's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VvContact {
    /// The A-side vertex (key into the A clone).
    pub a: VertexKey,
    /// The B-side vertex (key into the B clone).
    pub b: VertexKey,
}

/// A vertex on a face in the reduction's working set (`sonva`/`sonvb`):
/// `vertex`, of the operand the containing list names, lies within
/// `face` of the other operand ([`ReductionContacts`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VfContact {
    /// The piercing vertex.
    pub vertex: VertexKey,
    /// The pierced face of the other body.
    pub face: FaceKey,
}

/// A vertex resting on an edge's interior, within one operand carried
/// back into an op ([`OperandRecords`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VeContact {
    /// The resting vertex.
    pub vertex: VertexKey,
    /// The edge whose interior holds it.
    pub edge: crate::entity::EdgeKey,
}

/// Two edges whose interiors meet: within one operand carried back into
/// an op, both that operand's ([`OperandRecords`]); in the reduction's
/// working set, `a` an edge of A and `b` one of B, each in its operand
/// clone's keys ([`ReductionContacts`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EeContact {
    /// One edge.
    pub a: crate::entity::EdgeKey,
    /// The other.
    pub b: crate::entity::EdgeKey,
}

/// An edge whose interior meets a face's, within one operand carried
/// back into an op ([`OperandRecords`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EfContact {
    /// The edge.
    pub edge: crate::entity::EdgeKey,
    /// The face.
    pub face: FaceKey,
}

/// Two faces whose interiors meet along a tangent line: within one
/// operand carried back into an op, both that operand's
/// ([`OperandRecords`]); in the reduction's working set, `a` a face of A
/// and `b` one of B, each in its operand clone's keys
/// ([`ReductionContacts`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FfContact {
    /// One face.
    pub a: FaceKey,
    /// The other.
    pub b: FaceKey,
}

/// One edge split a boolean's reduction made in an operand clone:
/// `parent` keeps its key and its leading span, from its start to the
/// new `vertex`, and `child` runs from `vertex` to the parent's old
/// end. In split order, these rows are the lineage a carried
/// `(vertex, edge)` record follows onto the piece it rests on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EdgeSplit {
    /// The operand whose clone was split.
    pub(crate) operand: Operand,
    /// The split edge, which keeps the leading piece.
    pub(crate) parent: crate::entity::EdgeKey,
    /// The vertex the split minted.
    pub(crate) vertex: VertexKey,
    /// The trailing piece.
    pub(crate) child: crate::entity::EdgeKey,
    /// The cell of the other operand's clone whose crossing minted
    /// `vertex`: the face, edge or vertex the split edge met there.
    pub(crate) by: Cell,
}

/// **Where a Boolean reduction minted each vertex off its operands'
/// cells**, each operand's: on an edge it split ([`EdgeSplit`]), or
/// inside a face where a held pierce stands
/// (`vtxfac::faces_holding_pierce`). The splits are in split order, their
/// `(child, parent)` rows checked where each is recorded
/// ([`crate::lineage::EdgeDivisions`]), and beside each row the vertex
/// the split minted between them. A split mints its child, so the rows
/// cannot cycle, and the walk from a piece back to the operand edge it
/// lies on ([`Self::root`]) ends.
#[derive(Clone, Debug)]
pub(crate) struct Mints {
    divisions: [crate::lineage::EdgeDivisions; 2],
    /// Each operand's vertices minted on edges, one per row of its
    /// divisions and in the same order, each beside the cell of the other
    /// operand whose crossing minted it ([`EdgeSplit::by`]):
    /// [`Self::push`] is the one writer of both, and pushes the vertex
    /// only once the division's row is in, so the `i`-th vertex is the
    /// one minted between the `i`-th row's two edges (asserted where
    /// they are read together, [`Self::of`]).
    vertices: [Vec<(VertexKey, Cell)>; 2],
    /// Each operand's vertices minted inside faces, each with its face.
    in_faces: [Vec<(VertexKey, FaceKey)>; 2],
}

impl Default for Mints {
    fn default() -> Self {
        let of = |operand| {
            crate::lineage::EdgeDivisions::new(crate::lineage::LineageRows::EdgeDivisions(Some(
                operand,
            )))
        };
        Self {
            divisions: [of(Operand::A), of(Operand::B)],
            vertices: [Vec::new(), Vec::new()],
            in_faces: [Vec::new(), Vec::new()],
        }
    }
}

impl Mints {
    /// Records `split`.
    ///
    /// # Errors
    ///
    /// The division list's refusal where `split`'s child is not one the
    /// split minted: an earlier split names it.
    pub(crate) fn push(&mut self, split: EdgeSplit) -> Result<(), crate::lineage::RowRefusal> {
        let side = usize::from(split.operand == Operand::B);
        self.divisions[side].push((split.child, split.parent))?;
        self.vertices[side].push((split.vertex, split.by));
        Ok(())
    }

    /// `operand`'s splits, in the order made: `(parent, vertex, child)`.
    fn of(
        &self,
        operand: Operand,
    ) -> impl Iterator<Item = (crate::entity::EdgeKey, VertexKey, crate::entity::EdgeKey)> + '_
    {
        let side = usize::from(operand == Operand::B);
        debug_assert_eq!(
            self.divisions[side].rows().len(),
            self.vertices[side].len(),
            "one minted vertex per edge division row"
        );
        self.divisions[side]
            .rows()
            .iter()
            .zip(&self.vertices[side])
            .map(|(&(child, parent), &(vertex, _))| (parent, vertex, child))
    }

    /// The edge of `operand`'s input the piece `edge` of its clone lies
    /// on: read back through every split, each child to the parent it was
    /// split from. Total, since the rows cannot cycle.
    pub(crate) fn root(
        &self,
        operand: Operand,
        edge: crate::entity::EdgeKey,
    ) -> crate::entity::EdgeKey {
        self.divisions[usize::from(operand == Operand::B)].root(edge)
    }

    /// Records `v`, minted inside face `face` of `operand`'s clone.
    pub(crate) fn push_in_face(&mut self, operand: Operand, v: VertexKey, face: FaceKey) {
        self.in_faces[usize::from(operand == Operand::B)].push((v, face));
    }

    /// The face of `operand`'s clone `v` was minted inside, where it was.
    pub(crate) fn minted_in(&self, operand: Operand, v: VertexKey) -> Option<FaceKey> {
        self.in_faces[usize::from(operand == Operand::B)]
            .iter()
            .find(|&&(w, _)| w == v)
            .map(|&(_, face)| face)
    }

    /// The edge of `operand`'s clone a split minted `v` on, where one did.
    pub(crate) fn minted_on(
        &self,
        operand: Operand,
        v: VertexKey,
    ) -> Option<crate::entity::EdgeKey> {
        self.of(operand)
            .find(|&(_, vertex, _)| vertex == v)
            .map(|(parent, _, _)| parent)
    }

    /// The cell of the other operand's clone whose crossing minted `v`
    /// on an edge of `operand`'s, where a split minted it.
    pub(crate) fn minted_by(&self, operand: Operand, v: VertexKey) -> Option<Cell> {
        self.vertices[usize::from(operand == Operand::B)]
            .iter()
            .find(|&&(w, _)| w == v)
            .map(|&(_, by)| by)
    }

    /// The keys of `edge`'s pieces after `operand`'s splits, in order
    /// along it (a split keeps the parent's leading span, so its child is
    /// the piece right after the parent), and every vertex a split minted
    /// on it.
    pub(crate) fn pieces(
        &self,
        operand: Operand,
        edge: crate::entity::EdgeKey,
    ) -> (Vec<crate::entity::EdgeKey>, Vec<VertexKey>) {
        let mut keys = vec![edge];
        let mut minted = Vec::new();
        for (parent, vertex, child) in self.of(operand) {
            if let Some(i) = keys.iter().position(|&k| k == parent) {
                keys.insert(i + 1, child);
                minted.push(vertex);
            }
        }
        (keys, minted)
    }
}

/// One cell of a body: what a contact record names, two at a time
/// ([`Contact`]).
///
/// The order is vertex < edge < face, then key: a record's canonical
/// order, so its kind reads off its two cells' dimensions
/// ([`ContactKind`]).
///
/// It is also the live cell a point lies inside: where a Boolean's
/// result settles a vertex a later stage deleted without moving its
/// point ([`crate::boolean::Settler::settle`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    /// A vertex.
    Vertex(VertexKey),
    /// An edge.
    Edge(crate::entity::EdgeKey),
    /// A face.
    Face(FaceKey),
}

/// **The boolean reduction's working contact set** (Mäntylä's ON-sets
/// `sonvv`, `sonva`, `sonvb`, and the touches along a line the
/// classification reads), each row citing the reduction's own
/// decisions: operand-tagged, every key in its operand clone's arena.
/// It is the reduction's vocabulary, read by operand by the
/// classification and the insertion, and it is not a body's record:
/// the result's carry maps it onto the result's cells and drops the
/// tags ([`ContactRecords`]). Deterministic discovery order,
/// deduplicated.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReductionContacts {
    /// Coincident vertex pairs (`sonvv`).
    pub vv: Vec<Cited<VvContact>>,
    /// Vertices of A on faces of B (`sonva`).
    pub a_on_b: Vec<Cited<VfContact>>,
    /// Vertices of B on faces of A (`sonvb`).
    pub b_on_a: Vec<Cited<VfContact>>,
    /// Coincident line edges of A and B whose interiors meet with no
    /// germ between them, both staying on the result's boundary, each
    /// citing the vertex records at its two ends (`touch_rows`). Written
    /// by the classification, so empty at the sweep.
    pub ee: Vec<Cited<EeContact>>,
    /// Faces of A and B tangent along a line through both interiors,
    /// each citing the glue door's tangency decision
    /// ([`verify_tangency`]). Written at the door, whatever the op: the
    /// carry keeps a row where both faces survive.
    pub ff: Vec<Cited<FfContact>>,
}

impl ReductionContacts {
    /// Every row as its two operand-tagged cells and its citations, in
    /// one fixed order: the vertex pairs (A's vertex, B's), then A's
    /// vertices on B's faces, then B's on A's, each vertex first, then
    /// the edge pairs (A's edge, B's), then the face pairs (A's face,
    /// B's).
    pub fn rows(&self) -> impl Iterator<Item = ([(Operand, Cell); 2], &Cites)> + '_ {
        let vv = self.vv.iter().map(|c| {
            (
                [
                    (Operand::A, Cell::Vertex(c.a)),
                    (Operand::B, Cell::Vertex(c.b)),
                ],
                &c.cites,
            )
        });
        fn vf(
            side: Operand,
            other: Operand,
            list: &[Cited<VfContact>],
        ) -> impl Iterator<Item = ([(Operand, Cell); 2], &Cites)> + '_ {
            list.iter().map(move |c| {
                (
                    [(side, Cell::Vertex(c.vertex)), (other, Cell::Face(c.face))],
                    &c.cites,
                )
            })
        }
        let ee = self.ee.iter().map(|c| {
            (
                [(Operand::A, Cell::Edge(c.a)), (Operand::B, Cell::Edge(c.b))],
                &c.cites,
            )
        });
        let ff = self.ff.iter().map(|c| {
            (
                [(Operand::A, Cell::Face(c.a)), (Operand::B, Cell::Face(c.b))],
                &c.cites,
            )
        });
        vv.chain(vf(Operand::A, Operand::B, &self.a_on_b))
            .chain(vf(Operand::B, Operand::A, &self.b_on_a))
            .chain(ee)
            .chain(ff)
    }
}

/// **One operand's own contact records, carried into an op**: the
/// coincidences WITHIN that operand, keyed in its arena. The op remaps
/// survivors into result keys (the strict drop rule discovered records
/// take: a record whose entity was consumed drops), each citing the
/// operand's record it is ([`Backing::Carried`](crate::Backing::Carried)).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperandRecords {
    /// Coincident vertex pairs within the operand.
    pub vv: Vec<CarriedRecord<VvContact>>,
    /// Vertex-on-face rests within the operand.
    pub vf: Vec<CarriedRecord<VfContact>>,
    /// Vertex-on-edge records within the operand.
    pub ve: Vec<CarriedRecord<VeContact>>,
    /// Edge-edge records within the operand.
    pub ee: Vec<CarriedRecord<EeContact>>,
    /// Edge-face records within the operand.
    pub ef: Vec<CarriedRecord<EfContact>>,
    /// Face-face records within the operand.
    pub ff: Vec<CarriedRecord<FfContact>>,
}

/// A record of the operand's own list carried back in, and its index
/// in that list: the record the op's result cites for it
/// ([`Backing::Carried`](crate::Backing::Carried)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarriedRecord<R> {
    /// The record's cells.
    pub contact: R,
    /// Its index in the operand's records ([`ContactRecords::rows`]).
    pub record: u32,
}

impl OperandRecords {
    /// True iff nothing is carried.
    pub fn is_empty(&self) -> bool {
        let Self {
            vv,
            vf,
            ve,
            ee,
            ef,
            ff,
        } = self;
        vv.is_empty()
            && vf.is_empty()
            && ve.is_empty()
            && ee.is_empty()
            && ef.is_empty()
            && ff.is_empty()
    }
}

/// **What a boolean is handed besides its two operands**: each
/// operand's own contact records, carried into the result.
///
/// Every key is validated at the op door (live) — a dangling key is a
/// typed refusal ([`BooleanError::InvalidInput`]), never a silent drop.
#[derive(Clone, Debug, Default)]
pub struct BooleanInputs {
    /// Operand A's own contact records, carried into the result.
    pub records_a: OperandRecords,
    /// Operand B's own contact records, carried into the result.
    pub records_b: OperandRecords,
}

/// **A face pair a boolean glued on its own decision**: a face of
/// each operand, in that operation's operand keys, the class its
/// margin's relation glued them under, and that margin; for a tangency
/// verified along a line, that line, as the operation derived and
/// placed it. Only the kernel mints one ([`BooleanBody::verdicts`]),
/// from the face-pair rows of the operation's own coincidences: it is
/// what an n-ary union's pairwise judgement hands its fold steps (DM4).
#[derive(Clone, Copy, Debug)]
pub struct GlueVerdict<T: Real = f64> {
    a: FaceKey,
    b: FaceKey,
    class: GlueClass,
    margin: MarginDiag,
    line: Option<DecidedLine<T>>,
}

/// A tangent line as decided: `origin + t·dir`.
pub(crate) type DecidedLine<T> = (Point3<T>, geom_core::Vec3<T>);

impl<T: Real> GlueVerdict<T> {
    /// The A-operand face.
    pub fn a(&self) -> FaceKey {
        self.a
    }

    /// The B-operand face.
    pub fn b(&self) -> FaceKey {
        self.b
    }

    /// The decided relation.
    pub fn relation(&self) -> crate::Relation {
        self.class.relation()
    }

    /// **This verdict, given to a fold step**: the pair of `at`, an
    /// accumulation face whose lineage holds the judged `held` face, and
    /// the judged face of the other operand, under the judged class
    /// (every class is symmetric in its two faces), carrying the
    /// decision: its margin, and a tangency's line. The n-ary door
    /// ([`nary`]) passes each such accumulation face, read
    /// through the lineage it advances itself.
    pub(crate) fn given_at(&self, held: Operand, at: FaceKey) -> GluedPair<T> {
        let other = match held {
            Operand::A => self.b,
            Operand::B => self.a,
        };
        GluedPair {
            given: Some(GivenDecision {
                margin: self.margin,
                line: self.line,
            }),
            ..GluedPair::new(at, other, self.class)
        }
    }

    /// The verdicts `rows` record: each row relating an A-operand face
    /// and a B-operand face under a relation that glues, in row order,
    /// a tangency with the line `lines` holds for its pair.
    pub(crate) fn of_rows(
        rows: &[crate::Coincidence],
        lines: &std::collections::BTreeMap<(FaceKey, FaceKey), DecidedLine<T>>,
    ) -> Vec<Self> {
        rows.iter()
            .filter_map(|row| {
                let (a, b) = row.face_pair()?;
                let class = row.relation.glued_class()?;
                Some(Self {
                    a,
                    b,
                    class,
                    margin: row.margin,
                    line: match class {
                        GlueClass::Tangent => lines.get(&(a, b)).copied(),
                        GlueClass::Rest | GlueClass::Continuation => None,
                    },
                })
            })
            .collect()
    }
}

/// **The decision a given pair carries** ([`GlueVerdict::given_at`]):
/// the margin the deciding operation read, and a tangency's line as it
/// derived and placed it, which a fold step places against its pieces'
/// faces, deciding nothing of the carriers.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GivenDecision<T: Real> {
    pub(crate) margin: MarginDiag,
    pub(crate) line: Option<DecidedLine<T>>,
}

/// **The pairs one boolean glues**, and the records it carries: the
/// operation's own decisions ([`glue::decided_glue`]) or the
/// pairs it was given (a fold step of [`nary`]), in arena keys, each
/// pair under the class its relation makes it.
#[derive(Clone, Debug)]
pub(crate) struct Glue<T: Real> {
    /// Cross-operand glued face pairs, each with its class:
    /// classification treats a `Rest` or continuation pair's carriers as
    /// the same carrier ([`mod@carrier_eq`] rung 2), and the result's
    /// merge stage glues the pair's surviving coplanar-adjacent
    /// material (N3 `Merged`).
    pub(crate) pairs: Vec<GluedPair<T>>,
    /// Operand A's records, carried.
    pub(crate) records_a: OperandRecords,
    /// Operand B's records, carried.
    pub(crate) records_b: OperandRecords,
    /// Whether the operation decides its own coincidences or is given
    /// them ([`Verdicts`]).
    pub(crate) verdicts: Verdicts,
    /// The pairs the glue door read in band and left unglued, each with
    /// the reading it could not decide ([`InBandPair`]).
    pub(crate) in_band: Vec<InBandPair>,
}

/// **A pair the glue door read in band** over both faces and left
/// unglued: its faces and what the carrier ladder could not decide
/// there, which a corner meeting the pair quotes where it binds
/// ([`unglued_corner`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InBandPair {
    /// The A-operand face.
    pub(crate) a: FaceKey,
    /// The B-operand face.
    pub(crate) b: FaceKey,
    /// The undecided reading: a rung of the plane ladder in band, or a
    /// coincidence measure undecided with the orientation already
    /// decided.
    pub(crate) reading: carrier_eq::CarrierEqError,
}

/// **Where a boolean's coincidence verdicts come from.** An operation
/// decides every cross-operand coincidence it meets by its margin and
/// glues on Zero ([`Verdicts::Decided`]), except a step of an n-ary
/// union's fold, whose verdicts are the union's pairwise judgement's
/// (DM4), handed in ([`Verdicts::Given`]): a fold step decides no
/// carrier pair again.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Verdicts {
    /// The operation decides its coincidences.
    #[default]
    Decided,
    /// The given pairs are the operation's verdicts, and it decides
    /// none of its own.
    Given,
}

impl<T: Real> Glue<T> {
    /// Nothing glued and nothing carried.
    pub(crate) fn none() -> Self {
        Self {
            pairs: Vec::new(),
            records_a: OperandRecords::default(),
            records_b: OperandRecords::default(),
            verdicts: Verdicts::Decided,
            in_band: Vec::new(),
        }
    }

    /// The glue `inputs` carries, with `given`'s pairs under their
    /// classes, or none, to be decided ([`glue::decided_glue`]).
    pub(crate) fn of(inputs: &BooleanInputs, given: Option<&[GluedPair<T>]>) -> Self {
        let BooleanInputs {
            records_a,
            records_b,
        } = inputs;
        let (pairs, verdicts) = match given {
            None => (Vec::new(), Verdicts::Decided),
            Some(given) => (given.to_vec(), Verdicts::Given),
        };
        Self {
            pairs,
            records_a: records_a.clone(),
            records_b: records_b.clone(),
            verdicts,
            in_band: Vec::new(),
        }
    }
}

/// **One glued cross-operand face pair and its class**: a pair an
/// operation decided, or one an n-ary union's fold step was given
/// ([`GlueVerdict::given_at`]); a given pair carries the decision it
/// was given on, and a one-carrier pair the glue door decided the
/// margin it decided it on, which the declaration door records.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GluedPair<T: Real = f64> {
    /// The A-operand face.
    pub(crate) a: FaceKey,
    /// The B-operand face.
    pub(crate) b: FaceKey,
    /// The coincidence the pair glues as.
    pub(crate) class: GlueClass,
    /// The decision a given pair carries; none for a pair the operation
    /// decided.
    pub(crate) given: Option<GivenDecision<T>>,
    /// The margin the glue door decided a one-carrier pair on
    /// ([`carrier_pair::carrier_pair_decided`]); none for a tangency,
    /// which the declaration door reads along its locus, and for a
    /// given pair.
    pub(crate) decided: Option<MarginDiag>,
}

impl<T: Real> GluedPair<T> {
    /// A glued pair, under `class`, carrying no decision of its own.
    pub(crate) fn new(a: FaceKey, b: FaceKey, class: impl Into<GlueClass>) -> Self {
        Self {
            a,
            b,
            class: class.into(),
            given: None,
            decided: None,
        }
    }

    /// The pair `fa`, `fb` under `class` as the glue door reads it: a
    /// one-carrier class on the margin the face-pair door decides it on,
    /// a tangency as it stands. For the rows that hand a pair to the
    /// declaration door themselves.
    ///
    /// # Panics
    ///
    /// Where the face-pair door does not decide the pair one carrier
    /// under `class`'s relation on a margin: the glue door would not glue
    /// it so.
    #[cfg(test)]
    #[allow(clippy::panic)]
    pub(crate) fn read(
        (a, fa): (&Body<T>, FaceKey),
        (b, fb): (&Body<T>, FaceKey),
        class: GlueClass,
        band: Band,
    ) -> Self
    where
        T: Decide,
    {
        if !class.is_one_carrier() {
            return Self::new(fa, fb, class);
        }
        match carrier_pair::carrier_pair_decided(a, fa, b, fb, band) {
            Ok(Ok((relation, Some(margin)))) if Some(relation) == class.carrier_relation() => {
                Self::decided(fa, fb, class, margin)
            }
            other => panic!("the glue door glues no {class:?} pair on {other:?}"),
        }
    }

    /// A one-carrier pair the glue door decided under `class` on
    /// `margin`.
    pub(crate) fn decided(a: FaceKey, b: FaceKey, class: GlueClass, margin: MarginDiag) -> Self {
        Self {
            decided: Some(margin),
            ..Self::new(a, b, class)
        }
    }

    /// Whether `row` restates this given pair's decision, as a fold step
    /// records it for a face–face record to cite: its two faces, its
    /// class's relation, its margin.
    pub(crate) fn restated_by(&self, row: &crate::Coincidence) -> bool {
        self.given.is_some_and(|given| {
            row.face_pair() == Some((self.a, self.b))
                && row.relation.glued_class() == Some(self.class)
                && row.margin == given.margin
        })
    }
}

/// The classification stages' symmetric declared-face-pair index
/// (crate-internal): normalized `(A face, B face)` rows KEYED to their
/// class.
///
/// A map, not a set: the classification stages must be able to ask
/// "declared as WHAT", and a set can only answer "declared at all" —
/// which is the same erasure the payload change exists to remove. A
/// duplicate pair declared under two classes is a caller bug refused
/// at the door by an EXPLICIT check in `validate_declarations`, so the
/// last write here is never reached with disagreeing classes. (That
/// check exists because this sentence was measured vacuous: it held
/// only while every non-`Rest` class refused wholesale, and would have
/// become silent last-write-wins the day the op grew a second class
/// arm.)
#[derive(Debug)]
pub(crate) struct GluedPairs<T: Real> {
    map: std::collections::BTreeMap<(FaceKey, FaceKey), GlueClass>,
    /// What the declaration door VERIFIED, `(A face, B face)`.
    verified: VerifiedGlue<T>,
    /// The ORDERED one-sided cover ([`Self::cover`]): `(parent,
    /// target)`, where the parent face's carrier is certified to lie in
    /// one closed side of the target's, and which side. C4 states the
    /// certificate one way, so the key is directed.
    one_sided: std::collections::BTreeMap<(OperandFace, OperandFace), CoverSide>,
    /// Each declared pair's consumed extent, measured on the operands
    /// at rest ([`carrier_pair::pair_extent`]), `(A face, B face)`. Every
    /// declared pair has one: the production constructor
    /// ([`Self::build`]) refuses a pair whose extent does not read.
    extent: std::collections::BTreeMap<(FaceKey, FaceKey), carrier_pair::PairExtent<T>>,
    /// What the glue door read of each pair it left in band,
    /// `(A face, B face)` ([`Glue::in_band`]).
    in_band: std::collections::BTreeMap<(FaceKey, FaceKey), carrier_eq::CarrierEqError>,
    /// The operation's section table ([`section_table`]).
    pub(crate) sections: section_table::SectionTable<T>,
}

impl<T: Real> Default for GluedPairs<T> {
    fn default() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
            verified: VerifiedGlue::default(),
            one_sided: std::collections::BTreeMap::new(),
            extent: std::collections::BTreeMap::new(),
            in_band: std::collections::BTreeMap::new(),
            sections: section_table::SectionTable::default(),
        }
    }
}

/// The refusal of a declared face whose consumed extent cannot be read
/// ([`carrier_pair::PairUnread::Extent`]), at rest: an input condition, named
/// by the operand whose face it is.
pub(crate) fn unreadable_extent(face: carrier_pair::PairFace) -> BooleanError {
    BooleanError::InvalidInput {
        operand: match face {
            carrier_pair::PairFace::First => Operand::A,
            carrier_pair::PairFace::Second => Operand::B,
        },
        what: "a glued face's consumed extent cannot be read (its box has no claim to make, \
               or its boundary cannot be walked)",
    }
}

/// The Boolean's refusal of a coincidence between `pair` that did not
/// decide: in band, a sliver ([`Coincide::Carriers`], with what the
/// door read of the pair's declaration as `read`); poisoned,
/// [`BooleanError::PoisonedCarrierDatum`] ([`readable_coincidence`]).
pub(crate) fn undecided_coincidence(
    coincidence: CoincidenceMeasure,
    pair: [(Operand, FaceKey); 2],
    read: DeclarationRead,
) -> BooleanError {
    match readable_coincidence(coincidence, pair) {
        Ok(diag) => BooleanError::coincidence(Coincide::Carriers, read, diag),
        Err(poisoned) => poisoned,
    }
}

/// A coincidence decided Zero at a corner site between a pair the
/// operation's glue door did not glue: the faces read one carrier at
/// the corner's arm and not over their own extent, a near-coincidence
/// no glue was decided for ([`Coincide::Carriers`]).
pub(crate) fn unglued_coincidence(
    margin: MarginDiag,
    read: DeclarationRead,
    band: Band,
) -> BooleanError {
    BooleanError::coincidence(
        Coincide::Carriers,
        read,
        Indeterminate {
            margin,
            band,
            predicate: Some("carrier_unglued_coincidence"),
            terminal_sliver: false,
        },
    )
}

/// **An unglued pair a corner reads one carrier, refused on the reading
/// that binds it**: the one whose clearing the operation waits on.
///
/// - `bounds`, the sector's bounds' reading off the other carrier
///   ([`sectors::BoolSector::bounds_off`]), where a smaller tolerance a run
///   admits decides it: that tolerance reads the sector off the carrier
///   and the corner with it ([`Coincide::Sectors`]).
/// - Else the glue door's in-band reading over both faces
///   ([`GluedPairs::door_refusal`]): the sector lies on the other carrier
///   at every tolerance, and only a glue clears the pair. The door's
///   reading never offers a tolerance itself (a margin in band shows no
///   size a smaller tolerance passes), so it is never weighed against the
///   bounds.
/// - Else `corner`, the corner's own refusal, on its standing off two
///   carriers it lies on.
///
/// Both arms reach here only for a planar pair: an undeclared curved
/// pair keeps its frontier refusal before the ladder runs.
pub(crate) fn unglued_corner<T: Real>(
    declared: &GluedPairs<T>,
    pair: (Operand, FaceKey, Operand, FaceKey),
    senses: Option<CarrierRelation>,
    bounds: Option<Indeterminate>,
    corner: impl FnOnce() -> BooleanError,
) -> BooleanError {
    match bounds.filter(Indeterminate::offers_tolerance) {
        Some(diag) => BooleanError::coincidence(
            Coincide::Sectors,
            sectors_read(declared, pair, senses),
            diag,
        ),
        None => declared.door_refusal(pair, senses).unwrap_or_else(corner),
    }
}

/// **What a sector's coplanar refusal reads of the pair's declaration**
/// ([`Coincide::Sectors`]): the one-carrier class the pair's `senses` make
/// it against a planar face. A caller
/// passes `None` where no class settles it: a tilt decided off, or a
/// pierced face that is not a plane.
pub(crate) fn sectors_read<T: Real>(
    declared: &GluedPairs<T>,
    pair: (Operand, FaceKey, Operand, FaceKey),
    senses: Option<CarrierRelation>,
) -> DeclarationRead {
    let admitted = senses.and_then(GlueClass::of_senses);
    declared.read(&[pair], Coincide::Sectors, admitted.as_slice())
}

/// An undecided coincidence measure between `pair` as the Boolean
/// reads it: its diagnostics where it read in band, and
/// [`BooleanError::PoisonedCarrierDatum`] where a datum it compares is
/// not finite. Every door of the Boolean that reads a
/// [`CoincidenceMeasure`] takes poison through here.
pub(crate) fn readable_coincidence(
    coincidence: CoincidenceMeasure,
    pair: [(Operand, FaceKey); 2],
) -> Result<Indeterminate, BooleanError> {
    match coincidence {
        CoincidenceMeasure::Unreadable(diag) => {
            Err(BooleanError::PoisonedCarrierDatum { pair, diag })
        }
        CoincidenceMeasure::Undecided(diag) => Ok(diag),
    }
}

/// A face tagged with the operand it belongs to, ordered A before B;
/// the one-sided cover's key half.
type OperandFace = (bool, FaceKey);

/// `(o, f)` as a cover key half: `true` for A.
fn tagged(o: Operand, f: FaceKey) -> OperandFace {
    (o == Operand::A, f)
}

/// **Which tangencies along a curve certify a GLOBAL side** — the one
/// table C4's strut and rim sources share. C4's cover records an
/// edge's endpoints "only when the edge's parent carrier is certified to
/// lie in one closed side of that carrier". A structural tangency (an
/// edge described `TangentIntersection`) and a glued tangency along a
/// rim each certify that two carriers are tangent along a CURVE — a
/// local fact.
/// The cover needs the global one: `parent`'s carrier in one closed side
/// of `partner`'s everywhere. Tangency along a curve fixes the geometry
/// enough that local implies global exactly for these pairs, both ways
/// round:
///
/// - **plane and cylinder**: the cylinder is convex, so it lies in the
///   closed half-space of any plane tangent to it; every point of that
///   plane is at least the radius from the axis, the plane's distance to
///   it, so the plane lies in the cylinder's closed exterior.
/// - **two cylinders**: tangent along a curve only with parallel axes,
///   along a ruling — externally each lies outside the other, internally
///   the smaller lies in the larger and the larger outside the smaller.
/// - **sphere and cylinder**: tangent along a circle only centred on the
///   axis with the cylinder's radius, so the sphere lies in the closed
///   solid cylinder, and the cylinder keeps at least the radius from the
///   centre.
/// - **sphere and torus**: tangent along a circle in two ways. Centred
///   on the torus's axis, at a centre `c` every point of the centre
///   circle `C` is equidistant from, at `d`: the torus is `{x : dist(x,
///   C) = r}`, so every point of it lies between `d − r` and `d + r` of
///   `c`, on one side of a sphere of radius `d ∓ r`, and that sphere
///   keeps at least `r` from `C`, outside the open solid torus. Or
///   centred ON the centre circle with the minor radius `r` (the
///   ball-ended elbow, tangent along a meridian): every torus point is
///   `r` from `C`, so at least `r` from the centre, outside the open
///   ball, and the sphere lies within `r` of `C`, inside the closed
///   solid torus. Both are a global side.
/// - **plane and torus**: tangent along a circle only at a top or bottom
///   plane, with the torus in one half-space and the plane at least the
///   minor radius from the centre circle.
///
/// Every other pair gives no cover, and its crossing stays a typed
/// frontier. Two tori, or a torus and a cylinder, joined G1 along a
/// meridian diverge quadratically past it, so each crosses the other's
/// continuation. **That exclusion is conservative**, since the table
/// answers by KIND: a cylinder coaxial with a torus, of radius `R ± r`,
/// is tangent to it along an equator and is a global side. A kind row
/// cannot tell the coaxial cylinder from the tube chain, and admitting
/// the chain would cover a crossing, so the row stays out and the
/// coaxial case keeps its typed frontier. A cone's tangent plane leaves its second nappe on the
/// other side. A spline (STEP adoption, blends) carries no global
/// convexity. Two planes, two spheres or a plane and a sphere are never
/// tangent along a curve on distinct carriers.
///
/// **The source column.** A glued tangency along a rim certifies every
/// row above (a line from the witness lane is a global side by the
/// lane's own contract, both ways, and needs no row). A structural
/// tangency (the strut source) certifies only plane and cylinder, both
/// ways: that is what a fixture exercises. The other rows are as true
/// for a strut as for a rim, but nothing yet reaches them through one,
/// so they wait for a witness (filed: `the-strut-cover-on-cylinder-pairs`).
/// The old strut row for plane and sphere is not restored: those are
/// never tangent along a curve on distinct carriers.
fn tangency_certifies_side(
    source: TangencySource,
    parent: geom::SurfaceKind,
    partner: geom::SurfaceKind,
) -> bool {
    use geom::SurfaceKind::{Cylinder, Plane, Sphere, Torus};
    let global = matches!(
        (parent, partner),
        (Plane, Cylinder | Torus)
            | (Cylinder, Plane | Cylinder | Sphere)
            | (Sphere, Cylinder | Torus)
            | (Torus, Plane | Sphere)
    );
    global
        && match source {
            TangencySource::Rim => true,
            TangencySource::Strut => {
                matches!((parent, partner), (Plane, Cylinder) | (Cylinder, Plane))
            }
        }
}

/// Which certificate a tangency along a curve comes from
/// ([`tangency_certifies_side`]'s source column).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TangencySource {
    /// An operand edge described `TangentIntersection`.
    Strut,
    /// A glued tangency along a rim circle.
    Rim,
}

/// **What the one-sided cover certifies** about a parent face's carrier
/// against a target face's: which closed side of the target's implicit
/// residual ([`geom_brep::implicit_residual`]) holds it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CoverSide {
    /// One carrier: the parent lies ON the target's carrier, residual ≡ 0.
    On,
    /// One closed side, of this sign: read off the parent face's
    /// boundary once the certificate (a kind fact) says one side holds
    /// it all.
    Off(Sign),
    /// One closed side, but no definite reading said which.
    Unread,
}

impl CoverSide {
    /// Whether a point of the parent whose target residual decided
    /// `sign` is consistent with the certificate: on the carrier always,
    /// off it only on the certified side. A point on the other side is a
    /// crossing the certificate rules out, so it keeps its door.
    pub(crate) fn admits(self, sign: Sign) -> bool {
        match (self, sign) {
            (_, Sign::Zero) => true,
            (Self::Off(side), sign) => side == sign,
            (Self::On | Self::Unread, _) => false,
        }
    }
}

/// The side of `target`'s carrier the parent face lies on: the sign of
/// the target's implicit residual at the parent's boundary arc ends and
/// midpoints, required to agree, and every sample decided (on the
/// carrier, or definitely off it). Only consulted where a certificate
/// already says one closed side holds the whole parent carrier, so any
/// definite reading names it; readings that disagree contradict the
/// certificate and leave the side unread.
///
/// **Reading the side off boundary samples is sound HERE and nowhere
/// the side is only local.** The seam's departure read once sampled a
/// face's boundary the same way, and a face whose outline reached round
/// to the other bank of its locus fooled it: there the question was
/// which side the face leaves on AT the locus, a local fact the far
/// boundary says nothing about. This question is global by the time it
/// is asked: [`tangency_certifies_side`] has certified that every point
/// of the parent's CARRIER lies in one closed side of the target's,
/// so every point of the face, boundary samples included, lies there
/// too, and one definite sample names the side. A face cannot bulge to
/// the other side between samples because no point of its carrier is
/// there.
fn read_cover_side<T: Decide>(
    parent: &Body<T>,
    face: FaceKey,
    target: &geom::Surface<T>,
    band: Band,
) -> CoverSide {
    let (mut pos, mut neg) = (false, false);
    for (_, curve) in rim_wedge::face_boundary_arcs(parent, face) {
        let (t0, t1) = curve.params();
        for t in [t0, (t0 + t1) * T::from_f64(0.5), t1] {
            let r = geom_brep::implicit_residual(target, curve.carrier().eval(t));
            match crate::validate::decide("cover_side", Margin::of(r), band) {
                Ok(Sign::Positive) => pos = true,
                Ok(Sign::Negative) => neg = true,
                Ok(Sign::Zero) => {}
                // An in-band sample is a reading that did not decide;
                // the side stays unread rather than resting on the
                // samples that did.
                Err(_) => return CoverSide::Unread,
            }
        }
    }
    match (pos, neg) {
        (true, false) => CoverSide::Off(Sign::Positive),
        (false, true) => CoverSide::Off(Sign::Negative),
        _ => CoverSide::Unread,
    }
}

/// What [`verify_glued_contacts`] certified, per declared pair
/// `(A face, B face)`: the certificates, not the claims.
#[derive(Debug)]
pub(crate) struct VerifiedGlue<T: Real> {
    /// `Rest` and continuation pairs the carrier ladder called ONE
    /// carrier, with the sense the class demands.
    pub(crate) one_carrier: std::collections::BTreeSet<(FaceKey, FaceKey)>,
    /// Tangencies the witness lane verified, with what the cover reads
    /// of each.
    pub(crate) tangent: std::collections::BTreeMap<(FaceKey, FaceKey), TangentCert>,
    /// The verified tangencies whose line locus runs through both faces'
    /// interiors: the reduction's face–face rows
    /// ([`ReductionContacts::ff`]).
    pub(crate) touches: Vec<GluedTouch>,
    /// The line each tangency verified along a line was decided along
    /// ([`GlueVerdict`]).
    pub(crate) lines: std::collections::BTreeMap<(FaceKey, FaceKey), DecidedLine<T>>,
    /// The one-carrier pairs a margin decided, one row each in
    /// declaration order: a declaration's verification is a value
    /// decision, so its glue is recorded like any other
    /// ([`crate::coincidence`]).
    pub(crate) coincidences: Vec<crate::Coincidence>,
}

impl<T: Real> Default for VerifiedGlue<T> {
    fn default() -> Self {
        Self {
            one_carrier: std::collections::BTreeSet::new(),
            tangent: std::collections::BTreeMap::new(),
            touches: Vec::new(),
            lines: std::collections::BTreeMap::new(),
            coincidences: Vec::new(),
        }
    }
}

/// A glued tangency through both faces' interiors
/// ([`VerifiedGlue::touches`]): its faces, the margin that decided it,
/// and the index of the row recording that decision in
/// [`VerifiedGlue::coincidences`], which the face–face record cites.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GluedTouch {
    pub(crate) a: FaceKey,
    pub(crate) b: FaceKey,
    pub(crate) margin: MarginDiag,
    pub(crate) row: usize,
}

impl<T: Decide> GluedPairs<T> {
    /// The index over `decls`, with the door's certificates, the
    /// one-sided cover they and the operands' structural tangencies
    /// derive ([`Self::one_sided`]), and each declared pair's consumed
    /// extent measured on the operands `a` and `b` at rest. Mid-operation
    /// a face is a piece of its rest self, so that extent still encloses
    /// it, while its own box may no longer read (null scaffolding on its
    /// boundary).
    ///
    /// # Errors
    ///
    /// [`unreadable_extent`] for a declared face whose extent does not
    /// read.
    pub(crate) fn build(
        decls: &Glue<T>,
        verified: VerifiedGlue<T>,
        a: &Body<T>,
        b: &Body<T>,
        sections: section_table::SectionTable<T>,
        band: Band,
    ) -> Result<Self, BooleanError> {
        let mut pairs = Self::index(decls, verified);
        pairs.sections = sections;
        pairs.extent = decls
            .pairs
            .iter()
            .map(|d| {
                carrier_pair::pair_extent(a, d.a, b, d.b, band)
                    .map(|extent| ((d.a, d.b), extent))
                    .map_err(unreadable_extent)
            })
            .collect::<Result<_, _>>()?;
        // A structural tangency on either operand, between a strut face
        // and a partner the door verified one carrier with a face of
        // the other operand. Each direction is its own certificate,
        // admitted only for the kinds where a tangency along a curve is
        // a global side ([`tangency_certifies_side`]): strut face →
        // other face when the strut face lies on one side of the
        // partner, other face → strut face when the partner lies on
        // one side of the strut face.
        let (struts_a, struts_b) = (tangent_struts(a), tangent_struts(b));
        for &(fa, fb) in &pairs.verified.one_carrier {
            for (o, struts, k, target) in [
                (Operand::A, &struts_a, fa, (Operand::B, fb)),
                (Operand::B, &struts_b, fb, (Operand::A, fa)),
            ] {
                for &(f, f_kind, partner, partner_kind) in struts {
                    if partner != k {
                        continue;
                    }
                    let (strut_face, other_face) = (tagged(o, f), tagged(target.0, target.1));
                    // The strut face's carrier on one side of the
                    // partner's, which IS the other face's carrier.
                    if tangency_certifies_side(TangencySource::Strut, f_kind, partner_kind) {
                        pairs
                            .one_sided
                            .entry((strut_face, other_face))
                            .or_insert(CoverSide::Unread);
                    }
                    // The partner's carrier (the other face's) on one
                    // side of the strut face's.
                    if tangency_certifies_side(TangencySource::Strut, partner_kind, f_kind) {
                        pairs
                            .one_sided
                            .entry((other_face, strut_face))
                            .or_insert(CoverSide::Unread);
                    }
                }
            }
        }
        // Which side each one-sided certificate holds its parent on.
        let body = |(is_a, _): OperandFace| if is_a { a } else { b };
        for (&(parent, target), side) in &mut pairs.one_sided {
            if *side != CoverSide::Unread {
                continue;
            }
            if let Some(surface) = body(target)
                .get_face(target.1)
                .and_then(|f| body(target).get_surface(f.surface))
            {
                *side = read_cover_side(body(parent), parent.1, surface, band);
            }
        }
        Ok(pairs)
    }
}

impl<T: Real> GluedPairs<T> {
    /// [`Self::build`] over a one-carrier certificate set alone: no
    /// verified `Tangent` and no operand struts.
    #[cfg(test)]
    pub(crate) fn without_struts(
        decls: &Glue<T>,
        one_carrier: std::collections::BTreeSet<(FaceKey, FaceKey)>,
    ) -> Self {
        Self::index(
            decls,
            VerifiedGlue {
                one_carrier,
                ..VerifiedGlue::default()
            },
        )
    }

    /// The index over `decls` with the door's certificates and the
    /// cover they alone derive: [`Self::build`] without the operands'
    /// structural tangencies and without the extents.
    fn index(decls: &Glue<T>, verified: VerifiedGlue<T>) -> Self {
        // Both directions for a verified one-carrier pair (one carrier:
        // each lies ON the other) and for a tangency along a line (the
        // witness lane's kinds, plane × cylinder along a ruling and
        // parallel cylinders, each lie in one closed side of the other —
        // the lane's own contract in `geom_brep::tangent_locus`). A
        // tangency along a rim gives each direction its kinds certify
        // ([`tangency_certifies_side`]). Which side an off-carrier
        // certificate holds is read in [`Self::build`], where the bodies
        // are.
        let both = |(fa, fb): (FaceKey, FaceKey), side| {
            let (x, y) = (tagged(Operand::A, fa), tagged(Operand::B, fb));
            [((x, y), side), ((y, x), side)]
        };
        let one_sided: std::collections::BTreeMap<(OperandFace, OperandFace), CoverSide> = verified
            .one_carrier
            .iter()
            .flat_map(|&pair| both(pair, CoverSide::On))
            .chain(verified.tangent.iter().flat_map(|(&(fa, fb), cert)| {
                let (x, y) = (tagged(Operand::A, fa), tagged(Operand::B, fb));
                let (ka, kb) = cert.kinds;
                [
                    (cert.line || tangency_certifies_side(TangencySource::Rim, ka, kb))
                        .then_some(((x, y), CoverSide::Unread)),
                    (cert.line || tangency_certifies_side(TangencySource::Rim, kb, ka))
                        .then_some(((y, x), CoverSide::Unread)),
                ]
                .into_iter()
                .flatten()
            }))
            .collect();
        Self {
            map: decls.pairs.iter().map(|d| ((d.a, d.b), d.class)).collect(),
            verified,
            one_sided,
            extent: std::collections::BTreeMap::new(),
            in_band: decls
                .in_band
                .iter()
                .map(|p| ((p.a, p.b), p.reading))
                .collect(),
            sections: section_table::SectionTable::default(),
        }
    }

    /// **The glue door's refusal of a pair it read in band**, in its
    /// words at the pair's door ([`PlaneDoor::OnPair`]); `None` where the
    /// door decided the pair or never read it. `senses` is the
    /// orientation the caller read of the two faces, which a rung's door
    /// admits a class by.
    fn door_refusal(
        &self,
        pair: (Operand, FaceKey, Operand, FaceKey),
        senses: Option<CarrierRelation>,
    ) -> Option<BooleanError> {
        let (o1, f1, o2, f2) = pair;
        let reading = *self.in_band.get(&Self::key(o1, f1, o2, f2)?)?;
        Some(match reading {
            carrier_eq::CarrierEqError::Escalated { rung, diag } => {
                BooleanError::plane_identity(rung, self.on_pair_door(pair, senses), diag)
            }
            carrier_eq::CarrierEqError::Undecided {
                coincidence,
                relation,
            } => undecided_coincidence(
                coincidence,
                [(o1, f1), (o2, f2)],
                self.carriers_read(pair, relation),
            ),
        })
    }

    /// `(f1, f2)` as the `(A face, B face)` key, for a cross-operand
    /// pair; `None` for a same-operand one, which is never declared here
    /// (operand-internal coplanarity is the producing op's merge, not
    /// this op's).
    fn key(o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> Option<(FaceKey, FaceKey)> {
        match (o1, o2) {
            (Operand::A, Operand::B) => Some((f1, f2)),
            (Operand::B, Operand::A) => Some((f2, f1)),
            _ => None,
        }
    }

    /// The (operand-tagged) declared pair's consumed extent, its faces
    /// in the order asked, as [`Self::build`] measured it.
    ///
    /// # Errors
    ///
    /// [`BooleanError::ClassificationInvariant`] for a pair with no
    /// measured extent: every declared pair has one, so a miss is a
    /// pair the caller took for declared that is not.
    pub(crate) fn consumed(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Result<carrier_eq::ConsumedExtent<T>, BooleanError> {
        Self::key(o1, f1, o2, f2)
            .and_then(|key| Some(self.extent.get(&key)?.consumed()))
            .ok_or(BooleanError::ClassificationInvariant {
                what: "a declared face pair's consumed extent was not measured at rest",
            })
    }

    /// The ball of [`Self::consumed`].
    ///
    /// # Errors
    ///
    /// As [`Self::consumed`].
    pub(crate) fn reach_of(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Result<geom_brep::ExtentBall<T>, BooleanError> {
        Ok(self.consumed(o1, f1, o2, f2)?.reach)
    }

    /// Whether the (operand-tagged) pair is a `Rest` or continuation
    /// declaration the door VERIFIED as one carrier — the certificate,
    /// not the claim.
    pub(crate) fn verified_one_carrier(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> bool {
        Self::key(o1, f1, o2, f2).is_some_and(|k| self.verified.one_carrier.contains(&k))
    }

    /// **The crossing layer's one-sided cover** (C4): is the PARENT
    /// face `f1`'s carrier certified to lie in one closed side of the
    /// target `f2`'s, and which side? Directed: the answer for
    /// `(f1, f2)` says nothing about `(f2, f1)`.
    ///
    /// The certificate has exactly these sources, and no value reading:
    /// a verified `Rest` or continuation (residual ≡ 0), a tangency
    /// along a line (the witness lane's separation invariant), a
    /// tangency along a rim, or a structural tangency — an edge described
    /// `TangentIntersection` — on EITHER operand, from the parent to a
    /// face the door verified one carrier with the target; the last two
    /// only on kinds where a tangency along a curve is a global side
    /// ([`tangency_certifies_side`]). WHICH side is read off the
    /// parent's boundary once the certificate says one side holds it
    /// ([`CoverSide`]).
    pub(crate) fn cover(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Option<CoverSide> {
        if o1 == o2 {
            return None;
        }
        self.one_sided
            .get(&(tagged(o1, f1), tagged(o2, f2)))
            .copied()
    }

    /// The coincidence the (operand-tagged) face pair is declared under,
    /// if any.
    pub(crate) fn class_of(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Option<GlueClass> {
        Self::key(o1, f1, o2, f2).and_then(|k| self.map.get(&k).copied())
    }

    /// The pairs the declaration door verified one carrier, each with
    /// the relation its class demands: a `Rest` contact opposed, a
    /// continuation aligned (rung 2 of the coincidence ladder).
    pub(crate) fn settled(&self) -> impl Iterator<Item = SettledPair> + '_ {
        self.verified.one_carrier.iter().filter_map(|&(a, b)| {
            let relation = self.map.get(&(a, b))?.carrier_relation()?;
            Some(SettledPair { a, b, relation })
        })
    }

    /// **The relation the (operand-tagged) pair was glued under**, as
    /// the carrier ladder spells it: the glue door's decision, read
    /// from its record and measured nowhere again
    /// (`[coincidence.booleans]`). `None` for a pair not glued one
    /// carrier. A pair is glued one carrier where the door verified it
    /// so, the set [`Self::settled`] reads too, so the two never part.
    pub(crate) fn glued_relation(
        &self,
        o1: Operand,
        f1: FaceKey,
        o2: Operand,
        f2: FaceKey,
    ) -> Option<CarrierRelation> {
        self.verified_one_carrier(o1, f1, o2, f2)
            .then(|| self.class_of(o1, f1, o2, f2))
            .flatten()
            .and_then(GlueClass::carrier_relation)
    }

    /// Whether the pair is declared `Tangent`.
    pub(crate) fn glues_tangent(&self, o1: Operand, f1: FaceKey, o2: Operand, f2: FaceKey) -> bool {
        self.class_of(o1, f1, o2, f2) == Some(GlueClass::Tangent)
    }
}

/// Every structural tangency of `body`, as `(face, its kind, other
/// face, its kind)` both ways: the two faces across an edge described
/// `TangentIntersection` of exactly their two surfaces.
fn tangent_struts<T: Real>(
    body: &Body<T>,
) -> Vec<(FaceKey, geom::SurfaceKind, FaceKey, geom::SurfaceKind)> {
    let mut out = Vec::new();
    for (_, edge) in body.edges() {
        let (Some(f1), Some(f2)) = (
            body.face_of_half_edge(edge.he_plus),
            body.face_of_half_edge(edge.he_minus),
        ) else {
            continue;
        };
        let (Some(s1), Some(s2)) = (
            body.get_face(f1).map(|f| f.surface),
            body.get_face(f2).map(|f| f.surface),
        ) else {
            continue;
        };
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            continue;
        };
        if let geom_brep::EdgeDescription::TangentIntersection { pair, .. } = curve.description()
            && f1 != f2
            && *pair == geom_brep::SurfacePair::new(s1, s2)
            && let (Some(k1), Some(k2)) = (body.get_surface(s1), body.get_surface(s2))
        {
            let (k1, k2) = (k1.kind(), k2.kind());
            out.push((f1, k1, f2, k2));
            out.push((f2, k2, f1, k1));
        }
    }
    out
}

/// The cell of one operand a section germ lies in. A germ ray running
/// along a real edge of the operand lies in both faces that edge
/// bounds, so a face does not name it; the edge does. The sweep splits
/// an edge at every crossing, so both ends of a section segment along
/// an edge name the same key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locus {
    /// Inside this face, off every edge of it.
    InFace(FaceKey),
    /// Along this edge.
    OnEdge(crate::entity::EdgeKey),
}

impl Locus {
    /// The edge a germ along an edge lies on; `None` inside a face.
    pub fn edge(self) -> Option<crate::entity::EdgeKey> {
        match self {
            Self::OnEdge(e) => Some(e),
            Self::InFace(_) => None,
        }
    }
}

/// The **germ** a null-edge half faces (F9 as data, PR 5): every
/// surviving crossing record — a section-polygon edge emanating from
/// the classified vertex — lies on the intersection line of one A-face
/// and one B-face, and each null edge's two halves are spliced facing
/// its two germs. The joining step matches halves across sites by the
/// germ's per-operand [`Locus`] (equal on both operands, opposite record
/// parity — the book's he1↔he2 "opposite roles" test carried as data),
/// never by slot position or dynamic face lookups.
#[derive(Clone, Copy, Debug)]
pub struct HalfGerm<T: Real> {
    /// The half-edge facing this germ.
    pub he: crate::entity::HalfEdgeKey,
    /// The A-body face whose carrier the join's chord lanes section
    /// against: the face of the sector the germ was attributed to.
    pub a_face: FaceKey,
    /// The B-body face, likewise.
    pub b_face: FaceKey,
    /// The cell of A the germ lies in: the germ's identity on A.
    pub a_locus: Locus,
    /// The cell of B the germ lies in.
    pub b_locus: Locus,
    /// The germ's outgoing direction along the line (unit; points away
    /// from the site toward the polygon edge's other end) — the datum
    /// the joining's mutual-facing test decides on (`bool_join_facing`).
    pub dir: geom_core::Vec3<T>,
    /// The loop of its face pair's section the germ rides, decided once
    /// where the germ is minted ([`Branch`]).
    pub branch: Branch,
}

/// **Which loop of its face pair's section a germ rides.** On a pair the
/// operation's section table holds, the loop is a component of the
/// pair's carried section, by its index: the component the germ's site
/// has its foot on ([`geom_brep::CarriedSection::foot`]). On the pairs
/// read by a frame, a section of two loops — a plane cutting a torus in
/// two circles or two ovals, two equal cylinders with intersecting axes
/// in two bisector-plane ellipses — tells its loops apart by a margin at
/// a point of the section (`ring_path::LoopSplit`). Either is decided
/// once, at the germ's site where the germ is minted, and every reader
/// of the germ takes its loop from here: the join's matching and frames,
/// and each side's chord. Both operands' germs of one slot carry the
/// same branch, read at the A site.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Branch {
    /// The section is one loop, loops sharing one frame, or straight;
    /// or the germ runs along an edge, whose own curve it rides.
    Only,
    /// The loop on the split's positive side.
    Positive,
    /// The loop on its negative side.
    Negative,
    /// The component of the pair's carried section with this index.
    Component(usize),
}

/// One minted boolean null edge with its F9 side attribute (below ≙ IN
/// copy, above ≙ OUT copy — identity derived from the F3 chain, never
/// slot position) and its two germ facings.
#[derive(Clone, Copy, Debug)]
pub struct BoolNullEdgeRecord<T: Real> {
    /// Which operand's clone the keys index.
    pub operand: Operand,
    /// The vertex this edge was minted at: the classified vertex whose
    /// neighborhood minted it, or, for a dangling null edge whose
    /// segment another's holds whole at a shared vertex, that one's tip.
    pub at_vertex: VertexKey,
    /// The null edge.
    pub edge: EdgeKey,
    /// F9 attribute: `below_end` = IN-side copy, `above_end` = OUT-side.
    pub attr: crate::null::NullEdge,
    /// A dangling strut (single-sector double crossing, or a pierced-
    /// face ring null edge).
    pub dangling: bool,
    /// The two germ facings ([`HalfGerm`]), in mint order (the from-
    /// germ first).
    pub germs: [HalfGerm<T>; 2],
}

/// **One operand's null-edge copies**: the vertices its null edges join,
/// each on one point by construction. A vertex's copies are read
/// transitively, since a dangling null edge can hang at another's tip.
#[derive(Clone, Debug, Default)]
pub(super) struct NullCopies(std::collections::BTreeMap<VertexKey, Vec<VertexKey>>);

impl NullCopies {
    /// `operand`'s null edges among `null_edges`.
    pub(super) fn of_operand<T: Real>(
        null_edges: &[BoolNullEdgeRecord<T>],
        operand: Operand,
    ) -> Self {
        let mut map: std::collections::BTreeMap<VertexKey, Vec<VertexKey>> =
            std::collections::BTreeMap::new();
        for r in null_edges.iter().filter(|r| r.operand == operand) {
            let (u, w) = (r.attr.below_end, r.attr.above_end);
            map.entry(u).or_default().push(w);
            map.entry(w).or_default().push(u);
        }
        Self(map)
    }

    /// `v`, then every vertex null edges join it to, transitively, in
    /// the order the walk meets them.
    pub(super) fn of(&self, v: VertexKey) -> Vec<VertexKey> {
        let mut out = vec![v];
        let mut i = 0;
        while let Some(&at) = out.get(i) {
            for &c in self.0.get(&at).into_iter().flatten() {
                if !out.contains(&c) {
                    out.push(c);
                }
            }
            i += 1;
        }
        out
    }
}

/// The site a corresponding null-edge pair was minted at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairSite {
    /// A vertex-vertex classification at this contact record.
    VertexVertex(VvContact),
    /// A vertex of A piercing a face of B.
    VertexAOnFaceB(VfContact),
    /// A vertex of B piercing a face of A.
    VertexBOnFaceA(VfContact),
}

/// **Explicit cross-body correspondence** (F9/F12): the A-side null
/// edge and the B-side null edge minted together for one section-
/// polygon vertex — as data, never correlated array order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullEdgePairRecord {
    /// The A-clone null edge.
    pub a_edge: EdgeKey,
    /// The B-clone null edge.
    pub b_edge: EdgeKey,
    /// Where the pair was minted.
    pub site: PairSite,
}

/// A pierced-face ring insertion record: the lone ring vertex minted
/// inside the pierced face at the pierce point (the `vtxfacclassify`
/// delta 3 — see `vtxfac` module docs for the designed Euler sequence).
/// One ring serves every pierce of its face at its point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PierceRingRecord {
    /// Which operand's clone holds the pierced face.
    pub operand: Operand,
    /// The pierced face.
    pub face: FaceKey,
    /// The ring vertex at the pierce point.
    pub ring_vertex: VertexKey,
    /// The piercing vertices the ring serves, in the other operand's
    /// clone keys.
    pub pierces: Vec<VertexKey>,
    /// Each two of those pierces' germs that lie along one direction (two
    /// cones touching along a ray in the face), an order the ring broke
    /// by their chords (`vtxfac::germ_order`), leaving a corner of no
    /// angle between them: the strut halves facing them, the germ first
    /// clockwise about the face's outward normal first.
    pub ties: Vec<[HalfEdgeKey; 2]>,
}

/// **Two pierces of one face decided one point** (`vtxfac::ring_groups`):
/// the other operand's vertices `a` and `b`, which one ring serves. Where
/// they sit on two point keys the ledger records the decision, its
/// margin kept, beside a vertex-vertex record of that operand's
/// (`ops::ledger`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RingTie {
    /// The piercing operand.
    pub operand: Operand,
    pub a: VertexKey,
    pub b: VertexKey,
    /// The margin `bool_contact_vertex` decided Zero.
    pub margin: geom_core::MarginDiag,
}

impl<T: Real> BooleanReduction<T> {
    /// The ring serving `vertex`'s pierce of the `pierced` operand's
    /// face, as an index into [`Self::pierce_rings`].
    fn ring_of(&self, pierced: Operand, vertex: VertexKey) -> Option<usize> {
        self.pierce_rings
            .iter()
            .position(|r| r.operand == pierced && r.pierces.contains(&vertex))
    }

    /// Whether two pierce sites are served by one ring
    /// ([`PierceRingRecord::pierces`]).
    pub(crate) fn one_ring(&self, a: PairSite, b: PairSite) -> bool {
        let ring = |site| match site {
            PairSite::VertexAOnFaceB(c) => self.ring_of(Operand::B, c.vertex),
            PairSite::VertexBOnFaceA(c) => self.ring_of(Operand::A, c.vertex),
            _ => None,
        };
        ring(a).is_some_and(|r| ring(b) == Some(r))
    }
}

/// **Whether two points are one vertex** (`bool_contact_vertex`, the
/// distance between them): the margin that decided them one, `None`
/// decided apart, the escalation when the band cannot say. A distance
/// has no negative side: a magnitude.
pub(crate) fn one_vertex_at<T: Decide>(
    p: Point3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Option<MarginDiag>, Indeterminate> {
    let distance = geom_core::Margin::norm3(p - q);
    let (magnitude, margin) =
        geom_core::k_stats::decide_magnitude_reported("bool_contact_vertex", distance, band)?;
    Ok((magnitude == geom_core::k_stats::Magnitude::Zero).then_some(margin))
}

/// **The vertex of `body` at `p`**, the first in arena order (D9) that
/// [`one_vertex_at`] reads at `p`, with the margin that put it there.
///
/// # Errors
///
/// The band's escalation where a vertex sits in its gap.
pub(crate) fn vertex_at<T: Decide>(
    body: &crate::body::Body<T>,
    p: Point3<T>,
    band: Band,
) -> Result<Option<(crate::entity::VertexKey, MarginDiag)>, Indeterminate> {
    for (v, q) in body.vertex_points() {
        if let Some(margin) = one_vertex_at(p, q, band)? {
            return Ok(Some((v, margin)));
        }
    }
    Ok(None)
}

/// The result of [`boolean_reduce`]: both operands' annotated clones
/// plus every record the PR 5 joining step consumes.
#[derive(Debug)]
pub struct BooleanReduction<T: Real> {
    /// The op the classification was performed for.
    pub op: BooleanOp,
    /// Each operand's slit and ring tips, A's then B's, read at rest
    /// ([`crate::sector_shape::SlitTips`]).
    pub(crate) slit_tips: [crate::sector_shape::SlitTips; 2],
    /// The annotated A clone (crossings split, null edges inserted).
    pub a: Body<T>,
    /// The annotated B clone.
    pub b: Body<T>,
    /// The reduction's working contact set (the three ON-sets). Each
    /// row cites the reduction's own decisions (`pending`), which the
    /// carry onto the result renumbers onto the result's coincidences.
    pub contacts: ReductionContacts,
    /// Every null edge minted, both operands, insertion order.
    pub null_edges: Vec<BoolNullEdgeRecord<T>>,
    /// Each operand's null-edge copies, checked as each null edge was
    /// recorded ([`NullEdges::extend`]).
    pub(crate) copies: [crate::lineage::NullEdgeCopies; 2],
    /// The cross-body correspondence pairs.
    pub null_pairs: Vec<NullEdgePairRecord>,
    /// Pierced-face ring insertions.
    pub pierce_rings: Vec<PierceRingRecord>,
    /// `(A face, B face)`, in clone keys, sorted and deduplicated: each
    /// pair of coincident faces whose Eq. 15.3 lump keeps one copy of
    /// a region they share and drops the other, so the result holds
    /// either face's region through the other ([`fate`]).
    pub covered: Vec<(FaceKey, FaceKey)>,
    /// What the covered pairs owe the dropped copies: the kept edges
    /// along which a dropped face's fate changes, and the dropped
    /// sectors held wholly ([`fate`]).
    pub(crate) fates: fate::Fates<T>,
    /// The `Rest` declarations `(A face, B face)` the declaration door
    /// verified one carrier with opposed senses: the REST-contact pairs
    /// the containment fallback's extent scans exempt, read rather than
    /// re-verified.
    pub(crate) rest_contacts: Vec<(FaceKey, FaceKey)>,
    /// Every cross-operand face pair the declaration door verified ONE
    /// carrier on the operands at rest (rung 2), sorted and
    /// deduplicated. The whole-shell `On` verdict reads these
    /// (`shell_witness::on_verdict`).
    pub(crate) coincident: Vec<SettledPair>,
    /// Every vertex the reduction minted off its operands' cells, both
    /// clones ([`Mints`]).
    pub(crate) mints: Mints,
    /// Every operand edge piece the sector passes classified beside a
    /// vertex ([`EdgePieceClass`]), sorted.
    pub edge_classes: Vec<EdgePieceClass>,
    /// Every vertex in one pair whose partner the sector passes read
    /// nothing of ([`UnreadPartner`]), sorted.
    pub unread_partners: Vec<UnreadPartner>,
    /// The coincidences the declaration door decided from values whose
    /// faces the sweep found meeting ([`glue::touched`]), in operand
    /// keys ([`crate::coincidence`]).
    pub coincidences: Vec<crate::Coincidence>,
    /// The vertex identities the sweep decided, in decision order: the
    /// rows [`Self::contacts`] cite ([`crate::Backing::Decided`] names
    /// an index here), each one decided with a margin emitted into the
    /// result's coincidences ([`reduce::PendingRow`]).
    pub(crate) pending: Vec<reduce::PendingRow>,
    /// Each face–face row's decision in [`Self::pending`], with the
    /// index in [`Self::coincidences`] of the row the glue door
    /// recorded for it: one decision, one row.
    pub(crate) glued: Vec<(u32, u32)>,
    /// The line each glued tangency verified along a line was decided
    /// along, by its pair in operand keys ([`GlueVerdict::of_rows`]).
    pub(crate) tangent_lines: std::collections::BTreeMap<(FaceKey, FaceKey), DecidedLine<T>>,
    /// Each pair of pierces one ring serves, decided one point
    /// ([`RingTie`]).
    pub(crate) ring_ties: Vec<RingTie>,
    /// The operation's section table, built at the glue door
    /// ([`section_table`]).
    pub(crate) sections: section_table::SectionTable<T>,
}

/// A cross-operand face pair the coincidence ladder settled one
/// carrier, with its orientation ([`BooleanReduction`]'s `coincident`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SettledPair {
    /// The A face, in A's keys.
    pub(crate) a: FaceKey,
    /// The B face, in B's keys.
    pub(crate) b: FaceKey,
    /// `SameOriented` or `SameOpposite`, never `Distinct`.
    pub(crate) relation: CarrierRelation,
}

impl<T: Real> BooleanReduction<T> {
    /// Opens a surgery scope on each operand body — the join's
    /// declaration that tier 1 is paid once per operand at its end
    /// rather than once per Euler operator inside it
    /// ([`crate::surgery`]).
    ///
    /// **Guardless, and this is one of the two sites in the crate
    /// where it has to be.** A [`crate::construction::Surgery`] would borrow
    /// `self.a` for the scope's whole span, and the join takes the
    /// WHOLE reduction — both bodies, the contacts, the null-edge
    /// records — so the guard and the call cannot both exist. What
    /// makes the pair safe here is that the reduction is a local of
    /// the pipeline that opened it: a refusal on the way drops it, and
    /// no later call can reach a body a scope was left open on.
    pub(crate) fn enter_join_surgery(&mut self) {
        self.a.enter_surgery();
        self.b.enter_surgery();
    }

    /// Closes both scopes [`Self::enter_join_surgery`] opened, sweeping
    /// each operand **only if the join succeeded**.
    ///
    /// A refusal mid-join leaves a partially carved operand that no
    /// door undertook to certify, so the sweep is the success path's.
    pub(crate) fn leave_join_surgery(&mut self, joined: bool) {
        if joined {
            self.a.leave_surgery_and_sweep();
            self.b.leave_surgery_and_sweep();
        } else {
            self.a.leave_surgery();
            self.b.leave_surgery();
        }
    }

    /// The minted null edges of one operand's clone, insertion order —
    /// PR 5 (joining) walks each solid's scaffolding separately; this
    /// is the per-operand view of [`Self::null_edges`].
    pub fn null_edges_of(&self, operand: Operand) -> impl Iterator<Item = &BoolNullEdgeRecord<T>> {
        self.null_edges.iter().filter(move |r| r.operand == operand)
    }
}

/// Which site raised [`BooleanError::CurvedPairUnsupported`]. The three
/// share one meaning — this pair of face kinds has no sound lane under
/// this op — and differ in when it is known.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PairRefusalSite {
    /// The operand gate, up front: a kind with no wired arm whose box
    /// may meet an undeclared face of the other operand.
    OperandGate,
    /// The ∖/∩ front door, up front: a kind with no revert seam lane
    /// whose box may meet a face of the other operand.
    RevertRoster,
    /// The crossings path's interior-loop guard, after the reduction:
    /// the section certificate (`section_cert`) could not certify a face
    /// pair free of a closed loop interior to both faces — an
    /// intractable pose, a tangency, a certified interior loop, an
    /// undecided witness, or a face whose boundary walk could not be
    /// read (R-unread). The site carries no refusal reason; the
    /// no-crossings path's [`BooleanError::FallbackExtentUnsupported`]
    /// does.
    InteriorLoopGuard,
}

/// Which lane raised [`BooleanError::EdgeCarrierUnsupported`]. Each
/// reads an edge's carrier and has a row for a line, a circle and an
/// ellipse; the site says what it could not do with a spiric or a
/// spline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EdgeCarrierSite {
    /// The sweep's crossing lane against a plane face of the other
    /// operand.
    PlanarCrossing,
    /// The sweep's crossing lane against a curved face of the other
    /// operand.
    CurvedCrossing,
    /// The join's section frame for a germ that runs along an edge of
    /// either solid.
    GermFrame,
    /// The join's ring lane, winding the run of a face's loop that a new
    /// face is walled off along.
    RingRun,
}

impl EdgeCarrierSite {
    /// What the lane could not do with the edge, completing "the
    /// Boolean cannot yet".
    #[must_use]
    pub const fn clause(self) -> &'static str {
        match self {
            Self::PlanarCrossing => {
                "find whether or where such an edge crosses a plane face of the other operand"
            }
            Self::CurvedCrossing => {
                "find whether or where such an edge crosses a curved face of the other operand"
            }
            Self::GermFrame => "join the two solids along such an edge",
            Self::RingRun => "orient a new face whose boundary runs along such an edge",
        }
    }
}

/// One sector read of a vertex, named in
/// [`BooleanError::VertexReadTwice`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorRead {
    /// The vertex-on-face pass that classifies the vertex against this
    /// face of the other operand.
    Pierce(FaceKey),
    /// The vertex-vertex pass that pairs the vertex with this vertex of
    /// the other operand.
    Pair(VertexKey),
}

/// How a partner whose link runs along a touched face meets it where
/// the side of the face its solid lies on is not read
/// ([`BooleanError::PartnerAlongFace`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlongFace {
    /// The face is curved: an edge in its tangent plane leaves it only
    /// at second order, by the two carriers' relative curvature.
    Curved,
    /// A face of the partner lies on the face.
    FaceOnFace,
    /// A face of the touching vertex lies on the face.
    Flush,
    /// A face of the touching vertex and one of the partner lie in one
    /// plane.
    FacesInOnePlane,
}

/// **The loops a plane cuts on a ring torus**, as the section
/// certificate classifies them ([`BooleanError::GermSectionSpiric`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// A Rust caller can already name and match it — `pncad` re-exports
/// `topo` whole — and the refusal's sentence names the loops, while the
/// recourse it states is the same for all three (cut the torus with a
/// plane the frame serves). Carrying it would add the CUR3 property row
/// and a Python word with no reader that acts on the difference.
///
/// **The falsifier is a caller whose recourse depends on which loops a
/// slanted plane cuts**: the day the general rung serves one class (the
/// loops round the axis have a proved frame), a caller must tell it from
/// the others to know whether the cut builds. Then carry it and delete
/// this paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiricLoops {
    /// Two loops, each winding once round the torus's axis.
    AroundAxis,
    /// Two loops, each winding once round the tube.
    AroundTube,
    /// One loop on the tube that winds round neither.
    Null,
}

/// **Whose record a [`BooleanError::CarryRefused`] refused**, which says
/// what the refusal's record index counts.
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// A Rust caller can already name and match it — `pncad` re-exports
/// `topo` whole — and the refusal's sentence names whose record it was,
/// and the refusal it carries ([`super::CarryRefusal`]) says what went
/// wrong the same way for each. Carrying it
/// would add the CUR3 property row and a Python word with no reader that
/// acts on the difference.
///
/// **The falsifier is a caller that repairs its own records** — one that
/// drops or re-derives an operand's record the carry refused before
/// retrying: it must tell an operand's record from the op's own, and the
/// index from which list. Then carry it and delete this paragraph and the
/// disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusedRow {
    /// One of the operand's carried records: the index is that
    /// operand's record list's.
    Carried(Operand),
    /// A ring tie the reduction decided on the operand, carried as its
    /// records are: the index counts the ties carried on that operand.
    RingTie(Operand),
    /// A record the reduction found: the index is its working set's.
    Found,
}

/// **Which construction a slit's line lacks**
/// ([`BooleanError::SlitLineCrossed`]): the other solid has an edge
/// along an operand's slit or ring, and the germs on the line are not
/// built where the material places them, for the reason each case
/// names.
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// Every case is a construction gap with no lever, so nothing a person
/// does differs by case, and the refusal's sentence names it. A Rust
/// caller can already name and match it, since `pncad` re-exports
/// `topo` whole.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that moves a part off the slit's line when it reads
/// [`SlitLineGap::CrossedTwice`], say. Then carry it and delete this paragraph
/// and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlitLineGap {
    /// One solid's material about the line, a wedge with a face of its
    /// own folded in, crosses the other's twice at the slit's tip.
    CrossedTwice,
}

impl SlitLineGap {
    /// The gap in the refusal's words.
    pub(super) const fn words(self) -> &'static str {
        match self {
            Self::CrossedTwice => {
                "one solid's material about the line crosses the other's twice at the tip"
            }
        }
    }
}

/// **Which of [`BooleanError::SlitFaceGlued`]'s constructions is not
/// built**: the face carrying a slit or ring is glued to a face of the
/// other solid, and the result along the slit is not built, for the
/// reason each case names.
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// Every case is a construction gap with no lever, so nothing a person
/// does differs by case, and the refusal's sentence names it. A Rust
/// caller can already name and match it, since `pncad` re-exports
/// `topo` whole.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that flattens a curved face carrying a slit when it reads
/// [`SlitFaceGap::TipUnread`], say. Then carry it and delete this
/// paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlitFaceGap {
    /// The join's ring run along the slit encloses no area, on a lane
    /// that does not read the sliver between the slit's two edges (a
    /// curved face's ring lane) (`join::pinch_ray_refusal`).
    RingRun,
    /// The join's run along the slit walls off the sliver from neither
    /// side without separating a pending pair (`join::best_arc`).
    ArcsSeparate,
    /// A fate cut through the dropped one of the two flush faces ends at
    /// the slit's tip, where the face's turn is not read: a curved face,
    /// a curved edge on the tip's loop, or half-turns both ways
    /// (`fate::tip_wraps`).
    TipUnread,
}

impl SlitFaceGap {
    /// The gap in the refusal's words.
    pub(super) const fn words(self) -> &'static str {
        match self {
            Self::RingRun => {
                "the join's run along it encloses no area on a lane that reads no sliver"
            }
            Self::ArcsSeparate => {
                "the join's run along it walls off the sliver from neither side without \
                 separating a pending pair"
            }
            Self::TipUnread => {
                "a cut through the dropped flush face ends at its tip, where the face's turn is \
                 not read"
            }
        }
    }
}

/// **Why a node of an equal cylinder pair is not built**
/// ([`BooleanError::NodeUnbuilt`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// A Rust caller can already name and match it — `pncad` re-exports
/// `topo` whole — and the refusal's sentence names the case and states
/// its own recourse, which is all a person acts on: move or turn the
/// parts. Carrying it would add the CUR3 property row and a Python word
/// with no reader that acts on the difference.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// placement driver that slides a part along its axis when it reads
/// `OnBoundary`, which moves the seam or rim off the node, and turns it
/// when it reads `NearlyParallel`, which no slide mends. Then carry it
/// and delete this paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeWhy {
    /// The node lies on an edge or a vertex of the operand's face: a
    /// seam or rim through it.
    OnBoundary(Operand),
    /// The operand's face trim gives the node no verdict.
    Unplaced(Operand),
    /// A crossing of an edge of one operand with the other's wall lies
    /// at the node: its germ reads neither ellipse.
    GermOnNode,
    /// The node's arcs cannot be told apart by their turn toward an
    /// axis: the axes are this nearly parallel at the walls' radius.
    NearlyParallel,
}

thread_local! {
    /// Whether the pierce pass reads none of the branches the sweep
    /// recorded ([`forget_branches`]).
    static FORGET_BRANCHES: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Makes every boolean on this thread read its pierces as though the
/// sweep had recorded no branch for any of them, until called with
/// `false`: each pierce then reads the pierced face's tangent plane, the
/// fallback a missing record takes (`vtxfac::classify_vertex_on_face`).
/// `sweep-testing` only.
#[cfg(feature = "sweep-testing")]
pub fn forget_branches(on: bool) {
    FORGET_BRANCHES.with(|f| f.set(on));
}

/// **Why a germ pair's section was not built**
/// ([`BooleanError::GermSectionUnbuilt`]).
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names
/// this rung; the disposition table cites this paragraph as its home).
/// A Rust caller can already name and match it — `pncad` re-exports
/// `topo` whole — and the refusal's sentence names the case and states
/// its own recourse: move a face so the surfaces clearly cross or part,
/// or run the operation at `f64`. Carrying it would add the CUR3
/// property row and a Python word with no reader that acts on the
/// difference.
///
/// **The falsifier is a caller that acts on the case by itself**: a
/// driver that re-runs an operation at `f64` when it reads `NoLane`,
/// or nudges a part when it reads `Tangent`. Then
/// carry it and delete this paragraph and the disposition row with it.
#[derive(Clone, Debug)]
pub enum SectionUnbuilt {
    /// The pair's section refused at the glue door, decided: an operand
    /// whose radius is not definitely positive.
    Refused(geom_brep::SectionError),
    /// A component is tangent, which C7 constructs and the section only
    /// classifies.
    Tangent(&'static str),
    /// The general rung is not marched at this scalar.
    NoLane {
        /// The scalar, by name.
        scalar: &'static str,
    },
    /// The march refused.
    March(geom_brep::SsiError),
    /// A marched loop's winding about an operand was not read.
    Winds(geom_brep::WindsRefusal),
    /// A marched loop's limb-2 bound stands past the pad its points are
    /// read within (`geom_brep::CarriedComponent::pad`).
    PadShort {
        /// The pad, in metres.
        pad: f64,
        /// The certificate's limb-2 bound, in metres.
        hull_sup: f64,
    },
    /// A marched loop's uniqueness tube could not be read: an empty box
    /// chain, or a refused box or carrier point
    /// (`geom_brep::CarryError::TubeUnread`).
    TubeUnread,
    /// A closed loop of the section could not be opened at its first
    /// site: the knot edit refused.
    Seat(geom_core::spline::KnotAlgebraError),
}

impl core::fmt::Display for SectionUnbuilt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Refused(e) => e.fmt(f),
            Self::Tangent(why) => write!(
                f,
                "{why}. Recourse: move one face so the surfaces clearly cross or clearly part"
            ),
            Self::NoLane { scalar } => write!(
                f,
                "the section is marched on the general rung, which is written at f64 and not at \
                 {scalar}. Recourse: run the operation at f64"
            ),
            Self::March(e) => write!(f, "{}", e.render(geom_brep::recourse::Reading::Build)),
            Self::Winds(why) => write!(
                f,
                "a marched loop's turns about an operand's axis or core could not be read \
                 ({why:?})"
            ),
            Self::TubeUnread => write!(
                f,
                "a marched loop's uniqueness tube could not be read (an empty box chain, or a \
                 refused box or carrier point)"
            ),
            Self::PadShort { pad, hull_sup } => write!(
                f,
                "a marched loop's carrier lies {hull_sup:e} m from the surfaces, past the {pad:e} m \
                 its points are read within"
            ),
            Self::Seat(e) => write!(
                f,
                "the section's closed loop could not be opened at its first site ({e})"
            ),
        }
    }
}

/// Typed failure of [`boolean_reduce`]; the operands are never touched.
#[derive(Debug)]
pub enum BooleanError {
    /// The run's tolerance cannot form a valid band (D4 residue).
    Band(BandError),
    /// A face's kind has no wired boolean arm at the classification
    /// site that met it (M5 PR 9: the F5 planar-only gate retired PER
    /// C5 TABLE ARM — every kind on the operand roster passes the
    /// gate and pair-level refusals fire where an arm is actually
    /// exercised, citing the table's routing; `Approx` keeps the gate
    /// refusal).
    ///
    /// What is missing at the raising site is the boolean's own
    /// crossing layer for the kind — edge×face sweep events, curved trim
    /// containment, and the fitted-chord join lane — even where
    /// `geom_brep::intersect::route` already implements the pair at the
    /// INTERSECTION layer (plane×NURBS). One raising site is the
    /// germ-pair JOIN dispatch's catch-all (`join::bool_connect`), so
    /// a `(Sphere, Sphere)` or `(Cylinder, Sphere)` germ reaches it too,
    /// not only a cone or torus one; the pairs that dispatch does wire
    /// are stated once, at `meeting_recourse`. The pair-general
    /// SECTION-FRAME dispatch beside it is wider, but a frame is not a
    /// join arm: it names the locus's centre and axis for the facing
    /// test and moves no seam lane.
    ///
    /// `kind` is one half of the pair and says nothing about the other,
    /// so the Display never reads as "this kind is unsupported": a
    /// sphere reaches here from a sphere×sphere germ while plane×sphere
    /// is live. The Display is written for the person holding the
    /// mouse: what could not be done and the recourse, never this
    /// routing.
    CurvedBooleanUnsupported {
        /// The offending operand and face.
        operand: Operand,
        /// The face.
        face: FaceKey,
        /// Its surface kind — the C5 table row the refusal cites.
        kind: geom::SurfaceKind,
    },
    /// A pierced face's torus is definitely outside the ring convention
    /// (D3: a horn or spindle torus, or one with no tube), a shape the
    /// kernel has no representation for. The decided arm of the
    /// convention half whose in-band arm is
    /// [`BooleanDecision::Torus`], and it ends in the same lever.
    DegenerateTorus {
        /// The pierced face's operand.
        operand: Operand,
        /// The pierced face.
        face: FaceKey,
        /// The half of the convention that refused.
        convention: TorusConvention,
        /// Its decided verdict.
        verdict: geom_brep::recourse::Refused,
    },
    /// A pierce sector's FIRST-ORDER material verdict could not be
    /// resolved against the pierced face's curvature: the bound leaves
    /// the face within about `2·sqrt(band/R)` radians of tangent (`R`
    /// the face's smallest radius of curvature), or its reach is too
    /// short to witness its slope, so the largest separation from the
    /// face the first-order term certifies past the sagitta does not
    /// clear the band (`boolean::sectors::side_code` carries the
    /// argument and the witness). The **definite** half of a
    /// two-tolerance pair on `bool_pierce_sector_side_curved`; an
    /// in-band charge escalates as [`BooleanError::Escalated`] on the
    /// same predicate instead.
    ///
    /// A refusal, never a guess: the slope's sign is one the band
    /// cannot resolve, and a side read from it would be a guess at the
    /// topology. The side of a near-tangent bound is second order, so
    /// the kernel's own way through is the second-order sector trilean
    /// (`geom_brep::enters_material_order2`), which the declared-
    /// `Tangent` lump already consumes and which no lane wires into
    /// this verdict yet; the user's is the decision's lever
    /// ([`refusal_routes::PIERCE_CURVATURE`], shared with the in-band
    /// arm, [`BooleanDecision::PierceCurvature`]).
    CurvedSectorSideUnsupported {
        /// The charge's decided verdict at the sector's arm.
        verdict: geom_brep::recourse::Refused,
    },
    /// A sweep event definitely lands on a CURVED face away from its
    /// boundary, a vertex sits ON a curved surface, or a curved-carrier
    /// edge cannot be cleared against a curved face, and the curved
    /// PIERCE door cannot take it. That door takes a LINE, a CIRCLE or an
    /// ELLIPSE carrier definitely crossing a cylinder wall, a sphere, a
    /// torus or a cone (away from its apex: [`Self::CrossingAtConeApex`]),
    /// whose
    /// crossing parameters come from the certified root lanes (the line
    /// quadratics and quartic, and `boolean::circle_roots`' doors) and
    /// whose landing point the face's region
    /// places. What this variant reports is the rest: a tangency (not a
    /// crossing at any order the lanes see), a line parallel to a cone's
    /// generator, an undeclared on-carrier
    /// edge or conic, a root the band cannot place, or a
    /// trim the chart door declines to express (the M5 envelope's
    /// frontier; the C5 table routes the SECTIONS, this is the crossing
    /// layer). The
    /// **definite** half of a two-tolerance pair: the very same
    /// clearance margin one band-width away escalates as
    /// [`BooleanError::Escalated`] on `bool_line_cylinder_clearance`
    /// instead ([`Coincide::EdgeOnCurvedFace`]), and both halves end on
    /// the coincidence's levers: the declared-cover rung takes a
    /// declaration of the edge's face against the curved one, and this
    /// definite half names no tolerance.
    CurvedPierceUnsupported {
        /// The operand whose edge met the curved face.
        operand: Operand,
        /// The curved face (in the other operand).
        face: FaceKey,
        /// The edge.
        edge: EdgeKey,
        /// The band the clearance margins were classified against.
        band: Band,
    },
    /// An edge of `operand` meets a curved face of the other in band:
    /// a root door's decision falls in band (a classifying sign in the
    /// gap, or a root finder's touch) for a span the face may hold, or
    /// the curve-pair reading of the edge against the face's boundary
    /// does ([`BooleanDecision::CurveMeeting`] on the face's carrier,
    /// [`BooleanDecision::BoundaryMeeting`] off it). Its story is
    /// `decision`'s.
    CurvedPierceEscalated {
        /// The operand whose edge met the curved face.
        operand: Operand,
        /// The curved face (in the other operand).
        face: FaceKey,
        /// The edge.
        edge: EdgeKey,
        /// The decision that read the pair in band.
        decision: BooleanDecision,
        /// The reading.
        diag: Indeterminate,
    },
    /// An edge of `operand` meets a cone face of the other operand within
    /// the band of the cone's APEX, where the surface has no tangent
    /// plane and its quadric form's gradient vanishes: a root there has
    /// no material side to read and no slope to place it by, so the
    /// crossing layer refuses rather than answer. The definite half: an
    /// in-band apex distance refuses the same way, with no tolerance
    /// named.
    CrossingAtConeApex {
        /// The operand whose edge met the cone face.
        operand: Operand,
        /// The cone face (in the other operand).
        face: FaceKey,
        /// The edge.
        edge: EdgeKey,
        /// The band the apex distance was classified against.
        band: Band,
    },
    /// A vertex the boolean classifies sits within the band of the APEX
    /// of a cone face of `operand`: the cone's own apex vertex as the
    /// corner of one of its sectors, or another operand's vertex landing
    /// on the cone there. The face has no tangent plane at its apex:
    /// each ruling out of it has a normal of its own, and what the
    /// Boolean lacks is the sector pair algebra that reads a material
    /// side over that fan of rulings, so the sector and pierce lanes
    /// refuse rather than read one normal. The definite half: an in-band
    /// distance off the axis refuses the same way, with no tolerance
    /// named.
    NormalAtConeApex {
        /// The operand the cone face belongs to.
        operand: Operand,
        /// The cone face.
        face: FaceKey,
        /// The band the distance off the axis was classified against.
        band: Band,
    },
    /// A vertex the sector walk reads, or a point the pierce pass reads a
    /// pierced face's normal at, sits within the band of a crease or an
    /// irregular point of a spline face of `operand`: a line its
    /// chart's normal is not continuous across (an interior knot run of
    /// multiplicity at least the degree, a closed chart's seam), where
    /// the face has a normal on each side and the sector's side of the
    /// line is not read, or a point where `S_u × S_v` vanishes (a pole),
    /// where it has none. A vertex decided on the line or point and one
    /// whose distance from it the band cannot decide refuse alike: the
    /// arm has no escalating half, and the variant names no tolerance.
    ///
    /// No operand the boolean builds today reaches it: a crossing on a
    /// spline edge's own crease refuses first, at the crossing rung, and
    /// an at-rest body carries no pole. `sectors`' `spline_refusal_rows`
    /// pins the mapping on the shared walk's fixture, and `vtxfac`'s
    /// spline pierce rows on the pierce's.
    NormalAtSplineCrease {
        /// The operand the spline face belongs to.
        operand: Operand,
        /// The spline face.
        face: FaceKey,
        /// The band the vertex's distance from the crease was classified
        /// against.
        band: Band,
    },
    /// A pierce of a spline face of `operand` read no bound on how
    /// sharply the face bends round the pierce point: the chart's
    /// certified arm ([`geom_brep::NurbsLane::chart_arm`]) refused, for
    /// the reason `cause` names. The side each sector bound reads is
    /// charged for the face's bend over that arm, so with no arm no side
    /// is read. A patch bound that refuses a whole integral chart with a
    /// C⁰ crease anywhere refuses here too, however far the crease lies
    /// (`work/encl/the-patch-bound-refuses-an-integral-chart-with-a-crease-anywhere`).
    PierceLeverUnread {
        /// The operand the spline face belongs to.
        operand: Operand,
        /// The spline face.
        face: FaceKey,
        /// Why no arm was read.
        cause: geom_brep::ChartArmRefusal,
    },
    /// A lane met an edge whose carrier is a spiric or a spline, and has
    /// no row for either kind; `site` names the lane. At a crossing lane
    /// the edge's endpoints' sides neither find its crossings (it can
    /// cross a face and come back between same-side ends) nor place them
    /// (a parameter interpolated between them is not a point on the
    /// face), and the face is the first of the other operand's whose box
    /// the edge's box meets (a spline's box is the whole space). At the
    /// join, the face is the one of `operand` the germ or the ring runs
    /// on.
    EdgeCarrierUnsupported {
        /// The operand whose edge it is.
        operand: Operand,
        /// The edge: the operand's own, its split pieces' root.
        edge: EdgeKey,
        /// The face the lane read it against, as the lane holds it.
        face: FaceKey,
        /// The operand whose face it is: the other at a crossing lane,
        /// `operand` itself at the join.
        face_of: Operand,
        /// The lane.
        site: EdgeCarrierSite,
    },
    /// An edge of `operand` meets a face of `face_of` where a spline arm
    /// of the curve × face table cannot count its crossings (`why`): the
    /// ring composite for a spline edge against an analytic face
    /// ([`geom_brep::curve_face::composite::crossings`]), or the
    /// spline-face arm for any edge against a spline face
    /// ([`geom_brep::NurbsLane::chart_meetings`]). The edge comes
    /// within the zero band of the face's carrier without a crossing the
    /// arm can tell apart (it touches it, runs along it, or crosses it
    /// more than once within the band), a piece at the arm's floor is not
    /// read past the band, it meets the face within the band of a crease
    /// of the spline, where the spline has no tangent, or a crossing it
    /// located cannot be placed to the band. The definite half: a
    /// decision the arm needs that falls in the band's gap escalates as
    /// [`Self::Escalated`] instead.
    SplineCrossingUnsettled {
        /// The operand whose edge it is.
        operand: Operand,
        /// The edge: the operand's own, its split pieces' root.
        edge: EdgeKey,
        /// The face it was read against.
        face: FaceKey,
        /// The operand whose face it is.
        face_of: Operand,
        /// What the arm could not settle.
        why: geom_brep::roots::Unsettled,
    },
    /// The both-edges-split point lane needs a carrier with a point
    /// parameter ([`geom::Curve3::param_near`]) and got a spline, which
    /// has none. It is the split lane's own precondition, narrower than
    /// a crossing lane's ([`Self::EdgeCarrierUnsupported`]), which is why
    /// it is its own variant: one doc and one message covering both
    /// could name neither.
    PointSplitCarrierUnsupported {
        /// The offending operand and edge.
        operand: Operand,
        /// The edge: the operand's own, its split pieces' root.
        edge: EdgeKey,
    },
    /// **Two edges along one germ whose identity no rule decides.**
    /// Where both operands' edges leave a vertex pair along the germ and
    /// the reduction paired their far ends, the join reads them as one
    /// segment only for lines and circles, which meet a line or circle
    /// tangent to them at the site nowhere else, and for two sections of
    /// one plane and one cylinder or cone, the faces either side of each
    /// edge on carriers the glue door verified one with the other's. Any
    /// other conic or spline can meet such a partner again, so the paired
    /// ends do not make the two edges one, and nothing here reads whether
    /// they are (`closed` false). And a closed edge along the germ, whose
    /// far end is the site itself, where nothing reads whether its
    /// carrier lies on the partner's faces around the site
    /// (`sectors::closed_reach`, `closed` true: a spline on either side,
    /// a fat torus whose samples cannot certify a conic on it).
    GermEdgeCarrierUnsupported {
        /// The operand whose edge is not read: for two edges, the first,
        /// A before B, that is neither a line nor a circle, where the two
        /// are not sections of one plane and one cylinder or cone; for a
        /// closed edge, its own.
        operand: Operand,
        /// The edge: the operand's own, its split pieces' root.
        edge: EdgeKey,
        /// Whether the edge is one closed edge leaving the site along the
        /// germ, rather than one of two edges running together.
        closed: bool,
    },
    /// **Point-in-face on a loop no walk expresses at the point.** The
    /// in-plane walk reads each edge on its own carrier — a line as its
    /// chord, a circle or ellipse arc on its conic, a spiric or spline
    /// arc piece by piece. A point that a spiric's or spline's pieces
    /// ran out of depth or budget before placing is refused here rather
    /// than guessed.
    ArcLoopContainmentUnsupported {
        /// The operand whose face carries the loop.
        operand: Operand,
        /// The loop, and the edge whose reading ran out.
        cause: crate::splitting::Uncrossable,
    },
    /// **A point-in-face read of an operand face refused.** The
    /// reduction and the sphere extent scan ask which side of an
    /// operand face's boundary a point lies on, and the face door
    /// ([`contfp`], or a curved face's chart read) refused; its
    /// refusal is carried whole. A sliver margin is
    /// [`BooleanError::Escalated`] and an edge the walk cannot cross is
    /// [`BooleanError::ArcLoopContainmentUnsupported`], not this.
    PointInFaceRefused {
        /// The operand whose face was read.
        operand: Operand,
        /// The face.
        face: FaceKey,
        /// The face door's refusal.
        refusal: ContainError,
    },
    /// An operand is well-formed but not a closed solid at rest: tier 2
    /// ([`crate::validate_closed`]) refuses it for construction
    /// scaffolding an edit left behind — a strut's valence-1 vertex, an
    /// empty loop, a null edge, a shell in pieces. The Boolean serves
    /// finished solids only.
    ScaffoldingOperand {
        /// The offending operand.
        operand: Operand,
        /// The validator's tier-2 findings, each naming its entity.
        errors: Vec<ValidationError>,
    },
    /// An operand holds material wound negative: a solid whose signed
    /// volume tier 3's check 7 decides definitely negative
    /// ([`ValidationError::NegativeVolume`]), or a shell check 10 finds
    /// bounding a region of winding −1 inside a solid whose total is
    /// positive ([`ValidationError::ShellWinding`]), or a solid check 7
    /// passed in band that a ray reads inside-out
    /// ([`ValidationError::SliverInsideOut`]). Its faces bound the
    /// complement of the region they enclose. A finished body never
    /// holds one of the first two; the door reads both checks itself on
    /// an operand whose scalar runs no at-rest gate (a dual,
    /// [`crate::AtRestOutcome::NotRunAtThisScalar`]). The third it reads
    /// up front on each sliver the at-rest gate passed and on every solid
    /// of a dual's operand, where one point outside the solid's faces'
    /// reach boxes decides; and wherever a witness read of the operand
    /// meets its side at infinity inside its material. In each case it
    /// refuses rather than answering for the complement.
    InsideOutOperand {
        /// The offending operand.
        operand: Operand,
        /// The validator's findings, each naming its solid (and, for a
        /// shell, the shell).
        errors: Vec<ValidationError>,
    },
    /// An operand holds a vertex the join would take, or one whose
    /// reading lands in the sliver band (tier 3's check 11,
    /// [`ValidationError::JoinableVertexAtRest`] /
    /// [`ValidationError::JoinUndecidedAtRest`]). A finished body holds
    /// none; the door reads the check itself on an operand whose scalar
    /// runs no at-rest gate (a dual), as it reads checks 7 and 10.
    UnjoinedOperand {
        /// The offending operand.
        operand: Operand,
        /// The check-11 findings, each naming its vertex.
        errors: Vec<ValidationError>,
    },
    /// F7: two adjacent faces of one operand are structurally or
    /// declaredly coplanar — the operand is not maximal-faced; run
    /// `merge_coplanar_faces` explicitly first.
    NonMaximalFaces {
        /// The offending operand.
        operand: Operand,
        /// The shared edge whose two faces coincide.
        edge: EdgeKey,
    },
    /// F7: two neighbouring faces of one operand lie on one plane or
    /// nearly, at an offset in band. The maximal-faces gate compares
    /// faces of ONE operand, which no face-pair declaration names, so
    /// the refusal ends in the gate's own lever
    /// ([`refusal_routes::NEIGHBOUR_OFFSET`]) and offers no declaration.
    CoplanarNeighbours {
        /// The operand both faces belong to.
        operand: Operand,
        /// The two faces.
        faces: [FaceKey; 2],
        /// The offset the plane ladder's offset rung could not decide.
        offset: Indeterminate,
    },
    /// A vertex sector's bounding chord has **no finite length**: its
    /// components overflow the norm (past ~1e154), or one of them is
    /// not a number. Distinct from [`BooleanError::Escalated`] on
    /// purpose — nothing about this is a band question, and no
    /// tolerance lever reaches it.
    NonFiniteSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// A vertex sector's bounding chord has a length that **underflowed
    /// out of the format**: its components are too small for the norm
    /// to hold (below ~1e-162 at `f64`), so it measures exactly zero
    /// while still naming a direction. Distinct from
    /// [`BooleanError::Escalated`] on purpose — nothing about this is
    /// a band question, and no tolerance lever reaches it.
    UnderflowedSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// A reduction/classification predicate escalated (in-band margin),
    /// or the finished result holds a shell certified in band
    /// ([`BooleanDecision::ShellRole`], which definite cuts can compose):
    /// the operand pair is ill-conditioned at this ε — a genuine sliver
    /// (F6). Never a snap, never a guess.
    Escalated {
        /// The decision that escalated, which the refusal's ending
        /// follows from.
        decision: BooleanDecision,
        /// The predicate's escalation diagnostics.
        diag: Indeterminate,
    },
    /// A surface datum two faces are compared on is not finite (NaN or
    /// ±∞): poisoned input, which no declaration and no move of the
    /// parts reads. Both faces are operand faces, stored and read at
    /// rest, so the refusal is the kernel's or the file's the operand
    /// came from, as tier 3's [`ValidationError::PoisonedSurfaceDatum`]
    /// reads the same datum.
    PoisonedCarrierDatum {
        /// The face pair compared, each face with its operand: a face of
        /// each, or two neighbours of one at the maximal-faces gate.
        pair: [(Operand, FaceKey); 2],
        /// The datum's predicate, with its poisoned margin.
        diag: Indeterminate,
    },
    /// A tangency the Boolean glued meets definite counter-evidence
    /// when the op reads it again along its locus: the kernel's own
    /// readings disagree. The pair and its class, and the margin that
    /// decided. The same finding about a record fires at the at-rest
    /// gate as `ValidationError::ContactContradicted`.
    ContactContradicted {
        /// The face pair and the class it was glued as.
        pair: crate::contact::ContactPair,
        /// The fact that contradicted it, where the carrier ladder found
        /// the two faces one carrier; `None` where the counter-evidence
        /// is the tangent lane's, which the margin's predicate labels.
        fact: Option<Contradiction>,
        /// The margin that decided, and its predicate.
        margin: Indeterminate,
    },
    /// A tangency in a configuration this op's classification cannot act
    /// on: a pair with no witness
    /// locus (neither the DEV-1 closed-form lane — plane×cylinder along a
    /// ruling, parallel cylinders — nor a shared rim circle), a rim
    /// whose junction the routing cannot place, or a locus whose span
    /// bound the jet certificate's finest schedule leaves past the band,
    /// so no arm can consume it. Refused at the door rather than carried into stages that would
    /// ignore it — the vocabulary is wider than this op's envelope, and
    /// the gap is typed, not silent.
    TangencyUnsupported,
    /// **A glued shared rim whose carrier pair has no tangent-locus
    /// arm.**
    ///
    /// The rim routing reads the two faces' outward normals opposed
    /// across the rim (wedge 0 or 2π). Building that junction needs the
    /// tangent locus of the pair's two surface kinds, and this is raised
    /// only where [`geom_brep::tangent_locus`] answers `Unsupported`.
    RimCuspArmUnbuilt {
        /// The glued face pair that carries the rim, as a `Tangent`
        /// contact.
        pair: crate::contact::ContactPair,
        /// Which end of the wedge range the two faces' material
        /// subtends at the rim — the routing's own verdict, taken on the
        /// geometry.
        wedge: geom_brep::MaterialWedge,
    },
    /// **A pierce point that a curved face of the piercing operand may
    /// hold, where the Boolean cannot build a vertex**
    /// (`vtxfac::faces_holding_pierce`). An operand whose solids touch,
    /// one's edge lying in another's face, pierces the other operand's
    /// face where that face crosses its own, and the section of the two
    /// passes the point. The face holding the point takes a vertex
    /// there. Here the curved face's trim could not say whether it holds
    /// the point (a ringed wall, a spline face, an outline the chart door
    /// does not read), so nothing is guessed; or the two faces make a
    /// pose no row reaches (`placed` below). Refused
    /// before anything is built.
    HeldPierceUnbuilt {
        /// The piercing operand, whose face holds the point.
        operand: Operand,
        /// The face that holds the point.
        face: FaceKey,
        /// The other operand's face the point pierces.
        pierced: FaceKey,
        /// The holding face's surface kind.
        kind: geom::SurfaceKind,
        /// The pierced face's surface kind.
        pierced_kind: geom::SurfaceKind,
        /// Whether the holding face's trim placed the point inside it:
        /// `false` where it could not read its trim there; `true` where
        /// it did, in a pose no row reaches: both faces curved, a spline
        /// pierced face, a held face with no normal at the point, or two
        /// pierces of the operand holding one point in the face.
        placed: bool,
    },
    /// A boolean's input ([`BooleanInputs`]) references an entity that
    /// does not resolve in its operand (stale/foreign key, or a
    /// non-planar given face) — a caller bug, refused before any
    /// classification runs (M4 PR 5).
    InvalidInput {
        /// The operand whose key failed.
        operand: Operand,
        /// What was wrong.
        what: &'static str,
    },
    /// The 15.11 consecutive-pairing invariant failed: two of A's
    /// consecutive crossing-record pairs cross in B's walk order, or a
    /// run's codes disagree at its two ends — readings no two simple
    /// links round one point give (F12's guarded refusal — see
    /// `insert`).
    PairingMismatch {
        /// The A-side vertex of the neighborhood.
        a_vertex: VertexKey,
        /// The B-side vertex.
        b_vertex: VertexKey,
    },
    /// Two vertex-vertex coincidences that share a vertex both cross
    /// there, in a way the insertion cannot place: an operand holds
    /// several vertices at one point (its own contact's) and the other
    /// operand's vertex there crosses into more than one of their
    /// neighborhoods, and one pair has no run in the shared vertex's
    /// orbit that holds none of another pair's cuts: a null edge both
    /// of whose ways round hold one (`insert::reconcile_shared`). A
    /// dangling null edge whose segment holds another's whole builds:
    /// the inner hangs at its tip, and of two with one segment the Out
    /// one holds (`insert::holds_whole`).
    /// It also refuses where a run the shared vertex turned and a run of
    /// its own pair that held it would together cover the orbit, so
    /// neither can hang from the other (`insert::arc_holders`): a
    /// backstop no known pose reaches. Emitted
    /// by the insertion only: `insert::reconcile_pass` and
    /// `insert::hang_at_shared`.
    SharedVertexCrossings {
        /// The operand whose vertex both pairs share.
        operand: Operand,
        /// That vertex: a key of the operand's working copy, which the
        /// sweep may have minted on one of its edges.
        vertex: VertexKey,
        /// Two of the other operand's vertices at its point that it
        /// crosses into, in classification order: the first two, when
        /// it crosses into more. Keys of the other operand's working
        /// copy, as `vertex` is.
        partners: [VertexKey; 2],
    },
    /// The operands meet along a ray where one or both hold coincident
    /// edges, pieces of that operand touching along it (a contact line),
    /// and more than one pair of the two operands' edges on the ray
    /// crosses: two or more edge-edge germs along one ray of one vertex
    /// pair (`recl::recl_edges`), whose sections along the line the
    /// insertion does not yet place apart. The operation applied to the
    /// pieces before they are joined meets no contact line there.
    CrossingsAlongAContactLine {
        /// A's vertex on the line: a key of A's working copy.
        a_vertex: VertexKey,
        /// B's vertex there: a key of B's working copy.
        b_vertex: VertexKey,
    },
    /// Cones of one operand that touch along a ray lying in the other's
    /// face: the pierces of that face at one point read germs along one
    /// direction in a way the ring cannot order (`vtxfac::germ_order`):
    /// three or more of them, or a ray the join cannot wind round.
    PinchRaysInAFace {
        /// The operand whose cones touch.
        operand: Operand,
        /// The pierced face, a key of the other operand's working copy.
        face: FaceKey,
    },
    /// Under union, a plane face of each operand lies on the other's
    /// with opposed normals at a pair of their vertices, and the two
    /// faces' outlines leave the vertex tangent, one curving inside the
    /// other: the faces overlap in a crescent that narrows to the vertex,
    /// which the section cannot reach (`sectors::crescent_refusal` says
    /// why).
    TangentCrescentUnsupported {
        /// A's face, a key of A's working copy.
        a_face: FaceKey,
        /// B's face, a key of B's working copy.
        b_face: FaceKey,
    },
    /// A vertex of `operand` is read by two sector passes where the
    /// first read cannot be taken with the second: it pierces two faces
    /// of the other solid, or pierces one and coincides with a vertex of
    /// it while crossing the face, or while that vertex's link does not
    /// lie on one side of the face; or, touching a face or in pairs
    /// alone beside more than one partner, its edges read nothing
    /// against the other solid's cone at the point
    /// (`vtxfac::cone_classes`). That other solid holds its
    /// own contact at the vertex's point: two of its faces meet there in
    /// their interiors, or a vertex of it rests on one of its faces. A
    /// vertex-on-face pass hangs struts at a vertex that crosses the
    /// face, so a later pass would read an orbit an earlier one wrote
    /// (`vtxfac::pierced_and_paired`, `vtxfac::partner_side`). A partner
    /// along the face where its side is not read is
    /// [`BooleanError::PartnerAlongFace`]. Neither piece of the other
    /// solid touches itself there alone, so the operation applied to
    /// each piece, with the results then combined, meets one read at the
    /// vertex.
    VertexReadTwice {
        /// The operand whose vertex is read twice.
        operand: Operand,
        /// That vertex: a key of the operand's working copy, which the
        /// sweep may have minted on one of its edges.
        vertex: VertexKey,
        /// Its first pierce, then its next read: a second pierce, or a
        /// pair; in pairs alone, its first two partners.
        reads: [SectorRead; 2],
    },
    /// A vertex of `operand` touches `face` of the other solid and pairs
    /// with `partner`, a vertex of it whose link runs along that face,
    /// where the side of the face the partner's solid lies on is not
    /// read (`vtxfac::partner_side`): `along` says how the two meet.
    /// Lifted off the face, the partner reads strictly one side.
    PartnerAlongFace {
        /// The operand whose vertex touches the face.
        operand: Operand,
        /// That vertex: a key of the operand's working copy.
        vertex: VertexKey,
        /// The face it touches.
        face: FaceKey,
        /// The other solid's vertex it pairs with.
        partner: VertexKey,
        /// How the partner meets the face.
        along: AlongFace,
    },
    /// The result would hold a non-manifold vertex: both operands hold
    /// several vertices at one point, and A's crosses into two of B's
    /// (two crossing pairs share both their vertices). Each solid's
    /// pinch line runs into the point, one from below with its pieces'
    /// corners at the far end, so its two coincident edges pair each
    /// piece's own faces, while the corners the union leaves empty at
    /// the point are each bounded by a face of both pieces: every
    /// such edge ends at one vertex whose orbit passes the line twice.
    /// That shared-entity wedge fan is unrepresentable
    /// (`docs/DESIGN.md`, D1 tier 3′, "Representability boundary"), so
    /// this refusal is permanent at any tolerance
    /// (`insert::reconcile_shared`).
    NonManifoldResult {
        /// A's vertex at the point.
        a_vertex: VertexKey,
        /// Two of B's vertices there that it crosses into.
        b_vertices: [VertexKey; 2],
    },
    /// A classification invariant failed (e.g. a surviving record
    /// without one IN and one OUT code per side) — a kernel bug
    /// surfaced loudly, not silently mis-joined.
    ClassificationInvariant {
        /// Human-oriented description of the violated invariant.
        what: &'static str,
    },
    /// `split_edge` refused while inserting a crossing (site attached,
    /// inner error whole).
    CrossingInsertion {
        /// The operand whose edge was being split.
        operand: Operand,
        /// The edge.
        edge: EdgeKey,
        /// The underlying Euler refusal.
        source: EulerOpError,
    },
    /// **A germ PAIR with no boolean seam lane**, refused at the
    /// operand gate.
    ///
    /// The gate asks THREE questions, and this refusal is what a pair
    /// that answered wrong on all three earns: does this face's KIND
    /// have a wired arm; can this face REACH the other operand at all;
    /// and, failing both, do the caller's DECLARATIONS speak for this
    /// particular pair. Reach is decided at box-level conservatism
    /// (`reduce::first_unsupported_pair`), which is why the payload
    /// names a pair rather than a body: a face whose box clears every
    /// face of the other operand cannot enter a crossing,
    /// a section or a germ pair, so the operation does not depend on
    /// its kind and the gate says nothing about it.
    ///
    /// The third question is the newest and the narrowest. A
    /// declaration is the author supplying the verdict a germ arm would
    /// have supplied, verified at the front door before this gate runs;
    /// a pair it covers is one the pipeline has an answer for. What may
    /// be declared is bounded by the certified carrier inventory, so a
    /// kind with no rung there can never be covered here — which is why
    /// this refusal still names NURBS (under ∖ and ∩) and `Approx`
    /// unconditionally.
    ///
    /// **The overlap that DID fire is a may, not a does.** Boxes are
    /// supersets, so the two faces named here may in exact geometry
    /// be disjoint; the refusal claims only that the kernel cannot
    /// rule the meeting out, and that if they do meet it has no arm
    /// for the pair.
    ///
    /// The `op` field carries the op when the kind is admitted for
    /// OTHERS and refused for this one — `Nurbs` under ∖ and ∩ — and
    /// is `None` when no op has an arm for the kind.
    ///
    /// **What refuses, per class, and why it refuses HERE.** The
    /// blocker left is a JOIN lane, not `revert`:
    ///
    /// - **Sphere**: not gated here — the `(Plane, Sphere)` germ arm
    ///   (exact C5 Circle) plus the extent-certified fallback re-cut.
    /// - **Torus**: not gated up front — its pairs meet the same typed
    ///   doors under ∖ and ∩ as under ∪: the crossing layer's, the
    ///   no-crossings section pass, the join catch-all below, and this
    ///   variant at [`PairRefusalSite::InteriorLoopGuard`] for a torus
    ///   pair the section certificate cannot clear.
    /// - **Cone**: not gated up front, as the torus. The germ-pair JOIN
    ///   dispatch — `join::bool_connect`'s match on the two germ faces'
    ///   surfaces — wires only the pairs `meeting_recourse` names: a
    ///   plane against a plane, cylinder, sphere or cone, a sphere pair
    ///   (on its radical plane) and a cylinder pair with parallel offset
    ///   axes (on its rulings). Its catch-all raises
    ///   [`BooleanError::CurvedBooleanUnsupported`], not this error,
    ///   and a torus or `(Cylinder, Sphere)` germ lands there too; a
    ///   cone against a curved face refuses one door earlier, at its
    ///   germ frame ([`BooleanError::GermFrameUnsupported`]).
    ///
    ///   **A wider dispatch sits beside it and must not be confused
    ///   with it.** `join::pair_section_frame` — the pair-general
    ///   SECTION-FRAME dispatch — additionally names
    ///   `(Sphere, Sphere)` and `(Cylinder, Cylinder)` (as a proven
    ///   STRAIGHT locus, or its own pinch door); `(Cylinder, Sphere)` is
    ///   matched on its section (`boolean::section_table`), not on a
    ///   frame. But neither a frame nor a section is a join arm: they
    ///   place the germs for the matcher, which is a strictly smaller
    ///   thing than a seam lane, and do not move this refusal.
    ///
    ///   The cyl×sphere fitted-chord
    ///   window's blocker MOVED at M6-2: `Pcurve::Fitted` now exists
    ///   and certifies at rest (the SSI enclosure/certify stack is no
    ///   longer `f64`-only), so what is left is the JOIN LANE itself —
    ///   the C5 table has no cyl×sphere arm for the chord's carrier
    ///   (`chord_join::section_case`), and building one is still banked. **The exact coaxial
    ///   classification does not retire this**, and the sentence is
    ///   re-verified rather than moved: the coaxial arm's locus is two
    ///   exact CIRCLES and needs no fitted chord at all, so it gives
    ///   the window nothing to read. What would retire the sentence is
    ///   the window itself, for the TRANSVERSAL poses that march (the
    ///   section value, `geom_brep::pair_section`, carries them and does
    ///   not change this).
    /// - **NURBS**: no edge×NURBS-face crossing layer at all
    ///   (deviation 5), and the fallback's extent test is unwritable
    ///   for the kind ([`BooleanError::NurbsExtentUnsupported`]).
    ///
    /// **Three sites raise it, and [`PairRefusalSite`] says which.** The
    /// operand gate and the ∖/∩ revert roster refuse UP FRONT, on the
    /// operands' kinds and boxes. The crossings path's interior-loop
    /// guard (`ops::interior_loop_verdict`) refuses AFTER the pipeline
    /// would have answered: a face pair, one of them not a plane, that
    /// the section certificate cannot clear of a loop no edge event
    /// marks (the reduction saw crossings elsewhere, and the join and
    /// face-region propagation cannot see the loop).
    ///
    /// The up-front refusals exist because their downstream failure is
    /// **silent, not typed**: with no crossings found the pipeline
    /// falls through to vertex-probed containment, and a curved face
    /// can leave the other solid between its vertices without any
    /// vertex noticing. The executed witness — a ball half-buried in a
    /// slab, metered as if wholly contained — is pinned as
    /// `finding_sphere_class_containment_fallback_is_wrong_today` in
    /// `crates/sweep/tests/m5_s12_curved_ops.rs`, with its merge-base
    /// reproduction recorded. That defect predates S12 and stands on
    /// **union** too; S12 deliberately does not change ∪'s behaviour
    /// (a revert-wiring unit is the wrong place to re-cut the
    /// containment fallback), so ∪ is not gated here and the row is
    /// what keeps it visible.
    CurvedPairUnsupported {
        /// The op this refusal is specific to, or `None` when the kind
        /// has no arm under any op. `Union` is named only by the
        /// interior-loop guard ([`PairRefusalSite::InteriorLoopGuard`]).
        op: Option<BooleanOp>,
        /// Which site refused: the operand gate, the revert roster, or
        /// the crossings path's interior-loop guard.
        site: PairRefusalSite,
        /// The operand carrying the face whose kind has no arm.
        operand: Operand,
        /// That face — the first such in face-arena order.
        face: FaceKey,
        /// Its kind: the half of the germ pair with no arm.
        kind: geom::SurfaceKind,
        /// The other operand's face whose box it may meet.
        other_face: FaceKey,
        /// That face's kind: the other half of the germ pair.
        other_kind: geom::SurfaceKind,
    },
    /// The containment fallback's curved-EXTENT scan (M5 S13) met a
    /// NURBS face. The extent test is UNWRITABLE for the kind with
    /// what exists — `implicit_residual` is poison on a NURBS surface
    /// and the only foot-point projection
    /// (`NurbsSurface::project`) had no lane off `f64`. **Half of that
    /// is now false**: M6-2 lifted the projection to any
    /// bracket-carrying scalar (`impl<T: Bounds> NurbsSurface<T>`), so
    /// the Interval-lane objection is retired. What still blocks the
    /// extent test is the test ITSELF: `implicit_residual` is poison on
    /// a NURBS surface, so a certified extent needs a written
    /// projection-based extent argument — a foot point plus a bound on
    /// how far the patch can reach past it — which nothing has
    /// derived. The class stays RE-GATED at the fallback, explicitly
    /// and pinned, reach first: a NURBS face whose certified box (its
    /// control net's hull) meets no face of the other operand is
    /// certified disjoint from it by the box, and a NURBS face that may
    /// meet one refuses here, never the vertex-probe silence the S12
    /// finding executed.
    NurbsExtentUnsupported {
        /// The operand carrying the NURBS face.
        operand: Operand,
        /// The face.
        face: FaceKey,
    },
    /// The containment fallback's curved-extent scan (M5 S13) met a
    /// configuration it cannot certify: the no-crossings question
    /// ("which operand contains the other?") would otherwise be
    /// answered by vertex probes a curved boundary defeats (the S12
    /// finding), so anything the certified extents cannot clear is
    /// refused typed here — never guessed.
    FallbackExtentUnsupported {
        /// The operand whose sphere group (or face) the scan stopped at.
        operand: Operand,
        /// The face the refusal cites.
        face: FaceKey,
        /// The precise uncertifiable sub-configuration.
        what: &'static str,
    },
    /// **A decision on a re-charted sphere's cell is not stated by the
    /// input cells that cell reads back as.** The containment re-cut
    /// turns a closed sphere shell about its centre
    /// (`ops::apply_recuts`), so each cell of the turned shell reads back
    /// as the set of the input shell's faces, which wear that sphere. A
    /// point decided on such a cell (a meeting, a vertex on a carrier) is
    /// the meeting of those faces and the row's other cells, where a row
    /// holds that many; any other decision, of carriers or of shapes,
    /// holds of them only where the shell is one face and the cell that
    /// face. Otherwise it is refused here rather than published against
    /// faces it was not decided on.
    RechartedRowUnstated {
        /// The operand whose sphere shell was re-charted.
        operand: Operand,
        /// What was decided on the turned cell.
        relation: crate::Relation,
        /// The input shell's faces the turned cell reads back as, sorted.
        faces: Vec<FaceKey>,
        /// Why they do not state it.
        what: &'static str,
    },
    /// **Two sphere faces of the two solids touch** at the curved-extent
    /// scan, which runs only where the crossing layer found no edge
    /// crossing a face: their spheres touch within the tolerance, the
    /// smaller inside the larger (a decided zero), at a touch the section
    /// certificate does not certify off either face. Whatever the faces
    /// share lies off every edge, so the join's sphere-pair arm (the
    /// radical plane, `join::bool_connect`) had no chord to run. Spheres
    /// that cross, in a circle the certificate certifies inside both
    /// faces (its R-loop), are not refused: each sphere is re-cut so an
    /// edge of it reaches the circle (a closed ball re-charted with its
    /// pole on the centre line, any other face cut along a meridian
    /// through the circle, `ops::sphere_extent_scan`), and a crossing
    /// whose circle the certificate cannot place refuses with the
    /// certificate's own reason
    /// ([`BooleanError::FallbackExtentUnsupported`]). It is the decided
    /// refusal of [`SphereQuestion::Nested`], and ends as that question's
    /// escalation does ([`refusal_routes::SPHERES`]).
    /// One sphere touching the other on one carrier is not asked when
    /// every face of it is a verified `Rest` against the other face:
    /// such a pair touches without overlapping (`ops::Exempt::Rest`).
    SpheresMeet {
        /// The operand whose sphere face the scan stopped at.
        operand: Operand,
        /// The sphere face.
        face: FaceKey,
        /// The nesting clearance's decided verdict: zero (band-decided,
        /// a smaller tolerance may decide it positive) or negative (the
        /// boundaries cross).
        verdict: geom_brep::recourse::Refused,
    },
    /// **A germ pair whose section frame has no arm.** The join's
    /// matcher asks each germ pair for the LOCUS its germ line rides,
    /// and the answer drives which facing test runs: a straight locus
    /// takes the chord test, a locus that turns about an axis (a conic,
    /// or a cylinder × sphere loop) the rotational-sense test.
    /// "No frame" therefore MEANS "the locus is straight", and a pair
    /// EARNS that answer only by proof: a plane×plane section is a line
    /// by construction, a plane×cylinder one is a line where the C5
    /// table says so, and a cylinder pair's is rulings exactly when its
    /// axes are parallel. Every other pair either has a section arm
    /// that names the centre and axis its locus turns about, or has no
    /// arm at all; the second case is
    /// refused here rather than defaulting into the straight-chord
    /// test, which would mint a wrong chord silently the moment the
    /// germ-pair dispatch widens.
    GermFrameUnsupported {
        /// The A-side germ face.
        a_face: FaceKey,
        /// Its kind — the A half of the germ pair.
        a_kind: geom::SurfaceKind,
        /// The B-side germ face.
        b_face: FaceKey,
        /// Its kind — the B half of the germ pair.
        b_kind: geom::SurfaceKind,
    },
    /// **A germ pair whose section the Boolean cannot build**: the pair's
    /// section is held by the operation's section table
    /// (`boolean::section_table`, the C5 section value), and either it
    /// refused at the glue door (an operand's radius not definitely
    /// positive) or it did not carry: a tangent component, which C7
    /// constructs and the section only classifies; a scalar at which the
    /// general rung is not marched; or the march's own refusal.
    GermSectionUnbuilt {
        /// The A-side germ face.
        a_face: FaceKey,
        /// Its kind.
        a_kind: geom::SurfaceKind,
        /// The B-side germ face.
        b_face: FaceKey,
        /// Its kind.
        b_kind: geom::SurfaceKind,
        /// Why.
        why: SectionUnbuilt,
    },
    /// **A germ pair whose section is an open conic**: a plane cutting a
    /// cone face in a parabola (parallel to a generator) or a hyperbola
    /// (meeting both nappes). The section table serves either in closed
    /// form, its arcs minted per window as rational-quadratic NURBS
    /// (`geom_brep`'s C1), and the Boolean has no crossing layer for a
    /// NURBS carrier, so the pair is refused at its frame.
    GermSectionOpenConic {
        /// The A-side germ face.
        a_face: FaceKey,
        /// Its kind.
        a_kind: geom::SurfaceKind,
        /// The B-side germ face.
        b_face: FaceKey,
        /// Its kind.
        b_kind: geom::SurfaceKind,
        /// The conic.
        conic: geom_brep::OpenConic,
    },
    /// **A germ pair whose plane cuts a torus face in a general spiric
    /// quartic**: a plane neither through the torus's axis, nor square to
    /// it, nor parallel to it short of its inner equator. The C5 table
    /// routes that section to its general rung, which is unbuilt, and no
    /// carrier holds the curve, so the pair is refused at its frame.
    /// `loops` is the section certificate's own classification of the
    /// section, and `section` the table's refusal.
    GermSectionSpiric {
        /// The A-side germ face.
        a_face: FaceKey,
        /// Its kind.
        a_kind: geom::SurfaceKind,
        /// The B-side germ face.
        b_face: FaceKey,
        /// Its kind.
        b_kind: geom::SurfaceKind,
        /// The loops the plane cuts.
        loops: SpiricLoops,
        /// The section table's refusal.
        section: geom_brep::SectionError,
    },
    /// **A node of an equal cylinder pair's section that the Boolean
    /// does not build.** Two equal cylinder walls whose axes meet cut
    /// each other in two ellipses that cross at two NODES, `p ± r·n̂`
    /// (`n̂ = unit(â₁ × â₂)`), where the walls are tangent. A node inside
    /// a face of each wall, the two faces bounding their solids on the
    /// same side of their walls (two rods, or two bores), is built
    /// (`vtxfac::mint_node`); every other node refuses here, naming A's
    /// and B's faces and why ([`NodeWhy`]).
    ///
    /// Radius equality and coplanar axes are decided by their margins
    /// (`geom_brep::cylinder_cylinder_section`); at definitely unequal
    /// radii the locus is a space quartic, the skew pair's unbuilt
    /// class ([`Self::GermFrameUnsupported`]); a margin in band
    /// escalates rather than claim either class.
    NodeUnbuilt {
        /// A's face the node was read in.
        a_face: FaceKey,
        /// B's face, likewise.
        b_face: FaceKey,
        /// Why the node is not built.
        why: NodeWhy,
    },
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
    /// The result body's pcurve mint pass refused (M5 PR 9: curved
    /// results carry certified per-half-edge pcurves at rest — the
    /// PR 6 contract; loud rather than shipping uncertified caches,
    /// D4 ¶2).
    Pcurves {
        /// The typed pcurve-pass refusal, nested whole.
        source: crate::pcurves::PcurveMintError,
    },
    /// The joining stage's chord machinery refused, nested whole
    /// (`UnpairedLooseEnds`, `SectionLoopMixed`, …): never a lineage row,
    /// which is [`Self::LineageRow`] ([`BooleanJoin`]).
    Join(BooleanJoin),
    /// The output stage's join could not decide whether a valence-2
    /// vertex is a regular point of its edges' carrier: a reading in
    /// the margin band (D4 ¶3).
    JoinUndecided(edge_join::JoinUndecided),
    /// **A curved join on a carrier the stage cannot run on.** The join
    /// restates the kept edge over both edges' span on its own carrier:
    /// a closed join over the carrier's period, an open one through the
    /// killed edge's far end, recovered on the carrier
    /// (`Curve3::param_near`). A carrier with no period, or no
    /// parameter inverse there (a spline), cannot be run on, and the
    /// join refuses rather than leave the vertex standing
    /// (`work/splice/joining-a-spline-carrier-is-unbuilt`).
    JoinCarrierUnsupported {
        /// The edge the join keeps.
        edge: EdgeKey,
        /// Its carrier's kind.
        carrier: geom::CurveKind,
        /// Whether the join closes (it wants a period) or runs on (it
        /// wants a parameter inverse).
        closed: bool,
    },
    /// **A rest the op carries could not be placed on the pieces of the
    /// face it divided.** A record's span along a line edge is placed in
    /// a face by its midpoint (`ops::span_held`), and this piece's read
    /// failed, so the span may be on it.
    RestPieceUnread {
        /// The piece, in result keys.
        face: FaceKey,
        /// The containment door's refusal; `None` where a curved face's
        /// region gives no verdict for this face.
        refusal: Option<ContainError>,
    },
    /// **A record the op carries, or found, could not be placed on the
    /// result** (D1): the one carry refused it
    /// (`ops::carried`, `records::landings`): a pair of its cells'
    /// images the census cannot read, or a record that lands on no pair
    /// with nothing removed to consume it.
    CarryRefused {
        /// Whose record it is, which says what the refusal's record
        /// index counts.
        row: RefusedRow,
        /// The carry's refusal.
        refusal: super::CarryRefusal,
    },
    /// The A/B lockstep invariant failed during joining, finishing, or
    /// the combine door (a kernel bug or corrupt reduction, loudly).
    JoinDesync {
        /// Which lockstep invariant broke.
        what: &'static str,
    },
    /// An operand edge's `SplitEdge` provenance never reached a root:
    /// the walk back from an edge a germ runs along (`Body::split_root`),
    /// or the chain a discarded face's boundary edge records
    /// (`discard::discard_row`), came back to an edge it had passed. A
    /// kernel defect: `Body::split_edge` records each child's parent as
    /// it mints the child, so each record names an older key, and a
    /// graft forwards the records injectively onto keys it mints.
    SplitLineage(crate::SplitLineageCycle),
    /// A lineage list refused a row where a stage added it: a vertex
    /// fusion, a substitution or a face division that maps a cell to
    /// itself, names a cell an earlier row retired, or mints a cell an
    /// earlier row already names. Each list checks its rows there, so
    /// none of its readers meets a corrupt one; reaching this is a
    /// kernel defect.
    LineageRow {
        /// The list that refused the row.
        rows: crate::lineage::LineageRows,
        /// What is wrong with the row.
        defect: crate::lineage::RowDefect,
        /// The cell the defect is about.
        key: Cell,
    },
    /// An edge the Boolean minted, or a boundary edge of a face its
    /// merge left in place, could not be described against the two
    /// faces it lies between: its halves did not walk, or the
    /// attachment gate refused its description on a reading no move of
    /// the parts reaches. It ends in the defect ending (a kernel bug,
    /// loudly), or, on a fitted carrier, in the certification's own
    /// last resort. A line chord read in band of one of its faces is an
    /// [`BooleanError::Escalated`] of
    /// [`BooleanDecision::EdgeOnFaces`].
    EdgeUndescribed {
        /// The edge, diagnostic only: the result is never returned.
        edge: EdgeKey,
        /// What the describer could not do.
        failure: crate::merge_faces::EdgeDescribeFailure,
        /// The attachment gate's refusal, where a description was
        /// offered to it.
        cause: Option<Box<EulerOpError>>,
        /// Whether that description was offered on a fitted (spline)
        /// carrier.
        fitted: bool,
    },
    /// A seam of the result read smooth at its witness reads its two
    /// faces meeting at a corner at one station and smoothly at its
    /// neighbour: the dihedral passes through the ambiguous band between
    /// them, at a point no station read, so the seam is certifiable as
    /// neither a corner nor a smooth join (D2's mixed edge). Or it reads
    /// a corner at every station, refuting the smooth reading at its
    /// witness.
    MixedSeam {
        /// The seam, diagnostic only: the result is never returned.
        edge: EdgeKey,
        /// The two stations that disagree; `None` where every station
        /// reads a corner.
        stations: Option<geom_brep::MixedStations>,
    },
    /// One distributed component carries section faces of both sides
    /// (kernel bug, loudly).
    TornComponent {
        /// The operand whose clone tore.
        operand: Operand,
        /// The offending shell.
        shell: ShellKey,
    },
    /// No witness of a shell the other operand's boundary does not cut
    /// — its vertices, its edges' midpoints, an interior point of each
    /// planar face — decides which side of that boundary the shell lies
    /// on: each lies ON it or too near it to say (`shell_witness`'s
    /// module docs). A shell whose every witness lies ON the other
    /// boundary is asked the `On` question instead, and answers or
    /// refuses [`BooleanError::CoincidentShell`]; this is the in-band
    /// arm.
    ShellWitnessExhausted {
        /// The operand whose shell was probed.
        operand: Operand,
        /// The shell, in that operand's working copy.
        shell: ShellKey,
        /// Witnesses that read `OnBoundary`.
        on_boundary: usize,
        /// Witnesses whose reading was in-band.
        in_band: usize,
        /// The first in-band reading: evidence about one witness, not
        /// the cause, which is that none decided.
        first_in_band: Option<PointInSolidError>,
    },
    /// A shell every witness of which lies ON the other operand's
    /// boundary, which the settled coincidence pairs do not certify
    /// as lying on one shell of the other operand
    /// (`shell_witness::on_verdict`): it has a face with no settled
    /// pair, its pairs disagree on orientation, or no shell of the
    /// other operand is covered back by it and reads `On` in turn.
    CoincidentShell {
        /// The operand whose shell was probed.
        operand: Operand,
        /// The shell, in that operand's working copy.
        shell: ShellKey,
        /// What the settled pairs said about orientation.
        orientation: ShellOrientation,
    },
    /// A read the Boolean put to the solid door (`solid_contain`) about
    /// an operand refused; the door's refusal, carried whole, says what
    /// stopped it.
    Containment(PointInSolidError),
    /// The two seam cycles of a polygon pair are not antiparallel —
    /// the orientation chain broke (kernel bug, loudly).
    SeamOrientation {
        /// The A section face.
        a_face: FaceKey,
        /// The B section face (result key).
        b_face: FaceKey,
    },
    /// The seam zip's record-keyed correspondence could not be
    /// resolved (kernel bug or corrupt records, loudly).
    ZipCorrespondence {
        /// What failed to resolve.
        what: &'static str,
    },
    /// The F7 output stage (`merge_coplanar_faces`) refused, nested
    /// whole: never its re-description of a kept boundary, which refuses
    /// as the Boolean's own description does ([`BooleanMerge`]).
    Merge(BooleanMerge),
    /// The result did not pass the door's at-rest gate (tier 3,
    /// [`crate::AtRestPolicy::gate_at_rest_kept`]; tiers 1 and 2 and
    /// the scaffold fence where the scalar runs none), loudly — no body
    /// below it is ever returned. A result whose every finding is a
    /// shell certified in band is the operands' instead, refused
    /// [`BooleanError::Escalated`] on [`BooleanDecision::ShellRole`];
    /// this carries the rest. Where the scalar runs the at-rest
    /// gate, the operands are finished bodies, so no finding is carried
    /// in from an operand: each is either a defect in what the door
    /// built or a wrong verdict of the validator's own (a valid sliver
    /// refused on a sign check 7 misreads), and a kernel defect either
    /// way. Where it runs none (a dual), an operand carries no verdict:
    /// the door has read only its tiers 1 and 2, its edges and its
    /// orientation, so a finding may also trace to a defect of an
    /// operand's that those reads do not see.
    ResultInvalid {
        /// The validator's findings.
        errors: Vec<ValidationError>,
    },
    /// The result would be unbounded (only reachable with complement
    /// operands, e.g. ∪ of a body with its own complement) — no
    /// boundary representation exists for it.
    UnrepresentableResult,
    /// Re-attaching a grafted edge description against the combined
    /// body's surfaces, through the scalar's edge door, refused (the
    /// combine door's `graft_solid`). The inputs are bitwise the
    /// source's, so a certifying scalar re-derives what the source
    /// already holds; what reaches here is a certificate the source
    /// should not have held. Loud, never a dangling reference.
    GraftRecertify(geom_brep::CertifyError),
    /// The result sort could not read which piece of material a shell
    /// of the result belongs to ([`crate::pieces`]): a solid is one
    /// piece, and the result is not handed back under a guessed
    /// grouping.
    Pieces(crate::pieces::PieceSortError),
    /// ∖ reverts an operand's kept part, two of whose edges lie on one
    /// line, and nothing joins the reverted part afterwards: the
    /// complement pairs the faces meeting there differently, and the
    /// re-pairing is not built.
    Revert {
        /// The operand reverted.
        operand: Operand,
        /// The refusal: two of the operand's edges, and where the line
        /// runs.
        source: crate::revert::RevertError,
    },
    /// An edge of the result lies in one of its own faces, and the
    /// material parts it into two edges, face | wall each
    /// ([`EdgeInFaceGap`]): a parting the kernel does not build there,
    /// or ([`EdgeInFaceGap::is_defect`]) a face crossing the edge's
    /// material, or a parting or division that left what it never
    /// makes, which are the kernel's own defects.
    EdgeInFace {
        /// The edge, in the result's keys.
        edge: crate::entity::EdgeKey,
        /// The face it lies in.
        face: FaceKey,
        /// Why it is not parted.
        gap: EdgeInFaceGap,
    },
    /// The other solid's edges cross the two coincident edges of an
    /// operand's slit or zero-area ring where the solids meet along it
    /// (`recl::recl_edges`), and the germs on the line are not placed
    /// where [`SlitLineGap`] says. [`BooleanError::CrossingsAlongAContactLine`]
    /// where the line is the operand's own slit, which no piece of it
    /// makes: a construction gap, with no lever.
    SlitLineCrossed {
        /// The operand holding the slit.
        operand: Operand,
        /// The slit's tip, a key of the operand's working copy.
        vertex: VertexKey,
        /// The face carrying the slit.
        face: FaceKey,
        /// Which construction is missing.
        gap: SlitLineGap,
    },
    /// The face carrying an operand's slit or zero-area ring is glued to
    /// a face of the other solid, and the result along the slit is not
    /// built, where [`SlitFaceGap`] says. A construction gap, with no
    /// lever.
    SlitFaceGlued {
        /// The operand holding the slit.
        operand: Operand,
        /// The face carrying it, a key of the operand's working copy.
        face: FaceKey,
        /// Which construction is missing.
        gap: SlitFaceGap,
    },
    /// The result keeps a curved face on both sides of an operand's
    /// slit or zero-area ring, whose two edges the zip glued into one
    /// edge of that face with a free end, a strut, and deleting a strut
    /// from a curved face is not built (`slit::close_struts`,
    /// `crate::strut`'s `StrutRefusal::Curved`). Of a strut with both
    /// ends free, the lone ring's bridge is a line chord, which lies in
    /// no curved face; the strut deleted with one free end by `kev`
    /// mints nothing and is refused with it. A construction gap, with
    /// no lever.
    CurvedFaceClosesAcrossSlit {
        /// The curved face, in the result's keys.
        face: FaceKey,
        /// The edge it lies on both sides of.
        edge: crate::entity::EdgeKey,
    },
    /// The result keeps a face on both sides of part of an operand's
    /// slit or zero-area ring, between stretches of it the result keeps:
    /// the zip glues the slit's two edges into one edge of that face
    /// there, with the slit's edges running on past both its ends, and
    /// deleting it parts the slit in two, which is built only where the
    /// edge's two halves lie in one loop of the face and, of the face's
    /// outline, one side is the slit's run (`slit::part_mid_slit`). A
    /// construction gap, with no lever.
    SlitFaceClosesMidSlit {
        /// The face, in the result's keys.
        face: FaceKey,
        /// The edge it lies on both sides of.
        edge: crate::entity::EdgeKey,
    },
    /// **The result's edges on one line, where two or more lie, bound
    /// material the result gate cannot admit** ([`crate::census::line_wedges`]):
    /// each such edge classifies its own wedge (D1's
    /// coincident-distinct-edges class), so their wedges are disjoint.
    /// Where four or more sheets meet on a line the material decides
    /// which pair into edges, and the op's pairing is not that one; the
    /// re-pairing is not built.
    ///
    /// A definite finding at the finished-body gate is otherwise a kernel
    /// defect (`ResultInvalid`, DESIGN.md `[coincidence.booleans]`). This
    /// one is not: the operands are valid and their answer is defined,
    /// and the kernel has not built it, so it is the typed refusal naming
    /// its blocker that DESIGN.md `[loud-failure]` gives valid input the
    /// kernel has not built yet.
    LineWedges {
        /// What the line's reading found.
        fault: LineWedgeFault,
        /// Where the line read begins.
        from: String,
        /// Where it ends.
        to: String,
    },
}

/// What [`BooleanError::LineWedges`] found on the result's line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineWedgeFault {
    /// The two edges' material wedges overlap: the line's material is
    /// counted twice.
    Overlap {
        /// The result's two edges, in arena order.
        edges: [EdgeKey; 2],
    },
    /// This edge's material wedge has no reading on the line: a face of
    /// it with no tangent plane there, its faces meeting at a zero angle
    /// or a full turn (a cusp or a knife slit, D1's wedge ends), or a
    /// length too long or too short to measure. Tier 3′ names the same
    /// reading of a body's own coincident edges
    /// ([`crate::ValidationError::CoincidentWedgesOverlapUndecided`]).
    Unread {
        /// The result's edge.
        edge: EdgeKey,
        /// Why its wedge has no reading.
        why: crate::link::TouchUnread,
    },
}

/// Which arm of [`BooleanError`] refused — the discriminant alone,
/// with no payload.
///
/// [`BooleanError`] derives `Debug` and nothing else: its arms carry
/// arena keys, nested kernel refusals and margin diagnostics whose
/// scalars are floats, so the error is neither `Clone` nor `PartialEq`
/// and a consumer that wants the CLASS of a refusal back out of it has
/// only the prose to substring-match. This projection drops exactly
/// the part that cannot be cloned or compared, so the class rides
/// where the error itself cannot: into a `Clone + PartialEq` finding
/// record, a hash key, a test assertion, an FFI tag map.
///
/// One variant per [`BooleanError`] arm, and [`BooleanError::kind`]
/// matches exhaustively — an arm added to the error reds `kind` itself,
/// here in this crate. Its consumer is pncad-py's `boolean_error_tag`,
/// which maps every variant to an FFI word and reads the class through
/// [`BooleanError::kind`], off the evaluation ladder's error and off the
/// `Refusal` an advisory check's separation finding shares whole.
///
/// A variant HERE with no arm behind it is a phantom: nothing
/// constructs it, so no test can reach it, and a downstream map gives
/// it a word as readily as any other, so the red is this module's
/// tests': an exhaustive match over this enum names the phantom at
/// compile time. The fix at that red is to delete the phantom, never
/// to give it a tag: a tag minted for a phantom publishes a name no
/// refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors
/// [`BooleanError`]'s for reading, and nothing depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum BooleanErrorKind {
    /// [`BooleanError::Band`].
    Band,
    /// [`BooleanError::CurvedBooleanUnsupported`].
    CurvedBooleanUnsupported,
    /// [`BooleanError::DegenerateTorus`].
    DegenerateTorus,
    /// [`BooleanError::CurvedSectorSideUnsupported`].
    CurvedSectorSideUnsupported,
    /// [`BooleanError::CurvedPierceUnsupported`].
    CurvedPierceUnsupported,
    /// [`BooleanError::CurvedPierceEscalated`].
    CurvedPierceEscalated,
    /// [`BooleanError::CrossingAtConeApex`].
    CrossingAtConeApex,
    /// [`BooleanError::NormalAtConeApex`].
    NormalAtConeApex,
    /// [`BooleanError::NormalAtSplineCrease`].
    NormalAtSplineCrease,
    /// [`BooleanError::PierceLeverUnread`].
    PierceLeverUnread,
    /// [`BooleanError::EdgeCarrierUnsupported`].
    EdgeCarrierUnsupported,
    /// [`BooleanError::SplineCrossingUnsettled`].
    SplineCrossingUnsettled,
    /// [`BooleanError::PointSplitCarrierUnsupported`].
    PointSplitCarrierUnsupported,
    /// [`BooleanError::GermEdgeCarrierUnsupported`].
    GermEdgeCarrierUnsupported,
    /// [`BooleanError::ArcLoopContainmentUnsupported`].
    ArcLoopContainmentUnsupported,
    /// [`BooleanError::PointInFaceRefused`].
    PointInFaceRefused,
    /// [`BooleanError::ScaffoldingOperand`].
    ScaffoldingOperand,
    /// [`BooleanError::InsideOutOperand`].
    InsideOutOperand,
    /// [`BooleanError::UnjoinedOperand`].
    UnjoinedOperand,
    /// [`BooleanError::NonMaximalFaces`].
    NonMaximalFaces,
    /// [`BooleanError::CoplanarNeighbours`].
    CoplanarNeighbours,
    /// [`BooleanError::NonFiniteSectorChord`].
    NonFiniteSectorChord,
    /// [`BooleanError::UnderflowedSectorChord`].
    UnderflowedSectorChord,
    /// [`BooleanError::Escalated`].
    Escalated,
    /// [`BooleanError::PoisonedCarrierDatum`].
    PoisonedCarrierDatum,
    /// [`BooleanError::ContactContradicted`].
    ContactContradicted,
    /// [`BooleanError::TangencyUnsupported`].
    TangencyUnsupported,
    /// [`BooleanError::RimCuspArmUnbuilt`].
    RimCuspArmUnbuilt,
    /// [`BooleanError::HeldPierceUnbuilt`].
    HeldPierceUnbuilt,
    /// [`BooleanError::InvalidInput`].
    InvalidInput,
    /// [`BooleanError::PairingMismatch`].
    PairingMismatch,
    /// [`BooleanError::SharedVertexCrossings`].
    SharedVertexCrossings,
    /// [`BooleanError::PinchRaysInAFace`].
    PinchRaysInAFace,
    /// [`BooleanError::TangentCrescentUnsupported`].
    TangentCrescentUnsupported,
    /// [`BooleanError::CrossingsAlongAContactLine`].
    CrossingsAlongAContactLine,
    /// [`BooleanError::VertexReadTwice`].
    VertexReadTwice,
    /// [`BooleanError::PartnerAlongFace`].
    PartnerAlongFace,
    /// [`BooleanError::NonManifoldResult`].
    NonManifoldResult,
    /// [`BooleanError::ClassificationInvariant`].
    ClassificationInvariant,
    /// [`BooleanError::CrossingInsertion`].
    CrossingInsertion,
    /// [`BooleanError::CurvedPairUnsupported`].
    CurvedPairUnsupported,
    /// [`BooleanError::NurbsExtentUnsupported`].
    NurbsExtentUnsupported,
    /// [`BooleanError::FallbackExtentUnsupported`].
    FallbackExtentUnsupported,
    /// [`BooleanError::RechartedRowUnstated`].
    RechartedRowUnstated,
    /// [`BooleanError::SpheresMeet`].
    SpheresMeet,
    /// [`BooleanError::GermFrameUnsupported`].
    GermFrameUnsupported,
    /// [`BooleanError::GermSectionUnbuilt`].
    GermSectionUnbuilt,
    /// [`BooleanError::GermSectionOpenConic`].
    GermSectionOpenConic,
    /// [`BooleanError::GermSectionSpiric`].
    GermSectionSpiric,
    /// [`BooleanError::NodeUnbuilt`].
    NodeUnbuilt,
    /// [`BooleanError::Euler`].
    Euler,
    /// [`BooleanError::Pcurves`].
    Pcurves,
    /// [`BooleanError::Join`].
    Join,
    /// [`BooleanError::JoinUndecided`].
    JoinUndecided,
    /// [`BooleanError::JoinCarrierUnsupported`].
    JoinCarrierUnsupported,
    /// [`BooleanError::RestPieceUnread`].
    RestPieceUnread,
    /// [`BooleanError::CarryRefused`].
    CarryRefused,
    /// [`BooleanError::JoinDesync`].
    JoinDesync,
    /// [`BooleanError::LineageRow`].
    LineageRow,
    /// [`BooleanError::SplitLineage`].
    SplitLineage,
    /// [`BooleanError::EdgeUndescribed`].
    EdgeUndescribed,
    /// [`BooleanError::MixedSeam`].
    MixedSeam,
    /// [`BooleanError::TornComponent`].
    TornComponent,
    /// [`BooleanError::ShellWitnessExhausted`].
    ShellWitnessExhausted,
    /// [`BooleanError::CoincidentShell`].
    CoincidentShell,
    /// [`BooleanError::Containment`].
    Containment,
    /// [`BooleanError::SeamOrientation`].
    SeamOrientation,
    /// [`BooleanError::ZipCorrespondence`].
    ZipCorrespondence,
    /// [`BooleanError::Merge`].
    Merge,
    /// [`BooleanError::ResultInvalid`].
    ResultInvalid,
    /// [`BooleanError::UnrepresentableResult`].
    UnrepresentableResult,
    /// [`BooleanError::GraftRecertify`].
    GraftRecertify,
    /// [`BooleanError::Pieces`].
    Pieces,
    /// [`BooleanError::Revert`].
    Revert,
    /// [`BooleanError::EdgeInFace`].
    EdgeInFace,
    /// [`BooleanError::SlitLineCrossed`].
    SlitLineCrossed,
    /// [`BooleanError::SlitFaceGlued`].
    SlitFaceGlued,
    /// [`BooleanError::CurvedFaceClosesAcrossSlit`].
    CurvedFaceClosesAcrossSlit,
    /// [`BooleanError::SlitFaceClosesMidSlit`].
    SlitFaceClosesMidSlit,
    /// [`BooleanError::LineWedges`].
    LineWedges,
}

impl BooleanError {
    /// The operand gate's refusal on `operand`
    /// ([`crate::AtRestBody::gate_unverdicted`]), typed for the Boolean.
    pub(crate) fn unfinished(operand: Operand, unfinished: crate::validate::Unfinished) -> Self {
        match unfinished {
            crate::validate::Unfinished::Scaffolding(errors) => {
                Self::ScaffoldingOperand { operand, errors }
            }
            crate::validate::Unfinished::InsideOut(errors) => {
                Self::InsideOutOperand { operand, errors }
            }
            crate::validate::Unfinished::Unjoined(errors) => {
                Self::UnjoinedOperand { operand, errors }
            }
        }
    }

    /// An escalation of the coincidence `which` between parts of the two
    /// solids, at a site whose door read the pair's declaration as
    /// `read` ahead of it ([`BooleanDecision::Coincidence`]): the refusal
    /// offers a declaration exactly where `read` says one would settle
    /// the question. A site whose question is not a coincidence between
    /// the two solids names its own [`BooleanDecision`] instead.
    pub(crate) const fn coincidence(
        which: Coincide,
        read: DeclarationRead,
        diag: Indeterminate,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::Coincidence(which, read),
            diag,
        }
    }

    /// An escalation of a reading metered over a lever arm at `gate`,
    /// routed by the rung that raised it
    /// ([`BooleanDecision::of_lever`]): the arm is the gate's own
    /// length, and the reading is the decision the gate meters
    /// ([`refusal_routes::LeverArm::reading`]), with what its door read
    /// of the declaration.
    pub(crate) const fn of_lever(
        gate: refusal_routes::LeverArm,
        read: DeclarationRead,
        escalation: geom_brep::LeverEscalation,
    ) -> Self {
        Self::of_lever_rung(gate, read, escalation.rung(), escalation.diag())
    }

    /// [`BooleanError::of_lever`] from its parts, for a door that carries
    /// the rung apart from the escalation.
    pub(crate) const fn of_lever_rung(
        gate: refusal_routes::LeverArm,
        read: DeclarationRead,
        rung: geom_brep::LeverRung,
        diag: Indeterminate,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::of_lever(gate, read, rung),
            diag,
        }
    }

    /// An escalation of a plane-identity `rung` at `door`, routed to the
    /// decision the rung asks there ([`BooleanDecision::of_plane_rung`]).
    pub(crate) const fn plane_identity(
        rung: PlaneRung,
        door: refusal_routes::PlaneDoor,
        diag: Indeterminate,
    ) -> Self {
        Self::Escalated {
            decision: BooleanDecision::of_plane_rung(rung, door),
            diag,
        }
    }

    /// The refusal of a pierced face's outward normal at a pierce point
    /// (`face_normal::face_outward_normal_at`), in the Boolean's words.
    pub(crate) fn of_pierced_normal(
        refusal: crate::face_normal::NormalAtError,
        operand: Operand,
        face: FaceKey,
    ) -> Self {
        use crate::face_normal::NormalAtError;
        match refusal {
            NormalAtError::Escalated { decision, diag } => Self::Escalated {
                decision: BooleanDecision::of_normal(decision),
                diag,
            },
            NormalAtError::DegenerateTorus {
                convention,
                verdict,
            } => Self::DegenerateTorus {
                operand,
                face,
                convention,
                verdict,
            },
            NormalAtError::OffSurface => Self::ClassificationInvariant {
                what: "pierce point definitely off the pierced face's surface",
            },
            NormalAtError::AtConeApex { band } => Self::NormalAtConeApex {
                operand,
                face,
                band,
            },
            // Decided and in band alike, as the sector walk's crease
            // refuses: the arm has no escalating half.
            NormalAtError::SplineCrease { band, .. }
            | NormalAtError::SplineIrregular { band, .. } => Self::NormalAtSplineCrease {
                operand,
                face,
                band,
            },
        }
    }

    /// Which arm refused, without the payload.
    ///
    /// Exhaustive over [`BooleanError`]: adding an arm there is a
    /// compile error here and in every consumer that maps this enum.
    #[must_use]
    pub fn kind(&self) -> BooleanErrorKind {
        match self {
            Self::Band(_) => BooleanErrorKind::Band,
            Self::CurvedBooleanUnsupported { .. } => BooleanErrorKind::CurvedBooleanUnsupported,
            Self::DegenerateTorus { .. } => BooleanErrorKind::DegenerateTorus,
            Self::CurvedSectorSideUnsupported { .. } => {
                BooleanErrorKind::CurvedSectorSideUnsupported
            }
            Self::CurvedPierceUnsupported { .. } => BooleanErrorKind::CurvedPierceUnsupported,
            Self::CurvedPierceEscalated { .. } => BooleanErrorKind::CurvedPierceEscalated,
            Self::CrossingAtConeApex { .. } => BooleanErrorKind::CrossingAtConeApex,
            Self::NormalAtConeApex { .. } => BooleanErrorKind::NormalAtConeApex,
            Self::NormalAtSplineCrease { .. } => BooleanErrorKind::NormalAtSplineCrease,
            Self::PierceLeverUnread { .. } => BooleanErrorKind::PierceLeverUnread,
            Self::EdgeCarrierUnsupported { .. } => BooleanErrorKind::EdgeCarrierUnsupported,
            Self::SplineCrossingUnsettled { .. } => BooleanErrorKind::SplineCrossingUnsettled,
            Self::PointSplitCarrierUnsupported { .. } => {
                BooleanErrorKind::PointSplitCarrierUnsupported
            }
            Self::GermEdgeCarrierUnsupported { .. } => BooleanErrorKind::GermEdgeCarrierUnsupported,
            Self::ArcLoopContainmentUnsupported { .. } => {
                BooleanErrorKind::ArcLoopContainmentUnsupported
            }
            Self::PointInFaceRefused { .. } => BooleanErrorKind::PointInFaceRefused,
            Self::ScaffoldingOperand { .. } => BooleanErrorKind::ScaffoldingOperand,
            Self::InsideOutOperand { .. } => BooleanErrorKind::InsideOutOperand,
            Self::UnjoinedOperand { .. } => BooleanErrorKind::UnjoinedOperand,
            Self::NonMaximalFaces { .. } => BooleanErrorKind::NonMaximalFaces,
            Self::CoplanarNeighbours { .. } => BooleanErrorKind::CoplanarNeighbours,
            Self::NonFiniteSectorChord { .. } => BooleanErrorKind::NonFiniteSectorChord,
            Self::UnderflowedSectorChord { .. } => BooleanErrorKind::UnderflowedSectorChord,
            Self::Escalated { .. } => BooleanErrorKind::Escalated,
            Self::PoisonedCarrierDatum { .. } => BooleanErrorKind::PoisonedCarrierDatum,
            Self::ContactContradicted { .. } => BooleanErrorKind::ContactContradicted,
            Self::TangencyUnsupported => BooleanErrorKind::TangencyUnsupported,
            Self::RimCuspArmUnbuilt { .. } => BooleanErrorKind::RimCuspArmUnbuilt,
            Self::HeldPierceUnbuilt { .. } => BooleanErrorKind::HeldPierceUnbuilt,
            Self::InvalidInput { .. } => BooleanErrorKind::InvalidInput,
            Self::PairingMismatch { .. } => BooleanErrorKind::PairingMismatch,
            Self::SharedVertexCrossings { .. } => BooleanErrorKind::SharedVertexCrossings,
            Self::PinchRaysInAFace { .. } => BooleanErrorKind::PinchRaysInAFace,
            Self::TangentCrescentUnsupported { .. } => BooleanErrorKind::TangentCrescentUnsupported,
            Self::CrossingsAlongAContactLine { .. } => BooleanErrorKind::CrossingsAlongAContactLine,
            Self::VertexReadTwice { .. } => BooleanErrorKind::VertexReadTwice,
            Self::PartnerAlongFace { .. } => BooleanErrorKind::PartnerAlongFace,
            Self::NonManifoldResult { .. } => BooleanErrorKind::NonManifoldResult,
            Self::ClassificationInvariant { .. } => BooleanErrorKind::ClassificationInvariant,
            Self::CrossingInsertion { .. } => BooleanErrorKind::CrossingInsertion,
            Self::CurvedPairUnsupported { .. } => BooleanErrorKind::CurvedPairUnsupported,
            Self::NurbsExtentUnsupported { .. } => BooleanErrorKind::NurbsExtentUnsupported,
            Self::FallbackExtentUnsupported { .. } => BooleanErrorKind::FallbackExtentUnsupported,
            Self::RechartedRowUnstated { .. } => BooleanErrorKind::RechartedRowUnstated,
            Self::SpheresMeet { .. } => BooleanErrorKind::SpheresMeet,
            Self::GermFrameUnsupported { .. } => BooleanErrorKind::GermFrameUnsupported,
            Self::GermSectionUnbuilt { .. } => BooleanErrorKind::GermSectionUnbuilt,
            Self::GermSectionOpenConic { .. } => BooleanErrorKind::GermSectionOpenConic,
            Self::GermSectionSpiric { .. } => BooleanErrorKind::GermSectionSpiric,
            Self::NodeUnbuilt { .. } => BooleanErrorKind::NodeUnbuilt,
            Self::Euler(_) => BooleanErrorKind::Euler,
            Self::Pcurves { .. } => BooleanErrorKind::Pcurves,
            Self::Join(_) => BooleanErrorKind::Join,
            Self::JoinUndecided(_) => BooleanErrorKind::JoinUndecided,
            Self::JoinCarrierUnsupported { .. } => BooleanErrorKind::JoinCarrierUnsupported,
            Self::RestPieceUnread { .. } => BooleanErrorKind::RestPieceUnread,
            Self::CarryRefused { .. } => BooleanErrorKind::CarryRefused,
            Self::JoinDesync { .. } => BooleanErrorKind::JoinDesync,
            Self::LineageRow { .. } => BooleanErrorKind::LineageRow,
            Self::SplitLineage(_) => BooleanErrorKind::SplitLineage,
            Self::EdgeUndescribed { .. } => BooleanErrorKind::EdgeUndescribed,
            Self::MixedSeam { .. } => BooleanErrorKind::MixedSeam,
            Self::TornComponent { .. } => BooleanErrorKind::TornComponent,
            Self::ShellWitnessExhausted { .. } => BooleanErrorKind::ShellWitnessExhausted,
            Self::CoincidentShell { .. } => BooleanErrorKind::CoincidentShell,
            Self::Containment(_) => BooleanErrorKind::Containment,
            Self::SeamOrientation { .. } => BooleanErrorKind::SeamOrientation,
            Self::ZipCorrespondence { .. } => BooleanErrorKind::ZipCorrespondence,
            Self::Merge(_) => BooleanErrorKind::Merge,
            Self::ResultInvalid { .. } => BooleanErrorKind::ResultInvalid,
            Self::UnrepresentableResult => BooleanErrorKind::UnrepresentableResult,
            Self::GraftRecertify(_) => BooleanErrorKind::GraftRecertify,
            Self::Pieces(_) => BooleanErrorKind::Pieces,
            Self::Revert { .. } => BooleanErrorKind::Revert,
            Self::EdgeInFace { .. } => BooleanErrorKind::EdgeInFace,
            Self::SlitLineCrossed { .. } => BooleanErrorKind::SlitLineCrossed,
            Self::SlitFaceGlued { .. } => BooleanErrorKind::SlitFaceGlued,
            Self::CurvedFaceClosesAcrossSlit { .. } => BooleanErrorKind::CurvedFaceClosesAcrossSlit,
            Self::SlitFaceClosesMidSlit { .. } => BooleanErrorKind::SlitFaceClosesMidSlit,
            Self::LineWedges { .. } => BooleanErrorKind::LineWedges,
        }
    }
}

impl From<BandError> for BooleanError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

impl From<EulerOpError> for BooleanError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

/// The recourse of every refusal of a spiric or spline edge: the kinds
/// every lane reads are the line and the two conics.
pub(crate) const CONIC_EDGES_RECOURSE: &str =
    "rebuild that solid so its edges are lines, circles or ellipses";

/// What the spline arm could not settle, as a clause for
/// [`BooleanError::SplineCrossingUnsettled`]'s sentence.
fn unsettled_crossing(why: geom_brep::roots::Unsettled) -> &'static str {
    use geom_brep::roots::Unsettled as U;
    match why {
        U::Tangency => {
            "it comes within the tolerance of the face without a crossing the Boolean can \
             tell apart: it touches the face, runs along it, or crosses it more than once \
             within the tolerance"
        }
        U::Floor => {
            "it comes too near the face for the Boolean to resolve whether or where it \
             crosses: it may touch the face, run along it, or cross it more than once"
        }
        U::Crease => "it meets the face at a corner of the spline",
        U::Meter => {
            "a crossing it located cannot be placed on the face, or inside the edge, to \
             within the tolerance"
        }
        U::Exhausted => "its crossings could not be counted within the walk's budget",
        U::Resolvent | U::CountDisagrees { .. } => "the count of its crossings did not certify",
        U::AtApex => "it meets the face within the tolerance of a cone's apex",
        U::Degenerate => "it runs along a ruling of the face",
    }
}

/// How a refusal names an operand to the person who built it: by its
/// place in the operation, never by the enum spelling.
fn operand_word(operand: Operand) -> &'static str {
    match operand {
        Operand::A => "first",
        Operand::B => "second",
    }
}

/// The operation, as the noun a refusal sentence uses.
fn op_noun(op: BooleanOp) -> &'static str {
    match op {
        BooleanOp::Union => "union",
        BooleanOp::Intersect => "intersection",
        BooleanOp::Subtract => "subtraction",
    }
}

/// A surface kind as the person holding the mouse reads it. The
/// spline kinds get one spelling everywhere a Boolean refusal names
/// them; every other kind is its own name.
pub(crate) fn kind_word(kind: geom::SurfaceKind) -> &'static str {
    match kind {
        geom::SurfaceKind::Nurbs => "spline (NURBS)",
        geom::SurfaceKind::Approx => "approximated spline",
        other => other.name(),
    }
}

/// The recourse every "these faces cannot meet yet" refusal ends on.
///
/// **This is the one statement of the pairs the Boolean can join**: the
/// germ-pair join (`join::bool_connect`, and the chord layer it mints
/// sections through) joins a plane face against a plane, cylinder,
/// sphere or cone face, mirrors included, and against a torus face where
/// the plane holds the torus's axis or stands square to it, and two
/// spheres or two parallel cylinders along their radical plane.
/// The rustdoc that needs the set points here. The operand gate's box
/// test is conservative (a box overlap is a MAY),
/// so moving the face clear of the other solid is a real recourse even
/// where the faces never touch in exact geometry.
pub(crate) fn meeting_recourse(kind: &str) -> String {
    format!(
        "Recourse: reshape the parts so every meeting is a plane against a plane, cylinder, \
         sphere, cone, or torus whose axis it holds or is square to, or move the {kind} face \
         clear of the other solid"
    )
}

impl core::fmt::Display for BooleanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // The band's own refusal names itself and carries its recourse.
            Self::Band(e) => write!(f, "{e}"),
            // No operand is named: the raise sites disagree on whether
            // `operand` is the face's own operand or the operand of the
            // edge that met it.
            Self::CurvedBooleanUnsupported { kind, .. } => write!(
                f,
                "the Boolean cannot yet work out where one solid's {} face meets the \
                 face of the other solid it runs into: that pairing of faces is not \
                 supported yet. {}",
                kind_word(*kind),
                meeting_recourse(kind_word(*kind)),
            ),
            Self::DegenerateTorus {
                convention,
                verdict,
                ..
            } => write!(
                f,
                "{}. {}",
                convention.refused("a torus face's", *verdict),
                convention
                    .sized()
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::CurvedPierceUnsupported { operand, .. } => write!(
                f,
                "an edge of the {} operand touches or crosses a curved face of the \
                 other operand away from that face's edges, and the Boolean cannot yet \
                 settle where or whether it passes through. Recourse: \
                 {}",
                operand_word(*operand),
                geom_core::COINCIDENCE_RECOURSE,
            ),
            Self::CurvedPierceEscalated {
                operand,
                decision,
                diag,
                ..
            } => write!(
                f,
                "at an edge of the {} operand against a curved face of the other, {}",
                operand_word(*operand),
                decision.render(diag)
            ),
            Self::CrossingAtConeApex { operand, .. } => write!(
                f,
                "an edge of the {} operand meets a cone face of the other operand at \
                 the cone's tip, where the face has no direction to cross it by. \
                 Recourse: move the parts so the edge clearly passes the tip",
                operand_word(*operand),
            ),
            Self::NormalAtConeApex { operand, .. } => write!(
                f,
                "a corner of one part sits at the tip of a cone face of the {} operand, \
                 where the face has no direction to read the other part's side by. \
                 Recourse: move the parts so nothing meets the cone's tip",
                operand_word(*operand),
            ),
            Self::NormalAtSplineCrease { operand, .. } => write!(
                f,
                "a corner of one part sits on a crease or a pinch of a curved face of \
                 the {} operand, where the face has no single direction to read the \
                 other part's side by. Recourse: move the parts so the corner lies \
                 clear of the crease",
                operand_word(*operand),
            ),
            Self::PierceLeverUnread { operand, cause, .. } => write!(
                f,
                "a corner of one part sits on a curved face of the {} operand whose bend \
                 near the corner could not be bounded ({}), so the other part's side of \
                 it is not read. Recourse: move the parts so the corner lies clear of \
                 the face's creases and pinches",
                operand_word(*operand),
                match cause {
                    geom_brep::ChartArmRefusal::PatchBound =>
                        "the face's curvature bound refuses its whole surface",
                    geom_brep::ChartArmRefusal::CreaseWithinReach => "a crease lies nearby",
                    geom_brep::ChartArmRefusal::Unbounded =>
                        "the face's bend at the corner has no bound",
                    geom_brep::ChartArmRefusal::Poisoned =>
                        "the corner's position is not one point",
                    geom_brep::ChartArmRefusal::NoLane =>
                        "this number type reads no curvature bound",
                },
            ),
            Self::CurvedSectorSideUnsupported { verdict } => write!(
                f,
                "where an edge pierces a curved face, the Boolean cannot be sure which \
                 side of the face the material is on: {}. {}",
                match verdict {
                    geom_brep::recourse::Refused::Zero(_) => {
                        "a direction leaving the pierce point runs too close to tangent to \
                         the face, or along too short an edge, for this tolerance to tell"
                    }
                    geom_brep::recourse::Refused::Negative { .. } => {
                        "the face bends away over the edge's length by more than the edge \
                         departs from it"
                    }
                },
                refusal_routes::PIERCE_CURVATURE
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::EdgeCarrierUnsupported { operand, site, .. } => write!(
                f,
                "an edge of the {} operand is a spiric or spline (NURBS) curve, and the \
                 Boolean cannot yet {}. Recourse: {CONIC_EDGES_RECOURSE}",
                operand_word(*operand),
                site.clause(),
            ),
            Self::SplineCrossingUnsettled { operand, why, .. } => write!(
                f,
                "an edge of the {} operand meets a face of the other, one of the two a \
                 spline (NURBS), where the Boolean cannot count its crossings: {}. \
                 Recourse: move the \
                 parts so that edge crosses the face cleanly or stays clear of it",
                operand_word(*operand),
                unsettled_crossing(*why),
            ),
            Self::PointSplitCarrierUnsupported { operand, .. } => write!(
                f,
                "a spline (NURBS) edge of the {} operand has to be split where the \
                 solids meet, and the Boolean cannot find a point's parameter on a \
                 spline. Recourse: move the parts so that edge does not meet the other \
                 solid",
                operand_word(*operand),
            ),
            Self::GermEdgeCarrierUnsupported {
                operand,
                closed: false,
                ..
            } => write!(
                f,
                "an edge of each solid runs between the same two points, and the Boolean \
                 reads two such edges as one curve only when both are lines or circles, or \
                 both lie where one plane meets one cylinder or cone; the {} operand's edge \
                 is neither. Recourse: move the parts so those edges do not run together",
                operand_word(*operand),
            ),
            Self::GermEdgeCarrierUnsupported {
                operand,
                closed: true,
                ..
            } => write!(
                f,
                "a closed edge of the {} operand touches the other solid at its one vertex, \
                 and the Boolean cannot read whether the edge lies on the faces it touches \
                 there: their surface and its curve are a pair it does not read. Recourse: \
                 move the parts so the edge's vertex does not touch the other solid",
                operand_word(*operand),
            ),
            // No operand is named, for the same reason as above: some
            // raise sites carry the operand of the edge being placed, not
            // of the face whose loop has no walk.
            Self::ArcLoopContainmentUnsupported { cause, .. } => write!(
                f,
                "the Boolean cannot yet tell what lies inside a flat face whose outline \
                 has a {} edge near the point it asked about, so it refuses rather than \
                 guess. Recourse: model the outline with lines, circles or ellipses",
                cause.carrier.word()
            ),
            // `ContainError`'s own text names the loop by arena key; the
            // arms the reduction carries are re-worded for a user here,
            // and a curved chart read's refusal is the solid door's text.
            Self::PointInFaceRefused {
                operand, refusal, ..
            } => {
                let operand = operand_word(*operand);
                let preamble = format_args!(
                    "the Boolean cannot tell which side of a face of the {operand} solid a \
                     point lies on"
                );
                match refusal {
                    ContainError::EmptyLoop(_) => write!(
                        f,
                        "{preamble}: one of the face's loops is a lone vertex, which bounds \
                         no region"
                    ),
                    ContainError::LoopUnreadable(_) => write!(
                        f,
                        "{preamble}: one of the face's loops could not be walked as an \
                         outline (a whole-turn construction circle, or an arc wound past a \
                         full turn, is one it cannot read)"
                    ),
                    ContainError::RayExhausted => {
                        write!(f, "{preamble}: {}", crate::ray_walk::NoRaySettled)
                    }
                    ContainError::Curved(e) => write!(f, "the Boolean {e}"),
                    ContainError::Escalated { .. }
                    | ContainError::StaleFace(_)
                    | ContainError::Uncrossable(_)
                    | ContainError::ArcEndClearOfCorners(_) => write!(f, "{preamble}: {refusal}"),
                }
            }
            Self::ScaffoldingOperand { operand, .. } => write!(
                f,
                "the {} operand {}",
                operand_word(*operand),
                crate::validate::Unfinished::SCAFFOLDING_REFUSAL,
            ),
            Self::InsideOutOperand { operand, .. } => write!(
                f,
                "the {} operand {}",
                operand_word(*operand),
                crate::validate::Unfinished::INSIDE_OUT_REFUSAL,
            ),
            Self::UnjoinedOperand { operand, .. } => write!(
                f,
                "the {} operand {}",
                operand_word(*operand),
                crate::validate::Unfinished::UNJOINED_REFUSAL,
            ),
            Self::NonMaximalFaces { operand, .. } => write!(
                f,
                "the {} operand has two neighbouring faces that lie on one surface, so \
                 the Boolean refuses it. Recourse: merge those faces first \
                 (merge_coplanar_faces)",
                operand_word(*operand),
            ),
            Self::CurvedPairUnsupported {
                site: PairRefusalSite::InteriorLoopGuard,
                op,
                operand,
                kind,
                other_kind,
                ..
            } => write!(
                f,
                "the {} operand's {} face may meet the {} operand's {} face along a closed \
                 curve that crosses no edge of either solid, and the Boolean{} cannot yet \
                 see such a meeting, so it refuses rather than guess. Recourse: move the \
                 parts so every place they meet crosses an edge of one of them, or so the \
                 {} face stays clear of the other solid",
                operand_word(*operand),
                kind_word(*kind),
                operand_word(operand.other()),
                kind_word(*other_kind),
                op.map_or(String::new(), |op| format!(" {}", op_noun(op))),
                kind_word(*kind),
            ),
            Self::CurvedPairUnsupported {
                op,
                operand,
                kind,
                other_kind,
                ..
            } => write!(
                f,
                "the {} operand's {} face may meet the {} operand's {} face, and the \
                 Boolean{} cannot yet work out where such a face meets another solid. \
                 {}",
                operand_word(*operand),
                kind_word(*kind),
                operand_word(operand.other()),
                kind_word(*other_kind),
                op.map_or(String::new(), |op| format!(" {}", op_noun(op))),
                meeting_recourse(kind_word(*kind)),
            ),
            Self::NurbsExtentUnsupported { operand, .. } => write!(
                f,
                "the Boolean found no crossing between the two solids and cannot yet \
                 check whether the {} operand's spline (NURBS) face lies inside or \
                 outside the other, so it refuses rather than guess. Recourse: make \
                 the solids' boundaries cross, or build that face from planes, \
                 cylinders or spheres",
                operand_word(*operand),
            ),
            Self::RechartedRowUnstated {
                operand,
                relation,
                what,
                ..
            } => write!(
                f,
                "the Boolean turned a sphere of the {} operand about its centre to find where \
                 it crosses the other solid, and cannot state what it decided on the turned \
                 sphere ({}) on that sphere's own faces: {what}",
                operand_word(*operand),
                relation.noun(),
            ),
            // No operand is named: the raise sites pass the operand being
            // scanned, which is not always the one whose face the refusal cites.
            Self::FallbackExtentUnsupported { what, .. } => write!(
                f,
                "the solids' boundaries do not cross, and the Boolean cannot be sure \
                 whether one lies inside the other ({what}). Recourse: {}",
                refusal_routes::EXTENT_LEVER,
            ),
            // No operand is named, as the scan's other refusals name none.
            Self::SpheresMeet { verdict, .. } => write!(
                f,
                "the two solids' spheres {}, and where they meet lies off every edge of both \
                 solids, so the Boolean finds no crossing to join them along. {}",
                match verdict {
                    geom_brep::recourse::Refused::Zero(_) => {
                        "touch within the tolerance, the smaller inside the larger"
                    }
                    geom_brep::recourse::Refused::Negative { .. } => "cross",
                },
                refusal_routes::SPHERES
                    .recourse(verdict.arm(), geom_brep::recourse::Reading::Build)
            ),
            Self::GermSectionUnbuilt {
                a_kind,
                b_kind,
                why,
                ..
            } => write!(
                f,
                "the Boolean cannot build the section where the first operand's {} face meets \
                 the second operand's {} face: {why}",
                kind_word(*a_kind),
                kind_word(*b_kind),
            ),
            Self::GermFrameUnsupported { a_kind, b_kind, .. } => write!(
                f,
                "the Boolean cannot yet trace where the first operand's {} face meets \
                 the second operand's {} face, so it refuses rather than guess. {}",
                kind_word(*a_kind),
                kind_word(*b_kind),
                meeting_recourse(kind_word(*a_kind)),
            ),
            Self::GermSectionOpenConic { conic, .. } => write!(
                f,
                "a flat face of one part cuts a cone face of the other along a curve \
                 that never closes ({}), which the Boolean cannot yet cross with other \
                 edges. Recourse: tilt the parts so the flat face cuts the cone all the \
                 way round, or keep it clear of the cone face",
                conic.named(),
            ),
            Self::GermSectionSpiric { loops, .. } => write!(
                f,
                "a flat face of one part cuts a torus face of the other at a slant, along a \
                 curve the Boolean cannot yet trace ({}). {}",
                match loops {
                    SpiricLoops::AroundAxis => "two loops round the axis",
                    SpiricLoops::AroundTube => "two loops round the tube",
                    SpiricLoops::Null => "one loop on the tube",
                },
                meeting_recourse(kind_word(geom::SurfaceKind::Torus)),
            ),
            Self::NodeUnbuilt { why, .. } => {
                let (where_, recourse) = match why {
                    NodeWhy::OnBoundary(_) => (
                        "where they cross on an edge of either part",
                        "move or turn the parts so that each crossing point lies inside a \
                         face of both walls or outside one of them",
                    ),
                    NodeWhy::Unplaced(_) => (
                        "where they cross, which it cannot place inside or outside a face of \
                         either part",
                        "move or turn the parts so that each crossing point lies clearly \
                         inside a face of both walls or outside one of them",
                    ),
                    NodeWhy::GermOnNode => (
                        "where they cross, which an edge of the other part meets",
                        "move or turn the parts so that no edge of either part meets a wall \
                         at a crossing point",
                    ),
                    NodeWhy::NearlyParallel => (
                        "where they cross when the axes are this nearly parallel",
                        "turn the parts so that their axes cross at a wider angle",
                    ),
                };
                write!(
                    f,
                    "two cylinder walls of equal radius whose axes cross meet along two \
                     ellipses that cross each other, and the Boolean cannot yet build a \
                     point {where_}. Recourse: {recourse}"
                )
            }
            Self::Pcurves { source } => write!(
                f,
                "the result's pcurve mint pass refused (curved results carry \
                 certified per-half-edge pcurves at rest): {source}"
            ),
            Self::NonFiniteSectorChord { .. } => write!(
                f,
                "a direction the Boolean measures where the solids meet has no finite \
                 length \u{2014} its components overflow, or one of them is not a \
                 number. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::UnderflowedSectorChord { .. } => write!(
                f,
                "a direction the Boolean measures where the solids meet is too small \
                 for its length to be represented, so it measures exactly zero; no \
                 tolerance reaches this. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::Escalated { decision, diag } => f.write_str(&decision.render(diag)),
            Self::CoplanarNeighbours {
                operand, offset, ..
            } => write!(
                f,
                "two neighbouring faces of the {} operand lie on one plane, or nearly ({}). {}",
                operand_word(*operand),
                offset.payload(),
                refusal_routes::neighbour_ending(offset)
            ),
            Self::PoisonedCarrierDatum { .. } => write!(
                f,
                "a surface datum two faces of the operands are compared on is not finite, so \
                 the faces describe no shape to compare. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            // A glued tangency is the Boolean's own reading (the glue
            // door's, or a union step's given verdict), never the user's,
            // so a later reading that contradicts it is the kernel
            // disagreeing with itself.
            Self::ContactContradicted { fact, .. } => write!(
                f,
                "the Boolean read two of the operands' faces tangent along a line or rim, \
                 and a later reading of the same faces contradicts that: {}. {}",
                fact.map_or(crate::contact::CONTRADICTION_REASON, Contradiction::fact),
                KERNEL_DEFECT_ENDING,
            ),
            Self::TangencyUnsupported => write!(
                f,
                "a glued tangency lies outside the envelope this op's classification acts \
                 on (the closed-form witness lane — plane×cylinder along a ruling, parallel \
                 cylinders — or a rim circle the two faces share, as a smooth seam) — the pair \
                 is refused at the door rather than ignored inside"
            ),
            Self::RimCuspArmUnbuilt { pair, .. } => write!(
                f,
                "the faces glued {} meet along a rim circle with their outward normals \
                 opposed, and the Boolean cannot yet build that junction: it has no \
                 tangent locus for these two surfaces. There is no way through this in \
                 the kernel yet",
                pair.class.name()
            ),
            Self::HeldPierceUnbuilt {
                operand,
                kind,
                pierced_kind,
                placed,
                ..
            } => write!(
                f,
                "a solid of the {} operand has an edge lying in a {} face of another of its \
                 solids, and a {} face of the other operand crosses both there{}. Recourse: \
                 join that solid to the other operand first, cut any hole in that face after \
                 this join, or move the edge off the face",
                operand_word(*operand),
                kind_word(*kind),
                kind_word(*pierced_kind),
                if *placed {
                    "; the Boolean cannot build a vertex on that face"
                } else {
                    " (or may: the Boolean cannot read that face's outline there)"
                },
            ),
            Self::InvalidInput { operand, what } => write!(
                f,
                "an input on the {} operand is invalid: {what}",
                operand_word(*operand)
            ),
            Self::PairingMismatch { a_vertex, b_vertex } => write!(
                f,
                "null-edge pairing mismatch at vertex pair \
                 ({a_vertex:?}, {b_vertex:?}): two surviving crossing-record pairs cross in \
                 one neighborhood, or a run's side codes disagree at its ends (the 15.11 \
                 invariant's guarded refusal)"
            ),
            Self::SharedVertexCrossings { operand, .. } => write!(
                f,
                "a corner of the {} solid meets a point where the other solid holds \
                 several corners that only touch each other, and \
                 cuts into more than one of them in a way the Boolean cannot yet join. \
                 There is no way through this in the kernel yet",
                operand_word(*operand)
            ),
            Self::PinchRaysInAFace { operand, .. } => write!(
                f,
                "the {} solid has corners that touch along a line lying in a face of \
                 the other solid, in a way the Boolean cannot yet join. There is no \
                 way through this in the kernel yet",
                operand_word(*operand)
            ),
            Self::TangentCrescentUnsupported { .. } => write!(
                f,
                "a flat face of each part lies on the other's, and their outlines touch at a \
                 point where one curves inside the other, so the faces share a sliver that \
                 narrows to that point, which a union cannot yet join (subtract and \
                 intersect can). Recourse: move one part so the two outlines cross or stay \
                 clear of each other there"
            ),
            Self::CrossingsAlongAContactLine { .. } => write!(
                f,
                "the solids meet along a line where pieces of one or both touch, and \
                 they cut into each other there across more than one piece, which the \
                 Boolean cannot yet join. Recourse: apply the operation to the pieces \
                 before they are joined to each other, or move one solid so its edges \
                 leave the line"
            ),
            Self::VertexReadTwice {
                operand,
                reads: [SectorRead::Pair(_), SectorRead::Pair(_)],
                ..
            } => write!(
                f,
                "a corner of the {} solid lands where two corners of the other solid meet \
                 at one point, and one of those corners gives no side for its edges: its \
                 faces fold onto one another, exactly or to within the input tolerance \
                 (ε_input, K·ε), or it lies on fewer than two faces. The Boolean cannot \
                 classify the first corner against both without it. Recourse: move the \
                 parts so the other solid's corners do not meet at that point, or open its \
                 folded faces apart by more than ε_input",
                operand_word(*operand)
            ),
            Self::VertexReadTwice { operand, reads, .. } => write!(
                f,
                "a corner of the {} solid lands where the other solid touches itself \
                 ({}), and the Boolean cannot yet classify one corner against both. \
                 Recourse: apply the operation to each piece of the other solid that \
                 touches there, then combine the results",
                operand_word(*operand),
                match reads[1] {
                    SectorRead::Pierce(_) => "two of its faces meet there",
                    SectorRead::Pair(_) => "a corner of it rests on one of its faces",
                }
            ),
            Self::PartnerAlongFace { operand, along, .. } => write!(
                f,
                "a corner of the {} solid touches a face of the other solid beside one of \
                 its corners lying along that face ({}), on it or within ε_input (K·ε), so \
                 which side of the face that solid lies on is unread. Recourse: lift that \
                 corner's edges off the face by more than ε_input, or move the corners apart",
                operand_word(*operand),
                match along {
                    AlongFace::Curved => "the face is curved",
                    AlongFace::FaceOnFace => "a face of it on that face",
                    AlongFace::Flush => "a face of the first corner on that face",
                    AlongFace::FacesInOnePlane => "a face of each corner in one plane",
                }
            ),
            Self::NonManifoldResult { .. } => write!(
                f,
                "the result would meet itself in a fan of faces around one point, where \
                 both solids' pinch lines overlap: no boundary representation can hold \
                 that, at any tolerance. Recourse: move the parts so their pinch lines do \
                 not overlap along a length (meeting end to end at a point builds)"
            ),
            Self::ClassificationInvariant { what } => {
                write!(f, "classification invariant violated: {what}")
            }
            Self::CrossingInsertion {
                operand, source, ..
            } => write!(
                f,
                "the Boolean could not insert a crossing on an edge of the {} operand: \
                 {source}",
                operand_word(*operand)
            ),
            Self::Euler(e) => write!(f, "euler operation refused: {e}"),
            Self::Join(e) => write!(f, "the operands' sections could not be joined: {e}"),
            Self::JoinUndecided(e) => write!(f, "{e}"),
            Self::RestPieceUnread { face, refusal } => {
                let preamble = format_args!(
                    "the Boolean cannot tell which piece of a face it divided holds a vertex \
                     resting on that face: it cannot read whether piece {face:?} holds it"
                );
                match refusal {
                    Some(e) => write!(f, "{preamble} ({e})"),
                    None => write!(
                        f,
                        "{preamble} (the containment door does not read this curved face's trim)"
                    ),
                }
            }
            Self::JoinCarrierUnsupported {
                edge,
                carrier,
                closed,
            } => write!(
                f,
                "the Boolean cannot yet join edge {edge:?} across a vertex on its {} carrier: \
                 the join runs the edge on along its carrier {}, which this carrier has no \
                 reading for (work/splice/joining-a-spline-carrier-is-unbuilt)",
                carrier.name(),
                if *closed {
                    "over a whole period"
                } else {
                    "through the other edge's far end"
                }
            ),
            Self::CarryRefused { row, refusal } => {
                let whose = match row {
                    RefusedRow::Carried(operand) => {
                        format!("a contact record of the {} operand", operand_word(*operand))
                    }
                    RefusedRow::RingTie(operand) => {
                        format!("a ring tie on the {} operand", operand_word(*operand))
                    }
                    RefusedRow::Found => "a contact record the Boolean found".to_owned(),
                };
                write!(
                    f,
                    "the Boolean could not carry {whose} onto its result: {refusal}"
                )
            }
            Self::JoinDesync { what } => write!(
                f,
                "A/B lockstep invariant violated: {what} (kernel bug or corrupt \
                 reduction)"
            ),
            Self::SplitLineage(cycle) => write!(
                f,
                "the Boolean's split records of an operand edge are corrupt, so no body \
                 is returned: {cycle}. {KERNEL_DEFECT_ENDING}"
            ),
            Self::LineageRow { rows, defect, key } => write!(
                f,
                "{}, so no body is returned. {KERNEL_DEFECT_ENDING}",
                crate::lineage::RowRefusal::of(*rows, *defect, *key).refused_by("Boolean")
            ),
            Self::EdgeUndescribed {
                failure,
                cause,
                fitted,
                ..
            } => {
                write!(
                    f,
                    "an edge of the result cannot be described against the two faces it lies \
                     between: {}",
                    failure.clause()
                )?;
                if let Some(cause) = cause {
                    write!(f, " ({})", crate::merge_faces::refusal_payload(cause))?;
                }
                write!(
                    f,
                    ". {}",
                    crate::merge_faces::undescribed_ending(cause.as_deref(), *fitted)
                )
            }
            Self::MixedSeam {
                stations: Some(stations),
                ..
            } => {
                write!(
                    f,
                    "along a seam of the result, {stations}. Recourse: {}",
                    refusal_routes::SEAM_WEDGE.lever
                )
            }
            Self::MixedSeam { stations: None, .. } => write!(
                f,
                "a seam of the result reads smooth at its witness but a corner at every \
                 station along it. Recourse: {}",
                refusal_routes::SEAM_WEDGE.lever
            ),
            Self::TornComponent { operand, shell } => write!(
                f,
                "component {shell:?} of the {} operand carries section faces \
                 of both sides (kernel bug)",
                operand_word(*operand)
            ),
            Self::ShellWitnessExhausted {
                operand,
                on_boundary,
                in_band,
                ..
            } => write!(
                f,
                "the solids do not cross, and none of the {} points tried on the {} \
                 solid (corners, edge middles, flat-face interiors) tells whether it \
                 is inside the other: {on_boundary} lie on the other's boundary, \
                 {in_band} too near it to tell. Recourse: if they nearly touch, move \
                 them clearly together or apart",
                on_boundary + in_band,
                operand_word(*operand)
            ),
            Self::CoincidentShell {
                operand,
                orientation,
                ..
            } => {
                write!(
                    f,
                    "every point tried on a closed surface of the {} solid lies on the \
                     other's boundary, ",
                    operand_word(*operand)
                )?;
                match orientation {
                    ShellOrientation::Unpaired { .. } => write!(
                        f,
                        "but one of its faces is neither built from nor declared \
                         coincident with a face of the other. If that face lies on one, \
                         declare the pair; if it does not (a curved face the Boolean \
                         cannot probe inside), this is a kernel frontier"
                    ),
                    ShellOrientation::Mixed => write!(
                        f,
                        "but some of its faces face the same way as the other's faces \
                         they lie on and some the opposite way (kernel frontier)"
                    ),
                    ShellOrientation::Same | ShellOrientation::Opposite => write!(
                        f,
                        "but the faces it lies on are not one closed surface of the other \
                         lying back on it (kernel frontier)"
                    ),
                }
            }
            // The payload does not say which operand was being tested,
            // nor which question asked: the uncut-component probe asks it
            // of solids that do not cross, the reduction of ones that do.
            Self::Containment(e) => write!(f, "the Boolean {e}"),
            Self::SeamOrientation { a_face, b_face } => write!(
                f,
                "seam cycles of faces {a_face:?}/{b_face:?} are not antiparallel \
                 (orientation chain broke — kernel bug)"
            ),
            Self::ZipCorrespondence { what } => {
                write!(f, "seam zip correspondence failed: {what} (kernel bug)")
            }
            Self::Merge(e) => write!(f, "coplanar-merge output stage refused: {e}"),
            Self::ResultInvalid { errors } => crate::validate::write_result_not_at_rest(
                f,
                format_args!("the Boolean's result"),
                errors,
            ),
            Self::UnrepresentableResult => write!(
                f,
                "the result would be unbounded (complement operands) — no boundary \
                 representation exists"
            ),
            Self::GraftRecertify(e) => {
                write!(
                    f,
                    "grafted edge description failed re-certification: {}",
                    e.render(geom_brep::recourse::Reading::Build)
                )
            }
            Self::Pieces(e) => write!(f, "the result could not be sorted into solids: {e}"),
            Self::Revert { operand, source } => {
                let operand = operand_word(*operand);
                write!(
                    f,
                    "the subtraction reverses the {operand} solid's kept part, and {source}. \
                     {NOT_YET_ENDING}"
                )
            }
            Self::EdgeInFace { edge, face, gap } => {
                if gap.is_defect() {
                    write!(
                        f,
                        "edge {edge:?} of the result lies in face {face:?}, and {}. {}",
                        gap.words(),
                        geom_core::KERNEL_DEFECT_ENDING
                    )
                } else {
                    write!(
                        f,
                        "edge {edge:?} of the result lies in face {face:?}, whose material parts \
                         it into two edges, and the parting is not built where {}. {}",
                        gap.words(),
                        geom_core::NOT_YET_ENDING
                    )
                }
            }
            Self::SlitLineCrossed {
                operand,
                vertex,
                face,
                gap,
            } => write!(
                f,
                "the {} solid holds a slit or zero-area ring in face {face:?} ending at vertex \
                 {vertex:?}, where the solids meet along it, and the other solid's edges cross \
                 its two coincident edges there where {}, which the Boolean does not yet build. \
                 {}",
                operand_word(*operand),
                gap.words(),
                geom_core::NOT_YET_ENDING
            ),
            Self::SlitFaceGlued { operand, face, gap } => write!(
                f,
                "a face of the other solid lies flush on face {face:?} of the {} solid, which \
                 carries a slit or zero-area ring, and the boolean does not build the result \
                 along it there: {}. {}",
                operand_word(*operand),
                gap.words(),
                geom_core::NOT_YET_ENDING
            ),
            Self::CurvedFaceClosesAcrossSlit { face, edge } => write!(
                f,
                "the result keeps curved face {face:?} on both sides of edge {edge:?}, where an \
                 operand's slit or zero-area ring lay, so the face closes across it, and \
                 deleting such an edge with a free end from a curved face is not built. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::SlitFaceClosesMidSlit { face, edge } => write!(
                f,
                "the result keeps face {face:?} on both sides of edge {edge:?}, part of an \
                 operand's slit or zero-area ring whose edges run on past both its ends, so the \
                 face closes across that part and parts the slit in two, which the Boolean does \
                 not yet build where the edge's halves lie in two loops or the outline it parts \
                 has no side that is the slit's run. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::LineWedges { fault, from, to } => match fault {
                LineWedgeFault::Overlap { .. } => write!(
                    f,
                    "the result's edges on the line from {from} to {to} bound overlapping \
                     material: the Boolean pairs the faces meeting there as the material does \
                     not, and the re-pairing it needs is not built. {NOT_YET_ENDING}"
                ),
                LineWedgeFault::Unread { why, .. } => write!(
                    f,
                    "an edge on the line from {from} to {to}, where two or more of the \
                     result's edges lie, bounds material the check cannot read: {}",
                    why.sentence_on_line()
                ),
            },
        }
    }
}

impl std::error::Error for BooleanError {}

/// **`boolean_reduce`** — reduction + classification + paired
/// null-edge insertion across two bodies (module docs for the
/// pipeline). Functional: both operands are cloned and never touched;
/// the annotated clones come back in [`BooleanReduction`]. Joining and
/// result generation are PR 5. The operands are finished bodies, as at
/// the boolean doors ([`crate::AtRestBody`]); one with no verdict (a
/// dual) passes the door's own operand gate and orientation read first.
///
/// A plain body is not an operand: it does not coerce to a finished
/// one, so this example must NOT compile.
///
/// ```compile_fail,E0308
/// use geom_core::Tol;
/// use topo::{Body, BooleanOp, boolean_reduce};
///
/// let body = Body::<f64>::new();
/// let _ = boolean_reduce(BooleanOp::Union, &body, &body, Tol::witness());
/// ```
///
/// The same call on finished bodies compiles:
///
/// ```no_run
/// use geom_core::Tol;
/// use topo::{AtRestBody, Body, BooleanOp, boolean_reduce};
///
/// let body = AtRestBody::validate(Body::<f64>::new(), Tol::witness()).unwrap();
/// let _ = boolean_reduce(BooleanOp::Union, &body, &body, Tol::witness());
/// ```
///
/// Determinism (D9): gates, sweeps, contact processing, and
/// per-neighborhood classification all run in arena/discovery order —
/// no hash iteration anywhere.
///
/// It takes finished operands, and a plain body does not coerce to
/// one, so this example must NOT compile:
///
/// ```compile_fail,E0308
/// use geom_core::Tol;
/// use topo::{Body, BooleanOp, boolean_reduce};
///
/// let body = Body::<f64>::new();
/// let _ = boolean_reduce(BooleanOp::Union, &body, &body, Tol::witness());
/// ```
///
/// The same call on finished bodies compiles:
///
/// ```no_run
/// use geom_core::Tol;
/// use topo::{AtRestBody, Body, BooleanOp, boolean_reduce};
///
/// let body = AtRestBody::validate(Body::<f64>::new(), Tol::witness()).unwrap();
/// let _ = boolean_reduce(BooleanOp::Union, &body, &body, Tol::witness());
/// ```
///
/// # Errors
///
/// [`BooleanError`] — see each variant; the first failure wins and the
/// operands are never mutated (the clones are dropped).
pub fn boolean_reduce<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a_operand: &crate::AtRestBody<T>,
    b_operand: &crate::AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanReduction<T>, BooleanError> {
    let band = Band::linear(tol)?;
    for (operand, body) in [(Operand::A, a_operand), (Operand::B, b_operand)] {
        reduce::gate_unverdicted_operand(body, operand, band, tol)?;
    }
    let none = Glue::none();
    let decls = &*glue::decided_glue(a_operand, b_operand, &none, band)?;
    boolean_reduce_declared_strategy(
        op,
        a_operand,
        b_operand,
        decls,
        SweepStrategy::Realized,
        tol,
    )
}

/// The differential suite's sweep-level door (PERF-PLAN §4.4 / C10,
/// pins i and iii): clones the operands, runs the gates and BOTH
/// reduction sweep directions under `strategy`, and returns the
/// per-direction traces `(A→B, B→A)` — `examined` (the candidate set)
/// and `accepted` (pairs where the exact predicates accepted an
/// event). The suite pins `Realized.examined ⊇ Idealized.accepted`
/// per direction.
///
/// `plant` is pin (iii)'s failure injection: it empties ONE face box
/// of `b_operand` in the A→B direction (candidate generation loses
/// that face's events), proving the superset pin can fail. Production
/// code never passes it.
///
/// # Errors
///
/// [`BooleanError`] — the same gates and sweep refusals as
/// [`boolean_reduce`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_traces<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    strategy: SweepStrategy,
    plant: Option<PlantedDegradation>,
    tol: Tol,
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    sweep_traces_with_pad(a_operand, b_operand, strategy, plant, None, tol)
}

/// [`sweep_traces`] with a PAD OVERRIDE (fix-pass pin 1b): the suite
/// proves a too-small pad (e.g. `Some(0.0)`) LOSES accepted pairs and
/// the superset comparator catches it. A deliberately breakable knob —
/// `sweep-testing` only, never production surface.
///
/// The pairs glue as the boolean decides them, so a covered pair
/// reaches the settle stage and its accepted pairs reach the traces.
///
/// # Errors
///
/// [`BooleanError`] as [`sweep_traces`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_traces_with_pad<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    strategy: SweepStrategy,
    plant: Option<PlantedDegradation>,
    pad_override: Option<f64>,
    tol: Tol,
) -> Result<(SweepTrace, SweepTrace), BooleanError> {
    let band = Band::linear(tol)?;
    let none = Glue::none();
    let decls = &*glue::decided_glue(a_operand, b_operand, &none, band)?;
    validate_declarations(a_operand, b_operand, decls)?;
    let verified = verify_glued_contacts(a_operand, b_operand, decls, band)?;
    let (sections, _) = glue::section_door(a_operand, b_operand, decls, T::ssi_lane(), band)?;
    let declared = GluedPairs::build(decls, verified, a_operand, b_operand, sections, band)?;
    reduce::gate_operand_pairs(
        a_operand,
        b_operand,
        &declared,
        band,
        reduce::boolean_arm_exists,
    )?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;

    let mut a = a_operand.clone();
    let mut b = b_operand.clone();
    let mut acc = reduce::ContactAcc::default();
    let mut ab = SweepTrace::default();
    let mut ba = SweepTrace::default();
    let ab_knobs = reduce::SweepKnobs {
        plant: plant.map(|p| p.face),
        pad_override,
    };
    // The plant names a face of `b_operand`, so it applies to the A→B
    // direction only; the pad override applies to both.
    let ba_knobs = reduce::SweepKnobs {
        plant: None,
        pad_override,
    };
    reduce::sweep_and_settle(
        (&mut a, &mut b),
        &crate::sector_shape::SlitTips::of_operands(a_operand, b_operand),
        &declared,
        &mut acc,
        band,
        strategy,
        [&ab_knobs, &ba_knobs],
        [Some(&mut ab), Some(&mut ba)],
        tol,
    )?;
    Ok((ab, ba))
}

/// **The crossing sweep's split operands**: both sweep directions run
/// undeclared and realized, and the split operands they leave with
/// their traces. The one door that hands a suite the operands as the
/// SWEEP leaves them — [`sweep_traces`] returns the traces alone, and
/// `test_support::boolean_through_the_join` the operands after the
/// join has killed the null edges — so the crossing lanes' splits
/// are read on finished bodies; `sweep-testing` only.
///
/// # Errors
///
/// [`BooleanError`] as [`sweep_traces`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_split(
    a_operand: &Body<f64>,
    b_operand: &Body<f64>,
    tol: Tol,
) -> Result<(Body<f64>, Body<f64>, SweepTrace, SweepTrace), BooleanError> {
    let band = Band::linear(tol)?;
    let declared = GluedPairs::default();
    reduce::gate_operand_pairs(
        a_operand,
        b_operand,
        &declared,
        band,
        reduce::boolean_arm_exists,
    )?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;
    let mut a = a_operand.clone();
    let mut b = b_operand.clone();
    let mut acc = reduce::ContactAcc::default();
    let (mut ab, mut ba) = (SweepTrace::default(), SweepTrace::default());
    let knobs = reduce::SweepKnobs::default();
    reduce::sweep_and_settle(
        (&mut a, &mut b),
        &crate::sector_shape::SlitTips::of_operands(a_operand, b_operand),
        &declared,
        &mut acc,
        band,
        SweepStrategy::Realized,
        [&knobs, &knobs],
        [Some(&mut ab), Some(&mut ba)],
        tol,
    )?;
    Ok((a, b, ab, ba))
}

/// **The sweep's contact records and the split operands' sizes** under
/// `strategy`: what both sweep directions recorded, and the
/// `[A vertices, A edges, B vertices, B edges]` they leave. The pruning
/// suites compare these across strategies — box pruning is sound when
/// the realized and idealized sweeps record the same contacts and make
/// the same splits, which a trace of examined pairs alone cannot show
/// (a face-free record is attributed to every face on its carrier). Runs
/// with no glue, as [`sweep_traces`] does, at the recording
/// scalar only: the comparison is of keys and counts, not of brackets.
///
/// # Errors
///
/// [`BooleanError`] as [`sweep_traces`].
#[cfg(feature = "sweep-testing")]
pub fn sweep_records(
    a_operand: &Body<f64>,
    b_operand: &Body<f64>,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<(ReductionContacts, [usize; 4]), BooleanError> {
    let band = Band::linear(tol)?;
    let declared = GluedPairs::default();
    reduce::gate_operand_pairs(
        a_operand,
        b_operand,
        &declared,
        band,
        reduce::boolean_arm_exists,
    )?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;
    let mut a = a_operand.clone();
    let mut b = b_operand.clone();
    let mut acc = reduce::ContactAcc::default();
    let knobs = reduce::SweepKnobs::default();
    reduce::sweep_and_settle(
        (&mut a, &mut b),
        &crate::sector_shape::SlitTips::of_operands(a_operand, b_operand),
        &declared,
        &mut acc,
        band,
        strategy,
        [&knobs, &knobs],
        [None, None],
        tol,
    )?;
    let sizes = [
        a.vertices().count(),
        a.edges().count(),
        b.vertices().count(),
        b.edges().count(),
    ];
    Ok((acc.finish().0, sizes))
}

/// **The boolean pipeline through its join**, undeclared and realized:
/// the two operand clones as the join leaves them, every null edge
/// killed, before the finish, the zip and the closing mint — the
/// production sequence itself (`ops::through_the_join`), stopped there,
/// with the interior-loop guard's verdict the pipeline raises later.
/// `None` where the pipeline answers without a join to stop at. Test
/// vocabulary (`topo::test_support`), for the rows that read the rows
/// a face carries at that point.
///
/// # Errors
///
/// The pipeline's refusal on the way to its join.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn through_the_join(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Option<crate::test_support::JoinedOperands>, BooleanError> {
    Ok(
        match ops::through_the_join(op, a, b, &Glue::none(), SweepStrategy::Realized, true, tol)? {
            ops::Joined::Answered(_) => None,
            ops::Joined::Connected {
                red,
                interior_loops,
                ..
            } => Some(crate::test_support::JoinedOperands {
                a: red.a,
                b: red.b,
                interior_loops,
            }),
        },
    )
}

/// The operand's maximal-faces gate (F7) alone, at `tol`'s band: the
/// rung a body with neighbouring faces on one surface ends at, asked of
/// a body whatever its tier.
///
/// # Errors
///
/// The gate's refusal; [`BooleanError::Band`] where `tol` yields no
/// band.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn maximal_faces_gate<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    tol: Tol,
) -> Result<(), BooleanError> {
    reduce::gate_maximal_faces(body, operand, Band::linear(tol)?)
}

/// **The join's own refusal** of `op`, its pairs glued as the
/// operation decides them: what its matching
/// or its surgery refuses on the reduction. `None` where the join
/// connects, or where the reduction leaves no null pair to join. Test
/// vocabulary (`topo::test_support`).
///
/// # Errors
///
/// The reduction's refusal.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn join_refusal(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<Option<BooleanError>, BooleanError> {
    let band = Band::linear(tol)?;
    let none = Glue::none();
    let decls = &*glue::decided_glue(a, b, &none, band)?;
    let mut red = boolean_reduce_declared_strategy(op, a, b, decls, SweepStrategy::Realized, tol)?;
    if red.null_pairs.is_empty() {
        return Ok(None);
    }
    red.enter_join_surgery();
    let connected = join::bool_connect(&mut red, a, b, band, tol);
    red.leave_join_surgery(connected.is_ok());
    Ok(connected.err())
}

/// Every segment the join of `op` builds ([`join::join_segments`]):
/// how many pair records the reduction registered, and each segment's
/// two germ sites, read on the A clone (a one-site loop's twice). `None` where the reduction
/// registers no pair.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn section_segment_sites(
    op: BooleanOp,
    a: &crate::AtRestBody<f64>,
    b: &crate::AtRestBody<f64>,
    tol: Tol,
) -> Result<Option<SegmentSites>, BooleanError> {
    let band = Band::linear(tol)?;
    let red = boolean_reduce(op, a, b, tol)?;
    if red.null_pairs.is_empty() {
        return Ok(None);
    }
    join::segment_sites(&red, band).map(Some)
}

/// [`section_segment_sites`]' product: the pair-record count and the
/// segments' site pairs.
#[cfg(any(test, feature = "test-support"))]
pub type SegmentSites = (usize, Vec<[Point3<f64>; 2]>);

/// [`boolean_reduce`] with an explicit [`SweepStrategy`] —
/// the idealized/realized door (PERF-PLAN §4.4): production always
/// runs `Realized`; the differential suite runs both and pins
/// bit-equality. Reached via [`boolean_op_with`] for full ops.
pub(crate) fn boolean_reduce_declared_strategy<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a_operand: &Body<T>,
    b_operand: &Body<T>,
    decls: &Glue<T>,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanReduction<T>, BooleanError> {
    let band = Band::linear(tol)?;
    validate_declarations(a_operand, b_operand, decls)?;
    let mut verified = match decls.verdicts {
        Verdicts::Decided => verify_glued_contacts(a_operand, b_operand, decls, band)?,
        Verdicts::Given => given_verdicts(a_operand, b_operand, decls, band)?,
    };
    let (sections, rows) = glue::section_door(a_operand, b_operand, decls, T::ssi_lane(), band)?;
    verified.coincidences.extend(rows);
    let declared = GluedPairs::build(decls, verified, a_operand, b_operand, sections, band)?;
    reduce::gate_operand_pairs(
        a_operand,
        b_operand,
        &declared,
        band,
        reduce::boolean_arm_exists,
    )?;
    reduce::gate_maximal_faces(a_operand, Operand::A, band)?;
    reduce::gate_maximal_faces(b_operand, Operand::B, band)?;
    let mut coincident: Vec<SettledPair> = declared.settled().collect();
    coincident.sort_by_key(|p| (p.a, p.b));
    coincident.dedup();

    // The reduction carves both operand clones through the Euler
    // operators; tier 1 is paid once per clone at the end of the
    // phase rather than once per operator (`crate::surgery`). Two
    // guards over two locals: each owns its own borrow, so a refusal
    // on the way closes both scopes by dropping them.
    // The operands' slit and ring tips, read at rest: the reduction may
    // split a slit's two edges apart from its tip
    // ([`crate::sector_shape::SlitTips`]).
    let tips = crate::sector_shape::SlitTips::of_operands(a_operand, b_operand);
    let mut carved_a = a_operand.clone();
    let mut carved_b = b_operand.clone();
    let mut a = carved_a.begin_surgery();
    let mut b = carved_b.begin_surgery();

    // Reduction sweep, both directions (A's edges first — D9 order),
    // then the touches deferred for both directions' splits.
    let mut acc = reduce::ContactAcc::default();
    let knobs = reduce::SweepKnobs::default();
    reduce::sweep_and_settle(
        (&mut *a, &mut *b),
        &tips,
        &declared,
        &mut acc,
        band,
        strategy,
        [&knobs, &knobs],
        [None, None],
        tol,
    )?;
    let mut mints = core::mem::take(&mut acc.splits);
    let mut branches = acc.take_branches();
    if FORGET_BRANCHES.with(core::cell::Cell::get) {
        branches.clear();
    }
    let charts = acc.take_charts();
    let (mut contacts, mut pending) = acc.finish();
    let mut glued = Vec::with_capacity(declared.verified.touches.len());
    for touch in &declared.verified.touches {
        let k = reduce::pend(
            &mut pending,
            reduce::PendingRow::tangent(touch.a, touch.b, touch.margin),
        );
        glued.push((reduce::index(k), touch.row));
        contacts.ff.push(Cited::new(
            FfContact {
                a: touch.a,
                b: touch.b,
            },
            Cites::decided(reduce::index(k)),
        ));
    }

    let mut null_edges = NullEdges::default();
    let mut null_pairs = Vec::new();
    let mut pierce_rings = Vec::new();
    let mut ring_ties = Vec::new();
    let mut covered = Vec::new();
    let mut fates = fate::Fates::default();
    let mut edge_classes = Vec::new();
    let mut unread_partners = Vec::new();

    let rereads = vtxfac::pierced_and_paired(&contacts)?;
    // Each vertex that touches a face and pairs, its rows combined once
    // every pair is read, and each vertex in pairs alone, likewise
    // (`vtxfac::cone_classes`).
    let mut touches = std::collections::BTreeMap::new();
    // Each vertex's own sectors, with its pairs.
    let mut paired = std::collections::BTreeMap::new();
    // Vertex-on-face classification (sonva then sonvb, as 15.5). Each
    // pierce inserts its piercing side's null edges as it is read; the
    // rings follow, one per face and point (`vtxfac::mint_ring`).
    for (operand, pierced) in [
        (Operand::A, &contacts.a_on_b),
        (Operand::B, &contacts.b_on_a),
    ] {
        let mut minted = Vec::with_capacity(pierced.len());
        for (k, c) in pierced.iter().map(|c| c.record).enumerate() {
            let reread = rereads.get(&(operand, c.vertex)).copied();
            let (piercing_body, pierced_body) = match operand {
                Operand::A => (&mut a, &mut b),
                Operand::B => (&mut b, &mut a),
            };
            let out = match vtxfac::classify_vertex_on_face(
                piercing_body,
                pierced_body,
                operand,
                c,
                op,
                &declared,
                &contacts,
                reread,
                branches.get(&(operand, c.vertex)).copied(),
                charts.get(&(operand, c.vertex, c.face)).copied(),
                &tips[usize::from(operand == Operand::B)],
                band,
            ) {
                Ok(out) => out,
                // **A definite refusal outranks an escalation in this
                // pass**: a pierce read in band refuses only once every
                // later pierce of both operands is read, and one of them
                // refusing definitely is the pass's answer. So which
                // operand's vertices are read first does not choose
                // between the two.
                Err(escalated @ BooleanError::Escalated { .. }) => {
                    let later = pierced[k + 1..]
                        .iter()
                        .map(|c| (operand, c.record))
                        .chain(
                            (operand == Operand::A)
                                .then_some(&contacts.b_on_a)
                                .into_iter()
                                .flatten()
                                .map(|c| (Operand::B, c.record)),
                        )
                        .collect::<Vec<_>>();
                    for (operand, c) in later {
                        let (piercing_body, pierced_body) = match operand {
                            Operand::A => (&mut a, &mut b),
                            Operand::B => (&mut b, &mut a),
                        };
                        match vtxfac::classify_vertex_on_face(
                            piercing_body,
                            pierced_body,
                            operand,
                            c,
                            op,
                            &declared,
                            &contacts,
                            rereads.get(&(operand, c.vertex)).copied(),
                            branches.get(&(operand, c.vertex)).copied(),
                            charts.get(&(operand, c.vertex, c.face)).copied(),
                            &tips[usize::from(operand == Operand::B)],
                            band,
                        ) {
                            Ok(_) | Err(BooleanError::Escalated { .. }) => {}
                            Err(definite) => return Err(definite),
                        }
                    }
                    return Err(escalated);
                }
                Err(e) => return Err(e),
            };
            // `classify_vertex_on_face` refuses a re-read vertex that
            // would write, so a re-read one touched: its orbit, and the
            // pierced face, are as the operands gave them.
            debug_assert!(
                reread.is_none() || (out.edges.is_empty() && out.ring.is_none()),
                "a re-read pierce wrote"
            );
            minted.push((out.edges, out.ring));
            covered.extend(out.covered);
            fates.edges.extend(out.fates.edges);
            fates.corners.extend(out.fates.corners);
            if reread.is_some() {
                touches.insert(
                    (operand, c.vertex),
                    Touch {
                        classes: out.classes,
                        datum: out.datum,
                        flush: out.flush,
                        face: c.face,
                        own: None,
                        pairs: Vec::new(),
                    },
                );
            } else {
                edge_classes.extend(piece_classes(
                    piercing_body,
                    operand,
                    c.vertex,
                    &out.classes,
                    &mints,
                )?);
            }
        }
        // A pierce point inside a face of the piercing body's own adds a
        // pierce there, which its ring serves (`vtxfac::faces_holding_pierce`).
        let (piercing_body, pierced_body) = match operand {
            Operand::A => (&mut a, &b),
            Operand::B => (&mut b, &a),
        };
        // Each curved face's box, read once: a pierce point is placed in
        // a curved face only where its box holds the point.
        let mut curved_boxes = std::collections::BTreeMap::new();
        if minted.iter().any(|(_, plan)| plan.is_some()) {
            let pad = boxes::sweep_pad(band);
            let curved: Vec<FaceKey> = piercing_body
                .faces()
                .map(|(k, _)| k)
                .filter(|&k| reduce::face_plane(piercing_body, k).is_none())
                .collect();
            for face in curved {
                curved_boxes.insert(face, boxes::face_box(piercing_body, face, pad, band)?);
            }
        }
        let box_holds = |face: FaceKey, p: Point3<T>| {
            let point = bvh::Aabb::from_points([p]).unwrap_or_else(bvh::Aabb::poison);
            curved_boxes.get(&face).is_none_or(|b| b.overlaps(&point))
        };
        let plans: Vec<_> = minted
            .iter()
            .filter_map(|(_, plan)| plan.as_ref())
            .collect();
        let held = vtxfac::faces_holding_pierce(
            &declared.sections,
            piercing_body,
            pierced_body,
            &box_holds,
            &plans,
            band,
            tol,
        )?;
        let held_margin: std::collections::BTreeMap<_, _> =
            held.iter().map(|h| (h.plan.contact.vertex, h.on)).collect();
        for h in &held {
            mints.push_in_face(operand, h.plan.contact.vertex, h.face);
        }
        minted.extend(held.into_iter().map(|h| (vec![h.strut], Some(h.plan))));
        // Each ring is minted, and its records read, where its first
        // pierce was: a face pierced at one point alone mints as it
        // would beside its piercing side.
        let plans: Vec<(usize, &vtxfac::RingPlan<T>)> = minted
            .iter()
            .enumerate()
            .filter_map(|(i, (_, plan))| plan.as_ref().map(|plan| (i, plan)))
            .collect();
        let only: Vec<_> = plans.iter().map(|&(_, plan)| plan).collect();
        let pierced_body = match operand {
            Operand::A => &mut b,
            Operand::B => &mut a,
        };
        let mut rings = std::collections::BTreeMap::new();
        let (groups, ties) = vtxfac::ring_groups(&only, band)?;
        // A held pierce stands at its pierce's point because the point
        // lies on its face, so a tie to one cites that decision. Held
        // pierces follow every pierce the sweep found, and a tie's `j`
        // follows its `i`, so a tie holds one at `j` wherever it holds
        // one, and two held pierces tie by the later's.
        ring_ties.extend(ties.into_iter().map(|(i, j, margin)| {
            RingTie {
                operand,
                a: only[i].contact.vertex,
                b: only[j].contact.vertex,
                margin: held_margin
                    .get(&only[j].contact.vertex)
                    .copied()
                    .unwrap_or(margin),
            }
        }));
        for group in groups {
            let members: Vec<_> = group.iter().map(|&k| plans[k].1).collect();
            let ring = vtxfac::mint_ring(pierced_body, &members, band, tol)?;
            rings.insert(plans[group[0]].0, ring);
        }
        for (i, (edges, _)) in minted.into_iter().enumerate() {
            null_edges.extend(edges)?;
            if let Some(ring) = rings.remove(&i) {
                null_edges.extend(ring.edges)?;
                null_pairs.extend(ring.pairs);
                pierce_rings.push(ring.ring);
            }
        }
    }

    // The nodes of the equal cylinder pairs' sections, each a ring in
    // both walls (`node::mint_nodes`).
    let walls = glue::overlapping_pairs(a_operand, b_operand, node::is_wall, band)?;
    for ring in node::mint_nodes((&mut a, &mut b), (a_operand, b_operand), &walls, band, tol)? {
        null_edges.extend(ring.edges)?;
        null_pairs.extend(ring.pairs);
        pierce_rings.push(ring.ring);
    }

    // Vertex-vertex classification, every pair read before the first
    // insertion: a vertex may sit in more than one pair (an operand
    // whose own contact left two vertices at one point pairs both with
    // the other operand's vertex there), and an insertion moves its
    // vertices' orbits. A pair that crosses nowhere inserts nothing;
    // pairs that cross at one vertex are planned together
    // (`insert::reconcile_shared`), so no mint moves a half-edge another
    // pair's plan read.
    let mut classified = Vec::with_capacity(contacts.vv.len());
    let mut line_touches = Vec::new();
    for (i, c) in contacts.vv.iter().map(|c| c.record).enumerate() {
        let a_sectors = sectors::build_sectors(&a, Operand::A, c.a, &tips[0], band)?;
        let b_sectors = sectors::build_sectors(&b, Operand::B, c.b, &tips[1], band)?;
        // A touching vertex whose partner runs along the face, which
        // refuses should the pair read a sector pair on one plane.
        let mut along = None;
        for (operand, vertex, own, other, partner, at) in [
            (
                Operand::A,
                c.a,
                &a_sectors,
                &b_sectors,
                c.b,
                b.resolve_vertex_point(c.b, Proven),
            ),
            (
                Operand::B,
                c.b,
                &b_sectors,
                &a_sectors,
                c.a,
                a.resolve_vertex_point(c.a, Proven),
            ),
        ] {
            let pair = vtxfac::Partner {
                vertex: partner,
                at,
                sectors: other.clone(),
            };
            match touches.get_mut(&(operand, vertex)) {
                Some(touch) => {
                    let face = touch.face;
                    let along_face = |along| BooleanError::PartnerAlongFace {
                        operand,
                        vertex,
                        face,
                        partner,
                        along,
                    };
                    match vtxfac::partner_side(other, touch.datum, touch.flush, band)? {
                        vtxfac::PartnerRead::Side { along: on } => {
                            if on {
                                along.get_or_insert(along_face(AlongFace::FacesInOnePlane));
                            }
                            touch.own.get_or_insert_with(|| own.clone());
                            touch.pairs.push(pair);
                        }
                        vtxfac::PartnerRead::Across => {
                            return Err(BooleanError::VertexReadTwice {
                                operand,
                                vertex,
                                reads: [SectorRead::Pierce(face), SectorRead::Pair(partner)],
                            });
                        }
                        vtxfac::PartnerRead::Along(how) => return Err(along_face(how)),
                    }
                }
                None => paired
                    .entry((operand, vertex))
                    .or_insert_with(|| (own.clone(), Vec::new()))
                    .1
                    .push(pair),
            }
        }
        let sectors::PairSearch {
            mut records,
            touching,
        } = sectors::pair_search(&a_sectors, &b_sectors, band)?;
        if op == BooleanOp::Union {
            sectors::crescent_refusal(
                [&a, &b],
                [&a_sectors, &b_sectors],
                &touching,
                &contacts,
                band,
            )?;
        }
        // The layering beside a partner along the face reads no sector
        // pair on one plane: it refuses before `recl_sectors` reads one.
        if let Some(refusal) = along
            && records.iter().any(sectors::PairRecord::coplanar)
        {
            return Err(refusal);
        }
        // The codes as first read, which the germ loci are derived from.
        let mut raw = records.clone();
        recl::recl_sectors(
            &mut records,
            &mut raw,
            &a_sectors,
            &b_sectors,
            &a,
            &b,
            op,
            &declared,
            band,
            &mut covered,
            &mut fates,
        )?;
        let sector_read = (records.clone(), raw.clone());
        let mut touched = Vec::new();
        recl::recl_edges(
            &mut records,
            &mut raw,
            &a_sectors,
            &b_sectors,
            &a,
            &b,
            op,
            &declared,
            band,
            &recl::Reversed::default(),
            held_faces(&touches, c),
            &mut touched,
            &tips,
        )?;
        line_touches.extend(touched.into_iter().map(|he| (i, he)));
        classified.push((c, a_sectors, b_sectors, records, raw, sector_read));
    }
    let mut read = Vec::with_capacity(touches.len() + paired.len());
    let own_point = |operand: Operand, vertex| match operand {
        Operand::A => a.resolve_vertex_point(vertex, Proven),
        Operand::B => b.resolve_vertex_point(vertex, Proven),
    };
    for (&(operand, vertex), touch) in &touches {
        // A touching vertex classes every edge or refuses, as it did
        // before its touch was read at all.
        let own = touch.own.as_deref().unwrap_or_default();
        let edges: Vec<_> = touch.classes.iter().map(|&(he, _)| he).collect();
        let classes = vtxfac::cone_classes(
            own_point(operand, vertex),
            own,
            &edges,
            &touch.pairs,
            Some((touch.datum.normal, &touch.classes)),
            band,
        )?
        .ok_or_else(|| BooleanError::VertexReadTwice {
            operand,
            vertex,
            reads: [
                SectorRead::Pierce(touch.face),
                SectorRead::Pair(touch.pairs[0].vertex),
            ],
        })?;
        read.push(((operand, vertex), classes));
    }
    for (&(operand, vertex), (own, pairs)) in &paired {
        let edges: Vec<_> = own.iter().filter(|s| s.end_edge()).map(|s| s.he).collect();
        let classes =
            vtxfac::cone_classes(own_point(operand, vertex), own, &edges, pairs, None, band)?;
        let classes = match (classes, pairs.as_slice()) {
            (Some(classes), _) => classes,
            (None, [one]) => {
                unread_partners.push(UnreadPartner {
                    operand,
                    vertex,
                    partner: one.vertex,
                    neighbourhood: UnreadNeighbourhood::ConeUnread,
                });
                Vec::new()
            }
            (None, pairs) => {
                return Err(BooleanError::VertexReadTwice {
                    operand,
                    vertex,
                    reads: [
                        SectorRead::Pair(pairs[0].vertex),
                        SectorRead::Pair(pairs[1].vertex),
                    ],
                });
            }
        };
        read.push(((operand, vertex), classes));
    }
    for ((operand, vertex), classes) in read {
        let body = match operand {
            Operand::A => &a,
            Operand::B => &b,
        };
        edge_classes.extend(piece_classes(body, operand, vertex, &classes, &mints)?);
    }
    // An edge of a shared vertex that two crossing pairs' edge-edge
    // germs run along folds Out in each of them (`recl::Reversed`).
    // Read before the insertion moves any edge's end onto a copy.
    let ee = touch_rows(&a, &b, &contacts.vv, &line_touches)?;
    let edge_germs: Vec<[Vec<HalfEdgeKey>; 2]> = classified
        .iter()
        .map(|(_, a_s, b_s, records, raw, _)| {
            [true, false].map(|a_side| {
                records
                    .iter()
                    .zip(raw)
                    .filter(|(r, _)| r.survives())
                    .filter_map(|(_, w)| recl::edge_edge_bound(w, a_s, b_s, a_side))
                    .collect()
            })
        })
        .collect();
    let vv = &contacts.vv;
    for i in 0..classified.len() {
        let mut reversed = recl::Reversed::default();
        for (slot, list) in [&mut reversed.a, &mut reversed.b].into_iter().enumerate() {
            let key = |c: &VvContact| if slot == 0 { c.a } else { c.b };
            list.extend(edge_germs[i][slot].iter().copied().filter(|he| {
                (0..vv.len()).any(|j| {
                    j != i && key(&vv[j]) == key(&vv[i]) && edge_germs[j][slot].contains(he)
                })
            }));
        }
        if reversed.a.is_empty() && reversed.b.is_empty() {
            continue;
        }
        let (_, a_sectors, b_sectors, records, raw, (sector_records, sector_raw)) =
            &mut classified[i];
        (*records, *raw) = (sector_records.clone(), sector_raw.clone());
        let mut retouched = Vec::new();
        recl::recl_edges(
            records,
            raw,
            a_sectors,
            b_sectors,
            &a,
            &b,
            op,
            &declared,
            band,
            &reversed,
            held_faces(&touches, vv[i].record),
            &mut retouched,
            &tips,
        )?;
        // A reversal moves a germ between flankers, after the touch is
        // read: the pass reads the touches the first did.
        let first = line_touches
            .iter()
            .filter(|(j, _)| *j == i)
            .map(|(_, he)| he);
        if !retouched.iter().eq(first) {
            return Err(BooleanError::ClassificationInvariant {
                what: "a reversed edge pass reads other touches than the first pass at its vertex pair",
            });
        }
    }
    let mut plans = classified
        .iter()
        .map(|(c, a_sectors, b_sectors, records, raw, _)| {
            insert::plan_null_pairs(
                &a, &b, *c, a_sectors, b_sectors, records, raw, &declared, &contacts, op, band,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unused_mut))]
    let mut orbits: Vec<_> = classified
        .iter()
        .map(|(_, a_sectors, b_sectors, ..)| (a_sectors.as_slice(), b_sectors.as_slice()))
        .collect();
    #[cfg(any(test, feature = "test-support"))]
    insert::reverse_when_asked(&mut plans, &mut orbits);
    insert::reconcile_shared(&mut plans, &orbits, &a, &b, band)?;
    let out = insert::mint_plans(&mut a, &mut b, &plans, &orbits, band)?;
    null_edges.extend(out.edges)?;
    null_pairs.extend(out.pairs);
    let NullEdges {
        records: null_edges,
        copies,
    } = null_edges;
    let fates = fate::border_fates(fates, &covered, &null_edges, (&a, &b), |body| {
        crate::census::collinear_overlaps(body, band)
            .map(|lines| lines.into_iter().map(|line| line.edges).collect())
            .map_err(ops::collinear_escalated)
    })?;
    let rest_contacts = decls
        .pairs
        .iter()
        .filter(|d| d.class == GlueClass::Rest)
        .map(|d| (d.a, d.b))
        .filter(|pair| declared.verified.one_carrier.contains(pair))
        .collect();

    a.sweep_and_close();
    b.sweep_and_close();
    let edge_classes = with_copies(edge_classes, &null_edges);
    let unread_partners = with_copies(unread_partners, &null_edges);
    // A vertex read no partner of holds no class: naming reads the
    // list, never a missing row, as the reason. The copies keep it so
    // where a null edge's other end has no pair of its own, as an end
    // `insert::mint_plans` mints after the pair reading never does; an
    // end minted before it (a pierce ring's, `node::mint_nodes`') that
    // met a pair of its own would lend the listed vertex its classes,
    // and this says so in a debug build.
    debug_assert!(
        !edge_classes.iter().any(|c| unread_partners
            .iter()
            .any(|u| (u.operand, u.vertex) == (c.operand, c.vertex))),
        "a vertex read no partner of holds an edge class"
    );
    let kept = glue::touched(
        &declared.verified.coincidences,
        &pending,
        &mints,
        [a_operand, b_operand],
    );
    let coincidences = kept
        .iter()
        .map(|&i| declared.verified.coincidences[i])
        .collect();
    // A face–face row's tangency meets on the faces it names, so its
    // row is kept.
    let glued = glued
        .into_iter()
        .map(|(k, row)| {
            kept.binary_search(&row)
                .map(|at| (k, reduce::index(at)))
                .map_err(|_| BooleanError::JoinDesync {
                    what: "a face–face record's tangency row is not among the op's coincidences",
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BooleanReduction {
        op,
        slit_tips: tips,
        a: carved_a,
        b: carved_b,
        contacts: ReductionContacts { ee, ..contacts },
        null_edges,
        copies,
        null_pairs,
        pierce_rings,
        ring_ties,
        covered: covered
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect(),
        fates,
        rest_contacts,
        coincident,
        mints,
        edge_classes,
        unread_partners,
        coincidences,
        glued,
        tangent_lines: declared.verified.lines.clone(),
        pending,
        sections: declared.sections,
    })
}

/// **The reduction's edge-edge rows**: for each touch the
/// classification read at vertex pair `vv[i]` (`recl::recl_edges`), the
/// two half-edges' edges, where both are lines, recorded as A's edge
/// and B's. The reduction split each edge at every vertex of the other
/// lying on it, so the two edges run between the same two points, each
/// a vertex pair the classification reads, and it reads the touch at
/// both. The row cites what C3's shortest chain asks of a pair two
/// vertex identities imply: the vertex records at its two ends, the two
/// pairs it was read at. Read before the insertion, while every edge
/// still ends at the vertex its row names. A touch along two curved
/// edges writes no row: two arcs leaving a vertex along one tangent
/// need not coincide.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where a half-edge does not resolve or
/// its edge does not end at the pair's vertex;
/// [`BooleanError::ClassificationInvariant`] where a touch is read at
/// other than its two ends: an end with no vertex record is a bound D1
/// never infers.
fn touch_rows<T: Real>(
    a: &Body<T>,
    b: &Body<T>,
    vv: &[Cited<VvContact>],
    touches: &[(usize, [HalfEdgeKey; 2])],
) -> Result<Vec<Cited<EeContact>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    // The edge `he` lies on, which ends at `near`.
    let edge = |body: &Body<T>, he: HalfEdgeKey, near: VertexKey| {
        let edge = body
            .get_half_edge(he)
            .map(|h| h.edge)
            .ok_or_else(|| desync("a touch's half-edge does not resolve"))?;
        match body.edge_vertices(edge) {
            Some((v, w)) if near == v || near == w => Ok(edge),
            Some(_) => Err(desync("a touch's edge does not end at its vertex pair")),
            None => Err(desync("a touch's edge does not resolve")),
        }
    };
    // Each pair of edges, and the vertex pairs it was read at.
    let mut read: Vec<(EeContact, Vec<usize>)> = Vec::new();
    for &(i, [ha, hb]) in touches {
        let ea = edge(a, ha, vv[i].a)?;
        let eb = edge(b, hb, vv[i].b)?;
        if !(crate::census::edge_is_line(a, ea) && crate::census::edge_is_line(b, eb)) {
            continue;
        }
        let pair = EeContact { a: ea, b: eb };
        match read.iter_mut().find(|(p, _)| *p == pair) {
            Some((_, ends)) if ends.contains(&i) => {}
            Some((_, ends)) => ends.push(i),
            None => read.push((pair, vec![i])),
        }
    }
    read.into_iter()
        .map(|(pair, ends)| match ends[..] {
            [here, there] => Ok(Cited::new(pair, vv[here].cites.union(&vv[there].cites))),
            _ => Err(BooleanError::ClassificationInvariant {
                what: "two edges touching along a line are read at other than their two ends",
            }),
        })
        .collect()
}

/// **Each operand's face through a vertex pair's point** that the other
/// operand's vertex of the pair touches, A's then B's
/// ([`recl::HeldFace`]).
fn held_faces<T: Real>(
    touches: &std::collections::BTreeMap<(Operand, VertexKey), Touch<T>>,
    c: VvContact,
) -> [Option<recl::HeldFace<T>>; 2] {
    [(Operand::B, c.b, c.a), (Operand::A, c.a, c.b)].map(|(operand, vertex, partner)| {
        touches.get(&(operand, vertex)).map(|t| recl::HeldFace {
            datum: t.datum,
            operand,
            vertex,
            face: t.face,
            partner,
        })
    })
}

/// A vertex that touches a face and pairs ([`vtxfac::cone_classes`]).
struct Touch<T: Real> {
    /// Its pierce's classes, as first read.
    classes: Vec<(HalfEdgeKey, SideCode)>,
    /// The pierced face's datum they were read against.
    datum: vtxfac::PierceDatum<T>,
    /// Whether a sector of its pierce lies in the face's plane.
    flush: bool,
    /// The pierced face.
    face: FaceKey,
    /// Its own sectors, as its first pair read them (`None` before it):
    /// a re-read pierce writes nothing, so its orbit is the operand's.
    own: Option<Vec<sectors::BoolSector<T>>>,
    /// Its partners, in pair order.
    pairs: Vec<vtxfac::Partner<T>>,
}

/// The reduction's null edges as it records them, both operands, in
/// insertion order, with each operand's copies: every end of a null
/// edge other than the vertex it was minted at, which the null edge
/// minted ([`mev_null_run`](crate::Body::mev_null_run)).
struct NullEdges<T: Real> {
    records: Vec<BoolNullEdgeRecord<T>>,
    copies: [crate::lineage::NullEdgeCopies; 2],
}

impl<T: Real> Default for NullEdges<T> {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            copies: [Operand::A, Operand::B].map(|operand| {
                crate::lineage::NullEdgeCopies::new(crate::lineage::LineageRows::NullEdgeCopies(
                    operand,
                ))
            }),
        }
    }
}

impl<T: Real> NullEdges<T> {
    /// Records `minted`, each copy checked as it is added.
    ///
    /// # Errors
    ///
    /// [`BooleanError::LineageRow`] where a copy is not one the null edge
    /// minted: an earlier record names it.
    fn extend(
        &mut self,
        minted: impl IntoIterator<Item = BoolNullEdgeRecord<T>>,
    ) -> Result<(), BooleanError> {
        for r in minted {
            let copies = &mut self.copies[usize::from(r.operand == Operand::B)];
            for end in [r.attr.below_end, r.attr.above_end] {
                if end != r.at_vertex {
                    copies.push((end, r.at_vertex))?;
                }
            }
            self.records.push(r);
        }
        Ok(())
    }
}

/// The two vertices each null edge joins, `(operand, below end, above
/// end)` in clone keys (`BooleanNaming::null_copies`).
pub(super) fn null_copy_rows<T: Real>(
    null_edges: &[BoolNullEdgeRecord<T>],
) -> Vec<(Operand, VertexKey, VertexKey)> {
    null_edges
        .iter()
        .map(|r| (r.operand, r.attr.below_end, r.attr.above_end))
        .collect()
}

/// A record kept at one operand vertex, which the null edges' copies
/// of that vertex hold too ([`with_copies`]).
trait AtVertex: Copy + Ord {
    /// The operand and vertex it is kept at.
    fn at(&self) -> (Operand, VertexKey);
    /// The same record kept at `vertex` instead.
    fn moved(self, vertex: VertexKey) -> Self;
}

impl AtVertex for EdgePieceClass {
    fn at(&self) -> (Operand, VertexKey) {
        (self.operand, self.vertex)
    }
    fn moved(self, vertex: VertexKey) -> Self {
        Self { vertex, ..self }
    }
}

impl AtVertex for UnreadPartner {
    fn at(&self) -> (Operand, VertexKey) {
        (self.operand, self.vertex)
    }
    fn moved(self, vertex: VertexKey) -> Self {
        Self { vertex, ..self }
    }
}

/// `rows`, kept at every vertex the null edges join a row's vertex to
/// as well: the copies lie on its point, and the join hands each of
/// them some of its edges. An edge class is so read at each copy, and
/// an unread partner listed at each. Sorted and deduplicated.
fn with_copies<T: Real, R: AtVertex>(
    mut rows: Vec<R>,
    null_edges: &[BoolNullEdgeRecord<T>],
) -> Vec<R> {
    loop {
        let mut copied = Vec::new();
        for r in null_edges {
            let (x, y) = (r.attr.below_end, r.attr.above_end);
            for row in rows.iter().filter(|row| row.at().0 == r.operand) {
                for (from, to) in [(x, y), (y, x)] {
                    if row.at().1 == from {
                        copied.push(row.moved(to));
                    }
                }
            }
        }
        let before = rows.len();
        rows.extend(copied);
        rows.sort();
        rows.dedup();
        if rows.len() == before {
            return rows;
        }
    }
}

/// The rows ([`EdgePieceClass`]) for one classified vertex of
/// `operand`'s clone `body`: each edge, by the half-edge leaving
/// `vertex` along it, with its class. The edge is read back through
/// `splits` to the operand edge it lies on.
fn piece_classes<T: Real>(
    body: &Body<T>,
    operand: Operand,
    vertex: VertexKey,
    classes: &[(HalfEdgeKey, SideCode)],
    splits: &Mints,
) -> Result<Vec<EdgePieceClass>, BooleanError> {
    let desync = |what| BooleanError::ClassificationInvariant { what };
    let mut out = Vec::with_capacity(classes.len());
    for &(he, class) in classes {
        let piece = body
            .get_half_edge(he)
            .ok_or_else(|| desync("a classified half-edge does not resolve in its clone"))?
            .edge;
        let starts = body
            .get_edge(piece)
            .ok_or_else(|| desync("a classified edge does not resolve in its clone"))?
            .he_plus
            == he;
        out.push(EdgePieceClass {
            operand,
            vertex,
            edge: splits.root(operand, piece),
            starts,
            class,
        });
    }
    Ok(out)
}

/// **The declaration door**: EVERY glued pair's decision recorded
/// before the op runs, per class. A `Rest` or continuation pair records
/// the margin the glue door decided it on ([`one_carrier_row`]), and no
/// ladder reads it again; a tangency is read down the witness lane
/// ([`verify_tangency`], routed along a rim by [`route_rim`]) for its
/// locus, which the consumers place.
fn verify_glued_contacts<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
    band: Band,
) -> Result<VerifiedGlue<T>, BooleanError> {
    let mut verified = VerifiedGlue::default();
    for &GluedPair {
        a: fa,
        b: fb,
        class,
        decided,
        ..
    } in &decls.pairs
    {
        match class {
            GlueClass::Tangent => {
                // Every refusal is the op's: the door glued this pair on
                // the same reading.
                let spent = DeclarationRead::Spent(GlueClass::Tangent);
                let tangency =
                    glued_tangency(a, fa, b, fb, spent, band).map_err(|f| f.refusal(fa, fb))?;
                if tangency.interiors? {
                    verified.touches.push(GluedTouch {
                        a: fa,
                        b: fb,
                        margin: tangency.margin,
                        row: verified.coincidences.len(),
                    });
                }
                verified
                    .coincidences
                    .push(tangent_row(fa, fb, tangency.margin));
                if let Some(line) = tangency.line {
                    verified.lines.insert((fa, fb), line);
                }
                verified.tangent.insert(
                    (fa, fb),
                    TangentCert {
                        kinds: tangency.kinds,
                        line: matches!(tangency.locus, rim_wedge::Locus::Line { .. }),
                    },
                );
            }
            GlueClass::Rest | GlueClass::Continuation => {
                let row = one_carrier_row(a, fa, b, fb, class, decided)?;
                verified.one_carrier.insert((fa, fb));
                verified.coincidences.push(row);
            }
        }
    }
    Ok(verified)
}

/// **The given pairs as verified** (DM4: a union's fold step decides no
/// carrier pair again, so a piece is never judged at its own extent).
/// Each pair enters the set its class names. A given tangency's locus
/// is the line the deciding operation derived and placed, handed on
/// whole ([`GivenDecision`]), or a rim where it decided none. Where that
/// line runs through both of the step's faces' interiors
/// ([`locus_through_both_faces`], over where the faces' extents along it
/// meet, a placement), the touch is a face–face row
/// ([`VerifiedGlue::touches`]), and the step records the given decision
/// as its row, restated on its margin, for the record to cite. It
/// records no other row for a given pair.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where a given face has no
/// surface or a given pair carries no decision; the placement's own
/// refusal.
fn given_verdicts<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
    band: Band,
) -> Result<VerifiedGlue<T>, BooleanError> {
    let invariant = |what| BooleanError::ClassificationInvariant { what };
    let kind = |body: &Body<T>, f| {
        body.get_face(f)
            .and_then(|face| body.get_surface(face.surface))
            .map(geom::Surface::kind)
            .ok_or(invariant("a given pair's face has no surface"))
    };
    let pad = witness_pad::<T>(band);
    let mut verified = VerifiedGlue::default();
    for &GluedPair {
        a: fa,
        b: fb,
        class,
        given,
        ..
    } in &decls.pairs
    {
        match class {
            GlueClass::Tangent => {
                let kinds = (kind(a, fa)?, kind(b, fb)?);
                let given = given.ok_or(invariant("a given pair carries no decision"))?;
                if let Some(line) = given.line {
                    let (origin, dir) = line;
                    let (_, overlap) = witness_span(a, fa, b, fb, (origin, dir), band, pad)?;
                    if locus_through_both_faces(
                        [(a, fa, Operand::A, kinds.0), (b, fb, Operand::B, kinds.1)],
                        &geom::Curve3::Line { origin, dir },
                        overlap,
                        DeclarationRead::Spent(GlueClass::Tangent),
                        band,
                    )? {
                        verified.touches.push(GluedTouch {
                            a: fa,
                            b: fb,
                            margin: given.margin,
                            row: verified.coincidences.len(),
                        });
                        verified
                            .coincidences
                            .push(tangent_row(fa, fb, given.margin));
                        verified.lines.insert((fa, fb), line);
                    }
                }
                verified.tangent.insert(
                    (fa, fb),
                    TangentCert {
                        kinds,
                        line: given.line.is_some(),
                    },
                );
            }
            GlueClass::Rest | GlueClass::Continuation => {
                verified.one_carrier.insert((fa, fb));
            }
        }
    }
    Ok(verified)
}

/// The refusal of a glued tangency a later reading of the same faces
/// contradicts ([`BooleanError::ContactContradicted`]).
pub(super) fn tangency_contradicted(
    fa: FaceKey,
    fb: FaceKey,
    fact: Option<Contradiction>,
    margin: Indeterminate,
) -> BooleanError {
    BooleanError::ContactContradicted {
        pair: crate::contact::ContactPair {
            a: fa,
            b: fb,
            class: GlueClass::Tangent,
        },
        fact,
        margin,
    }
}

/// The row a glued tangency records: the tangency the witness lane
/// decided along its locus, on the margin that decided it.
fn tangent_row(fa: FaceKey, fb: FaceKey, margin: MarginDiag) -> crate::Coincidence {
    crate::Coincidence {
        cells: [
            crate::RowCell::face(Operand::A, fa),
            crate::RowCell::face(Operand::B, fb),
        ],
        more: [None; 3],
        relation: crate::Relation::SurfaceTangent,
        site: crate::DecisionSite::TangentWitness,
        margin,
        discharge: crate::Discharge::of(margin),
    }
}

/// **The row a one-carrier pair the glue door decided records**: its
/// class's relation, on the margin the door decided it on, at the
/// ladder that read it.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a pair that carries no
/// decided margin: only the glue door glues a one-carrier pair, and it
/// glues one only on a margin.
fn one_carrier_row<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    class: GlueClass,
    margin: Option<MarginDiag>,
) -> Result<crate::Coincidence, BooleanError> {
    let margin = margin.ok_or(BooleanError::ClassificationInvariant {
        what: "the declaration door: a one-carrier glued pair carries no decided margin",
    })?;
    Ok(crate::Coincidence {
        cells: [
            crate::RowCell::face(Operand::A, fa),
            crate::RowCell::face(Operand::B, fb),
        ],
        more: [None; 3],
        relation: class.relation(),
        site: decided_site(a, fa, b, fb),
        margin,
        discharge: crate::Discharge::of(margin),
    })
}

/// Which ladder decided a one-carrier pair: the plane ladder where both
/// faces are planes, the carrier ladder otherwise.
fn decided_site<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
) -> crate::DecisionSite {
    let plane = |body: &Body<T>, f| {
        matches!(
            carrier_pair::face_carrier(body, f),
            Some(carrier_eq::CarrierDesc::Plane { .. })
        )
    };
    if plane(a, fa) && plane(b, fb) {
        crate::DecisionSite::PlaneLadder
    } else {
        crate::DecisionSite::CarrierLadder
    }
}

/// **Why the door's tangency check did not certify a pair.**
pub(crate) enum TangencyFailure {
    /// Definite counter-evidence: the pair is not tangent along a locus
    /// (one carrier, apart, crossing, normals independent, a bad
    /// witness), labelled with what refused it.
    Contradicted {
        /// The carrier fact, where one did it.
        fact: Option<Contradiction>,
        /// The finding's reading.
        margin: Indeterminate,
    },
    /// No witness locus derives for the pair, or the jet certificate
    /// does not reach it.
    Unsupported,
    /// A refusal no reading of the pair settles: an in-band margin, an
    /// unreadable face.
    Refused(BooleanError),
    /// Along a rim, a junction the routing places outside what the zip
    /// builds (a cusp, or one it cannot place), refused as itself.
    Unrouted(BooleanError),
}

impl TangencyFailure {
    /// The refusal of a glued tangency the check failed.
    pub(crate) fn refusal(self, fa: FaceKey, fb: FaceKey) -> BooleanError {
        match self {
            Self::Contradicted { fact, margin } => tangency_contradicted(fa, fb, fact, margin),
            Self::Unsupported => BooleanError::TangencyUnsupported,
            Self::Refused(e) | Self::Unrouted(e) => e,
        }
    }
}

/// What the door's tangency check certified of a pair.
pub(crate) struct Tangency<T: Real> {
    /// The two carriers' kinds, A's first.
    pub(crate) kinds: (geom::SurfaceKind, geom::SurfaceKind),
    /// The locus the pair was verified along.
    pub(crate) locus: rim_wedge::Locus<T>,
    /// Whether the locus is a line some point of which lies inside both
    /// faces ([`locus_through_both_faces`]), so the two faces' interiors
    /// meet; that read's own refusal is the inner one, consumed only by
    /// the op.
    pub(crate) interiors: Result<bool, BooleanError>,
    /// The margin that decided the touch (the pair's separation along
    /// the witness, `contact_verify`'s `contact_tangent_touch`),
    /// standing jointly for the locus it was read along (the line's gap,
    /// or the shared rim's identity).
    pub(crate) margin: MarginDiag,
    /// The line locus as decided, to be handed on; none along a rim.
    pub(crate) line: Option<DecidedLine<T>>,
}

/// What [`GluedPairs`] keeps of a glued tangency: its carriers' kinds
/// and whether its locus is a line, which is what the one-sided cover
/// reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TangentCert {
    /// The two carriers' kinds, A's first.
    pub(crate) kinds: (geom::SurfaceKind, geom::SurfaceKind),
    /// The locus is a line from the witness lane, not a rim.
    pub(crate) line: bool,
}

/// **The closed-form tangent line of two faces' carriers** (DEV-1,
/// [`geom_brep::tangent_locus`]: plane × cylinder along a ruling,
/// parallel cylinders) and both faces' extents along it: the one witness
/// a glued tangency ([`verify_tangency`]) and a
/// face–face record on two carriers at rest ([`tangent_pair_at_rest`])
/// are verified along.
pub(crate) struct TangentLine<T: Real> {
    /// The line, `origin + t·dir` with `dir` unit.
    pub(crate) carrier: geom::Curve3<T>,
    /// The union of the two faces' extents along it.
    pub(crate) span: (T, T),
    /// Where both faces' extents along it meet: empty where `.0 > .1`.
    pub(crate) overlap: (T, T),
    /// The margin that decided the carriers tangent.
    pub(crate) gap: MarginDiag,
}

/// [`TangentLine`] of `fa` on `a` and `fb` on `b`, their surfaces given.
///
/// # Errors
///
/// The outer [`BooleanError`] where a face's extent or boundary cannot be
/// read; the inner [`geom_brep::TangentLocusError`] where the carriers
/// derive no line.
fn tangent_line<T: Decide>(
    (a, fa, sa): (&Body<T>, FaceKey, &geom::Surface<T>),
    (b, fb, sb): (&Body<T>, FaceKey, &geom::Surface<T>),
    band: Band,
) -> Result<Result<TangentLine<T>, geom_brep::TangentLocusError>, BooleanError> {
    let reach = carrier_pair::pair_extent(a, fa, b, fb, band)
        .map_err(unreadable_extent)?
        .reach;
    // Each face's padded reach along a direction, read in the frame it
    // aims, so a face that does not reach the locus settles an in-band
    // reading of the carriers.
    let pad = witness_pad::<T>(band);
    let extents = |across| {
        let frame = boxes::BoxFrame::aimed(across);
        [(a, fa), (b, fb)].map(|(body, face)| {
            crate::census::face_reach_in(body, face, band, &frame)
                .map(|(lo, hi)| (lo.x - pad, hi.x + pad))
        })
    };
    match geom_brep::tangent_locus_of_faces(sa, sb, reach, band, extents) {
        Ok(geom_brep::TangentLocus::Line { origin, dir, gap }) => {
            let (span, overlap) = witness_span(a, fa, b, fb, (origin, dir), band, pad)?;
            Ok(Ok(TangentLine {
                carrier: geom::Curve3::Line { origin, dir },
                span,
                overlap,
                gap,
            }))
        }
        Err(e) => Ok(Err(e)),
    }
}

/// What a face–face record on two carriers reads at rest
/// ([`tangent_pair_at_rest`]).
pub(crate) enum RestTangent<T: Real> {
    /// The two faces' extents along their tangent line are definitely
    /// disjoint or meet at one point, as the glue door reads them
    /// ([`locus_through_both_faces`]): no span of the line is in both,
    /// so their interiors do not meet along it.
    Apart,
    /// The two carriers derive no closed-form tangent line
    /// ([`geom_brep::TangentLocusError::Unsupported`]): the pair is
    /// outside the line lane.
    NoLine,
    /// The faces' extents along the line meet over `overlap`; whether
    /// the line lies in the two faces there is the caller's to read
    /// ([`rest_line_in_both_faces`]), and only then whether the table
    /// holds along it ([`tangent_table_at_rest`]).
    Line {
        /// The tangent line.
        carrier: geom::Curve3<T>,
        /// The parameter span both faces' extents hold.
        overlap: (T, T),
    },
}

/// **A face–face record on two carriers, read at rest** (C3): the
/// closed-form line their carriers derive ([`tangent_line`]) and the two
/// faces' extents along it. The `Tangent` table along it, material
/// sides included, is [`tangent_table_at_rest`], which reads only a
/// line known to run through both faces ([`RestLineInBothFaces`]): a
/// pair the line misses does not meet, whatever its sides.
///
/// # Errors
///
/// [`crate::contact::ContactRefusal::Contradicted`] where the carriers
/// are definitely not tangent, labelled with the row that refused it;
/// `Escalated` where a margin is in band; and `NotCertifiable` where a
/// face cannot be read (naming why).
pub(crate) fn tangent_pair_at_rest<T: Decide>(
    body: &Body<T>,
    fa: FaceKey,
    fb: FaceKey,
    band: Band,
) -> Result<RestTangent<T>, crate::contact::ContactRefusal> {
    use crate::contact::ContactRefusal;
    let ((sa, _), (sb, _)) = (record_face(body, fa)?, record_face(body, fb)?);
    let line = tangent_line((body, fa, sa), (body, fb, sb), band).map_err(|e| {
        ContactRefusal::NotCertifiable {
            what: match e {
                BooleanError::InvalidInput { what, .. } => what,
                _ => "a face of the face-face record has no extent to read",
            },
        }
    })?;
    let line = match line {
        Ok(line) => line,
        Err(geom_brep::TangentLocusError::Escalated(diag)) => {
            return Err(ContactRefusal::Escalated { diag });
        }
        Err(geom_brep::TangentLocusError::NotTangent { predicate, .. }) => {
            return Err(ContactRefusal::Contradicted {
                diag: Indeterminate {
                    margin: MarginDiag::INVALID,
                    band,
                    predicate: Some(predicate),
                    terminal_sliver: false,
                },
            });
        }
        Err(geom_brep::TangentLocusError::Unsupported { .. }) => {
            return Ok(RestTangent::NoLine);
        }
    };
    let (t0, t1) = line.overlap;
    match tangent_extents_meet((t0, t1), band) {
        Ok(true) => {}
        Ok(false) => return Ok(RestTangent::Apart),
        Err(diag) => return Err(ContactRefusal::Escalated { diag }),
    }
    Ok(RestTangent::Line {
        carrier: line.carrier,
        overlap: line.overlap,
    })
}

/// A face of a face–face record on two carriers: its surface and sense.
fn record_face<T: Real>(
    body: &Body<T>,
    f: FaceKey,
) -> Result<(&geom::Surface<T>, bool), crate::contact::ContactRefusal> {
    body.get_face(f)
        .and_then(|face| Some((body.get_surface(face.surface)?, face.sense)))
        .ok_or(crate::contact::ContactRefusal::NotCertifiable {
            what: "a face of the face-face record has no surface",
        })
}

/// A record's tangent line known to run through both faces' interiors
/// ([`rest_line_in_both_faces`], the only constructor): what
/// [`tangent_table_at_rest`] reads, so the sides are never read for a
/// pair the line misses.
pub(crate) struct RestLineInBothFaces<'b, T: Real> {
    body: &'b Body<T>,
    faces: [FaceKey; 2],
    carrier: geom::Curve3<T>,
    overlap: (T, T),
}

/// [`line_through_both_faces`] for a face–face record's line at rest
/// ([`RestTangent::Line`]): `Some` where the line runs through both
/// faces' interiors, `None` where it does not.
///
/// # Errors
///
/// What [`line_through_both_faces`] leaves unread.
pub(crate) fn rest_line_in_both_faces<'b, T: Decide, E>(
    body: &'b Body<T>,
    faces: [FaceKey; 2],
    carrier: geom::Curve3<T>,
    overlap: (T, T),
    band: Band,
    unread: impl Fn(usize, FaceUnread, &mut Vec<E>),
) -> Result<Option<RestLineInBothFaces<'b, T>>, Vec<E>> {
    let through =
        line_through_both_faces(faces.map(|f| (body, f)), &carrier, overlap, band, unread)?;
    Ok(through.then_some(RestLineInBothFaces {
        body,
        faces,
        carrier,
        overlap,
    }))
}

/// The C4 `Tangent` table of two faces on two carriers along their
/// tangent line, where it runs through both faces
/// ([`RestLineInBothFaces`]) ([`contact_verify::tangent_locus_relation`]):
/// the touch and the material sides, read as the glue door reads a
/// tangency, whether a record cites the pair or the census discovers it.
/// `Ok` carries the touch's margin.
///
/// # Errors
///
/// [`crate::contact::ContactRefusal::Contradicted`] where the table
/// contradicts the touch or the sides (`contact_tangent_opposed`,
/// `contact_tangent_overlap`); `Escalated` where a margin is in band;
/// `NotCertifiable` where a face has no surface.
pub(crate) fn tangent_table_at_rest<T: Decide>(
    line: &RestLineInBothFaces<'_, T>,
    band: Band,
) -> Result<MarginDiag, crate::contact::ContactRefusal> {
    let [fa, fb] = line.faces;
    let ((sa, sense_a), (sb, sense_b)) = (record_face(line.body, fa)?, record_face(line.body, fb)?);
    let (t0, t1) = line.overlap;
    contact_verify::tangent_locus_relation(sa, sense_a, sb, sense_b, &line.carrier, t0, t1, band)
}

/// **The glue door's tangency**: the check ([`verify_tangency`]), and
/// along a rim the routing ([`route_rim`]). The door
/// ([`glue::decided_glue`]) reads a refusal no reading settles
/// as the operation's and every other failure as no tangency; the op
/// ([`verify_glued_contacts`]) refuses each failure under its read
/// ([`TangencyFailure::refusal`]).
pub(crate) fn glued_tangency<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    read: DeclarationRead,
    band: Band,
) -> Result<Tangency<T>, TangencyFailure> {
    let tangency = verify_tangency(a, fa, b, fb, read, band)?;
    if let rim_wedge::Locus::Rim(rim) = tangency.locus {
        route_rim(a, fa, b, fb, rim, read, band)?;
    }
    Ok(tangency)
}

/// [`glued_tangency`] as the op reads it, for the unit rows that drive it.
#[cfg(test)]
pub(crate) fn verify_tangent_declaration<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    band: Band,
) -> Result<(), BooleanError> {
    let spent = DeclarationRead::Spent(GlueClass::Tangent);
    glued_tangency(a, fa, b, fb, spent, band)
        .map(|_| ())
        .map_err(|f| f.refusal(fa, fb))
}

/// A glued face's carrier and orientation bit, read off one lookup: a
/// key that does not resolve refuses typed here, so no later stage
/// answers from a sense it invented.
fn glued_face<T: Real>(
    body: &Body<T>,
    f: FaceKey,
    operand: Operand,
) -> Result<(geom::Surface<T>, bool), BooleanError> {
    let invalid = |what| BooleanError::InvalidInput { operand, what };
    let face = body
        .get_face(f)
        .ok_or_else(|| invalid("a glued face's key does not resolve"))?;
    let surface = body
        .get_surface(face.surface)
        .ok_or_else(|| invalid("a glued face lost its surface"))?;
    Ok((surface.clone(), face.sense))
}

/// **The glue door's tangency check** (C4's `Tangent`, claiming nothing
/// of the materials): whether `fa` and `fb` lie on distinct carriers
/// tangent along a locus the witness lane certifies.
///
/// 1. **The conformal screen** (detector posture): a pair the carrier
///    ladder calls one carrier — structurally (rung 1) or geometrically
///    (rung 4's coincidence refusal) — is a `Rest` pair or a
///    continuation, not a tangency, labelled with what failed (the two
///    faces share one carrier).
/// 2. **The witness locus.** The DEV-1 closed-form line where it
///    derives ([`geom_brep::tangent_locus`]: plane×cylinder along a
///    ruling, parallel cylinders); a definitely-apart or crossing pair
///    is contradicted, labelled with the row that refused it. Elsewhere,
///    the rim circle the two faces share; with neither the pair is
///    [`TangencyFailure::Unsupported`].
/// 3. **The tangency table** ([`contact_verify::tangency_reading`])
///    along the locus over the pair's honest extent: the touch, the
///    witness on both surfaces, first-order parallelism and a definite
///    second-order margin. Which way the outward normals point, and
///    whether the crescent between the sheets is void or shared, are
///    read and claimed nowhere: the lump, the junction's wedge and the
///    cover derive what each needs (C7).
///
/// Escalations carry `read`, what the caller holds of the pair.
fn verify_tangency<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    read: DeclarationRead,
    band: Band,
) -> Result<Tangency<T>, TangencyFailure> {
    use TangencyFailure::{Contradicted, Refused, Unsupported};
    // Display-only labels: the deciding `decide` calls ran under their
    // own names, so these name the FINDING for the reader and never
    // enter the K funnel (the `contact_rest_senses_opposed` precedent).
    let label = |predicate| Indeterminate {
        margin: MarginDiag::INVALID,
        band,
        predicate: Some(predicate),
        terminal_sliver: false,
    };
    // 1. The conformal screen (detector posture).
    match carrier_pair::carrier_pair_relation(a, fa, b, fb, band) {
        // No description to compare: the witness lane below answers
        // for the kinds it holds.
        Ok(Ok(CarrierRelation::Distinct)) | Err(carrier_pair::PairUnread::OutsideInventory) => {}
        Ok(Ok(CarrierRelation::SameOriented | CarrierRelation::SameOpposite)) => {
            return Err(Contradicted {
                fact: Some(Contradiction::OneCarrier),
                margin: label("tangency_conformal"),
            });
        }
        // Near one carrier, in band: the diag keeps the predicate that
        // measured it and its value. A poisoned datum refuses as itself.
        Ok(Err(carrier_eq::CarrierEqError::Undecided { coincidence, .. })) => {
            let diag = readable_coincidence(coincidence, [(Operand::A, fa), (Operand::B, fb)])
                .map_err(Refused)?;
            return Err(Contradicted {
                fact: Some(Contradiction::OneCarrier),
                margin: diag,
            });
        }
        Ok(Err(carrier_eq::CarrierEqError::Escalated { rung, diag })) => {
            return Err(Refused(BooleanError::plane_identity(
                rung,
                PlaneDoor::Screen(read),
                diag,
            )));
        }
        Err(carrier_pair::PairUnread::Extent(face)) => {
            return Err(Refused(unreadable_extent(face)));
        }
    }
    let (sa, sense_a) = glued_face(a, fa, Operand::A).map_err(Refused)?;
    let (sb, sense_b) = glued_face(b, fb, Operand::B).map_err(Refused)?;
    let kinds = (sa.kind(), sb.kind());
    // 2. The witness locus.
    let mut overlap = None;
    let mut decided = None;
    let (carrier, t0, t1, locus, located) =
        match tangent_line((a, fa, &sa), (b, fb, &sb), band).map_err(Refused)? {
            Ok(line) => {
                overlap = Some(line.overlap);
                if let geom::Curve3::Line { origin, dir } = line.carrier {
                    decided = Some((origin, dir));
                }
                let locus = match line.carrier {
                    geom::Curve3::Line { origin, dir } => rim_wedge::Locus::Line {
                        origin,
                        dir: dir / dir.norm(),
                    },
                    _ => {
                        return Err(Refused(BooleanError::ClassificationInvariant {
                            what: "the glue door: a tangent line's carrier is not a line",
                        }));
                    }
                };
                (line.carrier, line.span.0, line.span.1, locus, line.gap)
            }
            Err(geom_brep::TangentLocusError::Escalated(diag)) => {
                return Err(Refused(BooleanError::coincidence(
                    Coincide::TangentLocus,
                    read,
                    diag,
                )));
            }
            Err(geom_brep::TangentLocusError::NotTangent { predicate, .. }) => {
                return Err(Contradicted {
                    fact: None,
                    margin: label(predicate),
                });
            }
            Err(geom_brep::TangentLocusError::Unsupported { .. }) => {
                // An UNDECIDABLE rim identity escalates typed rather than
                // reading as "no rim here": only the second means the
                // geometry was examined and cleared.
                let (rim, identity) = rim_wedge::shared_rim(a, fa, b, fb, band)
                    .map_err(|diag| Refused(BooleanError::coincidence(Coincide::Rim, read, diag)))?
                    .ok_or(Unsupported)?;
                let circle = geom::Curve3::Circle {
                    center: rim.center,
                    axis: rim.axis,
                    radius: rim.radius,
                    u_ref: rim.u_ref,
                };
                (
                    circle,
                    T::zero(),
                    T::from_f64(core::f64::consts::TAU),
                    rim_wedge::Locus::Rim(rim),
                    identity,
                )
            }
        };
    // 3. The tangency table along the witness.
    use crate::contact::ContactRefusal;
    use contact_verify::TableRefusal as Table;
    let margin = match contact_verify::tangency_reading(
        &sa, sense_a, &sb, sense_b, &carrier, t0, t1, band,
    ) {
        // The reading is the touch's; it is a theorem only where the
        // locus it was read along is one too.
        Ok(touch) => touch.jointly(located),
        Err(Table::Refused(ContactRefusal::Contradicted { diag, .. })) => {
            return Err(Contradicted {
                fact: None,
                margin: diag,
            });
        }
        Err(Table::Refused(ContactRefusal::Escalated { diag })) => {
            return Err(Refused(BooleanError::coincidence(
                Coincide::Contact,
                read,
                diag,
            )));
        }
        // A pair in the lane whose span bound alone the finest schedule
        // leaves past the band is the lane's to refuse, typed, rather
        // than no tangency: it never meets a later stage unglued (C8).
        Err(Table::SagUnreached) => {
            return Err(Refused(BooleanError::TangencyUnsupported));
        }
        Err(Table::Refused(ContactRefusal::NotCertifiable { .. })) => return Err(Unsupported),
    };
    let interiors = match overlap {
        Some(both) => locus_through_both_faces(
            [(a, fa, Operand::A, kinds.0), (b, fb, Operand::B, kinds.1)],
            &carrier,
            both,
            read,
            band,
        ),
        None => Ok(false),
    };
    Ok(Tangency {
        kinds,
        locus,
        interiors,
        margin,
        line: decided,
    })
}

/// **The rim routing of a tangency along a circle** (C7): the material
/// wedge of the junction the two faces make, read off the operands'
/// outward normals and which way each face leaves the rim
/// ([`rim_wedge::departures`]). Wedge π with the faces leaving on
/// opposite sides is the smooth seam, which the zip mints, and the only
/// rim the glue door glues; the cusp family has no locus arm for a
/// circle ([`BooleanError::RimCuspArmUnbuilt`]); a junction the routing
/// cannot place is outside what the classification acts on
/// ([`BooleanError::TangencyUnsupported`]). The reading is the
/// operands', the same under every op: the junction the result carries
/// per op needs the membership, which this stage does not hold (filed:
/// `work/contacthold/a-touch-only-rim-with-no-derivable-locus-refuses-every-op-as-a-cusp.md`).
/// The wedge is levered at the farther of the two faces' extents, each
/// read in its own operand ([`fn@crate::wedge_reach`]).
fn route_rim<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    rim: rim_wedge::Rim<T>,
    read: DeclarationRead,
    band: Band,
) -> Result<(), TangencyFailure> {
    use TangencyFailure::{Refused, Unrouted};
    let unplaced = Unrouted(BooleanError::TangencyUnsupported);
    let in_band = |diag| Refused(BooleanError::coincidence(Coincide::Rim, read, diag));
    let (sa, sense_a) = glued_face(a, fa, Operand::A).map_err(Refused)?;
    let (sb, sense_b) = glued_face(b, fb, Operand::B).map_err(Refused)?;
    let circle = geom::Curve3::Circle {
        center: rim.center,
        axis: rim.axis,
        radius: rim.radius,
        u_ref: rim.u_ref,
    };
    let whole = (T::zero(), T::tau());
    let reach = crate::wedge_reach(a, [fa; 2], &circle, whole)
        .max(crate::wedge_reach(b, [fb; 2], &circle, whole));
    let routing = rim_wedge::classify_shared_rim(&sa, sense_a, &sb, sense_b, rim, reach, band)
        .map_err(in_band)?;
    match routing {
        rim_wedge::RimRouting::Seam => {
            match rim_wedge::departures(a, fa, b, fb, rim_wedge::Locus::Rim(rim), band)
                .map_err(in_band)?
            {
                rim_wedge::Departure::Opposite => Ok(()),
                rim_wedge::Departure::Same
                | rim_wedge::Departure::Mixed
                | rim_wedge::Departure::NoEdge
                | rim_wedge::Departure::Untouched
                | rim_wedge::Departure::EdgeKindUnread => Err(unplaced),
            }
        }
        rim_wedge::RimRouting::Cusp(wedge) => Err(Unrouted(BooleanError::RimCuspArmUnbuilt {
            pair: crate::contact::ContactPair {
                a: fa,
                b: fb,
                class: GlueClass::Tangent,
            },
            wedge,
        })),
        // The table verified the pair tangent with a definite
        // second-order margin, so neither a crossing nor an osculating
        // lamina remains for the routing to find.
        rim_wedge::RimRouting::Transverse | rim_wedge::RimRouting::Lamina => {
            Err(Refused(BooleanError::ClassificationInvariant {
                what: "rim routing: a verified tangency read as a crossing or a lamina",
            }))
        }
    }
}

/// The pad a face's reach along a witness line is widened by: the
/// band's sweep pad, a tolerance.
fn witness_pad<T: Real>(band: Band) -> T {
    T::from_f64(boxes::sweep_pad(band))
}

/// The parameter spans of the line `origin + t·dir` (`dir` unit) the
/// two faces reach along it: their union, the extent a witness along
/// it is metered over, and their overlap, where both faces' extents
/// meet ([`locus_through_both_faces`]). Each face's span holds every
/// point of the face: its boundary vertices' projections and, for a
/// circle or ellipse edge, its carrier's whole reach along the line
/// (an arc can bulge past its ends); a face with an edge of another
/// kind spans its box along the line, padded by `pad`
/// ([`crate::census::face_reach_in`]). So extents meeting at one point
/// meet at an end of each, where no interiors meet.
#[allow(clippy::type_complexity)]
fn witness_span<T: Decide>(
    a: &Body<T>,
    fa: FaceKey,
    b: &Body<T>,
    fb: FaceKey,
    (origin, dir): (Point3<T>, geom_core::Vec3<T>),
    band: Band,
    pad: T,
) -> Result<((T, T), (T, T)), BooleanError> {
    let along = |p: Point3<T>| (p - origin).dot(dir);
    let span = |body: &Body<T>, f, operand| -> Result<(T, T), BooleanError> {
        let invalid = |what| BooleanError::InvalidInput { operand, what };
        let mut ts: Vec<T> = face_boundary_points(body, f, operand)?
            .into_iter()
            .map(along)
            .collect();
        match conic_reach(body, f, along, dir) {
            Some(extra) => ts.extend(extra),
            None => {
                let aim = geom_core::UnitVec3::new(dir, "bool_tangent_line_aim", band)
                    .map_err(|_| invalid("the tangent line has no direction"))?;
                let (lo, hi) =
                    crate::census::face_reach_in(body, f, band, &boxes::BoxFrame::aimed(aim))
                        .ok_or(invalid(
                            "a face of the pair has no reach along its tangent line",
                        ))?;
                let at = along(Point3::origin());
                ts.extend([lo.x + at - pad, hi.x + at + pad]);
            }
        }
        let mut ts = ts.into_iter();
        let first = ts.next().ok_or(invalid(
            "declared face pair has no boundary vertex to meter the tangent witness",
        ))?;
        Ok(ts.fold((first, first), |(lo, hi), t| (t.min(lo), t.max(hi))))
    };
    let (sa, sb) = (span(a, fa, Operand::A)?, span(b, fb, Operand::B)?);
    Ok((
        (sa.0.min(sb.0), sa.1.max(sb.1)),
        (sa.0.max(sb.0), sa.1.min(sb.1)),
    ))
}

/// The reach along `dir` (unit; `along` the parameter of a point) of
/// each circle and ellipse edge of `f`'s boundary, its carrier's two
/// extremes; `None` where an edge is neither one nor a line, or the
/// boundary cannot be walked.
fn conic_reach<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    along: impl Fn(Point3<T>) -> T,
    dir: geom_core::Vec3<T>,
) -> Option<Vec<T>> {
    let face = body.get_face(f)?;
    let mut out = Vec::new();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(lk)?.boundary else {
            continue;
        };
        for he in body.loop_cycle(first)? {
            let edge = body.get_edge(body.get_half_edge(he)?.edge)?;
            let (center, reach) = match *body.get_curve_geom(edge.curve)?.certified()?.carrier() {
                geom::Curve3::Line { .. } => continue,
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => (center, radius * axis.cross(dir).norm()),
                geom::Curve3::Ellipse {
                    center,
                    axis,
                    semi_u,
                    semi_v,
                    u_ref,
                } => (
                    center,
                    ((semi_u * u_ref.dot(dir)).powi(2)
                        + (semi_v * axis.cross(u_ref).dot(dir)).powi(2))
                    .sqrt(),
                ),
                _ => return None,
            };
            out.extend([along(center) - reach, along(center) + reach]);
        }
    }
    Some(out)
}

/// Whether two faces' extents along their tangent line meet along a
/// length: the one gate the op's interiors read
/// ([`locus_through_both_faces`]) and the census's face–face confirm
/// ([`tangent_pair_at_rest`]) pass before reading the line. Extents
/// meeting at one point meet at an end of each, on both faces'
/// boundaries, where the interiors do not meet: `false`, as for extents
/// apart.
///
/// # Errors
///
/// The overlap's length in band.
pub(crate) fn tangent_extents_meet<T: Decide>(
    overlap: (T, T),
    band: Band,
) -> Result<bool, Indeterminate> {
    Ok(crate::validate::decide(
        "bool_tangent_line_overlap",
        Margin::of(overlap.1 - overlap.0),
        band,
    )? == Sign::Positive)
}

/// Whether a verified tangent line runs through both faces' interiors
/// over `overlap`: the op's read, [`line_through_both_faces`] under the
/// op's refusals. A line along a face's boundary is the edge form of
/// the touch, which the edge records carry.
///
/// # Errors
///
/// The overlap's length in band ([`tangent_extents_meet`]), or what
/// [`line_through_both_faces`] leaves unread: a placement margin in
/// band, the containment door's own refusal, or a curved trim the chart
/// door gives no verdict for ([`BooleanError::CurvedBooleanUnsupported`]).
fn locus_through_both_faces<T: Decide>(
    faces: [(&Body<T>, FaceKey, Operand, geom::SurfaceKind); 2],
    locus: &geom::Curve3<T>,
    overlap: (T, T),
    read: DeclarationRead,
    band: Band,
) -> Result<bool, BooleanError> {
    let refused = |diag| BooleanError::coincidence(Coincide::TangentLocus, read, diag);
    if !tangent_extents_meet(overlap, band).map_err(refused)? {
        return Ok(false);
    }
    line_through_both_faces(
        faces.map(|(body, face, ..)| (body, face)),
        locus,
        overlap,
        band,
        |i, why, held: &mut Vec<BooleanError>| {
            let (_, face, operand, kind) = faces[i];
            held.push(match why {
                FaceUnread::Trim => BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind,
                },
                FaceUnread::Placement(contain::PlacementError::OnPlane(diag)) => refused(diag),
                FaceUnread::Placement(contain::PlacementError::Door(e)) => {
                    reduce::esc(e, operand, face)
                }
            });
        },
    )
    .map_err(|held| {
        held.into_iter()
            .next()
            .unwrap_or(BooleanError::ClassificationInvariant {
                what: "a held tangent-line station with nothing pushed",
            })
    })
}

/// Why one face left a point of a tangent line unread
/// ([`line_through_both_faces`]).
pub(crate) enum FaceUnread {
    /// A curved trim the chart door gives no verdict for.
    Trim,
    /// The placement's own refusal.
    Placement(contain::PlacementError),
}

/// **Whether a tangent line runs through two faces' interiors**: some
/// point of it over `overlap` lies strictly inside both faces
/// ([`contain::face_placement`], a point off a face's carrier out). The
/// one reading the op's glue door ([`locus_through_both_faces`]) and
/// the census's face–face confirm (`census::confirm_face_pair`) make
/// of it.
///
/// The overlap is cut wherever either face's boundary can meet the line
/// ([`boundary_cuts`]), so no boundary crosses the open run between two
/// cuts and one station, its middle, answers for each run: the reading
/// is exact wherever both trims' carriers meet the line in closed form.
/// Where one does not (a spiric or spline edge), where a meeting could
/// not be placed (its edge's swing or its conic's rise in band), or
/// where the locus is not a line, the certificate schedule's parameters
/// are cuts too, and a run of that face's interior shorter than their
/// spacing can be missed.
/// A run inside the zero band is one cut met twice and is not read; a
/// run in band is read, and may hold.
///
/// Each station is read by both faces under [`crate::census::in_both`]:
/// either face deciding it out settles it, both in answers `true`. A
/// station neither decides out but one leaves unread is held, and only
/// where no station answers are the first held station's refusals
/// returned, so a run in band of one face's boundary does not stop the
/// line answering at the next.
///
/// `unread(i, why, held)` pushes face `i`'s refusal at a station.
///
/// # Errors
///
/// The first held station's refusals, where no station answers `true`.
pub(crate) fn line_through_both_faces<T: Decide, E>(
    faces: [(&Body<T>, FaceKey); 2],
    locus: &geom::Curve3<T>,
    overlap: (T, T),
    band: Band,
    unread: impl Fn(usize, FaceUnread, &mut Vec<E>),
) -> Result<bool, Vec<E>> {
    use contain::CurvedPlacement;
    let inside = |i: usize, p, held: &mut Vec<E>| {
        let (body, face) = faces[i];
        let why = match contain::face_placement(body, face, p, band, "bool_tangent_line_in_face") {
            Ok(CurvedPlacement::Trim(Some(at))) => return Some(at == FaceContainment::In),
            Ok(CurvedPlacement::OffCarrier) => return Some(false),
            Ok(CurvedPlacement::Trim(None)) => FaceUnread::Trim,
            Err(e) => FaceUnread::Placement(e),
        };
        unread(i, why, held);
        None
    };
    let mut cuts = vec![overlap.0, overlap.1];
    let exact = match *locus {
        geom::Curve3::Line { origin, dir } => faces.iter().all(|&(body, face)| {
            boundary_cuts(body, face, (origin, dir), overlap, band).is_some_and(|(ts, unsure)| {
                cuts.extend(ts);
                !unsure
            })
        }),
        _ => false,
    };
    if !exact {
        cuts.extend(
            (0..geom_brep::CERT_SAMPLES).map(|s| geom_brep::sample_param(overlap.0, overlap.1, s)),
        );
    }
    let mut cuts: Vec<T> = cuts
        .into_iter()
        .filter(|t| !t.is_poison())
        .map(|t| t.max(overlap.0).min(overlap.1))
        .collect();
    // A compare-exchange sort: `min`/`max` order any scalar.
    for i in 0..cuts.len() {
        for j in (i + 1)..cuts.len() {
            let (lo, hi) = (cuts[i].min(cuts[j]), cuts[i].max(cuts[j]));
            cuts[i] = lo;
            cuts[j] = hi;
        }
    }
    let mut held = None;
    for run in cuts.windows(2) {
        // A run inside the zero band is one cut met twice.
        if matches!(
            crate::validate::decide("bool_tangent_line_run", Margin::of(run[1] - run[0]), band),
            Ok(Sign::Zero | Sign::Negative)
        ) {
            continue;
        }
        let p = locus.eval((run[0] + run[1]) * T::from_f64(0.5));
        let mut station = Vec::new();
        match crate::census::in_both(|h| inside(0, p, h), |h| inside(1, p, h), &mut station) {
            Some(true) => return Ok(true),
            Some(false) => {}
            None => {
                held.get_or_insert(station);
            }
        }
    }
    held.map_or(Ok(false), Err)
}

/// Where `face`'s boundary can meet the line `origin + t·dir` (`dir`
/// unit) over `span`: the parameters, and whether some edge's meeting
/// could not be placed (the caller then reads the schedule over the
/// span as well).
///
/// The parameters are the face's vertices' projections, and each
/// edge's meeting with the line:
/// - **A straight edge**, from its two ends: their offsets across the
///   line, interpolated to where the offset is least, so no difference
///   of nearly equal directions enters. An edge whose ends' offsets
///   differ by less than the band stays within the band of the line
///   wherever it meets it, and its ends' projections bound that; one
///   in band of that is not placed.
/// - **A circle or ellipse**, from its carrier: where the line meets
///   its plane, the line leaving the plane by more than the band over
///   `span`; where it meets the conic within the plane, the line within
///   the band of the plane; neither placed, the rise in band.
///
/// A meeting within the band of one of the face's vertices is that
/// vertex's own event read off its edge's carrier, which agrees with
/// the vertex only to the band, and is not a parameter of its own. A
/// parameter that is no meeting only cuts a run in two, so the list may
/// hold more than the meetings and never fewer. `None` where the
/// boundary cannot be walked or an edge's carrier (a spiric, a spline,
/// null scaffolding) has no closed form here.
fn boundary_cuts<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (origin, dir): (Point3<T>, geom_core::Vec3<T>),
    span: (T, T),
    band: Band,
) -> Option<(Vec<T>, bool)> {
    let along = |p: Point3<T>| (p - origin).dot(dir);
    let across = |p: Point3<T>| (p - origin) - dir * along(p);
    let sign = |name, x: T| crate::validate::decide(name, Margin::of(x.abs()), band);
    let roots = |a: T, b: T, c: T, out: &mut Vec<T>| {
        let r = (b.powi(2) - T::from_f64(4.0) * a * c).max(T::zero()).sqrt();
        let q = (b + r.copysign(b)) * T::from_f64(-0.5);
        out.extend([q / a, c / q]);
    };
    let (start, end) = (origin + dir * span.0, origin + dir * span.1);
    let mut unsure = false;
    // A conic's plane: where the line meets it, or whether the line
    // lies within the band of it (its in-plane roots then cut); `None`
    // where the line's rise across the plane is in band.
    let conic = |center: Point3<T>, axis: geom_core::Vec3<T>, out: &mut Vec<T>| {
        let (h0, h1) = ((start - center).dot(axis), (end - center).dot(axis));
        match sign("bool_tangent_line_cut_plane", h1 - h0) {
            Ok(Sign::Positive) => {
                out.push((center - origin).dot(axis) / dir.dot(axis));
                Some(false)
            }
            Ok(_) => Some(sign("bool_tangent_line_cut_plane", h0) != Ok(Sign::Positive)),
            Err(_) => None,
        }
    };
    let point = |v| {
        body.get_vertex(v)
            .and_then(|v| body.get_point(v.point))
            .copied()
    };
    let f = body.get_face(face)?;
    let mut vertices = Vec::new();
    let mut meetings = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let first = match body.get_loop(lk)?.boundary {
            crate::entity::LoopBoundary::Empty { vertex } => {
                vertices.push(along(point(vertex)?));
                continue;
            }
            crate::entity::LoopBoundary::Cycle { first } => first,
        };
        for he in body.loop_cycle(first)? {
            let he = body.get_half_edge(he)?;
            vertices.push(along(point(he.start)?));
            let edge = body.get_edge(he.edge)?;
            match *body.get_curve_geom(edge.curve)?.certified()?.carrier() {
                geom::Curve3::Line { .. } => {
                    let ends = [edge.he_plus, edge.he_minus]
                        .map(|h| body.get_half_edge(h).and_then(|h| point(h.start)));
                    let [Some(a), Some(b)] = ends else {
                        return None;
                    };
                    let (oa, ob) = (across(a), across(b));
                    let swing = ob - oa;
                    match sign("bool_tangent_line_cut_swing", swing.norm()) {
                        Ok(Sign::Positive) => {
                            let s = -oa.dot(swing) / swing.dot(swing);
                            meetings.push(along(a) + (along(b) - along(a)) * s);
                        }
                        Ok(_) => {}
                        Err(_) => unsure = true,
                    }
                }
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => {
                    let in_plane = conic(center, axis, &mut meetings);
                    unsure |= in_plane.is_none();
                    if in_plane == Some(true) {
                        let w = origin - center;
                        roots(
                            T::one(),
                            w.dot(dir) + w.dot(dir),
                            w.dot(w) - radius.powi(2),
                            &mut meetings,
                        );
                    }
                }
                geom::Curve3::Ellipse {
                    center,
                    axis,
                    semi_u,
                    semi_v,
                    u_ref,
                } => {
                    let in_plane = conic(center, axis, &mut meetings);
                    unsure |= in_plane.is_none();
                    if in_plane == Some(true) {
                        let v_ref = axis.cross(u_ref);
                        let w = origin - center;
                        let (wu, wv) = (w.dot(u_ref) / semi_u, w.dot(v_ref) / semi_v);
                        let (du, dv) = (dir.dot(u_ref) / semi_u, dir.dot(v_ref) / semi_v);
                        roots(
                            du.powi(2) + dv.powi(2),
                            (wu * du + wv * dv) * T::from_f64(2.0),
                            wu.powi(2) + wv.powi(2) - T::one(),
                            &mut meetings,
                        );
                    }
                }
                _ => return None,
            }
        }
    }
    meetings.retain(|&m| {
        !m.is_poison()
            && vertices
                .iter()
                .all(|&v| sign("bool_tangent_line_cut_vertex", m - v) == Ok(Sign::Positive))
    });
    vertices.extend(meetings);
    Some((vertices, unsure))
}

/// The face's boundary vertex positions (outer loop then rings, cycle
/// order; an empty loop contributes its lone vertex) — the witness-
/// extent datum of `verify_tangent_declaration`.
fn face_boundary_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    operand: Operand,
) -> Result<Vec<Point3<T>>, BooleanError> {
    let bad = || BooleanError::InvalidInput {
        operand,
        what: "declared face's boundary is unwalkable",
    };
    let f = body.get_face(face).ok_or_else(bad)?;
    let mut out = Vec::new();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let l = body.get_loop(lk).ok_or_else(bad)?;
        let vertex_point = |vk| -> Result<Point3<T>, BooleanError> {
            body.get_vertex(vk)
                .and_then(|v| body.get_point(v.point))
                .copied()
                .ok_or_else(bad)
        };
        match l.boundary {
            crate::entity::LoopBoundary::Empty { vertex } => out.push(vertex_point(vertex)?),
            crate::entity::LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).ok_or_else(bad)? {
                    let start = body.get_half_edge(he).ok_or_else(bad)?.start;
                    out.push(vertex_point(start)?);
                }
            }
        }
    }
    Ok(out)
}

fn validate_declarations<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &Glue<T>,
) -> Result<(), BooleanError> {
    let bad = |operand, what| BooleanError::InvalidInput { operand, what };
    // THE C8 boundary, stated once: a declared face must sit on a
    // carrier the classification's certified ladder describes, its
    // `INVENTORY`. Kinds outside it refuse typed at this door; undeclared touching
    // refuses forever at the classification frontiers — the door only
    // widens what a VERIFIED declaration can unlock. Per-class
    // geometric admission (the `Tangent` witness lane) is
    // `verify_glued_contacts`' half of the door.
    //
    // This inventory is the ONE place a kind becomes declarable, and
    // that is what makes the operand gate's covered-pair admission
    // kind-generic without being kind-blind: a face whose carrier has
    // no rung here can never appear in a surviving declaration, so it
    // can never be covered there either.
    let inventory_face = |body: &Body<T>, f: FaceKey, operand| -> Result<(), BooleanError> {
        let face = body
            .get_face(f)
            .ok_or_else(|| bad(operand, "declared face key does not resolve"))?;
        match body.get_surface(face.surface) {
            Some(s) if carrier_eq::in_inventory(s.kind()) => Ok(()),
            Some(_) => Err(bad(operand, carrier_eq::OUTSIDE_REST_INVENTORY)),
            None => Err(bad(operand, "declared face lost its surface")),
        }
    };
    for &GluedPair {
        a: fa,
        b: fb,
        class,
        ..
    } in &decls.pairs
    {
        // A pair declared twice under DIFFERENT classes is a caller bug
        // refused here — the check the `GluedPairs` map's
        // last-write-wins build would otherwise resolve silently now
        // that the op holds two class arms.
        if decls
            .pairs
            .iter()
            .any(|d| (d.a, d.b) == (fa, fb) && d.class != class)
        {
            return Err(bad(
                Operand::A,
                "one face pair is declared twice under different contact classes",
            ));
        }
        inventory_face(a, fa, Operand::A)?;
        inventory_face(b, fb, Operand::B)?;
    }
    // A record's two cells' interiors meet, so neither bounds the other.
    let bounds = |body: &Body<T>, x, y| {
        matches!(Contact::new(body, x, y), Err(ContactPairError::Incident(_)))
    };
    let carried = |body: &Body<T>, c: &OperandRecords, operand| -> Result<(), BooleanError> {
        for carried in &c.vv {
            let pair = carried.contact;
            if body.get_vertex(pair.a).is_none() || body.get_vertex(pair.b).is_none() {
                return Err(bad(operand, "carried v-v vertex key does not resolve"));
            }
            if pair.a == pair.b {
                return Err(bad(operand, "carried v-v pair names one vertex twice"));
            }
        }
        for carried in &c.vf {
            let rest = carried.contact;
            if body.get_vertex(rest.vertex).is_none() {
                return Err(bad(operand, "carried v-on-f vertex key does not resolve"));
            }
            if body.get_face(rest.face).is_none() {
                return Err(bad(operand, "carried v-on-f face key does not resolve"));
            }
            if bounds(body, Cell::Vertex(rest.vertex), Cell::Face(rest.face)) {
                return Err(bad(operand, "carried v-on-f vertex bounds its face"));
            }
        }
        for CarriedRecord { contact: rest, .. } in &c.ve {
            if body.get_vertex(rest.vertex).is_none() {
                return Err(bad(operand, "carried v-on-e vertex key does not resolve"));
            }
            if body.get_edge(rest.edge).is_none() {
                return Err(bad(operand, "carried v-on-e edge key does not resolve"));
            }
            if bounds(body, Cell::Vertex(rest.vertex), Cell::Edge(rest.edge)) {
                return Err(bad(operand, "carried v-on-e vertex bounds its edge"));
            }
        }
        for CarriedRecord { contact: touch, .. } in &c.ee {
            if touch.a == touch.b {
                return Err(bad(operand, "carried e-e pair names one edge twice"));
            }
            for e in [touch.a, touch.b] {
                if body.get_edge(e).is_none() {
                    return Err(bad(operand, "carried e-e edge key does not resolve"));
                }
            }
        }
        for CarriedRecord { contact: touch, .. } in &c.ef {
            if body.get_edge(touch.edge).is_none() {
                return Err(bad(operand, "carried e-f edge key does not resolve"));
            }
            if body.get_face(touch.face).is_none() {
                return Err(bad(operand, "carried e-f face key does not resolve"));
            }
            if bounds(body, Cell::Edge(touch.edge), Cell::Face(touch.face)) {
                return Err(bad(operand, "carried e-f edge bounds its face"));
            }
        }
        for CarriedRecord { contact: touch, .. } in &c.ff {
            if touch.a == touch.b {
                return Err(bad(operand, "carried f-f pair names one face twice"));
            }
            for f in [touch.a, touch.b] {
                if body.get_face(f).is_none() {
                    return Err(bad(operand, "carried f-f face key does not resolve"));
                }
            }
        }
        Ok(())
    };
    carried(a, &decls.records_a, Operand::A)?;
    carried(b, &decls.records_b, Operand::B)?;
    Ok(())
}

pub use join_lift::BooleanJoin;

/// The one conversion of a chord-join refusal into the Boolean's, held
/// by the type: [`BooleanJoin`]'s field is private to this module, so
/// [`BooleanError::Join`] is built only here, and never around a
/// lineage row.
mod join_lift {
    use super::{BooleanError, SplitJoinError};

    /// A chord-join refusal the Boolean nests whole: any
    /// [`SplitJoinError`] but a lineage row.
    #[derive(Debug)]
    pub struct BooleanJoin(SplitJoinError);

    impl BooleanJoin {
        /// The join's own refusal.
        #[must_use]
        pub fn error(&self) -> &SplitJoinError {
            &self.0
        }
    }

    impl core::fmt::Display for BooleanJoin {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// Nested whole ([`BooleanError::Join`]), except a lineage row the
    /// join's division list refused, which is the Boolean's own
    /// [`BooleanError::LineageRow`], so one lineage refusal reaches a
    /// caller one way.
    impl From<SplitJoinError> for BooleanError {
        fn from(error: SplitJoinError) -> Self {
            match error {
                SplitJoinError::LineageRow(refusal) => refusal.into(),
                other => Self::Join(BooleanJoin(other)),
            }
        }
    }
}

pub use merge_lift::BooleanMerge;

/// The one conversion of a merge-door refusal into the Boolean's, held
/// by the type: [`BooleanMerge`]'s field is private to this module, so
/// [`BooleanError::Merge`] is built only here, and never around the
/// door's re-description of a kept boundary.
mod merge_lift {
    use super::{BooleanError, MergeCoplanarError};

    /// A merge-door refusal the Boolean nests whole: any
    /// [`MergeCoplanarError`] but a kept boundary left undescribed.
    #[derive(Debug)]
    pub struct BooleanMerge(MergeCoplanarError);

    impl BooleanMerge {
        /// The merge door's own refusal.
        #[must_use]
        pub fn error(&self) -> &MergeCoplanarError {
            &self.0
        }
    }

    impl core::fmt::Display for BooleanMerge {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// Nested whole ([`BooleanError::Merge`]), except the door's
    /// re-description of its kept faces' boundaries, which refuses as the
    /// Boolean's own description pass does (`ops::of_describe`), so one
    /// undescribed edge reaches a caller one way.
    impl From<MergeCoplanarError> for BooleanError {
        fn from(error: MergeCoplanarError) -> Self {
            match error.kept_boundary() {
                Some(described) => super::ops::of_describe(described),
                None => Self::Merge(BooleanMerge(error)),
            }
        }
    }
}

impl From<crate::lineage::RowRefusal> for BooleanError {
    fn from(refusal: crate::lineage::RowRefusal) -> Self {
        Self::LineageRow {
            rows: refusal.rows,
            defect: refusal.defect,
            key: refusal.key,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::COINCIDENCE_RECOURSE;

    /// **The interiors read places the line on each face whatever its
    /// carrier** ([`locus_through_both_faces`]): two cylinder sheets
    /// tangent from outside along the ruling `x = 1, y = 0`. Where both
    /// sheets' trims hold that ruling the line runs through both
    /// interiors; where one sheet's trim stops short of it, or the line
    /// is off one sheet's carrier, it does not. A read of plane faces
    /// alone answers `false` for the first.
    #[test]
    fn the_interiors_read_places_a_tangent_line_on_two_curved_faces() {
        use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
        use geom_core::Vec3;
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        // `u = π/2` of each frame is the side facing the other cylinder.
        let sheet = |origin: Point3<f64>, u_ref: Vec3<f64>, u: (f64, f64)| {
            let mut body = Body::<f64>::new();
            let frame = CylFrame {
                origin,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref,
            };
            let face = cyl_wall_sheet(&mut body, frame, u, (0.0, 1.0), tol);
            crate::test_support_fixtures::set_sheet_sense(&mut body, face, true, tol);
            (body, face)
        };
        let near = (1.0, 2.1);
        let a = sheet(Point3::origin(), -Vec3::unit_y(), near);
        let b_in = sheet(Point3::new(2.0, 0.0, 0.0), Vec3::unit_y(), near);
        let b_short = sheet(Point3::new(2.0, 0.0, 0.0), Vec3::unit_y(), (2.2, 3.0));
        let b_apart = sheet(Point3::new(2.5, 0.0, 0.0), Vec3::unit_y(), near);
        let line = geom::Curve3::Line {
            origin: Point3::new(1.0, 0.0, 0.0),
            dir: Vec3::unit_z(),
        };
        let kind = geom::SurfaceKind::Cylinder;
        for (what, b, want) in [
            ("both trims hold the ruling", &b_in, true),
            ("B's trim stops short of it", &b_short, false),
            ("the line is off B's carrier", &b_apart, false),
        ] {
            let got = locus_through_both_faces(
                [(&a.0, a.1, Operand::A, kind), (&b.0, b.1, Operand::B, kind)],
                &line,
                (0.2, 0.8),
                DeclarationRead::Moot,
                band,
            );
            assert_eq!(got.ok(), Some(want), "{what}");
        }
    }

    /// **A tangency along a curve gives cover only where it is a global
    /// side** ([`tangency_certifies_side`], shared by the strut and
    /// seam sources). The table is exhaustive over both kinds, so a kind
    /// added to `SurfaceKind` fails here until someone decides its arm.
    #[test]
    fn only_tangencies_that_are_a_global_side_certify_one() {
        use geom::SurfaceKind::{self, Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
        let all = [Plane, Cylinder, Cone, Sphere, Torus, Nurbs, Approx];
        let visit = |k: SurfaceKind| match k {
            Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx => k,
        };
        let admitted: Vec<(SurfaceKind, SurfaceKind)> = all
            .iter()
            .flat_map(|&p| all.iter().map(move |&q| (visit(p), q)))
            .filter(|&(p, q)| tangency_certifies_side(TangencySource::Rim, p, q))
            .collect();
        assert_eq!(
            admitted,
            [
                (Plane, Cylinder),
                (Plane, Torus),
                (Cylinder, Plane),
                (Cylinder, Cylinder),
                (Cylinder, Sphere),
                (Sphere, Cylinder),
                (Sphere, Torus),
                (Torus, Plane),
                (Torus, Sphere),
            ]
        );
        // The strut source's column: plane and cylinder only.
        let struts: Vec<(SurfaceKind, SurfaceKind)> = all
            .iter()
            .flat_map(|&p| all.iter().map(move |&q| (p, q)))
            .filter(|&(p, q)| tangency_certifies_side(TangencySource::Strut, p, q))
            .collect();
        assert_eq!(struts, [(Plane, Cylinder), (Cylinder, Plane)]);
        for k in [Nurbs, Approx, Cone] {
            for o in all {
                for source in [TangencySource::Strut, TangencySource::Rim] {
                    assert!(
                        !tangency_certifies_side(source, k, o),
                        "{k:?} against {o:?}"
                    );
                    assert!(
                        !tangency_certifies_side(source, o, k),
                        "{o:?} against {k:?}"
                    );
                }
            }
        }
    }

    /// **Every row of the table, measured rather than argued**: for each
    /// ordered kind pair, a G1 pair of carriers tangent along a curve,
    /// sampled over the whole parent carrier (a plane, cylinder or
    /// cylinder axis over ±4 m) against the partner's implicit residual.
    /// An admitted row must keep to one closed side on every fixture;
    /// each excluded torus row has a fixture reaching both sides. The
    /// fixtures cover every admitted row and every torus exclusion, so
    /// flipping any of them reds here:
    ///
    /// - plane `z = r` on a cylinder along `x`;
    /// - two parallel cylinders, tangent externally and internally;
    /// - a sphere on the axis of a cylinder of its radius;
    /// - a sphere centred on a torus's axis, tangent along its inner
    ///   equator, and the ball-ended elbow: a sphere of the minor radius
    ///   centred on the centre circle, tangent along a meridian;
    /// - the plane `z = r` on a torus;
    /// - excluded: the G1 tube chain, a straight tube leaving a torus's
    ///   end meridian, and the S-bend, two tori sharing a meridian and
    ///   bending opposite ways. Each carrier reaches both sides of the
    ///   other. (A cylinder coaxial with a torus, of radius `R ± r`, is
    ///   tangent along an equator and IS a global side: the exclusion
    ///   is conservative, see the table's doc.)
    #[test]
    fn the_table_matches_which_carriers_keep_to_one_side() {
        use geom::Surface;
        use geom::SurfaceKind::{self, Cylinder, Plane, Sphere, Torus};
        use geom_core::{Point3, Vec3};
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let p = |a: f64, b: f64, c: f64| Point3::new(a, b, c);
        let (big, r) = (2.0_f64, 0.5_f64);
        let plane = |h: f64| Surface::Plane {
            origin: p(0.0, 0.0, h),
            normal: z,
            u_ref: x,
        };
        let cyl =
            |o: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>| Surface::Cylinder {
                origin: o,
                axis,
                radius,
                u_ref,
            };
        let ball = |c: Point3<f64>, radius: f64| Surface::Sphere {
            center: c,
            radius,
            axis: z,
            u_ref: x,
        };
        let torus = |c: Point3<f64>| Surface::Torus {
            center: c,
            axis: z,
            major_radius: big,
            minor_radius: r,
            u_ref: x,
        };
        let samples = |s: &Surface<f64>| -> Vec<Point3<f64>> {
            let tau = core::f64::consts::TAU;
            let (us, vs): (Vec<f64>, Vec<f64>) = match s {
                Surface::Plane { .. } => (
                    (0..33).map(|i| -4.0 + f64::from(i) * 0.25).collect(),
                    (0..33).map(|i| -4.0 + f64::from(i) * 0.25).collect(),
                ),
                Surface::Cylinder { .. } => (
                    (0..48).map(|i| f64::from(i) * tau / 48.0).collect(),
                    (0..33).map(|i| -4.0 + f64::from(i) * 0.25).collect(),
                ),
                Surface::Sphere { .. } => (
                    (0..48).map(|i| f64::from(i) * tau / 48.0).collect(),
                    (1..24)
                        .map(|i| {
                            -core::f64::consts::FRAC_PI_2
                                + f64::from(i) * core::f64::consts::PI / 24.0
                        })
                        .collect(),
                ),
                _ => (
                    (0..96).map(|i| f64::from(i) * tau / 96.0).collect(),
                    (0..48).map(|i| f64::from(i) * tau / 48.0).collect(),
                ),
            };
            us.iter()
                .flat_map(|&u| vs.iter().map(move |&v| s.eval(u, v)))
                .collect()
        };
        let one_side = |parent: &Surface<f64>, partner: &Surface<f64>| {
            let rs: Vec<f64> = samples(parent)
                .into_iter()
                .map(|q| geom_brep::implicit_residual(partner, q))
                .collect();
            !(rs.iter().any(|&v| v > 1e-9) && rs.iter().any(|&v| v < -1e-9))
        };
        // The straight tube leaving the torus's end meridian at `+x`
        // (heading `+y`), and the torus bending the other way from it.
        let fixtures: Vec<(&str, Surface<f64>, Surface<f64>)> = vec![
            (
                "plane on a cylinder",
                plane(r),
                cyl(p(0.0, 0.0, 0.0), x, r, y),
            ),
            (
                "cylinders outside",
                cyl(p(0.0, 0.0, 0.0), z, 1.0, x),
                cyl(p(2.0, 0.0, 0.0), z, 1.0, x),
            ),
            (
                "cylinders inside",
                cyl(p(0.0, 0.0, 0.0), z, 1.0, x),
                cyl(p(0.5, 0.0, 0.0), z, 0.5, x),
            ),
            (
                "sphere in a cylinder",
                ball(p(0.0, 0.0, 0.0), r),
                cyl(p(0.0, 0.0, 0.0), z, r, x),
            ),
            (
                "sphere in a torus's hole",
                ball(p(0.0, 0.0, 0.0), big - r),
                torus(p(0.0, 0.0, 0.0)),
            ),
            (
                "the ball-ended elbow",
                ball(p(big, 0.0, 0.0), r),
                torus(p(0.0, 0.0, 0.0)),
            ),
            ("plane on a torus", plane(r), torus(p(0.0, 0.0, 0.0))),
            (
                "the tube chain",
                torus(p(0.0, 0.0, 0.0)),
                cyl(p(big, 0.0, 0.0), y, r, x),
            ),
            (
                "the S-bend",
                torus(p(0.0, 0.0, 0.0)),
                torus(p(2.0 * big, 0.0, 0.0)),
            ),
        ];
        let mut seen = std::collections::BTreeSet::new();
        for (label, s1, s2) in &fixtures {
            for (parent, partner) in [(s1, s2), (s2, s1)] {
                let kinds = (parent.kind(), partner.kind());
                let held = one_side(parent, partner);
                if tangency_certifies_side(TangencySource::Rim, kinds.0, kinds.1) {
                    assert!(
                        held,
                        "{label}: {kinds:?} is admitted, yet both sides are reached"
                    );
                } else {
                    assert!(!held, "{label}: {kinds:?} is excluded, yet one side holds");
                }
                seen.insert(kinds);
            }
        }
        let all = [Plane, Cylinder, Sphere, Torus];
        let rows: std::collections::BTreeSet<(SurfaceKind, SurfaceKind)> = all
            .iter()
            .flat_map(|&a| all.iter().map(move |&b| (a, b)))
            .filter(|&(a, b)| tangency_certifies_side(TangencySource::Rim, a, b))
            .chain([(Torus, Cylinder), (Cylinder, Torus), (Torus, Torus)])
            .collect();
        assert_eq!(seen, rows, "a fixture for every row and torus exclusion");
    }

    /// **An undecided sample leaves the cover's side unread.** The top
    /// face of the unit brick against the plane `x = −m`, `m` mid-band
    /// at the run's ε: the samples on `x = 0` sit in the band, those on
    /// `x = 1` and `x = ½` are definitely Positive. A side read off the definite ones alone
    /// would be `Off(Positive)`; the in-band ones keep it `Unread`. The
    /// plane `x = −1` reads every sample definitely, and the side is read.
    #[test]
    fn an_escalated_sample_leaves_the_cover_side_unread() {
        use geom_core::{Point3, Tol, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let body: crate::Body<f64> =
            crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
        let top = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Plane { origin, normal, .. })
                        if origin.z > 0.5 && normal.z.abs() > 0.5
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        let wall = |x: f64| geom::Surface::Plane {
            origin: Point3::new(x, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        assert_eq!(
            read_cover_side(
                &body,
                top,
                &wall(-(band.zero() + band.escalate()) * 0.5),
                band
            ),
            CoverSide::Unread
        );
        assert_eq!(
            read_cover_side(&body, top, &wall(-1.0), band),
            CoverSide::Off(Sign::Positive)
        );
    }

    /// **The cover carries its side, and admits an off-carrier point
    /// only there** ([`CoverSide::admits`]): a cover certified Positive
    /// whose end reads Negative keeps its door, as does any off point
    /// under a one-carrier cover or an unread side.
    #[test]
    fn a_cover_admits_an_off_point_only_on_its_certified_side() {
        use CoverSide::{Off, On, Unread};
        for cover in [On, Unread, Off(Sign::Positive), Off(Sign::Negative)] {
            assert!(cover.admits(Sign::Zero), "{cover:?}: a touch");
        }
        assert!(Off(Sign::Positive).admits(Sign::Positive));
        assert!(!Off(Sign::Positive).admits(Sign::Negative));
        assert!(Off(Sign::Negative).admits(Sign::Negative));
        assert!(!Off(Sign::Negative).admits(Sign::Positive));
        for s in [Sign::Positive, Sign::Negative] {
            assert!(!On.admits(s), "one carrier, off it: {s:?}");
            assert!(!Unread.admits(s), "unread, off it: {s:?}");
        }
    }

    /// **The one-sided cover is directed** (C4 states it one way): a
    /// key inserted parent → target answers for that direction only,
    /// and a same-operand pair is never covered.
    #[test]
    fn the_one_sided_cover_is_keyed_parent_to_target() {
        let mut faces: slotmap::SlotMap<FaceKey, ()> = slotmap::SlotMap::with_key();
        let (f, g) = (faces.insert(()), faces.insert(()));
        let mut pairs = GluedPairs::<f64>::default();
        pairs.one_sided.insert(
            (tagged(Operand::A, f), tagged(Operand::B, g)),
            CoverSide::On,
        );
        assert_eq!(
            pairs.cover(Operand::A, f, Operand::B, g),
            Some(CoverSide::On)
        );
        assert_eq!(pairs.cover(Operand::B, g, Operand::A, f), None);
        assert_eq!(pairs.cover(Operand::A, f, Operand::A, g), None);
    }

    /// **[`BooleanOp::ALL`] holds each operation once, and an
    /// operation added to the enum cannot reach a release without
    /// someone reading this row** — the idiom `VerbKind::ALL`
    /// (`crates/verbs/src/verb.rs`) is held to.
    ///
    /// **What is forced**: the match below is exhaustive with no
    /// wildcard, so an operation added to the enum fails this file
    /// until it is visited here. And the no-repeats half is what makes
    /// the count a census rather than a length: with every entry
    /// distinct, a `len` equal to `ops` means `ALL` holds each of them
    /// exactly once.
    ///
    /// **What is NOT forced, measured**: `ops` itself. Every arm names
    /// the same total so that visiting means re-deciding it — but
    /// nothing checks that number against the enum, and the arm an
    /// author adds is the arm they copied. A fourth variant with the
    /// arm `Xor => 3` compiles and passes GREEN with `Xor` absent from
    /// `ALL`. The row forces the visit, not the edit. That is the
    /// idiom's hole and not this row's alone — it is inherited from the
    /// two censuses cited above — so it is filed as
    /// `work/census/all-census-idiom-forces-the-visit-not-the-update`
    /// rather than patched here in one of three places.
    #[test]
    fn all_is_every_operation() {
        let ops = match BooleanOp::Union {
            BooleanOp::Union => 3,
            BooleanOp::Intersect => 3,
            BooleanOp::Subtract => 3,
        };
        for (i, op) in BooleanOp::ALL.iter().enumerate() {
            assert!(
                !BooleanOp::ALL[..i].contains(op),
                "{op:?} appears twice in BooleanOp::ALL"
            );
        }
        assert_eq!(
            BooleanOp::ALL.len(),
            ops,
            "BooleanOp::ALL has drifted from the declaration — it holds {} operations, the enum has {ops}",
            BooleanOp::ALL.len()
        );
    }

    /// **The tangency check's conformal screen refuses a carrier datum
    /// that is not finite as poison**, not as the one carrier that
    /// contradicts the claim: two stacked bricks' caps on `z = 1`, the
    /// lower one re-charted parallel through `z = +∞`.
    #[test]
    fn the_tangency_screen_refuses_an_infinite_offset_as_poison() {
        let tol = geom_core::Tol::witness();
        let mut a = crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let b = crate::test_support_fixtures::brick((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), tol);
        let cap = |body: &Body<f64>, sign: f64| {
            let hits: Vec<FaceKey> = crate::query::all_faces(body)
                .into_iter()
                .filter(|&f| {
                    matches!(face_carrier(body, f), Some(CarrierDesc::Plane { origin, normal })
                        if (origin.z - 1.0).abs() < 1e-12 && normal.z * sign > 0.5)
                })
                .collect();
            let [f] = hits[..] else {
                panic!("expected one cap on z = 1, got {hits:?}");
            };
            f
        };
        let (fa, fb) = (cap(&a, 1.0), cap(&b, -1.0));
        let face = a.get_face(fa).unwrap();
        let Some(geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        }) = a.get_surface(face.surface).cloned()
        else {
            panic!("the cap is planar");
        };
        let surface = crate::euler::FaceSurface::New {
            surface: geom::Surface::Plane {
                origin: Point3::new(origin.x, origin.y, f64::INFINITY),
                normal,
                u_ref,
            },
            sense: face.sense,
        };
        // Lifts both refusals: a cap whose offset datum is infinite is
        // the row's premise, and no edge certifies against it.
        a.set_face_surface_unvouched_for_tests(fa, surface).unwrap();
        let band = Band::linear(tol).unwrap();
        let err = verify_tangent_declaration(&a, fa, &b, fb, band)
            .expect_err("the screen reads no carrier through an infinite offset");
        let BooleanError::PoisonedCarrierDatum { pair, diag } = err else {
            panic!("the screen refuses the poisoned datum: {err:?}");
        };
        assert_eq!(pair, [(Operand::A, fa), (Operand::B, fb)]);
        assert_eq!(diag.predicate, Some("bool_plane_offset"), "{diag:?}");
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the boolean coincidence
    /// refusals — the plane-identity escalation and the undecided
    /// carrier pair rung 4 leaves to a site — carry one recourse per
    /// message, and none offers a declaration.
    #[test]
    fn coincidence_pair_carries_the_shared_recourse_once() {
        let diag = |margin| Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("bool_plane_offset"),
            terminal_sliver: false,
        };
        // The escalated arm, as the lookup mints it for an undeclared
        // pair of planar corners.
        let door = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default())
            .on_pair_door(
                (
                    Operand::A,
                    FaceKey::default(),
                    Operand::B,
                    FaceKey::default(),
                ),
                Some(PlaneRelation::SameOpposite),
            );
        for margin in [MarginDiag::value(5e-9), MarginDiag::INVALID] {
            let msg =
                BooleanError::plane_identity(PlaneRung::Parallel, door, diag(margin)).to_string();
            assert_eq!(msg.matches("Recourse:").count(), 1, "{msg}");
            assert!(!msg.contains("declare"), "{msg}");
        }
        // The undecided arm, in both shapes rung 4 leaves to a site:
        // an in-band margin and a poisoned one — one message, one
        // recourse where the margin reads, and the margin the measure
        // read as the payload.
        let pair = [
            (Operand::A, FaceKey::default()),
            (Operand::B, FaceKey::default()),
        ];
        let read = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default())
            .carriers_read(
                (
                    Operand::A,
                    FaceKey::default(),
                    Operand::B,
                    FaceKey::default(),
                ),
                CarrierRelation::SameOpposite,
            );
        let msg = undecided_coincidence(
            carrier_eq::CoincidenceMeasure::Undecided(diag(MarginDiag::value(5e-9))),
            pair,
            read,
        )
        .to_string();
        assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 1, "{msg}");
        assert!(!msg.contains("exactly zero"), "{msg}");
        let poisoned = undecided_coincidence(
            carrier_eq::CoincidenceMeasure::Unreadable(diag(MarginDiag::INVALID)),
            pair,
            read,
        );
        assert!(
            matches!(poisoned, BooleanError::PoisonedCarrierDatum { .. }),
            "{poisoned:?}"
        );
    }

    /// The non-finite chord arm gives the cause and a recourse that can
    /// WORK, and offers
    /// the coincidence recourse ZERO times — no tolerance lever
    /// reaches an overflowed chord, so naming one would be the
    /// wrong-recourse defect `memories/refusal-text-is-not-cause.md`
    /// is about.
    ///
    /// This and the `SplitReduceError` twin are the only direct pins
    /// on the two wrapper arms: there is no end-to-end row that drives
    /// a real `Body` into `sector_shape`'s rung 0, because that needs
    /// an orbit chord past ~1e154 surviving body construction. The
    /// translation itself (`SectorFault::NonFiniteChord` to this arm)
    /// is held by the exhaustive `map_err` in `sectors.rs` and by
    /// nothing else.
    #[test]
    fn non_finite_sector_chord_names_the_cause_and_no_tolerance_recourse() {
        let msg = BooleanError::NonFiniteSectorChord {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        }
        .to_string();
        assert!(msg.contains("has no finite length"), "{msg}");
        assert!(msg.contains(geom_core::RANGE_RECOURSE), "{msg}");
        assert_eq!(msg.matches(COINCIDENCE_RECOURSE).count(), 0, "{msg}");
        assert!(!msg.contains("zero length"), "{msg}");
        assert_eq!(
            BooleanError::NonFiniteSectorChord {
                vertex: VertexKey::default(),
                face: FaceKey::default(),
            }
            .kind(),
            BooleanErrorKind::NonFiniteSectorChord
        );
    }

    /// **A declared face key that resolves to no face refuses typed,
    /// naming what happened.** The `Tangent` verifier reads two facts
    /// off each declared face — its carrier and its orientation bit —
    /// and takes both from ONE resolution, so a key with no face
    /// behind it cannot reach a material verdict through a sense the
    /// door invented for it. The public entry refuses such a key
    /// earlier still, which is why this row calls the verifier
    /// directly: defence in depth is only depth if the inner layer is
    /// exercised.
    ///
    /// **This is a REGRESSION pin, not an anti-vacuity row**, and the
    /// distinction is worth having in writing: the refusal is not new
    /// — the carrier resolution at this position always refused a
    /// stale key — so nothing here goes red for the fold alone. Two
    /// things together would red it: the one resolution split back
    /// into two, AND a key that resolves to a face whose surface is
    /// missing rather than to no face at all. What it therefore pins
    /// is the `what` string, because that is the half that WAS wrong:
    /// a key with no face behind it used to report the face's surface
    /// as lost, conflating the two findings `validate_declarations`
    /// already distinguishes.
    #[test]
    fn a_declared_face_that_does_not_resolve_refuses_typed() {
        let empty = Body::<f64>::new();
        let err = verify_tangent_declaration(
            &empty,
            FaceKey::default(),
            &empty,
            FaceKey::default(),
            Band::new(1e-9, 1e-8).unwrap(),
        )
        .expect_err("a key with no face behind it cannot be verified");
        assert!(
            matches!(
                err,
                BooleanError::InvalidInput {
                    operand: Operand::A,
                    what: "a glued face's key does not resolve",
                }
            ),
            "a missing declared face is a caller bug, not a classification, and the \
             refusal says which caller bug it is: {err:?}"
        );
    }

    /// One [`BooleanError`] per arm whose payload is keys, spans,
    /// enums and `&'static str` — everything the projection can be
    /// checked on without reaching into another crate's error type.
    /// Arms nesting a foreign refusal (`Euler`, `Join`, `Merge`,
    /// `GraftRecertify`, `CrossingInsertion`, `Pieces`) are absent by
    /// the same rule.
    fn sample_errors() -> Vec<BooleanError> {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let diag = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("bool_plane_offset"),
            terminal_sliver: false,
        };
        let face = FaceKey::default();
        let edge = EdgeKey::default();
        let pair = crate::contact::ContactPair {
            a: face,
            b: face,
            class: GlueClass::Tangent,
        };
        vec![
            BooleanError::Band(Band::new(1.0, 1.0).unwrap_err()),
            BooleanError::CurvedBooleanUnsupported {
                operand: Operand::A,
                face,
                kind: geom::SurfaceKind::Cone,
            },
            BooleanError::CurvedSectorSideUnsupported {
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-1e-3),
                },
            },
            BooleanError::NonFiniteSectorChord {
                vertex: VertexKey::default(),
                face,
            },
            BooleanError::CurvedPierceUnsupported {
                operand: Operand::A,
                face,
                edge,
                band,
            },
            BooleanError::CurvedPierceEscalated {
                operand: Operand::A,
                face,
                edge,
                decision: BooleanDecision::CurveMeeting,
                diag,
            },
            BooleanError::CrossingAtConeApex {
                operand: Operand::B,
                face,
                edge,
                band,
            },
            BooleanError::NormalAtConeApex {
                operand: Operand::A,
                face,
                band,
            },
            BooleanError::NormalAtSplineCrease {
                operand: Operand::B,
                face,
                band,
            },
            BooleanError::PierceLeverUnread {
                operand: Operand::B,
                face,
                cause: geom_brep::ChartArmRefusal::PatchBound,
            },
            BooleanError::EdgeCarrierUnsupported {
                operand: Operand::B,
                edge,
                face,
                face_of: Operand::B,
                site: EdgeCarrierSite::RingRun,
            },
            BooleanError::EdgeCarrierUnsupported {
                operand: Operand::A,
                edge,
                face,
                face_of: Operand::B,
                site: EdgeCarrierSite::PlanarCrossing,
            },
            BooleanError::SplineCrossingUnsettled {
                operand: Operand::A,
                edge,
                face,
                face_of: Operand::B,
                why: geom_brep::roots::Unsettled::Tangency,
            },
            BooleanError::PointSplitCarrierUnsupported {
                operand: Operand::A,
                edge,
            },
            BooleanError::GermEdgeCarrierUnsupported {
                operand: Operand::B,
                edge,
                closed: false,
            },
            BooleanError::GermEdgeCarrierUnsupported {
                operand: Operand::A,
                edge,
                closed: true,
            },
            BooleanError::ArcLoopContainmentUnsupported {
                operand: Operand::A,
                cause: crate::splitting::Uncrossable {
                    r#loop: crate::entity::LoopKey::default(),
                    edge,
                    carrier: crate::splitting::UncrossableCarrier::Spiric,
                },
            },
            BooleanError::PointInFaceRefused {
                operand: Operand::B,
                face,
                refusal: ContainError::RayExhausted,
            },
            BooleanError::ScaffoldingOperand {
                operand: Operand::A,
                errors: vec![ValidationError::ScaffoldingStrutVertex {
                    vertex: VertexKey::default(),
                }],
            },
            BooleanError::InsideOutOperand {
                operand: Operand::B,
                errors: vec![ValidationError::NegativeVolume {
                    solid: crate::entity::SolidKey::default(),
                }],
            },
            BooleanError::UnjoinedOperand {
                operand: Operand::A,
                errors: vec![ValidationError::JoinableVertexAtRest {
                    vertex: VertexKey::default(),
                }],
            },
            BooleanError::NonMaximalFaces {
                operand: Operand::A,
                edge,
            },
            BooleanError::CoplanarNeighbours {
                operand: Operand::A,
                faces: [face, face],
                offset: diag,
            },
            BooleanError::plane_identity(
                PlaneRung::Parallel,
                GluedPairs::<f64>::without_struts(&Glue::none(), Default::default()).on_pair_door(
                    (Operand::A, face, Operand::B, face),
                    Some(PlaneRelation::SameOpposite),
                ),
                diag,
            ),
            BooleanError::ContactContradicted {
                pair,
                margin: diag,
                fact: None,
            },
            BooleanError::TangencyUnsupported,
            BooleanError::InvalidInput {
                operand: Operand::A,
                what: "a stale key",
            },
            BooleanError::PairingMismatch {
                a_vertex: VertexKey::default(),
                b_vertex: VertexKey::default(),
            },
            BooleanError::SharedVertexCrossings {
                operand: Operand::B,
                vertex: VertexKey::default(),
                partners: [VertexKey::default(); 2],
            },
            BooleanError::PinchRaysInAFace {
                operand: Operand::A,
                face,
            },
            BooleanError::TangentCrescentUnsupported {
                a_face: face,
                b_face: face,
            },
            BooleanError::CrossingsAlongAContactLine {
                a_vertex: VertexKey::default(),
                b_vertex: VertexKey::default(),
            },
            BooleanError::VertexReadTwice {
                operand: Operand::A,
                vertex: VertexKey::default(),
                reads: [
                    SectorRead::Pierce(face),
                    SectorRead::Pair(VertexKey::default()),
                ],
            },
            BooleanError::PartnerAlongFace {
                operand: Operand::A,
                vertex: VertexKey::default(),
                face,
                partner: VertexKey::default(),
                along: AlongFace::Curved,
            },
            BooleanError::NonManifoldResult {
                a_vertex: VertexKey::default(),
                b_vertices: [VertexKey::default(); 2],
            },
            BooleanError::ClassificationInvariant {
                what: "an invariant",
            },
            BooleanError::CurvedPairUnsupported {
                op: None,
                site: PairRefusalSite::OperandGate,
                operand: Operand::A,
                face,
                kind: geom::SurfaceKind::Cone,
                other_face: face,
                other_kind: geom::SurfaceKind::Plane,
            },
            BooleanError::CurvedPairUnsupported {
                op: Some(BooleanOp::Union),
                site: PairRefusalSite::InteriorLoopGuard,
                operand: Operand::A,
                face,
                kind: geom::SurfaceKind::Torus,
                other_face: face,
                other_kind: geom::SurfaceKind::Plane,
            },
            BooleanError::NurbsExtentUnsupported {
                operand: Operand::A,
                face,
            },
            BooleanError::FallbackExtentUnsupported {
                operand: Operand::A,
                face,
                what: "an uncertifiable pose",
            },
            BooleanError::RechartedRowUnstated {
                operand: Operand::B,
                relation: crate::Relation::SameOpposite,
                faces: vec![face],
                what: "only a point decided there is stated by the faces that hold it",
            },
            BooleanError::SpheresMeet {
                operand: Operand::A,
                face,
                verdict: geom_brep::recourse::Refused::Negative {
                    margin: MarginDiag::value(-0.5),
                },
            },
            BooleanError::GermFrameUnsupported {
                a_face: face,
                a_kind: geom::SurfaceKind::Cone,
                b_face: face,
                b_kind: geom::SurfaceKind::Torus,
            },
            BooleanError::GermSectionOpenConic {
                a_face: face,
                a_kind: geom::SurfaceKind::Plane,
                b_face: face,
                b_kind: geom::SurfaceKind::Cone,
                conic: geom_brep::OpenConic::Hyperbola,
            },
            BooleanError::GermSectionSpiric {
                a_face: face,
                a_kind: geom::SurfaceKind::Plane,
                b_face: face,
                b_kind: geom::SurfaceKind::Torus,
                loops: SpiricLoops::Null,
                section: geom_brep::SectionError::RoutesToGeneralRung {
                    pair: "plane×torus",
                    why: "a folded loop",
                },
            },
            BooleanError::NodeUnbuilt {
                a_face: face,
                b_face: face,
                why: NodeWhy::OnBoundary(Operand::A),
            },
            BooleanError::Pcurves {
                source: crate::pcurves::PcurveMintError::Unminted { face },
            },
            BooleanError::JoinDesync { what: "a lockstep" },
            BooleanError::SplitLineage(crate::SplitLineageCycle {
                edge: EdgeKey::default(),
            }),
            BooleanError::LineageRow {
                rows: crate::lineage::LineageRows::Substitutions,
                defect: crate::lineage::RowDefect::NamesRetired,
                key: Cell::Face(face),
            },
            BooleanError::EdgeUndescribed {
                edge: EdgeKey::default(),
                failure: crate::merge_faces::EdgeDescribeFailure::NotWalkable,
                cause: None,
                fitted: false,
            },
            BooleanError::MixedSeam {
                edge: EdgeKey::default(),
                stations: Some(geom_brep::MixedStations {
                    transverse: 3,
                    smooth: 4,
                }),
            },
            BooleanError::MixedSeam {
                edge: EdgeKey::default(),
                stations: None,
            },
            BooleanError::TornComponent {
                operand: Operand::A,
                shell: ShellKey::default(),
            },
            BooleanError::ShellWitnessExhausted {
                operand: Operand::B,
                shell: ShellKey::default(),
                on_boundary: 20,
                in_band: 6,
                first_in_band: None,
            },
            BooleanError::CoincidentShell {
                operand: Operand::A,
                shell: ShellKey::default(),
                orientation: ShellOrientation::Unpaired { face },
            },
            BooleanError::Containment(
                crate::boolean::solid_contain::PointInSolidError::RayExhausted,
            ),
            BooleanError::SeamOrientation {
                a_face: face,
                b_face: face,
            },
            BooleanError::ZipCorrespondence { what: "a record" },
            BooleanError::ResultInvalid { errors: Vec::new() },
            BooleanError::UnrepresentableResult,
            BooleanError::Revert {
                operand: Operand::B,
                source: crate::revert::RevertError::CoincidentEdges {
                    edges: [edge, edge],
                    from: "(0.0, 1.0, 2.0)".to_owned(),
                    to: "(3.0, 4.0, 5.0)".to_owned(),
                },
            },
            BooleanError::LineWedges {
                fault: LineWedgeFault::Overlap {
                    edges: [edge, edge],
                },
                from: "(0.0, 1.0, 2.0)".to_owned(),
                to: "(3.0, 4.0, 5.0)".to_owned(),
            },
        ]
        .into_iter()
        .chain(
            crate::census::WEDGE_UNREAD.map(|why| BooleanError::LineWedges {
                fault: LineWedgeFault::Unread { edge, why },
                from: "(0.0, 1.0, 2.0)".to_owned(),
                to: "(3.0, 4.0, 5.0)".to_owned(),
            }),
        )
        .collect()
    }

    /// **The phantom direction, closed by the compiler; the pairing
    /// direction, closed by construction.**
    ///
    /// [`BooleanError::kind`] is exhaustive over the ERROR, so an arm
    /// added there reds this crate. `label` below is exhaustive over
    /// the KIND, so a variant added to [`BooleanErrorKind`] alone reds
    /// HERE, by name, in the crate that owns both — rather than in
    /// whatever downstream crate next maps the enum, of which there are
    /// currently none. `path_error_tag`
    /// (`crates/pncad-py/src/tags.rs`) and the `VerbKind::ALL` census
    /// (`crates/verbs/src/verb.rs`) are the in-tree precedents for
    /// guarding a hand-written mirror with a compile-time visit.
    ///
    /// Neither exhaustiveness objects to an arm PROJECTED to the wrong
    /// kind, which type-checks. That is what the errors below are for:
    /// each is built, projected, and its kind's name compared with the
    /// variant name `Debug` prints for the error itself, so a
    /// mis-projected arm and a mis-labelled arm both fail here with no
    /// expected value written down twice.
    ///
    /// **The errors are spot checks, not a census.** They cover the
    /// arms whose payloads are keys, spans and `&'static str`; an arm
    /// whose payload is another module's or crate's typed refusal is
    /// not built here, so a mis-projection confined to one of those is
    /// not caught. Nothing reds when an arm is missing from this list —
    /// this row accuses no author of anything it has not measured.
    #[test]
    fn each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind() {
        fn label(kind: BooleanErrorKind) -> &'static str {
            match kind {
                BooleanErrorKind::Band => "Band",
                BooleanErrorKind::CurvedBooleanUnsupported => "CurvedBooleanUnsupported",
                BooleanErrorKind::DegenerateTorus => "DegenerateTorus",
                BooleanErrorKind::CurvedSectorSideUnsupported => "CurvedSectorSideUnsupported",
                BooleanErrorKind::CurvedPierceUnsupported => "CurvedPierceUnsupported",
                BooleanErrorKind::CurvedPierceEscalated => "CurvedPierceEscalated",
                BooleanErrorKind::CrossingAtConeApex => "CrossingAtConeApex",
                BooleanErrorKind::NormalAtConeApex => "NormalAtConeApex",
                BooleanErrorKind::NormalAtSplineCrease => "NormalAtSplineCrease",
                BooleanErrorKind::PierceLeverUnread => "PierceLeverUnread",
                BooleanErrorKind::EdgeCarrierUnsupported => "EdgeCarrierUnsupported",
                BooleanErrorKind::SplineCrossingUnsettled => "SplineCrossingUnsettled",
                BooleanErrorKind::PointSplitCarrierUnsupported => "PointSplitCarrierUnsupported",
                BooleanErrorKind::GermEdgeCarrierUnsupported => "GermEdgeCarrierUnsupported",
                BooleanErrorKind::ArcLoopContainmentUnsupported => "ArcLoopContainmentUnsupported",
                BooleanErrorKind::PointInFaceRefused => "PointInFaceRefused",
                BooleanErrorKind::ScaffoldingOperand => "ScaffoldingOperand",
                BooleanErrorKind::InsideOutOperand => "InsideOutOperand",
                BooleanErrorKind::UnjoinedOperand => "UnjoinedOperand",
                BooleanErrorKind::NonMaximalFaces => "NonMaximalFaces",
                BooleanErrorKind::CoplanarNeighbours => "CoplanarNeighbours",
                BooleanErrorKind::NonFiniteSectorChord => "NonFiniteSectorChord",
                BooleanErrorKind::UnderflowedSectorChord => "UnderflowedSectorChord",
                BooleanErrorKind::Escalated => "Escalated",
                BooleanErrorKind::PoisonedCarrierDatum => "PoisonedCarrierDatum",
                BooleanErrorKind::ContactContradicted => "ContactContradicted",
                BooleanErrorKind::TangencyUnsupported => "TangencyUnsupported",
                BooleanErrorKind::RimCuspArmUnbuilt => "RimCuspArmUnbuilt",
                BooleanErrorKind::HeldPierceUnbuilt => "HeldPierceUnbuilt",
                BooleanErrorKind::InvalidInput => "InvalidInput",
                BooleanErrorKind::PairingMismatch => "PairingMismatch",
                BooleanErrorKind::SharedVertexCrossings => "SharedVertexCrossings",
                BooleanErrorKind::PinchRaysInAFace => "PinchRaysInAFace",
                BooleanErrorKind::TangentCrescentUnsupported => "TangentCrescentUnsupported",
                BooleanErrorKind::CrossingsAlongAContactLine => "CrossingsAlongAContactLine",
                BooleanErrorKind::VertexReadTwice => "VertexReadTwice",
                BooleanErrorKind::PartnerAlongFace => "PartnerAlongFace",
                BooleanErrorKind::NonManifoldResult => "NonManifoldResult",
                BooleanErrorKind::ClassificationInvariant => "ClassificationInvariant",
                BooleanErrorKind::CrossingInsertion => "CrossingInsertion",
                BooleanErrorKind::CurvedPairUnsupported => "CurvedPairUnsupported",
                BooleanErrorKind::NurbsExtentUnsupported => "NurbsExtentUnsupported",
                BooleanErrorKind::FallbackExtentUnsupported => "FallbackExtentUnsupported",
                BooleanErrorKind::RechartedRowUnstated => "RechartedRowUnstated",
                BooleanErrorKind::SpheresMeet => "SpheresMeet",
                BooleanErrorKind::GermFrameUnsupported => "GermFrameUnsupported",
                BooleanErrorKind::GermSectionUnbuilt => "GermSectionUnbuilt",
                BooleanErrorKind::GermSectionOpenConic => "GermSectionOpenConic",
                BooleanErrorKind::GermSectionSpiric => "GermSectionSpiric",
                BooleanErrorKind::NodeUnbuilt => "NodeUnbuilt",
                BooleanErrorKind::Euler => "Euler",
                BooleanErrorKind::Pcurves => "Pcurves",
                BooleanErrorKind::Join => "Join",
                BooleanErrorKind::JoinUndecided => "JoinUndecided",
                BooleanErrorKind::JoinCarrierUnsupported => "JoinCarrierUnsupported",
                BooleanErrorKind::RestPieceUnread => "RestPieceUnread",
                BooleanErrorKind::CarryRefused => "CarryRefused",
                BooleanErrorKind::JoinDesync => "JoinDesync",
                BooleanErrorKind::LineageRow => "LineageRow",
                BooleanErrorKind::SplitLineage => "SplitLineage",
                BooleanErrorKind::EdgeUndescribed => "EdgeUndescribed",
                BooleanErrorKind::MixedSeam => "MixedSeam",
                BooleanErrorKind::TornComponent => "TornComponent",
                BooleanErrorKind::ShellWitnessExhausted => "ShellWitnessExhausted",
                BooleanErrorKind::CoincidentShell => "CoincidentShell",
                BooleanErrorKind::Containment => "Containment",
                BooleanErrorKind::SeamOrientation => "SeamOrientation",
                BooleanErrorKind::ZipCorrespondence => "ZipCorrespondence",
                BooleanErrorKind::Merge => "Merge",
                BooleanErrorKind::ResultInvalid => "ResultInvalid",
                BooleanErrorKind::UnrepresentableResult => "UnrepresentableResult",
                BooleanErrorKind::GraftRecertify => "GraftRecertify",
                BooleanErrorKind::Pieces => "Pieces",
                BooleanErrorKind::Revert => "Revert",
                BooleanErrorKind::EdgeInFace => "EdgeInFace",
                BooleanErrorKind::SlitLineCrossed => "SlitLineCrossed",
                BooleanErrorKind::SlitFaceGlued => "SlitFaceGlued",
                BooleanErrorKind::CurvedFaceClosesAcrossSlit => "CurvedFaceClosesAcrossSlit",
                BooleanErrorKind::SlitFaceClosesMidSlit => "SlitFaceClosesMidSlit",
                BooleanErrorKind::LineWedges => "LineWedges",
            }
        }
        /// The variant name `Debug` opens with.
        fn variant_of(err: &BooleanError) -> String {
            format!("{err:?}")
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect()
        }
        for err in sample_errors() {
            assert_eq!(
                label(err.kind()),
                variant_of(&err),
                "kind() projects each arm to its own kind, and label names it"
            );
        }
    }

    /// A message string broken across source lines keeps its line
    /// continuation: no sample renders a run of spaces.
    #[test]
    fn no_sample_message_renders_a_run_of_spaces() {
        for err in sample_errors() {
            let message = err.to_string();
            assert!(!message.contains("  "), "{message:?}");
        }
    }
}
