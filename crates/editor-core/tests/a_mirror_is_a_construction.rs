//! **A mirror is a construction** (`docs/INTENT-STAGE3-SPEC.md` §8
//! "Mirror", D10 `[repetition]`): `Node::Mirror { body, plane }`
//! defines a new body, the source reflected across a `Plane` pose read
//! off that source's own space, its cells named as the source names
//! them. A symmetric part is the union of a body and its mirror across
//! one of its own faces.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len};
use editor_core::EntityKey;
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
        Some(NodeResult::Ok(v)) => (v.payload.output_body(0).expect("one body")).body(),
        other => panic!("node {id:?} did not build: {other:?}"),
    }
}

fn volume(b: &Body<f64>) -> f64 {
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume
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

/// **The image is in its source's space**: `Doc::space_of` reads a
/// mirror's body as the space of the body it reflects, so a union whose
/// first member is the image is in the plate's space too.
#[test]
fn a_mirrors_image_and_a_union_it_leads_are_in_the_sources_space() {
    let (doc, plate) = plate();
    let plane = fixture::plane_on(&doc, plate, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let (doc, mirror) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(plate, 0).into(),
            plane,
        },
    );
    let (doc, union) = fixture::insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![
                Operand::output(mirror, 0).into(),
                Operand::output(plate, 0).into(),
            ]),
        },
    );
    let home = fixture::space(&doc, plate);
    let image = doc.output(mirror, 0).expect("a mirror defines its body");
    assert_eq!(doc.space_of(image), Some(home), "the image's body");
    assert_eq!(doc.node_space(mirror), Some(home), "the mirror node");
    assert_eq!(
        doc.node_space(union),
        Some(home),
        "the union the image leads"
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

/// `loops` on the xy plane, extruded `0.5`.
fn extruded(loops: Vec<Vec<(f64, f64)>>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("mirror_sweep", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        loops,
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(0.5),
        },
    )
}

/// The points of the entity `n` names in `b`, sorted.
fn points(b: &Body<f64>, t: &editor_core::NameTable, n: &StableName) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = match fixture::key_of(t, "an entity", n) {
        EntityKey::Vertex(v) => vec![fixture::point(b, v).to_array()],
        EntityKey::Edge(e) => fixture::ends(b, e)
            .iter()
            .map(|&v| fixture::point(b, v).to_array())
            .collect(),
        EntityKey::Face(f) => fixture::face_vertices(b, f)
            .into_iter()
            .map(|v| fixture::point(b, v).to_array())
            .collect(),
        EntityKey::Body => Vec::new(),
    };
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

fn near(a: &[[f64; 3]], b: &[[f64; 3]]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(p, q)| p.iter().zip(q).all(|(x, y)| (x - y).abs() < 1e-9))
}

