//! Review probe (flush/e-tail-review): balls against capped cylinders
//! and boxes with a cylinder wall, so a cap plane's segment may divide
//! the sphere face before the wall's segment refuses at `no_arm`. Run
//! with `REVIEW_NOARM=1` and read the `NOARM` lines on stderr.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::ball;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;

#[test]
#[ignore = "review probe"]
fn review_noarm_fuzz() {
    let tol = Tol::witness();
    for r in [0.3, 0.5, 0.8, 1.2] {
        for h in [0.4, 0.7, 1.0, 1.5] {
            for dx in [0.0, 0.2, 0.55, 0.9] {
                for dz in [0.0, 0.5, 0.9, -0.3] {
                    for tilt in [0.0, 0.3] {
                        let c = cyl(r, h);
                        let c = topo::transform_rigid(
                            &c,
                            &(Affine3::translation(Vec3::new(dx, 0.0, dz))
                                * Affine3::rotation_about_axis(
                                    Point3::origin(),
                                    Vec3::new(0.0, 1.0, 0.0),
                                    tilt,
                                )),
                            tol,
                        )
                        .unwrap();
                        let c = finished("the cylinder", c, tol);
                        let b = ball(1.0, [0.0, 0.0, 0.0]);
                        for (op, ab) in [
                            ("U", true),
                            ("U", false),
                            ("I", true),
                            ("S", true),
                            ("S", false),
                        ] {
                            let (l, rr) = if ab { (&b, &c) } else { (&c, &b) };
                            let res = match op {
                                "U" => topo::union(l, rr, tol),
                                "S" => topo::subtract(l, rr, tol),
                                _ => topo::intersect(l, rr, tol),
                            };
                            let line = match res {
                                Ok(_) => "OK".to_string(),
                                Err(e) => format!("{e:?}").chars().take(140).collect(),
                            };
                            eprintln!(
                                "POSE r={r} h={h} dx={dx} dz={dz} tilt={tilt} {op} ab={ab} => {line}"
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Domes (a sphere face and a plane disc) against cylinders, so a plane
/// of one operand cuts the other's wall before a wall×sphere segment.
#[test]
#[ignore = "review probe"]
fn review_noarm_dome_fuzz() {
    let tol = Tol::witness();
    let dome = finished("the dome", sweep::test_support::dome(1.0, tol), tol);
    for r in [0.3, 0.5, 0.8] {
        for h in [0.4, 0.7, 1.5] {
            for dx in [0.0, 0.2, 0.55, 0.9] {
                for dy in [-0.6, -0.2, 0.0, 0.3, 0.8] {
                    for (ax, tilt) in [
                        ((0.0, 0.0, 1.0), 0.0),
                        ((1.0, 0.0, 0.0), 0.3),
                        ((1.0, 0.0, 0.0), 1.5708),
                        ((0.0, 0.0, 1.0), 0.7),
                    ] {
                        let c = cyl(r, h);
                        let c = topo::transform_rigid(
                            &c,
                            &(Affine3::translation(Vec3::new(dx, dy, 0.0))
                                * Affine3::rotation_about_axis(
                                    Point3::origin(),
                                    Vec3::new(ax.0, ax.1, ax.2),
                                    tilt,
                                )),
                            tol,
                        )
                        .unwrap();
                        let c = finished("the cylinder", c, tol);
                        for (op, ab) in [
                            ("U", true),
                            ("U", false),
                            ("I", true),
                            ("S", true),
                            ("S", false),
                        ] {
                            let (l, rr) = if ab { (&dome, &c) } else { (&c, &dome) };
                            let res = match op {
                                "U" => topo::union(l, rr, tol),
                                "S" => topo::subtract(l, rr, tol),
                                _ => topo::intersect(l, rr, tol),
                            };
                            let line = match res {
                                Ok(_) => "OK".to_string(),
                                Err(e) => format!("{e:?}").chars().take(140).collect(),
                            };
                            eprintln!(
                                "POSE r={r} h={h} dx={dx} dy={dy} tilt={tilt} ax={ax:?} {op} ab={ab} => {line}"
                            );
                        }
                    }
                }
            }
        }
    }
}
