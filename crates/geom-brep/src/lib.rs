//! The B-rep geometry layer: D2's intensional edge descriptions,
//! certified carrier caches, the dihedral classification predicate, and
//! Newell face equations (M2 PR 3).
//!
//! This crate sits between the evaluators (`geom`) and the arena
//! store (`topo`): it defines **what an
//! edge's geometry is** ([`EdgeDescription`] — a description, never a bare
//! curve), **how a concrete cache earns its place** ([`EdgeCurve`] —
//! certification against the description, D4 ¶2; an uncertified carrier
//! is unrepresentable), and the two geometric classifiers M2's
//! constructions and tier-3 validation stand on:
//!
//! - [`classify_dihedral`] — the material wedge-angle predicate's first
//!   arrival: transverse / smooth / sliver-escalation, via
//!   implicit-form gradients (never chart normals) and D4 ¶1's
//!   displacement-through-lever-arm margins.
//! - [`newell_plane`] — certified planes from loop vertex data,
//!   translate-to-origin by default.
//!
//! [`ssi`] is M5 PR 7's addition: rung 3 of the C1 ladder — surface
//! intersection by march-then-certify, with the full three-limb C2
//! certificate and the in-op exhaustiveness subdivision that makes
//! "every branch found" a theorem or a typed refusal.
//!
//! [`offset_surface`] is the analytic offset mint: the analytic kinds
//! close under normal offset by struct-update on public fields, with
//! the door-owned degeneracy refusals (the realized-radius floor, the
//! torus ring convention) decided before any mint — see [`offset`].
//!
//! [`offset_fit`] is the approximating half that [`offset`]'s NURBS
//! arm refuses into: the Book's §9.4 grid interpolation plus a
//! refine-until-certified loop, and the two-limb certificate of
//! `sup ‖S_fit − (S + d·n)‖`. It stands on two meters
//! ([`offset_meters`]) — a certified LOWER bound on `‖S_u × S_v‖`
//! (the tree's first inf-side surface bound; the offset is undefined
//! where the normal degenerates) and the collapse headroom `|d|`
//! against the patch's certified curvature reach — both read off
//! [`patch_bound`]'s per-cell control-hull enclosures, which are also
//! what `mesh`'s tessellation deviation certificate consumes.
//!
//! The geometry-arena key types ([`PointKey`], [`CurveKey`],
//! [`SurfaceKey`]) are defined here (descriptions reference surfaces by
//! arena key) and re-exported by `topo` for its `Body<T>` arenas —
//! see [`keys`] for the layering rationale and Q1's lineage scoping.
//!
//! # Discipline (inherited, uniform)
//!
//! Everything is generic over [`geom_core::Real`] with decisions only
//! through [`geom_core::Decide`]'s trilean door (named predicates, D4
//! ¶3 escalation); evaluation is comparison-free and total (poison in,
//! poison out — including the `Nurbs` representable-unimplemented
//! placeholders, which certification rejects loudly); sampling is
//! deterministic by fixed schedule (D9). Instantiates at `f64`,
//! `Dual<f64>`, `Interval`, and `Dual<Interval>`.

mod carrier_net;
pub mod carrier_resolution;
pub mod certify;
pub mod chart_singularity;
pub mod conic_bend;
pub mod curve_face;
pub mod curve_pair;
pub(crate) mod descent;
pub mod description;
pub mod dihedral;
pub mod dihedral_side;
pub mod edge_nurbs;
pub mod enters;
#[cfg(test)]
mod exact_q;
pub mod extent;
pub mod fitted_lane;
#[cfg(test)]
mod grid_offsets;
pub mod half_angle;
pub(crate) mod harmonic;
pub mod identity;
pub mod implicit;
pub mod intersect;
pub mod keys;
mod krawczyk;
pub mod ladder;
pub mod locus;
pub mod mapped;
pub(crate) mod memo;
pub mod newell;
pub mod nurbs_iso;
pub mod offset;
pub mod offset_fit;
pub mod offset_fit_lane;
pub mod offset_meters;
pub mod order;
pub mod pair_section;
pub mod patch_bound;
pub mod pcurve;
pub mod pcurve_cache;
pub mod props;
pub mod quartic;
pub mod recourse;
pub mod roots;
pub(crate) mod shape_operator;
mod sphere_circle;
pub mod ssi;
pub mod tangent;
pub mod torus_convention;

