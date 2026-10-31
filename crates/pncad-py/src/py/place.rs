//! The absolute placement value: `Frame`.
//!
//! Dimensioned throughout: lengths cross as `Length`, angles as `Angle`, and
//! a bare float appears only where the Rust side is itself a
//! dimensionless direction or a matrix entry.

use pyo3::prelude::*;
use pyo3::types::PyString;

use super::quantity::{Angle, Length};
use crate::errors::ErrorClass;
use crate::py::typed_err;
use pncad::document as d;
use pncad::tolerance::Tol;

/// Raise `FrameError` carrying the refusal's stable tag and the arm's
/// payload.
///
/// The machine payload is `variant` plus the fields, each present on
/// every arm and `None` where that arm does not carry it. The tuple
/// is positional and the match is exhaustive, so an arm added
/// kernel-side arrives here as a compile error rather than as a
/// silently unprojected payload.
///
/// **The degenerate arm's `input` does not get an attribute of its
/// own**: it IS the `variant`, which `crate::tags::frame_error_tag`
/// mints per input (`degenerate_aim`, `degenerate_tangent`, ...), so
/// a second spelling of the same word would publish one fact twice.
///
/// What the degenerate arm does carry is the CLASSIFIER's payload,
/// present when the margin landed in the ambiguity band and absent
/// when it was a definite zero: `margin` (the in-band value),
/// `margin_low` / `margin_high` (the enclosure's bounds, where the
/// classifier saw an enclosure rather than a value), `zero` and
/// `escalate` (the band it was classified against), and `predicate`
/// (the decision's name, where the kernel attached one). A poisoned
/// margin carries the band and no number.
///
/// `zero` and `escalate` are one concept across two arms and cross on
/// one pair of names: the band a margin was classified against, and
/// the thresholds a `BandError::Empty` could not form a band from.
/// `value` is likewise the rejected number of either `InvalidValue`
/// or `InvalidLeverArm`.
///
/// The classifier half is [`crate::escalation::escalation`] rather
/// than a fork written here: one projection of the escalation, under
/// these same words, is what keeps its spellings from drifting.
pub(crate) fn frame_err(py: Python<'_>, err: &pncad::geom_core::FrameError) -> PyErr {
    use pncad::geom_core::{BandError, FrameError as E};

    let none = || py.None();
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    // `f64`'s conversion is INFALLIBLE (its error type is
    // `Infallible`), so this one degrades nowhere: the match is total.
    let real = |x: f64| -> Py<PyAny> {
        match x.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };
    let maybe = |x: Option<f64>| match x {
        Some(x) => real(x),
        None => none(),
    };
    let word = |t: Option<&'static str>| match t {
        Some(t) => text(t),
        None => none(),
    };

    let (inner, margin, margin_low, margin_high, zero, escalate, predicate, field, value) =
        match err {
            E::Degenerate {
                indeterminate: None,
                ..
            } => (
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
            ),
            E::Degenerate {
                indeterminate: Some(i),
                ..
            } => {
                // The margin's fork is `crate::escalation`'s, shared
                // with the frame witness mint's refusal.
                let e = crate::escalation::escalation(i);
                (
                    none(),
                    maybe(e.margin),
                    maybe(e.margin_low),
                    maybe(e.margin_high),
                    real(e.zero),
                    real(e.escalate),
                    word(e.predicate),
                    none(),
                    none(),
                )
            }
            // Nothing was classified: the door refused before any
            // margin reached the funnel, so the whole payload is
            // absent exactly as it is for a definite zero. Both format
            // arms are that case — overflowed and underflowed alike.
            E::NonFiniteLength { .. } | E::UnderflowedLength { .. } => (
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
                none(),
            ),
            E::Band(inner) => {
                let (which, v, z, e) = match inner {
                    BandError::InvalidValue { field, value } => (
                        text(crate::tags::band_field_tag(field)),
                        real(*value),
                        none(),
                        none(),
                    ),
                    BandError::InvalidLeverArm { value } => (none(), real(*value), none(), none()),
                    BandError::Empty { zero, escalate } => {
                        (none(), none(), real(*zero), real(*escalate))
                    }
                };
                (
                    text(crate::tags::band_error_tag(inner)),
                    none(),
                    none(),
                    none(),
                    z,
                    e,
                    none(),
                    which,
                    v,
                )
            }
        };
    typed_err(
        py,
        ErrorClass::Frame,
        // `FrameError` implements `Display`, so the human message is
        // the kernel's own prose (including its recourse);
        // the machine payload is the `variant` tag and the fields.
        err.to_string(),
        &[
            ("variant", text(crate::tags::frame_error_tag(err))),
            ("inner_variant", inner),
            ("margin", margin),
            ("margin_low", margin_low),
            ("margin_high", margin_high),
            ("zero", zero),
            ("escalate", escalate),
            ("predicate", predicate),
            ("field", field),
            ("value", value),
        ],
    )
}

