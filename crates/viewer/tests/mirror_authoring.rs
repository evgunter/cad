//! **The mirror tool, replayed headlessly**: a body and a picked face
//! through the selection stream the viewport feeds, one committed
//! `Node::Mirror` placed in the world, its typed refusals, and the
//! saved document read back.
//!
//! Every geometric row compares the image's corners with its source's
//! reflected across the picked face's plane, worked out from the box's
//! literal dimensions — a row goes red when the image stops being that
//! reflection.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{along_x, displayed_face_at, plate_index, session_insert};
use pncad::document::pose::PoseDef;
use pncad::document::{
    Dimension, Doc, DocEdit, EditError, Formula, LoopProgram, Node, NodeErrorKind, NodeResult,
    Operand, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use viewer::combine::MirrorEvent;
use viewer::seats::SeatError;
use viewer::session::{DocSession, FaceSelection, Refusal, Selection, SessionOp};
use viewer::tools::{ToolKind, ToolNotice, Tools};
use viewer::tree::RowStatus;

/// The block: 40 × 20 × 10 mm, centred on the sketch origin and
/// extruded up, so its `−x` wall stands at `x = −HALF_X`.
const BLOCK: [f64; 3] = [0.04, 0.02, 0.01];
/// Half the block's width along x.
const HALF_X: f64 = BLOCK[0] / 2.0;
/// A height every level ray aims at: inside the block's walls.
const MID_Z: f64 = BLOCK[2] / 2.0;

/// A session holding the block, landed — answering the extrude.
fn block(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("mirror-start", tol), tol);
    let block = common::xy_box_in(&mut session, BLOCK);
    session.pump();
    (session, block)
}

/// The face a level ray along x travelling toward `sense` meets first,
/// picked the way the viewport picks it.
fn face_toward(session: &DocSession, sense: f64) -> FaceSelection {
    let index = plate_index(session);
    displayed_face_at(session, &index, &along_x(sense, 0.0, MID_Z))
}

/// `node`'s evaluated body's points, sorted.
fn corners(session: &mut DocSession, node: RecipeNodeId) -> Vec<[f64; 3]> {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let value = eval
        .value(node)
        .unwrap_or_else(|| panic!("{node:?} evaluated: {:?}", eval.node_error(node)));
    let ValuePayload::Body(body) = &value.payload else {
        panic!("{node:?} is a body, not {:?}", value.payload)
    };
    let mut points: Vec<[f64; 3]> = body.points().map(|(_, p)| p.to_array()).collect();
    points.sort_by(|a, b| a.partial_cmp(b).expect("finite points"));
    points
}

/// `points` reflected across the plane `x = at`, sorted.
fn reflected(points: &[[f64; 3]], at: f64) -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = points
        .iter()
        .map(|&[x, y, z]| [2.0 * at - x, y, z])
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).expect("finite points"));
    out
}

/// Two sorted point lists agree point for point to 1e-12 m.
fn same_points(got: &[[f64; 3]], want: &[[f64; 3]]) -> bool {
    got.len() == want.len()
        && got
            .iter()
            .zip(want)
            .all(|(g, w)| g.iter().zip(w).all(|(a, b)| (a - b).abs() <= 1e-12))
}

/// Feed `faces` to an open mirror tool, every pick taken.
fn mirror_tool_picking(session: &DocSession, faces: &[FaceSelection]) -> Tools {
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picks: Vec<SessionOp> = faces
        .iter()
        .map(|face| SessionOp::Select(Selection::Face(face.clone())))
        .collect();
    assert!(
        tools.feed(session.committed_doc(), &picks).is_empty(),
        "every pick is taken"
    );
    tools
}

/// The mirror tool's committed op, which closes the tool.
fn mirror_op(tools: &Tools) -> SessionOp {
    let op = tools
        .mirror()
        .expect("the mirror tool is open")
        .op()
        .expect("a body and a plane are held");
    assert!(
        tools.commits_open_tool(&op),
        "the mirror is the tool's edit"
    );
    op
}

