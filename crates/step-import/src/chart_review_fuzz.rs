//! The chart recogniser's randomized inverse-of-eval sweep, in a file of
//! its own so the gate can skip IT without skipping `chart`'s deterministic
//! rows. A `test_utils::gated_to!` marker gates a whole file's module, so a
//! fuzz row sharing a file with pinned tests drags them along; splitting the
//! file is how the granularity is bought.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/step-import/src/chart.rs",
    "crates/step-import/src/recognize.rs",
    "crates/step-import/src/normalize.rs",
    "crates/geom/src/surfaces/",
    "crates/geom/src/surfaces.rs",
];

use crate::chart::uv_of;
use geom::Surface;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

fn frame(s: &mut fuzz::Rng) -> (Vec3<f64>, Vec3<f64>) {
    let a = Vec3::new(s.unit() - 0.5, s.unit() - 0.5, s.unit() - 0.5).normalize();
    let h = if a.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = a.cross(h).normalize();
    (a, u)
}

/// Fuzz inverse-of-eval over all five kinds, tilted frames,
/// near-seam and near-pole parameters included.
#[test]
fn review_fuzz_roundtrip() {
    let mut s = fuzz::start("chart::review_fuzz_roundtrip");
    let pi = core::f64::consts::PI;
    let mut worst: f64 = 0.0;
    for trial in 0..fuzz::scaled(50) {
        let (axis, u_ref) = frame(&mut s);
        let origin = Point3::new(
            s.unit() * 4.0 - 2.0,
            s.unit() * 4.0 - 2.0,
            s.unit() * 4.0 - 2.0,
        );
        let r = 0.2 + s.unit() * 3.0;
        let kinds: [Surface<f64>; 5] = [
            Surface::Plane {
                origin,
                normal: axis,
                u_ref,
            },
            Surface::Cylinder {
                origin,
                axis,
                radius: r,
                u_ref,
            },
            Surface::Cone {
                apex: origin,
                axis,
                half_angle: 0.05 + s.unit() * 1.4,
                u_ref,
            },
            Surface::Sphere {
                center: origin,
                radius: r,
                axis,
                u_ref,
            },
            Surface::Torus {
                center: origin,
                axis,
                major_radius: r + 1.0,
                minor_radius: r * 0.3,
                u_ref,
            },
        ];
        for (k, surf) in kinds.iter().enumerate() {
            // Parameter menu: generic, near-seam, near-pole/apex.
            let us = [s.unit() * 2.0 * pi - pi, pi - 1e-9, -pi + 1e-9];
            let vs = match k {
                1 => [s.unit() * 4.0 - 2.0, 1e-9, -1e-9],
                2 => [0.1 + s.unit() * 2.0, 1e-6, 3.0], // cone: v>0 (import never sees mirror nappe from eval side; also test v<0)
                3 => [
                    s.unit() * 3.0 - 1.5,
                    core::f64::consts::FRAC_PI_2 - 1e-7,
                    -core::f64::consts::FRAC_PI_2 + 1e-7,
                ],
                4 => [s.unit() * 2.0 * pi - pi, pi - 1e-9, 1e-9],
                _ => [s.unit() * 4.0 - 2.0, 0.0, 1.0],
            };
            for &u in &us {
                for &v in &vs {
                    let p = surf.eval(u, v);
                    let Some(uv) = uv_of(surf, p) else {
                        // Only poles/apex may answer None.
                        assert!(
                            matches!(surf, Surface::Sphere { .. } | Surface::Cone { .. }),
                            "t{trial} k{k} ({u},{v}): unexpected None — {}",
                            fuzz::replay()
                        );
                        continue;
                    };
                    let back = surf.eval(uv.x, uv.y);
                    let err = (back - p).norm();
                    worst = worst.max(err / (1.0 + (p - origin).norm()));
                    assert!(
                        err <= 1e-9 * (1.0 + (p - origin).norm()),
                        "t{trial} k{k} ({u},{v}) -> uv({},{}) err {err:e} — {}",
                        uv.x,
                        uv.y,
                        fuzz::replay()
                    );
                }
            }
        }
    }
    println!("review_fuzz worst rel err: {worst:e}");
    // Cone mirror nappe: eval at v<0 must round-trip too.
    let (axis, u_ref) = frame(&mut s);
    let cone = Surface::Cone {
        apex: Point3::new(0.3, -0.2, 0.9),
        axis,
        half_angle: 0.6,
        u_ref,
    };
    let nappe = fuzz::scaled(25);
    for i in 0..nappe {
        let u = ((i as f64) / (nappe as f64)) * 2.0 * pi - pi;
        let v = -(0.01 + s.unit() * 2.0);
        let p = cone.eval(u, v);
        let uv = uv_of(&cone, p).expect("mirror nappe preimage");
        let back = cone.eval(uv.x, uv.y);
        assert!(
            (back - p).norm() <= 1e-9,
            "nappe u={u} v={v}: {:?} — {}",
            (back - p).norm(),
            fuzz::replay()
        );
    }
}
