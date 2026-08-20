//! Consumer probes for **D21** — the discard sites outside W2c's
//! three-module census, converted to the D2 addendum's row 4.
//!
//! Each converted write states that its key cannot be stale. These
//! rows pin the other half: that the door leading to the write
//! **refuses that key typed**, so the arm is not reachable through the
//! API.
//!
//! **What already existed, stated precisely because a looser version
//! of this sentence was wrong twice.** The attach doors DO have
//! error-path coverage, in `topo/tests/` and `sweep/tests/`, and
//! `review_m2_pr3.rs`'s setter row already pairs a typed refusal with
//! a body-untouched snapshot — including one on `set_face_surface`,
//! for a stale *surface* key (`StaleGeometry`, raised by
//! `check_face_surface`). What no row anywhere pinned is a stale
//! **entity argument** at these doors: the `get_face(face)` /
//! `get_edge(edge)` gates that guard the converted writes are a
//! different gate from the ones already covered, and reaching them
//! needs a stale face or edge rather than a stale surface. Likewise
//! `movefac`'s existing stale-key row plants a stale *shell* and
//! returns before the component walk; only a dead face inside a live
//! shell reaches the walk's own `get_face`.
//!
//! Each row is a pair: the typed refusal, and a deep-equal body — a
//! door that refuses after mutating would satisfy the first alone.
//! Where a door takes an entity key, the row plants a **removed**
//! entity rather than a null key, so the slotmap generation check is
//! what refuses rather than the null slot.
//!
//! The last row makes `revert`'s proof executable rather than argued.
//! Its two remaining keyed writes rest on one fact — a cloned `Body`
//! resolves every key of the original, because cloning a slotmap
//! preserves its keys — and that fact is asserted here over a real
//! body rather than read off the dependency's documentation.
//!
//! **What these rows do NOT cover, so nobody infers a full set.**
//! Five of D21's seventeen sites have no door-level row here:
//! `boolean/combine.rs`'s two and `splitting/finish.rs`'s one sit in
//! `pub(crate)` helpers whose keys are minted by the same call, so
//! there is no door to hand a stale key to; and
//! `merge_coplanar_faces`' two are reached only through a merge that
//! has already staged a clone. Those five rest on their per-site
//! proofs and on the inversion evidence in the PR, not on a row here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Vec3};

use crate::EulerOpError;
use crate::entity::EntityId;
use crate::euler::FaceSurface;
use crate::fixtures::{deep_snapshot, ops_cube};

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
    let cube = ops_cube();
    let mut body = cube.body;
    // A removed face, not a null key: the generation check is what
    // must refuse, and a null slot would not exercise it.
    let dead = cube.mefs[0].face;
    body.faces.remove(dead);
    let before = deep_snapshot(&body);
    let err = body
        .set_face_surface(dead, FaceSurface::New(a_plane()))
        .unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Face(dead),
        },
        "set_face_surface must refuse a removed face typed, never panic"
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
    let cube = ops_cube();
    let mut body = cube.body;
    let dead = cube.mevs[0].edge;
    body.edges.remove(dead);
    let before = deep_snapshot(&body);
    let err = body
        .set_edge_curve(
            dead,
            EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)),
        )
        .unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Edge(dead),
        },
        "set_edge_curve must refuse a removed edge typed, never panic"
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
    let cube = ops_cube();
    let mut body = cube.body;
    let dead = cube.mevs[0].edge;
    body.edges.remove(dead);
    let before = deep_snapshot(&body);
    let err = body.split_edge(dead, 0.5).unwrap_err();
    assert_eq!(
        err,
        EulerOpError::StaleKey {
            key: EntityId::Edge(dead),
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

/// `revert`'s two keyed writes rest on one fact: `out = self.clone()`
/// resolves every key iterated out of `self`. Asserted here over a real
/// body rather than argued from the slotmap crate's documentation. The
/// edge arena is checked too even though its loop no longer looks
/// anything up — the fact is the same one, and a regression there
/// would resurface as a wrong key elsewhere.
#[test]
fn d21_a_cloned_body_resolves_every_key_of_the_original() {
    let body = ops_cube().body;
    let out = body.clone();
    assert!(
        !body.half_edges.is_empty(),
        "fixture must exercise the walk"
    );
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
