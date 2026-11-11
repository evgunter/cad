//! Review probes for PR 6836 (full review). Each prints its outcome;
//! the asserts encode what the PR claims.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point2, Point3, Real, Vec2, Vec3};

use super::Pcurve;
use super::spline_door_rows::band;
use crate::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey};

macro_rules! cert_at {
    ($t:ty, $surface:expr, $image:expr, $carrier:expr, $span:expr) => {{
        let c = <$t as Real>::from_f64;
        let (t0, t1): (f64, f64) = $span;
        let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> = slotmap::SlotMap::with_key();
        let key = table.insert($surface);
        let carrier: Curve3<$t> = $carrier;
        let (start, end) = (carrier.eval(c(t0)), carrier.eval(c(t1)));
        EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(key, $image),
                carrier,
                param_start: c(t0),
                param_end: c(t1),
            },
            start,
            end,
            |k| table.get(k).cloned(),
            geom_core::test_support::fixture_lever(<$t as Real>::one()),
            band(),
            NurbsLane::certified(),
        )
        .map(|e| format!("{:?}", e.certificate()))
    }};
}

fn both(
    name: &str,
    surface: &Surface<f64>,
    image: &Pcurve<f64>,
    image_i: Pcurve<Interval>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
) -> (Result<String, CertifyError>, Result<String, CertifyError>) {
    let f = cert_at!(f64, surface.clone(), image.clone(), carrier.clone(), span);
    let i = cert_at!(
        Interval,
        surface.map_scalar(Interval::from_f64),
        image_i,
        carrier.map_scalar(Interval::from_f64),
        span
    );
    println!("[probe] {name}: eps={:e}\n   f64: {f:?}\n   Interval: {i:?}", band().zero());
    (f, i)
}

fn lift(p: &Pcurve<f64>) -> Pcurve<Interval> {
    let f = Interval::from_f64;
    match p {
        Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine { p0: Point2::new(f(p0.x), f(p0.y)), pl: Vec2::new(f(pl.x), f(pl.y)) },
        Pcurve::General(n) => Pcurve::General(Arc::new(n.map_scalar(f))),
        Pcurve::Fitted(n) => Pcurve::Fitted(Arc::new(n.map_scalar(f))),
        _ => panic!("lift"),
    }
}

/// Dense knots clustered around 3/16 (between samples 1/8 and 2/8) on
/// a curve of `degree`.
fn knots(degree: usize) -> Vec<f64> {
    let mut k = vec![0.0; degree + 1];
    for x in [0.05, 0.17, 0.18, 0.1875, 0.195, 0.205, 0.5, 0.7] {
        k.push(x);
    }
    k.extend(vec![1.0; degree + 1]);
    k
}

fn greville(k: &[f64], p: usize) -> Vec<f64> {
    #[allow(clippy::cast_precision_loss)]
    (0..k.len() - p - 1).map(|i| k[i + 1..=i + p].iter().sum::<f64>() / p as f64).collect()
}

/// Index of the control whose greville is nearest 3/16.
fn mid(g: &[f64]) -> usize {
    (0..g.len()).min_by(|&a, &b| (g[a] - 0.1875).abs().total_cmp(&(g[b] - 0.1875).abs())).unwrap()
}

/// The image curve in UV with controls `uv(g)` at greville g, weights `w`,
/// and the carrier: the chart's points at those controls, one moved by
/// `bump` in (u, v).
fn pair(
    surface: &Surface<f64>,
    degree: usize,
    uv: impl Fn(f64) -> [f64; 2],
    w: impl Fn(usize) -> f64,
    bump: [f64; 2],
) -> (NurbsCurve2<f64>, Curve3<f64>) {
    let k = knots(degree);
    let g = greville(&k, degree);
    let m = mid(&g);
    let kv = KnotVector::clamped(k, degree).unwrap();
    let ctrl: Vec<Point2<f64>> = g.iter().map(|&t| { let [a, b] = uv(t); Point2::new(a, b) }).collect();
    let ws: Vec<f64> = (0..g.len()).map(&w).collect();
    let net: Vec<Point3<f64>> = ctrl
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let d = if i == m { bump } else { [0.0, 0.0] };
            surface.eval(p.x + d[0], p.y + d[1])
        })
        .collect();
    (
        NurbsCurve2::new(kv.clone(), ctrl, ws.clone()).unwrap(),
        Curve3::Nurbs(Arc::new(NurbsCurve3::new(kv, net, ws).unwrap())),
    )
}

