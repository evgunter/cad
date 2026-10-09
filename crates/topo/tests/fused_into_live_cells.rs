//! **Every fused vertex settles on a live cell of the result**
//! (`BooleanNaming::fused_into`): a vertex a weld or a zip fused away
//! lies at its survivor, and where a later stage deleted the survivor
//! without moving its point, inside the edge the output stage's join
//! made or the face the merge glued. Both stages record the kill, so the
//! chase reaches a live cell on every result.
//!
//! The corpus is a lattice of bricks: `[0, 2]³` against every brick
//! whose span on each axis is one of ten, flush, overlapping or clear of
//! it, under all three operations, every coplanar face pair declared
//! (rest where the normals oppose, continuation where they agree). The
//! zips' fusions live on those declared paths; undeclared, a flush pair
//! refuses `UndeclaredCoincidence` before any is made.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{
    Body, BooleanDeclarations, BooleanResult, Cell, FaceKey, FacePairDeclaration, intersect_with,
    subtract_with, union_with,
};

/// Each face of a brick with its outward plane, `(normal, offset)`.
fn planes(b: &Body<f64>) -> Vec<(FaceKey, [f64; 3], f64)> {
    b.faces()
        .map(|(k, f)| {
            let Some(geom::Surface::Plane { origin, normal, .. }) = b.get_surface(f.surface) else {
                panic!("a brick is planar")
            };
            let s = if f.sense { 1.0 } else { -1.0 };
            let n = [normal.x * s, normal.y * s, normal.z * s];
            (k, n, n[0] * origin.x + n[1] * origin.y + n[2] * origin.z)
        })
        .collect()
}

/// Every coplanar face pair of `a` and `b`, declared. The lattice's
/// planes are exact, so equality is the coincidence.
fn declarations(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::default();
    for (fa, na, da) in planes(a) {
        for (fb, nb, db) in planes(b) {
            let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
            if dot == 1.0 && da == db {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::continuation(fa, fb));
            } else if dot == -1.0 && da == -db {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::rest(fa, fb));
            }
        }
    }
    decls
}

/// The corpus, counted: every result builds, every fused vertex settles
/// on a live cell, and the cells split as measured on main before the
/// map was typed — 336 fused vertices whose survivor the output stage's
/// join deleted (inside the joined edge), 432 whose survivor the merge
/// pruned (inside the merged face: 420 by the dangling-seam pruning, 12
/// by a lone ring's deletion), and the rest at a live survivor. Before,
/// each of the 768 named a dead key.
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
    for x in spans {
        for y in spans {
            for z in spans {
                let b = finished("b", brick::<f64>(x, y, z, tol), tol);
                let d = declarations(&a, &b);
                for (op, result) in [
                    ("union", union_with(&a, &b, &d, tol)),
                    ("subtract", subtract_with(&a, &b, &d, tol)),
                    ("intersect", intersect_with(&a, &b, &d, tol)),
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
}
