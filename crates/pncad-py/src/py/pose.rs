//! **A pose defined at its reader's seat** (D10): `PoseDef`, the Python
//! spelling of `editor_core::pose::PoseDef` authored as an operand.
//!
//! No pose is free and none is defined from nothing: each constructor
//! reads geometry (a face as a plane, a carrier's axis or centre, a
//! vertex's point), moves another pose by a slide or a spin, or
//! constructs from other poses. A `PoseDef` is passed where an
//! operand is read; the edit door mints the pose variable there, as it
//! mints a selection authored at a seat. A pose defined at two seats is
//! two variables, each defined the same way.

use pyo3::prelude::*;

use pncad::document as d;
use pncad::document::pose::{PoseDef as Def, PoseRole, PoseSlot, RoleRead as Read};

use super::doc::{OperandArg, name_from_text};
use super::expr::SlotArg;

/// **A pose defined where it is read**: pass it to an operand that reads
/// a pose (a split's `tool`, a profile's frame, a circular rule's axis).
#[pyclass(module = "pncad", frozen, from_py_object)]
#[derive(Clone)]
pub(crate) struct PoseDef {
    /// The authored operand: a pose definition.
    pub(crate) inner: d::Operand,
}

/// **A finer pose read through a role** at a seat of a coarser kind
/// (D10): a frame's `"x"`, `"y"` or `"z"` axis, its `"origin"`, or its
/// `"xy"`, `"yz"` or `"zx"` plane; a plane's `"normal"`; an axis's
/// `"direction"`. The role is part of the read, so a frame read as its
/// z axis is the frame's one variable, read once.
#[pyclass(module = "pncad", frozen, from_py_object)]
#[derive(Clone)]
pub(crate) struct RoleRead {
    read: d::Operand,
    role: PoseRole,
}

/// The role words a read names.
pub(crate) fn role_of(word: &str) -> PyResult<PoseRole> {
    Ok(match word {
        "x" => PoseRole::X,
        "y" => PoseRole::Y,
        "z" => PoseRole::Z,
        "origin" => PoseRole::Origin,
        "xy" => PoseRole::Xy,
        "yz" => PoseRole::Yz,
        "zx" => PoseRole::Zx,
        "normal" => PoseRole::Normal,
        "direction" => PoseRole::Direction,
        other => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "not a pose role: {other:?} — one of \"x\", \"y\", \"z\", \"origin\", \"xy\", \
                 \"yz\", \"zx\" (a frame's), \"normal\" (a plane's) or \"direction\" (an axis's)"
            )));
        }
    })
}

/// The word a role is named by in Python ([`role_of`]'s inverse).
pub(crate) fn role_word(role: PoseRole) -> &'static str {
    match role {
        PoseRole::X => "x",
        PoseRole::Y => "y",
        PoseRole::Z => "z",
        PoseRole::Origin => "origin",
        PoseRole::Xy => "xy",
        PoseRole::Yz => "yz",
        PoseRole::Zx => "zx",
        PoseRole::Normal => "normal",
        PoseRole::Direction => "direction",
    }
}

#[pymethods]
impl RoleRead {
    /// `of` read through `role`.
    #[new]
    fn new(of: OperandArg, role: &str) -> PyResult<Self> {
        Ok(Self {
            read: of.read(),
            role: role_of(role)?,
        })
    }

    /// The role, as its word.
    #[getter]
    fn role(&self) -> &'static str {
        role_word(self.role)
    }

    fn __repr__(&self) -> String {
        format!("RoleRead({}, {:?})", self.read, role_word(self.role))
    }
}

/// **What a pose seat reads**: an operand read plainly, or one read
/// through a role.
#[derive(FromPyObject, Clone)]
pub(crate) enum PoseArg {
    /// Through a role.
    Role(RoleRead),
    /// Plainly.
    Plain(OperandArg),
}

impl PoseArg {
    /// The authored read.
    pub(crate) fn read(self) -> Read<d::Operand> {
        match self {
            Self::Role(role) => Read::through(role.read, role.role),
            Self::Plain(plain) => Read::plain(plain.read()),
        }
    }
}

fn scalar(py: Python<'_>, slot: PoseSlot, arg: &SlotArg) -> PyResult<d::Formula> {
    scalar_at(
        py,
        slot.dimension(),
        arg,
        &format!("pose's {}", slot.label()),
    )
}

/// **A move's or a value's number at `expected`**: `arg` read at that
/// dimension, a lone `0` the constant zero of it (VR6). `what` names
/// the slot in the refusal.
pub(crate) fn scalar_at(
    py: Python<'_>,
    expected: d::Dimension,
    arg: &SlotArg,
    what: &str,
) -> PyResult<d::Formula> {
    let formula = arg.formula(py, expected)?;
    if formula.dim() == expected {
        return Ok(formula);
    }
    if formula.is_zero() {
        return Ok(d::Formula::zero(expected));
    }
    Err(pyo3::exceptions::PyValueError::new_err(format!(
        "the {what} reads a {}, not a {}",
        expected,
        formula.dim()
    )))
}

