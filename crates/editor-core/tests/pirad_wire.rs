//! **The half-turn display unit on the wire**: the angle row of
//! `quantity::UNITS` whose factor is π carries the surface symbol
//! `pi rad` — the multiplier and the unit it multiplies — so a picker
//! offering it beside `deg` and `rad` reads as "multiples of π
//! radians" rather than as the number π.
//!
//! A display unit is FILE data — a literal persists the unit it
//! REMEMBERS as its symbol STRING, and the rebuild resolves that
//! string through the closed table — so the spelling is on the wire:
//! the round trip pins it, and a body carrying the retired symbol
//! `pi` shows the rebuild's own refusal, naming the symbol. (The
//! format carries no schema version — the persist module docs say
//! why — so there is no version pin here and no older golden to
//! refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{Dimension, Expr, Node, PersistError, ProfileDoc, RecipeNodeId, load, save};
use fixture::{desc, insert, len, scl};
use geom_core::Tol;

/// A document whose recipe carries an angle literal AUTHORED in
/// half-turns — a transform's rotation angle, which is where a stored
/// display unit reaches a file.
fn half_turn_doc() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("pirad-wire", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let angle = Expr::literal_with_unit(
        0.5 * core::f64::consts::PI,
        Dimension::Angle,
        quantity::PI.def(),
    )
    .expect("a half-turn multiple is an angle");
    let (doc, _tr) = insert(
        doc,
        Node::Transform {
            input: block,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: angle,
        },
    );
    doc
}

/// **The new spelling is what the wire carries**, and it round-trips:
/// the value is canonical radians either way, and the SYMBOL is what
/// moved.
#[test]
fn a_half_turn_literal_round_trips() {
    let doc = half_turn_doc();
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert!(
        text.contains(r#""unit": "pi rad""#),
        "the half-turn symbol is on the wire: {text}"
    );
    assert!(
        !text.contains(r#""unit": "pi""#),
        "the retired spelling is gone: {text}"
    );
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    let unit = match back.node(RecipeNodeId(2)) {
        Some(Node::Transform { rotation_angle, .. }) => {
            rotation_angle.display_unit().expect("the unit survives")
        }
        other => panic!("expected the transform, got {other:?}"),
    };
    assert_eq!(unit.symbol(), "pi rad");
    assert_eq!(unit, quantity::PI.def(), "the same table row, not a copy");
    let again = save(&back, &[], Tol::witness()).expect("the reloaded document saves");
    assert_eq!(text, again, "save . load is not a fixpoint");
}

/// **The spelling's own refusal**: a body carrying the retired symbol
/// — what a document written before the re-spell looks like — is
/// unreadable by this build, and the refusal names the symbol.
///
/// It refuses because the unit table is CLOSED over one spelling: the
/// rebuild resolves the symbol through it and announces the miss.
#[test]
fn the_retired_spelling_is_unreadable_by_this_build() {
    let text = save(&half_turn_doc(), &[], Tol::witness()).expect("the document saves");
    let retired = text.replace(r#""unit": "pi rad""#, r#""unit": "pi""#);
    assert_ne!(retired, text, "the substitution must actually land");
    match load(&retired, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            // The deserializer's message is the ONLY place the name
            // exists (serde exposes no structured accessor), so reading
            // it back is the assertion, not message sniffing; the
            // fuller phrase keeps a two-letter word from matching by
            // accident.
            assert!(
                detail.contains("unknown display unit \"pi\""),
                "the refusal names the symbol it could not read: {detail}"
            );
        }
        other => panic!("a retired unit symbol must refuse unreadable, got {other:?}"),
    }
}
