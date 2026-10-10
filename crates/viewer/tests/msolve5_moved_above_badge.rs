//! **The at-rest badge on a mate read below a pattern.**
//!
//! A mate's declaration is minted on the world copies of the bodies its
//! sides read. On the issue's document — a pattern over a transform
//! over the shelf, the mate read AT the transform — no placement reads
//! the transform, so the mate declares nothing, and the seat it would
//! have declared reaches the gate as an undeclared contact: the badge
//! turns red with the gate's own refusal.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::common::{ang, insert_into, len, scl};

use common::asm;
use pncad::document::{DocumentId, Formula, Node, PatternKind, ProfileDoc};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::session::{AtRestBadge, DocSession, SessionOp};

/// The issue's document over the bench's parts: a post, the shelf
/// lifted by a transform, a two-copy pattern of the lifted shelf, the
/// post and both copies placed, and the seat mate read AT the
/// transform. Stored beside the bench's
/// parts so the session's resolver finds them.
fn moved_above(bench: &asm::Bench, tol: Tol) -> std::path::PathBuf {
    let mut asm = ProfileDoc::empty(DocumentId::derive("msolve5-viewer"), tol);
    let post = insert_into(&mut asm, Node::instantiate_part(bench.post), tol);
    let shelf = insert_into(&mut asm, Node::instantiate_part(bench.shelf), tol);
    let lifted = insert_into(
        &mut asm,
        Node::transform(
            shelf,
            pncad::document::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.05)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        ),
        tol,
    );
    // The second copy clears the post and the first copy.
    let pattern = insert_into(
        &mut asm,
        Node::Pattern {
            input: lifted.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0 * asm::SHELF_LENGTH),
            },
        },
        tol,
    );
    // The world (A10): the post placed, then each of the pattern's two
    // copies through a `Part` per copy.
    insert_into(
        &mut asm,
        Node::place_in_world(post, pncad::document::Placement::IDENTITY),
        tol,
    );
    for index in 0..2 {
        let copy = insert_into(
            &mut asm,
            Node::Part {
                of: pattern.into(),
                select: pncad::document::PartSelect::Instance(Formula::count(index)),
            },
            tol,
        );
        insert_into(
            &mut asm,
            Node::place_in_world(copy, pncad::document::Placement::IDENTITY),
            tol,
        );
    }
    let b = asm::in_part(shelf, &bench.shelf_bottom);
    let mate = insert_into(
        &mut asm,
        Node::Mate {
            a: common::head(asm::in_part(post, &bench.post_top)).into(),
            b: common::head_at(lifted, b.clone()).into(),
            class: ContactClass::Rest,
            alignment: asm::middle_seat_alignment(),
        },
        tol,
    );
    let _ = mate;
    let mut ws = Workspace::open(&bench.dir).expect("the bench's workspace opens");
    ws.create(&asm, tol).expect("the assembly stores")
}

#[test]
fn a_mate_read_below_a_pattern_declares_nothing_and_the_badge_turns_red() {
    let tol = Tol::witness();
    let bench = asm::bench("msolve5-badge", tol);
    let path = moved_above(&bench, tol);
    let mut session = DocSession::inline(
        pncad::document::Doc::empty_derived("msolve5-boot", tol),
        tol,
    );
    let outcome = session.perform(SessionOp::Open(path));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert!(
        session.product_fault().is_none(),
        "the product gathers: {:?}",
        session.product_fault()
    );
    match session.at_rest() {
        Some(AtRestBadge::Refused { message }) => assert!(
            message.contains("is an undeclared contact"),
            "the seat the mate does not declare is the gate's undeclared contact: {message}"
        ),
        other => panic!("a mate read below a pattern turns the badge red, got {other:?}"),
    }
}
