//! The spline-face arm ([`super::isolate`], then [`super::settle`]) on Bézier and B-spline charts: a line crossing a
//! bicubic patch once, a circle crossing it twice, a line tangent to a
//! dome, a pass within the band of it and one past the band, a meeting
//! on a crease and one beside it, a meeting at an end of the span, and
//! a rational patch. The in-band geometry scales with the run's band.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::{Curve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ChartMeeting, SplineFaceRefusal};
use crate::roots::Unsettled;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A Bézier patch of bidegree `(p, q)` over `[0, 1]²`, its control
/// points `(i/p, j/q, z[i][j])`: a graph over the `xy` square whose
/// chart point is its `(x, y)`.
fn graph(p: usize, q: usize, z: &[Vec<f64>]) -> NurbsSurface<f64> {
    let ends = |d: usize| {
        let mut k = vec![0.0; d + 1];
        k.extend(vec![1.0; d + 1]);
        KnotVector::clamped(k, d).unwrap()
    };
    #[allow(clippy::cast_precision_loss)]
    let control = (0..=p)
        .flat_map(|i| (0..=q).map(move |j| (i, j)))
        .map(|(i, j)| Point3::new(i as f64 / p as f64, j as f64 / q as f64, z[i][j]))
        .collect();
    NurbsSurface::new(ends(p), ends(q), control, vec![1.0; (p + 1) * (q + 1)]).unwrap()
}

/// A gently waved bicubic patch, its height at most 0.15.
fn waved() -> NurbsSurface<f64> {
    graph(
        3,
        3,
        &[
            vec![0.00, 0.05, 0.10, 0.00],
            vec![0.05, 0.15, 0.05, 0.10],
            vec![0.10, 0.00, 0.15, 0.05],
            vec![0.00, 0.10, 0.05, 0.00],
        ],
    )
}

/// The paraboloid `z = (x − ½)² + (y − ½)²`, its apex `(½, ½, 0)`.
fn dome() -> NurbsSurface<f64> {
    let a = [0.25, -0.25, 0.25];
    let z: Vec<Vec<f64>> = (0..3)
        .map(|i| (0..3).map(|j| a[i] + a[j]).collect())
        .collect();
    graph(2, 2, &z)
}

fn line(origin: Point3<f64>, dir: Vec3<f64>) -> Curve3<f64> {
    Curve3::Line { origin, dir }
}

fn read(
    carrier: &Curve3<f64>,
    params: (f64, f64),
    chart: &NurbsSurface<f64>,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    crate::NurbsLane::<f64>::certified().chart_meetings(carrier, params, chart, band())
}

/// Every meeting's point lies on the chart at its chart point.
fn on_chart(chart: &NurbsSurface<f64>, found: &[ChartMeeting<f64>]) {
    for m in found {
        let s = chart.eval(m.uv.0, m.uv.1);
        assert!(
            (m.point - s).norm() <= band().zero(),
            "a meeting reads on the chart: {m:?} against {s:?}"
        );
    }
}

#[test]
fn a_line_crosses_a_bicubic_patch_once() {
    let chart = waved();
    let up = line(Point3::new(0.4, 0.6, -1.0), Vec3::unit_z());
    let found = read(&up, (0.0, 2.0), &chart).unwrap();
    assert_eq!(found.len(), 1, "one meeting: {found:?}");
    on_chart(&chart, &found);
    let m = found[0];
    assert!(
        (m.uv.0 - 0.4).abs() < 1e-9 && (m.uv.1 - 0.6).abs() < 1e-9,
        "the meeting's chart point is its (x, y): {m:?}"
    );
    assert!(
        (m.t - (1.0 + chart.eval(0.4, 0.6).z)).abs() < 1e-9,
        "at the patch's height: {m:?}"
    );
    // Short of the patch, the span is clear.
    assert!(
        read(&up, (0.0, 0.9), &chart).unwrap().is_empty(),
        "a span ending below the patch meets it nowhere"
    );
}

