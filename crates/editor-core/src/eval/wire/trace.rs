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
use crate::pose::{PoseReadFault, PoseValue, RoleRead};
use crate::program::ProfileProgram;
use crate::trace::TracedFace;
use crate::{OperandSlot, VarKind};

/// **A drawn profile, read at the lane**: its frame and its traced
/// faces.
pub(crate) struct Drawn<T: Decide> {
    /// The frame, in its space's coordinates: the sketch's plane is its
    /// `xy`.
    pub frame: RigidMotion<T>,
    /// The traced faces, by place.
    pub faces: Vec<TracedFace<T>>,
}

/// **`program`'s frame and traced faces, read at the lane scalar**;
/// `None` for a free profile.
///
/// # Errors
///
/// The frame's or a face's read refusing, as any pose or selection read
/// refuses; a carrier the face's body cannot read back.
pub(crate) fn drawn<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    program: &ProfileProgram,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Option<Drawn<T>>, NodeErrorKind> {
    let Some(on) = &program.on else {
        return Ok(None);
    };
    let read = RoleRead {
        read: on.frame,
        role: None,
    };
    let PoseValue::Frame(frame) =
        super::pose::eval_pose(doc, results, OperandSlot::Frame, read, env, tol)?
    else {
        unreachable!("the frame seat admits a frame alone")
    };
    let sketch = Frame::Map(Arc::new(frame));
    let identity = RigidMotion::identity;
    let faces = (0u32..)
        .zip(&on.faces)
        .map(|(at, &var)| {
            let slot = OperandSlot::Traced(at);
            let body_var = doc.selection(var).map_or(var, |select| select.body);
            let projected = super::project_ports(&[body_var], doc, results)?;
            let results = projected.as_ref().unwrap_or(results);
            let (node, ent) = select_one(doc, results, slot, var)?;
            let whole = doc
                .selection(var)
                .is_some_and(|select| crate::repetition::names_members(doc, select));
            let framed = read_named_body(results, node, ent.body, whole)?;
            let Some(face) = names::EntityKey::face(ent.key) else {
                unreachable!("the traced seat admits a face alone")
            };
            let (surface, sense) =
                topo::readback::face_carrier(framed.body(), face).map_err(|error| {
                    NodeErrorKind::PoseRead {
                        pose: VarKind::Face,
                        fault: PoseReadFault::Readback { error },
                    }
                })?;
            let body = framed.frame();
            Ok(TracedFace {
                surface: closed(surface),
                sense,
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
    Ok(Some(Drawn { frame, faces }))
}

/// A carrier with a closed form, or the kind of one without.
fn closed<T: Decide>(s: Surface<T>) -> Result<Surface<T>, geom::SurfaceKind> {
    match s {
        Surface::Nurbs(_) | Surface::Approx(_) => Err(s.kind()),
        closed => Ok(closed),
    }
}
