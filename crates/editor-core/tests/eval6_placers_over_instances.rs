//! **The placers are shape-preserving over the value**: `Transform`
//! and `Pattern` take an `Instances` value whole and yield
//! `Instances` — a transform of N bodies is N transforms under one
//! map, and a pattern of N bodies is their `N·M` placements laid out
//! placement-major (`wire_pattern`'s layout: output body `j·M + i` is
//! placement `j` of inner body `i`, named `Instance(j)` over the
//! inner name).
//!
//! Every identity here is BIT identity through the public doors: the
//! oracle for "the transform of instance `i`" is a `Transform` over
//! `Part(Instance(i))` with the same slots, and the oracle for "the
//! outer map applied to inner instance `i`" is the same `Pattern` over
//! `Part(Instance(i))` — the same kernel op on the same body under the
//! same map, so nothing is approximate and no rule is restated. The
//! lane rows of the same claims are `eval6_placers_over_instances_interval`.
//!
//! What is NOT compared is provenance: a placing node stamps every
//! description `Placed { node, instance }` with its own id and the
//! body's ordinal in its value, so two nodes' bodies never share a
//! source by construction, and the digest here is the geometry, the
//! topology and the arena keys — the bits a consumer reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
            profile,
            distance: len(1.0),
        },
    )
}

/// A linear pattern of `count` along `dir` at `spacing`.
pub(crate) fn linear(
    input: RecipeNodeId,
    dir: [f64; 3],
    spacing: f64,
    count: i64,
) -> Node<editor_core::ProfileProgram> {
    Node::Pattern {
        input,
        count: editor_core::Expr::count(count),
        kind: PatternKind::Linear {
            direction: dir.map(scl),
            spacing: len(spacing),
        },
    }
}

/// `Part(Instance(i))` of `of`.
pub(crate) fn part(of: RecipeNodeId, i: i64) -> Node<editor_core::ProfileProgram> {
    Node::Part {
        of,
        select: PartSelect::Instance(editor_core::Expr::count(i)),
    }
}

/// A transform that both rotates and translates, so a transform-of-
/// pattern and a pattern-of-transform are different placements.
pub(crate) fn skew(input: RecipeNodeId) -> Node<editor_core::ProfileProgram> {
    fixture::xform(input, [1.0, 2.0, 3.0], [1.0, 1.0, 1.0], 0.7)
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

// ---- claim 1: a transform over Instances is N transforms ----

/// **Claim 1.** `Transform(pattern)` is `Instances` of the pattern's
/// count, and its body `i` is bit-identical to `Transform(Part(i))`
/// under the same slots — the same map on the same body through the
/// same door.
#[test]
fn a_transform_of_a_pattern_is_the_transform_of_each_instance() {
    let (doc, cube) = cube_doc("eval6-c1");
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, whole) = insert(doc, skew(pattern));
    let mut doc = doc;
    let mut each = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(pattern, i));
        let (next, moved) = insert(next, skew(selected));
        doc = next;
        each.push(moved);
    }
    let ev = run(&doc, &opts());
    let placed = instances_of(&ev, whole);
    assert_eq!(placed.len() as i64, M, "N bodies in, N bodies out");
    for (i, moved) in each.iter().enumerate() {
        assert_eq!(
            bits(&placed[i]),
            bits(&body_of(&ev, *moved)),
            "body {i} of the transform of the pattern is the transform of instance {i}"
        );
    }
}

// ---- claim 2: names pass through ----

