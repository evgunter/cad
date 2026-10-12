//! **Every fused vertex settles on a live cell of the result**
//! (`BooleanNaming::fused_into`): a vertex a weld or a zip fused away
//! lies at its survivor, and where a later stage deleted the survivor
//! without moving its point, inside the edge the output stage's join
//! made or the face the merge glued. Both stages record the kill, so the
//! chase reaches a live cell on every result.
//!
//! The corpus is a lattice of bricks: `[0, 2]³` against every brick
//! whose span on each axis is one of ten, flush, overlapping or clear of
//! it, under all three operations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{BooleanResult, Cell, intersect, subtract, union};

/// The corpus, counted: every result builds, every fused vertex settles
/// on a live cell, and so does each end of every stretch a discard
/// bordered. 336 fused vertices lie inside an edge the output stage's
/// join made and 432 inside a face the merge glued (420 pruned as a
/// dangling seam's end, 12 with a lone ring); the rest at a live
/// survivor. Of the bordered stretches, those with an end inside an
/// edge or a face are counted too: the naming layer reads them as
/// bordering no edge.
#[test]
fn every_fused_vertex_of_the_lattice_corpus_settles_on_a_live_cell() {
    let tol = Tol::witness();
    let spans: [(f64, f64); 10] = [
        (-1.0, 1.0),
        (-1.0, 3.0),
        (0.0, 1.0),
        (0.0, 2.0),
        (0.5, 1.5),
        (1.0, 2.0),
        (1.0, 3.0),
        (2.0, 3.0),
        (-0.5, 0.5),
        (0.5, 2.5),
    ];
    let a = finished(
        "a",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol),
        tol,
    );
    let (mut bodies, mut vertices, mut edges, mut faces) = (0, 0, 0, 0);
    let (mut stretches, mut inside) = (0, 0);
    for x in spans {
        for y in spans {
            for z in spans {
                let b = finished("b", brick::<f64>(x, y, z, tol), tol);
                for (op, result) in [
                    ("union", union(&a, &b, tol)),
                    ("subtract", subtract(&a, &b, tol)),
                    ("intersect", intersect(&a, &b, tol)),
                ] {
                    let who = format!("{op} with {x:?} × {y:?} × {z:?}");
                    let result = result.unwrap_or_else(|e| panic!("{who}: {e}"));
                    let BooleanResult::Body(out) = result else {
                        continue;
                    };
                    bodies += 1;
                    let fused = out
                        .naming
                        .fused_into(&out.body)
                        .unwrap_or_else(|e| panic!("{who}: {e:?}"));
                    let settler = out.naming.settler(&out.body);
                    for d in &out.naming.discards {
                        for &(u, w) in &d.bordered {
                            let ends = [u, w].map(|v| {
                                settler
                                    .settle(v)
                                    .unwrap_or_else(|e| panic!("{who}: a bordered end: {e:?}"))
                            });
                            stretches += 1;
                            if ends.iter().any(|c| !matches!(c, Cell::Vertex(_))) {
                                inside += 1;
                            }
                        }
                    }
                    for cell in fused.values() {
                        match cell {
                            Cell::Vertex(_) => vertices += 1,
                            Cell::Edge(_) => edges += 1,
                            Cell::Face(_) => faces += 1,
                        }
                    }
                }
            }
        }
    }
    assert_eq!(bodies, 2721, "the results that build a body");
    assert_eq!(
        (vertices, edges, faces),
        (12_732, 336, 432),
        "fused vertices settled at a live survivor, inside a joined edge, inside a merged face"
    );
    assert_eq!(
        (stretches, inside),
        (27_000, 2448),
        "bordered stretches, and those with an end inside an edge or a face"
    );
}
