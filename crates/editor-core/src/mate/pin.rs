//! **A copy pinned to the world at a pose, stated over its geometry**
//! (INTENT stage 3 C's migration helper).
//!
//! A mate reads poses off geometry and there is no part frame to read
//! (FORK-S3M), so "this copy, at pose `P` in the world" is a `Frame`
//! mate: a frame read off the copy's own geometry, equated with the
//! world's frame slid and spun to where that frame sits at `P`. The
//! moves are numbers computed once, here.
//!
//! **The frame read is picked by one rule**: the copy's first edge by
//! id whose carrier has an axis (a line, a circle, an ellipse), and the
//! first vertex by id off that edge's line, decided at the run's band
//! — a frame `Through` the edge's axis and the vertex. A copy that
//! offers no such pair refuses ([`PinRefusal::NoFrame`]), never a loose
//! copy in silence.
//!
//! A copy standing where its body stands is placed by [`stand`]: a
//! plane mate on a face no variable reaches and the constant `0` on its
//! freedoms. `pin_at` is the pin of a pose off the identity — inline's
//! copy at a moved landing, and migration, tests and demos — not an
//! authoring gesture. The authoring gesture is [`place_at`]: "place this copy
//! at `P`" writes a plane mate on the copy's first planar face to the
//! world plane that face lies in at `P`, and the three values that
//! put the copy at `P` in that mate's chart — so the copy can then be
//! slid and spun by its numbers. A copy with no planar face falls back
//! to [`pin_at`], and the answer says so.

use geom_core::linalg::Mat3;
use geom_core::predicate::Band;
use geom_core::{RigidMotion, Tol, UnitVec3};

use super::{Mate, Target, WorldRead};
use crate::eval::{EvalOptions, Evaluation, NodeResult, ValuePayload};
use crate::expr::Dimension;
use crate::names::{EntityKey, EntityRef};
use crate::node::RecipeNodeId;
use crate::placement::Frame;
use crate::pose::{PoseDef, PoseRole, RoleRead};
use crate::{CancelToken, Formula, Operand};

/// **Why a copy could not be pinned over its geometry**.
#[derive(Debug, Clone, PartialEq)]
pub enum PinRefusal {
    /// The operand reads no node of the document.
    NoNode,
    /// The node did not evaluate to a value.
    Unevaluated {
        /// The node.
        node: RecipeNodeId,
    },
    /// The node's value is no body, so it has no geometry to read.
    NotABody {
        /// The node.
        node: RecipeNodeId,
    },
    /// The node's value is already placed: its geometry is not its own
    /// coordinates' (a pin is written for a copy before placement).
    AlreadyPlaced {
        /// The node.
        node: RecipeNodeId,
    },
    /// The copy has no edge with an axis and a vertex off its line:
    /// no frame can be read off it by the rule.
    NoFrame {
        /// The node.
        node: RecipeNodeId,
    },
    /// The run's tolerance gives no band to decide the pick at.
    Band,
}

impl core::fmt::Display for PinRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoNode => f.write_str("the copy is read through no node of the document"),
            Self::Unevaluated { node } => write!(f, "{node} did not evaluate"),
            Self::NotABody { node } => write!(f, "{node}'s value is no body"),
            Self::AlreadyPlaced { node } => write!(
                f,
                "{node} is already placed, so its geometry is not its own coordinates'"
            ),
            Self::NoFrame { node } => write!(
                f,
                "{node} has no edge with an axis and a vertex off its line, so no frame is \
                 read off it; pin it by a mate of its own"
            ),
            Self::Band => f.write_str("the tolerance gives no band"),
        }
    }
}

impl core::error::Error for PinRefusal {}

