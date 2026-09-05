//! **A fillet refusal about a carrier PAIR names every crossing it
//! tried** (issue 1281's attribution rule, ruled by Ev on PR 1734).
//!
//! The arc-carrier resolve derives 0, 1 or 2 corners from the pair,
//! runs the ratified construction at each that the anchor windows
//! admit, and — when none of them takes the fillet — reports
//! [`PathError::NoCornerOfPair`]: one entry per corner that refused at
//! the stage the answer comes from, each carrying that corner's own
//! reason and its own point, ordered by distance to the two bracketing
//! anchors.
//!
//! What these rows hold, and what they deliberately do not:
//!
//! - every refusing crossing is reported, with ITS reason — not the
//!   first one enumerated (the defect);
//! - a crossing the windows discarded is NOT listed beside a crossing
//!   that reached the construction — the spec's acceptance row asks for
//!   a one-entry envelope where only one crossing sits in the windows,
//!   so the envelope adds no noise;
//! - a refusal that names NO corner — the pair-level conditions, the
//!   M8 conditioning gate — outranks the envelope and reaches the
//!   caller as itself;
//! - the ORDER is presentation. The rows pin that it is a function of
//!   the anchors and that it is stable across repeats; nothing pins
//!   that a caller may read meaning into which entry came first.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::path::{CornerReason, CornerWindow};
use profile::{ArcSweep, Center, Open, PathError, ProfileLoop, Start};

use crate::common;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn on_circle(center: Point2<f64>, r: f64, angle: f64) -> Point2<f64> {
    p2(center.x + r * angle.cos(), center.y + r * angle.sin())
}

/// An arc x arc corner authored through the public lattice door, the
/// way an outside consumer writes one: the corner sits at the origin,
/// each carrier has radius `r_c` and winds `tau` (+1 ccw) with the
/// corner at angle `a` about its centre, and each far anchor runs
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

/// The pair whose two crossings refuse for DIFFERENT reasons: equal
/// R = 0.2 carriers, both clockwise, corner at the origin with the
/// second carrier's corner angle 1.2 rad, both anchors 0.3 rad from the
/// origin corner. At r = 0.25 the bracketed crossing's trim eats an
/// anchor while the far one demands the enclosing tangency.
fn both_crossings_refuse(r: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    arc_arc(0.0, 0.2, -1.0, 0.3, 1.2, 0.2, -1.0, 0.3, r)
}

/// The same carriers with the OUTGOING anchor free: at `dout = 0.25`
/// the origin crossing is nearest the anchors, at `dout = 0.5` the far
/// one is. Both crossings refuse on both settings.
fn anchors_moved(dout: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    arc_arc(0.0, 0.2, 1.0, 2.8, 0.8, 0.2, -1.0, dout, 0.25)
}

/// The sum of the distances from `at` to two anchors — the order the
/// envelope promises, recomputed here from the reported points so the
/// row checks the RULE and not the implementation's own key.
fn span(at: Point2<f64>, a: Point2<f64>, b: Point2<f64>) -> f64 {
    (at.x - a.x).hypot(at.y - a.y) + (at.x - b.x).hypot(at.y - b.y)
}

/// **C1: no corner is reported alone when another refused.**
///
/// Both crossings of this pair reach the construction and both refuse,
/// for different reasons and 0.33 m apart. The merge base reported one
/// of them — whichever `derive` enumerated first — and its sentence's
/// "these carriers" could not say which. Both are reported now, each
/// with its own reason and its own point.
#[test]
fn both_refusing_crossings_are_reported_each_with_its_own_reason() {
    let err = both_crossings_refuse(0.25).expect_err("neither crossing takes r = 0.25");
    let corners = common::corners(&err);
    assert_eq!(corners.len(), 2, "both crossings refuse: {err:?}");

    // Different reasons, which is the case the list exists for.
    assert!(
        matches!(
            corners[0].reason,
            CornerReason::AnchorOutsideTrimmedExtent { .. }
        ),
        "the bracketed crossing's trim eats an anchor: {err:?}"
    );
    assert!(
        matches!(corners[1].reason, CornerReason::EnclosesLegCarrier { .. }),
        "the far crossing demands the enclosing tangency: {err:?}"
    );

    // Two DIFFERENT points, far enough apart that no reader could take
    // one sentence's numbers for the other's.
    let (a, b) = (corners[0].at, corners[1].at);
    let apart = (a.x - b.x).hypot(a.y - b.y);
    assert!(apart > 0.3, "the two crossings are {apart} m apart");

    // The first entry is the one the anchors bracket: both anchors sit
    // 0.3 rad from the origin crossing.
    assert!(
        a.x.hypot(a.y) < 1e-12,
        "the anchors bracket the origin crossing, and it is reported first: {a:?}"
    );

    // The sentence says which corner each reason is about.
    let rendered = err.to_string();
    assert!(
        rendered.contains("no corner of these carriers takes a radius-0.25 m fillet"),
        "the header names the radius: {rendered}"
    );
    assert_eq!(
        rendered.matches("at the corner near").count(),
        2,
        "one sentence per crossing: {rendered}"
    );
    assert!(
        rendered.contains("SWALLOW") && rendered.contains("would eat"),
        "each entry keeps its own words: {rendered}"
    );
}

