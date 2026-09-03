//! LIB-PLACEDUNION — the group boolean's vocabulary on the wire.
//!
//! GROUP-BOOLEAN-DESIGN (ratified A′) grew the node vocabulary a
//! `PlacedUnion` and the placement-rule vocabulary an `Explicit` list
//! of absolute frames; the two ship together because neither is
//! expressible without the other at the site that motivated both. A
//! document carrying the group node under each rule must save and
//! reload as itself, frames bit-exact. (The format carries no schema
//! version — the persist module docs say why — so there is no version
//! pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{Expr, Frame, Node, PatternKind, load, save};
use fixture::{desc, len, scl};
use geom_core::Tol;

/// A document carrying a group node under EACH rule saves and reloads
/// as itself — the vocabulary crosses the wire.
#[test]
fn both_rules_round_trip() {
    let mut r = corpus::Recorder::new();
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    )));
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let stepped = r.insert(
        Node::placed_union(
            solid,
            Expr::count(3),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(4.0),
            },
        )
        .expect("a stepped rule takes a count"),
    );
    let listed = r.insert(Node::placed_union_at(
        solid,
        vec![
            Frame::translation([0.0, 20.0, 0.0]),
            Frame::rotate_then_translate(
                [0.0, 0.0, 1.0],
                std::f64::consts::FRAC_PI_2,
                [0.0, 40.0, 0.0],
            ),
        ],
    ));
    let text = save(&r.doc, &[], Tol::witness()).expect("the document saves");
    let back = load(&text, Tol::witness()).expect("the saved text loads");
    assert_eq!(back.doc.node(stepped), r.doc.node(stepped));
    assert_eq!(back.doc.node(listed), r.doc.node(listed));
    // Bit-exact frames: a placement is data, and `-0.0` is a different
    // placement from `0.0` to the content key, so it must be one on
    // the wire too.
    assert_eq!(
        save(&back.doc, &[], Tol::witness()).expect("the reloaded document saves"),
        text,
        "save ∘ load is a fixpoint"
    );
}
