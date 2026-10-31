//! **The document layer's curated surface**.
//!
//! The façade re-exports the KERNEL crates wholesale (`pncad::sweep`,
//! …; `pncad::topo` whole but for its construction tier): they are geometry, and a consumer that reaches
//! past the prelude into them finds nothing that outlives the value it
//! is holding. The document layer is different — its arena keys
//! (`EntityRef`, `EntityKey`, and the `topo` keys they wrap) are
//! body-lineage-scoped, meaningful only against the evaluation that
//! minted them, and `editor-core`'s own rule is that they never
//! leave that crate. A whole-crate re-export of `editor_core` would
//! hand them out anyway, one hop past the seal.
//!
//! **So the document layer is exposed through THIS list and nothing
//! else.** What is here is what the façade chose to expose; what is
//! absent is absent on purpose. In particular there is no
//! `EntityRef`, no `EntityKey`, and no `Entry` — the way to a named
//! entity's geometry is [`crate::select`]'s doors (`face_frame`,
//! `edge_frame`, `vertex_position`, `denotation`), which speak names
//! and answer with values.
//!
//! The naming vocabulary proper (selectors, `StableName`, the
//! materializers, the geometry doors) lives in [`crate::select`],
//! beside the worked examples; most of the authoring types here are
//! also in [`crate::prelude`], which draws from this list so there is
//! ONE curated surface rather than two that can drift.
//!
//! **Which list a PAYLOAD owes.** A payload rides with its carrier so
//! a consumer can match the variant AND name what it caught: that is
//! the rule the `NodeRefusal` stanza below states, and it places
//! every payload whose only home is the refusal holding it. It does
//! NOT place a payload that belongs to a vocabulary one of the
//! curated lists already owns — the naming roles are
//! [`crate::select`]'s, the profile layer's are [`crate::profile`]'s,
//! and the parameter and measurement vocabularies this module's own
//! doors author are THIS list's, where [`crate::analysis`] carries
//! their derived readers — because the lists are ONE surface cut into
//! rooms rather than self-sufficient exports. So such a payload is
//! spelled ONCE, on the list that owns its vocabulary, and the
//! carrier's list points at it. The cost is stated rather than
//! hidden: a consumer importing this module alone matches the arm and
//! needs a second `use` to name what it holds — one import, never a
//! second crate. The cost the other way is the same name on two
//! lists, which is two things that can drift.

// The recipe and its edits. `Applied` is `apply`'s return (the new
// document plus its `EditRecord`) — re-exported so a caller can STORE
// one in a typed field rather than only destructure it.
// `ProgramRefusal` rides with `EditError` by the `NodeRefusal` rule below:
// it is `EditError::ProfileProgramRefused`'s payload, and nothing else
// this crate carries answers in it, so without it a consumer can match
// the variant and read its refusal only out of prose. `AttrKind` rides
// for the same reason at the appearance arms — it is what
// `EditError::{RebindAppearanceCollision, AppearanceNotSet}` name, and
// which of the three display attributes collided is the whole of what
// those two refusals say beyond the name. Carrying the KIND is not
// carrying the appearance map: `Attr`, `AttrSet` and the record types
// stay out, because nothing a consumer of this module holds answers in
// them. `MetaVersionError` rides for the same rule at the metadata
// arm: it is the typed shape refusal `EditError::MetaUnversioned`
// holds, and which of the three ways a stored value breaks the D7
// producer convention — not a map, no `"v"` entry, a `"v"` that is not
// an integer — is the whole of what that arm says beyond the name and
// the key. Carrying the refusal is not carrying the value tree:
// `MetaValue` and `MetaError` stay out, because the arm names neither.
// `CarryForwardDoor` rides with `EditError` by that same rule at the
// unknown-variable arm: it is what `EditError::UnknownVar` carries
// beside the address, and which door was refused is the whole of what
// that arm says beyond the variable.
// `Maintenance` rides with `Applied` by the same rule: it is what
// `Applied::maintenance` answers in — the offset the mate door cleared
// (A11 (2)) and the references a delete stranded (DM7) — and a
// consumer that can hold an `Applied` in a typed field must be able to
// hold what it carries. `MaintenanceNet` rides with it: a consumer
// that applies several edits as one action (a cascade delete) folds
// their rows into what is true of the document the action ends at, and
// that rule has one spelling. `Recording` and `Recorded` are that
// action whole — the edits applied in order, their rows netted, the
// ids they minted — so a consumer composing one does not re-write the
// loop.
// `StepId` is what `DocEdit::SetProgram` keeps a step by — a caller
// who cannot spell it cannot author the edit — and `StepIdFault` is
// what `EditError::StepIdsRefused` carries, so a consumer matching that
// arm can name what it caught; `LoftFault` is the same for
// `EditError::LoftShape`.
// `AuthoredStep` is how an author who recorded a step reaches its id
// and its pieces (`ProfileProgram::step`, `ProfileProgram::piece`) and
// keeps it across a reshaping (`keep_grid`); `StepHandleRefusal` is
// what those doors refuse with, and the shape types are what an
// `AuthoredStep` is made of.
pub use editor_core::{
    Applied, AttrKind, CarryForwardDoor, Doc, DocEdit, EditError, EditRecord, LoftFault,
    Maintenance, MaintenanceNet, MetaVersionError, ProductReadFault, ProgramRefusal, Recorded,
    Recording, SlotValue, StepId, StepIdFault, Took, apply, apply_replayed,
};
pub use editor_core::{
    ArcShape, AuthoredStep, StepHandleRefusal, StepShape, TargetShape, keep_grid,
};
// The delete door's companion query: which nodes a delete of one node
// must take with it, in an order the door accepts. A GUI both states
// the cost of the button and builds the sequence behind it from this.
pub use editor_core::cascade_delete_order;
// "Sketch on this face" (DM1): the profile, its extrude, the copy placed
// on the face and the union, as one action (`Recording::sketch_on_face`
// composes it into a longer one).
pub use editor_core::{SketchOutcome, SketchedOnFace, sketch_on_face};