/// **The other half of the claim: no noise.**
///
/// Where only ONE crossing reaches the construction — the other having
/// been discarded by the anchor windows — the envelope carries that one
/// entry. Reporting the discarded crossing beside it would answer a
/// question the author did not ask; what keeps it out is the spec's own
/// acceptance row ("only one crossing sits in the windows → one
/// entry"), not the fence — merging the two channels would re-rank no
/// gate, since nothing branches on entry order and both channels yield
/// the same variant.
#[test]
fn a_crossing_the_windows_discarded_is_not_listed_beside_the_answer() {
    // A straight incoming ray whose origin sits between the two
    // crossings of the ray with the R = 2 carrier: the crossing behind
    // it is discarded by the advance gate, the one ahead is reached and
    // its trim eats the ray's own origin.
    let err = Open
        .at(p2(1.9, 0.0))
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
        .expect_err("the short straight side must refuse");
    let corners = common::corners(&err);
    assert_eq!(
        corners.len(),
        1,
        "only the crossing that reached the construction is reported: {err:?}"
    );
    assert!(
        corners[0].at.x > 0.0,
        "the reported crossing is the one ahead of the ray's origin: {:?}",
        corners[0].at
    );
    assert!(
        common::anchor_fit(&err).is_some(),
        "and it refused on the anchor fit: {err:?}"
    );
}

/// **Every `CornerReason` arm is reachable through the public door.**
///
/// One authored profile per arm, no hand-built payloads: an arm nothing
/// can reach is an FFI word and a Display sentence with no producer
/// behind them.
#[test]
fn every_corner_reason_arm_is_reachable_from_an_authored_profile() {
    // (1) OutsideAnchors(BehindIncomingRay): the incoming ray STARTS at
    // the derived corner, so no corner is ahead of the side authored.
    let behind_ray = Open
        .at(p2(2.0, 0.0))
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
        .expect_err("a zero-extent incoming side must refuse");
    assert!(
        common::any_reason(&behind_ray, |r| matches!(
            r,
            CornerReason::OutsideAnchors(CornerWindow::BehindIncomingRay)
        )),
        "{behind_ray:?}"
    );

    // (2) OutsideAnchors(BehindArrivalAnchor): the straight arrival's
    // anchor sits BEFORE the corner in its own travel sense, so the
    // arrival ray never came from the corner.
    let behind_anchor = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(0.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(3.0, -1.0), Tol::witness())
        .expect_err("an arrival anchor before the corner must refuse");
    assert!(
        common::any_reason(&behind_anchor, |r| matches!(
            r,
            CornerReason::OutsideAnchors(CornerWindow::BehindArrivalAnchor)
        )),
        "{behind_anchor:?}"
    );

    // (3) NoTangentCircle: a radius too large for the corner — the
    // offset carriers no longer meet, so no circle of that radius is
    // tangent to both.
    let no_circle = Open
        .at(p2(-3.0, 1.5))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            1.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("no tangent circle of r = 1.5 exists at this corner");
    assert!(
        common::any_reason(&no_circle, |r| matches!(
            r,
            CornerReason::NoTangentCircle(_)
        )),
        "{no_circle:?}"
    );

    // (4) AnchorOutsideTrimmedExtent: a straight pair whose arrival leg
    // is shorter than the setback.
    let eats_anchor = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet(2.5, Tol::witness())
        .unwrap()
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(3.0, 2.0), Tol::witness())
        .expect_err("the setback outruns the arrival leg");
    assert!(
        common::anchor_fit(&eats_anchor).is_some(),
        "{eats_anchor:?}"
    );

    // (5) EnclosesLegCarrier: a radius that swallows both carriers.
    let swallows = both_crossings_refuse(0.25).expect_err("r = 0.25 swallows the far crossing");
    assert!(common::is_enclosing(&swallows), "{swallows:?}");
}

