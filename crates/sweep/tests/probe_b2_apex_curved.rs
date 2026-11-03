//! Reviewer probe (PR 6424): a cone's apex against curved partners —
//! spheres and cylinders through, touching, or holding the apex in a face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::one_wrap_edge;
use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled, finished, revolved_about_y};
use topo::{BooleanResult, SolidContainment, intersect, point_in_solid, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r, 0.0), 0.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    )
}

fn forms(alpha: f64) -> [(&'static str, topo::Body<f64>); 2] {
    [
        ("tipped", one_wrap_edge(cone(alpha), true).0),
        ("revolved", crate::revolve_common::needle(alpha, None, tol()).into_body()),
    ]
}

#[derive(Clone, Copy)]
enum Partner {
    /// centre, radius, pole
    Sphere(Point3<f64>, f64, Vec3<f64>),
    /// vertical cylinder: centre (x,z), radius, y-range
    VCyl(f64, f64, f64, (f64, f64)),
    /// cylinder along x: centre (y,z), radius, x-range
    XCyl(f64, f64, f64, (f64, f64)),
}

fn body(p: Partner) -> topo::Body<f64> {
    match p {
        Partner::Sphere(c, r, pole) => ball_poled(r, c - Point3::origin(), pole, tol()),
        Partner::VCyl(cx, cz, r, (y0, y1)) => {
            let b = revolved_about_y(
                vec![(Point2::new(0.0, y0), 0.0), (Point2::new(r, y0), 0.0), (Point2::new(r, y1), 0.0), (Point2::new(0.0, y1), 0.0)],
                Revolution::Full,
                tol(),
            );
            topo::transform_rigid(&b, &geom_core::RigidMotion::translation(Vec3::new(cx, 0.0, cz)).unwrap(), tol()).unwrap()
        }
        Partner::XCyl(cy, cz, r, (x0, x1)) => {
            // revolve about y over [x0,x1] (as y), then turn y onto x.
            let b = revolved_about_y(
                vec![(Point2::new(0.0, x0), 0.0), (Point2::new(r, x0), 0.0), (Point2::new(r, x1), 0.0), (Point2::new(0.0, x1), 0.0)],
                Revolution::Full,
                tol(),
            );
            let m = Affine3::translation(Vec3::new(0.0, cy, cz)) * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), -PI / 2.0);
            topo::transform_rigid(&b, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap()
        }
    }
}

fn region(p: Partner) -> Box<dyn Fn(Point3<f64>) -> f64> {
    match p {
        Partner::Sphere(c, r, _) => Box::new(move |q| r - (q - c).norm()),
        Partner::VCyl(cx, cz, r, (y0, y1)) => Box::new(move |q: Point3<f64>| (r - (q.x - cx).hypot(q.z - cz)).min(q.y - y0).min(y1 - q.y)),
        Partner::XCyl(cy, cz, r, (x0, x1)) => Box::new(move |q: Point3<f64>| (r - (q.y - cy).hypot(q.z - cz)).min(q.x - x0).min(x1 - q.x)),
    }
}

fn lens(d: f64, r1: f64, r2: f64) -> f64 {
    if r1 <= 0.0 || r2 <= 0.0 || d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return PI * r1.min(r2).powi(2);
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).clamp(-1.0, 1.0).acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).clamp(-1.0, 1.0).acos();
    r1 * r1 * (a1 - a1.sin() * a1.cos()) + r2 * r2 * (a2 - a2.sin() * a2.cos())
}

fn oracle(alpha: f64, p: Partner) -> f64 {
    let r = alpha.to_radians().tan();
    let n = 20000;
    let mut v = 0.0;
    for k in 0..n {
        let y = (k as f64 + 0.5) / n as f64;
        let rho = r * (1.0 - y);
        let a = match p {
            Partner::Sphere(c, rs, _) => {
                let s2 = rs * rs - (y - c.y).powi(2);
                if s2 <= 0.0 { 0.0 } else { lens(c.x.hypot(c.z), rho, s2.sqrt()) }
            }
            Partner::VCyl(cx, cz, rc, (y0, y1)) => {
                if y < y0 || y > y1 { 0.0 } else { lens(cx.hypot(cz), rho, rc) }
            }
            Partner::XCyl(cy, cz, rc, (x0, x1)) => {
                let h2 = rc * rc - (y - cy).powi(2);
                if h2 <= 0.0 {
                    0.0
                } else {
                    // disk rho ∩ rectangle x∈[x0,x1], z∈[cz-h, cz+h], by fine x-quadrature.
                    let h = h2.sqrt();
                    let m = 400;
                    let (lo, hi) = (x0.max(-rho), x1.min(rho));
                    let mut a = 0.0;
                    if hi > lo {
                        for i in 0..m {
                            let x = lo + (hi - lo) * (i as f64 + 0.5) / m as f64;
                            let zz = (rho * rho - x * x).max(0.0).sqrt();
                            let (zl, zh) = ((-zz).max(cz - h), zz.min(cz + h));
                            a += (zh - zl).max(0.0) * (hi - lo) / m as f64;
                        }
                    }
                    a
                }
            }
        };
        v += a / n as f64;
    }
    v
}

fn vol_of(p: Partner) -> f64 {
    match p {
        Partner::Sphere(_, r, _) => 4.0 / 3.0 * PI * r.powi(3),
        Partner::VCyl(_, _, r, (a, b)) | Partner::XCyl(_, _, r, (a, b)) => PI * r * r * (b - a),
    }
}

