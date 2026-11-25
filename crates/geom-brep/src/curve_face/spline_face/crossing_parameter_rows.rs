//! **A meeting's stored parameter decides nothing its box does not**
//! (the crate README of `geom-core`, DL5.2): what [`super::settle`]
//! answers on the boxes that hold one meeting, how wide
//! [`super::isolate`]'s box round the true meeting is on a shallow
//! crossing, at `f64` and at `Interval`, and that a leaf and its witness
//! place alike or the leaf refuses.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, NurbsSurface};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};

use super::{ChartMeeting, Isolated, SpanCarrier, SplineFaceRefusal, isolate, settle};

const ZERO: f64 = 1e-9;
const ESCALATE: f64 = 10.0 * ZERO;

fn band() -> Band {
    Band::new(ZERO, ESCALATE).unwrap()
}

/// The sine of the crossing angle: about 0.57°.
const SIN: f64 = 1e-2;

/// The true meeting's parameter on [`shallow`].
const T_STAR: f64 = 0.25;

/// A Bézier patch of bidegree `(p, p)` over `[0, 1]²`, a graph over the
/// `xy` square scaled by `size`, its control heights `z(i, j)`.
fn graph(p: usize, size: f64, z: impl Fn(usize, usize) -> f64) -> NurbsSurface<f64> {
    let ends = || {
        let mut k = vec![0.0; p + 1];
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    };
    #[allow(clippy::cast_precision_loss)]
    let control = (0..=p)
        .flat_map(|i| (0..=p).map(move |j| (i, j)))
        .map(|(i, j)| {
            Point3::new(
                size * i as f64 / p as f64,
                size * j as f64 / p as f64,
                z(i, j),
            )
        })
        .collect();
    NurbsSurface::new(ends(), ends(), control, vec![1.0; (p + 1) * (p + 1)]).unwrap()
}

/// The unit square in the plane `z = 0`.
fn flat() -> NurbsSurface<f64> {
    graph(1, 1.0, |_, _| 0.0)
}

fn dir() -> Vec3<f64> {
    Vec3::new((1.0 - SIN * SIN).sqrt(), 0.0, SIN)
}

/// A line by arc length rising through `z = 0` at [`SIN`], meeting it
/// at `t = T_STAR` over `(x, y) = (x(T_STAR), ½)`.
fn shallow() -> Curve3<f64> {
    let d = dir();
    Curve3::Line(geom::Line {
        origin: Point3::new(0.25, 0.5, 0.0) - d * T_STAR,
        dir: geom::stated_direction(d),
    })
}

/// A point box at parameter `t` of [`shallow`] over [`flat`]: the state
/// `(t, x(t), ½)`, whose chart point is the foot of the carrier's point.
fn point_cell(t: f64) -> [(f64, f64); 3] {
    let x = shallow().eval(t).x;
    [(t, t), (x, x), (0.5, 0.5)]
}

fn settled(
    span: (f64, f64),
    roots: Vec<[(f64, f64); 3]>,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    // The line is by arc length: its speed is one.
    let isolated = Isolated {
        span,
        speed: 1.0,
        roots,
    };
    settle(&shallow(), &flat(), &isolated, band())
}

/// The span whose stop end lies two escalation lengths past the true
/// meeting: the meeting is definitely apart from the end.
fn span() -> (f64, f64) {
    (0.0, T_STAR + 2.0 * ESCALATE)
}

/// How far `t`'s point on [`shallow`] reads from [`flat`] at its foot:
/// what `spline_face_root_on` decides.
fn off_chart(t: f64) -> f64 {
    let cell = point_cell(t);
    (shallow().eval(t) - flat().eval(cell[1].0, cell[2].0)).norm()
}

/// The box about the middle `t` just wide enough to hold the true
/// meeting, as a box the walk proved would.
fn holding_cell(t: f64) -> [(f64, f64); 3] {
    let half = (t - T_STAR).abs() + ZERO;
    let x = shallow().eval(t).x;
    [(t - half, t + half), (x, x), (0.5, 0.5)]
}

