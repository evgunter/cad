//! Review probes for PR 5696: the certified rational-rim rule against an
//! independent 40-digit mpmath reference (scripts in the review branch).
#![allow(clippy::unwrap_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_brep::props::rational_rim::{rational_edge_q, rational_vector_area_split};
use geom_core::Bounds;
use geom_core::Point3;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use std::io::Write;

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }
}

struct Case {
    name: String,
    deg: usize,
    knots: Vec<f64>,
    pts: Vec<[f64; 3]>,
    w: Vec<f64>,
    t0: f64,
    t1: f64,
    reference: [f64; 3],
}

fn run(c: &Case, out: &mut impl Write) {
    let kv = KnotVector::clamped(c.knots.clone(), c.deg).unwrap();
    let f = NurbsCurve3::new(
        kv,
        c.pts.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
        c.w.clone(),
    )
    .unwrap();
    let iv = f.map_scalar(Interval::point);
    let r = Point3::new(
        Interval::point(c.reference[0]),
        Interval::point(c.reference[1]),
        Interval::point(c.reference[2]),
    );
    let e = match rational_edge_q(&iv, Interval::point(c.t0), Interval::point(c.t1), true, r) {
        Ok(e) => e,
        Err(err) => {
            writeln!(out, "{{\"name\":\"{}\",\"refused\":\"{err:?}\"}}", c.name).unwrap();
            return;
        }
    };
    for pieces in [16usize, 64, 256] {
        let (va, rem) = rational_vector_area_split(&e, pieces);
        let cert = va.iter().all(|x| x.is_certified());
        let bounds: Vec<String> = va
            .iter()
            .map(|x| format!("[{:e},{:e}]", x.lo(), x.hi()))
            .collect();
        writeln!(
            out,
            "{{\"name\":\"{}\",\"pieces\":{pieces},\"cert\":{cert},\"va\":[{}],\"rem\":[{:e},{:e},{:e}],\"deg\":{},\"knots\":{:?},\"pts\":{:?},\"w\":{:?},\"t0\":{:e},\"t1\":{:e},\"ref\":{:?}}}",
            c.name,
            bounds.join(","),
            rem[0], rem[1], rem[2],
            c.deg,
            c.knots,
            c.pts,
            c.w,
            c.t0,
            c.t1,
            c.reference
        )
        .unwrap();
    }
}

fn clamped(deg: usize, interior: &[f64]) -> Vec<f64> {
    let mut k = vec![0.0; deg + 1];
    k.extend_from_slice(interior);
    k.extend(vec![1.0; deg + 1]);
    k
}

