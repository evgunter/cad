//! **The at-rest badge on a mate read below a pattern.**
//!
//! A mate's at-rest record is minted on the world copies of the bodies
//! its sides read. On the issue's document — a family over a moved
//! body of the shelf, the mate read AT that body — no world placement
//! reads the moved body, so the mate records nothing: it places the
//! post in the space of the body it is read at, which no placement
//! copies into the world, so the post is no copy the checks see.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::common::insert_into;
use crate::fixture;

use common::asm;
use pncad::document::{
    BodyRead, DocumentId, Formula, Node, Operand, Placed, ProfileDoc, VarId, stand,
};
use pncad::geom_core::Tol;
use pncad::workspace::Workspace;
use viewer::session::{AtRestBadge, DocSession, SessionOp};

/// The issue's document over the bench's parts: the shelf on the world,
/// lifted clear of itself as the second member of a family of it in a
/// space of its own — a body no world placement copies — a two-member
/// family of the lifted shelf with both members placed in the world, and the post placed by the seat mate read
/// AT the lifted body. Stored beside the bench's parts so the session's
/// resolver finds them.
fn read_below_pattern(bench: &asm::Bench, tol: Tol) -> std::path::PathBuf {
    let mut asm = ProfileDoc::empty(DocumentId::derive("msolve5-viewer"), tol);
    let post = insert_into(&mut asm, crate::fixture::mated_instance(bench.post), tol);
    let shelf = asm::instance_at(&mut asm, bench.shelf, [0.0; 3], &bench.opts(), tol);
    // The lift lives in a space of its own, which no placement reaches
    // the world from: the lifted body is copied into the world only as
    // the second family's members.
    let room = insert_into(&mut asm, Node::Space, tol);
    let room = Operand::Var(asm.output(room, 0).expect("a new space defines its space"));
    let (doc, lifting) = fixture::index(asm, "lift", 2);
    let lifted_by = Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(lifting, [0.0, 0.0, 1.0], 0.1),
    );
    let (doc, lift) = fixture::insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(shelf)),
            room.clone(),
            lifted_by.bundle(),
        ),
    );
    let (doc, lifted) = fixture::insert(
        doc,
        Node::place(
            BodyRead::indexed(lift, vec![Formula::count(1)]),
            room.clone(),
            stand().bundle(),
        ),
    );
    // The second copy clears the post and the first copy.
    let (doc, stepping) = fixture::index(doc, "step", 2);
    let stepped = Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(stepping, [1.0, 0.0, 0.0], 2.0 * asm::SHELF_LENGTH),
    );
    let (doc, pattern) = fixture::insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(lifted)),
            room,
            stepped.bundle(),
        ),
    );
    // The world (A10): each of the family's two members copied in.
    let mut asm = doc;
    for index in 0..2 {
        insert_into(
            &mut asm,
            Node::place(
                BodyRead::indexed(pattern, vec![Formula::count(index)]),
                Operand::Var(VarId::WORLD),
                stand().bundle(),
            ),
            tol,
        );
    }
    // The shelf's bottom as the lifted body names it: the member's
    // name, as the lift's second member.
    let b = pncad::prelude::StableName {
        kind: pncad::prelude::EntityKind::Face,
        node: lift,
        path: vec![pncad::prelude::RoleSeg::Member {
            at: vec![(lifting, 1)],
            of: pncad::prelude::NameRef::new(bench.shelf_bottom.clone()),
        }],
    };
    common::mate_into(
        &mut asm,
        post,
        crate::fixture::rest(
            common::head(asm::in_part(post, &bench.post_top)),
            common::head_at(lifted, b),
        ),
        asm::own_space_seat_values(),
        tol,
    );
    // The product is gathered once the mate has placed the post, which
    // stands in the lift's space with the body it is read at.
    let asm = editor_core::test_support::assemble_world_copies(asm, tol);
    let mut ws = Workspace::open(&bench.dir).expect("the bench's workspace opens");
    ws.create(&asm, tol).expect("the assembly stores")
}

#[test]
fn a_mate_read_below_a_pattern_records_nothing_and_places_the_post_beside_it() {
    let tol = Tol::witness();
    let bench = asm::bench("msolve5-badge", tol);
    let path = read_below_pattern(&bench, tol);
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
    // The mate records nothing and refuses nothing: the badge stays
    // green.
    assert!(
        matches!(session.at_rest(), Some(AtRestBadge { .. })),
        "an unmated seat refuses nothing: {:?}",
        session.at_rest()
    );
    // The post stands in the lift's space, beside the body its mate
    // reads, and no placement copies it into the world.
    let doc = session.committed_doc();
    let posts_off_world: Vec<_> = doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
        .filter(|&id| doc.node_space(id) != Some(VarId::WORLD))
        .collect();
    assert_eq!(posts_off_world.len(), 1, "the post is placed off the world");
    let report = session.checks().expect("the checks ran");
    assert!(
        report.findings.is_empty(),
        "no copy the checks see touches another: {report:?}"
    );
}
