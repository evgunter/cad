//! **The mirror tool, replayed headlessly**: a body and a picked face
//! through the selection stream the viewport feeds, one committed
//! action — the image, and for a body the world draws, the symmetric
//! part taking over its copies — its typed refusals, and the saved
//! document read back.
//!
//! Every geometric row compares with its source reflected across the
//! picked face's plane, worked out from the boxes' literal dimensions:
//! the image's own corners, and the DRAWN symmetric part's extent and
//! volume where it stands in the world.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::drawn::{self, assert_drawn, corners};
use common::{along_x, displayed_face_at, plate_index, session_insert};
use pncad::document::SlotId;
use pncad::document::pose::PoseDef;
use pncad::document::{
    Dimension, Doc, DocEdit, EditError, Formula, LoopProgram, Node, NodeErrorKind, NodeResult,
    Operand, RecipeNodeId,
};
use pncad::geom_core::Tol;
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
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks
            )
            .is_empty(),
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

/// The small block a mate seats on the block's top: 20 × 10 × 6 mm.
const SMALL: [f64; 3] = [0.02, 0.01, 0.006];

/// The block's volume.
fn volume_of(size: [f64; 3]) -> f64 {
    size.iter().product()
}

/// **The mirror tool's commit, performed**: the image, and the union of
/// the symmetric part where one was made.
fn mirror_in(session: &mut DocSession, op: SessionOp) -> (RecipeNodeId, Option<RecipeNodeId>) {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let doc = session.committed_doc();
    let image = *outcome
        .minted
        .iter()
        .find(|&&n| matches!(doc.node(n), Some(Node::Mirror { .. })))
        .expect("the commit mints the mirror");
    let part = outcome
        .minted
        .iter()
        .copied()
        .find(|&n| matches!(doc.node(n), Some(Node::Union { .. })));
    (image, part)
}

/// The block's world copy pinned at the translation `at`: the session,
/// the block and its copy.
fn moved_block(tol: Tol, at: [f64; 3]) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (session, block) = block(tol);
    let (session, copy) = drawn::moved_copy(&session, block, &drawn::translated(at), tol);
    (session, block, copy)
}

/// An extrude's end cap, by its own name.
fn own_cap(node: RecipeNodeId) -> pncad::prelude::StableName {
    pncad::prelude::StableName {
        kind: pncad::prelude::EntityKind::Face,
        node,
        path: vec![pncad::select::RoleSeg::Cap(pncad::select::CapEnd::End)],
    }
}

/// **A body mirrored across one of its own faces becomes its symmetric
/// part**: one face pick holds the body and the plane; the commit
/// inserts the image, whose plane is that face read as a plane, and the
/// union of the block and its image, and re-points the block's one
/// drawn copy to the union. The image is the block reflected across
/// x = −20 mm, the copy draws the 80 mm part, and the saved document
/// reads back bit for bit and builds the same image.
#[test]
fn a_body_mirrored_across_its_own_face_becomes_its_symmetric_part() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let copy = common::copy_of(session.committed_doc(), block);
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
    let [Node::Mirror { body, plane }, Node::Union { .. }] = inserted[..] else {
        panic!("the image and the symmetric part, no placement: {inserted:?}")
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
    // A copy carries the block's own names, so the pick on the copy is
    // the block's own name for the face.
    let own = &wall.name;
    assert_eq!(
        names,
        &vec![own.clone()],
        "by the block's own name for the face"
    );
    let [image, part] = outcome.minted[..] else {
        panic!("two nodes minted: {:?}", outcome.minted)
    };
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::drawn_copies(doc),
        [copy],
        "the block's copy stays the one drawn copy"
    );
    assert_eq!(common::world(doc), [part], "and draws the symmetric part");
    assert_eq!(
        doc.node_space(part),
        doc.node_space(block),
        "in the block's space"
    );

    let source = corners(&mut session, block);
    let built = corners(&mut session, image);
    let want = reflected(&source, -HALF_X);
    assert!(
        same_points(&built, &want),
        "the image is the block across x = {}: {built:?} vs {want:?}",
        -HALF_X
    );
    assert_drawn(
        &mut session,
        copy,
        [-3.0 * HALF_X, -BLOCK[1] / 2.0, 0.0],
        [HALF_X, BLOCK[1] / 2.0, BLOCK[2]],
        2.0 * volume_of(BLOCK),
        "the copy",
    );
    let rows = session.tree_rows();
    for node in [image, part] {
        assert_eq!(
            common::status_of(&rows, node),
            RowStatus::Ok,
            "{node:?}'s row"
        );
    }

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

