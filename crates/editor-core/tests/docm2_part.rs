//! **DOCM-2 — `Node::Part` at f64** (DOCM-REFERENCES-DESIGN DM3):
//! acceptance rows A1–A6, the split-stamping row the stop clause's
//! amendment asks for, and the `Dual64` pin of the relaxed
//! same-source assertions on the exact corpus document. The
//! Interval-lane rows (A7) are `docm2_part_interval`.
//!
//! The oracle for "the half IS the half" is the kernel's own door fed
//! the body read straight off the split's or the pattern's value: a
//! consumer of a `Part` must produce the same body, description for
//! description, that the door produces from that body — and the
//! `Part`'s value must be that body's own `Arc`, not a copy of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::fixture::{Recorder, ang, len, scl};

use editor_core::{
    BooleanOp, CancelToken, Datum, Denotation, DocEdit, EditError, EntityKey, EntityKind, Entry,
    EvalOptions, Evaluation, Expr, Node, NodeError, NodeErrorKind, NodeResult, PartSelect,
    PatternKind, ProfileDoc, RecipeNodeId, ResolveError, RoleSeg, SlotId, SplitHalf, SplitSide,
    StableName, ValuePayload, all_edges, apply, denotation, evaluate, product,
};
use geom_core::{Affine3, Dual, Mat3, Tol, Vec3};
use topo::{Body, BooleanResult, mass_properties, transform_rigid};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    eval_after(doc, None)
}

fn eval_after(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The unit box `[0, 1]³`, or one shifted along x by `x0`.
fn unit_box(r: &mut Recorder, x0: f64) -> RecipeNodeId {
    let p = r.profile(
        [x0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    })
}

/// A horizontal tool plane at height `z`, normal +z.
fn plane_z(r: &mut Recorder, z: f64) -> RecipeNodeId {
    r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(z)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }))
}

fn part(r: &mut Recorder, of: RecipeNodeId, select: PartSelect) -> RecipeNodeId {
    r.insert(Node::Part { of, select })
}

fn half(h: SplitHalf) -> PartSelect {
    PartSelect::SplitHalf(h)
}

fn instance(i: i64) -> PartSelect {
    PartSelect::Instance(Expr::count(i))
}

/// A three-instance linear pattern of `input`, three metres apart.
fn pattern3(r: &mut Recorder, input: RecipeNodeId) -> RecipeNodeId {
    r.insert(Node::Pattern {
        input,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    })
}

fn lift(r: &mut Recorder, input: RecipeNodeId, dz: f64) -> RecipeNodeId {
    r.insert(Node::Transform {
        input,
        translation: [len(0.0), len(0.0), len(dz)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    })
}

/// The `Body` value of a node — a Part's, a transform's — as the Arc it
/// holds.
fn body_arc(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Arc<Body<f64>> {
    match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b,
        other => panic!("node {} is a {}, not a body", id.0, other.kind_name()),
    }
}

/// A single body read off any single-body value — a `Body` or a
/// boolean's non-empty result.
fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Body<f64> {
    corpus::body_of(ev, id)
}

/// Both halves read straight off the split's value.
fn sides(ev: &Evaluation<f64>, split: RecipeNodeId) -> (Arc<Body<f64>>, Arc<Body<f64>>) {
    let ValuePayload::Split { above, below } = &ev.value(split).expect("the split").payload else {
        panic!("a split value");
    };
    let body = |s: &SplitSide<f64>| match s {
        SplitSide::Body(b) => Arc::clone(b),
        SplitSide::Empty => panic!("both halves carry material"),
    };
    (body(above), body(below))
}

/// The instances read straight off the pattern's value.
fn instances(ev: &Evaluation<f64>, pattern: RecipeNodeId) -> Vec<Arc<Body<f64>>> {
    let ValuePayload::Instances(v) = &ev.value(pattern).expect("the pattern").payload else {
        panic!("an instances value");
    };
    v.clone()
}

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {} must fail typed, got {other:?}", id.0),
    }
}

/// `text` with every arena key (`SurfaceKey(..)`, `CurveKey(..)`, …)
/// blanked: a description quotes the keys of the descriptions it is
/// derived from, and keys are body-lineage-scoped — a product gather
/// re-keys what it grafts — so a description-for-description
/// comparison reads through them.
fn key_free(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("Key(") {
        let (head, tail) = rest.split_at(at + "Key(".len());
        out.push_str(head);
        out.push('_');
        rest = &tail[tail.find(')').expect("a closed key")..];
    }
    out.push_str(rest);
    out
}

