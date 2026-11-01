//! **The pieces of a line edge lying in a planar face, at the sweep**
//! ([`super::settle_in_plane`]).
//!
//! The poses are an edge in a wall: a triangular prism's apex edge on
//! `y = 0.5`, beside a brick over `y ∈ [0.5, 1.5]` whose face `y = 0.5`
//! holds part of the apex, a face of the brick crossing the apex on that
//! face's edge; and an edge of a lid brick lying in the top of a
//! U-shaped prism, over the notch whose floor stands within a band of
//! the edge's midpoint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{
    ContactAcc, InPlane, PendingRow, SweepKnobs, SweepStrategy, settle_in_plane, sweep_and_settle,
};
use crate::boolean::{
    Cell, EfContact, GluedPairs, Operand, ReductionContacts, VfContact, VvContact,
};
use crate::test_support_fixtures::{brick, prism_z};
use crate::{Backing, Body, EdgeKey, FaceKey, VertexKey};
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// The face of `body` whose plane has outward normal `n`.
fn face_along(body: &Body<f64>, n: [f64; 3]) -> FaceKey {
    let hits: Vec<FaceKey> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&f| {
            super::face_plane(body, f)
                .is_some_and(|p| p.normal.x * n[0] + p.normal.y * n[1] + p.normal.z * n[2] > 0.99)
        })
        .collect();
    assert_eq!(hits.len(), 1, "one face along {n:?}");
    hits[0]
}

/// The edge of `body` from `p` to `q`, either way round.
fn edge_at(body: &Body<f64>, p: Point3<f64>, q: Point3<f64>) -> EdgeKey {
    body.edges()
        .map(|(k, _)| k)
        .find(|&e| {
            let (u, v) = body.edge_vertices(e).unwrap();
            let (pu, pv) = (point(body, u), point(body, v));
            let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-12;
            (near(pu, p) && near(pv, q)) || (near(pu, q) && near(pv, p))
        })
        .expect("the edge is in the body")
}

/// Both directions and the settle, on clones, over the records `seed`
/// writes first: the swept clones, the records and the decisions they
/// cite.
fn sweep(
    a: &Body<f64>,
    b: &Body<f64>,
    seed: impl FnOnce(&mut ContactAcc),
) -> ([Body<f64>; 2], ReductionContacts, Vec<PendingRow>) {
    let tips = crate::sector_shape::SlitTips::of_operands(a, b);
    let (mut x, mut y) = (a.clone(), b.clone());
    let mut acc = ContactAcc::default();
    seed(&mut acc);
    sweep_and_settle(
        (&mut x, &mut y),
        &tips,
        &GluedPairs::default(),
        &mut acc,
        band(),
        SweepStrategy::Realized,
        [&SweepKnobs::default(); 2],
        [None, None],
        Tol::witness(),
    )
    .expect("the sweep runs");
    let (records, pending) = acc.finish();
    ([x, y], records, pending)
}

