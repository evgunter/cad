//! **A profile drawn on a frame, read at its lane** (D10, "Constructing
//! a coincidence"; [`crate::trace`]): the frame, in its space's
//! coordinates, and each traced face's carrier, with the maps between
//! the sketch's coordinates and its body's.
//!
//! The profile's first pass resolves at `f64` over the nominal, where
//! its structure is chosen, so it reads its traces off the nominal
//! document's bodies ([`super::LaneEnv::nominal_results`]); the lane's
//! own pass reads them off this run's and replays guided by the first.

use std::sync::Arc;

use geom::Surface;
use geom_core::{Decide, RigidMotion, Tol};

use super::{Results, read_named_body, select_one};
use crate::eval::{Frame, NodeErrorKind};
use crate::expr::VarEnv;
use crate::names;
use crate::pose::PoseReadFault;
use crate::program::ProfileProgram;
use crate::trace::TracedFace;
use crate::{OperandSlot, VarKind};

/// **The frame a profile is drawn on, as its seat reads it**
/// ([`super::pose::on_frame`]): a face's chart yet to carry anything, or
/// a frame already decided.
#[derive(Debug, Clone)]
pub(crate) enum OnFrame<T: geom_core::Real> {
    /// A face's chart, undecided.
    Chart(crate::mate::value::FaceChart<T>),
    /// A decided frame.
    Drawn(DrawnFrame<T>),
}

impl<T: crate::mate::SolveScalar> OnFrame<T> {
    /// **The frame, carrying what reaches `reach` from its origin**: a
    /// face's chart minted at this, its first, carriage; a decided frame
    /// decided over it too.
    ///
    /// # Errors
    ///
    /// The chart's refusal of the carriage.
    pub(crate) fn carrying(
        self,
        reach: Result<f64, crate::mate::LeverRefusal>,
        tol: Tol,
    ) -> Result<DrawnFrame<T>, NodeErrorKind> {
        match self {
            Self::Chart(chart) => Ok(DrawnFrame::Charted(chart.mint(
                reach,
                super::band(tol)?,
                tol,
            )?)),
            Self::Drawn(drawn) => {
                drawn.carry(reach, tol)?;
                Ok(drawn)
            }
        }
    }
}

/// **A decided frame a profile is drawn on**: a frame pose, bound where
/// it is read, or a face's chart, minted at its first carriage. Every
/// value computed in it holds its one map.
#[derive(Debug, Clone)]
pub(crate) enum DrawnFrame<T: geom_core::Real> {
    /// A frame pose.
    Posed(Arc<RigidMotion<T>>),
    /// A face's chart.
    Charted(crate::mate::value::MintedChart<T>),
}

impl<T: geom_core::Real> DrawnFrame<T> {
    /// The frame's map.
    pub(crate) fn map(&self) -> &Arc<RigidMotion<T>> {
        match self {
            Self::Posed(map) => map,
            Self::Charted(chart) => chart.motion(),
        }
    }

    /// The frame.
    pub(crate) fn frame(&self) -> Frame<T> {
        Frame::Map(Arc::clone(self.map()))
    }
}

impl<T: crate::mate::SolveScalar> DrawnFrame<T> {
    /// **The frame, carrying what reaches `reach` from its origin**: a
    /// face's chart's verdict over that reach, and its one map.
    ///
    /// # Errors
    ///
    /// The chart's refusal of the carriage.
    pub(crate) fn carry(
        &self,
        reach: Result<f64, crate::mate::LeverRefusal>,
        tol: Tol,
    ) -> Result<Frame<T>, NodeErrorKind> {
        if let Self::Charted(chart) = self {
            chart.carries(reach, super::band(tol)?, tol)?;
        }
        Ok(self.frame())
    }
}

