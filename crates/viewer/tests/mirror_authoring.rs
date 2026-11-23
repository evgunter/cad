//! **The mirror tool, replayed headlessly**: a body and a picked face,
//! or a plane variable the document holds, through the selection
//! stream the viewport feeds, one committed
//! action — the image, and the symmetric part, the body or the copy
//! picked unioned with its image where it stands (Ev, PR 6353) — its
//! typed refusals, and the saved document read back.
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
use viewer::combine::PlaneEvent;
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

/// A session holding the block added to the product, so placed into
/// the world, landed — answering the extrude.
fn block(tol: Tol) -> (DocSession, RecipeNodeId) {
    let (session, block) = unplaced_block(tol);
    let (session, _) = common::world_copy_in(&session, block, tol);
    (session, block)
}

/// A session holding the block as it lands, unplaced — answering the
/// extrude.
fn unplaced_block(tol: Tol) -> (DocSession, RecipeNodeId) {
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

/// **A copy mirrored across one of its own faces becomes its symmetric
/// part**: one face pick on the block's copy holds the copy and the
/// plane; the commit inserts the image of the copy, whose plane is that
/// face read as a plane, and the union of the copy and its image, in the
/// world where the copy stands, and writes no placement (Ev, PR 6353).
/// The image is the copy reflected across x = −20 mm, the part draws
/// 80 mm, and the saved document reads back bit for bit and builds the
/// same image.
#[test]
fn a_copy_mirrored_across_its_own_face_becomes_its_symmetric_part() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let copy = common::copy_of(session.committed_doc(), block);
    let wall = face_toward(&session, 1.0);
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let tool = tools.mirror().expect("open");
    assert_eq!(tool.body(), Some(copy), "a pick on the copy seats the copy");
    assert_eq!(tool.face(), Some(&wall), "and its face is the plane");

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
        Operand::Node(copy),
        "the image reflects the copy"
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
    assert!(
        outcome.committed.iter().all(|edit| matches!(
            edit,
            DocEdit::InsertNode { .. }
                | DocEdit::SetProduct { .. }
                | DocEdit::SetParam {
                    slot: pncad::document::SlotId::Operand(pncad::document::OperandSlot::Member(_)),
                    ..
                }
        )),
        "no placement is written: {:?}",
        outcome.committed
    );
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        [part],
        "the symmetric part is drawn where the copy stood"
    );
    assert_eq!(
        doc.node_space(part),
        Some(pncad::document::VarId::WORLD),
        "in the copy's space, the world"
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
        part,
        [-3.0 * HALF_X, -BLOCK[1] / 2.0, 0.0],
        [HALF_X, BLOCK[1] / 2.0, BLOCK[2]],
        2.0 * volume_of(BLOCK),
        "the part",
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
/// slot of its own placement, carries the whole part to [0.14, 0.22]:
/// the part reads the copy, and its image the copy's wall.
#[test]
fn the_part_is_drawn_where_its_copy_stands_and_moves_with_it() {
    let tol = Tol::witness();
    let (mut session, _, copy) = moved_block(tol, [0.1, 0.0, 0.0]);
    let wall = face_toward(&session, 1.0);
    assert_eq!(wall.node, copy, "the ray met the moved copy");
    let tools = mirror_tool_picking(&session, std::slice::from_ref(&wall));
    let (image, part) = mirror_in(&mut session, mirror_op(&tools));
    let part = part.expect("a picked copy becomes its part");
    let (y, z) = (BLOCK[1] / 2.0, BLOCK[2]);
    let volume = 2.0 * volume_of(BLOCK);
    assert_drawn(
        &mut session,
        part,
        [0.04, -y, 0.0],
        [0.12, y, z],
        volume,
        "at 0.1",
    );
    let source = corners(&mut session, copy);
    assert!(
        same_points(
            &corners(&mut session, image),
            &reflected(&source, 0.1 - HALF_X)
        ),
        "the image is across the copy's own wall"
    );

    let slide_x = pncad::document::Coord::slide(pncad::document::TargetAxis::X);
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
        part,
        [0.14, -y, 0.0],
        [0.22, y, z],
        volume,
        "moved to 0.2",
    );
}

