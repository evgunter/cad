//! The binding error taxonomy.
//!
//! Failures reach Python as **typed exceptions carrying the
//! structured error, never strings**. The items here say what that
//! means, and no count of them is kept in this paragraph — what
//! enumerates this file is an instrument rather than a sentence.
//! `tests::ERRORS_MINTING_ITEMS` names every `fn`, `const` and
//! `static` this file declares, and what holds whatever words each
//! puts on a Python wire. **An item is loud where that reader can see
//! it**: the population is the file's declarations, keyed by every
//! scope that holds them — an `impl`, a `mod`, a `trait`, a function
//! body — so a further one reds that roster by name whether it stands
//! at the top level or at depth, shares a line with its scope or not,
//! carries a trait or not, and whether or not it spells a literal at
//! all; and a word added to a rostered item moves that row's count. A
//! word MINTED here is loud, and one minted here with nothing holding
//! it is the finding that roster asks its author for.
//!
//! **What that reader cannot see, it says itself.**
//! `tests::SCOPE_WALK_BLIND_SPOTS` is the list, entry by entry, each
//! naming the test that executes it — a scope a macro expands to, a
//! `mod` whose body is another file, an `impl` whose generic argument
//! holds a brace, a block. An exclusivity claim written about this
//! reader has been short every time one has been written, so what
//! stands here is a pointer to a list something re-derives and not a
//! fence.
//!
//! **What it does not reach is a word this file carries without
//! declaring.** [`QuantityOpMismatch::op`] is a `&'static str` FIELD,
//! and its twelve words are minted at call sites under `crate::py`
//! (measured 2026-09-15) where no instrument reads them; a second such
//! field arrives with that roster silent, and
//! `tests::the_errors_mint_census_cannot_see_a_word_channel_that_is_not_a_declaration`
//! executes it. This paragraph claims no completeness beyond that —
//! an exclusivity claim written here has been short every time it has
//! been written — so what it offers is the measured case and not a
//! fence.
//!
//! The items:
//!
//! * [`QuantityOpMismatch`] — the boundary refusal a Python user can
//!   provoke that the Rust surface refuses at COMPILE time
//!   (`Length + Angle` is simply not an `impl` in `quantity`). Python
//!   has no such static gate, so the illegal combination has to become
//!   a runtime value; making it a STRUCTURED value rather than a
//!   formatted string is what keeps that promise. It is NOT the
//!   document layer's `DimensionError`, which is the expression
//!   layer's ten-arm refusal; the two are unrelated types and this
//!   one is deliberately not named after it.
//! * [`ErrorClass`] — which typed Python exception a kernel refusal
//!   becomes, and for the classes whose discriminant is this crate's
//!   own decision rather than a kernel refusal's tag, WHICH refusal
//!   inside it.
//! * [`EvalReason`] and [`ValidationRefusal`] — those discriminants:
//!   the complete vocabulary of `EvaluationError.reason`, and of
//!   `ValidationError`'s two attributes, each carried by its
//!   [`ErrorClass`] variant so that naming the class means naming the
//!   refusal.
//! * [`BoundaryEdit`], [`UnmirroredSelect`] and [`StlRefusal`] — the
//!   values a raise takes in place of a `&str` where the word is the
//!   BOUNDARY's own: naming one means naming a variant, and
//!   `crate::tags` holds the exhaustive map. They are declared here
//!   rather than beside their maps because that file's recogniser
//!   admits no `enum`.
//! * [`reads_as_prose`] — the predicate every raise is checked
//!   against, so a `Debug` dump never reaches a Python user's screen.
//!
//! The dimension tag is the curated surface's own
//! [`pncad::document::Dimension`], not a private copy, so the Python
//! tag set cannot drift from the document layer's.
//!
//! This module is deliberately free of `pyo3`: it compiles on the
//! default (no-Python) build path, so hosted CI type-checks and tests
//! the taxonomy even though the `#[pyclass]` wrappers are gated out.

use core::fmt;
use pncad::document::Dimension;

/// The lowercase tag a [`Dimension`] is exposed to Python under.
///
/// Total over the D6 closed set: adding a dimension stops this
/// function compiling.
///
/// One of three spellings of this word list. The FFI tag is this
/// crate's to own, but it happens to be word-for-word the kernel's
/// prose rendering (`Dimension`'s `Display`) and
/// `dimension_tags_match_the_kernel_prose` pins the two equal, so a
/// drift is a test failure rather than a quiet divergence. The third
/// is [`measurement_dimension_tag`], capitalized for the Python
/// `Measurement` repr and pinned to this one by
/// `the_two_dimension_alphabets_are_one_list_in_two_cases`.
pub const fn dimension_tag(dim: Dimension) -> &'static str {
    match dim {
        Dimension::Length => "length",
        Dimension::Angle => "angle",
        Dimension::Count => "count",
        Dimension::Scalar => "scalar",
    }
}

