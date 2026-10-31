//! The spline-face arm ([`super::isolate`], then [`super::settle`]) on Bézier and B-spline charts: a line crossing a
//! bicubic patch once, a circle crossing it twice, a line tangent to a
//! dome, a pass within the band of it and one past the band, a meeting
//! on a crease and one beside it, a meeting at an end of the span, and
//! a rational patch; then the same readings for a NURBS span, its ends
//! at its domain's. The in-band geometry scales with the run's band.

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
fn a_line_tangent_to_a_dome_refuses_at_the_floor() {
    let along = line(Point3::new(0.0, 0.5, 0.0), Vec3::unit_x());
    let got = read(&along, (0.1, 0.9), &dome());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a tangency refuses: {got:?}"
    );
}

#[test]
fn a_pass_within_the_band_refuses_at_the_floor() {
    let below = line(Point3::new(0.0, 0.5, -0.5 * band().zero()), Vec3::unit_x());
    let near = read(&below, (0.1, 0.9), &dome());
    assert!(
        matches!(near, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a pass within the band is never clear: {near:?}"
    );
}

/// **A line running along a flat patch within the band refuses**: the
/// fast set's in-band row, its walk reaching the floor in a few dozen
/// boxes.
#[test]
fn a_line_along_a_patch_within_the_band_refuses_at_the_floor() {
    let along = line(Point3::new(0.0, 0.5, 0.5 * band().zero()), Vec3::unit_x());
    let got = read(&along, (0.2, 0.8), &flat());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a run within the band is never clear: {got:?}"
    );
}

