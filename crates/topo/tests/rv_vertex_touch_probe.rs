//! REVIEW PROBE (PR 6212, claim 3): two prisms meeting at one vertex
//! `V` (the origin) with a face of each within a hair of coplanar, the
//! line of the two faces outside both faces' sectors, and one bound of
//! A's near face reading in band off B's plane. Geometrically the two
//! bodies share only `V`. Prints each case's outcome; asserts nothing.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
use topo::{AtRestBody, BooleanResult};

fn t() -> Tol {
    Tol::witness()
}

fn at(deg: f64) -> Vec3<f64> {
    let (s, c) = deg.to_radians().sin_cos();
    Vec3::new(c, s, 0.0)
}

/// A: a wedge-thin prism whose bottom face lies in z = 0 between the
/// bounds at `theta` (reach `l1`) and `theta + w`, its side face along
/// the `theta` bound at `phi2` to the plane.
fn a_body(theta: f64, w: f64, l1: f64, phi2: f64) -> AtRestBody<f64> {
    let e1 = at(theta) * l1;
    let e2 = at(theta + w) * l1;
    let u = at(theta + 90.0);
    let (s2, c2) = phi2.to_radians().sin_cos();
    let e3 = (u * c2 + Vec3::new(0.0, 0.0, s2)) * 0.5 + at(theta) * 0.3;
    let mut body = topo::Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
        (0.0, 1.0),
        |x, y, z| {
            let p = e1 * x + e2 * y + e3 * z;
            Point3::new(p.x, p.y, p.z)
        },
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("A", body, t())
}

/// B: an upright prism below z = 0 over the wedge `b0..b1` of radius
/// 2, turned by `phi` about x.
fn b_body(b0: f64, b1: f64, phi: f64) -> AtRestBody<f64> {
    let p0 = at(b0) * 2.0;
    let p1 = at(b1) * 2.0;
    let (s, c) = phi.sin_cos();
    let mut body = topo::Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(0.0, 0.0), (p0.x, p0.y), (p1.x, p1.y)],
        (-1.0, 0.0),
        |x, y, z| Point3::new(x, y * c - z * s, y * s + z * c),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("B", body, t())
}

fn vol(what: &str, b: &AtRestBody<f64>) -> f64 {
    topo::test_support::pinned_volume(what, b, t()).volume
}

fn outcome(
    what: &str,
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    want: Option<f64>,
) -> String {
    match r {
        Ok(BooleanResult::Empty) => {
            format!("EMPTY{}", if want.is_none() { " ok" } else { " WRONG" })
        }
        Ok(BooleanResult::Body(bb)) => {
            let g = topo::validate_geometric(&bb.body, t()).err();
            let p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, t()).err();
            let v = vol(what, &bb.body);
            let good = want.is_some_and(|w| (v - w).abs() < 1e-7) && g.is_none() && p.is_none();
            format!(
                "BODY v={v:.12} want={want:?} t3={} t3'={} {}",
                g.is_none(),
                p.is_none(),
                if good { "ok" } else { "WRONG" }
            )
        }
        Err(e) => {
            let s = format!("{e:?}");
            let d = s.find("decision").unwrap_or(0);
            let p = s
                .find("predicate")
                .map(|i| s[i..s.len().min(i + 40)].to_string())
                .unwrap_or_default();
            if std::env::var_os("RV_FULL").is_some() {
                println!("FULL {s}");
            }
            format!("REFUSED {} {}", &s[d..s.len().min(d + 48)], p)
        }
    }
}

#[test]
#[ignore = "review probe"]
fn rv_vertex_touch_near_coplanar_probe() {
    let eps = t().eps();
    let l1 = 1.0;
    for theta in [5.0f64, 11.0] {
        for (b0, b1) in [(-60.0, -30.0), (-60.0, -2.0), (-170.0, -120.0)] {
            for phi2 in [90.0, 30.0, 10.0, 5.0, 2.0] {
                for k in [0.5, 1.5, 3.0, 6.0, 9.0, 30.0] {
                    for sign in [1.0, -1.0] {
                        let phi = sign * (k * eps / (l1 * theta.to_radians().sin())).asin();
                        let what =
                            format!("theta {theta} B {b0}..{b1} phi2 {phi2} k {k} sign {sign}");
                        let a = a_body(theta, 40.0, l1, phi2);
                        let b = b_body(b0, b1, phi);
                        let (va, vb) = (vol("A", &a), vol("B", &b));
                        let u = outcome(&what, topo::union(&a, &b, t()), Some(va + vb));
                        let i = outcome(&what, topo::intersect(&a, &b, t()), None);
                        let s = outcome(&what, topo::subtract(&a, &b, t()), Some(va));
                        let s2 = outcome(&what, topo::subtract(&b, &a, t()), Some(vb));
                        println!("RV| {what} | U {u} | I {i} | A-B {s} | B-A {s2}");
                    }
                }
            }
        }
    }
}
