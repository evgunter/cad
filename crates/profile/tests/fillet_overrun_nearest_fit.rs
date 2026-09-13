//! **The anchor-fit refusal reports the nearest fit.**
//!
//! At one corner the offset carriers admit up to two candidate
//! circles, and both can round the corner the author named yet push a
//! tangent point off the far end of a leg. The refusal that follows
//! (`CornerReason::AnchorOutsideTrimmedExtent { side, carrier, setback,
//! available }`) ends in "reduce the radius or move the anchor", a
//! recourse metered against the setback it names — so the rows here
//! pin WHICH candidate's numbers, and which of its legs', the sentence
//! carries:
//!
//! - the candidate with the least DEFICIT, `max(setback − extent)` over
//!   its two legs (the least radius reduction that would make it fit),
//!   ties to the earlier in enumeration order;
//! - on the leg that deficit is on — the candidate's WORSE leg — ties
//!   to the incoming leg.
//!
//! The pick is `path::arc_fillet::map_refusal`'s, by `f64` enclosure
//! reads of margins `sugar::arc_fillet_trims` already classified; the
//! construction carries every corner-side overrun out at the scalar and
//! compares nothing. The fixtures are authorings of FILLET-ATTR's grid
//! A (`review_fillet_attr_r2_probes::grid_a`; the index `n` below is
//! the authoring's ordinal in that enumeration) and one line×arc
//! corner, and every number a row pins was READ off the kernel at the
//! merge base with the overrun arm instrumented to print both
//! candidates' four numbers — the PR that landed the rule carries the
//! table. They are measurements, stated as such; a row does not
//! re-derive the candidate geometry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats;
use geom_core::{Point2, Sign, Tol};
use profile::path::CornerReason;
use profile::{
    ArcSweep, Center, CornerRefusal, FilletLeg, FilletLegCarrier, Open, PathError, ProfileLoop,
    Start,
};

const PI: f64 = core::f64::consts::PI;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn on_circle(centre: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(centre.x + r * angle.cos(), centre.y + r * angle.sin())
}

/// Grid A's authoring, verbatim from `review_fillet_attr_r2_probes`:
/// the corner at the origin, each carrier of radius `r_c` winding
/// `tau` with the corner at angle `a` about its centre, each far anchor
/// `delta` radians from the corner along its own leg. `case` is
/// `[a_in, r_in, tau_in, delta_in, a_out, r_out, tau_out, delta_out]`.
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

