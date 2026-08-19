//! The binding error taxonomy.
//!
//! Failures reach Python as **typed exceptions carrying the
//! structured error, never strings**. That splits in two:
//!
//! * [`DimensionError`] — the boundary refusal a Python user can
//!   provoke that the Rust surface refuses at COMPILE time
//!   (`Length + Angle` is simply not an `impl` in `quantity`). Python
//!   has no such static gate, so the illegal combination has to become
//!   a runtime value; making it a STRUCTURED value rather than a
//!   formatted string is what keeps that promise.
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

/// A dimension mismatch at the Python boundary.
///
/// The fields are the payload, not the message: a caller inspects
/// `err.op`, `err.left`, `err.right` rather than parsing prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionError {
    /// The operator that was attempted, e.g. `"+"`.
    pub op: &'static str,
    /// Dimension of the left operand.
    pub left: Dimension,
    /// Dimension of the right operand.
    pub right: Dimension,
}

impl DimensionError {
    /// Construct a mismatch record.
    pub const fn new(op: &'static str, left: Dimension, right: Dimension) -> Self {
        Self { op, left, right }
    }
}

impl fmt::Display for DimensionError {
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

impl core::error::Error for DimensionError {}

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
    /// A dimension mismatch at the quantity boundary.
    Dimension,
    /// A value the expression layer refused (non-finite literal, a
    /// count written as continuous, ...).
    Literal,
    /// A save or load the persistence doors refused.
    Persist,
    /// An export the document-layer door refused.
    Export,
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
            Self::StepImport => "StepImportError",
            Self::Path => "PathError",
            Self::Select => "SelectRefusal",
            Self::Frame => "FrameError",
        }
    }
}
