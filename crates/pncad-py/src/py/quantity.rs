//! The typed quantities: `25 * mm` constructs a `Length`.
//!
//! These mirror `crates/quantity` exactly — canonical metres and
//! radians underneath, and the SAME infallible arithmetic
//! subset: same-dimension add/sub, negation, scalar scaling, scalar
//! division. `quantity` refuses everything else by having no `impl`
//! for it; Python has no compile step to refuse at, so those cases
//! become typed `DimensionError` raises here. That is the whole of
//! the rule that runtime checks live at the Rust boundary, once.

use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, QuantityOpMismatch, dimension_tag};
use crate::py::typed_err;
use pncad::document::Dimension;
use pncad::quantity as q;

/// The dimension of a Python object, for mismatch reporting.
///
/// `None` means "not a quantity and not a number" — the operation is
/// then genuinely undefined rather than dimensionally wrong, so a
/// plain `TypeError` is the honest refusal.
fn dimension_of(obj: &Bound<'_, PyAny>) -> Option<Dimension> {
    if obj.extract::<PyRef<'_, Length>>().is_ok() {
        Some(Dimension::Length)
    } else if obj.extract::<PyRef<'_, Angle>>().is_ok() {
        Some(Dimension::Angle)
    } else if obj.extract::<PyRef<'_, Count>>().is_ok() {
        Some(Dimension::Count)
    } else if obj.extract::<f64>().is_ok() {
        Some(Dimension::Scalar)
    } else {
        None
    }
}

/// Build the typed mismatch raise for `op` between `left` and `other`.
fn mismatch(py: Python<'_>, op: &'static str, left: Dimension, other: &Bound<'_, PyAny>) -> PyErr {
    let Some(right) = dimension_of(other) else {
        return pyo3::exceptions::PyTypeError::new_err(format!(
            "unsupported operand for `{op}` on a {} quantity",
            dimension_tag(left)
        ));
    };
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    typed_err(
        py,
        ErrorClass::Dimension,
        QuantityOpMismatch::new(op, left, right).to_string(),
        &[
            ("op", text(op)),
            ("left", text(dimension_tag(left))),
            ("right", text(dimension_tag(right))),
        ],
    )
}

/// Emit the whole `#[pymethods]` block for a continuous quantity.
///
/// One block per type: PyO3 accepts only one `#[pymethods]` impl per
/// class unless the `multiple-pymethods` feature is on, so the
/// type-specific methods are passed in as `$extra` and spliced in
/// alongside the shared arithmetic rather than living in a second
/// block.
macro_rules! continuous_quantity {
    ($py_ty:ident, $dim:expr, $canonical:ident, { $($extra:tt)* }) => {
        #[pymethods]
        impl $py_ty {
            $($extra)*

            fn __add__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
                match other.extract::<PyRef<'_, Self>>() {
                    Ok(rhs) => Ok(Self(self.0 + rhs.0)),
                    Err(_) => Err(mismatch(py, "+", $dim, other)),
                }
            }

            fn __sub__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
                match other.extract::<PyRef<'_, Self>>() {
                    Ok(rhs) => Ok(Self(self.0 - rhs.0)),
                    Err(_) => Err(mismatch(py, "-", $dim, other)),
                }
            }

            fn __neg__(&self) -> Self {
                Self(-self.0)
            }

            /// Scalar scaling only — `quantity` has no `Mul<Self>`.
            fn __mul__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
                match other.extract::<f64>() {
                    Ok(k) => Ok(Self(self.0 * k)),
                    Err(_) => Err(mismatch(py, "*", $dim, other)),
                }
            }

            fn __rmul__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
                self.__mul__(py, other)
            }

            /// Scalar divisor only — `quantity` has no `Div<Self>`.
            fn __truediv__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Self> {
                match other.extract::<f64>() {
                    Ok(k) => Ok(Self(self.0 / k)),
                    Err(_) => Err(mismatch(py, "/", $dim, other)),
                }
            }

            fn __richcmp__(
                &self,
                py: Python<'_>,
                other: &Bound<'_, PyAny>,
                op: CompareOp,
            ) -> PyResult<bool> {
                let rhs = other
                    .extract::<PyRef<'_, Self>>()
                    .map_err(|_| mismatch(py, "<=>", $dim, other))?;
                // Canonical f64s; NaN cannot arise from the
                // constructors (the boundary refuses non-finite
                // input), so an absent ordering is unreachable rather
                // than silently coerced.
                match self.0.$canonical().partial_cmp(&rhs.0.$canonical()) {
                    Some(ordering) => Ok(op.matches(ordering)),
                    None => Err(pyo3::exceptions::PyValueError::new_err(
                        "quantity comparison against a non-finite value",
                    )),
                }
            }

            fn __hash__(&self) -> u64 {
                self.0.$canonical().to_bits()
            }
        }
    };
}

