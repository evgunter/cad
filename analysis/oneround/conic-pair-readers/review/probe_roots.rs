//! Review probe (C1b): `Height::roots` on an arc-held ellipse at the
//! exact touch at its anchor, both senses of the normal, and near it.
//! Run by dropping into `crates/geom-brep/examples/`:
//! `cargo run -p geom-brep --features test-support --example probe_roots`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use geom::{Conic, Curve3, Ellipse, Frame};
use geom_brep::curve_face::plane_crossing::plane_crossing_lane;
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let anchor = Point3::new(1.0, 2.0, 3.0);
    for (a, b) in [(5.0, 3.0), (1e6, 3e5)] {
        let arc = Ellipse::anchored(anchor, z, x, a, b);
        // the same ellipse held at its centre
        let centre = Ellipse::new(Frame::new(anchor - x * a, z, x), a, b);
        for (name, e) in [("arc-held", arc), ("centre-held", centre)] {
            for (sense, n) in [("+u", x), ("-u", x * -1.0)] {
                // The tangent plane at the vertex: the height along n from the
                // anchor touches zero AT the vertex (theta = 0) and nowhere else.
                let h = e.height_along(anchor, n);
                let roots = h.roots(0.0);
                let (lo, hi) = h.extremes();
                println!(
                    "a={a:e} {name} n={sense}: extremes ({lo:e},{hi:e}) roots {roots:?} (truth: 0, 0) H(roots)=({:e},{:e})",
                    h.at(roots[0]),
                    h.at(roots[1])
                );
                let hi_ = e.map_scalar(|f| f.map_scalar(Interval::from_f64), Interval::from_f64);
                let hh = hi_.height_along(
                    Point3::new(Interval::from_f64(anchor.x), Interval::from_f64(anchor.y), Interval::from_f64(anchor.z)),
                    Vec3::new(Interval::from_f64(n.x), Interval::from_f64(n.y), Interval::from_f64(n.z)),
                );
                println!("    interval roots {:?}", hh.roots(Interval::from_f64(0.0)));
                // a level a hair inside: two roots straddling the vertex
                for lvl in [1e-9_f64, 1e-12] {
                    let inside = if sense == "+u" { -lvl } else { lvl };
                    let r = h.roots(inside);
                    let want = (2.0 * lvl / a).sqrt(); // a(1-cos t)=lvl
                    println!("    level {inside:e}: roots {r:?} (truth ±{want:e})");
                }
                let c = Curve3::Ellipse(e);
                println!(
                    "    plane door, tangent plane at the vertex over (-0.5,0.5): {:?}",
                    plane_crossing_lane(&c, -0.5, 0.5, anchor, n, band)
                );
            }
        }
    }
}
