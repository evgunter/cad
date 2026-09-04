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

use editor_core::persist::{load, save};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, DocEdit, EditError, EntityKey, EntityKind, Entry,
    EvalOptions, Expr, InterrogateError, Node, NodeError, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, ResolveError, RoleSeg, SlotId, StableName, ValuePayload,
    all_edges, all_faces, apply, edge_frame, evaluate, face_carrier_kind, face_frame,
};
use geom_brep::SurfaceKind;
use geom_core::{Tol, Vec3};
use topo::readback;
use topo::{DatumValue, UnitVec3};

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

// ---------------------------------------------------------------------
// DOCM-1 items 1, 2 and 5: the derived frame itself.
// ---------------------------------------------------------------------

fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("an angle literal")
}

/// The frame value a node landed, as (origin, u, v).
fn frame_of(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) =
        &ev.value(node).expect("the frame evaluated").payload
    else {
        panic!("a frame value");
    };
    (
        *origin - geom_core::Point3::origin(),
        UnitVec3::get(*u),
        UnitVec3::get(*v),
    )
}

fn near3(a: Vec3<f64>, b: Vec3<f64>, what: &str) {
    for (x, y) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
        assert!((x - y).abs() <= 1e-12, "{what}: {a:?} vs {b:?}");
    }
}

/// The face name of `node`'s face whose arena key is `key`.
fn name_of_key(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    key: EntityKey,
) -> StableName {
    let v = ev.value(node).expect("evaluated");
    all_faces(ev, node)
        .into_iter()
        .find(|n| matches!(v.name_table.lookup(n), Some(Entry::Unique(e)) if e.key == key))
        .expect("every face of a corpus body is named")
}

fn face_frame_node(at: RecipeNodeId, face: StableName, spin: f64) -> Node<ProfileProgram> {
    Node::Datum(Datum::FaceFrame {
        at,
        face,
        spin: ang(spin),
    })
}

fn top_cap(cube: RecipeNodeId) -> StableName {
    fixture::fname(cube, RoleSeg::Cap(CapEnd::End))
}

/// **A1 — derived, not frozen.** The corpus document's own probe:
/// raise the box, and the frame, the profile and the boss ride up
/// with the face; the box's untouched siblings are served from the
/// memo, the frame's cone recomputes.
#[test]
fn a1_the_frame_moves_with_the_face_and_the_memo_recomputes_the_cone() {
    let cd = corpus::face_sketch::document();
    let boss = cd.result.expect("the boss");
    let ev = eval(&cd.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let frame = cd
        .doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(cd.doc.node(*id), Some(Node::Datum(Datum::FaceFrame { .. }))))
        .expect("the derived frame");
    let (o, u, v) = frame_of(&ev, frame);
    near3(o, Vec3::new(0.0, 0.0, 1.0), "origin on the top cap");
    near3(u.cross(v), Vec3::new(0.0, 0.0, 1.0), "normal outward (+z)");
    let boss_bottom = |ev: &editor_core::Evaluation<f64>| {
        let body = corpus::body_of(ev, boss);
        body.points()
            .map(|(_, p)| p.z)
            .fold(f64::INFINITY, f64::min)
    };
    assert!((boss_bottom(&ev) - 1.0).abs() <= 1e-12);

    let bumped = cd.bumped();
    let ev2 = evaluate::<f64>(
        &bumped,
        Some(&ev),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        corpus::failures(&ev2).is_empty(),
        "{:?}",
        corpus::failures(&ev2)
    );
    let (o2, _, _) = frame_of(&ev2, frame);
    near3(o2, Vec3::new(0.0, 0.0, 1.5), "the frame followed the face");
    assert!(
        (boss_bottom(&ev2) - 1.5).abs() <= 1e-12,
        "the boss followed the frame"
    );
    let cone = corpus::cone(&bumped, cd.bump_root);
    assert!(cone.contains(&frame), "the frame is in the box's cone");
    assert_eq!(ev2.recomputed, cone.len(), "exactly the cone recomputed");
    assert_eq!(ev2.reused, bumped.len() - cone.len(), "the siblings reused");
}

