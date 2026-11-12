//! Review probes for PR 6856 (lane `placing-dual-6856-r1`): viewer rows
//! that try to falsify the named-source claims. Each asserts what the
//! ruling asks for; a red row is a finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use core::f64::consts::TAU;

use common::{ang, circle_at, len, session_insert};
use pncad::document::{Doc, Formula, Operand, ProfileProgram};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

/// A revolve (body, axis and space: three ports) in a session of its own.
fn revolved(tol: Tol) -> (DocSession, pncad::document::RecipeNodeId) {
    let mut session = DocSession::inline(Doc::empty_derived("r1-revolve", tol), tol);
    let profile = session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![circle_at([0.3, 0.0], 0.07)],
        },
    );
    let revolve = session_insert(
        &mut session,
        SessionOp::AddRevolve {
            profile,
            axis_through: [len(0.0), len(0.0)],
            axis_heading: Formula::turn_over(4).expect("a quarter turn"),
            angle: ang(TAU),
        },
    );
    session.pump();
    (session, revolve)
}

/// **A pattern of a revolve** places it through a source. The pattern
/// gesture reads a revolve as its body variable (`as_body`: a revolve
/// read whole names two outputs), and `Node::place` writes a variable
/// read as given, so the door may refuse `PlacementReadsNoSource`.
#[test]
fn probe_a_pattern_of_a_revolve() {
    let tol = Tol::witness();
    let (mut session, revolve) = revolved(tol);
    let out = session.perform(SessionOp::AddPattern {
        input: revolve.into(),
        count: 2,
        rule: PatternRuleSpec::Linear {
            direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
            spacing: len(1.0),
        },
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
}

/// **Place of a revolve** (the viewer's Place, which goes through
/// `place_read`): the control for the pattern row.
#[test]
fn probe_a_place_of_a_revolve() {
    let tol = Tol::witness();
    let (mut session, revolve) = revolved(tol);
    let SessionOp::Place { body, target, .. } =
        SessionOp::place_where_it_stands(session.committed_doc(), Operand::Node(revolve))
            .expect("in a space")
    else {
        unreachable!()
    };
    let out = session.perform(SessionOp::Place {
        body,
        target,
        name: None,
    });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
}

/// **The re-point of a member placement's source** (the variables
/// pane's "read the selected body", the copy-of row): `place(xs[0])`
/// reads a source of the family at index 0; redefined onto one body it
/// is `source = B` plain under an index. The committed document must
/// save.
#[test]
fn probe_repoint_of_a_member_placements_source() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("r1-member", tol), tol);
    let body = common::xy_box_in(&mut session, [0.02, 0.02, 0.01]);
    let family = session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: body.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.04),
            },
        },
    );
    let (doc, member) = common::member_read(session.committed_doc(), family, 0, tol);
    let source = doc.copied_read(member).expect("a source");
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let out = session.perform(SessionOp::RepointPlacements {
        source,
        onto: body.into(),
    });
    if out.refusal.is_none() {
        let doc: &Doc<ProfileProgram> = session.committed_doc();
        let saved = pncad::document::save(doc, &[], tol);
        assert!(
            saved.is_ok(),
            "the re-point committed a document that does not save: {:?}",
            saved.map(|_| ())
        );
    }
}
