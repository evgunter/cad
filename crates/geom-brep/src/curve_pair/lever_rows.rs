//! **The router's levers are the spans' reaches from the point it reads
//! its offset at** (the pose's module docs, "The router"): a tilt whose
//! Zero is level where the pair can meet is levered by the nearer span's
//! reach from above, and one whose definite sign is a crossing by the
//! farther span's from below. The rows read the tilt's verdicts off the
//! recorder by the read's name, never by their order.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::Curve3;
use geom_core::k_stats::Bracket;
use geom_core::{Point3, Sign};

use super::meet_rows::{band, circle, ellipse, line, over_an_arcs_far_side};
use super::{MeetFault, Meetings, Span, meet};

const TILT: &str = "curve_pair_pose_tilt";

/// `meet(a, b, band)` and the signs its tilt reads decided.
fn meet_and_tilts<'a>(
    a: &Span<'a, f64>,
    b: &Span<'a, f64>,
    band: geom_core::Band,
) -> (Result<Meetings<f64>, super::MeetRefusal<f64>>, Vec<Sign>) {
    let bracket = Bracket::open();
    let got = meet(a, b, band);
    let tilts = bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate == TILT)
        .map(|v| v.sign)
        .collect();
    (got, tilts)
}

/// A span's reach from `at`, from below and from above, as the router
/// reads it.
fn reach(carrier: &Curve3<f64>, (t0, t1): (f64, f64), at: Point3<f64>) -> (f64, f64) {
    let cover = carrier
        .cover_over(geom::cover::ParamWindow::new(t0, t1))
        .ok();
    let above = crate::ExtentBall::of_cover_from(cover.as_ref(), at).lever();
    let bounds = crate::edge_reach_bounds(carrier, (t0, t1), at, above);
    (bounds.below().length(), bounds.above().length())
}

/// **Two lines end to end at a shallow corner read their tilt over the
/// spans, not over the projected overlap.** The unit segment along `x̂`
/// and a unit segment leaving its end tilted `10⁻⁶`: their projections
/// share one point, so a lever read off the projected overlap is the
/// band's width and reads the tilt Zero. Each span reaches a metre from
/// that point, where the tilt is `10⁻⁶`, past the band. Red where the
/// level tilt is levered by the overlap's projected length.
#[test]
fn a_shallow_corners_tilt_is_levered_by_the_spans() {
    let t = 1e-6;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([1.0, 0.0, 0.0], [1.0, t, 0.0]);
    let (got, tilts) = meet_and_tilts(
        &Span::new(&a, (0.0, 1.0)),
        &Span::new(&b, (0.0, 1.0)),
        band(),
    );
    assert!(
        !tilts.is_empty() && tilts.iter().all(|&s| s == Sign::Positive),
        "every tilt read is definite over a metre: {tilts:?}, {got:?}"
    );
    let met = got.unwrap();
    assert!(
        met.points.len() == 1 && met.points[0].point.distance(Point3::new(1.0, 0.0, 0.0)) < 1e-9,
        "the corner is the one meeting: {met:?}"
    );
}

/// **Two segments crossing at their middles, within the band of each
/// other everywhere, are levered at their reach from the crossing, not
/// their diameter** (the review of the diameter levers). Unit
/// half-length segments tilted `0.9·10⁻⁹` apart under the band
/// `(10⁻⁹, 1.5·10⁻⁹)`: each reaches 1 from their common middle, where
/// the tilt reads `0.9·10⁻⁹`, Zero, and the pair is level. At the
/// diameter, 2, it would read `1.8·10⁻⁹`, definite, and pose a crossing
/// of two segments the band reads as one. Red where a lever is the
/// diameter.
#[test]
fn segments_within_the_band_everywhere_are_levered_at_their_reach() {
    let band = geom_core::Band::new(1e-9, 1.5e-9).unwrap();
    let s: f64 = 0.9e-9;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([0.0, 0.0, 0.0], [s.cos(), s.sin(), 0.0]);
    let (got, tilts) = meet_and_tilts(
        &Span::new(&a, (-1.0, 1.0)),
        &Span::new(&b, (-1.0, 1.0)),
        band,
    );
    assert!(
        !tilts.is_empty() && tilts.iter().all(|&s| s == Sign::Zero),
        "every tilt read is Zero at the reach: {tilts:?}, {got:?}"
    );
    assert!(
        got.as_ref().map_or(true, |m| m.shared.is_some()),
        "no crossing is posed: {got:?}"
    );
}