/// Mirror `source` across every planar face of it, plainly and flipped;
/// for each entity of the source, whether its image walks to the
/// source's construction ("fixed") and whether it is geometrically
/// fixed. Panics on a walk-fixed entity the reflection moves (a false
/// proof); returns how many images walk fixed.
fn sweep(what: &str, loops: Vec<Vec<(f64, f64)>>) -> usize {
    let (doc, src) = extruded(loops);
    let ev = eval(&doc);
    let b = body(&ev, src);
    let t = fixture::table(&ev, src);
    let names: Vec<StableName> = t.iter().map(|(n, _)| n.clone()).collect();
    let planar: Vec<StableName> = names
        .iter()
        .filter(|n| n.kind == EntityKind::Face)
        .filter(|n| {
            topo::readback::face_carrier_kind(b, fixture::face_of(t, "a face", n))
                .is_ok_and(|k| k == geom::SurfaceKind::Plane)
        })
        .cloned()
        .collect();
    let mut fixed = 0;
    for face in &planar {
        let plain: editor_core::pose::RoleRead<Operand> = editor_core::pose::RoleRead::plain(
            fixture::plane(Operand::select(Operand::output(src, 0), vec![face.clone()])),
        );
        let flipped = editor_core::pose::RoleRead::plain(fixture::flip(plain.read.clone()));
        for plane in [plain, flipped] {
            let (doc, m) = fixture::insert(
                doc.clone(),
                Node::Mirror {
                    body: Operand::output(src, 0).into(),
                    plane,
                },
            );
            let ev = eval(&doc);
            let (sb, st) = (body(&ev, src), fixture::table(&ev, src));
            let (ib, it) = (body(&ev, m), fixture::table(&ev, m));
            let image = doc.output(m, 0).unwrap();
            let source = doc.output(src, 0).unwrap();
            for n in &names {
                let a = coincide::construction(&doc, image, n, Tol::witness()).expect("walks");
                let s = coincide::construction(&doc, source, n, Tol::witness()).expect("walks");
                let walk_fixed = a == s;
                let geo_fixed = near(&points(ib, it, n), &points(sb, st, n));
                if walk_fixed {
                    fixed += 1;
                    assert!(
                        geo_fixed,
                        "{what}: across {face:?}, {n:?} walks fixed but the reflection moves it"
                    );
                } else {
                    assert_eq!(
                        a.maps.first(),
                        Some(&coincide::Map::Mirror(m)),
                        "{what}: an unfixed image is read through the mirror"
                    );
                }
            }
        }
    }
    fixed
}

/// Every walk-fixed image entity is geometrically fixed, across every
/// planar face, over profiles authored either way round, with a hole,
/// a near-collinear vertex, and a value-only coplanar prong; and under
/// a jitter that makes no two walls parallel, so nothing is fixed by
/// value.
#[test]
fn every_walk_fixed_image_is_geometrically_fixed() {
    let l = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let mut l_cw = l.clone();
    l_cw.reverse();
    let c = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 3.0),
        (0.0, 3.0),
        (0.0, 2.0),
        (1.0, 2.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];
    let jitter = |p: &[(f64, f64)]| -> Vec<(f64, f64)> {
        p.iter()
            .enumerate()
            .map(|(k, &(x, y))| {
                let k = k as f64;
                (
                    x + 0.07 * (1.3 * k + 0.4).sin(),
                    y + 0.05 * (0.9 * k + 1.1).cos(),
                )
            })
            .collect()
    };
    let hole = vec![
        vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)],
        vec![(1.0, 1.0), (1.0, 2.0), (2.0, 2.0), (2.0, 1.0)],
    ];
    // A hundred ε off the line: inside the near-collinear row, clear of
    // the band, so the junction still turns at every ε row.
    let nearer_off = 100.0 * Tol::witness().get().eps;
    let nearer = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 2.0),
        (0.0, 2.0),
        (nearer_off, 1.0),
    ];
    let near_collinear = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (1e-3, 1.0)];
    for (what, loops) in [
        ("L", vec![l.clone()]),
        ("L clockwise", vec![l_cw]),
        ("C", vec![c.clone()]),
        ("L jittered", vec![jitter(&l)]),
        ("C jittered", vec![jitter(&c)]),
        ("square with a hole", hole),
        ("near-collinear", vec![near_collinear]),
        ("nearer-collinear", vec![nearer]),
    ] {
        let fixed = sweep(what, loops);
        assert!(fixed > 0, "{what}: the sweep fixed something");
    }
}

