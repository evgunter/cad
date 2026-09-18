//! The expression READ side: `Expr`, and the two typed refusals the
//! doors that make and read one raise.
//!
//! The document layer's fourth vocabulary, after nodes, names and
//! values. A recipe slot is not always a number — `width / 2.0 -
//! margin` is a perfectly ordinary thing for a slot to hold — and a
//! consumer showing that slot needs its VALUE, which it cannot obtain
//! without the evaluator. That is the whole of this module's reason:
//! the façade's own words for it are "a panel that shows a slot
//! before editing it needs this".
//!
//! # Two doors, and the receiver they hang off
//!
//! The curated free functions are `parse_expr`, `eval` and
//! `eval_count`, and all three take a per-document table — `parse_expr`
//! the declared DIMENSIONS, the two evaluators the bound VALUES. So
//! all three arrive in Python as `Doc` methods (`py/doc.rs`), the
//! shape `Evaluation.face_frame` already has and for its reason: the
//! answer is only meaningful against the document that supplies the
//! table, and threading the table in separately would let the two
//! drift. `ParamEnv` itself therefore never crosses — it is built
//! inside the door from the document in hand, exactly as
//! `Evaluation` already builds one for `select_where`.
//!
//! # `Doc.eval` is not `evaluate(doc)`
//!
//! Two evaluations live in this library and they are not the same
//! verb. `evaluate(doc)` runs the RECIPE and answers geometry;
//! `Doc.eval(expr)` runs one expression's arithmetic and answers a
//! number. The Rust façade had the same collision and settled it by
//! module path (`editor_core::expr::eval`, because `editor_core::eval`
//! names the module too); here the receiver and the argument settle
//! it, and the names stay the kernel's own.
//!
//! # Dimensioned out, text in
//!
//! An expression knows its dimension, so [`super::doc::Doc::eval`]
//! answers `Length` for a length and `Angle` for an angle — the
//! crossing rule `py/place.rs` states and `py/readback.rs` follows,
//! not the raw kernel-unit `f64` the Rust `eval` returns. A `Count`
//! expression is exact and has its own door, `eval_count`; asking the
//! continuous one for a count is a typed refusal from the kernel
//! (`count_expr_in_continuous_eval`), never a silent promotion.
//!
//! Inward there are two doors, and they answer different questions.
//! `parse_expr` is the checking parser whose every reduction runs the
//! smart constructors, so it reaches the whole algebra — the
//! operators, the functions, the unit suffixes, the parameter
//! references — through a single call, with exactly the refusals the
//! constructors raise. The dozen individual builders are deliberately
//! not bound: they would be a second spelling of one grammar, and the
//! text is the spelling a panel already has in hand.
//!
//! The LITERAL constructors are the other door, and they are not a
//! second spelling of the parser: they are what an authoring caller
//! holding a `Length` has, where the parser wants a string it would
//! have to build. `Expr.literal` stores the canonical row for the
//! value's own dimension, `Expr.written_length` / `Expr.written_angle`
//! store the notation the author wrote, and `Expr.count` is the exact
//! integer a structural slot takes. They are the four kernel
//! constructors a Rust author reaches for, mirrored — and since every
//! dimensioned slot door takes an `Expr`, they are how a slot is
//! given a number at all. `Expr.length_in` and `Expr.angle_in` are
//! the kernel's sugar over the written pair, mirrored too: one call
//! at an authored number, and exactly the composition they spell.

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyString};

use crate::errors::{ErrorClass, dimension_tag};
use crate::py::doc::ParamName;
use crate::py::typed_err;
use crate::tags::{eval_error_tag, expr_dimension_error_tag, parse_error_tag};
use pncad::document as d;

/// One literal argument, as it crossed: the canonical `f64` and the
/// dimension the caller's own type says it is in.
///
/// A `Length`, an `Angle` or a bare `float`, and anything else is a
/// `TypeError` naming the three — the boundary refusal for an
/// argument that is not a quantity at all, rather than one whose
/// dimension is wrong.
fn quantity(obj: &Bound<'_, PyAny>) -> PyResult<(f64, d::Dimension)> {
    if let Ok(length) = obj.extract::<PyRef<'_, super::quantity::Length>>() {
        return Ok((length.0.meters(), d::Dimension::Length));
    }
    if let Ok(angle) = obj.extract::<PyRef<'_, super::quantity::Angle>>() {
        return Ok((angle.0.radians(), d::Dimension::Angle));
    }
    if let Ok(scalar) = obj.extract::<f64>() {
        return Ok((scalar, d::Dimension::Scalar));
    }
    Err(pyo3::exceptions::PyTypeError::new_err(
        "a literal is a Length, an Angle or a float — the value's own \
         type is the dimension it is written in",
    ))
}

