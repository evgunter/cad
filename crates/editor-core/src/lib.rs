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
pub mod clearance;
mod decision;
pub mod diff;
pub mod distribution;
pub mod doc;
/// The E6 subdivision driver — the analysis lane's parameter-box
/// verdict. The leaf protocol replays at the certified interval
/// scalar: a driver that fell back to `f64` would be a sampler.
pub mod drive;
pub mod edit;
pub mod eval;
pub mod expr;
mod finding;
pub mod ident;
pub mod label;
pub mod mate;
/// The E11.1 Monte-Carlo ADVISORY estimator lane (ruling Q3): pure f64
/// replay over samples drawn from the document's own distributions.
/// Never gates, never persists as an assertion, never enters the
/// accounting. Nothing in this module needs the certified scalar.
pub mod mc;
pub mod measure;
pub mod meta;
pub mod mint;
pub mod names;
pub mod node;
pub mod param_source;
pub mod parse;
pub mod part;
pub mod persist;
pub mod placement;
pub mod product;
pub mod program;
/// The certified locally-valid range of ONE field — the on-demand
/// query whose answer is meant to REPLACE the sampling probe's
/// reading, in a consumer nothing in this tree has built yet
/// (`work/offer/certify-affordance-on-the-bounds-panel`,
/// `work/lib/certified-range-has-no-python-door`). The certificate IS
/// a [`mod@drive`]'s leaves, and a query that fell back to `f64` would
/// be the sampler it exists to improve on.
pub mod range;
pub mod refactor;
pub mod refusal;
/// The E10/E11.6 reporting layer: the goldening and human forms every
/// derived report carries, the priced-vs-forced budget type, the
/// leaf-mass histogram, and the one content-key cache. Every report
/// in it is derived from a drive.
pub mod report;
pub mod resolve;
pub mod roots;
pub mod sentence;
pub mod spoken;
/// The E4 sensitivity driver and the E5 stackup — the analysis lane's
/// derivative and report services over [`mod@drive`]'s leaves. Every
/// sensitivity carries a chamber mark whose certified variant IS an E6
/// leaf identity, and the gating `worst_case` is a certified interval
/// enclosure.
pub mod stackup;
pub mod step_handle;
// Test fixtures (the literals and the pick door); see the module's
// docs. The gate is this crate's `test-support` feature, on only
// through dev-dependency edges. `doc(hidden)` because the rustdoc gate
// runs `--all-features`.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;
mod tree;
pub mod update;
mod verbs;
pub mod witness;

