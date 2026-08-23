//! **Solid-door bit-identity fingerprint** (review probe): builds the
//! SOLID tube door's two shapes (the `m6_tube` wedge window and the
//! full torus) and folds every stored surface parameter, vertex point
//! and entity count into one FNV-64 fingerprint, printed with
//! `--nocapture`. Run at the merge base and at the head: equal
//! fingerprints are a REAL-OUTPUT verification that the hollow-door
//! elaboration left the solid door's bodies bit-identical — red the
//! moment any stored bit or count moves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point3, Tol, Vec3};
use sweep::{TubeWindow, tube_along_arc};

fn fnv(h: &mut u64, x: u64) {
    *h ^= x;
    *h = h.wrapping_mul(0x0000_0100_0000_01B3);
}

fn fold3(h: &mut u64, p: (f64, f64, f64)) {
    fnv(h, p.0.to_bits());
    fnv(h, p.1.to_bits());
    fnv(h, p.2.to_bits());
}

#[test]
fn solid_door_fingerprint() {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for window in [TubeWindow::Arc { t0: 0.25, t1: 1.75 }, TubeWindow::Full] {
        let t = tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
            Vec3::unit_x(),
            2.0,
            window,
            0.5,
            Tol::witness(),
        )
        .expect("the solid tube builds");
        let b = &t.body;
        for n in [
            b.solids().count(),
            b.shells().count(),
            b.faces().count(),
            b.edges().count(),
            b.vertices().count(),
        ] {
            fnv(&mut h, n as u64);
        }
        let mut surf: Vec<Vec<u64>> = Vec::new();
        for (_, face) in b.faces() {
            let mut row = Vec::new();
            match b.get_surface(face.surface) {
                Some(Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    u_ref,
                }) => {
                    row.push(1u64);
                    for v in [
                        center.x,
                        center.y,
                        center.z,
                        axis.x,
                        axis.y,
                        axis.z,
                        *major_radius,
                        *minor_radius,
                        u_ref.x,
                        u_ref.y,
                        u_ref.z,
                    ] {
                        row.push(v.to_bits());
                    }
                }
                Some(Surface::Plane {
                    origin,
                    normal,
                    u_ref,
                }) => {
                    row.push(2u64);
                    for v in [
                        origin.x, origin.y, origin.z, normal.x, normal.y, normal.z, u_ref.x,
                        u_ref.y, u_ref.z,
                    ] {
                        row.push(v.to_bits());
                    }
                }
                other => panic!("unexpected surface on the solid tube: {other:?}"),
            }
            surf.push(row);
        }
        surf.sort_unstable();
        for row in surf {
            for x in row {
                fnv(&mut h, x);
            }
        }
        let mut pts: Vec<(u64, u64, u64)> = b
            .vertices()
            .map(|(_, v)| {
                let p = b.get_point(v.point).expect("vertex point");
                (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
            })
            .collect();
        pts.sort_unstable();
        for p in pts {
            fold3(
                &mut h,
                (
                    f64::from_bits(p.0),
                    f64::from_bits(p.1),
                    f64::from_bits(p.2),
                ),
            );
        }
    }
    println!("SOLID-DOOR-FINGERPRINT {h:#018x}");
}
