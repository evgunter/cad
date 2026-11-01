//! **A boss drawn to the edge of a block's top, through the op
//! vocabulary, unions.**
//!
//! A `40 × 20 × 10` mm block extruded from a centred rectangle; the
//! path through `(10, −5)`, `(20, −5)`, `(20, 5)`, `(10, 5)` mm,
//! extruded 4 mm and placed on the block's top, so the boss's `+x` wall
//! lies in the block's. The margins
//! decide the flush walls and the resting caps one carrier each, so the
//! union builds at `8000 + 400` mm³.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{desc, insert, len, slid};
use editor_core::{BooleanValue, Node, ProfileDoc, ValuePayload};
use geom_core::Tol;

#[test]
fn the_flush_boss_unions() {
    const MM: f64 = 1e-3;
    let doc = ProfileDoc::empty_derived("boss_flush_offer", Tol::witness());
    let (doc, blk) = block(
        doc,
        (-20.0 * MM, 20.0 * MM),
        (-10.0 * MM, 10.0 * MM),
        10.0 * MM,
    );
    let (doc, path) = insert(
        doc,
        Node::Profile(desc(vec![vec![
            (10.0 * MM, -5.0 * MM),
            (20.0 * MM, -5.0 * MM),
            (20.0 * MM, 5.0 * MM),
            (10.0 * MM, 5.0 * MM),
        ]])),
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: path.into(),
            distance: len(4.0 * MM),
        },
    );
    // The boss stands on the block's top, flush with its x = 20 wall.
    let (doc, boss) = slid(doc, blk, boss, [0.0, 0.0, 10.0 * MM]);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![blk.into(), boss.into()]),
        },
    );
    let ev = run(&doc);
    if let Some(refusal) = failure(&ev, union) {
        panic!("the union refuses {refusal:?}");
    }
    let volume = match &ev.value(union).expect("the union evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            topo::test_support::pinned_volume(
                "the union measures",
                &topo::test_support::finished(
                    "the measured body",
                    topo::Body::clone(body.body()),
                    Tol::witness(),
                ),
                Tol::witness(),
            )
            .volume
        }
        other => panic!("the union is a body, got {other:?}"),
    };
    let want = (8000.0 + 400.0) * MM * MM * MM;
    assert!(
        (volume - want).abs() <= 1e-12 * want,
        "the union's volume {volume} vs {want}"
    );
}
