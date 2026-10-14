//! Review 5089 r2 probes: a near-horn Villarceau circle moved along its
//! own normal, which the 64-sample incidence test can read on the torus
//! while its plane gate is definite.

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::chart_pcurve;
use geom_core::{Point3, Vec3};

#[test]
fn p1_near_horn_plane_gate_reaches_the_gates() {
    let e = eps();
    let big = 2.0;
    for gap in [1e-6, 1e-4] {
        let r = big - gap;
        let surface = Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            major_radius: big,
            minor_radius: r,
            u_ref: Vec3::unit_x(),
        };
        let st = r / big;
        let ct = (1.0 - st * st).sqrt();
        let n = Vec3::new(-st, 0.0, ct);
        // In-plane frame: the pinch direction from the centre is -y.
        let pinch = Vec3::new(0.0, -1.0, 0.0);
        let side = n.cross(pinch);
        for half in [0.0, 0.5] {
            let psi = half * core::f64::consts::TAU / 64.0;
            // u_ref such that the pinch sits at t = psi past a sample.
            let u_ref = pinch * psi.cos() - side * psi.sin();
            for k in [0.0, 3.0, 10.0, 30.0, 100.0] {
                let c = Point3::new(0.0, r, 0.0) + n * (k * e);
                let carrier = Curve3::Circle { center: c, axis: n, radius: big, u_ref };
                let got = chart_pcurve(&carrier, &surface, band());
                let what = match &got {
                    Ok(p) => format!("Ok({:?})", core::mem::discriminant(p)),
                    Err(err) => format!("{err:?}"),
                };
                println!("gap={gap:e} half={half} shift={k}eps -> {}", &what[..what.len().min(260)]);
            }
        }
    }
}
