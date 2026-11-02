//! **A frame read off a body's own geometry at a corner it has**
//! ([`corner_frame`]): the one reader of an evaluation among the
//! placement writers, and it writes no number — it finds the names of
//! the edge and vertex at corners its caller states exactly, and reads
//! the frame `Through` them, so a frame a caller means as axis-aligned
//! is written as one.

use geom_core::Tol;

use crate::eval::{EvalOptions, Evaluation, NodeResult, ValuePayload};
use crate::names::{EntityKey, EntityRef};
use crate::node::RecipeNodeId;
use crate::pose::{PoseDef, RoleRead};
use crate::{CancelToken, Formula, Operand};

/// **Why a frame could not be read off a body's geometry**.
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
    /// coordinates' (a corner is read off a body before placement).
    AlreadyPlaced {
        /// The node.
        node: RecipeNodeId,
    },
    /// The body has no line edge between the two corners, or no vertex
    /// at the third: no frame can be read off it there.
    NoFrame {
        /// The node.
        node: RecipeNodeId,
    },
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
                "{node} has no line edge between the corners named and no vertex at the \
                 third, so no frame is read off it there; read a frame off corners it has"
            ),
        }
    }
}

impl core::error::Error for PinRefusal {}

/// **The copy read through `body`, evaluated as it stands, before
/// placement**: its node, its first solid and its name table.
fn copy_of(
    doc: &crate::ProfileDoc,
    evaluation: &Evaluation<f64>,
    body: &Operand,
) -> Result<
    (
        RecipeNodeId,
        crate::eval::Framed<f64>,
        crate::names::NameTable,
    ),
    PinRefusal,
> {
    match read_of(doc, evaluation, body)? {
        (node, _, _, Some(_)) => Err(PinRefusal::AlreadyPlaced { node }),
        (node, solid, table, None) => Ok((node, solid, table)),
    }
}

/// What [`read_of`] reads: the node, its first solid, its name table and
/// the motion its placement moved it by.
pub(crate) type ReadBody = (
    RecipeNodeId,
    crate::eval::Framed<f64>,
    crate::names::NameTable,
    Option<geom_core::RigidMotion<f64>>,
);

