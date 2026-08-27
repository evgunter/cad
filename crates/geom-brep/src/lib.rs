//! The B-rep geometry layer: D2's intensional edge descriptions,
//! certified carrier caches, the dihedral classification predicate, and
//! Newell face equations (M2 PR 3).
//!
//! This crate sits between the evaluators (`geom`) and the arena
//! store (`topo`): it defines **what an
//! edge's geometry is** ([`EdgeGeometry`] — a description, never a bare
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

pub mod certify;
pub mod dihedral;
pub mod edge_geometry;
pub mod edge_nurbs;
pub mod enters;
pub mod implicit;
pub mod intersect;
pub mod keys;
pub mod newell;
pub mod nurbs_iso;
pub mod offset;
pub mod offset_fit;
pub mod offset_meters;
pub mod patch_bound;
pub mod pcurve;
pub mod pcurve_cache;
pub mod props;
pub mod ssi;
pub mod tangent;

pub use certify::{
    CERT_SAMPLES, CertCheck, Certificate, CertifyError, EdgeCurve, EdgeCurveSpec, edge_extent,
    sample_param,
};
pub use dihedral::{DihedralClass, classify_dihedral};
pub use edge_geometry::{EdgeGeometry, MappedCurve, SketchSegment};
pub use edge_nurbs::{EdgeNurbsLane, PlaneNurbsLimbs, PlaneNurbsRefusal};
pub use enters::{
    EntersMaterial, OutwardNormal, ReferenceNormal, enters_material, enters_material_order2,
};
pub use implicit::{
    circle_residual_curvature_bound, circle_residual_extremes, curvature_lever_arm,
    implicit_gradient, implicit_hessian_form, implicit_max_normal_curvature, implicit_residual,
};
pub use intersect::{
    EqualCylinderSection, PairRoute, PlaneConeSection, PlaneCylinderSection, PlaneSphereSection,
    RadiusEvidence, Rung, SectionError, SurfaceKind, cylinder_cylinder_section, plane_cone_section,
    plane_cylinder_section, plane_sphere_section, route,
};
pub use keys::{CurveKey, PointKey, SurfaceKey};
pub use newell::{NewellError, newell_plane};
pub use nurbs_iso::{IsoRowError, boundary_iso_u, boundary_iso_v, iso_boundary_row};
pub use offset::{ConeOffset, OffsetError, offset_surface};
pub use offset_fit::{
    OffsetCertificate, OffsetFitError, OffsetLimb, approx_offset_surface, certify_offset,
    fit_offset, recertify_approx,
};
pub use pcurve::{
    PCURVE_FIT_SAMPLES, PcurveError, ellipse_pcurve_on_cylinder, ellipse_pcurve_on_plane,
};
pub use pcurve_cache::{
    ChartWindow, EnvelopeStatement, Pcurve, PcurveCache, PcurveCertificate, PcurveCertifyError,
    PcurveCheck, PcurveFittedLane, chart_pcurve,
};
pub use props::{FaceContribution, LoopEdge, PropsError, curved_face, planar_face};
pub use ssi::{
    Exhaustiveness, SSI_FIT_DEGREE, SSI_FLOOR, SSI_MAX_STEPS, SsiBranch, SsiCertificate, SsiDomain,
    SsiError, SsiLimb, SsiOperand, SsiOutcome, StepperMode, certify_rung3, cylinder_sphere_ssi,
    idealized_trace_r3, plane_nurbs_ssi, trace_plane_nurbs_uncertified,
};
pub use tangent::{
    TangentJet, TangentSpanBounds, tangent_certificate_lane, tangent_jet, tangent_span_bounds,
};
