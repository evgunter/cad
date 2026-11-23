//! **A datum, created then edited** (D10 `[placement.datums]`): New
//! plane inserts a datum into a body's space at its origin, the
//! placement editor opens on it, and a face pick fills its first mate
//! row's far side; the datum's own xy plane is the near side.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::pose::{PoseRole, RoleRead};
use pncad::document::{Node, Operand, VarKind};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::matetool::MateTool;
use viewer::session::{DocSession, FaceSelection, SessionOp};

#[test]
fn a_new_plane_is_a_datum_its_editor_mates_flush_on_a_picked_face() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("datum-authoring", tol);
    let space = doc.node_space(block).expect("the block is in its space");
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::AddDatum { space });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let datum = outcome.made.expect("the op makes the datum");
    assert!(
        matches!(session.doc().node(datum), Some(Node::Datum { .. })),
        "the node is a datum"
    );
    assert_eq!(
        session.doc().node_space(datum),
        Some(space),
        "in the block's space"
    );

    let mut tool = MateTool::edit(session.doc(), datum).expect("the editor opens on a datum");
    tool.add_mate(VarKind::Plane);
    let top = FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: block,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: block,
        body: 0,
    };
    assert!(tool.pick(session.doc(), top).is_none(), "the pick is taken");
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, ev, tol)
        .expect("the datum's bundle proposes");
    let [mate] = &proposal.mates[..] else {
        panic!("one mate: {:?}", proposal.mates)
    };
    assert_eq!(
        mate.mate.on,
        RoleRead::through(
            Operand::Output {
                node: datum,
                port: 0
            },
            PoseRole::Xy
        ),
        "the near side is the datum's own xy plane"
    );
    assert!(!mate.to.is_empty(), "the far side is the picked face");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let Some(Node::Datum { constraints, .. }) = session.doc().node(datum) else {
        panic!("still a datum")
    };
    assert_eq!(
        constraints.iter().filter(|c| c.mate().is_some()).count(),
        1,
        "the datum is mated on the face"
    );
    session.pump();
    let rows = session.tree_rows();
    let row = rows
        .iter()
        .find(|row| row.id == datum)
        .expect("the datum has a tree row");
    assert_eq!(
        row.constraints.iter().filter(|c| c.is_mate).count(),
        1,
        "the datum's row lists its mate, which the edit placement button edits"
    );
    let outcome = session.perform(SessionOp::DeleteNode { node: datum });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.doc().node(datum).is_none(), "the datum deletes");
}

/// **A datum is drawn as a display aid in its space**: its xy plane's
/// square about its origin, which the pick index measures a cursor
/// against as it measures a committed profile's outline, so a pick on it
/// selects the datum.
#[test]
fn a_datum_is_drawn_as_a_square_on_its_xy_plane() {
    let tol = Tol::witness();
    let (doc, _, _) = viewer::test_support::boss_on_block("datum-drawn", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = session
        .perform(SessionOp::AddDatum {
            space: pncad::document::VarId::WORLD,
        })
        .made
        .expect("the op makes the datum");
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let drawn = viewer::sketch::committed(doc, ev, 1e-4, None);
    let datum_drawn = drawn
        .drawn
        .iter()
        .find(|d| d.node == datum)
        .expect("the datum is drawn");
    let h = viewer::sketch::DATUM_HALF;
    let square = datum_drawn.loops[0].drawn_on(&datum_drawn.plane);
    let corners: Vec<[f64; 3]> = square.iter().map(|p| [p.x, p.y, p.z]).collect();
    assert_eq!(
        corners,
        vec![
            [-h, -h, 0.0],
            [h, -h, 0.0],
            [h, h, 0.0],
            [-h, h, 0.0],
            [-h, -h, 0.0]
        ],
        "a datum at the world's origin is drawn on the world's xy plane"
    );
}

/// **A copy rests on a datum picked whole**: the placement editor on the
/// block's world copy, its bottom cap picked on the copy and a world
/// datum picked whole, mates the cap's plane to the datum's xy plane,
/// turned to face it, even with the datum at all-zero values.
#[test]
fn a_datum_picked_whole_is_a_mates_far_side_through_its_role() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("datum-far-side", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = session
        .perform(SessionOp::AddDatum {
            space: pncad::document::VarId::WORLD,
        })
        .made
        .expect("the op makes the datum");
    session.pump();
    let doc = session.doc();
    let copy = doc
        .placements()
        .into_iter()
        .find(|&p| {
            doc.copied_read(p)
                .is_some_and(|read| doc.operation_of(doc.through_sources(read).read) == Some(block))
        })
        .expect("the block's copy");
    let mut tool = MateTool::moving(doc, copy, None).expect("the editor opens on the copy");
    tool.add_mate(VarKind::Plane);
    let bottom = FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: block,
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        },
        node: copy,
        body: 0,
    };
    assert!(tool.pick(doc, bottom).is_none(), "the cap is taken");
    tool.pick_datum(doc, datum);
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, ev, tol)
        .expect("the copy's bundle proposes");
    let [mate] = &proposal.mates[..] else {
        panic!("one mate: {:?}", proposal.mates)
    };
    let own = RoleRead::through(
        Operand::Output {
            node: datum,
            port: 0,
        },
        PoseRole::Xy,
    );
    assert!(
        matches!(
            &mate.mate.to.read,
            Operand::Pose(def) if matches!(
                &**def,
                pncad::document::pose::PoseDef::Flip { pose } if *pose == own
            )
        ),
        "the far side is the datum's xy plane, turned to face the cap: {:?}",
        mate.mate.to
    );
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

