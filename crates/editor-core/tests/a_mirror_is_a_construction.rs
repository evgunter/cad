//! **A mirror is a construction** (`docs/INTENT-STAGE3-SPEC.md` §8
//! "Mirror", D10 `[repetition]`): `Node::Mirror { body, plane }`
//! defines a new body, the source reflected across a `Plane` pose read
//! off that source's own space, its cells named as the source names
//! them. A symmetric part is the union of a body and its mirror across
//! one of its own faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::{
    Bodies, CancelToken, CapEnd, EditError, EntityKind, Entry, EvalOptions, Evaluation, NamedCell,
    NamedCoincidence, Node, NodeErrorKind, NodeResult, Operand, OperandSlot, ProfileDoc, Proof,
    RecipeNodeId, RoleSeg, Rung, SlotKind, StableName, VarKind, coincide, evaluate,
};
use geom_core::Tol;
use topo::Body;
use topo::Relation;

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
}

/// **The plane reads the source's space alone, at the door**: a plane
/// read off another construction relates two spaces, which only a
/// placement does, so the insert refuses naming both, before any
/// evaluation.
#[test]
fn a_mirror_plane_off_another_construction_refuses_at_the_door() {
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
    let (space_a, space_b) = (fixture::space(&doc, plate), fixture::space(&doc, other));
    let plane = fixture::plane_on(&doc, other, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let refused = fixture::insert_refused(
        &doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane,
        },
    );
    match &refused {
        EditError::SpaceMismatch { a, b, .. } => {
            let mut got = [a.id(), b.id()];
            got.sort();
            let mut want = [space_a, space_b];
            want.sort();
            assert_eq!(got, want, "the refusal names both spaces");
        }
        other => panic!("a mirror reading two spaces refuses at the door, got {other:?}"),
    }
}

/// **The plane reads the source's space alone, at evaluation**: the
/// mirror's body is a copy of a block placed into the plate's space,
/// its plane read off the plate. Re-targeting the copy into another
/// space is admitted and strands the mirror; the document saves and
/// loads, and the mirror refuses `SpaceMismatch` at evaluation.
#[test]
fn a_re_target_strands_a_mirror_and_it_refuses_at_evaluation() {
    let (doc, plate) = plate();
    let (doc, other) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(6.0, 0.0, 0.5)],
    );
    let (doc, other) = fixture::insert(
        doc,
        Node::Extrude {
            profile: other.into(),
            distance: len(1.0),
        },
    );
    let (doc, block) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(3.0, 0.0, 0.5)],
    );
    let (doc, block) = fixture::insert(
        doc,
        Node::Extrude {
            profile: block.into(),
            distance: len(1.0),
        },
    );
    let (doc, copy) = fixture::stood_in(doc, block, plate);
    let plane = fixture::plane_on(&doc, plate, [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(copy, 0).into(),
            plane,
        },
    );
    assert!(
        matches!(eval(&doc).nodes.get(&mirror), Some(NodeResult::Ok(_))),
        "the mirror builds in the plate's space"
    );
    let moved = editor_core::apply(
        &doc,
        &editor_core::DocEdit::SetParam {
            node: copy,
            slot: editor_core::SlotId::Operand(OperandSlot::Space),
            value: editor_core::SlotValue::Read(Operand::Var(fixture::space(&doc, other))),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("a re-target is admitted")
    .doc;
    let text = editor_core::save(&moved, &[], Tol::witness()).expect("the stranded document saves");
    let loaded = editor_core::load(&text, Tol::witness())
        .expect("and loads")
        .doc;
    match eval(&loaded).nodes.get(&mirror) {
        Some(NodeResult::Failed(refusal)) => assert!(
            matches!(refusal.kind, NodeErrorKind::SpaceMismatch { .. }),
            "{refusal:?}"
        ),
        other => panic!("a stranded mirror refuses at evaluation, got {other:?}"),
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

/// Each row `node` decided, with the coincidence door's proof of it.
fn door(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
) -> Vec<(NamedCoincidence, Proof)> {
    let rows = ev
        .value(node)
        .expect("the node evaluated")
        .coincidences
        .to_vec();
    let proofs = coincide::prove(
        doc,
        &[(node, &rows)],
        None,
        Tol::witness(),
        &CancelToken::new(),
    )
    .expect("an uncanceled door")
    .pop()
    .expect("one node asked");
    rows.into_iter().zip(proofs).collect()
}

/// The one face of `node`'s body with a vertex at every point of `at`,
/// by name.
fn face_through(ev: &Evaluation<f64>, node: RecipeNodeId, at: &[[f64; 3]]) -> StableName {
    let b = body(ev, node);
    let t = fixture::table(ev, node);
    let found: Vec<StableName> = t
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Face)
        .filter(|(n, _)| {
            let corners: Vec<[f64; 3]> =
                fixture::face_vertices(b, fixture::face_of(t, "a face", n))
                    .into_iter()
                    .map(|v| fixture::point(b, v).to_array())
                    .collect();
            at.iter().all(|p| corners.contains(p))
        })
        .map(|(n, _)| n.clone())
        .collect();
    let [one] = found.as_slice() else {
        panic!("one face of {node:?} through {at:?}, found {found:?}")
    };
    one.clone()
}

/// The plane of `source`'s face `face`, read plainly.
fn plane_of(source: RecipeNodeId, face: &StableName) -> editor_core::pose::RoleRead<Operand> {
    editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
        Operand::output(source, 0),
        vec![face.clone()],
    )))
}