#[test]
fn a_pass_past_the_band_is_clear() {
    let below = line(
        Point3::new(0.0, 0.5, -64.0 * band().escalate()),
        Vec3::unit_x(),
    );
    let far = read(&below, (0.1, 0.9), &dome());
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

/// Review r1: the flat unit patch `z = 0` over `[0, 1]²`.
fn flat() -> NurbsSurface<f64> {
    graph(1, 1, &[vec![0.0, 0.0], vec![0.0, 0.0]])
}

/// Review r1: a circle of radius `r` in the plane `x = δ`, touching the
/// flat patch at `(δ, ½, 0)` at `θ = π/2`, read from `θ = 0` to `rβ`
/// metres past the touch.
fn touching(delta: f64, r: f64, past: f64) -> (Curve3<f64>, (f64, f64)) {
    let ring = Curve3::Circle {
        center: Point3::new(delta, 0.5, r),
        axis: Vec3::unit_x(),
        radius: r,
        u_ref: -Vec3::unit_y(),
    };
    (ring, (0.0, core::f64::consts::FRAC_PI_2 + past / r))
}

/// Review r1: a tangency strictly inside the span, `500·escalate` short
/// of its end and well past the band from it, refuses wherever it is on
/// the chart; the walk's chart-edge skip drops it when it lies `4·escalate`
/// from the chart's edge (past the band of it).
#[test]
fn review_r1_a_tangency_near_the_chart_edge_and_an_end_is_not_dropped() {
    let e = band().escalate();
    let chart = flat();
    for delta in [0.5, 4.0 * e] {
        let (ring, params) = touching(delta, 5e4 * e, 500.0 * e);
        let got = read(&ring, params, &chart);
        assert!(
            matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
            "δ = {delta:e}: a tangency 500·escalate inside the span refuses: {got:?}"
        );
    }
}

/// Review r1: a circle of radius `1000·escalate` dipping half its radius
/// through the flat patch crosses it twice, about 2 rad apart; each
/// meeting's growth must be re-proved, or one box swallows both.
#[test]
fn review_r1_a_small_circle_crosses_a_flat_patch_twice() {
    let r = 1000.0 * band().escalate();
    let ring = Curve3::Circle {
        center: Point3::new(0.5, 0.5, 0.5 * r),
        axis: Vec3::unit_y(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let found = read(&ring, (0.0, 2.0 * core::f64::consts::PI - 0.1), &flat()).unwrap();
    assert_eq!(found.len(), 2, "two meetings: {found:?}");
}

/// Review r1: a pass at twice the coincidence threshold, inside the
/// band's gap, is not clear either.
#[test]
fn review_r1_a_pass_in_the_bands_gap_refuses_at_the_floor() {
    let gap = 2.0 * band().zero();
    assert!(gap < band().escalate());
    let below = line(Point3::new(0.0, 0.5, -gap), Vec3::unit_x());
    let got = read(&below, (0.1, 0.9), &dome());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a pass in the band's gap is never clear: {got:?}"
    );
}

/// Review r1: a meeting half the coincidence threshold off the tent's
/// ridge is within the band of the crease, and refuses.
#[test]
fn review_r1_a_meeting_within_the_band_of_a_crease_refuses() {
    let x = 0.5 + 0.5 * band().zero();
    let at = line(Point3::new(x, 0.5, -1.0), Vec3::unit_z());
    let got = read(&at, (0.0, 2.0), &tent());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
        "within the band of the ridge: {got:?}"
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
fn review_r2_a_tangency_beside_the_charts_edge_near_an_end_refuses_at_the_floor() {
    let s = band().escalate();
    let l = 1_048_576.0 * s;
    let chart = trough(512.0 * s, 64.0 * s, l);
    let at = |y0: f64| line(Point3::new(0.5, y0, 0.0), Vec3::unit_x());
    // The control: the same touch mid-chart refuses.
    let mid = read(&at(0.5 * l), (-l + 128.0 * s, 128.0 * s), &chart);
    assert!(
        matches!(mid, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
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
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
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

/// **A pass in the band's sliver escalates**: the dome's row passes at
/// `½·zero`, which a clear rule reading `zero` as its threshold also
/// refuses; this one passes between `zero` and `escalate`.
#[test]
fn review_r2_a_pass_in_the_bands_sliver_refuses_at_the_floor() {
    let b = band();
    let chart = dome();
    let gap = 0.5 * (b.zero() + b.escalate());
    let below = line(Point3::new(0.0, 0.5, -gap), Vec3::unit_x());
    let got = read(&below, (0.1, 0.9), &chart);
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a pass in the sliver is never clear: {got:?}"
    );
}

/// **Two transverse meetings a few hundred escalation lengths apart are
/// both found**: a line crosses both walls of a trough whose floor lies
/// `61·escalate` below it, the meetings `1000·escalate` apart, so no isolating box may be grown over both.
#[test]
fn review_r2_two_close_meetings_are_both_found() {
    let s = band().escalate();
    let (a, h) = (512.0 * s, 64.0 * s);
    let d = h * (500.0f64 / 512.0).powi(2);
    let chart = trough(a, h, 1.0);
    let flat = line(Point3::new(0.5, 0.5, d), Vec3::unit_x());
    let found = read(&flat, (-2.0 * a, 2.0 * a), &chart).unwrap();
    assert_eq!(found.len(), 2, "both walls: {found:?}");
    on_chart(&chart, &found);
}

/// **A meeting within the band of a crease, not on it, refuses**; one a
/// few escalation lengths off it reads.
#[test]
fn review_r2_a_meeting_beside_a_crease_within_the_band_refuses() {
    let b = band();
    let chart = tent();
    let at = |x: f64| line(Point3::new(x, 0.5, -1.0), Vec3::unit_z());
    let near = read(&at(0.5 + 0.5 * b.zero()), (0.0, 2.0), &chart);
    assert!(
        matches!(near, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
        "within the band of the ridge: {near:?}"
    );
    let off = read(&at(0.5 + 8.0 * b.escalate()), (0.0, 2.0), &chart);
    assert!(
        matches!(off.as_deref(), Ok([_])),
        "eight escalation lengths off the ridge: {off:?}"
    );
}

/// A V valley over the `xy` square, bilinear on each half, its floor the
/// crease at `x = ½`, `z = −0.2`.
fn valley() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.5, -0.2), (1.0, 0.0)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// **Two meetings either side of a crease, apart from each other but each
/// within the band of it, refuse.** A level line `0.3·escalate` over the
/// valley's floor crosses both walls `1.5·escalate` apart, each meeting
/// `0.8·escalate` from the crease: each block isolates its own, so no box
/// at the floor is left, and only the crease reading of the meetings
/// refuses them.
#[test]
fn two_meetings_apart_but_each_within_the_band_of_a_crease_refuse() {
    let h = 0.3 * band().escalate();
    let level = line(Point3::new(0.0, 0.5, -0.2 + h), Vec3::unit_x());
    let got = read(&level, (0.1, 0.9), &valley());
    assert!(
        matches!(
            got,
            Err(SplineFaceRefusal::Escalated(_) | SplineFaceRefusal::Unsettled(Unsettled::Crease))
        ),
        "meetings within the band of the valley crease refuse: {got:?}"
    );
}

/// **A meeting within the band of a crease refuses where the chart is
/// flat across it.**
/// The plane `z = 0` as two bilinear halves, its knot at `x = ½` of
/// multiplicity the degree, a crease by its knots though the two halves
/// share one derivative: a box touching the crease reads the same
/// Jacobian on both sides, so only the crease rule, and not the
/// enclosure's width, keeps it from carrying a proof.
#[test]
fn a_meeting_on_a_flat_crease_refuses() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [0.0, 0.5, 1.0]
        .into_iter()
        .flat_map(|x| [Point3::new(x, 0.0, 0.0), Point3::new(x, 1.0, 0.0)])
        .collect();
    let chart = NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap();
    let x = 0.5 + 0.5 * band().zero();
    let up = line(Point3::new(x, 0.5, -1.0), Vec3::unit_z());
    let got = read(&up, (0.0, 2.0), &chart);
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
        "a meeting within the band of the knot refuses: {got:?}"
    );
    let off = line(Point3::new(0.25, 0.5, -1.0), Vec3::unit_z());
    assert!(
        matches!(read(&off, (0.0, 2.0), &chart).as_deref(), Ok([_])),
        "one off it reads"
    );
}

// ---- a NURBS span against a spline face ----

/// The NURBS span of degree `p` over the clamped `knots` through
/// `control`, weighted `weights` (all one where `None`).
fn spline(p: usize, knots: Vec<f64>, control: &[[f64; 3]], weights: Option<&[f64]>) -> Curve3<f64> {
    let control: Vec<Point3<f64>> = control
        .iter()
        .map(|&[x, y, z]| Point3::new(x, y, z))
        .collect();
    let weights = weights.map_or_else(|| vec![1.0; control.len()], <[f64]>::to_vec);
    Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, weights).unwrap(),
    ))
}