/// Every description of a body, as sorted key-free text: surfaces,
/// curves and points, plus the arena counts. Two bodies with equal
/// bits here are the same body description for description
/// (`docm3_union`'s instrument).
fn bits<T: geom_core::Decide + core::fmt::Debug>(b: &Body<T>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    v.push(format!(
        "counts f{} e{} v{}",
        b.faces().count(),
        b.edges().count(),
        b.vertices().count()
    ));
    let mut s: Vec<String> = b
        .surfaces()
        .map(|(_, s)| key_free(&format!("S {s:?}")))
        .collect();
    let mut c: Vec<String> = b
        .curves()
        .map(|(_, c)| key_free(&format!("C {c:?}")))
        .collect();
    let mut p: Vec<String> = b
        .points()
        .map(|(_, p)| key_free(&format!("P {p:?}")))
        .collect();
    s.sort();
    c.sort();
    p.sort();
    v.extend(s);
    v.extend(c);
    v.extend(p);
    v
}

/// The kernel's pair union of two bodies, no declaration — the same
/// door and strategy the document's `Boolean(Union)` runs.
fn kernel_union(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    match topo::union(a, b, Tol::witness()).expect("the kernel union succeeds") {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!("a union of material is not empty"),
    }
}

/// The edge keys `names` resolve to in `table` — the same keys the
/// fillet's own resolution hands the kernel.
fn edge_keys(ev: &Evaluation<f64>, node: RecipeNodeId, names: &[StableName]) -> Vec<topo::EdgeKey> {
    let table = &ev.value(node).expect("a value").name_table;
    let mut keys: Vec<topo::EdgeKey> = names
        .iter()
        .map(|n| match table.lookup(n) {
            Some(Entry::Unique(e)) => match e.key {
                EntityKey::Edge(k) => k,
                other => panic!("{n} is not an edge: {other:?}"),
            },
            other => panic!("{n} does not resolve uniquely: {other:?}"),
        })
        .collect();
    keys.sort_unstable();
    keys
}

const LIFT: f64 = 2.0;
const RADIUS: f64 = 0.05;

/// **A1 — the half IS the half.** For each half: the Part's value is
/// the side's own `Arc`; a transform, a union and a fillet of the Part
/// are, description for description, the kernel's own doors run on the
/// body read off the split's value; and the memo serves the split to
/// the consumers added later.
#[test]
fn a1_the_half_is_the_half_through_a_transform_a_boolean_and_a_fillet() {
    for h in SplitHalf::ALL {
        let mut r = Recorder::new();
        let cube = unit_box(&mut r, 0.0);
        let other = unit_box(&mut r, 3.0);
        let tool = plane_z(&mut r, 0.5);
        let split = r.insert(Node::Split { target: cube, tool });
        let p = part(&mut r, split, half(h));
        let moved = lift(&mut r, p, LIFT);
        let first = eval(&r.doc);
        assert!(
            corpus::failures(&first).is_empty(),
            "{h:?}: {:?}",
            corpus::failures(&first)
        );
        let before = r.doc.len();

        // The two consumers added afterwards: the split and the Part
        // are served from the memo, only the new nodes compute.
        let selection = all_edges(&first, p);
        assert!(!selection.is_empty(), "the half has edges");
        let joined = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: p,
            b: other,
            declare: None,
        });
        let rounded = r.insert(Node::fillet(p, len(RADIUS), selection.clone()));
        let ev = eval_after(&r.doc, Some(&first));
        assert!(
            corpus::failures(&ev).is_empty(),
            "{h:?}: {:?}",
            corpus::failures(&ev)
        );
        assert_eq!(
            ev.reused, before,
            "{h:?}: everything already evaluated is reused"
        );
        assert_eq!(ev.recomputed, 2, "{h:?}: the boolean and the fillet");

        // The Part's body is the side's own Arc — the same allocation,
        // not a clone of it.
        let (above, below) = sides(&ev, split);
        let side = match h {
            SplitHalf::Above => &above,
            SplitHalf::Below => &below,
        };
        assert!(
            Arc::ptr_eq(body_arc(&ev, p), side),
            "{h:?}: the Part holds the side's Arc"
        );

        // Each consumer against the kernel door fed the side directly.
        let map = Affine3::from_parts(
            Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), 0.0),
            Vec3::new(0.0, 0.0, LIFT),
        );
        let placed = transform_rigid(side, &map, Tol::witness()).expect("a rigid placement");
        assert_eq!(
            bits(body_arc(&ev, moved)),
            bits(&placed),
            "{h:?}: transform"
        );
        let fused = kernel_union(side, body_of(&ev, other));
        assert_eq!(bits(body_of(&ev, joined)), bits(&fused), "{h:?}: boolean");
        let keys = edge_keys(&ev, p, &selection);
        let filleted = sweep::blend::build::fillet_edges(side, &keys, RADIUS, Tol::witness())
            .expect("the kernel fillet succeeds");
        assert_eq!(
            bits(body_arc(&ev, rounded)),
            bits(&filleted.body),
            "{h:?}: fillet"
        );
    }
}

