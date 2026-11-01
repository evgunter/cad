//! **The anchor-fit refusal reports the nearest fit.**
//!
//! At one corner the offset carriers admit up to two candidate
//! circles, and both can round the corner the author named yet push a
//! tangent point off the far end of a leg. The refusal that follows
//! (`CornerReason::AnchorOutsideTrimmedExtent { side, carrier, setback,
//! available }`) names one candidate's numbers on one leg, so the rows
//! here pin WHICH:
//!
//! - the candidate nearest to fitting IN THE SETBACK METRIC —
//!   `fillet_select::nearest_candidate`'s ladder over the candidates'
//!   setback pairs, the crate's one home of "the nearest candidate at
//!   one corner" (a candidate tie is unreachable by geometry: one
//!   candidate is shallower on both legs);
//! - on that candidate's WORSE leg — the one whose setback outruns its
//!   extent by more — ties to the incoming leg.
//!
//! The pick is `path::arc_fillet::map_refusal`'s, by `f64` enclosure
//! reads of setbacks and margins `sugar::arc_fillet_trims` already
//! classified; the construction carries every corner-side overrun out
//! at the scalar and compares nothing.
//!
//! **What the reported number is not.** `setback − available` is the
//! reported leg's overrun in the setback metric. It is NOT the radius
//! reduction that would make the corner fit, and the recourse the
//! sentence ends in ("reduce the radius or move the anchor") is
//! un-metered on purpose: a setback does not scale 1:1 with the radius
//! on either leg kind. Followed as a radius reduction over grid A it is
//! not a request at 469 entries, leaves the same corner refusing at
//! 2 111 and builds at 605, never tight — the census and its
//! counter-fixtures are `review_fillet_overrun_nearest_fit_r1_probes`
//! and `_r2_probes`, and `fillet_recourse_followability`'s fit row is
//! an unremarked instance of the same class. The residue is
//! `work/blend/anchor-fit-refusal-reports-a-setback-excess-not-a-radius-reduction.md`.
//!
//! The fixtures are authorings of FILLET-ATTR's grid A
//! (`common::grid_a`; `n` names the authoring's ordinal in that
//! enumeration) and one line×arc corner (`common::line_arc`), and every
//! number a row pins was READ off the kernel with the overrun arm
//! instrumented to print both candidates' four numbers; the PR that
//! landed the rule carries the table, and R1's probes re-derive the
//! pick independently from the arm's own numbers over the whole grid.
//! They are measurements, stated as such.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::{anchor_fit_at, arc_arc, close, grid_a, line_arc};
use geom_core::Sign;
use geom_core::k_stats;
use profile::path::CornerReason;
use profile::{ArcSweep, FilletLeg, FilletLegCarrier, PathError, ProfileLoop};

const PI: f64 = core::f64::consts::PI;

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
/// `(reported side, setback, available, overrun)`: the first is
/// `(Incoming, 0.6089636547032122, 0.20858407346410207, 0.40037958)`,
/// the second `(Incoming, 0.2593548760147463, 0.20858407346410207,
/// 0.05077080)`. The nearest fit is the second.
const FAR_674: (f64, f64) = (-0.2724715508953347, -0.1864078171934453);
const FAR_674_NEAREST: (f64, f64) = (0.2593548760147463, 0.20858407346410207);
const FAR_674_FIRST_SETBACK: f64 = 0.6089636547032122;
/// Its authored corner, the origin: candidates `(Outgoing,
/// 0.2593548760147463, 0.06, 0.19935488)` then `(Outgoing,
/// 0.6089636547032126, 0.06, 0.54896365)` — the first is already the
/// nearest, so the rule and enumeration order agree there.
const ORIGIN_674_NEAREST: (f64, f64) = (0.2593548760147463, 0.06000000000000001);

/// Grid-A authoring 11131, r = 0.15: one crossing refuses with two
/// overrunning candidates, and on the nearer one the OUTGOING leg
/// overruns more than the incoming leg does. Measured, `[incoming,
/// outgoing]` as `(setback, extent)`: first candidate `(0.8111865164137458,
/// 0.1782922145466674)` / `(1.2592339366915768, 0.43060875772818263)`,
/// overrun 0.82862518 on its outgoing leg; second `(0.6865746126406342,
/// 0.1782922145466674)` / `(1.0129674746263981, 0.43060875772818263)`,
/// overrun 0.58235872 on its outgoing leg — the nearest, shallower on
/// both legs.
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
/// 0.6 — the outgoing leg's overrun is the larger.
fn line_arc_both_legs_overrun() -> Result<ProfileLoop<f64>, PathError<f64>> {
    line_arc(2.0, 0.2, ArcSweep::Ccw, 0.3, 1.0)
}