/// **A symmetric part**: `source` and its image across the plane of its
/// face through `at`, unioned. The document, that face, the mirror and
/// the union.
fn symmetric(
    doc: ProfileDoc,
    source: RecipeNodeId,
    at: &[[f64; 3]],
) -> (ProfileDoc, StableName, RecipeNodeId, RecipeNodeId) {
    let face = face_through(&eval(&doc), source, at);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(source, 0).into(),
            plane: plane_of(source, &face),
        },
    );
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                Operand::output(source, 0).into(),
                Operand::output(mirror, 0).into(),
            ]),
        },
    );
    (doc, face, mirror, union)
}

/// The name each of a row's cells reads, all entity cells.
fn names(row: &NamedCoincidence) -> Vec<&StableName> {
    row.cells
        .iter()
        .map(|cell| match cell {
            NamedCell::Entity { name, .. } => name,
            other => panic!("an entity cell, got {other:?}"),
        })
        .collect()
}

/// **The seam rows of a symmetric part**: the row of `node` gluing the
/// face `seam` to its image, and the vertex rows over that face's
/// corners, which name the vertices a face of `seam`'s carrier holds.
fn seam_rows(
    rows: &[(NamedCoincidence, Proof)],
    seam: &StableName,
    corners: &[StableName],
) -> (Proof, Vec<Proof>) {
    let face = rows
        .iter()
        .filter(|(row, _)| names(row) == [seam, seam])
        .map(|(row, proof)| {
            assert_eq!(
                row.relation,
                Relation::SameOpposite,
                "the seam meets opposed"
            );
            proof.clone()
        })
        .collect::<Vec<_>>();
    let [face] = face.as_slice() else {
        panic!("one row glues {seam:?} to its image: {rows:?}")
    };
    let vertices: Vec<Proof> = corners
        .iter()
        .map(|v| {
            let found: Vec<&Proof> = rows
                .iter()
                .filter(|(row, _)| names(row) == [v, v])
                .map(|(_, proof)| proof)
                .collect();
            let [one] = found.as_slice() else {
                panic!("one row fuses {v:?} with its image: {rows:?}")
            };
            (*one).clone()
        })
        .collect();
    (face.clone(), vertices)
}

/// The names of the vertices of `node`'s body at `at`.
fn vertices_at(ev: &Evaluation<f64>, node: RecipeNodeId, at: &[[f64; 3]]) -> Vec<StableName> {
    let b = body(ev, node);
    let t = fixture::table(ev, node);
    at.iter()
        .map(|p| {
            let found: Vec<StableName> = t
                .iter()
                .filter(|(n, _)| n.kind == EntityKind::Vertex)
                .filter(|(n, _)| match fixture::key_of(t, "a vertex", n) {
                    editor_core::EntityKey::Vertex(v) => fixture::point(b, v).to_array() == *p,
                    _ => false,
                })
                .map(|(n, _)| n.clone())
                .collect();
            let [one] = found.as_slice() else {
                panic!("one vertex of {node:?} at {p:?}, found {found:?}")
            };
            one.clone()
        })
        .collect()
}

