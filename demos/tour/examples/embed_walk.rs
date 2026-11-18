//! Walks the tour and asks `topo::clearance::embedded` of every finished body.
use pncad::geom_core::{Band, Tol};
use pncad::topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, Side, embedded};

fn main() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let work = std::env::temp_dir().join("embed-walk-store");
    let _ = std::fs::create_dir_all(&work);
    let mut n = 0;
    let mut held = 0;
    demo_tour::walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                n += 1;
                let side = Side {
                    body: &sb.body,
                    faces: sb.body.faces().map(|(k, _)| k).collect(),
                };
                let t = std::time::Instant::now();
                let r = embedded(
                    &side,
                    band,
                    Budgets {
                        max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
                        max_cell_pairs: 65_536,
                    },
                );
                let v = match &r.verdict {
                    Embedding::Embedded => {
                        held += 1;
                        "Embedded".to_string()
                    }
                    Embedding::Crosses(c) => format!("Crosses {:?}", c.faces),
                    Embedding::Refused(e) => format!("Refused {e:?}"),
                };
                eprintln!(
                    "WALK {}/{} | {} | cand {} disc {} refused {} splits {} | {:.2}s",
                    stop.name,
                    sb.name,
                    v,
                    r.receipt.candidates,
                    r.receipt.discharged,
                    r.receipt.refused,
                    r.receipt.splits,
                    t.elapsed().as_secs_f64()
                );
            }
        },
        &work,
        tol,
    );
    eprintln!("WALK TOTAL {held} of {n} hold");
}
