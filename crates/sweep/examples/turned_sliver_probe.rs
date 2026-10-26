//! **Does a rigid turn keep a section-ellipse sliver valid at rest?** A
//! measurement probe for
//! `work/cleave/a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out.md`.
//!
//! The unit cylinder split by a plane `ψ` off the wall's tangent plane at
//! azimuth `φ` (an ellipse of semi-major axis `1/sin ψ`); each side that
//! passes tier 3 is turned rigidly about four axes through
//! `(0.3, −0.2, 0.5)`, seven angles each, and validated again. Prints each
//! turn tier 3 refuses (`FLIP …`) and the count.
//!
//! `cargo run -p sweep --release --example turned_sliver_probe`
//! (`CAD_TOLERANCE_EPS` picks the row).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::split;
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

fn main() {
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cyl = AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol()), tol()).unwrap();
    let axes = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(-0.3, 0.7, 0.2),
    ];
    let angles = [0.1, 0.7, 1.3, 2.2, 3.0, -1.1, 5.9];
    let (mut total, mut flips) = (0, 0);
    for psi in [1e-2_f64, 1e-3, 1e-4, 1e-5, 3e-6] {
        for (phi, l) in [(0.0_f64, 0.0_f64), (0.4, 0.3), (2.2, 0.1)] {
            // The plane through the seam-side point at azimuth `phi`,
            // tilted `psi` off the wall's tangent plane: an ellipse of
            // semi-major axis 1/sin ψ.
            let (c, s) = (phi.cos(), phi.sin());
            let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin());
            let at = Point3::new(c, s, 0.0) + Vec3::new(-s, c, 0.0) * l;
            let plane = topo::test_support::split_plane(at, n, tol());
            let Ok(r) = split(&cyl, &plane, tol()) else {
                println!("psi {psi:e} phi {phi}: split refused");
                continue;
            };
            for (word, part) in [("above", r.above.body()), ("below", r.below.body())] {
                let Some(b) = part else { continue };
                // The split's side is finished: its gate kept the verdict.
                let fin: AtRestBody<f64> = b.clone();
                for axis in axes {
                    let axis = axis / axis.norm();
                    for a in angles {
                        let turn =
                            Affine3::rotation_about_axis(Point3::new(0.3, -0.2, 0.5), axis, a);
                        let moved: Body<f64> = topo::transform_rigid(
                            &fin,
                            &geom_core::test_support::rigid(&turn, tol()),
                            tol(),
                        )
                        .unwrap();
                        total += 1;
                        if let Err(e) = topo::validate_geometric(&moved, tol()) {
                            flips += 1;
                            println!(
                                "FLIP psi {psi:e} phi {phi} L {l} {word} axis {axis:?} a {a}: {:?}",
                                e.first()
                            );
                        }
                    }
                }
            }
        }
    }
    println!(
        "eps {:e}: {flips} flips of {total} turned valid sides",
        tol().eps()
    );
}