/// **`settle` answers one way on the boxes that hold one meeting, or
/// refuses.** The line crosses the chart at `SIN`, so every parameter
/// within `ZERO / SIN` (a hundred zero bands along the edge) of the true
/// meeting reads ON, and the true meeting lies two escalation lengths
/// short of the span's end. Four boxes hold it, each about another
/// middle: the true meeting; fifty zero bands before it; half a zero band
/// short of the end; five zero bands short of the end. Read on its
/// middle's point alone, each answered for itself: reported, reported at
/// the early parameter, dropped as the end's, escalated. Read with its
/// box's length as the reach, the narrow box about the true meeting is
/// reported, and the three whose reach holds the end refuse at the end's
/// row. A box wholly within the zero band of the end is the end's.
#[test]
fn settle_answers_one_way_or_refuses_on_the_boxes_that_hold_one_meeting() {
    let (a, b) = span();
    let early = T_STAR - 50.0 * ZERO;
    let at_end = b - 0.5 * ZERO;
    let in_gap = b - 5.0 * ZERO;
    for t in [T_STAR, early, at_end, in_gap] {
        assert!(
            off_chart(t) <= ZERO && a < t && t < b,
            "t = {t} is inside the span and reads ON: {} m off",
            off_chart(t)
        );
    }
    let got = settled((a, b), vec![holding_cell(T_STAR)]).unwrap();
    assert!(
        got.len() == 1 && got[0].t.to_bits() == T_STAR.to_bits() && got[0].reach.along() >= 2.0 * ZERO,
        "the narrow box about the true meeting is reported with its length: {got:?}"
    );
    for t in [early, at_end, in_gap] {
        let got = settled((a, b), vec![holding_cell(t)]);
        assert!(
            matches!(got, Err(SplineFaceRefusal::Escalated(d)) if d.predicate == Some("spline_face_root_at_end")),
            "the end lies within the reach of the box about {t}: {got:?}"
        );
    }
    let t = b - 0.25 * ZERO;
    let x = shallow().eval(t).x;
    let beside_end = [(t - 0.1 * ZERO, t + 0.1 * ZERO), (x, x), (0.5, 0.5)];
    let got = settled((a, b), vec![beside_end]).unwrap();
    assert!(got.is_empty(), "every point of the box is the end's: {got:?}");
}

/// **A box that holds an end of the span is never apart from it.** Both
/// boxes hold the true meeting, two escalation lengths short of the end,
/// and both reach past the end. Read on its middle alone, the one whose
/// middle is the true meeting refused (`Unsettled::Meter`: its middle
/// apart from the end and the box not inside the span) and the one whose
/// middle lies half a zero band short of the end was dropped as the
/// end's, the box never compared. Read with the box's length as the
/// reach, both refuse at the end's row.
#[test]
fn a_box_past_the_end_refuses_wherever_its_middle_is() {
    let (a, b) = span();
    let boxed = |lo: f64, hi: f64| {
        let mid = geom::bracket_mid((lo, hi));
        let x = shallow().eval(mid).x;
        assert!(
            lo < T_STAR && T_STAR < hi && hi > b,
            "holds the meeting, past the end"
        );
        settled((a, b), vec![[(lo, hi), (x, x), (0.5, 0.5)]])
    };
    let lo = T_STAR - ZERO;
    for (lo, hi) in [
        (T_STAR - 3.0 * ESCALATE, T_STAR + 3.0 * ESCALATE),
        (lo, 2.0 * (b - 0.5 * ZERO) - lo),
    ] {
        let got = boxed(lo, hi);
        assert!(
            matches!(got, Err(SplineFaceRefusal::Escalated(d)) if d.predicate == Some("spline_face_root_at_end")),
            "the box ({lo}, {hi}) holds the end: {got:?}"
        );
    }
}

/// A bicubic patch over the square of side `size`, waved by a twentieth
/// of it.
#[allow(clippy::cast_precision_loss)]
fn waved(size: f64) -> NurbsSurface<f64> {
    graph(3, size, |i, j| {
        size * 0.05 * (((i * 7 + j * 3) % 5) as f64 - 2.0) / 2.0
    })
}

/// A line by arc length through `chart`'s point at `(0.37, 0.61)`, at
/// [`SIN`] to the chart there, and half its span: the meeting is at the
/// span's middle to the accuracy of the central differences the line is
/// aimed by.
fn shallow_over(chart: &NurbsSurface<f64>, size: f64) -> (Curve3<f64>, f64) {
    let (u0, v0, h) = (0.37, 0.61, 1e-6);
    let unit = |v: Vec3<f64>| v * (1.0 / v.norm());
    let su = chart.eval(u0 + h, v0) - chart.eval(u0 - h, v0);
    let sv = chart.eval(u0, v0 + h) - chart.eval(u0, v0 - h);
    let d = unit(unit(su) * (1.0 - SIN * SIN).sqrt() + unit(su.cross(sv)) * SIN);
    let half = 0.005 * size;
    let line = Curve3::Line(geom::Line {
        origin: chart.eval(u0, v0) - d * half,
        dir: geom::stated_direction(d),
    });
    (line, half)
}

