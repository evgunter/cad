//! `editor-core` — the recipe substrate: `Doc` as a plain value, the v1
//! feature-node vocabulary as data, the dimension-checked expression
//! sublanguage, and the `DocEdit` vocabulary with a pure `apply`.
//!
//! Born in M4 PR 1 under the ratified M4-PLAN forks: F1 (restrictive
//! dimension lattice), F4 (node vocabulary), F7 (expression AST with no
//! conditionals — total by construction). This crate holds NO geometry
//! evaluation (PR 2) and NO name resolution (PR 3/4) in its document
//! layer; persistence
//! arrived in M4 PR 6 as [`persist`].
//!
//! Layering (M4 PR 2 spec D1, G1): editor-core sits ABOVE the kernel —
//! the evaluation service ([`mod@eval`]) depends on the op crates it wires
//! (`profile`, `sweep`, `topo`); the kernel crates gain no editor-core
//! dependency. Profiles are carried opaquely in the document (a type
//! parameter, never a re-model); [`ProfileDoc`] is the canonical
//! instantiation at the profile crate's public description type.

pub mod analysis;
pub mod appearance;
pub mod assembly;
pub mod checks;
#[cfg(feature = "interval")]
pub mod clearance;
pub mod diff;
pub mod distribution;
pub mod doc;
/// The E6 subdivision driver — the analysis lane's parameter-box
/// verdict. Gated on `interval` because the leaf protocol replays at
/// the certified interval scalar: without that scalar there is no leaf
/// to certify, and a driver that fell back to `f64` would be a
/// sampler.
#[cfg(feature = "interval")]
pub mod drive;
pub mod edit;
pub mod eval;
pub mod expr;
mod finding;
pub mod ident;
pub mod mate;
pub mod measure;
pub mod meta;
pub mod names;
pub mod node;
pub mod parse;
pub mod part;
pub mod persist;
pub mod placement;
pub mod product;
pub mod program;
pub mod refactor;
pub mod resolve;
pub mod roots;
/// The E4 sensitivity driver and the E5 stackup — the analysis lane's
/// derivative and report services over [`mod@drive`]'s leaves. Gated on
/// `interval` for the driver's own reason: every sensitivity carries a
/// chamber mark whose certified variant IS an E6 leaf identity, and
/// the gating `worst_case` is a certified interval enclosure; without
/// the certified scalar neither exists to be minted.
#[cfg(feature = "interval")]
pub mod stackup;
pub mod update;
mod verbs;
pub mod witness;

