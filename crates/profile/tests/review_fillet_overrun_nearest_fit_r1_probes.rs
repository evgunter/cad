//! **What the anchor-fit refusal's number promises the author, measured
//! over the whole of grid A.**
//!
//! The nearest-fit rule picks the reported candidate and leg by the
//! least deficit `max(setback − extent)`, and both sites justify the
//! pick by calling that deficit *the least radius reduction that would
//! make it fit*: the recourse the sentence ends in ("reduce the radius
//! or move the anchor") is metered against the setback it names. These
//! rows execute that sentence at every anchor-fit entry grid A
//! produces, rather than at one fixture, and pin what following it
//! actually does:
//!
//! - the reported leg always really overran — `setback > available` on
//!   every entry, which is what makes the rendered sentence true;
//! - the deficit is never the reduction that would make the corner
//!   fit. At most entries it is too SMALL: reducing the radius by
//!   exactly it leaves the same corner refusing with the same reason.
//!
//! The census is the mechanical guard the "metered against the
//! reported setback" claim owes: it moves the day the pick moves, in
//! either direction, and it says which way the number errs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::path::CornerReason;
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};

const PI: f64 = core::f64::consts::PI;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn on_circle(centre: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(centre.x + r * angle.cos(), centre.y + r * angle.sin())
}

/// Grid A's authoring (PR 1895's parameters, as
/// `review_fillet_attr_r2_probes` spells them): the corner at the
/// origin, each carrier of radius `r_c` winding `tau` with the corner
/// at angle `a` about its centre, each far anchor `delta` radians from
/// the corner along its own leg.
fn arc_arc(case: [f64; 8], r: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let [
        a_in,
        r_in,
        tau_in,
        delta_in,
        a_out,
        r_out,
        tau_out,
        delta_out,
    ] = case;
    let c1 = p2(-r_in * a_in.cos(), -r_in * a_in.sin());
    let c2 = p2(-r_out * a_out.cos(), -r_out * a_out.sin());
    let head = on_circle(c1, r_in, a_in - tau_in * delta_in);
    let next = on_circle(c2, r_out, a_out + tau_out * delta_out);
    let w = |t: f64| if t > 0.0 { ArcSweep::Ccw } else { ArcSweep::Cw };
    let closed = Open
        .arc_fillet_arc(
            Center {
                c: c1,
                winding: w(tau_in),
                p: head,
            },
            r,
            Center {
                c: c2,
                winding: w(tau_out),
                p: next,
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())?;
    Ok(closed.loop_)
}

/// Grid A, visited with each authoring's ordinal (from 1), its case and
/// its radius.
fn grid_a(mut visit: impl FnMut(usize, [f64; 8], f64)) -> usize {
    let mut n = 0_usize;
    for r_in in [0.2, 0.4, 0.15] {
        for r_out in [0.2, 0.15, 0.5] {
            for tau_in in [1.0, -1.0] {
                for tau_out in [1.0, -1.0] {
                    for k in 1..=7 {
                        let a_out = 0.4 * f64::from(k);
                        for delta_in in [0.3, 0.95 * PI, 2.6] {
                            for delta_out in [0.3, 0.95 * PI / 2.0, 2.6] {
                                for m in 1..=8 {
                                    n += 1;
                                    let case = [
                                        0.0, r_in, tau_in, delta_in, a_out, r_out, tau_out,
                                        delta_out,
                                    ];
                                    visit(n, case, 0.05 * f64::from(m));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    n
}

/// Every anchor-fit entry of one refusal, as `(corner, setback,
/// available)`.
fn anchor_fit_entries(err: &PathError<f64>) -> Vec<(Point2<f64>, f64, f64)> {
    let PathError::NoCornerOfPair { corners, .. } = err else {
        return Vec::new();
    };
    corners
        .iter()
        .filter_map(|c| match c.reason {
            CornerReason::AnchorOutsideTrimmedExtent {
                setback, available, ..
            } => Some((c.at, setback, available)),
            _ => None,
        })
        .collect()
}

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
    grid_a(|n, case, r| {
        let Err(err) = arc_arc(case, r) else { return };
        for (at, setback, available) in anchor_fit_entries(&err) {
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
    let (at, setback, available) = entries[0];
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
            .any(|(q, _, _)| same_point(*q, at)),
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
    grid_a(|_, case, r| {
        let Err(err) = arc_arc(case, r) else { return };
        for (at, setback, available) in anchor_fit_entries(&err) {
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
                        .any(|(q, _, _)| same_point(*q, at))
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
