//! Reviewer probes for PR 5696: the rational rim lane's enclosure
//! against an independent high-precision reference (scripts in the
//! review branch). Prints one JSON line per case.

use geom::NurbsCurve3;
use geom_brep::props::rational_rim::{rational_edge_q, rational_vector_area_split};
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Point3};
use geom_core::interval::certification::Certification;

struct Case {
    name: &'static str,
    knots: Vec<f64>,
    degree: usize,
    pts: Vec<[f64; 3]>,
    w: Vec<f64>,
    reference: [f64; 3],
    t: (f64, f64),
    widen: f64,
}

fn build(c: &Case) -> NurbsCurve3<Interval> {
    let iv = |x: f64| Interval::from_bounds(x - c.widen, x + c.widen);
    NurbsCurve3::new(
        geom_core::spline::Knots::from(KnotVector::clamped(c.knots.clone(), c.degree).unwrap())
            .lift(geom_core::Real::from_f64),
        c.pts.iter().map(|p| Point3::new(iv(p[0]), iv(p[1]), iv(p[2]))).collect(),
        c.w.iter().map(|&x| Interval::point(x)).collect(),
    )
    .unwrap()
}

fn cases() -> Vec<Case> {
    let q = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let base = |name, w: Vec<f64>| Case {
        name,
        knots: q.clone(),
        degree: 2,
        pts: vec![[1.0, 0.0, 0.0], [1.5, 1.2, 0.0], [-0.2, 1.0, 0.0]],
        w,
        reference: [1.0, 0.0, 0.0],
        t: (0.0, 1.0),
        widen: 0.0,
    };
    vec![
        Case { pts: vec![[1.02925, 0.0, 0.0], [1.02925, 1.02925, 0.0], [0.0, 1.02925, 0.0]], reference: [1.02925, 0.0, 0.0], ..base("quarter", vec![1.0, s2, 1.0]) },
        base("ellipse", vec![1.0, 0.6, 1.3]),
        base("parabola k=1", vec![1.0, 1.3f64.sqrt(), 1.3]),
        base("hyperbola", vec![1.0, 1.8, 1.3]),
        base("hyper w1=20", vec![1.0, 20.0, 1.3]),
        base("hyper w1=1e3", vec![1.0, 1e3, 1.3]),
        base("w1=1e-3", vec![1.0, 1e-3, 1.3]),
        base("w ends small", vec![1e-3, 1.0, 1e-3]),
        base("w ends 1e-6", vec![1e-6, 1.0, 1.0]),
        base("big weights", vec![1e6, 3e6, 2e6]),
        Case { reference: [1e6, -1e6, 0.0], ..base("far reference", vec![1.0, 0.6, 1.3]) },
        Case { reference: [1e3, 0.0, 0.0], ..base("far reference 1e3", vec![1.0, 1.8, 1.3]) },
        Case { reference: [0.3, 0.1, 0.5], pts: vec![[1.0, 0.0, 0.0], [1.5, 1.2, 0.7], [-0.2, 1.0, -0.4]], ..base("3d", vec![1.0, 1.8, 1.3]) },
        Case { pts: vec![[1.0, 0.0, 0.0], [1.0 + 1e-9, 1e-9, 0.0], [1.0, 2e-9, 0.0]], ..base("tiny rim", vec![1.0, 2.0, 1.0]) },
        Case { pts: vec![[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [-0.2, 1.0, 0.0]], ..base("doubled control (B' small)", vec![1.0, 3.0, 1.0]) },
        Case { t: (0.123, 0.777), ..base("sub-interval hyper", vec![1.0, 1.8, 1.3]) },
        Case { widen: 1e-9, ..base("interval-built", vec![1.0, 1.8, 1.3]) },
        Case {
            name: "cubic two spans",
            knots: vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0],
            degree: 3,
            pts: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, -1.0, 0.0], [3.0, 1.0, 0.0], [2.5, 2.5, 0.0]],
            w: vec![1.0, 2.0, 0.5, 1.0, 1.5],
            reference: [-1.0, 0.5, 0.0],
            t: (0.1, 0.95),
            widen: 0.0,
        },
        Case {
            name: "quintic, double knot, wild weights",
            knots: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            degree: 5,
            pts: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.1], [2.0, -1.0, 0.0], [3.0, 1.0, -0.2], [2.5, 2.5, 0.0], [1.0, 3.0, 0.0], [0.0, 2.0, 0.3], [-1.0, 1.0, 0.0], [-0.5, 0.0, 0.0]],
            w: vec![1.0, 50.0, 0.05, 3.0, 0.2, 10.0, 0.5, 7.0, 1.0],
            reference: [0.2, 0.3, 0.0],
            t: (0.0, 1.0),
            widen: 0.0,
        },
        Case {
            name: "degree 7 one span",
            knots: vec![0.0; 8].into_iter().chain(vec![2.0; 8]).collect(),
            degree: 7,
            pts: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.0], [2.0, -1.0, 0.0], [3.0, 1.0, 0.0], [2.5, 2.5, 0.0], [1.0, 3.0, 0.0], [0.0, 2.0, 0.0], [-1.0, 1.0, 0.0]],
            w: vec![1.0, 9.0, 0.1, 4.0, 0.3, 6.0, 0.2, 1.0],
            reference: [0.0, 0.0, 0.0],
            t: (0.0, 2.0),
            widen: 0.0,
        },
        Case {
            name: "linear rational",
            knots: vec![0.0, 0.0, 0.5, 1.0, 1.0],
            degree: 1,
            pts: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            w: vec![1.0, 5.0, 0.2],
            reference: [0.3, -0.4, 0.0],
            t: (0.0, 1.0),
            widen: 0.0,
        },
    ]
}