/// **A drawn profile, read at the lane**: the frame it is drawn on and
/// its traced faces.
pub(crate) struct Drawn<T: geom_core::Real> {
    /// The frame, in its space's coordinates: the sketch's plane is its
    /// `xy`.
    pub on: OnFrame<T>,
    /// The traced faces, by place.
    pub faces: Vec<TracedFace<T>>,
}

/// **`program`'s frame and traced faces, read at the lane scalar**;
/// `None` for a free profile. A traced face is read, not carried: the
/// turn never moves it, and the trace reads it through the turn once
/// that turn's pole is decided ([`crate::mate::value::FaceChart::select`]).
///
/// # Errors
///
/// The frame's or a face's read refusing, as any pose or selection read
/// refuses; a carrier the face's body cannot read back; the chart's
/// pole refused over the session box.
pub(crate) fn drawn<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    program: &ProfileProgram,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Option<Drawn<T>>, NodeErrorKind> {
    drawn_in(doc, results, program, None, env, tol)
}

/// [`drawn`], its faces read through `decided`, the frame the profile's
/// own carriage decided, where it is in hand: a reader after the profile
/// reads its frame, never selects it again.
///
/// # Errors
///
/// As [`drawn`].
pub(crate) fn drawn_in<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    program: &ProfileProgram,
    decided: Option<&DrawnFrame<T>>,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Option<Drawn<T>>, NodeErrorKind> {
    let Some(on) = &program.on else {
        return Ok(None);
    };
    let frame = match decided {
        Some(decided) => OnFrame::Drawn(decided.clone()),
        None => super::pose::on_frame(doc, results, on.frame, env, tol)?,
    };
    let band = super::band(tol)?;
    if on.faces.is_empty() {
        return Ok(Some(Drawn {
            on: frame,
            faces: Vec::new(),
        }));
    }
    // The trace reads the faces through the frame's turn before the
    // loops it is consumed over resolve: a face's chart is selected here,
    // its pole decided over the session box, and the profile's carriage
    // decides the rest over the loops.
    let drawn = match frame {
        OnFrame::Chart(chart) => DrawnFrame::Charted(chart.select(band)?),
        OnFrame::Drawn(drawn) => drawn,
    };
    let sketch = drawn.frame();
    let identity = RigidMotion::identity;
    let faces = on
        .faces
        .iter()
        .enumerate()
        .map(|(at, &var)| {
            let slot = OperandSlot::Traced(at);
            let projected = super::project_selection(doc, slot, var, results, env)?;
            let results = projected.as_ref().unwrap_or(results);
            let (node, ent) = select_one(doc, results, slot, var, env)?;
            let framed = read_named_body(results, node, ent.body)?;
            let Some(face) = names::EntityKey::face(ent.key) else {
                unreachable!("the traced seat admits a face alone")
            };
            let (surface, sense) = topo::readback::face_stored_surface(framed.body(), face)
                .map_err(|error| NodeErrorKind::PoseRead {
                    pose: VarKind::Face,
                    fault: PoseReadFault::Readback { error },
                })?;
            let body = framed.frame();
            Ok(TracedFace {
                surface: closed(surface),
                sense,
                cover: topo::face_cover(framed.body(), face, band),
                sketch_to_body: body
                    .relative(&sketch)
                    .map_err(NodeErrorKind::Motion)?
                    .unwrap_or_else(identity),
                body_to_sketch: sketch
                    .relative(body)
                    .map_err(NodeErrorKind::Motion)?
                    .unwrap_or_else(identity),
            })
        })
        .collect::<Result<_, NodeErrorKind>>()?;
    Ok(Some(Drawn {
        on: OnFrame::Drawn(drawn),
        faces,
    }))
}

/// A carrier with a closed form, or the kind of one without.
fn closed<T: Decide>(s: Surface<T>) -> Result<Surface<T>, geom::SurfaceKind> {
    match s {
        Surface::Nurbs(_) | Surface::Approx(_) => Err(s.kind()),
        closed => Ok(closed),
    }
}
