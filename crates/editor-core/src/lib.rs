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
pub mod checks;
pub mod clearance;
pub mod coincide;
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
pub mod formula;
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
pub mod operand;
pub mod param_source;
pub mod parse;
pub mod part;
pub mod persist;
pub mod placement;
pub mod pose;
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
mod ratio;
pub mod refactor;
pub mod refusal;
pub mod repetition;
pub use repetition::{MemberCountFault, MemberEnv, Reached};
/// The E10/E11.6 reporting layer: the goldening and human forms every
/// derived report carries, the priced-vs-forced budget type, the
/// leaf-mass histogram, and the one content-key cache. Every report
/// in it is derived from a drive.
pub mod report;
pub mod resolve;
pub mod sentence;
pub mod spoken;
/// The E4 sensitivity driver and the E5 stackup — the analysis lane's
/// derivative and report services over [`mod@drive`]'s leaves. Every
/// sensitivity carries a chamber mark whose certified variant IS an E6
/// leaf identity, and the gating `worst_case` is a certified interval
/// enclosure.
pub mod stackup;
pub mod step_handle;
pub mod take_over;
// Test fixtures (the literals and the pick door); see the module's
// docs. The gate is this crate's `test-support` feature, on only
// through dev-dependency edges. `doc(hidden)` because the rustdoc gate
// runs `--all-features`.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;
pub mod trace;
mod tree;
pub mod update;
pub mod var;
mod verbs;
pub mod witness;

pub use analysis::{
    AnalysisPolicy, AnalysisPolicyError, AnalyzedBox, AnalyzedParam, AxisScalar, BoxAxis,
    DEFAULT_QUANTILE_MASS, MeasureUnavailable, OffsetInterval, ParamBox, ParamBoxError, SeedError,
    SeedScalar, analyzed_box, box_mass, is_axis, sample_offset, seed_env, std_deviation, tail_mass,
    var_env_over,
};
pub use appearance::{
    AppearanceKey, AppearanceLoss, AppearanceLossCause, AppearanceMap, AppearanceRecord,
    AppearanceResolution, Attr, AttrKind, AttrSet, Rgba8,
};
pub use checks::at_rest::{
    AtRestEvidence, AtRestRow, CarriedAtRestFinding, CarriedCoincidence, CarriedCoincidenceRows,
    CarriedFindings, CarriedInterference, CopyRef, FaceSite, InterferenceFinding, Overlap,
    PinnedDoc, Route, Unlocalized,
};
pub use checks::{
    Advisory, ChartCoherenceLane, CheckEvidence, CheckFinding, CheckId, CheckKind, CheckRefusal,
    ChecksConfig, ChecksError, ChecksReport, DoorFindings, DoorJob, FindingSubject, Severity,
    Subject, enforce_checks, run_checks, run_checks_at_rest_on, run_checks_on, subject_body,
};
pub use coincide::{
    CitedInput, NamedCell, NamedCoincidence, Proof, Residual, RowOrigin, Rung, Unreplayed,
};
pub use diff::{DocDiff, NodeChange};
pub use distribution::{Distribution, DistributionFault, DistributionField};
pub use doc::{
    DisplayUnitRefusal, DistributionRefusal, Doc, DocParamField, FreeValue, FreeVar, Unused,
    VarName,
};
pub use drive::{
    BudgetKind, CertifiedLeaf, DEFAULT_MAX_DEPTH, DEFAULT_MAX_LEAVES, DriveConfig, DriveRefusal,
    FlipEvidence, LeafResults, MeasureAccounting, ParamBoxVerdict, ReasonClass, Receipt,
    RefusalReason, RefusedLeaf, StructureFlip, drive,
};
pub use edit::{
    Applied, CarryForwardDoor, DEFINITION_NODE_BOUND, DocEdit, EditError, EditRecord, IndexFault,
    LoftFault, Maintenance, MaintenanceNet, MeasureOutcome, Measured, ProductReadFault, Recorded,
    Recording, SketchOutcome, SketchedOnFace, SlotValue, Took, apply, apply_replayed,
    cascade_delete_order, measure, sketch_on_face,
};
pub use eval::measure::{Observed, ObservedRefusal};
pub use eval::{
    At, BooleanValue, CancelToken, CanonicalSegment, CarriedChain, CarriedIn, CarriedLevel, Carry,
    ComputingTwist, ContentBits, ContentKey, DirectionRefusal, Epoch, EvalCounters, EvalOptions,
    EvalOutcome, EvalScalar, Evaluation, Framed, NamingKey, NodeError, NodeErrorClass,
    NodeErrorKind, NodeRefusal, NodeResult, NodeStanding, NodeValue, PartFault, Piece, Pieces,
    PoseValue, ProfileLane, ProfilePieces, SplitSide, ValuePayload, evaluate, traced_faces,
};
pub use refusal::Refusal;
pub use sentence::{Labelled, Labels, PASS_A_RESOLVER, Recourse, Staged};
pub use spoken::{
    FullId, HeldNodes, NameTables, Said, Say, Speaker, SpokenName, SpokenNode, SpokenSpace,
    SpokenVar, held_by, node_kind_noun, spoken_by, spoken_within,
};

