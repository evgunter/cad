#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::AtRestBody;

fn tol() -> Tol { Tol::witness() }
fn fin(w: &str, b: topo::Body<f64>) -> AtRestBody<f64> { topo::test_support::finished(w, b, tol()) }
fn rev(w: &str, v: &[(f64, f64, f64)]) -> AtRestBody<f64> {
    fin(w, revolved_about_y(v.iter().map(|&(x, y, q)| (Point2::new(x, y), q)).collect(), Revolution::Full, tol()))
}
pub fn torus() -> AtRestBody<f64> { rev("torus", &[(1.0, 0.0, 1.0), (3.0, 0.0, 1.0)]) }

fn run(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: (f64, f64, f64)) {
    let (va, vb, vab) = want;
    for (name, x, y, w) in [
        ("A∪B", a, b, va + vb - vab), ("B∪A", b, a, va + vb - vab),
        ("A∩B", a, b, vab), ("B∩A", b, a, vab),
        ("A∖B", a, b, va - vab), ("B∖A", b, a, vb - vab),
    ] {
        let r = match name {
            "A∪B" | "B∪A" => topo::union(x, y, tol()),
            "A∩B" | "B∩A" => topo::intersect(x, y, tol()),
            _ => topo::subtract(x, y, tol()),
        };
        let err = r.as_ref().err().map(|e| format!("{e:?}"));
        let shells = r.as_ref().ok().and_then(|b| b.body()).map(|b| b.body.shells().count());
        let w = if w.is_nan() { None } else { Some(w) };
        let j = crate::common::differential::judged(r, w, tol()).line;
        let line = format!("{j} shells={shells:?} err={err:?}");
        let line: String = line.chars().take(900).collect();
        eprintln!("PROBE {what} {name}: {line}");
    }
}


enum Piece { Seg((f64, f64)), Arc((f64, f64), (f64, f64)) }
/// Pappus: 2π∮x²/2 dy round a loop of segments and short circular arcs.
fn pappus(start: (f64, f64), pieces: &[Piece]) -> f64 {
    let mut at = start;
    let mut sum = 0.0;
    for p in pieces {
        match *p {
            Piece::Seg(to) => {
                sum += (to.1 - at.1) / 6.0 * (at.0 * at.0 + at.0 * to.0 + to.0 * to.0);
                at = to;
            }
            Piece::Arc(c, to) => {
                let r = ((at.0 - c.0).powi(2) + (at.1 - c.1).powi(2)).sqrt();
                let t0 = (at.1 - c.1).atan2(at.0 - c.0);
                let mut d = (to.1 - c.1).atan2(to.0 - c.0) - t0;
                while d > PI { d -= 2.0 * PI; }
                while d <= -PI { d += 2.0 * PI; }
                let f = |t: f64| 0.5 * r * (c.0 * c.0 * t.sin() + 2.0 * c.0 * r * (t / 2.0 + (2.0 * t).sin() / 4.0) + r * r * (t.sin() - t.sin().powi(3) / 3.0));
                sum += f(t0 + d) - f(t0);
                at = to;
            }
        }
    }
    2.0 * PI * sum.abs()
}
const C: (f64, f64) = (2.0, 0.0);

#[test]
fn probe_torus_poses() {
    eprintln!("PROBE eps={:?}", std::env::var("CAD_TOLERANCE_EPS"));
    let t = torus();
    let vt = 4.0 * PI * PI;
    let only = std::env::var("PROBE_ONLY").unwrap_or_default();
    // straddles: vertex P on the tube circle, Q_in inside, Q_out outside, X where Q_in-Q_out leaves the disc
    let sq = |v: f64| v.sqrt();
    let h = |y: f64| sq(1.0 - y * y);
    let s3 = 3f64.sqrt() / 2.0;
    let poses: Vec<(&str, (f64, f64), (f64, f64), (f64, f64), (f64, f64))> = vec![
        ("outer-equator", (3.0, 0.0), (2.5, 0.5), (3.5, 0.5), (2.0 + h(0.5), 0.5)),
        ("inner-equator", (1.0, 0.0), (1.5, 0.5), (0.5, 0.5), (2.0 - h(0.5), 0.5)),
        ("outer-upper", (2.5, s3), (2.2, 0.966), (2.8, 0.966), (2.0 + h(0.966), 0.966)),
        ("inner-upper", (1.5, s3), (1.8, 0.966), (1.2, 0.966), (2.0 - h(0.966), 0.966)),
        ("outer-lower", (2.5, -s3), (2.2, -0.966), (2.8, -0.966), (2.0 + h(0.966), -0.966)),
        ("top", (2.0, 1.0), (2.3, 0.7), (2.3, 1.3), (2.3, h(0.3))),
    ];
    for (what, p, qi, qo, x) in poses {
        if !only.is_empty() && !what.contains(&only) { continue; }
        let tri = [p, qi, qo];
        let area = ((qi.0 - p.0) * (qo.1 - p.1) - (qo.0 - p.0) * (qi.1 - p.1)).abs() / 2.0;
        let vs = 2.0 * PI * area * (p.0 + qi.0 + qo.0) / 3.0;
        let vab = pappus(p, &[Piece::Seg(qi), Piece::Seg(x), Piece::Arc(C, p)]);
        let s = rev(what, &tri.map(|(x, y)| (x, y, 0.0)));
        run(what, &t, &s, (vt, vs, vab));
    }
    if only.is_empty() || only == "disc" {
        let d = rev("disc", &[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (2.0, 1.0, 0.0), (0.0, 1.0, 0.0)]);
        run("disc-under-top", &t, &d, (vt, 4.0 * PI, 2.0 * PI * (FRAC_PI_2 - 1.0 / 3.0)));
        let d = rev("disc", &[(0.0, 1.0, 0.0), (2.0, 1.0, 0.0), (2.0, 2.0, 0.0), (0.0, 2.0, 0.0)]);
        run("disc-on-top", &t, &d, (vt, 4.0 * PI, 0.0));
        let d = rev("disc", &[(0.0, 0.0, 0.0), (2.5, 0.0, 0.0), (2.5, s3, 0.0), (0.0, s3, 0.0)]);
        let vab = pappus((1.0, 0.0), &[Piece::Seg((2.5, 0.0)), Piece::Seg((2.5, s3)), Piece::Seg((1.5, s3)), Piece::Arc(C, (1.0, 0.0))]);
        run("disc-outer-upper", &t, &d, (vt, PI * 6.25 * s3, vab));
    }
    if only.is_empty() || only == "meridian" {
        let m = revolved_about_y(vec![(Point2::new(1.0, 0.0), 0.0), (Point2::new(1.5, 0.5), 0.0), (Point2::new(0.5, 0.5), 0.0)], Revolution::Full, tol());
        let place = Affine3::translation(Vec3::new(2.0, 0.0, 0.0)) * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), FRAC_PI_2);
        let m = fin("meridian", topo::transform_rigid(&m, &geom_core::test_support::rigid(&place, tol())).unwrap());
        run("meridian-straddle", &t, &m, (vt, f64::NAN, f64::NAN));
    }
}