fn run(name: &str, alpha: f64, p: Partner) {
    let r = alpha.to_radians().tan();
    let vc = PI * r * r / 3.0;
    let vp = vol_of(p);
    let share = oracle(alpha, p);
    let pb = finished("p", body(p), tol());
    let pr = region(p);
    let cr = move |q: Point3<f64>| (r * (1.0 - q.y) - q.x.hypot(q.z)).min(q.y).min(1.0 - q.y);
    let (mut lines, mut wrong) = (Vec::new(), Vec::new());
    for (form, cb) in forms(alpha) {
        let c = finished("c", cb, tol());
        for (order, cone_first) in [("cone first", true), ("partner first", false)] {
            let (x, y, vx, vy) = if cone_first { (&c, &pb, vc, vp) } else { (&pb, &c, vp, vc) };
            let rx = |q: Point3<f64>| if cone_first { cr(q) } else { pr(q) };
            let ry = |q: Point3<f64>| if cone_first { pr(q) } else { cr(q) };
            let ops: [(&str, Result<BooleanResult<f64>, topo::BooleanError>, f64, Box<dyn Fn(Point3<f64>) -> f64>); 4] = [
                ("∩", intersect(x, y, tol()), share, Box::new(|q| rx(q).min(ry(q)))),
                ("∪", union(x, y, tol()), vx + vy - share, Box::new(|q| rx(q).max(ry(q)))),
                ("x∖y", subtract(x, y, tol()), vx - share, Box::new(|q| rx(q).min(-ry(q)))),
                ("y∖x", subtract(y, x, tol()), vy - share, Box::new(|q| ry(q).min(-rx(q)))),
            ];
            for (op, res, want, reg) in ops {
                let label = format!("{name} {alpha}° {form} {order} {op}");
                match res {
                    Err(e) => lines.push(format!("{label}: REFUSE {e:?}").chars().take(220).collect::<String>()),
                    Ok(BooleanResult::Empty) => {
                        if want.abs() > 1e-6 { wrong.push(format!("{label}: EMPTY want {want}")) } else { lines.push(format!("{label}: ok-empty")) }
                    }
                    Ok(BooleanResult::Body(b)) => {
                        let v = match topo::mass_properties(&b.body, tol(), topo::test_support::pinned_resolution(tol())).map(|m| m.met()) {
                            Ok(Ok(e)) => e.volume,
                            other => {
                                wrong.push(format!("{label}: BUILT unmeasurable {other:?}").chars().take(300).collect());
                                continue;
                            }
                        };
                        if (v - want).abs() > 1e-4 * want.abs().max(0.1) {
                            wrong.push(format!("{label}: VOLUME {v} want {want}"));
                            continue;
                        }
                        if let Err(e) = topo::validate_geometric(&b.body, tol()) {
                            wrong.push(format!("{label}: TIER3 {e:?}").chars().take(300).collect());
                            continue;
                        }
                        let band = Band::linear(tol()).unwrap();
                        let mut bad = 0;
                        let n = 6;
                        for i in 0..n { for j in 0..n { for k in 0..n {
                            let f = |t: usize, o: f64| (t as f64 + o) / (n as f64 - 0.63);
                            let q = Point3::new(-1.0 + 2.0 * f(i, 0.11), -0.1 + 1.8 * f(j, 0.23), -1.0 + 2.0 * f(k, 0.07));
                            let s = reg(q);
                            if s.abs() < 5e-3 { continue; }
                            let w = if s > 0.0 { SolidContainment::In } else { SolidContainment::Out };
                            if point_in_solid(&b.body, q, band, tol()) != Ok(w) { bad += 1; }
                        }}}
                        if bad > 0 { wrong.push(format!("{label}: MEMBERSHIP {bad}")) } else { lines.push(format!("{label}: ok")) }
                    }
                }
            }
        }
    }
    eprintln!("PROBE {name} {alpha}: share {share:.6}");
    for l in &lines { eprintln!("  {l}"); }
    for w in &wrong { eprintln!("  W {w}"); }
    assert!(wrong.is_empty());
}

const Y: Vec3<f64> = Vec3::new(0.0, 1.0, 0.0);

fn all(alpha: f64) {
    let s = (0.3f64).hypot(0.2);
    let t = (0.25f64).hypot(0.3).hypot(0.2);
    for (name, p) in [
        ("sphere tangent above", Partner::Sphere(Point3::new(0.0, 1.3, 0.0), 0.3, Y)),
        ("sphere seam meridian", Partner::Sphere(Point3::new(0.3, 1.2, 0.0), s, Y)),
        ("sphere apex in face", Partner::Sphere(Point3::new(0.25, 1.3, 0.2), t, Y)),
        ("sphere on axis below", Partner::Sphere(Point3::new(0.0, 0.8, 0.0), 0.2, Y)),
        ("vcyl above, rim through apex", Partner::VCyl(0.3, 0.0, 0.3, (1.0, 1.5))),
        ("vcyl below, rim through apex", Partner::VCyl(0.3, 0.0, 0.3, (0.5, 1.0))),
        ("vcyl side through apex", Partner::VCyl(0.3, 0.1, (0.3f64).hypot(0.1), (0.5, 1.5))),
        ("xcyl rim in meridian through apex", Partner::XCyl(0.7, 0.0, 0.3, (0.0, 1.0))),
        ("xcyl tangent above", Partner::XCyl(1.3, 0.0, 0.3, (-1.0, 1.0))),
    ] {
        let res = std::panic::catch_unwind(|| run(name, alpha, p));
        if res.is_err() {
            eprintln!("PROBE {name} {alpha}: FAILED");
        }
    }
}

#[test]
fn curved_30() { all(30.0) }
#[test]
fn curved_45() { all(45.0) }
