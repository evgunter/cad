//! Review r2 of PR 5855: probes of the NURBS span arm's end's box, its
//! span-crease cut and its ruled sheet's enclosures.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, NurbsCurve3, NurbsSurface};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Point3, Tol, Vec3};

use super::{ChartMeeting, SplineFaceRefusal};
use crate::roots::Unsettled;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

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

fn flat() -> NurbsSurface<f64> {
    graph(1, 1, &[vec![0.0, 0.0], vec![0.0, 0.0]])
}

fn dome() -> NurbsSurface<f64> {
    let a = [0.25, -0.25, 0.25];
    let z: Vec<Vec<f64>> = (0..3)
        .map(|i| (0..3).map(|j| a[i] + a[j]).collect())
        .collect();
    graph(2, 2, &z)
}

fn nurbs(
    p: usize,
    knots: Vec<f64>,
    control: &[[f64; 3]],
    weights: Option<&[f64]>,
) -> NurbsCurve3<f64> {
    let control: Vec<Point3<f64>> = control
        .iter()
        .map(|&[x, y, z]| Point3::new(x, y, z))
        .collect();
    let weights = weights.map_or_else(|| vec![1.0; control.len()], <[f64]>::to_vec);
    NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, weights).unwrap()
}

fn spline(p: usize, knots: Vec<f64>, control: &[[f64; 3]], weights: Option<&[f64]>) -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(nurbs(p, knots, control, weights)))
}

fn bezier(control: &[[f64; 3]]) -> Curve3<f64> {
    let p = control.len() - 1;
    let mut knots = vec![0.0; p + 1];
    knots.extend(vec![1.0; p + 1]);
    spline(p, knots, control, None)
}

