//! DELTA-REVIEW probe (D5): for each pose of `arc_held_rows::poses()`, how
//! far the centre-held twin's point stands from the arc-held one's
//! (`Near::held_off`), against the row's gauge `0.1·ε` at ε 1e-6, 1e-9, 1e-12:
//! where it is past the gauge, "an answer is owed" is not enforced.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use geom::{Conic, Ellipse, Frame};
use geom_core::{Point3, Vec3};
fn main() {
    let axis = Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
    let toward = Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582);
    for a in [1e6_f64, 1e9] {
        for b in [0.3 * a, 1.04 * a.sqrt()] {
            for s in [0.3, 5.0] {
                let e = Ellipse::anchored(Point3::new(1.3, -0.7, 0.4), axis, toward, a, b);
                let theta = s / b;
                let held = Ellipse::new(Frame::new(e.center(), e.axis(), e.u_ref()), a, b);
                let off = (held.point_at(theta) - e.point_at(theta)).norm();
                println!(
                    "a={a:e} b={b:.3e} s={s}: held_off {off:.2e}; gauged at 1e-6: {}, 1e-9: {}, 1e-12: {}",
                    off <= 1e-7, off <= 1e-10, off <= 1e-13
                );
            }
        }
    }
}