/// **The copy read through `body`, evaluated as it stands, before
/// placement**: its node, its first solid and its name table.
fn copy_of(
    doc: &crate::ProfileDoc,
    body: &Operand,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<
    (
        RecipeNodeId,
        std::sync::Arc<topo::Body<f64>>,
        crate::names::NameTable,
    ),
    PinRefusal,
> {
    let node = match body {
        Operand::Output { node, .. } | Operand::Node(node) => *node,
        Operand::Var(var) => doc.operation_of(*var).ok_or(PinRefusal::NoNode)?,
        _ => return Err(PinRefusal::NoNode),
    };
    let evaluation: Evaluation<f64> =
        crate::eval::evaluate(doc, None, &CancelToken::new(), opts, tol);
    let Some(NodeResult::Ok(value)) = evaluation.result(node) else {
        return Err(PinRefusal::Unevaluated { node });
    };
    if value.motion.is_some() {
        return Err(PinRefusal::AlreadyPlaced { node });
    }
    let port = match body {
        Operand::Output { port, .. } => *port,
        Operand::Var(var) => match doc.var(*var).map(crate::Var::def) {
            Some(crate::var::VarDef::Output { port, .. }) => *port,
            _ => 0,
        },
        _ => 0,
    };
    let solid = match &value.payload {
        ValuePayload::Body(b)
        | ValuePayload::Boolean(crate::eval::BooleanValue::Body { body: b, .. }) => {
            std::sync::Arc::clone(b)
        }
        ValuePayload::Copies(copies) => copies
            .bodies
            .first()
            .cloned()
            .ok_or(PinRefusal::NotABody { node })?,
        ValuePayload::Instances(bodies) => bodies
            .first()
            .cloned()
            .ok_or(PinRefusal::NotABody { node })?,
        ValuePayload::Split { above, below } => {
            let half = crate::names::SplitHalf::of_output_body(u32::from(port))
                .ok_or(PinRefusal::NotABody { node })?;
            let side = if half == crate::names::SplitHalf::Above {
                above
            } else {
                below
            };
            match side {
                crate::eval::SplitSide::Body(b) => std::sync::Arc::clone(b),
                crate::eval::SplitSide::Empty => return Err(PinRefusal::NotABody { node }),
            }
        }
        _ => return Err(PinRefusal::NotABody { node }),
    };
    let var = match body {
        Operand::Var(var) => Some(*var),
        Operand::Output { node, port } => doc.output(*node, *port),
        _ => doc.output(node, 0),
    };
    let table = var
        .and_then(|var| evaluation.table(var))
        .ok_or(PinRefusal::NotABody { node })?;
    Ok((node, solid, table.clone()))
}

/// **A mate pinning the copy read through `body` to the world at
/// `pose`**: `on` a frame read off the copy's geometry by the module's
/// rule, `to` the world's frame spun and slid to where that frame sits
/// at `pose`. `body` is the operand the copy is read through (a
/// placement's shape, an instance's own output), evaluated in `doc`
/// under `opts` as it stands, before placement.
///
/// # Errors
///
/// [`PinRefusal`]: the copy has no value, no body, is placed already,
/// or offers no frame.
pub fn pin_at(
    doc: &crate::ProfileDoc,
    body: &Operand,
    pose: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Mate<Formula>, PinRefusal> {
    let band = Band::linear(tol).map_err(|_| PinRefusal::Band)?;
    let (node, solid, table) = copy_of(doc, body, opts, tol)?;
    let name = |key| table.name_of(&EntityRef { body: 0, key }).cloned();
    let no_frame = PinRefusal::NoFrame { node };
    let anchors = super::anchor::Anchors::of(doc, node);
    let picked = picked_frame(&solid, band, &|key| name(key).filter(|n| anchors.fixed(n)))
        .or_else(|| picked_frame(&solid, band, &name));
    let (edge_name, vertex_name, frame) = picked.ok_or(no_frame)?;
    let read = |def: PoseDef<Operand, Formula>| RoleRead::plain(Operand::Pose(Box::new(def)));
    let on = Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: Operand::select(body.clone(), vec![edge_name]),
        }),
        point: read(PoseDef::Point {
            of: Operand::select(body.clone(), vec![vertex_name]),
        }),
    }));
    let at = pose.affine() * frame.to_affine();
    Ok(Mate {
        on,
        to: Target::World(world_at(at.linear, at.translation)),
        class: super::ContactClass::Rest,
    })
}

/// **The frame [`pin_at`] reads**: the first edge by id with an axis
/// and the first vertex by id off its line, as the evaluation reads
/// them, among the entities `name` names (an entity it names `None` is
/// passed over).
fn picked_frame(
    solid: &topo::Body<f64>,
    band: Band,
    name: &dyn Fn(EntityKey) -> Option<crate::StableName>,
) -> Option<(crate::StableName, crate::StableName, RigidMotion<f64>)> {
    let mut picked = None;
    'edges: for (edge, _) in solid.edges() {
        let Ok(kind) = topo::readback::edge_carrier_kind(solid, edge) else {
            continue;
        };
        if !matches!(
            kind,
            geom::CurveKind::Line | geom::CurveKind::Circle | geom::CurveKind::Ellipse
        ) {
            continue;
        }
        let Ok(line) = topo::readback::edge_pose(solid, edge) else {
            continue;
        };
        let Ok(dir) = UnitVec3::new(line.axis, topo::query::DATUM_UNIT_NORM, band) else {
            continue;
        };
        let Some(edge_name) = name(EntityKey::Edge(edge)) else {
            continue;
        };
        for (vertex, _) in solid.vertices() {
            let Ok(point) = topo::readback::vertex_point(solid, vertex) else {
                continue;
            };
            let off = point - line.origin;
            let across = off - dir.get() * off.dot(dir.get());
            if UnitVec3::new(across, "pin_vertex_off_the_line", band).is_err() {
                continue;
            }
            let Some(vertex_name) = name(EntityKey::Vertex(vertex)) else {
                continue;
            };
            let foot = line.origin + dir.get() * off.dot(dir.get());
            let Ok(frame) = RigidMotion::from_aim_and_reference(
                foot,
                dir,
                off,
                topo::query::DATUM_UNIT_NORM,
                band,
            ) else {
                continue;
            };
            picked = Some((edge_name, vertex_name, frame));
            break 'edges;
        }
    }
    picked
}

/// **The world's frame moved to `(linear, translation)`**
/// ([`moved_by`] of the frame itself).
fn world_at(linear: Mat3<f64>, translation: geom_core::linalg::Vec3<f64>) -> WorldRead<Formula> {
    moved_by(WorldRead::Frame, linear, translation)
}

/// **A number read back from a pose, as a literal**: a zero, of either
/// sign, is the constant `0` (VR6), which mints no variable and so
/// moves under no analysis; any other number a typed value. The one
/// writer of `pin_at`'s and `place_at`'s numbers.
fn written(value: f64, dim: Dimension) -> Formula {
    if value == 0.0 {
        return Formula::zero(dim);
    }
    Formula::literal(value, dim).unwrap_or_else(|_| unreachable!("a finite literal of {dim:?}"))
}