/// **A mirror on one of two copies is that copy's part alone** (Ev, PR
/// 6353): the block at 0.1 m along x and at 0.1 m along y, its wall
/// clicked on the first. The first copy becomes the block and its image
/// across that copy's `−x` wall; the second copy is untouched, and
/// still places the block.
#[test]
fn a_mirror_on_one_of_two_copies_is_that_copys_part_alone() {
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
    let part = part.expect("the picked copy becomes its part");
    let doc = session.committed_doc();
    assert_eq!(
        doc.placements_of(block),
        [first, second],
        "both copies still place the block"
    );
    let mut drawn = viewer::world::drawn_nodes(doc);
    drawn.sort_unstable();
    let mut want = vec![second, part];
    want.sort_unstable();
    assert_eq!(
        drawn, want,
        "the second copy and the first's part are drawn"
    );
    let (y, z) = (BLOCK[1] / 2.0, BLOCK[2]);
    assert_drawn(
        &mut session,
        part,
        [0.04, -y, 0.0],
        [0.12, y, z],
        2.0 * volume_of(BLOCK),
        "the first's part",
    );
    assert_drawn(
        &mut session,
        second,
        [-HALF_X, 0.1 - y, 0.0],
        [HALF_X, 0.1 + y, z],
        volume_of(BLOCK),
        "the second, as it was",
    );
}

/// **A mated copy becomes its part where the mate seats it**: a small
/// block standing 0.1 m along x is rested on the plane of the block's
/// top by the mate tool's flow (its slides keep their 0.1 and 0), then
/// mirrored across its own `−x` wall. The mate still places the copy,
/// and the part, which reads the copy, is drawn seated on that plane.
#[test]
fn a_mated_copy_becomes_its_symmetric_part_where_the_mate_seats_it() {
    let tol = Tol::witness();
    let (session, base) = block(tol);
    let mut session = session;
    let small = common::xy_box_in(&mut session, SMALL);
    let (doc, small_copy) = common::world_copy(session.committed_doc(), small, tol);
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
    assert_eq!(tools.mirror().and_then(|t| t.body()), Some(small_copy));
    let (_, part) = mirror_in(&mut session, mirror_op(&tools));
    let part = part.expect("the seated copy becomes its part");
    assert_drawn(
        &mut session,
        part,
        [0.1 - 3.0 * x, -y, z],
        [0.1 + x, y, z + SMALL[2]],
        2.0 * volume_of(SMALL),
        "the part, seated",
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
}

/// **An unplaced body becomes its symmetric part too**: the mirror
/// writes `body ∪ image` whatever reads the body, so a body nothing
/// copies is read by the union and the union is drawn in its place;
/// the world is untouched.
#[test]
fn an_unplaced_body_becomes_its_symmetric_part() {
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
        plane: viewer::session::ToolPlane::Face(FaceSelection {
            name: own_cap(loose),
            node: loose,
            body: 0,
        }),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(
            &outcome.committed[..],
            [DocEdit::InsertNode { node: image, .. }, DocEdit::InsertNode { node: part, .. }]
                if matches!(**image, Node::Mirror { .. }) && matches!(**part, Node::Union { .. })
        ),
        "the image and the union, nothing else: {:?}",
        outcome.committed
    );
    let doc = session.committed_doc();
    let union = *outcome
        .minted
        .iter()
        .find(|&&n| matches!(doc.node(n), Some(Node::Union { .. })))
        .expect("the symmetric part");
    assert!(viewer::world::is_drawn(doc, union), "the union is drawn");
    assert!(!viewer::world::is_drawn(doc, loose), "in the body's place");
    assert_eq!(common::world(doc), world, "the world stands");
}

/// **The mirror's label lands on the mirror, placed body or not**: the
/// gesture's noun is the mirror, so its insert labels the mirror
/// `Mirror N` and answers it as what it made, the node the chrome
/// selects; the union of the symmetric part takes no label.
///
/// Red if the label, or the made node, is the union, for either body.
#[test]
fn the_mirrors_label_lands_on_the_mirror_whether_its_body_is_placed_or_not() {
    let tol = Tol::witness();
    for (placed, (mut session, block)) in [(true, block(tol)), (false, unplaced_block(tol))] {
        let plane = FaceSelection {
            name: own_cap(block),
            node: block,
            body: 0,
        };
        let op = SessionOp::AddMirror {
            body: block.into(),
            plane: viewer::session::ToolPlane::Face(plane),
        };
        assert!(op.selects_what_it_makes(), "placed {placed}");
        let label =
            viewer::tree::proposed_label(session.committed_doc(), "Mirror").expect("a proposal");
        assert_eq!(label.as_str(), "Mirror 1", "placed {placed}");
        let outcome = session.perform(SessionOp::CreateLabelled {
            creation: viewer::session::Creation::of(op).expect("a creation"),
            label,
        });
        assert!(
            outcome.refusal.is_none(),
            "placed {placed}: {:?}",
            outcome.refusal
        );
        let doc = session.committed_doc();
        let (image, part) = (
            outcome
                .minted
                .iter()
                .copied()
                .find(|&n| matches!(doc.node(n), Some(Node::Mirror { .. })))
                .expect("the mirror"),
            outcome
                .minted
                .iter()
                .copied()
                .find(|&n| matches!(doc.node(n), Some(Node::Union { .. })))
                .expect("the symmetric part"),
        );
        assert_eq!(
            outcome.made,
            Some(image),
            "placed {placed}: the mirror is made"
        );
        assert_eq!(
            doc.label(image).map(|l| l.as_str()),
            Some("Mirror 1"),
            "placed {placed}: the mirror carries the label"
        );
        assert_eq!(
            doc.label(part),
            None,
            "placed {placed}: the union carries none"
        );
    }
}

