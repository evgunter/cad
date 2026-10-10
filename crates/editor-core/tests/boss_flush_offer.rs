//! **A boss drawn to the edge of a block's top, through the op
//! vocabulary, unions declared or not.**
//!
//! A `40 × 20 × 10` mm block extruded from a centred rectangle; a frame
//! read off its top cap (`Datum::FaceFrame`, spin 0); on it the path
//! through `(10, −5)`, `(20, −5)`, `(20, 5)`, `(10, 5)` mm, extruded
//! 4 mm, so the boss's `+x` wall lies in the block's. The margins
//! decide the flush walls and the resting caps one carrier each, so the
//! undeclared union builds at `8000 + 400` mm³, and declaring every
//! finding the detector reports (the flush walls and the resting caps)
//! builds the same body bit for bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ang, built_bits, desc, findings_declared, fname, insert, len};
use editor_core::{
    BooleanCoincidence, BooleanValue, CapEnd, ExtrudeSide, Node, ProfileDoc, RoleSeg, ValuePayload,
};
use geom_core::Tol;

#[test]
fn the_flush_boss_unions_declared_or_not() {
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
    let union_of = |declare| {
        let (doc, union) = insert(
            doc.clone(),
            Node::Union {
                members: editor_core::Bodies::Spelled(vec![blk.into(), boss.into()]),
                declare,
            },
        );
        let ev = run(&doc);
        if let Some(refusal) = failure(&ev, union) {
            panic!("the union refuses {refusal:?}");
        }
        (ev, union)
    };
    let (undeclared, union) = union_of(Vec::new());
    let volume = match &undeclared
        .value(union)
        .expect("the union evaluated")
        .payload
    {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            topo::mass_properties(body, Tol::witness())
                .expect("the union measures")
                .volume
        }
        other => panic!("the union is a body, got {other:?}"),
    };
    let want = (8000.0 + 400.0) * MM * MM * MM;
    assert!(
        (volume - want).abs() <= 1e-12 * want,
        "the union's volume {volume} vs {want}"
    );
    let found = findings_declared(&undeclared, &doc, &[blk, boss]);
    let classes: Vec<BooleanCoincidence> = found.iter().map(|(_, c)| *c).collect();
    assert!(
        classes.contains(&BooleanCoincidence::Continuation)
            && classes.contains(&BooleanCoincidence::REST)
            && classes
                .iter()
                .all(|c| [BooleanCoincidence::Continuation, BooleanCoincidence::REST].contains(c)),
        "the detector finds the flush walls and the resting caps: {classes:?}"
    );
    let (declared, declared_union) = union_of(found);
    assert_eq!(
        built_bits(&declared, declared_union),
        built_bits(&undeclared, union),
        "the declared union is the undeclared one's body"
    );
}
