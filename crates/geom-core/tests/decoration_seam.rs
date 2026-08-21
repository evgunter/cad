//! **The split between "carries a bracket" and "may enter certified
//! code", executed at the seam where it matters.**
//!
//! `Interval` records a domain violation in its IEEE 1788 decoration, not
//! in its endpoints: the backend *clamps* a partially-out-of-domain
//! operand rather than failing, so `sqrt([−1, 4])` is `[0, 2]` with
//! decoration `Trv`. Two facts about that value have to coexist, and the
//! whole point of the split is that they are different questions:
//!
//! 1. **It is a sound bracket.** `[0, 2]` really does contain every value
//!    the expression was defined on. Containment properties written
//!    against [`Bounds`] hold for it and must keep holding —
//!    `geom`'s F1 rows assert exactly this, on enclosures that are
//!    `Trv` and even unbounded.
//! 2. **It may not certify anything.** The bracket describes a
//!    *different* expression from the one that was asked for, so nothing
//!    that produces a certificate may consume it.
//!
//! `Bounds` answers (1) and never refuses. [`CertifiedEnclosure`] answers
//! (2) and refuses below `Def`. These rows pin both halves, and pin that
//! the three C9-ring crossings follow the second door rather than the
//! first — which is the actual defect S41 found: they read the bracket,
//! so a `Trv` enclosure crossed into `RingInterval` as a healthy bound.
//!
//! The sweeps are **paired**: each walks an operand across the domain
//! boundary and requires refusal *iff* the decoration degraded, with
//! non-vacuity assertions on both halves. A laundering implementation
//! certifies the whole sweep and fails; an implementation that poisons
//! indiscriminately refuses the whole sweep and fails too.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Decide, Indeterminate, MarginDiag};
use geom_core::spline::{KnotVector, hull};
use geom_core::{Bounds, CertifiedEnclosure, Interval, Real, RingInterval};

/// A domain violation with finite endpoints: `sqrt([−1, 4])` clamps to
/// `[0, 2]` and records the violation only in the decoration.
fn trv() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt()
}

/// The same shape of value with nothing wrong with it — `sqrt([1, 4])` is
/// `[1, 2]` at `Com`. The discriminating partner of [`trv`].
fn healthy() -> Interval {
    Interval::from_bounds(1.0, 4.0).sqrt()
}

fn nai() -> Interval {
    Interval::from_f64(f64::NAN)
}

/// Fully out of domain: `sqrt([−4, −1])` is empty.
fn empty() -> Interval {
    Interval::from_bounds(-4.0, -1.0).sqrt()
}

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// The fixture is the case the split is about, and not by accident:
/// `Trv`, nonempty, finite endpoints, refused by the decision door. If
/// the backend ever stopped clamping — went empty instead — the rest of
/// this suite would be pinning nothing, and this row says so first.
#[test]
fn the_trv_fixture_is_nonempty_with_finite_endpoints() {
    let x = trv();
    assert!(!x.is_certified(), "fixture is not domain-violated");
    let (lo, hi) = (Bounds::lo(x), Bounds::hi(x));
    assert!(
        lo.is_finite() && hi.is_finite() && lo <= hi,
        "fixture is not a nonempty finite enclosure: [{lo}, {hi}]"
    );
    assert_eq!((lo, hi), (0.0, 2.0), "fixture is not the clamped sqrt");
    assert!(matches!(
        x.sign_within(band()),
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        })
    ));
    // And the healthy partner is genuinely certified, so the pairing
    // below has a live positive half.
    assert!(healthy().is_certified(), "control is not certified");
}

/// **Half one of the split: `Bounds` never refuses.**
///
/// A `Trv` enclosure is still a sound bracket, so the bracket door
/// reports its endpoints unchanged. This is not a tolerated weakness —
/// it is the property `geom`'s F1 containment rows depend on, and
/// making `Bounds` consult the decoration breaks them. This row is the
/// local guard on that: it goes red the moment the bracket door starts
/// answering the certification question.
#[test]
fn the_bracket_door_reports_stored_endpoints_and_never_refuses() {
    assert_eq!((Bounds::lo(trv()), Bounds::hi(trv())), (0.0, 2.0));
    assert_eq!((Bounds::lo(healthy()), Bounds::hi(healthy())), (1.0, 2.0));
    // Containment through `Bounds` holds for the violated value, which is
    // the shape `geom` asserts.
    let (lo, hi) = (Bounds::lo(trv()), Bounds::hi(trv()));
    for v in [0.0, 0.5, 1.0, 2.0] {
        assert!(lo <= v && v <= hi, "Trv bracket must still contain {v}");
    }
    // NaI and empty are stored with NaN bounds, so they surface poison
    // through the bracket on their own — no decoration read involved.
    for (tag, x) in [("NaI", nai()), ("Empty", empty())] {
        assert!(
            Bounds::lo(x).is_nan() && Bounds::hi(x).is_nan(),
            "{tag}: expected NaN bracket from storage"
        );
    }
}

