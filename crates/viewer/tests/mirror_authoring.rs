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

/// The block's world copy pinned at the translation `at`: the session,
/// the block and its copy.
fn moved_block(tol: Tol, at: [f64; 3]) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (session, block) = block(tol);
    let doc = session.committed_doc().clone();
    let copy = common::copy_of(&doc, block);
    let doc = pinned(doc, copy, at);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, block, copy)
}

/// `doc` with `placement` pinned at the translation `at`.
fn pinned(
    doc: Doc<pncad::document::ProfileProgram>,
    placement: RecipeNodeId,
    at: [f64; 3],
) -> Doc<pncad::document::ProfileProgram> {
    let pose = pncad::document::Frame {
        translation: at,
        ..pncad::document::Frame::IDENTITY
    };
    crate::fixture::pin(
        doc,
        placement,
        &pose,
        &pncad::document::EvalOptions::default(),
    )
}

/// The least and greatest x of `points`.
fn x_span(points: &[[f64; 3]]) -> (f64, f64) {
    points
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
            (lo.min(p[0]), hi.max(p[0]))
        })
}

/// **The image is drawn across the face where the person saw it**: the
/// block's copy stands 0.1 m along x, and its `−x` wall is clicked
/// where it is drawn, at x = 0.08. The image's world copy takes the
/// clicked copy's placement, so the DRAWN image is the drawn block
/// reflected across x = 0.08 — while the image itself, in the block's
/// own space, is the block reflected across x = −0.02.
#[test]
fn the_image_is_drawn_across_the_face_where_it_was_clicked() {
    let tol = Tol::witness();
    let (mut session, block, copy) = moved_block(tol, [0.1, 0.0, 0.0]);
    let wall = face_toward(&session, 1.0);
    assert_eq!(wall.node, copy, "the ray met the moved copy");
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let image = session_insert(&mut session, mirror_op(&tools));
    let drawn_source = corners(&mut session, copy);
    let (at, _) = x_span(&drawn_source);
    assert!(
        (at - 0.08).abs() <= 1e-12,
        "the clicked wall is drawn at x = 0.08: {at}"
    );
    let image_copy = common::copy_of(session.committed_doc(), image);
    let drawn = corners(&mut session, image_copy);
    let want = reflected(&drawn_source, at);
    assert!(
        same_points(&drawn, &want),
        "the drawn image is across the clicked face: {drawn:?} vs {want:?}"
    );
    let own = corners(&mut session, image);
    let source = corners(&mut session, block);
    assert!(
        same_points(&own, &reflected(&source, -HALF_X)),
        "in its own space the image is across the block's own wall"
    );
}

