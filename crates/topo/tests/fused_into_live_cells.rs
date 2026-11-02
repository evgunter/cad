//! **Every fused vertex settles on the live cell holding its point**
//! (`BooleanNaming::fused_into`, `BooleanNaming::settler`): a vertex a
//! zip fused away lies at its survivor, and where a later stage deleted
//! the survivor without moving its point, inside the edge the output
//! stage's join made or the face the merge glued. Both stages record the
//! kill, so the chase reaches a live cell on every result, and that cell
//! holds the point.
//!
//! The corpus is a lattice of bricks: `[0, 2]³` against every brick
//! whose span on each axis is one of ten, flush, overlapping or clear of
//! it, under all three operations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::common;

use common::{brick, finished};
use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol};
use topo::{Body, BooleanResult, Cell, VertexKey, intersect, subtract, union};

/// How far a point may sit off the cell that holds it: the lattice's
/// coordinates are exact, so this only absorbs the carriers' rounding.
const ON: f64 = 1e-12;

/// Each vertex's point as the output stage found it, before its merge
/// and join (`topo::take_output_stage_points`, armed by the row), merged over every output
/// stage the op ran: a key the stages disagree on panics.
fn output_stage_points(who: &str) -> BTreeMap<VertexKey, Point3<f64>> {
    let mut at = BTreeMap::new();
    for stage in topo::take_output_stage_points().expect("the row arms its thread") {
        for (v, [x, y, z]) in stage {
            let p = Point3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
            if let Some(q) = at.insert(v, p) {
                assert!(same(q, p), "{who}: two output stages place {v:?} apart");
            }
        }
    }
    at
}

/// Whether `p` and `q` are one point, bit for bit.
fn same(p: Point3<f64>, q: Point3<f64>) -> bool {
    [p.x, p.y, p.z].map(f64::to_bits) == [q.x, q.y, q.z].map(f64::to_bits)
}

/// Whether `cell` of `body` holds `p`: a vertex at it, bit for bit; an
/// edge (every brick edge is a line) with `p` on its carrier strictly
/// between its ends; a face (every brick face is a plane) with `p` on
/// its plane and inside it. "On" is this row's own claim, `ON`, far
/// tighter than the band: a fused point sits on its cell to rounding.
/// A face's region is then read by the census's door
/// ([`topo::test_support::face_holds`]), whose own on-plane decision on
/// the band the `ON` check already passes.
fn holds(body: &Body<f64>, cell: Cell, p: Point3<f64>, band: Band) -> bool {
    match cell {
        Cell::Vertex(v) => same(topo::readback::vertex_point(body, v).unwrap(), p),
        Cell::Edge(e) => {
            let curve = body.get_edge(e).unwrap().curve;
            let c = body
                .get_curve_geom(curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            let Curve3::Line { origin, dir } = c.carrier() else {
                panic!("a brick edge is a line: {:?}", c.carrier());
            };
            let (t0, t1) = c.params();
            let t = (p - *origin).dot(*dir) / dir.dot(*dir);
            let foot = *origin + *dir * t;
            foot.distance(p) <= ON && t0.min(t1) + ON < t && t < t0.max(t1) - ON
        }
        Cell::Face(f) => {
            let face = body.get_face(f).unwrap();
            let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(face.surface) else {
                panic!("a brick face is a plane");
            };
            (p - *origin).dot(*normal).abs() <= ON
                && topo::test_support::face_holds(body, f, p, band)
                    == Ok(Some(topo::FaceContainment::In))
        }
    }
}

/// The corpus, counted. Every result builds, every fused vertex settles
/// on a live cell, and so does each end of every stretch a discard
/// bordered; and each such cell holds the point its vertex had when the
/// output stage began. 336 fused vertices lie inside an edge the output
/// stage's join made and 432 inside a face the merge glued (420 pruned
/// as a dangling seam's end, 12 with a lone ring); the rest at a live
/// survivor. Of the bordered stretches, those with an end inside an edge
/// or a face are counted too: the naming layer reads them as bordering
/// no edge.
#[test]
fn every_fused_vertex_of_the_lattice_corpus_settles_on_a_live_cell() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
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
    topo::arm_output_stage_points();
    for x in spans {
        for y in spans {
            for z in spans {
                let b = finished("b", brick::<f64>(x, y, z, tol), tol);
                for op in ["union", "subtract", "intersect"] {
                    let who = format!("{op} with {x:?} × {y:?} × {z:?}");
                    topo::take_output_stage_points().expect("the row arms its thread");
                    let result = match op {
                        "union" => union(&a, &b, tol),
                        "subtract" => subtract(&a, &b, tol),
                        _ => intersect(&a, &b, tol),
                    }
                    .unwrap_or_else(|e| panic!("{who}: {e}"));
                    let at = output_stage_points(&who);
                    let BooleanResult::Body(out) = result else {
                        continue;
                    };
                    bodies += 1;
                    let body = &out.body;
                    let fusions = out.naming.vertex_merges.rows();
                    // The point a result vertex had when the output stage
                    // began: its survivor's, through every fusion.
                    let survivor = |v: VertexKey| {
                        fusions
                            .iter()
                            .fold(v, |at, &(dead, kept)| if at == dead { kept } else { at })
                    };
                    let point = |v: VertexKey| {
                        *at.get(&survivor(v))
                            .unwrap_or_else(|| panic!("{who}: no output-stage point for {v:?}"))
                    };
                    let fused = out
                        .naming
                        .fused_into(body)
                        .unwrap_or_else(|e| panic!("{who}: {e:?}"));
                    for (&v, &cell) in &fused {
                        assert!(
                            holds(body, cell, point(v), band),
                            "{who}: {v:?} settles on {cell:?}, which does not hold its point"
                        );
                        match cell {
                            Cell::Vertex(_) => vertices += 1,
                            Cell::Edge(_) => edges += 1,
                            Cell::Face(_) => faces += 1,
                        }
                    }
                    let settler = out.naming.settler(body);
                    for d in &out.naming.discards {
                        for &(u, w) in d.bordered.iter().chain(&d.bordered_through) {
                            let ends = [u, w].map(|v| {
                                let cell = settler
                                    .settle(v)
                                    .unwrap_or_else(|e| panic!("{who}: a bordered end: {e:?}"));
                                assert!(
                                    holds(body, cell, point(v), band),
                                    "{who}: bordered end {v:?} settles on {cell:?}, which does \
                                     not hold its point"
                                );
                                cell
                            });
                            stretches += 1;
                            if ends.iter().any(|c| !matches!(c, Cell::Vertex(_))) {
                                inside += 1;
                            }
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
    // Pinned red at main's count since PR 5724 met PR 5706 (was
    // (25 512, 1392)); whether the new discards are right is
    // `work/issues/main-red-the-lattice-discard-pin-moved-where-pred-5724-met-flush-5706`.
    assert_eq!(
        (stretches, inside),
        (26_292, 2004),
        "bordered stretches, and those with an end inside an edge or a face"
    );
}
