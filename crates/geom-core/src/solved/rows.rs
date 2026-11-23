//! The door's own rows: what it admits and refuses at the scalars that
//! certify, on a residual written out by hand.

use super::{Unadmitted, admit};
use crate::{Band, Bounds, Interval, Margin, Real, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// The residual of `t` against the root `root`, widened by `spread`
/// on each side: the gap a family of carriers reads at one parameter.
fn residual_about<T: Real + crate::SpanLocate>(
    root: f64,
    spread: f64,
) -> impl FnOnce(T) -> (Margin<T>, T) {
    move |t: T| {
        let off = t - T::from_f64(root);
        let widened = off + T::from_f64(-spread).enclosure_hull(T::from_f64(spread));
        (Margin::of(widened), off)
    }
}

#[test]
fn a_candidate_on_its_root_is_admitted_with_what_its_residual_read() {
    let admitted = admit::<f64, f64>("solved_row", 0.25, band(), residual_about(0.25, 0.0));
    assert_eq!(
        admitted,
        Ok((0.25, 0.0)),
        "the candidate is the admitted value and the residual's witness rides with it"
    );
}

#[test]
fn a_candidate_off_its_root_is_refused_off_by_its_row() {
    let refused = admit::<f64, f64>("solved_row", 0.25, band(), residual_about(0.5, 0.0));
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
    let narrow = admit::<Interval, Interval>(
        "solved_row",
        0.25,
        band(),
        residual_about(0.25, 0.25 * zero),
    );
    assert!(
        matches!(narrow, Ok((t, _)) if t.lo() == 0.25 && t.hi() == 0.25),
        "a family whose members all read on at the candidate admits the candidate as one point: {narrow:?}"
    );
    let wide = admit::<Interval, Interval>("solved_row", 0.25, band(), residual_about(0.25, 1.0));
    assert!(
        matches!(wide, Err(Unadmitted::Undecided(_))),
        "a family whose residual at the candidate reaches past the band refuses at the door: {wide:?}"
    );
}
