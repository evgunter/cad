//! The placement vocabulary: `Frame` and `PatternKind`.
//!
//! GROUP-BOOLEAN-DESIGN's rule vocabulary, crossing whole. A
//! `PlacedUnion` node needs exactly two values that no other node
//! kind needs — an absolute placement and a replication rule — so
//! they live together here rather than swelling the document module.
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

/// Raise `FrameError` carrying the refusal's stable tag.
pub(crate) fn frame_err(py: Python<'_>, err: &pncad::geom_core::FrameError) -> PyErr {
    typed_err(
        py,
        ErrorClass::Frame,
        // `FrameError` implements `Display`, so the human message is
        // the kernel's own prose (including its coincidence recourse);
        // the machine payload is the `variant` tag.
        err.to_string(),
        &[(
            "variant",
            PyString::new(py, crate::tags::frame_error_tag(err))
                .unbind()
                .into_any(),
        )],
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
/// This is the value [`PatternKind::explicit`] lists and the one
/// `Node.placed_union_at` places a prototype at. It is authored
/// through the two constructors below rather than field by field,
/// because a frame's linear part is a MATRIX and hand-writing one is
/// how a mirror or a shear arrives by accident — and improper frames
/// (determinant ≤ 0) are refused at the edit door, not admitted.
///
/// The frame is not a rigidity claim: rigidity is a decided predicate
/// the kernel's placement door owns, and what the edit door checks is
/// that the coordinates are finite and the determinant positive.
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
    /// The order is `Node.transform`'s own (D9), so a placement and a
    /// modeled transform of the same part agree BIT FOR BIT — which is
    /// what makes the group-versus-chain equality testable. The axis
    /// is normalized here, exactly once more than a caller expects,
    /// for the same reason: same input, same expression, same bits.
    ///
    /// A zero (or non-finite) axis normalizes to NaN and yields a
    /// non-finite frame, refused typed at the edit door
    /// (`non_finite_placement`) rather than read as "no rotation".
    #[staticmethod]
    fn rotate_then_translate(
        axis: (f64, f64, f64),
        angle: &Angle,
        v: (Length, Length, Length),
    ) -> Self {
        Self(d::Frame::rotate_then_translate(
            [axis.0, axis.1, axis.2],
            angle.0.radians(),
            meters(v),
        ))
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
        .map(|a| Self(d::Frame::from_affine(a)))
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
        .map(|a| Self(d::Frame::from_affine(a)))
        .map_err(|err| frame_err(py, &err))
    }

    /// Reflection across the plane through `point` with `normal`
    /// — an IMPROPER frame, determinant −1.
    ///
    /// It is constructible and it is not placeable: the edit door
    /// refuses a mirror placement (`improper_placement`), so this
    /// constructor is how a caller
    /// obtains the reflection as a VALUE — and how the refusal can be
    /// exercised at all.
    #[staticmethod]
    fn mirror_across_plane(
        py: Python<'_>,
        point: (Length, Length, Length),
        normal: (f64, f64, f64),
    ) -> PyResult<Self> {
        let tol = Tol::witness();
        let [px, py_, pz] = meters(point);
        pncad::geom_core::linalg::frame::mirror_across_plane::<f64>(
            pncad::authoring::p3(px, py_, pz),
            pncad::authoring::v3(normal.0, normal.1, normal.2),
            tol,
        )
        .map(|a| Self(d::Frame::from_affine(a)))
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

    /// The linear part's determinant — positive for a placement the
    /// edit door admits, ≤ 0 for the mirrors the edit door refuses.
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

    /// Consistent with [`Self::__eq__`] BY CONSTRUCTION: it hashes the
    /// same twelve bit patterns the comparison reads.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        for column in self.0.columns {
            for c in column {
                c.to_bits().hash(&mut h);
            }
        }
        for c in self.0.translation {
            c.to_bits().hash(&mut h);
        }
        h.finish()
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

/// A pattern's replication rule: how a prototype's placements are
/// generated (`PatternKind`, crossing whole).
///
/// Three rules and no fourth. The two PARAMETRIC ones step — along a
/// direction, or around a datum axis — and take their count from the
/// node's structural slot; the EXPLICIT one lists absolute frames and
/// **the list IS the count**, which is why the node door that takes it
/// takes no count at all.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct PatternKind(pub(crate) d::PatternKind);

#[pymethods]
impl PatternKind {
    /// Instances stepped along `direction`, `spacing` apart.
    ///
    /// The direction is a dimensionless triple (`SlotId::Direction` is
    /// `Scalar`); the spacing is a `Length`.
    #[staticmethod]
    fn linear(py: Python<'_>, direction: (f64, f64, f64), spacing: &Length) -> PyResult<Self> {
        let scalar = |v: f64| super::doc::literal(py, v, d::Dimension::Scalar);
        Ok(Self(d::PatternKind::Linear {
            direction: [
                scalar(direction.0)?,
                scalar(direction.1)?,
                scalar(direction.2)?,
            ],
            spacing: super::doc::literal(py, spacing.0.meters(), d::Dimension::Length)?,
        }))
    }

    /// Instances stepped `step` apart around `axis`, an upstream
    /// `datum_axis` node.
    #[staticmethod]
    fn circular(py: Python<'_>, axis: &super::doc::NodeId, step: &Angle) -> PyResult<Self> {
        Ok(Self(d::PatternKind::Circular {
            axis: axis.0,
            step: super::doc::literal(py, step.0.radians(), d::Dimension::Angle)?,
        }))
    }

    /// Instances at the ABSOLUTE frames listed — the non-parametric
    /// member, for placements no linear or circular step generates.
    ///
    /// Order is data: the index is what a name's `Instance(i)`
    /// segment carries, so appending a placement changes no existing
    /// index. An EMPTY list is this rule's `count < 1` and refuses
    /// typed at the edit door (`empty_placement_list`).
    #[staticmethod]
    fn explicit(frames: Vec<Frame>) -> Self {
        Self(d::PatternKind::Explicit(
            frames.into_iter().map(|f| f.0).collect(),
        ))
    }
}

/// Register the placement vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Frame>()?;
    m.add_class::<PatternKind>()?;
    Ok(())
}