pub use analysis::{
    AnalysisPolicy, AnalysisPolicyError, AnalyzedBox, AnalyzedParam, AxisScalar, BoxAxis,
    DEFAULT_QUANTILE_MASS, MeasureUnavailable, OffsetInterval, ParamBox, ParamBoxError, SeedError,
    SeedScalar, analyzed_box, box_mass, param_env_over, seed_env, std_deviation, tail_mass,
};
pub use appearance::{
    AppearanceLoss, AppearanceLossCause, AppearanceMap, AppearanceRecord, AppearanceResolution,
    Attr, AttrKind, AttrSet, Rgba8,
};
pub use assembly::{
    Assembly, AssemblyError, AtRestFinding, Attribution, MintRefusal, MintedDeclaration,
    RefusedRef, assemble,
};
pub use checks::{
    Advisory, CheckEvidence, CheckFinding, CheckId, CheckKind, CheckRefusal, ChecksConfig,
    ChecksError, ChecksReport, Severity, enforce_checks, run_checks, subject_body,
};
pub use diff::{DocDiff, NodeChange};
pub use distribution::{Distribution, DistributionFault, DistributionField};
pub use doc::{Doc, DocParam, DocParamValue, ParamName};
#[cfg(feature = "interval")]
pub use drive::{
    BudgetKind, CertifiedLeaf, DEFAULT_MAX_DEPTH, DEFAULT_MAX_LEAVES, DriveConfig, DriveRefusal,
    FlipEvidence, LeafResults, MeasureAccounting, ParamBoxVerdict, ReasonClass, Receipt,
    RefusalReason, RefusedLeaf, ReplayOutcome, StructureFlip, VerdictRow, VerdictVector,
    VerdictVectorKey, drive,
};
pub use edit::{Applied, DocEdit, EditError, EditRecord, apply, cascade_delete_order};
pub use eval::{
    Arity, BooleanValue, CancelToken, ContentBits, ContentKey, DatumValue, Epoch, EvalOptions,
    EvalOutcome, EvalScalar, Evaluation, NamingKey, NodeError, NodeErrorKind, NodeResult,
    NodeValue, PartFault, ProfileLift, SplitSide, UnitVec3, UnitVec3Error, ValuePayload, VerbKind,
    evaluate,
};
pub use expr::{
    Dimension, DimensionError, EvalError, Expr, ExprPath, ParamEnv, ParamValue, UnitSym, eval,
    eval_count, unparse,
};
pub use ident::{ContentPin, DocRef, DocumentId};
pub use mate::{
    Alignment, AxisSense, CLASS_DEFERRAL, ClassAdmission, ClusterMaintenance, Coset, MateFault,
    MateFrame, MatePrimitive, MateRole, MateSide, SolvedPoses, Subgroup, UNDER_RECOURSE,
    class_admission, clusters, gauge_of, reading_edges, relative_freedom_components,
    solve_document,
};
pub use measure::{
    ASSERT_BOUND, AssertionDir, AssertionVerdict, MeasureExpr, MeasurePrimitive, UnevaluatedReason,
};
pub use meta::{MetaError, MetaValue, MetaVersionError, from_value, to_value};
pub use names::{
    ALL_SURFACE_KINDS, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactRefusal, ContactVerdict,
    CurveKind, CurveKindSet, DeclareError, DeclaredContact, Denotation, DuplicateName, EntityKey,
    EntityKind, EntityRef, Entry, FIT_DEFERRAL, FlushEvidence, FlushFinding, FlushRung, GeomPred,
    InterrogateError, MeridianEnd, NameOrigin, NamePat, NameTable, NamingError, OpGroup,
    ProfileEdgeRef, ProfileVertexRef, Qualifier, RimSupport, RolePath, RoleSeg, SEL_DATUM_DISTANCE,
    SegPat, SegTag, SelectRefusal, Selector, Side, SideVerdict, SplitHalf, StableName,
    SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices, attribute, declare,
    declare_all, declare_node, denotation, edge_frame, face_frame, find_flush_candidates, select,
    select_where, vertex_position,
};
pub use node::{
    Axis3, BooleanOp, Datum, InterfaceCrossing, InterfaceRecord, MeasureNodeFault, MeasureRef,
    Node, PatternKind, PlacementRuleFault, RecipeNodeId, SlotId, StepArg, VectorSlot,
};
pub use parse::{ParseError, parse_expr};
pub use part::{PartResolver, ResolveFailure, ResolveFault};
pub use persist::{
    Loaded, PersistError, REGENERATE_RECOURSE, canonical_bytes, content_pin, header_document_id,
    load, save,
};
pub use persist::{NonFiniteSite, ProgramFault, SnapshotError};
pub use placement::Frame;
pub use product::{Product, ProductError, product, product_named, product_recorded};
pub use program::{
    LoopProgram, ProfileDoc, ProfilePayload, ProfileProgram, ProgramArcData, ProgramRefusal,
    ProgramStep, ProgramTarget, RecordedProgramError,
};
pub use refactor::{InlineError, InlineOutcome, NodeMap, SplitError, SplitOutcome, inline, split};
pub use resolve::{
    Diagnosis, FlipSet, HitTestError, MeshPatchKey, NodeVerdictDelta, PredicateDivergence,
    RecipeEditRef, Resolution, ResolutionFailure, ResolveError, ResolveIndeterminate, Resolved,
    RunCtx, RunStatus, TieWitness, Tombstone, VerdictFlip, appearance_rebind_suggestions,
    apply_with_names, body_name, derivation_nodes, diff_verdicts, edge_name,
    enrich_appearance_loss, enrich_appearance_loss_with_prior, entity_name, face_name,
    rebind_suggestions, resolve, resolve_with_prior, vertex_name,
};
pub use resolve::{
    NodeVerdicts, SummaryDelta, SummaryDivergence, SummaryFlip, SummaryFlipSet, VerdictSummary,
    diff_summaries, verdict_summary,
};
// GUI-1: the hit-test service (G1 `ray → stable ref`), with the ray
// vocabulary re-exported from `bvh` so a layer-3 consumer needs no
// direct bvh dependency.
pub use bvh::Ray;
pub use resolve::{
    MeshPick, MeshPickError, NodePick, NodePickError, PickHit, PickTarget, pick_face,
};
pub use roots::RootFault;
#[cfg(feature = "interval")]
pub use stackup::{
    Chamber, ChamberSpan, LiftRefusal, PairingViolation, PerParam, Rss, Sensitivity,
    SensitivityOutcome, SensitivityRefusal, Stackup, StackupRefusal, Unavailable, WorstCase,
    sensitivities, stackup,
};
pub use update::{PinMultiplicity, PinSites, UpdateError, mixed_pins, update_references};
pub use witness::{
    BifurcationKind, BranchCertification, BranchMarginEvidence, Implicated, WitnessAge,
    WitnessBifurcation, WitnessDatum,
};