/// The Bézier span through `control`, of degree one less than their count.
fn bezier(control: &[[f64; 3]]) -> Curve3<f64> {
    let p = control.len() - 1;
    let mut knots = vec![0.0; p + 1];
    knots.extend(vec![1.0; p + 1]);
    spline(p, knots, control, None)
}

/// The meetings of a span over its whole domain `[0, 1]`.
fn read_whole(
    carrier: &Curve3<f64>,
    chart: &NurbsSurface<f64>,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    read(carrier, (0.0, 1.0), chart)
}

/// Every meeting's point is the span's at its parameter.
fn on_span(carrier: &Curve3<f64>, found: &[ChartMeeting<f64>]) {
    for m in found {
        assert!(
            (carrier.eval(m.t) - m.point).norm() <= band().zero(),
            "a meeting's point is the span's: {m:?}"
        );
    }
}

#[test]
fn a_nurbs_span_crosses_a_bicubic_patch_once() {
    let chart = waved();
    let rising = bezier(&[
        [0.3, 0.4, -0.5],
        [0.5, 0.4, 0.0],
        [0.4, 0.7, 0.3],
        [0.6, 0.7, 0.8],
    ]);
    let found = read_whole(&rising, &chart).unwrap();
    assert_eq!(found.len(), 1, "one meeting: {found:?}");
    on_chart(&chart, &found);
    on_span(&rising, &found);
    let m = found[0];
    assert!(
        (m.uv.0 - m.point.x).abs() < 1e-9 && (m.uv.1 - m.point.y).abs() < 1e-9,
        "the meeting's chart point is its (x, y): {m:?}"
    );
    // Strictly inside its domain, short of the patch, the span is clear:
    // the walk reads past its ends and finds the meeting beyond them.
    let below = read(&rising, (0.05, 0.6 * m.t), &chart);
    assert!(
        matches!(below.as_deref(), Ok([])),
        "short of the patch: {below:?}"
    );
}

