//! **DOCM-2 — one body of a multi-body value at f64**
//! (`crates/editor-core/REFERENCES.md` DM3): a read of a split's port,
//! and `Node::Part` over a pattern's copies. Acceptance rows A1–A6, the
//! split-stamping row the stop clause's amendment asks for, and the
//! `Dual64` pin of the relaxed same-source assertions on the exact
//! corpus document. The Interval-lane rows (A7) are
//! `docm2_part_interval`.
//!
//! The oracle for "the half IS the half" is the kernel's own door fed
//! the body read straight off the split's or the pattern's value: a
//! consumer of a port or a `Part` must produce the same body,
//! description for description, that the door produces from that body
//! — and a `Part`'s value must be that body's own `Arc`, not a copy of
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::sync::Arc;

use crate::corpus;
use crate::fixture::{Recorder, ang, len, scl};

use editor_core::{
    BooleanOp, CancelToken, Datum, Denotation, DocEdit, EditError, EntityKey, EntityKind, Entry,
    EvalOptions, Evaluation, Formula, Node, NodeError, NodeErrorKind, NodeResult, Operand,
    PartSelect,
    PatternKind, ProfileDoc, RecipeNodeId, ResolveError, RoleSeg, SlotId, SplitHalf, SplitSide,
    StableName, ValuePayload, all_edges, apply, denotation, evaluate, product,
};
use geom_core::{Affine3, Dual, Mat3, Tol, Vec3};
use topo::{Body, BooleanResult, mass_properties, transform_rigid};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    eval_after(doc, None)
}

fn eval_after(doc: &editor_core::ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
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
        profile: p.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    })
}

/// A horizontal tool plane at height `z`, normal +z.
fn plane_z(r: &mut Recorder, z: f64) -> RecipeNodeId {
    r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(z)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }))
}

fn part(r: &mut Recorder, of: RecipeNodeId, select: PartSelect<Formula>) -> RecipeNodeId {
    r.insert(Node::Part {
        of: of.into(),
        select,
    })
}

/// The read of `split`'s half `h`.
fn port(split: RecipeNodeId, h: SplitHalf) -> Operand {
    Operand::output(split, h.port())
}

fn instance(i: i64) -> PartSelect<Formula> {
    PartSelect::Instance(Formula::count(i))
}

/// A three-instance linear pattern of `input`, three metres apart.
fn pattern3(r: &mut Recorder, input: RecipeNodeId) -> RecipeNodeId {
    r.insert(Node::Pattern {
        input: input.into(),
        count: Formula::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    })
}

fn lift(r: &mut Recorder, input: impl Into<Operand>, dz: f64) -> RecipeNodeId {
    r.insert(Node::transform(
        input,
        editor_core::Step::Rigid {
            translation: [len(0.0), len(0.0), len(dz)],
            axis: [scl(0.0), scl(0.0), scl(1.0)],
            angle: ang(0.0),
        },
    ))
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
    let a = topo::test_support::finished("operand A", a.clone(), Tol::witness());
    let b = topo::test_support::finished("operand B", b.clone(), Tol::witness());
    match topo::union(&a, &b, Tol::witness()).expect("the kernel union succeeds") {
        BooleanResult::Body(bb) => bb.body.into_body(),
        BooleanResult::Empty => panic!("a union of material is not empty"),
    }
}