/// **`world` moved by the rigid motion `(linear, translation)`**: spun
/// about the world's x, then y, then z — `linear = Rz(γ)·Ry(β)·Rx(α)`,
/// the angles taken once by `atan2` — and slid along the world's x, y
/// and z. Each move is about the world's own axes, so appended to any
/// read it composes on the left whatever the read holds. A move by
/// exactly zero is no move written.
pub(crate) fn moved_by(
    world: WorldRead<Formula>,
    linear: Mat3<f64>,
    translation: geom_core::linalg::Vec3<f64>,
) -> WorldRead<Formula> {
    // Entry `r(i, j)`: row `i` of column `j`.
    let r = |i: usize, j: usize| {
        let c = [linear.c0, linear.c1, linear.c2][j];
        [c.x, c.y, c.z][i]
    };
    let cos_beta = r(0, 0).hypot(r(1, 0));
    let beta = (-r(2, 0)).atan2(cos_beta);
    let (alpha, gamma) = if cos_beta > f64::EPSILON {
        (r(2, 1).atan2(r(2, 2)), r(1, 0).atan2(r(0, 0)))
    } else {
        // Gimbal lock: the x and z turns share an axis, so z's is none.
        ((-r(1, 2)).atan2(r(1, 1)), 0.0)
    };
    let mut world = world;
    for (role, angle) in [
        (PoseRole::X, alpha),
        (PoseRole::Y, beta),
        (PoseRole::Z, gamma),
    ] {
        if angle != 0.0 {
            world = world.spin(WorldRead::through(role), written(angle, Dimension::Angle));
        }
    }
    for (role, length) in [
        (PoseRole::X, translation.x),
        (PoseRole::Y, translation.y),
        (PoseRole::Z, translation.z),
    ] {
        if length != 0.0 {
            world = world.slide(WorldRead::through(role), written(length, Dimension::Length));
        }
    }
    world
}

/// **The run of world-axis moves `world` ends in, taken off**: the read
/// under the run, and the run as one rigid motion, `(linear,
/// translation)` (the identity where there is none). A move is in the
/// run where it turns about or slides along a world axis by a number
/// `number` answers; a move it answers none for ends the run, and what
/// lies under it is kept. What inline folds into the motion it moves a
/// part's world read by, so that [`moved_by`] writes one run where a
/// read is moved twice, and its depth does not grow with the nesting
/// it is flattened from.
pub(crate) fn written_moves<S: crate::Slot>(
    world: WorldRead<S>,
    number: impl Fn(&S) -> Option<f64>,
) -> (WorldRead<S>, Mat3<f64>, geom_core::linalg::Vec3<f64>) {
    use geom_core::linalg::Vec3;
    let axis_of = |read: &WorldRead<S>| match read {
        WorldRead::Role { pose, role } if matches!(**pose, WorldRead::Frame) => match role {
            PoseRole::X => Some(Vec3::new(1.0, 0.0, 0.0)),
            PoseRole::Y => Some(Vec3::new(0.0, 1.0, 0.0)),
            PoseRole::Z => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        },
        _ => None,
    };
    // Peeled outermost first: the run is `held = M_top ∘ … ∘ M_k`.
    let (mut linear, mut translation) = (Mat3::identity(), Vec3::new(0.0, 0.0, 0.0));
    let mut world = world;
    loop {
        match world {
            WorldRead::Spin { pose, about, by }
                if let (Some(axis), Some(angle)) = (axis_of(&about), number(&by)) =>
            {
                linear = linear * Mat3::rotation_about(axis, angle);
                world = *pose;
            }
            WorldRead::Slide { pose, along, by }
                if let (Some(axis), Some(length)) = (axis_of(&along), number(&by)) =>
            {
                translation = translation + linear * (axis * length);
                world = *pose;
            }
            other => return (other, linear, translation),
        }
    }
}

/// **How [`place_at`] placed a copy.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacedBy {
    /// A plane mate on the copy's first planar face and its three
    /// values.
    PlanarFace,
    /// An axis mate on a revolve's or a full ring's own axis and its
    /// two values ([`stand`]).
    Axis,
    /// The copy has no planar face, or its first one stands at the pose
    /// within the band about the chart's one pole (no value could be
    /// charted on it): pinned over a frame read off its geometry
    /// ([`pin_at`]), which leaves no freedom to set and reads no chart.
    Pin,
}

/// **The "place part" gesture's constraints** ([`place_at`]): the
/// mate, then the values that name it, each a freedom and its number.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    /// The mate.
    pub mate: Mate<Formula>,
    /// Its values, to add once the mate's id is minted.
    pub values: Vec<(super::Coord, Formula)>,
    /// How the copy was placed: a gesture whose copy has no planar face
    /// says so in its report.
    pub by: PlacedBy,
}

