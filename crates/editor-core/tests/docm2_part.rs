//! **DOCM-2 — one body of a multi-body value at f64**
//! (`crates/editor-core/REFERENCES.md` DM3): a read of a split's port,
//! and an indexed read `row[i]` of one member of a family (D10
//! `[repetition]`). Acceptance rows A1–A6, the
//! split-stamping row the stop clause's amendment asks for, and the
//! `Dual64` pin of the exact corpus document. The Interval-lane rows
//! (A7) are `docm2_part_interval`.
//!
//! The oracle for "the half IS the half" is the kernel's own door fed
//! the body read straight off the split's or the pattern's value: a
//! consumer of a port or of a member must produce the same body,
//! description for description, that the door produces from that body
//! — and a member's value must share its source's `Arc`, not a copy of
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::fixture::{Recorder, len};

use editor_core::{
    CancelToken, Denotation, DocEdit, EditError, EntityKey, EntityKind, Entry, EvalOptions,
    Evaluation, Formula, Node, NodeError, NodeErrorKind, NodeResult, Operand, ProfileDoc,
    RecipeNodeId, ResolveError, RoleSeg, SlotId, SplitHalf, SplitSide, StableName, ValuePayload,
    VarId, all_edges, apply, denotation, evaluate, product,
};
use geom_core::{Dual, Tol};
use topo::{Body, BooleanResult, transform_rigid};

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

/// The unit box `[0, 1]³`.
fn unit_box(r: &mut Recorder) -> RecipeNodeId {
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    })
}

/// A unit box placed against `root`, shifted along its x by `x0`: the
/// placement.
fn unit_box_at(r: &mut Recorder, root: RecipeNodeId, x0: f64) -> RecipeNodeId {
    let b = unit_box(r);
    r.against(root, b, &editor_core::Frame::translation([x0, 0.0, 0.0]))
}

/// A horizontal tool plane at height `z`, normal +z, read off `body`.
fn plane_z(r: &Recorder, body: RecipeNodeId, z: f64) -> editor_core::pose::RoleRead<Operand> {
    crate::fixture::plane_on(&r.doc, body, [0.0, 0.0, z], [0.0, 0.0, 1.0])
}

/// The read of `split`'s half `h`.
fn port(split: RecipeNodeId, h: SplitHalf) -> Operand {
    Operand::output(split, h.port())
}

/// A three-member row of `input`, three metres apart: its index and
/// the placement.
fn pattern3(r: &mut Recorder, input: RecipeNodeId) -> (VarId, RecipeNodeId) {
    r.linear(input, [1.0, 0.0, 0.0], 3.0, 3)
}

/// Whether `seg` is member `i` of the index `k`.
fn is_member(seg: Option<&RoleSeg>, k: VarId, i: u32) -> bool {
    matches!(seg, Some(RoleSeg::Member { at, .. }) if at.as_slice() == [(k, i)])
}

/// A copy of `input` in the world, `dz` up: the placement, a reader of
/// `input`.
fn lift(r: &mut Recorder, input: impl Into<Operand>, dz: f64) -> RecipeNodeId {
    r.in_world(input, &editor_core::Frame::translation([0.0, 0.0, dz]))
}

/// The `Body` value of a node — a member's, a placement's one copy — as
/// the Arc it holds.
fn body_arc(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Arc<Body<f64>> {
    match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b.shared(),
        other => panic!("node {} is a {}, not a body", id.0, other.kind_name()),
    }
}

/// A single body read off any single-body value — a `Body` or a
/// boolean's non-empty result.
fn body_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> Body<f64> {
    corpus::body_of(ev, id)
}

/// Both halves read straight off the split's value.
fn sides(ev: &Evaluation<f64>, split: RecipeNodeId) -> (Arc<Body<f64>>, Arc<Body<f64>>) {
    let ValuePayload::Split { above, below } = &ev.value(split).expect("the split").payload else {
        panic!("a split value");
    };
    let body = |s: &SplitSide<f64>| match s {
        SplitSide::Body(b) => Arc::clone(b.shared()),
        SplitSide::Empty => panic!("both halves carry material"),
    };
    (body(above), body(below))
}