/// The **capitalized** spelling of a [`Dimension`], which is what
/// `Measurement.dimension` answers.
///
/// It lives here rather than in [`crate::tags`] for the reader's
/// sake, not for the alphabet's: that file's tag-table reader refuses
/// a value that is not lower snake case — *"every tag in this file
/// is, and a reader that accepted anything would be guessing"* — so
/// moving this map in means weakening the one claim that makes the
/// reader exact, for four words. Capitalised Python-visible
/// vocabulary is not rare and this is not the only list of it:
/// [`ErrorClass::class_name`] mints 35 exception-class names below,
/// `py::value::Verdict`'s `status` answers `Holds`/`Violated`/
/// `Unevaluated`, and every fieldless `#[pyclass]` enum under
/// `src/py/` carries capitalised member names. What is true of this
/// list alone is that its four words are a second spelling of
/// [`dimension_tag`]'s four, which is why it belongs beside them —
/// where a reader sees both spellings at once and
/// `the_two_dimension_alphabets_are_one_list_in_two_cases` holds them
/// to one list: each word here is its lower-case sibling
/// capitalized, over the kernel's own [`Dimension::ALL`] rather than
/// a roster written down twice.
pub const fn measurement_dimension_tag(dim: Dimension) -> &'static str {
    match dim {
        Dimension::Length => "Length",
        Dimension::Angle => "Angle",
        Dimension::Count => "Count",
        Dimension::Scalar => "Scalar",
    }
}

/// The canonical unit a [`Dimension`] is stored in (metres and
/// radians underneath), or `None` for the dimensionless kinds.
pub const fn canonical_unit(dim: Dimension) -> Option<&'static str> {
    match dim {
        Dimension::Length => Some("m"),
        Dimension::Angle => Some("rad"),
        Dimension::Count | Dimension::Scalar => None,
    }
}

/// An operator applied to two quantities whose dimensions do not
/// admit it — `Length + Angle`, at the Python boundary.
///
/// The fields are the payload, not the message: a caller inspects
/// `err.op`, `err.left`, `err.right` rather than parsing prose.
///
/// Raised to Python as the `DimensionError` class. That class name is
/// the SURFACE spelling and this is the Rust type behind it; the
/// document layer's own `DimensionError` is a different type entirely
/// (the expression layer's ten-arm refusal), which is why this one
/// does not share its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantityOpMismatch {
    /// The operator that was attempted, e.g. `"+"`.
    pub op: &'static str,
    /// Dimension of the left operand.
    pub left: Dimension,
    /// Dimension of the right operand.
    pub right: Dimension,
}

impl QuantityOpMismatch {
    /// Construct a mismatch record.
    pub const fn new(op: &'static str, left: Dimension, right: Dimension) -> Self {
        Self { op, left, right }
    }
}

impl fmt::Display for QuantityOpMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot apply `{}` to {} and {}",
            self.op,
            dimension_tag(self.left),
            dimension_tag(self.right)
        )
    }
}

impl core::error::Error for QuantityOpMismatch {}

// The binding carries no literal pre-check of its own: `Expr::literal`'s
// own error type (`pncad::document::DimensionError`) is curated, so the
// binding matches the kernel's refusal instead of predicting it; the tag
// mapping is `crate::tags::expr_dimension_error_tag`.

