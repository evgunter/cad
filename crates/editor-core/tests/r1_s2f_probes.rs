//! R1's probes for INTENT stage 2 F (review only).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    CapEnd, DocumentId, ExtrudeSide, MeasurePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    SitedRef,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{fname, insert, len, on_frame, run, xform};
use geom_core::Tol;
use std::collections::BTreeSet;

fn cube(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [x, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

fn closure(doc: &ProfileDoc, roots: &[RecipeNodeId]) -> BTreeSet<RecipeNodeId> {
    let mut out = BTreeSet::new();
    let mut stack: Vec<RecipeNodeId> = roots.to_vec();
    while let Some(n) = stack.pop() {
        if out.insert(n) {
            stack.extend(doc.upstream(n));
        }
    }
    out
}

/// A remainder measure reads a face of `t = Transform(x, +5 z)`; only
/// `x` itself is placed. Cut `x`, `t` and `x`'s placement: does the
/// split keep the measured value?
#[test]
fn r1_p1_split_carries_a_selection_read_through_an_unplaced_transform() {
    let doc = ProfileDoc::empty(DocumentId::derive("r1-p1"), Tol::witness());
    let (doc, x) = cube(doc, 0.0);
    let (doc, y) = cube(doc, 3.0);
    let (doc, t) = insert(doc, xform(x, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, px) = fixture::place(doc, x);
    let (doc, _py) = fixture::place(doc, y);
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(t, fname(x, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(y, fname(y, RoleSeg::Cap(CapEnd::End))),
        ],
    );
    let ev = run(&doc, &editor_core::EvalOptions::default());
    let before = fixture::reading(&doc, &ev, fixture::output(&doc, m));
    let mut cut = closure(&doc, &[t]);
    cut.insert(px);
    let out = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-p1-part"),
        Tol::witness(),
        None,
    );
    let out = match out {
        Err(e) => {
            eprintln!("R1-P1: split refused: {e:?}");
            return;
        }
        Ok(out) => out,
    };
    let mut store = PartStore::new();
    store.insert(out.part.clone(), Tol::witness());
    let rev = run(&out.remainder, &with_resolver(store));
    let after = fixture::reading(&out.remainder, &rev, fixture::output(&out.remainder, m));
    eprintln!("R1-P1: before {before:?}, after {after:?}");
    assert_eq!(before, after, "the split moved the measured value");
}

/// P1's document: (doc, x, y, t, px, measure).
fn p1_doc(
    label: &str,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, x) = cube(doc, 0.0);
    let (doc, y) = cube(doc, 3.0);
    let (doc, t) = insert(doc, xform(x, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, px) = fixture::place(doc, x);
    let (doc, _py) = fixture::place(doc, y);
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(t, fname(x, RoleSeg::Cap(CapEnd::End))),
            SitedRef::new(y, fname(y, RoleSeg::Cap(CapEnd::End))),
        ],
    );
    (doc, t, px, m, x)
}

/// P1b: where the split accepts P1's cut, `inline(split(d))` must be
/// `d` up to ids.
#[test]
fn r1_p1b_round_trip_of_a_selection_read_through_an_unplaced_transform() {
    let (doc, t, px, _m, _x) = p1_doc("r1-p1b");
    let mut cut = closure(&doc, &[t]);
    cut.insert(px);
    let Ok(out) = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-p1b-part"),
        Tol::witness(),
        None,
    ) else {
        eprintln!("R1-P1b: split refused");
        return;
    };
    let mut store = PartStore::new();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &crate::p2_gauges::resolver(store.clone()),
        Tol::witness(),
    );
    let back = match back {
        Ok(b) => b,
        Err(e) => panic!("R1-P1b: split accepted, inline refuses: {e}"),
    };
    let (map, steps) = fixture::round_trip::composed(&doc, &out, &back);
    if let Err(e) = fixture::round_trip::same_up_to_ids(&doc, &back.doc, &map, &steps) {
        panic!("R1-P1b: inline(split(d)) != d:\n{e}");
    }
}

