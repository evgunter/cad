//! Review probe for PR 4540: wall time of a realistic loft build (which
//! certifies every edge) at f64 and Interval.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Affine3, Interval, Point2, Tol, Vec3};
use profile::ProfileLoop;
#[allow(unused_imports)]
use profile::RawLoop;
use sweep::{Section, loft_body};

fn sections(n: usize, a: f64) -> (Vec<Section>, Vec<Affine3<f64>>) {
    let sq = || {
        vec![ProfileLoop::polygon(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .iter()
                .map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let places = (0..n)
        .map(|i| {
            let dx = a * (i as f64 * 0.7).sin();
            Affine3::translation(Vec3::new(dx, 0.0, i as f64))
        })
        .collect();
    ((0..n).map(|_| sq()).collect(), places)
}

#[test]
fn loft_build_cost() {
    for (n, deg) in [(4, 2), (12, 3), (40, 3)] {
        let (s, p) = sections(n, 0.3);
        let reps = 5;
        let t = std::time::Instant::now();
        for _ in 0..reps {
            loft_body::<f64>(&s, &p, deg, Tol::witness()).unwrap();
        }
        let f = t.elapsed() / reps;
        let t = std::time::Instant::now();
        for _ in 0..reps {
            loft_body::<Interval>(&s, &p, deg, Tol::witness()).unwrap();
        }
        let i = t.elapsed() / reps;
        eprintln!("COST n {n} deg {deg}: f64 {f:?}  interval {i:?}");
    }
}