/// **The level reading takes the nearer span, the crossing the
/// farther.** Two lines crossing at the origin, one reaching half a
/// metre from it and the other ten, tilted `1.5·10⁻⁹` under the band
/// `(10⁻⁹, 10⁻⁸)`: over the nearer reach the tilt reads Zero, so the
/// pair is level where it can meet; the farther's ends then stand past
/// the band off the other line, and the farther reach reads a crossing
/// (whose stretch then escalates, a crossing this shallow). Red where
/// the two reaches are swapped.
#[test]
fn the_level_tilt_takes_the_nearer_span_and_the_crossing_the_farther() {
    let t: f64 = 1.5e-9;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([0.0, 0.0, 0.0], [t.cos(), t.sin(), 0.0]);
    let (got, tilts) = meet_and_tilts(
        &Span::new(&a, (-0.5, 0.5)),
        &Span::new(&b, (-10.0, 10.0)),
        band(),
    );
    assert!(
        tilts.contains(&Sign::Zero) && tilts.contains(&Sign::Positive),
        "Zero over the nearer reach, definite over the farther: {tilts:?}, {got:?}"
    );
    match got {
        Ok(met) => assert!(
            met.points.len() == 1 && met.points[0].point.distance(Point3::origin()) < 1e-9,
            "the crossing at the origin: {met:?}"
        ),
        Err(r) => assert!(
            matches!(r.fault, MeetFault::Escalated(_)),
            "a crossing this shallow may only escalate: {r:?}"
        ),
    }
}

/// **A line against a short arc of a large circle levers its tilt by
/// the two spans, never by the circle's size.** The arc is 10⁻² rad of a
/// radius-100 circle in `z = 0` (a chord of about a metre) and the line
/// a metre through its middle, tilted `3·10⁻⁹` out of the plane: in band
/// over either span's reach, definite over the circle's 100 m
/// semi-axis. So the tilt is in band and escalates. Red where the
/// crossing's lever is the carrier's semi-axis.
#[test]
fn a_line_on_a_short_arc_levers_its_tilt_by_the_spans() {
    let k = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 100.0);
    let l = line([100.0, 0.0, 0.0], [0.0, 1.0, 3e-9]);
    let arc = Span::new(&k, (-0.005, 0.005));
    let seg = Span::new(&l, (-0.5, 0.5));
    for (what, got) in [
        ("line first", meet(&seg, &arc, band())),
        ("arc first", meet(&arc, &seg, band())),
    ] {
        match got {
            Err(r) => assert!(
                matches!(r.fault, MeetFault::Escalated(d) if d.predicate == Some(TILT)),
                "{what}: the tilt escalates: {r:?}"
            ),
            Ok(m) => panic!("{what}: a tilt in band over both spans decided: {m:?}"),
        }
    }
}

/// **A segment over a short arc's far side, inside the zero band,
/// meets it at the arc's middle** ([`over_an_arcs_far_side`] at
/// `d = 1.15r`, `ℓ = 1.3r`, the band `(10⁻⁹, 1.05·10⁻⁹)`): the segment
/// passes `9.29·10⁻¹⁰` m over the arc's middle. Where the pair can meet
/// lies on the arc, whose reach reads the tilt Zero, so the pair is
/// level there and its in-plane reading finds the meeting. A lever read
/// off the conic's whole projection on the line, `2r` and the band's
/// width, reads the tilt definite and refuses the crossing it then
/// polishes. Red where the level tilt is levered by the projected
/// overlap.
#[test]
fn a_segment_over_a_short_arcs_far_side_meets_it_at_its_middle() {
    let r = 1e-4;
    let band = geom_core::Band::new(1e-9, 1.05e-9).unwrap();
    let middle = Point3::new(-r, 0.0, 0.0);
    for (order, got) in ["segment first", "arc first"]
        .into_iter()
        .zip(over_an_arcs_far_side((1.15 * r, 1.3 * r), band))
    {
        let met = got.unwrap_or_else(|e| panic!("{order}: the meeting refused: {e:?}"));
        assert!(
            met.points.len() == 1 && met.points[0].point.distance(middle) <= band.zero(),
            "{order}: one meeting, within the zero band of the arc's middle: {met:?}"
        );
    }
}

