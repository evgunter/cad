//! Review probe (PR 4121): extra bit-identity rows for fully requested
//! planar bodies, dumped with the naming record. Unarmed without BITDUMP_DIR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, missing_docs)]

use std::fmt::Write as _;

use geom_core::{Point2, Tol};
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{block, cube, pocket_die, prism};
use topo::{Body, query};

use crate::common::bitdump::{dump, dump_dir, save};

fn both(dir: &str, name: &str, body: &Body<f64>, edges: &[topo::EdgeKey], d: f64) {
    for (verb, r) in [
        ("chamfer", chamfer_edges(body, edges, d, Tol::witness())),
        ("fillet", fillet_edges(body, edges, d, Tol::witness())),
    ] {
        let text = match r {
            Ok(out) => {
                let mut t = dump(&out.body);
                let _ = writeln!(
                    t,
                    "blend={:?} corner={:?} band={:?}\nnaming={:?}",
                    out.blend_faces, out.corner_faces, out.band_faces, out.naming
                );
                t
            }
            Err(e) => format!("ERR {:?}", e.error),
        };
        save(dir, &format!("x_{name}_{verb}"), &text);
    }
}

#[test]
fn bitdump_review_4121_full_planar() {
    let Some(dir) = dump_dir() else { return };
    let t = Tol::witness();
    let c = cube(1.0, t);
    both(&dir, "cube", &c, &query::all_edges(&c), 0.1);
    both(&dir, "cube015", &c, &query::all_edges(&c), 0.15);
    let b = block(2.0, 1.5, 1.0, t);
    both(&dir, "block", &b, &query::all_edges(&b), 0.1);
    let tri = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(0.7, 1.3), 0.0),
        ],
        1.0,
        t,
    );
    both(&dir, "tri", &tri, &query::all_edges(&tri), 0.08);
    let pent = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.4, 1.0), 0.0),
            (Point2::new(1.0, 1.8), 0.0),
            (Point2::new(-0.3, 1.0), 0.0),
        ],
        1.0,
        t,
    );
    both(&dir, "pent", &pent, &query::all_edges(&pent), 0.08);
    let split = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ],
        1.0,
        t,
    );
    both(&dir, "split", &split, &query::all_edges(&split), 0.1);
    let die = pocket_die(0.0, 0.0, 0.0, t);
    both(&dir, "pocketdie_all", &die, &query::all_edges(&die), 0.05);
}