/// **A2 — the instance IS the instance.** `Part(1) ∪ Part(2)` is the
/// kernel's pair union of `v[1]` and `v[2]` read off the value, and
/// `Part(0)` is the input body's own `Arc`: instance 0 is the input
/// itself, through the pattern and through the Part.
#[test]
fn a2_the_instance_is_the_instance() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let pat = pattern3(&mut r, cube);
    let p0 = part(&mut r, pat, instance(0));
    let p1 = part(&mut r, pat, instance(1));
    let p2 = part(&mut r, pat, instance(2));
    let joined = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: p1,
        b: p2,
        declare: None,
    });
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let v = instances(&ev, pat);
    assert_eq!(v.len(), 3);
    for (p, i) in [(p0, 0), (p1, 1), (p2, 2)] {
        assert!(
            Arc::ptr_eq(body_arc(&ev, p), &v[i]),
            "Part({i}) holds instance {i}'s Arc"
        );
    }
    assert!(
        Arc::ptr_eq(body_arc(&ev, p0), body_arc(&ev, cube)),
        "instance 0 is the input body itself"
    );
    let fused = kernel_union(&v[1], &v[2]);
    assert_eq!(bits(body_of(&ev, joined)), bits(&fused));
}

/// The edge names of ONE output body of a node's table.
fn edges_of_body(ev: &Evaluation<f64>, node: RecipeNodeId, body: u32) -> Vec<StableName> {
    ev.value(node)
        .expect("a value")
        .name_table
        .iter()
        .filter_map(|(name, entry)| match entry {
            Entry::Unique(e) if e.body == body && name.kind == EntityKind::Edge => {
                Some(name.clone())
            }
            _ => None,
        })
        .collect()
}

