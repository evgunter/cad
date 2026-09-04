//! DOCM-1 review lane R1 — probes on the frozen head `20f04189`.
//!
//! Each row names the claim it falsifies. None of these is the
//! implementer's fixture: a different body, a different edit, a spin
//! the implementer did not choose, a face made to VANISH by an edit
//! rather than by a `Rebind` to a name that never existed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{
    Axis3, BooleanOp, CancelToken, CapEnd, Datum, Dimension, DocEdit, EntityKind, Entry, EvalOptions, Expr,
    Node, NodeError, NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, ResolveError, RoleSeg,
    SlotId, StableName, ValuePayload, all_faces, apply, evaluate, face_frame,
};
use geom_brep::SurfaceKind;
use geom_core::{Tol, Vec3};
use topo::readback;
use topo::{DatumValue, UnitVec3};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("an angle literal")
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

fn eval_prior(doc: &ProfileDoc, prior: &editor_core::Evaluation<f64>) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        Some(prior),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn frame_of(ev: &editor_core::Evaluation<f64>, node: RecipeNodeId) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
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

/// A washer standing at height `z0`: rectangle (1..2) x (0..1) in the
/// xz frame at origin (0,0,z0), revolved a full turn about the frame's
/// +y (= world z). Returns (doc, the authored frame, the revolve).
fn washer_at(z0: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("r1_washer", Tol::witness()),
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = fixture::insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, rev) = fixture::insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    (doc, plane, rev)
}

/// Every planar face of `node` with its stored sense and its pose,
/// by name.
fn planar_faces(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> Vec<(StableName, bool, readback::Pose<f64>)> {
    let v = ev.value(node).expect("evaluated");
    let body = corpus::body_of(ev, node);
    all_faces(ev, node)
        .into_iter()
        .filter_map(|name| {
            let Some(Entry::Unique(e)) = v.name_table.lookup(&name) else {
                return None;
            };
            let editor_core::EntityKey::Face(k) = e.key else {
                return None;
            };
            if readback::face_carrier_kind(body, k) != Ok(SurfaceKind::Plane) {
                return None;
            }
            let pose = readback::face_pose(body, k).expect("a plane has a pose");
            Some((name, body.get_face(k).expect("live").sense, pose))
        })
        .collect()
}

// ---------------------------------------------------------------------
// C1 — the frame is derived: a DIFFERENT body and a DIFFERENT edit.
// ---------------------------------------------------------------------

/// A washer (revolve), a derived frame on its sense-FALSE annulus, a
/// boss on that frame, plus an unrelated box. Edit 1 moves the
/// washer's authored frame (origin z) — the derived frame must follow
/// and the cone must be exactly {authored frame, profile, axis,
/// revolve, derived frame, boss profile, boss}; the box's three nodes
/// reuse. Edit 2 touches only the box — the derived frame and its
/// sketch reuse.
#[test]
fn r1_c1_a_derived_frame_on_a_revolve_moves_with_it_and_the_memo_recomputes_exactly_the_cone() {
    let (doc, authored, rev) = washer_at(0.0);
    let ev0 = eval(&doc);
    let (name, sense, pose) = planar_faces(&ev0, rev)
        .into_iter()
        .find(|(_, s, _)| !*s)
        .expect("the washer has a sense-false annulus");
    assert!(!sense);
    let (doc, frame) = fixture::insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: rev,
            face: name,
            spin: ang(0.0),
        }),
    );
    let (doc, boss_p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(1.5, 0.0, 0.1)])),
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: boss_p,
            distance: len(0.2),
        },
    );
    // The unrelated box.
    let (doc, box_p) = fixture::on_frame(
        doc,
        [5.0, 5.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile: box_p,
            distance: len(1.0),
        },
    );
    let ev1 = eval(&doc);
    assert!(corpus::failures(&ev1).is_empty(), "{:?}", corpus::failures(&ev1));
    let (o1, u1, v1) = frame_of(&ev1, frame);
    near3(o1, pose.origin - geom_core::Point3::origin(), "origin is the carrier's");
    near3(u1.cross(v1), -pose.axis, "sense false: normal is -axis");

    // Edit 1: lift the washer's AUTHORED frame by 0.3.
    let lifted = apply(
        &doc,
        &DocEdit::SetParam {
            node: authored,
            slot: SlotId::Origin(Axis3::Z),
            expr: len(0.3),
        },
        Tol::witness(),
    )
    .expect("a length into an origin slot")
    .doc;
    let ev2 = eval_prior(&lifted, &ev1);
    assert!(corpus::failures(&ev2).is_empty(), "{:?}", corpus::failures(&ev2));
    let (o2, u2, v2) = frame_of(&ev2, frame);
    near3(o2, o1 + Vec3::new(0.0, 0.0, 0.3), "the derived frame followed the washer");
    near3(u2, u1, "u unchanged by a translation");
    near3(v2, v1, "v unchanged by a translation");
    let cone = corpus::cone(&lifted, authored);
    assert!(cone.contains(&frame) && cone.contains(&boss_p) && cone.contains(&boss));
    assert!(!cone.contains(&cube));
    assert_eq!(cone.len(), 7, "{cone:?}");
    assert_eq!(ev2.recomputed, cone.len(), "exactly the cone recomputed");
    assert_eq!(ev2.reused, lifted.len() - cone.len(), "the box's three nodes reused");
    // The boss followed: its lowest point is the annulus at the new
    // height (the sense-false annulus is the UNDERSIDE, z = 0.3, and
    // the boss extrudes OUTWARD = downward).
    let body = corpus::body_of(&ev2, boss);
    let zs: Vec<f64> = body.points().map(|(_, p)| p.z).collect();
    let zmax = zs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let zmin = zs.iter().cloned().fold(f64::INFINITY, f64::min);
    assert!((zmax - 0.3).abs() <= 1e-12, "boss top on the underside annulus: {zmax}");
    assert!((zmin - 0.1).abs() <= 1e-12, "boss extrudes outward (down): {zmin}");

    // Edit 2: touch only the box.
    let boxed = apply(
        &lifted,
        &DocEdit::SetParam {
            node: cube,
            slot: SlotId::Distance,
            expr: len(2.0),
        },
        Tol::witness(),
    )
    .expect("a length into a distance slot")
    .doc;
    let ev3 = eval_prior(&boxed, &ev2);
    assert_eq!(ev3.recomputed, 1, "only the box");
    assert_eq!(ev3.reused, boxed.len() - 1, "the derived frame and its sketch reused");
}

