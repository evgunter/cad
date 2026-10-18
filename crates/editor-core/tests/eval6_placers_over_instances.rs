//! **A pattern is shape-preserving over the value**: `Pattern` takes an
//! `Instances` value whole and yields `Instances` — a pattern of N
//! bodies is their `N·M` placements laid out placement-major
//! (`wire_pattern`'s layout: output body `j·M + i` is placement `j` of
//! inner body `i`, named `Instance(j)` over the inner name).
//!
//! Every identity here is BIT identity through the public doors: the
//! oracle for "the outer map applied to inner instance `i`" is the same
//! `Pattern` over `Part(Instance(i))` — the same kernel op on the same
//! body under the same map, so nothing is approximate and no rule is
//! restated. The
//! lane rows of the same claims are `eval6_placers_over_instances_interval`.
//! The digest here is the geometry, the topology and the arena keys —
//! the bits a consumer reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::AuthoredNode;
use std::sync::Arc;

use crate::fixture;

use editor_core::{
    DocumentId, EntityKey, Entry, EvalOptions, Node, NodeErrorKind, PartSelect, PatternKind,
    ProfileDoc, RecipeNodeId, RoleSeg, ValuePayload,
};
use fixture::{in_copy, insert, len, on_frame, run, scl};
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

/// A linear pattern of `count` along `dir` at `spacing`.
pub(crate) fn linear(input: RecipeNodeId, dir: [f64; 3], spacing: f64, count: i64) -> AuthoredNode {
    Node::Pattern {
        input: input.into(),
        count: editor_core::Formula::count(count),
        kind: PatternKind::Linear {
            direction: dir.map(scl),
            spacing: len(spacing),
        },
    }
}

/// `Part(Instance(i))` of `of`.
pub(crate) fn part(of: RecipeNodeId, i: i64) -> AuthoredNode {
    Node::Part {
        of: of.into(),
        select: PartSelect::Instance(editor_core::Formula::count(i)),
    }
}

/// The inner count and the outer count of the nested documents.
pub(crate) const M: i64 = 3;
pub(crate) const N: i64 = 2;

/// **The nested document**: a cube, an inner pattern of `M` along x,
/// an outer pattern of `N` along y over the inner's VALUE. Returns
/// `(doc, cube, inner, outer)`.
pub(crate) fn nested_doc(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let (doc, cube) = cube_doc(label);
    let (doc, inner) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, outer) = insert(doc, linear(inner, [0.0, 1.0, 0.0], 2.0, N));
    (doc, cube, inner, outer)
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

/// The `Instances` a node evaluated to.
pub(crate) fn instances_of<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    id: RecipeNodeId,
) -> Vec<Arc<Body<T>>> {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Instances(v) => v.clone(),
        other => panic!("{id:?} is a {}, not instances", other.kind_name()),
    }
}

/// The one body a node evaluated to.
pub(crate) fn body_of<T: geom_core::Decide>(
    ev: &editor_core::Evaluation<T>,
    id: RecipeNodeId,
) -> Arc<Body<T>> {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.node_error(id)))
        .payload
    {
        ValuePayload::Body(b) => Arc::clone(b),
        other => panic!("{id:?} is a {}, not a body", other.kind_name()),
    }
}

fn opts() -> EvalOptions {
    EvalOptions::default()
}

// ---- claim 3: the nested layout ----

/// **Claim 3.** `Pattern_N(Pattern_M(cube))` is `N·M` bodies; body
/// `j·M + i` is bit-identical to placement `j` of `Pattern_N(Part(i))`
/// — the outer rule over the inner instance alone, through the same
/// door; every name is `Instance(j)` over `Instance(i)` over a master
/// name, at row body `j·M + i`; and every entity of every output body
/// is named.
#[test]
fn a_nested_pattern_lays_out_placement_major() {
    let (doc, cube, inner, outer) = nested_doc("eval6-c3");
    let mut doc = doc;
    let mut per_instance = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(inner, i));
        let (next, over_one) = insert(next, linear(selected, [0.0, 1.0, 0.0], 2.0, N));
        doc = next;
        per_instance.push(over_one);
    }
    let ev = run(&doc, &opts());
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M, "N·M bodies");
    for (i, over_one) in per_instance.iter().enumerate() {
        let alone = instances_of(&ev, *over_one);
        assert_eq!(alone.len() as i64, N);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[j * M as usize + i]),
                bits(&alone[j]),
                "body j·M + i = {}·{M} + {i} is placement {j} of inner instance {i}",
                j
            );
        }
    }
    // The names: Instance(j) over Instance(i) over the cube's own name,
    // at body j·M + i, and the inner row it wraps sits at body i.
    let (of_inner, of_outer) = (
        ev.table_at(inner).expect("the inner"),
        ev.table_at(outer).expect("the outer"),
    );
    assert_eq!(of_outer.len(), of_inner.len() * N as usize);
    for (name, entry) in of_outer.iter() {
        assert_eq!(name.node, outer);
        let [
            RoleSeg::Instance {
                i: j,
                of: inner_name,
            },
        ] = name.path.as_slice()
        else {
            panic!("one Instance(j) qualifier: {name:?}");
        };
        assert_eq!(inner_name.node, inner);
        let [RoleSeg::Instance { i, of: master }] = inner_name.path.as_slice() else {
            panic!("over one Instance(i) qualifier: {name:?}");
        };
        assert_eq!(master.node, cube, "over the master's own name");
        let inner_entry = of_inner.lookup(inner_name).expect("the inner row exists");
        let flat = |e: &editor_core::EntityRef| editor_core::EntityRef {
            body: j * (M as u32) + e.body,
            key: e.key,
        };
        let expected = match inner_entry {
            Entry::Unique(e) => {
                assert_eq!(e.body, *i, "the inner row's body is its instance index");
                Entry::Unique(flat(e))
            }
            Entry::Tied(es) => Entry::Tied(es.iter().map(flat).collect()),
        };
        assert_eq!(entry, &expected, "{name:?} at body j·M + i");
    }
    // Totality over every output body.
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