#[test]
fn a_circle_crosses_a_bicubic_patch_twice() {
    let chart = waved();
    let ring = Curve3::Circle {
        center: Point3::new(0.5, 0.5, 0.0),
        axis: Vec3::unit_y(),
        radius: 0.3,
        u_ref: Vec3::unit_x(),
    };
    let found = read(&ring, (0.0, 2.0 * core::f64::consts::PI - 0.1), &chart).unwrap();
    assert_eq!(found.len(), 2, "two meetings: {found:?}");
    on_chart(&chart, &found);
    // From `θ = 0` at `x = 0.8` the circle runs down, under the patch,
    // and meets it first at its `x < ½` side.
    assert!(found[0].t < found[1].t, "ascending");
    assert!(
        found[0].uv.0 < 0.5 && found[1].uv.0 > 0.5,
        "one meeting each side of the centre: {found:?}"
    );
}

#[test]
fn a_line_tangent_to_a_dome_escalates() {
    let along = line(Point3::new(0.0, 0.5, 0.0), Vec3::unit_x());
    let got = read(&along, (0.1, 0.9), &dome());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Tangency))),
        "a tangency refuses: {got:?}"
    );
}

#[test]
fn a_pass_within_the_band_escalates_and_one_past_it_is_clear() {
    let chart = dome();
    let below = |gap: f64| line(Point3::new(0.0, 0.5, -gap), Vec3::unit_x());
    let near = read(&below(0.5 * band().zero()), (0.1, 0.9), &chart);
    assert!(
        matches!(near, Err(SplineFaceRefusal::Unsettled(Unsettled::Tangency))),
        "a pass within the band is never clear: {near:?}"
    );
    let far = read(&below(64.0 * band().escalate()), (0.1, 0.9), &chart);
    assert!(
        matches!(far.as_deref(), Ok([])),
        "a pass past the band is clear: {far:?}"
    );
}

/// A tent over the `xy` square, bilinear on each half, its ridge the
/// crease at `x = ½`, `z = 0.2`.
fn tent() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.5, 0.2), (1.0, 0.0)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