/// **A body mirrored across a face of another body in its space**: the
/// unplaced block placed twice is a stand-in and a stepped copy, both in
/// the block's own space. The first face pick (the stand-in's) seats
/// the stand-in; the second (the stepped copy's far `+x` wall) re-picks
/// the plane alone; the image is the stand-in reflected across that
/// wall, and the symmetric part draws both.
#[test]
fn a_body_mirrored_across_another_bodys_face_reads_that_faces_plane() {
    let tol = Tol::witness();
    let (mut session, block) = unplaced_block(tol);
    let place_stand_in = common::place_named(session.committed_doc(), block, "bracket");
    let stand_in = common::placed_by(&mut session, place_stand_in);
    let place_copy = common::place_named(session.committed_doc(), block, "bracket");
    let copy = common::placed_by(&mut session, place_copy);
    let op = common::transformed(
        &mut session,
        copy,
        &common::slides([8.0 * HALF_X, 0.0, 0.0]),
    );
    let stepped = session.perform(op);
    assert!(stepped.refusal.is_none(), "{:?}", stepped.refusal);
    session.pump();

    let own = face_toward(&session, 1.0);
    let far = face_toward(&session, -1.0);
    let tools = mirror_tool_picking(&session, &[own, far.clone()]);
    let tool = tools.mirror().expect("open");
    assert_eq!(
        tool.body(),
        Some(stand_in),
        "the second face keeps the body"
    );
    assert_eq!(tool.face(), Some(&far), "and re-picks the plane");

    let at = corners(&mut session, copy)
        .iter()
        .map(|p| p[0])
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        at > HALF_X,
        "the stepped copy's far wall is clear of the block: {at}"
    );
    let (image, part) = mirror_in(&mut session, mirror_op(&tools));
    let source = corners(&mut session, stand_in);
    assert_eq!(
        source,
        corners(&mut session, block),
        "the stand-in is the block"
    );
    let want = reflected(&source, at);
    let built = corners(&mut session, image);
    assert!(
        same_points(&built, &want),
        "the image is the block across x = {at}: {built:?} vs {want:?}"
    );
    let part = part.expect("the symmetric part");
    assert!(
        viewer::world::is_drawn(session.committed_doc(), part),
        "the symmetric part is drawn"
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
                &[SessionOp::Select(Selection::Node(common::drawn_node(
                    session.committed_doc(),
                    torus
                )))]
            )
            .is_empty(),
        "a tree pick of the drawn copy seats the body"
    );
    match tools.mirror().expect("open").op() {
        Err(error @ SeatError::NoPlane { role }) => {
            assert_eq!(role, viewer::seats::PlaneRole::Mirror, "the plane is named");
            assert_eq!(
                error.to_string(),
                "no mirror plane picked yet: pick a face or a datum",
                "the refusal names both kinds of plane the seat takes"
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
        tools.mirror().and_then(|t| t.body()),
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
        .filter(|n| matches!(n, ToolNotice::Mirror(PlaneEvent::PlaneLost { .. })))
        .count();
    assert_eq!((seat, plane), (1, 1), "one drop each: {notices:?}");
    let tool = tools.mirror().expect("still open");
    assert_eq!((tool.body(), tool.face()), (None, None), "both are empty");
    assert!(
        tools
            .reconcile(session.doc(), session.landed_pair())
            .is_empty(),
        "and a drop is said once"
    );
}

/// **A drop says the plane as the line said it**, after the copy it
/// was clicked on is deleted: the line read the click on the block the
/// copy places, so the drop says the block, and not the deleted copy by
/// its tag. Red if the drop reads the click's body again against the
/// document that dropped it.
#[test]
fn a_deleted_copy_drops_the_plane_in_the_words_the_line_said() {
    let tol = Tol::witness();
    let (mut session, block) = block(tol);
    let wall = face_toward(&session, 1.0);
    let copy = wall.node;
    assert_ne!(
        copy, block,
        "the premise: the wall was clicked on the block's copy"
    );
    let mut tools = mirror_tool_picking(&session, &[wall]);
    let line = tools
        .mirror()
        .expect("open")
        .line(viewer::frame::landed_speaker(session.landed_pair()));
    let plane = line
        .split_once("mirror plane: ")
        .map(|(_, plane)| plane.to_owned())
        .unwrap_or_else(|| panic!("the line holds the plane: {line:?}"));
    let (block_said, copy_said) = (
        viewer::tree::spelled(&session.doc().spoken(block)),
        viewer::tree::spelled(&session.doc().spoken(copy)),
    );
    assert!(
        plane.starts_with("the side wall ")
            && plane.ends_with(&format!(" of {block_said}, on {copy_said}")),
        "the line says the block's wall on the copy clicked: {line:?}"
    );
    let outcome = session.perform(SessionOp::DeleteNode { node: copy });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    tools.respeak(session.doc());
    session.pump();
    let dropped: Vec<String> = tools
        .reconcile(session.doc(), session.landed_pair())
        .iter()
        .filter(|notice| matches!(notice, ToolNotice::Mirror(PlaneEvent::PlaneLost { .. })))
        .map(|notice| notice.said(session.landed_pair()))
        .collect();
    assert_eq!(
        dropped,
        [format!(
            "mirror tool: the mirror plane pick ({plane}) no longer resolves; the tool dropped it"
        )],
        "the drop says the plane as the line did"
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
    let (doc, b_copy) = common::world_copy(session.committed_doc(), b, tol);
    let mut session = common::inline_without_door(
        drawn::pinned(doc, b_copy, &drawn::translated([0.0, 0.1, 0.0])),
        tol,
    );
    session.pump();
    let (image_b, _) = mirror_in(
        &mut session,
        SessionOp::AddMirror {
            body: b.into(),
            plane: viewer::session::ToolPlane::Face(FaceSelection {
                name: own_cap(b),
                node: b,
                body: 0,
            }),
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
            plane: viewer::session::ToolPlane::Face(image_face),
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
            plane: viewer::session::ToolPlane::Face(b_wall),
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
    assert_eq!(tools.mirror().and_then(|t| t.body()), Some(b));
    refused(
        &mut session,
        mirror_op(&tools),
        "a body re-picked into another space",
    );
}

/// **A click on a copy seats the copy and says it**: a face clicked on
/// the block's copy seats the copy as the body (Ev, PR 6353), and the
/// plane is said as the block's wall on the copy it was clicked on.
#[test]
fn a_click_on_a_copy_seats_the_copy_and_says_the_plane_on_it() {
    let tol = Tol::witness();
    let (session, block) = block(tol);
    let wall = face_toward(&session, 1.0);
    let wall_copy = wall.node;
    let tools = mirror_tool_picking(&session, &[wall]);
    let said = viewer::tree::spelled(&session.doc().spoken(block));
    let copy = viewer::tree::spelled(&session.doc().spoken(wall_copy));
    let line = tools
        .mirror()
        .expect("open")
        .line(viewer::frame::landed_speaker(session.landed_pair()));
    let plane = line
        .strip_prefix(&format!("mirrored body: {copy}; mirror plane: "))
        .unwrap_or_else(|| panic!("the body seat says the copy: {line:?}"));
    assert!(
        plane.starts_with("the side wall ") && plane.ends_with(&format!(" of {said}, on {copy}")),
        "the plane is the block's wall in its words, said on the copy clicked: {line:?}"
    );
}

/// **A face on a split's second half is read on that half**: the
/// block split at mid-height, the split drawn in the block's place, and
/// the lower half's underside clicked there with the lower half the
/// body. The mirror's plane reads that face on the split's LOWER output
/// by its port — not the upper half, which has no such face, nor the
/// block, whose start cap lies in the same plane — and the image is the
/// lower half reflected below z = 0.
#[test]
fn a_face_on_a_splits_lower_half_reads_that_half() {
    let tol = Tol::witness();
    let (session, block, split) = drawn::split_box(BLOCK, tol);
    let below = pncad::select::SplitHalf::Below;
    let index = plate_index(&session);
    let floor = displayed_face_at(&session, &index, &common::up_at(0.0, 0.0));
    assert_eq!(floor.node, split, "the ray met the split drawn as itself");
    assert_eq!(floor.body, u32::from(below.port()), "on its lower half");
    let mut session = session;
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picks = [SessionOp::Select(Selection::Face(floor))];
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
                    } => Some(body.read.clone()),
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
    // The lower half: the block's corners, its top brought down to the
    // cut at mid-height.
    let source = corners(&mut session, block);
    let built = corners(&mut session, image);
    let want: Vec<[f64; 3]> = {
        let mut w: Vec<[f64; 3]> = source
            .iter()
            .map(|&[x, y, z]| [x, y, -z.min(MID_Z)])
            .collect();
        w.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        w
    };
    assert!(
        same_points(&built, &want),
        "the image is the lower half below z = 0: {built:?} vs {want:?}"
    );
}

/// **The unplaced block split along the plane `plane` reads off it,
/// the split's plane variable named `name`**: the session, the block,
/// the split and the variable. The split is the plane's first reader;
/// naming the variable is what lets a second read it (VR7).
fn split_with_named_plane(
    plane: impl FnOnce(&DocSession, RecipeNodeId) -> viewer::session::ToolPlane,
    name: &str,
    tol: Tol,
) -> (
    DocSession,
    RecipeNodeId,
    RecipeNodeId,
    pncad::document::VarId,
) {
    let (mut session, block) = unplaced_block(tol);
    let plane = plane(&session, block);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: block.into(),
            plane,
        },
    );
    session.pump();
    let Some(Node::Split { tool, .. }) = session.committed_doc().node(split) else {
        panic!("the split")
    };
    let var = tool.read;
    let outcome = session.perform(SessionOp::RenameVar {
        var,
        name: Some(pncad::document::VarName::new(name).expect("a name")),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (session, block, split, var)
}

/// **The mirror tool across the plane variable `var`**: the split's
/// lower half seated by a pick of its floor, the plane then re-picked
/// as the variable. Asserts the tool holds the variable, the commit
/// reads it `through` the role a frame is read through, and the image
/// is the lower half reflected across the cut at `z = MID_Z`, to
/// `within` metres.
fn mirror_the_lower_half_across(
    mut session: DocSession,
    (block, split): (RecipeNodeId, RecipeNodeId),
    var: pncad::document::VarId,
    through: Option<pncad::document::pose::PoseRole>,
    within: f64,
) {
    let index = plate_index(&session);
    let floor = displayed_face_at(&session, &index, &common::up_at(0.0, 0.0));
    assert_eq!(floor.node, split, "the ray met the split drawn as itself");
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let picks = [
        SessionOp::Select(Selection::Face(floor.clone())),
        SessionOp::Select(Selection::Variable(var)),
    ];
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &picks,
    );
    let tool = tools.mirror().expect("open");
    assert!(
        matches!(
            tool.plane(),
            Some(viewer::combine::HeldPlane::Defined { var: held, through: role })
                if held.id() == var && *role == through
        ),
        "the variable re-picks the plane: {:?}",
        tool.plane()
    );
    assert_eq!(tool.face(), None, "and no face is held");
    let op = mirror_op(&tools);
    let SessionOp::AddMirror {
        plane: viewer::session::ToolPlane::Read(read),
        ..
    } = &op
    else {
        panic!("the commit reads the variable, not {op:?}")
    };
    assert_eq!(
        (&read.read, read.role),
        (&Operand::Var(var), through),
        "the plane is the variable, read through its role"
    );
    let (image, _) = mirror_in(&mut session, op);
    let Some(Node::Mirror { plane, .. }) = session.committed_doc().node(image) else {
        panic!("the image")
    };
    assert_eq!(
        (plane.read, plane.role),
        (var, through),
        "the stored mirror reads the split's own variable, not a copy of it"
    );
    let built = corners(&mut session, image);
    let lower: Vec<[f64; 3]> = corners(&mut session, block)
        .into_iter()
        .map(|[x, y, z]| [x, y, z.min(MID_Z)])
        .collect();
    let want = reflected_in_z(&lower, MID_Z);
    let close = built.len() == want.len()
        && built
            .iter()
            .zip(&want)
            .all(|(g, w)| g.iter().zip(w).all(|(a, b)| (a - b).abs() <= within));
    assert!(
        close,
        "the image is the lower half reflected across z = {MID_Z}: {built:?} vs {want:?}"
    );
}

/// `points` reflected across the plane `z = at`, sorted.
fn reflected_in_z(points: &[[f64; 3]], at: f64) -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = points
        .iter()
        .map(|&[x, y, z]| [x, y, 2.0 * at - z])
        .collect();
    out.sort_by(|a, b| a.partial_cmp(b).expect("finite points"));
    out
}

/// **A `Plane` variable the document holds is a mirror plane**: the
/// split's own cut, named, picked as a variable after a face seats the
/// body, is read as it is, and the image of the lower half stands
/// where the upper half did.
///
/// Red if the mirror tool drops a variable pick, the commit reads a
/// face, or the image is not reflected across the cut.
#[test]
fn a_named_plane_variable_is_a_mirror_plane() {
    let tol = Tol::witness();
    let (session, block, split, var) = split_with_named_plane(
        |session, block| common::cut_across(session, block, MID_Z),
        "cut",
        tol,
    );
    assert_eq!(
        session
            .committed_doc()
            .var(var)
            .map(pncad::document::Var::kind),
        Some(pncad::document::VarKind::Plane),
        "the premise: the split's cut is a Plane variable"
    );
    mirror_the_lower_half_across(session, (block, split), var, None, 1e-12);
}

/// **A `Frame` variable the document holds is a mirror plane, read
/// through its `xy`**: a frame standing on the block at mid-height,
/// first read by a split through its `xy`, named, and picked as the
/// mirror's plane, is read through the same role.
///
/// Red if the mirror tool drops a frame pick, reads it plainly (a
/// frame is no plane), or the image is not reflected across its `xy`.
#[test]
fn a_named_frame_variable_is_a_mirror_plane_through_its_xy() {
    let tol = Tol::witness();
    let (session, block, split, var) = split_with_named_plane(
        |session, block| {
            viewer::session::ToolPlane::Read(editor_core::test_support::plane_on_frame(
                session.committed_doc(),
                block,
                [0.0, 0.0, MID_Z],
                [0.0, 0.0, 1.0],
                tol,
            ))
        },
        "datum",
        tol,
    );
    assert_eq!(
        session
            .committed_doc()
            .var(var)
            .map(pncad::document::Var::kind),
        Some(pncad::document::VarKind::Frame),
        "the premise: the split reads a Frame variable"
    );
    // A frame read off the body's axes stands there to the rounding of
    // the turns it is built from (`plane_on_frame`).
    mirror_the_lower_half_across(
        session,
        (block, split),
        var,
        Some(pncad::document::pose::PoseRole::Xy),
        1e-9,
    );
}

/// **A plane variable in another space refuses at the insert door**:
/// the second block's cut, named, read by a mirror of the first block,
/// reached through the tool's variable pick, refuses naming the two
/// spaces and lands nothing — the variable door is held to the same
/// space rule as a face.
///
/// Red if the mirror commits a plane read in a space other than its
/// body's.
#[test]
fn a_plane_variable_in_another_space_is_refused() {
    let tol = Tol::witness();
    let (mut session, a) = block(tol);
    let b = common::xy_box_in(&mut session, BLOCK);
    let (doc, b_copy) = common::world_copy(session.committed_doc(), b, tol);
    let mut session = common::inline_without_door(
        drawn::pinned(doc, b_copy, &drawn::translated([0.0, 0.1, 0.0])),
        tol,
    );
    session.pump();
    let plane = common::cut_across(&session, b, MID_Z);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: b.into(),
            plane,
        },
    );
    session.pump();
    let Some(Node::Split { tool, .. }) = session.committed_doc().node(split) else {
        panic!("the split")
    };
    let var = tool.read;
    let renamed = session.perform(SessionOp::RenameVar {
        var,
        name: Some(pncad::document::VarName::new("cut_b").expect("a name")),
    });
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    session.pump();
    let doc = session.committed_doc().clone();
    assert_ne!(doc.node_space(a), doc.node_space(b), "two spaces");
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Node(a)),
            SessionOp::Select(Selection::Variable(var)),
        ],
    );
    let op = mirror_op(&tools);
    let before = session.committed_doc().clone();
    let out = session.perform(op);
    assert!(
        matches!(
            out.refusal.as_ref(),
            Some(Refusal::Edit(error))
                if matches!(error.as_ref(), EditError::SpaceMismatch { .. })
        ),
        "a plane variable of another space refuses naming the spaces, not {:?}",
        out.refusal
    );
    eprintln!(
        "said: {}",
        out.refusal
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    );
    assert!(session.committed_doc().bit_eq(&before), "nothing lands");
}

