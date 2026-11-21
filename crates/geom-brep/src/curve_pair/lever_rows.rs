//! **The router's levers are the spans' diameters** (the pose's module
//! docs, "The router"): a tilt whose Zero is level where the pair can
//! meet is levered by the shorter span's diameter from above, and one
//! whose definite sign is a crossing by the longer span's from below.
//! Each row reads the tilt's verdicts off the recorder.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Sign;
use geom_core::k_stats::Bracket;

use super::meet_rows::{band, circle, ellipse, line, over_an_arcs_far_side};
use super::{MeetFault, Span, meet};

const TILT: &str = "curve_pair_pose_tilt";

/// **Two lines end to end at a shallow corner read their tilt over the
/// shorter span.** The unit segment along `x̂` and a unit segment
/// leaving its end tilted `10⁻⁶`: their projections share one point, so
/// a lever read off the projected overlap is the band's width and reads
/// the tilt Zero there. Over the shorter span's diameter (a metre) the
/// tilt is `10⁻⁶`, past the band. Red where the tilt that routes level
/// is levered by the overlap's projected length.
#[test]
fn a_shallow_corners_tilt_is_levered_by_the_shorter_span() {
    let t = 1e-6;
    let a = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let b = line([1.0, 0.0, 0.0], [1.0, t, 0.0]);
    let bracket = Bracket::open();
    let got = meet(
        &Span::new(&a, (0.0, 1.0)),
        &Span::new(&b, (0.0, 1.0)),
        band(),
    );
    let recorded = bracket.finish();
    let first = recorded.verdicts.iter().find(|v| v.predicate == TILT);
    assert!(
        matches!(first.map(|v| v.sign), Some(Sign::Positive)),
        "the first tilt reading is definite over a metre: {first:?}, {got:?}"
    );
    let met = got.unwrap();
    assert!(
        met.points.len() == 1
            && met.points[0]
                .point
                .distance(geom_core::Point3::new(1.0, 0.0, 0.0))
                < 1e-9,
        "the corner is the one meeting: {met:?}"
    );
}

/// **A line against a short arc of a large circle levers its tilt by
/// the two spans, never by the circle's size.** The arc is 10⁻² rad of a
/// radius-100 circle in `z = 0` (a chord of about a metre) and the line
/// a metre through its middle, tilted `3·10⁻⁹` out of the plane: in band
/// over a metre, definite over the circle's 100 m semi-axis. A crossing
/// claimed at the semi-axis is a reading of no span the pair holds, so
/// the tilt is in band and escalates. Red where the crossing's lever is
/// the carrier's semi-axis.
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
/// lies within the arc, whose diameter (`2r·sin 0.3`) reads the tilt
/// Zero, so the pair is level there and its in-plane reading finds the
/// meeting. A lever read off the conic's whole projection on the line,
/// `2r` and the band's width, reads the tilt definite and refuses the
/// crossing it then polishes. Red where the level tilt is levered by
/// the projected overlap.
#[test]
fn a_segment_over_a_short_arcs_far_side_meets_it_at_its_middle() {
    let r = 1e-4;
    let band = geom_core::Band::new(1e-9, 1.05e-9).unwrap();
    let middle = geom_core::Point3::new(-r, 0.0, 0.0);
    for (order, got) in ["segment first", "arc first"]
        .into_iter()
        .zip(over_an_arcs_far_side((1.15 * r, 1.3 * r), band))
    {
        let met = got.unwrap_or_else(|e| panic!("{order}: the meeting refused: {e:?}"));
        assert!(
            met.points.len() == 1 && met.points[0].point.distance(middle) < 1e-12,
            "{order}: one meeting, at the arc's middle: {met:?}"
        );
    }
}

/// **Where the pair can meet is bounded from above.** A line through
/// the middle of an ellipse arc, tilted out of its plane so that the
/// tilt levered by the arc's diameter from below reads Zero and from
/// above reads in band: the arc's two bounds straddle the band's zero,
/// and the level reading, which acts on its Zero, must not read Zero
/// there. The line, a hundred metres, then reads the crossing. Red where the
/// shorter span's diameter is read from below.
#[test]
fn the_level_tilt_reads_the_shorter_span_from_above() {
    let k = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 1.0);
    let (t0, t1) = (0.2, 2.9);
    let (inf, sup) = (
        crate::edge_extent_inf(&k, t0, t1).length(),
        crate::edge_extent_sup(&k, t0, t1).length(),
    );
    let z = band().zero();
    let tilt = 2.0 * z / (inf + sup);
    assert!(
        tilt * inf < z && tilt * sup > z && tilt * sup < band().escalate(),
        "the premise: the arc's bounds {inf}, {sup} straddle the zero"
    );
    let middle = k.eval(0.5 * (t0 + t1));
    let l = line(middle.to_array(), [1.0, 0.0, tilt]);
    let bracket = Bracket::open();
    let got = meet(
        &Span::new(&l, (-50.0, 50.0)),
        &Span::new(&k, (t0, t1)),
        band(),
    );
    let recorded = bracket.finish();
    let first = recorded.verdicts.iter().find(|v| v.predicate == TILT);
    assert!(
        matches!(first.map(|v| v.sign), Some(Sign::Positive)),
        "the level reading is not Zero, and the longer span reads the crossing: {first:?}, {got:?}"
    );
}

/// **A crossing is claimed over the longer span from below.** A metre
/// of line through the middle of a longer ellipse arc, tilted so that
/// the tilt levered by the arc's diameter from below is in band and
/// from above is definite: no crossing is claimed, and the tilt
/// escalates. Red where the longer span's diameter is read from above.
#[test]
fn a_crossing_reads_the_longer_span_from_below() {
    let k = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 2.0, 1.0);
    let (t0, t1) = (0.2, 2.9);
    let (inf, sup) = (
        crate::edge_extent_inf(&k, t0, t1).length(),
        crate::edge_extent_sup(&k, t0, t1).length(),
    );
    let e = band().escalate();
    let tilt = 2.0 * e / (inf + sup);
    assert!(
        tilt * inf < e && tilt * sup > e && tilt > band().zero(),
        "the premise: the arc's bounds {inf}, {sup} straddle the escalation"
    );
    let middle = k.eval(0.5 * (t0 + t1));
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
