//! An arc-held ellipse's cover and extent against points of its stored
//! curve, over eccentricity, size, seam, reversal and window (long,
//! asymmetric, away from the anchor and across it), at `f64` and at
//! `Interval`. Each sweep's seed is fixed: the rows are read against a
//! table of mutants, case by case.
//!
//! What these rows cannot see: the points are the ellipse's own
//! evaluator's (`Conic::point_at`), so a defect the evaluator and the
//! cover share is invisible here (`held_arc_sweep` judges against an
//! independent double-double evaluation); and each miss is read net of
//! twice the frame's skew (`Case::skew`), so a stored seam off square to
//! its axis, which puts a stored point off the extent, is excused
//! (`work/onecopy/a-conics-harmonic-takes-its-stored-seam-as-square-to-its-axis.md`).
#![allow(clippy::unwrap_used, clippy::print_stdout, clippy::float_cmp)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz::Rng;

/// How many roundings of the anchor's distance plus the semi-axis the
/// support along the arc's own normal may be wide (measured: 8.8).
const CEILING: f64 = 32.0;

use super::{ParamWindow, Site, ipt, iv, ivec, point, unit, unit_at};
use crate::{Conic, Curve3, Ellipse, Line};

/// One arc-held ellipse and a window, at f64 and as point intervals.
struct Case {
    e: Ellipse<f64>,
    ie: Ellipse<Interval>,
    lo: f64,
    hi: f64,
    scale: f64,
    /// How far the stored seam is off square to the stored axis: the
    /// readings take the frame as square, so a stored point may stand
    /// this much of the semi-axis off them (the base's convention).
    skew: f64,
    what: String,
}

fn cases(rng: &mut Rng, n: usize) -> Vec<Case> {
    let ratios = [1e-3, 0.1, 0.5, 0.8, 1.0, 1.25, 10.0, 1e3];
    let lens = [
        1e-7, 1e-4, 1e-2, 0.3, 1.0, FRAC_PI_2, 2.0, PI, 4.0, TAU, 7.0, 13.0,
    ];
    let seams = [
        None,
        Some(0.0),
        Some(FRAC_PI_2),
        Some(PI),
        Some(-FRAC_PI_2),
        Some(2.3),
    ];
    (0..n)
        .map(|i| {
            let a = rng.log_uniform(1e-2, 3e7);
            let b = a * ratios[rng.below(ratios.len())];
            let reach = [0.0, 1.0, 50.0, 1e3][rng.below(4)];
            let anchor = point(rng, reach);
            let axis = unit(rng);
            let toward = unit(rng);
            let seam = seams[rng.below(seams.len())];
            let rev = rng.below(4) == 0;
            let len = lens[rng.below(lens.len())];
            let start = match rng.below(3) {
                0 => -0.5 * len,
                1 => rng.range(-7.0, 7.0),
                _ => rng.range(-0.01, 0.01),
            };
            let (lo, hi) = if rng.below(5) == 0 {
                (start + len, start)
            } else {
                (start, start + len)
            };
            let turn = |e: Ellipse<f64>| {
                let e = seam.map_or(e, |t| e.seamed_near(t));
                if rev { e.reversed() } else { e }
            };
            let iturn = |e: Ellipse<Interval>| {
                let e = seam.map_or(e, |t| e.seamed_near(iv(t)));
                if rev { e.reversed() } else { e }
            };
            let e = turn(Ellipse::anchored(anchor, axis, toward, a, b));
            let ie = iturn(Ellipse::anchored(ipt(anchor), ivec(axis), ivec(toward), iv(a), iv(b)));
            Case {
                e,
                ie,
                lo,
                hi,
                scale: (anchor - Point3::origin()).norm() + a.max(b),
                skew: e.u_ref().dot(e.axis()).abs() + e.v_ref().dot(e.axis()).abs(),
                what: format!(
                    "#{i} a {a:e} b {b:e} seam {seam:?} rev {rev} window [{lo}, {hi}] anchor {anchor:?}"
                ),
            }
        })
        .collect()
}

