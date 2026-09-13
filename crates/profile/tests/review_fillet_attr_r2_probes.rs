//! FILLET-ATTR review lane r2: probes for claims no shipped row makes.
//!
//! Four rows, each falsifying a claim of `docs/FILLET-ATTR-SPEC.md`
//! §Review or of PR 1895's body that the unit's own suite pins on a
//! single fixture or not at all:
//!
//! - `p1_grid_a_re_taken_at_the_head` re-takes the PR body's Phase 1
//!   cells (envelope length distribution, and the share of refusals
//!   mixing DIFFERENT reasons across the two crossings) on the body's
//!   own grid A, at the HEAD instead of at the merge base. The lane
//!   measured them with reverted instrumentation; nothing in the
//!   committed suite re-takes them, so nothing goes red if the channel
//!   content moves.
//! - `c2_the_order_rule_holds_over_the_whole_grid` checks the ORDER
//!   rule — nearest the two bracketing anchors first — on every
//!   two-entry envelope grid A produces, rather than on the one
//!   fixture `fillet_refusal_envelope.rs` pins it on.
//! - `c1_a_window_discarded_crossing_that_really_refused_is_dropped`
//!   exhibits the deviation the PR discloses as its item 2, as
//!   behaviour: the same carrier pair, two anchor settings. In one the
//!   far crossing is an entry of the envelope; in the other the
//!   windows discard it and the envelope reports ONE corner while that
//!   crossing still refuses. C1 as the spec states it — "no corner is
//!   ever reported alone when another refused" — is false on the
//!   second authoring.
//! - `c3_the_envelope_radius_is_the_authored_one_on_every_arm` pins the
//!   one payload number that changed house: `radius`, deleted from the
//!   enclosing arm and re-homed on the envelope. Nothing else asserts
//!   it on the enclosing path.
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

/// PR 1895's grid-A authoring, spelled from its own parameters: the
/// corner at the origin, each carrier of radius `r_c` winding `tau`
/// with the corner at angle `a` about its centre, each far anchor
/// `delta` radians from the corner along its own leg.
#[allow(clippy::too_many_arguments)]
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
) -> Result<ProfileLoop<f64>, PathError<f64>> {
    let corner = p2(0.0, 0.0);
    let c1 = p2(corner.x - r_in * a_in.cos(), corner.y - r_in * a_in.sin());
    let c2 = p2(
        corner.x - r_out * a_out.cos(),
        corner.y - r_out * a_out.sin(),
    );
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

/// The two bracketing anchors of a grid-A authoring: the incoming
/// side's `head` and the arrival side's `next`, recomputed here so the
/// order rule is checked against the authoring and not against the
/// implementation's own key.
#[allow(clippy::too_many_arguments)]
fn anchors(
    a_in: f64,
    r_in: f64,
    tau_in: f64,
    delta_in: f64,
    a_out: f64,
    r_out: f64,
    tau_out: f64,
    delta_out: f64,
) -> (Point2<f64>, Point2<f64>) {
    let c1 = p2(-r_in * a_in.cos(), -r_in * a_in.sin());
    let c2 = p2(-r_out * a_out.cos(), -r_out * a_out.sin());
    (
        on_circle(c1, r_in, a_in - tau_in * delta_in),
        on_circle(c2, r_out, a_out + tau_out * delta_out),
    )
}

fn span(at: Point2<f64>, a: Point2<f64>, b: Point2<f64>) -> f64 {
    (at.x - a.x).hypot(at.y - a.y) + (at.x - b.x).hypot(at.y - b.y)
}

/// A short discriminant of an entry's reason, for the mixed-reason
/// count: two entries "mix" when these differ.
fn reason_kind(reason: &CornerReason<f64>) -> &'static str {
    match reason {
        CornerReason::OutsideAnchors(_) => "OA",
        CornerReason::NoTangentCircle(_) => "NT",
        CornerReason::AnchorOutsideTrimmedExtent { .. } => "AF",
        CornerReason::EnclosesLegCarrier { .. } => "Enc",
    }
}

