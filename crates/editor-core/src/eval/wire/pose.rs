//! **The pose evaluator** (D10; FORK-S3P, FORK-1b): every pose a node
//! reads, bound at its reader at the lane scalar.
//!
//! A pose's value can depend on built geometry (a face read as a plane
//! reads a `Face`), so a pose definition is evaluated when its reader
//! is, after every operation it reaches ([`crate::Doc::read_ports`] puts
//! them upstream of the reader). One door, [`eval_pose`], answers an
//! operation's pose output (a datum's, a revolve's axis) and a pose
//! definition alike; a definition reading another pose reads it through
//! the same door.
//!
//! Every read off geometry copies a stored fact out — a carrier's tag,
//! its distinguished point, its axis, a face's sense — and decides
//! nothing. The constructions decide one length each, under the pose
//! direction funnel ([`DATUM_UNIT_NORM`]), and refuse its zero as their
//! degenerate case ([`PoseConstruction`]).

use geom_brep::OutwardNormal;
use geom_core::{Decide, OrthoFrame, Point3, Tol, UnitVec3, UnitVec3Error, Vec3};
use topo::DATUM_UNIT_NORM;

use super::{
    DATUM_AXIS_ROLE, DirectionRefusal, PLANE_NORMAL_ROLE, Results, band, datum_unit, frame_axes,
    read_body, refusal, select, slots, value_of,
};
use crate::eval::{NodeErrorKind, ValuePayload};
use crate::expr::{VarEnv, eval_var};
use crate::names;
use crate::node::{Node, SlotId};
use crate::mate::PoseSymmetry;
use crate::pose::{Carrier, PoseConstruction, PoseCoords, PoseDef, PoseReadFault, PoseValue};
use crate::program::ProfileProgram;
use crate::var::{VarDef, VarId, VarKind};
use crate::{OperandSlot, VarKind as K};

/// The role word a pose direction written in a frame is normalized
/// under.
const POSE_DIRECTION_ROLE: &str = "pose direction";

/// **The pose `var` the node reads at `slot`, evaluated** at the lane
/// scalar: an operation's pose output read off its value, or a pose
/// definition bound here. Its symmetry is its kind's (the door asserts
/// the two tables agree).
///
/// # Errors
///
/// The read unresolved or of no pose; a pose read off geometry that
/// finds none ([`NodeErrorKind::PoseRead`]); a construction's
/// degenerate case ([`NodeErrorKind::PoseDegenerate`]); a direction the
/// direction door refuses; a scalar that does not evaluate.
pub(crate) fn eval_pose<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    var: VarId,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let Some(held) = doc.var(var) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    let value = match held.def() {
        VarDef::Output { node, port } => output(doc, results, *node, *port, env, tol)?,
        VarDef::Pose(def) => definition(doc, results, slot, def, env, tol)?,
        VarDef::Free(_) | VarDef::Defined(_) | VarDef::Select(_) => {
            return Err(NodeErrorKind::UnresolvedRead { slot, var });
        }
    };
    assert_eq!(
        value.symmetry().family(),
        held.kind().symmetry(),
        "a pose value's subgroup is its kind's"
    );
    Ok(value)
}

/// **An operation's pose output**: a datum's value, or a revolve's
/// axis, the lift of its 2-D axis line through its profile's plane.
fn output<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    node: crate::RecipeNodeId,
    port: u8,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let value = value_of(results, node)?;
    match (&value.payload, doc.node(node)) {
        (ValuePayload::Datum(pose), _) => Ok(pose.clone()),
        (_, Some(revolve @ Node::Revolve { profile, .. })) if port == 1 => {
            let at = super::super::read_at(doc, OperandSlot::Profile, *profile)?;
            let ValuePayload::Profile(drawn) = &value_of(results, at)?.payload else {
                return Err(NodeErrorKind::MissingInput { input: at });
            };
            let vals = slots::eval_slots(revolve, env)
                .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
            let origin = super::need_point2(&vals, SlotId::Origin)?;
            let dir = super::need_vec2(&vals, SlotId::Direction)?;
            let plane = drawn.validated.plane();
            let (u, v) = (plane.u(), plane.v());
            Ok(PoseValue::Axis {
                origin: plane.origin() + u * origin.x + v * origin.y,
                dir: datum_unit(u * dir.x + v * dir.y, DATUM_AXIS_ROLE, band(tol)?)
                    .map_err(DirectionRefusal::node_error)?,
            })
        }
        _ => Err(super::wrong_operand(value, node, super::super::family::DATUM)),
    }
}

