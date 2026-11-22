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
/// `None` for a free profile. The traced faces stand in the frame (their
/// maps into the sketch's coordinates are its), so the frame carries
/// them, over the farthest any of them reaches from its space's origin:
/// the frame's own origin stands within its slides and the face's
/// distance from there, which the arm adds.
///
/// # Errors
///
/// The frame's or a face's read refusing, as any pose or selection read
/// refuses; a carrier the face's body cannot read back; the frame's
/// refusal to carry the traced faces.
pub(crate) fn drawn<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    program: &ProfileProgram,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Option<Drawn<T>>, NodeErrorKind> {
    let Some(on) = &program.on else {
        return Ok(None);
    };
    let frame = super::pose::on_frame(doc, results, on.frame, env, tol)?;
    let band = super::band(tol)?;
    let read = on
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
            let reach = reach(&framed, face, node, band);
            Ok((framed, face, surface, sense, reach))
        })
        .collect::<Result<Vec<_>, NodeErrorKind>>()?;
    if read.is_empty() {
        return Ok(Some(Drawn {
            on: frame,
            faces: Vec::new(),
        }));
    }
    let farthest = read
        .iter()
        .try_fold(0.0_f64, |held, (.., r)| r.clone().map(|r| held.max(r)));
    let drawn = frame.carrying(farthest, tol)?;
    let sketch = drawn.frame();
    let identity = RigidMotion::identity;
    let faces = read
        .into_iter()
        .map(|(framed, face, surface, sense, _)| {
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

/// **How far a traced face reaches from its space's origin**: its cover
/// from where its body's frame puts that origin, as a placed copy's
/// faces are read ([`crate::mate::reach`]).
fn reach<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    framed: &crate::eval::Framed<T>,
    face: topo::entity::FaceKey,
    node: crate::RecipeNodeId,
    band: geom_core::Band,
) -> Result<f64, crate::mate::LeverRefusal> {
    use crate::mate::ReachRefusal;
    framed
        .space_origin()
        .map_err(|_| ReachRefusal::NoFiniteBound)
        .and_then(|origin| {
            crate::mate::reach::lever_reach(
                crate::mate::reach::faces_reach_from(framed.body(), [face], origin, band)
                    .and_then(|r| r.ok_or(ReachRefusal::NoExtent)),
            )
        })
        .map_err(|refusal| crate::mate::LeverRefusal::Shape {
            shape: node,
            refusal,
        })
}

/// A carrier with a closed form, or the kind of one without.
fn closed<T: Decide>(s: Surface<T>) -> Result<Surface<T>, geom::SurfaceKind> {
    match s {
        Surface::Nurbs(_) | Surface::Approx(_) => Err(s.kind()),
        closed => Ok(closed),
    }
}