// ---------------------------------------------------------------------
// C2 — spin about THE OUTWARD normal on a sense-FALSE face, θ = 2.3.
// ---------------------------------------------------------------------

/// On the washer's sense-false annulus the outward normal is `-axis`.
/// With θ = 2.3 the right-handed rotation about `-axis` and the one
/// about `+axis` differ by the sign of the sin term, so the row
/// discriminates the handedness: `u` must equal the rotation about
/// `n = -axis`, must NOT equal the rotation about `+axis`, and the
/// triad `(u, v, n)` must be right-handed.
#[test]
fn r1_c2_spin_is_right_handed_about_the_outward_normal_on_a_sense_false_face() {
    let (doc, _, rev) = washer_at(0.0);
    let ev0 = eval(&doc);
    let (name, sense, pose) = planar_faces(&ev0, rev)
        .into_iter()
        .find(|(_, s, _)| !*s)
        .expect("a sense-false annulus");
    assert!(!sense);
    let theta = 2.3_f64;
    let (doc, frame) = fixture::insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: rev,
            face: name.clone(),
            spin: ang(theta),
        }),
    );
    let ev = eval(&doc);
    assert!(corpus::failures(&ev).is_empty(), "{:?}", corpus::failures(&ev));
    let (_, u, v) = frame_of(&ev, frame);
    let n = -pose.axis;
    let u_ref = pose.u_ref.expect("a plane fixes u_ref");
    // Rodrigues about n (u_ref ⟂ n): u cosθ + (n × u) sinθ.
    let about_n = u_ref * theta.cos() + n.cross(u_ref) * theta.sin();
    let about_axis = u_ref * theta.cos() + pose.axis.cross(u_ref) * theta.sin();
    near3(u, about_n, "u = rotate(u_ref, n = -axis, θ)");
    assert!(
        (u - about_axis).norm() > 1.0,
        "u must not be the rotation about +axis (wrong handedness): {u:?} vs {about_axis:?}"
    );
    near3(v, n.cross(u), "v = n × u");
    assert!((u.cross(v).dot(n) - 1.0).abs() <= 1e-12, "right-handed about n");
    // And the same read through the StableName door agrees on sense.
    let pose2 = face_frame(&ev, rev, &name).expect("the annulus");
    assert!(!pose2.sense);
}

// ---------------------------------------------------------------------
// C3 — a face made to VANISH by an EDIT (a boolean tool moved over
// it), never re-anchored, and Rebind repairs.
// ---------------------------------------------------------------------