/// **A pose definition, bound** at the lane scalar.
fn definition<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    def: &PoseDef,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let pose = |var: VarId| eval_pose(doc, results, slot, var, env, tol);
    let scalar = |var: VarId, dim| {
        eval_var(var, dim, env).map_err(|source| NodeErrorKind::PoseScalar { var, source })
    };
    let b = band(tol)?;
    Ok(match def {
        PoseDef::Plane { face } => read_off(doc, results, slot, *face, K::Plane, b)?,
        PoseDef::Axis { of } => read_off(doc, results, slot, *of, K::Axis, b)?,
        PoseDef::Point { of } => read_off(doc, results, slot, *of, K::Point, b)?,
        PoseDef::InFrame { frame, coords } => {
            let PoseValue::Frame(f) = pose(*frame)? else {
                unreachable!("the door admits a frame read alone at an in-frame pose")
            };
            in_frame(f, coords, &scalar, b)?
        }
        PoseDef::Through { axis, point } => {
            let (PoseValue::Axis { origin, dir }, PoseValue::Point { position }) =
                (pose(*axis)?, pose(*point)?)
            else {
                unreachable!("the door admits an axis and a point at a frame through them")
            };
            let a = dir.get();
            let along = (position - origin).dot(a);
            let foot = origin + a * along;
            OrthoFrame::from_aim_and_reference(foot, dir, position - origin, DATUM_UNIT_NORM, b)
                .map(PoseValue::Frame)
                .map_err(|e| degenerate(e.error, PoseConstruction::Through))?
        }
        PoseDef::Meet { a, b: second } => {
            let (
                PoseValue::Plane {
                    origin: oa,
                    normal: na,
                },
                PoseValue::Plane {
                    origin: ob,
                    normal: nb,
                },
            ) = (pose(*a)?, pose(*second)?)
            else {
                unreachable!("the door admits two planes at a meeting line")
            };
            let d = na.get().cross(nb.get());
            let dir = UnitVec3::new(d, DATUM_UNIT_NORM, b)
                .map_err(|e| degenerate(e, PoseConstruction::Meet))?;
            // The line's point nearest `a`'s origin: from it across `a`
            // along `d × na` to `b`. `nb · (d × na) = |d|²`, decided
            // nonzero above.
            let across = d.cross(na.get());
            let reach = nb.get().dot(ob - oa) / d.norm_squared();
            PoseValue::Axis {
                origin: oa + across * reach,
                dir,
            }
        }
        PoseDef::Flip { pose: read } => match pose(*read)? {
            PoseValue::Direction { dir } => PoseValue::Direction { dir: -dir },
            PoseValue::Axis { origin, dir } => PoseValue::Axis { origin, dir: -dir },
            PoseValue::Plane { origin, normal } => PoseValue::Plane {
                origin,
                normal: -normal,
            },
            // A half-turn about the frame's own x: `u` kept, `v` and
            // the normal reversed.
            PoseValue::Frame(f) => PoseValue::Frame(
                frame_axes(f.origin(), f.u().get(), -f.v().get(), b)
                    .map_err(DirectionRefusal::node_error)?,
            ),
            PoseValue::Point { .. } => {
                unreachable!("the door admits no point at a flip: a point has no sense")
            }
        },
        PoseDef::Standoff { plane, by } => {
            let PoseValue::Plane { origin, normal } = pose(*plane)? else {
                unreachable!("the door admits a plane alone at a standoff")
            };
            let by = scalar(*by, crate::expr::Dimension::Length)?;
            PoseValue::Plane {
                origin: origin + normal.get() * by,
                normal,
            }
        }
        PoseDef::Project { of, to } => match (pose(*of)?, *to) {
            (PoseValue::Frame(f), K::Plane) => PoseValue::Plane {
                origin: f.origin(),
                normal: f.w(),
            },
            (PoseValue::Frame(f), K::Axis) => PoseValue::Axis {
                origin: f.origin(),
                dir: f.w(),
            },
            (PoseValue::Frame(f), K::Point) => PoseValue::Point {
                position: f.origin(),
            },
            (PoseValue::Plane { normal: dir, .. } | PoseValue::Axis { dir, .. }, K::Direction) => {
                PoseValue::Direction { dir }
            }
            (from, to) => unreachable!(
                "the door admits a projection with an arm alone: {} to {to}",
                from.kind()
            ),
        },
    })
}

