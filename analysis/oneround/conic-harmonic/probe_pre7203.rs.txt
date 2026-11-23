//! SCRATCH (conic-harmonic probe, sweep item 3): `curve_pair::meet` of a
//! line through a true point of a cone's section and the section the
//! mint answers. Self-contained, so the same file runs on any tree.
#![allow(clippy::all, clippy::pedantic, clippy::disallowed_methods, dead_code)]
use geom::{Curve3, Frame, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

fn triple() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let ez = Vec3::new(0.36, 0.48, 0.8);
    let ex = Vec3::new(0.8, -0.6, 0.0);
    (ex, ez.cross(ex), ez)
}
const ALPHA: f64 = 1.3;

fn mint(tau: f64, band: Band) -> Option<(Curve3<f64>, Point3<f64>, Point3<f64>, Vec3<f64>)> {
    let (ex, ey, ez) = triple();
    let apex = Point3::new(1.3, -0.7, -0.6);
    let cone = Surface::Cone { frame: Frame::new(apex, ez, ex), half_angle: ALPHA };
    let n = ex * tau.sin() + ez * tau.cos();
    let q = apex + ez * 1.0;
    let plane = Surface::Plane { frame: Frame::new(q, n, ey) };
    let reach = crate::Reach::fixture(apex, 10.0);
    match crate::intersect::plane_cone_section(&plane, &cone, &reach, band) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(c)) => Some((c, apex, q, n)),
        _ => None,
    }
}

fn major(c: &Curve3<f64>) -> f64 {
    let Curve3::Ellipse(e) = c else { unreachable!() };
    let (a, b) = e.semi_axes();
    a.max(b)
}

#[test]
#[ignore = "probe"]
fn pre7203_meet_through_a_true_section_point() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (ex, ey, ez) = triple();
    for target in [1e4, 1e5, 1e6, 1e7, 1e8, 1e9] {
        let (mut lo, mut hi) = (1e-3, core::f64::consts::FRAC_PI_2 - ALPHA - 1e-12);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if mint(mid, band).is_some_and(|m| major(&m.0) < target) { lo = mid } else { hi = mid }
        }
        let Some((carrier, apex, q, n)) = mint(lo, band) else {
            println!("PRE {{\"target\":{target:?},\"mint\":\"refused\"}}");
            continue;
        };
        for azimuth in [0.3_f64, -1.0, 3.09] {
            // A generator of the cone on the side the plane cuts nearest
            // the apex, turned `azimuth` about the axis; where it meets
            // the plane is a point of the true section.
            let radial = ex * azimuth.cos() + ey * azimuth.sin();
            let g = ez * ALPHA.cos() + radial * ALPHA.sin();
            let t = (q - apex).dot(n) / g.dot(n);
            let p = apex + g * t;
            let d = (g - n * g.dot(n)).normalize();
            let line = Curve3::line(p - d * 2.0, d);
            // The section's parameter at p: the nearer of the feet seeded at the two vertices.
            let foot = [0.0, core::f64::consts::PI, core::f64::consts::FRAC_PI_2, -core::f64::consts::FRAC_PI_2]
                .into_iter()
                .map(|s| carrier.foot(p, s).seed())
                .min_by(|x, y| (carrier.eval(*x) - p).norm().total_cmp(&(carrier.eval(*y) - p).norm()))
                .unwrap();
            let Curve3::Ellipse(e) = &carrier else { unreachable!() };
            let (a, b) = e.semi_axes();
            let half = 10.0 / a.min(b);
            let got = crate::curve_pair::meet(
                &crate::curve_pair::Span::new(&line, (0.0, 4.0)),
                &crate::curve_pair::Span::new(&carrier, (foot - half, foot + half)),
                band,
            );
            let kind = match &got {
                Ok(m) => format!("ok {}", m.points.len()),
                Err(r) => format!("refused {}", format!("{:?}", r.fault).chars().take(60).collect::<String>()),
            };
            let pt = got.ok().and_then(|m| m.points.first().map(|m| m.point));
            println!(
                "PRE {{\"target\":{target:?},\"a\":{a:?},\"b\":{b:?},\"azimuth\":{azimuth:?},\"apex\":[{:?},{:?},{:?}],\"axis\":[{:?},{:?},{:?}],\"alpha\":{ALPHA:?},\"q\":[{:?},{:?},{:?}],\"n\":[{:?},{:?},{:?}],\"p\":[{:?},{:?},{:?}],\"foot_miss\":{:?},\"kind\":{kind:?},\"meet\":{}}}",
                apex.x, apex.y, apex.z, ez.x, ez.y, ez.z, q.x, q.y, q.z, n.x, n.y, n.z, p.x, p.y, p.z,
                (carrier.eval(foot) - p).norm(),
                pt.map_or("null".to_owned(), |m| format!("[{:?},{:?},{:?}]", m.x, m.y, m.z))
            );
        }
    }
}