/// P2: a mate between two unplaced boolean operands, whose side no
/// longer resolves. Spec §7: a mate whose members are not both placed
/// mints nothing. Does the gate raise it anyway?
#[test]
fn r1_p2_an_unevaluated_mate_on_unplaced_operands_at_the_gate() {
    use editor_core::{
        Alignment, AxisSense, ContactClass, DocEdit, MateFrame, MatePrimitive, OperandSlot,
    };
    use fixture::resolver::in_part;
    let mut store = PartStore::default();
    let (part, pbody) = {
        let d = ProfileDoc::empty(DocumentId::derive("r1-p2-part"), Tol::witness());
        cube(d, 0.0)
    };
    let (doc_ref, body) = store.insert_part((part, pbody), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-p2"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref.clone()));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let fr = |o: [f64; 3]| {
        MateFrame::authored(o, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness()).unwrap()
    };
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: fixture::head_at(a, in_part(a, body, CapEnd::End)).into(),
            b: fixture::head_at(b, in_part(b, body, CapEnd::Start)).into(),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: fr([0.0, 0.0, 1.0]),
                b: fr([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    let (doc, fused) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    );
    let doc = fixture::place_all(doc, &[fused]);
    let ev = run(&doc, &with_resolver(store.clone()));
    eprintln!(
        "R1-P2 healthy: {:?}",
        editor_core::assemble(&doc, &ev, Tol::witness()).map(|_| ())
    );
    // Re-point side b at the other end cap of b (still resolves), then at
    // a ghost.
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Face,
        node: b,
        path: vec![RoleSeg::InPart {
            of: editor_core::StableName {
                kind: editor_core::EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(77)),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetParam {
            node: mate,
            slot: editor_core::SlotId::Operand(OperandSlot::Side(editor_core::MateSide::B)),
            value: editor_core::SlotValue::Read(fixture::head_at(b, ghost).into()),
            fresh: Vec::new(),
        },
    );
    let ev = run(&doc, &with_resolver(store));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).map(|_| ());
    let gate = editor_core::assemble(&doc, &ev, Tol::witness()).map(|_| ());
    eprintln!("R1-P2 broken: product {product:?}; gate {gate:?}");
    assert!(
        gate.is_ok(),
        "an unplaced-operand mate gates the product: {gate:?}"
    );
}

/// P4: a mate side reads a face of a pattern copy in the pattern's
/// `Bodies` (admitted only at a side). Share that selection with a
/// measure: does the edit door admit it, and does the result save?
#[test]
fn r1_p4_a_side_selection_on_a_pattern_shared_with_a_measure() {
    use editor_core::{
        Alignment, AxisSense, ContactClass, DocEdit, MateFrame, MatePrimitive, PatternKind,
    };
    use fixture::resolver::in_part;
    use fixture::{in_copy, scl, step};
    let block = |label: &str| {
        let d = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
        cube(d, 0.0)
    };
    let mut store = PartStore::new();
    let (leg_ref, leg_body) = store.insert_part(block("r1-p4-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(block("r1-p4-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("r1-p4"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: editor_core::Formula::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let fr = |o: [f64; 3]| {
        MateFrame::authored(o, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness()).unwrap()
    };
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Mate {
                a: fixture::head(in_copy(pattern, 2, in_part(leg, leg_body, CapEnd::End))).into(),
                b: fixture::head(in_part(top, top_body, CapEnd::Start)).into(),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: fr([0.0, 0.0, 1.0]),
                    b: fr([0.0, 0.0, 0.0]),
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            }),
            fresh: Vec::new(),
        },
    );
    let mate = mate.unwrap();
    let side_a = match doc.node(mate) {
        Some(Node::Mate { a, .. }) => *a,
        _ => unreachable!(),
    };
    eprintln!(
        "R1-P4: side a's body kind: {:?}",
        doc.selection(side_a)
            .and_then(|s| doc.var(s.body))
            .map(|v| v.kind())
    );
    let (doc, _) = step(
        doc,
        DocEdit::RenameVar {
            var: side_a.into(),
            name: Some(editor_core::VarName::from_static("leg_top")),
        },
    );
    let saved = editor_core::persist::save(&doc, &[], Tol::witness());
    eprintln!("R1-P4: mate-only save: {:?}", saved.as_ref().map(|_| ()));
    let measured = editor_core::measure(
        &doc,
        &[MeasurePrimitive::Distance {
            a: editor_core::Operand::Var(side_a),
            b: editor_core::Operand::from(SitedRef::new(
                top,
                in_part(top, top_body, CapEnd::Start),
            )),
        }],
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    match measured {
        Err(e) => eprintln!("R1-P4: the door refuses the shared read: {e:?}"),
        Ok(out) => {
            let saved = editor_core::persist::save(&out.doc, &[], Tol::witness());
            eprintln!("R1-P4: door admits; save: {:?}", saved.as_ref().map(|_| ()));
            let ev = run(&out.doc, &with_resolver(store));
            eprintln!(
                "R1-P4: measure result: {:?}",
                ev.node_error(out.measured.measures[0]).map(|e| &e.kind)
            );
            assert!(saved.is_ok(), "the edit door admits what save refuses");
        }
    }
}

/// P3: P1 with a MATE: side `b` reads instance `ib`'s face through
/// `t = Transform(ib, +5 z)`, which no placement reads; `ib` itself is
/// placed. The mate declares (`ia` on a gauge, `ib` at an offset). Cut
/// `ib`, `t` and `ib`'s placement, leaving the mate.
#[test]
fn r1_p3_split_carries_a_mate_side_read_through_an_unplaced_transform() {
    use editor_core::{Alignment, AxisSense, ContactClass, DocEdit, MateFrame, MatePrimitive};
    use fixture::resolver::in_part;
    use fixture::step;
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(
        cube(
            ProfileDoc::empty(DocumentId::derive("r1-p3-part"), Tol::witness()),
            0.0,
        ),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive("r1-p3"), Tol::witness());
    let (doc, ia) = insert(doc, Node::instantiate_part(doc_ref.clone()));
    let (doc, ib) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, g) = insert(
        doc,
        Node::gauge(
            None,
            editor_core::Placement::literal(&editor_core::Frame::translation([0.0, 0.0, -5.0])),
        ),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetGauge {
            node: ia,
            gauge: Some(g),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetOffset {
            instance: ib,
            offset: Some(editor_core::Placement::literal(
                &editor_core::Frame::translation([0.0, 0.0, 5.0]),
            )),
            fresh: Vec::new(),
        },
    );
    let (doc, t) = insert(doc, xform(ib, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, _pa) = fixture::place(doc, ia);
    let (doc, pb) = fixture::place(doc, ib);
    let fr = |o: [f64; 3]| {
        MateFrame::authored(o, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness()).unwrap()
    };
    let (doc, mate) = insert(
        doc,
        Node::Mate {
            a: fixture::head_at(ia, in_part(ia, body, CapEnd::End)).into(),
            b: fixture::head_at(t, in_part(ib, body, CapEnd::Start)).into(),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: fr([0.0, 0.0, 1.0]),
                b: fr([0.0, 0.0, 0.0]),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    let opts = with_resolver(store.clone());
    let ev = run(&doc, &opts);
    eprintln!(
        "R1-P3 before: mate {:?}",
        ev.result(mate)
            .map(|r| format!("{r:?}").chars().take(120).collect::<String>())
    );
    let cut: BTreeSet<_> = [ib, t, pb].into_iter().collect();
    match editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("r1-p3-cell"),
        Tol::witness(),
        opts.resolver.as_ref(),
    ) {
        Err(e) => eprintln!("R1-P3: split refused: {e:?}"),
        Ok(out) => {
            let side_b = match out.remainder.node(mate) {
                Some(Node::Mate { b, .. }) => *b,
                other => panic!("{other:?}"),
            };
            eprintln!(
                "R1-P3: split ACCEPTED; side b now reads {:?} named {:?}",
                out.remainder
                    .selection(side_b)
                    .and_then(|s| out.remainder.operation_of(s.body))
                    .and_then(|n| out.remainder.node(n))
                    .map(|n| format!("{n:?}").chars().take(60).collect::<String>()),
                out.remainder.selection(side_b).map(|s| s.names.clone())
            );
            panic!("R1-P3: the split carried a mate side read through an unplaced transform");
        }
    }
}
