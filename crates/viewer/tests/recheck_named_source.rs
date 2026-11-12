//! Re-check probes for PR 6856 (lane placing-recheck-6856), viewer side.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{Doc, EditError, Operand, RecipeNodeId};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, SessionOp};

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

fn landed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a node")
}

fn pattern(input: RecipeNodeId, axis: pncad::document::TargetAxis) -> SessionOp {
    SessionOp::AddPattern {
        input: input.into(),
        count: 2,
        rule: viewer::session::PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(axis),
            spacing: common::len(3.0 * SIZE[0]),
        },
    }
}

/// **The re-point offer leaves out exactly what the door refuses**: a
/// plain copy `c` of `body`, a pattern of `c`; re-pointing `c`'s source
/// onto a family `row` makes `c` a family under a pattern, which the door
/// refuses. The offer should then have no row for that source.
#[test]
fn recheck_the_offer_has_no_row_the_door_refuses() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("rc-offer", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    let other = common::xy_box_in(&mut session, SIZE);
    session.pump();
    let row = landed(&mut session, pattern(other, pncad::document::TargetAxis::Y));
    let op = SessionOp::place_where_it_stands(session.committed_doc(), Operand::Node(body))
        .expect("placeable");
    let c = landed(&mut session, op);
    let _ = landed(&mut session, pattern(c, pncad::document::TargetAxis::X));
    let doc = session.committed_doc().clone();
    let source = doc.copied_read(c).expect("a source");
    let body_var = doc.through_sources(source).read;
    let out = session.perform(SessionOp::RepointPlacements {
        source,
        onto: row.into(),
    });
    let refused = out.refusal;
    eprintln!("repoint: {refused:?}");
    let offered = viewer::world::repoint_targets(&doc, body_var, row)
        .iter()
        .any(|r| r.source == source);
    eprintln!("offered: {offered}");
    if let Some(viewer::session::Refusal::Edit(e)) = &refused {
        let _: &EditError = e.as_ref();
        assert!(
            !offered,
            "the offer has a row the door refuses: {refused:?}"
        );
    }
}