pub use expr::{
    AuthoredLeaf, Dimension, DimensionError, EvalError, Expr, ExprPath, ExprTree, LeafSet,
    ParamValue, Quantity, Ratio, Slot, StoredLeaf, UnitSym, Unlowered, VarEnv, eval, eval_count,
    eval_var, eval_var_count, unparse,
};
pub use formula::{Formula, FreshFault, LowerFault, NameFault};
pub use ident::{ContentPin, DocRef, DocumentId, Mispaired};
pub use label::{Label, LabelFault};
pub use mate::{
    Composing, Constraint, ConstraintKind, Coord, Coset, FrameMove, LeverRefusal, Mate, MateFault,
    MateRef, MateSide, Member, NoChartWhy, OVERCONSTRAINED_RECOURSE, Orientation, PinRefusal,
    Placed, Placing, PoseRefusal, PoseSymmetry, ReachRefusal, SignedAxis, SlideAxes, SolvedPoses,
    Subgroup, SubgroupFamily, TargetAxis, UNPINNED_RECOURSE, Unobservable, Value, corner_frame,
    corner_frame_in, head_face, head_of, in_space, member_of, member_reading, moved_frame,
    relative_freedom_components, solve_document, stand, unobservable,
};
pub use mc::{
    DEFAULT_SAMPLES, DEFAULT_SEED, McAssertion, McConfig, McMeasure, McRefusal, McReport, McValue,
    monte_carlo, sample_offsets,
};
pub use measure::{
    ASSERT_BOUND, AssertionRelation, AssertionVerdict, MeasurePrimitive, MeasureUnavailableAt,
    MeasureVerb, MinClearanceOperand, UnevaluatedReason,
};
pub use meta::{MetaError, MetaInt, MetaValue, MetaVersionError, from_value, to_value};
pub use mint::{Mint, MintId, Minted};
pub use names::{
    CONTACT_RECOURSE, CapEnd, Cmp, ContactPair, ContactRefusal, CrossingSensesShape, CurveKind,
    CurveKindSet, Denotation, DisagreeShape, DuplicateName, EntityKey, EntityKind, EntityNamed,
    EntityRef, Entry, FaceName, FragmentGroups, GeomPred, InterrogateError, JoinShape,
    KindMismatch, LeafRole, MemberAt, MeridianEnd, NameOrigin, NamePat, NameRef, NameTable,
    NameTextError, NamingError, NotAFaceName, OpGroup, PieceRole, PieceRun, ProfileEdgeRef,
    ProfileVertexRef, Qualifier, RimShare, RimSupport, RolePath, RoleSeg, RunLine,
    SEL_PLANE_DISTANCE, SeamSidesShape, SeamVertexShape, SeatingRefusal, SectionCircle, SegPat,
    SegTag, SelectRefusal, Selector, Sense, ShortTie, Side, SplitHalf, StableName, SurfaceKindSet,
    TagPat, UnionNameShape, WayShape, all_bodies, all_edges, all_faces, all_vertices, attribute,
    band, band_pi, band_rim, band_rim_pi, carried, denotation, edge_carrier_kind, edge_frame,
    face_carrier_kind, face_frame, leaf_role, meridian_vertex, role_leaf, select, select_where,
    vertex_position,
};
pub use node::{
    AuthoredNode, Axis3, ConstraintId, InputFault, InterfaceCrossing, InterfaceRecord, ListFault,
    LoftSection, Node, OutputPort, PortKind, RecipeNodeId, SitedFace, SitedRef, SlotId,
    StationMove, StepArg, StepId, TubeWindow, VectorSlot,
};
pub use operand::{Bodies, BodyRead, Operand, OperandSlot, SlotKind};
pub use parse::{ParseError, VarNameFault, VarNameReason, parse_formula};
pub use part::{PartResolver, ResolveFailure, ResolveFault};
pub use persist::{
    Loaded, PersistError, REGENERATE_RECOURSE, canonical_bytes, content_pin, header_document_id,
    load, save,
};
pub use persist::{NonFiniteSite, OutputFault, ProgramFault, SelectionBodyFault, SnapshotError};
pub use placement::{AxisRefusal, Frame, FrameFault};
#[cfg(debug_assertions)]
pub use product::gathers_on_this_thread;
pub use product::{
    AssembleGather, Product, ProductError, ProductErrorKind, ProductRefusal, SourceFinding,
    assemble_gathers, product, product_named, product_recorded,
};
pub use program::{
    DrawnOn, LoopProgram, ProfileDoc, ProfilePayload, ProfileProgram, ProgramArcData,
    ProgramRefusal, ProgramResolveFault, ProgramStep, ProgramTarget, RecordedNotation,
    RecordedProgramError, SlotPayload, StepIdFault, StepSegmentsError, TraceCrossing, TracePassage,
    TraceSense, WrittenLoopFault, resolve_loops, resolve_written_loops,
};
pub use range::{
    CertifiedRange, DerivedRange, RangeField, RangeRefusal, RangeSeed, RangeSide, certified_range,
};
pub use refactor::{
    FlattenError, FlattenOutcome, InlineError, InlineOutcome, NodeMap, RemapError, SplitError,
    SplitOutcome, StepMap, Uncarried, Unmapped, flatten, inline, remap_name, split,
};
pub use report::{
    HistogramRow, LeafHistogram, MassBasis, MassBudget, ReportCache, leaf_histogram, report_key,
};
pub use resolve::{
    AboutReference, Diagnosis, FlipSet, GroupCutters, HitTestError, MeshPatchKey, NodeVerdictDelta,
    PredicateDivergence, ReadEdit, RecipeEditRef, Resolution, ResolutionFailure, ResolveError,
    ResolveIndeterminate, ResolveRefusal, Resolved, RunCtx, RunStatus, TieWitness, Tombstone,
    UnnamedEntity, UpstreamCause, VerdictFlip, appearance_rebind_suggestions, apply_with_names,
    body_name, derivation_nodes, derivation_reads, diff_verdicts, edge_name,
    enrich_appearance_loss, enrich_appearance_loss_with_prior, entity_name, face_name,
    rebind_suggestions, resolve, resolve_with_prior, stranded_at, vertex_name,
};
pub use resolve::{
    NodeVerdicts, SummaryDelta, SummaryDivergence, SummaryFlip, SummaryFlipSet, VerdictRow,
    VerdictSummary, VerdictVector, VerdictVectorKey, diff_summaries, verdict_summary,
};
pub use step_handle::{
    ArcShape, AuthoredStep, StepHandleRefusal, StepShape, TargetShape, keep_grid,
};
pub use take_over::{Moved, TakeOverOutcome, Taken, TakenOver, take_over};
pub use trace::{
    TraceMark, TraceMarks, TraceRefusal, TraceStepFault, TracedFace, TracedFrameFault, Traces,
    trace_marks, traced_frame,
};
pub use var::{
    FreshEntry, Index, Pick, Select, SelectionCount, SelectionFault, Var, VarDecl, VarDef, VarId,
    VarKind, VarRef, WrittenDef,
};
// GUI-1: the hit-test service (G1 `ray → stable ref`), with the ray
// vocabulary re-exported from `bvh` so a layer-3 consumer needs no
// direct bvh dependency.
pub use bvh::Ray;
pub use resolve::{
    MeshPick, MeshPickError, NameLookupError, NodePick, NodePickError, PickHit, PickMemo,
    PickTarget, pick_face,
};
pub use stackup::{
    BindingViolation, Chamber, ChamberSpan, DivergedAt, LiftRefusal, PerParam, RecordReading, Rss,
    Sensitivity, SensitivityOutcome, SensitivityRefusal, Stackup, StackupRefusal, Unavailable,
    WorstCase, render_sensitivity, sensitivities, stackup,
};
pub use update::{PinMultiplicity, PinSites, UpdateError, mixed_pins, update_references};
pub use witness::{
    BifurcationKind, BranchCertification, BranchMarginEvidence, Implicated, WitnessAge,
    WitnessBifurcation, WitnessDatum,
};