/// The Bézier of the power-basis polynomials `coeffs[k]` (per coordinate,
/// lowest power first) over `s ∈ [0, 1]`.
fn power(coeffs: [&[f64]; 3]) -> Curve3<f64> {
    let n = coeffs[0].len() - 1;
    let binom = |n: usize, k: usize| -> f64 {
        (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
    };
    let control: Vec<[f64; 3]> = (0..=n)
        .map(|k| {
            core::array::from_fn(|c| {
                (0..=k)
                    .map(|j| binom(k, j) / binom(n, j) * coeffs[c][j])
                    .sum()
            })
        })
        .collect();
    bezier(&control)
}

fn read(
    carrier: &Curve3<f64>,
    params: (f64, f64),
    chart: &NurbsSurface<f64>,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    crate::NurbsLane::<f64>::certified().chart_meetings(carrier, params, chart, band())
}

/// A swallow: the walk answered with no meeting where a meeting lies
/// `dist` metres from the nearest end, past `esc`.
fn swallowed(got: &Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal>, dist: f64) -> bool {
    matches!(got.as_deref(), Ok([])) && dist > band().escalate()
}

/// **An end exactly on a crease of the span, on the chart, is the end's
/// meeting**, as an end at the carrier's domain end is
/// (`a_span_ending_on_the_chart_at_its_domains_end_is_the_ends`) and as a
/// line's end is.
#[test]
fn review_r2_an_end_on_a_crease_of_the_span_on_the_chart_is_the_ends() {
    let bent = spline(
        1,
        vec![0.0, 0.0, 0.5, 1.0, 1.0],
        &[[0.3, 0.5, -0.5], [0.5, 0.5, 0.0], [0.6, 0.5, 0.5]],
        None,
    );
    // The same legs, each its own carrier: the end at the domain's end.
    let leg = bezier(&[[0.3, 0.5, -0.5], [0.5, 0.5, 0.0]]);
    let line = Curve3::Line {
        origin: Point3::new(0.3, 0.5, -0.5),
        dir: Vec3::new(0.2, 0.0, 0.5),
    };
    let by_leg = read(&leg, (0.0, 1.0), &flat());
    let by_line = read(&line, (0.0, 1.0), &flat());
    let first = read(&bent, (0.0, 0.5), &flat());
    let second = read(&bent, (0.5, 1.0), &flat());
    assert!(
        matches!(by_leg.as_deref(), Ok([])),
        "the leg alone: {by_leg:?}"
    );
    assert!(
        matches!(by_line.as_deref(), Ok([])),
        "the line: {by_line:?}"
    );
    assert!(
        matches!(first.as_deref(), Ok([])) && matches!(second.as_deref(), Ok([])),
        "an end on the span's crease, on the chart, is the end's: {first:?} {second:?}"
    );
}

/// **A crossing near a domain end is never absorbed by the end's box**:
/// the end `lift` above the flat patch, the span arriving at `slope`
/// (rise over run); the crossing lies `lift·√(1 + 1/slope²)` from the end.
#[test]
fn review_r2_a_crossing_near_a_domain_end_is_reported_or_refused() {
    let (z, e) = (band().zero(), band().escalate());
    let mut bad = Vec::new();
    for slope in [1e6, 1.0, 1e-2, 1e-4] {
        for lift in [
            0.5 * z,
            2.0 * z,
            0.5 * e,
            1.5 * e,
            4.0 * e,
            32.0 * e,
            1e3 * e,
        ] {
            let run = 0.4f64;
            let rise = run * slope;
            // The span must reach below the patch for there to be a crossing.
            if slope < 1e3 && lift >= 0.5 * rise {
                continue;
            }
            let (x0, z0) = if slope > 1e3 {
                (0.4, lift - 0.5)
            } else {
                (0.4 - run, lift - rise)
            };
            let span = bezier(&[[x0, 0.5, z0], [0.4, 0.5, lift]]);
            let got = read(&span, (0.0, 1.0), &flat());
            let dist = lift * (1.0 + 1.0 / (slope * slope)).sqrt();
            let n = got.as_ref().map(Vec::len);
            eprintln!(
                "slope {slope:e} lift {lift:e} dist {dist:e}: {n:?} {:?}",
                got.as_ref().err()
            );
            if swallowed(&got, dist) {
                bad.push((slope, lift, dist));
            }
            // Reversed: the end is the span's start.
            let back = bezier(&[[0.4, 0.5, lift], [x0, 0.5, z0]]);
            let got = read(&back, (0.0, 1.0), &flat());
            if swallowed(&got, dist) {
                bad.push((-slope, lift, dist));
            }
        }
    }
    assert!(bad.is_empty(), "swallowed (slope, lift, dist): {bad:?}");
}

/// **A tangency near a domain end refuses**: the cup touching the dome at
/// its apex, split `k·escalate` (in `x`) past the touch, so the touch lies
/// that far inside one piece's end.
#[test]
fn review_r2_a_tangency_near_a_domain_end_refuses() {
    let e = band().escalate();
    let cup = nurbs(
        2,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[[0.1, 0.5, 0.2], [0.5, 0.5, -0.2], [0.9, 0.5, 0.2]],
        None,
    );
    let mut bad = Vec::new();
    for k in [2.0, 4.0, 16.0, 128.0] {
        let d = k * e / 0.8;
        for (c, keep_left) in [(0.5 + d, true), (0.5 - d, false)] {
            let (l, r) = cup.split_at(c).unwrap();
            let piece = if keep_left { l } else { r };
            let (lo, hi) = piece.domain();
            let carrier = Curve3::Nurbs(std::sync::Arc::new(piece));
            let got = read(&carrier, (lo, hi), &dome());
            eprintln!("k {k} left {keep_left}: {got:?}");
            if !matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))) {
                bad.push((k, keep_left, format!("{got:?}")));
            }
        }
    }
    assert!(bad.is_empty(), "a tangency near an end answered: {bad:?}");
}