// Node vocabulary. A union or intersect takes one `Bodies` argument (a
// family read, or spelled reads); a subtract is binary. An operand
// field is an
// `Operand` read, written at an `OperandSlot` (a `SlotId::Operand`)
// that admits a `SlotKind`; `DocEdit::SetParam` writes a `SlotValue`.
// A placement copies one source; a read of one member of a `⊔` (an
// instance's copy of a part's product, say) is a pick of it. A
// placement's constraints are addressed by their `ConstraintId`.
// Repetition is an index (D10 `[repetition]`): `k = index(N)` is a
// `VarDecl::Index`, a family's member is named under `Member { at }`.
pub use editor_core::{
    Axis3, Bodies, BodyRead, ConstraintId, InputFault, ListFault, LoftSection, MintId, Node,
    Operand, OperandSlot, OutputPort, PortKind, RecipeNodeId, SlotId, SlotKind, StationMove,
    TubeWindow, VectorSlot,
};
pub use editor_core::{Index, IndexFault, MemberAt};

// How a sentence names a node: the kind noun and tag a person reads, the
// speaker a refusal holding bare ids is said by, the nodes a door kept
// beside a refusal it carries whole, and the full-width id a machine
// channel prints.
pub use editor_core::{
    FullId, HeldNodes, NameTables, Said, Say, Speaker, SpokenName, SpokenNode, SpokenSpace,
    SpokenVar, held_by, node_kind_noun, spoken_by, spoken_within,
};

// A node's label (DESIGN.md Band 1, "Node labels"): document data the
// kernel stores and speaks, never identity.
pub use editor_core::{Label, LabelFault};