/// **At `f64` the walk's box is far narrower than the band.** On a
/// bicubic patch a metre and a kilometre across, crossed at [`SIN`],
/// [`isolate`]'s narrowed box holds the meeting in a parameter stretch
/// under a hundredth of the zero band (measured: `1.4·10⁻¹⁵` m and
/// `1.5·10⁻¹²` m), so its middle, which [`settle`] stores, is the meeting
/// to that width. The width is the operator's fixed point, the residual's
/// rounding over the sine of the crossing, and the reach [`settle`]
/// places the meeting with is the line's length over it.
#[test]
fn at_f64_the_walks_box_is_far_narrower_than_the_band() {
    for size in [1.0, 1e3] {
        let chart = waved(size);
        let (line, half) = shallow_over(&chart, size);
        let iso = isolate(&SpanCarrier::of(&line), (0.0, 2.0 * half), &chart, band()).unwrap();
        assert_eq!(iso.roots.len(), 1, "size {size}: {iso:?}");
        let (lo, hi) = iso.roots[0][0];
        assert!(
            hi - lo < 1e-2 * ZERO,
            "size {size}: the box is {} m of the line wide",
            hi - lo
        );
        let placed = settle(&line, &chart, &iso, band()).unwrap();
        assert!(
            placed.len() == 1 && placed[0].reach.along() < 1e-2 * ZERO,
            "size {size}: the meeting's reach is its box's length: {placed:?}"
        );
    }
}

/// **At `Interval` the leaf's box is bands wide, and the leaf places its
/// meeting as its witness does or refuses.** The chart's heights are
/// widened by a tenth of the zero band each way. Every member's meeting
/// lies within `0.1·zero / SIN` of the witness's, and the leaf's box
/// holds them all: it is more than ten zero bands of the line wide, and
/// that length is the leaf's reach. Its middle agrees with the witness's
/// proposal to a thousandth of the zero band, the box of members being
/// symmetric about the witness.
///
/// Against the span's own end, far from the meeting, both report it.
/// Against an end fifteen zero bands past the meeting, outside the leaf's
/// box and inside its reach, the witness reports and the leaf refuses at
/// the end's row.
#[test]
fn at_interval_the_leaf_places_its_meeting_as_its_witness_does_or_refuses() {
    let chart = waved(1.0);
    let (line, half) = shallow_over(&chart, 1.0);
    let witness = isolate(&SpanCarrier::of(&line), (0.0, 2.0 * half), &chart, band()).unwrap();
    let w = 0.1 * ZERO;
    let fat = chart
        .map_scalar(Interval::point)
        .map_points(|q| Point3::new(q.x, q.y, q.z + Interval::from_bounds(-w, w)));
    let fat_line = line.map_scalar(Interval::point);
    let span = (Interval::point(0.0), Interval::point(2.0 * half));
    let leaf = isolate(&SpanCarrier::of(&fat_line), span, &fat, band()).unwrap();
    assert!(
        witness.roots.len() == 1 && leaf.roots.len() == 1,
        "{witness:?} {leaf:?}"
    );
    let (lo, hi) = leaf.roots[0][0];
    assert!(hi - lo > 10.0 * ZERO, "the leaf's box: {} m wide", hi - lo);
    let middle = geom::bracket_mid(witness.roots[0][0]);
    let middles = geom::bracket_mid(leaf.roots[0][0]) - middle;
    assert!(
        middles.abs() < 1e-3 * ZERO,
        "the middles agree: {middles} m apart"
    );
    let ended = |iso: &Isolated, end: f64| Isolated {
        span: (0.0, end),
        ..iso.clone()
    };
    let far = 2.0 * half;
    let (of_witness, of_leaf) = (
        settle(&line, &chart, &ended(&witness, far), band()).unwrap(),
        settle(&fat_line, &fat, &ended(&leaf, far), band()).unwrap(),
    );
    assert!(
        of_witness.len() == 1 && of_leaf.len() == 1,
        "both report against the far end: {of_witness:?} {of_leaf:?}"
    );
    assert!(
        of_witness[0].reach.along() < 1e-2 * ZERO && of_leaf[0].reach.along() >= hi - lo,
        "the leaf's reach is its box's length: {of_witness:?} {of_leaf:?}"
    );
    let near = middle + 15.0 * ZERO;
    assert!(hi < near, "the near end is outside the leaf's box");
    let of_witness = settle(&line, &chart, &ended(&witness, near), band()).unwrap();
    assert_eq!(of_witness.len(), 1, "the witness's meeting is apart: {of_witness:?}");
    let of_leaf = settle(&fat_line, &fat, &ended(&leaf, near), band());
    assert!(
        matches!(of_leaf, Err(SplineFaceRefusal::Escalated(d)) if d.predicate == Some("spline_face_root_at_end")),
        "the near end lies within the leaf's reach: {of_leaf:?}"
    );
}