/// The edge keys `names` resolve to in `node`'s table, among the rows
/// of output `body` — the same keys the fillet's own resolution hands
/// the kernel.
fn edge_keys(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    body: u32,
    names: &[StableName],
) -> Vec<topo::EdgeKey> {
    let table = &ev.value(node).expect("a value").name_table;
    let mut keys: Vec<topo::EdgeKey> = names
        .iter()
        .map(|n| match table.lookup(n) {
            Some(Entry::Unique(e)) if e.body == body => match e.key {
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

/// **A1 — the half IS the half.** For each half: a transform, a union
/// and a fillet of the split's port are, description for description,
/// the kernel's own doors run on the body read off the split's value;
/// and the memo serves the split to the consumers added later.
#[test]
fn a1_the_half_is_the_half_through_a_transform_a_boolean_and_a_fillet() {
    for h in SplitHalf::ALL {
        let mut r = Recorder::new();
        let cube = unit_box(&mut r, 0.0);
        let other = unit_box(&mut r, 3.0);
        let tool = plane_z(&mut r, 0.5);
        let split = r.insert(Node::Split {
            target: cube.into(),
            tool: tool.into(),
        });
        let moved = lift(&mut r, port(split, h), LIFT);
        let first = eval(&r.doc);
        assert!(
            corpus::failures(&first).is_empty(),
            "{h:?}: {:?}",
            corpus::failures(&first)
        );
        let before = r.doc.len();

        // The two consumers added afterwards: the split is served from
        // the memo, only the new nodes compute.
        let selection = edges_of_body(&first, split, h.output_body());
        assert!(!selection.is_empty(), "the half has edges");
        let joined = r.insert(Node::Boolean {
            op: BooleanOp::Union,
            a: port(split, h),
            b: other.into(),
            declare: Vec::new(),
        });
        let rounded = r.insert(Node::fillet(port(split, h), len(RADIUS), selection.clone()));
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

        let (above, below) = sides(&ev, split);
        let side = match h {
            SplitHalf::Above => &above,
            SplitHalf::Below => &below,
        };

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
        let keys = edge_keys(&ev, split, h.output_body(), &selection);
        let filleted = sweep::blend::build::fillet_edges(
            &sweep::test_support::at_rest(side, Tol::witness()),
            &keys,
            RADIUS,
            Tol::witness(),
        )
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
        a: p1.into(),
        b: p2.into(),
        declare: Vec::new(),
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
/// spelled against the split's own above-half edge rows resolves on a
/// read of the above port, every name uniquely, and a transform of
/// that read carries exactly those rows; a below-half name read there
/// refuses through the N5 ladder, never re-anchored to a congruent
/// above-half edge; a selector for instance 2 against `Part(1)` refuses
/// `Vanished` the same way; `Part(1)`'s table is the master's row count
/// and every name carries `Instance { i: 1 }`.
#[test]
fn a3_names_pass_through_and_only_the_selected_bodys() {
    // The split's own rows, by output body.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: tool.into(),
    });
    let moved = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let base = eval(&r.doc);
    let spelled = edges_of_body(&base, split, SplitHalf::Above.output_body());
    let below_rows = edges_of_body(&base, split, SplitHalf::Below.output_body());
    assert!(!spelled.is_empty() && !below_rows.is_empty());
    let mut selection = spelled.clone();
    selection.sort();
    selection.dedup();
    let rounded = r.insert(Node::fillet(
        port(split, SplitHalf::Above),
        len(RADIUS),
        selection.clone(),
    ));
    let stray = r.insert(Node::fillet(
        port(split, SplitHalf::Above),
        len(RADIUS),
        below_rows[..1].to_vec(),
    ));
    let ev = eval(&r.doc);
    assert!(ev.value(rounded).is_some(), "{:?}", error_of(&ev, rounded));
    for name in &selection {
        assert_eq!(
            denotation(&ev, moved, name),
            Ok(Denotation::Unique),
            "{name} resolves through the port as it did on the split"
        );
    }
    assert_eq!(
        all_edges(&ev, moved).len(),
        spelled.len(),
        "the port's edge rows are exactly the above half's"
    );
    // A below-half row finds no row on the above port: absent, not
    // re-anchored to a congruent above-half edge.
    for name in &below_rows {
        assert_eq!(
            denotation(&ev, moved, name),
            Err(editor_core::InterrogateError::NoSuchName),
            "{name} is the other half's"
        );
    }
    match error_of(&ev, stray) {
        NodeErrorKind::BlendSelectionResolve { error, .. } => assert!(
            matches!(**error, ResolveError::Vanished { .. }),
            "the N5 arm the situation warrants: {error}"
        ),
        other => panic!("the fillet must refuse through the ladder, got {other:?}"),
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
    // A plane that misses the box: a read of the empty side refuses
    // `EmptyHalf`, a read of the other evaluates.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 2.0);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: tool.into(),
    });
    let above = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let below = lift(&mut r, port(split, SplitHalf::Below), 0.0);
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
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
    // The pattern (its count moved) and all three Parts — the two that
    // failed before are not in the memo and the live one reads the
    // pattern — recompute; the frame, the profile and the box are
    // served.
    assert_eq!(ev.recomputed, 4, "the pattern and its three Parts");
    assert_eq!(ev.reused, lowered.len() - 4);

    // A part reads a pattern's copies and nothing else, refused at the
    // door: a plain body and a split's half are one body each, and a
    // split named alone is two.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: tool.into(),
    });
    let pat = pattern3(&mut r, cube);
    let index_of = |of: Operand| {
        apply(
            &r.doc,
            &DocEdit::InsertNode {
                node: Box::new(Node::Part {
                    of,
                    select: instance(0),
                }),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .err()
    };
    let of_slot = editor_core::SlotId::Operand(editor_core::OperandSlot::Of);
    for (of, label) in [
        (cube.into(), "a plain body"),
        (port(split, SplitHalf::Above), "a split's half"),
    ] {
        let refused = index_of(of);
        assert!(
            matches!(
                &refused,
                Some(EditError::SlotVarKind { slot, found: editor_core::VarKind::Body, .. })
                    if *slot == of_slot
            ),
            "an index of {label}: {refused:?}"
        );
    }
    let refused = index_of(split.into());
    assert!(
        matches!(
            &refused,
            Some(EditError::AmbiguousOutput { input, slot, .. })
                if input.id() == split && *slot == of_slot
        ),
        "{refused:?}"
    );
    let index_of_body = part(&mut r, pat, instance(0));

    // The index is structural: `SetParam` refuses it.
    let refused = apply(
        &r.doc,
        &DocEdit::SetParam {
            node: index_of_body,
            slot: SlotId::Instance,
            value: Formula::count(1).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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

/// **A5 — the key separates what the memo must separate.** Reads of
/// the two halves key apart, the two instances key apart, and an edit
/// of the index recomputes the Part and nothing upstream. (The tag census is
/// `eval::tag_vocabulary_tests::node_kind_vocabulary_is_injective`.)
#[test]
fn a5_the_content_key_separates_the_halves_and_the_instances() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let tool = plane_z(&mut r, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: tool.into(),
    });
    let above = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let below = lift(&mut r, port(split, SplitHalf::Below), 0.0);
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
    assert_ne!(key(above), key(below), "reads of the two halves of one split");
    assert_ne!(key(p1), key(p2), "two instances of one pattern");

    let edited = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: p1,
            slot: SlotId::Instance,
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool: tool.into(),
    });
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
        .ids()
        .iter()
        .find(|id| matches!(cd.doc.node(**id), Some(Node::Split { .. })))
        .expect("the split");
    let (above, below) = sides(&ev, split);
    let section = |b: &Body<f64>| {
        let minted: Vec<_> = b
            .surfaces()
            .filter_map(|(k, s)| {
                b.surface_source(k)
                    .filter(|src| src.node == split.0.digest())
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

/// A prism from a polygon at height `z0`, extruded `dz`.
fn prism(r: &mut Recorder, pts: Vec<(f64, f64)>, z0: f64, dz: f64) -> RecipeNodeId {
    let p = r.profile([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(dz),
        side: ExtrudeSide::Along,
    })
}

/// `lib_g14_split_walls::u_cutter_tie`, the reviewers' fixture: a
/// 4×4×4 block minus a U-shaped cutter whose two prongs cross one wall
/// — a body whose table carries genuine N2 ties, symmetric about
/// `y = 2`.
fn u_cutter_tie(r: &mut Recorder) -> RecipeNodeId {
    let a = prism(
        r,
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        0.0,
        4.0,
    );
    let b = prism(
        r,
        vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ],
        1.0,
        2.0,
    );
    r.insert(Node::Boolean {
        op: BooleanOp::Subtract,
        a: a.into(),
        b: b.into(),
        declare: Vec::new(),
    })
}

/// The tie rows of a table: name and candidates.
fn ties(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
) -> Vec<(StableName, Vec<editor_core::EntityRef>)> {
    ev.value(node)
        .expect("a value")
        .name_table
        .iter()
        .filter_map(|(n, e)| match e {
            Entry::Tied(c) => Some((n.clone(), c.clone())),
            Entry::Unique(_) => None,
        })
        .collect()
}

/// **A tie the split separates, and what a read of each half makes of
/// it.** A split at `y = 2` puts the U-cutter tie's two candidates in
/// different halves WITHOUT cutting either: each passes through intact
/// under its upstream name, so the split's table holds one `Tied` row
/// straddling body 0 and body 1. A read of each half sees the name
/// `Unique` — the projection separated the candidates as an op would,
/// by the one narrowing rule the emitter's flush uses — so a selector
/// spelled against the tied name resolves through each port to that
/// half's own entity. The split's table stays tied.
#[test]
fn a_tie_the_split_separates_is_unique_through_each_port() {
    let mut r = Recorder::new();
    let sub = u_cutter_tie(&mut r);
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(2.0), len(0.0)],
        normal: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let split = r.insert(Node::Split {
        target: sub.into(),
        tool: tool.into(),
    });
    let above = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let below = lift(&mut r, port(split, SplitHalf::Below), 0.0);
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let straddling: Vec<_> = ties(&ev, split)
        .into_iter()
        .filter(|(_, c)| {
            let bodies: std::collections::BTreeSet<u32> = c.iter().map(|e| e.body).collect();
            bodies.len() > 1
        })
        .collect();
    assert!(
        !straddling.is_empty(),
        "the premise: the split's table holds a tie straddling its two halves"
    );
    for (name, candidates) in &straddling {
        for (p, h) in [(above, SplitHalf::Above), (below, SplitHalf::Below)] {
            let mine: Vec<_> = candidates
                .iter()
                .filter(|e| e.body == h.output_body())
                .collect();
            assert_eq!(mine.len(), 1, "{name}: one candidate per half");
            assert_eq!(
                denotation(&ev, p, name),
                Ok(Denotation::Unique),
                "{name} is unique through the {h:?} port"
            );
        }
        // The split's own table is untouched: still tied there.
        assert!(
            ev.value(split).expect("the split").name_table.is_tied(name),
            "{name} stays tied on the split"
        );
    }
}

/// **The contrast: a pattern of a tied master.** The pattern wraps the
/// master's tie per instance, every candidate in that instance's body,
/// so no row straddles and the Part of an instance is simply that
/// instance's tie, still tied.
#[test]
fn a_part_of_an_instance_of_a_tied_master_keeps_the_tie() {
    let mut r = Recorder::new();
    let sub = u_cutter_tie(&mut r);
    let pat = r.insert(Node::Pattern {
        input: sub.into(),
        count: Formula::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(20.0),
        },
    });
    let p1 = part(&mut r, pat, instance(1));
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let master_ties = ties(&ev, sub);
    assert!(!master_ties.is_empty(), "the fixture carries a tie");
    for (_, c) in ties(&ev, pat) {
        let bodies: std::collections::BTreeSet<u32> = c.iter().map(|e| e.body).collect();
        assert_eq!(
            bodies.len(),
            1,
            "a pattern's tie lives in one instance: {c:?}"
        );
    }
    let projected = ties(&ev, p1);
    assert_eq!(
        projected.len(),
        master_ties.len(),
        "one tie per master tie, still tied"
    );
    for (name, c) in &projected {
        assert!(
            matches!(name.path.first(), Some(RoleSeg::Instance { i: 1, .. })),
            "{name}"
        );
        assert!(c.iter().all(|e| e.body == 0), "re-keyed to body 0: {c:?}");
    }
}

/// **`project`'s three tie shapes, as a unit row.** A tie inside the
/// selected body survives as a tie; one entirely outside is dropped;
/// one that straddles narrows to the selected body's survivor —
/// `Unique` when one, as the flush rule says.
#[test]
fn project_narrows_a_tie_by_the_flush_rule() {
    use editor_core::{EntityRef, NameTable};
    let mut r = Recorder::new();
    let cube = unit_box(&mut r, 0.0);
    let ev = eval(&r.doc);
    let keys: Vec<topo::FaceKey> = body_of(&ev, cube).faces().map(|(k, _)| k).take(4).collect();
    assert_eq!(keys.len(), 4);
    let face = |k: usize| EntityKey::Face(keys[k]);
    let ent = |body: u32, key: EntityKey| EntityRef { body, key };
    let name = |h: SplitHalf| StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 7),
        path: vec![RoleSeg::SplitBody(h)],
    };
    let (inside, outside) = (name(SplitHalf::Above), name(SplitHalf::Below));

    let mut t = NameTable::new();
    t.insert_tied(inside.clone(), vec![ent(0, face(0)), ent(0, face(1))])
        .expect("a tie inside body 0");
    t.insert_tied(outside.clone(), vec![ent(1, face(2)), ent(1, face(3))])
        .expect("a tie inside body 1");
    let p = t.project(0).expect("projects");
    assert!(matches!(p.lookup(&inside), Some(Entry::Tied(c)) if c.len() == 2));
    assert!(p.lookup(&outside).is_none());

    let mut t = NameTable::new();
    t.insert_tied(inside.clone(), vec![ent(0, face(0)), ent(1, face(1))])
        .expect("a straddling tie is a legal table");
    let p0 = t.project(0).expect("projects");
    assert_eq!(p0.lookup(&inside), Some(&Entry::Unique(ent(0, face(0)))));
    let p1 = t.project(1).expect("projects");
    assert_eq!(p1.lookup(&inside), Some(&Entry::Unique(ent(0, face(1)))));
    assert!(t.project(2).expect("projects").is_empty());
}
