//! **How far a field can move before something breaks** — the search
//! against synthetic oracles, and the session's probe against a real
//! document.
//!
//! The two halves are tested separately on purpose. The SEARCH is where
//! the mistakes live — brackets, strides, termination, the integral
//! case — and it is a pure function of a validity oracle, so it is
//! exercised against arithmetic predicates whose answers are known
//! exactly. The SESSION half is one claim: that the oracle it builds is
//! "the failing set did not grow", measured against the value the field
//! has now.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{Dimension, Doc, Expr, Node, ProfileProgram, SlotId};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use viewer::bounds::{Bound, Bounds, BoundsProbe, Verdict, probe};
use viewer::props;
use viewer::session::{BoundsTarget, DocSession, Refusal, SessionOp};

/// Run the search against `valid`, from `origin`, stepping by `seed`.
fn search(origin: f64, seed: f64, valid: impl FnMut(f64) -> bool) -> Bounds {
    probe(BoundsProbe::new(origin, seed, false), valid)
}

/// **A two-sided window is bracketed on both sides**, tightly enough to
/// be useful and never claiming more than was sampled.
#[test]
fn a_two_sided_window_is_bracketed_from_both_directions() {
    // Valid exactly on (2, 9); the origin sits inside it.
    let result = search(5.0, 1.0, |v| v > 2.0 && v < 9.0);
    let Bound::Edge {
        valid: low,
        invalid: below,
    } = result.low
    else {
        panic!("the lower edge should have been found: {:?}", result.low);
    };
    let Bound::Edge {
        valid: high,
        invalid: above,
    } = result.high
    else {
        panic!("the upper edge should have been found: {:?}", result.high);
    };
    // Each bracket straddles the true edge, and the reported side is
    // the one actually found valid.
    assert!(below <= 2.0 && low > 2.0, "low bracket {below}..{low}");
    assert!(above >= 9.0 && high < 9.0, "high bracket {high}..{above}");
    // Converged to well under a seed step — the bisection did its work.
    assert!(low - 2.0 < 0.05, "low edge {low} not tight");
    assert!(9.0 - high < 0.05, "high edge {high} not tight");
    assert!(
        result.samples <= BoundsProbe::MAX_SAMPLES,
        "{} samples",
        result.samples
    );
}

/// **A field with no failure near it reports how far it looked**, not
/// "unbounded" — the claim the search can support.
#[test]
fn a_field_with_no_nearby_failure_reports_its_reach() {
    let result = search(0.0, 1.0, |_| true);
    let (Bound::Open { probed: low }, Bound::Open { probed: high }) = (result.low, result.high)
    else {
        panic!("nothing should have been bracketed: {result:?}");
    };
    assert!(low < 0.0 && high > 0.0, "it looked both ways");
    // The reach is geometric, so it gets far in few samples.
    assert!(high >= 1024.0, "reached only {high}");
    let words = result.wording(None);
    assert!(words.contains("as far as"), "{words}");
    assert!(!words.contains("unbounded"), "{words}");
}

/// **A one-sided limit** — the shape most real fields have (a thickness
/// that cannot go to zero, with no ceiling anywhere near).
#[test]
fn a_one_sided_limit_brackets_one_side_and_reaches_on_the_other() {
    let result = search(10.0, 1.0, |v| v > 0.0);
    let Bound::Edge { valid, .. } = result.low else {
        panic!("the floor should have been found: {:?}", result.low);
    };
    assert!(valid > 0.0 && valid < 0.05, "floor found at {valid}");
    assert!(matches!(result.high, Bound::Open { .. }));
    let words = result.wording(None);
    assert!(words.contains("valid from"), "{words}");
}

/// **The value the field has now is valid by construction**, so a
/// direction with no room at all still answers — with a bracket that
/// starts at the origin rather than with a panic or an empty range.
#[test]
fn a_direction_with_no_room_brackets_at_the_origin() {
    // Valid only at exactly the origin and above.
    let result = search(0.0, 1.0, |v| v >= 0.0);
    let Bound::Edge { valid, invalid } = result.low else {
        panic!("the floor is at the origin: {:?}", result.low);
    };
    assert!(valid <= 0.0 && valid > -0.01, "valid {valid}");
    assert!(invalid < 0.0, "invalid {invalid}");
    assert_eq!(result.origin, 0.0);
}

/// **A count's answer is an integer**, exactly — the bracket closes on
/// whole numbers rather than leaving a fractional residue a reader
/// would have to interpret.
#[test]
fn an_integral_field_closes_on_whole_numbers() {
    // A pattern count that is legal from 2 through 7.
    let result = probe(BoundsProbe::new(4.0, 1.0, true), |v| {
        (2.0..=7.0).contains(&v)
    });
    let Bound::Edge {
        valid: low,
        invalid: below,
    } = result.low
    else {
        panic!("{:?}", result.low);
    };
    let Bound::Edge {
        valid: high,
        invalid: above,
    } = result.high
    else {
        panic!("{:?}", result.high);
    };
    assert_eq!((low, below), (2.0, 1.0), "exactly the legal floor");
    assert_eq!((high, above), (7.0, 8.0), "exactly the legal ceiling");
}

