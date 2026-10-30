//! **The pose evaluator** (D10; FORK-S3P, FORK-1b): every pose a node
//! reads, bound at its reader at the lane scalar.
//!
//! A pose's value can depend on built geometry (a face read as a plane
//! reads a `Face`), so a pose definition is evaluated when its reader
//! is, after every operation it reaches ([`crate::Doc::read_ports`] puts
//! them upstream of the reader). One door, [`eval_pose`], answers an
//! operation's pose output (a revolve's axis) and a pose
//! definition alike; a definition reading another pose reads it through
//! the same door.
//!
//! Every read off geometry copies a stored fact out — a carrier's tag,
//! its distinguished point, its axis, a face's sense — and decides
//! nothing. The constructions decide one length each, under the pose
//! direction funnel ([`POSE_UNIT_NORM`]), and refuse its zero as their
//! degenerate case ([`PoseConstruction`]).

use geom_brep::OutwardNormal;
use geom_core::{Decide, Point3, RigidMotion, Tol, UnitVec3, UnitVec3Error, Vec3};
use topo::POSE_UNIT_NORM;

use super::{
    AXIS_DIRECTION_ROLE, DirectionRefusal, PLANE_NORMAL_ROLE, Results, band, frame_axes,
    pose_direction_escalated, pose_unit, refusal, select_one, slots, value_of,
};
use crate::eval::{Frame, NodeErrorKind, ValuePayload};
use crate::expr::{VarEnv, eval_var};
use crate::mate::PoseSymmetry;
use crate::names;
use crate::node::{Node, SlotId};
use crate::pose::{Carrier, PoseConstruction, PoseDef, PoseReadFault, PoseValue, RoleRead};
use crate::program::ProfileProgram;
use crate::var::{VarDef, VarId, VarKind};
use crate::{OperandSlot, VarKind as K};

/// The role word a construction's direction is decided under.
const POSE_DIRECTION_ROLE: &str = "pose direction";

/// The role word a carried pose's direction is normalized under, once
/// its copy's motion has turned it.
const CARRIED_DIRECTION_ROLE: &str = "carried pose direction";

/// **`value` as a copy at `motion` carries it** ([`PoseDef::Carried`]):
/// its points mapped, its directions turned and normalized again.
fn carried<T: Decide>(
    value: PoseValue<T>,
    motion: &geom_core::RigidMotion<T>,
    b: geom_core::Band,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let turn = |dir: UnitVec3<T>| {
        pose_unit(motion.transform_vec(dir.get()), CARRIED_DIRECTION_ROLE, b)
            .map_err(DirectionRefusal::node_error)
    };
    Ok(match value {
        PoseValue::Point { position } => PoseValue::Point {
            position: motion.transform_point(position),
        },
        PoseValue::Direction { dir } => PoseValue::Direction { dir: turn(dir)? },
        PoseValue::Axis(line) => {
            PoseValue::axis_through(motion.transform_point(line.foot()), turn(line.dir())?)
        }
        // `p · n = c` carried to `x = R p + t` is `x · R n = c + t · R n`.
        PoseValue::Plane { normal, offset } => {
            let normal = turn(normal)?;
            PoseValue::Plane {
                normal,
                offset: offset + (motion.origin() - Point3::origin()).dot(normal.get()),
            }
        }
        PoseValue::Frame(f) => PoseValue::Frame(
            frame_axes(
                motion.transform_point(f.origin()),
                motion.transform_vec(f.u().get()),
                motion.transform_vec(f.v().get()),
                b,
            )
            .map_err(DirectionRefusal::node_error)?,
        ),
    })
}

/// **A pose in the coordinates of a frame** (D10 `[placement.frame]`):
/// one read off a body is in that body's, and a definition computes in
/// its first read's frame, reading every other through `relative`, as
/// an operation reads its operands.
pub(crate) struct Posed<T: Decide> {
    value: PoseValue<T>,
    frame: Frame<T>,
}

