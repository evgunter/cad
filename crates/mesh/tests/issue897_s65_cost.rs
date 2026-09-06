//! Cost instrument for the two S65 cases issue 897 names: the full-2π
//! seam (no mechanical check in the curved lane) and cross-face
//! identification (no check in any lane). It prices what a guard for
//! each would have to compute, end to end through the public door, so
//! the coverage decision is made on numbers rather than on an estimate.
//!
//! **Inert unless armed** (`CAD_S65_COST=1`), like `budget_meter`'s
//! feature gate: it is a stopwatch, and a stopwatch in the gate would
//! report the runner's neighbours rather than this crate. Arming it
//! prints one row per (body, δ) — the tour corpus at the byte
//! instrument's three deltas — and the run's ε is the ambient one, so
//! the three ε rows are three runs of the same command.
//!
//! Two columns, and they are different KINDS of number:
//!
//! * `tess` is `mesh::tessellate` end to end, the denominator.
//! * `check` is [`mesh::validate::check_mesh`] on the mesh that came
//!   out — the whole-mesh edge census, which is exactly the work a
//!   cross-face identification guard does. It is measurable from
//!   outside the crate because the checker is already public and
//!   already the oracle; nothing here re-implements it.
//!
//! The seam case's guard is not priced from outside: it widens the
//! emit pass's pole-incident edge census to every identified id, which
//! is interior to `curved`. It is priced by A/B on `tess` — this file
//! at two revisions, one command apart.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::common;
use common::*;
use geom_core::Tol;
use std::time::Instant;
use topo::Body;

/// Reps per row after a warm-up (S65's own option-B pricing used 40).
const REPS: u32 = 40;

fn armed() -> bool {
    std::env::var("CAD_S65_COST").is_ok_and(|v| v != "0")
}

#[test]
fn issue897_guard_cost_report() {
    if !armed() {
        return;
    }
    let bodies: Vec<(&str, Body<f64>)> = vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("wedge", wedge()),
        ("cone_wedge", cone_wedge(0.05, 0.5)),
    ];
    println!("S65-COST eps = {}", eps());
    println!("S65-COST body delta tris tess_us check_us check_pct");
    for (name, b) in &bodies {
        for d in [0.1f64, 0.02, 0.004] {
            let warm = mesh::tessellate(b, d, Tol::witness()).unwrap();
            let tris = mesh::validate::triangle_count(&warm);
            let t0 = Instant::now();
            for _ in 0..REPS {
                let m = mesh::tessellate(b, d, Tol::witness()).unwrap();
                std::hint::black_box(&m);
            }
            let tess = t0.elapsed().as_secs_f64() / f64::from(REPS);
            let t1 = Instant::now();
            for _ in 0..REPS {
                std::hint::black_box(mesh::validate::check_mesh(&warm).is_ok());
            }
            let check = t1.elapsed().as_secs_f64() / f64::from(REPS);
            println!(
                "S65-COST {name} {d} {tris} {:.1} {:.1} {:.2}",
                tess * 1e6,
                check * 1e6,
                100.0 * check / tess
            );
        }
    }
}
