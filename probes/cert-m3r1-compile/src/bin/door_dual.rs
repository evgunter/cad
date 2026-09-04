use geom_core::{Dual64, Tol};
use topo::{Body, EdgeCurveSpec, entity::EdgeKey};
fn lane_door(b: &mut Body<Dual64>, e: EdgeKey, c: EdgeCurveSpec<Dual64>, tol: Tol) {
    let _ = b.set_edge_curve_nurbs_lane(e, c, tol);
}
fn main() {}