// ---- item 5: Part over a nested value indexes the flat list ----

/// `Part(Instance(k))` over the nested value hands back body `k` of
/// the flat list — the very `Arc` — with the outer's table projected
/// onto it (the nested name resolves there verbatim), and refuses one
/// index past the flat count with that count.
#[test]
fn a_part_over_a_nested_pattern_indexes_the_flat_list() {
    let (doc, cube, inner, outer) = nested_doc("eval6-part");
    let mut doc = doc;
    let mut parts = Vec::new();
    for k in 0..=N * M {
        let (next, p) = insert(doc, part(outer, k));
        doc = next;
        parts.push(p);
    }
    let ev = run(&doc, &opts());
    let nested = instances_of(&ev, outer);
    for (k, p) in parts.iter().enumerate().take((N * M) as usize) {
        let body = body_of(&ev, *p);
        assert!(
            Arc::ptr_eq(&body, &nested[k]),
            "Part({k}) is flat body {k} itself"
        );
        let (j, i) = (k as i64 / M, k as i64 % M);
        let cube_body = editor_core::StableName {
            kind: editor_core::EntityKind::Body,
            node: cube,
            path: vec![RoleSeg::OutputBody],
        };
        let name = in_copy(outer, j as u32, in_copy(inner, i as u32, cube_body));
        assert_eq!(
            ev.table_at(*p).expect("the part").lookup(&name),
            Some(&Entry::Unique(editor_core::EntityRef {
                body: 0,
                key: EntityKey::Body,
            })),
            "the (j, i) = ({j}, {i}) name resolves in Part({k})'s table"
        );
    }
    let past = parts[(N * M) as usize];
    assert!(
        matches!(
            ev.node_error(past).map(|e| &e.kind),
            Some(NodeErrorKind::InstanceOutOfRange { index, count, .. })
                if *index == N * M && *count == (N * M) as usize
        ),
        "one past the flat count refuses with the flat count: {:?}",
        ev.node_error(past)
    );
}

// ---- the door ----

/// **The operand door.** A pattern reads a body or instances and the
/// door refuses everything else typed, before any evaluation: a split
/// named alone is two bodies, so the read names a port. A boolean reads
/// ONE body and refuses instances — the asymmetry
/// `ValuePayload::Instances` states — as a kind at the same door.
#[test]
fn the_placers_admit_a_body_or_instances_and_the_boolean_one_body() {
    use editor_core::{EditError, OperandSlot, SlotKind, VarKind};
    let (doc, cube) = cube_doc("eval6-door");
    let tool = fixture::plane_on(&doc, cube, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cube.into(),
            tool,
        },
    );
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let refusal = fixture::insert_refused(&doc, linear(split, [0.0, 1.0, 0.0], 2.0, N));
    assert!(
        matches!(
            &refusal,
            EditError::AmbiguousOutput { input, slot: editor_core::SlotId::Operand(OperandSlot::Input), .. } if input.id() == split
        ),
        "a split named alone: {refusal:?}"
    );
    let (doc, pat_pattern) = insert(doc, linear(pattern, [0.0, 1.0, 0.0], 2.0, N));
    let refusal = fixture::insert_refused(
        &doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![pattern.into(), cube.into()]),
        },
    );
    assert!(
        matches!(
            &refusal,
            EditError::SlotVarKind {
                slot: editor_core::SlotId::Operand(OperandSlot::Member(0)),
                found: VarKind::Bodies,
                expected: SlotKind::Is(VarKind::Body),
                ..
            }
        ),
        "a boolean reads one body: {refusal:?}"
    );
    let ev = run(&doc, &opts());
    assert_eq!(instances_of(&ev, pat_pattern).len() as i64, N * M);
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

/// Claim 3's per-body identity at `Dual64`.
#[test]
fn a_nested_pattern_lays_out_placement_major_at_dual64() {
    let (doc, _cube, inner, outer) = nested_doc("eval6-c3-dual");
    let mut doc = doc;
    let mut per_instance = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(inner, i));
        let (next, over_one) = insert(next, linear(selected, [0.0, 1.0, 0.0], 2.0, N));
        doc = next;
        per_instance.push(over_one);
    }
    let ev = run_dual(&doc);
    let nested = instances_of(&ev, outer);
    assert_eq!(nested.len() as i64, N * M);
    for (i, over_one) in per_instance.iter().enumerate() {
        let alone = instances_of(&ev, *over_one);
        for j in 0..N as usize {
            assert_eq!(
                bits(&nested[j * M as usize + i]),
                bits(&alone[j]),
                "body {j}·{M} + {i}"
            );
        }
    }
}
