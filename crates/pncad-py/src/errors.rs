//! The binding error taxonomy.
//!
//! Failures reach Python as **typed exceptions carrying the
//! structured error, never strings**. That splits in two:
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
//!   becomes.
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
/// is `py::value::dimension_name`, capitalized for the Python
/// `Measurement` repr.
pub const fn dimension_tag(dim: Dimension) -> &'static str {
    match dim {
        Dimension::Length => "length",
        Dimension::Angle => "angle",
        Dimension::Count => "count",
        Dimension::Scalar => "scalar",
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
    Evaluation,
    /// A body that failed a topological or geometric validator.
    Validation,
    /// An operator applied to two quantities whose dimensions do not
    /// admit it ([`QuantityOpMismatch`]). The Python class is
    /// `DimensionError`.
    Dimension,
    /// A value the expression layer refused (non-finite literal, a
    /// count written as continuous, ...) — the document layer's
    /// `DimensionError`, raised on the LITERAL-CONSTRUCTION door.
    /// The Python class is `LiteralError`.
    ///
    /// That type has genuine dimension-mismatch arms too, and `load`
    /// reaches them (`WireExpr::rebuild` re-runs every check through
    /// the operator builders), but they arrive as
    /// [`ErrorClass::Persist`] with the `parse` tag rather than under
    /// any dimension class — issue #694. Nothing here is routed to
    /// [`ErrorClass::Dimension`], which is the quantity boundary's
    /// own check and a different type.
    Literal,
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
}

impl ErrorClass {
    /// The Python class name this maps to.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Edit => "EditError",
            Self::Evaluation => "EvaluationError",
            Self::Validation => "ValidationError",
            Self::Dimension => "DimensionError",
            Self::Literal => "LiteralError",
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
            Self::Checks => "ChecksError",
            Self::Enforce => "CheckRefusal",
        }
    }
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