// ---------------------------------------------------------------- rows

/// **The reported numbers are the nearest candidate's**, and at a
/// corner where the first candidate is already the nearest the report
/// is the first's — both halves on one authoring.
#[test]
fn two_overrunning_candidates_report_the_least_deficit_one() {
    let err = arc_arc(CASE_674, R_674).expect_err("r = 0.1 refuses at both crossings");
    let (side, carrier, setback, available) = anchor_fit_at(&err, FAR_674);
    assert_eq!(side, FilletLeg::Incoming);
    assert!(
        matches!(carrier, FilletLegCarrier::Arc { .. }),
        "{carrier:?}"
    );
    assert!(
        close(setback, FAR_674_NEAREST.0) && close(available, FAR_674_NEAREST.1),
        "the far crossing must report the nearest candidate \
         (setback {}, available {}), got setback {setback} available {available}",
        FAR_674_NEAREST.0,
        FAR_674_NEAREST.1
    );
    assert!(
        !close(setback, FAR_674_FIRST_SETBACK),
        "the first-enumerated candidate's setback {FAR_674_FIRST_SETBACK} is not the report"
    );
    let (side, _, setback, available) = anchor_fit_at(&err, (0.0, 0.0));
    assert_eq!(side, FilletLeg::Outgoing);
    assert!(
        close(setback, ORIGIN_674_NEAREST.0) && close(available, ORIGIN_674_NEAREST.1),
        "at the origin the first candidate is the nearest fit: got {setback} / {available}"
    );
}

/// **The reported leg is the one whose setback outruns its extent by
/// more.** On authoring 11131's nearer candidate the outgoing leg
/// overruns by more than the incoming one, so the sentence names the
/// outgoing leg's numbers — not the incoming leg's, which is the leg
/// the fit gate classifies first.
#[test]
fn the_reported_leg_is_the_one_the_deficit_is_on() {
    let err = arc_arc(CASE_11131, R_11131).expect_err("r = 0.15 refuses");
    let (side, _, setback, available) = anchor_fit_at(&err, CORNER_11131);
    assert_eq!(side, FilletLeg::Outgoing, "the worse leg");
    assert!(
        close(setback, CORNER_11131_WORSE.0) && close(available, CORNER_11131_WORSE.1),
        "got setback {setback} available {available}"
    );
    assert!(
        setback - available > CORNER_11131_INCOMING.0 - CORNER_11131_INCOMING.1,
        "the reported leg's overrun is the candidate's larger one"
    );
}