/// A box [0,1]³; a tool slab x∈[-1,0.5]+t, y∈[-1,2], z∈[0.75,1.25]
/// subtracted from it (t = 0: half the top cap survives, trimmed);
/// a derived frame on the boolean's FromA(top cap); a boss on it.
/// Edit t → 0.6: the tool covers the whole top, the cap is gone.
#[test]
fn r1_c3_a_face_consumed_by_a_moved_boolean_tool_fails_the_frame_typed_and_rebind_repairs() {
    let (doc, box_p) = fixture::on_frame(
        ProfileDoc::empty_derived("r1_cut", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile: box_p,
            distance: len(1.0),
        },
    );
    let (doc, tool_p) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.75],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(-1.0, -1.0), (0.5, -1.0), (0.5, 2.0), (-1.0, 2.0)]],
    );
    let (doc, tool) = fixture::insert(
        doc,
        Node::Extrude {
            profile: tool_p,
            distance: len(0.5),
        },
    );
    let (doc, moved) = fixture::insert(
        doc,
        Node::Transform {
            input: tool,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let (doc, cut) = fixture::insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cube,
            b: moved,
            declare: None,
        },
    );
    let ev0 = eval(&doc);
    assert!(corpus::failures(&ev0).is_empty(), "{:?}", corpus::failures(&ev0));
    let top_of_cube = fixture::fname(cube, RoleSeg::Cap(CapEnd::Top));
    let names = all_faces(&ev0, cut);
    let top = names
        .iter()
        .find(|n| matches!(n.path.first(), Some(RoleSeg::FromA(inner)) if **inner == top_of_cube))
        .cloned()
        .unwrap_or_else(|| panic!("the trimmed top cap keeps its FromA name; table: {names:?}"));
    let (doc, frame) = fixture::insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: cut,
            face: top.clone(),
            spin: ang(0.0),
        }),
    );
    let (doc, boss_p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(frame, vec![fixture::square(0.75, 0.5, 0.1)])),
    );
    let (doc, boss) = fixture::insert(
        doc,
        Node::Extrude {
            profile: boss_p,
            distance: len(0.2),
        },
    );
    let ev1 = eval(&doc);
    assert!(corpus::failures(&ev1).is_empty(), "{:?}", corpus::failures(&ev1));
    let (o1, u1, v1) = frame_of(&ev1, frame);
    near3(u1.cross(v1), Vec3::new(0.0, 0.0, 1.0), "outward +z");
    assert!((o1.z - 1.0).abs() <= 1e-12);

    // The EDIT: slide the tool +0.6 in x so it covers the whole top.
    let gone = apply(
        &doc,
        &DocEdit::SetParam {
            node: moved,
            slot: SlotId::Translation(Axis3::X),
            expr: len(0.6),
        },
        Tol::witness(),
    )
    .expect("a length into a translation slot")
    .doc;
    let ev2 = eval_prior(&gone, &ev1);
    // The boolean itself is fine; the cap is simply not in its table.
    assert!(matches!(ev2.nodes.get(&cut), Some(NodeResult::Ok(_))), "{:?}", ev2.nodes.get(&cut));
    assert!(
        !all_faces(&ev2, cut).contains(&top),
        "the top cap is consumed: {:?}",
        all_faces(&ev2, cut)
    );
    match ev2.nodes.get(&frame) {
        Some(NodeResult::Failed(NodeError {
            kind: NodeErrorKind::FaceFrameResolve { error },
            ..
        })) => {
            println!("R1 C3: the frame refused {error}");
            assert!(
                matches!(**error, ResolveError::Vanished { .. }),
                "the N5 arm: {error:?}"
            );
        }
        other => panic!("the frame must fail typed, never re-anchor: {other:?}"),
    }
    for above in [boss_p, boss] {
        assert!(
            matches!(ev2.nodes.get(&above), Some(NodeResult::Poisoned { through }) if *through == frame),
            "{above:?} poisoned through the frame: {:?}",
            ev2.nodes.get(&above)
        );
    }
    // A tool edit that leaves the cap ALIVE (t = 0.1: the cap is still
    // there, trimmed differently) must NOT refuse — the frame moves
    // with the face, it does not die with the tool's payload.
    let nudged = apply(
        &doc,
        &DocEdit::SetParam {
            node: moved,
            slot: SlotId::Translation(Axis3::X),
            expr: len(0.1),
        },
        Tol::witness(),
    )
    .expect("a length into a translation slot")
    .doc;
    let evn = eval_prior(&nudged, &ev1);
    assert!(
        all_faces(&evn, cut).contains(&top),
        "the cap survives a 0.1 nudge: {:?}",
        all_faces(&evn, cut)
    );
    assert!(
        corpus::failures(&evn).is_empty(),
        "a derived frame on a boolean must survive a tool edit that keeps the face: {:?}",
        corpus::failures(&evn)
    );
    // Rebind to a live planar face of the boolean (the tool's bottom
    // cap, now the cut's top at z = 0.75) repairs the whole sketch.
    let live = planar_faces(&ev2, cut)
        .into_iter()
        .find(|(_, _, p)| (p.origin.z - 0.75).abs() <= 1e-12 && p.axis.z.abs() > 0.5)
        .map(|(n, _, _)| n)
        .expect("the cut's new top at z = 0.75");
    let repaired = apply(
        &gone,
        &DocEdit::Rebind {
            from: top,
            to: live,
        },
        Tol::witness(),
    )
    .expect("a rebind between face names applies")
    .doc;
    let ev3 = eval_prior(&repaired, &ev2);
    assert!(corpus::failures(&ev3).is_empty(), "{:?}", corpus::failures(&ev3));
    let (o3, u3, v3) = frame_of(&ev3, frame);
    assert!((o3.z - 0.75).abs() <= 1e-12, "{o3:?}");
    // The tool's bottom cap faces -z as the TOOL's face; as the cut's
    // face it is the outward +z of the remaining material.
    assert!(u3.cross(v3).z > 0.5, "outward for the cut body: {:?}", u3.cross(v3));
}

