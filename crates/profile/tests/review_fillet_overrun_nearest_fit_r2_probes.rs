//! **Review probes for the nearest-fit anchor-fit refusal (reviewer R2).**
//!
//! Rows the unit's own suite (`fillet_overrun_nearest_fit`) does not
//! carry, each measured off the kernel at the reviewed head with the
//! overrun arm privately instrumented to print both candidates' four
//! numbers, and stated as measurements:
//!
//! - the pick on corners OFF grid A's lattice (an arc×arc authoring
//!   whose report changes candidate, and one whose two candidates both
//!   overrun the outgoing leg), so the rule is pinned on fixtures the
//!   unit did not choose;
//! - a corner where the two legs' margins tie TO THE BIT, pinning the
//!   leg rule's "ties to the incoming leg" directly rather than through
//!   a side count;
//! - the four-classifications-per-candidate invariance on an
//!   off-lattice two-candidate corner, as a K count rather than a log;
//! - a line×arc grid's side census under the leg rule;
//! - **the recourse census**: over every anchor-fit entry grid A
//!   reports, what "reduce the radius by the reported deficit" actually
//!   does. It builds at 605 of 3 185, is not a request (the deficit is
//!   not below the radius) at 469, and STILL REFUSES at 2 111 — the
//!   reported deficit is a setback-currency number, and the radius
//!   reduction a corner needs ranges from 0.04× to 29× of it on this
//!   grid. The rule's stated reason, "the least radius reduction that
//!   would make it fit", is pinned here as false so that a reader of
//!   the two site comments finds the measurement beside them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats;
use geom_core::{Point2, Tol};
use profile::path::CornerReason;
use profile::{ArcSweep, Center, FilletLeg, Open, PathError, ProfileLoop, Start};

const PI: f64 = core::f64::consts::PI;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn on_circle(centre: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(centre.x + r * angle.cos(), centre.y + r * angle.sin())
}

/// Grid A's authoring shape (`review_fillet_attr_r2_probes`): the
/// corner at the origin, `case` = `[a_in, r_in, tau_in, delta_in,
/// a_out, r_out, tau_out, delta_out]`.
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