/// **Claim 2.** The transform contributes no segment: its table IS the
/// pattern's — the same rows, the same keys, the same body indices —
/// and is the same allocation, not a copy that happens to agree.
#[test]
fn a_transform_of_a_pattern_passes_the_patterns_table_through() {
    let (doc, cube) = cube_doc("eval6-c2");
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, whole) = insert(doc, skew(pattern));
    let ev = run(&doc, &opts());
    let (of_pattern, of_transform) = (
        &ev.value(pattern).expect("the pattern").name_table,
        &ev.value(whole).expect("the transform").name_table,
    );
    assert!(
        Arc::ptr_eq(of_pattern, of_transform),
        "the transform's table is the pattern's own"
    );
    assert_eq!(**of_pattern, **of_transform);
    // And every row resolves in the placed value: instance `i`'s rows
    // name body `i`, which is the body the transform placed there.
    let placed = instances_of(&ev, whole);
    for (name, entry) in of_transform.iter() {
        let RoleSeg::Instance { i, .. } = &name.path[0] else {
            panic!("a pattern row wears Instance(i): {name:?}");
        };
        let rows = match entry {
            Entry::Unique(e) => vec![*e],
            Entry::Tied(es) => es.clone(),
        };
        for e in rows {
            assert_eq!(e.body, *i, "row body is the instance index");
            let body = &placed[e.body as usize];
            let live = match e.key {
                EntityKey::Body => true,
                EntityKey::Face(f) => body.get_face(f).is_some(),
                EntityKey::Edge(k) => body.get_edge(k).is_some(),
                EntityKey::Vertex(v) => body.get_vertex(v).is_some(),
            };
            assert!(live, "{name:?} names a live entity of the placed body");
        }
    }
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
        &ev.value(inner).expect("the inner").name_table,
        &ev.value(outer).expect("the outer").name_table,
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
            ev.value(*p).expect("the part").name_table.lookup(&name),
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

/// **The operand door.** A placer takes a body or instances and
/// refuses everything else typed, naming both admitted shapes; a
/// boolean still takes ONE body and refuses instances — the asymmetry
/// `ValuePayload::Instances` states.
#[test]
fn the_placers_admit_a_body_or_instances_and_the_boolean_one_body() {
    let (doc, cube) = cube_doc("eval6-door");
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cube,
            tool: plane,
        },
    );
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, xf_split) = insert(doc, skew(split));
    let (doc, xf_plane) = insert(doc, skew(plane));
    let (doc, pat_split) = insert(doc, linear(split, [0.0, 1.0, 0.0], 2.0, N));
    let (doc, pat_pattern) = insert(doc, linear(pattern, [0.0, 1.0, 0.0], 2.0, N));
    let (doc, xf_pattern) = insert(doc, skew(pattern));
    let (doc, boolean) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: pattern,
            b: cube,
            declare: None,
        },
    );
    let ev = run(&doc, &opts());
    for (node, found) in [
        (xf_split, "split"),
        (xf_plane, "datum"),
        (pat_split, "split"),
    ] {
        assert!(
            matches!(
                ev.node_error(node).map(|e| &e.kind),
                Some(NodeErrorKind::WrongOperand { expected, found: f, .. })
                    if *expected == "body or instances" && *f == found
            ),
            "{found}: {:?}",
            ev.node_error(node)
        );
    }
    assert_eq!(instances_of(&ev, pat_pattern).len() as i64, N * M);
    assert_eq!(instances_of(&ev, xf_pattern).len() as i64, M);
    assert!(
        matches!(
            ev.node_error(boolean).map(|e| &e.kind),
            Some(NodeErrorKind::WrongOperand {
                expected: "body",
                found: "instances",
                ..
            })
        ),
        "a boolean takes one body: {:?}",
        ev.node_error(boolean)
    );
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

/// Claim 1 at `Dual64`.
#[test]
fn a_transform_of_a_pattern_is_the_transform_of_each_instance_at_dual64() {
    let (doc, cube) = cube_doc("eval6-c1-dual");
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, whole) = insert(doc, skew(pattern));
    let mut doc = doc;
    let mut each = Vec::new();
    for i in 0..M {
        let (next, selected) = insert(doc, part(pattern, i));
        let (next, moved) = insert(next, skew(selected));
        doc = next;
        each.push(moved);
    }
    let ev = run_dual(&doc);
    let placed = instances_of(&ev, whole);
    assert_eq!(placed.len() as i64, M);
    for (i, moved) in each.iter().enumerate() {
        assert_eq!(bits(&placed[i]), bits(&body_of(&ev, *moved)), "body {i}");
    }
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

