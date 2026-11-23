//! Review probe (C6, C7).
//! (C6) the row `an_interval_window_reads_a_part_it_has_not_decided_empty`'s
//! pose on an ellipse held at its CENTRE (the same locus, `Ellipse::new` at
//! the derived centre) beside the arc-held one, at `Interval`: is the
//! skipped part the placement class, and is it on the base.
//! (C7) pairs in ONE plane at `Interval`, where `Over` reads
//! `Height::extremes` along the conic's own normal.
//! Run on both trees: `CAD_TOLERANCE_EPS=<eps> cargo run -p geom-brep
//! --features test-support --example probe_window`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use geom::{Conic, Curve3, Ellipse, Frame};
use geom_brep::curve_pair::{Span, meet};
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, Vec3};

fn say<T: geom_core::Decide>(r: &Result<geom_brep::curve_pair::Meetings<T>, geom_brep::curve_pair::MeetRefusal<T>>) -> String {
    match r {
        Ok(m) => format!("Ok, {} meeting(s), shared {}", m.points.len(), m.shared.is_some()),
        Err(e) => {
            let s = format!("{:?}", e.fault);
            format!("refused: {}", &s[..s.len().min(110)])
        }
    }
}

fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let i = <Interval as Real>::from_f64;
    let anchor = Point3::new(1.3, -0.7, 0.4);
    let axis = Vec3::new(0.5522240050794545, 0.3110026418424273, 0.7735153553621528);
    let toward = Vec3::new(0.6816514125372342, -0.702621773621506, -0.20414209516648582);
    println!("== C6: a unit circle across the ellipse through its point, at Interval (truth: they meet)");
    for (a, theta) in [(1e4, 0.5), (1e6, 2.0), (1e2, 0.5), (1e2, 2.0), (10.0, 0.5), (10.0, 2.0)] {
        let arc = Ellipse::anchored(anchor, axis, toward, a, 0.3 * a);
        let centre = Ellipse::new(Frame::new(Conic::center(&arc), arc.axis(), arc.u_ref()), a, 0.3 * a);
        for (name, e) in [("arc-held", arc), ("centre-held", centre)] {
            let p = e.point_at(theta);
            let tan = e.derivative_at(theta).normalize();
            let across = e.axis().cross(tan).normalize();
            let w = (across * 0.7_f64.cos() + e.axis() * 0.7_f64.sin()).normalize();
            let in_plane = tan.cross(w.cross(tan)).normalize();
            let circle = Curve3::circle(p - in_plane, tan, 1.0, in_plane);
            let carrier = Curve3::Ellipse(e);
            let f = meet(&Span::new(&carrier, (theta - 0.1, theta + 0.1)), &Span::new(&circle, (-1.0, 1.0)), band);
            let (ci, ki) = (circle.map_scalar(i), carrier.map_scalar(i));
            let g = meet(&Span::new(&ki, (i(theta - 0.1), i(theta + 0.1))), &Span::new(&ci, (i(-1.0), i(1.0))), band);
            println!("a={a:e} theta={theta} {name}: f64 {} | Interval {}", say(&f), say(&g));
        }
    }
    println!("== C7: pairs in one plane at Interval (0.3 m of arc from the anchor)");
    for a in [1e2, 1e6, 1e9] {
        let e = Ellipse::anchored(anchor, axis, toward, a, 0.3 * a);
        let theta = 0.3 / (0.3 * a);
        let (p, tan, n) = (e.point_at(theta), e.derivative_at(theta).normalize(), e.axis());
        let half = 10.0 / (0.3 * a);
        let carrier = Curve3::Ellipse(e);
        let ki = carrier.map_scalar(i);
        // the extremes of the height along the conic's own normal
        let hi_ = ki.conic().unwrap().height_along(
            Point3::new(i(p.x), i(p.y), i(p.z)),
            Vec3::new(i(n.x), i(n.y), i(n.z)),
        );
        let (lo, hi) = hi_.extremes();
        println!("a={a:e}: Height along its own axis at Interval: extremes lo [{:e},{:e}] hi [{:e},{:e}]", lo.lo(), lo.hi(), hi.lo(), hi.hi());
        for lift in [0.0, 30.0 * band.zero(), 1000.0 * band.zero()] {
            let circle = Curve3::circle(p + tan * 0.6 + n * lift, n, 1.0, -tan);
            let small = Curve3::Ellipse(Ellipse::new(Frame::new(p + tan * 1.2 + n * lift, n, -tan), 2.0, 1.0));
            for (name, other) in [("circle", circle), ("small ellipse", small)] {
                let f = meet(&Span::new(&other, (-1.0, 1.0)), &Span::new(&carrier, (theta - half, theta + half)), band);
                let oi = other.map_scalar(i);
                let g = meet(&Span::new(&oi, (i(-1.0), i(1.0))), &Span::new(&ki, (i(theta - half), i(theta + half))), band);
                println!("   {name} in the plane lifted {lift:e}: f64 {} | Interval {}", say(&f), say(&g));
            }
        }
    }
}