/// The search is BOUNDED: whatever the oracle does, it stops, and it
/// stops within the cost the module advertises. Asserted against an
/// oracle that changes its mind on every call, which is the worst case
/// for a bisection that assumed monotonicity.
#[test]
fn the_search_terminates_within_its_advertised_cost() {
    let mut flip = false;
    let result = search(0.0, 1.0, |_| {
        flip = !flip;
        flip
    });
    assert!(
        result.samples <= BoundsProbe::MAX_SAMPLES,
        "{} samples exceeds the {} cap",
        result.samples,
        BoundsProbe::MAX_SAMPLES
    );
}

/// A verdict is "no worse than", not "the same": a value that FIXES an
/// existing failure is not a boundary.
#[test]
fn a_value_that_fixes_a_failure_is_not_a_boundary() {
    let a = pncad::document::RecipeNodeId(1);
    let b = pncad::document::RecipeNodeId(2);
    let baseline = Verdict::from_nodes([a]);
    assert!(Verdict::from_nodes([a]).no_worse_than(&baseline));
    assert!(
        Verdict::default().no_worse_than(&baseline),
        "fixing is fine"
    );
    assert!(
        !Verdict::from_nodes([a, b]).no_worse_than(&baseline),
        "a new failure is not"
    );
    assert!(
        !Verdict::from_nodes([b]).no_worse_than(&baseline),
        "a DIFFERENT failure is a new one"
    );
}

/// **The session's probe against a real document**: an extrude whose
/// distance may not be zero.
///
/// The kernel refuses a zero-height extrude, so the range has a floor
/// just above zero and no ceiling anywhere near — and the probe finds
/// exactly that without the test naming a number the kernel decides.
#[test]
fn the_session_probes_a_real_slots_range() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("valid-range", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let target = BoundsTarget::Slot {
        node: extrude,
        slot: SlotId::Distance,
    };
    let outcome = session.perform(SessionOp::ProbeBounds {
        target: target.clone(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a probe changes no document");
    assert!(outcome.previewed.is_empty());

    let (probed, result) = session.bounds().expect("a probe landed").clone();
    assert_eq!(probed, target);
    assert_eq!(result.origin, 0.008);
    let Bound::Edge { valid, invalid } = result.low else {
        panic!("a thin extrude fails, so there is a floor below 8 mm: {result:?}");
    };
    // **The shape, not the location.** WHERE the floor sits is the
    // kernel's business and it moves with ε — a sub-tolerance height is
    // refused long before zero, so an assertion that the floor is at
    // zero passes at the witness ε and fails at a coarser one. (It did:
    // this row named `invalid <= 0.0` and went red on the interval lane
    // at ε = 1e-6, which is the lane doing its job.) What the probe
    // PROMISES is ε-independent and is what is asserted: a real bracket,
    // at or below where the field is now, narrowed to within one seed
    // step.
    assert!(invalid < valid, "a bracket straddles: {invalid}..{valid}");
    assert!(valid <= 0.008, "the floor is at or below the current value");
    assert!(
        valid - invalid <= 0.001,
        "the bracket should close to within one seed (1 mm): {invalid}..{valid}"
    );
    // And the reported numbers are checked against the document rather
    // than trusted: setting the field to the valid end introduces no
    // failure the baseline lacks, and setting it to the invalid end
    // does. That is the claim a bisection bug would break, and it is
    // the same statement at any ε.
    let baseline = Verdict::of(session.evaluation().expect("an evaluation landed"));
    for (value, want_ok, what) in [(valid, true, "valid"), (invalid, false, "invalid")] {
        session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: props::SlotValue::Continuous(value),
        });
        session.pump();
        let here = Verdict::of(session.evaluation().expect("an evaluation landed"));
        assert_eq!(
            here.no_worse_than(&baseline),
            want_ok,
            "the probe reported {value} as the {what} end"
        );
        session.perform(SessionOp::Undo);
        session.pump();
    }

    // The reading comes back in the unit the field is written in.
    let words = result.wording(props::rendering_unit(Dimension::Length, Some(MM.def())));
    assert!(words.contains("mm"), "{words}");

    // A document change discards it: a range is a statement about one
    // document, and a stale one beside a fresh number is exactly the
    // confident wrong answer to avoid.
    //
    // Re-probed first, so this is not vacuous: the verification loop
    // above edited the document and therefore already discarded the
    // original reading.
    session.perform(SessionOp::ProbeBounds { target });
    assert!(session.bounds().is_some(), "a fresh probe landed");
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: props::SlotValue::Continuous(0.02),
    });
    assert!(session.bounds().is_none(), "the probe was discarded");
}

/// The probe refuses typed for a field that is not there, and commits
/// nothing when it does.
#[test]
fn probing_a_field_that_is_not_there_refuses_typed() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let outcome = session.perform(SessionOp::ProbeBounds {
        target: BoundsTarget::Slot {
            node: extrude,
            slot: SlotId::Radius,
        },
    });
    assert!(matches!(outcome.refusal, Some(Refusal::NoSuchSlot { .. })));
    assert!(session.bounds().is_none());

    let outcome = session.perform(SessionOp::ProbeBounds {
        target: BoundsTarget::Param {
            name: pncad::document::ParamName::new("nope"),
        },
    });
    assert!(matches!(outcome.refusal, Some(Refusal::NoSuchParam(_))));
}
