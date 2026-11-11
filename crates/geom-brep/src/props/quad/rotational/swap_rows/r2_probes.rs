//! Reviewer r2's probes of PR 6808 (not for merge).

use super::*;

/// A signed area huge against a whole torus: does the count window
/// terminate? `lo += 1.0` is a no-op past 2^53.
#[test]
fn r2_count_window_terminates_on_a_huge_x() {
    let (big, r) = (1.3_f64, 0.45_f64);
    for x in [(1e17, 1e17 + 1e3), (1e20, 1e20), (-1e20, 1e20), (1e20, 2e20)] {
        let t = std::time::Instant::now();
        let face = super::super::torus_face(
            (pt(big), pt(r)),
            true,
            &[[pt(0.25), Interval::from_bounds(x.0, x.1)]],
        );
        eprintln!("x {x:?}: area {:?} flux {:?} in {:?}", face.area, face.flux, t.elapsed());
    }
}

/// X exactly at an end of the window, thin.
#[test]
fn r2_count_window_at_the_ends() {
    let (big, r) = (pt(1.3_f64), pt(0.45_f64));
    let [_, a] = super::super::torus_whole(big, r);
    for x in [a, Interval::zero(), -a, pt(2.0) * a] {
        let face = super::super::torus_face((big, r), true, &[[pt(0.25), x]]);
        eprintln!("x {x:?}: area {:?} flux {:?}", face.area, face.flux);
    }
}