/// A length. Canonical unit: metres.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Length(pub(crate) q::Length);

continuous_quantity!(Length, Dimension::Length, meters, {
    /// The value in metres — the canonical unit.
    #[getter]
    fn meters(&self) -> f64 {
        self.0.meters()
    }

    /// The value expressed in `unit`.
    fn in_unit(&self, unit: &LengthUnit) -> f64 {
        self.0.in_unit(unit.0)
    }

    fn __repr__(&self) -> String {
        format!("Length({} m)", self.0.meters())
    }
});

/// An angle. Canonical unit: radians.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Angle(pub(crate) q::Angle);

continuous_quantity!(Angle, Dimension::Angle, radians, {
    /// The value in radians — the canonical unit.
    #[getter]
    fn radians(&self) -> f64 {
        self.0.radians()
    }

    /// The value expressed in `unit`.
    fn in_unit(&self, unit: &AngleUnit) -> f64 {
        self.0.in_unit(unit.0)
    }

    fn __repr__(&self) -> String {
        format!("Angle({} rad)", self.0.radians())
    }
});

/// A dimensionless integer count.
///
/// Deliberately has NO arithmetic, mirroring `quantity::Count`, which
/// implements none: D4's checked count algebra lives in `Expr`, not in
/// the boundary newtype. Binding arithmetic here would invent a
/// semantics the Rust surface does not have.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Count(pub(crate) q::Count);

#[pymethods]
impl Count {
    #[new]
    fn new(value: i64) -> Self {
        Self(q::Count::new(value))
    }

    /// The underlying integer.
    #[getter]
    fn value(&self) -> i64 {
        self.0.get()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        op.matches(self.0.get().cmp(&other.0.get()))
    }

    fn __hash__(&self) -> i64 {
        self.0.get()
    }

    fn __repr__(&self) -> String {
        format!("Count({})", self.0.get())
    }
}

/// A length unit. Multiplying a number by one constructs a `Length`:
/// `25 * mm`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct LengthUnit(pub(crate) q::LengthUnit);

#[pymethods]
impl LengthUnit {
    /// The unit's symbol, e.g. `"mm"`.
    #[getter]
    fn symbol(&self) -> &'static str {
        self.0.symbol()
    }

    /// Metres per unit.
    #[getter]
    fn factor(&self) -> f64 {
        self.0.factor()
    }

    fn __rmul__(&self, value: f64) -> Length {
        Length(value * self.0)
    }

    fn __mul__(&self, value: f64) -> Length {
        Length(value * self.0)
    }

    fn __repr__(&self) -> String {
        format!("LengthUnit({})", self.0.symbol())
    }
}

/// An angle unit. Multiplying a number by one constructs an `Angle`:
/// `90 * deg`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct AngleUnit(pub(crate) q::AngleUnit);

#[pymethods]
impl AngleUnit {
    /// The unit's symbol, e.g. `"deg"`.
    #[getter]
    fn symbol(&self) -> &'static str {
        self.0.symbol()
    }

    /// Radians per unit.
    #[getter]
    fn factor(&self) -> f64 {
        self.0.factor()
    }

    fn __rmul__(&self, value: f64) -> Angle {
        Angle(value * self.0)
    }

    fn __mul__(&self, value: f64) -> Angle {
        Angle(value * self.0)
    }

    fn __repr__(&self) -> String {
        format!("AngleUnit({})", self.0.symbol())
    }
}

/// Register the quantity surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Length>()?;
    m.add_class::<Angle>()?;
    m.add_class::<Count>()?;
    m.add_class::<LengthUnit>()?;
    m.add_class::<AngleUnit>()?;

    // The unit constants, named as `quantity` names them — except
    // `IN`, whose natural spelling `in` is a Python keyword; it is
    // bound as `inch` (reported as a naming fork).
    m.add("mm", LengthUnit(q::MM))?;
    m.add("cm", LengthUnit(q::CM))?;
    m.add("m", LengthUnit(q::M))?;
    m.add("inch", LengthUnit(q::IN))?;
    m.add("deg", AngleUnit(q::DEG))?;
    m.add("rad", AngleUnit(q::RAD))?;
    m.add("pi", AngleUnit(q::PI))?;
    Ok(())
}
