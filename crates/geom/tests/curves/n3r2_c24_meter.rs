//! CERT-N3 R2 probe: C24's number, re-taken (release; the root profile
//! keeps debug-assertions on).

#![allow(clippy::unwrap_used, clippy::panic, clippy::approx_constant)]

use geom::Curve3;
use geom_core::{Point3, Vec3};

fn bench(carrier: &Curve3<f64>, n: u32) -> (f64, f64) {
    // The consumer's shape: deriv(t) + deriv2(t) at one t.
    let mut acc = Vec3::new(0.0, 0.0, 0.0);
    let t0 = std::time::Instant::now();
    for i in 0..n {
        let t = 0.3 + f64::from(i) * 1e-7;
        let d = carrier.deriv(t);
        let dd = carrier.deriv2(t);
        acc = acc + d + dd;
    }
    let pair = t0.elapsed().as_secs_f64() / f64::from(n) * 1e9;
    // The fused jet: one sin_cos, the frame built once.
    let (center, u, v, su, sv) = match carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (*center, *u_ref, axis.cross(*u_ref), *radius, *radius),
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (*center, *u_ref, axis.cross(*u_ref), *major, *minor),
        _ => panic!("conic only"),
    };
    let _ = center;
    let mut acc2 = Vec3::new(0.0, 0.0, 0.0);
    let t1 = std::time::Instant::now();
    for i in 0..n {
        let t = 0.3 + f64::from(i) * 1e-7;
        let (s, c) = t.sin_cos();
        let d = u * (-su * s) + v * (sv * c);
        let dd = u * (-su * c) + v * (-sv * s);
        acc2 = acc2 + d + dd;
    }
    let jet = t1.elapsed().as_secs_f64() / f64::from(n) * 1e9;
    assert!(acc.x.is_finite() && acc2.x.is_finite());
    (pair, jet)
}

#[test]
fn n3r2_c24_meter() {
    let axis = Vec3::new(0.3, 0.4, 0.866_025_403_784).normalize();
    let helper = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = axis.cross(helper).normalize();
    let circle = Curve3::Circle {
        center: Point3::new(0.1, 0.2, 0.3),
        axis,
        radius: 1.3,
        u_ref,
    };
    let ellipse = Curve3::Ellipse {
        center: Point3::new(0.1, 0.2, 0.3),
        axis,
        major: 2.1,
        minor: 0.7,
        u_ref,
    };
    for (name, c) in [("circle", &circle), ("ellipse", &ellipse)] {
        let _ = bench(c, 1 << 20); // warm
        let (pair, jet) = bench(c, 20_480_000);
        println!(
            "{name}: pair {pair:.2} ns, jet {jet:.2} ns, saving {:.2} ns",
            pair - jet
        );
    }
}