impl<T: Decide> Posed<T> {
    /// **The pose in `into`'s coordinates**: itself where the two are one
    /// frame, else moved once by the relative map.
    ///
    /// # Errors
    ///
    /// The relative map's overflow; the direction door's refusal of a
    /// moved direction.
    pub(crate) fn read_into(
        self,
        into: &Frame<T>,
        b: geom_core::Band,
    ) -> Result<PoseValue<T>, NodeErrorKind> {
        match into.relative(&self.frame).map_err(NodeErrorKind::Motion)? {
            None => Ok(self.value),
            Some(map) => carried(self.value, &map, b),
        }
    }

    /// **The pose in its space's coordinates**: what a mate equates, and
    /// what a report reads.
    ///
    /// # Errors
    ///
    /// As [`Self::read_into`].
    pub(crate) fn in_its_space(self, b: geom_core::Band) -> Result<PoseValue<T>, NodeErrorKind> {
        self.read_into(&Frame::Own, b)
    }

    /// The same pose read through `role`, in the same frame.
    fn through(self, role: Option<crate::pose::PoseRole>) -> Self {
        Self {
            value: through(self.value, role),
            frame: self.frame,
        }
    }
}

/// **The pose the node reads at `slot`, evaluated** at the lane
/// scalar, in its space's coordinates ([`Posed::in_its_space`]).
///
/// # Errors
///
/// As [`eval_posed`].
pub(crate) fn eval_pose<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    read: RoleRead<VarId>,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    eval_posed(doc, results, slot, read, env, tol)?.in_its_space(band(tol)?)
}

/// **The pose the node reads at `slot`, evaluated** at the lane
/// scalar: an operation's pose output read off its value, or a pose
/// definition bound here, read through its role where it carries one,
/// in the frame it computed in. Its symmetry is its kind's (the door
/// asserts the two tables agree).
///
/// # Errors
///
/// The read unresolved or of no pose; a pose read off geometry that
/// finds none ([`NodeErrorKind::PoseRead`]); a construction's
/// degenerate case ([`NodeErrorKind::PoseDegenerate`]); a direction the
/// direction door refuses; a scalar that does not evaluate.
pub(crate) fn eval_posed<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    read: RoleRead<VarId>,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Posed<T>, NodeErrorKind> {
    let admits = read.admits(slot.kind());
    let value = eval_pose_as(doc, results, slot, admits, read.read, env, tol)?;
    Ok(value.through(read.role))
}

/// **A pose read bound against an evaluation's results**, with no
/// reader seat: what a report over the document's pose variables reads
/// ([`crate::mate::unobservable()`]). Its refusals name no seat a reader
/// holds, so it is for reports, never for an op.
pub(crate) fn read_pose<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    read: RoleRead<VarId>,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let admits = crate::SlotKind::Pose(crate::pose::PoseAdmits::Any);
    let value = eval_pose_as(
        doc,
        results,
        OperandSlot::Frame,
        admits,
        read.read,
        env,
        tol,
    )?
    .in_its_space(band(tol)?)?;
    Ok(match read.role {
        None => value,
        Some(role) => role.read(&value).ok_or(NodeErrorKind::UnresolvedRead {
            slot: OperandSlot::Frame,
            var: read.read,
        })?,
    })
}

/// `value` read through `role`, or itself for a plain read.
fn through<T: Decide>(value: PoseValue<T>, role: Option<crate::pose::PoseRole>) -> PoseValue<T> {
    match role {
        None => value,
        Some(role) => role.read(&value).unwrap_or_else(|| {
            unreachable!(
                "the door admits a role its pose offers alone: {role} of {}",
                value.kind()
            )
        }),
    }
}

/// [`eval_pose`] for a read admitting `admits`: the reader's seat's
/// kind at the top, a read's own kind inside a definition, where the
/// door checked it against what the definition admits there.
fn eval_pose_as<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    admits: crate::SlotKind,
    var: VarId,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Posed<T>, NodeErrorKind> {
    let Some(held) = doc.var(var) else {
        return Err(NodeErrorKind::UnresolvedRead { slot, var });
    };
    let value = match held.def() {
        VarDef::Output { node, port } => output(doc, results, admits, *node, *port, env, tol)?,
        VarDef::Pose(def) => definition(doc, results, slot, def, env, tol)?,
        VarDef::Free(_)
        | VarDef::Defined(_)
        | VarDef::Select(_)
        | VarDef::Pick(_)
        | VarDef::Index(_) => {
            return Err(NodeErrorKind::UnresolvedRead { slot, var });
        }
    };
    assert_eq!(
        Some(value.value.symmetry().family()),
        held.kind().symmetry(),
        "a pose value's subgroup is its kind's"
    );
    Ok(value)
}