/// PR 1895's grid A, verbatim from the body: R_in in {0.2, 0.4, 0.15},
/// R_out in {0.2, 0.15, 0.5}, tau in {+1, -1} on both sides, corner
/// angle 0.4k for k = 1..7, deltas in {0.3, 0.95pi, 2.6} x {0.3,
/// 0.95pi/2, 2.6}, r = 0.05m for m = 1..8 — 18 144 authorings.
fn grid_a(mut visit: impl FnMut(&[f64; 8], &PathError<f64>)) -> usize {
    let mut authorings = 0_usize;
    for r_in in [0.2, 0.4, 0.15] {
        for r_out in [0.2, 0.15, 0.5] {
            for tau_in in [1.0, -1.0] {
                for tau_out in [1.0, -1.0] {
                    for k in 1..=7 {
                        let a_out = 0.4 * f64::from(k);
                        for delta_in in [0.3, 0.95 * PI, 2.6] {
                            for delta_out in [0.3, 0.95 * PI / 2.0, 2.6] {
                                for m in 1..=8 {
                                    let r = 0.05 * f64::from(m);
                                    authorings += 1;
                                    let case = [
                                        0.0, r_in, tau_in, delta_in, a_out, r_out, tau_out,
                                        delta_out,
                                    ];
                                    if let Err(e) = arc_arc(
                                        0.0, r_in, tau_in, delta_in, a_out, r_out, tau_out,
                                        delta_out, r,
                                    ) {
                                        visit(&case, &e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    authorings
}

/// **Phase 1's table, re-taken on the head.**
///
/// The PR body measured grid A at the merge base with instrumentation
/// that was reverted: 18 144 authorings, 10 694 refusals through the
/// answering channel, 8 182 (76.5 %) of length one, 2 512 (23.5 %) of
/// length two, and 1 727 (16.1 %) mixing different reasons across the
/// crossings. The change is supposed to leave both channels' CONTENT
/// alone and only stop discarding — so the same cells, read off the
/// shipped envelope at the head, must reproduce those counts. This row
/// is the guard that number lost when the instrumentation was reverted.
#[test]
fn p1_grid_a_re_taken_at_the_head() {
    let (mut refusals, mut len1, mut len2, mut longer, mut mixed) = (0, 0, 0, 0, 0);
    let authorings = grid_a(|_, e| {
        let PathError::NoCornerOfPair { corners, .. } = e else {
            return;
        };
        refusals += 1;
        match corners.len() {
            1 => len1 += 1,
            2 => {
                len2 += 1;
                if reason_kind(&corners[0].reason) != reason_kind(&corners[1].reason) {
                    mixed += 1;
                }
            }
            _ => longer += 1,
        }
    });
    println!(
        "R2 P1 grid A: authorings {authorings}, envelope refusals {refusals}, \
         len1 {len1}, len2 {len2}, len>2 {longer}, mixed {mixed}"
    );
    assert_eq!(authorings, 18_144, "grid A is the PR body's grid");
    assert_eq!(longer, 0, "a carrier pair derives at most two corners");
    // The PR body's cells, to the count.
    assert_eq!(refusals, 10_694, "the PR body's refusal count");
    assert_eq!(len1, 8_182, "the PR body's length-1 cell");
    assert_eq!(len2, 2_512, "the PR body's length-2 cell");
    assert_eq!(mixed, 1_727, "the PR body's mixed-reason cell");
}

/// **C2 over the grid, not over one fixture.**
///
/// `fillet_refusal_envelope.rs` checks the order rule on two
/// authorings. Here every two-entry envelope grid A produces is checked
/// against the rule recomputed from the authoring's own anchors: the
/// first entry's summed distance to the two bracketing anchors is not
/// the larger one.
#[test]
fn c2_the_order_rule_holds_over_the_whole_grid() {
    let mut checked = 0_usize;
    let mut worst = 0.0_f64;
    grid_a(|case, e| {
        let PathError::NoCornerOfPair { corners, .. } = e else {
            return;
        };
        if corners.len() != 2 {
            return;
        }
        let (a, b) = anchors(
            case[0], case[1], case[2], case[3], case[4], case[5], case[6], case[7],
        );
        let (s0, s1) = (span(corners[0].at, a, b), span(corners[1].at, a, b));
        checked += 1;
        worst = worst.max(s0 - s1);
        assert!(
            s0 <= s1 + 1e-12,
            "entry 0 is farther from the anchors than entry 1 ({s0} vs {s1}) on {case:?}"
        );
    });
    println!("R2 C2: {checked} two-entry envelopes, worst (s0 - s1) = {worst:e}");
    assert!(checked > 2_000, "the grid must exercise the order rule");
}

/// **C1 as the spec states it is false, by the lane's own channel
/// rule.**
///
/// One carrier pair, two anchor settings. With the outgoing anchor 2.6
/// rad round its carrier both crossings reach the construction and both
/// are entries. Pull that anchor back to 0.3 rad and the reach gate
/// discards the far crossing — which is the SAME crossing, at the same
/// point, still refusing — and the envelope reports one corner alone.
///
/// The unit discloses this as deviation 2. The row exists so that the
/// behaviour is pinned rather than argued: if a later pass merges the
/// two channels, this row goes red and says which claim changed.
#[test]
fn c1_a_window_discarded_crossing_that_really_refused_is_dropped() {
    // One carrier pair: R = 0.2 circles, both ccw, the corner at the
    // origin and the arrival carrier's corner angle 0.8 rad; the
    // incoming anchor 2.6 rad back. Only the OUTGOING anchor moves
    // between the two authorings, and it moves along its own carrier,
    // so both crossings stay exactly where they are.
    let at = |delta_out: f64| arc_arc(0.0, 0.2, 1.0, 2.6, 0.8, 0.2, 1.0, delta_out, 0.4);

    let both = at(2.6).expect_err("neither crossing takes r = 0.4");
    let PathError::NoCornerOfPair { corners: two, .. } = &both else {
        panic!("expected the envelope, got {both:?}")
    };
    assert_eq!(two.len(), 2, "both crossings are entries here: {both:?}");

    let one = at(0.3).expect_err("neither crossing takes r = 0.4");
    let PathError::NoCornerOfPair { corners: just, .. } = &one else {
        panic!("expected the envelope, got {one:?}")
    };
    assert_eq!(
        just.len(),
        1,
        "the second crossing is discarded by the windows, not listed: {one:?}"
    );

    // The surviving entry is one of the two crossings the first
    // authoring named, and the OTHER one — a real derived corner of
    // this pair — is reported by nothing.
    let kept = just[0].at;
    let same = |a: Point2<f64>, b: Point2<f64>| (a.x - b.x).hypot(a.y - b.y) < 1e-9;
    let dropped = if same(kept, two[0].at) {
        two[1].at
    } else {
        assert!(
            same(kept, two[1].at),
            "the entry is one of the two crossings: {kept:?} vs {two:?}"
        );
        two[0].at
    };
    assert!(
        !same(kept, dropped),
        "two distinct crossings: {kept:?} / {dropped:?}"
    );
    let rendered = one.to_string();
    assert_eq!(
        rendered.matches("at the corner near").count(),
        1,
        "one sentence, for one of the two crossings: {rendered}"
    );
}

/// **The one payload number that changed house.**
///
/// `FilletEnclosesLegCarrier` carried `radius`; `CornerReason::
/// EnclosesLegCarrier` does not, and the envelope carries it instead.
/// `review_s2.rs` reads the envelope's `radius` on its enclosing rows,
/// but nothing pins that the enclosing entry and the envelope radius
/// agree with the AUTHORED radius on a two-entry envelope, which is
/// where a header radius belonging to the wrong entry would show.
#[test]
fn c3_the_envelope_radius_is_the_authored_one_on_every_arm() {
    for r in [0.25, 0.3, 0.35, 0.4] {
        let err =
            arc_arc(0.0, 0.2, -1.0, 0.3, 1.2, 0.2, -1.0, 0.3, r).expect_err("r swallows a carrier");
        let PathError::NoCornerOfPair { radius, corners } = &err else {
            panic!("expected the envelope, got {err:?}")
        };
        assert_eq!(*radius, r, "the envelope renamed the radius");
        assert!(
            corners
                .iter()
                .any(|c| matches!(c.reason, CornerReason::EnclosesLegCarrier { .. })),
            "r = {r} must reach the enclosing arm: {err:?}"
        );
        // Every entry's sentence is rendered under this one radius.
        let rendered = err.to_string();
        assert_eq!(
            rendered.matches("at the corner near").count(),
            corners.len(),
            "one sentence per entry: {rendered}"
        );
    }
}
