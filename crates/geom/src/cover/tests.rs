//! Every piece's methods hold its locus, at `f64` and at `Interval`.
//!
//! Each row builds a piece from `f64` data at both scalars and samples
//! its locus as `Interval` enclosures of exact points of that data:
//! densely, and at the points where each bound is attained (a support's
//! crest, the nearest and farthest point from a site), so a piece that
//! drops its rounding charge reads a hair short of an attained point and
//! goes red. A sample passes when its whole enclosure lies inside the
//! bound.
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

fn dense(lo: f64, hi: f64, n: usize) -> impl Iterator<Item = f64> {
    (0..=n).map(move |i| lo + (hi - lo) * (i as f64) / (n as f64))
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

#[allow(clippy::too_many_lines)]
fn check(case: &Case) {
    let what = |m: &str| format!("{}: {m}; {}", case.name, fuzz::replay());
    assert!(!case.samples.is_empty(), "{}", what("no samples"));
    let mut frames = vec![Frame::World];
    frames.extend(case.dirs.iter().map(|&e| Frame::aimed(unit_at(e))));
    for &e in &case.dirs {
        let (sf, si) = (case.at_f64.support(e), case.at_iv.support(ivec(e)).hi());
        for s in &case.samples {
            let x = (*s - Point3::origin()).dot(ivec(e)).hi();
            assert!(
                x <= sf,
                "{}",
                what(&format!("f64 support {sf} along {e:?} short of {x}"))
            );
            assert!(
                x <= si,
                "{}",
                what(&format!("interval support {si} along {e:?} short of {x}"))
            );
        }
    }
    for frame in &frames {
        let rows = frame.rows();
        let (lf, hf) = case.at_f64.box_in(frame);
        let (li, hi) = case.at_iv.box_in(&Frame::Rows(rows.map(ivec)));
        let lows = [(lf.x, li.x.lo()), (lf.y, li.y.lo()), (lf.z, li.z.lo())];
        let highs = [(hf.x, hi.x.hi()), (hf.y, hi.y.hi()), (hf.z, hi.z.hi())];
        for s in &case.samples {
            let v = *s - Point3::origin();
            for (k, &r) in rows.iter().enumerate() {
                let c = v.dot(ivec(r));
                assert!(
                    lows[k].0 <= c.lo() && lows[k].1 <= c.lo(),
                    "{}",
                    what(&format!("box row {k} low {:?} above {c:?}", lows[k]))
                );
                assert!(
                    highs[k].0 >= c.hi() && highs[k].1 >= c.hi(),
                    "{}",
                    what(&format!("box row {k} high {:?} below {c:?}", highs[k]))
                );
            }
        }
    }
    let aabb = case.at_f64.aabb(0.0);
    let (cf, rf) = case.at_f64.ball();
    let (ci, ri) = case.at_iv.ball();
    for s in &case.samples {
        assert!(
            aabb.min_x <= s.x.lo()
                && s.x.hi() <= aabb.max_x
                && aabb.min_y <= s.y.lo()
                && s.y.hi() <= aabb.max_y
                && aabb.min_z <= s.z.lo()
                && s.z.hi() <= aabb.max_z,
            "{}",
            what(&format!("aabb {aabb:?} misses {s:?}"))
        );
        let df = (*s - ipt(cf)).norm().hi();
        assert!(
            df <= rf,
            "{}",
            what(&format!("f64 ball {rf} short of {df}"))
        );
        let di = (*s - ci).norm().hi();
        assert!(
            di <= ri.hi(),
            "{}",
            what(&format!("interval ball {ri:?} short of {di}"))
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
        for s in &case.samples {
            let d = distance(*s, site);
            assert!(
                nf <= d.lo() && ni.lo() <= d.lo(),
                "{}",
                what(&format!("nearest {nf}/{ni:?} above {d:?} from {site:?}"))
            );
            assert!(
                ff >= d.hi() && fi.hi() >= d.hi(),
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
/// frames, both radius signs, tori out to spindle data (the tube's
/// support holds for any frame and sign).
#[test]
fn the_whole_carrier_covers_hold_the_sphere_and_the_torus() {
    let mut rng = fuzz::start("cover::the_whole_carrier_covers_hold_the_sphere_and_the_torus");
    for case in 0..fuzz::scaled(6) {
        let scale = rng.log_uniform(1e-3, 1e3);
        let center = point(&mut rng, 10.0 * scale);
        let axis = unit(&mut rng);
        let u_ref = across(axis, &mut rng);
        let sign = if rng.unit() < 0.25 { -1.0 } else { 1.0 };
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
            minor_radius: small,
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
        let tube = Cover::tube(center, axis, across(axis, &mut rng), big, small);
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
        );
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
/// and ellipse arcs in either orientation and past a turn, spirics, and
/// a rational spline.
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
        let circle = Curve3::Circle {
            center,
            axis,
            radius: r,
            u_ref,
        };
        run("circle", circle, conic_extremes(r, r));
        let (a, b) = (scale, scale * rng.range(0.1, 1.0));
        let ellipse = Curve3::Ellipse {
            center,
            axis,
            semi_u: a,
            semi_v: b,
            u_ref,
        };
        run("ellipse", ellipse, conic_extremes(a, b));
        let (big, small) = (scale, scale * rng.range(0.1, 0.5));
        let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let offset = (big - small) * rng.range(0.05, 0.95) * side;
        let spiric = Curve3::spiric(center, axis, u_ref, big, small, offset, band()).unwrap();
        run("spiric", spiric, dense(0.0, TAU, 240).collect());
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
        let v_ref = axis.cross(u_ref);
        let dirs = directions(&mut rng, 3);
        let sites = sites(&mut rng, center, scale);
        let points = site_points(&sites);
        let h0 = scale * rng.range(-2.0, 2.0);
        let h1 = h0 + scale * rng.range(-2.0, 2.0);
        let (lo, hi) = (h0.min(h1), h0.max(h1));
        let solid = |radius_at: &dyn Fn(f64) -> f64, base: Point3<f64>| {
            let mut out = Vec::new();
            let mut at = |t: f64, rho: f64, dir: Vec3<f64>| {
                out.push(ipt(base) + ivec(axis) * iv(t) + ivec(dir) * iv(rho));
            };
            for t in dense(lo, hi, 6).chain([0.0_f64.max(lo).min(hi)]) {
                for phi in dense(0.0, TAU, 12) {
                    let dir = u_ref * phi.cos() + v_ref * phi.sin();
                    for f in [0.0, 0.5, 1.0] {
                        at(t, radius_at(t) * f, dir);
                    }
                }
                for &e in &dirs {
                    let off = e - axis * axis.dot(e);
                    if off.norm() > 1e-9 {
                        at(t, radius_at(t), off.normalize());
                    }
                }
                for &q in &points {
                    let w = q - base;
                    let off = w - axis * axis.dot(w);
                    if off.norm() > 1e-9 {
                        at(t, radius_at(t), off.normalize());
                        at(t, radius_at(t), -off.normalize());
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
                at_f64: Cover::slab(center, unit_at(axis), radius, window),
                at_iv: Cover::slab(ipt(center), unit_at(ivec(axis)), iv(radius), iwin),
                samples: solid(&|_| radius, center),
                dirs: dirs.clone(),
                sites: sites.clone(),
            });
            let tan = rng.range(0.05, 3.0);
            check(&Case {
                name: format!("frustum #{case}"),
                at_f64: Cover::frustum(center, unit_at(axis), tan, window),
                at_iv: Cover::frustum(ipt(center), unit_at(ivec(axis)), iv(tan), iwin),
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
        let torus = Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: small,
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
                    (big, small),
                    u,
                    ParamWindow::new(v0, v1),
                ),
                at_iv: Cover::patch(
                    ipt(center),
                    (ivec(axis), ivec(u_ref)),
                    (iv(big), iv(small)),
                    iu,
                    ParamWindow::new(iv(v0), iv(v1)),
                ),
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
                ),
                at_iv: Cover::patch(
                    ipt(center),
                    (ivec(axis), ivec(u_ref)),
                    (iv(0.0), iv(small)),
                    iu,
                    ParamWindow::new(iv(lat0), iv(lat1)),
                ),
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
        let b = torus.eval(rng.range(0.0, TAU), rng.range(0.0, TAU));
        let rho = rng.range(0.2, 2.0);
        let inside: Vec<_> = all
            .iter()
            .copied()
            .filter(|s| (*s - ipt(b)).norm().hi() <= rho)
            .collect();
        check(&Case {
            name: format!("tube ∧ ball #{case}"),
            at_f64: torus.cover().unwrap().meet(Cover::whole_ball(b, rho)),
            at_iv: torus
                .map_scalar(iv)
                .cover()
                .unwrap()
                .meet(Cover::whole_ball(ipt(b), iv(rho))),
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
    let ball: Covered<f64> = Ok(Cover::whole_ball(Point3::new(1.0, 2.0, 3.0), 0.5));
    let refused = plane.cover();
    let unbounded = Uncovered::new(Entity::Surface(SurfaceKind::Plane), Unknown::Unbounded);
    assert_eq!(
        refused.as_ref().err(),
        Some(&unbounded),
        "a plane is unbounded"
    );
    let met = ball.clone().meet(refused.clone()).expect("Ok ∧ Err is Ok");
    assert_eq!(
        met.support(Vec3::unit_x()),
        Cover::whole_ball(Point3::new(1.0, 2.0, 3.0), 0.5).support(Vec3::unit_x()),
        "the meet keeps the proved cover"
    );
    assert!(refused.clone().meet(ball.clone()).is_ok(), "Err ∧ Ok is Ok");
    assert_eq!(
        ball.clone().hull(refused.clone()).err(),
        Some(unbounded.clone()),
        "Ok ∨ Err is the Err"
    );
    assert_eq!(
        refused.clone().hull(ball).err(),
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
        Curve3::Circle {
            center: Point3::new(0.0, 1.0, 0.0),
            axis: Vec3::unit_y(),
            radius: 1.5,
            u_ref: Vec3::unit_z(),
        }
        .cover_over(window)
        .unwrap(),
    ];
    let lift = |c: &Curve3<f64>| c.map_scalar(Dual::constant);
    let d = [
        torus.map_scalar(Dual::constant).cover().unwrap(),
        lift(&spiric)
            .cover_over(ParamWindow::new(Dual::constant(0.3), Dual::constant(2.9)))
            .unwrap(),
        lift(&Curve3::Circle {
            center: Point3::new(0.0, 1.0, 0.0),
            axis: Vec3::unit_y(),
            radius: 1.5,
            u_ref: Vec3::unit_z(),
        })
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
        Curve3::Circle {
            center: Point3::new(0.0, 1.0, 0.0),
            axis: Vec3::unit_y(),
            radius: 1.5,
            u_ref: Vec3::unit_z(),
        }
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