/// **A second crossing inside the end's reach is reported or refused**:
/// `z = A·(x − x_end)·(x − x_end + d)` over `x ∈ [x_end − 0.4, x_end]`, so
/// the end lies on the flat patch and a second crossing lies `d` short of
/// it; `x_end = 1` puts the end on the patch's edge.
#[test]
fn review_r2_a_second_crossing_beside_an_end_on_the_chart_is_not_absorbed() {
    let e = band().escalate();
    let mut bad = Vec::new();
    for x_end in [0.6, 1.0] {
        for amp in [1.0, 100.0] {
            for k in [2.0, 8.0, 64.0, 1024.0] {
                let d = k * e;
                // x = x_end − 0.4 + 0.4·s; z = A·(0.4s − 0.4)·(0.4s − 0.4 + d).
                let a = amp;
                let (p, q) = (-0.4, -0.4 + d);
                // (0.4s + p)(0.4s + q) = 0.16 s² + 0.4 (p + q) s + p q
                let zc = [a * p * q, a * 0.4 * (p + q), a * 0.16];
                let span = power([&[x_end - 0.4, 0.4, 0.0], &[0.5, 0.0, 0.0], &zc]);
                let got = read(&span, (0.0, 1.0), &flat());
                eprintln!("x_end {x_end} A {amp} k {k}: {got:?}");
                if swallowed(&got, d) {
                    bad.push((x_end, amp, k));
                }
            }
        }
    }
    assert!(bad.is_empty(), "absorbed (x_end, A, k): {bad:?}");
}

