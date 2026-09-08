//! The typed quantities: `25 * mm` constructs a `Length`.
//!
//! These mirror `crates/quantity` exactly — canonical metres and
//! radians underneath, and the SAME infallible arithmetic
//! subset: same-dimension add/sub, negation, scalar scaling, scalar
//! division. `quantity` refuses everything else by having no `impl`
//! for it; Python has no compile step to refuse at, so those cases
//! become typed `DimensionError` raises here. That is the whole of
//! the rule that runtime checks live at the Rust boundary, once.
//!
//! # Reading a quantity out: two doors, and why the second is not a
//! convenience
//!
//! `in_unit` answers a bare `float` — the magnitude, with the unit
//! erased on the way out. `format` answers TEXT: the same magnitude
//! rendered with the unit's own symbol beside it, at digits chosen so
//! that reading the text back recovers the value's exact bits
//! (`quantity::fmt`'s pin; the module docs there carry the
//! derivation and the canonical-unit fallback that pays for it).
//!
//! The distance between those two is the whole of what LIB-B-FORMAT
//! closed. Choosing digits from a bare float is a decision — how many,
//! and does the answer read back as the same number — and until this
//! door crossed, every Python consumer showing a dimension made that
//! decision by hand, differently, with `%g` or `round()` or an
//! f-string, none of which round-trips. The formatter is the library's
//! one answer, and the text it produces is exactly the text
//! `Doc.parse_expr` reads.
//!
//! # The receiver is the quantity, not a float
//!
//! `quantity::fmt_length` takes canonical METRES as an `f64` and
//! `fmt_angle` canonical radians, because Rust reaches this module
//! from below, where the newtype has already been unwrapped. Python
//! has no such caller: what a Python consumer holds is a `Length` or
//! an `Angle`, and it holds one precisely so that a length and an
//! angle cannot be interchanged. Binding the free functions as free
//! functions would hand back that interchange — `fmt_length(
//! (90 * deg).radians, mm)` type-checks and prints plausible
//! nonsense — so the door lands where the value it needs already
//! lives. `Length.format(deg)` is then a `ty` error statically and a
//! `TypeError` at run time, which is the same pair of answers every
//! other dimension confusion at this boundary gets.
//!
//! # The authored pair, beside the arithmetic pair
//!
//! `Length` and `Angle` erase: `25 * mm` is canonical metres and the
//! `mm` is gone at the multiply, which is what makes their arithmetic
//! closed and what the kernel below wants. `WrittenLength` and
//! `WrittenAngle` are the other half of the same boundary — the value
//! WITH the notation it was typed in — and they exist because a
//! document has to be readable back the way it was written.
//!
//! They are separate types rather than a field on `Length` for the
//! reason `quantity::written`'s module docs give: an authored quantity
//! has no arithmetic, because there is no answer to what notation the
//! sum of a millimetre and an inch is written in. A `Length` that
//! carried a unit would have to invent one at every `+`.

use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, QuantityOpMismatch, dimension_tag};
use crate::py::typed_err;
use crate::tags::fmt_quantity_error_tag;
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