/// A construction's direction decision found its zero: the degenerate
/// case, refused, or an escalation as any direction's.
fn degenerate(error: UnitVec3Error, construction: PoseConstruction) -> NodeErrorKind {
    match error {
        UnitVec3Error::Degenerate | UnitVec3Error::UnderflowedLength => {
            NodeErrorKind::PoseDegenerate { construction }
        }
        other => refusal(other, POSE_DIRECTION_ROLE, DATUM_UNIT_NORM),
    }
}

/// **Coordinates in a frame**, lifted through its axes.
fn in_frame<T: Decide>(
    f: OrthoFrame<T>,
    coords: &PoseCoords,
    scalar: &impl Fn(VarId, crate::expr::Dimension) -> Result<T, NodeErrorKind>,
    band: geom_core::Band,
) -> Result<PoseValue<T>, NodeErrorKind> {
    use crate::expr::Dimension;
    let (u, v, w) = (f.u().get(), f.v().get(), f.w().get());
    let local = |xs: &[VarId; 3], dim| -> Result<Vec3<T>, NodeErrorKind> {
        Ok(Vec3::new(
            scalar(xs[0], dim)?,
            scalar(xs[1], dim)?,
            scalar(xs[2], dim)?,
        ))
    };
    let point = |xs: &[VarId; 3]| -> Result<Point3<T>, NodeErrorKind> {
        let p = local(xs, Dimension::Length)?;
        Ok(f.origin() + u * p.x + v * p.y + w * p.z)
    };
    let direction = |xs: &[VarId; 3]| -> Result<Vec3<T>, NodeErrorKind> {
        let d = local(xs, Dimension::Scalar)?;
        Ok(u * d.x + v * d.y + w * d.z)
    };
    let unit = |d: Vec3<T>, role| datum_unit(d, role, band).map_err(DirectionRefusal::node_error);
    Ok(match coords {
        PoseCoords::Point { position } => PoseValue::Point {
            position: point(position)?,
        },
        PoseCoords::Direction { direction: d } => PoseValue::Direction {
            dir: unit(direction(d)?, POSE_DIRECTION_ROLE)?,
        },
        PoseCoords::Axis {
            origin,
            direction: d,
        } => PoseValue::Axis {
            origin: point(origin)?,
            dir: unit(direction(d)?, DATUM_AXIS_ROLE)?,
        },
        PoseCoords::Plane { origin, normal } => PoseValue::Plane {
            origin: point(origin)?,
            normal: unit(direction(normal)?, PLANE_NORMAL_ROLE)?,
        },
        PoseCoords::Frame { origin, u: ux, v: vx } => PoseValue::Frame(
            frame_axes(point(origin)?, direction(ux)?, direction(vx)?, band)
                .map_err(DirectionRefusal::node_error)?,
        ),
    })
}

