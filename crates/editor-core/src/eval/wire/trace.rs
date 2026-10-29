//! **A profile drawn on a frame, read at its lane** (D10, "Constructing
//! a coincidence"; [`crate::trace`]): the frame, in its space's
//! coordinates, and each traced face's carrier, with the maps between
//! the sketch's coordinates and its body's.
//!
//! The profile's first pass resolves at `f64`, where its structure is
//! chosen. A trace is read off the lane's bodies, so at a lane whose
//! values are enclosures the first pass reads each carrier at its
//! bracket's midpoint: it only proposes the structure there, and the
//! lane's own pass, guided by it, re-decides every choice at the lane
//! and refuses one it cannot reproduce (`profile::replay_guided`). At a
//! point lane the midpoint is the value.

use std::sync::Arc;

use geom::Surface;
use geom_core::{Affine3, Bounds, Decide, Mat3, Point3, RigidMotion, Tol, Vec3};

use super::{Results, band, read_named_body, select_one};
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

impl<T: Decide + Bounds> Drawn<T> {
    /// **The traced faces as the first pass reads them**, at `f64`:
    /// each value its bracket's midpoint (module docs).
    ///
    /// # Errors
    ///
    /// A map whose midpoints are not rigid within the band.
    pub(crate) fn nominal(&self, tol: Tol) -> Result<Vec<TracedFace<f64>>, NodeErrorKind> {
        let band = band(tol)?;
        let motion = |m: &RigidMotion<T>| {
            let a = m.to_affine();
            let map = Affine3 {
                linear: Mat3 {
                    c0: vec(a.linear.c0),
                    c1: vec(a.linear.c1),
                    c2: vec(a.linear.c2),
                },
                translation: vec(a.translation),
            };
            RigidMotion::check_rigid(&map, band).map_err(NodeErrorKind::Motion)
        };
        self.faces
            .iter()
            .map(|face| {
                Ok(TracedFace {
                    surface: face.surface.as_ref().map(surface).map_err(|&kind| kind),
                    sense: face.sense,
                    sketch_to_body: motion(&face.sketch_to_body)?,
                    body_to_sketch: motion(&face.body_to_sketch)?,
                })
            })
            .collect()
    }
}

/// A value's bracket's midpoint: the value itself at a point scalar.
fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

fn vec<T: Bounds>(v: Vec3<T>) -> Vec3<f64> {
    Vec3::new(mid(v.x), mid(v.y), mid(v.z))
}

fn point<T: Bounds>(p: Point3<T>) -> Point3<f64> {
    Point3::new(mid(p.x), mid(p.y), mid(p.z))
}

/// A carrier with a closed form, or the kind of one without.
fn closed<T: Decide>(s: Surface<T>) -> Result<Surface<T>, geom::SurfaceKind> {
    match s {
        Surface::Nurbs(_) | Surface::Approx(_) => Err(s.kind()),
        closed => Ok(closed),
    }
}

/// `s`, a carrier with a closed form, at its midpoints.
fn surface<T: Decide + Bounds>(s: &Surface<T>) -> Surface<f64> {
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: point(origin),
            normal: vec(normal),
            u_ref: vec(u_ref),
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: point(origin),
            axis: vec(axis),
            radius: mid(radius),
            u_ref: vec(u_ref),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: point(apex),
            axis: vec(axis),
            half_angle: mid(half_angle),
            u_ref: vec(u_ref),
        },
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center: point(center),
            radius: mid(radius),
            axis: vec(axis),
            u_ref: vec(u_ref),
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Torus {
            center: point(center),
            axis: vec(axis),
            major_radius: mid(major_radius),
            minor_radius: mid(minor_radius),
            u_ref: vec(u_ref),
        },
        Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("a traced carrier with no closed form is held as its kind")
        }
    }
}
