//! FILLET-ATTR review, lane r1 — probes for the refusal envelope
//! (`PathError::NoCornerOfPair`, `docs/FILLET-ATTR-SPEC.md` §Review).
//!
//! What each row asserts:
//!
//! - `grid_a_every_two_entry_envelope_is_ordered_by_the_anchors` —
//!   Phase 1's grid A re-taken on the head: every two-entry envelope
//!   has its first entry nearest the two bracketing anchors (C2, the
//!   ORDER rule recomputed from the reported points, not the sort key),
//!   and the cell counts (length 1 / length 2 / mixed reasons /
//!   enumeration-first-is-not-nearest) are printed for the report.
//! - `a_construction_stage_refusal_about_the_bracketed_corner_is_not_hidden_behind_a_window_entry`
//!   — C1 across the two stages: the bracketed corner reaches the
//!   construction and refuses there with a PAIR-level reason
//!   (`AlreadyTangent` → `CarriersParallel`, a collapsed turn lever),
//!   the other corner is window-discarded. The row asserts that the
//!   refusal is either about the pair or names the bracketed corner.
//!   It is RED on the frozen head: the envelope names only the
//!   unbracketed corner, and the merge base's precedence (construction
//!   channel first) is inverted for this shape.
//! - `the_straight_pair_names_its_one_corner_point` — the straight
//!   pair's one-entry envelope carries the derived corner point (the
//!   only Rust-side pin of `at` on that channel; the Python suite pins
//!   it too).
//! - `a_gate_only_pair_lists_both_window_discarded_corners` — when NO
//!   corner reaches the construction the window channel answers with
//!   every discarded corner, each carrying its own window.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::path::{CornerReason, CornerWindow, PathNoCornerReason};
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};

use crate::common;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn on_circle(center: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(center.x + r * angle.cos(), center.y + r * angle.sin())
}

/// The arc x arc authoring `tests/fillet_refusal_envelope.rs` and
/// `tests/blend7_review_probes.rs` both build (copied rather than
/// shared: both keep theirs private). Returns the two carrier centres
/// and the two anchors beside the result so the order rule can be
/// recomputed from the authored geometry.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn arc_arc(
    a_in: f64,
    r_in: f64,
    tau_in: f64,
    delta_in: f64,
    a_out: f64,
    r_out: f64,
    tau_out: f64,
    delta_out: f64,
    r: f64,
) -> (
    Result<ProfileLoop<f64>, PathError<f64>>,
    [Point2<f64>; 2],
    [Point2<f64>; 2],
) {
    let corner = p2(0.0, 0.0);
    let c1 = p2(corner.x - r_in * a_in.cos(), corner.y - r_in * a_in.sin());
    let c2 = p2(
        corner.x - r_out * a_out.cos(),
        corner.y - r_out * a_out.sin(),
    );
    let head = on_circle(c1, r_in, a_in - tau_in * delta_in);
    let next = on_circle(c2, r_out, a_out + tau_out * delta_out);
    let w = |t: f64| if t > 0.0 { ArcSweep::Ccw } else { ArcSweep::Cw };
    let res = Open
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
        )
        .and_then(|s| s.line_to(Start, Tol::witness()))
        .map(|closed| closed.loop_);
    (res, [c1, c2], [head, next])
}

fn dist(a: Point2<f64>, b: Point2<f64>) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

/// The order the envelope promises, recomputed from the entry's point
/// and the two authored anchors.
fn span(at: Point2<f64>, anchors: [Point2<f64>; 2]) -> f64 {
    dist(at, anchors[0]) + dist(at, anchors[1])
}

fn reason_word(r: &CornerReason<f64>) -> &'static str {
    match r {
        CornerReason::OutsideAnchors(_) => "Outside",
        CornerReason::NoTangentCircle(_) => "NoTangent",
        CornerReason::AnchorOutsideTrimmedExtent { .. } => "AnchorFit",
        CornerReason::EnclosesLegCarrier { .. } => "Encloses",
    }
}