/// **A datum picked whole fills the split's and the mirror's plane**,
/// read through its xy plane, as a frame variable picked in the
/// variables list does.
#[test]
fn a_datum_picked_whole_is_a_plane_seats_plane_through_its_xy() {
    use viewer::combine::HeldPlane;
    use viewer::session::Selection;
    use viewer::tools::{ToolKind, Tools};
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("datum-plane-seat", tol);
    let space = doc.node_space(block).expect("the block is in its space");
    let mut session = DocSession::inline(doc, tol);
    let datum = session
        .perform(SessionOp::AddDatum { space })
        .made
        .expect("the op makes the datum");
    let doc = session.committed_doc();
    for kind in [ToolKind::Split, ToolKind::Mirror] {
        let mut tools = Tools::new();
        tools.open(kind);
        let notices = tools.feed(
            doc,
            Err(viewer::pickcache::NotIndexed::Absent),
            &[
                SessionOp::Select(Selection::Node(block)),
                SessionOp::Select(Selection::Node(datum)),
            ],
        );
        assert!(notices.is_empty(), "{kind:?}: both picks land: {notices:?}");
        let (body, plane) = match kind {
            ToolKind::Split => {
                let tool = tools.split().expect("the split is open");
                (tool.body(), tool.plane())
            }
            _ => {
                let tool = tools.mirror().expect("the mirror is open");
                (tool.body(), tool.plane())
            }
        };
        assert_eq!(body, Some(block), "{kind:?}: the block is the body");
        assert!(
            matches!(
                plane,
                Some(HeldPlane::Defined {
                    through: Some(PoseRole::Xy),
                    ..
                })
            ),
            "{kind:?}: the plane is the datum's frame through its xy: {plane:?}"
        );
    }
}

/// **A datum offset from a face** is its plane mate's far side slid along
/// the face's outward normal (the editor's offset row, `set_offset`):
/// the datum stands `0.005` above the block's top cap, its `z` the cap's
/// outward normal.
#[test]
fn a_datum_offset_from_a_face_stands_off_it_along_its_normal() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("datum-offset", tol);
    let space = doc.node_space(block).expect("the block is in its space");
    let depth = viewer::test_support::BOSS_BLOCK[2];
    let mut session = DocSession::inline(doc, tol);
    let datum = session
        .perform(SessionOp::AddDatum { space })
        .made
        .expect("the op makes the datum");
    let mut tool = MateTool::edit(session.doc(), datum).expect("the editor opens on a datum");
    tool.add_mate(VarKind::Plane);
    let top = FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: block,
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        node: block,
        body: 0,
    };
    assert!(tool.pick(session.doc(), top).is_none(), "the pick is taken");
    tool.set_offset(0, Some(viewer::test_support::len(0.005)));
    session.pump();
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, ev, tol)
        .expect("the datum's bundle proposes");
    let outcome = session.perform(proposal.op());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (_, ev) = session.landed_pair().expect("landed");
    let Some(pncad::document::NodeResult::Ok(value)) = ev.nodes.get(&datum) else {
        panic!("the datum evaluates: {:?}", ev.nodes.get(&datum))
    };
    let pncad::document::ValuePayload::Datum(frame) = &value.payload else {
        panic!("a datum's value is its frame: {:?}", value.payload)
    };
    let origin = frame.origin();
    let z = frame.w().get();
    assert!(
        (origin.z - (depth + 0.005)).abs() < 1e-12,
        "the datum stands 0.005 above the cap at {depth}: {origin:?}"
    );
    assert!(
        (z.z - 1.0).abs() < 1e-12,
        "its z is the cap's outward normal: {z:?}"
    );
}