/// Which typed Python exception a refusal becomes.
///
/// One variant per exception class in the module's hierarchy. The
/// mapping is exhaustive by construction: adding a class means adding
/// a variant, and the `match` in the PyO3 layer stops compiling until
/// it is handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorClass {
    /// An edit the document layer refused (bad node reference, cycle,
    /// slot dimension mismatch, ...).
    Edit,
    /// A node whose evaluation failed with a typed geometry refusal,
    /// or that was poisoned by an upstream failure.
    ///
    /// **The first of the two classes that carry their own
    /// discriminant** ([`Self::Validation`] is the other). Most
    /// variants here answer "which exception", and their payload is
    /// whatever the raise site hands over; this one answers "which
    /// exception AND which reason", because `EvaluationError.reason`
    /// is not a kernel refusal's tag — it is this crate's own
    /// decision about what "the node produced no value" can mean
    /// ([`EvalReason`]). Carrying it here is what makes the word
    /// unspellable at a raise site: a site cannot name this class
    /// without naming a variant of that enum, and
    /// `crate::py::typed_err` mints the word from it rather than
    /// reading one off the field list.
    Evaluation(EvalReason),
    /// A body that failed a topological or geometric validator, or a
    /// measurement the same class refuses under.
    ///
    /// **The second class that carries its own discriminant**, for
    /// the same reason [`Self::Evaluation`] does: neither of this
    /// class's two words is a kernel refusal's tag. `door` names a
    /// PYTHON METHOD — `Body.validate` and its three siblings — and
    /// `reason` names the one refusal a measurement door has, and
    /// both are this crate's own decisions about vocabulary the
    /// kernel never spells. Carrying [`ValidationRefusal`] here is
    /// what takes the CHOICE of word away from the raise site: a site
    /// cannot name this class without naming a refusal, and both
    /// attributes the class writes are minted from one map. What it
    /// does not do is make the attribute names unspellable in a
    /// payload list — `crate::py::typed_err`'s assertion is what
    /// covers that, over [`ValidationRefusal::ATTRIBUTES`] rather
    /// than over the one word this refusal writes.
    Validation(ValidationRefusal),
    /// An operator applied to two quantities whose dimensions do not
    /// admit it ([`QuantityOpMismatch`]). The Python class is
    /// `DimensionError`.
    Dimension,
    /// The display formatter refused a value: it is NaN or ±∞, and a
    /// non-finite quantity has no display form. The Python class
    /// keeps the Rust type's own name, `FmtQuantityError`.
    ///
    /// The quantity boundary's SECOND refusal, and the other one is
    /// not its neighbour by accident: [`Self::Dimension`] is about
    /// the pair of dimensions an operator was handed, this one about
    /// the single number a formatter was handed. `quantity`'s
    /// newtypes are plain value wrappers and refuse no float, so a
    /// non-finite `Length` is constructible in Python exactly as it
    /// is in Rust (`float("inf") * mm`); the fail-loud doors are the
    /// ones values LEAVE through, and rendering one for a human is
    /// one of those.
    FmtQuantity,
    /// A value the expression layer refused (non-finite literal, a
    /// count written as continuous, ...) — the document layer's
    /// `DimensionError`, raised on the LITERAL-CONSTRUCTION door.
    /// The Python class is `LiteralError`.
    ///
    /// That type has genuine dimension-mismatch arms too, and three
    /// other doors reach them. `load` does (`WireExpr::rebuild`
    /// re-runs every check through the operator builders) and they
    /// arrive as [`ErrorClass::Persist`] with the `parse` tag rather
    /// than under any dimension class — issue #694. The expression
    /// TEXT door does too, and they arrive as [`ErrorClass::Parse`]
    /// with `variant == "dimension"` and the mismatch's own tag as
    /// `kind`, which is the one of those two that keeps the inner
    /// refusal branchable. And the MEASUREMENT sublanguage's
    /// arithmetic constructors do, arriving on THIS class with the
    /// mismatch's own tag as `kind` — they are the same kernel type
    /// refusing at the same layer, because that language asks `Expr`'s
    /// own constructors for its dimensions rather than restating the
    /// F1 table. Nothing anywhere is routed to
    /// [`ErrorClass::Dimension`], which is the quantity boundary's
    /// own check and a different type.
    ///
    /// So `value` is the offending number where the refusing door had
    /// one in hand and `None` where it did not: a measurement
    /// constructor refuses over two operands' DIMENSIONS, and there is
    /// no single float to name.
    Literal,
    /// The expression TEXT door refused: `parse_expr` could not read
    /// the source as an expression. The Python class keeps the Rust
    /// type's own name, `ParseError`.
    ///
    /// Its `Dimension` arm forwards the expression layer's
    /// `DimensionError` — the type [`Self::Literal`] also carries —
    /// and it lands here rather than there because what refused is
    /// the PARSE: the byte offset of the token whose reduction failed
    /// is the recourse, and a `LiteralError` carries no position to
    /// put it in. The inner refusal's own tag rides along as `kind`,
    /// so nothing is lost by the routing.
    Parse,
    /// An expression the evaluator refused: a parameter with no
    /// binding, a dimension the environment disagrees with, a count
    /// crossing the continuous door (or the reverse), exact count
    /// arithmetic that overflowed, or a non-finite result. The Python
    /// class keeps the Rust type's own name, `EvalError`.
    ///
    /// Deliberately NOT a numeric-domain class. Division by zero and
    /// out-of-domain trig are not refusals in the expression layer at
    /// all — they follow the kernel's poison-value policy through the
    /// scalar, because the AST has no branches to hide them behind —
    /// and they reach this class only where they make the FINAL value
    /// non-finite, under the `non_finite_result` tag.
    Eval,
    /// A save or load the persistence doors refused.
    Persist,
    /// An export the document-layer door refused.
    Export,
    /// A body the tessellator refused. The Python class keeps the Rust
    /// type's own name: the refusal IS `TessellateError`, crossing.
    Tessellate,
    /// An STL write the writers refused, or a solid name / binary
    /// header they would not admit. The Python class keeps the
    /// writers' own error name, `StlError`, and the two
    /// option-construction refusals ride it under their own tags —
    /// they refuse the same call, because an option struct is a
    /// keyword argument here.
    StlExport,
    /// A STEP text the importer refused, or one that parsed to a
    /// non-solid (the export test oracle's refusal class).
    StepImport,
    /// Geometry the PATHS authoring algebra refused at the call site
    /// (junction checks, `NoCornerForFillet`, the tangent-line close,
    /// ...).
    Path,
    /// A selection query refused (an in-band decided margin, a tied
    /// name whose candidates disagree, a non-datum reference, ...).
    /// The Python class keeps the Rust type's own name: the refusal
    /// IS `SelectRefusal`, crossing.
    Select,
    /// A frame the linear-algebra constructors refused: a direction
    /// that was not DEFINITELY usable (coincident points, a roll
    /// reference along the aim, a zero mirror normal), or a tolerance
    /// yielding no usable band.
    Frame,
    /// A document identity that could not be minted: the OS entropy
    /// source refused. Identity is not defaultable — a document with
    /// a made-up id is a document that collides with another part —
    /// so the refusal surfaces instead.
    Identity,
    /// The workspace store refused: a scan that could not read a
    /// file's header, two files claiming one id, an id the store does
    /// not hold, or — the arm the store exists to make loud — a
    /// reference whose pin is not the pin the document now hashes to.
    ///
    /// Shares its Rust type with [`Self::Identity`] and is
    /// deliberately a different class: minting an identity is not a
    /// store operation, and a caller catching one should not catch
    /// the other.
    Workspace,
    /// A mate the solve refused. The Python class is `MateError`, and
    /// the solve itself is TOTAL — this is raised only by the doors
    /// that must answer with a pose or not at all, never by
    /// `solve_document`, which records the fault per node instead.
    Mate,
    /// The at-rest assembly gate refused. The Python class is
    /// `AssemblyError`, and its arms are NOT interchangeable: a
    /// verdict against the document (`at_rest`) and the declared
    /// direction's frontier (`uncertified`) are different facts, so
    /// `variant` is what a caller must branch on.
    Assembly,
    /// A whole-document gather refused. The Python class is
    /// `ProductError`.
    Product,
    /// A `split` refactoring refused. The Python class is
    /// `SplitError`.
    Split,
    /// An `inline` refactoring refused. The Python class is
    /// `InlineError`.
    Inline,
    /// A whole-document pin update produced no edit list. The Python
    /// class is `UpdateError`.
    Update,
    /// A read-back door refused to say what a name denotes or where
    /// it sits. The Python class is `ReadbackError`.
    ///
    /// Two Rust types flatten onto it, because they refuse the same
    /// CALL: the document layer's `InterrogateError`, which resolves
    /// the name, and the kernel's own `ReadbackError`, which reads
    /// the carrier — the second is an arm of the first, and its arms
    /// arrive under their OWN tags rather than a wrapper tag, so
    /// which invariant broke is what a caller branches on.
    Readback,
    /// A hit test could not answer: a target was offered whose node
    /// this evaluation has no `Ok` value for, or the winning face
    /// inverted to no name at all. The Python class keeps the Rust
    /// type's own name, `HitTestError`.
    ///
    /// A MISS never reaches here. The ray hitting no offered triangle
    /// is the typed `None`, and errors are never flattened into it.
    HitTest,
    /// A pick index could not be built: the node has no `Ok` value, its
    /// value never draws, it has no output body at that index, or
    /// tessellating and indexing that body refused. The Python class is
    /// `NodePickError`.
    ///
    /// A separate class from [`Self::HitTest`] because the two are
    /// different stages of one story: this one means there is nothing
    /// to pick AGAINST, that one means the pick itself could not
    /// answer. The standing ladder is shared, and arrives here under
    /// the hit-test door's own tags rather than a wrapper's.
    NodePick,
    /// The advisory-check registry could not RUN: a root without a
    /// value, a tolerance that forms no band, roots that gather into
    /// no product. The Python class is `ChecksError`.
    ///
    /// Not a finding. A check that ran and disagreed is a value in the
    /// report; this class means nothing was checked, which is the
    /// difference the registry exists to keep visible.
    Checks,
    /// The registry's ONE refusing path (`enforce_checks`) refused: the
    /// report carries findings whose check the caller configured at
    /// `Severity.Error`. The Python class is `CheckRefusal`, the Rust
    /// type's own name crossing.
    ///
    /// A separate class from [`Self::Checks`] because the two are
    /// opposite answers: that one means the checks did not run, this
    /// one means they ran, found something, and the CALLER asked to be
    /// refused on it. `run_checks` never raises this and
    /// `enforce_checks` never raises the other.
    Enforce,
    /// A [`Distribution`](pncad::document::Distribution) that breaks
    /// an E2 invariant, refused at the Python constructor. The Python
    /// class keeps the Rust type's own name,
    /// [`DistributionFault`](pncad::document::DistributionFault).
    ///
    /// The one class in this taxonomy raised by a VALUE constructor
    /// rather than by a door that touches a document. It is not an
    /// invented pre-check: `Distribution::check` is the kernel's own
    /// single statement of the invariants, the one the edit door and
    /// the persistence validator both run, and the binding calls it
    /// rather than restating it. Calling it EARLY is what the value
    /// class buys — a Python caller learns that a sigma is negative
    /// where the sigma is written, not three edits later.
    Distribution,
    /// The analysis lane could not price a mass: the parameter
    /// carries a band, which states limits without a shape (E2). The
    /// Python class keeps the Rust type's own name,
    /// [`MeasureUnavailable`](pncad::analysis::MeasureUnavailable).
    ///
    /// A REFUSAL, not an absence. "I know the limits but not the
    /// shape" is real information and no report may quietly promote
    /// it to uniform, so the door that would have to guess raises
    /// instead, naming the parameter.
    Measure,
    /// A [`Node::Measure`](pncad::document::Node)'s expression reads a
    /// reference the node does not carry, refused at the Python
    /// construction door. The Python class keeps the Rust type's own
    /// name,
    /// [`MeasureNodeFault`](pncad::document::MeasureNodeFault).
    ///
    /// The second class in this taxonomy raised by a VALUE
    /// constructor rather than by a door that touches a document, and
    /// for [`Self::Distribution`]'s reason: `Node::measure` is the
    /// kernel's ONE construction door and it runs the same check the
    /// edit door and the load door's re-check run, so the binding
    /// calls it rather than restating it. What the timing buys is
    /// that an index past the end of the reference list refuses where
    /// it is written, not at the `Doc.apply` after it — where the
    /// same fault arrives as `EditError` with `variant ==
    /// "measure_malformed"`.
    MeasureNode,
    /// A measure whose value is an ENCLOSURE, read at a build whose
    /// scalar is a point (E3/E7, M10-6). The Python class keeps the
    /// Rust type's own name,
    /// [`MeasureUnavailableAt`](pncad::document::MeasureUnavailableAt).
    ///
    /// **Deliberately not [`Self::Measure`], and the two names are
    /// one word apart on purpose.** That one is the ANALYSIS lane
    /// refusing to price a mass whose parameter carries a band; this
    /// one is the MEASUREMENT lane saying a `min_clearance` has no
    /// value at the `f64` scalar the binding evaluates at. Different
    /// kernel types, different questions, and a caller catching one
    /// must not catch the other — so each keeps its own Rust name and
    /// neither subclasses the other.
    ///
    /// A typed ABSENCE rather than a failure: the measure node
    /// evaluates successfully and says what it cannot say, which is
    /// why an assertion over it reports `Unevaluated` carrying this
    /// reason instead of being poisoned. The payload names the DOOR
    /// that could answer, so the recourse is in the refusal.
    MeasureUnavailableAt,
    /// An analysis policy that cannot be honoured: a quantile mass
    /// outside `(0, 1)`. The Python class keeps the Rust type's own
    /// name,
    /// [`AnalysisPolicyError`](pncad::analysis::AnalysisPolicyError).
    ///
    /// Deliberately not [`Self::Measure`]: that one is about a
    /// distribution the document declared, this one about the knob the
    /// REQUEST set, and E2's whole point is that the analyzed box is
    /// the analysis's property rather than the distribution's.
    AnalysisPolicy,
    /// A Monte-Carlo run that produced nothing (ERROR-DESIGN E11.1).
    /// The Python class keeps the Rust type's own name,
    /// [`McRefusal`](pncad::analysis::McRefusal).
    ///
    /// Three arms and one class, because all three say the same thing
    /// to a caller: there is no advisory estimate for this request.
    /// One of them CARRIES a [`Self::Measure`] refusal — the band a
    /// draw cannot be taken from — and it is still raised as this
    /// class rather than as that one, because what refused is the RUN.
    /// The carried fault is not lost: its parameter rides on the
    /// payload and its tag is the word this class's `variant` answers,
    /// so a caller who branches on `band_has_no_measure` reads the
    /// same word from either door.
    Mc,
}