/// **A mirror over a variable undoes, redoes, survives a rename and
/// fails typed once its variable is gone**: undo takes the mirror and
/// its union away bit-for-bit, redo brings back a mirror reading the
/// same variable, a rename leaves it reading the same id and standing
/// where it stood, and deleting the variable leaves it failing on its
/// own row, which a further undo restores.
#[test]
fn a_mirror_over_a_variable_undoes_redoes_and_outlives_a_rename() {
    let tol = Tol::witness();
    let (mut session, block, split, var) = split_with_named_plane(
        |session, block| common::cut_across(session, block, MID_Z),
        "cut",
        tol,
    );
    let before = session.committed_doc().clone();
    let op = SessionOp::AddMirror {
        body: Operand::output(split, pncad::select::SplitHalf::Below.port()),
        plane: viewer::session::ToolPlane::Read(pncad::document::pose::RoleRead::plain(
            Operand::Var(var),
        )),
    };
    let _ = block;
    let (image, _) = mirror_in(&mut session, op);
    let stood = corners(&mut session, image);
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    session.pump();
    assert!(
        session.committed_doc().bit_eq(&before),
        "undo takes it away"
    );
    let redone = session.perform(SessionOp::Redo);
    assert!(redone.refusal.is_none(), "{:?}", redone.refusal);
    session.pump();
    let Some(Node::Mirror { plane, .. }) = session.committed_doc().node(image) else {
        panic!("redo brings the mirror back under its id")
    };
    assert_eq!(plane.read, var, "reading the same variable");
    assert_eq!(
        corners(&mut session, image),
        stood,
        "and standing where it stood"
    );

    let renamed = session.perform(SessionOp::RenameVar {
        var,
        name: Some(pncad::document::VarName::new("datum").expect("a name")),
    });
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    session.pump();
    let Some(Node::Mirror { plane, .. }) = session.committed_doc().node(image) else {
        panic!("the mirror")
    };
    assert_eq!(plane.read, var, "a rename keeps the id");
    assert_eq!(corners(&mut session, image), stood, "and the image");

    let deleted = session.perform(SessionOp::DeleteVar { var });
    eprintln!("delete: {:?}", deleted.refusal);
    if deleted.refusal.is_none() {
        session.pump();
        let eval = session.evaluation().expect("landed");
        assert!(
            matches!(eval.nodes.get(&image), Some(NodeResult::Failed(_))),
            "a mirror whose variable is gone fails on its own row: {:?}",
            eval.nodes.get(&image)
        );
        let back = session.perform(SessionOp::Undo);
        assert!(back.refusal.is_none(), "{:?}", back.refusal);
        session.pump();
        assert_eq!(corners(&mut session, image), stood, "undo restores it");
    }
}