/// Every planar face of a body, as (name, stored sense, pose).
fn planar_faces(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> Vec<(StableName, bool, readback::Pose<f64>)> {
    let v = ev.value(node).expect("evaluated");
    let ValuePayload::Body(body) = &v.payload else {
        panic!("a body");
    };
    body.faces()
        .filter(|(k, _)| readback::face_carrier_kind(body, *k) == Ok(SurfaceKind::Plane))
        .map(|(k, f)| {
            let pose = readback::face_pose(body, k).expect("a plane has a pose");
            (name_of_key(ev, node, EntityKey::Face(k)), f.sense, pose)
        })
        .collect()
}

/// **A2 — the normal is outward.** On every planar face of the washer
/// (its two annuli are minted with OPPOSITE senses — the under-side
/// one is an inward wall) and of a box, the frame's `u × v` is
/// `sense · axis` against `face_pose`'s own axis and sense; a row per
/// sense, and both senses seen.
#[test]
fn a2_the_normal_is_sense_times_axis_on_both_senses() {
    let mut seen = [false, false];
    for (doc, node) in [washer_doc(), box_doc()] {
        let ev = eval(&doc);
        for (name, sense, pose) in planar_faces(&ev, node) {
            let (doc2, frame) = fixture::insert(doc.clone(), face_frame_node(node, name, 0.0));
            let ev2 = eval(&doc2);
            let (o, u, v) = frame_of(&ev2, frame);
            assert_eq!(pose.sense, sense);
            let expected = if sense { pose.axis } else { -pose.axis };
            near3(u.cross(v), expected, "n = sense · axis");
            near3(
                o,
                pose.origin - geom_core::Point3::origin(),
                "origin is the carrier's",
            );
            seen[usize::from(sense)] = true;
        }
    }
    assert_eq!(seen, [true, true], "a row per sense");
}

/// **A3 — spin.** `u` is the carrier's u-reference rotated by θ about
/// the OUTWARD normal, `v = n × u` right-handed; `spin` is a
/// continuous Angle slot that `SetParam` reaches, `slots()` lists,
/// and the dimension check refuses a length in.
#[test]
fn a3_spin_rotates_about_the_outward_normal_and_is_a_continuous_angle_slot() {
    let (doc, cube) = box_doc();
    let ev = eval(&doc);
    let theta = 0.7;
    let (doc, frame) = fixture::insert(doc, face_frame_node(cube, top_cap(cube), theta));
    let ev2 = eval(&doc);
    let pose = face_frame(&ev, cube, &top_cap(cube)).expect("the top cap");
    let n = if pose.sense { pose.axis } else { -pose.axis };
    let u_ref = pose.u_ref.expect("a plane fixes u_ref");
    let expected_u = u_ref * theta.cos() + n.cross(u_ref) * theta.sin();
    let (_, u, v) = frame_of(&ev2, frame);
    near3(u, expected_u, "u = rotate(u_ref, n, θ)");
    near3(v, n.cross(u), "v = n × u");
    near3(u.cross(v), n, "right-handed about n");

    let node = doc.node(frame).expect("live");
    assert_eq!(node.slots(), vec![SlotId::Spin]);
    assert_eq!(SlotId::Spin.dimension(), Dimension::Angle);
    assert!(!SlotId::Spin.is_structural());
    let set = |expr: Expr| {
        apply(
            &doc,
            &DocEdit::SetParam {
                node: frame,
                slot: SlotId::Spin,
                expr,
            },
            Tol::witness(),
        )
    };
    let turned = set(ang(-theta)).expect("an angle goes in").doc;
    let (_, u3, _) = frame_of(&eval(&turned), frame);
    near3(
        u3,
        u_ref * theta.cos() - n.cross(u_ref) * theta.sin(),
        "SetParam reached the spin",
    );
    assert!(matches!(
        set(len(1.0)),
        Err(EditError::SlotDimensionMismatch { .. })
    ));
}

/// **A4 — the fillet's failure mode.** `Rebind` the face to a name the
/// table lacks: the frame refuses `FaceFrameResolve { Vanished }`,
/// the profile and the extrude above it are POISONED through the
/// frame, never re-anchored; `Rebind` to a live face repairs all of it.
#[test]
fn a4_a_vanished_face_fails_the_frame_typed_and_poisons_the_sketch_and_rebind_repairs() {
    let (doc, cube) = box_doc();
    let (doc, frame) = fixture::insert(doc, face_frame_node(cube, top_cap(cube), 0.0));
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.5, 0.5, 0.2)])),
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.3),
        },
    );
    assert!(corpus::failures(&eval(&doc)).is_empty());

    // A lateral face a 4-gon does not have: the name is well-formed,
    // its node is live, and the table lacks it — N5's `Vanished`.
    let gone = fixture::fname(cube, fixture::wall(7));
    let rebind = |doc: &ProfileDoc, from: StableName, to: StableName| {
        apply(doc, &DocEdit::Rebind { from, to }, Tol::witness())
            .expect("a rebind between face names applies")
            .doc
    };
    let broken = rebind(&doc, top_cap(cube), gone.clone());
    let ev = eval(&broken);
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(NodeError {
            kind: NodeErrorKind::FaceFrameResolve { error },
            ..
        })) => assert!(
            matches!(**error, ResolveError::Vanished { .. }),
            "the N5 arm the situation warrants: {error}"
        ),
        other => panic!("the frame must fail typed, got {other:?}"),
    }
    for above in [profile, boss] {
        assert!(
            matches!(ev.nodes.get(&above), Some(NodeResult::Poisoned { through }) if *through == frame),
            "node {} is poisoned through the frame",
            above.0
        );
    }

    let repaired = rebind(&broken, gone, top_cap(cube));
    assert!(
        corpus::failures(&eval(&repaired)).is_empty(),
        "Rebind to a live face repairs"
    );
}