impl ErrorClass {
    /// The Python class name this maps to.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Edit => "EditError",
            Self::Evaluation(_) => "EvaluationError",
            Self::Validation(_) => "ValidationError",
            Self::Dimension => "DimensionError",
            Self::FmtQuantity => "FmtQuantityError",
            Self::Literal => "LiteralError",
            Self::Parse => "ParseError",
            Self::Eval => "EvalError",
            Self::Persist => "PersistError",
            Self::Export => "ExportError",
            Self::Tessellate => "TessellateError",
            Self::StlExport => "StlError",
            Self::StepImport => "StepImportError",
            Self::Path => "PathError",
            Self::Select => "SelectRefusal",
            Self::Frame => "FrameError",
            Self::Identity => "IdentityError",
            Self::Workspace => "WorkspaceError",
            Self::Mate => "MateError",
            Self::Assembly => "AssemblyError",
            Self::Product => "ProductError",
            Self::Split => "SplitError",
            Self::Inline => "InlineError",
            Self::Update => "UpdateError",
            Self::Readback => "ReadbackError",
            Self::HitTest => "HitTestError",
            Self::NodePick => "NodePickError",
            Self::Checks => "ChecksError",
            Self::Enforce => "CheckRefusal",
            Self::Distribution => "DistributionFault",
            Self::Measure => "MeasureUnavailable",
            Self::MeasureNode => "MeasureNodeFault",
            Self::MeasureUnavailableAt => "MeasureUnavailableAt",
            Self::AnalysisPolicy => "AnalysisPolicyError",
            Self::Mc => "McRefusal",
        }
    }
}

