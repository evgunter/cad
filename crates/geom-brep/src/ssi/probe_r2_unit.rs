//! r2 review probes (in-crate).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use super::*;
use geom::{NurbsCurve2, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Vec3};

fn fold() -> (NurbsSurface<f64>, f64, f64) {
    let beta = 1e-9;
    let c = 80.0 * beta;
    let a = 0.28 * c * c / beta;
    let w = beta / c;
    let l = 1.2 * w;
    let (x0, x1) = (-1.5 * w, 1.8 * w);
    let g = |x: f64| c * x + a * x * x;
    let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)];
    let hb = [0.0, 2.0 * beta, 0.0];
    let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|i| Point3::new(xs[i / 3], ys[i % 3], gb[i / 3] + hb[i % 3]))
        .collect();
    (NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap(), x0, x1)
}

fn ivp(p: Point3<f64>, n: Vec3<f64>) -> ([Interval; 3], [Interval; 3]) {
    (
        [n.x, n.y, n.z].map(Interval::from_certified),
        [p.x, p.y, p.z].map(Interval::from_certified),
    )
}

#[test]
fn r2u_fold_single_window_reason() {
    let (wall, x0, x1) = fold();
    let boxes = NurbsBoxes::new(&wall);
    let plane = ivp(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let u0 = -x0 / (x1 - x0);
    let whole = UvRect { u: (0.0, 1.0), v: (0.0, 1.0) };
    eprintln!("PROBE fold whole-wall boundary_zeros = {:?}", boundary_zeros(&boxes, plane, whole));
    eprintln!("PROBE fold holds_zero at mid along u = {}", holds_zero(&boxes, plane, (u0, 0.5), (1.0, 0.0), whole));
}

#[test]
fn r2u_fold_two_windows_linking_is_load_bearing() {
    let (wall, x0, x1) = fold();
    let u0 = -x0 / (x1 - x0);
    let across = NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
        vec![Point2::new(u0, 0.0), Point2::new(u0, 0.5), Point2::new(u0, 1.0)],
        vec![1.0, 1.0, 1.0],
    )
    .unwrap();
    let boxes = NurbsBoxes::new(&wall);
    let plane = ivp(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let domain = UvRect { u: (0.0, 1.0), v: (0.0, 1.0) };
    for pad in [(3.0, 0.1), (3.0, 0.05), (3.0, 0.15)] {
        let ws = chart_tube_windows(&across, pad).unwrap();
        for w in &ws {
            let r = meet(w.rect, domain).unwrap();
            eprintln!("PROBE two-window pad={pad:?} window {:?} clipped {:?}: zeros {:?}", w.ends, r, boundary_zeros(&boxes, plane, r));
        }
        let probe = probe_tube_chart(&across, &wall, (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)), pad, true)
            .unwrap()
            .unwrap();
        eprintln!("PROBE two-window pad={pad:?}: margin {} windows {} one_arc {}", probe.margin, probe.windows, probe.one_arc);
    }
}

/// z = gx(x) + hy(y) biquadratic with Bernstein coefficients; domain [0,1]^2 in x,y.
fn biq(gc: [f64; 3], hc: [f64; 3]) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|i| Point3::new(0.5 * (i / 3) as f64, 0.5 * (i % 3) as f64, gc[i / 3] + hc[i % 3]))
        .collect();
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

#[test]
fn r2u_boundary_zero_edge_cases() {
    let plane = ivp(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    // phi = (x-0.5)^2 + y - 0.2 : (x-.5)^2 Bernstein [0.25, -0.25, 0.25]; y - 0.2 Bernstein [-0.2, 0.3, 0.8]
    let w = biq([0.25, -0.25, 0.25], [-0.2, 0.3, 0.8]);
    let b = NurbsBoxes::new(&w);
    for r in [
        UvRect { u: (0.0, 1.0), v: (0.2, 1.0) },   // bottom edge tangent from outside at (0.5,0.2)
        UvRect { u: (0.0, 1.0), v: (0.0, 1.0) },   // parabola crosses: left/right? phi(0,y)=0.05+y : no; bottom: (x-.5)^2-0.2 two zeros
        UvRect { u: (0.0, 1.0), v: (0.1, 1.0) },   // bottom edge cuts twice near the touch
        UvRect { u: (0.0, 1.0), v: (0.199999, 1.0) },
        UvRect { u: (0.0, 1.0), v: (0.2000001, 1.0) },
        UvRect { u: (0.5, 1.0), v: (0.0, 1.0) },   // corner-free, one crossing of bottom? phi(.5,0)=-0.2, phi(1,0)=0.05 -> 1, left edge x=.5: y-.2 ->1
        UvRect { u: (0.5, 1.0), v: (0.2, 1.0) },   // zero exactly at corner (0.5, 0.2), curve leaves the rect
    ] {
        eprintln!("PROBE edge-case parabola rect {:?}: zeros {:?}", r, boundary_zeros(&b, plane, r));
    }
    // phi = x + y - 0.5: line through corner (0.5,0) and (0,0.5)
    let w = biq([0.0, 0.25, 0.5], [-0.5, -0.25, 0.0]);
    let b = NurbsBoxes::new(&w);
    for r in [
        UvRect { u: (0.0, 0.5), v: (0.0, 1.0) },
        UvRect { u: (0.0, 0.5), v: (0.0, 0.5) },
        UvRect { u: (0.5, 1.0), v: (0.0, 1.0) },
    ] {
        eprintln!("PROBE edge-case line rect {:?}: zeros {:?}", r, boundary_zeros(&b, plane, r));
    }
}

#[test]
fn r2u_face_roots_edge_cases() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let ground = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bx = |x: (f64, f64), y: (f64, f64), z: (f64, f64)| Box3 {
        x: Interval::from_bounds(x.0, x.1),
        y: Interval::from_bounds(y.0, y.1),
        z: Interval::from_bounds(z.0, z.1),
    };
    let t = 0.6f64;
    for (tag, r) in [
        ("clean", bx((0.5, 0.9), (0.3, 0.9), (-0.1, 0.1))),
        ("graze x=1", bx((0.5, 1.0), (-0.3, 0.3), (-0.1, 0.1))),
        ("edge through root", bx((0.8, 1.2), (0.6, 1.2), (-0.1, 0.1))),
        ("root on z face?", bx((0.5, 0.9), (0.3, 0.9), (0.0, 0.1))),
        ("loop inside", bx((-2.0, 2.0), (-2.0, 2.0), (-0.1, 0.1))),
    ] {
        let _ = t;
        let roots = face_roots(&sphere, &ground, r);
        eprintln!("PROBE face_roots {tag}: {:?}", roots.map(|v| v.len()));
    }
}
