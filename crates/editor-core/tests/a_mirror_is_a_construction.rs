//! **A mirror is a construction** (`docs/INTENT-STAGE3-SPEC.md` §8
//! "Mirror", D10 `[repetition]`): `Node::Mirror { body, plane }`
//! defines a new body, the source reflected across a `Plane` pose read
//! off that source's own space, its cells named as the source names
//! them. A symmetric part is the union of a body and its mirror across
//! one of its own faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::{
    Bodies, CancelToken, CapEnd, EditError, EntityKind, Entry, EvalOptions, Evaluation, Node,
    NodeErrorKind, NodeResult, Operand, OperandSlot, ProfileDoc, RecipeNodeId, RoleSeg, SlotKind,
    VarKind, evaluate,
};
use geom_core::Tol;
use topo::Body;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// An L-shaped plate, `2 × 2` with a `1 × 1` notch off its `+x, +y`
/// corner, extruded `0.5` on its own xy plane: no mirror plane of
/// interest is one of its symmetries.
fn plate() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("mirror_plate", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(0.5),
        },
    )
}

fn body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Body<f64> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body"),
        other => panic!("node {id:?} did not build: {other:?}"),
    }
}

fn volume(b: &Body<f64>) -> f64 {
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    topo::mass_properties(&measured, Tol::witness())
        .unwrap()
        .volume
}

/// Every vertex point of `b`, sorted, so two bodies compare by point set.
fn corners(b: &Body<f64>) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = b.points().map(|(_, p)| p.to_array()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

/// **The image is the source reflected, named as the source.** The
/// plate mirrored across the plane `x = 0` read off its own `−x` face
/// has the plate's volume, its every corner at `(−x, y, z)`, and its
/// start cap under the plate's own name.
#[test]
fn a_mirror_is_its_sources_reflection_under_its_sources_names() {
    let (doc, plate) = plate();
    let plane = fixture::plane_on(&doc, plate, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane,
        },
    );
    let ev = eval(&doc);
    let (source, image) = (body(&ev, plate), body(&ev, mirror));
    assert!(
        (volume(image) - volume(source)).abs() <= 1e-12,
        "the image's volume {} is the source's {}",
        volume(image),
        volume(source)
    );
    let mut reflected: Vec<[f64; 3]> = corners(source)
        .into_iter()
        .map(|[x, y, z]| [-x, y, z])
        .collect();
    reflected.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(corners(image), reflected, "the image's corners");
    let cap = fixture::fname(plate, RoleSeg::Cap(CapEnd::Start));
    let out = doc.output(mirror, 0).expect("the mirror defines its body");
    assert!(
        ev.table(out)
            .expect("the image has names")
            .lookup(&cap)
            .is_some(),
        "the image names its start cap as the plate does"
    );
}

/// **A symmetric part** is the union of a body and its mirror across
/// one of its own faces: one solid of twice the volume, the face read
/// twice gone into the interior.
#[test]
fn a_body_and_its_mirror_across_its_own_face_union_to_one_solid() {
    let (doc, plate) = plate();
    let face = Operand::select(
        Operand::output(plate, 0),
        vec![fixture::fname(plate, RoleSeg::Cap(CapEnd::Start))],
    );
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane: editor_core::pose::RoleRead::plain(fixture::plane(face)),
        },
    );
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                Operand::output(plate, 0).into(),
                Operand::output(mirror, 0).into(),
            ]),
        },
    );
    let ev = eval(&doc);
    let whole = body(&ev, union);
    assert_eq!(whole.solids().count(), 1, "one solid");
    let v = volume(whole);
    assert!(
        (v - 2.0 * volume(body(&ev, plate))).abs() <= 1e-12,
        "the union's volume {v} is twice the plate's"
    );
}

/// **The plane holds its kind**: an axis at a mirror's plane refuses at
/// the door, expecting a plane.
#[test]
fn a_mirror_plane_refuses_an_axis() {
    let (doc, plate) = plate();
    let edge = fixture::rim_edge(plate, CapEnd::End, fixture::piece(&doc, plate, 0, 0));
    let refused = fixture::insert_refused(
        &doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane: fixture::axis(Operand::select(Operand::output(plate, 0), vec![edge])).into(),
        },
    );
    assert!(
        matches!(
            &refused,
            EditError::SlotVarKind {
                expected: SlotKind::Is(VarKind::Plane),
                found: VarKind::Axis,
                ..
            }
        ),
        "{refused:?}"
    );
    let _ = OperandSlot::Plane;
}

/// **The plane reads the source's space alone**: a plane read off
/// another construction relates two spaces, which only a placement
/// does, so the mirror refuses naming both.
#[test]
fn a_mirror_plane_off_another_construction_refuses_its_space() {
    let (doc, plate) = plate();
    let profile = match doc.node(plate) {
        Some(Node::Extrude { profile, .. }) => Operand::Var(*profile),
        _ => unreachable!(),
    };
    let (doc, other) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let plane = fixture::plane_on(&doc, other, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane,
        },
    );
    let ev = eval(&doc);
    match ev.nodes.get(&mirror) {
        Some(NodeResult::Failed(refusal)) => assert!(
            matches!(refusal.kind, NodeErrorKind::SpaceMismatch { .. }),
            "{refusal:?}"
        ),
        other => panic!("a mirror reading two spaces must refuse, got {other:?}"),
    }
}

/// **A body and its mirror, side by side, name every entity apart**
/// (NAMES N0: identical operands are told apart by their reads). The
/// image carries its source's names verbatim; a union over the plate
/// and its mirror across a plane clear of it keeps both as separate
/// solids and names each member's entities by the read it came through,
/// so no name in the union's table is tied, and each solid's every face
/// is named once.
#[test]
fn a_body_and_its_mirror_side_by_side_name_every_entity_apart() {
    let (doc, plate) = plate();
    let plane = fixture::plane_on(&doc, plate, [-0.75, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane,
        },
    );
    let (doc, both) = fixture::insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                Operand::output(plate, 0).into(),
                Operand::output(mirror, 0).into(),
            ]),
        },
    );
    let ev = eval(&doc);
    let faces = body(&ev, plate).faces().count();
    assert_eq!(body(&ev, both).solids().count(), 2, "two solids, apart");
    let mut face_names = 0;
    for (name, entry) in fixture::table(&ev, both).iter() {
        assert!(
            matches!(entry, Entry::Unique(_)),
            "{name:?} names more than one entity: {entry:?}"
        );
        face_names += usize::from(name.kind == EntityKind::Face);
    }
    assert_eq!(
        face_names,
        2 * faces,
        "each solid's every face is named once"
    );
}