/// **A3 — names pass through, and only the selected body's.** A fillet
/// spelled against the split's own above-half edge rows resolves on
/// `Part(Above)`, every name uniquely; a selector for instance 2
/// against `Part(1)` refuses `Vanished` through the N5 ladder and is
/// never re-anchored; `Part(1)`'s table is the master's row count and
/// every name carries `Instance { i: 1 }`.
#[test]
fn a3_names_pass_through_and_only_the_selected_bodys() {
    // The split's own rows, by output body.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split { target: cube, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    let base = eval(&r.doc);
    let spelled = edges_of_body(&base, split, SplitHalf::Above.output_body());
    let below_rows = edges_of_body(&base, split, SplitHalf::Below.output_body());
    assert!(!spelled.is_empty() && !below_rows.is_empty());
    let mut selection = spelled.clone();
    selection.sort();
    selection.dedup();
    let rounded = r.insert(Node::fillet(above, len(RADIUS), selection.clone()));
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let _ = rounded;
    for name in &selection {
        assert_eq!(
            denotation(&ev, above, name),
            Ok(Denotation::Unique),
            "{name} resolves on the Part as it did on the split"
        );
    }
    assert_eq!(
        all_edges(&ev, above).len(),
        spelled.len(),
        "the Part's edge rows are exactly the above half's"
    );
    // A below-half row finds no row on Part(Above): absent, not
    // re-anchored to a congruent above-half edge.
    for name in &below_rows {
        assert_eq!(
            denotation(&ev, above, name),
            Err(editor_core::InterrogateError::NoSuchName),
            "{name} is the other half's"
        );
    }

    // The pattern side.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let pat = pattern3(&mut r, cube);
    let p1 = part(&mut r, pat, instance(1));
    let base = eval(&r.doc);
    let master = base.value(cube).expect("the master").name_table.len();
    let table = &base.value(p1).expect("the Part").name_table;
    assert_eq!(table.len(), master, "exactly the master's row count");
    for (name, _) in table.iter() {
        assert!(
            matches!(name.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
            "{name} carries Instance {{ i: 1 }}: {:?}",
            name.path
        );
    }
    // `Instance { i: 2, .. }` spelled against Part(1): the ladder's
    // third rung — the node is live, the table lacks it.
    let edge = all_edges(&base, p1)
        .into_iter()
        .next()
        .expect("an edge of instance 1");
    let Some(RoleSeg::Instance { of, .. }) = edge.path.first() else {
        panic!("an instance name");
    };
    let other_instance = StableName {
        kind: EntityKind::Edge,
        node: pat,
        path: vec![RoleSeg::Instance {
            i: 2,
            of: of.clone(),
        }],
    };
    let rounded = r.insert(Node::fillet(p1, len(RADIUS), vec![other_instance]));
    let ev = eval(&r.doc);
    match error_of(&ev, rounded) {
        NodeErrorKind::BlendSelectionResolve { error, .. } => assert!(
            matches!(**error, ResolveError::Vanished { .. }),
            "the N5 arm the situation warrants: {error}"
        ),
        other => panic!("the fillet must refuse through the ladder, got {other:?}"),
    }
}

/// **A4 — refusals, one row each, typed.**
#[test]
fn a4_every_refusal_is_typed() {
    // A plane that misses the box: the empty side's Part refuses
    // `EmptyHalf`, the other's evaluates.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 2.0);
    let split = r.insert(Node::Split { target: cube, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    let below = part(&mut r, split, half(SplitHalf::Below));
    let ev = eval(&r.doc);
    assert!(
        matches!(
            error_of(&ev, above),
            NodeErrorKind::EmptyHalf { input, half: SplitHalf::Above } if *input == split
        ),
        "{:?}",
        error_of(&ev, above)
    );
    assert!(
        ev.value(below).is_some(),
        "the side with material evaluates"
    );

    // The index at the count and negative; then the count lowered under
    // a live index.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let pat = pattern3(&mut r, cube);
    let at_count = part(&mut r, pat, instance(3));
    let negative = part(&mut r, pat, instance(-1));
    let live = part(&mut r, pat, instance(2));
    let ev = eval(&r.doc);
    assert!(
        matches!(
            error_of(&ev, at_count),
            NodeErrorKind::InstanceOutOfRange { input, index: 3, count: 3 } if *input == pat
        ),
        "{:?}",
        error_of(&ev, at_count)
    );
    assert!(
        matches!(
            error_of(&ev, negative),
            NodeErrorKind::InstanceOutOfRange {
                index: -1,
                count: 3,
                ..
            }
        ),
        "{:?}",
        error_of(&ev, negative)
    );
    assert!(ev.value(live).is_some());
    let lowered = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: pat,
            slot: SlotId::Count,
            expr: Expr::count(2),
        },
        Tol::witness(),
    )
    .expect("a structural edit")
    .doc;
    let ev = eval_after(&lowered, Some(&ev));
    assert!(ev.value(pat).is_some(), "the pattern is Ok at two");
    assert!(
        matches!(
            error_of(&ev, live),
            NodeErrorKind::InstanceOutOfRange {
                index: 2,
                count: 2,
                ..
            }
        ),
        "{:?}",
        error_of(&ev, live)
    );
    assert!(ev.recomputed >= 2, "the pattern and the Parts recompute");

    // The selector and the value must agree in kind.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split { target: cube, tool });
    let pat = pattern3(&mut r, cube);
    let half_of_pattern = part(&mut r, pat, half(SplitHalf::Above));
    let index_of_split = part(&mut r, split, instance(0));
    let half_of_body = part(&mut r, cube, half(SplitHalf::Below));
    let index_of_body = part(&mut r, cube, instance(0));
    let ev = eval(&r.doc);
    for (node, expected, found) in [
        (half_of_pattern, "split", "instances"),
        (index_of_split, "instances", "split"),
        (half_of_body, "split", "body"),
        (index_of_body, "instances", "body"),
    ] {
        assert!(
            matches!(
                error_of(&ev, node),
                NodeErrorKind::WrongOperand { expected: e, found: f, .. } if *e == expected && *f == found
            ),
            "{expected} on {found}: {:?}",
            error_of(&ev, node)
        );
    }

    // The index is structural: `SetParam` refuses it.
    let refused = apply(
        &r.doc,
        &DocEdit::SetParam {
            node: index_of_split,
            slot: SlotId::Instance,
            expr: Expr::count(1),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::StructuralSlotNeedsStructuralEdit {
                slot: SlotId::Instance
            })
        ),
        "{refused:?}"
    );
}