/// **The body read through `body`, placed or not**: [`copy_of`]'s
/// answer, with the motion its placement moved it by (`None` for a
/// body that stands where its construction put it). A frame read off a
/// placed copy is read in the copy's space, and the motion carries a
/// frame given in the copy's own coordinates there.
pub(crate) fn read_of(
    doc: &crate::ProfileDoc,
    evaluation: &Evaluation<f64>,
    body: &Operand,
) -> Result<ReadBody, PinRefusal> {
    let node = match body {
        Operand::Output { node, .. } | Operand::Node(node) => *node,
        Operand::Var(var) => doc.operation_of(*var).ok_or(PinRefusal::NoNode)?,
        _ => return Err(PinRefusal::NoNode),
    };
    let Some(NodeResult::Ok(value)) = evaluation.result(node) else {
        return Err(PinRefusal::Unevaluated { node });
    };
    // A family is read through its first member, in that member's own
    // names: what a reader evaluated once per member reads at each
    // (D10 `[repetition]`).
    if let Some(members) = &value.members {
        let (_, first) = members
            .members
            .first()
            .ok_or(PinRefusal::NotABody { node })?;
        let (ValuePayload::Body(solid)
        | ValuePayload::Boolean(crate::eval::BooleanValue::Body { body: solid, .. })) =
            &first.payload
        else {
            return Err(PinRefusal::NotABody { node });
        };
        let table = first
            .output(0)
            .ok_or(PinRefusal::NotABody { node })?
            .table();
        return Ok((node, solid.clone(), table.clone(), first.motion));
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
        | ValuePayload::Boolean(crate::eval::BooleanValue::Body { body: b, .. }) => b.clone(),
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
                crate::eval::SplitSide::Body(b) => b.clone(),
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
    Ok((node, solid, table.clone(), value.motion))
}

/// **A frame read off `body`'s own geometry at a corner it has**: the
/// frame `Through` the line edge from `corner` to `up` (its `z`, that
/// way) and the vertex at `toward` (its `x` the way to it), each picked
/// where the evaluation puts it exactly, in the body's own coordinates.
/// Where a box's edges run along its own axes this is its own frame,
/// read with no number, so a frame stated from it by exact moves
/// ([`super::pin::moved_frame`]) stands exactly where it is meant — a frame a
/// caller means as axis-aligned, written as one.
///
/// # Errors
///
/// [`PinRefusal`]: the body has no value or is placed already, or it
/// has no line edge from `corner` to `up` or no vertex at `toward`
/// ([`PinRefusal::NoFrame`]).
pub fn corner_frame_in(
    doc: &crate::ProfileDoc,
    ev: &Evaluation<f64>,
    body: &Operand,
    corner: [f64; 3],
    up: [f64; 3],
    toward: [f64; 3],
) -> Result<Operand, PinRefusal> {
    use geom_core::linalg::Point3;
    let (node, framed, table) = copy_of(doc, ev, body)?;
    // The corners are stated in the body's space: its geometry is read
    // there.
    let solid = framed.body();
    let no_frame = || PinRefusal::NoFrame { node };
    let at = |p: [f64; 3]| Point3::new(p[0], p[1], p[2]);
    let (corner, up, toward) = (at(corner), at(up), at(toward));
    let name = |key| {
        table
            .name_of(&EntityRef { body: 0, key })
            .cloned()
            .ok_or_else(no_frame)
    };
    let vertex_at = |p: Point3<f64>| {
        solid
            .vertices()
            .find(|(v, _)| {
                topo::readback::vertex_point(solid, *v)
                    .map(|q| framed.point_in_space(q))
                    .is_ok_and(|q| [q.x, q.y, q.z] == [p.x, p.y, p.z])
            })
            .map(|(v, _)| v)
    };
    let (from, to) = (
        vertex_at(corner).ok_or_else(no_frame)?,
        vertex_at(up).ok_or_else(no_frame)?,
    );
    let edge = solid
        .edges()
        .find(|(e, _)| {
            topo::readback::edge_carrier_kind(solid, *e) == Ok(geom::CurveKind::Line)
                && solid.get_edge(*e).is_some_and(|edge| {
                    let start = solid.get_half_edge(edge.he_plus).map(|h| h.start);
                    let end = solid.half_edge_end(edge.he_plus);
                    matches!((start, end), (Some(a), Some(b))
                        if (a == from && b == to) || (a == to && b == from))
                })
        })
        .map(|(e, _)| e)
        .ok_or_else(no_frame)?;
    let line = topo::readback::edge_pose(solid, edge).map_err(|_| no_frame())?;
    let read = |def: PoseDef<Operand, Formula>| RoleRead::plain(Operand::Pose(Box::new(def)));
    let axis = read(PoseDef::Axis {
        of: Operand::select(body.clone(), vec![name(EntityKey::Edge(edge))?]),
    });
    // The edge's axis reads its carrier's sense; the frame's `z` runs
    // from `corner` to `up`.
    let axis = if framed.vec_in_space(line.axis).dot(up - corner) < 0.0 {
        read(PoseDef::Flip { pose: axis })
    } else {
        axis
    };
    let point = read(PoseDef::Point {
        of: Operand::select(
            body.clone(),
            vec![name(EntityKey::Vertex(
                vertex_at(toward).ok_or_else(no_frame)?,
            ))?],
        ),
    });
    Ok(Operand::Pose(Box::new(PoseDef::Through { axis, point })))
}

/// [`corner_frame_in`], over an evaluation of `doc` under `opts` made
/// here, which `cancel` cancels.
///
/// # Errors
///
/// As [`corner_frame_in`]'s.
pub fn corner_frame(
    doc: &crate::ProfileDoc,
    body: &Operand,
    corner: [f64; 3],
    up: [f64; 3],
    toward: [f64; 3],
    opts: &EvalOptions,
    cancel: &CancelToken,
    tol: Tol,
) -> Result<Operand, PinRefusal> {
    let ev: Evaluation<f64> = crate::eval::evaluate(doc, None, cancel, opts, tol);
    corner_frame_in(doc, &ev, body, corner, up, toward)
}
