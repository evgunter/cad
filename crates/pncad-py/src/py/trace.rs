//! **A profile drawn on a frame, tracing faces** (`pncad::trace`): a
//! chain written in the document's own vocabulary, and the profile node
//! drawn on a frame of a body's space that reads it. Every method
//! appends the step `pncad::trace::Chain` names; the words are read by
//! `pncad::trace::passage` and `sense`, each argument's dimension is
//! checked by `pncad::trace::drawn_profile`, and the chain by the
//! document, at `evaluate`. This module only turns Python values into
//! the façade's.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use pncad::document as d;
use pncad::trace as t;

use super::doc::{Node, OperandArg, edit_err, name_from_text};
use super::expr::SlotArg;
use super::path::{ArcSweep, StartToken};

/// **Where a piece of a traced chain runs to**, beyond a point and the
/// start: a crossing of a face's trace (`TraceTarget.on_trace`), or the
/// point along the heading at a written sketch coordinate
/// (`TraceTarget.at_x`, `TraceTarget.at_y`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct TraceTarget(d::ProgramTarget<d::Formula>);

/// What a traced chain's piece runs to.
#[derive(FromPyObject)]
pub(crate) enum ChainTarget {
    /// A crossing or a coordinate.
    Trace(TraceTarget),
    /// The chain's start: the piece closes it.
    Start(StartToken),
    /// A point of the sketch.
    Point((SlotArg, SlotArg)),
}

/// A word the façade could not read, raised as Python's.
fn word_err(err: &t::TraceWordError) -> PyErr {
    PyValueError::new_err(err.to_string())
}

/// The value written for an argument `arg`, as a formula of its
/// dimension; `pncad::trace::drawn_profile` checks it at its slot.
fn at(py: Python<'_>, arg: d::StepArg, value: &SlotArg) -> PyResult<d::Formula> {
    value.formula(py, arg.dimension())
}

#[pymethods]
impl TraceTarget {
    /// The `nth` (from 1) crossing where the piece enters, or leaves,
    /// the material of traced face `face` (its place in the profile's
    /// `faces`).
    #[staticmethod]
    #[pyo3(signature = (face, passage, nth = 1))]
    fn on_trace(face: u32, passage: &str, nth: u32) -> PyResult<Self> {
        Ok(Self(d::ProgramTarget::OnTrace {
            face,
            crossing: d::TraceCrossing {
                passage: t::passage(passage).map_err(|e| word_err(&e))?,
                nth,
            },
        }))
    }

    /// The point along the heading whose sketch `x` is `x`.
    #[staticmethod]
    fn at_x(py: Python<'_>, x: SlotArg) -> PyResult<Self> {
        Ok(Self(d::ProgramTarget::AtX(
            x.formula(py, d::Dimension::Length)?,
        )))
    }

    /// The point along the heading whose sketch `y` is `y`.
    #[staticmethod]
    fn at_y(py: Python<'_>, y: SlotArg) -> PyResult<Self> {
        Ok(Self(d::ProgramTarget::AtY(
            y.formula(py, d::Dimension::Length)?,
        )))
    }

    fn __repr__(&self) -> String {
        format!("TraceTarget({:?})", self.0)
    }
}

/// **A chain written step by step** (`pncad::trace::Chain`): start it
/// with `Chain.at`; each method answers the chain with one more step.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Chain(pub(crate) t::Chain);

impl Chain {
    fn target(
        &self,
        py: Python<'_>,
        target: ChainTarget,
    ) -> PyResult<d::ProgramTarget<d::Formula>> {
        Ok(match target {
            ChainTarget::Trace(trace) => trace.0,
            ChainTarget::Start(_) => d::ProgramTarget::Start,
            ChainTarget::Point((x, y)) => d::ProgramTarget::Point([
                at(py, d::StepArg::TargetX, &x)?,
                at(py, d::StepArg::TargetY, &y)?,
            ]),
        })
    }
}