/// Raise `LiteralError` for a value the expression layer refused,
/// carrying the refusal's stable tag and the offending number.
fn literal_err(py: Python<'_>, value: f64, err: &d::DimensionError) -> PyErr {
    let tag = expr_dimension_error_tag(err);
    let value_obj = match value.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed.into(),
    };
    typed_err(
        py,
        ErrorClass::Literal,
        format!("literal {value}: {err}"),
        &[
            ("kind", PyString::new(py, tag).unbind().into_any()),
            ("value", value_obj),
        ],
    )
}

/// Build a dimensioned literal expression, refusing at the boundary.
pub(crate) fn literal(py: Python<'_>, value: f64, dim: d::Dimension) -> PyResult<d::Expr> {
    d::Expr::literal(value, dim).map_err(|err| literal_err(py, value, &err))
}

/// **A dimension-checked expression** — the recipe's arithmetic, as a
/// value.
///
/// Built by `Doc.parse_expr` or by one of the four literal
/// constructors below (or `length_in` / `angle_in`, the composition
/// of two of them): the dimension checker runs at CONSTRUCTION, so
/// an ill-dimensioned tree does not exist to be handed around, and
/// every door runs those checks on the way in.
///
/// It is what every dimensioned slot takes — `Node.extrude(profile,
/// Expr.written_length(w))` — which is the Rust slot's own type
/// (`Node::Extrude { distance: Expr }`) reaching Python unchanged. A
/// slot therefore holds a literal, a written literal or a parsed tree
/// with no second seat and no conversion.
///
/// Read it three ways. `dimension` says what it measures, and it is
/// the fact that decides which evaluator answers. `text` is the
/// source it reads back as — `unparse`, the text door outward, which
/// is what a panel showing a stored expression needs. `params` names
/// the document parameters it references, which is what tells a
/// consumer when a value it displayed has gone stale.
///
/// **No `__hash__`.** Equality is the kernel's own `PartialEq`, and
/// that is an IEEE comparison of the literals inside — so `0.0` and
/// `-0.0` are equal expressions while their bit patterns are not, and
/// there is no hash that respects the first without lying about the
/// second. `DocParam` folds `-0.0` and hashes; an expression TREE has
/// no such cheap fold, so it stays unhashable rather than shipping a
/// hash that disagrees with `==` on a value a document can hold.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Expr(pub(crate) d::Expr);