/// **A5 — non-planar refuses typed**, naming the carrier kind: the
/// washer's band is a cylinder, the pip ball a sphere.
#[test]
fn a5_a_non_planar_face_refuses_naming_its_carrier() {
    for ((doc, node), wanted) in [
        (washer_doc(), SurfaceKind::Cylinder),
        (ball_doc(), SurfaceKind::Sphere),
    ] {
        let ev = eval(&doc);
        let curved = all_faces(&ev, node)
            .into_iter()
            .find(|n| face_carrier_kind(&ev, node, n) == Ok(wanted))
            .expect("a face of the wanted kind");
        let (doc, frame) = fixture::insert(doc, face_frame_node(node, curved, 0.0));
        let ev = eval(&doc);
        assert!(
            matches!(
                ev.nodes.get(&frame),
                Some(NodeResult::Failed(NodeError {
                    kind: NodeErrorKind::FaceFrameNotPlanar { carrier },
                    ..
                })) if *carrier == wanted
            ),
            "{wanted:?}: {:?}",
            ev.nodes.get(&frame)
        );
    }
}

/// **A7 — by value.** A derived frame is the plane of a profile AND
/// the `plane` of an `AxisInPlane`, and a revolve about that axis
/// evaluates to a closed body sitting on the box's top face.
#[test]
fn a7_a_derived_frame_serves_a_profile_and_an_in_plane_axis_by_value() {
    let (doc, cube) = box_doc();
    let (doc, frame) = fixture::insert(doc, face_frame_node(cube, top_cap(cube), 0.0));
    // A square off the axis, so the revolve is a washer standing on
    // the top cap.
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.35, 0.0, 0.1)])),
    );
    let (doc, axis) = fixture::insert(doc, fixture::axis_in_plane(frame, (0.0, 0.0), (0.0, 1.0)));
    let (doc, ring) = fixture::insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    let ev = eval(&doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let body = corpus::body_of(&ev, ring);
    assert_eq!(topo::validate_closed(body), Ok(()));
    // The revolve's axis is the frame's +y = the cap's v, lying IN the
    // cap plane at z = 1: every point of the ring has z in [1 - 0.1,
    // 1 + 0.1].
    for (_, p) in body.points() {
        assert!((p.z - 1.0).abs() <= 0.1 + 1e-9, "{p:?}");
    }
}

/// **A8 — wire.** A document carrying a derived frame saves, loads and
/// replays bit-identical, and the loaded document carries the same
/// `FaceFrame` node.
#[test]
fn a8_a_document_with_a_derived_frame_round_trips_bit_identical() {
    let cd = corpus::face_sketch::document();
    let text = save(
        &ProfileDoc::empty_derived("mod", Tol::witness()),
        &cd.edits,
        Tol::witness(),
    )
    .expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    let again = save(&loaded.snapshot, &loaded.edits, Tol::witness()).expect("re-save");
    assert_eq!(text, again, "save ∘ load is a fixpoint, byte for byte");
    let frames: Vec<_> = loaded
        .doc
        .order()
        .iter()
        .filter_map(|id| match loaded.doc.node(*id) {
            Some(Node::Datum(Datum::FaceFrame { at, face, .. })) => Some((*at, face.clone())),
            _ => None,
        })
        .collect();
    let original: Vec<_> = cd
        .doc
        .order()
        .iter()
        .filter_map(|id| match cd.doc.node(*id) {
            Some(Node::Datum(Datum::FaceFrame { at, face, .. })) => Some((*at, face.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(frames, original);
    assert_eq!(frames.len(), 1);
    assert!(corpus::failures(&eval(&loaded.doc)).is_empty());
}

/// **DM1c at f64 — a loft SECTION on a derived frame evaluates** on
/// the f64 lane, placed through the same by-value read (the Interval
/// half of the row is in the interval suite).
#[test]
fn dm1c_a_section_on_a_derived_frame_evaluates_at_f64() {
    let (doc, loft) = lofted_on_face_frame();
    let ev = eval(&doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let body = corpus::body_of(&ev, loft);
    assert_eq!(topo::validate_closed(body), Ok(()));
}

/// A loft whose LOWER section is drawn on a frame derived from a box's
/// top face, skinning up to a smaller square on an authored frame 0.5
/// above the cap.
pub(crate) fn lofted_on_face_frame() -> (ProfileDoc, RecipeNodeId) {
    let (doc, cube) = box_doc();
    let (doc, frame) = fixture::insert(doc, face_frame_node(cube, top_cap(cube), 0.0));
    let (doc, lower) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.5, 0.5, 0.4)])),
    );
    let (doc, upper) = fixture::on_frame(
        doc,
        [0.0, 0.0, 1.5],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.2)],
    );
    fixture::insert(
        doc,
        Node::Loft {
            profiles: vec![lower, upper],
            v_degree: Expr::count(1),
        },
    )
}
