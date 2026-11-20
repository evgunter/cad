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
/// ([`super::pose::on_frame`]).
#[derive(Debug, Clone)]
pub(crate) enum OnFrame<T: geom_core::Real> {
    /// A frame pose, bound where it is read.
    Posed(RigidMotion<T>),
    /// A face's chart, decided at each carriage over what it carries,
    /// and where its seat moves it, that move in its coordinates.
    Chart {
        /// The chart.
        chart: crate::mate::value::FaceChart<T>,
        /// The move.
        moved: Option<RigidMotion<T>>,
    },
}

impl<T: crate::mate::SolveScalar> OnFrame<T> {
    /// **The frame, carrying what reaches `reach` from its origin**: a
    /// face's chart carried over it ([`crate::mate::value::FaceChart::carry`]),
    /// and over how far its move stands that origin from the chart's own,
    /// then moved; a frame pose as it was bound.
    ///
    /// # Errors
    ///
    /// The chart's refusal of that carriage.
    pub(crate) fn carry(
        &self,
        reach: Result<f64, crate::mate::LeverRefusal>,
        tol: Tol,
    ) -> Result<RigidMotion<T>, NodeErrorKind> {
        match self {
            Self::Posed(frame) => Ok(*frame),
            Self::Chart { chart, moved } => {
                let off = moved.as_ref().map_or(0.0, |m| {
                    crate::mate::SolveScalar::upper(
                        (m.origin() - geom_core::Point3::origin()).norm(),
                    )
                });
                let carried = chart.carry(reach.map(|r| r + off), super::band(tol)?, tol)?;
                match moved {
                    Some(m) => carried.compose(*m).map_err(NodeErrorKind::Motion),
                    None => Ok(carried),
                }
            }
        }
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
            let cover = topo::face_cover(framed.body(), face, band);
            let reach = reach(&framed, (face, surface.kind()), &cover, node);
            Ok((framed, surface, sense, cover, reach))
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
    let sketch = Frame::Map(Arc::new(frame.carry(farthest, tol)?));
    let identity = RigidMotion::identity;
    let faces = read
        .into_iter()
        .map(|(framed, surface, sense, cover, _)| {
            let body = framed.frame();
            Ok(TracedFace {
                surface: closed(surface),
                sense,
                cover,
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
    Ok(Some(Drawn { on: frame, faces }))
}

/// **How far a traced face reaches from its space's origin**, as its
/// cover reads it: the farthest the cover stands from where its body's
/// frame puts that origin.
fn reach<T: topo::AtRestPolicy + crate::mate::SolveScalar>(
    framed: &crate::eval::Framed<T>,
    (face, kind): (topo::entity::FaceKey, geom::SurfaceKind),
    cover: &topo::FaceCovered<T>,
    node: crate::RecipeNodeId,
) -> Result<f64, crate::mate::LeverRefusal> {
    let refused = |refusal| crate::mate::LeverRefusal::Shape {
        shape: node,
        refusal,
    };
    let origin = framed
        .space_origin()
        .map_err(|_| refused(crate::mate::ReachRefusal::NoFiniteBound))?;
    let cover = cover
        .as_ref()
        .map_err(|_| refused(crate::mate::ReachRefusal::FaceUnbounded { face, kind }))?;
    let (_, far) = cover.distance_range(&geom::cover::Site::Point(origin));
    let far = crate::mate::SolveScalar::upper(far);
    if far.is_finite() {
        Ok(far)
    } else {
        Err(refused(crate::mate::ReachRefusal::NoFiniteBound))
    }
}

/// A carrier with a closed form, or the kind of one without.
fn closed<T: Decide>(s: Surface<T>) -> Result<Surface<T>, geom::SurfaceKind> {
    match s {
        Surface::Nurbs(_) | Surface::Approx(_) => Err(s.kind()),
        closed => Ok(closed),
    }
}