#[pymethods]
impl Expr {
    /// A continuous literal in the CANONICAL unit for its dimension
    /// — `Expr::literal`, and the door a slot takes a computed
    /// quantity through.
    ///
    /// `value` is a `Length`, an `Angle` or a bare `float`, and the
    /// argument's own type is the literal's dimension: there is no
    /// second fact to keep in step and no dimension to spell. What it
    /// stores is the canonical row — metres, radians, the
    /// dimensionless one — so `Expr.literal(25 * mm)` reads back
    /// `0.025 m`. A value written in a unit the document should
    /// remember is [`Self::written_length`]'s or
    /// [`Self::written_angle`]'s, not this door's.
    ///
    /// A `Count` is not reachable here and that is the kernel's rule,
    /// not a narrowing: a count is an exact integer, so it has its
    /// own door ([`Self::count`]).
    ///
    /// The refusal is `Expr::literal`'s OWN error type, matched — not
    /// predicted: the binding carries no pre-check of its own, so it
    /// cannot drift from what the kernel refuses. The exception
    /// carries `kind` (the stable tag) AND `value`, the offending
    /// number — the kernel error deliberately carries no float, but
    /// the boundary has it in hand.
    #[staticmethod]
    fn literal(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<Self> {
        let (canonical, dim) = quantity(value)?;
        Ok(Self(literal(py, canonical, dim)?))
    }

    /// A continuous literal from an AUTHORED length — the value and
    /// the notation it was written in, together
    /// (`Expr::written_length`).
    ///
    /// The door a recipe records `25 mm` through rather than the
    /// canonical `0.025 m`: the unit is presentation metadata, it
    /// round-trips through the file, and it is what a reader sees
    /// when the slot is shown. `WrittenLength.in_unit(25.0, mm)` is
    /// the value; there is no dimension to spell, because an authored
    /// length is a length.
    ///
    /// Refuses what [`Self::literal`] refuses and nothing more: a
    /// non-finite value. A unit that measures the wrong quantity
    /// cannot be written, so the mismatch has no door here.
    #[staticmethod]
    fn written_length(py: Python<'_>, written: &super::quantity::WrittenLength) -> PyResult<Self> {
        d::Expr::written_length(written.0)
            .map(Self)
            .map_err(|err| literal_err(py, written.0.meters(), &err))
    }

    /// A continuous literal from an AUTHORED angle —
    /// [`Self::written_length`]'s mirror, and everything that door
    /// says holds here with an `AngleUnit`.
    #[staticmethod]
    fn written_angle(py: Python<'_>, written: &super::quantity::WrittenAngle) -> PyResult<Self> {
        d::Expr::written_angle(written.0)
            .map(Self)
            .map_err(|err| literal_err(py, written.0.radians(), &err))
    }

    /// A length authored as `value` in `unit`, in ONE call — exactly
    /// `Expr.written_length(WrittenLength.in_unit(value, unit))`
    /// (`Expr::length_in`).
    ///
    /// Sugar over the two doors and nothing besides: the same stored
    /// notation, the same `LiteralError` for a non-finite value, no
    /// type of its own. It is what an authoring caller holding a
    /// number and a unit writes at every authored length;
    /// [`Self::written_length`] stays the door for a `WrittenLength`
    /// already in hand.
    #[staticmethod]
    fn length_in(py: Python<'_>, value: f64, unit: &super::quantity::LengthUnit) -> PyResult<Self> {
        d::Expr::length_in(value, unit.0)
            .map(Self)
            .map_err(|err| literal_err(py, value, &err))
    }

    /// An angle authored as `value` in `unit`, in one call —
    /// [`Self::length_in`]'s mirror, exactly
    /// `Expr.written_angle(WrittenAngle.in_unit(value, unit))`.
    #[staticmethod]
    fn angle_in(py: Python<'_>, value: f64, unit: &super::quantity::AngleUnit) -> PyResult<Self> {
        d::Expr::angle_in(value, unit.0)
            .map(Self)
            .map_err(|err| literal_err(py, value, &err))
    }

    /// A `Count` literal — an exact integer, and the door every
    /// structural slot takes its value through (`Expr::count`).
    ///
    /// Total: a count is an integer and every integer is a count, so
    /// there is nothing to refuse. It is a separate door from
    /// [`Self::literal`] for the reason spec D4 gives — a count is
    /// exact, and reaching it through a float would be the implicit
    /// promotion the kernel refuses.
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::Expr::count(value))
    }

    /// What this expression measures: `"length"`, `"angle"`,
    /// `"count"` or `"scalar"`.
    ///
    /// Correct by construction — the checker computed it as the tree
    /// was built — and the fact a caller branches on to choose
    /// between `Doc.eval` and `Doc.eval_count`.
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.0.dim())
    }

    /// The source text this expression reads back as (`unparse`).
    ///
    /// The text door OUTWARD: what a panel shows in an edit box, and
    /// what `Doc.parse_expr` reads back to an equal expression. It is
    /// a RENDERING, not the caller's original string — whitespace and
    /// redundant parentheses are the parser's to normalise.
    #[getter]
    fn text(&self) -> String {
        d::unparse(&self.0)
    }

    /// The number a BARE literal carries, in canonical kernel units
    /// (metres, radians), or `None` for anything else.
    ///
    /// Deliberately narrow, and deliberately not a shortcut around
    /// the evaluator: it answers for `25 mm` and not for `width /
    /// 2.0`, which is exactly the case that made the evaluator's
    /// absence bite. A count literal answers `None` here — a count is
    /// an exact integer, and handing it back as an `f64` would be the
    /// implicit promotion spec D4 refuses.
    #[getter]
    fn literal_value(&self) -> Option<f64> {
        self.0.literal_value()
    }

    /// The document parameters this expression references, in sorted
    /// order and without repeats.
    ///
    /// What tells a consumer WHEN to re-evaluate: a displayed value
    /// is stale exactly when one of these parameters moves. Sorted
    /// and deduplicated rather than given in tree order, because the
    /// question is which names are involved and not how many times
    /// each is written.
    #[getter]
    fn params(&self) -> Vec<ParamName> {
        let mut refs = Vec::new();
        self.0.param_refs(&mut refs);
        let mut names: Vec<_> = refs.into_iter().map(|(name, _)| name).collect();
        names.sort();
        names.dedup();
        names.into_iter().map(ParamName).collect()
    }

    fn __repr__(&self) -> String {
        format!("Expr({:?}, {})", d::unparse(&self.0), self.dimension())
    }

    /// The kernel's own `PartialEq`: same tree, and IEEE-equal
    /// literals inside it. NOT `bit_eq`, which is the persistence
    /// layer's replay-identity comparison and a different question.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Raise `ParseError` carrying the refusal's stable tag, its byte