// The measurement vocabulary (ERROR-DESIGN E3/E10, CONTACT-DESIGN C5).
// A `Node::Measure` holds one `MeasurePrimitive`; the builder (`measure`,
// `Recording::measure`) records one measure per primitive and answers
// their outputs as `Measured` (in a `MeasureOutcome`). A caller who
// cannot spell them cannot author one at all; `AssertionRelation` is a field
// of `Node::Assertion` for the same reason. `Observed` and
// `ObservedRefusal` are what `Evaluation::reading` answers a measured
// value with. `AssertionVerdict` and `UnevaluatedReason` are the READING
// half — the payload an evaluated assertion carries — and E10's whole
// point is that a verdict is consumed by reports. `ASSERT_BOUND` is
// the funnel site name, carried like `SEL_PLANE_DISTANCE` so a
// K-census consumer can name the row rather than spell the string.
// `MeasureUnavailableAt` and `MinSeparationRefusal` (with `Which`, the
// side its arms name a face by) are carried for the
// reason a payload's payload always is: they are what
// `UnevaluatedReason::MeasureUnavailable` and
// `NodeErrorKind::MeasureClearanceRefused` CARRY, so a consumer who can
// name the outer type and not the inner one can see that there is a
// reason and never read it.
// `SitedFace` is a mate's head — a `SitedRef` whose name is a
// `FaceName`, so a mate whose head names an edge does not compile —
// and `FaceName`/`NotAFaceName` are the type that makes that true and
// the refusal its one constructor answers with. A caller authoring a
// mate needs all three: the constructor is the door, and its refusal
// is what a caller who read a name out of a file has to handle.
pub use editor_core::clearance::{MinSeparationRefusal, Which};
pub use editor_core::{
    ASSERT_BOUND, AssertionRelation, AssertionVerdict, FaceName, MeasureOutcome, MeasurePrimitive,
    MeasureUnavailableAt, MeasureVerb, Measured, NotAFaceName, Observed, ObservedRefusal,
    SitedFace, SitedRef, UnevaluatedReason, measure,
};

// Expressions and their text door.
// `Formula` is what a caller writes (VARIABLES-DESIGN VR6) and `Expr`
// what a document stores; the edit door lowers the one to the other, so
// a node an edit carries is an `AuthoredNode`. `LowerFault` is the
// lowering's refusal, for a caller that lowers a formula itself
// (`Doc::lowered`): a `NameFault` (`Unlowered` says why) or a
// `FreshFault`, a read of an edit's fresh table outside that edit; `Slot` is the bound a
// reader generic over the two node forms states, and `ExprTree` over
// `LeafSet` (`StoredLeaf`, `AuthoredLeaf`) the tree both forms share.
// `VarEnv` joins them because `select_where` takes one, so a
// caller who cannot spell the type cannot call the door.
// `DimensionError` is the refusal `Formula`'s constructor doors return
// (`literal`, the operator builders) — re-exported so a caller can
// MATCH on it rather than pre-check the conditions it refuses.
// `Ratio` is the exact constant a formula holds (VR5) and `Quantity` a
// written value (VR6), which `Formula::as_ratio` / `as_quantity` hand
// back, so a caller reading a formula can name what it holds.
// `unparse` is `parse_formula`'s inverse, the text door OUTWARD: the
// source text an expression reads back from, which is what a panel
// showing a stored expression needs and cannot otherwise derive.
// `ExprPath` is here by the payload rule: it is the ADDRESS
// `DocEdit::SetExpression` takes, so without it a consumer cannot spell
// which expression the edit replaces.
pub use editor_core::{
    AuthoredLeaf, AuthoredNode, Dimension, DimensionError, Expr, ExprPath, ExprTree, Formula,
    FreshFault, LeafSet, LowerFault, NameFault, ParseError, Quantity, Ratio, Slot, StoredLeaf,
    Unlowered, VarEnv, parse_formula, unparse,
};

// The expression READ side: an expression's current value under a
// document's parameter environment (`Doc::var_env`). A panel that
// shows a slot before editing it needs this — `Formula::literal_value`
// answers only for a lone written quantity, and a slot driven by
// `width/2 - margin` has a value the consumer otherwise cannot obtain
// without re-implementing the evaluator. `EvalError` rides along so a
// slot whose value cannot be computed says which parameter is missing
// rather than displaying a blank.
//
// Reached through the `expr` module path rather than through
// `editor_core::eval`, which names BOTH the evaluation module and this
// function: a bare `pub use editor_core::eval` would re-export the
// module too, opening a second door onto the layer this list exists to
// curate. `eval_var` / `eval_var_count` read one variable — what a slot
// holds — at the dimension its slot reads it at.
pub use editor_core::expr::{EvalError, eval, eval_count, eval_var, eval_var_count};