/// Grid A, PR 1895's parameters: R_in in {0.2, 0.4, 0.15}, R_out in
/// {0.2, 0.15, 0.5}, tau in {+1, -1} on both sides, corner angle 0.4k
/// for k = 1..7, deltas in {0.3, 0.95pi, 2.6} x {0.3, 0.95pi/2, 2.6},
/// r = 0.05m for m = 1..8 — 18 144 authorings, visited with their
/// ordinal (from 1) and their refusal.
fn grid_a(mut visit: impl FnMut(usize, &PathError<f64>)) -> usize {
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
                                    if let Err(e) = arc_arc(case, 0.05 * f64::from(m)) {
                                        visit(n, &e);
                                    }
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

/// The envelope's entry at `at`, which must be an anchor-fit refusal:
/// `(side, carrier, setback, available)`.
fn anchor_fit_at(err: &PathError<f64>, at: Point2<f64>) -> (FilletLeg, FilletLegCarrier, f64, f64) {
    let PathError::NoCornerOfPair { corners, .. } = err else {
        panic!("not the envelope: {err:?}");
    };
    let entry: &CornerRefusal<f64> = corners
        .iter()
        .find(|c| (c.at.x - at.x).abs() < 1e-9 && (c.at.y - at.y).abs() < 1e-9)
        .unwrap_or_else(|| panic!("no entry at {at:?} in {err:?}"));
    match entry.reason {
        CornerReason::AnchorOutsideTrimmedExtent {
            side,
            carrier,
            setback,
            available,
        } => (side, carrier, setback, available),
        ref other => panic!("the entry at {at:?} is not the anchor-fit refusal: {other:?}"),
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

// ------------------------------------------------------------ fixtures

/// Grid-A authoring 674, r = 0.1: both crossings refuse with two
/// overrunning candidates each, and they are the two halves of the
/// rule.
const CASE_674: [f64; 8] = [
    0.0,
    0.2,
    1.0,
    2.9845130209103035,
    1.2000000000000002,
    0.2,
    -1.0,
    0.3,
];
const R_674: f64 = 0.1;
/// Its far crossing. Measured candidates, in enumeration order, each
/// `(reported side, setback, available, deficit)`: the first is
/// `(Incoming, 0.6089636547032122, 0.20858407346410207, 0.40037958)`,
/// the second `(Incoming, 0.2593548760147463, 0.20858407346410207,
/// 0.05077080)`. The least deficit is the second's.
const FAR_674: (f64, f64) = (-0.2724715508953347, -0.1864078171934453);
const FAR_674_LEAST: (f64, f64) = (0.2593548760147463, 0.20858407346410207);
const FAR_674_FIRST_SETBACK: f64 = 0.6089636547032122;
/// Its authored corner, the origin: candidates `(Outgoing,
/// 0.2593548760147463, 0.06, 0.19935488)` then `(Outgoing,
/// 0.6089636547032126, 0.06, 0.54896365)` — the first already has the
/// least deficit, so the rule and enumeration order agree there.
const ORIGIN_674_LEAST: (f64, f64) = (0.2593548760147463, 0.06000000000000001);

/// Grid-A authoring 11131, r = 0.15: one crossing refuses with two
/// overrunning candidates, and on the nearer one the OUTGOING leg
/// overruns more than the incoming leg does. Measured, `[incoming,
/// outgoing]` as `(setback, extent)`: first candidate `(0.8111865164137458,
/// 0.1782922145466674)` / `(1.2592339366915768, 0.43060875772818263)`,
/// deficit 0.82862518 on its outgoing leg; second `(0.6865746126406342,
/// 0.1782922145466674)` / `(1.0129674746263981, 0.43060875772818263)`,
/// deficit 0.58235872, on its outgoing leg — the least.
const CASE_11131: [f64; 8] = [0.0, 0.4, -1.0, 2.9845130209103035, 0.4, 0.5, 1.0, 2.6];
const R_11131: f64 = 0.15000000000000002;
const CORNER_11131: (f64, f64) = (-0.7294982469364131, 0.22678382937465041);
const CORNER_11131_WORSE: (f64, f64) = (1.0129674746263981, 0.43060875772818263);
const CORNER_11131_INCOMING: (f64, f64) = (0.6865746126406342, 0.1782922145466674);

/// A line×arc corner with ONE tangent circle (the offset line is
/// tangent to the offset circle) that overruns BOTH legs: a ray from
/// (0.2, 0) east onto the radius-2 circle about the origin, anchored
/// 0.3 rad up it, filleted at r = 1. Measured: incoming (straight)
/// setback 2.0 against 1.8 available, outgoing (arc) setback π against
/// 0.6 — the outgoing leg's deficit is the larger.
fn line_arc_both_legs_overrun() -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(0.2, 0.0))
        .toward(1.0, 0.0, Tol::witness())?
        .fillet_arc(
            1.0,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: on_circle(p2(0.0, 0.0), 2.0, 0.3),
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())
        .map(|c| c.loop_)
}

// ---------------------------------------------------------------- rows

/// **The reported numbers are the least-deficit candidate's**, and at a
/// corner where the first candidate already has the least deficit the
/// report is unchanged — both halves on one authoring.
#[test]
fn two_overrunning_candidates_report_the_least_deficit_one() {
    let err = arc_arc(CASE_674, R_674).expect_err("r = 0.1 refuses at both crossings");
    let (side, carrier, setback, available) = anchor_fit_at(&err, p2(FAR_674.0, FAR_674.1));
    assert_eq!(side, FilletLeg::Incoming);
    assert!(
        matches!(carrier, FilletLegCarrier::Arc { .. }),
        "{carrier:?}"
    );
    assert!(
        close(setback, FAR_674_LEAST.0) && close(available, FAR_674_LEAST.1),
        "the far crossing must report the least-deficit candidate \
         (setback {}, available {}), got setback {setback} available {available}",
        FAR_674_LEAST.0,
        FAR_674_LEAST.1
    );
    assert!(
        !close(setback, FAR_674_FIRST_SETBACK),
        "the first-enumerated candidate's setback {FAR_674_FIRST_SETBACK} is not the report"
    );
    let (side, _, setback, available) = anchor_fit_at(&err, p2(0.0, 0.0));
    assert_eq!(side, FilletLeg::Outgoing);
    assert!(
        close(setback, ORIGIN_674_LEAST.0) && close(available, ORIGIN_674_LEAST.1),
        "at the origin the first candidate is the nearest fit: got {setback} / {available}"
    );
}

/// **The reported leg is the one the deficit is on.** On authoring
/// 11131's nearer candidate the outgoing leg overruns by more than the
/// incoming one, so the sentence names the outgoing leg's numbers —
/// not the incoming leg's, which is the leg the fit gate classifies
/// first.
#[test]
fn the_reported_leg_is_the_one_the_deficit_is_on() {
    let err = arc_arc(CASE_11131, R_11131).expect_err("r = 0.15 refuses");
    let (side, _, setback, available) = anchor_fit_at(&err, p2(CORNER_11131.0, CORNER_11131.1));
    assert_eq!(side, FilletLeg::Outgoing, "the worse leg");
    assert!(
        close(setback, CORNER_11131_WORSE.0) && close(available, CORNER_11131_WORSE.1),
        "got setback {setback} available {available}"
    );
    assert!(
        setback - available > CORNER_11131_INCOMING.0 - CORNER_11131_INCOMING.1,
        "the reported leg's overrun IS the candidate's deficit"
    );
}

/// The same leg rule on the line×arc channel, where the offset line is
/// tangent to the offset circle and the ONE candidate overruns both
/// legs: the straight leg by 0.2 m, the circular one by π − 0.6 m.
#[test]
fn a_single_candidate_reports_its_worse_leg_on_the_line_arc_channel() {
    let err = line_arc_both_legs_overrun().expect_err("r = 1 overruns both legs");
    let (side, carrier, setback, available) = anchor_fit_at(&err, p2(2.0, 0.0));
    assert_eq!(side, FilletLeg::Outgoing);
    assert!(
        matches!(carrier, FilletLegCarrier::Arc { radius, .. } if close(radius, 2.0)),
        "{carrier:?}"
    );
    assert!(
        close(setback, PI) && close(available, 0.6),
        "got {setback} / {available}"
    );
}

/// **The recourse, followed.** At authoring 674 the report's deficit is
/// `setback − available` = 0.05077 m on the far crossing. Reducing the
/// radius by it builds; reducing it by the DISCARDED candidate's
/// deficit (0.40038 m) is more than the radius itself, so it is not
/// even a request. Measured and stated rather than claimed: the
/// reported deficit is followable but NOT tight on an arc leg — a
/// setback there shrinks faster than the radius, and the largest
/// building radius is 0.0903 m, a reduction of 0.0097 m.
#[test]
fn reducing_the_radius_by_the_reported_deficit_builds() {
    let err = arc_arc(CASE_674, R_674).expect_err("r = 0.1 refuses");
    let (_, _, setback, available) = anchor_fit_at(&err, p2(FAR_674.0, FAR_674.1));
    let deficit = setback - available;
    assert!(
        close(deficit, 0.05077080255064423),
        "the measured deficit, got {deficit}"
    );
    // One ulp of slack below the reduction, so the row does not sit on
    // the exact bit the subtraction lands on.
    let followed = R_674 - deficit - f64::EPSILON;
    arc_arc(CASE_674, followed)
        .unwrap_or_else(|e| panic!("r − deficit = {followed} must build, got {e:?}"));
    // Measured: the largest radius that builds. A reduction well
    // short of the reported one already fits, so the recourse is
    // sufficient, not necessary.
    arc_arc(CASE_674, 0.0903).expect("0.0903 builds: the reported deficit is not tight");
    arc_arc(CASE_674, 0.0904).expect_err("0.0904 still refuses");
    let discarded_deficit = FAR_674_FIRST_SETBACK - FAR_674_LEAST.1;
    assert!(
        R_674 - discarded_deficit < 0.0,
        "the first-enumerated candidate's deficit {discarded_deficit} exceeds the radius"
    );
    assert!(matches!(
        arc_arc(CASE_674, R_674 - discarded_deficit),
        Err(PathError::NonpositiveFilletRadius { .. })
    ));
}

/// **The grid-A census.** Over the 3 185 anchor-fit entries grid A
/// reports: the 19 entries whose CANDIDATE changed under the
/// least-deficit rule carry the nearest fit's numbers and not the
/// first-enumerated candidate's (both measured at the merge base, per
/// authoring); and the leg rule moved 230 entries from the incoming to
/// the outgoing side (one of them also a candidate change), so the
/// side census is `Incoming 1555, Outgoing 1630` where enumeration
/// order gave `1785 / 1400`. Restoring either pick turns a cell red.
#[test]
fn grid_a_census_of_the_reports_that_changed() {
    /// One report the rule moved: the authoring, its corner, the side
    /// and numbers reported at the head, and the setback enumeration
    /// order reported at the merge base.
    struct Changed {
        authoring: usize,
        at: (f64, f64),
        side: FilletLeg,
        least_setback: f64,
        first_setback: f64,
        available: f64,
    }
    const CHANGED: [Changed; 19] = [
        Changed {
            authoring: 674,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Incoming,
            least_setback: 0.2593548760147463,
            first_setback: 0.6089636547032122,
            available: 0.20858407346410207,
        },
        Changed {
            authoring: 698,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Incoming,
            least_setback: 0.2593548760147463,
            first_setback: 0.6089636547032122,
            available: 0.13168146928204139,
        },
        Changed {
            authoring: 1170,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Outgoing,
            least_setback: 0.25935487601474616,
            first_setback: 0.6089636547032125,
            available: 0.13168146928204152,
        },
        Changed {
            authoring: 2763,
            at: (-0.1399540948519864, -0.1907733977129824),
            side: FilletLeg::Incoming,
            least_setback: 0.41974955089623195,
            first_setback: 0.5837148994756088,
            available: 0.3437299931179843,
        },
        Changed {
            authoring: 2787,
            at: (-0.1399540948519864, -0.19077339771298238),
            side: FilletLeg::Incoming,
            least_setback: 0.4197495508962323,
            first_setback: 0.5837148994756085,
            available: 0.2668273889359236,
        },
        Changed {
            authoring: 3258,
            at: (-0.1399540948519864, -0.1907733977129824),
            side: FilletLeg::Outgoing,
            least_setback: 0.2734527234317653,
            first_setback: 0.3964267348662917,
            available: 0.1174016622211193,
        },
        Changed {
            authoring: 10682,
            at: (-0.783750042724489, -0.11285346564883554),
            side: FilletLeg::Incoming,
            least_setback: 0.3338783017653157,
            first_setback: 1.0371657191466082,
            available: 0.05157510640421066,
        },
        Changed {
            authoring: 10756,
            at: (-0.6554795318585245, -0.3077827298620721),
            side: FilletLeg::Incoming,
            least_setback: 0.6217397816881525,
            first_setback: 0.9860933897365228,
            available: 0.2883642569169621,
        },
        Changed {
            authoring: 10780,
            at: (-0.6554795318585244, -0.3077827298620721),
            side: FilletLeg::Incoming,
            least_setback: 0.6217397816881516,
            first_setback: 0.9860933897365238,
            available: 0.13455904855284082,
        },
        Changed {
            authoring: 11131,
            at: (-0.7294982469364131, 0.22678382937465041),
            side: FilletLeg::Outgoing,
            least_setback: 1.0129674746263981,
            first_setback: 0.8111865164137458,
            available: 0.43060875772818263,
        },
        Changed {
            authoring: 11155,
            at: (-0.7294982469364131, 0.22678382937465047),
            side: FilletLeg::Incoming,
            least_setback: 0.6865746126406336,
            first_setback: 0.8111865164137471,
            available: 0.02448700618254609,
        },
        Changed {
            authoring: 11254,
            at: (-0.6554795318585243, -0.3077827298620722),
            side: FilletLeg::Outgoing,
            least_setback: 0.938179589624243,
            first_setback: 1.3936215996847054,
            available: 0.4902085357191556,
        },
        Changed {
            authoring: 12842,
            at: (-0.18660545980264856, -0.14546491093262548),
            side: FilletLeg::Incoming,
            least_setback: 0.2734527234317634,
            first_setback: 0.3964267348662937,
            available: 0.17507861535766484,
        },
        Changed {
            authoring: 12866,
            at: (-0.1866054598026486, -0.14546491093262554),
            side: FilletLeg::Incoming,
            least_setback: 0.27345272343176297,
            first_setback: 0.39642673486629426,
            available: 0.1174016622211194,
        },
        Changed {
            authoring: 13331,
            at: (-0.1866054598026486, -0.1454649109326255),
            side: FilletLeg::Outgoing,
            least_setback: 0.4197495508962348,
            first_setback: 0.583714899475606,
            available: 0.04527869102695395,
        },
        Changed {
            authoring: 13339,
            at: (-0.18660545980264864, -0.1454649109326255),
            side: FilletLeg::Outgoing,
            least_setback: 0.41974955089623633,
            first_setback: 0.5837148994756048,
            available: 0.2668273889359235,
        },
        Changed {
            authoring: 14713,
            at: (-0.25450600640207477, -0.10760341363492834),
            side: FilletLeg::Incoming,
            least_setback: 0.1762757514694486,
            first_setback: 0.4149631465690201,
            available: 0.09643805509807649,
        },
        Changed {
            authoring: 14737,
            at: (-0.25450600640207477, -0.10760341363492848),
            side: FilletLeg::Incoming,
            least_setback: 0.17627575146944843,
            first_setback: 0.4149631465690206,
            available: 0.03876110196153109,
        },
        Changed {
            authoring: 15209,
            at: (-0.25450600640207477, -0.1076034136349284),
            side: FilletLeg::Outgoing,
            least_setback: 0.17627575146944863,
            first_setback: 0.41496314656902017,
            available: 0.038761101961531076,
        },
    ];
    let (mut entries, mut incoming, mut outgoing, mut hits) = (0_usize, 0_usize, 0_usize, 0_usize);
    let authorings = grid_a(|n, e| {
        let PathError::NoCornerOfPair { corners, .. } = e else {
            return;
        };
        for c in corners {
            if let CornerReason::AnchorOutsideTrimmedExtent { side, .. } = c.reason {
                entries += 1;
                match side {
                    FilletLeg::Incoming => incoming += 1,
                    FilletLeg::Outgoing => outgoing += 1,
                }
            }
        }
        if let Some(row) = CHANGED.iter().find(|row| row.authoring == n) {
            hits += 1;
            let (got_side, _, setback, got_available) = anchor_fit_at(e, p2(row.at.0, row.at.1));
            assert_eq!(got_side, row.side, "authoring {n}");
            assert!(
                close(setback, row.least_setback) && close(got_available, row.available),
                "authoring {n}: expected the nearest fit's {} / {}, got {setback} / {got_available}",
                row.least_setback,
                row.available
            );
            assert!(
                !close(setback, row.first_setback),
                "authoring {n} reports the first-enumerated {}",
                row.first_setback
            );
        }
    });
    assert_eq!(authorings, 18_144);
    assert_eq!(hits, 19, "every changed authoring is visited");
    assert_eq!(entries, 3_185, "grid A's anchor-fit entries");
    assert_eq!(
        (incoming, outgoing),
        (1_555, 1_630),
        "the side census under the leg rule"
    );
}

/// **The sample sequence is a function of the corner class, not of the
/// pick.** The construction classifies all four gates for every
/// candidate before the overrun arm, so carrying every overrun out —
/// and choosing among them at the door — fires nothing. Pinned as the
/// verdict log of each fixture, recorded at the merge base: the
/// authoring with two two-candidate crossings, the one whose leg
/// flips, and the line×arc tangent case.
#[test]
fn the_verdict_sequence_is_unchanged_by_the_pick() {
    fn sequence(work: impl FnOnce()) -> (Vec<(&'static str, Sign)>, usize) {
        let ((), rec) = k_stats::detached(work);
        let r = rec.recorded();
        (
            r.verdicts.iter().map(|v| (v.predicate, v.sign)).collect(),
            r.escalations.len(),
        )
    }
    use Sign::{Negative, Positive, Zero};
    let (seq, esc) = sequence(|| {
        let _ = arc_arc(CASE_674, R_674);
    });
    assert_eq!(esc, 0);
    // The two crossings, each: arm, turn, two enclosing gates, the two
    // offset-circle clearances, the lever, then reach×2 + fit×2 for
    // BOTH candidates — the second candidate's four gates fire although
    // the first already overran.
    let per_corner = |turn: Sign, fits: [Sign; 4]| {
        vec![
            ("fillet_corner_arm", Positive),
            ("fillet_corner_turn", turn),
            ("fillet_enclosing_carrier", Positive),
            ("fillet_enclosing_carrier", Positive),
            ("fillet_offset_circles_external", Positive),
            ("fillet_offset_circles_internal", Positive),
            ("fillet_offset_lever", Positive),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_fit", fits[0]),
            ("fillet_leg_fit", fits[1]),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_fit", fits[2]),
            ("fillet_leg_fit", fits[3]),
        ]
    };
    let mut expect = vec![
        ("path_arc_center_radius", Positive),
        ("path_fillet_radius", Positive),
        ("path_arc_center_radius", Positive),
        ("path_carrier_meet", Positive),
        ("path_carrier_meet", Positive),
        ("path_carrier_meet", Positive),
        ("path_corner_advance_arc", Positive),
        ("path_corner_reach_arc", Positive),
        ("path_corner_advance_arc", Positive),
        ("path_corner_reach_arc", Positive),
    ];
    expect.extend(per_corner(
        Negative,
        [Positive, Negative, Positive, Negative],
    ));
    expect.extend(per_corner(
        Positive,
        [Negative, Positive, Negative, Positive],
    ));
    assert_eq!(seq, expect, "authoring 674's verdict log");
    assert_eq!(seq.len(), 40);

    let (seq, esc) = sequence(|| {
        let _ = arc_arc(CASE_11131, R_11131);
    });
    assert_eq!(esc, 0);
    let mut expect = vec![
        ("path_arc_center_radius", Positive),
        ("path_fillet_radius", Positive),
        ("path_arc_center_radius", Positive),
        ("path_carrier_meet", Positive),
        ("path_carrier_meet", Positive),
        ("path_carrier_meet", Positive),
        ("path_corner_advance_arc", Positive),
        ("path_corner_reach_arc", Positive),
        ("path_corner_advance_arc", Positive),
        ("path_corner_reach_arc", Positive),
        // The first crossing's offset carriers are disjoint (internal
        // clearance Negative): no candidate, no reach/fit samples.
        ("fillet_corner_arm", Positive),
        ("fillet_corner_turn", Negative),
        ("fillet_enclosing_carrier", Positive),
        ("fillet_enclosing_carrier", Positive),
        ("fillet_offset_circles_external", Positive),
        ("fillet_offset_circles_internal", Negative),
    ];
    expect.extend(per_corner(
        Positive,
        [Negative, Negative, Negative, Negative],
    ));
    assert_eq!(seq, expect, "authoring 11131's verdict log");

    let (seq, esc) = sequence(|| {
        let _ = line_arc_both_legs_overrun();
    });
    assert_eq!(esc, 0);
    assert_eq!(
        seq,
        vec![
            ("path_director_norm", Positive),
            ("path_fillet_radius", Positive),
            ("path_arc_center_radius", Positive),
            ("path_carrier_meet", Positive),
            ("path_corner_advance", Negative),
            ("path_corner_advance", Positive),
            ("path_corner_reach_arc", Positive),
            ("fillet_corner_arm", Positive),
            ("fillet_corner_turn", Positive),
            ("fillet_enclosing_carrier", Positive),
            ("fillet_offset_line_circle", Zero),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_reach", Positive),
            ("fillet_leg_fit", Negative),
            ("fillet_leg_fit", Negative),
        ],
        "the line×arc tangent case's verdict log"
    );
}