/// **A held variable that is deleted is dropped from the mirror, said
/// once**, as the split's is (`PlaneEvent::DefinedLost`).
#[test]
fn a_deleted_variable_drops_the_mirror_plane() {
    let tol = Tol::witness();
    let (mut session, _, _split, var) = split_with_named_plane(
        |session, block| common::cut_across(session, block, MID_Z),
        "cut",
        tol,
    );
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Face(displayed_face_at(
                &session,
                &plate_index(&session),
                &common::up_at(0.0, 0.0),
            ))),
            SessionOp::Select(Selection::Variable(var)),
        ],
    );
    assert!(tools.mirror().and_then(|t| t.plane()).is_some(), "held");
    let deleted = session.perform(SessionOp::DeleteVar { var });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    tools.respeak(session.doc());
    session.pump();
    let notices = tools.reconcile(session.doc(), session.landed_pair());
    let lost = notices
        .iter()
        .filter(|n| matches!(n, ToolNotice::Mirror(PlaneEvent::DefinedLost { .. })))
        .count();
    assert_eq!(lost, 1, "one drop: {notices:?}");
    assert!(
        tools.mirror().and_then(|t| t.plane()).is_none(),
        "and none held"
    );
    assert!(
        matches!(
            tools.mirror().expect("open").op(),
            Err(SeatError::NoPlane { .. })
        ),
        "the commit names the missing plane again"
    );
}

