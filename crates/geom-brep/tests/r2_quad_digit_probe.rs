//! R2 digit probe: `nurbs_patch_face` on the multiplicity-=-degree
//! rational wall (the `Dir::Raw` / `raw_deriv` path), full-precision
//! Debug output for a bit-level merge-base vs HEAD comparison.
use geom_core::spline::KnotVector;
use geom_core::{Band, Tol, ring_interval::RingInterval};
use geom_brep::props::quad::{RVec3, nurbs_patch_face};

fn pt(x: f64) -> RingInterval {
    RingInterval::point(x)
}

#[test]
fn r2_quad_raw_digit_probe() {
    let band = Band::linear(Tol::witness()).unwrap();
    let kv_v = KnotVector::unit_segment(1);
    let (pu, nv, height) = (3usize, 2usize, 2.0f64);
    for mult in [2usize, 3] {
        let mut knots = vec![0.0; pu + 1];
        knots.extend(core::iter::repeat_n(0.5, mult));
        knots.extend(core::iter::repeat_n(1.0, pu + 1));
        let kv_u = KnotVector::clamped(knots, pu).unwrap();
        let count = kv_u.control_count();
        let profile: Vec<(f64, f64)> = (0..count)
            .map(|i| {
                let s = i as f64 / (count - 1) as f64;
                let (a, r) = (s * core::f64::consts::FRAC_PI_2, 0.2f64.mul_add(s, 1.0));
                (r * a.cos(), r * a.sin())
            })
            .collect();
        let profile_w: Vec<f64> = (0..count)
            .map(|i| 0.06f64.mul_add((i % 3) as f64, 1.0))
            .collect();
        let mut net: Vec<RVec3> = Vec::with_capacity(count * nv);
        let mut ws: Vec<f64> = Vec::with_capacity(count * nv);
        for (i, (x, y)) in profile.iter().enumerate() {
            for z in [0.0, height] {
                net.push([pt(*x), pt(*y), pt(z)]);
                ws.push(profile_w[i]);
            }
        }
        let out = nurbs_patch_face::<f64>(
            &kv_u,
            &kv_v,
            &net,
            &ws,
            (0.0, 1.0, 0.0, 1.0),
            10.0,
            0.0,
            Tol::witness().get().eps,
            band,
        );
        println!("R2QUAD mult={mult}: {out:?}");
    }
}