/// **C2 on Phase 1's grid A, re-taken on the head.**
///
/// The PR body's grid A: R_in in {0.2, 0.4, 0.15}, R_out in {0.2, 0.15,
/// 0.5}, tau in {+1, -1} on both sides, corner angle 0.4k (k = 1..7),
/// deltas in {0.3, 0.95 pi, 2.6} x {0.3, 0.95 pi/2, 2.6}, r = 0.05 m
/// (m = 1..8): 18 144 authorings. The row asserts the ORDER rule on
/// every two-entry envelope — the first entry's span to the two anchors
/// is the smaller — and prints the cells the PR body reports so they
/// can be compared: length 1 / length 2 / mixed reasons, and how often
/// the FIRST-ENUMERATED corner (the `+n` root of `circle_circle`, which
/// is what the merge base reported) is not the entry the head lists
/// first — the before-cell of C2, recomputed without a merge-base build.
#[test]
fn grid_a_every_two_entry_envelope_is_ordered_by_the_anchors() {
    let pi = core::f64::consts::PI;
    let mut authorings = 0_u32;
    let mut builds = 0_u32;
    let mut envelopes = 0_u32;
    let mut len1 = 0_u32;
    let mut len2 = 0_u32;
    let mut other_len = 0_u32;
    let mut mixed = 0_u32;
    let mut first_enumerated_not_first_listed = 0_u32;
    let mut apart_sum = 0.0_f64;
    let mut apart_max = 0.0_f64;
    let mut gate_entries = 0_u32;
    let mut other_refusals = 0_u32;
    let mut pair_words: std::collections::BTreeMap<String, u32> = Default::default();
    for &r_in in &[0.2, 0.4, 0.15] {
        for &r_out in &[0.2, 0.15, 0.5] {
            for &tau_in in &[1.0, -1.0] {
                for &tau_out in &[1.0, -1.0] {
                    for k in 1..=7 {
                        let a_out = 0.4 * f64::from(k);
                        for &d_in in &[0.3, 0.95 * pi, 2.6] {
                            for &d_out in &[0.3, 0.95 * pi / 2.0, 2.6] {
                                for m in 1..=8 {
                                    let r = 0.05 * f64::from(m);
                                    authorings += 1;
                                    let (res, [c1, c2], anchors) = arc_arc(
                                        0.0, r_in, tau_in, d_in, a_out, r_out, tau_out, d_out, r,
                                    );
                                    match res {
                                        Ok(_) => builds += 1,
                                        Err(PathError::NoCornerOfPair { corners, .. }) => {
                                            envelopes += 1;
                                            assert!(!corners.is_empty(), "an empty envelope");
                                            if corners.iter().any(|c| {
                                                matches!(c.reason, CornerReason::OutsideAnchors(_))
                                            }) {
                                                gate_entries += 1;
                                            }
                                            match corners.len() {
                                                1 => len1 += 1,
                                                2 => {
                                                    len2 += 1;
                                                    let (a, b) = (corners[0].at, corners[1].at);
                                                    // The ORDER rule, from the authored anchors.
                                                    assert!(
                                                        span(a, anchors) <= span(b, anchors),
                                                        "first entry is not nearest the anchors: \
                                                         r_in {r_in} r_out {r_out} tau {tau_in}/{tau_out} \
                                                         a_out {a_out} d {d_in}/{d_out} r {r}: \
                                                         {a:?} span {} vs {b:?} span {}",
                                                        span(a, anchors),
                                                        span(b, anchors)
                                                    );
                                                    let (wa, wb) = (
                                                        reason_word(&corners[0].reason),
                                                        reason_word(&corners[1].reason),
                                                    );
                                                    if wa != wb {
                                                        mixed += 1;
                                                    }
                                                    let mut key = [wa, wb];
                                                    key.sort_unstable();
                                                    *pair_words
                                                        .entry(format!("{}+{}", key[0], key[1]))
                                                        .or_default() += 1;
                                                    // `circle_circle` enumerates the +n root first,
                                                    // n the left normal of c1 -> c2.
                                                    let d = (c2.x - c1.x, c2.y - c1.y);
                                                    let n = (-d.1, d.0);
                                                    let side = |p: Point2<f64>| {
                                                        (p.x - c1.x) * n.0 + (p.y - c1.y) * n.1
                                                    };
                                                    if side(a) < 0.0 && side(b) > 0.0 {
                                                        first_enumerated_not_first_listed += 1;
                                                        let apart = dist(a, b);
                                                        apart_sum += apart;
                                                        apart_max = apart_max.max(apart);
                                                    }
                                                }
                                                _ => other_len += 1,
                                            }
                                        }
                                        Err(_) => other_refusals += 1,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("GRID A on the head: authorings {authorings}, builds {builds}, envelopes {envelopes}");
    println!(
        "  envelope length 1: {len1} ({:.1} %), length 2: {len2} ({:.1} %), other {other_len}",
        100.0 * f64::from(len1) / f64::from(envelopes),
        100.0 * f64::from(len2) / f64::from(envelopes)
    );
    println!(
        "  mixed reasons across the two crossings: {mixed} ({:.1} % of envelopes)",
        100.0 * f64::from(mixed) / f64::from(envelopes)
    );
    println!(
        "  first-enumerated (+n root) is NOT the first listed: {first_enumerated_not_first_listed} \
         ({:.1} % of envelopes); apart mean {:.3} m, max {:.3} m",
        100.0 * f64::from(first_enumerated_not_first_listed) / f64::from(envelopes),
        if first_enumerated_not_first_listed > 0 {
            apart_sum / f64::from(first_enumerated_not_first_listed)
        } else {
            0.0
        },
        apart_max
    );
    println!(
        "  envelopes carrying a window entry: {gate_entries}; non-envelope refusals: {other_refusals}"
    );
    println!("  two-entry reason pairs: {pair_words:?}");
    assert_eq!(authorings, 18_144, "the grid is the PR body's grid A");
    assert!(
        len2 > 0 && mixed > 0,
        "the grid must reach the case the list exists for"
    );
    assert_eq!(other_len, 0, "a carrier pair derives at most two corners");
}

/// **C1 across the two stages — RED on the frozen head.**
///
/// A ray x circle pair. The ray's origin sits `t` before the AHEAD
/// crossing and far past the behind one, so the behind crossing is
/// window-discarded. At the ahead crossing the incoming lever is `t`
/// and the ray meets the circle at the small angle `phi`, so the
/// construction's `fillet_corner_turn` margin `t * sin(phi)` classifies
/// Zero while every window and meet gate is definite — the construction
/// refuses `AlreadyTangent`, which `map_refusal` maps to the PAIR-level
/// `NoCornerForFillet { CarriersParallel }`.
///
/// The merge base reported that construction refusal (its build channel
/// outranked its gate channel whatever the refusal's shape). The head
/// keeps pair-level construction refusals in `whole_refused`, which
/// surfaces only when BOTH entry lists are empty — so here the envelope
/// names the corner the author did not bracket, alone, and says nothing
/// about the one they did. The row asserts what the spec's claim asks
/// for: a refusal that is about the pair, or one that names the
/// bracketed corner.
#[test]
fn a_construction_stage_refusal_about_the_bracketed_corner_is_not_hidden_behind_a_window_entry() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    // phi <= 1/(2k) keeps t * sin(phi) <= eps for t = 1.5 k eps; the
    // meet margin R (1 - cos phi) ~ R phi^2 / 2 must clear k eps, so
    // R >= 16 k^3 eps (and at least 1 m so the fixture stays readable).
    let phi = 0.5 / k;
    let t = 1.5 * k * eps;
    let radius = (16.0 * k.powi(3) * eps).max(1.0);
    let h = radius * phi.sin();
    let centre = p2(0.0, radius * phi.cos());
    let ahead = p2(h, 0.0);
    let behind = p2(-h, 0.0);
    let origin = p2(h - t, 0.0);
    // The arrival anchor a radian further round, counter-clockwise from
    // the ahead crossing.
    let ahead_angle = (ahead.y - centre.y).atan2(ahead.x - centre.x);
    let next = on_circle(centre, radius, ahead_angle + 1.0);
    let err = Open
        .at(origin)
        .toward(1.0, 0.0, tol)
        .unwrap()
        .fillet_arc(
            0.01 * radius,
            Center {
                c: centre,
                winding: ArcSweep::Ccw,
                p: next,
            },
            tol,
        )
        .expect_err("a collapsed turn lever at the bracketed crossing must refuse");
    println!(
        "hairline-lever pair: R {radius}, phi {phi:e}, t {t:e}, crossings {behind:?} / {ahead:?}"
    );
    println!("  the head answers: {err}");
    match &err {
        // The pair-level answer the merge base gave.
        PathError::NoCornerForFillet {
            reason: PathNoCornerReason::CarriersParallel,
            ..
        } => {}
        PathError::NoCornerOfPair { corners, .. } => {
            assert!(
                corners.iter().any(|c| dist(c.at, ahead) < 1e-6),
                "the bracketed crossing {ahead:?} refused at the construction stage and is not \
                 named; the envelope lists only {:?}",
                corners
                    .iter()
                    .map(|c| (c.at, reason_word(&c.reason)))
                    .collect::<Vec<_>>()
            );
        }
        other => panic!("unexpected refusal shape: {other:?}"),
    }
}

/// The straight pair's one-entry envelope names the derived corner.
///
/// Ray from the origin along +x, arrival along +y through (3, 2): the
/// carriers meet at (3, 0), and r = 2.5 outruns the 2 m arrival leg.
#[test]
fn the_straight_pair_names_its_one_corner_point() {
    let err = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(2.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(3.0, 2.0), Tol::witness())
        .expect_err("the setback outruns the arrival leg");
    let corners = common::corners(&err);
    assert_eq!(corners.len(), 1);
    assert!(
        dist(corners[0].at, p2(3.0, 0.0)) < 1e-12,
        "the entry's point is the carriers' intersection: {:?}",
        corners[0].at
    );
    assert!(common::anchor_fit(&err).is_some(), "{err:?}");
}

/// When NO corner reaches the construction, the window channel answers
/// with every discarded corner and its own window — the list, not a
/// pick. A ray through the circle whose origin sits past BOTH crossings:
/// both are behind the incoming ray's start.
#[test]
fn a_gate_only_pair_lists_both_window_discarded_corners() {
    let err = Open
        .at(p2(3.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("both crossings sit behind the ray's start");
    let corners = common::corners(&err);
    assert_eq!(corners.len(), 2, "{err:?}");
    for c in corners {
        assert!(
            matches!(
                c.reason,
                CornerReason::OutsideAnchors(CornerWindow::BehindIncomingRay)
            ),
            "{err:?}"
        );
        assert!(c.at.x < 3.0 && c.at.y.abs() < 1e-12, "{:?}", c.at);
    }
    // Nearest the anchors first: the anchors are (3, 0) and (0, 2), so
    // the x = +2 crossing precedes x = -2.
    assert!(corners[0].at.x > corners[1].at.x, "{err}");
}