#[test]
fn a_nurbs_span_crosses_a_bicubic_patch_twice() {
    let chart = waved();
    let arch = bezier(&[
        [0.2, 0.5, 0.6],
        [0.4, 0.5, -0.7],
        [0.6, 0.5, -0.7],
        [0.8, 0.5, 0.6],
    ]);
    let found = read_whole(&arch, &chart).unwrap();
    assert_eq!(found.len(), 2, "two meetings: {found:?}");
    on_chart(&chart, &found);
    on_span(&arch, &found);
    assert!(found[0].t < found[1].t, "ascending");
    assert!(
        found[0].uv.0 < 0.5 && found[1].uv.0 > 0.5,
        "one meeting each side of the arch's middle: {found:?}"
    );
}

/// The quadratic `z = 0.8·(s − ½)² + lift` over `x = 0.1 + 0.8·s` at
/// `y = ½`, over the dome, which it touches at its apex where `lift` is
/// zero, its curvature larger than the dome's along it.
fn cup(lift: f64) -> Curve3<f64> {
    bezier(&[
        [0.1, 0.5, 0.2 + lift],
        [0.5, 0.5, -0.2 + lift],
        [0.9, 0.5, 0.2 + lift],
    ])
}

#[test]
fn a_nurbs_span_tangent_to_a_dome_refuses_at_the_floor() {
    let got = read_whole(&cup(0.0), &dome());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a tangency refuses: {got:?}"
    );
}

/// The quadratic `z = lift + 3.25·(x − ½)²` over `x ∈ [0.1, 0.9]` at
/// `y = ½`: `lift` over the dome's apex, and bending up from it faster
/// than the dome does.
fn steep(lift: f64) -> Curve3<f64> {
    bezier(&[
        [0.1, 0.5, lift + 0.52],
        [0.5, 0.5, lift - 0.52],
        [0.9, 0.5, lift + 0.52],
    ])
}

#[test]
fn a_nurbs_pass_within_the_band_refuses_at_the_floor() {
    let near = read_whole(&steep(0.5 * band().zero()), &dome());
    assert!(
        matches!(near, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "a pass within the band is never clear: {near:?}"
    );
    let gap = read_whole(&steep(2.0 * band().zero()), &dome());
    assert!(
        matches!(gap, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))),
        "nor one in the band's gap: {gap:?}"
    );
}

#[test]
fn a_nurbs_pass_past_the_band_is_clear() {
    let far = read_whole(&steep(64.0 * band().escalate()), &dome());
    assert!(
        matches!(far.as_deref(), Ok([])),
        "a pass past the band is clear: {far:?}"
    );
}

/// A polyline of degree one with its corner, a crease of the span's own
/// knots, at `corner`.
fn bent(corner: [f64; 3]) -> Curve3<f64> {
    spline(
        1,
        vec![0.0, 0.0, 0.5, 1.0, 1.0],
        &[[0.3, 0.5, -0.5], corner, [0.6, 0.5, 0.5]],
        None,
    )
}

#[test]
fn a_meeting_on_a_crease_of_the_span_refuses_and_one_beside_it_reads() {
    for lift in [0.0, 0.5 * band().zero()] {
        let got = read_whole(&bent([0.5, 0.5, lift]), &flat());
        assert!(
            matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
            "a meeting within the band of the span's corner refuses (lift {lift:e}): {got:?}"
        );
    }
    let span = bent([0.5, 0.5, 0.2]);
    let beside = read_whole(&span, &flat()).unwrap();
    assert_eq!(beside.len(), 1, "one meeting off the corner: {beside:?}");
    on_span(&span, &beside);
    assert!(beside[0].t < 0.5, "on the first leg: {beside:?}");
}