/// **The "place part" gesture** (ruling, scope): the copy read through
/// `body` placed at `pose`, as constraints its author can then move by
/// their numbers — a plane mate on the copy's first planar face by id
/// (`on` its plane, outward normal and all) to the world plane that
/// face lies in at `pose`, and the three values (slides along the
/// chart's `x` and `y`, and the spin) that put the copy at `pose` in
/// that mate's chart ([`super::Value`]). The values are computed from
/// `pose`, never assumed zero, so re-evaluating reproduces `pose` to
/// the rounding of one re-composition.
///
/// The world plane is the world's frame swung onto the face's normal at
/// `pose` by the shortest turn ([`super::reference_about`]) and slid to
/// the face's point, read through its `xy` plane. A copy with no planar
/// face, or whose face's chart the band cannot swing at `pose`, falls
/// back to [`pin_at`] ([`PlacedBy::Pin`]).
///
/// # Errors
///
/// [`PinRefusal`], as [`pin_at`]'s.
pub fn place_at(
    doc: &crate::ProfileDoc,
    body: &Operand,
    pose: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Placed, PinRefusal> {
    use geom_core::linalg::Point3;
    let band = Band::linear(tol).map_err(|_| PinRefusal::Band)?;
    let (node, solid, table) = copy_of(doc, body, opts, tol)?;
    let anchors = super::anchor::Anchors::of(doc, node);
    // The first planar face by id whose definition reaches no document
    // variable, else the first planar face by id.
    let planar = |fixed_only: bool| {
        solid.faces().find_map(|(face, _)| {
            let kind = topo::readback::face_carrier_kind(&solid, face).ok()?;
            if kind != geom::SurfaceKind::Plane {
                return None;
            }
            let name = table.name_of(&EntityRef {
                body: 0,
                key: EntityKey::Face(face),
            })?;
            if fixed_only && !anchors.fixed(name) {
                return None;
            }
            let at = topo::readback::face_pose(&solid, face).ok()?;
            let normal = geom_brep::OutwardNormal::from_chart(at.axis, at.sense).vec();
            let normal = UnitVec3::new(normal, topo::query::DATUM_UNIT_NORM, band).ok()?;
            Some((name.clone(), at.origin, normal))
        })
    };
    let pinned = || {
        Ok(Placed {
            mate: pin_at(doc, body, pose, opts, tol)?,
            values: Vec::new(),
            by: PlacedBy::Pin,
        })
    };
    // A face no variable reaches, else a plane-less construction's own
    // axis, else any planar face, else the pin.
    let planar = match planar(true) {
        Some(fixed) => Some(fixed),
        None => {
            if own_axis(doc, body, &solid, &table, &anchors).is_some() {
                return axis_in_world(doc, body, pose, opts, tol);
            }
            planar(false)
        }
    };
    let Some((face, origin, normal)) = planar else {
        return pinned();
    };
    let at = pose.affine();
    let on = crate::pose::PoseValue::Plane { origin, normal };
    let to = crate::pose::PoseValue::Plane {
        origin: at.transform_point(origin),
        normal: UnitVec3::new(at.linear * normal.get(), topo::query::DATUM_UNIT_NORM, band)
            .map_err(|_| PinRefusal::Band)?,
    };
    let crate::pose::PoseValue::Plane {
        origin: world_origin,
        normal: world_normal,
    } = to
    else {
        unreachable!("the target was built a plane")
    };
    let reach = super::part_reach(&solid).map_err(|_| PinRefusal::NoFrame { node })?;
    let datum = (origin - Point3::origin()).norm() + (world_origin - Point3::origin()).norm();
    let arm = super::coset::Arm::of(reach, datum).map_err(|_| PinRefusal::Band)?;
    // The world plane: the world's frame swung onto the normal, slid to
    // the face's point, read through its `xy` plane. A normal within the
    // band about the swing's pole has no chart: the gesture pins instead.
    let Ok(x) = super::reference_about(world_normal, arm.get(), band) else {
        return pinned();
    };
    let n = world_normal.get();
    let frame = geom_core::linalg::Mat3::from_cols(x, n.cross(x), n);
    let world = super::WorldRead::Role {
        pose: Box::new(moved_by(
            super::WorldRead::Frame,
            frame,
            world_origin - Point3::origin(),
        )),
        role: PoseRole::Xy,
    };
    let Ok(chart) =
        super::value::Chart::of(&on, &to, false, &geom_core::Affine3::identity(), arm, band)
    else {
        return pinned();
    };
    let chart = chart.unwrap_or_else(|| unreachable!("a plane mate has a chart"));
    let values = vec![
        (
            super::Coord::Slide(super::SlideAxis::X),
            written(chart.slide_at(&at, super::SlideAxis::X), Dimension::Length),
        ),
        (
            super::Coord::Slide(super::SlideAxis::Y),
            written(chart.slide_at(&at, super::SlideAxis::Y), Dimension::Length),
        ),
        (
            super::Coord::Spin,
            written(chart.spin_at(&at), Dimension::Angle),
        ),
    ];
    let on = Operand::Pose(Box::new(PoseDef::Plane {
        face: Operand::select(body.clone(), vec![face]),
    }));
    Ok(Placed {
        mate: Mate {
            on,
            to: Target::World(world),
            class: super::ContactClass::Rest,
        },
        values,
        by: PlacedBy::PlanarFace,
    })
}

/// **A copy standing where its body stands** (D10: a body has no
/// location): a plane mate on the copy's first planar face by id whose
/// definition reaches no document variable ([`super::anchor`]) — an
/// extrude's start cap, which lies on its profile's plane — to the
/// world plane it lies in, and the constant `0` on the three freedoms
/// the mate leaves, which says the copy's own coordinates agree with
/// the world's. The copy side carries a `Flip` where the face looks
/// down its own `z`, so the mate's direction is the world's `+z` and no
/// move is written on the world side.
///
/// A copy with no such face stands on its own axis where it is a
/// revolve or a full ring ([`on_its_axis`]), and is placed by the
/// gesture at the identity otherwise ([`place_at`]): its numbers are
/// then its anchor's nominal.
///
/// # Errors
///
/// [`PinRefusal`], as [`place_at`]'s.
pub fn stand(
    doc: &crate::ProfileDoc,
    body: &Operand,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Placed, PinRefusal> {
    let (node, solid, table) = copy_of(doc, body, opts, tol)?;
    let anchors = super::anchor::Anchors::of(doc, node);
    let on_its_plane = solid.faces().find_map(|(face, _)| {
        let kind = topo::readback::face_carrier_kind(&solid, face).ok()?;
        if kind != geom::SurfaceKind::Plane {
            return None;
        }
        let name = table.name_of(&EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        })?;
        if !anchors.fixed(name) {
            return None;
        }
        let at = topo::readback::face_pose(&solid, face).ok()?;
        let normal = geom_brep::OutwardNormal::from_chart(at.axis, at.sense).vec();
        // A face no variable reaches is built from constants alone, so
        // it lies on its body's own `xy` exactly or not at all: normal
        // ±z, through a point at z = 0.
        let on_xy = [at.origin.z, normal.x, normal.y]
            .into_iter()
            .all(|c| c == 0.0);
        on_xy.then(|| (name.clone(), normal.z < 0.0))
    });
    let Some((face, down)) = on_its_plane else {
        if let Some(placed) = on_its_axis(doc, body, &solid, &table, &anchors) {
            return Ok(placed);
        }
        return place_at(doc, body, &Frame::IDENTITY, opts, tol);
    };
    let plane = Operand::Pose(Box::new(PoseDef::Plane {
        face: Operand::select(body.clone(), vec![face]),
    }));
    let on = if down {
        Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::plain(plane),
        }))
    } else {
        plane
    };
    let zero = |coord, dim| (coord, Formula::zero(dim));
    Ok(Placed {
        mate: Mate {
            on,
            to: Target::World(WorldRead::through(PoseRole::Xy)),
            class: super::ContactClass::Rest,
        },
        values: vec![
            zero(super::Coord::Slide(super::SlideAxis::X), Dimension::Length),
            zero(super::Coord::Slide(super::SlideAxis::Y), Dimension::Length),
            zero(super::Coord::Spin, Dimension::Angle),
        ],
        by: PlacedBy::PlanarFace,
    })
}

