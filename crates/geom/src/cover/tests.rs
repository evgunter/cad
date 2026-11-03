//! Every piece's methods hold its locus, at `f64` and at `Interval`.
//!
//! Each row builds a piece from `f64` data at both scalars and samples
//! its locus as `Interval` enclosures of exact points of that data:
//! densely, and at the points where each bound is attained (a support's
//! crest, the nearest and farthest point from a site), so a bound that
//! falls short of an attained point goes red. What each scalar owes is
//! `check`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;

use geom_core::{Band, Bounds, Dual, Interval, KnotVector, Point3, Real, Sym, UnitVec3, Vec3};
use test_utils::fuzz::{self, Rng};

use super::{Cover, Covered, Entity, Frame, Lattice, ParamWindow, Site, Uncovered, Unknown};
use crate::curves::NurbsCurve3;
use crate::{Curve3, Line, Surface, SurfaceKind};

test_utils::gated_to![
    "crates/geom/src/curves/boxes.rs",
    "crates/geom/src/curves.rs"
];

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn ipt(p: Point3<f64>) -> Point3<Interval> {
    p.map(iv)
}

fn ivec(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(iv(v.x), iv(v.y), iv(v.z))
}

fn unit(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if v.norm() > 0.2 {
            return v.normalize();
        }
    }
}

fn point(rng: &mut Rng, scale: f64) -> Point3<f64> {
    Point3::new(
        rng.range(-scale, scale),
        rng.range(-scale, scale),
        rng.range(-scale, scale),
    )
}

/// A unit `u_ref` across `axis`.
fn across(axis: Vec3<f64>, rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = unit(rng).cross(axis);
        if v.norm() > 0.2 {
            return v.normalize();
        }
    }
}

/// The solid ball about `c` of radius `|r|`: a sphere's whole cover, the
/// tube with no spine.
fn ball<T: Real>(c: Point3<T>, r: T) -> Cover<T> {
    Cover::tube(c, Vec3::unit_z(), Vec3::unit_x(), T::zero(), r).unwrap()
}

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

fn unit_at<T: geom_core::Decide>(v: Vec3<T>) -> UnitVec3<T> {
    UnitVec3::new(v, "cover_test_axis", band()).unwrap()
}

/// The angle of `p·cos t + q·sin t`'s crest, and its trough's.
fn crests(p: f64, q: f64) -> [f64; 2] {
    let t = q.atan2(p);
    [t, t + PI]
}

/// The angles of `ts` that fall in `[lo, hi]` modulo a turn, plus the
/// two ends.
fn in_window(ts: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let (lo, hi) = (lo.min(hi), lo.max(hi));
    let mut out = vec![lo, hi];
    for &t in ts {
        let mut x = t;
        while x > lo {
            x -= TAU;
        }
        while x < lo {
            x += TAU;
        }
        while x <= hi {
            out.push(x);
            x += TAU;
        }
    }
    out
}

/// `n + 1` parameters from `lo` to `hi`, both ends exact and every one
/// inside the span, which a rounded step can leave.
fn dense(lo: f64, hi: f64, n: usize) -> impl Iterator<Item = f64> {
    let (a, b) = (lo.min(hi), lo.max(hi));
    (0..=n).map(move |i| match i {
        0 => lo,
        _ if i == n => hi,
        _ => (lo + (hi - lo) * (i as f64) / (n as f64)).clamp(a, b),
    })
}

/// One piece at both scalars, its locus samples, the directions and sites
/// its extremal samples were taken for.
struct Case {
    name: String,
    at_f64: Cover<f64>,
    at_iv: Cover<Interval>,
    samples: Vec<Point3<Interval>>,
    dirs: Vec<Vec3<f64>>,
    sites: Vec<Site<f64>>,
}

fn lift_site(site: &Site<f64>) -> Site<Interval> {
    match site {
        Site::Point(q) => Site::Point(ipt(*q)),
        Site::Line(l) => Site::Line(Line {
            origin: ipt(l.origin),
            dir: ivec(l.dir),
        }),
    }
}

fn distance(s: Point3<Interval>, site: &Site<f64>) -> Interval {
    match site {
        Site::Point(q) => (s - ipt(*q)).norm(),
        Site::Line(l) => {
            let w = s - ipt(l.origin);
            let d = ivec(l.dir);
            (w - d * (w.dot(d) / d.dot(d))).norm()
        }
    }
}

/// **The two scalars' obligations** (`[simple.conditioned]`): at
/// `Interval` every bound encloses every sample, with nothing to spare;
/// at `f64` every bound is the same real-arithmetic value to rounding,
/// so it lies in the interval bound's enclosure up to `tol`, and holds
/// the samples up to `tol`, both a test-side allowance for `f64`'s own
/// rounding and never a charge the cover carries.
#[allow(clippy::too_many_lines)]
fn check(case: &Case) {
    let what = |m: &str| format!("{}: {m}; {}", case.name, fuzz::replay());
    assert!(!case.samples.is_empty(), "{}", what("no samples"));
    let scale = case
        .samples
        .iter()
        .map(|s| (*s - Point3::origin()).norm().hi())
        .chain(case.sites.iter().map(|site| match site {
            Site::Point(q) => (*q - Point3::origin()).norm(),
            Site::Line(l) => (l.origin - Point3::origin()).norm(),
        }))
        .fold(1e-300, f64::max);
    let tol = 64.0 * f64::EPSILON * scale;
    let same = |f: f64, i: Interval, m: &str| {
        assert!(
            i.lo() - tol <= f && f <= i.hi() + tol,
            "{}",
            what(&format!(
                "f64 {m} {f} is not the interval's {i:?} to rounding"
            ))
        );
    };
    let mut frames = vec![Frame::World];
    frames.extend(case.dirs.iter().map(|&e| Frame::aimed(unit_at(e))));
    for &e in &case.dirs {
        let (sf, si) = (case.at_f64.support(e), case.at_iv.support(ivec(e)));
        same(sf, si, "support");
        for s in &case.samples {
            let x = (*s - Point3::origin()).dot(ivec(e)).lo();
            assert!(
                x <= si.hi(),
                "{}",
                what(&format!("interval support {si:?} along {e:?} short of {x}"))
            );
            assert!(
                x <= sf + tol,
                "{}",
                what(&format!("f64 support {sf} along {e:?} short of {x}"))
            );
        }
    }
    for frame in &frames {
        let rows = frame.rows();
        let (lf, hf) = case.at_f64.box_in(frame);
        let (li, hi) = case.at_iv.box_in(&Frame::Rows(rows.map(ivec)));
        let lows = [(lf.x, li.x), (lf.y, li.y), (lf.z, li.z)];
        let highs = [(hf.x, hi.x), (hf.y, hi.y), (hf.z, hi.z)];
        for k in 0..3 {
            same(lows[k].0, lows[k].1, "box low");
            same(highs[k].0, highs[k].1, "box high");
        }
        for s in &case.samples {
            let v = *s - Point3::origin();
            for (k, &r) in rows.iter().enumerate() {
                let c = v.dot(ivec(r));
                assert!(
                    lows[k].1.lo() <= c.hi() && lows[k].0 <= c.hi() + tol,
                    "{}",
                    what(&format!("box row {k} low {:?} above {c:?}", lows[k]))
                );
                assert!(
                    highs[k].1.hi() >= c.lo() && highs[k].0 >= c.lo() - tol,
                    "{}",
                    what(&format!("box row {k} high {:?} below {c:?}", highs[k]))
                );
            }
        }
    }
    let aabb = case.at_iv.aabb(0.0);
    let (cf, rf) = case.at_f64.ball();
    let (ci, ri) = case.at_iv.ball();
    same(rf, ri, "ball radius");
    for s in &case.samples {
        assert!(
            aabb.min_x <= s.x.hi()
                && s.x.lo() <= aabb.max_x
                && aabb.min_y <= s.y.hi()
                && s.y.lo() <= aabb.max_y
                && aabb.min_z <= s.z.hi()
                && s.z.lo() <= aabb.max_z,
            "{}",
            what(&format!("interval aabb {aabb:?} misses {s:?}"))
        );
        let di = (*s - ci).norm().lo();
        assert!(
            di <= ri.hi(),
            "{}",
            what(&format!("interval ball {ri:?} short of {di}"))
        );
        let df = (*s - ipt(cf)).norm().lo();
        assert!(
            df <= rf + tol,
            "{}",
            what(&format!("f64 ball {rf} short of {df}"))
        );
    }
    for site in &case.sites {
        let (nf, ff) = case.at_f64.distance_range(site);
        let (ni, fi) = case.at_iv.distance_range(&lift_site(site));
        assert!(
            nf >= 0.0 && ni.lo() >= 0.0,
            "{}",
            what("a negative nearest distance")
        );
        same(ff, fi, "farthest");
        same(nf, ni, "nearest");
        for s in &case.samples {
            let d = distance(*s, site);
            assert!(
                ni.lo() <= d.hi() && nf <= d.hi() + tol,
                "{}",
                what(&format!("nearest {nf}/{ni:?} above {d:?} from {site:?}"))
            );
            assert!(
                fi.hi() >= d.lo() && ff >= d.lo() - tol,
                "{}",
                what(&format!("farthest {ff}/{fi:?} below {d:?} from {site:?}"))
            );
        }
    }
}