/// **A symmetric part's seam is its face read twice** (D10
/// `[repetition]`, `[coincidence.structural]`'s first rung): the plate
/// mirrored across the plane of its own `−x` wall, and across its own
/// start cap, each unioned with its image. A reflection fixes its
/// plane, so the image of the face the plane is read off is that face
/// again, and each vertex on it that vertex again: the seam row and
/// every seam vertex row are the same construction, proven by the walk
/// alone. A face off the plane is its image read through the mirror,
/// another construction.
#[test]
fn a_symmetric_parts_seam_is_its_face_read_twice() {
    let t = 0.5;
    let wall = [[0.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, t]];
    let cap = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    let wall_corners = [
        [0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, t],
        [0.0, 2.0, t],
    ];
    let cap_corners = [
        [0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0],
        [2.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [1.0, 2.0, 0.0],
        [0.0, 2.0, 0.0],
    ];
    for (what, at, corners) in [
        ("its -x wall", &wall, &wall_corners[..]),
        ("its start cap", &cap, &cap_corners[..]),
    ] {
        let (doc, plate) = plate();
        let corners = vertices_at(&eval(&doc), plate, corners);
        let (doc, seam, mirror, union) = symmetric(doc, plate, at);
        let ev = eval(&doc);
        let rows = door(&doc, &ev, union);
        let (face, vertices) = seam_rows(&rows, &seam, &corners);
        assert_eq!(
            face,
            Proof::Structural(Rung::SameConstruction),
            "across {what}: the seam is the face read twice"
        );
        for (v, proof) in corners.iter().zip(&vertices) {
            assert_eq!(
                proof,
                &Proof::Structural(Rung::SameConstruction),
                "across {what}: {v:?} lies on the plane's face, read twice"
            );
        }
        let image = doc.output(mirror, 0).expect("the mirror defines its body");
        let source = doc.output(plate, 0).expect("the plate defines its body");
        let walk = |read, name: &StableName| {
            coincide::construction(&doc, read, name, Tol::witness()).expect("the cell walks")
        };
        assert_eq!(walk(image, &seam), walk(source, &seam), "across {what}");
        let off = fixture::fname(plate, RoleSeg::Cap(CapEnd::End));
        assert_eq!(
            walk(image, &off).maps,
            [coincide::Map::Mirror(mirror)],
            "across {what}: the far cap's image is read through the mirror"
        );
        assert_eq!(walk(image, &off).origin, walk(source, &off).origin);
        assert!(walk(source, &off).maps.is_empty(), "across {what}");
    }
}

/// A C, `2 × 3` open to `−x`, its two prongs' ends both on `x = 0`,
/// extruded `0.5`.
fn c_shape() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("mirror_c", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 3.0),
            (0.0, 3.0),
            (0.0, 2.0),
            (1.0, 2.0),
            (1.0, 1.0),
            (0.0, 1.0),
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

/// **A face on the mirror plane by value alone is a finding**: a C
/// mirrored across the plane of its lower prong's end. Its upper prong's
/// end lies on that plane too, at these values, but the C is not built
/// so: the lower prong's seam is proven the face read twice, and the
/// upper prong's rows are left unproven, its image read through the
/// mirror.
#[test]
fn a_prong_on_the_mirror_plane_by_value_alone_is_left_unproven() {
    let t = 0.5;
    let (doc, c) = c_shape();
    let ev = eval(&doc);
    let lower_corners = [
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, t],
        [0.0, 1.0, t],
    ];
    let lower_face = face_through(&ev, c, &lower_corners);
    let lower = vertices_at(&ev, c, &lower_corners);
    let upper_corners = [
        [0.0, 2.0, 0.0],
        [0.0, 3.0, 0.0],
        [0.0, 2.0, t],
        [0.0, 3.0, t],
    ];
    let upper_face = face_through(&ev, c, &upper_corners);
    let upper = vertices_at(&ev, c, &upper_corners);
    let (doc, _, mirror, union) =
        symmetric(doc, c, &[[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, t]]);
    let ev = eval(&doc);
    let rows = door(&doc, &ev, union);
    let (face, vertices) = seam_rows(&rows, &lower_face, &lower);
    assert_eq!(
        face,
        Proof::Structural(Rung::SameConstruction),
        "the lower prong's seam"
    );
    assert!(
        vertices
            .iter()
            .all(|p| p == &Proof::Structural(Rung::SameConstruction)),
        "the lower prong's corners: {vertices:?}"
    );
    let (face, vertices) = seam_rows(&rows, &upper_face, &upper);
    for proof in core::iter::once(&face).chain(&vertices) {
        let Proof::Unproven { residual, .. } = proof else {
            panic!("the upper prong meets its image by value alone: {proof:?}")
        };
        let [Ok(source), Ok(image)] = residual.constructions.as_slice() else {
            panic!("both cells walk: {residual:?}")
        };
        assert_eq!(source.origin, image.origin, "one extrude minted both");
        assert!(source.maps.is_empty(), "{source:?}");
        assert_eq!(image.maps, [coincide::Map::Mirror(mirror)], "{image:?}");
    }
}

