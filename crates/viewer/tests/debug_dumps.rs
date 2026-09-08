//! **A summarised field renders as a summary.**
//!
//! The four exhaustive `Debug` walks (`crates/viewer/README.md`, "The
//! dump is held to the same declaration") draw a three-way
//! distinction — printed in full, summarised, not carried — and
//! `finish`/`finish_non_exhaustive` can only say whether every FIELD
//! is shown. The half the marker cannot carry is carried at the field:
//! a summarised field renders as something no value of that field's
//! own type could render as, so a reader who knows `std` and not this
//! crate cannot take the summary for the whole.
//!
//! These rows are the only readers of those dumps in the tree, and
//! they read them for that property alone: the presence elisions and
//! the counts, never the fields printed in full beside them.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::geom_core::Tol;
use viewer::pickcache::{CacheStep, IndexLanding, PickCache};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, Landing, SessionOp};

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

/// **Every presence carried by a live session renders as an elision.**
/// One session holds all three: `gesture` and `scratch` while a drag
/// previews, `body` from the landing under it.
#[test]
fn a_summarised_presence_renders_as_an_elision_naming_what_is_there() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);
    // The plate's distance is driven by the `thickness` parameter, so
    // the gesture that moves it is the parameter's.
    let begun = session.perform(SessionOp::BeginParamGesture {
        name: common::thickness_param(),
    });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);
    let previewed = session.perform(SessionOp::PreviewGesture { value: 0.02 });
    assert!(previewed.refusal.is_none(), "{:?}", previewed.refusal);

    let dump = format!("{session:?}");
    for elision in [
        "gesture: Some(<Gesture>)",
        "scratch: Some(<Doc>)",
        "body: Some(<Body>)",
    ] {
        assert!(dump.contains(elision), "{elision} not in {dump}");
    }
    // A count is already a summary — nothing could read `states: 1` as
    // the history — and so is the checks pair.
    assert!(dump.contains("states: 1"), "{dump}");
    assert!(dump.contains("finding(s),"), "{dump}");
    // Nothing summarised is spelled as the `bool` its presence is.
    for bare in ["gesture: true", "scratch: true", "body: true"] {
        assert!(!dump.contains(bare), "{bare} in {dump}");
    }
}

/// **An absent one renders as `None`**, which is the whole field: a
/// missing document is missing, and there is nothing elided about it.
#[test]
fn an_absent_summarised_field_renders_as_none() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    // No `pump`, so nothing has landed and no gesture is in flight.
    let session = DocSession::inline(doc, tol);

    let dump = format!("{session:?}");
    for absent in ["gesture: None", "scratch: None", "resolver: None"] {
        assert!(dump.contains(absent), "{absent} not in {dump}");
    }
    for bare in ["gesture: false", "scratch: false", "resolver: false"] {
        assert!(!dump.contains(bare), "{bare} in {dump}");
    }
}

/// **A held index renders as an elision around the generation it
/// describes**, not as the generation: `index` is an index, and a dump
/// that said `Some(Generation(1))` would name a field this cache does
/// not have.
#[test]
fn a_held_pick_index_renders_as_an_elision_around_its_generation() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    assert_eq!(session.pump(), vec![Landing::Landed]);
    let mut cache = PickCache::inline();

    assert!(format!("{cache:?}").contains("index: None"));
    assert_eq!(
        cache.sync(session.index_inputs(), delta()),
        CacheStep::Submitted
    );
    assert_eq!(cache.pump(), vec![IndexLanding::Built]);
    let generation = cache.index().expect("the plate indexes").generation();

    let dump = format!("{cache:?}");
    assert!(
        dump.contains(&format!("index: Some(<PickIndex for {generation:?}>)")),
        "{dump}"
    );
}
