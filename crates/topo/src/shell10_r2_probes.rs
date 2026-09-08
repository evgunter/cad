//! **SHELL-10 review, R2 crate-internal probes.** Two things only the
//! crate's own corruption door and its private types can reach: the
//! asserting setters' PANIC through a scoped door whose out-of-scope
//! solid is malformed (the PR's stated reason acceptance row 2 stops
//! at the scope walk), and the third mutant the review tried —
//! `re_scope` as a plain `Vec` swap — through the one path that would
//! catch it, a re-scope UP.
//!
//! The helpers restate `offset_together::scope_walks`'s (private to
//! that module), verbatim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, SolidKey};
use crate::offset_together::{ChartMove, Scope, offset_planes_together};
use crate::splitting::reassembly::quad_prism;
use geom_core::{Affine3, Band, Tol, Vec3};

const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

fn two_boxes() -> (Body<f64>, SolidKey, SolidKey) {
    let tol = Tol::witness();
    let mut body = quad_prism(&SQUARE, 1.0, tol);
    let first = body.solids().next().unwrap().0;
    let other = quad_prism(&SQUARE, 1.0, tol);
    let placed = crate::transform_rigid(
        &other,
        &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)),
        tol,
    )
    .unwrap();
    let second = crate::graft_disjoint(&mut body, &placed, tol).unwrap();
    assert!(crate::validate::validate_closed(&body).is_ok());
    (body, first, second)
}

fn faces_of(body: &Body<f64>, solid: SolidKey) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, d)| body.get_shell(d.shell).unwrap().solid == solid)
        .map(|(k, _)| k)
        .collect()
}

fn moves_of(body: &Body<f64>, solid: SolidKey, distance: f64) -> Vec<ChartMove<f64>> {
    let mut out: Vec<(crate::geometry::SurfaceKey, Vec<FaceKey>)> = Vec::new();
    for face in faces_of(body, solid) {
        let key = body.get_face(face).unwrap().surface;
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => v.push(face),
            None => out.push((key, vec![face])),
        }
    }
    out.into_iter()
        .map(|(_, faces)| ChartMove { faces, distance })
        .collect()
}

fn break_a_loop(body: &mut Body<f64>, solid: SolidKey) {
    let face = faces_of(body, solid)[0];
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("a box face bounds a cycle");
    };
    body.get_half_edge_mut(first).unwrap().next = HalfEdgeKey::default();
}

/// **The door PANICS, it does not refuse**, on a body whose
/// out-of-scope solid is structurally malformed: the scope walk
/// accepts it (the PR's row), and the first `set_face_surface` runs the
/// whole-body tier-1 postcondition and asserts. Compiled into release
/// too (`debug-assertions = true` in the workspace profile).
#[test]
#[should_panic(expected = "postcondition: result is not tier-1 valid")]
fn r2_the_door_panics_in_the_first_setter_on_an_out_of_scope_malformed_solid() {
    let (mut body, first, second) = two_boxes();
    break_a_loop(&mut body, second);
    let moves = moves_of(&body, first, 0.0);
    let tol = Tol::witness();
    let mut work = body.clone();
    let _ = offset_planes_together(&mut work, &moves, Band::linear(tol).unwrap(), tol);
}

/// **A re-scope UP walks the difference** — the only path on which a
/// `re_scope` that merely swapped the `Vec` (the base's shape, the
/// review's third mutant) answers wrongly. No committed row reaches
/// it: the verb only ever re-scopes DOWN from `Scope::whole`.
#[test]
fn r2_a_re_scope_up_holds_the_solid_it_was_aimed_at() {
    let (body, first, second) = two_boxes();
    let mut scope = Scope::of_solids(&body, &[first]).unwrap();
    for f in faces_of(&body, second) {
        assert!(!scope.holds_face(f));
        assert_eq!(scope.solid_of(f), None);
    }
    scope
        .re_scope(&body, &[second])
        .expect("the walk of the difference");
    for f in faces_of(&body, second) {
        assert!(
            scope.holds_face(f),
            "a re-scope UP holds the new solid's faces"
        );
        assert_eq!(scope.solid_of(f), Some(second));
    }
    for f in faces_of(&body, first) {
        assert!(!scope.holds_face(f), "and no longer names the old one");
        assert_eq!(
            scope.solid_of(f),
            Some(first),
            "though its maps still hold it"
        );
    }
    assert_eq!(
        scope.faces_in_scope(&body).unwrap(),
        faces_of(&body, second)
    );
}
