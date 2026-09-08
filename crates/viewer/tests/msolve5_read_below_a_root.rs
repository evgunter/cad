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

use common::asm;
use pncad::document::{
    AssemblyError, Dimension, DocEdit, DocumentId, Expr, MateSide, Node, PatternKind, ProfileDoc,
    ProfileProgram, RecipeNodeId, RefusedRef, SitedRef, apply,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::session::{AtRestBadge, DocSession, SessionOp};

fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a length literal")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a scalar literal")
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode { node },
        tol,
        &pncad::document::RefusingReach,
    )
    .expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

/// The issue's document over the bench's parts: a post, the shelf
/// lifted by a transform, a two-copy pattern of the lifted shelf, and
/// the seat mate read AT the transform. Stored beside the bench's
/// parts so the session's resolver finds them.
fn read_below_a_root(bench: &asm::Bench, tol: Tol) -> (std::path::PathBuf, AssemblyError) {
    let mut asm = ProfileDoc::empty(DocumentId::derive("msolve5-viewer"), tol);
    let post = insert(&mut asm, Node::instantiate_part(bench.post), tol);
    let shelf = insert(&mut asm, Node::instantiate_part(bench.shelf), tol);
    let lifted = insert(
        &mut asm,
        Node::Transform {
            input: shelf,
            translation: [len(0.0), len(0.0), len(0.05)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("an angle literal"),
        },
        tol,
    );
    // The second copy clears the post and the first copy.
    insert(
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
    let mate = insert(
        &mut asm,
        Node::Mate {
            a: SitedRef::at_mint(asm::in_part(post, &bench.post_top)),
            b: SitedRef::new(lifted, b.clone()),
            class: ContactClass::Rest,
            alignment: asm::seat_alignment(asm::SHELF_LENGTH / 2.0, None),
        },
        tol,
    );
    let mut ws = Workspace::open(&bench.dir).expect("the bench's workspace opens");
    let path = ws.create(&asm, tol).expect("the assembly stores");
    let expected = AssemblyError::Reference {
        mate,
        side: MateSide::B,
        name: Box::new(b),
        why: RefusedRef::ReadBelowARoot { at: lifted },
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
