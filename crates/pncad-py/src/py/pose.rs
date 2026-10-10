//! **A pose defined at its reader's seat** (D10): `PoseDef`, the Python
//! spelling of `editor_core::pose::PoseDef` authored as an operand.
//!
//! No pose is free and none is defined from nothing: each constructor
//! reads geometry (a face as a plane, a carrier's axis or centre, a
//! vertex's point), writes coordinates in a frame the definition reads,
//! or constructs from other poses. A `PoseDef` is passed where an
//! operand is read; the edit door mints the pose variable there, as it
//! mints a selection authored at a seat. To share one between two
//! readers, read it back with `Doc.output`'s sibling, name it, and read
//! it by name.

use pyo3::prelude::*;

use pncad::document as d;
use pncad::document::pose::{PoseCoords, PoseDef as Def, PoseSlot};

use super::doc::{OperandArg, name_from_text};
use super::expr::SlotArg;

/// **A pose defined where it is read**: pass it to an operand that reads
/// a pose (a split's `tool`, a profile's frame, a circular rule's axis).
#[pyclass(module = "pncad", frozen, from_py_object)]
#[derive(Clone)]
pub(crate) struct PoseDef {
    /// The authored operand: a pose definition, or for a flip of a flip
    /// the pose it flips back to.
    pub(crate) inner: d::Operand,
}

fn scalar(py: Python<'_>, slot: PoseSlot, arg: &SlotArg) -> PyResult<d::Formula> {
    let expected = slot.dimension();
    let formula = arg.formula(py, expected)?;
    if formula.dim() == expected {
        return Ok(formula);
    }
    Err(pyo3::exceptions::PyValueError::new_err(format!(
        "the pose's {} reads a {}, not a {}",
        slot.label(),
        expected,
        formula.dim()
    )))
}

fn triple(
    py: Python<'_>,
    at: fn(d::Axis3) -> PoseSlot,
    xs: &(SlotArg, SlotArg, SlotArg),
) -> PyResult<[d::Formula; 3]> {
    Ok([
        scalar(py, at(d::Axis3::X), &xs.0)?,
        scalar(py, at(d::Axis3::Y), &xs.1)?,
        scalar(py, at(d::Axis3::Z), &xs.2)?,
    ])
}

fn select(at: OperandArg, name: &str) -> PyResult<d::Operand> {
    Ok(d::Operand::select(at.read(), vec![name_from_text(name)?]))
}

fn def(inner: Def<d::Operand, d::Formula>) -> PoseDef {
    PoseDef {
        inner: d::Operand::Pose(Box::new(inner)),
    }
}

/// The kind words a projection names.
fn kind_of(word: &str) -> PyResult<d::VarKind> {
    Ok(match word {
        "point" => d::VarKind::Point,
        "direction" => d::VarKind::Direction,
        "axis" => d::VarKind::Axis,
        "plane" => d::VarKind::Plane,
        "frame" => d::VarKind::Frame,
        other => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "not a pose kind: {other:?} — one of \"point\", \"direction\", \"axis\", \
                 \"plane\", \"frame\""
            )));
        }
    })
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

    /// **A pose written by coordinates in a frame** the definition
    /// reads. The keywords given decide the kind: `origin` alone a
    /// point, `direction` alone a direction, `origin` and `direction`
    /// an axis, `origin` and `normal` a plane, `origin`, `u` and `v` a
    /// frame. Positions are lengths, directions dimensionless and
    /// normalized at evaluation.
    #[staticmethod]
    #[pyo3(signature = (frame, *, origin=None, direction=None, normal=None, u=None, v=None))]
    #[allow(clippy::too_many_arguments)]
    fn in_frame(
        py: Python<'_>,
        frame: OperandArg,
        origin: Option<(SlotArg, SlotArg, SlotArg)>,
        direction: Option<(SlotArg, SlotArg, SlotArg)>,
        normal: Option<(SlotArg, SlotArg, SlotArg)>,
        u: Option<(SlotArg, SlotArg, SlotArg)>,
        v: Option<(SlotArg, SlotArg, SlotArg)>,
    ) -> PyResult<Self> {
        let o = |xs: &(SlotArg, SlotArg, SlotArg)| triple(py, PoseSlot::Origin, xs);
        let coords = match (&origin, &direction, &normal, &u, &v) {
            (Some(p), None, None, None, None) => PoseCoords::Point { position: o(p)? },
            (None, Some(dir), None, None, None) => PoseCoords::Direction {
                direction: triple(py, PoseSlot::Direction, dir)?,
            },
            (Some(p), Some(dir), None, None, None) => PoseCoords::Axis {
                origin: o(p)?,
                direction: triple(py, PoseSlot::Direction, dir)?,
            },
            (Some(p), None, Some(n), None, None) => PoseCoords::Plane {
                origin: o(p)?,
                normal: triple(py, PoseSlot::Normal, n)?,
            },
            (Some(p), None, None, Some(x), Some(y)) => PoseCoords::Frame {
                origin: o(p)?,
                u: triple(py, PoseSlot::U, x)?,
                v: triple(py, PoseSlot::V, y)?,
            },
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "a pose in a frame is `origin` (a point), `direction` (a direction), \
                     `origin` and `direction` (an axis), `origin` and `normal` (a plane), or \
                     `origin`, `u` and `v` (a frame)",
                ));
            }
        };
        Ok(def(Def::InFrame {
            frame: frame.read(),
            coords,
        }))
    }

    /// **The frame through an axis and a point**: origin the foot of
    /// the point on the axis, `z` the axis, `x` towards the point. A
    /// point on the axis refuses `pose_degenerate` at `evaluate`.
    #[staticmethod]
    fn through(axis: OperandArg, point: OperandArg) -> Self {
        def(Def::Through {
            axis: axis.read(),
            point: point.read(),
        })
    }

    /// **The axis two planes meet in**, along `a`'s normal crossed with
    /// `b`'s. Parallel planes refuse `pose_degenerate` at `evaluate`.
    #[staticmethod]
    fn meet(a: OperandArg, b: OperandArg) -> Self {
        def(Def::Meet {
            a: a.read(),
            b: b.read(),
        })
    }

    /// **The opposite sense** of a direction, an axis, a plane or a
    /// frame. A flip of a flip defined here is the pose it flips, so
    /// that is what comes back.
    #[staticmethod]
    fn flip(pose: OperandArg) -> Self {
        match pose.read() {
            d::Operand::Pose(inner) if matches!(*inner, Def::Flip { .. }) => {
                let Def::Flip { pose } = *inner else {
                    unreachable!("matched a flip")
                };
                Self { inner: pose }
            }
            read => def(Def::Flip { pose: read }),
        }
    }

    /// **A plane moved along its own normal** by the length `by`.
    #[staticmethod]
    fn standoff(py: Python<'_>, plane: OperandArg, by: SlotArg) -> PyResult<Self> {
        Ok(def(Def::Standoff {
            plane: plane.read(),
            by: scalar(py, PoseSlot::By, &by)?,
        }))
    }

    /// **A coarser pose of a finer one**: a frame's `"plane"`, `"axis"`
    /// or `"point"`, or a plane's or an axis's `"direction"`.
    #[staticmethod]
    fn project(of: OperandArg, to: &str) -> PyResult<Self> {
        Ok(def(Def::Project {
            of: of.read(),
            to: kind_of(to)?,
        }))
    }

    fn __repr__(&self) -> String {
        format!("PoseDef({})", self.inner)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PoseDef>()?;
    Ok(())
}
