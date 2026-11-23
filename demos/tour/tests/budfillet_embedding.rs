//! **The bud fillet's body is embedded at f64**: the verdict door
//! (`topo::clearance::embedded`) at the default depth and the at-rest
//! gate's pair budget holds every body of the `budfillet` stop. Its base
//! disc's window is read off its rim circle's cover, and a window an ulp
//! wider than the circle's own bound halves to cells a rounding off the
//! circle that no presentation reads
//! (`work/embed/a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth.md`).
//! Red if the stop refuses or crosses.

use pncad::geom_core::{Band, Tol};
use pncad::topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, Side, embedded};

#[test]
fn every_budfillet_body_is_embedded() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budgets = Budgets {
        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
        max_cell_pairs: 65_536,
    };
    let mut asked = 0;
    for stop in demo_tour::bud::stops(tol) {
        for sb in &stop.bodies {
            let side = Side {
                body: &sb.body,
                faces: sb.body.faces().map(|(k, _)| k).collect(),
            };
            let report = embedded(&side, band, budgets);
            assert!(
                matches!(report.verdict, Embedding::Embedded),
                "{}: {:?} {:?}",
                sb.name,
                report.verdict,
                report.receipt
            );
            asked += 1;
        }
    }
    assert!(asked > 0, "the bud stops carry a body");
}