#[test]
fn print_enclosures() {
    for c in cases() {
        let curve = build(&c);
        let r = c.reference.map(Interval::point);
        let e = rational_edge_q(&curve, Interval::point(c.t.0), Interval::point(c.t.1), true, Point3::new(r[0], r[1], r[2]))
            .unwrap();
        for pieces in [1usize, 4, 16, 64, 256] {
            let (va, rem) = rational_vector_area_split(&e, pieces);
            println!(
                "{{\"name\":\"{}\",\"pieces\":{},\"knots\":{:?},\"degree\":{},\"pts\":{:?},\"w\":{:?},\"ref\":{:?},\"t\":[{:e},{:e}],\"va\":[{}],\"rem\":{:?}}}",
                c.name, pieces, c.knots, c.degree, c.pts, c.w, c.reference, c.t.0, c.t.1,
                va.iter().map(|x| format!("[{:e},{:e}]", x.lo(), x.hi())).collect::<Vec<_>>().join(","),
                rem
            );
        }
    }
}

/// Where the high-degree / small-end-weight carriers first give a
/// finite enclosure, against the schedule's reach (16·2^r pieces).
#[test]
fn liveness_by_pieces() {
    for c in cases() {
        if !["quintic, double knot, wild weights", "degree 7 one span", "w ends small", "w ends 1e-6", "hyper w1=1e3"].contains(&c.name) {
            continue;
        }
        let curve = build(&c);
        let r = c.reference.map(Interval::point);
        let e = rational_edge_q(&curve, Interval::point(c.t.0), Interval::point(c.t.1), true, Point3::new(r[0], r[1], r[2])).unwrap();
        for k in 4..=17 {
            let pieces = 1usize << k;
            let t = std::time::Instant::now();
            let (va, _) = rational_vector_area_split(&e, pieces);
            println!("LIVE {} pieces=2^{k} z=[{:e},{:e}] width={:e} {:?}", c.name, va[2].lo(), va[2].hi(), va[2].width(), t.elapsed());
        }
    }
}
