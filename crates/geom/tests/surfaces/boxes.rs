//! Containment pin for the certified-conservative NURBS surface box
//! (`geom::surfaces::boxes`): sampled surface points lie inside the
//! control-net hull box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::surfaces::boxes::nurbs_surface_aabb;
use geom::surfaces::nurbs::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::KnotVector;

#[test]
fn surface_box_is_the_control_net_hull_and_contains_samples() {
    // A 3×3 biquadratic patch (row-major control net).
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            control.push(Point3::new(
                f64::from(i),
                f64::from(j),
                0.25 * f64::from(i * j) - 0.5,
            ));
        }
    }
    let weights = vec![1.0, 0.5, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0];
    let surf = NurbsSurface::new(ku, kv, control, weights).unwrap();
    let b = nurbs_surface_aabb(&surf);
    assert_eq!((b.min_x, b.max_x), (0.0, 2.0));
    assert_eq!((b.min_y, b.max_y), (0.0, 2.0));
    assert_eq!((b.min_z, b.max_z), (-0.5, 0.5));
    for i in 0..=8u32 {
        for j in 0..=8u32 {
            let (u, v) = (f64::from(i) / 8.0, f64::from(j) / 8.0);
            let p = surf.eval(u, v);
            assert!(
                p.x >= b.min_x - 1e-12
                    && p.x <= b.max_x + 1e-12
                    && p.y >= b.min_y - 1e-12
                    && p.y <= b.max_y + 1e-12
                    && p.z >= b.min_z - 1e-12
                    && p.z <= b.max_z + 1e-12,
                "sample ({u}, {v}) escaped the hull box"
            );
        }
    }
}
