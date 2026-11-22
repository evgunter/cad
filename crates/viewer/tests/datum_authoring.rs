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