/// Raise `FmtQuantityError` carrying the refusal's stable tag and the
/// refused value.
///
/// The field shape is the projected one `py/readback.rs` states and
/// `py/expr.rs` follows: `variant` plus every arm's payload, present
/// on every arm. There is one arm and it carries one number, so the
/// projection is `variant` and `value` and nothing is `None` — the
/// smallest case of the rule rather than an exception to it.
fn fmt_err(py: Python<'_>, err: &q::FmtQuantityError) -> PyErr {
    let q::FmtQuantityError::NonFinite { value } = err;
    let tag = PyString::new(py, fmt_quantity_error_tag(err))
        .unbind()
        .into_any();
    // `f64`'s conversion into a Python float is infallible, the same
    // reason `py/expr.rs`'s integer conversions are: the `Err` type is
    // uninhabited, so the one arm IS exhaustive.
    let refused: Py<PyAny> = match (*value).into_pyobject(py) {
        Ok(object) => object.into_any().unbind(),
    };
    typed_err(
        py,
        ErrorClass::FmtQuantity,
        err.to_string(),
        &[("variant", tag), ("value", refused)],
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
                // Canonical f64s. The `None` arm is REACHABLE and the
                // comment here used to say it was not — "NaN cannot
                // arise from the constructors (the boundary refuses
                // non-finite input)", which is false in both halves:
                // `quantity`'s newtypes are plain value wrappers that
                // refuse no float (its module docs say so outright),
                // and `float("nan") * mm` is an ordinary `Length`
                // here. LIB-B-FORMAT found it by binding the door
                // that has to have an opinion about poison.
                //
                // What it does about it is NOT settled, and the
                // untyped `ValueError` below is the evidence: `==`
                // goes through this same match, so two NaN lengths
                // RAISE rather than answering `False` the way IEEE
                // and every other Python float do. Banked in `work/lib`
                // as `the-quantity-boundary-compares-and-hashes-as-if-
                // poison-and-signed-zero-cannot-arrive` rather than
                // decided here: it is a semantics call on a door
                // LIB-B-FORMAT does not bind, and it shares a root
                // with `__hash__`'s signed-zero split, which is in the
                // same item. `tests/test_quantities.py` pins the
                // behaviour AS IT STANDS, so changing it goes red.
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

    /// This length as DISPLAY TEXT in `unit` — `"25 mm"` — with
    /// digits chosen so the text reads back to this exact value
    /// (`quantity::fmt_length`).
    ///
    /// The pin, which is the reason to use this rather than an
    /// f-string: `doc.parse_expr(x.format(u))` evaluates to `x`
    /// BIT-EXACTLY, for every finite length and every length unit.
    /// The formatter searches the f64 quotients around
    /// `meters / unit.factor` for one whose product with the factor
    /// reproduces these bits, and renders the shortest.
    ///
    /// **The suffix is not guaranteed to be `unit`'s.** Some values
    /// have no preimage in the asked unit at all — the multiply by
    /// the factor steps over them — and those render in metres
    /// instead, because the pin above is the promise and the suffix
    /// is what pays for it. A length AUTHORED in the unit (`25 * mm`,
    /// or parsed from `"25 mm"`) always keeps it; the fallback bites
    /// values arrived at by arithmetic. Read the suffix off the text
    /// rather than assuming it.
    ///
    /// # Raises
    ///
    /// `FmtQuantityError` when this length is NaN or ±∞ — poison
    /// never lands in display text.
    fn format(&self, py: Python<'_>, unit: &LengthUnit) -> PyResult<String> {
        q::fmt_length(self.0.meters(), unit.0).map_err(|err| fmt_err(py, &err))
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

    /// This angle as DISPLAY TEXT in `unit` — `"90 deg"` — with
    /// digits chosen so the text reads back to this exact value
    /// (`quantity::fmt_angle`). Same pin, same canonical fallback
    /// (to radians here) and same caveats as `Length.format`.
    ///
    /// `pi_rad` is the row this matters most for: a quarter turn
    /// written `0.5 * pi_rad` and one written `90 * deg` are the same
    /// canonical radians to within an ulp, and NEITHER is exact, so
    /// nothing but the unit you ask for distinguishes them —
    /// `format(pi_rad)` says `"0.5 pi rad"` and `format(deg)` says
    /// `"90 deg"` for the respective values, each read back in the
    /// notation it was authored in.
    ///
    /// # Raises
    ///
    /// `FmtQuantityError` when this angle is NaN or ±∞.
    fn format(&self, py: Python<'_>, unit: &AngleUnit) -> PyResult<String> {
        q::fmt_angle(self.0.radians(), unit.0).map_err(|err| fmt_err(py, &err))
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

    /// Rust's derived `PartialEq`, mirrored: two views of the same
    /// table row are the same unit. The symbol DETERMINES the row (the
    /// #650 seal), so comparing symbols compares rows.
    ///
    /// Bound because a unit is now something a caller gets BACK —
    /// `WrittenLength.unit` — and not only something the module hands
    /// out as a constant. Without it the fallback is identity, and a
    /// unit read off a value compares unequal to the `mm` it was
    /// written in.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: the row's symbol, which is
    /// what that comparison reads.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.symbol().hash(&mut h);
        h.finish()
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

    /// [`LengthUnit::__eq__`]'s mirror, for its reason.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// [`LengthUnit::__hash__`]'s mirror.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.symbol().hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!("AngleUnit({})", self.0.symbol())
    }
}

/// Fold `-0.0` to `0.0` before hashing, so a hash never splits two
/// values the IEEE equality above calls the same.
fn fold_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0 } else { v }
}

/// A length as it was AUTHORED: canonical metres plus the notation it
/// was written in (`quantity::WrittenLength`).
///
/// `Length` is the arithmetic type and it erases — `25 * mm` is
/// `Length(0.025)` and the `mm` is gone at the multiply, which is
/// correct for everything downstream, since the kernel only ever sees
/// metres. This type is the record of what was TYPED, so a document
/// can be read back the way it was written, and it is what
/// `DocParam.written_length` takes.
///
/// **No arithmetic, deliberately** — the Rust type has none for the
/// reason its module docs give: there is no answer to what notation
/// the sum of a millimetre and an inch is written in. Compute on the
/// `Length` inside (`length`), where the algebra is closed and the
/// unit has already erased, and author the result through
/// `canonical_in`.
///
/// **The notation is not optional.** Every value here names a unit;
/// there is no "canonical, notation unknown" state, which is what
/// stops two readers of one document from applying two fallbacks.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct WrittenLength(pub(crate) q::WrittenLength);

#[pymethods]
impl WrittenLength {
    /// `value` written in `unit` — the multiply happens here, and the
    /// notation survives it. `WrittenLength.in_unit(25.0, mm)` is
    /// `25 * mm` that remembers the `mm`.
    ///
    /// The name is `Length.in_unit`'s, in the opposite direction, and
    /// the collision is the kernel's own: `WrittenLength::in_unit` is
    /// documented there as "the inverse of `Length::in_unit`". A
    /// second spelling here would make the two surfaces disagree about
    /// one pair of doors.
    #[staticmethod]
    fn in_unit(value: f64, unit: &LengthUnit) -> Self {
        Self(q::WrittenLength::in_unit(value, unit.0))
    }