/// The same leg rule on the line×arc channel, where the offset line is
/// tangent to the offset circle and the ONE candidate overruns both
/// legs: the straight leg by 0.2 m, the circular one by π − 0.6 m.
#[test]
fn a_single_candidate_reports_its_worse_leg_on_the_line_arc_channel() {
    let err = line_arc_both_legs_overrun().expect_err("r = 1 overruns both legs");
    let (side, carrier, setback, available) = anchor_fit_at(&err, (2.0, 0.0));
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

/// **The grid-A census.** Over the 3 185 anchor-fit entries grid A
/// reports: the 19 entries whose CANDIDATE differs from the
/// first-enumerated one carry the nearest fit's numbers and not the
/// first's (both measured, per authoring); and the leg rule names the
/// outgoing side at 230 entries where the incoming leg also overran
/// (one of them also a candidate change), so the side census is
/// `Incoming 1555, Outgoing 1630` where naming the incoming leg
/// whenever it overran gives `1785 / 1400`. Restoring either pick
/// turns a cell red.
#[test]
fn grid_a_census_of_the_reports_that_changed() {
    /// One report the rule moves off enumeration order: the authoring,
    /// its corner, the side and numbers reported, and the setback the
    /// first-enumerated candidate carries instead.
    struct Changed {
        authoring: usize,
        at: (f64, f64),
        side: FilletLeg,
        nearest_setback: f64,
        first_setback: f64,
        available: f64,
    }
    const CHANGED: [Changed; 19] = [
        Changed {
            authoring: 674,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Incoming,
            nearest_setback: 0.2593548760147463,
            first_setback: 0.6089636547032122,
            available: 0.20858407346410207,
        },
        Changed {
            authoring: 698,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Incoming,
            nearest_setback: 0.2593548760147463,
            first_setback: 0.6089636547032122,
            available: 0.13168146928204139,
        },
        Changed {
            authoring: 1170,
            at: (-0.2724715508953347, -0.1864078171934453),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.25935487601474616,
            first_setback: 0.6089636547032125,
            available: 0.13168146928204152,
        },
        Changed {
            authoring: 2763,
            at: (-0.1399540948519864, -0.1907733977129824),
            side: FilletLeg::Incoming,
            nearest_setback: 0.41974955089623195,
            first_setback: 0.5837148994756088,
            available: 0.3437299931179843,
        },
        Changed {
            authoring: 2787,
            at: (-0.1399540948519864, -0.19077339771298238),
            side: FilletLeg::Incoming,
            nearest_setback: 0.4197495508962323,
            first_setback: 0.5837148994756085,
            available: 0.2668273889359236,
        },
        Changed {
            authoring: 3258,
            at: (-0.1399540948519864, -0.1907733977129824),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.2734527234317653,
            first_setback: 0.3964267348662917,
            available: 0.1174016622211193,
        },
        Changed {
            authoring: 10682,
            at: (-0.783750042724489, -0.11285346564883554),
            side: FilletLeg::Incoming,
            nearest_setback: 0.3338783017653157,
            first_setback: 1.0371657191466082,
            available: 0.05157510640421066,
        },
        Changed {
            authoring: 10756,
            at: (-0.6554795318585245, -0.3077827298620721),
            side: FilletLeg::Incoming,
            nearest_setback: 0.6217397816881525,
            first_setback: 0.9860933897365228,
            available: 0.2883642569169621,
        },
        Changed {
            authoring: 10780,
            at: (-0.6554795318585244, -0.3077827298620721),
            side: FilletLeg::Incoming,
            nearest_setback: 0.6217397816881516,
            first_setback: 0.9860933897365238,
            available: 0.13455904855284082,
        },
        Changed {
            authoring: 11131,
            at: (-0.7294982469364131, 0.22678382937465041),
            side: FilletLeg::Outgoing,
            nearest_setback: 1.0129674746263981,
            first_setback: 0.8111865164137458,
            available: 0.43060875772818263,
        },
        Changed {
            authoring: 11155,
            at: (-0.7294982469364131, 0.22678382937465047),
            side: FilletLeg::Incoming,
            nearest_setback: 0.6865746126406336,
            first_setback: 0.8111865164137471,
            available: 0.02448700618254609,
        },
        Changed {
            authoring: 11254,
            at: (-0.6554795318585243, -0.3077827298620722),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.938179589624243,
            first_setback: 1.3936215996847054,
            available: 0.4902085357191556,
        },
        Changed {
            authoring: 12842,
            at: (-0.18660545980264856, -0.14546491093262548),
            side: FilletLeg::Incoming,
            nearest_setback: 0.2734527234317634,
            first_setback: 0.3964267348662937,
            available: 0.17507861535766484,
        },
        Changed {
            authoring: 12866,
            at: (-0.1866054598026486, -0.14546491093262554),
            side: FilletLeg::Incoming,
            nearest_setback: 0.27345272343176297,
            first_setback: 0.39642673486629426,
            available: 0.1174016622211194,
        },
        Changed {
            authoring: 13331,
            at: (-0.1866054598026486, -0.1454649109326255),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.4197495508962348,
            first_setback: 0.583714899475606,
            available: 0.04527869102695395,
        },
        Changed {
            authoring: 13339,
            at: (-0.18660545980264864, -0.1454649109326255),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.41974955089623633,
            first_setback: 0.5837148994756048,
            available: 0.2668273889359235,
        },
        Changed {
            authoring: 14713,
            at: (-0.25450600640207477, -0.10760341363492834),
            side: FilletLeg::Incoming,
            nearest_setback: 0.1762757514694486,
            first_setback: 0.4149631465690201,
            available: 0.09643805509807649,
        },
        Changed {
            authoring: 14737,
            at: (-0.25450600640207477, -0.10760341363492848),
            side: FilletLeg::Incoming,
            nearest_setback: 0.17627575146944843,
            first_setback: 0.4149631465690206,
            available: 0.03876110196153109,
        },
        Changed {
            authoring: 15209,
            at: (-0.25450600640207477, -0.1076034136349284),
            side: FilletLeg::Outgoing,
            nearest_setback: 0.17627575146944863,
            first_setback: 0.41496314656902017,
            available: 0.038761101961531076,
        },
    ];
    let (mut entries, mut incoming, mut outgoing, mut hits) = (0_usize, 0_usize, 0_usize, 0_usize);
    let authorings = grid_a(|n, _, _, out| {
        let Err(e) = out else { return };
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
            let (got_side, _, setback, got_available) = anchor_fit_at(e, row.at);
            assert_eq!(got_side, row.side, "authoring {n}");
            assert!(
                close(setback, row.nearest_setback) && close(got_available, row.available),
                "authoring {n}: expected the nearest fit's {} / {}, got {setback} / {got_available}",
                row.nearest_setback,
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
/// verdict log of each fixture: the authoring with two two-candidate
/// crossings, the one whose leg is the outgoing one, and the line×arc
/// tangent case.
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
