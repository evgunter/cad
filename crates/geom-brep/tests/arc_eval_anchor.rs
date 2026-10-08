//! The arc evaluation's anchoring, pinned at the certified scalar.
//!
//! `SketchSegment::eval` evaluates `a + (R − I)·v` rather than
//! `center + R·v`. The two are the same point over the reals, so no
//! f64 row can tell them apart; what separates them is the enclosure
//! at `T = Interval`. The center-anchored form mentions the center
//! twice and interval arithmetic cannot cancel it, so the result
//! carries `2·width(center)`. `restrict` re-derives its endpoints
//! through `eval`, so an evaluation's width is stored back into the
//! description and successive splits compound it.
//!
//! The row below is the guard: on a short arc whose centre is derived
//! from its own short chord (so the centre is wide), the
//! evaluated point's enclosure must stay at the scale of the sub-arc's
//! own endpoint data. Under the center-anchored form the same fixture
//! is two orders wider than that and the row is red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::arc::lowered_arc;
use crate::shared::interval::iv;
use geom_brep::SketchSegment;
use geom_core::{Bounds, Interval, Point2, Real};

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

/// The width of an enclosure, and (below) the widest coordinate of a
/// point's.
///
/// **Deliberately not shared with `review_arceval_r1_probes.rs`'s
/// pair**, which is this text: that suite is R1's independent
/// re-derivation of this file's anchor comparison, and the METER it
/// measures widths with is the thing its verdict turns on.
fn width(x: Interval) -> f64 {
    x.hi() - x.lo()
}

fn point_width(p: Point2<Interval>) -> f64 {
    width(p.x).max(width(p.y))
}

/// The pip meridian's shape: a semicircle of radius 0.09 through the
/// sketch origin's axis, which is what a revolve profile hands to the
/// splitters, lowered from its unit bulge. Its apothem is exactly zero
/// and its center is exactly the chord midpoint.
fn meridian() -> SketchSegment<Interval> {
    lowered_arc(p2(0.0, -0.09), p2(0.0, 0.09), iv(1.0))
}

/// A short arc whose carrier is derived from its own chord: the 0.4 %
/// window of the meridian from `s = 0.4`, its endpoints cut by
/// `restrict` (so they carry an evaluation's width), and its carrier
/// lowered from that short chord and the window's bulge
/// `tan(atan(1)·0.004)` — how the profile's lift derives a short
/// authored arc's carrier at `Interval`. (`restrict` itself keeps the
/// parent's carrier, whose centre here is exact.)
fn short_arc() -> SketchSegment<Interval> {
    let SketchSegment::Arc { a, b, .. } = meridian().restrict(iv(0.4), iv(0.404)) else {
        panic!("restriction changed the segment kind");
    };
    lowered_arc(a, b, (iv(1.0).atan() * (iv(0.404) - iv(0.4))).tan())
}

/// A short arc's evaluated points stay at the scale of its own
/// endpoint data, even where its centre is wide.
///
/// The window is 0.4 % of a semicircle — θ′ ≈ 0.0126 rad, chord
/// ≈ 1.1 mm against a 90 mm radius, so a reconstructed center carries
/// roughly `R/chord ≈ 80` times the chord's relative width and the
/// center-anchored form pays it twice. The bound here is 8× the
/// endpoint width: two orders below what that form produces, and
/// several times looser than what the anchored form delivers — a bound
/// with room on both sides, not a knife edge.
///
/// Sampled at both ends and mid-arc. The form is not symmetric in
/// `a`/`b` — `a` is the anchor — so s = 1 is the direction a
/// regression would take, and it is a certification schedule sample.
/// At s = 0 the result is `a` up to the interval `sin`'s outward
/// rounding at the exact point 0 (ulp-scale, decades under the
/// endpoint width); it is NOT bitwise `a`, so this row bounds the
/// width rather than pinning equality.
#[test]
fn a_short_arc_with_a_derived_centre_evaluates_at_its_endpoints_scale() {
    let sub = short_arc();
    let SketchSegment::Arc { a, b, .. } = sub else {
        panic!("the fixture is an arc");
    };
    let endpoint_width = point_width(a).max(point_width(b));
    assert!(
        endpoint_width > 0.0,
        "FIXTURE: the restricted endpoints must carry width, else the bound is vacuous"
    );

    // Sample 0 is where the certification schedule's mapped-source
    // check reads the description, and where the center-anchored form
    // is worst: the rotation is the identity there, so every bit of
    // the width is the center round trip.
    let at_start = sub.eval(iv(0.0));
    assert!(
        point_width(at_start) <= 8.0 * endpoint_width,
        "the sub-arc's start enclosure is {:e} wide against endpoint width {:e} — the \
         evaluation is reconstructing a center rather than anchoring on the endpoint",
        point_width(at_start),
        endpoint_width
    );

    // And the claim is about the whole short arc, not just its start:
    // mid-arc the rotation is real, but |R − I| = 2·|sin(sθ/2)| is
    // small on a short arc and scales the center's width DOWN. s = 1 is
    // the far end, away from the anchor, and a schedule sample.
    for s in [0.5, 1.0] {
        let at = sub.eval(iv(s));
        assert!(
            point_width(at) <= 8.0 * endpoint_width,
            "the sub-arc's enclosure at s = {s} is {:e} wide against endpoint width {:e}",
            point_width(at),
            endpoint_width
        );
    }
}