/// **An operation's pose output**: a revolve's axis, the lift of its 2-D axis line through its profile's plane.
fn output<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    admits: crate::SlotKind,
    node: crate::RecipeNodeId,
    port: u8,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Posed<T>, NodeErrorKind> {
    let value = value_of(results, node)?;
    match (&value.payload, doc.node(node)) {
        (_, Some(revolve @ Node::Revolve { profile, .. })) if port == 1 => {
            let at = super::super::read_at(doc, OperandSlot::Profile, *profile)?;
            let ValuePayload::Profile(drawn) = &value_of(results, at)?.payload else {
                return Err(NodeErrorKind::MissingInput { input: at });
            };
            let vals = slots::eval_slots(revolve, env)
                .map_err(|(slot, source)| NodeErrorKind::Expr { slot, source })?;
            let origin = super::slot_point2(&vals, SlotId::Origin);
            let dir = super::axis_heading(&vals);
            // The sweep's own arithmetic (`sweep::revolve`'s axis frame):
            // the origin placed as a point, the direction decided in the
            // sketch and turned by the plane's frame, so the line this
            // port answers is the one the body turns about.
            let plane = drawn.validated.plane();
            let sketch = pose_unit(
                Vec3::new(dir.x, dir.y, T::zero()),
                AXIS_DIRECTION_ROLE,
                band(tol)?,
            )
            .map_err(DirectionRefusal::node_error)?;
            // In the revolve's own coordinates, the ones its profile is
            // drawn in, whatever frame its body was then read into: the
            // frame the profile is drawn on, where it is drawn on one.
            Ok(Posed {
                value: PoseValue::axis_through(plane.to_world(origin), plane.frame().turn(sketch)),
                frame: drawn.frame.clone().unwrap_or(Frame::Own),
            })
        }
        // A read the door would have refused, in a document built past
        // it: the pose the seat asks for, in the operand door's words.
        _ => {
            use super::super::phrase;
            let expected = match admits {
                crate::SlotKind::Is(K::Axis) => phrase::AXIS_POSE,
                crate::SlotKind::Is(K::Plane) => phrase::PLANE_POSE,
                crate::SlotKind::Is(K::Frame) => phrase::FRAME_POSE,
                _ => super::super::family::POSE,
            };
            Err(super::wrong_operand(value, node, expected))
        }
    }
}