fn refused(r: &(Result<String, CertifyError>, Result<String, CertifyError>)) -> bool {
    r.0.is_err() && r.1.is_err()
}

fn mirrored_plane() -> Surface<f64> {
    let th: f64 = 0.7;
    Surface::Plane {
        origin: Point3::new(3.0, -2.0, 1.5),
        normal: Vec3::new(0.0, 0.0, -1.0),
        u_ref: Vec3::new(th.cos(), th.sin(), 0.0),
    }
}

fn tilted_plane() -> Surface<f64> {
    let n = Vec3::new(0.3, -0.5, 0.8).normalize();
    let u = (Vec3::new(1.0, 0.2, 0.0) - n * Vec3::new(1.0, 0.2, 0.0).dot(n)).normalize();
    Surface::Plane { origin: Point3::new(-7.0, 4.0, 2.0), normal: n, u_ref: u }
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::new(0.2, 0.3, 0.9).normalize(),
        radius: 2.5,
        u_ref: {
            let a = Vec3::new(0.2, 0.3, 0.9).normalize();
            (Vec3::unit_x() - a * a.x).normalize()
        },
    }
}

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.5, -1.0, 2.0),
        axis: Vec3::new(-0.1, 0.4, 0.9).normalize(),
        half_angle: 0.7,
        u_ref: {
            let a = Vec3::new(-0.1, 0.4, 0.9).normalize();
            (Vec3::unit_x() - a * a.x).normalize()
        },
    }
}

