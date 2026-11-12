//! **The re-point offer lists exactly what the definition door takes**
//! (VARIABLES VR4): each row is the redefinition the action writes, asked
//! of the door first, so no row is one the action then refuses. Rows
//! first written as the re-check probes of PR 6856's fix pass (lane
//! `placing-recheck-6856`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{Doc, Operand, ProfileProgram, RecipeNodeId, TargetAxis, VarId};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, Refusal, SessionOp};

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

fn landed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a node")
}

fn pattern(input: RecipeNodeId, axis: TargetAxis) -> SessionOp {
    SessionOp::AddPattern {
        input: input.into(),
        count: 2,
        rule: viewer::session::PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(axis),
            spacing: common::len(3.0 * SIZE[0]),
        },
    }
}

/// The re-point of `source` onto `onto` refuses at the door, and the
/// offer for `body` onto `onto` has no row for it.
fn refused_and_not_offered(
    session: &mut DocSession,
    body: VarId,
    source: VarId,
    onto: RecipeNodeId,
) {
    let doc = session.committed_doc().clone();
    let offered =
        viewer::world::repoint_targets(&doc, body, onto, pncad::geom_core::Tol::witness())
            .iter()
            .any(|row| row.source == source);
    let out = session.perform(SessionOp::RepointPlacements {
        source,
        onto: onto.into(),
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Edit(_))),
        "the door takes it: {:?}",
        out.refusal
    );
    assert!(!offered, "the offer has a row the door refuses");
    assert_eq!(
        session.committed_doc().var_ids(),
        doc.var_ids(),
        "nothing written"
    );
}

/// A plain copy `c` of `body` and a pattern of `c`: re-pointing `c`'s
/// source onto a family `row` would make the pattern repeat a family,
/// which the door refuses, so the offer has no row for it.
#[test]
fn the_offer_has_no_row_the_door_refuses() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("rc-offer", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let row = landed(&mut session, pattern(other, TargetAxis::Y));
    let op = SessionOp::place_where_it_stands(session.committed_doc(), Operand::Node(body))
        .expect("placeable");
    let c = landed(&mut session, op);
    let _ = landed(&mut session, pattern(c, TargetAxis::X));
    let doc = session.committed_doc().clone();
    let source = doc.copied_read(c).expect("a source");
    let body_var = doc.through_sources(source).read;
    refused_and_not_offered(&mut session, body_var, source, row);
}

/// **A family of two indices of `body`**, `i` and `j` within it, each
/// copy slid along `x` by `i` and along `y` by `j`.
fn grid_of(
    doc: &Doc<ProfileProgram>,
    body: RecipeNodeId,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    use pncad::document::{Coord, Dimension, DocEdit, Formula, Node, VarDecl, VarName};
    let declare = |doc: &Doc<ProfileProgram>, name: &'static str, within: Option<VarId>| {
        let doc = doc
            .apply(
                &DocEdit::DeclareVar {
                    name: VarName::from_static(name),
                    def: VarDecl::Index {
                        count: Formula::count(2),
                        within: within.map(|w| Formula::var(w, Dimension::Count)),
                    },
                },
                tol,
            )
            .expect("an index")
            .doc;
        let id = doc.var_named(name).expect("declared");
        (doc, id)
    };
    let (doc, i) = declare(doc, "i", None);
    let (doc, j) = declare(&doc, "j", Some(i));
    let slide = |index: VarId| {
        Formula::mul(
            Formula::count_to_scalar(Formula::var(index, Dimension::Count)).expect("a count"),
            common::len(3.0 * SIZE[0]),
        )
        .expect("a length")
    };
    let space = doc.node_space(body).expect("in a space");
    let placed = pncad::document::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::slide(TargetAxis::X), slide(i)),
            (Coord::slide(TargetAxis::Y), slide(j)),
        ],
    );
    viewer::test_support::inserted(
        &doc,
        Node::place(
            pncad::document::BodyRead::plain(Operand::Node(body)),
            Operand::Var(space),
            placed.bundle(),
        ),
        tol,
    )
}

/// A member placement of a family of one index, its source re-pointed
/// onto a family of two: the member's one index is not the new family's
/// rank, which the door refuses, so the offer has no row for it.
#[test]
fn the_offer_has_no_row_for_a_family_of_another_rank() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("rc-rank", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let family = landed(&mut session, pattern(body, TargetAxis::X));
    let (doc, grid) = grid_of(session.committed_doc(), other, tol);
    let (doc, member) = common::member_read(&doc, family, 0, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let doc = session.committed_doc().clone();
    assert_eq!(
        doc.node_reach(grid).chain().map(<[VarId]>::len),
        Some(2),
        "a family of two indices"
    );
    let source = doc.copied_read(member).expect("a source");
    let family_var = doc.through_sources(source).read;
    refused_and_not_offered(&mut session, family_var, source, grid);
}