fn cases() -> Vec<Case> {
    let mut g = Lcg(0x5696);
    let mut v = Vec::new();
    let h = core::f64::consts::FRAC_1_SQRT_2;
    // A full circle of radius 2 as four rational quadratics, interior
    // multiplicity 2 (= p): no insertion.
    v.push(Case {
        name: "full circle, mult p".into(),
        deg: 2,
        knots: vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0],
        pts: vec![
            [2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [0.0, 2.0, 0.0], [-2.0, 2.0, 0.0],
            [-2.0, 0.0, 0.0], [-2.0, -2.0, 0.0], [0.0, -2.0, 0.0], [2.0, -2.0, 0.0],
            [2.0, 0.0, 0.0],
        ],
        w: vec![1.0, h, 1.0, h, 1.0, h, 1.0, h, 1.0],
        t0: 0.0,
        t1: 1.0,
        reference: [0.3, -0.7, 0.0],
    });
    // Extreme weights, far reference, near-degenerate spans.
    for (name, w1) in [
        ("tiny middle weight", 1e-9),
        ("huge middle weight", 1e9),
        ("hyperbolic k=50", 50.0),
        ("parabolic k=1", 1.0),
    ] {
        v.push(Case {
            name: name.into(),
            deg: 2,
            knots: clamped(2, &[]),
            pts: vec![[1.0, 0.0, 0.2], [1.5, 1.2, -0.3], [-0.2, 1.0, 0.1]],
            w: vec![1.0, w1, 1.0],
            t0: 0.0,
            t1: 1.0,
            reference: [0.0, 0.0, 0.0],
        });
    }
    v.push(Case {
        name: "tiny end weights".into(),
        deg: 2,
        knots: clamped(2, &[]),
        pts: vec![[1.0, 0.0, 0.0], [1.5, 1.2, 0.0], [-0.2, 1.0, 0.0]],
        w: vec![1e-6, 1.0, 1e-6],
        t0: 0.0,
        t1: 1.0,
        reference: [0.0, 0.0, 0.0],
    });
    v.push(Case {
        name: "far reference 1e4".into(),
        deg: 3,
        knots: clamped(3, &[0.3, 0.6]),
        pts: vec![[0.0, 0.0, 0.0], [1.0, 2.0, 0.5], [2.0, -1.0, 0.0], [3.0, 1.0, 1.0], [2.5, 2.5, 0.0], [1.0, 3.0, -1.0]],
        w: vec![1.0, 2.0, 0.5, 1.0, 1.5, 0.8],
        t0: 0.1,
        t1: 0.95,
        reference: [1e4, -2e4, 3e3],
    });
    v.push(Case {
        name: "coincident controls (B' small)".into(),
        deg: 3,
        knots: clamped(3, &[]),
        pts: vec![[1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        w: vec![1.0, 3.0, 0.2, 1.0],
        t0: 0.0,
        t1: 1.0,
        reference: [0.0, 0.0, 0.0],
    });
    v.push(Case {
        name: "very short span".into(),
        deg: 2,
        knots: clamped(2, &[0.5, 0.5 + 1e-9]),
        pts: vec![[1.0, 0.0, 0.0], [1.5, 1.2, 0.0], [0.5, 2.0, 0.0], [-0.2, 1.0, 0.0], [-1.0, 0.0, 0.0]],
        w: vec![1.0, 0.3, 2.0, 0.7, 1.0],
        t0: 0.0,
        t1: 1.0,
        reference: [0.0, 0.0, 0.0],
    });
    for i in 0..60 {
        let deg = 2 + (i % 5);
        let nint = (g.f() * 4.0) as usize;
        let mut interior: Vec<f64> = (0..nint).map(|_| (g.r(0.05, 0.95) * 64.0).round() / 64.0).collect();
        interior.sort_by(f64::total_cmp);
        if i % 7 == 3 && !interior.is_empty() {
            let k = interior[0];
            interior.push(k); // raise a multiplicity
            interior.sort_by(f64::total_cmp);
        }
        let interior: Vec<f64> = interior
            .chunk_by(|a, b| a == b)
            .flat_map(|c| c.iter().take(deg).copied())
            .collect();
        let n = deg + 1 + interior.len();
        let wide = i % 3 == 0;
        let pts = (0..n).map(|_| [g.r(-3.0, 3.0), g.r(-3.0, 3.0), g.r(-3.0, 3.0)]).collect();
        let w = (0..n)
            .map(|_| if wide { 10f64.powf(g.r(-3.0, 3.0)) } else { g.r(0.2, 3.0) })
            .collect();
        let (a, b) = (g.r(0.0, 0.5), g.r(0.5, 1.0));
        let (t0, t1) = if i % 2 == 0 { (0.0, 1.0) } else { ((a * 1024.0f64).round() / 1024.0, (b * 1024.0f64).round() / 1024.0) };
        v.push(Case {
            name: format!("random {i} deg {deg}{}", if wide { " wide weights" } else { "" }),
            deg,
            knots: clamped(deg, &interior),
            pts,
            w,
            t0,
            t1,
            reference: [g.r(-5.0, 5.0), g.r(-5.0, 5.0), g.r(-5.0, 5.0)],
        });
    }
    v
}

/// Dumps enclosures for `review_5696_ref.py` (`PROBE_OUT=<file>`).
#[test]
#[ignore = "review probe: dump for the mpmath reference"]
fn dump_rational_rim_enclosures() {
    let path = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/dev/null".into());
    let mut out = std::fs::File::create(path).unwrap();
    for c in cases() {
        run(&c, &mut out);
    }
}

/// **A carrier with two distinct interior knots holds its 40-digit
/// truth** (mpmath, `review_5696_ref.py`): the rim rows' only insertion
/// cases have every right-hand knot neighbour at the clamped end, where a
/// wrong Boehm ratio (`knots[i + p + 1]` for `knots[i + p]`) is
/// equivalent, so it survives them; this row goes red under it.
#[test]
fn two_interior_knots_hold_the_mpmath_truth() {
    let c = cases().into_iter().find(|c| c.name == "far reference 1e4").unwrap();
    let truth = [-6941.638973648976952, -5715.356377599417599, -14975.94551559863074];
    let f = NurbsCurve3::new(
        KnotVector::clamped(c.knots.clone(), c.deg).unwrap(),
        c.pts.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
        c.w.clone(),
    )
    .unwrap()
    .map_scalar(Interval::point);
    let r = Point3::new(
        Interval::point(c.reference[0]),
        Interval::point(c.reference[1]),
        Interval::point(c.reference[2]),
    );
    let e = rational_edge_q(&f, Interval::point(c.t0), Interval::point(c.t1), true, r).unwrap();
    let (va, _) = rational_vector_area_split(&e, 64);
    for k in 0..3 {
        // The truth's f64 rounding is far inside these widths (~1e-4).
        assert!(
            va[k].is_certified() && va[k].lo() <= truth[k] && truth[k] <= va[k].hi(),
            "component {k}: {:?} ∌ {}",
            va[k],
            truth[k]
        );
    }
}
