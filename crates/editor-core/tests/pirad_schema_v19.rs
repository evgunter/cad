//! **The half-turn display unit re-spelled `pi rad` on the wire** —
//! schema **v19**, and the gate that refuses everything older.
//!
//! Before v19 the angle row of `quantity::UNITS` whose factor is π
//! carried the surface symbol `pi`; it now carries `pi rad`, the
//! multiplier and the unit it multiplies, so a picker offering it
//! beside `deg` and `rad` reads as "multiples of π radians" rather
//! than as the number π.
//!
//! A display unit is FILE data — a literal persists the unit it
//! REMEMBERS as its symbol STRING, and the rebuild resolves that
//! string through the closed table — so the spelling is on the wire
//! and the break runs in BOTH directions: a v18 body says `"pi"`,
//! which a v19 table has no row for, and a v19 body says `"pi rad"`,
//! which a v18 table has no row for.
//!
//! **Which gate fires, in order.** A real v18 file never reaches the
//! symbol at all: the VERSION DOOR refuses it first (`SchemaTooOld`,
//! regenerate recourse), and a file from the future is refused there
//! too (`UnknownSchema`) — `blend5_r1_probes` pins that ordering and
//! this suite re-pins the too-old half on the v18 golden. The
//! spelling's own refusal covers only what the header cannot see: a
//! HYBRID, v19 in the header with the retired symbol in the body,
//! which `the_retired_spelling_dies_at_the_rebuild` builds and
//! executes. That one is typed — the closed table has no `pi` row and
//! says so — not a serde variant failure.
//!
//! The disposition is the family's: the older file refuses TYPED at
//! the version door with the regenerate recourse, and the migration
//! table stays empty (LQ7a).
//!
//! **The mapping the break does not write.** `"pi"` → `"pi rad"` is
//! total and meaning-preserving: the row's quantity and factor did not
//! move, so every v18 literal spelled `"pi"` denotes exactly the
//! half-turn the v19 spelling denotes. What stops the migration being
//! written is the standing rule, not the mapping. Both halves are
//! executed below: the round trip pins the new spelling, and a
//! hand-built v18 symbol shows the old one refusing.
//!
//! **Why 19.** Read by eye from main's constant
//! (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`), which read 18, because units have repeatedly had
//! a same-number claim merge CLEAN — both sides write the identical
//! line, so git never conflicts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Dimension, Expr, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, RecipeNodeId,
    SCHEMA_VERSION, load, save,
};
use fixture::{desc, insert, len, scl};
use geom_core::Tol;

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust.
const V18: &str = include_str!("golden/v18_golden.cad");
/// One further back, to show the gate has no notion of "nearly
/// current".
const V17: &str = include_str!("golden/v17_golden.cad");

#[test]
fn schema_version_is_current() {
    assert_eq!(SCHEMA_VERSION, 19);
}

#[test]
fn the_checked_in_older_goldens_are_really_older() {
    assert_eq!(V18.lines().next(), Some("schema: 18"));
    assert_eq!(V17.lines().next(), Some("schema: 17"));
}

/// The break, in the direction that matters: a v18 file refuses TYPED
/// at the VERSION DOOR — naming the version found, the version
/// supported, and the step that does not exist — rather than reaching
/// serde and dying on the symbol inside it.
#[test]
fn v18_refuses_too_old_at_the_version_door() {
    match load(V18, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 18);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 18,
                "the 18 -> 19 step is the one that does not exist"
            );
        }
        other => panic!("v18 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v18", V18), ("v17", V17)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

/// A document whose recipe carries an angle literal AUTHORED in
/// half-turns — a transform's rotation angle, which is where a stored
/// display unit reaches a file.
fn half_turn_doc() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("pirad-schema-v19", Tol::witness());
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
fn a_half_turn_literal_round_trips_at_v19() {
    let doc = half_turn_doc();
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..])
    );
    assert!(
        text.contains(r#""unit":"pi rad""#),
        "the half-turn symbol is on the wire: {text}"
    );
    assert!(
        !text.contains(r#""unit":"pi""#),
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

/// **The spelling's own refusal, on the one input the version door
/// cannot catch**: a HYBRID — this build's header over a body carrying
/// the retired symbol, which is what a hand-edited or half-migrated
/// file looks like. A real v18 file never gets this far.
///
/// It refuses because the unit table is CLOSED over one spelling: the
/// rebuild resolves the symbol through it and announces the miss.
#[test]
fn the_retired_spelling_dies_at_the_rebuild() {
    let text = save(&half_turn_doc(), &[], Tol::witness()).expect("the document saves");
    let v18ish = text.replace(r#""unit":"pi rad""#, r#""unit":"pi""#);
    assert_ne!(v18ish, text, "the substitution must actually land");
    match load(&v18ish, Tol::witness()) {
        Err(PersistError::Parse { message, .. }) => {
            assert!(
                message.contains("pi"),
                "the refusal names the symbol it could not read: {message}"
            );
        }
        other => panic!("a retired unit symbol must refuse, got {other:?}"),
    }
}