/// Claim 1: wiggles between samples on every class the route admits.
#[test]
fn probe_wiggles_between_samples_never_certify() {
    let mut bad = Vec::new();
    let mut check = |name: &str, r: (Result<String, CertifyError>, Result<String, CertifyError>), want_refused: bool| {
        if refused(&r) != want_refused {
            bad.push(format!("{name}: {r:?}"));
        }
    };
    for (pname, plane) in [("mirrored", mirrored_plane()), ("tilted", tilted_plane())] {
        // An iso line in an oblique direction, degree-1 carrier.
        for (h, want) in [(1e-3, true), (0.0, false)] {
            let (p0, pl) = ([1.3, -2.1], [0.6, 0.8]);
            let (_img, carrier) =
                pair(&plane, 1, |t| [p0[0] + pl[0] * t, p0[1] + pl[1] * t], |_| 1.0, [-0.8 * h, 0.6 * h]);
            let iso = Pcurve::IsoLine { p0: Point2::new(p0[0], p0[1]), pl: Vec2::new(pl[0], pl[1]) };
            let r = both(&format!("{pname} plane iso h={h}"), &plane, &iso, lift(&iso), &carrier, (0.0, 1.0));
            check(&format!("{pname} plane iso h={h}"), r, want);
        }
        // Rational cubic General, and Fitted, with a wide v extent.
        for (h, want) in [(1e-3, true), (0.0, false)] {
            let (img, carrier) = pair(
                &plane,
                3,
                |t| [4.0 * t - 1.0, 40.0 * (t - 0.5) * (t - 0.5) - 3.0],
                |i| 1.0 + 0.2 * (i % 3) as f64,
                [0.0, h],
            );
            let img = Arc::new(img);
            for (kind, p) in [("General", Pcurve::General(img.clone())), ("Fitted", Pcurve::Fitted(img.clone()))] {
                let n = format!("{pname} plane rational cubic {kind} h={h}");
                let r = both(&n, &plane, &p, lift(&p), &carrier, (0.0, 1.0));
                check(&n, r, want);
                let n = format!("{pname} plane rational cubic {kind} h={h} sub-span");
                let r = both(&n, &plane, &p, lift(&p), &carrier, (0.1, 0.9));
                check(&n, r, want);
            }
        }
    }
    // Rulings: a wiggle ALONG the ruling stays exactly on the chart, so
    // only the image's fidelity can see it.
    for (sname, s, u0, v0) in [("cylinder", cylinder(), 0.1, -1.0), ("cone", cone(), 2.7, 0.5)] {
        for (h, want) in [(1e-3, true), (0.0, false)] {
            let (img, carrier) = pair(&s, 1, |t| [u0, v0 + 2.5 * t], |_| 1.0, [0.0, h]);
            let iso = Pcurve::IsoLine { p0: Point2::new(u0, v0), pl: Vec2::new(0.0, 2.5) };
            let n = format!("{sname} ruling iso along-ruling h={h}");
            let r = both(&n, &s, &iso, lift(&iso), &carrier, (0.0, 1.0));
            check(&n, r, want);
            let g = Pcurve::General(Arc::new(img));
            let n = format!("{sname} ruling General along-ruling h={h}");
            let r = both(&n, &s, &g, lift(&g), &carrier, (0.0, 1.0));
            check(&n, r, want);
        }
        // A cubic General on the ruling whose v goes back and forth.
        for (h, want) in [(1e-3, true), (0.0, false)] {
            let (img, carrier) =
                pair(&s, 3, |t| [u0, v0 + 2.0 * t + 0.3 * (9.0 * t).sin()], |i| 1.0 + 0.1 * (i % 2) as f64, [0.0, h]);
            let g = Pcurve::General(Arc::new(img));
            let n = format!("{sname} ruling rational cubic General h={h}");
            let r = both(&n, &s, &g, lift(&g), &carrier, (0.0, 1.0));
            check(&n, r, want);
        }
    }
    // A line carrier (closed form, Taylor net path): the image wiggles.
    {
        let plane = tilted_plane();
        let (p0, pl) = ([0.5, 0.25], [1.5, -0.5]);
        let (img, _) = pair(&plane, 1, |t| [p0[0] + pl[0] * t, p0[1] + pl[1] * t], |_| 1.0, [0.0, 0.0]);
        let mut ctrl = img.control().to_vec();
        let m = mid(&greville(&knots(1), 1));
        ctrl[m].y += 1e-3;
        let bumped = NurbsCurve2::new(KnotVector::clamped(knots(1), 1).unwrap(), ctrl, img.weights().to_vec()).unwrap();
        let o = plane.eval(p0[0], p0[1]);
        let d = plane.eval(p0[0] + pl[0], p0[1] + pl[1]) - o;
        let line = Curve3::line(o, d);
        let g = Pcurve::General(Arc::new(bumped));
        let r = both("tilted plane line carrier, bumped General", &plane, &g, lift(&g), &line, (0.0, 1.0));
        check("line carrier bumped General", r, true);
        let g = Pcurve::General(Arc::new(img));
        let r = both("tilted plane line carrier, true General", &plane, &g, lift(&g), &line, (0.0, 1.0));
        check("line carrier true General", r, false);
    }
    // Interval image whose u straddles: controls with wide u brackets.
    {
        let s = cylinder();
        let (img, carrier) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [0.0, 1e-3]);
        let wide = img.map_scalar(|x| Interval::from_bounds(x, x));
        let ctrl: Vec<Point2<Interval>> = wide
            .control()
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let w = 1e-7 * (1 + i % 3) as f64;
                Point2::new(Interval::from_bounds(0.1 - w, 0.1 + 2.0 * w), p.y)
            })
            .collect();
        let wide = NurbsCurve2::new(wide.knot_values().clone(), ctrl, wide.weights().to_vec()).unwrap();
        let w: Pcurve<Interval> = Pcurve::General(Arc::new(wide));
        println!("[probe] straddle class: {:?}", crate::pcurve_cache::StatedDoor::of(&s.map_scalar(Interval::from_f64), &w));
        let r = cert_at!(Interval, s.map_scalar(Interval::from_f64), w, carrier.map_scalar(Interval::from_f64), (0.0, 1.0));
        println!("[probe] straddling-u Interval General, 1mm along-ruling wiggle: {r:?}");
        if r.is_ok() {
            bad.push("straddle certified".into());
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Claim 3: a wrong PROJECTED image on an analytic chart: which check
/// ends it? (A second spelling if `ChartImageUnavailable`.)
#[test]
fn probe_wrong_projected_image_ending() {
    let s = cylinder();
    let (_img, straight) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [0.0, 0.0]);
    let (_img, wiggled) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [1e-5, 1e-3]);
    let Ok(image) = super::projected::project(&straight, Some((0.0, 1.0)), &s, band()) else {
        panic!("no projection");
    };
    let fi = Interval::from_f64;
    let image_i = super::projected::project(&straight.map_scalar(fi), Some((fi(0.0), fi(1.0))), &s.map_scalar(fi), band()).unwrap();
    let p = Pcurve::Projected(Box::new(image));
    let pi = Pcurve::Projected(Box::new(image_i));
    let r = both("projected image of straight, carrier wiggled", &s, &p, pi.clone(), &wiggled, (0.0, 1.0));
    for e in [&r.0, &r.1] {
        if let Err(e) = e {
            println!("   ending at build: {:?}", e.ending(crate::recourse::Reading::Build));
        }
    }
    let ok = both("projected image of straight, carrier straight", &s, &p, pi, &straight, (0.0, 1.0));
    assert!(ok.0.is_ok() && ok.1.is_ok());
}

