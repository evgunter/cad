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
//! This is a helper for migration, tests and demos, not an authoring
//! gesture. The authoring gesture is [`place_at`]: "place this copy
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
    // The first edge by id with an axis, and the first vertex by id off
    // its line, as the evaluation reads them.
    let mut picked = None;
    'edges: for (edge, _) in solid.edges() {
        let Ok(kind) = topo::readback::edge_carrier_kind(&solid, edge) else {
            continue;
        };
        if !matches!(
            kind,
            geom::CurveKind::Line | geom::CurveKind::Circle | geom::CurveKind::Ellipse
        ) {
            continue;
        }
        let Ok(line) = topo::readback::edge_pose(&solid, edge) else {
            continue;
        };
        let Ok(dir) = UnitVec3::new(line.axis, topo::query::DATUM_UNIT_NORM, band) else {
            continue;
        };
        let Some(edge_name) = name(EntityKey::Edge(edge)) else {
            continue;
        };
        for (vertex, _) in solid.vertices() {
            let Ok(point) = topo::readback::vertex_point(&solid, vertex) else {
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

/// **The world's frame moved to `(linear, translation)`**
/// ([`moved_by`] of the frame itself).
fn world_at(linear: Mat3<f64>, translation: geom_core::linalg::Vec3<f64>) -> WorldRead<Formula> {
    moved_by(WorldRead::Frame, linear, translation)
}

/// **A number read back from a pose, as a literal**, written without a
/// sign on its zero: `-0.0` and `0.0` are one value, and a held
/// variable binds them alike, so the written literal is the one both
/// read. The one writer of `pin_at`'s and `place_at`'s numbers.
fn written(value: f64, dim: Dimension) -> Formula {
    Formula::literal(value + 0.0, dim)
        .unwrap_or_else(|_| unreachable!("a finite literal of {dim:?}"))
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

/// **How [`place_at`] placed a copy.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacedBy {
    /// A plane mate on the copy's first planar face and its three
    /// values.
    PlanarFace,
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
    let planar = solid.faces().find_map(|(face, _)| {
        let kind = topo::readback::face_carrier_kind(&solid, face).ok()?;
        if kind != geom::SurfaceKind::Plane {
            return None;
        }
        let at = topo::readback::face_pose(&solid, face).ok()?;
        let normal = geom_brep::OutwardNormal::from_chart(at.axis, at.sense).vec();
        let normal = UnitVec3::new(normal, topo::query::DATUM_UNIT_NORM, band).ok()?;
        let name = table.name_of(&EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        })?;
        Some((name.clone(), at.origin, normal))
    });
    let pinned = || {
        Ok(Placed {
            mate: pin_at(doc, body, pose, opts, tol)?,
            values: Vec::new(),
            by: PlacedBy::Pin,
        })
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
