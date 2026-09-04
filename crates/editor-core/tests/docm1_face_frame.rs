//! **DOCM-1 — the sense beside the pose and the carrier-kind read, at
//! the name door** (DOCM-REFERENCES-DESIGN DM1a, DM2).
//!
//! The kernel half is pinned in `topo`'s own suite; what is pinned
//! HERE is the document-layer twin: the `StableName` door answers
//! exactly what the arena-key door answers on the same face, the
//! refusal ladder is `face_frame`'s, and the pose's sense matches the
//! stored flag on EVERY face of every corpus body — the row that would
//! catch a door copying the wrong bit on a face kind the seed-face
//! rows never mint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, Dimension, EntityKey, EntityKind, Entry, EvalOptions, Expr, InterrogateError,
    Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, StableName, ValuePayload,
    all_edges, all_faces, edge_frame, evaluate, face_carrier_kind, face_frame,
};
use geom_brep::SurfaceKind;
use geom_core::Tol;
use topo::readback;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit box as an extruded square, and its extrude node.
fn box_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("docm1_box", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

/// A washer: a rectangle off the axis, revolved a full turn — two
/// cylinder bands and two plane annuli.
fn washer_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("docm1_washer", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = fixture::insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    fixture::insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: fixture::ang(std::f64::consts::TAU),
        },
    )
}

/// The die corpus's pip ball: one exact half-disc meridian revolved a
/// full turn — a sphere.
fn ball_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm1_ball", Tol::witness());
    let (doc, plane) = fixture::insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let (doc, axis) = fixture::insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![corpus::die_pips::half_disc_program()],
        }),
    );
    fixture::insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: fixture::ang(std::f64::consts::TAU),
        },
    )
}

/// The kernel door's answer for `name`, reached the way the name
/// table would — the arena key stays inside this function.
fn kernel_kind(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    name: &StableName,
) -> SurfaceKind {
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("the node evaluated");
    };
    let Some(Entry::Unique(e)) = v.name_table.lookup(name) else {
        panic!("a unique face");
    };
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body value");
    };
    let EntityKey::Face(f) = e.key else {
        panic!("a face key");
    };
    readback::face_carrier_kind(body, f).expect("a live face")
}

/// **The two read doors agree, kind by kind.** A box is all planes; a
/// washer carries cylinders; a revolved half-disc is a sphere — and on
/// every face the `StableName` twin says what the arena-key door says.
#[test]
fn face_carrier_kind_answers_plane_cylinder_and_sphere_and_the_twin_agrees() {
    let expect = |doc: ProfileDoc, node: RecipeNodeId, wanted: SurfaceKind, label: &str| {
        let ev = eval(&doc);
        let faces = all_faces(&ev, node);
        assert!(!faces.is_empty(), "{label}: faces to read");
        let mut seen = false;
        for name in &faces {
            let kind = face_carrier_kind(&ev, node, name).expect("a live face");
            assert_eq!(kind, kernel_kind(&ev, node, name), "{label}: twin agrees");
            seen |= kind == wanted;
        }
        assert!(seen, "{label}: at least one {wanted:?} face");
        (ev, faces)
    };
    let (box_doc, cube) = box_doc();
    let (ev, faces) = expect(box_doc, cube, SurfaceKind::Plane, "box");
    for name in &faces {
        assert_eq!(
            face_carrier_kind(&ev, cube, name),
            Ok(SurfaceKind::Plane),
            "a box is all planes"
        );
    }
    let (washer, band) = washer_doc();
    expect(washer, band, SurfaceKind::Cylinder, "washer");
    let (ball, sphere) = ball_doc();
    expect(ball, sphere, SurfaceKind::Sphere, "ball");
}

