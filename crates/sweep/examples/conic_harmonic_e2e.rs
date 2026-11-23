//! SCRATCH (oneround/conic-harmonic-probe): a frustum split by a
//! near-parabolic plane, then the part carrying the far-centred section
//! split again by a plane crossing that section beside its anchor.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::{Conic, Curve3};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::split;

fn main() {
    let tol = Tol::witness();
    let alpha: f64 = 1.0;
    let m = Affine3::rotation_about_axis(Point3::new(0.37, -0.21, 0.13), Vec3::new(0.3, 0.5, 0.8).normalize(), 0.7);
    let (h, r1) = (1.0, 0.5);
    let r0 = r1 + h * alpha.tan();
    for f in [0.9999, 0.99999, 0.999999] {
        let frustum = sweep::test_support::revolved_about_y(
            vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r0, 0.0), 0.0), (Point2::new(r1, h), 0.0), (Point2::new(0.0, h), 0.0)],
            sweep::Revolution::Full,
            tol,
        );
        let body = topo::finish(topo::transform_rigid(&frustum, &geom_core::test_support::rigid(&m, tol)).unwrap(), tol).unwrap();
        let phi = f * (core::f64::consts::FRAC_PI_2 - alpha);
        let n = m.transform_vec(Vec3::new(phi.sin(), phi.cos(), 0.0));
        let q = m.transform_point(Point3::new(0.0, 0.5 * h, 0.0));
        let first = match split(&body, &topo::test_support::split_plane(q, n / n.norm(), tol), tol) {
            Ok(r) => r,
            Err(e) => { println!("{{\"f\":{f:?},\"first\":\"refused\",\"why\":{:?}}}", format!("{e:?}").chars().take(120).collect::<String>()); continue; }
        };
        for (side, part) in [("above", &first.above), ("below", &first.below)] {
            let Some(part) = part.body() else { continue };
            for (_, geomc) in part.curves() {
                let Some(curve) = geomc.certified() else { continue };
                let Curve3::Ellipse(e) = curve.carrier() else { continue };
                let (t0, t1) = curve.params();
                let (s, (kc, ks)) = e.probe_stored();
                let (a, b) = e.semi_axes();
                for (gi, gamma) in [90.0_f64, 30.0, 5.0].into_iter().enumerate() {
                    for frac in [0.3, 0.6] {
                        let theta = t0 + (t1 - t0) * frac;
                        let p = e.point_at(theta);
                        let tan = e.derivative_at(theta).normalize();
                        let ax = e.axis();
                        let w = (ax.cross(tan).normalize() * 0.7_f64.cos() + ax * 0.7_f64.sin()).normalize();
                        let g = gamma.to_radians();
                        let normal = (tan * g.sin() + w * g.cos()).normalize();
                        let got = split(part, &topo::test_support::split_plane(p, normal, tol), tol);
                        let (kind, pts) = match &got {
                            Ok(r) => {
                                let mut pts = Vec::new();
                                for half in [&r.above, &r.below] {
                                    if let Some(bd) = half.body() {
                                        for (_, v) in bd.vertex_points() {
                                            if (v - p).norm() < 1e-3 { pts.push(format!("[{:?},{:?},{:?}]", v.x, v.y, v.z)); }
                                        }
                                    }
                                }
                                ("splits".to_owned(), pts)
                            }
                            Err(err) => (format!("refused:{}", format!("{err:?}").chars().filter(|c| !c.is_whitespace()).take(140).collect::<String>()), vec![]),
                        };
                        println!(
                            "{{\"f\":{f:?},\"side\":\"{side}\",\"gamma\":{gamma:?},\"gi\":{gi},\"frac\":{frac:?},\"span\":[{t0:?},{t1:?}],\"theta\":{theta:?},\"S\":[{:?},{:?},{:?}],\"axis\":[{:?},{:?},{:?}],\"u\":[{:?},{:?},{:?}],\"a\":{a:?},\"b\":{b:?},\"phase\":[{kc:?},{ks:?}],\"o\":[{:?},{:?},{:?}],\"n\":[{:?},{:?},{:?}],\"kind\":{kind:?},\"pts\":[{}]}}",
                            s.x, s.y, s.z, ax.x, ax.y, ax.z, e.u_ref().x, e.u_ref().y, e.u_ref().z, p.x, p.y, p.z, normal.x, normal.y, normal.z, pts.join(",")
                        );
                    }
                }
            }
        }
    }
}