/// Claim 5: the filed witnesses (expected to certify: the P1's class).
#[test]
fn probe_filed_witnesses() {
    let plane = Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let line = Curve3::line(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x());
    for b in [1e-3, 1e-6] {
        let h = Pcurve::free_harmonic(Point2::new(0.0, 0.3), Vec2::new(0.0, 0.0), Vec2::new(0.0, b), Vec2::new(1.0, 0.0));
        let f = Interval::from_f64;
        let hi = Pcurve::free_harmonic(Point2::new(f(0.0), f(0.3)), Vec2::new(f(0.0), f(0.0)), Vec2::new(f(0.0), f(b)), Vec2::new(f(1.0), f(0.0)));
        let r = both(&format!("harmonic witness b={b}"), &plane, &h, hi, &line, (0.0, 8.0 * core::f64::consts::PI));
        println!("   harmonic b={b} certified: {} / {}", r.0.is_ok(), r.1.is_ok());
    }
    // The rim circle of the unit cylinder at z = 0.5, an off-ruling
    // polyline in UV with its 1/128 vertex slid du.
    let cyl = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let circle = Curve3::circle(Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), 1.0, Vec3::unit_x());
    for du in [1e-3, 1e-6] {
        let k = vec![0.0, 0.0, 1.0 / 256.0, 1.0 / 128.0, 3.0 / 256.0, 1.0, 1.0];
        let ctrl: Vec<Point2<f64>> = [0.0, 1.0 / 256.0, 1.0 / 128.0 + du, 3.0 / 256.0, 1.0]
            .iter()
            .map(|&u| Point2::new(u, 0.5))
            .collect();
        let n = NurbsCurve2::new(KnotVector::clamped(k, 1).unwrap(), ctrl, vec![1.0; 5]).unwrap();
        let g = Pcurve::General(Arc::new(n));
        let r = both(&format!("rim off-ruling du={du}"), &cyl, &g, lift(&g), &circle, (0.0, 1.0));
        println!("   rim du={du} certified: {} / {}", r.0.is_ok(), r.1.is_ok());
    }
}