// Document variables (VARIABLES-DESIGN VR1–VR3).
// `VarId` is a variable's minted identity and `VarName` the unique name
// held beside it — a string newtype admissible by construction (one
// identifier an expression reads back), whose fallible constructor
// answers `VarNameFault`. `Var` is the variable a document holds, of a
// `VarKind` fixed at minting and defined by a `VarDef` (free, defined
// by an `Expr` over other variables, or an output of an operation,
// which `Doc::output` reads); `VarDecl` is the definition as
// an edit carries it, read by name before the door lowers it; `FreeVar`
// is a free definition's dimension plus exact stored value. `VarRef` is how
// an edit addresses a variable, by id or by name. Recipe vocabulary,
// plain values, no arena key anywhere: they complete doors this module
// already carries — `DocEdit::DeclareVar` takes a name and a
// `VarDecl`, the variable edits take a `VarRef`, and `Formula::named`
// takes a `VarName` — so without them the parametric flagship
// (`plate_param`, guide §3.2) could not be authored façade-only.
// `FreshEntry` is an entry of an edit's fresh table: a variable the
// edit mints for its formulas, under a name when two readers share it
// (an unnamed variable has one reader, VR2).
// `SpokenVar` is a variable as a refusal speaks it, and `SpokenSpace` a
// space.
// `FreeValue` is the value half of a free variable, and the reason it is
// curated is the door it opens: `DocEdit::SetVarValue` writes a new
// number into a standing variable and carries the whole definition —
// kind AND distribution — forward. Rebuilding a `FreeVar` from
// `(dim, value)` for `DocEdit::DefineVar` is the natural spelling and it
// silently DELETES an annotation, because a definition is replaced
// whole; a façade that curated only the deleting door would be handing
// every caller that trap.
// `UnitSym` is the display-unit CODE a `FreeVar::Continuous` carries
// beside its dimension — the notation the variable was authored in.
// It rides here for `Distribution`'s reason: the field is `pub`, so a
// façade that could not spell its TYPE could not build the struct at
// all, and `UnitSym::canonical_for` is how a caller authoring in
// metres says so.
// `DisplayUnitRefusal` is what `FreeVar::with_display_unit` answers
// when a notation cannot be written — the same `NodeRefusal` rule: it is
// that door's `Err`, and a consumer calling the door on a `FreeVar`
// it holds could otherwise read the reason only out of prose.
// `DistributionRefusal` is the same thing at the third field, for
// `FreeVar::with_distribution`.
// `DEFINITION_NODE_BOUND` is the expansion bound
// `EditError::DefinitionTooLarge` refuses past, so a caller holding that
// refusal's count can read what it was measured against.
pub use editor_core::{
    DEFINITION_NODE_BOUND, DisplayUnitRefusal, DistributionRefusal, FreeValue, FreeVar, FreshEntry,
    Pick, Select, SelectionCount, SelectionFault, UnitSym, Var, VarDecl, VarDef, VarId, VarKind,
    VarName, VarNameFault, VarNameReason, VarRef, WrittenDef,
};

// A parameter's optional uncertainty (ERROR-DESIGN E1/E2), and the
// typed refusals its invariants raise at the edit and persistence
// doors. It rides on `FreeVar::Continuous`, so a façade that can
// author a parameter but not annotate one could not express an
// error-analysis document at all; `DistributionFault` is what
// `EditError::InvalidDistribution` and `PersistError::Distribution`
// carry, so a caller diagnosing a refusal needs it too. Reading a
// distribution back is `analysis`'s door, not this one.
pub use editor_core::{Distribution, DistributionFault, DistributionField};

// WHICH float of a continuous parameter a non-finite refusal is about
// — the nominal or one of the annotation's offsets. Both doors' typed
// refusals carry it (`EditError::NonFiniteVar`,
// `NonFiniteSite::DocParam`), so a caller matching either needs to be
// able to name it.
pub use editor_core::DocParamField;

