use geom_core::{Dual64, Tol};
use topo::Body;
fn door(b: &Body<Dual64>, tol: Tol) {
    let _ = topo::validate_geometric(b, tol);
}
fn main() {}