/// **Each edge-face record cites the decisions at its own piece's two
/// ends, and no others**, in either operand order. The apex is
/// recorded once, the piece the wall face holds; every decision behind
/// that record names an end of that piece, never the apex's end off the
/// face. Upright, the brick's bottom face splits the apex before the
/// wall face reads it; laid along `x`, the wall face reads the whole
/// apex before the brick's end face splits it.
///
/// Each apex vertex on the wall's plane also carries decoys written
/// before the sweep, a record on the wall's far face and a tie with a
/// wall vertex there, which the record must not cite.
///
/// Mutants: citing the decisions at the ends of the fragment the wall
/// face read names the laid apex's far end; citing an end's record on
/// another face of the wall, or its tie with a wall vertex off the
/// face, names a decoy.
#[test]
fn each_edge_face_record_cites_its_own_pieces_ends() {
    let tol = Tol::witness();
    let tri = [(0.5, 0.0), (0.0, 0.5), (-0.5, 0.0)];
    let laid = |z0, z1| {
        let along_x = geom_core::Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::unit_y(),
            core::f64::consts::FRAC_PI_2,
        );
        crate::transform::transform_rigid(
            &prism_z::<f64>(&tri, z0, z1, tol).body,
            &geom_core::test_support::rigid(&along_x, tol),
            tol,
        )
        .unwrap()
    };
    let p = Point3::new;
    let poses = [
        (
            "upright",
            prism_z::<f64>(&tri, 0.6, 1.3, tol).body,
            brick((-1.0, 1.0), (0.5, 1.5), (1.0, 2.0), tol),
            [p(0.0, 0.5, 1.0), p(0.0, 0.5, 1.3)],
        ),
        (
            "laid out past x = 1",
            laid(0.5, 1.5),
            brick((-1.0, 1.0), (0.5, 1.5), (-1.0, 1.0), tol),
            [p(0.5, 0.5, 0.0), p(1.0, 0.5, 0.0)],
        ),
        (
            "laid out past x = -1",
            laid(-1.5, -0.5),
            brick((-1.0, 1.0), (0.5, 1.5), (-1.0, 1.0), tol),
            [p(-1.0, 0.5, 0.0), p(-0.5, 0.5, 0.0)],
        ),
    ];
    for (pose, apex, wall, ends) in &poses {
        for (x_is, a, b) in [(Operand::A, apex, wall), (Operand::B, wall, apex)] {
            let what = format!("{pose}, apex {x_is:?}");
            let far = face_along(wall, [0.0, 1.0, 0.0]);
            let off = wall
                .vertices()
                .map(|(k, _)| k)
                .find(|&k| point(wall, k).y > 1.0)
                .unwrap();
            let decoys = |acc: &mut ContactAcc| {
                let on_plane = apex.vertices().map(|(k, _)| k);
                for w in on_plane.filter(|&w| (point(apex, w).y - 0.5).abs() < 1e-12) {
                    let mine = (x_is, Cell::Vertex(w));
                    let vf = VfContact {
                        vertex: w,
                        face: far,
                    };
                    acc.vf(
                        x_is,
                        vf,
                        PendingRow::on(mine, (x_is.other(), Cell::Face(far)), None),
                    );
                    let vv = match x_is {
                        Operand::A => VvContact { a: w, b: off },
                        Operand::B => VvContact { a: off, b: w },
                    };
                    acc.vv(
                        vv,
                        PendingRow::on(mine, (x_is.other(), Cell::Vertex(off)), None),
                    );
                }
            };
            let ([sa, sb], records, pending) = sweep(a, b, decoys);
            let (x, y, list, other) = match x_is {
                Operand::A => (&sa, &sb, &records.a_in_b, &records.b_in_a),
                Operand::B => (&sb, &sa, &records.b_in_a, &records.a_in_b),
            };
            assert!(
                other.is_empty(),
                "{what}: the wall lies in no face of the apex: {other:?}"
            );
            let face = face_along(y, [0.0, -1.0, 0.0]);
            let [c] = list.as_slice() else {
                panic!("{what}: one piece of the apex is recorded: {list:?}");
            };
            assert_eq!(c.face, face, "{what}: the piece lies in the wall face");
            let (u, v) = x.edge_vertices(c.edge).unwrap();
            let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-12;
            let (pu, pv) = (point(x, u), point(x, v));
            assert!(
                (near(pu, ends[0]) && near(pv, ends[1]))
                    || (near(pu, ends[1]) && near(pv, ends[0])),
                "{what}: the piece runs {ends:?}: {pu:?} {pv:?}"
            );
            let piece_end = |cell| cell == Cell::Vertex(u) || cell == Cell::Vertex(v);
            let bounding = super::face_vertices(y, face).unwrap();
            let on_face = |cell| match cell {
                Cell::Face(f) => f == face,
                Cell::Vertex(w) => bounding.contains(&w),
                _ => false,
            };
            for backing in c.cites.iter() {
                let Backing::Decided(i) = backing else {
                    panic!("{what}: the sweep cites its own decisions: {backing:?}");
                };
                let row = pending[usize::try_from(i).unwrap()];
                let cells = |side: Operand| -> Vec<Cell> {
                    row.cells
                        .iter()
                        .filter(|&&(o, _)| o == side)
                        .map(|&(_, cell)| cell)
                        .collect()
                };
                let (mine, theirs) = (cells(x_is), cells(x_is.other()));
                assert!(
                    !mine.is_empty() && mine.into_iter().all(piece_end),
                    "{what}: cited row {i} names the apex at the piece's ends only: {row:?}"
                );
                assert!(
                    !theirs.is_empty() && theirs.into_iter().all(on_face),
                    "{what}: cited row {i} names the wall at the face or a vertex bounding it: {row:?}"
                );
            }
        }
    }
}

/// **An end recorded inside the face holds its piece without its
/// midpoint being read.** The face is the top of a U whose notch floor
/// stands half a band past the edge's midpoint, so that midpoint reads
/// in band of the floor; both ends are recorded strictly inside the face
/// (as the sweep would have recorded them), and the piece is recorded.
///
/// Mutant: holding a piece by its midpoint alone loses the record.
#[test]
fn an_end_resting_in_the_face_holds_its_piece() {
    let tol = Tol::witness();
    let d = 0.5 * band().zero();
    let u: Body<f64> = prism_z(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.5 + d),
            (1.0, 1.5 + d),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        -1.0,
        0.0,
        tol,
    )
    .body;
    let lid: Body<f64> = brick((0.5, 2.5), (0.0, 1.5), (0.0, 1.0), tol);
    let top = face_along(&u, [0.0, 0.0, 1.0]);
    let edge = edge_at(&lid, Point3::new(0.5, 1.5, 0.0), Point3::new(2.5, 1.5, 0.0));
    let (w0, w1) = lid.edge_vertices(edge).unwrap();
    let mut acc = ContactAcc::default();
    for w in [w0, w1] {
        acc.vf(
            Operand::A,
            VfContact {
                vertex: w,
                face: top,
            },
            PendingRow::on(
                (Operand::A, Cell::Vertex(w)),
                (Operand::B, Cell::Face(top)),
                None,
            ),
        );
    }
    acc.in_plane.push(InPlane {
        x_is: Operand::A,
        fragment: edge,
        mark: acc.splits.mark(Operand::A),
        face: top,
    });
    settle_in_plane([&lid, &u], &mut acc, band()).expect("the settle reads no midpoint");
    let (records, _) = acc.finish();
    assert_eq!(
        records.a_in_b,
        [EfContact { edge, face: top }],
        "the edge is recorded in the U's top"
    );
}