/// **Picks a plane seat does not take leave it as it was**, for both
/// tools: a variable that is no pose (a declared length) neither
/// replaces a held face nor seats a body; a variable picked before any
/// body holds the plane alone; an edge pick seats its body and keeps
/// the plane.
#[test]
fn a_non_pose_variable_is_no_plane_for_either_tool() {
    let tol = Tol::witness();
    let (mut session, block, _split, var) = split_with_named_plane(
        |session, block| common::cut_across(session, block, MID_Z),
        "cut",
        tol,
    );
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::new("w").expect("a name"),
        value: pncad::document::FreeVar::continuous(Dimension::Length, 0.002),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let length = session.committed_doc().var_named("w").expect("w");
    let index = plate_index(&session);
    let floor = displayed_face_at(&session, &index, &common::up_at(0.0, 0.0));
    let _ = block;
    for kind in [ToolKind::Mirror, ToolKind::Split] {
        let mut tools = Tools::new();
        tools.open(kind);
        let notices = tools.feed(
            session.committed_doc(),
            Err(viewer::pickcache::NotIndexed::Absent),
            &[
                SessionOp::Select(Selection::Face(floor.clone())),
                SessionOp::Select(Selection::Variable(length)),
            ],
        );
        assert!(
            matches!(
                &notices[..],
                [ToolNotice::Mirror(PlaneEvent::NotAPlane { var, .. })
                    | ToolNotice::Split(PlaneEvent::NotAPlane { var, .. })]
                    if var.id() == length
            ),
            "{kind:?}: the length is declined, said once: {notices:?}"
        );
        let face = match kind {
            ToolKind::Mirror => tools.mirror().and_then(|t| t.face()).cloned(),
            _ => tools.split().and_then(|t| t.face()).cloned(),
        };
        assert_eq!(
            face.as_ref(),
            Some(&floor),
            "{kind:?}: a length keeps the face"
        );

        let mut tools = Tools::new();
        tools.open(kind);
        let _ = tools.feed(
            session.committed_doc(),
            Err(viewer::pickcache::NotIndexed::Absent),
            &[SessionOp::Select(Selection::Variable(var))],
        );
        let op = match kind {
            ToolKind::Mirror => tools.mirror().expect("open").op(),
            _ => tools.split().expect("open").op(),
        };
        assert!(
            matches!(op, Err(SeatError::Empty { .. })),
            "{kind:?}: a variable alone seats no body: {op:?}"
        );
    }
}