pub use carrier_net::CarrierEnclosureRefusal;
pub use certify::{
    CERT_SAMPLES, CertCheck, Certificate, CertifiedTurn, CertifyError, DraftedIntersection,
    EdgeCurve, EdgeCurveSpec, ForwardReading, IntersectionDraft, MintedEdge, NurbsLane, RowRefusal,
    SplineFoot, StatedImage, StatedSpan, WindingReading, folded_parallelism, sample_param,
    schedule_param,
};
pub use conic_bend::{BendBounds, ConicBend, ConicDeparture, conic_bend_side};
pub use description::{
    ChartCurve, CurveSelector, EdgeAuthority, EdgeDescription, EdgeDescriptionSpec, SharedRow,
    Structural, authority_of,
};
pub use dihedral::{
    DIHEDRAL_ARM, DIHEDRAL_ARM_CLAUSE, DIHEDRAL_ARM_NOUN, DihedralClass, DihedralReading,
    MATERIAL_PAIRING, MATERIAL_PAIRING_CLAUSE, MaterialPairing, MaterialWedge, MixedStations,
    MustCarryDescription, MustCarryEscalation, MustCarryRefusal, MustCarryVerdict, PairRefusal,
    RestLocus, SeamEscalation, SeamFace, SecondOrder, SecondOrderWalk, SmoothJoin, Station,
    StationHook, classify_dihedral, classify_material_pairing, classify_material_pairing_as,
    classify_seam, folded_lever_arm, interior_stations, material_kappa_rel, must_carry_description,
    must_carry_over_edge, reask_chart, reask_pair, second_order_walk, tangent_second_order,
};
pub use dihedral_side::{
    CreaseReading, EdgeHalf, EdgeWalk, FaceSide, SideEscalation, SideFace, SideRole,
    chart_normal_at, dihedral_side, regular_jet,
};
pub use edge_nurbs::{
    AnalyticRung3Refusal, ArcRimRefusal, CARRIER_DOMAIN_RECOURSE, CarrierDomainFault,
    CarrierDomainRefusal, KERNEL_LIMIT_EXTENT_FACTOR, KERNEL_LIMIT_ROUNDING_FACTOR,
    PlaneNurbsIncidence, PlaneNurbsLimbs, PlaneNurbsRefusal, Rung3Limb, analytic_edge_limbs,
    analytic_rung3, arc_rim_candidates, arc_rim_measured, first_within, is_analytic,
    nurbs_pair_limbs, pick_arc_rim, plane_nurbs_incidence, plane_nurbs_limbs, probe_pick,
    sub_arc_joint,
};
pub use enters::{
    EntersMaterial, LeverEscalation, LeverRung, NormalAlignment, OutwardNormal, ParallelNormal,
    ParallelSense, ReferenceNormal, TiedGermsError, WallBend, WallBendError, bends_into_material,
    enters_material, enters_material_order2, enters_material_reported, normal_alignment,
    tied_germ_turn,
};
pub use extent::{
    ExtentBall, LeverBounds, Reach, control_net_reach, edge_extent_inf, edge_extent_sup,
    edge_reach_bounds, edge_reach_from,
};
pub use fitted_lane::{FITTED_DOOR_HOLDERS, FittedLane};
/// The ring-torus convention's one home is `geom` (below this crate, so
/// the spiric carrier's constructor reads it too); re-exported so the
/// doors above read it by the name they already use.
pub use geom::ring_torus;
pub use identity::{IdentityWalk, identity_walk};
pub use implicit::{
    ARC_RESIDUAL_SAMPLES, CircleSphereHarmonic, Conic, ConicHarmonics, ConicTorusHarmonics,
    SurfaceSide, circle_arc_residual_range, circle_residual_curvature_bound,
    circle_residual_extremes, circle_sphere_harmonic, cone_elevation, conic_arc_residual_range,
    conic_cone_harmonics, conic_cone_residual, conic_cylinder_harmonics, conic_quadric_residual,
    conic_residual_extremes, conic_sphere_harmonics, conic_torus_harmonics, conic_torus_implicit,
    conic_torus_jet, conic_torus_residual, curvature_lever_arm, implicit_gradient,
    implicit_graph_acceleration, implicit_hessian_form, implicit_max_normal_curvature,
    implicit_outward_normal, implicit_residual, implicit_section_acceleration,
    min_radius_of_curvature, min_radius_of_curvature_toward,
};
pub use intersect::{
    ConeCylinderSection, ConicArcError, ConicBranch, EqualCylinderSection, OpenConic,
    OpenConicSection, PairRoute, ParallelAxes, PlaneConeSection, PlaneCylinderSection, PlaneSite,
    PlaneSphereSection, PlaneTorusSection, Rung, SectionError, SectionPlane, SectionRadius,
    SphereSphereSection, cone_cylinder_section, cylinder_axes_coplanar, cylinder_axes_parallel,
    cylinder_cylinder_section, parallel_axes_at, plane_cone_apex_extrema, plane_cone_section,
    plane_cylinder_section, plane_cylinder_section_read, plane_sphere_section, plane_torus_section,
    route, route_pose, sphere_sphere_section, tilted_section_major,
};
pub use keys::{CurveKey, PointKey, SurfaceKey, SurfacePair};
pub use locus::{
    TangentLocus, TangentLocusError, tangent_locus, tangent_locus_kinds, tangent_locus_of_faces,
};
pub use mapped::{MappedCurve, MappedSource, SketchSegment, SubRange};
pub use newell::{NewellError, newell_plane};
pub use nurbs_iso::{
    IsoRowError, boundary_iso_u, boundary_iso_v, interior_iso_u, iso_boundary_row, reversed_column,
};
pub use offset::{
    ConeOffset, Nappe, OffsetDistanceError, OffsetError, offset_distance, offset_surface,
};
pub use offset_fit::{
    BestBound, OffsetCertificate, OffsetFitError, OffsetFitWall, OffsetLimb, approx_offset_surface,
    approx_offset_surface_at, certify_offset, certify_offset_at, certify_offset_over,
    certify_offset_over_at, fit_offset, fit_offset_at, recertify_approx, recertify_approx_at,
};
pub use offset_fit_lane::{OFFSET_FIT_DOOR_HOLDERS, OffsetFitLane, ScalarList};
pub use pair_section::{
    AxisPose, CarriedComponent, CarriedSection, CarryError, ClassCarrier, Coincident,
    ComponentShape, Components, CsDecision, CsStance, CtStance, CylinderSpherePose,
    CylinderTorusPose, FittedCarrier, Gap, PairPose, PairSection, PoseRefusal, Ring, Seated,
    SectionComponent, SectionDecision, SectionRead, SiteFoot, SiteRefusal, SphereTorusPose,
    SquareWall, SsiLane, StStance, TT_COINCIDENT, TT_COINCIDENT_FLOOR, TorusCoincidence, TorusData,
    TorusDatum, TorusDisplacement, TorusGap, TorusPoseRefusal, TorusQuestion, TorusTorusPose,
    TtStance, Turn, Winding, WindsRefusal, ZeroKind, ZeroVerdict, axis_pose, line_tilt,
    pair_section, square_part, tori_coincide,
};
pub use pcurve::{
    PCURVE_FIT_SAMPLES, PcurveError, ellipse_pcurve_on_cylinder, ellipse_pcurve_on_plane,
};
pub use pcurve_cache::{
    ANGLE_WINDOW_START, AxisMap, Azimuth, BranchMiss, Channel, ChartMap, ChartMapRefusal,
    ChartStretchInf, ChartWindow, EnvelopeStatement, EnvelopeTerm, FocalImage, FramedCarrier,
    ISO_FAMILY_CLAUSE, IncidenceRead, IsoFamily, IsoFamilyRefusal, LaneGap, NoChartSup, OffMargin,
    Pcurve, PcurveCache, PcurveCertificate, PcurveCertifyError, PcurveCheck, PcurveKind, Period,
    ProjectedChart, ProjectedImage, SectorChannel, SpiricImage, WalkStop, WallTorus, angle_rate,
    chart_iso_family, chart_pcurve, chart_pcurve_over, chart_stretch_inf, chart_stretch_sup,
    chart_stretch_sup_v, chart_sup_arms_at, harmonic_form_of, iso_arc_spans,
};
pub use props::{
    FaceContribution, LoopEdge, PropsError, curved_face, planar_face, require_iso_rectangle,
    require_one_chart_branch,
};
pub use ssi::{
    BandVerdict, BoundaryChart, BoundaryPoint, BoundarySection, ChartAxis, ChartCorner, ChartEnd,
    ChartSide, ChartSpeedRefusal, ChartedNurbs, DomainField, ExhaustLane, Exhaustiveness,
    ExhaustivenessRefusal, FloorFault, FloorKind, FloorRefusal, PlaneSection, ReachBound,
    SSI_FIT_DEGREE, SSI_FLOOR, SSI_MAX_STEPS, Section, SectionReading, SectionRoot,
    SettlingRefusal, SsiBoundaryContact, SsiBranch, SsiCertificate, SsiDecision, SsiDomain,
    SsiError, SsiLimb, SsiOperand, SsiOutcome, SsiTube, StepFault, StepperMode, TangentPair,
    TorusPair, TraceDecision, TubeDegeneracy, boundary_section, certify_rung3, cylinder_sphere_ssi,
    cylinder_torus_ssi, idealized_trace_r3, march_cylinder_sphere, march_cylinder_torus,
    march_torus_torus, plane_nurbs_ssi, sphere_torus_ssi, torus_torus_ssi,
    trace_plane_nurbs_uncertified,
};
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use ssi::{charted_nurbs_mints, implicit_pair_marches};
pub use tangent::{
    TangentJet, TangentSpanBounds, tangent_certificate_lane, tangent_jet, tangent_span_bounds,
};
pub use torus_convention::{TorusConvention, TorusRefusal, TorusVerdict};