#[test]
fn a_nurbs_meeting_on_a_crease_of_the_chart_refuses_and_one_beside_it_reads() {
    let chart = tent();
    let at = |x: f64| bezier(&[[x, 0.5, -1.0], [x, 0.4, 0.5], [x, 0.6, 2.0]]);
    for x in [0.5, 0.5 + 0.5 * band().zero()] {
        let on = read_whole(&at(x), &chart);
        assert!(
            matches!(on, Err(SplineFaceRefusal::Unsettled(Unsettled::Crease))),
            "a meeting within the band of the ridge refuses (x {x}): {on:?}"
        );
    }
    let beside = read_whole(&at(0.25), &chart).unwrap();
    assert_eq!(beside.len(), 1, "one meeting off the ridge: {beside:?}");
    on_chart(&chart, &beside);
    assert!(
        (beside[0].point.z - 0.1).abs() < 1e-9,
        "halfway up the tent's flank: {beside:?}"
    );
}

/// **Rational on both sides**: the quarter circle of radius one about
/// `(1, ½, 0)` in the plane `y = ½`, from `(0, ½, 0)` to `(1, ½, 1)`,
/// crosses the rational quarter cylinder `x² + z² = 1` once, where
/// `(1 + cos θ)² + sin² θ = 1`: at `(½, ½, √3/2)`.
#[test]
fn a_rational_span_crosses_a_rational_quarter_cylinder() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    let w = [1.0, FRAC_1_SQRT_2, 1.0];
    let weights = w.iter().flat_map(|&w| [w, w]).collect();
    let chart = NurbsSurface::new(ku, kv, control, weights).unwrap();
    let arc = spline(
        2,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[[0.0, 0.5, 0.0], [0.0, 0.5, 1.0], [1.0, 0.5, 1.0]],
        Some(&w),
    );
    let found = read_whole(&arc, &chart).unwrap();
    assert_eq!(found.len(), 1, "one meeting: {found:?}");
    on_chart(&chart, &found);
    on_span(&arc, &found);
    assert!(
        (found[0].point - Point3::new(0.5, 0.5, 0.75f64.sqrt())).norm() < 1e-9,
        "at (½, ½, √3/2): {found:?}"
    );
}

/// **A span whose end is its domain's, on the chart, is the end's
/// meeting**: the walk cannot read past the carrier's domain, so the end
/// is accounted for by a box round it on which every solution lies
/// within the band of the end. An end on the chart's interior, and one
/// on its domain's edge, the span leaving the chart there (the lily's
/// tip seams, leaving the other body's wall at its rim).
#[test]
fn a_span_ending_on_the_chart_at_its_domains_end_is_the_ends() {
    let interior = bezier(&[[0.5, 0.5, 0.0], [0.6, 0.5, 0.3], [0.6, 0.5, 0.6]]);
    let got = read_whole(&interior, &flat());
    assert!(
        matches!(got.as_deref(), Ok([])),
        "an end on the chart's interior: {got:?}"
    );
    let edge = bezier(&[[0.0, 0.5, 0.0], [-0.2, 0.5, 0.2], [-0.3, 0.5, 0.5]]);
    let got = read_whole(&edge, &flat());
    assert!(
        matches!(got.as_deref(), Ok([])),
        "an end on the chart's edge: {got:?}"
    );
    // Reversed, the end is the span's start.
    let back = bezier(&[[-0.3, 0.5, 0.5], [-0.2, 0.5, 0.2], [0.0, 0.5, 0.0]]);
    let got = read_whole(&back, &flat());
    assert!(
        matches!(got.as_deref(), Ok([])),
        "a start on the chart's edge: {got:?}"
    );
}