fn samples(c: &Case, n: usize) -> Vec<Point3<f64>> {
    // One turn from `lo` holds every point of a longer window.
    let len = (c.hi - c.lo).clamp(-TAU, TAU);
    (0..=n)
        .map(|k| c.e.point_at(c.lo + len * (k as f64) / (n as f64)))
        .collect()
}

/// f64: no sampled point of the stored curve lies outside the ball, past
/// the support, or outside the distance range, by more than a few
/// roundings of the conic's scale.
#[test]
fn an_arc_held_cover_holds_its_stored_points_at_f64() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0001);
    let mut worst = [0.0_f64; 5];
    let mut worst_at = [const { String::new() }; 5];
    let mut slack_ball: f64 = 0.0;
    for c in cases(&mut rng, 6000) {
        let cover = Curve3::Ellipse(c.e)
            .cover_over(ParamWindow::new(c.lo, c.hi))
            .unwrap();
        let (mid, radius) = cover.ball();
        let pts = samples(&c, 400);
        let centre = Conic::center(&c.e);
        let sites = [
            Site::Point(
                pts[rng.below(pts.len())]
                    + unit(&mut rng) * rng.log_uniform(1e-6, 1.0) * c.e.speed_hi(),
            ),
            Site::Point(centre),
            Site::Point(point(&mut rng, 3.0 * c.scale)),
            Site::Point(pts[pts.len() / 2]),
            Site::Line(Line {
                origin: point(&mut rng, c.scale),
                dir: unit_at(unit(&mut rng)),
            }),
            Site::Line(Line {
                origin: centre,
                dir: unit_at(c.e.axis()),
            }),
            Site::Line(Line {
                origin: pts[0],
                dir: unit_at(unit(&mut rng)),
            }),
        ];
        let dirs = [
            c.e.axis(),
            c.e.u_ref(),
            c.e.v_ref(),
            -c.e.axis(),
            unit(&mut rng),
            unit(&mut rng),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        let sups = dirs.map(|d| cover.support(d));
        let ranges = sites.map(|s| cover.distance_range(&s));
        let dist = |s: &Site<f64>, p: Point3<f64>| match s {
            Site::Point(q) => (p - *q).norm(),
            Site::Line(l) => l.distance(p),
        };
        let mut note = |i: usize, miss: f64| {
            let rel = miss / c.scale - 2.0 * c.skew;
            if rel > worst[i] {
                worst[i] = rel;
                worst_at[i] = format!("{} miss {miss:e}", c.what);
            }
        };
        let mut far_from_mid: f64 = 0.0;
        for &p in &pts {
            let r = (p - mid).norm();
            far_from_mid = far_from_mid.max(r);
            note(0, r - radius);
            for (d, s) in dirs.iter().zip(sups) {
                note(1, (p - Point3::origin()).dot(*d) - s);
            }
            for (s, (near, far)) in sites.iter().zip(ranges) {
                let d = dist(s, p);
                note(2, near - d);
                note(3, d - far);
            }
        }
        for (near, far) in ranges {
            note(
                4,
                if near.is_finite() && far.is_finite() && near >= 0.0 {
                    0.0
                } else {
                    f64::INFINITY
                },
            );
        }
        if far_from_mid > 0.0 {
            slack_ball = slack_ball.max(radius / far_from_mid);
        }
    }
    let u = f64::EPSILON;
    for (i, w) in worst.iter().enumerate() {
        assert!(*w <= 16.0 * u, "probe {i}: {w:e} at {}", worst_at[i]);
    }
}