/// **The complete vocabulary of `EvaluationError.reason`** — the word
/// a Python caller branches on when a node produced no value.
///
/// **A reason is a TYPE so that its word cannot be minted anywhere
/// else, and the type is on the CLASS so that the wall guards the
/// door rather than one function.** [`ErrorClass::Evaluation`]
/// carries this enum, so no raise of that class can be written
/// without naming a variant of it — not `crate::py::value::eval_err`,
/// not the two direct `crate::py::typed_err` raises beside it, and
/// not a raise in a file that does not exist yet. The word itself is
/// minted in `typed_err` from [`crate::tags::eval_reason_tag`], whose
/// `match` is exhaustive over this enum, so an arm added here stops
/// the build until a word is written into `src/tags.rs`; and that
/// file is what
/// `tests::the_whole_tag_table_matches_its_committed_inventory`
/// reads, so the word reds against the committed inventory when it
/// lands. The alternative — a word spelled at a construction site
/// under `src/py/` — is public Python vocabulary no inventory reads.
///
/// **What the wall does not cover, measured rather than assumed.**
/// A raise site may still pass a field of its own named `reason`.
/// Nothing type-level forbids it, because `typed_err`'s payload is a
/// list of `(&str, Py<PyAny>)` pairs; what happens instead is that
/// the minted word is attached LAST and wins, and a `debug_assert`
/// (live in release in this workspace) names the site. The word a
/// caller reads is therefore always this enum's, and the hand-spelled
/// one is loud rather than silent.
///
/// **Why the type is here and its map is in `crate::tags`.** Not
/// because the taxonomy belongs on this side of the line: a reason
/// and its word are one concept and would sit together in an empty
/// tree. The constraint is the INSTRUMENT. `src/tags.rs` is read as
/// data by the tag-table guard, whose recogniser admits `use` items,
/// `pub fn` tag maps and `pub const` tag words and refuses everything
/// else, so an `enum` declared there stops the guard dead. Teaching
/// the recogniser a form in order to move a declaration is a cost
/// paid against a reader that already needs a guard of its own, and
/// `E0004` over an exhaustive `match` is a real wall between the two
/// halves whichever files they sit in. `crate::node_kind` is the same
/// arrangement for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvalReason {
    /// The document holds no node under that id.
    UnknownNode,
    /// The node evaluated to a value of a kind this door cannot read.
    WrongKind,
    /// A Boolean that succeeded and produced nothing to hand back.
    EmptyBoolean,
    /// The run was canceled before it reached the node, so this
    /// evaluation holds the completed prefix only. The same rung the
    /// read-back and picking doors speak
    /// ([`crate::tags::hit_test_error_tag`],
    /// [`crate::tags::interrogate_error_tag`]), spelled identically on
    /// purpose and pinned against both by
    /// `tests::the_evaluation_door_speaks_the_standing_ladder`.
    NodeNotEvaluated,
    /// The node ITSELF failed; `kind` carries the refusal's own tag.
    NodeFailed,
    /// An ancestor failed, so the node never ran; `through` names the
    /// nearest failed one.
    Poisoned,
}

