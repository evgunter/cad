//! **The at-rest badge names the operand for a mate read below a
//! product root.**
//!
//! The badge renders the gate's own `Display`, so the viewer needs no
//! code for the arm: this row pins that the sentence a user reads on
//! the issue's document — a pattern over a transform over the shelf,
//! the mate read AT the transform — IS the gate's `ReadBelowARoot`
//! refusal, word for word. The words themselves are pinned once, in
//! `editor-core`'s `display_contract`.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::common::{ang, insert_into, len, scl};

use common::asm;
use pncad::document::{
    AssemblyError, DocumentId, Expr, MateSide, MintRefusal, Node, PatternKind, ProfileDoc,
    RefusedRef,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::session::{AtRestBadge, DocSession, SessionOp};

/// The issue's document over the bench's parts: a post, the shelf
/// lifted by a transform, a two-copy pattern of the lifted shelf, and
/// the seat mate read AT the transform. Stored beside the bench's
/// parts so the session's resolver finds them.
fn read_below_a_root(bench: &asm::Bench, tol: Tol) -> (std::path::PathBuf, AssemblyError) {
    let mut asm = ProfileDoc::empty(DocumentId::derive("msolve5-viewer"), tol);
    let post = insert_into(&mut asm, Node::instantiate_part(bench.post), tol);
    let shelf = insert_into(&mut asm, Node::instantiate_part(bench.shelf), tol);
    let lifted = insert_into(
        &mut asm,
        Node::Transform {
            input: shelf,
            translation: [len(0.0), len(0.0), len(0.05)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
        tol,
    );
    // The second copy clears the post and the first copy.
    insert_into(
        &mut asm,
        Node::Pattern {
            input: lifted,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0 * asm::SHELF_LENGTH),
            },
        },
        tol,
    );
    let b = asm::in_part(shelf, &bench.shelf_bottom);
    let mate = insert_into(
        &mut asm,
        Node::Mate {
            a: common::head(asm::in_part(post, &bench.post_top)),
            b: common::head_at(lifted, b.clone()),
            class: ContactClass::Rest,
            alignment: asm::middle_seat(),
        },
        tol,
    );
    let mut ws = Workspace::open(&bench.dir).expect("the bench's workspace opens");
    let path = ws.create(&asm, tol).expect("the assembly stores");
    let expected = AssemblyError::Mint {
        refusals: vec![MintRefusal::Reference {
            mate,
            side: MateSide::B,
            name: Box::new(b),
            why: RefusedRef::ReadBelowARoot { at: lifted },
        }],
    };
    (path, expected)
}

#[test]
fn the_badge_names_the_operand_of_a_mate_read_below_a_root() {
    let tol = Tol::witness();
    let bench = asm::bench("msolve5-badge", tol);
    let (path, expected) = read_below_a_root(&bench, tol);
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
        Some(AtRestBadge::Refused { message }) => assert_eq!(
            *message,
            expected.to_string(),
            "the badge is the gate's own refusal, word for word"
        ),
        other => panic!("a mate read below a root turns the badge red, got {other:?}"),
    }
}
