//! **What the anchor-fit refusal's number promises the author, measured
//! over the whole of grid A.**
//!
//! The nearest-fit rule picks the reported candidate and leg by the
//! least overrun `setback − extent`, and the sentence that carries the
//! number ends in "reduce the radius or move the anchor". A reader can
//! take the number for the radius reduction that would make the corner
//! fit; it is the overrun in the SETBACK metric, and the recourse is
//! un-metered on purpose. These rows execute that reading at every
//! anchor-fit entry grid A produces, rather than at one fixture, and
//! pin what following it actually does:
//!
//! - the reported leg always really overran — `setback > available` on
//!   every entry, which is what makes the rendered sentence true;
//! - the number is never the reduction that would make the corner fit.
//!   At most entries it is too SMALL: reducing the radius by exactly it
//!   leaves the same corner refusing with the same reason.
//!
//! The census is the mechanical guard of the "not a radius amount"
//! sentence at the site: it moves the day the pick moves, in either
//! direction, and it says which way the number errs. The residue that
//! owns a metered form is
//! `work/blend/anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{anchor_fit_entries, arc_arc, grid_a};
use geom_core::Point2;

const PI: f64 = core::f64::consts::PI;

fn same_point(a: Point2<f64>, b: Point2<f64>) -> bool {
    (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9
}

/// **The reported leg is one that really overran.** The rendered
/// sentence says "tangent setback {setback} m exceeds the {available} m
/// the anchor pins", so an entry whose setback does NOT exceed its
/// available would be a false sentence. The nearest-fit pick names the
/// candidate's WORSE leg, whose margin the fit gate classified
/// Negative, so the inequality is the rule's own consequence — and this
/// row is what goes red if a future pick reads the other leg.
#[test]
fn every_anchor_fit_report_names_a_leg_whose_setback_outruns_its_extent() {
    let mut entries = 0_usize;
    grid_a(|n, _, _, out| {
        let Err(err) = out else { return };
        for (at, _, setback, available) in anchor_fit_entries(err) {
            entries += 1;
            assert!(
                setback > available,
                "authoring {n} at {at:?}: the sentence claims {setback} exceeds {available}"
            );
        }
    });
    assert_eq!(entries, 3_185, "grid A's anchor-fit entries");
}

/// **The reported deficit is not the reduction that makes the corner
/// fit, and at this authoring it is twenty-one times too small.** Grid-A
/// authoring 120 (`r = 0.4`) reports a deficit of 0.00228 m; reducing
/// the radius by exactly it leaves the SAME corner refusing with the
/// SAME reason, and the loop first builds at a reduction of 0.0481 m.
/// The counter-fixture to a single measured overshoot: the number the
/// recourse is metered against errs in both directions.
#[test]
fn following_the_reported_deficit_can_leave_the_same_corner_refusing() {
    const CASE_120: [f64; 8] = [0.0, 0.2, 1.0, 0.95 * PI, 0.8, 0.2, 1.0, 2.6];
    const R_120: f64 = 0.4;
    let err = arc_arc(CASE_120, R_120).expect_err("r = 0.4 refuses");
    let entries = anchor_fit_entries(&err);
    assert_eq!(entries.len(), 1, "one anchor-fit entry");
    let (at, _, setback, available) = entries[0];
    let deficit = setback - available;
    assert!(
        (deficit - 0.0022838433353832913).abs() < 1e-15,
        "the measured deficit, got {deficit}"
    );
    let followed = R_120 - deficit - f64::EPSILON;
    let err = arc_arc(CASE_120, followed).expect_err("the recourse, followed, still refuses");
    assert!(
        anchor_fit_entries(&err)
            .iter()
            .any(|(q, ..)| same_point(*q, at)),
        "the same corner still refuses on the anchor fit: {err:?}"
    );
    // Where it does start building: a reduction twenty-one times the
    // reported one.
    arc_arc(CASE_120, R_120 - 0.0482).expect("a reduction of 0.0482 builds");
    arc_arc(CASE_120, R_120 - 0.0480).expect_err("a reduction of 0.0480 still refuses");
}

/// **The grid-A recourse census.** For each of the 3 185 anchor-fit
/// entries, "reduce the radius by the reported deficit" is followed
/// literally (one ulp of slack below) and the outcome at THAT corner is
/// counted:
///
/// - 469 entries where the deficit is not smaller than the radius, so
///   the request cannot be made at all;
/// - 2 111 where it is made and the same corner still refuses on the
///   anchor fit — the deficit was too small;
/// - 605 where the loop builds.
///
/// Zero entries where the deficit is the reduction. The cells are the
/// observable of the pick: moving either the candidate rule or the leg
/// rule moves them.
#[test]
fn the_grid_a_recourse_census_says_which_way_the_reported_deficit_errs() {
    let (mut not_a_request, mut still_refusing, mut builds, mut other) = (0, 0, 0, 0);
    grid_a(|_, case, r, out| {
        let Err(err) = out else { return };
        for (at, _, setback, available) in anchor_fit_entries(err) {
            let deficit = setback - available;
            if deficit >= r {
                not_a_request += 1;
                continue;
            }
            match arc_arc(case, r - deficit - f64::EPSILON) {
                Ok(_) => builds += 1,
                Err(e) => {
                    if anchor_fit_entries(&e)
                        .iter()
                        .any(|(q, ..)| same_point(*q, at))
                    {
                        still_refusing += 1;
                    } else {
                        other += 1;
                    }
                }
            }
        }
    });
    assert_eq!(
        (not_a_request, still_refusing, builds, other),
        (469, 2_111, 605, 0),
        "the recourse census over grid A"
    );
}
