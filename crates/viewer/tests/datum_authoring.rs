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
    let outcome = session.perform(SessionOp::AddDatum {
        space,
        kind: VarKind::Plane,
    });
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
            kind: VarKind::Axis,
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
            kind: VarKind::Plane,
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

/// **A datum picked whole fills the split's plane**, read through its xy
/// plane, as a frame variable picked in the variables list does.
#[test]
fn a_datum_picked_whole_is_the_splits_plane_through_its_xy() {
    use viewer::combine::HeldPlane;
    use viewer::session::Selection;
    use viewer::tools::{ToolKind, Tools};
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("datum-split", tol);
    let space = doc.node_space(block).expect("the block is in its space");
    let mut session = DocSession::inline(doc, tol);
    let datum = session
        .perform(SessionOp::AddDatum {
            space,
            kind: VarKind::Plane,
        })
        .made
        .expect("the op makes the datum");
    let doc = session.committed_doc();
    let mut tools = Tools::new();
    tools.open(ToolKind::Split);
    let notices = tools.feed(
        doc,
        Err(viewer::pickcache::NotIndexed::Absent),
        &[
            SessionOp::Select(Selection::Node(block)),
            SessionOp::Select(Selection::Node(datum)),
        ],
    );
    assert!(notices.is_empty(), "both picks land: {notices:?}");
    let split = tools.split().expect("the split is open");
    assert_eq!(split.target(), Some(block), "the block is the body split");
    assert!(
        matches!(
            split.plane(),
            Some(HeldPlane::Defined {
                through: Some(PoseRole::Xy),
                ..
            })
        ),
        "the plane is the datum's frame through its xy: {:?}",
        split.plane()
    );
}
