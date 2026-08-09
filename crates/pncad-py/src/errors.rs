//! The binding error taxonomy.
//!
//! LIBRARY-DESIGN §L4: failures reach Python as **typed exceptions
//! carrying the structured error, never strings**. That splits in two:
//!
//! * [`DimensionError`] — the boundary refusal a Python user can
//!   provoke that the Rust surface refuses at COMPILE time
//!   (`Length + Angle` is simply not an `impl` in `quantity`). Python
//!   has no such static gate, so the illegal combination has to become
//!   a runtime value; making it a STRUCTURED value rather than a
//!   formatted string is what keeps §L4's promise.
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

/// The canonical unit a [`Dimension`] is stored in (GQ5: metres and
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

/// A value the boundary refuses before it can reach the kernel.
///
/// `Expr::literal` rejects non-finite values and integer-dimension
/// literals, but its error type is NOT re-exported by the façade (see
/// the FINDING in the PR body), so the binding pre-checks the same two
/// conditions itself and raises its own typed refusal. The kernel call
/// downstream is then infallible by construction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralRefusal {
    /// NaN or an infinity crossed the boundary.
    NonFinite {
        /// The offending value.
        value: f64,
    },
    /// A `Count` was written as a continuous literal; counts are
    /// integers (D4).
    CountIsInteger,
}

impl fmt::Display for LiteralRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { value } => write!(f, "value is not finite: {value}"),
            Self::CountIsInteger => f.write_str("a count literal must be an integer"),
        }
    }
}

impl core::error::Error for LiteralRefusal {}

/// Check a literal against the two conditions `Expr::literal` refuses.
///
/// Returns the value unchanged when it is admissible, so callers can
/// use it as a gate in expression position.
pub fn check_literal(value: f64, dim: Dimension) -> Result<f64, LiteralRefusal> {
    if !value.is_finite() {
        return Err(LiteralRefusal::NonFinite { value });
    }
    if dim == Dimension::Count {
        return Err(LiteralRefusal::CountIsInteger);
    }
    Ok(value)
}

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
    /// A value refused before it reached the kernel.
    Literal,
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
        }
    }
}