/// **A pose read off the geometry a selection names**: a face's plane
/// (its outward normal, DM1a), a carrier's axis or an edge's line, a
/// carrier's centre or a vertex's point. Stored facts copied out; a
/// carrier without the pose refuses typed.
fn read_off<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    of: VarId,
    pose: VarKind,
    band: geom_core::Band,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let selected = select(doc, results, slot, of)?;
    let body = read_body(results, selected.at)?;
    let Some(key) = selected.ents.first().map(|ent| ent.key) else {
        unreachable!("a singleton selection holds one entity")
    };
    let fault = |fault| NodeErrorKind::PoseRead { pose, fault };
    let readback = |error| fault(PoseReadFault::Readback { error });
    // A carrier's stored direction is unit up to its rounding, and is
    // decided as any pose direction is.
    let unit = |v: Vec3<T>, role| datum_unit(v, role, band).map_err(DirectionRefusal::node_error);
    if let Some(vertex) = names::EntityKey::vertex(key) {
        return match pose {
            K::Point => Ok(PoseValue::Point {
                position: topo::readback::vertex_point(&body, vertex).map_err(readback)?,
            }),
            _ => unreachable!("the door admits a vertex at a point read alone"),
        };
    }
    if let Some(edge) = names::EntityKey::edge(key) {
        let kind = topo::readback::edge_carrier_kind(&body, edge).map_err(readback)?;
        let carrier = Carrier::Curve(kind);
        let pose_of = topo::readback::edge_pose(&body, edge).map_err(readback)?;
        let has = |kinds: &[geom::CurveKind]| kinds.contains(&kind);
        return match pose {
            K::Axis
                if has(&[
                    geom::CurveKind::Line,
                    geom::CurveKind::Circle,
                    geom::CurveKind::Ellipse,
                ]) =>
            {
                Ok(PoseValue::Axis {
                    origin: pose_of.origin,
                    dir: unit(pose_of.axis, DATUM_AXIS_ROLE)?,
                })
            }
            K::Axis => Err(fault(PoseReadFault::NoAxis { carrier })),
            K::Point if has(&[geom::CurveKind::Circle, geom::CurveKind::Ellipse]) => {
                Ok(PoseValue::Point {
                    position: pose_of.origin,
                })
            }
            K::Point => Err(fault(PoseReadFault::NoCentre { carrier })),
            _ => unreachable!("the door admits an edge at an axis or a point read alone"),
        };
    }
    let Some(face) = names::EntityKey::face(key) else {
        unreachable!("a selection names a face, an edge or a vertex")
    };
    let kind = topo::readback::face_carrier_kind(&body, face).map_err(readback)?;
    let carrier = Carrier::Surface(kind);
    use geom::SurfaceKind as S;
    match (pose, kind) {
        (K::Plane, S::Plane) => {
            let face_pose = topo::readback::face_pose(&body, face).map_err(readback)?;
            // DM1a: the outward normal is the chart axis folded through
            // the face's sense — the bit selects, nothing is computed.
            let normal = OutwardNormal::from_chart(face_pose.axis, face_pose.sense).vec();
            Ok(PoseValue::Plane {
                origin: face_pose.origin,
                normal: unit(normal, PLANE_NORMAL_ROLE)?,
            })
        }
        (K::Plane, _) => Err(fault(PoseReadFault::NotPlanar { carrier: kind })),
        (K::Axis, S::Cylinder | S::Cone | S::Torus) => {
            let face_pose = topo::readback::face_pose(&body, face).map_err(readback)?;
            Ok(PoseValue::Axis {
                origin: face_pose.origin,
                dir: unit(face_pose.axis, DATUM_AXIS_ROLE)?,
            })
        }
        (K::Axis, _) => Err(fault(PoseReadFault::NoAxis { carrier })),
        (K::Point, S::Sphere | S::Torus | S::Cone) => Ok(PoseValue::Point {
            position: topo::readback::face_pose(&body, face)
                .map_err(readback)?
                .origin,
        }),
        (K::Point, _) => Err(fault(PoseReadFault::NoCentre { carrier })),
        _ => unreachable!("the door admits a face at a plane, an axis or a point read alone"),
    }
}