/// **A mirror reading the split's still-anonymous plane** — a second
/// reader of a variable one node already reads (VR7) — refuses typed
/// at the commit, and lands nothing.
#[test]
fn a_mirror_over_an_anonymous_plane_variable_refuses_typed() {
    let tol = Tol::witness();
    let (mut session, block) = unplaced_block(tol);
    let plane = common::cut_across(&session, block, MID_Z);
    let split = session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: block.into(),
            plane,
        },
    );
    session.pump();
    let Some(Node::Split { tool, .. }) = session.committed_doc().node(split) else {
        panic!("the split")
    };
    let var = tool.read;
    let mut tools = Tools::new();
    tools.open(ToolKind::Mirror);
    let _ = tools.feed(
        session.committed_doc(),
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Face(displayed_face_at(
                &session,
                &plate_index(&session),
                &common::up_at(0.0, 0.0),
            ))),
            SessionOp::Select(Selection::Variable(var)),
        ],
    );
    assert!(tools.mirror().and_then(|t| t.plane()).is_some(), "held");
    {
        let op = mirror_op(&tools);
        let before = session.committed_doc().clone();
        let out = session.perform(op);
        eprintln!(
            "anonymous: {:?} / {}",
            out.refusal,
            out.refusal
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default()
        );
        assert!(out.refusal.is_some(), "a second reader refuses");
        assert!(session.committed_doc().bit_eq(&before), "nothing lands");
    }
}

