//! **The at-rest badge on a mate read below a pattern.**
//!
//! A mate's at-rest record is minted on the world copies of the bodies
//! its sides read. On the issue's document — a pattern over a moved
//! copy of the shelf, the mate read AT that copy — no world placement
//! reads the moved copy, so the mate records nothing, and the seat it places
//! the post on reaches the checks as a touch between two copies.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use crate::common::{insert_into, len, scl};

use common::asm;
use pncad::document::{DocumentId, Formula, Node, PatternKind, ProfileDoc};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::session::{AtRestBadge, DocSession, SessionOp};

/// The issue's document over the bench's parts: the shelf on the world,
/// lifted clear of itself by a placement against its own geometry, a
/// two-copy pattern of the lifted shelf with both copies placed, and the
/// post placed by the seat mate read AT the lifted copy. Stored beside the bench's parts so
/// the session's resolver finds them.
fn read_below_pattern(bench: &asm::Bench, tol: Tol) -> std::path::PathBuf {
    let mut asm = ProfileDoc::empty(DocumentId::derive("msolve5-viewer"), tol);
    let post = insert_into(&mut asm, crate::fixture::mated_instance(bench.post), tol);
    let shelf = asm::instance_at(&mut asm, bench.shelf, [0.0; 3], &bench.opts(), tol);
    let (moved, lifted) = crate::fixture::xform(asm, shelf, [0.0, 0.0, 0.1], [0.0, 0.0, 1.0], 0.0);
    asm = moved;
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
    // The world (A10): each of the pattern's two copies through a `Part`
    // per copy.
    for index in 0..2 {
        let copy = insert_into(
            &mut asm,
            Node::Part {
                of: pattern.into(),
                select: pncad::document::PartSelect::Instance(Formula::count(index)),
            },
            tol,
        );
        asm = common::placed_read(&asm, copy.into(), &bench.opts(), tol).0;
    }
    // The shelf's bottom as the lifted copy names it: the instance's
    // name, under the placement that copies it.
    let b = pncad::prelude::StableName {
        kind: pncad::prelude::EntityKind::Face,
        node: lifted,
        path: vec![pncad::prelude::RoleSeg::Placed {
            of: pncad::prelude::NameRef::new(asm::in_part(shelf, &bench.shelf_bottom)),
        }],
    };
    common::mate_into(
        &mut asm,
        post,
        crate::fixture::rest(
            common::head(asm::in_part(post, &bench.post_top)),
            common::head_at(lifted, b),
            ContactClass::Rest,
        ),
        asm::middle_seat_values(),
        tol,
    );
    let mut ws = Workspace::open(&bench.dir).expect("the bench's workspace opens");
    ws.create(&asm, tol).expect("the assembly stores")
}

#[test]
fn a_mate_read_below_a_pattern_records_nothing_and_its_seat_is_a_finding() {
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
    // The seat the mate records nothing for is a touch between two copies:
    // the gather records it, the badge stays green, and the checks
    // window reports it against the two copies (D10).
    assert!(
        matches!(session.at_rest(), Some(AtRestBadge::Certified { .. })),
        "an unmated seat refuses nothing: {:?}",
        session.at_rest()
    );
    let report = session.checks().expect("the checks ran");
    assert!(
        report
            .findings
            .iter()
            .any(|f| matches!(f.subject, pncad::document::FindingSubject::Copies { .. })),
        "the seat is the two copies' finding: {report:?}"
    );
}