/// **A body mirrored across one of its own faces**: one face pick holds
/// both the body and the plane, the commit inserts one `Node::Mirror`
/// whose plane is that face read as a plane and places the image in the
/// world, the image is the block reflected across `x = −HALF_X`, and the
/// saved document reads back bit for bit and builds the same image.
#[test]
fn a_body_mirrored_across_its_own_face_authors_evaluates_saves_and_reloads() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let wall = face_toward(&session, 1.0);
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let tool = tools.mirror().expect("open");
    assert_eq!(
        tool.input(),
        Some(block),
        "a pick on the copy seats its body"
    );
    assert_eq!(tool.plane(), Some(&wall), "and its face is the plane");

    let outcome = session.perform(mirror_op(&tools));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let inserted: Vec<_> = outcome
        .committed
        .iter()
        .filter_map(|edit| match edit {
            DocEdit::InsertNode { node, .. } => Some(&**node),
            _ => None,
        })
        .collect();
    let [Node::Mirror { body, plane }, Node::Place { .. }] = inserted[..] else {
        panic!("a mirror and its world placement: {inserted:?}")
    };
    assert_eq!(
        body.read,
        Operand::Node(block),
        "the image reflects the block"
    );
    assert!(plane.role.is_none(), "the plane is read plainly");
    let Operand::Pose(pose) = &plane.read else {
        panic!("the plane is a pose read, not {:?}", plane.read)
    };
    let PoseDef::Plane {
        face: Operand::Select { names, .. },
    } = &**pose
    else {
        panic!("the plane is a picked face's plane, not {pose:?}")
    };
    let (_, own) = wall
        .name
        .copy_of()
        .expect("the pick is on the block's copy");
    assert_eq!(
        names,
        &vec![own.clone()],
        "by the block's own name for the face"
    );
    let image = outcome.minted[0];
    assert!(
        common::world(session.committed_doc()).contains(&image),
        "the image is placed in the world, beside the block"
    );
    assert!(
        common::world(session.committed_doc()).contains(&block),
        "and the block stays placed"
    );

    let source = corners(&mut session, block);
    let want = reflected(&source, -HALF_X);
    let built = corners(&mut session, image);
    assert!(
        same_points(&built, &want),
        "the image is the block across x = {}: {built:?} vs {want:?}",
        -HALF_X
    );
    let rows = session.tree_rows();
    assert_eq!(
        common::status_of(&rows, image),
        RowStatus::Ok,
        "the mirror's row"
    );

    let dir = common::tempdir("authtail-mirror");
    let path = dir.join("mirror.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    let authored = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        session.committed_doc().bit_eq(&authored),
        "the reloaded document is the authored one, bit for bit"
    );
    let reloaded = corners(&mut session, image);
    assert_eq!(
        reloaded
            .iter()
            .flatten()
            .map(|c| c.to_bits())
            .collect::<Vec<_>>(),
        built
            .iter()
            .flatten()
            .map(|c| c.to_bits())
            .collect::<Vec<_>>(),
        "the reloaded image is the authored one, bit for bit"
    );
}

/// **A body mirrored across a face of another body in its space**: the
/// block's duplicate is a copy read out of a pattern of the block, so
/// it lives in the block's space. The first face pick (the block's)
/// seats the block; the second (the duplicate's far `+x` wall) re-picks
/// the plane alone; the image is the block reflected across that wall.
///
/// A face of a body in ANOTHER space — a second, independent block —
/// refuses at the insert door naming both spaces, and nothing lands.
#[test]
fn a_body_mirrored_across_another_bodys_face_reads_that_faces_plane() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let duplicated = session.perform(SessionOp::Duplicate {
        input: block.into(),
    });
    assert!(duplicated.refusal.is_none(), "{:?}", duplicated.refusal);
    session.pump();
    let world = common::world(session.committed_doc());
    let [first, copy] = world[..] else {
        panic!("the block and its duplicate are drawn: {world:?}")
    };
    assert_eq!(first, block, "the block is drawn first");

    let own = face_toward(&session, 1.0);
    let far = face_toward(&session, -1.0);
    let tools = mirror_tool_picking(&session, &[own, far.clone()]);
    let tool = tools.mirror().expect("open");
    assert_eq!(tool.input(), Some(block), "the second face keeps the body");
    assert_eq!(tool.plane(), Some(&far), "and re-picks the plane");

    let at = corners(&mut session, copy)
        .iter()
        .map(|p| p[0])
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        at > HALF_X,
        "the duplicate's far wall is clear of the block: {at}"
    );
    let image = session_insert(&mut session, mirror_op(&tools));
    let source = corners(&mut session, block);
    let want = reflected(&source, at);
    let built = corners(&mut session, image);
    assert!(
        same_points(&built, &want),
        "the image is the block across x = {at}: {built:?} vs {want:?}"
    );

    // A second block is its own construction, in a space of its own.
    let other = common::xy_box_in(&mut session, BLOCK);
    let before = session.committed_doc().clone();
    let refused = session.perform(SessionOp::AddMirror {
        body: block.into(),
        plane: FaceSelection {
            name: own_cap(other),
            node: other,
            body: 0,
        },
    });
    let spaces = [
        before.node_space(block).expect("the block is in its space"),
        before.node_space(other).expect("and the other in its own"),
    ];
    assert!(
        matches!(
            refused.refusal.as_ref(),
            Some(Refusal::Edit(error)) if matches!(
                error.as_ref(),
                EditError::SpaceMismatch { a, b, .. }
                    if spaces.contains(&a.id()) && spaces.contains(&b.id())
            )
        ),
        "a plane off another space refuses at the door, naming both: {:?}",
        refused.refusal
    );
    assert!(refused.committed.is_empty(), "nothing is recorded");
    assert!(
        session.committed_doc().bit_eq(&before),
        "the document stands"
    );
}

/// An extrude's end cap, by its own name.
fn own_cap(node: RecipeNodeId) -> pncad::prelude::StableName {
    pncad::prelude::StableName {
        kind: pncad::prelude::EntityKind::Face,
        node,
        path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
    }
}