// Evaluation: the service, its options, its results, and the payloads
// a result can carry. `NodeResult`/`NodeValue`/`EvalOutcome` complete
// the result vocabulary: `Evaluation::result` and
// `Evaluation::node_error` answer in these types, so failed and
// poisoned nodes are typed data, not a collapsed `None`. The
// detect/declare protocol leans on the same door: the boolean's
// undeclared-coincidence
// REFUSAL is the detect/declare protocol's trigger, and
// `NodeError`/`NodeErrorKind` were unreachable without the result
// enum that carries them.
// `PoseValue`'s direction fields are `geom_core::UnitVec3`, reached
// through the re-exported `geom_core` crate rather than curated here:
// a consumer cannot read a pose's normal, or build a pose at all,
// without naming the type that makes it unit, and it names it at the
// crate that mints it — the constructor's refusal (`UnitVec3Error`,
// there too) is the only way a pose direction is rejected.
// `NodeRefusal` rides with `NodeErrorKind` for the same reason: it is
// what `MateFault::PlacerRefused` and `EditError::PlacementAxis` carry
// an evaluation refusal in, so a consumer can match either variant but
// not read the cause out of it without naming the wrapper (the
// prelude's `BlendKind` rule — the discriminant crosses with the
// refusal).
// `CarriedChain`, `CarriedLevel` and `CarriedIn` ride with it: they are
// `NodeErrorKind::carried_chain`'s answer, the one reading of which
// refusal a failure carries and which document its node is in.
// `Mispaired` rides with `Evaluation` by the same rule: it is
// `Evaluation::prior_refused`'s payload, so a consumer cannot read why
// a memo was refused without naming it. The name is not the memo's —
// it is the one payload every pairing door carries (DI3; which doors
// those are is `editor-core`'s `ASSEMBLY.md` A2a), which is why it is
// spelled for the QUESTION rather than for any one door.
// `NodeStanding` rides with `Evaluation` by the same rule: it is
// `Evaluation::usable`'s refusal and the payload every door that
// needs a node's value refuses with, so a consumer can match those
// arms but not read which node, or which standing, without naming it.
// `NodeErrorClass` rides with `NodeErrorKind`: it is `NodeErrorKind::class`'s
// answer, the refusal's class a consumer can clone, compare and hash where
// the refusal itself cannot be.
// `StepTurns` rides with `NodeErrorKind`: it is `FullRangeStep`'s
// `turns`, how the copies of a step a turn or more would land.
// `Framed` is a body value: its body in its own coordinates and the
// frame it computed in, which a payload holds wherever it holds a body;
// `EvalCounters` counts what an evaluation built (a copy builds
// nothing); `ComputingTwist` is `EvalOptions::computing_twist`'s type,
// the second computing frame a test runs an evaluation in.
pub use editor_core::{
    BooleanValue, CancelToken, CarriedChain, CarriedIn, CarriedLevel, ComputingTwist,
    DirectionRefusal, EvalCounters, EvalOptions, EvalOutcome, Evaluation, Framed, Mispaired,
    NodeError, NodeErrorClass, NodeErrorKind, NodeRefusal, NodeResult, NodeStanding, NodeValue,
    PoseValue, SplitSide, ValuePayload, evaluate,
};
// What a body value carries from its source (`NodeValue::carry`), and
// the pieces it holds: what a `⊔` grafted, each by its read, which a
// pick reads one of.
pub use editor_core::{Carry, Piece, Pieces};

// Persistence: the doors, verbatim.
// `save`/`load` speak `ProfileDoc` + `DocEdit` — exactly this module's
// vocabulary — and every refusal is a typed `PersistError`, whose
// payload types ride along so each arm is matchable from here (all
// crate-root re-exports in `editor_core`). The format carries no
// schema version (the persist module docs say why), so there is no
// version constant to carry either.
pub use editor_core::{
    Loaded, NonFiniteSite, OutputFault, PersistError, ProgramFault, REGENERATE_RECOURSE,
    SelectionBodyFault, SnapshotError, load, save,
};

// A refusal's two renderings: under its stage word (`Display`), and as
// the sentence a carrier that names the stage renders
// ([`Staged::sentence`]); the recourse label, and what an API door
// given no part resolver says to do.
pub use editor_core::{Labelled, Labels, PASS_A_RESOLVER, Recourse, Staged};

// Document identity and content pins.
// `DocumentId` answers "which part" (authored at construction —
// `DocumentId::derive` for deterministic callers, this crate's
// `workspace::random_document_id` for interactive authoring);
// `ContentPin` answers "which version" (SHA-256 of the canonical
// semantic bytes); `DocRef` pairs them — the value cross-document
// references carry. `canonical_bytes`/`content_pin` are the pin
// doors; `header_document_id` is the workspace scan's cheap header
// read. The store itself lives in [`crate::workspace`].
pub use editor_core::{
    ContentPin, DocRef, DocumentId, canonical_bytes, content_pin, header_document_id,
};

