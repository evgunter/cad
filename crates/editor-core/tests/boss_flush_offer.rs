//! **A boss drawn to the edge of a block's top, through the op
//! vocabulary, unions once the boolean's offers are accepted.**
//!
//! A `40 × 20 × 10` mm block extruded from a centred rectangle; a frame
//! read off its top cap (`Datum::FaceFrame`, spin 0); on it the path
//! through `(10, −5)`, `(20, −5)`, `(20, 5)`, `(10, 5)` mm, extruded
//! 4 mm, so the boss's `+x` wall lies in the block's. The union is
//! refused once per undeclared coincidence, each refusal carrying the
//! finding to declare; accepting them in turn (the flush walls, then the
//! resting caps) ends in the union at `8000 + 400` mm³.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, desc, fname, insert, len};
use editor_core::{
    BooleanCoincidence, BooleanOp, BooleanValue, CapEnd, DeclaredPair, ExtrudeSide, Node,
    NodeErrorKind, ProfileDoc, RoleSeg, ValuePayload,
};
use geom_core::Tol;

#[test]
fn accepting_each_offer_in_turn_builds_the_flush_boss_union() {
    const MM: f64 = 1e-3;
    let doc = ProfileDoc::empty_derived("boss_flush_offer", Tol::witness());
    let (doc, blk) = block(
        doc,
        (-20.0 * MM, 20.0 * MM),
        (-10.0 * MM, 10.0 * MM),
        0.0,
        10.0 * MM,
    );
    let (doc, top) = insert(
        doc,
        Node::Datum(editor_core::Datum::FaceFrame {
            at: blk.into(),
            face: fname(blk, RoleSeg::Cap(CapEnd::End)),
            spin: ang(0.0),
        }),
    );
    let (doc, path) = insert(
        doc,
        Node::Profile(desc(
            top,
            vec![vec![
                (10.0 * MM, -5.0 * MM),
                (20.0 * MM, -5.0 * MM),
                (20.0 * MM, 5.0 * MM),
                (10.0 * MM, 5.0 * MM),
            ]],
        )),
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: path.into(),
            distance: len(4.0 * MM),
            side: ExtrudeSide::Along,
        },
    );
    let mut accepted: Vec<DeclaredPair> = Vec::new();
    let mut classes = Vec::new();
    let volume = loop {
        let (doc, union) = insert(
            doc.clone(),
            Node::Boolean {
                op: BooleanOp::Union,
                a: blk.into(),
                b: boss.into(),
                declare: accepted.clone(),
            },
        );
        let ev = run(&doc);
        match failure(&ev, union) {
            Some(NodeErrorKind::UndeclaredCoincidence { finding, .. }) if accepted.len() < 4 => {
                classes.push(finding.class);
                accepted.push((finding.pair.clone(), finding.class));
            }
            Some(other) => panic!("after {classes:?}: the union refuses {other:?}"),
            None => match &ev.value(union).expect("the union evaluated").payload {
                ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                    break topo::mass_properties(body, Tol::witness())
                        .expect("the union measures")
                        .volume;
                }
                other => panic!("the union is a body, got {other:?}"),
            },
        }
    };
    assert_eq!(
        classes,
        [BooleanCoincidence::Continuation, BooleanCoincidence::REST],
        "the flush walls are offered first, then the resting caps"
    );
    let want = (8000.0 + 400.0) * MM * MM * MM;
    assert!(
        (volume - want).abs() <= 1e-12 * want,
        "the union's volume {volume} vs {want}"
    );
}