/// **Half two: the certified door refuses below `Def`.**
#[test]
fn the_certified_door_refuses_a_violated_decoration() {
    for (tag, x) in [("Trv", trv()), ("NaI", nai()), ("Empty", empty())] {
        assert!(
            x.certified_bracket().is_none(),
            "{tag}: certified door admitted a domain violation"
        );
    }
    assert_eq!(
        healthy().certified_bracket(),
        Some((1.0, 2.0)),
        "a certified enclosure must hand over its own endpoints unchanged"
    );
    // The other two lanes read their poison off the value itself rather
    // than off a decoration, and refuse on it. `certified_door.rs` sweeps
    // both; these two pairs are here so this row's `Def` threshold is not
    // mistaken for the only way the door can refuse.
    assert_eq!(2.5_f64.certified_bracket(), Some((2.5, 2.5)));
    assert!(f64::NAN.certified_bracket().is_none());
    assert_eq!(
        RingInterval::from_bounds(-1.0, 1.0).certified_bracket(),
        Some((-1.0, 1.0))
    );
    assert!(RingInterval::poison().certified_bracket().is_none());
}

/// The C9 hull bound over a two-coefficient degree-1 spline whose first
/// coefficient is `c` — the shortest path from an evaluation scalar to
/// `RingInterval` through `hull::bracket`.
fn hull_bound(c: Interval) -> RingInterval {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    hull::domain_hull(&kv, &[c, Interval::from_f64(1.0)])
}

/// The seam, swept across the domain boundary rather than probed at one
/// point: `sqrt([a, 4])` degrades exactly when `a < 0` forces a clamp,
/// and the hull bound built from it must refuse on exactly that side.
#[test]
fn the_hull_bound_refuses_exactly_where_the_decoration_degrades() {
    let (mut certified, mut refused) = (0, 0);
    for a in [-4.0, -1.0, -0.25, -1e-300, 0.0, 1e-300, 0.25, 1.0] {
        let c = Interval::from_bounds(a, 4.0).sqrt();
        let bound = hull_bound(c);
        assert_eq!(
            bound.is_poison(),
            !c.is_certified(),
            "sqrt([{a}, 4]) is {} but its hull bound is {bound:?}",
            if c.is_certified() {
                "certified"
            } else {
                "violated"
            }
        );
        if c.is_certified() {
            certified += 1;
        } else {
            refused += 1;
        }
    }
    assert!(certified > 0, "sweep never certified: the row is vacuous");
    assert!(refused > 0, "sweep never refused: the row is vacuous");
}

/// **The crossing must follow the certified door, not the bracket door.**
///
/// This is the row that goes red if a crossing is ever re-bounded to
/// plain [`Bounds`]. The fixture is chosen so the two doors give visibly
/// different answers — bracket says `[0, 2]`, certified door says
/// "refuse" — so the crossing's output identifies which one it consulted.
/// A `Bounds`-bounded crossing reproduces the bracket answer exactly, and
/// the assertion names that in its failure message rather than just
/// reporting a boolean.
#[test]
fn the_crossing_follows_the_certified_door_not_the_bracket() {
    let x = trv();
    let bracket_answer = (Bounds::lo(x), Bounds::hi(x));
    assert_eq!(bracket_answer, (0.0, 2.0), "fixture drifted");
    assert!(x.certified_bracket().is_none(), "fixture drifted");

    let bound = hull_bound(x);
    assert!(
        bound.is_poison(),
        "the hull crossing reproduced the BRACKET answer {bracket_answer:?} \
         as {bound:?} — it is reading `Bounds`, not `CertifiedEnclosure`. \
         The crossing's bound must require the certified door."
    );
}

/// The `Bounds`-extract → `Interval::from_bounds`-rebuild round trip the
/// constructor's docs warn about is still a laundering door — and that is
/// correct under the split, because `from_bounds` is the *driver's*
/// constructor and the extract it consumes is the bracket door, which by
/// design does not refuse. What closes the hole is that certification
/// crossings no longer go through the bracket door at all. This row pins
/// the warning as still true, so nobody reads the split as having fixed
/// something it deliberately did not.
#[test]
fn the_from_bounds_round_trip_still_launders_and_is_still_only_for_the_driver() {
    let rebuilt = Interval::from_bounds(Bounds::lo(trv()), Bounds::hi(trv()));
    assert!(
        rebuilt.is_certified(),
        "the round trip is expected to scrub the decoration; if it no \
         longer does, `from_bounds`' laundering warning needs rewriting"
    );
    // NaI and empty still cannot survive it: their brackets are NaN.
    for (tag, x) in [("NaI", nai()), ("Empty", empty())] {
        let r = Interval::from_bounds(Bounds::lo(x), Bounds::hi(x));
        assert!(!r.is_certified(), "{tag}: rebuilt as certified");
    }
}
