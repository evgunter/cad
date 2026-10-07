//! **A vertex read by two sector passes refuses typed, in every op and
//! both operand orders.**
//!
//! A vertex-on-face pass hangs struts at its piercing vertex, so each
//! sector pass reads an orbit as its operand gave it only while a
//! piercing vertex pierces one face of the other solid and pairs with
//! none of its vertices. Where the other solid holds its own contact at
//! the piercing point, that fails, and the boolean refuses
//! `VertexReadTwice` before any pass writes.
//!
//! The rows, each at every pose of the scene:
//! - **the arches**: the plate's union with the union of three pyramids
//!   standing on their apexes at `MEET`, which keeps their apexes on
//!   its top without a vertex of the top there; against a fourth
//!   standing pyramid, whose apex pierces the top and pairs with the
//!   arches';
//! - **one standing pyramid**, the same with one arch: also the plate
//!   folded with the arches one at a time, at its second arch;
//! - **a prism through the top** (`meeting::wedge`) against the plate
//!   and one standing pyramid: the vertex the sweep mints on its edge
//!   at `MEET` pierces the top with runs on both sides, so its pass
//!   hangs struts there before the pair would read it;
//! - **two blocks in face contact**, one body built through the Euler
//!   doors, against the standing pyramid, whose apex pierces both
//!   blocks' faces at a point of their contact.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{
    MEET, PLATE, Pose, at, posed_box, posed_boxes, posed_prism, posed_pyramid, poses, wedge,
};
use geom_core::Tol;
use topo::{
    AtRestBody, BooleanError, BooleanResult, Operand, SectorRead, intersect, readback, subtract,
    union,
};

fn t() -> Tol {
    Tol::witness()
}

/// A pyramid standing on its apex at [`MEET`]: a horizontal base `rise`
/// above it, two corners at radius `r` 15° either side of `bearing`
/// (degrees) and one at `0.6 r` on it.
fn standing(bearing: f64, rise: f64, r: f64, pose: &Pose) -> AtRestBody<f64> {
    let corner = |d: f64, r: f64| {
        let (s, c) = (bearing + d).to_radians().sin_cos();
        [r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]), MEET[2] + rise]
    };
    // Counterclockwise seen from below, the apex's side.
    let base = [corner(15.0, r), corner(-15.0, r), corner(0.0, 0.6 * r)];
    posed_pyramid(&base, MEET, pose)
}

fn union_of(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) -> AtRestBody<f64> {
    match union(x, y, t()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    }
}

/// Every op on `(x, y)`, in both orders, refuses `VertexReadTwice` on
/// `x`'s vertex at [`MEET`], with a pierce for its first read and
/// `second` for its next. Where `minted`, the sweep mints that vertex,
/// splitting an edge of `x` there, so `x` does not hold it.
fn every_op(
    label: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
    pose: &Pose,
    second: &str,
    minted: bool,
) {
    let meet = at(pose.at(MEET));
    let at_meet = |body: &AtRestBody<f64>, v| at(readback::vertex_point(body, v).unwrap()) == meet;
    for (what, x_is, r) in [
        ("x − y", Operand::A, subtract(x, y, t())),
        ("y − x", Operand::B, subtract(y, x, t())),
        ("x ∪ y", Operand::A, union(x, y, t())),
        ("y ∪ x", Operand::B, union(y, x, t())),
        ("x ∩ y", Operand::A, intersect(x, y, t())),
        ("y ∩ x", Operand::B, intersect(y, x, t())),
    ] {
        let what = format!("{label}, {}, {what}", pose.label);
        match r {
            Err(BooleanError::VertexReadTwice {
                operand,
                vertex,
                reads: [SectorRead::Pierce(first), next],
            }) => {
                assert_eq!(operand, x_is, "{what}: the operand read twice");
                if minted {
                    assert!(
                        x.vertices().all(|(k, _)| k != vertex),
                        "{what}: the vertex read twice is minted"
                    );
                } else {
                    assert!(
                        at_meet(x, vertex),
                        "{what}: the vertex read twice is at MEET"
                    );
                }
                match (second, next) {
                    ("pair", SectorRead::Pair(partner)) => {
                        assert!(at_meet(y, partner), "{what}: its partner is at MEET");
                    }
                    ("pierce", SectorRead::Pierce(face)) => {
                        assert_ne!(first, face, "{what}: it pierces two faces");
                    }
                    _ => panic!("{what}: its second read is a {second}, got {next:?}"),
                }
            }
            other => panic!(
                "{what}: refuses VertexReadTwice with a pierce first, got {:?}",
                other.map(|_| ())
            ),
        }
    }
}

/// **A vertex that pierces a face and pairs with a vertex resting on
/// that face refuses typed**: the arches, one standing pyramid, and a
/// prism through the top beside it.
#[test]
fn a_vertex_piercing_a_face_and_paired_on_it_refuses_typed_in_every_op() {
    for pose in poses() {
        let plate = posed_box("the plate", PLATE, &pose);
        let cone = standing(240.0, 0.7, 0.5, &pose);
        let one = union_of(
            "the plate and one arch",
            &plate,
            &standing(60.0, 0.5, 0.4, &pose),
        );
        // The arches meet only at their apexes, so their union pairs
        // vertices and pierces nothing; folded onto the plate one at a
        // time instead, the second is the one-pyramid row.
        let [first, rest @ ..] = [60.0, 180.0, 300.0].map(|b| standing(b, 0.5, 0.4, &pose));
        let arches = rest
            .iter()
            .fold(first, |u, a| union_of("the arches", &u, a));
        let arches = union_of("the plate and the arches", &plate, &arches);
        every_op("the arches", &cone, &arches, &pose, "pair", false);
        every_op("one standing pyramid", &cone, &one, &pose, "pair", false);
        let prism = posed_prism(&wedge(200.0, 260.0, 0), &pose);
        every_op("a prism through the top", &prism, &one, &pose, "pair", true);
    }
}

/// **A vertex that pierces two faces of a body touching itself refuses
/// typed**: two blocks in face contact, one body.
#[test]
fn a_vertex_piercing_two_faces_refuses_typed_in_every_op() {
    for pose in poses() {
        let blocks = posed_boxes(
            "two blocks in face contact",
            &[PLATE, [(0.5, 2.5), (0.5, 1.5), (1.0, 1.5)]],
            &pose,
        );
        let cone = standing(240.0, 0.7, 0.5, &pose);
        every_op("two blocks", &cone, &blocks, &pose, "pierce", false);
    }
}