/// **A pose definition, bound** at the lane scalar.
fn definition<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    def: &PoseDef,
    env: &VarEnv<T>,
    tol: Tol,
) -> Result<Posed<T>, NodeErrorKind> {
    // A read inside the definition admits its own kind: the door checked
    // it against what the definition reads there.
    let pose = |read: &RoleRead<VarId>| {
        let admits = crate::SlotKind::Is(doc.var(read.read).map_or(K::Scalar, crate::Var::kind));
        let value = eval_pose_as(doc, results, slot, admits, read.read, env, tol)?;
        Ok::<_, NodeErrorKind>(value.through(read.role))
    };
    let scalar = |var: VarId, dim| {
        eval_var(var, dim, env).map_err(|source| NodeErrorKind::PoseScalar { var, source })
    };
    let b = band(tol)?;
    // A definition computes in its first read's frame and reads every
    // other into it.
    let in_frame = |frame: Frame<T>, value| Posed { value, frame };
    Ok(match def {
        PoseDef::Plane { face } => read_off(doc, results, slot, *face, K::Plane, b)?,
        PoseDef::Axis { of } => read_off(doc, results, slot, *of, K::Axis, b)?,
        PoseDef::Point { of } => read_off(doc, results, slot, *of, K::Point, b)?,
        // Positions moved along the direction, directions kept.
        PoseDef::Slide {
            pose: read,
            along,
            by,
        } => {
            let first = pose(read)?;
            let Some(dir) = pose(along)?.read_into(&first.frame, b)?.heading() else {
                unreachable!("the door admits a direction or an axis alone at a slide's direction")
            };
            let by = scalar(*by, crate::expr::Dimension::Length)?;
            in_frame(first.frame, slide(first.value, dir, by)?)
        }
        // The pose turned about the line by the right-hand rule: a rigid
        // motion, so each direction is the motion's turn of a unit one.
        PoseDef::Spin {
            pose: read,
            about,
            by,
        } => {
            let first = pose(read)?;
            let PoseValue::Axis(about) = pose(about)?.read_into(&first.frame, b)? else {
                unreachable!("the door admits an axis alone at a spin's axis")
            };
            let angle = crate::expr::eval_angle(*by, env)
                .map_err(|source| NodeErrorKind::PoseScalar { var: *by, source })?;
            in_frame(
                first.frame,
                spin(first.value, about.foot(), about.dir(), angle)?,
            )
        }
        PoseDef::Through { axis, point } => {
            let first = pose(axis)?;
            let (PoseValue::Axis(axis), PoseValue::Point { position }) =
                (first.value, pose(point)?.read_into(&first.frame, b)?)
            else {
                unreachable!("the door admits an axis and a point at a frame through them")
            };
            let (foot, dir) = (axis.foot(), axis.dir());
            let a = dir.get();
            let along = (position - foot).dot(a);
            let at = foot + a * along;
            in_frame(
                first.frame,
                RigidMotion::from_aim_and_reference(at, dir, position - foot, POSE_UNIT_NORM, b)
                    .map(PoseValue::Frame)
                    .map_err(|e| degenerate(e.error, PoseConstruction::Through))?,
            )
        }
        PoseDef::Meet { a, b: second } => {
            let first = pose(a)?;
            let (
                PoseValue::Plane {
                    normal: na,
                    offset: ca,
                },
                PoseValue::Plane {
                    normal: nb,
                    offset: cb,
                },
            ) = (first.value, pose(second)?.read_into(&first.frame, b)?)
            else {
                unreachable!("the door admits two planes at a meeting line")
            };
            let d = na.get().cross(nb.get());
            let dir = UnitVec3::new(d, POSE_UNIT_NORM, b)
                .map_err(|e| degenerate(e, PoseConstruction::Meet))?;
            // The line's point nearest the origin, in closed form:
            // `(ca (nb × d) + cb (d × na)) / |d|²` lies on both planes
            // (`na · (nb × d) = nb · (d × na) = |d|²`) and is
            // perpendicular to `d`; `|d|` is decided nonzero above.
            let foot = (nb.get().cross(d) * ca + d.cross(na.get()) * cb) / d.norm_squared();
            in_frame(
                first.frame,
                PoseValue::axis_through(Point3::origin() + foot, dir),
            )
        }
        PoseDef::Flip { pose: read } => {
            let first = pose(read)?;
            in_frame(first.frame, flip(first.value, b)?)
        }
        // The pose as its copy carries it: its frame moved by the motion
        // the copy's placement solved to, the pose itself unchanged.
        PoseDef::Carried { copy, pose: read } => {
            let placement = super::super::read_at(doc, slot, *copy)?;
            let value = pose(read)?;
            match value_of(results, placement)?.motion {
                None => value,
                Some(motion) => Posed {
                    frame: value.frame.moved(&motion).map_err(NodeErrorKind::Motion)?,
                    value: value.value,
                },
            }
        }
    })
}

/// **A pose slid along `dir` by `by`**: its position moved, its
/// directions kept. A direction has no position and is no slide's.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`] where a frame's translation is no motion.
pub(crate) fn slide<T: Decide>(
    value: PoseValue<T>,
    dir: UnitVec3<T>,
    by: T,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let step = dir.get() * by;
    Ok(match value {
        PoseValue::Point { position } => PoseValue::Point {
            position: position + step,
        },
        PoseValue::Axis(line) => PoseValue::axis_through(line.foot() + step, line.dir()),
        PoseValue::Plane { normal, offset } => PoseValue::Plane {
            normal,
            offset: offset + by * dir.get().dot(normal.get()),
        },
        PoseValue::Frame(f) => PoseValue::Frame(
            RigidMotion::translation(step)
                .and_then(|t| t.compose(f))
                .map_err(NodeErrorKind::Motion)?,
        ),
        PoseValue::Direction { .. } => {
            unreachable!("the door admits no direction at a slide: it has no position")
        }
    })
}

