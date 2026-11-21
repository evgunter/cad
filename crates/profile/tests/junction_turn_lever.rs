//! Reproduction probe (temporary).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Open, Start};

#[test]
fn probe_the_64_degree_rows() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let phi = 64.0_f64.to_radians();
    let (c, s) = (phi.cos(), phi.sin());
    for k in [0.3, 0.9, 1.05, 1.1, 3.0, 50.0] {
        let leg = Open
            .at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(k * eps, 0.0), tol)
            .unwrap();
        let toward = leg.clone().toward(c, s, tol).map(|_| ());
        let line = leg
            .line_to(Point2::new(k * eps + c, s), tol)
            .map(|_| ());
        // The seam: a closing leg of k·ε, continuing a long leg along
        // 64°, arriving at Start whose first side runs +x.
        let p = Point2::new(-k * eps * c, -k * eps * s);
        let q = Point2::new(p.x - c, p.y - s);
        let seam = Open
            .at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(1.0, 0.0), tol)
            .unwrap()
            .line_to(q, tol)
            .unwrap()
            .line_to(p, tol)
            .unwrap()
            .continue_to(Start, tol)
            .map(|_| ());
        eprintln!("k={k}\n  toward: {toward:?}\n  line:   {line:?}\n  seam:   {seam:?}");
        if let Err(e) = &line {
            eprintln!("  line says: {e}");
        }
        if let Err(e) = &seam {
            eprintln!("  seam says: {e}");
        }
    }
}