/// **A face picked after a variable seats the body and keeps the
/// variable**, for both tools: with no body held, the variable is the
/// plane and the face fills only the empty body seat; once the body is
/// held, a further face pick re-picks the plane.
///
/// Red if a face pick drops a held variable while seating the body, or
/// a face picked with the body held does not re-pick the plane.
#[test]
fn a_face_after_a_variable_seats_the_body_and_keeps_the_variable() {
    let tol = Tol::witness();
    let (session, _block, split, var) = split_with_named_plane(
        |session, block| common::cut_across(session, block, MID_Z),
        "cut",
        tol,
    );
    let index = plate_index(&session);
    let floor = displayed_face_at(&session, &index, &common::up_at(0.0, 0.0));
    assert_eq!(floor.node, split, "the premise: the floor is the split's");
    for kind in [ToolKind::Mirror, ToolKind::Split] {
        let mut tools = Tools::new();
        tools.open(kind);
        let feed = |tools: &mut Tools, pick: Selection| {
            let notices = tools.feed(
                session.committed_doc(),
                Err(viewer::pickcache::NotIndexed::Absent),
                &[SessionOp::Select(pick)],
            );
            assert!(notices.is_empty(), "{kind:?}: taken: {notices:?}");
        };
        let held = |tools: &Tools| match kind {
            ToolKind::Mirror => {
                let tool = tools.mirror().expect("open");
                (tool.body(), tool.plane().cloned())
            }
            _ => {
                let tool = tools.split().expect("open");
                (tool.body(), tool.plane().cloned())
            }
        };
        feed(&mut tools, Selection::Variable(var));
        feed(&mut tools, Selection::Face(floor.clone()));
        let (body, plane) = held(&tools);
        assert_eq!(body, Some(split), "{kind:?}: the face seats its body");
        assert!(
            matches!(
                &plane,
                Some(viewer::combine::HeldPlane::Defined { var: held, .. }) if held.id() == var
            ),
            "{kind:?}: and the variable stays the plane: {plane:?}"
        );
        feed(&mut tools, Selection::Face(floor.clone()));
        let (_, plane) = held(&tools);
        assert!(
            matches!(&plane, Some(viewer::combine::HeldPlane::Face(_))),
            "{kind:?}: with the body held, a face re-picks the plane: {plane:?}"
        );
    }
}