fn select(at: OperandArg, name: &str) -> PyResult<d::Operand> {
    Ok(d::Operand::select(at.read(), vec![name_from_text(name)?]))
}

fn def(inner: Def<d::Operand, d::Formula>) -> PoseDef {
    PoseDef {
        inner: d::Operand::Pose(Box::new(inner)),
    }
}

#[pymethods]
impl PoseDef {
    /// **A face read as a plane**: its carrier's plane, with the face's
    /// outward normal. `at` is the body the face is read out of, `face`
    /// one of the names `Evaluation.all_faces` answers with. A curved
    /// face refuses `pose_read` at `evaluate`.
    #[staticmethod]
    fn plane(at: OperandArg, face: &str) -> PyResult<Self> {
        Ok(def(Def::Plane {
            face: select(at, face)?,
        }))
    }

    /// **An axis read off geometry**: a cylinder's, a cone's or a
    /// torus's axis, a straight edge's line, or a circular edge's axis.
    /// `name` names a face or an edge of the body `at` reads.
    #[staticmethod]
    fn axis(at: OperandArg, name: &str) -> PyResult<Self> {
        Ok(def(Def::Axis {
            of: select(at, name)?,
        }))
    }

    /// **A point read off geometry**: a sphere's, a torus's or a
    /// circle's centre, a cone's apex, or a vertex's point.
    #[staticmethod]
    fn point(at: OperandArg, name: &str) -> PyResult<Self> {
        Ok(def(Def::Point {
            of: select(at, name)?,
        }))
    }

    /// **A pose slid** along a direction by the length `by`: a point,
    /// an axis, a plane or a frame moved, its kind kept. `along` is a
    /// direction, or an axis read for its direction (a frame's
    /// `RoleRead(frame, "x")`, a plane's `RoleRead(plane, "normal")`).
    #[staticmethod]
    fn slide(py: Python<'_>, pose: PoseArg, along: PoseArg, by: SlotArg) -> PyResult<Self> {
        Ok(def(Def::Slide {
            pose: pose.read(),
            along: along.read(),
            by: scalar(py, PoseSlot::Slide, &by)?,
        }))
    }

    /// **A pose spun** about an axis by the angle `by`, right-handed:
    /// any pose turned about the line, its kind kept.
    #[staticmethod]
    fn spin(py: Python<'_>, pose: PoseArg, about: PoseArg, by: SlotArg) -> PyResult<Self> {
        Ok(def(Def::Spin {
            pose: pose.read(),
            about: about.read(),
            by: scalar(py, PoseSlot::Spin, &by)?,
        }))
    }

    /// **The point at `(x, y, z)` in `frame`**: the frame's origin slid
    /// along its x, its y and its z axis, each by its length.
    #[staticmethod]
    fn point_in(
        py: Python<'_>,
        frame: OperandArg,
        x: SlotArg,
        y: SlotArg,
        z: SlotArg,
    ) -> PyResult<Self> {
        let at = [
            scalar(py, PoseSlot::Slide, &x)?,
            scalar(py, PoseSlot::Slide, &y)?,
            scalar(py, PoseSlot::Slide, &z)?,
        ];
        Ok(def(Def::point_in(&frame.read(), at)))
    }

    /// **The frame through an axis and a point**: origin the foot of
    /// the point on the axis, `z` the axis, `x` towards the point. A
    /// point on the axis refuses `pose_degenerate` at `evaluate`.
    #[staticmethod]
    fn through(axis: PoseArg, point: PoseArg) -> Self {
        def(Def::Through {
            axis: axis.read(),
            point: point.read(),
        })
    }

    /// **The axis two planes meet in**, along `a`'s normal crossed with
    /// `b`'s. Parallel planes refuse `pose_degenerate` at `evaluate`.
    #[staticmethod]
    fn meet(a: PoseArg, b: PoseArg) -> Self {
        def(Def::Meet {
            a: a.read(),
            b: b.read(),
        })
    }

    /// **The opposite sense** of a direction, an axis, a plane or a
    /// frame. A flip of a flip is the pose it flips: the edit door
    /// normalises it there, and a point, which has no sense, refuses.
    #[staticmethod]
    fn flip(pose: PoseArg) -> Self {
        def(Def::Flip { pose: pose.read() })
    }

    fn __repr__(&self) -> String {
        format!("PoseDef({})", self.inner)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PoseDef>()?;
    m.add_class::<RoleRead>()?;
    Ok(())
}