/// **A construction with no plane of its own, standing on its own
/// axis** (spec §5): an `Axis` mate from a revolve's `axis` output, or
/// from a full ring's torus carrier axis, which reads nothing, to the
/// world's axis that line is in the construction's own coordinates,
/// and the constant `0` on the slide along it and the spin about it.
/// A revolve's axis slots are moves of the target reading what the
/// slots read, so the copy stands where the revolve put it at every
/// value; a constant `0` slot writes no move. `None` where no revolve
/// or full ring is carried unmoved into the copied body.
fn on_its_axis(
    doc: &crate::ProfileDoc,
    body: &Operand,
    solid: &topo::Body<f64>,
    table: &crate::names::NameTable,
    anchors: &super::anchor::Anchors<'_>,
) -> Option<Placed> {
    let (_, on, to) = own_axis(doc, body, solid, table, anchors)?;
    let zero = |coord, dim| (coord, Formula::zero(dim));
    Some(Placed {
        mate: Mate {
            on,
            to: Target::World(to),
            class: super::ContactClass::Rest,
        },
        values: vec![
            zero(super::Coord::Slide(super::SlideAxis::Z), Dimension::Length),
            zero(super::Coord::Spin, Dimension::Angle),
        ],
        by: PlacedBy::Axis,
    })
}

/// **A plane-less construction's own axis** ([`on_its_axis`]): the
/// construction carried unmoved into the copy, the axis read off it,
/// and the world's axis that line is in the construction's own
/// coordinates.
fn own_axis(
    doc: &crate::ProfileDoc,
    body: &Operand,
    solid: &topo::Body<f64>,
    table: &crate::names::NameTable,
    anchors: &super::anchor::Anchors<'_>,
) -> Option<(RecipeNodeId, Operand, WorldRead<Formula>)> {
    use crate::node::{Node, TubeWindow};
    anchors.unmoved().find_map(|at| match doc.node(at)? {
        Node::Revolve {
            axis_through,
            axis_heading,
            ..
        } => {
            // The revolve's `axis` output where the copy is the revolve's
            // own body; else the axis of the copy's first circular edge
            // the revolve minted (a revolved vertex's circle turns about
            // the revolve's axis), since a mate reads only its copy.
            let own = match body {
                Operand::Output { node, .. } | Operand::Node(node) => *node == at,
                Operand::Var(var) => doc.operation_of(*var) == Some(at),
                _ => false,
            };
            let on = if own {
                Operand::Output { node: at, port: 1 }
            } else {
                let edge = solid.edges().find_map(|(edge, _)| {
                    let kind = topo::readback::edge_carrier_kind(solid, edge).ok()?;
                    (kind == geom::CurveKind::Circle).then_some(())?;
                    let name = table.name_of(&EntityRef {
                        body: 0,
                        key: EntityKey::Edge(edge),
                    })?;
                    minted_by(name, at).then(|| name.clone())
                })?;
                Operand::Pose(Box::new(PoseDef::Axis {
                    of: Operand::select(body.clone(), vec![edge]),
                }))
            };
            Some((at, on, revolve_axis(doc, *axis_through, *axis_heading)))
        }
        Node::Tube {
            window: TubeWindow::Full,
            ..
        }
        | Node::HollowTube {
            window: TubeWindow::Full,
            ..
        } => {
            // The outer torus: the first torus face by id the ring
            // minted. Its axis is the ring's own `z`, whatever its radii.
            let face = solid.faces().find_map(|(face, _)| {
                let kind = topo::readback::face_carrier_kind(solid, face).ok()?;
                (kind == geom::SurfaceKind::Torus).then_some(())?;
                let name = table.name_of(&EntityRef {
                    body: 0,
                    key: EntityKey::Face(face),
                })?;
                (name.node == at).then(|| name.clone())
            })?;
            let on = Operand::Pose(Box::new(PoseDef::Axis {
                of: Operand::select(body.clone(), vec![face]),
            }));
            Some((at, on, WorldRead::through(PoseRole::Z)))
        }
        _ => None,
    })
}