/// Two mirrors of one body across one plane, and a mirror of a mirror
/// across the same plane: an off-plane cell's images are two
/// constructions (each mirror is its own link), and the double image is
/// not walked back to its source; an on-plane cell is its source in all.
#[test]
fn two_mirrors_across_one_plane_are_two_links() {
    let (doc, src) = extruded(vec![vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ]]);
    let ev = eval(&doc);
    let (b, t) = (body(&ev, src), fixture::table(&ev, src));
    let names: Vec<StableName> = t.iter().map(|(n, _)| n.clone()).collect();
    let wall = names
        .iter()
        .find(|n| {
            n.kind == EntityKind::Face && {
                let p = points(b, t, n);
                p.len() == 4 && p.iter().all(|q| q[0] == 0.0)
            }
        })
        .unwrap()
        .clone();
    let plane = || {
        editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
            Operand::output(src, 0),
            vec![wall.clone()],
        )))
    };
    let (doc, m1) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(src, 0).into(),
            plane: plane(),
        },
    );
    let (doc, m2) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(src, 0).into(),
            plane: plane(),
        },
    );
    let (doc, mm) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(m1, 0).into(),
            plane: plane(),
        },
    );
    let walk = |node, n: &StableName| {
        coincide::construction(&doc, doc.output(node, 0).unwrap(), n, Tol::witness()).unwrap()
    };
    let cap = names
        .iter()
        .find(|n| {
            matches!(
                n.path.as_slice(),
                [editor_core::RoleSeg::Cap(editor_core::CapEnd::End)]
            )
        })
        .unwrap();
    assert_eq!(walk(m1, cap).maps, [coincide::Map::Mirror(m1)]);
    assert_ne!(walk(m1, cap), walk(m2, cap), "two mirrors, two links");
    assert_eq!(
        walk(mm, cap).maps,
        [coincide::Map::Mirror(mm), coincide::Map::Mirror(m1)],
        "a double image is not walked back to its source"
    );
    for node in [m1, m2, mm] {
        assert_eq!(
            walk(node, &wall),
            walk(src, &wall),
            "the plane's face is fixed"
        );
    }
}

/// **A copy's cell on the copy of the plane's face is not on the plane**:
/// a copy of the plate slid 5 along `x` into the plate's own space,
/// mirrored across the plane of the plate's `−x` wall. The copy's `−x`
/// wall and every edge and vertex on it are named as the plate's are,
/// but stand 5 off the plane, so each image is read through the mirror
/// and is no cell of the copy. `lies_on`'s "one chain of maps" guard is
/// the only thing that tells these apart.
#[test]
fn a_slid_copys_wall_is_not_on_the_plates_plane() {
    let (doc, src) = extruded(vec![vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ]]);
    let ev = eval(&doc);
    let (b, t) = (body(&ev, src), fixture::table(&ev, src));
    let wall = t
        .iter()
        .map(|(n, _)| n.clone())
        .find(|n| {
            n.kind == EntityKind::Face && {
                let p = points(b, t, n);
                p.len() == 4 && p.iter().all(|q| q[0] == 0.0)
            }
        })
        .unwrap();
    let (doc, copy) = fixture::slid(doc, src, src, [5.0, 0.0, 0.0]);
    let plane = editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
        Operand::output(src, 0),
        vec![wall.clone()],
    )));
    let (doc, m) = fixture::insert(
        doc,
        Node::Mirror {
            body: Operand::output(copy, 0).into(),
            plane,
        },
    );
    let ev = eval(&doc);
    let (cb, ct) = (body(&ev, copy), fixture::table(&ev, copy));
    let (ib, it) = (body(&ev, m), fixture::table(&ev, m));
    let (copy_read, image_read) = (doc.output(copy, 0).unwrap(), doc.output(m, 0).unwrap());
    let mut checked = 0;
    for (n, _) in ct.iter() {
        if n.kind == EntityKind::Body {
            continue;
        }
        let a = coincide::construction(&doc, image_read, n, Tol::witness()).expect("walks");
        let s = coincide::construction(&doc, copy_read, n, Tol::witness()).expect("walks");
        assert!(!near(&points(ib, it, n), &points(cb, ct, n)), "{n:?} moves");
        assert_ne!(a, s, "{n:?}: the image is not the copy's cell");
        assert_eq!(a.maps.first(), Some(&coincide::Map::Mirror(m)), "{n:?}");
        checked += 1;
    }
    assert!(checked > 30, "{checked}");
}