/// Interval: every reading is bounded, and holds the f64 sample points
/// to the samples' own rounding.
#[test]
fn an_arc_held_cover_encloses_its_stored_points_at_interval() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0002);
    let mut worst = [0.0_f64; 4];
    let mut worst_at = [const { String::new() }; 4];
    let mut unbounded = Vec::new();
    let mut huge = Vec::new();
    for c in cases(&mut rng, 3000) {
        let cover = Curve3::Ellipse(c.ie)
            .cover_over(ParamWindow::new(iv(c.lo), iv(c.hi)))
            .unwrap();
        let (mid, radius) = cover.ball();
        let pts = samples(&c, 200);
        let sites = [
            Site::Point(
                pts[rng.below(pts.len())]
                    + unit(&mut rng) * rng.log_uniform(1e-6, 1.0) * c.e.speed_hi(),
            ),
            Site::Point(point(&mut rng, 3.0 * c.scale)),
            Site::Line(Line {
                origin: point(&mut rng, c.scale),
                dir: unit_at(unit(&mut rng)),
            }),
        ];
        let dirs = [
            c.e.u_ref(),
            c.e.v_ref(),
            unit(&mut rng),
            unit(&mut rng),
            Vec3::new(1.0, 0.0, 0.0),
        ];
        let sups = dirs.map(|d| cover.support(ivec(d)));
        let isite = |s: &Site<f64>| match s {
            Site::Point(q) => Site::Point(ipt(*q)),
            Site::Line(l) => Site::Line(Line {
                origin: ipt(l.origin),
                dir: l.dir.map_scalar(Interval::from_f64),
            }),
        };
        let ranges = sites.map(|s| cover.distance_range(&isite(&s)));
        let mut all = vec![radius];
        all.extend(sups);
        all.extend(ranges.iter().flat_map(|r| [r.0, r.1]));
        if all.iter().any(|x| !x.is_bounded()) {
            unbounded.push(format!("{}: {all:?}", c.what));
            continue;
        }
        let w = all
            .iter()
            .map(|x| Bounds::hi(*x) - Bounds::lo(*x))
            .fold(0.0, f64::max);
        if w > 1e-3 * c.scale {
            huge.push(format!(
                "{}: widest reading {w:e} (scale {:e})",
                c.what, c.scale
            ));
        }
        let dist = |s: &Site<f64>, p: Point3<f64>| match s {
            Site::Point(q) => (p - *q).norm(),
            Site::Line(l) => l.distance(p),
        };
        let m = |x: Interval| 0.5 * (Bounds::lo(x) + Bounds::hi(x));
        let centre = Point3::new(m(mid.x), m(mid.y), m(mid.z));
        let spread = Bounds::hi((mid - ipt(centre)).norm());
        let mut note = |i: usize, miss: f64| {
            let rel = miss / c.scale - 2.0 * c.skew;
            if rel > worst[i] {
                worst[i] = rel;
                worst_at[i] = format!("{} miss {miss:e}", c.what);
            }
        };
        for &p in &pts {
            note(0, (p - centre).norm() - spread - Bounds::hi(radius));
            for (d, s) in dirs.iter().zip(sups) {
                note(1, (p - Point3::origin()).dot(*d) - Bounds::hi(s));
            }
            for (s, (near, far)) in sites.iter().zip(ranges) {
                let d = dist(s, p);
                note(2, Bounds::lo(near) - d);
                note(3, d - Bounds::hi(far));
            }
        }
    }
    let u = f64::EPSILON;
    for (i, w) in worst.iter().enumerate() {
        // An enclosure misses nothing: the bar is the rounding of the
        // oracle's own last difference.
        assert!(*w <= 0.5 * u, "probe {i}: {w:e} at {}", worst_at[i]);
    }
    assert!(unbounded.is_empty() && huge.is_empty());
}