/// **The copy read through `body`, a plane-less construction, placed
/// against `target` at `at`** ([`place_against`]'s second form): an
/// `Axis` mate from the construction's own axis ([`own_axis`]) to the
/// axis a frame read off the target ([`frame_at`]) stands on where `at`
/// puts that axis, and the two values (the slide along it, the spin
/// about it) that put the copy at `at` in the mate's chart.
fn axis_against(
    doc: &crate::ProfileDoc,
    target: &Operand,
    body: &Operand,
    at: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Placed, PinRefusal> {
    axis_placed(doc, body, at, opts, tol, &|line: &Frame| {
        let frame = frame_at(doc, target, line, opts, tol)?;
        Ok(Target::Pose(Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::through(frame.clone(), PoseRole::Z),
            along: RoleRead::through(frame, PoseRole::Z),
            by: Formula::zero(Dimension::Length),
        }))))
    })
}

/// **The copy read through `body`, a plane-less construction, placed
/// in the world at `at`** ([`place_at`]'s axis form): an `Axis` mate
/// from its own axis to the world's frame moved onto the line `at` puts
/// that axis on ([`moved_by`]), read through its `z`.
fn axis_in_world(
    doc: &crate::ProfileDoc,
    body: &Operand,
    at: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Placed, PinRefusal> {
    axis_placed(doc, body, at, opts, tol, &|line: &Frame| {
        let line = line.affine::<f64>();
        Ok(Target::World(WorldRead::Role {
            pose: Box::new(world_at(line.linear, line.translation)),
            role: PoseRole::Z,
        }))
    })
}

/// [`axis_against`] and [`axis_in_world`]: `target` is handed the frame,
/// in the target's coordinates, whose `z` is the line the copy's own
/// axis stands on at `at`.
fn axis_placed(
    doc: &crate::ProfileDoc,
    body: &Operand,
    at: &Frame,
    opts: &EvalOptions,
    tol: Tol,
    target: &dyn Fn(&Frame) -> Result<Target<Formula>, PinRefusal>,
) -> Result<Placed, PinRefusal> {
    use geom_core::linalg::{Point3, Vec3};
    let band = Band::linear(tol).map_err(|_| PinRefusal::Band)?;
    let (node, solid, table) = copy_of(doc, body, opts, tol)?;
    let anchors = super::anchor::Anchors::of(doc, node);
    let no_frame = PinRefusal::NoFrame { node };
    let (construction, on, _) =
        own_axis(doc, body, &solid, &table, &anchors).ok_or(no_frame.clone())?;
    // The axis in the copy's own coordinates: a revolve's `axis`
    // output as the evaluation reads it, a ring's own `z`.
    let (origin, dir) = match doc.output(construction, 1) {
        Some(axis) => {
            let evaluation: Evaluation<f64> =
                crate::eval::evaluate(doc, None, &CancelToken::new(), opts, tol);
            match crate::eval::read_pose_in(doc, &evaluation, RoleRead::plain(axis), tol) {
                Ok(crate::pose::PoseValue::Axis { origin, dir }) => (origin, dir),
                _ => return Err(no_frame),
            }
        }
        None => (Point3::origin(), UnitVec3::exact_z()),
    };
    let seed = if dir.get().x.abs() < 0.5 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let own = RigidMotion::from_aim_and_reference(origin, dir, seed, "axis_against", band)
        .map_err(|_| no_frame.clone())?;
    let own = Frame::from_affine(own.to_affine());
    let to = target(&at.compose(&own))?;
    let placed = at.affine::<f64>();
    let unit = |v| UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band).map_err(|_| PinRefusal::Band);
    let on_value = crate::pose::PoseValue::Axis { origin, dir };
    let to_value = crate::pose::PoseValue::Axis {
        origin: placed.transform_point(origin),
        dir: unit(placed.linear * dir.get())?,
    };
    let reach = super::part_reach(&solid).map_err(|_| no_frame.clone())?;
    let arm = super::coset::Arm::of(reach, placed.translation.norm()).map_err(|_| PinRefusal::Band)?;
    let chart = super::value::Chart::of(
        &on_value,
        &to_value,
        false,
        &geom_core::Affine3::identity(),
        arm,
        band,
    )
    .map_err(|_| PinRefusal::Band)?
    .ok_or(no_frame)?;
    Ok(Placed {
        mate: Mate {
            on,
            to,
            class: super::ContactClass::Rest,
        },
        values: vec![
            (
                super::Coord::Slide(super::SlideAxis::Z),
                written(chart.slide_at(&placed, super::SlideAxis::Z), Dimension::Length),
            ),
            (
                super::Coord::Spin,
                written(chart.spin_at(&placed), Dimension::Angle),
            ),
        ],
        by: PlacedBy::Axis,
    })
}

/// **Whether `name` is an entity `node` minted**, carried through the
/// operations that keep an entity where it was (a boolean's `From`, a
/// split's fragment, a merge of coplanar faces).
fn minted_by(name: &crate::StableName, node: RecipeNodeId) -> bool {
    use crate::RoleSeg;
    let whole = &name.path[..crate::names::role::fragment_tail_start(&name.path)];
    match whole {
        [RoleSeg::From { of, .. }] | [RoleSeg::SplitFragment { parent: of }] => minted_by(of, node),
        [RoleSeg::Merged(merged)] => merged.iter().any(|n| minted_by(n, node)),
        [RoleSeg::Placed { .. } | RoleSeg::Instance { .. } | RoleSeg::InPart { .. }, ..] => false,
        _ => name.node == node,
    }
}

