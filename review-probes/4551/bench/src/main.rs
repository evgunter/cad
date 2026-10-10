use geom_core::spline::compose::{CanonicalSurface, CurveCertData, canonical_composite};
use geom_core::{Interval, KnotVector};
use std::time::Instant;
fn main() {
    let p = 3; let spans = 24;
    let mut knots = vec![0.0; p + 1];
    for i in 1..spans { knots.push(i as f64 / spans as f64); }
    knots.extend(vec![1.0; p + 1]);
    let kv = KnotVector::clamped(knots, p).unwrap();
    let n = kv.control_count();
    let w: Vec<f64> = (0..n).map(|i| 1.0 + 0.1 * ((i as f64) * 0.7).sin().abs()).collect();
    let coords: Vec<Vec<Interval>> = (0..3).map(|d| (0..n).map(|i| { let t = i as f64 / n as f64; let v = match d {0=>(6.0*t).cos(),1=>(6.0*t).sin(),_=>0.3*t}; Interval::from_bounds(v, v) }).collect()).collect();
    let cuts: Vec<f64> = (0..spans).flat_map(|s| (1..8).map(move |j| (s as f64 + j as f64 / 8.0) / spans as f64)).collect();
    let surf = CanonicalSurface::Sphere { radius: Interval::from_bounds(1.0, 1.0) };
    let mut sink = 0.0;
    for (tag, extra) in [("whole", &[][..]), ("8 cuts/span", &cuts[..])] {
        let reps = 300;
        let t0 = Instant::now();
        for _ in 0..reps {
            let d = CurveCertData::new(&kv, &w, &coords).unwrap().with_breaks(extra);
            let c = canonical_composite(&d, &surf).unwrap();
            sink += c.sup_bound();
        }
        println!("{tag}: {:.1} us/composite (sup {:e})", t0.elapsed().as_secs_f64() * 1e6 / reps as f64, sink / reps as f64);
        sink = 0.0;
    }
}
