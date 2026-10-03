//! Delta reviewer probe (reach-delta-3977): a thin disc (one cylindrical
//! wall, so the walk is not all-planar and the re-derivation keeps the
//! world-origin form) placed far from the world origin, upright and
//! inside out. Oracle: π r² h for an n-arc circle (exact arcs).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol};
use sweep::test_support::cylinder_of_arcs_at;
use topo::{mass_properties, validate_geometric};

#[test]
fn dprobe_far_thin_disc() {
    let tol = Tol::witness();
    println!("DPROBE|eps={:e}", tol.eps());
    for &(r, h) in &[(1e-3, 1e-7), (1e-3, 1e-6), (1e-3, 1e-5), (1e-2, 1e-6), (1e-2, 1e-5), (1e-1, 1e-5)] {
        if h < 15.0 * tol.eps() { continue; }
        for &d in &[0.0, 1.0, 10.0, 100.0, 1e3, 5e3, 2e4] {
            let built = std::panic::catch_unwind(|| cylinder_of_arcs_at(4, r, Point2::new(d * 0.6, d * 0.48), d * 0.64, h, tol));
            let Ok(b) = built else { println!("DPROBE|r={r:e} h={h:e} d={d:e}|no build"); continue; };
            let exact = std::f64::consts::PI * r * r * h;
            for (tag, body, want) in [("upright", b.clone(), exact), ("INVERTED", b.revert().unwrap(), -exact)] {
                let v = mass_properties(&body, tol).map(|p| p.volume).unwrap_or(f64::NAN);
                let g = validate_geometric(&body, tol);
                let flag = match (&g, want > 0.0) {
                    (Ok(()), false) => " <-- INSIDE-OUT PASSES",
                    (Err(_), true) => " <-- VALID REFUSED",
                    _ => "",
                };
                let gs: String = format!("{g:?}").chars().take(90).collect();
                println!("DPROBE|r={r:e} h={h:e} d={d:e}|{tag}|exact={want:+.3e}|f64={v:+.3e}|tier3={gs}{flag}");
            }
        }
    }
}