#[pymethods]
impl Chain {
    /// A chain starting at `(x, y)`.
    #[staticmethod]
    fn at(py: Python<'_>, x: SlotArg, y: SlotArg) -> PyResult<Self> {
        Ok(Self(t::Chain::at(
            at(py, d::StepArg::PointX, &x)?,
            at(py, d::StepArg::PointY, &y)?,
        )))
    }

    /// The departure as an angle from the sketch's `x`.
    fn angle(&self, py: Python<'_>, theta: SlotArg) -> PyResult<Self> {
        let theta = at(py, d::StepArg::AngleVal, &theta)?;
        Ok(Self(self.0.clone().angle(theta)))
    }

    /// The departure as components, their ratio read.
    fn toward(&self, py: Python<'_>, dx: SlotArg, dy: SlotArg) -> PyResult<Self> {
        let dx = at(py, d::StepArg::DirX, &dx)?;
        let dy = at(py, d::StepArg::DirY, &dy)?;
        Ok(Self(self.0.clone().toward(dx, dy)))
    }

    /// A straight leg of `length` along the heading.
    fn line(&self, py: Python<'_>, length: SlotArg) -> PyResult<Self> {
        let length = at(py, d::StepArg::Length, &length)?;
        Ok(Self(self.0.clone().line(length)))
    }

    /// A straight leg to a point `(x, y)`, to `Start`, or, along a
    /// director, to a `TraceTarget`.
    fn line_to(&self, py: Python<'_>, target: ChainTarget) -> PyResult<Self> {
        let target = self.target(py, target)?;
        Ok(Self(self.0.clone().line_to(target)))
    }

    /// The straight continuation of the previous piece, to a point,
    /// `Start` or a `TraceTarget`.
    fn continue_to(&self, py: Python<'_>, target: ChainTarget) -> PyResult<Self> {
        let target = self.target(py, target)?;
        Ok(Self(self.0.clone().continue_to(target)))
    }

    /// An arc about `centre`, turning `winding`, to a point, `Start` or
    /// a crossing.
    fn arc_about(
        &self,
        py: Python<'_>,
        centre: (SlotArg, SlotArg),
        winding: ArcSweep,
        target: ChainTarget,
    ) -> PyResult<Self> {
        let centre = [
            at(py, d::StepArg::CenterX, &centre.0)?,
            at(py, d::StepArg::CenterY, &centre.1)?,
        ];
        let target = self.target(py, target)?;
        Ok(Self(self.0.clone().arc_about(
            centre,
            winding.to_kernel(),
            target,
        )))
    }

    /// The heading along the trace the previous piece ran onto:
    /// `"with"` it (the face's material on the left) or `"against"` it.
    fn along_trace(&self, sense: &str) -> PyResult<Self> {
        let sense = t::sense(sense).map_err(|e| word_err(&e))?;
        Ok(Self(self.0.clone().along_trace(sense)))
    }

    /// How many steps the chain holds.
    fn __len__(&self) -> usize {
        self.0.steps().len()
    }

    fn __repr__(&self) -> String {
        format!("Chain({} steps)", self.0.steps().len())
    }
}

/// A traced chain, or a list of them, outer first.
#[derive(FromPyObject)]
pub(crate) enum Outline {
    /// One loop.
    One(Chain),
    /// Several, outer first.
    Many(Vec<Chain>),
}

/// **A profile drawn on `frame`, tracing `faces`** (`Node.drawn_profile`).
pub(crate) fn drawn_profile(
    py: Python<'_>,
    frame: OperandArg,
    faces: Vec<(OperandArg, String)>,
    outline: Outline,
) -> PyResult<Node> {
    let faces = faces
        .into_iter()
        .map(|(body, name)| {
            Ok(d::Operand::select(
                body.read(),
                vec![name_from_text(&name)?],
            ))
        })
        .collect::<PyResult<Vec<_>>>()?;
    let loops = match outline {
        Outline::One(chain) => vec![chain.0],
        Outline::Many(chains) => chains.into_iter().map(|c| c.0).collect(),
    };
    Ok(Node {
        inner: t::drawn_profile(frame.read(), faces, loops).map_err(|e| edit_err(py, &e))?,
    })
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Chain>()?;
    m.add_class::<TraceTarget>()?;
    Ok(())
}
