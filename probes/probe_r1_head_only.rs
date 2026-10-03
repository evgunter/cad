//! Reviewer probe (reach-dual3977-r1), HEAD ONLY: the width of the
//! interval re-derivation check 7 now certifies through, against the
//! f64 sum's actual error (exact volume is a box's closed form).
use geom::Surface;
use geom_core::{Band, Tol};

#[test]
fn probe_r1_interval_width() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for &(dx, dz) in &[(1e-3, 1e-7), (1e-2, 1e-7), (1e-3, 1e-6)] {
        for &d in &[10.0, 100.0, 1e3, 3e3] {
            let mut body = crate::test_support::brick::<f64>((0.0, dx), (0.0, dx), (0.0, dz), tol);
            let surfaces: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
            for key in surfaces {
                let surface = body.surfaces.get_mut(key).unwrap();
                let Surface::Plane { origin, normal, u_ref } = *surface else { panic!() };
                *surface = Surface::Plane { origin: origin + (u_ref * 1.0 + normal.cross(u_ref) * 0.37) * d, normal, u_ref };
            }
            let solid = body.solids().next().unwrap().0;
            let faces = body.faces_of_solid(solid).unwrap();
            let (out, _) = crate::props::sign_walk(&body, &faces, band, tol, Some(crate::props::QuadLane::certified()),
                |r| Some((r.enclosure().volume_lo, format!("{:?}", r.interval().unwrap().unwrap().volume_lo))), |_| (f64::NAN, String::new())).unwrap();
            println!("PROBE|{dx:e}x{dx:e}x{dz:e}|d={d:e}|exact={:+.3e}|f64={:+.3e}|interval={}", dx*dx*dz, out.0, out.1);
        }
    }
}
