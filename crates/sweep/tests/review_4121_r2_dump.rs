//! Review probe (band-dual-4121-r2): base/head dump of every fully
//! requested body, body bits AND birth records, armed by `R2DUMP`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use crate::common::bitdump::dump;
use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{block, cube, pocket_die};
use topo::{Body, query};

fn run(name: &str, body: &Body<f64>, r: f64, out: &mut String) {
    let edges = query::all_edges(body);
    for (verb, res) in [
        (
            "fillet",
            fillet_edges(body, &edges, r, Tol::witness()).map_err(|e| format!("{}", e.error)),
        ),
        (
            "chamfer",
            chamfer_edges(body, &edges, r, Tol::witness()).map_err(|e| format!("{}", e.error)),
        ),
    ] {
        let _ = writeln!(out, "=== {name} {verb} r={r}");
        match res {
            Ok(b) => {
                out.push_str(&dump(&b.body));
                let _ = writeln!(
                    out,
                    "blend={:?} corner={:?} band={:?}\nnaming={:#?}",
                    b.blend_faces, b.corner_faces, b.band_faces, b.naming
                );
            }
            Err(e) => {
                let _ = writeln!(out, "ERR {e}");
            }
        }
    }
}

#[test]
fn r2_dump_fully_requested() {
    let Ok(dir) = std::env::var("R2DUMP") else {
        return;
    };
    let mut s = String::new();
    run("cube1", &cube(1.0, Tol::witness()), 0.15, &mut s);
    run("cube1", &cube(1.0, Tol::witness()), 0.1, &mut s);
    run("box", &block(2.0, 1.5, 1.0, Tol::witness()), 0.1, &mut s);
    run(
        "pocket_die",
        &pocket_die(0.0, 0.0, 0.0, Tol::witness()),
        0.05,
        &mut s,
    );
    std::fs::write(format!("{dir}/dump.txt"), s).unwrap();
}
