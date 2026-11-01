//! **A placement over a family is a family of families** (D10
//! `[repetition]`): a placement whose source reads a family reaches
//! that family's index, so with its own index `j` declared within the
//! source's `i` it is one member per `(i, j)`, outer first — member
//! `(i, j)` is placement `j` of the source's member `i`, named
//! `Member { (i, j) }` over the name the source member's entity wears
//! in its own body: one qualifier, keyed by every index it reaches.
//!
//! Every identity here is BIT identity through the public doors: the
//! oracle for "placement `j` of member `i`" is the same placement over
//! the indexed read `inner[i]` alone — the same kernel op on the same
//! body under the same map, so nothing is approximate and no rule is
//! restated. The lane rows of the same claims are
//! `eval6_placers_over_instances_interval`. The digest here is the
//! geometry, the topology and the arena keys — the bits a consumer
//! reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;

use editor_core::{
    BodyRead, DocumentId, EntityKey, Entry, EvalOptions, Formula, Node, NodeErrorKind, ProfileDoc,
    RecipeNodeId, RoleSeg, ValuePayload, VarId,
};
use fixture::{in_member, insert, len, on_frame, run};
use geom_core::Tol;
use topo::Body;

/// A unit cube at the origin, as a document, with its extrude's id.
pub(crate) fn cube_doc(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The inner count and the outer count of the nested documents.
pub(crate) const M: i64 = 3;
pub(crate) const N: i64 = 2;

/// **A copy of `source`, a read of `at`, in its own space slid
/// `scalar(by)·2` along y**: the document and the placement.
pub(crate) fn along_y(
    doc: ProfileDoc,
    (at, source): (RecipeNodeId, BodyRead<Formula>),
    by: VarId,
) -> (ProfileDoc, RecipeNodeId) {
    let target = fixture::space(&doc, at);
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(by, [0.0, 1.0, 0.0], 2.0),
    );
    insert(
        doc,
        Node::place(source, editor_core::Operand::Var(target), placed.bundle()),
    )
}

/// **The nested document**: a cube, an inner row of `M` along x over
/// `i`, an outer row of `N` along y over the inner's family and
/// `j = index(N) within i`. Returns `(doc, cube, inner, outer, i, j)`.
pub(crate) fn nested_doc(
    label: &str,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    VarId,
    VarId,
) {
    let (doc, cube) = cube_doc(label);
    let (doc, i, inner) = fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, M);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(N), Some(i));
    let read = BodyRead::plain(fixture::body_read(&doc, inner.into()));
    let (doc, outer) = along_y(doc, (inner, read), j);
    (doc, cube, inner, outer, i, j)
}

/// **Each inner member alone, placed as the outer places it**: one row
/// of `N` over `inner[i]` per `i`, under one index of its own. The
/// document and the rows, by `i`.
pub(crate) fn per_member(doc: ProfileDoc, inner: RecipeNodeId) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (mut doc, alone) = fixture::index(doc, "alone", N);
    let mut rows = Vec::new();
    for i in 0..M {
        let read = BodyRead::indexed(inner, vec![Formula::count(i)]);
        let (next, row) = along_y(doc, (inner, read), alone);
        doc = next;
        rows.push(row);
    }
    (doc, rows)
}

/// Every geometric description of a body with its arena key, plus the
/// entity census — the bits, keyed. Keys are stable across a rigid
/// placement, so two placements of ONE body compare key for key.
pub(crate) fn bits<T: geom_core::Decide + core::fmt::Debug>(b: &Body<T>) -> Vec<String> {
    let mut v = vec![format!(
        "counts f{} e{} v{}",
        b.faces().count(),
        b.edges().count(),
        b.vertices().count()
    )];
    v.extend(b.surfaces().map(|(k, s)| format!("S {k:?} {s:?}")));
    v.extend(b.curves().map(|(k, c)| format!("C {k:?} {c:?}")));
    v.extend(b.points().map(|(k, p)| format!("P {k:?} {p:?}")));
    v
}

/// The bodies of the family a node evaluated to, each in its space.
pub(crate) fn instances_of<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &editor_core::Evaluation<T>,
    id: RecipeNodeId,
) -> Vec<Arc<Body<T>>> {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Instances(v) => v
            .iter()
            .map(|b| Arc::new(crate::fixture::in_space(b)))
            .collect(),
        other => panic!("{id:?} is a {}, not a family", other.kind_name()),
    }
}

