//! **DOCM-1 — the read doors at f64**
//! (`crates/editor-core/REFERENCES.md` DM1, DM1a, DM1b, DM2): the read
//! doors' document-layer twins.
//!
//! The kernel half of the read doors is pinned in `topo`'s own suite;
//! what is pinned HERE is everything a document exercises — the
//! `StableName` door answers exactly what the arena-key door answers
//! on the same face and on the same edge, and the pose's sense matches
//! the stored flag on EVERY face of every corpus body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use crate::fixture::len;
use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, InterrogateError, Node, NodeResult,
    ProfileDoc, ProfileProgram, RecipeNodeId, StableName, ValuePayload, all_edges, all_faces,
    edge_carrier_kind, edge_frame, evaluate, face_carrier_kind, face_frame,
};
use geom::SurfaceKind;
use geom_core::Tol;
use topo::CurveKind;
use topo::readback;

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
            profile: p.into(),
            distance: len(1.0),
        },
    )
}

/// A washer: a rectangle off the axis, revolved a full turn — two
/// cylinder bands and two plane annuli.
fn washer_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("docm1_washer", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    fixture::insert(
        doc,
        fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    )
}

/// The die corpus's pip ball: one exact half-disc meridian revolved a
/// full turn — a sphere.
fn ball_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("docm1_ball", Tol::witness());
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![corpus::die_pips::half_disc_program()],
            ids: Vec::new(),
            on: None,
        }),
    );
    fixture::insert(
        doc,
        fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
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
    let Some(Entry::Unique(e)) = ev.table_at(node).expect("one table").lookup(name) else {
        panic!("a unique face");
    };
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body value");
    };
    let EntityKey::Face(f) = e.key else {
        panic!("a face key");
    };
    readback::face_carrier_kind(body.body(), f).expect("a live face")
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
            for (key, face) in body.body().faces() {
                match readback::face_pose(body.body(), key) {
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
                let Some(Entry::Unique(e)) = ev.table_at(*node).expect("one table").lookup(&name)
                else {
                    panic!("a materialized name is unique");
                };
                let EntityKey::Face(f) = e.key else {
                    panic!("a face name")
                };
                assert_eq!(pose.sense, body.body().get_face(f).expect("live").sense);
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

/// The kernel door's answer for an EDGE `name`, reached the way the
/// name table would — the arena key stays inside this function.
fn kernel_edge_kind(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    name: &StableName,
) -> CurveKind {
    let Some(NodeResult::Ok(v)) = ev.nodes.get(&node) else {
        panic!("the node evaluated");
    };
    let Some(Entry::Unique(e)) = ev.table_at(node).expect("one table").lookup(name) else {
        panic!("a unique edge");
    };
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body value");
    };
    let EntityKey::Edge(k) = e.key else {
        panic!("an edge key");
    };
    readback::edge_carrier_kind(body.body(), k).expect("a live edge")
}

/// **The edge-side kind door answers a named edge and walks the same
/// ladder.** Every edge of a box is straight, and the kernel door
/// asked with the arena key says what the name door says; a FACE name
/// through the edge door is `WrongKind` naming both kinds, and a name
/// the table lacks refuses at the rung `edge_frame` refuses at.
#[test]
fn edge_carrier_kind_answers_a_named_edge_and_walks_the_edge_frame_ladder() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    let edges = all_edges(&ev, cube);
    assert!(!edges.is_empty(), "edges to read");
    for name in &edges {
        assert_eq!(
            edge_carrier_kind(&ev, cube, name),
            Ok(CurveKind::Line),
            "a box is all straight edges"
        );
        assert_eq!(
            edge_carrier_kind(&ev, cube, name),
            Ok(kernel_edge_kind(&ev, cube, name)),
            "twin agrees"
        );
    }

    let face = all_faces(&ev, cube).into_iter().next().expect("a box face");
    assert_eq!(
        edge_carrier_kind(&ev, cube, &face),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Edge,
            found: EntityKind::Face,
        })
    );

    let (other_doc, other_cube) = washer_doc();
    let other_ev = eval(&other_doc);
    let stale = all_edges(&other_ev, other_cube)
        .into_iter()
        .next()
        .expect("a washer edge");
    assert_eq!(
        edge_carrier_kind(&ev, cube, &stale),
        Err(InterrogateError::NoSuchName)
    );
    assert_eq!(
        edge_carrier_kind(&ev, cube, &stale).map_err(|e| e.to_string()),
        edge_frame(&ev, cube, &stale)
            .map(|_| CurveKind::Line)
            .map_err(|e| e.to_string()),
        "same rung as edge_frame"
    );
}

/// **The door answers a CURVED carrier too, so the read is a read.**
///
/// The box fixture the row above uses is all straight edges, and the
/// arena-key door it compares against answers `Line` on every one of
/// them — so both of that row's positive assertions hold of a door
/// that returns `CurveKind::Line` and reads nothing at all. The washer
/// is a revolved rectangle: its edges include circles, and a door that
/// does not read cannot say so.
#[test]
fn edge_carrier_kind_answers_a_curved_carrier_not_only_the_box_lines() {
    let (doc, washer) = washer_doc();
    let ev = eval(&doc);
    let names = all_edges(&ev, washer);
    assert!(!names.is_empty(), "edges to read");

    let mut kinds: Vec<CurveKind> = names
        .iter()
        .map(|name| edge_carrier_kind(&ev, washer, name).expect("a live named edge"))
        .collect();
    kinds.sort_unstable();
    kinds.dedup();
    assert!(
        kinds.contains(&CurveKind::Circle),
        "a revolved rectangle has circular edges; got {kinds:?}"
    );

    for name in &names {
        assert_eq!(
            edge_carrier_kind(&ev, washer, name),
            Ok(kernel_edge_kind(&ev, washer, name)),
            "twin agrees off the all-line fixture too"
        );
    }
}

/// The leading `//!` block of a Rust file, read through the shared
/// reader's PROSE view.
///
/// [`test_utils::source::comments_only`] blanks code and literals and
/// keeps every comment byte, so the leading run of `//!` lines comes
/// back exactly as written — while a refusal sentence spelled in a
/// string literal can no longer answer for one spelled in the docs.
/// The `//!` line test is the selection the module doc sanctions for a
/// caller that means inner docs rather than any comment; it is not a
/// second lexer. This site's ledger row is in
/// `crates/test-utils/tests/reader_census.rs`.
fn module_prose(text: &str) -> String {
    test_utils::source::comments_only(text)
        .lines()
        .take_while(|l| l.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// **Rule 1 says NUMERIC.** The sentence that listed "is this face
/// planar" among the refusals is gone from both statements of the
/// rule, and both say which predicates a door does not decide.
#[test]
fn rule_one_names_numeric_predicates_in_both_statements() {
    const KERNEL: &str = include_str!("../../topo/src/readback.rs");
    const DOOR: &str = include_str!("../src/names/interrogate.rs");
    let kernel_doc = module_prose(KERNEL);
    let door_doc = module_prose(DOOR);
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
