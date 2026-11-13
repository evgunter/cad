//! **Probe: the re-point offer does not reach through a second
//! feature** (placing program, offer depth). With `bracket = B` read by
//! two placements, `F1 = fillet(B)` picked as the body itself offers to
//! redefine `bracket`; left unaccepted, a second feature `F2 =
//! chamfer(F1)` makes `F1` an intermediate body the picture no longer
//! draws, so `F1` offers nothing, and `F2` offers nothing either: its
//! take is `F1`'s body, which no source reads (`world::repoint_offer`
//! walks the node's direct takes, `world::repoint_targets` asks
//! `sources_of` that body).
//!
//! This records what the code does today; it is the witness of the
//! offer-depth row, not a ruling on what it should do.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{Doc, Operand, ProfileProgram, RecipeNodeId, VarName};
use pncad::geom_core::Tol;
use pncad::prelude::RoleSeg;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::RowStatus;

const SIZE: [f64; 3] = [0.02, 0.02, 0.01];

/// The viewer's Place of `body` where it stands, under `name`.
fn place_op(doc: &Doc<ProfileProgram>, body: Operand, name: Option<&'static str>) -> SessionOp {
    let SessionOp::Place { body, target, .. } =
        SessionOp::place_where_it_stands(doc, body).expect("a body in a space")
    else {
        unreachable!("the viewer's Place")
    };
    SessionOp::Place {
        body,
        target,
        name: name.map(VarName::from_static),
    }
}

/// `op` performed and landed: the node it made.
fn landed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a node")
}

/// **A second feature on an unaccepted fillet: neither offers.**
///
/// *Breaks if* the offer starts reaching through a feature chain (F2
/// offers `bracket`), or an intermediate F1 keeps offering.
#[test]
fn a_feature_on_an_unaccepted_fillet_offers_nothing() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("offer-depth", tol), tol);
    let body = common::xy_box_in(&mut session, SIZE);
    session.pump();
    for name in [None, Some("bracket")] {
        let op = place_op(session.committed_doc(), body.into(), name);
        landed(&mut session, op);
    }
    let bracket = session
        .committed_doc()
        .var_named("bracket")
        .expect("the source");

    let lateral = |session: &DocSession, node: RecipeNodeId| {
        pncad::select::all_edges(session.evaluation().expect("landed"), node)
            .into_iter()
            .filter(|name| matches!(name.path.as_slice(), [RoleSeg::LateralEdge(_)]))
            .collect::<Vec<_>>()
    };
    let edges = lateral(&session, body);
    let f1 = landed(
        &mut session,
        SessionOp::AddFillet {
            target: body,
            radius: common::len(0.001),
            selection: vec![edges[0].clone()],
        },
    );
    let offer = viewer::world::repoint_offer(session.committed_doc(), f1, tol)
        .expect("F1, drawn, offers bracket");
    assert_eq!(
        offer.rows.iter().map(|row| row.source).collect::<Vec<_>>(),
        [bracket],
        "F1 offers bracket"
    );

    // Not accepted: a second feature on F1.
    let carried = pncad::select::all_edges(session.evaluation().expect("landed"), f1)
        .into_iter()
        .find(|name| {
            matches!(name.path.as_slice(),
                [RoleSeg::From { of, .. }] if *of.name() == edges[1])
        })
        .expect("F1 carries a second lateral edge of B");
    let f2 = landed(
        &mut session,
        SessionOp::AddChamfer {
            target: f1,
            distance: common::len(0.0005),
            selection: vec![carried],
        },
    );
    for row in session.tree_rows() {
        assert_eq!(row.status, RowStatus::Ok, "{row:?}");
    }
    let doc = session.committed_doc();
    let picture = viewer::world::Picture::of(doc);
    let drawn = |node| picture.drawn.iter().any(|d| d.of_node(node));
    assert_eq!(
        [drawn(f1), drawn(f2)],
        [false, true],
        "F2 is drawn, F1 is not"
    );
    assert_eq!(
        doc.sources_of(doc.output(f1, 0).expect("F1's body")),
        [],
        "no source reads F1's body"
    );
    let f2_offer = viewer::world::repoint_offer(doc, f2, tol);
    let f1_offer = viewer::world::repoint_offer(doc, f1, tol);
    eprintln!("probe: F2 offer = {f2_offer:?}");
    eprintln!("probe: F1 offer = {f1_offer:?}");
    assert_eq!(f2_offer, None, "F2 offers nothing: its take is F1's body");
    assert_eq!(f1_offer, None, "F1, no longer drawn, offers nothing");
    assert_eq!(
        doc.source(bracket).map(|read| read.read),
        doc.output(body, 0),
        "bracket still reads B"
    );
}