fn opts() -> EvalOptions {
    EvalOptions::default()
}

/// The place of member `(i, j)` among the outer family's bodies: `i`
/// outer.
fn flat(i: usize, j: usize) -> usize {
    i * N as usize + j
}

// ---- the nested layout ----

/// **A family of families is one member per `(i, j)`, `i` outer**: the
/// outer placement is `N·M` bodies; member `(i, j)` is bit-identical to
/// placement `j` of `inner[i]` alone, through the same door; every name
/// is one `Member { (i, j) }` over a master name, at body `i·N + j`,
/// where the inner family holds that master name at body `i` under
/// `Member { (i) }`; and every entity of every member is named. Red if
/// the outer family is laid out placement-major, or its names drop the
/// inner index.
#[test]
fn a_family_of_families_is_one_member_per_pair_outer_first() {
    let (doc, cube, inner, outer, i_var, j_var) = nested_doc("eval6-c3");
    let (doc, rows) = per_member(doc, inner);
    let ev = run(&doc, &opts());
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M, "N·M bodies");
    for (i, row) in rows.iter().enumerate() {
        let alone = instances_of(&ev, *row);
        assert_eq!(alone.len() as i64, N);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[flat(i, j)]),
                bits(&alone[j]),
                "member ({i}, {j}) is placement {j} of inner member {i}"
            );
        }
    }
    let (of_inner, of_outer) = (
        ev.table_at(inner).expect("the inner"),
        ev.table_at(outer).expect("the outer"),
    );
    assert_eq!(of_outer.len(), of_inner.len() * N as usize);
    for (name, entry) in of_outer.iter() {
        assert_eq!(name.node, outer);
        let [RoleSeg::Member { at, of: inner_name }] = name.path.as_slice() else {
            panic!("one Member qualifier: {name:?}");
        };
        // A copy names its own body; every other entity wears its
        // source's name.
        if inner_name.node == outer {
            assert_eq!(inner_name.path, [RoleSeg::OutputBody], "{name:?}");
            continue;
        }
        let [(iv, i), (jv, j)] = at.as_slice() else {
            panic!("two indices: {name:?}");
        };
        assert_eq!((*iv, *jv), (i_var, j_var), "outer first");
        let master = inner_name;
        assert_eq!(master.node, cube, "over the master's own name: {name:?}");
        let inner_entry = of_inner
            .lookup(&in_member(
                inner,
                vec![(i_var, *i)],
                master.as_ref().clone(),
            ))
            .expect("the inner row exists");
        let at_flat = |e: &editor_core::EntityRef| editor_core::EntityRef {
            body: u32::try_from(flat(*i as usize, *j as usize)).unwrap(),
            key: e.key,
        };
        let expected = match inner_entry {
            Entry::Unique(e) => {
                assert_eq!(e.body, *i, "the inner row's body is its member's place");
                Entry::Unique(at_flat(e))
            }
            Entry::Tied(es) => Entry::Tied(es.iter().map(at_flat).collect()),
        };
        assert_eq!(entry, &expected, "{name:?} at body i·N + j");
    }
    for (ix, body) in nested.iter().enumerate() {
        let ix = ix as u32;
        let named = |key: EntityKey| {
            of_outer
                .name_of(&editor_core::EntityRef { body: ix, key })
                .is_some()
        };
        assert!(named(EntityKey::Body), "body {ix} is named");
        assert!(
            body.faces().all(|(f, _)| named(EntityKey::Face(f))),
            "faces of {ix}"
        );
        assert!(
            body.edges().all(|(e, _)| named(EntityKey::Edge(e))),
            "edges of {ix}"
        );
        assert!(
            body.vertices().all(|(v, _)| named(EntityKey::Vertex(v))),
            "vertices of {ix}"
        );
    }
}

// ---- one member of a family of families ----