/// **The refusal ladder is `face_frame`'s**: a name the table lacks
/// refuses `NoSuchName`, and an edge name refuses `WrongKind` naming
/// both kinds — through the same rungs, so a caller that already
/// handles `face_frame` handles this door.
#[test]
fn face_carrier_kind_walks_the_face_frame_ladder() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    let (other_doc, other_cube) = washer_doc();
    let other_ev = eval(&other_doc);
    let stale = all_faces(&other_ev, other_cube)
        .into_iter()
        .next()
        .expect("a washer face");
    assert_eq!(
        face_carrier_kind(&ev, cube, &stale),
        Err(InterrogateError::NoSuchName)
    );
    assert_eq!(
        face_carrier_kind(&ev, cube, &stale).map_err(|e| e.to_string()),
        face_frame(&ev, cube, &stale)
            .map(|_| SurfaceKind::Plane)
            .map_err(|e| e.to_string()),
        "same rung as face_frame"
    );
    let edge = all_edges(&ev, cube).into_iter().next().expect("a box edge");
    assert_eq!(
        face_carrier_kind(&ev, cube, &edge),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Face,
            found: EntityKind::Edge,
        })
    );
}

/// **The pose's sense is the stored flag on every face of every
/// corpus body**, and an edge's pose carries the identity sense. The
/// corpus is where faces minted `false` live (a revolve's inward
/// walls, extrude's concave arc walls), so the row asserts that both
/// senses were actually seen rather than trusting the seed-face rows
/// alone.
#[test]
fn face_frame_sense_matches_the_stored_flag_on_every_corpus_face() {
    let mut seen = [false, false];
    for cd in corpus::documents() {
        let ev = eval(&cd.doc);
        for (node, result) in &ev.nodes {
            let NodeResult::Ok(v) = result else { continue };
            let ValuePayload::Body(body) = &v.payload else {
                continue;
            };
            for (key, face) in body.faces() {
                match readback::face_pose(body, key) {
                    Ok(pose) => {
                        assert_eq!(
                            pose.sense, face.sense,
                            "{}: node {} face {key:?}",
                            cd.name, node.0
                        );
                        seen[usize::from(face.sense)] = true;
                    }
                    Err(readback::ReadbackError::NoCanonicalFrame { .. }) => {}
                    Err(other) => panic!("{}: {other}", cd.name),
                }
            }
            // The name door forwards the same bit for every named face.
            for name in all_faces(&ev, *node) {
                let Ok(pose) = face_frame(&ev, *node, &name) else {
                    continue;
                };
                let Some(Entry::Unique(e)) = v.name_table.lookup(&name) else {
                    panic!("a materialized name is unique");
                };
                let EntityKey::Face(f) = e.key else {
                    panic!("a face name")
                };
                assert_eq!(pose.sense, body.get_face(f).expect("live").sense);
            }
        }
    }
    assert_eq!(seen, [true, true], "both senses were seen in the corpus");

    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    for name in all_edges(&ev, cube) {
        assert!(edge_frame(&ev, cube, &name).expect("a line edge").sense);
    }
}

/// **Rule 1 says NUMERIC.** The sentence that listed "is this face
/// planar" among the refusals is gone from both statements of the
/// rule, and both say which predicates a door does not decide.
#[test]
fn rule_one_names_numeric_predicates_in_both_statements() {
    const KERNEL: &str = include_str!("../../topo/src/readback.rs");
    const DOOR: &str = include_str!("../src/names/interrogate.rs");
    let kernel_doc: String = KERNEL
        .lines()
        .take_while(|l| l.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    let door_doc: String = DOOR
        .lines()
        .take_while(|l| l.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    for (label, text) in [("readback.rs", &kernel_doc), ("interrogate.rs", &door_doc)] {
        assert!(
            text.contains("NUMERIC predicate"),
            "{label}: rule 1 names numeric predicates"
        );
        assert!(
            !text.contains("No door answers \"is this face planar\"")
                && !text.contains("(no door answers \"is this face planar\")"),
            "{label}: planarity is no longer listed among the refusals"
        );
        assert!(
            text.contains("face_carrier_kind"),
            "{label}: the tag read is named as the door that answers it"
        );
    }
}
