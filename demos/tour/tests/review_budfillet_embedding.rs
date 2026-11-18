//! Review probe (PR 7120): `budfillet`'s embedding verdict through the public
//! door, the tour walk's question for this one scene. Prints; asserts nothing.
//! Run with a `topo` dev-dependency and a `[[test]]` entry (not wired here).
use pncad::geom_core::{Band, Tol};
use topo::clearance::{Budgets, Side, embedded, DEFAULT_MAX_CELL_DEPTH};

#[test]
fn budfillet_embedding() {
    let tol = Tol::witness();
    for stop in demo_tour::bud::stops(tol) {
        for sb in &stop.bodies {
            let body = &sb.body;
            let side = Side { body, faces: body.faces().map(|(k, _)| k).collect() };
            let r = embedded(&side, Band::linear(tol).unwrap(), Budgets { max_cell_depth: DEFAULT_MAX_CELL_DEPTH, max_cell_pairs: 65_536 });
            println!("REVIEW {} : {:?} {:?}", sb.name, r.verdict, r.receipt);
        }
    }
}