// The content-hashing trait a scalar must satisfy to be evaluated
// through the document layer (the memo currency's substrate).
pub use editor_core::ContentBits;

// The product (D10 `[operations]`): the one body the document's slot
// reads, set by `DocEdit::SetProduct`. `product` is its gather: a `⊔`'s
// operands, each one arrival of the census at its door.
// `AssembleGather` is a `⊔` the slot does not read, gathered by itself
// for the at-rest check, and `assemble_gathers` gathers every one.
// `AtRestRow` is one of the product's at-rest decisions, beside the two
// copies whose pair the census decided.
pub use editor_core::{
    AssembleGather, AtRestRow, Product, ProductError, ProductErrorKind, ProductRefusal, Refusal,
    SourceFinding, assemble_gathers, product, product_recorded,
};

// The gather's own witness, and only where `debug_assertions` are on:
// how many times this thread has gathered a product. A consumer that
// owes one gather per operation asserts it on the DIFFERENCE across
// that operation; a release build carries no counter at all.
#[cfg(debug_assertions)]
pub use editor_core::gathers_on_this_thread;

// Instantiated parts. `Frame` is the rigid motion a placement solves
// to;
// `PartResolver` is the document seam evaluation crosses to reach a
// referenced document, `ResolveFailure`/`ResolveFault` its classified
// refusal, and `PartFault` the evaluation-side cause an
// `InstantiatePart` node reports. `product_named` is the gather that
// carries the product's stable names — what an instance's own names
// are minted from.
pub use editor_core::{
    AxisRefusal, Frame, FrameFault, PartFault, PartResolver, ResolveFailure, ResolveFault,
    product_named,
};

// Mates: a placement's constraints (`Constraint`, addressed by a
// `MateRef`), each a `Mate` of two poses of one kind — `on` read off
// the copy's geometry, `to` a pose of the space the placement places
// into, with the `Orientation` a mate fixing a direction carries — or
// a `Value` setting one freedom (`Coord`: a slide along one of the
// target space's axes as `TargetAxis` names it, the spin about the
// target's own direction, or a spin about an axis of a chain), which names no
// mate: the bundle's mates together leave it, and it charts on the
// target space's axes carried onto the one mated direction by the least
// turn from where the orientation puts the copy's. The solve's outcome (`SolvedPoses`, each copy's target
// space and motion; a pose kind's symmetry `Subgroup` and its
// `SubgroupFamily`, whose meeting decides a bundle), and `MateFault` —
// the typed refusal every door carries. A placement's bundle is a
// `Placed` — its mates and its values — built by the one pure
// constructor `Placed::of` (each freedom the mates leave and no value
// sets written the constant `0`), and `Placed::bundle` writes it
// whole; `stand` is the identity bundle. `unobservable` reports each
// number an evaluation shows moves nothing (`Unobservable`): a pose
// move inside its pose's own symmetry, a spin value about a line its
// copy is round about. `member_of` is A11's member vocabulary itself
// (`Member` is its answer); `member_reading` is the same walk with the
// name it reached at the member's instance, and `head_face` the face
// of the member's part a head names. `OVERCONSTRAINED_RECOURSE` and
// `UNPINNED_RECOURSE` are the recourse sentences a bundle's refusals
// end on. `in_space` is the one door that composes a body's frame into
// its space's coordinates (`Composing` is what it takes), for export
// and the viewer's picture alone.
/// Why a mate could not form its lever, which
/// [`MateFault::Unleverable`] carries — by the payload rule this list
/// states at `NodeRefusal`.
///
/// A parallelism verdict is levered over the mated copies' own
/// extent: a part's reach that is not in hand is carried in the
/// reach's own words ([`ReachRefusal`]) beside the instance and the
/// part it is about — the part does not resolve (the resolver's own
/// fault, unaltered), a face of its body cannot be bounded (which
/// face, and its `prelude::SurfaceKind`), the body is malformed, has
/// no faces, or reads back non-finite — or the member stands on no
/// instance, or the lever the two parts form is out of the format's
/// range. A consumer that could match `Unleverable` and not name this
/// type would read all of that out of the message prose.
///
/// Its only home is the refusal holding it: nothing else on the
/// curated lists answers in a `LeverRefusal`, so it rides its carrier
/// here rather than being spelled on a list that owns its vocabulary.
pub use editor_core::LeverRefusal;
pub use editor_core::{
    Composing, Constraint, ConstraintKind, Coord, FrameMove, Mate, MateFault, MateRef, MateSide,
    Member, NoChartWhy, OVERCONSTRAINED_RECOURSE, Orientation, PinRefusal, Placed, Placing,
    PoseRefusal, PoseSymmetry, ReachRefusal, SignedAxis, SlideAxes, SolvedPoses, Subgroup,
    SubgroupFamily, TargetAxis, UNPINNED_RECOURSE, Unobservable, Value, corner_frame,
    corner_frame_in, head_face, head_of, in_space, member_of, member_reading, moved_frame,
    relative_freedom_components, solve_document, stand, unobservable,
};