impl EvalReason {
    /// The Python attribute this class's word is written to, spelled
    /// where the vocabulary is rather than at the mint.
    ///
    /// Every variant writes the same one, which is what makes this a
    /// const rather than [`ValidationRefusal::attribute`]'s map.
    ///
    /// It is the same English word as
    /// [`ValidationRefusal::MassProperties`]'s attribute and nothing
    /// holds the two equal, deliberately: they are attributes of two
    /// different Python classes, and a caller reading
    /// `EvaluationError.reason` learns nothing about
    /// `ValidationError.reason`. Renaming one is not a reason to
    /// rename the other.
    pub const ATTRIBUTE: &'static str = "reason";

    /// Every Python attribute this class mints a word onto — the
    /// one-element case of [`ValidationRefusal::ATTRIBUTES`], derived
    /// from [`Self::ATTRIBUTE`] so the word is spelled once.
    pub const ATTRIBUTES: &'static [&'static str] = &[Self::ATTRIBUTE];
}

/// **The complete vocabulary of the `ValidationError` class**, carried
/// by [`ErrorClass::Validation`] so that naming the class means naming
/// the refusal.
///
/// One enum for two attributes, because one exception class has two
/// shapes. Four variants are validator DOORS and their word is
/// `ValidationError.door` — the Python method that spoke, which is why
/// [`Self::Geometric`] is the word even when the caller called
/// `validate_geometric_measured`: a caller reading `door` learns which
/// gate refused, not which door it called. The fifth is a measurement
/// that had no number to answer with, and its word is
/// `ValidationError.reason`. [`Self::attribute`] is which of the two,
/// and `crate::tags::validation_refusal_tag` is the word; both are
/// exhaustive over this enum.
///
/// **Neither word has a kernel arm behind it**, which is why the enum
/// is the binding's own. `topo::validate` and its siblings answer
/// `Result<(), Vec<topo::ValidationError>>` — the failures are the
/// kernel's and cross as `findings`, projected per finding — and which
/// DOOR was called is a fact only this crate knows. The measurement's
/// `topo::MassPropsError` is a kernel type, but the word is not its
/// tag: it says the measurement refused, not which arm refused, and the
/// arm is in the message prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationRefusal {
    /// `Body.validate` — the whole battery.
    Validate,
    /// `Body.validate_closed` — closure only.
    Closed,
    /// `Body.validate_geometric`, tier 3 — and the gate half of
    /// `Body.validate_geometric_measured`, which is the same gate.
    Geometric,
    /// `Body.validate_pseudomanifold`, tier 3′.
    Pseudomanifold,
    /// A measurement door with no number to answer with — this class's
    /// one refusal that is not a validator's.
    MassProperties,
}

