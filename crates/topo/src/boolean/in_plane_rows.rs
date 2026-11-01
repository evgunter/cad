//! **The pieces of a line edge lying in a planar face, at the sweep**
//! ([`super::settle_in_plane`]).
//!
//! The pose is an edge in a wall: a triangular prism over
//! `z ∈ [0.6, 1.3]` whose apex edge runs up `x = 0, y = 0.5`, beside a
//! brick over `y ∈ [0.5, 1.5]`, `z ∈ [1, 2]` whose face `y = 0.5` holds
//! the apex from `z = 1` up. The brick's bottom face splits the apex at
//! `z = 1`, on the face's bottom edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{
    ContactAcc, InPlane, PendingRow, SweepKnobs, SweepStrategy, settle_in_plane, sweep_and_settle,
};
use crate::boolean::{Cell, EfContact, GluedPairs, Operand, ReductionContacts, VfContact};
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

/// Both directions and the settle, on clones: the swept clones, the
/// records and the decisions they cite.
fn sweep(a: &Body<f64>, b: &Body<f64>) -> ([Body<f64>; 2], ReductionContacts, Vec<PendingRow>) {
    let tips = crate::sector_shape::SlitTips::of_operands(a, b);
    let (mut x, mut y) = (a.clone(), b.clone());
    let mut acc = ContactAcc::default();
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
/// ends, and no others.** The apex is recorded once, its piece from
/// `z = 1` to `z = 1.3` in the wall face; the decisions behind that
/// record are the apex top's resting inside the face and the split
/// vertex's tie with the brick's, never one at the apex foot
/// `z = 0.6`, which lies off the face, in either operand order.
///
/// Mutant: citing the decisions at the ends of the fragment the arm
/// read (the foot and the top) names the foot.
#[test]
fn each_edge_face_record_cites_its_own_pieces_ends() {
    let apex: Body<f64> = prism_z(
        &[(0.5, 0.0), (0.0, 0.5), (-0.5, 0.0)],
        0.6,
        1.3,
        Tol::witness(),
    )
    .body;
    let wall: Body<f64> = brick((-1.0, 1.0), (0.5, 1.5), (1.0, 2.0), Tol::witness());
    for (x_is, a, b) in [(Operand::A, &apex, &wall), (Operand::B, &wall, &apex)] {
        let ([sa, sb], records, pending) = sweep(a, b);
        let (x, y, list, other) = match x_is {
            Operand::A => (&sa, &sb, &records.a_in_b, &records.b_in_a),
            Operand::B => (&sb, &sa, &records.b_in_a, &records.a_in_b),
        };
        assert!(
            other.is_empty(),
            "{x_is:?}: the wall lies in no face of the apex: {other:?}"
        );
        let face = face_along(y, [0.0, -1.0, 0.0]);
        let [c] = list.as_slice() else {
            panic!("{x_is:?}: one piece of the apex is recorded: {list:?}");
        };
        assert_eq!(c.face, face, "{x_is:?}: the piece lies in the wall face");
        let (u, v) = x.edge_vertices(c.edge).unwrap();
        let mut zs = [point(x, u).z, point(x, v).z];
        zs.sort_by(f64::total_cmp);
        assert!(
            (zs[0] - 1.0).abs() < 1e-12 && (zs[1] - 1.3).abs() < 1e-12,
            "{x_is:?}: the piece runs from the split to the apex top: {zs:?}"
        );
        for backing in c.cites.iter() {
            let Backing::Decided(i) = backing else {
                panic!("{x_is:?}: the sweep cites its own decisions: {backing:?}");
            };
            let row = pending[usize::try_from(i).unwrap()];
            assert!(
                row.cells
                    .iter()
                    .any(|&(o, cell)| o == x_is
                        && (cell == Cell::Vertex(u) || cell == Cell::Vertex(v))),
                "{x_is:?}: cited row {i} names an end of the piece: {row:?}"
            );
            for &(o, cell) in &row.cells {
                if o == x_is {
                    assert!(
                        cell == Cell::Vertex(u) || cell == Cell::Vertex(v),
                        "{x_is:?}: cited row {i} names nothing of the apex but the piece's ends: {row:?}"
                    );
                }
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