pub use analysis::{
    AnalysisPolicy, AnalysisPolicyError, AnalyzedBox, AnalyzedParam, AxisScalar, BoxAxis,
    DEFAULT_QUANTILE_MASS, MeasureUnavailable, OffsetInterval, ParamBox, ParamBoxError, SeedError,
    SeedScalar, analyzed_box, box_mass, param_env_over, sample_offset, seed_env, std_deviation,
    tail_mass,
};
pub use appearance::{
    AppearanceLoss, AppearanceLossCause, AppearanceMap, AppearanceRecord, AppearanceResolution,
    Attr, AttrKind, AttrSet, Rgba8,
};
pub use assembly::{
    Assembly, AssemblyError, AtRestFinding, Attribution, CarriedDeclaration, CarriedDeclarations,
    CarriedRefusal, CarriedUnplaced, MintRefusal, MintedDeclaration, RefusedRef, Relation, Route,
    assemble, assemble_gathered,
};
pub use checks::{
    Advisory, ChartCoherenceLane, CheckEvidence, CheckFinding, CheckId, CheckKind, CheckRefusal,
    ChecksConfig, ChecksError, ChecksReport, Severity, Subject, enforce_checks, run_checks,
    run_checks_on, subject_body,
};
pub use diff::{DocDiff, NodeChange};
pub use distribution::{Distribution, DistributionFault, DistributionField};
pub use doc::{
    DisplayUnitRefusal, DistributionRefusal, Doc, DocParam, DocParamField, DocParamValue, ParamName,
};
pub use drive::{
    BudgetKind, CertifiedLeaf, DEFAULT_MAX_DEPTH, DEFAULT_MAX_LEAVES, DriveConfig, DriveRefusal,
    FlipEvidence, LeafResults, MeasureAccounting, ParamBoxVerdict, ReasonClass, Receipt,
    RefusalReason, RefusedLeaf, StructureFlip, drive,
};
pub use edit::{
    Applied, CarryForwardDoor, DocEdit, EditError, EditRecord, Maintenance, MaintenanceNet,
    Recorded, Recording, RegaugeThenMateOutcome, apply, apply_replayed, cascade_delete_order,
    regauge_then_mate,
};
pub use eval::{
    Arity, BooleanValue, CancelToken, CanonicalSegment, CarriedChain, CarriedIn, CarriedLevel,
    ContentBits, ContentKey, DatumValue, DirectionRefusal, Epoch, EvalOptions, EvalOutcome,
    EvalScalar, Evaluation, FramePlacement, NamingKey, NodeError, NodeErrorClass, NodeErrorKind,
    NodeRefusal, NodeResult, NodeStanding, NodeValue, PartFault, PartReach, PiecesFault,
    ProfileLift, ProfilePieces, SectionScalar, SplitSide, ValuePayload, VerbKind, evaluate,
    mate_reach,
};
pub use refusal::Refusal;
pub use sentence::{Labelled, Labels, PASS_A_RESOLVER, Recourse, Staged};
pub use spoken::{
    FullId, HeldNodes, Said, Say, Speaker, SpokenName, SpokenNode, held_by, node_kind_noun,
    spoken_by,
};
// The entity door's token: a field of four `NodeErrorKind` variants, so
// a reader that matches one needs to be able to name it here rather
// than through the module path.
pub use eval::entity_door::Found;
pub use expr::{
    Dimension, DimensionError, EvalError, Expr, ExprPath, ParamEnv, ParamValue, UnitSym, eval,
    eval_count, unparse,
};
pub use ident::{ContentPin, DocRef, DocumentId, Mispaired};
pub use label::{Label, LabelFault};
pub use mate::{
    Alignment, AuthoredFrame, AxisSense, CLASS_DEFERRAL, CONTRADICTORY_RECOURSE, Clash,
    ClassAdmission, Coset, FacePoseRefusal, FaceRefusal, Lever, LeverRefusal, MateFault, MateFrame,
    MatePrimitive, MateReach, MateRole, MateSide, Member, NO_AT_REST_RECORD_RECOURSE,
    OFFSET_RECOURSE, OffsetCheck, PlacerRow, PoseRefusal, ReachRefusal, RefusingReach, SolvedPoses,
    Space, Subgroup, UNDER_RECOURSE, UNPLACED_RECOURSE, Unplaced, class_admission, gauge_chain,
    groups, head_face, member_of, places, reading_edges, relative_freedom_components, root_of,
    solve_document, table_gap,
};
pub use mc::{
    DEFAULT_SAMPLES, DEFAULT_SEED, McAssertion, McConfig, McMeasure, McRefusal, McReport,
    monte_carlo, sample_offsets,
};
pub use measure::{
    ASSERT_BOUND, AssertionDir, AssertionVerdict, Certified, MeasureExpr, MeasurePrimitive,
    MeasureUnavailableAt, MinClearanceLane, MinClearanceOperand, UnevaluatedReason,
    WINDOW_TIGHTENING,
};
pub use meta::{MetaError, MetaValue, MetaVersionError, from_value, to_value};
pub(crate) use mint::NodeIdCollides;
pub use mint::{Mint, Minted};
pub use names::{
    BooleanCoincidence, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactRefusal,
    ContactVerdict, CurveKind, CurveKindSet, DeclareError, DeclaredContact, Denotation,
    DuplicateName, EntityKey, EntityKind, EntityRef, Entry, FIT_DEFERRAL, FaceName, FlushEvidence,
    FlushFinding, FlushRung, FragmentGroups, GeomPred, InterrogateError, MeridianEnd, NameOrigin,
    NamePat, NameRef, NameTable, NameTextError, NamingError, NotAFaceName, OpGroup, PieceRole,
    PieceRun, ProfileEdgeRef, ProfileVertexRef, Qualifier, RimShare, RimSupport, RolePath, RoleSeg,
    SEL_DATUM_DISTANCE, SectionCircle, SegPat, SegTag, SelectRefusal, Selector, Side, SplitHalf,
    StableName, SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices, attribute,
    band, band_pi, band_rim, carried, declare, declare_all, declared_pairs, denotation,
    edge_carrier_kind, edge_frame, face_carrier_kind, face_frame, find_flush_candidates,
    meridian_vertex, select, select_where, vertex_position,
};
pub use node::{
    Axis3, BooleanOp, CountMismatch, Datum, DeclaredPair, InputFault, InterfaceCrossing,
    InterfaceRecord, ListFault, MeasureNodeFault, Node, PartSelect, PatternKind,
    PlacementRuleFault, RecipeNodeId, RigidArg, SitedFace, SitedRef, SlotId, StepArg, StepId,
    TubeWindow, VectorSlot, declare_continuation, declare_rest,
};
pub use parse::{ParamNameFault, ParamNameReason, ParseError, parse_expr};
pub use part::{PartResolver, ResolveFailure, ResolveFault};
pub use persist::{
    Loaded, PersistError, REGENERATE_RECOURSE, canonical_bytes, content_pin, header_document_id,
    load, save,
};
pub use persist::{NonFiniteSite, ProgramFault, SnapshotError};
pub use placement::{AxisRefusal, Frame, FrameFault, FrameSite, Placement, Step};
#[cfg(debug_assertions)]
pub use product::gathers_on_this_thread;
pub use product::{
    OwnSpace, Product, ProductError, ProductErrorKind, ProductRefusal, SourceFinding, own_spaces,
    product, product_named, product_recorded,
};
pub use program::{
    LoopProgram, ProfileDoc, ProfilePayload, ProfileProgram, ProgramArcData, ProgramRefusal,
    ProgramStep, ProgramTarget, RecordedNotation, RecordedProgramError, StepIdFault,
    StepSegmentsError, resolve_loops,
};
pub use range::{
    CertifiedRange, DerivedRange, RangeField, RangeRefusal, RangeSeed, RangeSide, certified_range,
};
pub use refactor::{
    InlineError, InlineOutcome, NodeMap, SplitError, SplitOutcome, StepMap, Unmapped, inline,
    remap_name, split,
};
pub use report::{
    HistogramRow, LeafHistogram, MassBasis, MassBudget, ReportCache, leaf_histogram, report_key,
};
pub use resolve::{
    Diagnosis, FlipSet, FoldConsumption, GroupCutters, HitTestError, MeshPatchKey,
    NodeVerdictDelta, PredicateDivergence, RecipeEditRef, Resolution, ResolutionFailure,
    ResolveError, ResolveIndeterminate, Resolved, RunCtx, RunStatus, TieWitness, Tombstone,
    UnnamedEntity, UpstreamCause, VerdictFlip, appearance_rebind_suggestions, apply_with_names,
    body_name, derivation_nodes, diff_verdicts, edge_name, enrich_appearance_loss,
    enrich_appearance_loss_with_prior, entity_name, face_name, rebind_suggestions, resolve,
    resolve_with_prior, vertex_name,
};
pub use resolve::{
    NodeVerdicts, SummaryDelta, SummaryDivergence, SummaryFlip, SummaryFlipSet, VerdictRow,
    VerdictSummary, VerdictVector, VerdictVectorKey, diff_summaries, verdict_summary,
};
pub use step_handle::{
    ArcShape, AuthoredStep, StepHandleRefusal, StepShape, TargetShape, keep_grid,
};
// GUI-1: the hit-test service (G1 `ray → stable ref`), with the ray
// vocabulary re-exported from `bvh` so a layer-3 consumer needs no
// direct bvh dependency.
pub use bvh::Ray;
pub use resolve::{
    MeshPick, MeshPickError, NameLookupError, NodePick, NodePickError, PickHit, PickMemo,
    PickTarget, pick_face,
};
pub use roots::RootFault;
pub use stackup::{
    Chamber, ChamberSpan, DivergedAt, LiftRefusal, PairingViolation, PerParam, Rss, Sensitivity,
    SensitivityOutcome, SensitivityRefusal, Stackup, StackupRefusal, Unavailable, WorstCase,
    render_sensitivity, sensitivities, stackup,
};
pub use update::{PinMultiplicity, PinSites, UpdateError, mixed_pins, update_references};
pub use witness::{
    BifurcationKind, BranchCertification, BranchMarginEvidence, Implicated, WitnessAge,
    WitnessBifurcation, WitnessDatum,
};