/// A ray from `(sx·R/2, 0)` east onto the radius-`big_r` circle about
/// the origin, anchored `ang` radians up it, filleted at `r`.
fn line_arc(
    big_r: f64,
    sx: f64,
    winding: ArcSweep,
    ang: f64,
    r: f64,
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(sx * big_r / 2.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())?
        .fillet_arc(
            r,
            Center {
                c: p2(0.0, 0.0),
                winding,
                p: on_circle(p2(0.0, 0.0), big_r, ang),
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())
        .map(|c| c.loop_)
}

/// Every anchor-fit entry of a refusal, in envelope order:
/// `(corner, side, setback, available)`.
fn anchor_fit_entries(e: &PathError<f64>) -> Vec<(Point2<f64>, FilletLeg, f64, f64)> {
    let PathError::NoCornerOfPair { corners, .. } = e else {
        return Vec::new();
    };
    corners
        .iter()
        .filter_map(|c| match c.reason {
            CornerReason::AnchorOutsideTrimmedExtent {
                side,
                setback,
                available,
                ..
            } => Some((c.at, side, setback, available)),
            _ => None,
        })
        .collect()
}

fn anchor_fit_at(e: &PathError<f64>, at: (f64, f64)) -> (FilletLeg, f64, f64) {
    anchor_fit_entries(e)
        .into_iter()
        .find(|(c, ..)| (c.x - at.0).abs() < 1e-9 && (c.y - at.1).abs() < 1e-9)
        .map(|(_, s, sb, av)| (s, sb, av))
        .unwrap_or_else(|| panic!("no anchor-fit entry at {at:?} in {e:?}"))
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

/// Grid A, as `review_fillet_attr_r2_probes` and the unit's suite walk
/// it: every authoring, with its ordinal (from 1), its radius and its
/// outcome.
fn grid_a(mut visit: impl FnMut(usize, [f64; 8], f64, &Result<ProfileLoop<f64>, PathError<f64>>)) {
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
                                    let r = 0.05 * f64::from(m);
                                    visit(n, case, r, &arc_arc(case, r));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(n, 18_144);
}

// ------------------------------------------------------------ fixtures

/// Off grid A's lattice: R_in 0.25 wound +1 with the head 2.85 rad
/// back, R_out 0.28 wound −1 at corner angle 1.15 with the anchor 0.35
/// rad on, r = 0.11. Measured at the far crossing, in enumeration
/// order, `(setback_in, extent_in, setback_out, extent_out)`: first
/// `(0.7457935573383649, 0.17103970864892504, 0.3358106949726515,
/// 0.6068563596970803)`, deficit 0.5748 on the incoming leg; second
/// `(0.28354247810545674, 0.17103970864892504, 0.17304566472442873,
/// 0.6068563596970803)`, deficit 0.1125 on the incoming leg — the
/// least, and the shallower candidate on BOTH legs.
const OFF_LATTICE_PICK_CHANGES: [f64; 8] = [0.0, 0.25, 1.0, 2.85, 1.15, 0.28, -1.0, 0.35];
const OFF_LATTICE_PICK_CHANGES_R: f64 = 0.11;
const OFF_LATTICE_PICK_CHANGES_AT: (f64, f64) = (-0.3901364717405869, -0.2070308413934979);

/// Off the lattice, both candidates worse on the OUTGOING leg: R_in
/// 0.25 wound −1 with the head 0.45 rad back, R_out 0.28 wound +1 at
/// corner angle 1.9 with the anchor 1.7 rad on, r = 0.23. Measured:
/// first `(0.17190965078963386, 0.44312613599503914, 0.7888498664262863,
/// 0.15100938630415942)`, deficit 0.6378 outgoing; second
/// `(0.1587164852054053, 0.44312613599503914, 0.6454514058881576,
/// 0.15100938630415942)`, deficit 0.4944 outgoing — the least.
const OFF_LATTICE_BOTH_OUTGOING: [f64; 8] = [0.0, 0.25, -1.0, 0.45, 1.9, 0.28, 1.0, 1.7];
const OFF_LATTICE_BOTH_OUTGOING_R: f64 = 0.23;
const OFF_LATTICE_BOTH_OUTGOING_AT: (f64, f64) = (-0.18856287843833558, -0.2423334068885618);

/// Grid-A authoring 1806, r = 0.3: a symmetric corner (equal carrier
/// radii, equal windings, equal anchor offsets) whose one overrunning
/// candidate has `margin_in == margin_out` TO THE BIT
/// (−0.07134169216330714 on both legs) while its setbacks differ in
/// their last bits: incoming `0.1313416921633071` against
/// `0.059999999999999956` available, outgoing `0.13134169216330713`
/// against `0.059999999999999984`.
const LEG_TIE: [f64; 8] = [0.0, 0.2, -1.0, 0.3, 2.0, 0.2, -1.0, 0.3];
const LEG_TIE_R: f64 = 0.30000000000000004;
const LEG_TIE_AT: (f64, f64) = (2.7755575615628914e-17, 4.163336342344337e-17);

// ---------------------------------------------------------------- rows

/// **C2, off the lattice.** The report carries the shallower
/// candidate's numbers, which are the second-enumerated one's here.
#[test]
fn an_off_lattice_two_candidate_corner_reports_the_shallower_candidate() {
    let err = arc_arc(OFF_LATTICE_PICK_CHANGES, OFF_LATTICE_PICK_CHANGES_R).expect_err("refuses");
    let (side, setback, available) = anchor_fit_at(&err, OFF_LATTICE_PICK_CHANGES_AT);
    assert_eq!(side, FilletLeg::Incoming);
    assert!(
        close(setback, 0.28354247810545674) && close(available, 0.17103970864892504),
        "got {setback} / {available}"
    );
    assert!(
        !close(setback, 0.7457935573383649),
        "the first-enumerated candidate"
    );
}

/// **C2, off the lattice, both candidates worse on the same leg.** The
/// least deficit is the second's, on the outgoing leg.
#[test]
fn an_off_lattice_pair_both_worse_on_the_outgoing_leg_reports_the_least() {
    let err = arc_arc(OFF_LATTICE_BOTH_OUTGOING, OFF_LATTICE_BOTH_OUTGOING_R).expect_err("refuses");
    let (side, setback, available) = anchor_fit_at(&err, OFF_LATTICE_BOTH_OUTGOING_AT);
    assert_eq!(side, FilletLeg::Outgoing);
    assert!(
        close(setback, 0.6454514058881576) && close(available, 0.15100938630415942),
        "got {setback} / {available}"
    );
    assert!(
        !close(setback, 0.7888498664262863),
        "the first-enumerated candidate"
    );
}

/// **C5, the leg tie.** Equal margins to the bit name the incoming leg
/// — pinned on the SIDE and on the incoming leg's own bits, which the
/// outgoing leg's differ from, so reversing the tie flips the row
/// without a census in between.
#[test]
fn a_bit_equal_leg_margin_tie_names_the_incoming_leg() {
    let err = arc_arc(LEG_TIE, LEG_TIE_R).expect_err("refuses");
    let (side, setback, available) = anchor_fit_at(&err, LEG_TIE_AT);
    assert_eq!(side, FilletLeg::Incoming, "ties to the incoming leg");
    assert_eq!(setback.to_bits(), 0.1313416921633071_f64.to_bits());
    assert_eq!(available.to_bits(), 0.059999999999999956_f64.to_bits());
    assert_eq!(
        (setback - available).to_bits(),
        (0.13134169216330713_f64 - 0.059999999999999984_f64).to_bits(),
        "the two legs' deficits are the same bits"
    );
}

/// **C4, as a K count on an off-lattice two-candidate corner.** Reach
/// and fit fire twice per candidate at every corner the carriers admit
/// — two crossings, two candidates each — whichever of them the door
/// later reports, and nothing escalates.
#[test]
fn every_candidate_is_classified_four_times_off_the_lattice() {
    let ((), rec) = k_stats::detached(|| {
        let _ = arc_arc(OFF_LATTICE_PICK_CHANGES, OFF_LATTICE_PICK_CHANGES_R);
    });
    let r = rec.recorded();
    let count = |name: &str| r.verdicts.iter().filter(|v| v.predicate == name).count();
    assert_eq!(r.escalations.len(), 0);
    assert_eq!(
        count("fillet_leg_reach"),
        8,
        "two candidates at each of two crossings"
    );
    assert_eq!(
        count("fillet_leg_fit"),
        8,
        "all four gates for every candidate"
    );
}

/// **C1/C5 on a second corner class.** A 960-authoring line×arc grid
/// (R ∈ {2, 1, 0.5}, ray origin at `sx·R/2` for sx ∈ {0.2, 0.8, 1.4,
/// 1.9}, both windings, anchor angle ∈ {0.3, 1.0, 2.0, 2.9}, r = 0.05mR
/// for m = 1..10): 234 build, 246 anchor-fit entries, and under the
/// leg rule the side census is `Incoming 180, Outgoing 66` where
/// naming the incoming leg whenever it overran gave `216 / 30`.
#[test]
fn line_arc_grid_side_census_under_the_leg_rule() {
    let (mut built, mut incoming, mut outgoing) = (0_usize, 0_usize, 0_usize);
    for big_r in [2.0, 1.0, 0.5] {
        for sx in [0.2, 0.8, 1.4, 1.9] {
            for winding in [ArcSweep::Ccw, ArcSweep::Cw] {
                for ang in [0.3, 1.0, 2.0, 2.9] {
                    for m in 1..=10 {
                        match line_arc(big_r, sx, winding, ang, 0.05 * f64::from(m) * big_r) {
                            Ok(_) => built += 1,
                            Err(e) => {
                                for (_, side, ..) in anchor_fit_entries(&e) {
                                    match side {
                                        FilletLeg::Incoming => incoming += 1,
                                        FilletLeg::Outgoing => outgoing += 1,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(built, 234);
    assert_eq!((incoming, outgoing), (180, 66));
}

/// **C3, the recourse census.** For every anchor-fit entry grid A
/// reports, reduce the radius by that entry's `setback − available`
/// (and `f64::EPSILON`, as the unit's row does) and build: it builds at
/// 605 entries, the deficit is not below the radius at 469, and the
/// corner STILL refuses at 2 111. Three entries pin the spread: at
/// authoring 3608 the reported deficit is 0.0013 m and the largest
/// building radius is 0.0384 m below r (29×); at 4831 following the
/// deficit lands 0.0004 m above the largest building radius and
/// refuses; at 10829 the deficit is 0.208 m where 0.0077 m already
/// builds (0.04×). The reported number meters the setback, not the
/// radius.
#[test]
fn the_recourse_census_on_grid_a() {
    let (mut builds, mut refuses, mut not_a_request) = (0_usize, 0_usize, 0_usize);
    grid_a(|_, case, r, out| {
        let Err(e) = out else { return };
        for (_, _, setback, available) in anchor_fit_entries(e) {
            let followed = r - (setback - available) - f64::EPSILON;
            if followed <= 0.0 {
                not_a_request += 1;
            } else if arc_arc(case, followed).is_ok() {
                builds += 1;
            } else {
                refuses += 1;
            }
        }
    });
    assert_eq!(
        builds + refuses + not_a_request,
        3_185,
        "grid A's anchor-fit entries"
    );
    assert_eq!(
        (builds, refuses, not_a_request),
        (605, 2_111, 469),
        "builds / still refuses / not a request"
    );

    // 3608: r = 0.4, one entry, deficit 0.0013286600616288571; the
    // largest building radius is 0.3616275035137324.
    let case = [0.0, 0.2, -1.0, 0.3, 0.8, 0.15, -1.0, 0.3];
    let err = arc_arc(case, 0.4).expect_err("refuses");
    let (_, setback, available) = anchor_fit_at(&err, (-1.3877787807814457e-17, 0.0));
    let deficit = setback - available;
    assert!(close(deficit, 0.0013286600616288571), "got {deficit}");
    assert!(
        arc_arc(case, 0.4 - deficit - f64::EPSILON).is_err(),
        "not enough"
    );
    assert!(
        arc_arc(case, 0.4 - 20.0 * deficit).is_err(),
        "twenty times is not enough"
    );
    assert!(arc_arc(case, 0.3616).is_ok() && arc_arc(case, 0.3617).is_err());

    // 4831: r = 0.35, deficit 0.21744323949862795, largest building
    // radius 0.13212218091166017 — 0.0004 m below the followed one.
    let case = [0.0, 0.2, 1.0, 0.3, 2.0, 0.5, -1.0, 0.3];
    let err = arc_arc(case, 0.35).expect_err("refuses");
    let (_, setback, available) = anchor_fit_at(&err, (0.0, -1.249000902703301e-16));
    let deficit = setback - available;
    assert!(close(deficit, 0.21744323949862795), "got {deficit}");
    assert!(arc_arc(case, 0.35 - deficit - f64::EPSILON).is_err());
    assert!(arc_arc(case, 0.1321).is_ok() && arc_arc(case, 0.1322).is_err());

    // 10829: r = 0.25, the origin's deficit 0.2081833393421254; the
    // largest building radius is 0.24231439371687716.
    let case = [0.0, 0.4, 1.0, 2.9845130209103035, 1.6, 0.5, -1.0, 0.3];
    let err = arc_arc(case, 0.25).expect_err("refuses");
    let (_, setback, available) = anchor_fit_at(&err, (0.0, 8.326672684688674e-17));
    let deficit = setback - available;
    assert!(close(deficit, 0.2081833393421254), "got {deficit}");
    assert!(arc_arc(case, 0.25 - deficit - f64::EPSILON).is_ok());
    assert!(arc_arc(case, 0.2423).is_ok() && arc_arc(case, 0.2424).is_err());
}