/// **A pose turned by `angle` about the line `(at, dir)`**, by the
/// right-hand rule: a rigid motion, so each direction is the motion's
/// turn of a unit one.
///
/// # Errors
///
/// [`NodeErrorKind::Motion`] where the turn is no motion.
pub(crate) fn spin<T: Decide>(
    value: PoseValue<T>,
    at: Point3<T>,
    dir: UnitVec3<T>,
    angle: geom_core::Angle<T>,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let turn =
        RigidMotion::rotation_about_axis_by(at, dir, angle).map_err(NodeErrorKind::Motion)?;
    // A turned point takes `q + R·(p − q)`, which widens by its offset
    // from the axis, not by the composite map's anchor.
    let point = |p| geom_core::Affine3::rotate_point_about_unit_axis_by(at, dir, angle, p);
    Ok(match value {
        PoseValue::Point { position } => PoseValue::Point {
            position: point(position),
        },
        PoseValue::Direction { dir } => PoseValue::Direction {
            dir: turn.turn(dir),
        },
        PoseValue::Axis(line) => PoseValue::axis_through(point(line.foot()), turn.turn(line.dir())),
        // `p · n = c` turned to `q + R (p − q)`: its offset along `R n`
        // is `c + q · (R n − n)`.
        PoseValue::Plane { normal, offset } => {
            let turned = turn.turn(normal);
            let q = at - Point3::origin();
            PoseValue::Plane {
                normal: turned,
                offset: offset + q.dot(turned.get()) - q.dot(normal.get()),
            }
        }
        PoseValue::Frame(f) => PoseValue::Frame(turn.compose(f).map_err(NodeErrorKind::Motion)?),
    })
}

/// **A pose's opposite sense**: a direction, an axis or a plane
/// reversed, a frame half-turned about its own x. A point has no sense.
///
/// # Errors
///
/// The direction door's refusal where a frame's reversed axes are no
/// frame at `b`.
pub(crate) fn flip<T: Decide>(
    value: PoseValue<T>,
    b: geom_core::predicate::Band,
) -> Result<PoseValue<T>, NodeErrorKind> {
    Ok(match value {
        PoseValue::Direction { dir } => PoseValue::Direction { dir: -dir },
        PoseValue::Axis(line) => PoseValue::Axis(line.reversed()),
        PoseValue::Plane { normal, offset } => PoseValue::Plane {
            normal: -normal,
            offset: -offset,
        },
        // A half-turn about the frame's own x: `u` kept, `v` and the
        // normal reversed.
        PoseValue::Frame(f) => PoseValue::Frame(
            frame_axes(f.origin(), f.u().get(), -f.v().get(), b)
                .map_err(DirectionRefusal::node_error)?,
        ),
        PoseValue::Point { .. } => {
            unreachable!("the door admits no point at a flip: a point has no sense")
        }
    })
}

/// A construction's direction decision found its zero: the degenerate
/// case, refused, or an escalation as any direction's.
fn degenerate(error: UnitVec3Error, construction: PoseConstruction) -> NodeErrorKind {
    match error {
        UnitVec3Error::Degenerate | UnitVec3Error::UnderflowedLength => {
            NodeErrorKind::PoseDegenerate { construction }
        }
        other => refusal(other, POSE_DIRECTION_ROLE, pose_direction_escalated),
    }
}

/// **A pose read off the geometry a selection names**: a face's plane
/// (its outward normal, DM1a), a carrier's axis or an edge's line, a
/// carrier's centre or a vertex's point. Stored facts copied out; a
/// carrier without the pose refuses typed.
fn read_off<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    of: VarId,
    pose: VarKind,
    band: geom_core::Band,
) -> Result<Posed<T>, NodeErrorKind> {
    // The half of a split a selection reads is projected here, for the
    // pose's own reads: `results` are its reader's, unprojected.
    let body_var = doc.selection(of).map_or(of, |select| select.body);
    let projected = super::project_ports(&[body_var], doc, results)?;
    let results = projected.as_ref().unwrap_or(results);
    let (at, ent) = select_one(doc, results, slot, of)?;
    let whole = doc
        .selection(of)
        .is_some_and(|select| crate::repetition::names_members(doc, select));
    let framed = super::read_named_body(results, at, ent.body, whole)?;
    // In the coordinates of the body it is read off.
    Ok(Posed {
        value: read_off_body(framed.body(), ent.key, pose, band)?,
        frame: framed.frame().clone(),
    })
}