/// **A revolve's axis line as a world read**: the world's x axis spun
/// about its z by the heading, then slid along its x and y to the axis
/// point, each move reading what its slot reads. A slot that is the
/// constant `0` writes no move.
fn revolve_axis(
    doc: &crate::ProfileDoc,
    through: [crate::VarId; 2],
    heading: crate::VarId,
) -> WorldRead<Formula> {
    let slot = |var: crate::VarId, dim: Dimension| {
        let formula = match doc.var(var).map(crate::Var::def) {
            Some(crate::var::VarDef::Defined(expr)) => Formula::from(expr),
            _ => Formula::var(var, dim),
        };
        (!formula.is_zero()).then_some(formula)
    };
    let mut axis = WorldRead::through(PoseRole::X);
    if let Some(by) = slot(heading, Dimension::Angle) {
        axis = axis.spin(WorldRead::through(PoseRole::Z), by);
    }
    for (role, var) in [(PoseRole::X, through[0]), (PoseRole::Y, through[1])] {
        if let Some(by) = slot(var, Dimension::Length) {
            axis = axis.slide(WorldRead::through(role), by);
        }
    }
    axis
}

/// **A frame read off `body`'s own geometry, standing at `at` in its
/// own coordinates** (spec §5, the restate ladder's second rung): the
/// frame [`pin_at`] reads (`Through` an axis-bearing edge and a vertex
/// off it, preferring an anchor no variable reaches), spun about that
/// frame's own x, y and z and slid along them to `at` — the angles taken
/// once by `atan2` and the lengths by projection, as [`moved_by`] takes
/// a world read's. Read through a plane role it is a split's tool or a
/// mate's target at an absolute pose of the body's own.
///
/// # Errors
///
/// [`PinRefusal`], as [`pin_at`]'s.
pub fn frame_at(
    doc: &crate::ProfileDoc,
    body: &Operand,
    at: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Operand, PinRefusal> {
    let band = Band::linear(tol).map_err(|_| PinRefusal::Band)?;
    let (node, solid, table) = copy_of(doc, body, opts, tol)?;
    let name = |key| table.name_of(&EntityRef { body: 0, key }).cloned();
    let anchors = super::anchor::Anchors::of(doc, node);
    let picked = picked_frame(&solid, band, &|key| name(key).filter(|n| anchors.fixed(n)))
        .or_else(|| picked_frame(&solid, band, &name));
    let (edge_name, vertex_name, frame) = picked.ok_or(PinRefusal::NoFrame { node })?;
    let read = |def: PoseDef<Operand, Formula>| RoleRead::plain(Operand::Pose(Box::new(def)));
    let base = Operand::Pose(Box::new(PoseDef::Through {
        axis: read(PoseDef::Axis {
            of: Operand::select(body.clone(), vec![edge_name]),
        }),
        point: read(PoseDef::Point {
            of: Operand::select(body.clone(), vec![vertex_name]),
        }),
    }));
    let rel = frame.to_affine().inverse() * at.affine();
    Ok(moved_pose(base, rel.linear, rel.translation))
}

/// **`base`, a frame read, moved by the rigid motion `(linear,
/// translation)` written in its own axes**: spun about its x, then y,
/// then z — `linear = Rz(γ)·Ry(β)·Rx(α)` — and slid along them, each
/// move about the base frame's own fixed axes, so the moves compose as
/// [`moved_by`]'s about the world's. A move by exactly zero is no move
/// written.
fn moved_pose(
    base: Operand,
    linear: Mat3<f64>,
    translation: geom_core::linalg::Vec3<f64>,
) -> Operand {
    let r = |i: usize, j: usize| {
        let c = [linear.c0, linear.c1, linear.c2][j];
        [c.x, c.y, c.z][i]
    };
    let cos_beta = r(0, 0).hypot(r(1, 0));
    let beta = (-r(2, 0)).atan2(cos_beta);
    let (alpha, gamma) = if cos_beta > f64::EPSILON {
        (r(2, 1).atan2(r(2, 2)), r(1, 0).atan2(r(0, 0)))
    } else {
        ((-r(1, 2)).atan2(r(1, 1)), 0.0)
    };
    let axis = |role| RoleRead::through(base.clone(), role);
    let mut pose = base.clone();
    for (role, angle) in [
        (PoseRole::X, alpha),
        (PoseRole::Y, beta),
        (PoseRole::Z, gamma),
    ] {
        if angle != 0.0 {
            pose = Operand::Pose(Box::new(PoseDef::Spin {
                pose: RoleRead::plain(pose),
                about: axis(role),
                by: written(angle, Dimension::Angle),
            }));
        }
    }
    for (role, length) in [
        (PoseRole::X, translation.x),
        (PoseRole::Y, translation.y),
        (PoseRole::Z, translation.z),
    ] {
        if length != 0.0 {
            pose = Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(pose),
                along: axis(role),
                by: written(length, Dimension::Length),
            }));
        }
    }
    pose
}