/// offset, and the arm's payload.
///
/// The message is the parser's own `Display` — which for a parser IS
/// the recourse, since it states the position and what the grammar
/// wanted — and the machine payload is `variant` plus the fields,
/// each present on every arm and `None` where that arm does not carry
/// it. `pos` is on every arm and so is never `None`.
///
/// The two numeric fields of `WrongArity` are `arity` and `given`
/// rather than the kernel's `expected`/`found`: those two names are
/// already taken here by the grammar's own STRINGS, and one attribute
/// that is sometimes a word and sometimes a number is a payload a
/// caller cannot branch on. `given` is deliberately not spelled
/// `args`, which is unusable — see [`crate::py::typed_err`].
pub(crate) fn parse_err(py: Python<'_>, err: &d::ParseError) -> PyErr {
    use d::ParseError as P;

    let none = || py.None();
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    // `usize`'s conversion into Python is INFALLIBLE (its error type
    // is `Infallible`), so this match is total and degrades nowhere.
    let int = |n: usize| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };

    // Every field on every arm, `None` where the arm does not carry
    // it: the `ReadbackError` shape, so `getattr` never raises and a
    // caller reads the payload without first branching on `variant`.
    // The tuple is positional and the match is exhaustive, so an arm
    // added kernel-side arrives here as a compile error rather than
    // as a silently unprojected payload.
    let (pos, char_, expected, found, wanted_text, symbol, name, arity, given, kind) = match err {
        P::UnexpectedChar { pos, ch } => (
            *pos,
            text(&ch.to_string()),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        P::UnexpectedEnd { pos, expected } => (
            *pos,
            none(),
            text(expected),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        P::UnexpectedToken {
            pos,
            found,
            expected,
        } => (
            *pos,
            none(),
            text(expected),
            text(found),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        P::TrailingInput { pos, found } => (
            *pos,
            none(),
            none(),
            text(found),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        P::MalformedNumber { pos, text: t } | P::IntegerOverflow { pos, text: t } => (
            *pos,
            none(),
            none(),
            none(),
            text(t),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        P::UnknownUnit { pos, symbol } => (
            *pos,
            none(),
            none(),
            none(),
            none(),
            text(symbol),
            none(),
            none(),
            none(),
            none(),
        ),
        P::UnknownFunction { pos, name } | P::UnknownParam { pos, name } => (
            *pos,
            none(),
            none(),
            none(),
            none(),
            none(),
            text(name),
            none(),
            none(),
            none(),
        ),
        P::WrongArity {
            pos,
            name,
            expected,
            found,
        } => (
            *pos,
            none(),
            none(),
            none(),
            none(),
            none(),
            text(name),
            int(*expected),
            int(*found),
            none(),
        ),
        P::Dimension { pos, error } => (
            *pos,
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(expr_dimension_error_tag(error)),
        ),
    };
    let fields = [
        ("variant", text(parse_error_tag(err))),
        ("pos", int(pos)),
        ("char", char_),
        ("expected", expected),
        ("found", found),
        ("text", wanted_text),
        ("symbol", symbol),
        ("name", name),
        ("arity", arity),
        ("given", given),
        ("kind", kind),
    ];
    typed_err(py, ErrorClass::Parse, err.to_string(), &fields)
}

/// Raise `EvalError` carrying the refusal's stable tag and the arm's
/// payload.
///
/// The message is the evaluator's own `Display` and the machine
/// payload is `variant` plus the fields, on the shape [`parse_err`]
/// uses. The dimension fields cross as the tag strings
/// `crate::errors::dimension_tag` names, which is the spelling
/// `Expr.dimension` and `Measurement.dimension` already answer in —
/// the kernel's `Dimension` type itself does not cross, by the
/// census's own reading of it.
pub(crate) fn eval_err(py: Python<'_>, err: &d::EvalError) -> PyErr {
    use d::EvalError as E;

    let none = || py.None();
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    let dim = |d: d::Dimension| text(dimension_tag(d));
    // `i64`'s conversion is infallible for the same reason `usize`'s
    // is in `parse_err`.
    let int = |n: i64| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };

    let (name, expected, found, count) = match err {
        E::UnknownParam(param) => (text(&param.0), none(), none(), none()),
        E::ParamDimensionMismatch {
            name,
            expected,
            found,
        } => (text(&name.0), dim(*expected), dim(*found), none()),
        E::ContinuousExprInCountEval { found } => (none(), none(), dim(*found), none()),
        E::CountToScalarOutOfRange(value) => (none(), none(), none(), int(*value)),
        E::CountExprInContinuousEval | E::CountOverflow | E::NonFiniteResult => {
            (none(), none(), none(), none())
        }
    };
    let fields = [
        ("variant", text(eval_error_tag(err))),
        ("name", name),
        ("expected", expected),
        ("found", found),
        ("count", count),
    ];
    typed_err(py, ErrorClass::Eval, err.to_string(), &fields)
}

/// Register the expression vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Expr>()?;
    Ok(())
}
