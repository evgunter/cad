//! Topology arenas, [`Body<T>`], and the structural validation harness —
//! the `topo` layer with M1's half-edge representation (D1 in
//! `docs/DESIGN.md`).
//!
//! A B-rep body is a plain value: typed generational arenas
//! (slotmap-style, D1) for the seven manifold topology kinds — solid,
//! shell, face, loop, half-edge, edge, vertex — plus geometry arenas,
//! plus a D5 provenance record per entity from birth. M1 PR 1 landed the
//! half-edge structure itself (adjacency, orientation conventions, the
//! tier-1 structural validator); PR 2 added the first Euler operators —
//! [`Body::mvfs`], [`Body::mev`], [`Body::mef`] (see [`euler`]) — the
//! sanctioned construction path; PR 3 added the ring/genus operators —
//! [`Body::kemr`], [`Body::mekr`], [`Body::kfmrh`] — and the non-Euler
//! [`Body::ring_move`] helper (see [`euler_ring`]); PR 4 completed the
//! ten-operator catalog with the kill-direction duals — [`Body::kvfs`],
//! [`Body::kev`], [`Body::kef`], [`Body::mfkrh`] (see [`euler_kill`]) —
//! plus the make/kill roundtrip property-test machinery; PR 5 completed
//! the validator (validity tiers — [`fn@validate`] accepts every
//! Euler-reachable state, [`validate_closed`] the finished closed
//! solids) and retired the raw-insertion builder to `pub(crate)`: **the
//! Euler operators are the only public construction path** (D1).
//!
//! **M2 PR 3 — real geometry.** The M0 placeholder geometry retired:
//! the curve arena holds certified `geom_brep::EdgeCurve`s (D2
//! intensional description + carrier cache + certification record) and
//! the surface arena holds `geom_surfaces::Surface`s. Edge-minting
//! operators take an uncertified `EdgeCurveSpec` and run the D4 ¶2
//! certification gate before mutating (chord-line sugar:
//! [`Body::mev_line`] / [`Body::mef_chord`] / [`Body::mekr_chord`] /
//! [`Body::mfkrh_plug`]); face-minting operators take a
//! [`FaceSurface`] spec; the two attachment setters
//! ([`Body::set_face_surface`], [`Body::set_edge_curve`] — see
//! [`attach`]) cover the construction-order cases mint-time attachment
//! cannot reach. Tier 3 ([`validate_geometric`]) re-runs every
//! certification at rest and adds the dihedral classification pass.
//!
//! # Orientation conventions
//!
//! Documented **once**, in the [`entity`] module docs: the interior-left
//! rule (counterclockwise outer loops viewed from outside), antiparallel
//! half-edge pairs, derived end vertices, the vertex-orbit step, and the
//! GWB-is-mirrored transcription warning. Start there before touching
//! topology code.
//!
//! # The genericity boundary (Q1, settled by this crate's shape)
//!
//! `Body<T>` is **scalar-free topology plus geometry arenas over `T`**.
//! Topology entities contain no scalar values and no comparisons — only
//! typed keys — while the geometry arenas (points, curves, surfaces) are
//! the only `T`-carrying storage. Topology never branches on `T`: every
//! topology-determining decision was made at construction time through
//! named predicates (Q1 in `docs/DESIGN.md`), and a body is the *result*
//! of those decisions. An interval replay therefore materializes a
//! `Body<Interval>` with identical topology keys and interval-valued
//! geometry.
//!
//! # Determinism (D9)
//!
//! Arena iteration is slot-index order — deterministic given identical
//! construction history, which the pure-replay model guarantees. The
//! standing invariant (documented at [`body`]): no arena iteration may
//! feed a geometry decision unless the construction sequence is itself
//! deterministic; accordingly this crate contains no `HashMap`/`HashSet`
//! in any position that can feed a decision. One `HashSet` does exist —
//! `transform.rs`'s `rewritten` set, which tracks which curve keys the
//! walk remapped so an orphaned entry is refused as corruption. It is
//! **membership-only and never iterated**, so no hash order can reach
//! an output; a `SecondaryMap` would be both cheaper and consistent
//! with the rule, and is the preferred form for anything new.
//! Half-edge traversal is **bounded** — the walks cap at the arena
//! length and fail loud instead of spinning on corrupt input (the kernel
//! never hangs, D9).
//!
//! # Example
//!
//! Build the unit cube through the Euler operators — the §9.4.2-minimal
//! sequence, 1 `mvfs` + 7 `mev` + 5 `mef` — and validate it at both
//! tiers. Every intermediate state is tier-1 valid ([`fn@validate`]); the
//! finished cube is a tier-2 closed solid ([`validate_closed`]).
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, MefSite, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let pt = Point3::new;
//! let mut body = Body::<f64>::new();
//!
//! // The seed: solid + shell + one face holding lone vertex A.
//! let seed = body.mvfs(pt(0.0, 0.0, 0.0))?;
//! // The bottom rim A → B → C → D, grown by three mev …
//! let e_ab = body.mev_line(MevSite::Lone { r#loop: seed.r#loop }, pt(1.0, 0.0, 0.0))?;
//! let strut = |he| MevSite::Fan { he1: he, he2: he };
//! let e_bc = body.mev_line(strut(e_ab.he_minus), pt(1.0, 1.0, 0.0))?;
//! let e_cd = body.mev_line(strut(e_bc.he_minus), pt(0.0, 1.0, 0.0))?;
//! // … and closed by a mef splitting off the bottom face.
//! let he_dc = body.find_half_edge(seed.face, e_cd.vertex, e_bc.vertex).unwrap();
//! let f_bot = body.mef_chord(MefSite::Chords { he1: he_dc, he2: e_ab.he_plus })?;
//! // Four vertical struts up from the rim …
//! let e_aa = body.mev_line(strut(e_ab.he_plus), pt(0.0, 0.0, 1.0))?;
//! let e_bb = body.mev_line(strut(e_bc.he_plus), pt(1.0, 0.0, 1.0))?;
//! let e_cc = body.mev_line(strut(e_cd.he_plus), pt(1.0, 1.0, 1.0))?;
//! let e_dd = body.mev_line(strut(f_bot.he_plus), pt(0.0, 1.0, 1.0))?;
//! // … and four mef close the side faces (the seed face becomes the top).
//! let chord = |he1, he2| MefSite::Chords { he1, he2 };
//! let f_front = body.mef_chord(chord(e_aa.he_minus, e_bb.he_minus))?;
//! body.mef_chord(chord(e_bb.he_minus, e_cc.he_minus))?;
//! body.mef_chord(chord(e_cc.he_minus, e_dd.he_minus))?;
//! body.mef_chord(chord(e_dd.he_minus, f_front.he_plus))?;
//!
//! assert_eq!(body.vertices().count(), 8);
//! assert_eq!(body.edges().count(), 12);
//! assert_eq!(body.faces().count(), 6);
//! assert_eq!(topo::validate(&body), Ok(()));        // tier 1: euler-valid
//! assert_eq!(topo::validate_closed(&body), Ok(())); // tier 2: closed solid
//!
//! // Mid-construction scaffolding is tier-1 legal but not tier-2: a
//! // strut fails `validate_closed` with a typed, entity-named error.
//! let scaffold = body.mev_line(strut(e_ab.he_plus), pt(2.0, 0.0, 0.0))?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert_eq!(
//!     topo::validate_closed(&body),
//!     Err(vec![topo::ValidationError::ScaffoldingStrutVertex {
//!         vertex: scaffold.vertex,
//!     }]),
//! );
//! # Ok(()) }
//! # run().unwrap();
//! ```