/// **A5 — the key separates what the memo must separate.** The two
/// halves key apart, the two instances key apart, and an edit of the
/// index recomputes the Part and nothing upstream. (The tag census is
/// `eval::verb_content_tag_tests::node_tag_space_is_injective`.)
#[test]
fn a5_the_content_key_separates_the_halves_and_the_instances() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split { target: cube, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    let below = part(&mut r, split, half(SplitHalf::Below));
    let pat = pattern3(&mut r, cube);
    let p1 = part(&mut r, pat, instance(1));
    let p2 = part(&mut r, pat, instance(2));
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let key = |id| ev.value(id).expect("a value").content_key;
    assert_ne!(key(above), key(below), "the two halves of one split");
    assert_ne!(key(p1), key(p2), "two instances of one pattern");

    let edited = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: p1,
            slot: SlotId::Instance,
            expr: Expr::count(2),
        },
        Tol::witness(),
    )
    .expect("a structural edit")
    .doc;
    let again = eval_after(&edited, Some(&ev));
    assert_eq!(again.recomputed, 1, "the Part alone");
    assert_eq!(again.reused, r.doc.len() - 1, "nothing upstream moves");
    assert_eq!(
        again.value(p1).expect("the Part").content_key,
        key(p2),
        "an index of 2 keys as the other Part of 2 does"
    );
}

/// **The product of a document whose only root is `Part(Above)` is
/// that one half** — the unselected half is in no product (A7's
/// product row; `sources_of`'s doc says so).
#[test]
fn a7_the_product_of_a_lone_part_root_is_that_half() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split { target: cube, tool });
    let above = part(&mut r, split, half(SplitHalf::Above));
    assert_eq!(r.doc.roots(), &[above], "the Part is the only sink");
    let ev = eval(&r.doc);
    let body = product(&r.doc, &ev, Tol::witness()).expect("the product gathers");
    let m = mass_properties(&body, Tol::witness()).expect("mass properties");
    assert_eq!(m.volume, 0.5, "the upper half of the unit box, exactly");
    let (side, _) = sides(&ev, split);
    assert_eq!(bits(&body), bits(&side));
}

/// **The split-stamping row** (the stop clause's amendment, item 1):
/// a boolean of the two halves of one split succeeds at f64, and the
/// two section planes carry DISTINCT sources — their descriptions are
/// opposed bit for bit, so one source over both would violate the
/// same-source theorem and, at rung 1, read two opposed planes as one.
#[test]
fn the_two_section_planes_of_one_split_carry_distinct_sources() {
    let cd = corpus::part_select::document();
    let ev = eval(&cd.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let split = *cd
        .doc
        .order()
        .iter()
        .find(|id| matches!(cd.doc.node(**id), Some(Node::Split { .. })))
        .expect("the split");
    let (above, below) = sides(&ev, split);
    let section = |b: &Body<f64>| {
        let minted: Vec<_> = b
            .surfaces()
            .filter_map(|(k, s)| {
                b.surface_source(k)
                    .filter(|src| src.node == split.0)
                    .map(|src| (src.clone(), s.clone()))
            })
            .collect();
        assert_eq!(minted.len(), 1, "one section plane per half");
        minted.into_iter().next().unwrap()
    };
    let (src_a, plane_a) = section(&above);
    let (src_b, plane_b) = section(&below);
    assert!(
        !src_a.same_base(&src_b),
        "distinct sources: {src_a:?} / {src_b:?}"
    );
    let (topo::Surface::Plane { normal: na, .. }, topo::Surface::Plane { normal: nb, .. }) =
        (plane_a, plane_b)
    else {
        panic!("planes")
    };
    assert_eq!(
        format!("{na:?}"),
        format!("{:?}", -nb),
        "the two section planes face away from each other"
    );
}

/// **The `Dual64` pin** (the amendment, item 2): the exact corpus
/// document — whose union rejoins two pieces carrying one pass-through
/// source — evaluates green at a scalar with no bit channel. The value
/// channel equalling f64's is `m10_di_dual_corpus`'s row; this one
/// names the document.
#[test]
fn the_part_select_document_evaluates_at_dual64() {
    let cd = corpus::part_select::document();
    let ev = corpus::eval::<Dual<f64>>(&cd.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
}
