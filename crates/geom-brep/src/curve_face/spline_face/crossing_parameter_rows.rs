//! Measurement rows for the work item
//! `work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge.md`:
//! what [`super::settle`] answers on parameters its own ON reading
//! admits, and how wide [`super::isolate`]'s box round the true meeting
//! is on a shallow crossing, at `f64` and at `Interval`. Each row pins
//! today's behaviour.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, NurbsSurface};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};

use super::{ChartMeeting, Isolated, SpanCarrier, SplineFaceRefusal, isolate, settle};
use crate::roots::Unsettled;

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
    settle(&shallow(), &flat(), &Isolated { span, roots }, band())
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

/// **`settle` answers three ways on parameters its ON reading admits.**
/// The line crosses the chart at `SIN`, so every parameter within
/// `ZERO / SIN` (a hundred zero bands along the edge) of the true meeting
/// reads ON. With the true meeting two escalation lengths short of the
/// span's end, a point box at the true meeting is reported; one fifty
/// zero bands before it is reported at that parameter; one half a zero
/// band short of the end is dropped as the end's; one five zero bands
/// short of the end escalates. The boxes are hand-made, each a point,
/// so nothing here says [`isolate`] proposes them.
#[test]
fn settle_answers_three_ways_on_parameters_its_on_reading_admits() {
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
    for t in [T_STAR, early] {
        let got = settled((a, b), vec![point_cell(t)]).unwrap();
        assert!(
            got.len() == 1 && got[0].t.to_bits() == t.to_bits(),
            "reported at the box's own parameter {t}: {got:?}"
        );
    }
    let got = settled((a, b), vec![point_cell(at_end)]).unwrap();
    assert!(got.is_empty(), "dropped as the end's: {got:?}");
    let got = settled((a, b), vec![point_cell(in_gap)]);
    assert!(
        matches!(got, Err(SplineFaceRefusal::Escalated(_))),
        "in the gap of the end: {got:?}"
    );
}

/// **The at-end reading is taken on the box's middle before the box is
/// compared to the ends.** Both boxes hold the true meeting, two
/// escalation lengths short of the end, and both reach past the end. The
/// one whose middle is the true meeting refuses (`Unsettled::Meter`): its
/// middle is definitely apart from the end and the box is not inside the
/// span. The one whose middle lies half a zero band short of the end is
/// dropped as the end's, and the box is never compared. Hand-made boxes:
/// the measurement of how wide [`isolate`]'s are is the row below.
#[test]
fn a_box_past_the_end_is_dropped_on_its_middle_before_it_is_compared() {
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
    let centred = boxed(T_STAR - 3.0 * ESCALATE, T_STAR + 3.0 * ESCALATE);
    assert!(
        matches!(
            centred,
            Err(SplineFaceRefusal::Unsettled(Unsettled::Meter {
                row: "spline_face_root_at_end"
            }))
        ),
        "middle apart from the end, box past it: {centred:?}"
    );
    let lo = T_STAR - ZERO;
    let near_end = boxed(lo, 2.0 * (b - 0.5 * ZERO) - lo);
    assert!(
        matches!(&near_end, Ok(found) if found.is_empty()),
        "middle at the end: dropped, the box unread: {near_end:?}"
    );
}

/// A bicubic patch over the square of side `size`, waved by a twentieth
/// of it.
fn waved(size: f64) -> NurbsSurface<f64> {
    #[allow(clippy::cast_precision_loss)]
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
/// rounding over the sine of the crossing; nothing compares it to the
/// band, and [`settle`] compares the box only to the span's ends.
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
    }
}

/// **At `Interval` the leaf's box is bands wide and its middle is its
/// witness's proposal.** The chart's heights are widened by a tenth of
/// the zero band each way. Every member's meeting lies within
/// `0.1·zero / SIN` of the witness's, and the leaf's box holds them all:
/// it is more than ten zero bands of the line wide. Its middle agrees
/// with the witness's proposal to a thousandth of the zero band, the box
/// of members being symmetric about the witness, and [`settle`] reports
/// it: the middle's point reads ON at every member. So the leaf and its
/// witness store one parameter here, where the composite arm's
/// (`composite::crossing_parameter_rows`) store two; what the leaf's
/// at-end and apart readings are taken on is the middle's point, with
/// the box compared only to the span's ends.
#[test]
fn at_interval_the_leafs_box_is_bands_wide_and_its_middle_is_its_witnesss() {
    let chart = waved(1.0);
    let (line, half) = shallow_over(&chart, 1.0);
    let witness = isolate(&SpanCarrier::of(&line), (0.0, 2.0 * half), &chart, band()).unwrap();
    let w = 0.1 * ZERO;
    let fat = chart
        .map_scalar(Interval::point)
        .map_points(|q| Point3::new(q.x, q.y, q.z + Interval::from_bounds(-w, w)));
    let line = line.map_scalar(Interval::point);
    let span = (Interval::point(0.0), Interval::point(2.0 * half));
    let leaf = isolate(&SpanCarrier::of(&line), span, &fat, band()).unwrap();
    assert!(
        witness.roots.len() == 1 && leaf.roots.len() == 1,
        "{witness:?} {leaf:?}"
    );
    let (lo, hi) = leaf.roots[0][0];
    assert!(hi - lo > 10.0 * ZERO, "the leaf's box: {} m wide", hi - lo);
    let middles = geom::bracket_mid(leaf.roots[0][0]) - geom::bracket_mid(witness.roots[0][0]);
    assert!(
        middles.abs() < 1e-3 * ZERO,
        "the middles agree: {middles} m apart"
    );
    let placed = settle(&line, &fat, &leaf, band()).unwrap();
    assert_eq!(placed.len(), 1, "the leaf's middle is reported: {placed:?}");
}
