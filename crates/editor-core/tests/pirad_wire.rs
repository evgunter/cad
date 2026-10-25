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
//! `pi` shows the rebuild's own refusal, carrying the symbol. (The
//! format carries no schema version — the persist module docs say
//! why — so there is no version pin here and no older golden to
//! refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::expr::DimensionError;
use editor_core::{Dimension, Formula, Node, PersistError, ProfileDoc, SlotId, load, save};
use fixture::{insert, on_frame};
use geom_core::Tol;

/// A document whose recipe carries an angle literal AUTHORED in
/// half-turns — a revolve's sweep angle, which is where a stored display
/// unit reaches a file.
fn half_turn_doc() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("pirad-wire", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let angle = Formula::literal_with_unit(
        0.5 * core::f64::consts::PI,
        Dimension::Angle,
        quantity::PI.def(),
    )
    .expect("a half-turn multiple is an angle");
    let (doc, _revolve) = insert(
        doc,
        crate::fixture::revolve_about(profile, (0.0, 0.0), crate::fixture::along_y(), angle),
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
        text.contains(r#""display_unit": "pi rad""#),
        "the half-turn symbol is on the wire: {text}"
    );
    assert!(
        !text.contains(r#""display_unit": "pi""#),
        "the retired spelling is gone: {text}"
    );
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    // The profile, then the revolve.
    let unit = match back.node(back.ids()[1]) {
        Some(Node::Revolve { .. }) => {
            back.slot_value(back.ids()[1], SlotId::RevolveAngle)
                .expect("a revolve reads its written angle")
                .1
        }
        other => panic!("expected the revolve, got {other:?}"),
    };
    assert_eq!(unit.symbol(), "pi rad");
    assert_eq!(unit, quantity::PI.def(), "the same table row, not a copy");
    let again = save(&back, &[], Tol::witness()).expect("the reloaded document saves");
    assert_eq!(text, again, "save . load is not a fixpoint");
}

/// **The spelling's own refusal**: a body carrying the retired symbol
/// — what a document written before the re-spell looks like — refuses,
/// and the refusal carries the symbol it could not read.
///
/// It refuses because the unit table is CLOSED over one spelling: the
/// rebuild resolves the symbol through it and announces the miss. The
/// miss is a `DimensionError`, so it crosses the load door whole and
/// the symbol is a FIELD — the assertion reads the value rather than a
/// message that happens to quote it.
#[test]
fn the_retired_spelling_refuses_and_carries_the_symbol() {
    let text = save(&half_turn_doc(), &[], Tol::witness()).expect("the document saves");
    let retired = text.replace(r#""display_unit": "pi rad""#, r#""display_unit": "pi""#);
    assert_ne!(retired, text, "the substitution must actually land");
    match load(&retired, Tol::witness()) {
        Err(PersistError::Dimension { error, .. }) => assert_eq!(
            error,
            DimensionError::UnknownDisplayUnit {
                symbol: "pi".to_owned()
            },
            "the refusal carries the symbol it could not read"
        ),
        other => panic!("a retired unit symbol must refuse typed, got {other:?}"),
    }
}