/// **A face with no plane refuses typed, on the mirror's own row**: a
/// torus mirrored across its curved wall. Whether a face is planar
/// is the evaluation's to say (`PoseReadFault::NotPlanar`), as it is
/// for a mate's face, so the edit lands and the mirror refuses where it
/// evaluates, naming the plane it could not read.
///
/// Before any face is held, the commit refuses naming the plane.
#[test]
fn a_mirror_across_a_curved_face_refuses_typed_where_it_evaluates() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("mirror-curved", tol), tol);
    // A circle off the revolve's axis: the revolve is a torus about
    // world +y, every face of it curved.
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![LoopProgram::Circle {
                centre: [common::len(0.02), Formula::zero(Dimension::Length)],
                radius: common::len(0.005),
            }],
        },
    );
    let torus = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [
                Formula::zero(Dimension::Length),
                Formula::zero(Dimension::Length),
            ],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: common::ang(core::f64::consts::TAU),
        },
    );
    session.pump();

    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    assert!(
        tools
            .feed(
                session.committed_doc(),
                &[SessionOp::Select(Selection::Node(common::copy_of(
                    session.committed_doc(),
                    torus
                )))]
            )
            .is_empty(),
        "a tree pick of the drawn copy seats the body"
    );
    match tools.mirror().expect("open").op() {
        Err(SeatError::NoFace { role }) => {
            assert_eq!(role, viewer::combine::MIRROR_PLANE, "the plane is named");
        }
        other => panic!("no plane held yet refuses typed, not {other:?}"),
    }

    let index = plate_index(&session);
    let wall = displayed_face_at(&session, &index, &along_x(1.0, 0.002, 0.003));
    let _ = tools.feed(
        session.committed_doc(),
        &[SessionOp::Select(Selection::Face(wall))],
    );
    assert_eq!(
        tools.mirror().and_then(|t| t.input()),
        Some(torus),
        "a face pick with a body held re-picks the plane alone"
    );
    let image = session_insert(&mut session, mirror_op(&tools));
    session.pump();
    let eval = session.evaluation().expect("landed");
    let refusal = match eval.nodes.get(&image) {
        Some(NodeResult::Failed(refusal)) => refusal,
        other => panic!("the mirror refuses on its own row, not {other:?}"),
    };
    assert!(
        matches!(
            &refusal.kind,
            NodeErrorKind::PoseRead {
                pose: pncad::document::VarKind::Plane,
                fault: pncad::document::pose::PoseReadFault::NotPlanar { .. },
            }
        ),
        "a curved face has no plane to read: {refusal:?}"
    );
    let rows = session.tree_rows();
    assert!(
        matches!(common::status_of(&rows, image), RowStatus::Failed { .. }),
        "and the tree badges the mirror itself"
    );
}

/// **The plane survives as a face does**: deleting the block drops the
/// body seat (by the document) and the plane (by its resolution against
/// the landed evaluation), each said once.
#[test]
fn a_deleted_body_drops_the_seat_and_the_plane() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let wall = face_toward(&session, 1.0);
    let mut tools = mirror_tool_picking(&session, &[wall]);
    let outcome = session.perform(SessionOp::DeleteNode { node: block });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    let seat = notices
        .iter()
        .filter(|n| {
            matches!(
                n,
                ToolNotice::Seated {
                    tool: ToolKind::Mirror,
                    ..
                }
            )
        })
        .count();
    let plane = notices
        .iter()
        .filter(|n| matches!(n, ToolNotice::Mirror(MirrorEvent::PlaneLost { .. })))
        .count();
    assert_eq!((seat, plane), (1, 1), "one drop each: {notices:?}");
    let tool = tools.mirror().expect("still open");
    assert_eq!((tool.input(), tool.plane()), (None, None), "both are empty");
    assert!(
        tools
            .reconcile(session.doc(), session.landed_pair())
            .is_empty(),
        "and a drop is said once"
    );
}

/// **The symmetric part is the union of the body and its image**,
/// authored with the boolean tool after the mirror, in either pick
/// order: the union's volume is twice the block's, and it is the one
/// body the world draws.
#[test]
fn a_body_and_its_image_union_to_the_symmetric_part_in_either_order() {
    let tol = Tol::witness();
    let volume = BLOCK.iter().product::<f64>();
    for image_first in [false, true] {
        let (mut session, block) = block(tol);
        let wall = face_toward(&session, 1.0);
        let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
        let image = session_insert(&mut session, mirror_op(&tools));
        let picks = if image_first {
            [image, block]
        } else {
            [block, image]
        };
        let mut tools = Tools::new();
        tools.open(ToolKind::Boolean);
        let picks: Vec<SessionOp> = picks
            .into_iter()
            .map(|node| SessionOp::Select(Selection::Node(node)))
            .collect();
        assert!(tools.feed(session.committed_doc(), &picks).is_empty());
        let op = tools.boolean().expect("open").op().expect("two members");
        let union = session_insert(&mut session, op);
        let got = common::body_volume(&mut session, union, tol);
        assert!(
            common::near(got, 2.0 * volume),
            "image first {image_first}: the symmetric part is twice the block, {got}"
        );
        assert_eq!(
            common::world(session.committed_doc()),
            [union],
            "image first {image_first}: the world draws the union alone"
        );
    }
}