/// **The reach of the face a pose is read off**, from the pose's own
/// point ([`crate::eval::measure::reach_of`]): the lever a measure prices
/// a tilt between two axes over. `None` for a pose read off no face (an
/// edge's or a vertex's, a construction's, a datum's).
pub(crate) fn source_reach<T: crate::mate::SolveScalar>(
    doc: &crate::doc::Doc<ProfileProgram>,
    results: &Results<T>,
    slot: OperandSlot,
    var: VarId,
    posed: &Posed<T>,
) -> Option<T> {
    let VarDef::Pose(PoseDef::Axis { of } | PoseDef::Point { of } | PoseDef::Plane { face: of }) =
        doc.var(var)?.def()
    else {
        return None;
    };
    let body_var = doc.selection(*of)?.body;
    let projected = super::project_ports(&[body_var], doc, results).ok()?;
    let results = projected.as_ref().unwrap_or(results);
    let (at, ent) = select_one(doc, results, slot, *of).ok()?;
    let whole = doc
        .selection(*of)
        .is_some_and(|select| crate::repetition::names_members(doc, select));
    let framed = super::read_named_body(results, at, ent.body, whole).ok()?;
    let face = names::EntityKey::face(ent.key)?;
    let from = crate::eval::measure::Pose::of(&posed.value)?.at();
    crate::eval::measure::reach_of(framed.body(), face, from)
}