/// **A split across its body's own face cuts on that face's
/// construction**: the plate split by the plane of its notch's `x = 1`
/// wall, read plainly and flipped. A plane read off a face is that
/// face's construction, its sense aside, so the section face the split
/// mints on it is that wall's construction, read in the plate.
#[test]
fn a_split_across_its_bodys_own_face_cuts_on_that_faces_construction() {
    let (doc, plate) = plate();
    let wall = face_through(
        &eval(&doc),
        plate,
        &[[1.0, 1.0, 0.0], [1.0, 2.0, 0.0], [1.0, 1.0, 0.5]],
    );
    let source = doc.output(plate, 0).expect("the plate defines its body");
    let want = coincide::construction(&doc, source, &wall, Tol::witness()).expect("the wall walks");
    let plain = plane_of(plate, &wall);
    let flipped = editor_core::pose::RoleRead::plain(Operand::Pose(Box::new(
        editor_core::pose::PoseDef::Flip {
            pose: plain.clone(),
        },
    )));
    for (what, tool) in [("plainly", plain), ("flipped", flipped)] {
        let (doc, split) = fixture::insert(
            doc.clone(),
            Node::Split {
                target: Operand::output(plate, 0).into(),
                tool,
            },
        );
        let ev = eval(&doc);
        let section = fixture::fname(split, RoleSeg::SectionFace);
        let halves: Vec<editor_core::VarId> = [0u8, 1]
            .iter()
            .map(|&k| {
                doc.output(split, k)
                    .expect("a split defines a port per half")
            })
            .filter(|&half| ev.table(half).is_some_and(|t| t.lookup(&section).is_some()))
            .collect();
        assert!(
            !halves.is_empty(),
            "{what}: a half holds the section face whole"
        );
        for half in halves {
            assert_eq!(
                coincide::construction(&doc, half, &section, Tol::witness()),
                Ok(want.clone()),
                "{what}: the section face is the wall's construction"
            );
        }
    }
}

/// **The symmetric-plate corpus document's seam** (row (b)): its mirror
/// plane is the plate's `−x` wall read plainly, so its seam and the
/// seam's corners are proven the face read twice.
#[test]
fn the_symmetric_plate_corpus_documents_seam_is_its_face_read_twice() {
    let corpus = crate::corpus::symmetric_plate::document();
    let (doc, union) = (&corpus.doc, corpus.result.expect("the union"));
    let Some(Node::Union {
        members: Bodies::Spelled(members),
    }) = doc.node(union)
    else {
        panic!("the union")
    };
    let plate = doc.operation_of(members[0].read).expect("the plate");
    let t = 0.5;
    let wall_corners = [
        [0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, t],
        [0.0, 2.0, t],
    ];
    let ev = eval(doc);
    let seam = face_through(&ev, plate, &wall_corners);
    let corners = vertices_at(&ev, plate, &wall_corners);
    let (face, vertices) = seam_rows(&door(doc, &ev, union), &seam, &corners);
    assert_eq!(face, Proof::Structural(Rung::SameConstruction), "the seam");
    assert!(
        vertices
            .iter()
            .all(|p| p == &Proof::Structural(Rung::SameConstruction)),
        "the seam's corners: {vertices:?}"
    );
}
