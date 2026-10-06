//! **A shell the other operand's boundary does not cut takes its side
//! from its first point off that boundary** (`boolean::shell_witness`).
//!
//! The fixture is a block `b` lying inside the union of two blocks `a`
//! and `c`, flush with it on four sides: every vertex and every edge of
//! `b` lies on the accumulation's boundary, and only the interiors of
//! `b`'s two end faces lie strictly inside it. Both callers of the
//! witness are reached: the containment fallback, when `b` is a whole
//! operand, and `setopfinish`'s uncut-component probe, when `b` is one
//! lump of an operand whose other lump crosses the accumulation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::flush::{declare_all, find_flush_candidates};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, mass_properties, union_with};

/// The union of `a` and `b`, every flush pair the detector finds
/// declared.
fn declared_union(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Result<AtRestBody<f64>, BooleanError> {
    let tol = Tol::witness();
    let decls = declare_all(&find_flush_candidates(a, b, tol).expect("the flush detector decides"));
    match union_with(a, b, &decls, tol)? {
        BooleanResult::Body(body) => Ok(body.body),
        BooleanResult::Empty => panic!("a union of non-empty blocks cannot be empty"),
    }
}

/// `a` = x 0..1, `b` = x 0.5..1.5, `c` = x 0.8..2, all over y 0..1 and
/// z 0..1: each pair overlaps and is flush on the four y and z walls.
fn trio() -> [AtRestBody<f64>; 3] {
    let tol = Tol::witness();
    let unit = (0.0, 1.0);
    [
        finished("a", brick((0.0, 1.0), unit, unit, tol), tol),
        finished("b", brick((0.5, 1.5), unit, unit, tol), tol),
        finished("c", brick((0.8, 2.0), unit, unit, tol), tol),
    ]
}

fn assert_valid_with_volume(label: &str, body: &Body<f64>, volume: f64) {
    let tol = Tol::witness();
    let got = mass_properties(body, tol).unwrap().volume;
    assert!(
        (got - volume).abs() < 1e-9,
        "{label}: volume {got}, expected {volume}"
    );
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: the result is tier-3 valid"
    );
}

/// Every member order folds to the x 0..2 block. In `[a, c, b]` and
/// `[c, a, b]` the last member, `b`, lies inside the accumulation with
/// all eight vertices on its boundary: the end faces' interiors decide.
#[test]
fn every_member_order_of_a_flush_trio_fuses_to_the_accumulation() {
    let blocks = trio();
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let first = declared_union(&blocks[order[0]], &blocks[order[1]])
            .unwrap_or_else(|e| panic!("order {order:?}, first step: {e:?}"));
        let all = declared_union(&first, &blocks[order[2]])
            .unwrap_or_else(|e| panic!("order {order:?}, second step: {e:?}"));
        assert_valid_with_volume(&format!("order {order:?}"), &all, 2.0);
    }
}

/// `b` as one lump of an operand whose other lump `x` crosses the
/// accumulation's x = 2 wall: the join runs, and `b`'s shell is an uncut
/// component the finish classifies by the same witness. `x` adds its
/// 0.5 × 0.6 × 0.6 outside x = 2.
#[test]
fn an_uncut_lump_flush_inside_the_other_operand_takes_its_side_from_a_face() {
    let tol = Tol::witness();
    let [a, b, c] = trio();
    let x = finished(
        "x",
        brick::<f64>((1.8, 2.5), (0.2, 0.8), (0.2, 0.8), tol),
        tol,
    );
    let BooleanResult::Body(bx) = topo::union(&b, &x, tol).expect("two disjoint blocks") else {
        panic!("a union of non-empty blocks cannot be empty");
    };
    let ac = declared_union(&a, &c).expect("a ∪ c");
    let all = declared_union(&ac, &bx.body).expect("the uncut lump classifies");
    assert_valid_with_volume("(a ∪ c) ∪ (b ∪ x)", &all, 2.0 + 0.5 * 0.36);
}

/// The same body twice, every flush pair declared: every witness of
/// either operand lies on the other's boundary, and the declarations
/// settle each face with its twin, so the shell is `On` and one copy is
/// kept.
#[test]
fn one_body_twice_declared_is_one_copy() {
    let [a, _, _] = trio();
    let both = declared_union(&a, &a).expect("one body twice, declared, is answered");
    assert_valid_with_volume("a ∪ a", &both, 1.0);
    assert_eq!(both.shells().count(), 1, "one copy of the shell is kept");
}

/// The trio again along z with an L-shaped profile, so `b`'s end faces
/// are non-convex: the centroid of the triple at the reflex corner lies
/// OUTSIDE the face, and outside the accumulation. Each rotation of the
/// profile starts the face's loop at a different vertex, so one of them
/// offers that centroid first; only `point_in_face`'s certificate keeps
/// it from deciding `b` lies outside (union volume 9, an assembly).
#[test]
fn a_non_convex_end_face_offers_only_a_certified_interior_point() {
    let tol = Tol::witness();
    let l = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let a = finished("a", common::prism_z::<f64>(&l, 0.0, 1.0, tol).body, tol);
    let c = finished("c", common::prism_z::<f64>(&l, 0.8, 2.0, tol).body, tol);
    let ac = declared_union(&a, &c).expect("a ∪ c");
    for start in 0..l.len() {
        let profile: Vec<(f64, f64)> = l[start..].iter().chain(&l[..start]).copied().collect();
        let b = finished(
            "b",
            common::prism_z::<f64>(&profile, 0.5, 1.5, tol).body,
            tol,
        );
        let all = declared_union(&ac, &b)
            .unwrap_or_else(|e| panic!("profile from vertex {start}: {e:?}"));
        assert_valid_with_volume(&format!("profile from vertex {start}"), &all, 6.0);
    }
}