// **What the at-rest check reports** (A5): the census each `⊔` runs
// over its arrivals is read by the checks registry's `AtRest` resident
// (`run_checks`, `CheckId::AtRest`), and nothing at rest refuses; a
// caller that wants a gate runs `enforce_checks` at `Error`. An overlap
// between two copies' material is an `InterferenceFinding`: its copies
// (`CopyRef`), its `Overlap` (the bounding faces as `FaceSite`s, or
// `Unlocalized` with why), the assertion that quiets it, if any, and
// whether one could (`quietable`): one no assertion could quiet refuses
// at no severity. A pair the census could not look at is an
// `AtRestEvidence`, loud and never refusing. A part's own findings cross the seam with it, each
// reported once, as the part's: `CarriedInterference`,
// `CarriedAtRestFinding` and `CarriedCoincidence`, each with the `Route`
// it arrived by (one type, on every carrier of that fact);
// `CarriedFindings` is what an instantiated value carries up, its
// `CarriedCoincidenceRows` the rows a part's own door judges, each over
// the part's `PinnedDoc`.
pub use editor_core::{
    AtRestEvidence, CarriedAtRestFinding, CarriedCoincidence, CarriedCoincidenceRows,
    CarriedFindings, CarriedInterference, CopyRef, FaceSite, InterferenceFinding, Overlap,
    PinnedDoc, Route, Unlocalized,
};

// Split, inline and flatten: the first-class
// recorded refactorings. `split` cuts a closed node set out into a new
// document (identity supplied by the caller — `DocumentId::derive` or
// `workspace::random_document_id`) and leaves an instance behind;
// `inline` splices a referenced document back in through a
// `PartResolver`, into a new space placed under the instance's bundle;
// `flatten` folds an identity placement out of such a space back into
// its target. Both return the new document VALUES plus the
// ordinary recorded edits producing them; persistence of the results
// is the workspace write side (`workspace::Workspace::create` /
// `resave`). `InterfaceRecord`/`InterfaceCrossing` are the split
// seam's crossing-declaration record, inhabited by
// `InterfaceCrossing::Mate`.
pub use editor_core::{
    FlattenError, FlattenOutcome, InlineError, InlineOutcome, InterfaceCrossing, InterfaceRecord,
    NodeMap, SplitError, SplitOutcome, StepMap, Uncarried, flatten, inline, split,
};

// The pin-update door. `DocEdit`'s
// `UpdateReference` arm is the per-reference primitive;
// `update_references` is the whole-document ELABORATION over it,
// returning the ordinary edits and applying none of them (purity =
// atomicity), and `UpdateError` is its typed refusal. `mixed_pins`
// is the multiplicity LINT — a report, never a gate:
// one entry per referenced id carrying more than one pin
// (`PinMultiplicity`), each pin listed with the nodes holding it
// (`PinSites`). The store-facing convenience that computes the new
// pin from disk is `workspace::update_to_store`.
pub use editor_core::{PinMultiplicity, PinSites, UpdateError, mixed_pins, update_references};