/// A dimensioned triple as the kernel's bare metres.
fn meters(v: (Length, Length, Length)) -> [f64; 3] {
    [v.0.0.meters(), v.1.0.meters(), v.2.0.meters()]
}

/// Bare metres as the dimensioned triple they always were.
fn lengths(v: [f64; 3]) -> (Length, Length, Length) {
    let len = |x: f64| Length(pncad::quantity::Length::from_meters(x));
    (len(v[0]), len(v[1]), len(v[2]))
}

/// An ABSOLUTE placement: a linear part and a translation, together
/// the affine map that puts a prototype somewhere in the world.
///
/// It is what a placement's pose reads back as. It is authored
/// through the constructors below rather than field by field,
/// because a frame's linear part is a MATRIX and hand-writing one is
/// how a mirror or a shear arrives by accident — and a frame that is
/// not a rigid motion is refused at the edit door, not admitted. A
/// mirror is a construction (`Node.mirror`), never a frame.
///
/// The frame is not a rigidity claim: rigidity is a decided predicate
/// the kernel's placement door owns, and the edit door asks it.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Frame(pub(crate) d::Frame);

#[pymethods]
impl Frame {
    /// The pure translation by `v` — the identity linear part.
    #[staticmethod]
    fn translation(v: (Length, Length, Length)) -> Self {
        Self(d::Frame::translation(meters(v)))
    }

    /// Rotate by `angle` about the axis through the WORLD ORIGIN with
    /// direction `axis`, THEN translate by `v`.
    ///
    /// The axis is normalized here, so the same input gives the same
    /// expression and the same bits wherever it is built.
    ///
    /// A zero (or non-finite) axis has no definite direction and is
    /// refused HERE, naming the axis and its role
    /// (`EditError`, `placement_axis`) — never read as "no rotation",
    /// and never reported as a frame that is not finite.
    #[staticmethod]
    fn rotate_then_translate(
        py: Python<'_>,
        axis: (f64, f64, f64),
        angle: &Angle,
        v: (Length, Length, Length),
    ) -> PyResult<Self> {
        // The BAND is the run's tolerance configuration, not the
        // axis: it crosses as the band refusal it is (`ChecksError`'s
        // arm, the one door in this crate whose subject is a whole
        // document's tolerance), never wearing the axis's words.
        let band = pncad::geom_core::Band::linear(Tol::witness())
            .map_err(|error| super::checks::checks_err(py, &d::ChecksError::Band { error }))?;
        d::Frame::rotate_then_translate(
            [axis.0, axis.1, axis.2],
            angle.0.radians(),
            meters(v),
            band,
        )
        .map(Self)
        .map_err(|err| super::doc::edit_err(py, &d::EditError::from(err)))
    }