impl ValidationRefusal {
    /// **Every refusal this class can carry**, so that the attributes
    /// it writes are DERIVED from the enum rather than restated.
    ///
    /// Held to the enum by
    /// `tests::validation_refusals_are_the_roster_the_inventory_reads`:
    /// the words this roster maps to through
    /// `crate::tags::validation_refusal_tag` are compared against that
    /// map's row in `TAG_INVENTORY`, which is derived by READING
    /// `src/tags.rs`. So a sixth refusal is an arm in that map — a
    /// compile error until it is written — then a word the reader sees
    /// — a red row until the inventory names it — and then a member
    /// here, a red row until it joins.
    pub const ALL: &'static [Self] = &[
        Self::Validate,
        Self::Closed,
        Self::Geometric,
        Self::Pseudomanifold,
        Self::MassProperties,
    ];

    /// **Every Python attribute this class mints a word onto**, which
    /// is the image of [`Self::attribute`] over [`Self::ALL`].
    ///
    /// It is a SET and not one word because the set is what a raise
    /// site may not spell: `crate::py::typed_err` asserts over this,
    /// so a site passing `reason` beside a refusal that writes `door`
    /// is caught too. Asserting the one attribute the refusal in hand
    /// writes would leave the class's other vocabulary open, which is
    /// the gap this const exists to close.
    ///
    /// `tests::the_validation_class_mints_exactly_these_attributes`
    /// holds it equal to that image in both directions, walking
    /// [`Self::ALL`] rather than a second list.
    pub const ATTRIBUTES: &'static [&'static str] = &["door", "reason"];

    /// Which Python attribute this refusal's word is written to.
    ///
    /// Total over the enum, so a sixth refusal has to say which of the
    /// class's two vocabularies it joins rather than defaulting into
    /// one.
    ///
    /// **Both words are Python-visible**: a raise writes one of them
    /// onto the exception, so a caller reads `ValidationError.door` on
    /// a validator refusal and `ValidationError.reason` on the
    /// measurement one. `pncad.pyi` declares the first and not the
    /// second, which
    /// `tests::the_discriminant_attribute_names_are_declared_in_the_stub`
    /// holds as a gap the stub has not closed. Neither word is a TAG:
    /// `TAG_INVENTORY`'s population is the tag words
    /// `src/tags.rs`'s maps mint, and an attribute NAME is not one, so
    /// the inventory cannot read this map however it grows. The two
    /// tests named above are the pin instead, and the routing — which
    /// refusal writes which — is
    /// `tests::every_validation_refusal_writes_the_attribute_it_is_committed_to`,
    /// because moving a variant between these arms changes a Python
    /// contract and nothing else here would notice.
    #[must_use]
    pub const fn attribute(self) -> &'static str {
        match self {
            Self::Validate | Self::Closed | Self::Geometric | Self::Pseudomanifold => "door",
            Self::MassProperties => "reason",
        }
    }
}