/// **Where the pair can meet is bounded from above.** A hundred metres
/// of line through the middle of an ellipse arc, tilted out of its plane
/// so that the tilt levered by the arc's reach from that point reads
/// Zero from below and in band from above: the level reading, which
/// acts on its Zero, must not read Zero there. Red where the nearer
/// span's reach is read from below.
#[test]
fn the_level_tilt_reads_the_nearer_span_from_above() {
    let k = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 1.0);
    let (t0, t1) = (0.2, 2.9);
    let middle = k.eval(0.5 * (t0 + t1));
    let (below, above) = reach(&k, (t0, t1), middle);
    let z = band().zero();
    let tilt = 2.0 * z / (below + above);
    assert!(
        tilt * below < z && tilt * above > z && tilt * above < band().escalate(),
        "the premise: the arc's reach bounds {below}, {above} straddle the zero"
    );
    let l = line(middle.to_array(), [1.0, 0.0, tilt]);
    let (got, tilts) = meet_and_tilts(
        &Span::new(&l, (-50.0, 50.0)),
        &Span::new(&k, (t0, t1)),
        band(),
    );
    assert!(
        !tilts.contains(&Sign::Zero),
        "no tilt read is Zero where the arc's reach straddles it: {tilts:?}, {got:?}"
    );
}

/// **A crossing is claimed over the farther span from below.** A metre
/// of line through the middle of an ellipse arc that reaches farther
/// from that point, tilted so that the tilt levered by the arc's reach
/// is in band from below and definite from above: no crossing is
/// claimed, and the tilt escalates. Red where the farther span's reach
/// is read from above.
#[test]
fn a_crossing_reads_the_farther_span_from_below() {
    let k = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 1.0);
    let (t0, t1) = (0.2, 2.9);
    let middle = k.eval(0.5 * (t0 + t1));
    let (below, above) = reach(&k, (t0, t1), middle);
    let e = band().escalate();
    let tilt = 2.0 * e / (below + above);
    assert!(
        tilt * below < e && tilt * above > e && tilt * 0.5 > band().zero(),
        "the premise: the arc's reach bounds {below}, {above} straddle the escalation"
    );
    let l = line(middle.to_array(), [1.0, 0.0, tilt]);
    let got = meet(
        &Span::new(&l, (-0.5, 0.5)),
        &Span::new(&k, (t0, t1)),
        band(),
    );
    assert!(
        matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(d) if d.predicate == Some(TILT))),
        "the tilt escalates over the arc: {got:?}"
    );
}

/// **The spans' reaches are read from the overlap's middle, not from
/// each span's own.** A line along `x̂` over `[−5, 1]` and a half-metre
/// line crossing it at the origin, tilted `2.5·10⁻⁹`, under the band
/// `(10⁻⁹, 10⁻⁸)`. The overlap's middle is the origin, from which the
/// long span reaches 5 and the tilt reads `1.25·10⁻⁸`, a definite
/// crossing. From the long span's own middle, `x = −2`, it reaches 3,
/// where the tilt reads `7.5·10⁻⁹`, in band, and escalates. Red where a
/// span's reach is read from its own middle.
#[test]
fn the_reaches_are_read_from_the_overlaps_middle() {
    let t: f64 = 2.5e-9;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([0.0, 0.0, 0.0], [t.cos(), t.sin(), 0.0]);
    for (order, (got, tilts)) in [
        (
            "long first",
            meet_and_tilts(
                &Span::new(&a, (-5.0, 1.0)),
                &Span::new(&b, (-0.5, 0.5)),
                band(),
            ),
        ),
        (
            "short first",
            meet_and_tilts(
                &Span::new(&b, (-0.5, 0.5)),
                &Span::new(&a, (-5.0, 1.0)),
                band(),
            ),
        ),
    ] {
        assert!(
            tilts.contains(&Sign::Positive),
            "{order}: the tilt reads a crossing over the long span's reach from the origin: {tilts:?}, {got:?}"
        );
        assert!(
            !matches!(&got, Err(r) if matches!(r.fault, MeetFault::Escalated(d) if d.predicate == Some(TILT))),
            "{order}: the tilt does not escalate: {got:?}"
        );
    }
}