/// **With no clicked copy, the image arrives wherever its source
/// does**: the block drawn twice (at 0.1 m along x and 0.1 m along y),
/// its body picked in the tree and its plane named on the block itself,
/// so no copy of it was clicked. The image takes BOTH copies'
/// placements, each drawn image the block's copy beside it reflected
/// across that copy's `−x` wall.
#[test]
fn a_tree_picked_body_is_mirrored_at_every_copy_of_it() {
    let tol = Tol::witness();
    let (session, block, first) = moved_block(tol, [0.1, 0.0, 0.0]);
    let doc = session.committed_doc().clone();
    let (doc, second) = crate::fixture::insert(
        doc,
        Node::place(
            vec![pncad::document::BodyRead::plain(Operand::Node(block))],
            Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
    );
    let doc = pinned(doc, second, [0.0, 0.1, 0.0]);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let wall = face_toward(&session, 1.0);
    let (_, own) = wall.name.copy_of().expect("a pick on a copy");
    let plane = FaceSelection {
        name: own.clone(),
        node: block,
        body: 0,
    };
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picks = [
        SessionOp::Select(Selection::Node(block)),
        SessionOp::Select(Selection::Face(plane)),
    ];
    assert!(tools.feed(session.committed_doc(), &picks).is_empty());
    let image = session_insert(&mut session, mirror_op(&tools));
    let doc = session.committed_doc().clone();
    let copies = viewer::world::placements_of(&doc, image);
    assert_eq!(
        copies.len(),
        2,
        "one image copy per copy of the block: {copies:?}"
    );
    let mut drawn: Vec<Vec<[f64; 3]>> = copies.iter().map(|&c| corners(&mut session, c)).collect();
    let mut want: Vec<Vec<[f64; 3]>> = [first, second]
        .into_iter()
        .map(|c| {
            let source = corners(&mut session, c);
            let (at, _) = x_span(&source);
            reflected(&source, at)
        })
        .collect();
    drawn.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    want.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    assert!(
        drawn.iter().zip(&want).all(|(d, w)| same_points(d, w)),
        "each drawn image is across its copy's wall: {drawn:?} vs {want:?}"
    );
}

/// **A plane read in another space is refused, however it is reached**:
/// a face of a second block's IMAGE (its own space's, since an image is
/// in its source's), and a tree click that moves the body to the second
/// block while the plane stays on the first. Both refuse at the insert
/// door, and nothing lands.
#[test]
fn a_plane_in_another_space_is_refused_through_an_image_or_a_re_picked_body() {
    let tol = Tol::witness();
    let (mut session, a) = block(tol);
    let wall = face_toward(&session, 1.0);
    let b = common::xy_box_in(&mut session, BLOCK);
    let image_b = session_insert(
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
    let mut tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    assert!(
        tools
            .feed(
                session.committed_doc(),
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

/// **A symmetric part of a moved block is drawn in one place whichever
/// member leads**: block and image unioned by the boolean tool, in both
/// orders, the union's one drawn copy spans the drawn block and its
/// drawn image, x ∈ [0.04, 0.12].
#[test]
fn a_moved_symmetric_part_is_drawn_in_one_place_in_either_order() {
    let tol = Tol::witness();
    for image_first in [false, true] {
        let (mut session, block, _) = moved_block(tol, [0.1, 0.0, 0.0]);
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
            .map(|n| SessionOp::Select(Selection::Node(n)))
            .collect();
        assert!(tools.feed(session.committed_doc(), &picks).is_empty());
        let outcome = session.perform(tools.boolean().expect("open").op().expect("two"));
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        let inserted = outcome
            .committed
            .iter()
            .filter(|e| matches!(e, DocEdit::InsertNode { .. }))
            .count();
        assert_eq!(
            inserted, 1,
            "image first {image_first}: the union alone, no placement"
        );
        let copies = viewer::world::drawn_copies(session.committed_doc());
        let [copy] = copies[..] else {
            panic!("image first {image_first}: one drawn copy, {copies:?}")
        };
        let (lo, hi) = x_span(&corners(&mut session, copy));
        assert!(
            (lo - 0.04).abs() <= 1e-12 && (hi - 0.12).abs() <= 1e-12,
            "image first {image_first}: drawn over [{lo}, {hi}]"
        );
    }
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
/// along x, and the half's underside clicked there. The plane is the
/// lower half's floor, z = 0, so the image is the block reflected
/// below it — not a read of the upper half, which has no such face.
#[test]
fn a_face_on_a_splits_lower_half_reads_that_half() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: block.into(),
            origin: common::len3([0.0, 0.0, MID_Z]),
            normal: common::scl3([0.0, 0.0, 1.0]),
        },
    );
    let below = pncad::select::SplitHalf::Below;
    let (doc, half) = crate::fixture::insert(
        session.committed_doc().clone(),
        Node::place(
            vec![pncad::document::BodyRead::plain(Operand::output(
                split,
                below.port(),
            ))],
            Operand::Var(pncad::document::VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
    );
    let doc = pinned(doc, half, [0.1, 0.0, 0.0]);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
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
    assert!(tools.feed(session.committed_doc(), &picks).is_empty());
    let image = session_insert(&mut session, mirror_op(&tools));
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