/// The `EditError.variant` of a refusal the BOUNDARY built rather than
/// the document layer.
///
/// Taken by `crate::py::doc`'s boundary raise instead of a
/// `&'static str`, so the three refusals this crate decides for itself
/// are a closed set: a fourth is a variant here, an arm in
/// `crate::tags::boundary_edit_tag`, and a word the tag inventory sees.
///
/// Two of the three carry the KERNEL VALUE whose word they publish
/// rather than a word of their own, and that is the rule
/// `crate::py::doc`'s boundary raise states: where a kernel enum arm
/// stands behind the refusal, the word is that enum's to spell, so the
/// two refusals a caller can reach through either door stay one word.
/// Only [`Self::NameSerialize`] mints, because a `serde_json` failure
/// has no arm anywhere.
#[derive(Debug, Clone, Copy)]
pub enum BoundaryEdit<'a> {
    /// A stable name that would not serialize. The one arm with no
    /// kernel refusal behind it: `StableName` has exactly one
    /// serialization and the document layer never refuses a name for
    /// failing to produce it.
    NameSerialize,
    /// An insert that minted no node id, worded by the declare
    /// sugar's own map — the same refusal reaches Python through
    /// `Doc.declare`, and it is the same word there.
    Declare(&'a pncad::select::DeclareError),
    /// A placement rule spelled through the wrong constructor, worded
    /// by the document layer's own fault map.
    PlacementRule(&'a pncad::document::PlacementRuleFault),
    /// A mate head that is not a FACE. `Node.mate` takes names as
    /// TEXT, so this boundary is one of the three that turn data into
    /// names, and it is the door where `FaceName::new` is called: the
    /// kernel expresses the rule in the TYPE of a head, which a
    /// dynamically-typed caller cannot be held to by the compiler, so
    /// the binding holds it here and answers with the constructor's
    /// own refusal.
    MateHead(&'a pncad::document::NotAFaceName),
    /// A text that is not a parameter name. `ParamName(text)` is the
    /// boundary that turns text into a name, and the document layer's
    /// one rule for one — an identifier an expression reads back — is
    /// held by the constructor there, so the binding answers with the
    /// constructor's own refusal at the call that offered the text.
    ParamName(&'a pncad::document::ParamNameFault),
}

/// Which `#[non_exhaustive]` kernel enum at the SELECTION boundary has
/// grown a variant this binding predates.
///
/// Both arms answer one word, and that is the point of the type rather
/// than an accident of it: a caller reads `SelectRefusal.reason ==
/// "unclassified"` and learns one fact — this binding could not
/// classify the refusal — whichever kernel enum out-grew it. The word
/// used to be spelled twice, once at `crate::tags::select_refusal_tag`'s
/// forced wildcard and once at the contact-class crossing, with nothing
/// holding the two equal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnmirroredSelect {
    /// A `pncad::select::SelectRefusal` arm this binding does not
    /// mirror, reached through the query door.
    Refusal,
    /// A `pncad::select::ContactClass` this binding does not mirror,
    /// reached through a flush finding's `class_`.
    ContactClass,
}

/// Everything that can refuse a `to_stl_*` call, as ONE value.
///
/// Four refusals share the `StlError` exception class because they
/// refuse the same CALL — the writers' own, the two validated option
/// newtypes' (which are this call's keyword arguments), and the
/// boundary's own non-UTF-8 residue. Naming them together is what lets
/// the projection in `crate::py::mesh` be a single exhaustive match
/// instead of four, so an arm added to any of the three kernel enums
/// arrives there as a compile error.
///
/// It lives here rather than beside that projection so that
/// `crate::tags::stl_refusal_tag` — which is what the tag inventory
/// reads — can match on it: this module compiles on the no-Python
/// path, and `crate::py` does not.
#[derive(Debug, Clone, Copy)]
pub enum StlRefusal<'a> {
    /// The writers' refusal: the mesh and the sink.
    Write(&'a pncad::stl::StlError),
    /// The `solid <name>` name the ASCII writer was handed.
    Name(&'a pncad::stl::SolidNameError),
    /// The 80-byte header the binary writer was handed.
    Header(&'a pncad::stl::BinaryHeaderError),
    /// The ASCII writer emitted bytes that are not UTF-8. Not a kernel
    /// arm: the writer emits ASCII by construction, so this is a kernel
    /// defect surfaced rather than lossily replaced, and it is the one
    /// arm whose word this crate mints.
    NotUtf8(&'a std::string::FromUtf8Error),
}

/// Whether a refusal message reads as prose rather than a `Debug`
/// rendering of a kernel value.
///
/// A typed exception's human message is the kernel error's own
/// `Display`; a `Debug` dump in its place is a leak of Rust field
/// names into a Python user's screen and a payload the caller cannot
/// branch on either way. `crate::py::typed_err` — the single
/// construction site — asserts this on every raise, so the rule holds
/// at doors written after it as well as at the ones it was written
/// for.
///
/// Two fingerprints, each of which prose in this crate does not carry:
///
/// * `" { "` — the field-brace `std` puts in every struct and
///   struct-variant rendering, at any nesting depth (`Some(E::V { .. })`
///   and `[V { .. }]` both carry it).
/// * a message that is one bare CamelCase token — a fieldless
///   variant's whole rendering. A sentence is never one word.
///
/// What it cannot see: a tuple variant of scalars (`Escalated(1, 2)`)
/// and a fieldless variant embedded mid-sentence.
///
/// Its false positive is **user text echoed into kernel `Display`
/// prose** — a path, a name, an OS message — which reaches the message
/// verbatim, braces included, and so can carry the struct fingerprint
/// without any `Debug` being involved. `WorkspaceError`'s path arms are
/// the live example. Delimiting the echo makes it legible but does not
/// neutralise it: a caller who names a directory `a { b` turns an
/// honest typed refusal into a panic, and because this workspace keeps
/// `debug_assert` on under release, in a built wheel too. The trade is
/// deliberate — the fingerprint has to be something prose does not
/// carry, and no cheaper discriminator was available — but it is a
/// trade, not a free check.
///
/// One arm disagrees with it on purpose: `crate::py::flush`'s
/// unknown-`ContactClass` refusal renders the unknown variant through
/// `Debug`, having nothing else to render it with. Today that is a
/// fieldless name mid-sentence, which passes. A future STRUCT variant
/// of that kernel enum would trip this assertion and panic where that
/// arm means to refuse gracefully; the site says so too.
#[must_use]
pub fn reads_as_prose(message: &str) -> bool {
    !message.contains(" { ") && !is_bare_camel_token(message)
}

/// Whether the whole string is one identifier-shaped word starting
/// with an uppercase letter — what `{:?}` renders a fieldless variant
/// as, and what no sentence is.
fn is_bare_camel_token(message: &str) -> bool {
    let mut chars = message.chars();
    chars.next().is_some_and(char::is_uppercase)
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
