//! The door's own rows: what it admits and refuses at the scalars that
//! certify, on a residual written out by hand.

#![allow(clippy::expect_used)]

use super::{Unadmitted, decide_admit};
use crate::{Band, Bounds, Interval, Margin, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The residual of `t` against the root `root`, widened by `spread`
/// on each side: the gap a family of carriers reads at one parameter
/// (`f64` holds no hull, so its rows pass no spread).
fn residual_about<T: crate::SpanLocate>(
    root: f64,
    spread: f64,
) -> impl FnOnce(T) -> (Margin<T>, T) {
    move |t: T| {
        let off = t - T::from_f64(root);
        let widened = if spread == 0.0 {
            off
        } else {
            off + T::from_f64(-spread).enclosure_hull(T::from_f64(spread))
        };
        (Margin::of(widened), off)
    }
}

#[test]
fn a_candidate_on_its_root_is_admitted_with_what_its_residual_read() {
    let admitted = decide_admit::<f64, f64>("solved_row", 0.25, band(), residual_about(0.25, 0.0));
    assert_eq!(
        admitted,
        Ok((0.25, 0.0)),
        "the candidate is the admitted value and the residual's witness rides with it"
    );
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
        matches!(narrow, Ok((t, _)) if t.lo() == 0.25 && t.hi() == 0.25),
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
