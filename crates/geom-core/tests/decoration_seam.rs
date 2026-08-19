//! **The bracket accessors are not a laundering door around the
//! decoration channel.**
//!
//! `Interval` carries a domain violation in its IEEE 1788 decoration,
//! not in its endpoints: the backend *clamps* a partially-out-of-domain
//! operand rather than failing, so `sqrt([−1, 4])` is `[0, 2]` with
//! decoration `Trv` — finite, tight, and completely healthy-looking to
//! anything that reads only two `f64`s. `Decide::sign_within` refuses
//! it, but certification code does not decide: it crosses into the C9
//! ring (`RingInterval`, two states and no decorations) through
//! `Bounds`/`Enclosure`, and the ring can learn nothing the bracket does
//! not say.
//!
//! So the refusal has to live in the accessors, and these rows are what
//! make that behavioural. Each pairs the same computation on a **healthy**
//! enclosure with the `Trv` one and demands that the healthy operand
//! certify while the violated one refuses. The pairing is the point: an
//! implementation that poisons everything passes the refusal half and
//! fails the healthy half, and an implementation that launders passes the
//! healthy half and fails the refusal half. Neither direction is
//! satisfiable by weakening a bound.
//!
//! The `Trv` fixture is deliberately *nonempty with finite endpoints*.
//! NaI and the empty enclosure are stored with NaN bounds, so they
//! surface poison however the accessors are written; they are pinned here
//! too, as the control that says the fixture is the interesting case
//! rather than the only case.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::predicate::{Band, Decide, Indeterminate, MarginDiag};
use geom_core::spline::{KnotVector, hull};
use geom_core::{Bounds, Interval, Real, RingInterval};

/// A domain violation with finite endpoints: `sqrt([−1, 4])` clamps to
/// `[0, 2]` and records the violation only in the decoration.
fn trv() -> Interval {
    Interval::from_bounds(-1.0, 4.0).sqrt()
}

/// The same shape of value with nothing wrong with it — `sqrt([1, 4])`,
/// which is `[1, 2]` at `Com`. The discriminating partner of [`trv`].
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

/// Every enclosure whose decoration is below `Def`, by the name the
/// failure message should carry.
fn violated() -> [(&'static str, Interval); 3] {
    [("Trv", trv()), ("NaI", nai()), ("Empty", empty())]
}

/// The fixture is the case the finding is about, and not by accident:
/// `Trv`, nonempty, finite endpoints, and refused by the decision door.
/// If the backend ever stopped clamping — went empty instead — the rest
/// of this suite would be pinning nothing, and this row says so first.
#[test]
fn the_trv_fixture_is_nonempty_with_finite_endpoints() {
    let (lo_bits, hi_bits, dec) = trv().repr_bits();
    assert_eq!(dec, 1, "fixture decoration is not Trv");
    let (lo, hi) = (f64::from_bits(lo_bits), f64::from_bits(hi_bits));
    assert!(
        lo.is_finite() && hi.is_finite() && lo <= hi,
        "fixture is not a nonempty finite enclosure: [{lo}, {hi}]"
    );
    assert_eq!((lo, hi), (0.0, 2.0), "fixture is not the clamped sqrt");
    // The decision door already refuses it; the bracket must agree.
    let band = Band::new(1e-9, 1e-8).unwrap();
    assert!(matches!(
        trv().sign_within(band),
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            ..
        })
    ));
    // And the healthy partner is genuinely healthy — `Com`, not merely
    // "not Trv" — so the pairing below has a live positive half.
    assert_eq!(healthy().repr_bits().2, 4, "control is not Com");
}

/// `Bounds`/`Enclosure` refuse below `Def`: both accessors NaN, so
/// nothing downstream can read past the violation.
#[test]
fn bracket_accessors_refuse_a_violated_decoration() {
    for (tag, x) in violated() {
        assert!(
            Bounds::lo(x).is_nan() && Bounds::hi(x).is_nan(),
            "{tag}: bracket survived as [{}, {}]",
            Bounds::lo(x),
            Bounds::hi(x)
        );
    }
    let ok = healthy();
    assert_eq!(
        (Bounds::lo(ok), Bounds::hi(ok)),
        (1.0, 2.0),
        "a healthy enclosure must report its own endpoints unchanged"
    );
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
///
/// This is the row that cannot be satisfied in the wrong direction. A
/// laundering seam certifies the whole sweep; a seam that poisons
/// indiscriminately refuses the whole sweep; only one that reads the
/// decoration tracks the boundary. The final two assertions keep the
/// sweep from going vacuous if the backend's domain handling changes.
#[test]
fn the_hull_bound_refuses_exactly_where_the_decoration_degrades() {
    let mut certified = 0;
    let mut refused = 0;
    for a in [-4.0, -1.0, -0.25, -1e-300, 0.0, 1e-300, 0.25, 1.0] {
        let c = Interval::from_bounds(a, 4.0).sqrt();
        let degraded = c.repr_bits().2 < 2; // below `Def`
        let bound = hull_bound(c);
        assert_eq!(
            bound.is_poison(),
            degraded,
            "sqrt([{a}, 4]) has decoration {} but its hull bound is {bound:?}",
            c.repr_bits().2
        );
        if degraded {
            refused += 1;
        } else {
            certified += 1;
        }
    }
    assert!(certified > 0, "sweep never certified: the row is vacuous");
    assert!(refused > 0, "sweep never refused: the row is vacuous");
}

/// The `Bounds`-extract → `Interval::from_bounds`-rebuild round trip the
/// constructor's docs warn about cannot scrub a violation: the extract
/// refuses first, so the rebuild is NaI and stays refused.
#[test]
fn the_round_trip_through_from_bounds_cannot_scrub_a_violation() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    for (tag, x) in violated() {
        let rebuilt = Interval::from_bounds(Bounds::lo(x), Bounds::hi(x));
        assert!(
            matches!(
                rebuilt.sign_within(band),
                Err(Indeterminate {
                    margin: MarginDiag::Invalid,
                    ..
                })
            ),
            "{tag}: laundered by the round trip into {:?}",
            rebuilt.repr_bits()
        );
    }
}

/// Narrowing an enclosure by an independently known window must not
/// resurrect poison as the window. `f64::max`/`min` return the non-NaN
/// operand, so the naive spelling turns a poisoned bracket into `[−1, 1]`
/// — sound-looking, and exactly the wide-but-accepted bound that makes a
/// certification pass for no reason.
#[test]
fn clamping_a_poisoned_enclosure_yields_poison_not_the_window() {
    assert!(RingInterval::poison().clamped_to(-1.0, 1.0).is_poison());
    // The healthy half: a real narrowing still happens.
    let narrowed = RingInterval::from_bounds(-3.0, 0.5).clamped_to(-1.0, 1.0);
    assert!(!narrowed.is_poison());
    assert_eq!((narrowed.lo(), narrowed.hi()), (-1.0, 0.5));
    // A NaN window is poison too — it bounds nothing.
    assert!(
        RingInterval::from_bounds(0.0, 1.0)
            .clamped_to(f64::NAN, 1.0)
            .is_poison()
    );
    // And a window disjoint from the enclosure has no intersection to
    // report, so it refuses rather than inverting.
    assert!(
        RingInterval::from_bounds(4.0, 5.0)
            .clamped_to(-1.0, 1.0)
            .is_poison()
    );
}