/// **A crossing past the band of the span's end is the span's**, though
/// the walk cannot read past it: the end lies `64·escalate` above the
/// flat patch, so no box round it holds every solution within the band
/// of it.
#[test]
fn a_crossing_past_the_band_of_a_domain_end_is_reported() {
    let lift = 64.0 * band().escalate();
    let span = bezier(&[[0.4, 0.5, -0.5], [0.4, 0.5, lift]]);
    let found = read_whole(&span, &flat()).unwrap();
    assert_eq!(found.len(), 1, "the crossing short of the end: {found:?}");
    on_span(&span, &found);
    assert!(
        found[0].point.z.abs() <= band().zero(),
        "on the patch: {found:?}"
    );
}

/// A spline span crossing a spline face twice within the band of each
/// other refuses.
#[test]
fn two_nurbs_meetings_within_the_band_of_each_other_refuse() {
    let dip = 0.25 * band().zero();
    let got = read_whole(&cup(-dip), &dome());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
        "two meetings within the band of each other: {got:?}"
    );
}

// ---- review r1 (PR 5855) probes ----

/// A degree-one NURBS span from `p` to `q`, its ends its domain's.
fn segment(p: [f64; 3], q: [f64; 3]) -> Curve3<f64> {
    bezier(&[p, q])
}

/// Not silently empty: a reported meeting or a refusal.
fn not_dropped(got: &Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal>, what: &str) {
    assert!(!matches!(got.as_deref(), Ok([])), "{what}: {got:?}");
}

/// A crossing a few escalation lengths short of a NURBS span's domain
/// end, the end off the chart.
#[test]
fn review_r1_a_crossing_a_few_bands_from_a_domain_end_is_not_dropped() {
    let s = band().escalate();
    for k in [2.0, 3.0, 4.0, 8.0, 16.0] {
        let span = segment([0.4, 0.5, -0.5], [0.4, 0.5, k * s]);
        not_dropped(
            &read_whole(&span, &flat()),
            &format!("vertical, {k} escalations"),
        );
        let back = segment([0.4, 0.5, k * s], [0.4, 0.5, -0.5]);
        not_dropped(
            &read_whole(&back, &flat()),
            &format!("vertical reversed, {k}"),
        );
        // Grazing at slope 1/64: the end is 64× nearer the chart than the
        // crossing is to the end.
        let graze = segment([0.1, 0.5, k * s - 0.3 / 64.0], [0.4, 0.5, k * s / 64.0]);
        let got = read_whole(&graze, &flat());
        not_dropped(&got, &format!("grazing, {k}"));
    }
}

/// A span ending ON a dome, which it crosses a second time a few bands
/// away: the second crossing is never absorbed by the end's box. Both an
/// analytic line and a NURBS segment, both orders.
#[test]
fn review_r1_a_second_crossing_beside_an_end_on_the_chart_is_not_dropped() {
    let s = band().escalate();
    for k in [2.0, 4.0, 8.0, 64.0] {
        let half = 0.5 * k * s;
        let c = half * half;
        let x0 = 0.5 - half;
        let lin = line(Point3::new(0.0, 0.5, c), Vec3::unit_x());
        not_dropped(
            &read(&lin, (x0, 0.9), &dome()),
            &format!("line from the end, {k}"),
        );
        not_dropped(
            &read(&lin, (0.1, 0.5 + half), &dome()),
            &format!("line to the end, {k}"),
        );
        let fwd = segment([x0, 0.5, c], [0.9, 0.5, c]);
        not_dropped(
            &read_whole(&fwd, &dome()),
            &format!("nurbs from the end, {k}"),
        );
        let rev = segment([0.9, 0.5, c], [x0, 0.5, c]);
        not_dropped(
            &read_whole(&rev, &dome()),
            &format!("nurbs to the end, {k}"),
        );
    }
}

/// The NURBS cup tangent to the dome at its apex, its span ending a few
/// bands past the tangency: never clear.
#[test]
fn review_r1_a_tangency_near_an_end_is_not_dropped() {
    let s = band().escalate();
    for k in [4.0, 16.0, 64.0] {
        let dt = k * s / 0.8;
        not_dropped(
            &read(&cup(0.0), (0.0, 0.5 + dt), &dome()),
            &format!("end past, {k}"),
        );
        not_dropped(
            &read(&cup(0.0), (0.5 - dt, 1.0), &dome()),
            &format!("start before, {k}"),
        );
    }
}