/// [`read_off`]'s read of the entity `key` of `body`, in the body's
/// coordinates.
fn read_off_body<T: crate::mate::SolveScalar>(
    body: &topo::Body<T>,
    key: names::EntityKey,
    pose: VarKind,
    band: geom_core::Band,
) -> Result<PoseValue<T>, NodeErrorKind> {
    let fault = |fault| NodeErrorKind::PoseRead { pose, fault };
    let readback = |error| fault(PoseReadFault::Readback { error });
    // A carrier's stored direction is unit up to its rounding, and is
    // decided as any pose direction is; one on an axis exactly is that
    // axis, so the offset or foot read against it is no wider than the
    // stored point.
    let unit = |v: Vec3<T>, role| {
        pose_unit(v, role, band)
            .map(crate::mate::solve::on_its_axis)
            .map_err(DirectionRefusal::node_error)
    };
    if let Some(vertex) = names::EntityKey::vertex(key) {
        return match pose {
            K::Point => Ok(PoseValue::Point {
                position: topo::readback::vertex_point(body, vertex).map_err(readback)?,
            }),
            _ => unreachable!("the door admits a vertex at a point read alone"),
        };
    }
    if let Some(edge) = names::EntityKey::edge(key) {
        let kind = topo::readback::edge_carrier_kind(body, edge).map_err(readback)?;
        let carrier = Carrier::Curve(kind);
        let pose_of = topo::readback::edge_pose(body, edge).map_err(readback)?;
        let has = |kinds: &[geom::CurveKind]| kinds.contains(&kind);
        return match pose {
            K::Axis
                if has(&[
                    geom::CurveKind::Line,
                    geom::CurveKind::Circle,
                    geom::CurveKind::Ellipse,
                ]) =>
            {
                Ok(PoseValue::axis_through(
                    pose_of.origin,
                    unit(pose_of.axis, AXIS_DIRECTION_ROLE)?,
                ))
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
    let kind = topo::readback::face_carrier_kind(body, face).map_err(readback)?;
    let carrier = Carrier::Surface(kind);
    use geom::SurfaceKind as S;
    match (pose, kind) {
        (K::Plane, S::Plane) => {
            let face_pose = topo::readback::face_pose(body, face).map_err(readback)?;
            // DM1a: the outward normal is the chart axis folded through
            // the face's sense — the bit selects, nothing is computed.
            let normal = OutwardNormal::from_chart(face_pose.axis, face_pose.sense).vec();
            Ok(PoseValue::plane_through(
                face_pose.origin,
                unit(normal, PLANE_NORMAL_ROLE)?,
            ))
        }
        (K::Plane, _) => Err(fault(PoseReadFault::NotPlanar { carrier: kind })),
        (K::Axis, S::Cylinder | S::Cone | S::Torus) => {
            let face_pose = topo::readback::face_pose(body, face).map_err(readback)?;
            Ok(PoseValue::axis_through(
                face_pose.origin,
                unit(face_pose.axis, AXIS_DIRECTION_ROLE)?,
            ))
        }
        (K::Axis, _) => Err(fault(PoseReadFault::NoAxis { carrier })),
        (K::Point, S::Sphere | S::Torus | S::Cone) => Ok(PoseValue::Point {
            position: topo::readback::face_pose(body, face)
                .map_err(readback)?
                .origin,
        }),
        (K::Point, _) => Err(fault(PoseReadFault::NoCentre { carrier })),
        _ => unreachable!("the door admits a face at a plane, an axis or a point read alone"),
    }
}

/// **One word of the stream a node's content key reads off the pose
/// definitions it reads** (`tag::pose` states the grammar).
pub(crate) enum PoseWord<T> {
    /// A structural word.
    Tag(u8),
    /// A count or an operand's place.
    Count(u64),
    /// A selection's name.
    Name(names::StableName),
    /// A scalar at the lane and at the nominal, and the fraction of a
    /// turn its definition is.
    Value(T, f64, Option<crate::expr::Ratio>),
}

/// **The words a node's content key reads off its pose definitions**,
/// `None` for a node reading none, so its key is the stream it was.
///
/// # Errors
///
/// [`NodeErrorKind::PoseScalar`] for a scalar that does not evaluate at
/// the lane or the nominal.
pub(crate) fn key_words<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    node: &Node<ProfileProgram>,
    env: &VarEnv<T>,
    nominal: &VarEnv<f64>,
) -> Result<Option<Vec<PoseWord<T>>>, NodeErrorKind> {
    use crate::eval::tag::pose as w;
    let mut out = Vec::new();
    for (place, (seat, var)) in node.operand_rows().into_iter().enumerate() {
        let role = node.seat_role(seat);
        if role.is_none() && doc.var(var).and_then(|held| held.def().pose()).is_none() {
            continue;
        }
        // The read's ports are the node's upstream for this read, each
        // once (`Doc::read_ports`); a leaf below says which of them it
        // is, so two trees that reach the same operations in another
        // arrangement key apart.
        let ports = doc.read_ports(var);
        out.push(PoseWord::Tag(w::READ));
        out.push(PoseWord::Count(place as u64));
        out.push(PoseWord::Tag(role_word(role)));
        out.push(PoseWord::Count(ports.len() as u64));
        walk(doc, var, &ports, env, nominal, &mut out)?;
    }
    Ok((!out.is_empty()).then_some(out))
}

fn walk<T: Decide>(
    doc: &crate::doc::Doc<ProfileProgram>,
    var: VarId,
    ports: &[(crate::RecipeNodeId, u8)],
    env: &VarEnv<T>,
    nominal: &VarEnv<f64>,
    out: &mut Vec<PoseWord<T>>,
) -> Result<(), NodeErrorKind> {
    use crate::eval::tag::{pose as w, pose_arm as arm};
    let def = doc.var(var).map(crate::Var::def);
    // Which of the read's ports `at` is: its place in the upstream list.
    let port_word = |at: Option<(crate::RecipeNodeId, u8)>| {
        PoseWord::Count(
            at.and_then(|at| ports.iter().position(|&p| p == at))
                .map_or(u64::MAX, |place| place as u64),
        )
    };
    match def {
        Some(VarDef::Select(select)) => {
            out.push(PoseWord::Tag(w::SELECTION));
            out.push(port_word(doc.read_ports(select.body).first().copied()));
            // A selection in a pick is in that piece: its path keys it.
            if let Some(pick) = doc.pick(select.body) {
                out.push(PoseWord::Count(pick.of.len() as u64));
                out.extend(pick.of.iter().map(|step| PoseWord::Count(step.0.digest())));
            }
            out.push(PoseWord::Count(select.names.len() as u64));
            out.extend(select.names.iter().cloned().map(PoseWord::Name));
        }
        Some(VarDef::Pose(def)) => {
            out.push(PoseWord::Tag(w::DEFINITION));
            out.push(PoseWord::Tag(match def {
                PoseDef::Plane { .. } => arm::PLANE,
                PoseDef::Axis { .. } => arm::AXIS,
                PoseDef::Point { .. } => arm::POINT,
                PoseDef::Slide { .. } => arm::SLIDE,
                PoseDef::Spin { .. } => arm::SPIN,
                PoseDef::Through { .. } => arm::THROUGH,
                PoseDef::Meet { .. } => arm::MEET,
                PoseDef::Flip { .. } => arm::FLIP,
                PoseDef::Carried { .. } => arm::CARRIED,
            }));
            for (_, &read, role, _) in def.reads() {
                out.push(PoseWord::Tag(role_word(role)));
                walk(doc, read, ports, env, nominal, out)?;
            }
            for (slot, &scalar) in def.scalars() {
                let dim = slot.dimension();
                let at = |source| NodeErrorKind::PoseScalar {
                    var: scalar,
                    source,
                };
                out.push(PoseWord::Value(
                    eval_var(scalar, dim, env).map_err(at)?,
                    eval_var(scalar, dim, nominal).map_err(at)?,
                    env.turns.get(&scalar).copied(),
                ));
            }
        }
        // An output, whose key the upstream list carries at the place
        // this word names, or a read the door admits at no pose seat.
        Some(VarDef::Output { node, port }) => {
            out.push(PoseWord::Tag(w::OUTPUT));
            out.push(port_word(Some((*node, *port))));
        }
        Some(VarDef::Free(_) | VarDef::Defined(_) | VarDef::Pick(_) | VarDef::Index(_)) | None => {
            out.push(PoseWord::Tag(w::OUTPUT));
            out.push(port_word(None));
        }
    }
    Ok(())
}

/// A read's role word ([`crate::eval::tag::pose_role`]), a plain
/// read's included.
pub(crate) fn role_word(role: Option<crate::pose::PoseRole>) -> u8 {
    use crate::eval::tag::pose_role as w;
    use crate::pose::PoseRole as R;
    match role {
        None => w::PLAIN,
        Some(R::X) => w::X,
        Some(R::Y) => w::Y,
        Some(R::Z) => w::Z,
        Some(R::Origin) => w::ORIGIN,
        Some(R::Xy) => w::XY,
        Some(R::Yz) => w::YZ,
        Some(R::Zx) => w::ZX,
        Some(R::Normal) => w::NORMAL,
        Some(R::Direction) => w::DIRECTION,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod held_text {
    use geom_core::{Band, Indeterminate, MarginDiag, UnitVec3Error};

    use crate::eval::NodeErrorKind;
    use crate::pose::PoseConstruction;

    /// **A pose construction's undecided direction keeps its sentence**
    /// (D10's pose ground is held): through this door it is a
    /// [`NodeErrorKind::PoseDirectionEscalated`], said "too close to
    /// call" with the bare escalation's tail.
    #[test]
    fn a_pose_constructions_undecided_direction_keeps_its_sentence() {
        let source = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).expect("zero < escalate"),
            predicate: Some(topo::POSE_UNIT_NORM),
            terminal_sliver: false,
        };
        let got = super::degenerate(UnitVec3Error::Escalated(source), PoseConstruction::Through);
        assert!(
            matches!(got, NodeErrorKind::PoseDirectionEscalated { .. }),
            "{got:?}"
        );
        assert_eq!(
            got.to_string(),
            "whether a direction has any length is too close to call: margin 5e-9 lies inside \
             the ambiguity band (1e-9, 1e-8) — a near-coincidence; Recourse: move the geometry, \
             or, if this size is intended, tighten the tolerance below 5e-10 m"
        );
    }
}