// The advisory-check registry (DISCIPLINES-DESIGN DS6) and its
// residents, the connectedness check among them. `run_checks` REPORTS, never
// gates (the `mixed_pins` posture: nothing calls it from apply, load,
// or evaluation); `enforce_checks` is the one refusing path, and the
// CALLER chooses where to gate on it. Deliberately NOT in the prelude
// (prelude membership is corpus-measured).
// `subject_body` resolves a placement-output finding's (placement, output_ix) attribution
// back to the flagged body and the declarations its producer minted
// for it, in the same evaluation.
// `run_checks_on` is the registry over a `Subject` the caller gathered
// — the door `run_checks` wraps, for a caller that already holds the
// document's product.
// `ChartCoherenceLane` is a BOUND on both registry doors, so it is
// nameable by necessity: a caller writing its own function generic
// over the decision lane has to spell it. The trait says which lanes
// carry a chart-coherence examination — `f64` does, and a lane that
// does not reports that as a finding rather than as a clean body.
// `Unused` is what `Doc::unused` answers: the tips the `unused` check
// reports, read off the recipe with nothing evaluated.
pub use editor_core::{
    Advisory, ChartCoherenceLane, CheckEvidence, CheckFinding, CheckId, CheckKind, CheckRefusal,
    ChecksConfig, ChecksError, ChecksReport, DoorFindings, DoorJob, FindingSubject, Severity,
    Subject, Unused, enforce_checks, run_checks, run_checks_at_rest_on, run_checks_on,
    subject_body,
};
// The coincidence door (D10): the rows an evaluation's nodes decided
// from values, and what the door decides about each — the payload of
// `CheckEvidence::UnprovenCoincidence`. The record's relation and
// decision site are the kernel's own words (`topo::coincidence`), and
// the door is a module so its `Recourse` keeps its name. A cell on a
// profile piece names it by `select`'s `ProfileEdgeRef`, curated there.
pub use editor_core::coincide;
pub use editor_core::{
    CitedInput, NamedCell, NamedCoincidence, Proof, Residual, RowOrigin, Rung, Unreplayed,
};
// The pose definitions (D10): how a pose variable is defined, read off
// geometry or constructed, and the faults the doors and the evaluator
// name. A module, so its arms keep their names (`pose::PoseDef::Plane`).
pub use editor_core::pose;
/// The shell door's typed refusal, which two `CheckEvidence` arms
/// carry — by the payload rule this list states at `NodeRefusal`.
///
/// [`CheckEvidence::Escalated`] and [`CheckEvidence::Unsupported`] are
/// two different findings about the SAME thing: the component count
/// for this subject is unknowable, because a shell's orientation read
/// escalated or because a face of it is outside the flux inventory.
/// Which shell, and which of the five ways the door refused, is
/// `source` — and a consumer that could match the arm and not name its
/// type read that only out of the message prose.
///
/// It is `topo`'s rather than the document layer's, like
/// [`crate::select`]'s `ContactFinding`: the check registry inherits
/// the props lane's refusal unaltered rather than restating it. Its
/// own rungs are already reachable — [`crate::prelude`] carries
/// `BandError` and `Indeterminate`, and the shell key and the mass-
/// properties refusal are one module hop away at `pncad::topo::…`.
pub use topo::ShellClassifyError;
pub use topo::coincidence;

// The profile description node type and its document alias, plus the
// refusal of the door that reads a step's profile edges — matchable
// here because a caller that asked which edges a step became has to be
// able to say WHY it was not told. `CanonicalSegment` is what that door
// answers in: a canonical position, which is what emission iterates
// and the pieces (`crate::select::ProfilePieces`) translate.
//
// `RecordedNotation` rides with them because a recorded path program is
// bare `f64`s and a document literal names its notation (D6): it is what
// a caller writing `25 mm` through the path algebra hands
// `LoopProgram::from_recorded_with_notation` so the document reads back
// what they wrote.
//
// A profile drawn on a frame (`DrawnOn`) reads the traces of faces of
// the frame's space: `ProgramTarget::OnTrace` and
// `ProgramStep::AlongTrace` say which crossing and which way
// (`TraceCrossing`, `TracePassage`, `TraceSense`), and a resolution
// that refuses says why (`ProgramResolveFault`, `TraceRefusal`,
// `TraceStepFault`).
pub use editor_core::{
    CanonicalSegment, DrawnOn, LoopProgram, ProfileDoc, ProfileProgram, ProgramArcData,
    ProgramResolveFault, ProgramStep, ProgramTarget, RecordedNotation, RecordedProgramError,
    StepArg, StepSegmentsError, TraceCrossing, TracePassage, TraceRefusal, TraceSense,
    TraceStepFault, WrittenLoopFault, resolve_loops, resolve_written_loops,
};