/// A NURBS span ending on the flat chart's open edge `x = 0`, which it
/// re-crosses a few bands inside: the crossing is not the end's.
#[test]
fn review_r1_a_crossing_beside_an_end_on_the_charts_edge_is_not_dropped() {
    let s = band().escalate();
    for k in [2.0, 4.0, 16.0, 64.0] {
        let d = k * s;
        // z = A·x·(x − d) over x ∈ [0, 0.4], A = 4: on the edge at x = 0,
        // below the chart, back across it at x = d.
        let a = 4.0;
        let (x1, z1) = (0.2, -a * 0.2 * d);
        let z2 = a * 0.4 * (0.4 - d);
        let span = bezier(&[[0.0, 0.5, 0.0], [x1, 0.5, z1], [0.4, 0.5, z2]]);
        not_dropped(&read_whole(&span, &flat()), &format!("edge, {k}"));
    }
}

/// An end exactly on a crease of the span, the corner on the chart, is
/// the end's meeting. And one with the corner off the chart reads.
#[test]
fn review_r1_an_end_on_a_crease_of_the_span() {
    let on = read(&bent([0.5, 0.5, 0.0]), (0.0, 0.5), &flat());
    assert!(matches!(on.as_deref(), Ok([])), "{on:?}");
    let off = read(&bent([0.5, 0.5, 0.2]), (0.5, 1.0), &flat());
    assert!(matches!(off.as_deref(), Ok([])), "{off:?}");
}

/// The NURBS rows read the same at `Interval`.
#[test]
fn review_r1_the_nurbs_rows_read_alike_at_interval() {
    use geom_core::Interval;
    use geom_core::interval::certification::Certification;
    let lane = crate::NurbsLane::<Interval>::certified();
    let up = |c: &Curve3<f64>| c.map_scalar(Interval::point);
    let cases: Vec<(Curve3<f64>, NurbsSurface<f64>, usize)> = vec![
        (
            bezier(&[
                [0.2, 0.5, 0.6],
                [0.4, 0.5, -0.7],
                [0.6, 0.5, -0.7],
                [0.8, 0.5, 0.6],
            ]),
            waved(),
            2,
        ),
        (
            segment([0.4, 0.5, -0.5], [0.4, 0.5, 64.0 * band().escalate()]),
            flat(),
            1,
        ),
        (
            bezier(&[[0.5, 0.5, 0.0], [0.6, 0.5, 0.3], [0.6, 0.5, 0.6]]),
            flat(),
            0,
        ),
    ];
    for (c, chart, n) in cases {
        let got = lane.chart_meetings(
            &up(&c),
            (Interval::point(0.0), Interval::point(1.0)),
            &chart.map_scalar(Interval::point),
            band(),
        );
        assert_eq!(got.as_ref().map(Vec::len).ok(), Some(n), "{got:?}");
    }
}