/// The members read straight off the family's value.
fn instances(ev: &Evaluation<f64>, pattern: RecipeNodeId) -> Vec<Arc<Body<f64>>> {
    let ValuePayload::Instances(v) = &ev.value(pattern).expect("the pattern").payload else {
        panic!("a family of bodies");
    };
    v.iter().map(|b| Arc::clone(b.shared())).collect()
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
    // The read holding output `body`, and its index there (a split's
    // half is body 0 of its own table, NAMES N1).
    let (var, body) = ev.body_read_of(node, body).expect("a value");
    let table = ev.table(var).expect("a value");
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

/// **A1 — the half IS the half.** For each half: a placement, a union
/// and a fillet of the split's port are, description for description,
/// the kernel's own doors run on the body read off the split's value;
/// and the memo serves the split to the consumers added later.
#[test]
fn a1_the_half_is_the_half_through_a_placement_a_boolean_and_a_fillet() {
    for h in SplitHalf::ALL {
        let mut r = Recorder::new();
        let cube = unit_box(&mut r);
        let other = unit_box_at(&mut r, cube, 3.0);
        let tool = plane_z(&r, cube, 0.5);
        let split = r.insert(Node::Split {
            target: cube.into(),
            tool,
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
        let joined = r.insert(Node::Union {
            members: editor_core::Bodies::Spelled(vec![port(split, h).into(), other.into()]),
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

        // Each consumer against the kernel door fed the side directly:
        // the placement at the pose its mate solves to, which is the
        // lift up z.
        let pose = crate::fixture::solve(&r.doc, &EvalOptions::default(), Tol::witness())
            .placement(&r.doc, moved)
            .expect("the lift solves");
        let lifted = editor_core::Frame::translation([0.0, 0.0, LIFT]);
        let near = pose
            .columns
            .iter()
            .flatten()
            .chain(&pose.translation)
            .zip(lifted.columns.iter().flatten().chain(&lifted.translation))
            .all(|(a, b)| (a - b).abs() <= 1e-12);
        assert!(near, "{h:?}: the lift is {LIFT} up z: {pose:?}");
        // The copy is the half itself: its body is the half's `Arc`,
        // under the lift.
        assert!(
            Arc::ptr_eq(body_arc(&ev, moved), side),
            "{h:?}: placement shares the half"
        );
        let fused = kernel_union(side, &(body_of(&ev, other)));
        assert_eq!(
            bits(&(body_of(&ev, joined))),
            bits(&fused),
            "{h:?}: boolean"
        );
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

/// **A2 — the member IS the member.** `row[1] ∪ row[2]` is the
/// kernel's pair union of `v[1]` and `v[2]` read off the value, and
/// every member holds the input body's own `Arc` (a copy shares its
/// source's body): member 0 is the input itself, through the family
/// and through the read.
#[test]
fn a2_the_member_is_the_member() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r);
    let (_, pat) = pattern3(&mut r, cube);
    let p0 = r.member(pat, 0);
    let p1 = r.member(pat, 1);
    let p2 = r.member(pat, 2);
    let joined = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![p1.into(), p2.into()]),
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
            "row[{i}] holds member {i}'s Arc"
        );
        assert!(
            Arc::ptr_eq(&v[i], body_arc(&ev, cube)),
            "member {i} shares the input body"
        );
    }
    // The union computes in its first member's frame and reads the
    // second into it through `relative`.
    let framed = |id: RecipeNodeId| match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b.clone(),
        other => panic!("node {} is a {}, not a body", id.0, other.kind_name()),
    };
    let (first, second) = (framed(p1), framed(p2));
    let map = first
        .relative(&second)
        .expect("a finite relative map")
        .expect("two members apart");
    let read = transform_rigid(second.body(), &map, Tol::witness()).expect("a rigid read");
    let fused = kernel_union(first.body(), &read);
    let joined = match &ev.value(joined).expect("the union").payload {
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => body.clone(),
        other => panic!("the union is a {}", other.kind_name()),
    };
    assert_eq!(bits(joined.body()), bits(&fused));
}

/// The edge names of ONE output body of a node's table.
fn edges_of_body(ev: &Evaluation<f64>, node: RecipeNodeId, body: u32) -> Vec<StableName> {
    // The read holding the body, and its index in that read's table (a
    // split's half is body 0 of its own, NAMES N1).
    let (var, body) = ev.body_read_of(node, body).expect("a value");
    ev.table(var)
        .expect("a value")
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
/// read of the above port, every name uniquely, and a placement of
/// that read carries exactly those rows, verbatim; a below-half name read there
/// refuses through the N5 ladder, never re-anchored to a congruent
/// above-half edge; a selector for member 2 against `row[1]` refuses
/// `Vanished` the same way; `row[1]`'s table is the master's row count
/// and every name carries `Member { (k, 1) }`.
#[test]
fn a3_names_pass_through_and_only_the_selected_bodys() {
    // The split's own rows, by output body.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r);
    let tool = plane_z(&r, cube, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
    let moved = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let base = eval(&r.doc);
    let spelled = edges_of_body(&base, split, SplitHalf::Above.output_body());
    // The below half's own rows: a name both halves hold (a section
    // edge, scoped by the read of each half, NAMES N1) is each half's.
    let below_rows: Vec<StableName> = edges_of_body(&base, split, SplitHalf::Below.output_body())
        .into_iter()
        .filter(|n| !spelled.contains(n))
        .collect();
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
            denotation(&ev, moved, &name.clone()),
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
            denotation(&ev, moved, &name.clone()),
            Err(editor_core::InterrogateError::NoSuchName),
            "{name} is the other half's"
        );
    }
    match error_of(&ev, stray) {
        NodeErrorKind::SelectResolve { error, .. } => assert!(
            matches!(**error, ResolveError::Vanished { .. }),
            "the N5 arm the situation warrants: {error}"
        ),
        other => panic!("the fillet must refuse through the ladder, got {other:?}"),
    }

    // The family side.
    let mut r = Recorder::new();
    let cube = unit_box(&mut r);
    let (k, pat) = pattern3(&mut r, cube);
    let p1 = r.member(pat, 1);
    let base = eval(&r.doc);
    let master = base.table_at(cube).expect("the master").len();
    let table = base.table_at(p1).expect("the member");
    assert_eq!(table.len(), master, "exactly the master's row count");
    for (name, _) in table.iter().filter(|(name, _)| name.node != p1) {
        assert!(
            is_member(name.path.first(), k, 1),
            "{name} carries Member {{ (k, 1) }}: {:?}",
            name.path
        );
    }
    // Member 2's name spelled against row[1]: the ladder's third rung
    // — the node is live, the table lacks it.
    let edge = all_edges(&base, p1)
        .into_iter()
        .next()
        .expect("an edge of member 1");
    let Some(RoleSeg::Member { of, .. }) = edge.path.first() else {
        panic!("a member name");
    };
    let other_instance = StableName {
        kind: EntityKind::Edge,
        node: pat,
        path: vec![RoleSeg::Member {
            at: vec![(k, 2)],
            of: of.clone(),
        }],
    };
    let rounded = r.insert(Node::fillet(p1, len(RADIUS), vec![other_instance]));
    let ev = eval(&r.doc);
    match error_of(&ev, rounded) {
        NodeErrorKind::SelectResolve { error, .. } => assert!(
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
    let cube = unit_box(&mut r);
    let tool = plane_z(&r, cube, 2.0);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
    // The empty side offers no geometry to place by: its reader is a
    // placement with no mate yet.
    let above = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(port(
        split,
        SplitHalf::Above,
    ))));
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
    let cube = unit_box(&mut r);
    let (k, pat) = pattern3(&mut r, cube);
    let at_count = r.member(pat, 3);
    let negative = r.member(pat, -1);
    let live = r.member(pat, 2);
    let ev = eval(&r.doc);
    assert!(
        matches!(
            error_of(&ev, at_count),
            NodeErrorKind::MemberOutOfRange { input, at } if *input == pat && at.as_slice() == [3]
        ),
        "{:?}",
        error_of(&ev, at_count)
    );
    assert!(
        matches!(
            error_of(&ev, negative),
            NodeErrorKind::MemberOutOfRange { at, .. } if at.as_slice() == [-1]
        ),
        "{:?}",
        error_of(&ev, negative)
    );
    assert!(ev.value(live).is_some());
    let lowered = apply(
        &r.doc,
        &DocEdit::DefineVar {
            var: editor_core::VarRef::Id(k),
            def: editor_core::VarDecl::Index {
                count: Formula::count(2),
                within: None,
            },
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("an index's count edit")
    .doc;
    let ev = eval_after(&lowered, Some(&ev));
    assert!(ev.value(pat).is_some(), "the family is Ok at two");
    assert!(
        matches!(
            error_of(&ev, live),
            NodeErrorKind::MemberOutOfRange { at, .. } if at.as_slice() == [2]
        ),
        "{:?}",
        error_of(&ev, live)
    );
    // The three reads recompute — the two that failed before are not
    // in the memo, and the live one reads the family. The family's
    // members 0 and 1 keep their keys (D9: a member keys by its node's
    // key and its index values), so the family is served, as are the
    // frame, the profile and the box.
    assert_eq!(ev.recomputed, 3, "the three reads");
    assert_eq!(ev.reused, lowered.len() - 3);

    // The index of a read is structural: `SetParam` refuses it.
    let slot = SlotId::Index {
        seat: editor_core::OperandSlot::Source,
        k: 0,
    };
    let refused = apply(
        &r.doc,
        &DocEdit::SetParam {
            node: live,
            slot,
            value: Formula::count(1).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::StructuralSlotNeedsStructuralEdit { slot: s }) if s == slot
        ),
        "{refused:?}"
    );
}

/// **A5 — the key separates what the memo must separate.** Reads of
/// the two halves key apart, the two members key apart, and an edit
/// of the index recomputes the read and nothing upstream. (The tag census is
/// `eval::tag_vocabulary_tests::node_kind_vocabulary_is_injective`.)
#[test]
fn a5_the_content_key_separates_the_halves_and_the_instances() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r);
    let tool = plane_z(&r, cube, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
    let above = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let below = lift(&mut r, port(split, SplitHalf::Below), 0.0);
    let (_, pat) = pattern3(&mut r, cube);
    let p1 = r.member(pat, 1);
    let p2 = r.member(pat, 2);
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let key = |id| ev.value(id).expect("a value").content_key;
    assert_ne!(
        key(above),
        key(below),
        "reads of the two halves of one split"
    );
    assert_ne!(key(p1), key(p2), "two members of one family");

    let edited = apply(
        &r.doc,
        &DocEdit::SetStructuralParam {
            node: p1,
            slot: SlotId::Index {
                seat: editor_core::OperandSlot::Source,
                k: 0,
            },
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("a structural edit")
    .doc;
    let again = eval_after(&edited, Some(&ev));
    assert_eq!(again.recomputed, 1, "the read alone");
    assert_eq!(again.reused, r.doc.len() - 1, "nothing upstream moves");
    let framed = |ev: &Evaluation<f64>, id| match &ev.value(id).expect("a value").payload {
        ValuePayload::Body(b) => b.clone(),
        other => panic!("a body, got {}", other.kind_name()),
    };
    assert!(
        matches!(framed(&again, p1).relative(&framed(&again, p2)), Ok(None)),
        "a read at 2 stands where the other read at 2 does"
    );
}

/// **The product of a document whose one placement reads the split's
/// `above` port is that one half** — the other half is in no product
/// (A7's product row; `sources_of`'s doc says so).
#[test]
fn a7_the_product_of_a_lone_placed_half_is_that_half() {
    let mut r = Recorder::new();
    let cube = unit_box(&mut r);
    let tool = plane_z(&r, cube, 0.5);
    let split = r.insert(Node::Split {
        target: cube.into(),
        tool,
    });
    r.place(port(split, SplitHalf::Above));
    let ev = eval(&r.doc);
    let body = topo::test_support::finished(
        "body",
        product(&r.doc, &ev, Tol::witness()).expect("the product gathers"),
        Tol::witness(),
    );
    let m = topo::mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass properties");
    assert_eq!(
        m.enclosure.volume, 0.5,
        "the upper half of the unit box, exactly"
    );
    let (side, _) = sides(&ev, split);
    assert_eq!(bits(&body), bits(&side));
}

/// **The two section planes of one split face away from each other**,
/// and a boolean of the two halves succeeds at f64: their descriptions
/// are opposed bit for bit, so a reader taking them for one plane would
/// rejoin the halves along a face that is not there.
#[test]
fn the_two_section_planes_of_one_split_face_away_from_each_other() {
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
    // Each half's section face, by its name among that half's rows.
    let section = |side: SplitHalf, body: &Body<f64>| {
        let name = corpus::part_select::section_face(split);
        let table = crate::fixture::half(&ev, split, side);
        let face = match table.lookup(&name) {
            Some(Entry::Unique(e)) if e.body == 0 => match e.key {
                EntityKey::Face(k) => k,
                other => panic!("{name} is not a face: {other:?}"),
            },
            other => panic!("{name} does not resolve in its half: {other:?}"),
        };
        let surface = body.get_face(face).expect("a live face").surface;
        body.get_surface(surface).expect("a live carrier").clone()
    };
    let (topo::Surface::Plane { normal: na, .. }, topo::Surface::Plane { normal: nb, .. }) = (
        section(SplitHalf::Above, &above),
        section(SplitHalf::Below, &below),
    ) else {
        panic!("planes")
    };
    assert_eq!(
        format!("{na:?}"),
        format!("{:?}", -nb),
        "the two section planes face away from each other"
    );
}

/// **The `Dual64` pin** (the amendment, item 2): the exact corpus
/// document — whose union rejoins the two halves of one split —
/// evaluates green at a scalar with no bit channel. The value
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

/// A prism from a polygon on its own xy, extruded `dz`.
fn prism(r: &mut Recorder, pts: Vec<(f64, f64)>, dz: f64) -> RecipeNodeId {
    let p = r.profile([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![pts]);
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(dz),
    })
}

/// `lib_g14_split_walls::u_cutter_tie`, the reviewers' fixture: a
/// 4×4×4 block minus a U-shaped cutter whose two prongs cross one wall
/// — a body whose table carries genuine N2 ties, symmetric about
/// `y = 2`. Returns the subtract and the block, its root.
fn u_cutter_tie(r: &mut Recorder) -> (RecipeNodeId, RecipeNodeId) {
    let a = prism(r, vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 4.0);
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
        2.0,
    );
    let b = r.against(a, b, &editor_core::Frame::translation([0.0, 0.0, 1.0]));
    let sub = r.insert(Node::Subtract {
        from: a.into(),
        tool: b.into(),
    });
    (sub, a)
}

/// The tie rows of a table: name and candidates.
fn ties(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
) -> Vec<(StableName, Vec<editor_core::EntityRef>)> {
    ev.table_at(node)
        .expect("a value")
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
/// under its upstream name. Each half is its own table (NAMES N1), so
/// each holds the name `Unique` — narrowed as an op narrows, by the one
/// rule the emitter's flush uses — and a selector spelled against the
/// tied name resolves through each port to that half's own entity. The
/// subtract's table stays tied.
#[test]
fn a_tie_the_split_separates_is_unique_through_each_port() {
    let mut r = Recorder::new();
    let (sub, root) = u_cutter_tie(&mut r);
    let tool = crate::fixture::plane_on(&r.doc, root, [0.0, 2.0, 0.0], [0.0, 1.0, 0.0]);
    let split = r.insert(Node::Split {
        target: sub.into(),
        tool,
    });
    let above = lift(&mut r, port(split, SplitHalf::Above), 0.0);
    let below = lift(&mut r, port(split, SplitHalf::Below), 0.0);
    let ev = eval(&r.doc);
    assert!(
        corpus::failures(&ev).is_empty(),
        "{:?}",
        corpus::failures(&ev)
    );
    let separated: Vec<_> = ties(&ev, sub)
        .into_iter()
        .filter(|(name, _)| {
            [SplitHalf::Above, SplitHalf::Below].into_iter().all(|h| {
                matches!(
                    crate::fixture::half(&ev, split, h).lookup(name),
                    Some(Entry::Unique(_))
                )
            })
        })
        .collect();
    assert!(
        !separated.is_empty(),
        "the premise: a tie of the subtract the split puts one candidate in each half"
    );
    for (name, candidates) in &separated {
        assert_eq!(candidates.len(), 2, "{name}: one candidate per half");
        for (p, h) in [(above, SplitHalf::Above), (below, SplitHalf::Below)] {
            assert_eq!(
                denotation(&ev, p, &name.clone()),
                Ok(Denotation::Unique),
                "{name} is unique through the {h:?} port"
            );
        }
        // The subtract's own table is untouched: still tied there.
        assert!(
            ev.table_at(sub).expect("the subtract").is_tied(name),
            "{name} stays tied on the subtract"
        );
    }
}

/// **The contrast: a family of a tied master.** The family wraps the
/// master's tie per member, every candidate in that member's body, so
/// no row straddles and the read of a member is simply that member's
/// tie, still tied.
#[test]
fn a_member_of_a_family_of_a_tied_master_keeps_the_tie() {
    let mut r = Recorder::new();
    let (sub, _) = u_cutter_tie(&mut r);
    let (k, pat) = r.linear(sub, [1.0, 0.0, 0.0], 20.0, 3);
    let p1 = r.member(pat, 1);
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
        assert_eq!(bodies.len(), 1, "a family's tie lives in one member: {c:?}");
    }
    let projected = ties(&ev, p1);
    assert_eq!(
        projected.len(),
        master_ties.len(),
        "one tie per master tie, still tied"
    );
    for (name, c) in &projected {
        assert!(is_member(name.path.first(), k, 1), "{name}");
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
    let cube = unit_box(&mut r);
    let ev = eval(&r.doc);
    let keys: Vec<topo::FaceKey> = body_of(&ev, cube).faces().map(|(k, _)| k).take(4).collect();
    assert_eq!(keys.len(), 4);
    let face = |k: usize| EntityKey::Face(keys[k]);
    let ent = |body: u32, key: EntityKey| EntityRef { body, key };
    let name = |path: Vec<RoleSeg>| StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 7),
        path,
    };
    let (inside, outside) = (
        name(vec![RoleSeg::SectionFace]),
        name(vec![RoleSeg::Cap(editor_core::CapEnd::End)]),
    );

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