// ---- the order, and the stamps ----

/// Under a ROTATION, `Transform(Pattern)` and `Pattern(Transform)` are
/// different placements body for body past body 0, and the map is
/// applied to EVERY body of the value — none is passed through unmoved.
#[test]
fn a_transform_of_a_pattern_is_not_a_pattern_of_a_transform_under_rotation() {
    let (doc, cube) = cube_doc("eval6-order");
    let (doc, pattern) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, M));
    let (doc, t_of_p) = insert(doc, skew(pattern));
    let (doc, moved) = insert(doc, skew(cube));
    let (doc, p_of_t) = insert(doc, linear(moved, [1.0, 0.0, 0.0], 2.0, M));
    let ev = run(&doc, &opts());
    let a = instances_of(&ev, t_of_p);
    let b = instances_of(&ev, p_of_t);
    let p = instances_of(&ev, pattern);
    assert_eq!(a.len(), b.len());
    assert_eq!(
        bits(&a[0]),
        bits(&b[0]),
        "body 0 is the skewed cube either way"
    );
    for i in 1..a.len() {
        assert_ne!(
            bits(&a[i]),
            bits(&b[i]),
            "body {i}: the rotation separates the orders"
        );
    }
    for i in 0..a.len() {
        assert_ne!(
            bits(&a[i]),
            bits(&p[i]),
            "body {i} of the transform is MOVED"
        );
    }
}

/// The outermost placement stamp of a body's first surface: the placing
/// node and ordinal, or `None` for a description minted rather than
/// placed.
fn outer_stamp(b: &Body<f64>) -> Option<(u64, u32)> {
    let (key, _) = b.surfaces().next().expect("a cube has surfaces");
    match &b
        .surface_source(key)
        .expect("a placed description is sourced")
        .expr
    {
        topo::SourceExpr::Placed { node, instance, .. } => Some((*node, *instance)),
        topo::SourceExpr::Minted { .. } => None,
    }
}

/// **The stamps are pairwise distinct across a value's bodies** —
/// `compose_placed`'s ordinal rule, pinned on the two values that
/// would collide under a constant ordinal: a nested pattern (placement
/// 0 carries the inner's own stamps; every placed body wears the
/// outer node at its flat index) and a transform of a pattern (body
/// `i` wears the transform at `i`). A stamp shared by two bodies of
/// one node would read as one source over two geometries at a
/// boolean's identity rung.
#[test]
fn placement_stamps_are_pairwise_distinct_across_a_values_bodies() {
    let (doc, _cube, inner, outer) = nested_doc("eval6-stamps");
    let (doc, moved) = insert(doc, skew(inner));
    let ev = run(&doc, &opts());
    for (what, node, bodies) in [
        ("the nested pattern", outer, instances_of(&ev, outer)),
        (
            "the transform of the pattern",
            moved,
            instances_of(&ev, moved),
        ),
    ] {
        let stamps: Vec<Option<(u64, u32)>> = bodies.iter().map(|b| outer_stamp(b)).collect();
        for (x, sx) in stamps.iter().enumerate() {
            for (y, sy) in stamps.iter().enumerate().skip(x + 1) {
                assert_ne!(sx, sy, "{what}: bodies {x} and {y} share a stamp {sx:?}");
            }
        }
        // And every body this node PLACED wears this node at its own
        // flat index; the ones it passed through verbatim do not wear
        // it at all.
        for (k, stamp) in stamps.iter().enumerate() {
            match stamp {
                Some((by, ordinal)) if *by == node.0 => {
                    assert_eq!(*ordinal as usize, k, "{what}: body {k}'s ordinal")
                }
                _ => assert!(
                    node == outer && k < M as usize,
                    "{what}: body {k} is unstamped by its node yet is not a verbatim placement 0"
                ),
            }
        }
    }
}
