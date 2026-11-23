//! DELTA-REVIEW probe (D4): `Height::roots` at exact touches: at the anchor
//! from both sides, at the antipode, a quarter turn either way, on an
//! arc-held ellipse (seam at the anchor and turned), the same locus held at
//! its centre, and a circle; at `f64` and `Interval`. Each line prints the
//! roots, the height there (truth: 0) and the true touch parameter.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use geom::{Circle, Conic, Ellipse, Frame};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn lift(p: Point3<f64>) -> Point3<Interval> {
    let i = <Interval as Real>::from_f64;
    Point3::new(i(p.x), i(p.y), i(p.z))
}
fn liftv(p: Vec3<f64>) -> Vec3<Interval> {
    let i = <Interval as Real>::from_f64;
    Vec3::new(i(p.x), i(p.y), i(p.z))
}

fn show(name: &str, k: &dyn Conic<f64>, ki: &dyn Conic<Interval>, at: f64, flip: f64) {
    // the tangent plane at the conic's point `at`, normal along the outward
    // radial direction (flip = 1) or inward (flip = -1): the height touches 0 at `at` only.
    let p = k.point_at(at);
    let tan = k.derivative_at(at).normalize();
    let n = tan.cross(k.axis()).normalize() * flip;
    let h = k.height_along(p, n);
    let r = h.roots(0.0);
    let tau = core::f64::consts::TAU;
    let off = |x: f64| { let d = (x - at).rem_euclid(tau); d.min(tau - d) };
    let hi_ = ki.height_along(lift(p), liftv(n));
    let ri = hi_.roots(<Interval as Real>::from_f64(0.0));
    let holds = |x: Interval| {
        // does the enclosure hold the touch parameter, mod a turn
        (-2..=2).any(|k| x.lo() <= at + f64::from(k) * tau + 1e-7 && at + f64::from(k) * tau - 1e-7 <= x.hi())
    };
    let bad = off(r[0]).max(off(r[1])) > 1e-6 || !(holds(ri[0]) && holds(ri[1]));
    println!(
        "{}{name} touch at {at:+.4} side {flip:+}: f64 roots off the touch by ({:.2e}, {:.2e}) rad, H there ({:.2e}, {:.2e}); Interval [{:.4},{:.4}] [{:.4},{:.4}]",
        if bad { "BAD  " } else { "ok   " },
        off(r[0]), off(r[1]), h.at(r[0]), h.at(r[1]), ri[0].lo(), ri[0].hi(), ri[1].lo(), ri[1].hi()
    );
}

fn main() {
    let i = <Interval as Real>::from_f64;
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let skew_n = Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
    let skew_u = Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582);
    let pi = core::f64::consts::PI;
    for (fname, n, u) in [("axis-aligned", z, x), ("skew", skew_n, skew_u)] {
        for (a, b) in [(5.0, 3.0), (1e6, 3e5)] {
            let anchor = Point3::new(1.0, 2.0, 3.0);
            let arc = Ellipse::anchored(anchor, n, u, a, b);
            let turned = arc.seamed_near(core::f64::consts::FRAC_PI_2);
            let centre = Ellipse::new(Frame::new(Conic::center(&arc), arc.axis(), arc.u_ref()), a, b);
            let circle = Circle::new(Frame::new(anchor, n, u), a);
            println!("== {fname} a={a:e} b={b:e}");
            for at in [0.0, pi, 0.5 * pi, -0.5 * pi, 1.0] {
                for flip in [1.0, -1.0] {
                    show("arc-held ", &arc, &arc.map_scalar(|f| f.map_scalar(i), i), at, flip);
                    show("re-seamed", &turned, &turned.map_scalar(|f| f.map_scalar(i), i), at, flip);
                    show("centre   ", &centre, &centre.map_scalar(|f| f.map_scalar(i), i), at, flip);
                    show("circle   ", &circle, &circle.map_scalar(|f| f.map_scalar(i), i), at, flip);
                }
            }
        }
    }
    let _ = Bounds::lo(i(0.0));
}