/// **The part is drawn across the face where it was clicked, and moves
/// with the copy**: the block's copy stands 0.1 m along x and its `−x`
/// wall is clicked where it is drawn, at x = 0.08, so the drawn part
/// spans x ∈ [0.04, 0.12]. Sliding the copy to 0.2 m afterwards, by the
/// slot of its own placement, carries the whole part to [0.14, 0.22].
#[test]
fn the_part_is_drawn_where_its_copy_stands_and_moves_with_it() {
    let tol = Tol::witness();
    let (mut session, block, copy) = moved_block(tol, [0.1, 0.0, 0.0]);
    let wall = face_toward(&session, 1.0);
    assert_eq!(wall.node, copy, "the ray met the moved copy");
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let (image, part) = mirror_in(&mut session, mirror_op(&tools));
    assert!(part.is_some(), "a drawn block becomes its part");
    let (y, z) = (BLOCK[1] / 2.0, BLOCK[2]);
    let volume = 2.0 * volume_of(BLOCK);
    assert_drawn(
        &mut session,
        copy,
        [0.04, -y, 0.0],
        [0.12, y, z],
        volume,
        "at 0.1",
    );
    let source = corners(&mut session, block);
    assert!(
        same_points(&corners(&mut session, image), &reflected(&source, -HALF_X)),
        "in its own space the image is across the block's own wall"
    );

    let slide_x = pncad::document::Coord::Slide(pncad::document::TargetAxis::X);
    let constraint = match session.committed_doc().node(copy) {
        Some(Node::Place { constraints, .. }) => constraints
            .iter()
            .find(|c| c.value().is_some_and(|v| v.coord == slide_x))
            .map(|c| c.id)
            .expect("the copy is pinned by a slide along x"),
        other => panic!("the copy is a placement, not {other:?}"),
    };
    let moved = session.perform(SessionOp::SetSlot {
        node: copy,
        slot: SlotId::ValueSlide { constraint },
        value: viewer::props::SlotValue::of(Dimension::Length, 0.2)
            .expect("a finite length is a value"),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    assert_drawn(
        &mut session,
        copy,
        [0.14, -y, 0.0],
        [0.22, y, z],
        volume,
        "moved to 0.2",
    );
}

/// **A body drawn twice becomes its part at both copies**: the block at
/// 0.1 m along x and at 0.1 m along y, its wall clicked on the first.
/// Both copies are taken over, each drawing the block and its image
/// across that copy's own `−x` wall.
#[test]
fn a_body_drawn_twice_is_its_symmetric_part_at_both_copies() {
    let tol = Tol::witness();
    let (session, block, first) = moved_block(tol, [0.1, 0.0, 0.0]);
    let (doc, second) = crate::fixture::insert(
        session.committed_doc().clone(),
        Node::place(
            pncad::document::BodyRead::plain(Operand::Node(block)),
            Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
    );
    let mut session = common::inline_without_door(
        drawn::pinned(doc, second, &drawn::translated([0.0, 0.1, 0.0])),
        tol,
    );
    session.pump();
    let wall = face_toward(&session, 1.0);
    assert_eq!(wall.node, first, "the ray met the first copy");
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let (_, part) = mirror_in(&mut session, mirror_op(&tools));
    let part = part.expect("a drawn block becomes its part");
    assert_eq!(
        viewer::world::placements_of(session.committed_doc(), part),
        [first, second],
        "both copies draw the part"
    );
    let (y, z) = (BLOCK[1] / 2.0, BLOCK[2]);
    let volume = 2.0 * volume_of(BLOCK);
    assert_drawn(
        &mut session,
        first,
        [0.04, -y, 0.0],
        [0.12, y, z],
        volume,
        "the first",
    );
    assert_drawn(
        &mut session,
        second,
        [-3.0 * HALF_X, 0.1 - y, 0.0],
        [HALF_X, 0.1 + y, z],
        volume,
        "the second",
    );
}

/// **A mated copy becomes its part where the mate seats it**: a small
/// block standing 0.1 m along x is rested on the plane of the block's
/// top by the mate tool's flow (its slides keep their 0.1 and 0), then
/// mirrored across its own `−x` wall. The mate still places the copy,
/// now drawing the small block's part, seated on that plane.
#[test]
fn a_mated_copy_becomes_its_symmetric_part_where_the_mate_seats_it() {
    let tol = Tol::witness();
    let (session, base) = block(tol);
    let mut session = session;
    let small = common::xy_box_in(&mut session, SMALL);
    let doc = session.committed_doc().clone();
    let small_copy = common::copy_of(&doc, small);
    let mut session = common::inline_without_door(
        drawn::pinned(doc, small_copy, &drawn::translated([0.1, 0.0, 0.0])),
        tol,
    );
    session.pump();
    let index = plate_index(&session);
    let underside = displayed_face_at(&session, &index, &common::up_at(0.1, 0.0));
    let top = displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    assert_eq!(underside.node, small_copy, "the small block's underside");
    assert_eq!(
        top.node,
        common::copy_of(session.committed_doc(), base),
        "the block's top"
    );
    drawn::rest_on(&mut session, underside, top);
    let (x, y, z) = (SMALL[0] / 2.0, SMALL[1] / 2.0, BLOCK[2]);
    assert_drawn(
        &mut session,
        small_copy,
        [0.1 - x, -y, z],
        [0.1 + x, y, z + SMALL[2]],
        volume_of(SMALL),
        "seated",
    );

    let index = plate_index(&session);
    let wall = displayed_face_at(&session, &index, &along_x(1.0, 0.0, z + SMALL[2] / 2.0));
    assert_eq!(wall.node, small_copy, "the ray met the seated copy's wall");
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    assert_eq!(tools.mirror().and_then(|t| t.input()), Some(small));
    let (_, part) = mirror_in(&mut session, mirror_op(&tools));
    assert!(part.is_some(), "the seated block becomes its part");
    assert_drawn(
        &mut session,
        small_copy,
        [0.1 - 3.0 * x, -y, z],
        [0.1 + x, y, z + SMALL[2]],
        2.0 * volume_of(SMALL),
        "the part, seated",
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
}

/// **A body nothing places gets its image alone**: the image is written
/// in the body's space and reached in the tree; no part is made and the
/// world is untouched.
#[test]
fn an_unplaced_body_gets_its_image_alone() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let profile = match session.committed_doc().node(block) {
        Some(Node::Extrude { profile, .. }) => Operand::Var(*profile),
        other => panic!("the block is an extrude, not {other:?}"),
    };
    let (doc, loose) = crate::fixture::insert(
        session.committed_doc().clone(),
        Node::Extrude {
            profile,
            distance: common::len(BLOCK[2]),
        },
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let world = common::world(session.committed_doc());
    let outcome = session.perform(SessionOp::AddMirror {
        body: loose.into(),
        plane: FaceSelection {
            name: own_cap(loose),
            node: loose,
            body: 0,
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(
            &outcome.committed[..],
            [DocEdit::InsertNode { node, .. }] if matches!(**node, Node::Mirror { .. })
        ),
        "the image alone: {:?}",
        outcome.committed
    );
    assert_eq!(
        common::world(session.committed_doc()),
        world,
        "the world stands"
    );
}

/// **A body mirrored across a face of another body in its space**: the
/// block's duplicate is a copy read out of a pattern of the block, so
/// it lives in the block's space. The first face pick (the block's)
/// seats the block; the second (the duplicate's far `+x` wall) re-picks
/// the plane alone; the image is the block reflected across that wall,
/// and the block's copy draws the block and that image.
#[test]
fn a_body_mirrored_across_another_bodys_face_reads_that_faces_plane() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let duplicated = session.perform(SessionOp::Duplicate {
        input: block.into(),
        beside: None,
    });
    assert!(duplicated.refusal.is_none(), "{:?}", duplicated.refusal);
    session.pump();
    let world = common::world(session.committed_doc());
    let [first, copy] = world[..] else {
        panic!("the block and its duplicate are drawn: {world:?}")
    };
    assert_eq!(first, block, "the block is drawn first");
    let block_copy = common::copy_of(session.committed_doc(), block);

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
    let (image, _) = mirror_in(&mut session, mirror_op(&tools));
    let source = corners(&mut session, block);
    let want = reflected(&source, at);
    let built = corners(&mut session, image);
    assert!(
        same_points(&built, &want),
        "the image is the block across x = {at}: {built:?} vs {want:?}"
    );
    assert_drawn(
        &mut session,
        block_copy,
        [-HALF_X, -BLOCK[1] / 2.0, 0.0],
        [2.0 * at + HALF_X, BLOCK[1] / 2.0, BLOCK[2]],
        2.0 * volume_of(BLOCK),
        "the block's copy",
    );
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
                Err(viewer::pickcache::NotIndexed::Absent),
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
            assert_eq!(
                role,
                viewer::seats::FaceRole::MirrorPlane,
                "the plane is named"
            );
        }
        other => panic!("no plane held yet refuses typed, not {other:?}"),
    }

    let index = plate_index(&session);
    let wall = displayed_face_at(&session, &index, &along_x(1.0, 0.002, 0.003));
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[SessionOp::Select(Selection::Face(wall))],
    );
    assert_eq!(
        tools.mirror().and_then(|t| t.input()),
        Some(torus),
        "a face pick with a body held re-picks the plane alone"
    );
    let (image, _) = mirror_in(&mut session, mirror_op(&tools));
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

/// **A plane read in another space is refused, however it is reached**:
/// a face of a second block's IMAGE (its own space's, since an image is
/// in its source's), a face clicked on the second block's drawn copy,
/// and a tree click that moves the body to the second block while the
/// plane stays on the first. Each refuses at the insert door naming the
/// two spaces, and nothing lands.
#[test]
fn a_plane_in_another_space_is_refused_through_an_image_or_a_re_picked_body() {
    let tol = Tol::witness();
    let (mut session, a) = block(tol);
    let wall = face_toward(&session, 1.0);
    let b = common::xy_box_in(&mut session, BLOCK);
    let doc = session.committed_doc().clone();
    let b_copy = common::copy_of(&doc, b);
    let mut session = common::inline_without_door(
        drawn::pinned(doc, b_copy, &drawn::translated([0.0, 0.1, 0.0])),
        tol,
    );
    session.pump();
    let (image_b, _) = mirror_in(
        &mut session,
        SessionOp::AddMirror {
            body: b.into(),
            plane: FaceSelection {
                name: own_cap(b),
                node: b,
                body: 0,
            },
        },
    );
    session.pump();
    let doc = session.committed_doc().clone();
    assert_eq!(
        doc.node_space(image_b),
        doc.node_space(b),
        "the image is in its source's space"
    );
    assert_ne!(doc.node_space(a), doc.node_space(b), "two spaces");
    let refused = |session: &mut DocSession, op: SessionOp, what: &str| {
        let before = session.committed_doc().clone();
        let out = session.perform(op);
        assert!(
            matches!(
                out.refusal.as_ref(),
                Some(Refusal::Edit(error))
                    if matches!(error.as_ref(), EditError::SpaceMismatch { .. })
            ),
            "{what}: refused naming the two spaces, not {:?}",
            out.refusal
        );
        assert!(
            session.committed_doc().bit_eq(&before),
            "{what}: nothing lands"
        );
    };
    let image_face = FaceSelection {
        name: own_cap(b),
        node: image_b,
        body: 0,
    };
    refused(
        &mut session,
        SessionOp::AddMirror {
            body: a.into(),
            plane: image_face,
        },
        "a plane on another space's image",
    );
    let index = plate_index(&session);
    let b_wall = displayed_face_at(&session, &index, &along_x(1.0, 0.1, MID_Z));
    assert_eq!(b_wall.node, b_copy, "the ray met the second block's copy");
    refused(
        &mut session,
        SessionOp::AddMirror {
            body: a.into(),
            plane: b_wall,
        },
        "a face clicked on another space's drawn copy",
    );
    let mut tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &[SessionOp::Select(Selection::Node(b))]
            )
            .is_empty()
    );
    assert_eq!(tools.mirror().and_then(|t| t.input()), Some(b));
    refused(
        &mut session,
        mirror_op(&tools),
        "a body re-picked into another space",
    );
}

/// **One click is named by one node**: a face clicked on the block's
/// copy is said as the block in both of the tool's items, the body and
/// the plane.
#[test]
fn a_click_on_a_copy_names_the_body_and_the_plane_by_one_node() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let wall = face_toward(&session, 1.0);
    let tools = mirror_tool_picking(&session, &[wall]);
    let doc = session.committed_doc();
    let said = doc.spoken(block).to_string();
    assert_eq!(
        tools.mirror().expect("open").line(doc),
        format!("mirrored body: {said}; mirror plane: face of {said}"),
    );
}