/// **The order is a function of the anchors, and the envelope is
/// deterministic.**
///
/// Two calls on the same authoring produce the same envelope, entry for
/// entry (D2's replay contract reaches the refusal channel too); and
/// moving the outgoing anchor past the midpoint between the crossings
/// flips which entry is reported first, without changing either entry's
/// reason or point. The order is presentation — the SET is what the
/// refusal claims.
#[test]
fn the_envelope_replays_and_its_order_follows_the_anchors() {
    let first = both_crossings_refuse(0.25).unwrap_err();
    let again = both_crossings_refuse(0.25).unwrap_err();
    assert_eq!(
        format!("{first:?}"),
        format!("{again:?}"),
        "two calls, one envelope"
    );

    // The anchors move; the crossings do not.
    let near = anchors_moved(0.25).expect_err("neither crossing takes r = 0.25");
    let far = anchors_moved(0.5).expect_err("neither crossing takes r = 0.25");
    let (n, f) = (common::corners(&near), common::corners(&far));
    assert_eq!((n.len(), f.len()), (2, 2), "{near:?} / {far:?}");

    let flipped = (n[0].at.x - f[0].at.x).abs() + (n[0].at.y - f[0].at.y).abs();
    assert!(
        flipped > 1e-9,
        "moving the outgoing anchor must change which crossing reads first: {near} / {far}"
    );

    // Both orders ARE the stated rule: nearest the two bracketing
    // anchors first. The anchors are the incoming ray's own origin and
    // the arrival's anchor; both are on-path points of the authored
    // profile, so the row recomputes the rule from the entries rather
    // than trusting the sort.
    for corners in [n, f] {
        let (a, b) = (corners[0].at, corners[1].at);
        // The two crossings are the same pair of points in both runs,
        // so the anchors are what re-ordered them: whichever entry is
        // first, its span to the run's own anchors is the smaller one.
        assert!(
            (a.x - b.x).abs() + (a.y - b.y).abs() > 1e-9,
            "two distinct crossings"
        );
    }
    // The rule itself, checked against the anchors the authoring names.
    let (in_anchor, out_near, out_far) = anchors_of();
    assert!(
        span(n[0].at, in_anchor, out_near) <= span(n[1].at, in_anchor, out_near),
        "the near-anchor run reports the nearer crossing first"
    );
    assert!(
        span(f[0].at, in_anchor, out_far) <= span(f[1].at, in_anchor, out_far),
        "the far-anchor run reports the nearer crossing first"
    );
}

/// **C4 at EVERY band: the M8 conditioning gate ABORTS the resolve, and
/// the twin corner's build never masks it.**
///
/// `review_s2`'s pin of the same gate is ε-keyed to 1e-12 — at the
/// other two bands its mined geometry refuses one gate earlier — so a
/// mutant that demotes the abort to the whole-pair slot stays green
/// there at the default band and at 1e-6. This row closes that: it
/// authors a rung where the gate fires AND the pair's other crossing
/// would build, so what the abort buys is exactly what the row sees.
///
/// **Why the far carrier is scaled by ε.** The least lever the band
/// supports goes as 1/ε, while whether the twin crossing builds moves
/// with the scene's own scale; the rung where both hold at once
/// therefore MOVES with the band, and a fixed one would pin the abort
/// at one ε and nothing at the others — the defect this row exists to
/// remove. Scaling the far carrier with ε keeps the same rung at every
/// band: measured, the gate fires at 1e-6, 1e-9 and 1e-12 alike, and
/// demoting the `return` to the whole-pair slot silently BUILDS at all
/// three.
///
/// The assertion is the bare variant — not an envelope, not an entry —
/// because a corner whose tangent point the band cannot certify is a
/// fact about the run's conditioning, not about one crossing of a
/// pair.
#[test]
fn the_offset_lever_gate_aborts_the_resolve_at_every_band() {
    let far = 1.0e4 * (Tol::witness().eps() / 1.0e-9);
    let err = arc_arc(0.0, far, 1.0, 1.0, 3.0, 1.0, -1.0, 1.0, 0.5)
        .expect_err("a collapsed offset lever must refuse at every band");
    let PathError::FilletOffsetLeverTooShort {
        side,
        carrier_radius,
        offset_radius,
        least_lever,
        margin,
    } = err
    else {
        panic!(
            "the conditioning gate must reach the caller as itself, never as an envelope \
             entry and never behind the twin corner's build: {err:?}"
        )
    };
    assert_eq!(
        side,
        profile::FilletLeg::Outgoing,
        "the gate measures the outgoing leg's offset lever"
    );
    assert!(
        (carrier_radius - 1.0).abs() < 0.1,
        "the exposed carrier is the unit one, got {carrier_radius}"
    );
    assert!(
        offset_radius > 0.0,
        "rho is positive here — this is the conditioning gate, not the enclosing class \
         (rho {offset_radius})"
    );
    assert!(
        least_lever > offset_radius && margin < 0.0,
        "the gate fires because the lever is under the least the band supports \
         (rho {offset_radius}, least {least_lever}, margin {margin})"
    );
}

/// The three anchor points [`anchors_moved`] authors: the incoming
/// ray's own anchor (shared) and the two outgoing anchors.
fn anchors_of() -> (Point2<f64>, Point2<f64>, Point2<f64>) {
    let c1 = p2(-0.2, 0.0);
    let a_out = 0.8_f64;
    let c2 = p2(-0.2 * a_out.cos(), -0.2 * a_out.sin());
    (
        on_circle(c1, 0.2, -2.8),
        on_circle(c2, 0.2, a_out - 0.25),
        on_circle(c2, 0.2, a_out - 0.5),
    )
}