pub mod attach;
pub mod body;
pub mod boolean;
pub(crate) mod census;
pub mod chart_region;
pub mod contact;
pub mod entity;
pub mod euler;
pub mod euler_kill;
pub mod euler_ring;
#[cfg(test)]
pub(crate) mod fixtures;
pub mod geometry;

pub mod instance;
#[cfg(test)]
pub(crate) mod iso;
pub mod merge_faces;
pub mod movefac;
pub mod null;
pub mod pcurves;
pub mod props;
pub mod provenance;
pub mod revert;
#[cfg(test)]
mod review_m0_pr7;
#[cfg(test)]
mod review_m1_pr1;
#[cfg(test)]
mod review_m1_pr2;
#[cfg(test)]
mod review_m1_pr3;
#[cfg(test)]
mod review_m1_pr4;
#[cfg(test)]
mod review_m1_pr5_internal;
pub mod separation;
#[cfg(test)]
pub(crate) mod seqgen;
pub mod source;
pub mod split;
pub mod splitting;
#[cfg(test)]
mod tier3_tests;
pub mod transform;
pub mod validate;

pub use body::Body;
pub use boolean::{
    BoolNullEdgeRecord, BooleanBody, BooleanDeclarations, BooleanError, BooleanNaming, BooleanOp,
    BooleanReduction, BooleanResult, BooleanResultKind, CarriedContacts, CarriedVf, CarriedVv,
    CarrierDesc, CarrierEqError, CarrierRelation, CompletedPolygonPair, ContactRecords,
    CurveContact, FaceContainment, FacePairDeclaration, NullEdgePairRecord, Operand, OperandKeys,
    PairSite, PatchContact, PierceRingRecord, PlaneDesc, PlaneEqError, PlaneIdentity,
    PlaneRelation, PointInSolidError, SideCode, SolidContainment, SweepStrategy, SweepTrace,
    VfContact, VvContact, boolean_op_with, boolean_reduce, boolean_reduce_declared, carrier_eq,
    contfp, face_carrier, flush_pair_relation, intersect, intersect_with, oriented_plane_eq,
    point_in_solid, subtract, subtract_with, tangent_pair_relation, union, union_with,
};
// The contact vocabulary (C3/C4), defined once at the lowest crate
// that can hold it: upward layers RE-EXPORT these, never redefine.
#[cfg(feature = "sweep-testing")]
pub use boolean::{PlantedDegradation, sweep_traces, sweep_traces_with_pad};
pub use contact::{
    CONTACT_RECOURSE, ContactClass, ContactFinding, ContactRefusal, ContactVerdict,
    DeclaredContact, FIT_DEFERRAL,
};
pub use entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
pub use euler::{EulerOpError, FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
pub use euler_kill::{KefResult, KevResult, KvfsResult, MfkrhCreated};
pub use euler_ring::{KemrResult, KfmrhResult, MekrResult, MekrSite};
// The types that appear in this crate's own operator signatures, so a
// consumer of the ops needs no direct geom-* imports for the common
// path (the full geometry vocabulary still lives in those crates).
pub use chart_region::{ChartOverlap, ChartRegionError, chart_region_overlap};
pub use geom_brep::{
    CertifyError, ChartWindow, EdgeCurve, EdgeCurveSpec, EdgeGeometry, EdgeNurbsLane, Pcurve,
    PcurveCache, PcurveCertifyError,
};
pub use geom_curves::Curve3;
pub use geom_surfaces::Surface;
pub use geometry::{CurveKey, PointKey, SurfaceKey};
pub use instance::{
    GraftKeys, graft_disjoint, graft_disjoint_all, graft_disjoint_all_keyed,
    graft_disjoint_all_onto_keyed,
};
pub use merge_faces::{MergeCoplanarError, MergeCoplanarOutcome, MergedGroup, SkippedMerge};
pub use null::{CurveGeom, NewVertexSide, NullEdge, NullFacePair};
pub use pcurves::{PcurveMintError, mint_pcurves, pcurve_of};
pub use props::{MassProperties, MassPropsError, PropsQuadLane, mass_properties};
pub use provenance::Provenance;
pub use revert::RevertError;
pub use separation::{PlacementsMeet, Separation};
pub use source::{GeomSource, Or, SourceAttachError, SourceExpr};
pub use split::SplitEdgeCreated;
pub use splitting::{
    ArcWindowCase, LoopContainment, NullEdgeRecord, PlaneSide, PointInLoopError, Section,
    SectionPolygon, SectorEntry, SectorEntryKind, SplitError, SplitFinishError, SplitJoinError,
    SplitPart, SplitPlane, SplitReduceError, SplitReduction, SplitResult, classify_neighborhood,
    plane_section, point_in_loop, split, split_reduce, vertex_sides,
};
pub use transform::{TransformError, transform_rigid};
pub use validate::{
    CensusContact, ContactMark, StaleDeclaration, ValidationError, contact_marks, validate,
    validate_closed, validate_geometric, validate_pseudomanifold,
};