/// Directions for the extremal samples, both senses of each.
fn directions(rng: &mut Rng, n: usize) -> Vec<Vec3<f64>> {
    let mut dirs = vec![Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    dirs.extend((0..n).map(|_| unit(rng)));
    dirs.iter().flat_map(|&e| [e, -e]).collect()
}

fn sites(rng: &mut Rng, center: Point3<f64>, scale: f64) -> Vec<Site<f64>> {
    let mut out = vec![Site::Point(center)];
    for _ in 0..2 {
        out.push(Site::Point(
            center + (point(rng, 2.0 * scale) - Point3::origin()),
        ));
        out.push(Site::Line(Line {
            origin: center + (point(rng, 2.0 * scale) - Point3::origin()),
            dir: unit(rng) * rng.range(0.5, 2.0),
        }));
    }
    out
}

fn site_points(sites: &[Site<f64>]) -> Vec<Point3<f64>> {
    sites
        .iter()
        .filter_map(|s| match s {
            Site::Point(q) => Some(*q),
            Site::Line(_) => None,
        })
        .collect()
}

/// A torus or sphere surface's samples: dense over `us × vs`, plus the
/// crest of every direction and the nearest and farthest point from every
/// site point that falls in the windows.
fn chart_samples(
    surface: &Surface<f64>,
    (center, axis, u_ref, major): (Point3<f64>, Vec3<f64>, Vec3<f64>, f64),
    us: (f64, f64),
    vs: (f64, f64),
    dirs: &[Vec3<f64>],
    points: &[Point3<f64>],
) -> Vec<Point3<Interval>> {
    let at = surface.map_scalar(iv);
    let v_ref = axis.cross(u_ref);
    let mut params: Vec<(f64, f64)> = Vec::new();
    for u in dense(us.0, us.1, 12) {
        for v in dense(vs.0, vs.1, 12) {
            params.push((u, v));
        }
    }
    for &e in dirs {
        let (p, q, a) = (u_ref.dot(e), v_ref.dot(e), axis.dot(e));
        for u in in_window(&crests(p, q), us.0, us.1) {
            let radial = u_ref * u.cos() + v_ref * u.sin();
            for v in in_window(&crests(radial.dot(e), a), vs.0, vs.1) {
                params.push((u, v));
            }
        }
    }
    for &q in points {
        let w = q - center;
        for u in in_window(&crests(u_ref.dot(w), v_ref.dot(w)), us.0, us.1) {
            let radial = u_ref * u.cos() + v_ref * u.sin();
            let d = q - (center + radial * major);
            for v in in_window(&crests(radial.dot(d), axis.dot(d)), vs.0, vs.1) {
                params.push((u, v));
            }
        }
    }
    params
        .into_iter()
        .map(|(u, v)| at.eval(iv(u), iv(v)))
        .collect()
}

fn curve_samples(curve: &Curve3<f64>, ts: impl IntoIterator<Item = f64>) -> Vec<Point3<Interval>> {
    let at = curve.map_scalar(iv);
    ts.into_iter().map(|t| at.eval(iv(t))).collect()
}

/// **A sphere's and a torus's whole cover hold the carrier.** Tilted
/// frames, odd cases skewed (axis and `u_ref` neither unit nor
/// orthogonal), both signs of each radius, tori out to spindle data:
/// every method holds for any frame and sign.
#[test]
fn the_whole_carrier_covers_hold_the_sphere_and_the_torus() {
    let mut rng = fuzz::start("cover::the_whole_carrier_covers_hold_the_sphere_and_the_torus");
    for case in 0..fuzz::scaled(6) {
        let scale = rng.log_uniform(1e-3, 1e3);
        let center = point(&mut rng, 10.0 * scale);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let (axis, u_ref) = if case % 2 == 1 {
            let tilt = axis * rng.range(-0.5, 0.5);
            (
                axis * rng.range(0.5, 2.0),
                (u_ref + tilt) * rng.range(0.5, 2.0),
            )
        } else {
            (axis, u_ref)
        };
        let sign = if rng.unit() < 0.25 { -1.0 } else { 1.0 };
        let minor_sign = if rng.unit() < 0.25 { -1.0 } else { 1.0 };
        let dirs = directions(&mut rng, 3);
        let sites = sites(&mut rng, center, 2.0 * scale);
        let points = site_points(&sites);
        let r = scale * rng.range(0.3, 1.0);
        let sphere = Surface::Sphere {
            center,
            radius: sign * r,
            axis,
            u_ref,
        };
        check(&Case {
            name: format!("sphere #{case}"),
            at_f64: sphere.cover().unwrap(),
            at_iv: sphere.map_scalar(iv).cover().unwrap(),
            samples: chart_samples(
                &sphere,
                (center, axis, u_ref, 0.0),
                (0.0, TAU),
                (-FRAC_PI_2, FRAC_PI_2),
                &dirs,
                &points,
            ),
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
        let (big, small) = (scale, scale * rng.range(0.05, 1.2));
        let torus = Surface::Torus {
            center,
            axis,
            major_radius: sign * big,
            minor_radius: minor_sign * small,
            u_ref,
        };
        check(&Case {
            name: format!("torus #{case}"),
            at_f64: torus.cover().unwrap(),
            at_iv: torus.map_scalar(iv).cover().unwrap(),
            samples: chart_samples(
                &torus,
                (center, axis, u_ref, sign * big),
                (0.0, TAU),
                (0.0, TAU),
                &dirs,
                &points,
            ),
            dirs,
            sites,
        });
    }
}

/// **The tube's support is exact**: along a unit `e`, `c·e + R·|e⊥| + r`
/// to within its charge, not the box's `(R + r)·|e⊥| + r·|eₐ|`.
#[test]
fn the_tubes_support_is_r_times_the_cross_part_plus_r() {
    let mut rng = fuzz::start("cover::the_tubes_support_is_r_times_the_cross_part_plus_r");
    for _ in 0..fuzz::scaled(200) {
        let center = point(&mut rng, 5.0);
        let axis = unit(&mut rng);
        let (big, small) = (rng.range(1.0, 3.0), rng.range(0.1, 0.9));
        let tube = Cover::tube(center, axis, across(axis, &mut rng), big, small).unwrap();
        let e = unit(&mut rng);
        let exact = (center - Point3::origin()).dot(e) + big * axis.cross(e).norm() + small;
        let got = tube.support(e);
        assert!(
            (got - exact).abs() < 1e-12,
            "support {got} against R·|e⊥| + r = {exact}; {}",
            fuzz::replay()
        );
    }
}

/// **An interval torus's cover encloses every torus its brackets admit.**
#[test]
fn a_bracketed_tube_encloses_every_torus_it_admits() {
    let mut rng = fuzz::start("cover::a_bracketed_tube_encloses_every_torus_it_admits");
    for case in 0..fuzz::scaled(4) {
        let center = point(&mut rng, 4.0);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let (big, small) = (rng.range(1.0, 3.0), rng.range(0.1, 0.9));
        let w = rng.log_uniform(1e-9, 1e-2);
        let wide = |x: f64| Interval::from_bounds(x - w, x + w);
        let cover = Cover::tube(
            center.map(wide),
            ivec(axis),
            ivec(u_ref),
            wide(big),
            wide(small),
        )
        .unwrap();
        let dirs = directions(&mut rng, 3);
        for _ in 0..4 {
            let mut jitter = |x: f64| x + rng.range(-w, w);
            let c = Point3::new(jitter(center.x), jitter(center.y), jitter(center.z));
            let torus = Surface::Torus {
                center: c,
                axis,
                major_radius: jitter(big),
                minor_radius: jitter(small),
                u_ref,
            };
            let samples = chart_samples(
                &torus,
                (c, axis, u_ref, big),
                (0.0, TAU),
                (0.0, TAU),
                &dirs,
                &[],
            );
            for &e in &dirs {
                let s = cover.support(ivec(e)).hi();
                for p in &samples {
                    let x = (*p - Point3::origin()).dot(ivec(e)).hi();
                    assert!(
                        x <= s,
                        "case {case}: bracketed support {s} short of {x}; {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
}

/// **A curve's cover over a span holds the curve there**: lines, circle
/// and ellipse arcs in either orientation and past a turn, spiric arcs
/// over the same windows, and a rational spline whole and over a
/// sub-window across a knot, in either orientation.
#[test]
fn a_curves_cover_over_a_span_holds_it() {
    let mut rng = fuzz::start("cover::a_curves_cover_over_a_span_holds_it");
    for case in 0..fuzz::scaled(4) {
        let scale = rng.log_uniform(1e-2, 1e2);
        let center = point(&mut rng, 10.0 * scale);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let v_ref = axis.cross(u_ref);
        let dirs = directions(&mut rng, 3);
        let sites = sites(&mut rng, center, scale);
        let t0 = rng.range(-7.0, 7.0);
        let t1 = t0 + rng.range(-8.0, 8.0);
        let window = ParamWindow::new(t0, t1);
        let iwin = ParamWindow::new(iv(t0), iv(t1));
        let run = |name: &str, curve: Curve3<f64>, extra: Vec<f64>| {
            let mut ts: Vec<f64> = dense(t0, t1, 48).collect();
            ts.extend(extra);
            check(&Case {
                name: format!("{name} #{case}"),
                at_f64: curve.cover_over(window).unwrap(),
                at_iv: curve.map_scalar(iv).cover_over(iwin).unwrap(),
                samples: curve_samples(&curve, ts),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
        };
        let line = Curve3::Line(Line {
            origin: center,
            dir: unit(&mut rng) * scale,
        });
        run("line", line, vec![]);
        let conic_extremes = |a: f64, b: f64| {
            let mut ts = Vec::new();
            for &e in &dirs {
                ts.extend(in_window(
                    &crests(a * u_ref.dot(e), b * v_ref.dot(e)),
                    t0,
                    t1,
                ));
            }
            for q in site_points(&sites) {
                let w = q - center;
                ts.extend(in_window(
                    &crests(a * u_ref.dot(w), b * v_ref.dot(w)),
                    t0,
                    t1,
                ));
            }
            ts
        };
        let r = scale * rng.range(0.2, 1.0) * if rng.unit() < 0.2 { -1.0 } else { 1.0 };
        let circle = Curve3::Circle(crate::Circle::new(
            crate::Frame::new(center, axis, u_ref),
            r,
        ));
        run("circle", circle, conic_extremes(r, r));
        let (a, b) = (scale, scale * rng.range(0.1, 1.0));
        let ellipse = Curve3::Ellipse(crate::Ellipse::new(
            crate::Frame::new(center, axis, u_ref),
            a,
            b,
        ));
        run("ellipse", ellipse, conic_extremes(a, b));
        let (big, small) = (scale, scale * rng.range(0.1, 0.5));
        let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let offset = (big - small) * rng.range(0.05, 0.95) * side;
        let spiric = Curve3::spiric(center, axis, u_ref, big, small, offset, band()).unwrap();
        // The spiric's rectangle over the window turns at the crests of
        // `cos v` and `sin v` in it.
        let turns = [crests(1.0, 0.0), crests(0.0, 1.0)]
            .iter()
            .flat_map(|c| in_window(c, t0, t1))
            .collect();
        run("spiric", spiric, turns);
        let control: Vec<Point3<f64>> = (0..6)
            .map(|_| center + (point(&mut rng, scale) - Point3::origin()))
            .collect();
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 3.0], 3).unwrap();
        let weights = vec![1.0, 2.0, 0.5, 1.0, 3.0, 1.0];
        let spline = Curve3::Nurbs(Arc::new(NurbsCurve3::new(knots, control, weights).unwrap()));
        check(&Case {
            name: format!("spline #{case}"),
            at_f64: spline.cover_over(ParamWindow::new(0.0, 3.0)).unwrap(),
            at_iv: spline
                .map_scalar(iv)
                .cover_over(ParamWindow::new(iv(0.0), iv(3.0)))
                .unwrap(),
            samples: curve_samples(&spline, dense(0.0, 3.0, 120)),
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
        // Over a sub-window the pieces' nets hold it there, across a knot.
        let (a, b) = (rng.range(0.2, 1.4), rng.range(1.6, 2.8));
        check(&Case {
            name: format!("spline over [{a}, {b}] #{case}"),
            at_f64: spline.cover_over(ParamWindow::new(b, a)).unwrap(),
            at_iv: spline
                .map_scalar(iv)
                .cover_over(ParamWindow::new(iv(b), iv(a)))
                .unwrap(),
            samples: curve_samples(&spline, dense(a, b, 120)),
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
    }
}

/// The trim pieces' rows, one family per test: the axial slab and the
/// cone frustum; a ring torus's chart rectangle and latitude zone and a
/// sphere's; a vertex set and a control net.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trim {
    Axial,
    Chart,
    Hull,
}

/// **The axial slab and the cone frustum hold their solids.**
#[test]
fn the_axial_pieces_hold_their_solids() {
    trim_rows(Trim::Axial, "cover::the_axial_pieces_hold_their_solids");
}

/// **A ring torus's and a sphere's chart rectangle and latitude zone hold
/// the chart's image.**
#[test]
fn the_chart_patches_hold_their_image() {
    trim_rows(Trim::Chart, "cover::the_chart_patches_hold_their_image");
}

/// **A vertex set holds its points and a control net its hull.**
#[test]
fn a_point_set_and_a_net_hold_their_points() {
    trim_rows(Trim::Hull, "cover::a_point_set_and_a_net_hold_their_points");
}

#[allow(clippy::too_many_lines)]
fn trim_rows(part: Trim, label: &str) {
    let mut rng = fuzz::start(label);
    for case in 0..fuzz::scaled(4) {
        let scale = rng.log_uniform(1e-2, 1e2);
        let center = point(&mut rng, 10.0 * scale);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let dirs = directions(&mut rng, 3);
        let sites = sites(&mut rng, center, scale);
        let points = site_points(&sites);
        let h0 = scale * rng.range(-2.0, 2.0);
        let h1 = h0 + scale * rng.range(-2.0, 2.0);
        let (lo, hi) = (h0.min(h1), h0.max(h1));
        // The solid about the interval cover's own unit axis, along
        // interval unit directions across it: exact members of the region.
        let iaxis = unit_at(ivec(axis));
        let (ib1, ib2) = iaxis.orthonormal_basis();
        let ia = iaxis.get();
        let across_of = |v: Vec3<f64>| {
            let v = ivec(v);
            let off = v - ia * v.dot(ia);
            off / off.norm()
        };
        let solid = |radius_at: &dyn Fn(f64) -> f64, base: Point3<f64>| {
            let mut out = Vec::new();
            let mut at = |t: f64, rho: f64, dir: Vec3<Interval>| {
                out.push(ipt(base) + ia * iv(t) + dir * iv(rho));
            };
            for t in dense(lo, hi, 6).chain([0.0_f64.max(lo).min(hi)]) {
                for phi in dense(0.0, TAU, 12) {
                    let dir = ib1 * iv(phi.cos()) + ib2 * iv(phi.sin());
                    let dir = dir / dir.norm();
                    for f in [0.0, 0.5, 1.0] {
                        at(t, radius_at(t) * f, dir);
                    }
                }
                for &e in &dirs {
                    if (e - axis * axis.dot(e)).norm() > 1e-9 {
                        at(t, radius_at(t), across_of(e));
                    }
                }
                for &q in &points {
                    let w = q - base;
                    if (w - axis * axis.dot(w)).norm() > 1e-9 {
                        at(t, radius_at(t), across_of(w));
                        at(t, radius_at(t), -across_of(w));
                    }
                }
            }
            out
        };
        let radius = scale * rng.range(0.1, 1.0);
        let window = ParamWindow::new(h0, h1);
        let iwin = ParamWindow::new(iv(h0), iv(h1));
        if part == Trim::Axial {
            check(&Case {
                name: format!("slab #{case}"),
                at_f64: Cover::slab(center, unit_at(axis), radius, window).unwrap(),
                at_iv: Cover::slab(ipt(center), unit_at(ivec(axis)), iv(radius), iwin).unwrap(),
                samples: solid(&|_| radius, center),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
            let tan = rng.range(0.05, 3.0);
            check(&Case {
                name: format!("frustum #{case}"),
                at_f64: Cover::frustum(center, unit_at(axis), tan, window).unwrap(),
                at_iv: Cover::frustum(ipt(center), unit_at(ivec(axis)), iv(tan), iwin).unwrap(),
                samples: solid(&|t: f64| t.abs() * tan, center),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
        }
        let u0 = rng.range(-7.0, 7.0);
        let u1 = u0 + rng.range(0.1, 6.0);
        let v0 = rng.range(-7.0, 7.0);
        let v1 = v0 + rng.range(0.1, 6.0);
        let (big, small) = (scale, scale * rng.range(0.05, 0.95));
        // The torus patch takes either sign of `r`; the sphere's chart
        // premise is `r ≥ 0`.
        let signed = if rng.unit() < 0.5 { -small } else { small };
        let torus = Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: signed,
            u_ref,
        };
        for (zone, us) in [(false, (u0, u1)), (true, (0.0, TAU))] {
            if part != Trim::Chart {
                break;
            }
            let u = (!zone).then(|| ParamWindow::new(u0, u1));
            let iu = (!zone).then(|| ParamWindow::new(iv(u0), iv(u1)));
            check(&Case {
                name: format!("torus patch (zone {zone}) #{case}"),
                at_f64: Cover::patch(
                    center,
                    (axis, u_ref),
                    (big, signed),
                    u,
                    ParamWindow::new(v0, v1),
                )
                .unwrap(),
                at_iv: Cover::patch(
                    ipt(center),
                    (ivec(axis), ivec(u_ref)),
                    (iv(big), iv(signed)),
                    iu,
                    ParamWindow::new(iv(v0), iv(v1)),
                )
                .unwrap(),
                samples: chart_samples(
                    &torus,
                    (center, axis, u_ref, big),
                    us,
                    (v0, v1),
                    &dirs,
                    &points,
                ),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
            let lat0 = rng.range(-FRAC_PI_2, FRAC_PI_2);
            let lat1 = rng.range(-FRAC_PI_2, FRAC_PI_2);
            let sphere = Surface::Sphere {
                center,
                radius: small,
                axis,
                u_ref,
            };
            check(&Case {
                name: format!("sphere patch (zone {zone}) #{case}"),
                at_f64: Cover::patch(
                    center,
                    (axis, u_ref),
                    (0.0, small),
                    u,
                    ParamWindow::new(lat0, lat1),
                )
                .unwrap(),
                at_iv: Cover::patch(
                    ipt(center),
                    (ivec(axis), ivec(u_ref)),
                    (iv(0.0), iv(small)),
                    iu,
                    ParamWindow::new(iv(lat0), iv(lat1)),
                )
                .unwrap(),
                samples: chart_samples(
                    &sphere,
                    (center, axis, u_ref, 0.0),
                    us,
                    (lat0, lat1),
                    &dirs,
                    &points,
                ),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
        }
        if part != Trim::Hull {
            continue;
        }
        let net: Vec<Point3<f64>> = (0..7)
            .map(|_| center + (point(&mut rng, scale) - Point3::origin()))
            .collect();
        check(&Case {
            name: format!("points #{case}"),
            at_f64: Cover::points(net.clone()).unwrap(),
            at_iv: Cover::points(net.iter().map(|&p| ipt(p)).collect()).unwrap(),
            samples: net.iter().map(|&p| ipt(p)).collect(),
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
        let mut hull: Vec<Point3<Interval>> = net.iter().map(|&p| ipt(p)).collect();
        for _ in 0..60 {
            let w: Vec<f64> = net.iter().map(|_| rng.unit().powi(4)).collect();
            let total: f64 = w.iter().sum();
            let p = net
                .iter()
                .zip(&w)
                .fold(Vec3::new(iv(0.0), iv(0.0), iv(0.0)), |acc, (&p, &wi)| {
                    acc + (ipt(p) - Point3::origin()) * (iv(wi) / iv(total))
                });
            hull.push(Point3::origin() + p);
        }
        check(&Case {
            name: format!("net #{case}"),
            at_f64: Cover::net(net.clone()).unwrap(),
            at_iv: Cover::net(net.iter().map(|&p| ipt(p)).collect()).unwrap(),
            samples: hull,
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
    }
}

/// **A meet holds what lies in both, a hull what lies in either.** The
/// meet's samples are the torus's that lie certainly inside a ball about
/// one of its points; the hull's are a torus's and a sphere's.
#[test]
fn a_meet_holds_what_lies_in_both_and_a_hull_what_lies_in_either() {
    let mut rng =
        fuzz::start("cover::a_meet_holds_what_lies_in_both_and_a_hull_what_lies_in_either");
    for case in 0..fuzz::scaled(4) {
        let center = point(&mut rng, 5.0);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let (big, small) = (rng.range(1.0, 3.0), rng.range(0.1, 0.9));
        let torus = Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: small,
            u_ref,
        };
        let dirs = directions(&mut rng, 3);
        let sites = sites(&mut rng, center, 3.0);
        let all = chart_samples(
            &torus,
            (center, axis, u_ref, big),
            (0.0, TAU),
            (0.0, TAU),
            &dirs,
            &site_points(&sites),
        );
        let (u, v) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
        let b = torus.eval(u, v);
        let rho = rng.range(0.2, 2.0);
        // The ball's own centre's torus point is always in both.
        let centre = torus.map_scalar(iv).eval(iv(u), iv(v));
        let inside: Vec<_> = all
            .iter()
            .chain([&centre])
            .copied()
            .filter(|s| (*s - ipt(b)).norm().hi() <= rho)
            .collect();
        check(&Case {
            name: format!("tube ∧ ball #{case}"),
            at_f64: torus.cover().unwrap().meet(ball(b, rho)),
            at_iv: torus
                .map_scalar(iv)
                .cover()
                .unwrap()
                .meet(ball(ipt(b), iv(rho))),
            samples: inside,
            dirs: dirs.clone(),
            sites: sites.clone(),
        });
        let c2 = point(&mut rng, 5.0);
        let sphere = Surface::Sphere {
            center: c2,
            radius: rng.range(0.1, 2.0),
            axis,
            u_ref,
        };
        let mut both = all.clone();
        both.extend(chart_samples(
            &sphere,
            (c2, axis, u_ref, 0.0),
            (0.0, TAU),
            (-FRAC_PI_2, FRAC_PI_2),
            &dirs,
            &site_points(&sites),
        ));
        check(&Case {
            name: format!("tube ∨ ball #{case}"),
            at_f64: torus.cover().unwrap().hull(sphere.cover().unwrap()),
            at_iv: torus
                .map_scalar(iv)
                .cover()
                .unwrap()
                .hull(sphere.map_scalar(iv).cover().unwrap()),
            samples: both,
            dirs,
            sites,
        });
    }
}

/// **A refusal is the meet's identity and the hull's absorber**, two
/// refusals keep both reasons, and each unbounded or poisoned carrier
/// refuses naming its kind and what was not known.
#[test]
fn the_lattice_keeps_what_was_proved_and_what_was_not_known() {
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let proved: Covered<f64> = Ok(ball(Point3::new(1.0, 2.0, 3.0), 0.5));
    let refused = plane.cover();
    let unbounded = Uncovered::new(Entity::Surface(SurfaceKind::Plane), Unknown::Unbounded);
    assert_eq!(
        refused.as_ref().err(),
        Some(&unbounded),
        "a plane is unbounded"
    );
    let met = proved
        .clone()
        .meet(refused.clone())
        .expect("Ok ∧ Err is Ok");
    assert_eq!(
        met.support(Vec3::unit_x()),
        ball(Point3::new(1.0, 2.0, 3.0), 0.5).support(Vec3::unit_x()),
        "the meet keeps the proved cover"
    );
    assert!(
        refused.clone().meet(proved.clone()).is_ok(),
        "Err ∧ Ok is Ok"
    );
    assert_eq!(
        proved.clone().hull(refused.clone()).err(),
        Some(unbounded.clone()),
        "Ok ∨ Err is the Err"
    );
    assert_eq!(
        refused.clone().hull(proved).err(),
        Some(unbounded.clone()),
        "Err ∨ Ok is the Err"
    );
    let spline = Curve3::<f64>::nurbs_placeholder().cover_over(ParamWindow::new(0.0, 1.0));
    let poisoned = Uncovered::new(Entity::Curve(crate::CurveKind::Nurbs), Unknown::PoisonedNet);
    assert_eq!(
        spline.as_ref().err(),
        Some(&poisoned),
        "a placeholder spline's net bounds nothing"
    );
    let both = refused.meet(spline).expect_err("Err ∧ Err is Err");
    assert_eq!(
        both.unknowns(),
        &[
            (Entity::Surface(SurfaceKind::Plane), Unknown::Unbounded),
            (Entity::Curve(crate::CurveKind::Nurbs), Unknown::PoisonedNet)
        ],
        "two refusals keep both reasons"
    );
    for (surface, kind) in [
        (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            SurfaceKind::Cylinder,
        ),
        (
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.3,
                u_ref: Vec3::unit_x(),
            },
            SurfaceKind::Cone,
        ),
    ] {
        assert_eq!(
            surface.cover().err(),
            Some(Uncovered::new(Entity::Surface(kind), Unknown::Unbounded)),
            "{kind:?} is unbounded"
        );
    }
    assert_eq!(
        Surface::<f64>::nurbs_placeholder().cover().err(),
        Some(Uncovered::new(
            Entity::Surface(SurfaceKind::Nurbs),
            Unknown::PoisonedNet
        )),
        "a placeholder surface's net bounds nothing"
    );
}

/// **Every scalar reads the same arithmetic**: a dual's value channel and
/// a symbolic lane's bracket give the `f64` bounds bit for bit, so the
/// `f64` rows above carry to them.
#[test]
fn a_dual_and_a_symbolic_cover_read_the_f64_bounds() {
    let torus = Surface::Torus {
        center: Point3::new(0.3, -1.2, 2.5),
        axis: Vec3::new(0.48, 0.6, 0.64),
        major_radius: 2.0,
        minor_radius: 0.4,
        u_ref: Vec3::new(0.8, -0.6, 0.0),
    };
    let spiric = Curve3::spiric(
        Point3::new(1.0, 0.0, 0.0),
        Vec3::unit_z(),
        Vec3::unit_x(),
        2.0,
        0.5,
        0.7,
        band(),
    )
    .unwrap();
    let window = ParamWindow::new(0.3, 2.9);
    let f = [
        torus.cover().unwrap(),
        spiric.cover_over(window).unwrap(),
        Curve3::Circle(crate::Circle::new(
            crate::Frame::new(Point3::new(0.0, 1.0, 0.0), Vec3::unit_y(), Vec3::unit_z()),
            1.5,
        ))
        .cover_over(window)
        .unwrap(),
    ];
    let lift = |c: &Curve3<f64>| c.map_scalar(Dual::constant);
    let d = [
        torus.map_scalar(Dual::constant).cover().unwrap(),
        lift(&spiric)
            .cover_over(ParamWindow::new(Dual::constant(0.3), Dual::constant(2.9)))
            .unwrap(),
        lift(&Curve3::Circle(crate::Circle::new(
            crate::Frame::new(Point3::new(0.0, 1.0, 0.0), Vec3::unit_y(), Vec3::unit_z()),
            1.5,
        )))
        .cover_over(ParamWindow::new(Dual::constant(0.3), Dual::constant(2.9)))
        .unwrap(),
    ];
    let sym = |x: f64| <Sym<f64> as Real>::from_f64(x);
    let s = [
        torus.map_scalar(sym).cover().unwrap(),
        spiric
            .map_scalar(sym)
            .cover_over(ParamWindow::new(sym(0.3), sym(2.9)))
            .unwrap(),
        Curve3::Circle(crate::Circle::new(
            crate::Frame::new(Point3::new(0.0, 1.0, 0.0), Vec3::unit_y(), Vec3::unit_z()),
            1.5,
        ))
        .map_scalar(sym)
        .cover_over(ParamWindow::new(sym(0.3), sym(2.9)))
        .unwrap(),
    ];
    let e = Vec3::new(0.36, -0.48, 0.8);
    let q = Point3::new(2.0, 2.0, -1.0);
    for k in 0..3 {
        let want = (
            f[k].support(e),
            f[k].distance_range(&Site::Point(q)),
            f[k].ball().1,
        );
        let dual = (
            d[k].support(e.map(Dual::constant)).hi(),
            d[k].distance_range(&Site::Point(q.map(Dual::constant))),
            d[k].ball().1.hi(),
        );
        let symbolic = (
            s[k].support(e.map(sym)).hi(),
            s[k].distance_range(&Site::Point(q.map(sym))),
            s[k].ball().1.hi(),
        );
        assert_eq!(
            want.0.to_bits(),
            dual.0.to_bits(),
            "cover {k}: the dual's support"
        );
        assert_eq!(
            want.0.to_bits(),
            symbolic.0.to_bits(),
            "cover {k}: the symbolic support"
        );
        assert_eq!(
            (want.1.0.to_bits(), want.1.1.to_bits()),
            (dual.1.0.hi().to_bits(), dual.1.1.hi().to_bits()),
            "cover {k}: the dual's range"
        );
        assert_eq!(
            (want.1.0.to_bits(), want.1.1.to_bits()),
            (symbolic.1.0.hi().to_bits(), symbolic.1.1.hi().to_bits()),
            "cover {k}: the symbolic range"
        );
        assert_eq!(
            want.2.to_bits(),
            dual.2.to_bits(),
            "cover {k}: the dual's ball"
        );
        assert_eq!(
            want.2.to_bits(),
            symbolic.2.to_bits(),
            "cover {k}: the symbolic ball"
        );
    }
}

/// **Each radius's sign is read, once**: a tube, a ball, a slab and a
/// frustum of negative radius occupy what the magnitude names, and a
/// torus patch of negative tube radius reaches where its locus does.
/// `patch(c = 0, R = 3, r = −1, every azimuth, v ∈ [0, π/2])` is
/// `(3 − cos v)·ρ − sin v·axis`: along the axis it reaches `z = 0` (at
/// `v = 0`) and down to `z = −1`.
#[test]
fn a_negative_radius_occupies_what_its_magnitude_or_its_locus_names() {
    let o = Point3::origin();
    let (z, x) = (Vec3::unit_z(), Vec3::unit_x());
    let near = |got: f64, want: f64, what: &str| {
        assert!((got - want).abs() < 1e-12, "{what}: {got} against {want}");
    };
    let tube = Cover::tube(o, z, x, 2.0, -0.5).unwrap();
    near(tube.support(z), 0.5, "the tube along its axis");
    near(tube.support(x), 2.5, "the tube across its axis");
    near(tube.ball().1, 2.5, "the tube's ball");
    near(ball(o, -1.0).support(x), 1.0, "a ball");
    let window = ParamWindow::new(-1.0, 2.0);
    near(
        Cover::slab(o, unit_at(z), -0.5, window).unwrap().support(x),
        0.5,
        "a slab across its axis",
    );
    near(
        Cover::frustum(o, unit_at(z), -0.5, window)
            .unwrap()
            .support(x),
        1.0,
        "a frustum across its axis at its wide end",
    );
    let patch = Cover::patch(
        o,
        (z, x),
        (3.0, -1.0),
        None,
        ParamWindow::new(0.0, FRAC_PI_2),
    )
    .unwrap();
    near(patch.support(z), 0.0, "the patch along the axis");
    near(patch.support(-z), 1.0, "the patch against the axis");
    near(
        patch.support(x),
        2.0 + 1.0,
        "the patch across the axis, at v = π/2",
    );
}

/// **A hull's ball reaches both parts' far sides, in either order**: the
/// ball about the two centres' midpoint must reach a small ball's far
/// side past a large one's in one order and the large one's in the
/// other, so each of the two reaches it takes the greater of is read.
#[test]
fn a_hulls_ball_reaches_each_parts_far_side_in_either_order() {
    let large = || ball(Point3::origin(), 5.0);
    let small = || ball(Point3::new(10.0, 0.0, 0.0), 0.1);
    for (what, hull) in [
        ("large then small", large().hull(small())),
        ("small then large", small().hull(large())),
    ] {
        let (c, r) = hull.ball();
        for far in [Point3::new(-5.0, 0.0, 0.0), Point3::new(10.1, 0.0, 0.0)] {
            assert!(
                (far - c).norm() <= r,
                "{what}: the ball ({c:?}, {r}) misses {far:?}"
            );
        }
    }
}

/// **No data that is not a finite number builds a cover**: the
/// constructors refuse it, and the readings refuse `NonFinite` naming the
/// kind, so no region a cover names is the whole space.
#[test]
fn a_non_finite_datum_builds_no_cover() {
    let o = Point3::origin();
    let (z, x) = (Vec3::unit_z(), Vec3::unit_x());
    let (nan, inf) = (f64::NAN, f64::INFINITY);
    let window = ParamWindow::new(0.0, 1.0);
    assert!(Cover::tube(o, z, x, 0.0, inf).is_none(), "an infinite ball");
    assert!(
        Cover::points(vec![o, Point3::new(0.0, 0.0, inf)]).is_none(),
        "an infinite vertex"
    );
    assert!(
        Cover::arc(o, (x, Vec3::unit_y()), (1.0, nan), window).is_none(),
        "a NaN semi-axis"
    );
    assert!(
        Cover::slab(o, unit_at(z), inf, window).is_none(),
        "an infinite slab radius"
    );
    assert!(
        Cover::frustum(o, unit_at(z), nan, window).is_none(),
        "a NaN cone half-angle"
    );
    for (curve, kind) in [
        (
            Curve3::Circle(crate::Circle::new(crate::Frame::new(o, z, x), nan)),
            crate::CurveKind::Circle,
        ),
        (
            Curve3::Ellipse(crate::Ellipse::new(
                crate::Frame::new(Point3::new(inf, 0.0, 0.0), z, x),
                2.0,
                1.0,
            )),
            crate::CurveKind::Ellipse,
        ),
    ] {
        assert_eq!(
            curve.cover_over(window).err(),
            Some(Uncovered::new(Entity::Curve(kind), Unknown::NonFinite)),
            "a non-finite {kind:?}"
        );
    }
    assert!(
        Cover::tube(Point3::new(f64::NAN, 0.0, 0.0), z, x, 2.0, 0.5).is_none(),
        "a NaN centre"
    );
    assert!(
        Cover::segment(o, Point3::new(f64::INFINITY, 0.0, 0.0)).is_none(),
        "an infinite end"
    );
    assert!(
        Cover::net(vec![o, Point3::new(0.0, f64::NEG_INFINITY, 0.0)]).is_none(),
        "an infinite net point"
    );
    assert!(
        Cover::patch(
            o,
            (z, x),
            (3.0, 1.0),
            None,
            ParamWindow::new(0.0, f64::INFINITY)
        )
        .is_none(),
        "an infinite window"
    );
    let sphere = Surface::Sphere {
        center: o,
        radius: f64::INFINITY,
        axis: z,
        u_ref: x,
    };
    assert_eq!(
        sphere.cover().err(),
        Some(Uncovered::new(
            Entity::Surface(SurfaceKind::Sphere),
            Unknown::NonFinite
        )),
        "an infinite sphere"
    );
    let line = Curve3::Line(Line { origin: o, dir: x });
    assert_eq!(
        line.cover_over(ParamWindow::new(0.0, f64::INFINITY)).err(),
        Some(Uncovered::new(
            Entity::Curve(crate::CurveKind::Line),
            Unknown::NonFinite
        )),
        "a line over an unbounded span"
    );
}

/// **A segment whose end cancels is enclosed at `Interval`.** The line
/// `x = 1e5 + 0.1·t` at `t = −1e6`: the product `0.1·t` rounds at the
/// scale of `1e5` while the end lands near zero, so an end rounded once
/// at `f64` misses its exact point by far more than an ulp of itself.
/// The interval end encloses it, and the cover built at `Interval`
/// holds that enclosure on both sides.
#[test]
fn a_cancelling_segment_end_is_enclosed_at_interval() {
    let line = Curve3::Line(Line {
        origin: Point3::new(1e5, 0.0, 0.0),
        dir: Vec3::new(0.1, 0.0, 0.0),
    });
    let at = line.map_scalar(iv);
    let cover = at.cover_over(ParamWindow::new(iv(-1e6), iv(0.0))).unwrap();
    let end = at.eval(iv(-1e6)).x;
    let x = ivec(Vec3::unit_x());
    let (lo, hi) = (iv(0.0) - cover.support(-x), cover.support(x));
    assert!(
        lo.lo() <= end.lo() && end.hi() <= hi.hi(),
        "the end {end:?} against the cover's [{lo:?}, {hi:?}]"
    );
    let ((near, _), q) = (
        cover.distance_range(&Site::Point(ipt(Point3::new(-1.0, 0.0, 0.0)))),
        Point3::new(iv(-1.0), iv(0.0), iv(0.0)),
    );
    assert!(
        near.lo() <= (end - q.x).abs().lo(),
        "the nearest distance {near:?} above the end's {:?}",
        (end - q.x).abs()
    );
}

/// **The nearest distance is posed as a distance**: from a point `δ`
/// off a thin tube of a large ring, off a sphere, and off a circle arc,
/// along the surface's own normal, both scalars read `δ` to the
/// rounding of the point's coordinates. The squared form this replaced
/// read `1.05e-6` for `δ = 1e-6` at `R = 1000`, `r = 1e-3`, and its
/// enclosure fell to `[0, 1.1e-5]` at `R = 1e4`.
#[test]
fn the_nearest_distance_just_off_the_surface_is_the_offset() {
    let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
    let delta = 1e-6;
    // (what, at f64, at Interval, the point, the carrier's size)
    type Row = (String, Cover<f64>, Cover<Interval>, Point3<f64>, f64);
    let mut cases: Vec<Row> = Vec::new();
    for (big, small) in [(1e3, 1e-3), (1e4, 1e-3), (3.0, 1.0)] {
        let torus = Surface::Torus {
            center: Point3::new(5.0, -2.0, 1.0),
            axis: z,
            major_radius: big,
            minor_radius: small,
            u_ref: x,
        };
        let (u, v) = (0.7_f64, 0.3_f64);
        let normal = (x * u.cos() + y * u.sin()) * v.cos() + z * v.sin();
        let q = torus.eval(u, v) + normal * delta;
        cases.push((
            format!("torus R = {big}, r = {small}"),
            torus.cover().unwrap(),
            torus.map_scalar(iv).cover().unwrap(),
            q,
            big + small,
        ));
    }
    let c = Point3::new(1.0, 2.0, 3.0);
    let r = 1e3;
    cases.push((
        "sphere".to_string(),
        ball(c, r),
        ball(ipt(c), iv(r)),
        c + y * (r + delta),
        r,
    ));
    let circle = Curve3::Circle(crate::Circle::new(crate::Frame::new(c, z, x), r));
    let t = 0.4_f64;
    let q = circle.eval(t) + (x * t.cos() + y * t.sin()) * delta;
    cases.push((
        "circle arc".to_string(),
        circle.cover_over(ParamWindow::new(0.0, 1.0)).unwrap(),
        circle
            .map_scalar(iv)
            .cover_over(ParamWindow::new(iv(0.0), iv(1.0)))
            .unwrap(),
        q,
        r,
    ));
    for (what, f, i, q, size) in cases {
        let scale = (q - Point3::origin()).norm() + size;
        let tol = 64.0 * f64::EPSILON * scale;
        let (nf, _) = f.distance_range(&Site::Point(q));
        let (ni, _) = i.distance_range(&Site::Point(ipt(q)));
        assert!(
            (nf - delta).abs() <= tol,
            "{what}: the f64 nearest {nf} against the offset {delta}"
        );
        assert!(
            ni.lo() <= delta + tol && ni.lo() >= delta - tol,
            "{what}: the interval nearest {ni:?} against the offset {delta}"
        );
    }
}

/// **The farthest distance from a point near a short arc of a large
/// conic is posed as a distance.** A circle and an ellipse of size `1e3`
/// placed `1e3` from the origin, an arc of `1e-6` rad (about `1e-3` m)
/// read from a point on it: the farthest is the chord to the far end.
/// The squared harmonic form reads it as the root of `|w|² + …` with
/// `|w|² ≈ 1e6`, whose rounding (about `1e-10`) is a relative `1e-4` of
/// the chord's square, and goes red here. Each distance form differences
/// points placed about `2e3` from the origin, so its error is a few ulps
/// of that placement, not of the arc: about `1e-12` m, a relative `1e-9`
/// of the millimetre chord, and a relative error that grows as the arc
/// shrinks against its placement (a `1.6e-9` rad arc placed `13` out
/// reads to `3e-8`). The interval lane encloses it.
#[test]
fn the_farthest_distance_from_a_point_by_a_short_arc_is_the_chord() {
    let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
    let c = Point3::new(700.0, -400.0, 500.0);
    let (t0, t1) = (0.3_f64, 0.3 + 1e-6);
    let circle = Curve3::Circle(crate::Circle::new(crate::Frame::new(c, z, x), 1e3));
    let ellipse = Curve3::Ellipse(crate::Ellipse::new(
        crate::Frame::new(c, z, x),
        1.3e3,
        0.9e3,
    ));
    for (what, carrier) in [("circle", circle), ("ellipse", ellipse)] {
        let (a, b) = (carrier.eval(t0), carrier.eval(t1));
        let chord = (b - a).norm();
        let f = carrier.cover_over(ParamWindow::new(t0, t1)).unwrap();
        let (_, far) = f.distance_range(&Site::Point(a));
        assert!(
            far >= chord * (1.0 - 1e-9) && far <= chord * (1.0 + 1e-6),
            "{what}: the f64 farthest {far:e} against the chord {chord:e}"
        );
        let i = carrier
            .map_scalar(iv)
            .cover_over(ParamWindow::new(iv(t0), iv(t1)))
            .unwrap();
        let (_, far) = i.distance_range(&Site::Point(ipt(a)));
        assert!(
            far.hi() >= chord,
            "{what}: the interval farthest {far:?} against the chord {chord:e}"
        );
    }
}

/// **A closed arc's distance range at `Interval` is decided and tight.**
/// A circle over exactly one turn read from a point off it: the span's
/// `π − h` straddles zero, and the farthest and nearest are the whole
/// circle's, `√(h² + (ρ ± r)²)`. A crest test that asked whether the
/// span is a full turn read both arms there and spanned the whole range
/// between the ends and the crest; a Gram bound read through self-dots
/// of near-zero enclosures rooted below zero and poisoned the read.
#[test]
fn a_closed_arcs_range_at_interval_is_decided() {
    let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
    let c = Point3::new(0.1, 0.2, 0.3);
    let circle = Curve3::Circle(crate::Circle::new(crate::Frame::new(c, z, x), 1.5));
    let q = Point3::new(2.0, -1.0, 0.5);
    let w = q - c;
    let rho = (w.x * w.x + w.y * w.y).sqrt();
    let want = [
        (w.z.powi(2) + (rho - 1.5).powi(2)).sqrt(),
        (w.z.powi(2) + (rho + 1.5).powi(2)).sqrt(),
    ];
    for t0 in [0.0, 1.0] {
        let cover = circle
            .map_scalar(iv)
            .cover_over(ParamWindow::new(iv(t0), iv(t0 + TAU)))
            .unwrap();
        let (near, far) = cover.distance_range(&Site::Point(ipt(q)));
        for (what, got, want) in [("nearest", near, want[0]), ("farthest", far, want[1])] {
            assert!(
                !got.is_poison()
                    && got.lo() <= want
                    && want <= got.hi()
                    && got.hi() - got.lo() <= 1e-12,
                "t0 = {t0}: the {what} {got:?} against {want}"
            );
        }
    }
}

/// **A circle arc on a tilted frame reads its range at `Interval`.** A
/// circle's two semi-axes are one radius, so the deviation of its arc
/// from its mean circle is zero in the reals, and an enclosure of it
/// straddles zero where the radius enters as an enclosure with width
/// and the frame is off the world's axes (`axis × u_ref` rounds). The Gram bound under the root reads each
/// side's length as a squared norm, which an enclosure keeps at or above
/// zero; read as a self-dot product, the enclosure of a near-zero
/// vector reaches below zero, the root is a domain violation (decorated
/// below `Def`), and the deciding lanes refuse the range.
#[test]
fn a_tilted_circle_arcs_range_at_interval_is_decided() {
    let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
    let u_ref = Vec3::new(3.0, 0.0, -1.0).normalize();
    let c = Point3::new(0.3, -1.2, 2.5);
    let circle = Curve3::Circle(crate::Circle::new(crate::Frame::new(c, axis, u_ref), 0.7));
    // The radius as an enclosure a few ulps wide, as a computed datum
    // enters: the two semi-axes are that one enclosure, so their
    // difference straddles zero.
    let radius = Interval::from_bounds(0.7, 0.7 * (1.0 + 8.0 * f64::EPSILON));
    let wide = Curve3::Circle(crate::Circle::new(
        crate::Frame::new(ipt(c), ivec(axis), ivec(u_ref)),
        radius,
    ));
    let q = Point3::new(4.0, 1.0, -2.0);
    for (t0, t1) in [(0.2, 1.9), (-1.0, 4.0), (0.5, 0.5 + TAU)] {
        let cover = wide.cover_over(ParamWindow::new(iv(t0), iv(t1))).unwrap();
        let (near, far) = cover.distance_range(&Site::Point(ipt(q)));
        for (what, got) in [("nearest", near), ("farthest", far)] {
            assert!(
                got.is_certified(),
                "[{t0}, {t1}]: the {what} distance {got:?} is not certified"
            );
        }
        for p in curve_samples(&circle, dense(t0, t1, 400)) {
            let d = (p - ipt(q)).norm();
            assert!(
                near.lo() <= d.hi() && d.lo() <= far.hi(),
                "[{t0}, {t1}]: the arc's point at {d:?} from q leaves [{near:?}, {far:?}]"
            );
        }
    }
}
