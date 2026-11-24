//! The tour's embedding walk: asks [`pncad::topo::clearance::embedded`]
//! of every finished body of every tour stop and prints one verdict
//! line per body, then the total.
//!
//! # What it asks
//!
//! For each body the scope is every face of the body, in the body's own
//! face order, at `f64`. The band is [`Band::linear`] of the run's
//! tolerance ([`Tol::witness`]: ε from `CAD_TOLERANCE_EPS`, 1e-9 when
//! unset), and the budgets are [`DEFAULT_MAX_CELL_DEPTH`] per pair and
//! 65 536 cell pairs per body. The header line prints the ε and both
//! budgets the run used.
//!
//! # Running it
//!
//! From `demos/tour/` (a cargo root of its own, outside the kernel
//! workspace), with a target directory outside the checkout:
//!
//! ```sh
//! cd demos/tour
//! CARGO_TARGET_DIR=/home/user/<lane>-target \
//!   cargo run --release --example embed_walk | grep '^WALK' > walk.txt
//! ```
//!
//! # Output
//!
//! The report is the lines of **stdout** that begin `WALK`; the tour's
//! scenes narrate on stdout as they build, which is what the `grep`
//! above drops. The report is a function of the tree and ε, so two
//! runs' `walk.txt` diff clean when nothing changed:
//!
//! ```text
//! WALK HEADER | eps 1e-9 | max_cell_depth 40 | max_cell_pairs 65536
//! WALK <stop>/<body> | <verdict> | cand <n> disc <n> refused <n> splits <n>
//! WALK TOTAL <held> of <bodies> hold
//! ```
//!
//! One line per body, in tour order and, within a stop, the stop's body
//! order. `<verdict>` is `Embedded`, `Crosses <the two faces>` or
//! `Refused <the engine's refusal>`; only `Embedded` counts as held.
//! The four counts are the query's [`CellReceipt`]: candidate face
//! pairs, then cell pairs discharged, refused and split.
//!
//! Wall time per body goes to **stderr**, as `TIME <stop>/<body> <s>s`,
//! so it never lands in a captured report.
//!
//! [`CellReceipt`]: pncad::topo::clearance::CellReceipt
use pncad::geom_core::{Band, Tol};
use pncad::topo::clearance::{Budgets, DEFAULT_MAX_CELL_DEPTH, Embedding, embedded};

const BUDGETS: Budgets = Budgets {
    max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
    max_cell_pairs: 65_536,
};

fn main() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's ε admits a linear band");
    // The assembly stop's document store.
    let work = std::env::temp_dir().join("embed-walk-store");
    std::fs::create_dir_all(&work).expect("create the assembly stop's store");
    println!(
        "WALK HEADER | eps {:e} | max_cell_depth {} | max_cell_pairs {}",
        tol.eps(),
        BUDGETS.max_cell_depth,
        BUDGETS.max_cell_pairs
    );
    let mut bodies = 0;
    let mut held = 0;
    demo_tour::walk_tour(
        &mut |stop| {
            for sb in &stop.bodies {
                bodies += 1;
                let started = std::time::Instant::now();
                // The question takes tier 3's facts: a body tier 3's
                // local battery refuses is not asked.
                let geometric = match pncad::topo::geometric(&sb.body, tol) {
                    Ok(geometric) => geometric,
                    Err(findings) => {
                        println!(
                            "WALK {}/{} | NotGeometric {findings:?} | cand 0 disc 0 refused 0 splits 0",
                            stop.name, sb.name
                        );
                        continue;
                    }
                };
                let faces = sb.body.faces().map(|(k, _)| k).collect();
                let report = embedded(&geometric, faces, band, BUDGETS);
                let verdict = match &report.verdict {
                    Embedding::Embedded => {
                        held += 1;
                        "Embedded".to_string()
                    }
                    Embedding::Crosses(c) => format!("Crosses {:?}", c.faces),
                    Embedding::Refused(e) => format!("Refused {e:?}"),
                };
                println!(
                    "WALK {}/{} | {} | cand {} disc {} refused {} splits {}",
                    stop.name,
                    sb.name,
                    verdict,
                    report.receipt.candidates,
                    report.receipt.discharged,
                    report.receipt.refused,
                    report.receipt.splits,
                );
                eprintln!(
                    "TIME {}/{} {:.2}s",
                    stop.name,
                    sb.name,
                    started.elapsed().as_secs_f64()
                );
            }
        },
        &work,
        tol,
    );
    println!("WALK TOTAL {held} of {bodies} hold");
}