/// **`outer[i, j]` reads member `(i, j)`**: the very body, in its frame,
/// its nested name resolving in the read's table verbatim; and a read
/// one past the inner count refuses `MemberOutOfRange` at the values
/// written. Red if a two-index read is a flat index.
#[test]
fn a_two_index_read_picks_its_member() {
    let (doc, cube, _, outer, i_var, j_var) = nested_doc("eval6-part");
    let mut doc = doc;
    let mut reads = Vec::new();
    for i in 0..M {
        for j in 0..N {
            let (next, p) =
                fixture::member_at(doc, outer, vec![Formula::count(i), Formula::count(j)]);
            doc = next;
            reads.push((i, j, p));
        }
    }
    let (doc, past) = fixture::member_at(doc, outer, vec![Formula::count(M), Formula::count(0)]);
    let ev = run(&doc, &opts());
    let ValuePayload::Instances(bodies) =
        &ev.value(outer).expect("the outer family evaluated").payload
    else {
        panic!("the outer placement is a family")
    };
    let cube_cap = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: cube,
        path: vec![RoleSeg::Cap(editor_core::CapEnd::End)],
    };
    for &(i, j, p) in &reads {
        let k = flat(i as usize, j as usize);
        let ValuePayload::Body(picked) = &ev.value(p).expect("the read evaluated").payload else {
            panic!("a member read is a body")
        };
        assert!(
            Arc::ptr_eq(picked.shared(), bodies[k].shared())
                && matches!(picked.relative(&bodies[k]), Ok(None)),
            "outer[{i}, {j}] is member body {k} itself, in its frame"
        );
        let at = |v: i64| u32::try_from(v).unwrap();
        let name = in_member(
            outer,
            vec![(i_var, at(i)), (j_var, at(j))],
            cube_cap.clone(),
        );
        assert!(
            ev.table_at(p).expect("the read").lookup(&name).is_some(),
            "the ({i}, {j}) name resolves in outer[{i}, {j}]'s table"
        );
    }
    assert!(
        matches!(
            ev.node_error(past).map(|e| &e.kind),
            Some(NodeErrorKind::MemberOutOfRange { input, at })
                if *input == outer && at.as_slice() == [M, 0]
        ),
        "one past the inner count refuses at the values written: {:?}",
        ev.node_error(past)
    );
}

// ---- the door ----

/// **The operand door.** A placement reads a body or a family and the
/// door refuses a split named alone, typed, before any evaluation: it
/// is two bodies, so the read names a port. A union reads a family
/// spelled at a member's seat as its every member (D10 `[repetition]`).
#[test]
fn the_placers_admit_a_body_or_a_family_and_a_union_reads_a_family_whole() {
    use editor_core::{EditError, OperandSlot};
    let (doc, cube) = cube_doc("eval6-door");
    let tool = fixture::plane_on(&doc, cube, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cube.into(),
            tool,
        },
    );
    let (doc, k, row) = fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, M);
    let target = fixture::space(&doc, cube);
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        fixture::slides_along(k, [0.0, 1.0, 0.0], 2.0),
    );
    let refusal = fixture::insert_refused(
        &doc,
        Node::place(
            BodyRead::plain(editor_core::Operand::Node(split)),
            editor_core::Operand::Var(target),
            placed.bundle(),
        ),
    );
    assert!(
        matches!(
            &refusal,
            EditError::AmbiguousOutput { input, slot: editor_core::SlotId::Operand(OperandSlot::Source), .. }
                if input.id() == split
        ),
        "a split named alone: {refusal:?}"
    );
    let (doc, both) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![row.into(), cube.into()]),
        },
    );
    let ev = run(&doc, &opts());
    let ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) = &ev
        .value(both)
        .unwrap_or_else(|| panic!("{:?}", ev.node_error(both)))
        .payload
    else {
        panic!("the union is a body")
    };
    // The cube is member 0 of its own row: three solids, the cube
    // fused with its copy.
    assert_eq!(body.body().solids().count() as i64, M);
}

// ---- the lanes the hosted matrix does not draw: Dual64 ----

fn run_dual(doc: &ProfileDoc) -> editor_core::Evaluation<geom_core::Dual64> {
    editor_core::evaluate::<geom_core::Dual64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &opts(),
        Tol::witness(),
    )
}

/// The nested layout's per-member identity at `Dual64`.
#[test]
fn a_family_of_families_is_one_member_per_pair_at_dual64() {
    let (doc, _cube, inner, outer, _, _) = nested_doc("eval6-c3-dual");
    let (doc, rows) = per_member(doc, inner);
    let ev = run_dual(&doc);
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M);
    for (i, row) in rows.iter().enumerate() {
        let alone = instances_of(&ev, *row);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[flat(i, j)]),
                bits(&alone[j]),
                "member ({i}, {j})"
            );
        }
    }
}
