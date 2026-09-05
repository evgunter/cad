use geom_core::{CertifiedEnclosure, Decide, Tol};
use topo::{Body, EdgeCurveSpec, entity::EdgeKey};
fn door<T: Decide + CertifiedEnclosure>(
    b: &mut Body<T>,
    e: EdgeKey,
    c: EdgeCurveSpec<T>,
    tol: Tol,
) {
    let _ = b.set_edge_curve_nurbs_lane(e, c, tol);
}
fn main() {}