/// **The copy read through `body` placed against `target`'s geometry at
/// `at`** — `at` the copy's own frame in `target`'s own coordinates
/// (spec §5: a construction combined with another body is placed
/// against that body's root). A plane mate on the copy's face that lies
/// on its own `xy` and reads no variable (an extrude's start cap) to
/// the target's such face, slid along its normal to where the copy's
/// face stands at `at` — or, where `at` tilts the face off the target's
/// normal, a frame read off the target ([`frame_at`]) through its `xy`
/// — and the three values that put the copy at `at` in the mate's
/// chart, computed once. A copy with no such face that is a revolve or
/// a full ring is placed by its own axis instead ([`axis_against`]).
///
/// # Errors
///
/// [`PinRefusal`]: either body has no value or no face on its own
/// `xy`, or the target offers no frame for a tilted face.
pub fn place_against(
    doc: &crate::ProfileDoc,
    target: &Operand,
    body: &Operand,
    at: &Frame,
    opts: &EvalOptions,
    tol: Tol,
) -> Result<Placed, PinRefusal> {
    use geom_core::linalg::{Point3, Vec3};
    let band = Band::linear(tol).map_err(|_| PinRefusal::Band)?;
    let own_plane = |shape: &Operand| -> Result<(crate::StableName, f64, f64), PinRefusal> {
        let (node, solid, table) = copy_of(doc, shape, opts, tol)?;
        let anchors = super::anchor::Anchors::of(doc, node);
        let reach = super::part_reach(&solid).map_err(|_| PinRefusal::NoFrame { node })?;
        solid
            .faces()
            .find_map(|(face, _)| {
                let kind = topo::readback::face_carrier_kind(&solid, face).ok()?;
                if kind != geom::SurfaceKind::Plane {
                    return None;
                }
                let name = table.name_of(&EntityRef {
                    body: 0,
                    key: EntityKey::Face(face),
                })?;
                if !anchors.fixed(name) {
                    return None;
                }
                let at = topo::readback::face_pose(&solid, face).ok()?;
                let n = geom_brep::OutwardNormal::from_chart(at.axis, at.sense).vec();
                [at.origin.z, n.x, n.y]
                    .into_iter()
                    .all(|c| c == 0.0)
                    .then(|| (name.clone(), n.z.signum(), reach))
            })
            .ok_or(PinRefusal::NoFrame { node })
    };
    let (copy_face, copy_z, copy_reach) = match own_plane(body) {
        Ok(plane) => plane,
        Err(refusal) => {
            return axis_against(doc, target, body, at, opts, tol).map_err(|_| refusal);
        }
    };
    let (target_face, target_z, _) = own_plane(target)?;
    let plane = |shape: &Operand, face: crate::StableName| {
        Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(shape.clone(), vec![face]),
        }))
    };
    let flip = |pose: Operand| {
        Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::plain(pose),
        }))
    };
    let placed = at.affine::<f64>();
    let z = Vec3::new(0.0, 0.0, 1.0);
    // The copy's face's normal where `at` puts it, in the target's
    // coordinates.
    let m = placed.linear * (z * copy_z);
    let untilted = m.x == 0.0 && m.y == 0.0;
    let (on, to, to_normal) = if untilted {
        // The target's face slid along its own normal onto the copy's.
        let along = target_z * placed.translation.z * -1.0;
        let to_plane = plane(target, target_face);
        let to = if along == 0.0 {
            to_plane
        } else {
            Operand::Pose(Box::new(PoseDef::Slide {
                pose: RoleRead::plain(to_plane.clone()),
                along: RoleRead::through(to_plane, PoseRole::Normal),
                by: written(along, Dimension::Length),
            }))
        };
        let on = plane(body, copy_face);
        let on = if m.z == target_z { on } else { flip(on) };
        (on, to, z * target_z)
    } else {
        // A frame read off the target, standing where the copy's face
        // stands with its `z` the face's normal.
        let turn = if copy_z > 0.0 {
            Frame::IDENTITY
        } else {
            Frame {
                columns: [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]],
                translation: [0.0; 3],
            }
        };
        let frame = frame_at(doc, target, &at.compose(&turn), opts, tol)?;
        let to = Operand::Pose(Box::new(PoseDef::Slide {
            pose: RoleRead::through(frame.clone(), PoseRole::Xy),
            along: RoleRead::through(frame, PoseRole::Z),
            by: Formula::zero(Dimension::Length),
        }));
        (plane(body, copy_face), to, m)
    };
    let unit = |v| UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band).map_err(|_| PinRefusal::Band);
    let on_value = crate::pose::PoseValue::Plane {
        origin: Point3::origin(),
        normal: unit(placed.linear.transpose() * to_normal)?,
    };
    let to_value = crate::pose::PoseValue::Plane {
        origin: Point3::origin() + placed.translation,
        normal: unit(to_normal)?,
    };
    let datum = placed.translation.norm();
    let arm = super::coset::Arm::of(copy_reach, datum).map_err(|_| PinRefusal::Band)?;
    let chart = super::value::Chart::of(
        &on_value,
        &to_value,
        false,
        &geom_core::Affine3::identity(),
        arm,
        band,
    )
    .map_err(|_| PinRefusal::Band)?
    .unwrap_or_else(|| unreachable!("a plane mate has a chart"));
    Ok(Placed {
        mate: Mate {
            on,
            to: Target::Pose(to),
            class: super::ContactClass::Rest,
        },
        values: vec![
            (
                super::Coord::Slide(super::SlideAxis::X),
                written(chart.slide_at(&placed, super::SlideAxis::X), Dimension::Length),
            ),
            (
                super::Coord::Slide(super::SlideAxis::Y),
                written(chart.slide_at(&placed, super::SlideAxis::Y), Dimension::Length),
            ),
            (
                super::Coord::Spin,
                written(chart.spin_at(&placed), Dimension::Angle),
            ),
        ],
        by: PlacedBy::PlanarFace,
    })
}
