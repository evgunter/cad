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
        }
    }
}