    /// Rotate by `num/den` of a turn about the axis through the WORLD
    /// ORIGIN with direction `axis`, THEN translate by `v`: the
    /// rotation's sine and cosine read from the fraction, so a whole
    /// number of quarter turns about a coordinate axis has columns of
    /// `0` and `±1` exactly. Refuses the axis as `rotate_then_translate`
    /// does; a zero `den` is a `ValueError`.
    #[staticmethod]
    fn rotate_by_turns_then_translate(
        py: Python<'_>,
        axis: (f64, f64, f64),
        num: i64,
        den: u64,
        v: (Length, Length, Length),
    ) -> PyResult<Self> {
        let turns = pncad::geom_core::Turns::new(num, den).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "a fraction of a turn has a nonzero denominator",
            )
        })?;
        let band = pncad::geom_core::Band::linear(Tol::witness())
            .map_err(|error| super::checks::checks_err(py, &d::ChecksError::Band { error }))?;
        d::Frame::rotate_by_turns_then_translate([axis.0, axis.1, axis.2], turns, meters(v), band)
            .map(Self)
            .map_err(|err| super::doc::edit_err(py, &d::EditError::from(err)))
    }

    /// The frame at `eye` whose local **+Z aims at** `target`, with
    /// local +X placed by `roll_reference`.
    ///
    /// The roll reference fixes the one degree of freedom aiming
    /// leaves: local +X runs along `roll_reference × aim`. A reference
    /// PARALLEL to the aim fixes nothing, and refuses rather than
    /// picking a fallback — as does an aim of no definite length
    /// (coincident eye and target).
    #[staticmethod]
    fn point_at(
        py: Python<'_>,
        eye: (Length, Length, Length),
        target: (Length, Length, Length),
        roll_reference: (f64, f64, f64),
    ) -> PyResult<Self> {
        let tol = Tol::witness();
        let [ex, ey, ez] = meters(eye);
        let [tx, ty, tz] = meters(target);
        pncad::geom_core::linalg::frame::point_at::<f64>(
            pncad::authoring::p3(ex, ey, ez),
            pncad::authoring::p3(tx, ty, tz),
            pncad::authoring::v3(roll_reference.0, roll_reference.1, roll_reference.2),
            tol,
        )
        .map(|f| Self(d::Frame::from_affine(f.to_affine())))
        .map_err(|err| frame_err(py, &err))
    }

    /// The profile frame at the start of a swept path: a frame at
    /// `origin` whose local **+Z is the path `tangent`**, so the local
    /// XY plane is the plane the profile is drawn in.
    ///
    /// There is no tip to read a roll from, so the roll comes from a
    /// stated LADDER — world +Z, then world +X — and the frame
    /// refuses if neither is definitely off the tangent line rather
    /// than returning an all-zero frame.
    #[staticmethod]
    fn path_start_frame(
        py: Python<'_>,
        origin: (Length, Length, Length),
        tangent: (f64, f64, f64),
    ) -> PyResult<Self> {
        let tol = Tol::witness();
        let [ox, oy, oz] = meters(origin);
        pncad::geom_core::linalg::frame::path_start_frame::<f64>(
            pncad::authoring::p3(ox, oy, oz),
            pncad::authoring::v3(tangent.0, tangent.1, tangent.2),
            tol,
        )
        .map(|f| Self(d::Frame::from_affine(f.to_affine())))
        .map_err(|err| frame_err(py, &err))
    }

    /// The linear part's columns: `columns[j]` is the image of basis
    /// vector `j` — dimensionless, as the matrix entries are.
    ///
    /// Tuples, not lists: the frame is frozen, and a mutable read-back
    /// would let a caller edit a copy and believe the placement moved.
    #[getter]
    #[allow(clippy::type_complexity)]
    fn columns(&self) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        let [c0, c1, c2] = self.0.columns;
        (
            (c0[0], c0[1], c0[2]),
            (c1[0], c1[1], c1[2]),
            (c2[0], c2[1], c2[2]),
        )
    }

    /// The image of the coordinate origin, as a displacement from it —
    /// the Rust value's `translation` field, read back.
    ///
    /// It is spelled `origin` because `translation` is already this
    /// class's pure-translation CONSTRUCTOR and one Python name cannot
    /// be both; `origin` is what `SketchPlane` calls the same
    /// quantity, so the binding reuses a name it already has rather
    /// than minting a third.
    #[getter]
    fn origin(&self) -> (Length, Length, Length) {
        lengths(self.0.translation)
    }

    /// The linear part's determinant — `+1`, within rounding, for a
    /// placement the edit door admits.
    #[getter]
    fn determinant(&self) -> f64 {
        self.0.determinant()
    }

    /// BIT-exact frame equality — Rust's `Frame::bit_eq`, crossing
    /// unchanged (the `SketchPlane` precedent): `0.0` and `-0.0` are
    /// different placements here.
    ///
    /// Bit equality is the only equality a frame can honestly offer:
    /// it carries no ε, and "the same placement up to tolerance" is a
    /// geometric question the kernel answers about BODIES.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    fn __repr__(&self) -> String {
        let [c0, c1, c2] = self.0.columns;
        let t = self.0.translation;
        format!(
            "Frame(columns=(({}, {}, {}), ({}, {}, {}), ({}, {}, {})), \
             origin=({}, {}, {}))",
            c0[0], c0[1], c0[2], c1[0], c1[1], c1[2], c2[0], c2[1], c2[2], t[0], t[1], t[2]
        )
    }
}

/// Register the placement vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Frame>()?;
    Ok(())
}