/// Claim 3, the second spelling: a wrong projected image (pure
/// along-ruling bump, so no incidence read can see it) and a wrong iso
/// line on a SPLINE chart: which check ends each, and the door's raw
/// refusal.
#[test]
fn probe_second_spellings() {
    let s = cylinder();
    let (_i, straight) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [0.0, 0.0]);
    let (_i, wiggled) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [0.0, 1e-3]);
    let image = super::projected::project(&straight, Some((0.0, 1.0)), &s, band()).unwrap();
    let fi = Interval::from_f64;
    let image_i = super::projected::project(&straight.map_scalar(fi), Some((fi(0.0), fi(1.0))), &s.map_scalar(fi), band()).unwrap();
    let p = Pcurve::Projected(Box::new(image));
    let r = both("projected, along-ruling bump 1mm", &s, &p, Pcurve::Projected(Box::new(image_i)), &wiggled, (0.0, 1.0));
    if let Err(e) = &r.0 {
        println!("   ending build: {:?}\n   ending rest: {:?}", e.ending(crate::recourse::Reading::Build), e.ending(crate::recourse::Reading::AtRest));
    }
    let raw = super::PcurveCache::<f64>::certify_stated_image(&p, (0.0, 1.0), &wiggled, &s, band(), Some(crate::FittedLane::certified())).err();
    println!("   raw door refusal: {raw:?}");

    // A spline chart: an affine bilinear patch.
    let a = Vec3::new(2.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 3.0, 0.5);
    let o = Point3::new(0.0, 0.0, 0.0);
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ns = geom::NurbsSurface::new(kv(), kv(), vec![o, o + b, o + a, o + a + b], vec![1.0; 4]).unwrap();
    let sp = Surface::Nurbs(Arc::new(ns));
    for h in [1e-3, 0.0] {
        let (img, carrier) = pair(&sp, 1, |t| [0.3, 0.1 + 0.8 * t], |_| 1.0, [0.0, h]);
        let iso = Pcurve::IsoLine { p0: Point2::new(0.3, 0.1), pl: Vec2::new(0.0, 0.8) };
        let r = both(&format!("spline chart iso h={h}"), &sp, &iso, lift(&iso), &carrier, (0.0, 1.0));
        if let Err(e) = &r.0 {
            println!("   ending build: {:?}", e.ending(crate::recourse::Reading::Build));
            let raw = super::PcurveCache::<f64>::certify_stated_image(&iso, (0.0, 1.0), &carrier, &sp, band(), Some(crate::FittedLane::certified())).err();
            println!("   raw door refusal: {raw:?}");
        }
        let g = Pcurve::General(Arc::new(img));
        let r = both(&format!("spline chart General h={h}"), &sp, &g, lift(&g), &carrier, (0.0, 1.0));
        if let Err(e) = &r.0 {
            println!("   ending build: {:?}", e.ending(crate::recourse::Reading::Build));
        }
    }
}

/// Claim 2: correct images far from the origin (rounding of the patch's
/// corners against the band), line and NURBS carriers.
#[test]
fn probe_far_correct_images() {
    let mut refused_rows = Vec::new();
    for far in [1e2, 1e3, 1e4, 1e5] {
        let n = Vec3::new(0.3, -0.5, 0.8).normalize();
        let u = (Vec3::new(1.0, 0.2, 0.0) - n * Vec3::new(1.0, 0.2, 0.0).dot(n)).normalize();
        let plane = Surface::Plane { origin: Point3::new(far, -far, 0.7 * far), normal: n, u_ref: u };
        let (p0, pl) = ([0.37 * far, -0.21 * far], [0.6 * far * 1e-2, 0.8 * far * 1e-2]);
        let (img, net) = pair(&plane, 1, |t| [p0[0] + pl[0] * t, p0[1] + pl[1] * t], |_| 1.0, [0.0, 0.0]);
        let o = plane.eval(p0[0], p0[1]);
        let line = Curve3::line(o, plane.eval(p0[0] + pl[0], p0[1] + pl[1]) - o);
        let iso = Pcurve::IsoLine { p0: Point2::new(p0[0], p0[1]), pl: Vec2::new(pl[0], pl[1]) };
        let g = Pcurve::General(Arc::new(img));
        for (name, im, c) in [("iso/net", &iso, &net), ("iso/line", &iso, &line), ("general/net", &g, &net), ("general/line", &g, &line)] {
            let r = both(&format!("far plane {far:e} {name}"), &plane, im, lift(im), c, (0.0, 1.0));
            if r.0.is_err() || r.1.is_err() {
                refused_rows.push(format!("plane {far:e} {name}: {:?} / {:?}", r.0.as_ref().err(), r.1.as_ref().err()));
            }
        }
        let cyl = Surface::Cylinder { origin: Point3::new(far, far, -far), axis: n, radius: 0.5 * far, u_ref: u };
        let (img, net) = pair(&cyl, 1, |t| [1.3, -0.2 * far + 0.01 * far * t], |_| 1.0, [0.0, 0.0]);
        let iso = Pcurve::IsoLine { p0: Point2::new(1.3, -0.2 * far), pl: Vec2::new(0.0, 0.01 * far) };
        let g = Pcurve::General(Arc::new(img));
        for (name, im) in [("iso", &iso), ("general", &g)] {
            let r = both(&format!("far cylinder {far:e} {name}"), &cyl, im, lift(im), &net, (0.0, 1.0));
            if r.0.is_err() || r.1.is_err() {
                refused_rows.push(format!("cyl {far:e} {name}: {:?} / {:?}", r.0.as_ref().err(), r.1.as_ref().err()));
            }
        }
    }
    println!("[probe] far refusals: {refused_rows:#?}");
}