    /// An ALREADY-canonical length that remembers `unit` as its
    /// notation — no multiply, because the caller's arithmetic has
    /// happened.
    ///
    /// This is the door for a value arrived at by computing: a length
    /// summed or scaled out of others is canonical metres, and this
    /// says which notation to record it in. It takes a `Length` where
    /// the Rust door takes bare metres, for the reason this module's
    /// docs give for `format`'s receiver — Rust reaches the type from
    /// below, with the newtype already unwrapped, and what a Python
    /// caller holds is the quantity.
    #[staticmethod]
    fn canonical_in(value: &Length, unit: &LengthUnit) -> Self {
        Self(q::WrittenLength::canonical_in(value.0.meters(), unit.0))
    }

    /// The canonical value — the erasure door, past which nothing
    /// knows this was authored in anything.
    #[getter]
    fn length(&self) -> Length {
        Length(self.0.length())
    }

    /// The canonical value in metres (`length` then `meters`).
    #[getter]
    fn meters(&self) -> f64 {
        self.0.meters()
    }

    /// The notation this was authored in.
    #[getter]
    fn unit(&self) -> LengthUnit {
        LengthUnit(self.0.unit())
    }

    /// Rust's derived `PartialEq`, mirrored: BOTH halves, so the same
    /// magnitude authored in two units is two authorings. That is the
    /// opposite of the stored literal this feeds, where the display
    /// unit is presentation metadata excluded from expression
    /// identity — and the difference is the point, because here the
    /// notation is the payload.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`], on `DocParam`'s already
    /// settled shape rather than `Length`'s: `-0.0` folds to `0.0`
    /// before the value's bits are hashed, because the equality this
    /// mirrors calls the two spellings of zero the same value and a
    /// hash that split them would break the invariant Python dicts
    /// rely on. The unit is part of the equality, so it is part of the
    /// hash.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        fold_zero(self.0.meters()).to_bits().hash(&mut h);
        self.0.unit().symbol().hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "WrittenLength({} m in {})",
            self.0.meters(),
            self.0.unit().symbol()
        )
    }
}

/// An angle as it was AUTHORED — [`WrittenLength`]'s mirror, canonical
/// radians plus its notation (`quantity::WrittenAngle`). Everything
/// that type's docs say holds here.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct WrittenAngle(pub(crate) q::WrittenAngle);

#[pymethods]
impl WrittenAngle {
    /// `value` written in `unit` — [`WrittenLength::in_unit`]'s
    /// mirror, and the one door here that multiplies.
    #[staticmethod]
    fn in_unit(value: f64, unit: &AngleUnit) -> Self {
        Self(q::WrittenAngle::in_unit(value, unit.0))
    }

    /// An already-canonical angle that remembers `unit` —
    /// [`WrittenLength::canonical_in`]'s mirror.
    #[staticmethod]
    fn canonical_in(value: &Angle, unit: &AngleUnit) -> Self {
        Self(q::WrittenAngle::canonical_in(value.0.radians(), unit.0))
    }

    /// The canonical value — the erasure door.
    #[getter]
    fn angle(&self) -> Angle {
        Angle(self.0.angle())
    }

    /// The canonical value in radians (`angle` then `radians`).
    #[getter]
    fn radians(&self) -> f64 {
        self.0.radians()
    }

    /// The notation this was authored in.
    #[getter]
    fn unit(&self) -> AngleUnit {
        AngleUnit(self.0.unit())
    }

    /// [`WrittenLength::__eq__`]'s mirror: both halves.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// [`WrittenLength::__hash__`]'s mirror, same fold.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        fold_zero(self.0.radians()).to_bits().hash(&mut h);
        self.0.unit().symbol().hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "WrittenAngle({} rad in {})",
            self.0.radians(),
            self.0.unit().symbol()
        )
    }
}

/// Register the quantity surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Length>()?;
    m.add_class::<Angle>()?;
    m.add_class::<Count>()?;
    m.add_class::<WrittenLength>()?;
    m.add_class::<WrittenAngle>()?;
    m.add_class::<LengthUnit>()?;
    m.add_class::<AngleUnit>()?;

    // The unit constants, named as `quantity` names them — except two
    // whose natural spellings are not Python identifiers: `IN`, whose
    // symbol `in` is a keyword, is bound as `inch`, and `PI`, whose
    // symbol is the two-word `pi rad`, is bound as `pi_rad` (both
    // reported as naming forks). `symbol()` still answers the table's
    // own spelling in every case.
    m.add("mm", LengthUnit(q::MM))?;
    m.add("cm", LengthUnit(q::CM))?;
    m.add("m", LengthUnit(q::M))?;
    m.add("inch", LengthUnit(q::IN))?;
    m.add("deg", AngleUnit(q::DEG))?;
    m.add("rad", AngleUnit(q::RAD))?;
    m.add("pi_rad", AngleUnit(q::PI))?;
    Ok(())
}