/// A name of the wrong KIND (an edge) refuses typed and never lands on
/// a face; a `FaceFrame` on a name minted by a node that is NOT the
/// body it reads refuses typed too.
#[test]
fn r1_c3_a_non_face_name_and_a_foreign_name_refuse_typed() {
    let (doc, _, rev) = washer_at(0.0);
    let ev0 = eval(&doc);
    let edge = editor_core::all_edges(&ev0, rev).into_iter().next().expect("an edge");
    assert_eq!(edge.kind, EntityKind::Edge);
    let (doc2, frame) = fixture::insert(
        doc.clone(),
        Node::Datum(Datum::FaceFrame {
            at: rev,
            face: edge,
            spin: ang(0.0),
        }),
    );
    let ev = eval(&doc2);
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => {
            println!("R1 C3 edge name: {kind}");
            assert!(matches!(
                kind,
                NodeErrorKind::FaceFrameKind { .. } | NodeErrorKind::FaceFrameResolve { .. }
            ));
        }
        other => panic!("typed refusal expected: {other:?}"),
    }
    // A name minted by ANOTHER body (a box), read through the washer.
    let (doc3, box_p) = fixture::on_frame(
        doc,
        [5.0, 5.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let (doc3, cube) = fixture::insert(
        doc3,
        Node::Extrude {
            profile: box_p,
            distance: len(1.0),
        },
    );
    let (doc3, frame) = fixture::insert(
        doc3,
        Node::Datum(Datum::FaceFrame {
            at: rev,
            face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
            spin: ang(0.0),
        }),
    );
    let ev = eval(&doc3);
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => {
            println!("R1 C3 foreign name: {kind}");
            assert!(matches!(kind, NodeErrorKind::FaceFrameResolve { .. }));
        }
        other => panic!("typed refusal expected: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// C7 — the section door is decided by TYPE: `Dual64` carries an exact
// f64 value channel with a zero tangent, and must still refuse.
// ---------------------------------------------------------------------

#[test]
fn r1_c7_a_section_on_a_derived_frame_refuses_at_dual_even_with_a_zero_tangent() {
    use geom_core::Dual64;
    let (doc, loft) = crate::docm1_face_frame::lofted_on_face_frame();
    let ev = evaluate::<Dual64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&loft) {
        Some(NodeResult::Failed(NodeError {
            kind: NodeErrorKind::DerivedFrameSection { .. },
            ..
        })) => {}
        other => panic!("the loft must refuse DerivedFrameSection at Dual: {other:?}"),
    }
    // The corpus document (profile + extrude on a derived frame, no
    // section) evaluates at Dual under the default Pinned lift.
    let cd = corpus::face_sketch::document();
    let ev = evaluate::<Dual64>(
        &cd.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(corpus::failures(&ev).is_empty(), "{:?}", corpus::failures(&ev));
}