/// **A face on a split's second half is read on that half**: the
/// block split at mid-height, its lower half placed and drawn 0.1 m
/// along x, and the half's underside clicked there. The mirror's plane
/// reads that face on the split's LOWER output by its port — not the
/// upper half, which has no such face, nor the block, whose start cap
/// lies in the same plane — and the image is the block reflected below
/// z = 0.
#[test]
fn a_face_on_a_splits_lower_half_reads_that_half() {
    let tol = Tol::witness();
    let (session, block, split) = drawn::split_box(BLOCK, tol);
    let below = pncad::select::SplitHalf::Below;
    let (mut session, half) = drawn::half_placed(
        &session,
        split,
        below,
        &drawn::translated([0.1, 0.0, 0.0]),
        tol,
    );
    let index = plate_index(&session);
    let floor = displayed_face_at(&session, &index, &common::up_at(0.1, 0.0));
    assert_eq!(floor.node, half, "the ray met the lower half's copy");
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picks = [
        SessionOp::Select(Selection::Node(common::copy_of(
            session.committed_doc(),
            block,
        ))),
        SessionOp::Select(Selection::Face(floor)),
    ];
    assert!(
        tools
            .feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &picks
            )
            .is_empty()
    );
    let outcome = session.perform(mirror_op(&tools));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let read = outcome.committed.iter().find_map(|edit| match edit {
        DocEdit::InsertNode { node, .. } => match &**node {
            Node::Mirror { plane, .. } => match &plane.read {
                Operand::Pose(pose) => match &**pose {
                    PoseDef::Plane {
                        face: Operand::Select { body, .. },
                    } => Some((**body).clone()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    });
    assert_eq!(
        read,
        Some(Operand::output(split, below.port())),
        "the plane's face is read on the lower half by its port"
    );
    let image = outcome.minted[0];
    let source = corners(&mut session, block);
    let built = corners(&mut session, image);
    let want: Vec<[f64; 3]> = {
        let mut w: Vec<[f64; 3]> = source.iter().map(|&[x, y, z]| [x, y, -z]).collect();
        w.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        w
    };
    assert!(
        same_points(&built, &want),
        "the image is the block below z = 0: {built:?} vs {want:?}"
    );
}