#[test]
fn a_meeting_on_a_crease_refuses_and_one_beside_it_reads() {
    let chart = tent();
    let at = |x: f64| line(Point3::new(x, 0.5, -1.0), Vec3::unit_z());
    let on = read(&at(0.5), (0.0, 2.0), &chart);
    assert!(
        matches!(on, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
        "a meeting on the ridge refuses: {on:?}"
    );
    let beside = read(&at(0.25), (0.0, 2.0), &chart).unwrap();
    assert_eq!(beside.len(), 1, "one meeting off the ridge: {beside:?}");
    on_chart(&chart, &beside);
    assert!(
        (beside[0].t - 1.1).abs() < 1e-9,
        "halfway up the tent's flank: {beside:?}"
    );
}

#[test]
fn a_meeting_at_an_end_of_the_span_is_the_ends() {
    let chart = waved();
    let up = line(Point3::new(0.4, 0.6, -1.0), Vec3::unit_z());
    let t = read(&up, (0.0, 2.0), &chart).unwrap()[0].t;
    for params in [(0.0, t), (t, 2.0)] {
        let got = read(&up, params, &chart).unwrap();
        assert!(got.is_empty(), "{params:?}: the end's meeting: {got:?}");
    }
}

#[test]
fn a_line_crosses_a_rational_quarter_cylinder() {
    // The quarter of the cylinder `x² + z² = 1` over `y ∈ [0, 1]`.
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    let w = [1.0, FRAC_1_SQRT_2, 1.0];
    let weights = w.iter().flat_map(|&w| [w, w]).collect();
    let chart = NurbsSurface::new(ku, kv, control, weights).unwrap();
    let across = line(Point3::new(0.0, 0.5, 0.5), Vec3::unit_x());
    let found = read(&across, (0.0, 2.0), &chart).unwrap();
    assert_eq!(found.len(), 1, "one meeting: {found:?}");
    on_chart(&chart, &found);
    assert!(
        (found[0].t - 0.75f64.sqrt()).abs() < 1e-9,
        "on the circle x² + z² = 1: {found:?}"
    );
}

// ---- review r2 (PR 5826): the chart-edge skip near an end of the span ----

/// The parabolic trough `z = h·((x − ½)/a)²` over `x ∈ [½ − a, ½ + a]`,
/// `y ∈ [0, y1]`: a Bézier patch of bidegree (2, 1).
fn trough(a: f64, h: f64, y1: f64) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.5 - a, h), (0.5, -h), (0.5 + a, h)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, y1, z)])
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// **A tangency just inside an open chart's edge, near an end of the
/// span, is not silently cleared.** A line touches the trough along its
/// floor at `x = ½`, `32·escalate` inside the chart's `y = 0` edge
/// (past the band of that edge, so the boundary pass reads it clear),
/// and the span ends `128·escalate` past the touch (past the band of
/// it, so the touch is no end's). An in-band reading escalates.
#[test]
fn review_r2_a_tangency_beside_the_charts_edge_near_an_end_escalates() {
    let s = band().escalate();
    let l = 1_048_576.0 * s;
    let chart = trough(512.0 * s, 64.0 * s, l);
    let at = |y0: f64| line(Point3::new(0.5, y0, 0.0), Vec3::unit_x());
    // The control: the same touch mid-chart refuses.
    let mid = read(&at(0.5 * l), (-l + 128.0 * s, 128.0 * s), &chart);
    assert!(
        matches!(mid, Err(SplineFaceRefusal::Unsettled(Unsettled::Tangency))),
        "a touch mid-chart refuses: {mid:?}"
    );
    let edge = read(&at(32.0 * s), (-l + 128.0 * s, 128.0 * s), &chart);
    assert!(
        matches!(edge, Err(SplineFaceRefusal::Unsettled(_))),
        "a touch 32 escalation lengths inside the chart's edge refuses too: {edge:?}"
    );
}

/// The full rational circle cylinder `x² + y² = 1`, `z ∈ [0, 1]`, its
/// chart closed in `u` with the seam at `(1, 0, z)`.
fn full_cylinder() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let control = ring
        .into_iter()
        .flat_map(|(x, y)| [Point3::new(x, y, 0.0), Point3::new(x, y, 1.0)])
        .collect();
    let weights = (0..9)
        .flat_map(|i| {
            let w = if i % 2 == 0 { 1.0 } else { FRAC_1_SQRT_2 };
            [w, w]
        })
        .collect();
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

/// **A transverse crossing on a closed chart's seam, near an end of the
/// span, is not silently dropped.** The seam is interior to a face that
/// wraps the cylinder, so it is no boundary for the boundary pass to
/// read. The line crosses the wall at the seam `(1, 0, ½)`, its span
/// ending `64·escalate` past the crossing: past the band of it, so the
/// crossing is the span's, not the end's.
#[test]
fn review_r2_a_crossing_on_a_closed_charts_seam_near_an_end_is_not_dropped() {
    let s = band().escalate();
    let chart = full_cylinder();
    let across = line(Point3::new(0.0, 0.0, 0.5), Vec3::unit_x());
    let start = 1.0 - (1_048_576.0 * s).min(0.5);
    // The control: far from the end the seam refuses, as filed in
    // `a-meeting-on-a-closed-charts-seam-is-not-isolated`.
    let far = read(&across, (start, 1.0 + 4096.0 * s), &chart);
    assert!(
        matches!(far, Err(SplineFaceRefusal::Unsettled(_))),
        "the seam crossing refuses: {far:?}"
    );
    let near = read(&across, (start, 1.0 + 64.0 * s), &chart);
    assert!(
        !matches!(near.as_deref(), Ok([])),
        "a crossing 64 escalation lengths short of the end is not dropped: {near:?}"
    );
}
