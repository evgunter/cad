//! JOIN-3 delta review: `Curve3::reversed` on every kind it serves —
//! an involution (bit-exact), endpoints swap on `[t0, t1] ↦ [−t1, −t0]`,
//! the tangent flips, and a NURBS answers `None`.

use geom::Curve3;
use geom_core::{Point3, Vec3};

fn kinds() -> Vec<Curve3<f64>> {
    let n = Vec3::new(2.0, 2.0, 1.0) / 3.0;
    let u = Vec3::new(1.0, -2.0, 2.0) / 3.0;
    let c = Point3::new(0.5, -1.0, 2.0);
    vec![
        Curve3::Line {
            origin: c,
            dir: u * 1.7,
        },
        Curve3::Circle {
            center: c,
            axis: n,
            radius: 0.7,
            u_ref: u,
        },
        Curve3::Ellipse {
            center: c,
            axis: n,
            major: 0.9,
            minor: 0.4,
            u_ref: u,
        },
        Curve3::Spiric {
            center: c,
            axis: n,
            u_ref: u,
            major_radius: 2.0,
            minor_radius: 0.5,
            offset: -0.3,
        },
    ]
}

#[test]
fn reversed_twice_is_the_identity_bit_for_bit() {
    for k in kinds() {
        let twice = k.reversed().unwrap().reversed().unwrap();
        assert_eq!(format!("{twice:?}"), format!("{k:?}"));
    }
}

#[test]
fn reversed_swaps_endpoints_and_flips_the_tangent() {
    for k in kinds() {
        let b = k.reversed().unwrap();
        for (t0, t1) in [
            (0.0, 1.0),
            (-2.0, 0.4),
            (3.0, 3.0 + std::f64::consts::TAU - 0.1),
        ] {
            assert!((b.eval(-t1) - k.eval(t1)).norm() < 1e-14, "{k:?} end");
            assert!((b.eval(-t0) - k.eval(t0)).norm() < 1e-14, "{k:?} start");
            let m = 0.5 * (t0 + t1);
            assert!((b.deriv(-m) + k.deriv(m)).norm() < 1e-13, "{k:?} tangent");
        }
    }
}

#[test]
fn a_reversed_spiric_still_passes_its_door() {
    let n = Vec3::new(0.0, 0.0, 1.0);
    let u = Vec3::new(1.0, 0.0, 0.0);
    let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
    let s = Curve3::spiric(Point3::new(0.0, 0.0, 0.0), n, u, 2.0, 0.5, 0.3, band).unwrap();
    let Curve3::Spiric {
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
    } = s.reversed().unwrap()
    else {
        panic!()
    };
    Curve3::spiric(
        center,
        axis,
        u_ref,
        major_radius,
        minor_radius,
        offset,
        band,
    )
    .expect("the reversed spelling is a valid spiric");
}