/// The ruled sheet's enclosures hold the span's point and tangent over
/// windows across a non-crease knot, polynomial and rational.
#[test]
fn review_r1_the_sheets_enclosures_hold_the_span() {
    use geom_core::{Bounds, Interval};
    let cubic = spline(
        3,
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.6, 1.0, 1.0, 1.0, 1.0],
        &[
            [0.0, 0.0, 0.0],
            [0.2, 0.7, -0.3],
            [0.5, -0.4, 0.9],
            [0.7, 0.8, 0.1],
            [0.9, 0.1, -0.5],
            [1.0, 0.5, 0.4],
        ],
        Some(&[1.0, 0.5, 2.0, 0.7, 1.3, 1.0]),
    );
    let carrier = super::SpanCarrier::of(&cubic);
    let path = super::Path::of(&carrier).unwrap();
    let windows = [
        (0.25, 0.35),
        (0.29, 0.31),
        (0.0, 1.0),
        (0.55, 0.65),
        (0.3, 0.3),
        (0.1, 0.2),
    ];
    for (t0, t1) in windows {
        let w = Interval::from_bounds(t0, t1);
        let (p, d) = (path.at(w), path.tangent(w));
        for i in 0..=64 {
            let t = t0 + (t1 - t0) * f64::from(i) / 64.0;
            let c = cubic.eval(t);
            let h = 1e-6;
            let dc = (cubic.eval((t + h).min(1.0)) - cubic.eval((t - h).max(0.0)))
                * (1.0 / ((t + h).min(1.0) - (t - h).max(0.0)));
            for (k, (cv, dv)) in [(c.x, dc.x), (c.y, dc.y), (c.z, dc.z)]
                .into_iter()
                .enumerate()
            {
                let ulps = 1e-14 * (1.0 + cv.abs());
                assert!(
                    p[k].lo() - ulps <= cv && cv <= p[k].hi() + ulps,
                    "point {k} at {t}: {cv} in {:?}",
                    p[k]
                );
                let slack = 1e-4 * (1.0 + dv.abs());
                assert!(
                    d[k].lo() - slack <= dv && dv <= d[k].hi() + slack,
                    "tangent {k} at {t}: {dv} in {:?}",
                    d[k]
                );
            }
        }
    }
}

/// A NURBS segment crossing the closed cylinder's seam near its domain
/// end: refused or reported, never dropped.
#[test]
fn review_r1_a_nurbs_crossing_on_a_closed_charts_seam_near_a_domain_end_is_not_dropped() {
    let s = band().escalate();
    for k in [2.0, 8.0, 64.0] {
        let span = segment([0.5, 0.0, 0.5], [1.0 + k * s, 0.0, 0.5]);
        not_dropped(&read_whole(&span, &full_cylinder()), &format!("seam, {k}"));
        let back = segment([1.0 + k * s, 0.0, 0.5], [0.5, 0.0, 0.5]);
        not_dropped(
            &read_whole(&back, &full_cylinder()),
            &format!("seam reversed, {k}"),
        );
    }
}

/// **A crossing in the band's gap from a domain end is not read as the
/// end's**: a segment square to the flat patch ends `2·zero` above it, so
/// its crossing lies `2·zero` from the end, which no end's box may hold
/// (its bound is at least that distance); the crossing is then read
/// against the end in the band's gap, and escalates.
#[test]
fn a_crossing_in_the_bands_gap_from_a_domain_end_escalates() {
    let lift = 2.0 * band().zero();
    assert!(lift < band().escalate());
    let span = bezier(&[[0.4, 0.5, -0.5], [0.4, 0.5, lift]]);
    let got = read_whole(&span, &flat());
    assert!(
        matches!(got, Err(SplineFaceRefusal::Escalated(_))),
        "a crossing 2·zero from the end is neither the end's nor the span's: {got:?}"
    );
}

/// **A NURBS carrier read over a parameter that does not certify
/// refuses.** The sheet's boxes carry no decoration of `t`'s, so the
/// carrier's point and tangent over a refused point bracket on its
/// domain refuse where the same point certifies. Red under a window
/// read off the bracket's ends.
#[test]
fn a_nurbs_carrier_over_a_refused_parameter_refuses() {
    use geom_core::Interval;
    let curve = Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 2.0, 3.0)],
            vec![1.0; 2],
        )
        .unwrap(),
    ));
    let carrier = super::SpanCarrier::of(&curve);
    let path = super::Path::of(&carrier).unwrap();
    let certified = |v: [Interval; 3]| v.iter().all(|c| c.is_certified());
    let t = Interval::from_bounds(0.5, 0.5);
    assert!(certified(path.at(t)) && certified(path.tangent(t)));
    let refused = geom_core::test_support::refused_point(0.5);
    assert!(!certified(path.at(refused)), "the point over a refused t");
    assert!(
        !certified(path.tangent(refused)),
        "the tangent over a refused t"
    );
}