/// **A touch beside an end on the open chart's edge refuses** (the shape
/// PR 5826 pruned): `z = A·x·(x − x₁)²`, `x ∈ [0, 0.3]`, from inside the
/// flat patch to its edge at `x = 0`, the end on the edge and a touch
/// `x₁` inside it.
#[test]
fn review_r2_a_touch_beside_an_end_on_the_charts_edge_refuses() {
    let e = band().escalate();
    let mut bad = Vec::new();
    for amp in [1.0, 1e3] {
        for k in [2.0, 8.0, 64.0, 1024.0] {
            let x1 = k * e;
            // x = 0.3·(1 − s): end at s = 1. z = A x (x − x1)².
            // Expand in x then substitute: done numerically on the power basis of s.
            let xs = [0.3, -0.3];
            let mul = |a: &[f64], b: &[f64]| {
                let mut out = vec![0.0; a.len() + b.len() - 1];
                for (i, &u) in a.iter().enumerate() {
                    for (j, &w) in b.iter().enumerate() {
                        out[i + j] += u * w;
                    }
                }
                out
            };
            let xm = [0.3 - x1, -0.3];
            let z = mul(&mul(&xs, &xm), &xm)
                .iter()
                .map(|c| c * amp)
                .collect::<Vec<_>>();
            let span = power([&[0.3, -0.3, 0.0, 0.0], &[0.5, 0.0, 0.0, 0.0], &z]);
            let got = read(&span, (0.0, 1.0), &flat());
            eprintln!("A {amp} k {k}: {got:?}");
            if !matches!(got, Err(SplineFaceRefusal::Unsettled(Unsettled::Floor))) {
                bad.push((amp, k, format!("{got:?}")));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "a touch beside an edge end answered: {bad:?}"
    );
}

/// **The ruled sheet's boxes enclose the carrier and its derivative**
/// over windows across a non-crease knot, rational, at `f64` and at
/// `Interval`.
#[test]
fn review_r2_the_sheets_boxes_enclose_the_carrier_and_its_derivative() {
    let curve = nurbs(
        3,
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.55, 1.0, 1.0, 1.0, 1.0],
        &[
            [0.0, 0.0, 0.0],
            [0.3, 0.9, -0.2],
            [0.5, -0.4, 0.7],
            [0.9, 0.2, 0.1],
            [1.1, 1.0, -0.6],
            [1.5, 0.1, 0.3],
        ],
        Some(&[1.0, 2.5, 0.4, 1.7, 0.6, 1.0]),
    );
    let carrier = Curve3::Nurbs(std::sync::Arc::new(curve.clone()));
    let sheet = super::sheet(&carrier).unwrap();
    let boxes = NurbsBoxes::new(&sheet);
    let icarrier = Curve3::Nurbs(std::sync::Arc::new(
        curve.map_scalar(geom_core::Interval::point),
    ));
    let isheet = super::sheet(&icarrier).unwrap();
    let iboxes = NurbsBoxes::new(&isheet);
    let windows = [
        (0.0, 1.0),
        (0.25, 0.35),
        (0.29, 0.31),
        (0.3, 0.3),
        (0.5, 0.6),
        (0.549_999, 0.550_001),
        (0.0, 0.0),
        (1.0, 1.0),
        (0.1, 0.9),
    ];
    let mut worst = 0.0f64;
    for (t0, t1) in windows {
        let w = UvWindow::new(t0, t1, 0.0, 0.0).unwrap();
        for (name, b, d) in [
            ("f64", boxes.rect_box(w), boxes.deriv_box(w, true)),
            ("interval", iboxes.rect_box(w), iboxes.deriv_box(w, true)),
        ] {
            for i in 0..=200 {
                let t = t0 + (t1 - t0) * f64::from(i) / 200.0;
                let p = curve.eval(t);
                let v = curve.deriv(t);
                for (iv, x) in [
                    (b.x, p.x),
                    (b.y, p.y),
                    (b.z, p.z),
                    (d.x, v.x),
                    (d.y, v.y),
                    (d.z, v.z),
                ] {
                    let out = (iv.lo() - x).max(x - iv.hi()).max(0.0);
                    if out > 1e-12 {
                        eprintln!("{name} [{t0}, {t1}] at {t}: {x} outside {iv:?}");
                    }
                    worst = worst.max(out);
                }
            }
        }
    }
    assert!(
        worst <= 1e-12,
        "the sheet's boxes miss the carrier by {worst:e}"
    );
}

/// **A sharp second crossing beside an end on the chart is not absorbed**:
/// the quadratic `P0 = (x_end, ½, H)`, `P1 = (x_end, ½, −G)`, `P2 = (x_end, ½, 0)`
/// shifted along `x` so it is not vertical, ends on the flat patch and
/// crosses it again `≈ d` short of the end, dipping `≈ d/2` between.
#[test]
fn review_r2_a_sharp_second_crossing_beside_an_end_is_not_absorbed() {
    let e = band().escalate();
    let mut bad = Vec::new();
    for x_end in [0.6, 1.0] {
        for k in [4.0, 16.0, 64.0, 1024.0] {
            let d = k * e;
            let g = 0.1;
            let h = 4.0 * g * g / d;
            let span = bezier(&[
                [x_end - 0.3, 0.5, h],
                [x_end - 0.01, 0.5, -g],
                [x_end, 0.5, 0.0],
            ]);
            let got = read(&span, (0.0, 1.0), &flat());
            eprintln!("x_end {x_end} k {k}: {got:?}");
            if matches!(got.as_deref(), Ok([])) {
                bad.push((x_end, k));
            }
        }
    }
    assert!(bad.is_empty(), "absorbed (x_end, k): {bad:?}");
}

/// **The analytic arm moved**: a line ending on the open chart's edge
/// refused `Tangency` at the merge base and reads the end's (`Ok([])`)
/// here, through the end's box. No row of the PR pins the analytic case.
#[test]
fn review_r2_a_line_ending_on_the_charts_edge_is_the_ends() {
    let edge = Curve3::Line {
        origin: Point3::new(0.3, 0.5, -0.3),
        dir: Vec3::new(-0.3, 0.0, 0.3),
    };
    let got = read(&edge, (0.0, 1.0), &flat());
    assert!(matches!(got.as_deref(), Ok([])), "the end's: {got:?}");
}
