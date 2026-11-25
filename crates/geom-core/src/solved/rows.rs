//! The door's own rows: what it admits and refuses at the scalars that
//! certify, on a residual written out by hand.

#![allow(clippy::expect_used)]

use super::{Checked, Unadmitted, decide_admit, decide_reaching};
use crate::{Band, Bounds, Interval, Margin, Sign, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The residual of `t` against the root `root`, widened by `spread`
/// on each side: the gap a family of carriers reads at one parameter
/// (`f64` holds no hull, so its rows pass no spread).
fn residual_about<T: crate::SpanLocate>(root: f64, spread: f64) -> impl FnOnce(T) -> Checked<T, T> {
    reaching(root, spread, 0.0)
}

/// [`residual_about`], its check stating `reach` along.
fn reaching<T: crate::SpanLocate>(
    root: f64,
    spread: f64,
    reach: f64,
) -> impl FnOnce(T) -> Checked<T, T> {
    move |t: T| {
        let off = t - T::from_f64(root);
        let widened = if spread == 0.0 {
            off
        } else {
            off + T::from_f64(-spread).enclosure_hull(T::from_f64(spread))
        };
        Checked {
            residual: Margin::of(widened),
            reach,
            witness: off,
        }
    }
}

#[test]
fn a_candidate_on_its_root_is_admitted_with_what_its_residual_read() {
    let (admitted, witness) =
        decide_admit::<f64, f64>("solved_row", 0.25, band(), reaching(0.25, 0.0, 3e-7))
            .expect("a candidate on its root");
    let (value, bound) = admitted.into_parts();
    assert_eq!(
        (value, witness),
        (0.25, 0.0),
        "the candidate is the admitted value and the residual's witness rides with it"
    );
    assert_eq!(
        (bound.across(), bound.along()),
        (band().zero(), 3e-7),
        "the bound is the band's zero across and the check's reach along"
    );
}

/// A check that bounds no reach admits nothing, whatever its residual
/// read: a value with no bound has no reader that decides for every
/// candidate.
#[test]
fn a_candidate_whose_check_bounds_no_reach_is_refused_unbounded() {
    for reach in [f64::NAN, f64::INFINITY, -1.0] {
        let refused =
            decide_admit::<f64, f64>("solved_row", 0.25, band(), reaching(0.25, 0.0, reach));
        assert!(
            matches!(
                refused,
                Err(Unadmitted::Unbounded {
                    row: "solved_row",
                    ..
                })
            ),
            "reach {reach}: {refused:?}"
        );
    }
}

/// A distance decided with a reach is the answer at every point within
/// the reach of the one it was read at: past the band only where the
/// nearest such point is, zero only where the farthest is, and the
/// band's escalation between, a straddle where each end decided.
#[test]
fn a_distance_is_decided_for_every_candidate_within_the_reach() {
    let (zero, escalate) = (band().zero(), band().escalate());
    let read =
        |length: f64, reach: f64| decide_reaching("solved_row", Margin::of(length), reach, band());
    assert_eq!(read(3.0 * escalate, escalate).ok(), Some(Sign::Positive));
    assert_eq!(read(0.25 * zero, 0.5 * zero).ok(), Some(Sign::Zero));
    let straddle = read(0.25 * zero, 2.0 * escalate);
    assert!(
        matches!(straddle, Err(d) if d.predicate == Some("solved_row")),
        "on at the point, past the band at the reach: {straddle:?}"
    );
    let near = read(1.5 * escalate, escalate);
    assert!(
        matches!(near, Err(d) if d.predicate == Some("solved_row")),
        "past the band at the point, in its gap at the nearest candidate: {near:?}"
    );
}

/// The bands a band-taking reader is handed: the escalation moved out by
/// the reach, and the zero moved in by it where the reach is inside it.
#[test]
fn the_reach_moves_the_bands_thresholds_apart() {
    let (zero, escalate) = (band().zero(), band().escalate());
    let reach = |along: f64| {
        decide_admit::<f64, f64>("solved_row", 0.25, band(), reaching(0.25, 0.0, along))
            .expect("a candidate on its root")
            .0
            .into_parts()
            .1
    };
    let (clear, on) = reach(0.25 * zero).bands(band()).expect("finite");
    assert_eq!(
        (clear.zero(), clear.escalate()),
        (zero, escalate + 0.25 * zero)
    );
    let on = on.expect("a reach inside the zero keeps a zero reading");
    assert_eq!(
        (on.zero(), on.escalate()),
        (0.75 * zero, escalate + 0.25 * zero)
    );
    let (clear, on) = reach(2.0 * zero).bands(band()).expect("finite");
    assert_eq!(clear.escalate(), escalate + 2.0 * zero);
    assert!(on.is_none(), "no zero reading is every candidate's: {on:?}");
}

#[test]
fn a_candidate_off_its_root_is_refused_off_by_its_row() {
    let refused = decide_admit::<f64, f64>("solved_row", 0.25, band(), residual_about(0.5, 0.0));
    assert_eq!(
        refused,
        Err(Unadmitted::Off {
            row: "solved_row",
            candidate: 0.25,
        }),
        "a residual definitely past the band refuses, naming the check and the candidate"
    );
}

#[test]
fn at_interval_one_point_is_admitted_only_where_every_member_reads_on() {
    let zero = band().zero();
    let narrow = decide_admit::<Interval, Interval>(
        "solved_row",
        0.25,
        band(),
        residual_about(0.25, 0.25 * zero),
    );
    assert!(
        matches!(narrow.map(|(a, _)| a.into_parts().0), Ok(t) if t.lo() == 0.25 && t.hi() == 0.25),
        "a family whose members all read on at the candidate admits the candidate as one point: {narrow:?}"
    );
    let wide =
        decide_admit::<Interval, Interval>("solved_row", 0.25, band(), residual_about(0.25, 1.0));
    assert!(
        matches!(wide, Err(Unadmitted::Undecided(_))),
        "a family whose residual at the candidate reaches past the band refuses at the door: {wide:?}"
    );
}

/// The band is the one the door decides against, not a looser one: a
/// family spread three times the band's zero about the candidate reads
/// in the escalation and refuses, where a band ten times wider would
/// admit it.
#[test]
fn at_interval_a_family_spread_past_the_bands_zero_refuses_by_its_row() {
    let zero = band().zero();
    assert!(
        3.0 * zero < band().escalate(),
        "the spread sits inside the escalation"
    );
    let spread = decide_admit::<Interval, Interval>(
        "solved_row",
        0.25,
        band(),
        residual_about(0.25, 3.0 * zero),
    );
    assert!(
        matches!(spread, Err(Unadmitted::Undecided(d)) if d.predicate == Some("solved_row")),
        "a family spread 3·zero about the candidate refuses at the door, named: {spread:?}"
    );
}
