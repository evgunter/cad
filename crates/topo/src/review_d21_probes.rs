//! Consumer probes for **D21** — the discard sites outside W2c's
//! three-module census, converted to the D2 addendum's row 4.
//!
//! Each converted write states that its key cannot be stale. What no
//! row in the tree pinned before is the other half of that claim: that
//! the doors leading to those writes **refuse a stale key typed**, so
//! the `unreachable!` is not reachable through the public API. The
//! files these rows cover are the ones the brief named as uncovered:
//! `attach.rs` carried **no tests at all**, and `movefac.rs`'s only
//! stale-key row plants a stale *shell*, never a face the component
//! walk reaches.
//!
//! Each row is a pair: the typed refusal, and a deep-equal body — a
//! door that refuses after mutating would satisfy the first alone.
//!
//! The last row makes `revert`'s proof executable rather than argued.
//! Its three writes rest on one fact — a cloned `Body` resolves every
//! key of the original, because cloning a slotmap preserves its keys —
//! and that fact is asserted here over a real body rather than read off
//! the dependency's documentation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Vec3};

use crate::entity::{EntityId, FaceKey};
use crate::euler::FaceSurface;
use crate::fixtures::{deep_snapshot, ops_cube};
use crate::EulerOpError;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn a_plane() -> Surface<f64> {
    Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// `set_face_surface`'s plan phase refuses a stale face, so the write
/// it guards (`f.surface = new`) never runs on an unproven key.
#[test]
fn d21_set_face_surface_refuses_a_stale_face_typed() {
    let mut body = ops_cube().body;
    let before = deep_snapshot(&body);
    let err = body
        .set_face_surface(FaceKey::default(), FaceSurface::New(a_plane()))
        .unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Face(FaceKey::default()),
        },
        "set_face_surface must refuse a stale face typed, never panic"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "set_face_surface atomicity: the body must be untouched on Err"
    );
}

/// `set_edge_curve`'s plan phase refuses a stale edge, so the write it
/// guards (`e.curve = new`) never runs on an unproven key.
#[test]
fn d21_set_edge_curve_refuses_a_stale_edge_typed() {
    let mut body = ops_cube().body;
    let before = deep_snapshot(&body);
    let err = body
        .set_edge_curve(
            crate::entity::EdgeKey::default(),
            EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)),
        )
        .unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Edge(crate::entity::EdgeKey::default()),
        },
        "set_edge_curve must refuse a stale edge typed, never panic"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "set_edge_curve atomicity: the body must be untouched on Err"
    );
}

/// `split_edge`'s plan phase refuses a stale edge, so its mutation
/// phase's `get_edge_mut(edge)` never runs on an unproven key. The
/// sibling writes in the same phase (`w`, `v`) are minted and
/// plan-proven respectively; this row pins the one that is neither.
#[test]
fn d21_split_edge_refuses_a_stale_edge_typed() {
    let mut body = ops_cube().body;
    let before = deep_snapshot(&body);
    let err = body
        .split_edge(crate::entity::EdgeKey::default(), 0.5)
        .unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Edge(crate::entity::EdgeKey::default()),
        },
        "split_edge must refuse a stale edge typed, never panic"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "split_edge atomicity: the body must be untouched on Err"
    );
}

/// `movefac`'s component walk refuses a face of the shell list that no
/// longer resolves. The op's `face_data.shell = new_shell` write reads
/// its keys out of that walk's labelling, so the walk's own refusal is
/// what makes the write's `unreachable!` sound.
///
/// The existing `movefac_stale_shell_is_typed` plants a stale *shell*
/// and never reaches the walk; this plants a dead face inside a live
/// shell, which is the shape the walk is the only guard against.
#[test]
fn d21_movefac_refuses_a_dead_face_reached_by_the_walk_typed() {
    let cube = ops_cube();
    let mut body = cube.body;
    let shell = cube.seed.shell;
    let dead = body.get_shell(shell).unwrap().faces[0];
    // Raw arena removal (crate-internal): the shell's list keeps the
    // key, so only the walk's own lookup can catch it.
    body.faces.remove(dead);

    let before = deep_snapshot(&body);
    let err = body.movefac(shell).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Face(dead),
        },
        "movefac must refuse a dead face in the shell list typed, never panic"
    );
    assert_eq!(
        deep_snapshot(&body),
        before,
        "movefac atomicity: the body must be untouched on Err"
    );
}

/// `revert`'s three writes rest on one fact: `out = self.clone()`
/// resolves every key iterated out of `self`. Asserted here over a real
/// body rather than argued from the slotmap crate's documentation.
#[test]
fn d21_a_cloned_body_resolves_every_key_of_the_original() {
    let body = ops_cube().body;
    let out = body.clone();
    assert!(!body.half_edges.is_empty(), "fixture must exercise the walk");
    for (he_key, _) in body.half_edges.iter() {
        assert!(
            out.get_half_edge(he_key).is_some(),
            "clone dropped half-edge {he_key:?}"
        );
    }
    for (edge_key, _) in body.edges.iter() {
        assert!(
            out.get_edge(edge_key).is_some(),
            "clone dropped edge {edge_key:?}"
        );
    }
    for (vertex_key, _) in body.vertices.iter() {
        assert!(
            out.get_vertex(vertex_key).is_some(),
            "clone dropped vertex {vertex_key:?}"
        );
    }
    // And the op the fact serves still runs clean on the same body.
    assert!(body.revert().is_ok(), "revert must accept a tier-1 body");
}