/// **At `Interval`, an arc-held ellipse's support along its own stored
/// normal is wide by a rounding of its semi-axis, and no more.** Along
/// the normal the harmonic's two coefficients enclose zero, each a
/// rounding of the semi-axis wide, and the support is the anchor's
/// height within that: under 32 roundings of the anchor's distance plus
/// the semi-axis, at every size from 2 m to 2.2e7 m, over a short, a
/// long and a whole window, on a frame a rounding off a world axis (as
/// a placed part's is) and on a general one. That is the width the same
/// locus held at its centre reads; it is not the arc's own scale. Red
/// where the crest is a ratio whose divisor encloses zero (a width of
/// `1e280`), and where the reading widens by a constant `4e-7` m.
#[test]
fn the_interval_support_along_the_arcs_own_normal_is_a_rounding_of_its_semi_axis() {
    let (sn, cs) = FRAC_PI_2.sin_cos();
    let frames = [
        (Vec3::new(cs, 0.0, sn), Vec3::new(0.6 * sn, 0.8, -0.6 * cs)),
        (
            Vec3::new(1.0, 2.0, 2.0) / 3.0,
            Vec3::new(2.0, 1.0, -2.0) / 3.0,
        ),
    ];
    let anchor = Point3::new(40.25, -7.5, 3.125);
    let w = |x: Interval| Bounds::hi(x) - Bounds::lo(x);
    for (n, toward) in frames {
        for a in [2.0, 2.2e1, 2.2e4, 2.2e7] {
            let ie = Ellipse::anchored(ipt(anchor), ivec(n), ivec(toward), iv(a), iv(0.8 * a));
            for (lo, hi) in [(-1e-3, 1e-3), (0.0, 1.0), (0.0, TAU)] {
                let win = ParamWindow::new(iv(lo), iv(hi));
                let cover = Curve3::Ellipse(ie).cover_over(win).unwrap();
                let size = (anchor - Point3::origin()).norm() + a;
                for dir in [n, -n] {
                    let width = w(cover.support(ivec(dir)));
                    assert!(
                        width <= CEILING * f64::EPSILON * size,
                        "a {a:e}, window [{lo}, {hi}], along {dir:?}: the support is \
                         {width:e} wide, {:.1} roundings of {size:e}",
                        width / (f64::EPSILON * size)
                    );
                }
            }
        }
    }
}

/// Both ends of the extent door, f64 and Interval, against sampled
/// points of the stored curve (the cover reads only the upper end).
#[test]
fn an_arc_held_extent_holds_both_its_ends() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0003);
    let mut worst = [0.0_f64; 4];
    let mut worst_at = [const { String::new() }; 4];
    let mut slack: f64 = 0.0;
    for c in cases(&mut rng, 4000) {
        let pts = samples(&c, 400);
        let o = point(&mut rng, c.scale);
        let dirs = [
            c.e.u_ref(),
            c.e.v_ref(),
            -c.e.u_ref(),
            -c.e.v_ref(),
            unit(&mut rng),
            unit(&mut rng),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        for d in dirs {
            let (lo, hi) = crate::curves::boxes::conic_arc_extent_along(
                &Curve3::Ellipse(c.e),
                c.lo,
                c.hi,
                o,
                d,
            )
            .unwrap();
            let (ilo, ihi) = crate::curves::boxes::conic_arc_extent_along(
                &Curve3::Ellipse(c.ie),
                iv(c.lo),
                iv(c.hi),
                ipt(o),
                ivec(d),
            )
            .unwrap();
            let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
            for &p in &pts {
                let h = (p - o).dot(d);
                smin = smin.min(h);
                smax = smax.max(h);
            }
            let scale = c.scale + (o - Point3::origin()).norm();
            for (i, miss) in [
                lo - smin,
                smax - hi,
                Bounds::lo(ilo) - smin,
                smax - Bounds::hi(ihi),
            ]
            .into_iter()
            .enumerate()
            {
                if miss / scale - 2.0 * c.skew > worst[i] {
                    worst[i] = miss / scale - 2.0 * c.skew;
                    worst_at[i] = format!("{} dir {d:?} miss {miss:e}", c.what);
                }
            }
            // How loose: the f64 extent past the sampled one, over the conic's size.
            slack = slack.max(((smin - lo).max(hi - smax)) / c.e.speed_hi());
        }
    }
    for (i, w) in worst.iter().enumerate() {
        assert!(
            *w <= 16.0 * f64::EPSILON,
            "probe {i}: {w:e} at {}",
            worst_at[i]
        );
    }
}