/// Style/claim 1: `StatedDoor::of` reads an iso line's `pl.x` while
/// `fitted_lane` re-reads the iso net's controls (`of_net`). At the
/// interval scalar with a `u` that is an enclosure, not a point, do the
/// two readings agree?
#[test]
fn probe_iso_line_with_a_wide_u_at_interval() {
    let s = cylinder();
    let (_img, carrier) = pair(&s, 1, |t| [0.1, -1.0 + 2.5 * t], |_| 1.0, [0.0, 0.0]);
    let iso: Pcurve<Interval> = Pcurve::IsoLine {
        p0: Point2::new(Interval::from_bounds(0.1, 0.1 + 1e-15), Interval::from_f64(-1.0)),
        pl: Vec2::new(Interval::from_f64(0.0), Interval::from_f64(2.5)),
    };
    let si = s.map_scalar(Interval::from_f64);
    println!("[probe] wide-u iso door: {:?}", crate::pcurve_cache::StatedDoor::of(&si, &iso));
    let r = cert_at!(Interval, si, iso, carrier.map_scalar(Interval::from_f64), (0.0, 1.0));
    println!("[probe] wide-u iso on its ruling, carrier on it: {r:?}");
    if let Err(e) = &r {
        println!("   ending build: {:?}", e.ending(crate::recourse::Reading::Build));
    }
}

#[test]
fn probe_far_raw() {
    for far in [1e3, 1e4] {
        let n = Vec3::new(0.3, -0.5, 0.8).normalize();
        let u = (Vec3::new(1.0, 0.2, 0.0) - n * Vec3::new(1.0, 0.2, 0.0).dot(n)).normalize();
        let plane = Surface::Plane { origin: Point3::new(far, -far, 0.7 * far), normal: n, u_ref: u };
        let (p0, pl) = ([0.37 * far, -0.21 * far], [0.6 * far * 1e-2, 0.8 * far * 1e-2]);
        let (img, net) = pair(&plane, 1, |t| [p0[0] + pl[0] * t, p0[1] + pl[1] * t], |_| 1.0, [0.0, 0.0]);
        let o = plane.eval(p0[0], p0[1]);
        let line = Curve3::line(o, plane.eval(p0[0] + pl[0], p0[1] + pl[1]) - o);
        let iso = Pcurve::IsoLine { p0: Point2::new(p0[0], p0[1]), pl: Vec2::new(pl[0], pl[1]) };
        let g = Pcurve::General(Arc::new(img));
        for (name, im, c) in [("iso/net", &iso, &net), ("iso/line", &iso, &line), ("general/net", &g, &net), ("general/line", &g, &line)] {
            let door = crate::pcurve_cache::StatedDoor::of(&plane, im).unwrap();
            let raw = super::PcurveCache::<f64>::certify_stated_bound(door, im, (0.0, 1.0), (0.0, 1.0), c, &plane, band(), crate::FittedLane::certified());
            println!("[probe] raw far {far:e} {name}: {:?}", raw.err());
        }
    }
}
