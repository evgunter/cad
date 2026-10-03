#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol};
use sweep::test_support::cylinder_of_arcs_at;
use topo::validate_geometric;
#[test]
fn dprobe_disc_detail() {
    let tol = Tol::witness();
    for &(r, h, d) in &[(1e-3, 1e-6, 5e3), (1e-2, 1e-6, 2e4)] {
        let Ok(b) = std::panic::catch_unwind(|| cylinder_of_arcs_at(4, r, Point2::new(d * 0.6, d * 0.48), d * 0.64, h, tol)) else { continue };
        println!("DETAIL r={r:e} h={h:e} d={d:e} upright: {:?}", validate_geometric(&b, tol));
        println!("DETAIL r={r:e} h={h:e} d={d:e} inverted: {:?}", validate_geometric(&b.revert().unwrap(), tol));
    }
}
