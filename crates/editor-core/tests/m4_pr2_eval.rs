//! M4 PR 2 acceptance (spec D7): the die document EVALUATES — exact
//! oracle volume, incremental-recompute counting, D9 cross-checks
//! (memo vs scratch, sequential vs parallel), poisoning, cancelation,
//! the typed empty boolean, and split's both-parts value.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::fmt::Write as _;

use editor_core::{
    BooleanValue, CancelToken, EvalOptions, EvalOutcome, Evaluation, NodeResult, ProfileDoc,
    SlotId, ValuePayload, evaluate,
};
use fixture::{DEPTH, DIE_VOLUME, die, len};
use topo::{Body, mass_properties, validate, validate_closed};

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, parallel: bool) -> Evaluation<f64> {
    let opts = EvalOptions {
        parallel,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts)
}

/// The final boolean body of an evaluation.
fn final_body(ev: &Evaluation<f64>, id: editor_core::RecipeNodeId) -> &Body<f64> {
    match &ev.value(id).expect("final node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected boolean body, got {}", other.kind_name()),
    }
}

/// A bit-level fingerprint of a body: every geometry arena entry (in
/// key order) plus the topology and the mass-property bits. Two
/// fingerprint-equal bodies are bit-identical in everything the
/// kernel stores (Debug prints floats shortest-round-trip, which is
/// bit-faithful for the non-NaN values bodies carry).
fn fingerprint(b: &Body<f64>) -> String {
    let mut s = String::new();
    let m = mass_properties(b).unwrap();
    let _ = writeln!(
        s,
        "V {:016x} A {:016x}",
        m.volume.to_bits(),
        m.surface_area.to_bits()
    );
    for (k, p) in b.points() {
        let _ = writeln!(
            s,
            "P {k:?} {:016x} {:016x} {:016x}",
            p.x.to_bits(),
            p.y.to_bits(),
            p.z.to_bits()
        );
    }
    for (k, c) in b.curves() {
        let _ = writeln!(s, "C {k:?} {c:?}");
    }
    for (k, sf) in b.surfaces() {
        let _ = writeln!(s, "S {k:?} {sf:?}");
    }
    for (k, e) in b.edges() {
        let _ = writeln!(s, "E {k:?} {e:?}");
    }
    for (k, f) in b.faces() {
        let _ = writeln!(s, "F {k:?} {f:?}");
    }
    s
}

#[test]
fn die_evaluates_to_the_exact_oracle() {
    let d = die();
    assert_eq!(d.n_nodes, 77); // 2 cube + 12 masters + 21×3 pip triples (M4 PR 5: + Declare)
    let ev = run(&d.doc, None, false);
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    assert_eq!(ev.order.len(), 77);
    assert_eq!((ev.recomputed, ev.reused), (77, 0));
    let body = final_body(&ev, d.final_node);
    let m = mass_properties(body).unwrap();
    assert_eq!(m.volume, DIE_VOLUME); // exactly 7.8359375
    assert_eq!(m.surface_area, 24.0 + 21.0 * 4.0 * 0.25 * DEPTH);
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
}

#[test]
fn incremental_edit_recomputes_only_the_downstream_cone() {
    let d = die();
    let full = run(&d.doc, None, false);

    // Move the +z pip from (1,1) to (0.5,1) — one slot edit on the
    // LAST Transform; its cone is exactly {transform, final subtract}.
    let edited = d
        .doc
        .apply(&editor_core::DocEdit::SetParam {
            node: d.pz_transform,
            slot: SlotId::Translation(editor_core::Axis3::X),
            expr: len(0.5),
        })
        .unwrap()
        .doc;

    let memo = run(&edited, Some(&full), false);
    assert_eq!(memo.outcome, EvalOutcome::Completed);
    assert_eq!((memo.recomputed, memo.reused), (2, 75)); // D4 acceptance
    let vol = mass_properties(final_body(&memo, d.final_node))
        .unwrap()
        .volume;
    assert_eq!(vol, DIE_VOLUME); // pip still fully in the face

    // D9 cross-checks: the memoized result bit-matches a from-scratch
    // evaluation (D4), and both schedules produce the same bits with
    // parallelism on and off (D6).
    let scratch = run(&edited, None, false);
    let par_scratch = run(&edited, None, true);
    let par_memo = run(&edited, Some(&full), true);
    assert_eq!((par_memo.recomputed, par_memo.reused), (2, 75));
    let fp = fingerprint(final_body(&memo, d.final_node));
    for other in [&scratch, &par_scratch, &par_memo] {
        assert_eq!(fp, fingerprint(final_body(other, d.final_node)));
    }
}

#[test]
fn doc_param_edit_recomputes_the_param_cone() {
    let d = die();
    let full = run(&d.doc, None, false);
    // pip_depth 0.125 → 0.0625: every pip master extrude and all 42
    // downstream pip nodes recompute; the 7 profiles, the cube
    // extrude, and the 21 Declare nodes (pure recipe data) are
    // reused.
    let edited = d
        .doc
        .apply(&editor_core::DocEdit::SetDocParam {
            name: editor_core::ParamName::new("pip_depth"),
            value: editor_core::DocParam::Continuous {
                dim: editor_core::Dimension::Length,
                value: 0.0625,
            },
        })
        .unwrap()
        .doc;
    let memo = run(&edited, Some(&full), false);
    assert_eq!((memo.recomputed, memo.reused), (48, 29));
    let vol = mass_properties(final_body(&memo, d.final_node))
        .unwrap()
        .volume;
    assert_eq!(vol, 8.0 - 21.0 * 0.25 * 0.25 * 0.0625); // 7.91796875
}

#[test]
fn poisoning_hits_descendants_only_and_is_walkable() {
    let d = die();
    // Break the +z master extrude's distance: pip_depth / 0 evaluates
    // to a non-finite value — the PR 1 NonFiniteResult obligation
    // surfaces with (node, slot) context.
    let broken = d
        .doc
        .apply(&editor_core::DocEdit::SetParam {
            node: d.pz_extrude,
            slot: SlotId::Distance,
            expr: editor_core::Expr::div(
                editor_core::Expr::param(
                    editor_core::ParamName::new("pip_depth"),
                    editor_core::Dimension::Length,
                ),
                fixture::scl(0.0),
            )
            .unwrap(),
        })
        .unwrap()
        .doc;
    let ev = run(&broken, None, false);
    assert_eq!(ev.outcome, EvalOutcome::Completed);

    // The root cause: Failed with (node, slot) context, unaltered.
    match ev.nodes.get(&d.pz_extrude) {
        Some(NodeResult::Failed(e)) => {
            assert_eq!(e.node, d.pz_extrude);
            match &e.kind {
                editor_core::NodeErrorKind::Expr { slot, source } => {
                    assert_eq!(*slot, SlotId::Distance);
                    assert_eq!(*source, editor_core::EvalError::NonFiniteResult);
                }
                other => panic!("expected Expr error, got {other:?}"),
            }
        }
        other => panic!("expected Failed, got {other:?}"),
    }

    // Descendants ONLY: exactly the +z transform and the final
    // subtract are poisoned, each pointing at the failed extrude.
    let poisoned: Vec<_> = ev
        .nodes
        .iter()
        .filter_map(|(id, r)| match r {
            NodeResult::Poisoned { through } => Some((*id, *through)),
            _ => None,
        })
        .collect();
    assert_eq!(poisoned.len(), 2);
    for (_, through) in &poisoned {
        assert_eq!(*through, d.pz_extrude);
    }
    // The independent prefix completed: the 20th subtract (last node
    // before the poisoned pair) carries real material.
    let ok_count = ev
        .nodes
        .values()
        .filter(|r| matches!(r, NodeResult::Ok(_)))
        .count();
    assert_eq!(ok_count, 77 - 3); // all but Failed + 2 Poisoned
}

#[test]
fn cancelation_returns_a_typed_partial_result() {
    let d = die();
    let cancel = CancelToken::new();
    cancel.cancel();
    let opts = EvalOptions::default();
    let ev = evaluate::<f64>(&d.doc, None, &cancel, &opts);
    assert_eq!(ev.outcome, EvalOutcome::Canceled);
    assert!(ev.nodes.is_empty()); // canceled before the first node
    assert_eq!(ev.order.len(), 77); // order is data, not schedule
    assert_eq!(ev.epoch, opts.epoch); // the identity token round-trips

    // Distinct evaluations carry distinct minted epochs (GQ2's
    // stale-result discrimination hook).
    let opts2 = EvalOptions::default();
    assert_ne!(opts.epoch, opts2.epoch);

    // A canceled evaluation is a legal (empty) memo: a fresh run from
    // it completes and computes everything.
    let full = evaluate::<f64>(&d.doc, Some(&ev), &CancelToken::new(), &opts2);
    assert_eq!(full.outcome, EvalOutcome::Completed);
    assert_eq!((full.recomputed, full.reused), (77, 0));
}

#[test]
fn disjoint_subtract_to_empty_is_a_typed_success() {
    use editor_core::{BooleanOp, Node};
    // A 1×1×1 cube inside a 3×3×3 cube: inner ∖ outer = ∅.
    let doc = ProfileDoc::empty_derived("m4_pr2_eval");
    let (doc, small_p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]],
        )),
    );
    let (doc, small) = fixture::insert(
        doc,
        Node::Extrude {
            profile: small_p,
            distance: len(1.0),
        },
    );
    let (doc, big_p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, -1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]],
        )),
    );
    let (doc, big) = fixture::insert(
        doc,
        Node::Extrude {
            profile: big_p,
            distance: len(3.0),
        },
    );
    let (doc, sub) = fixture::insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: small,
            b: big,
            declare: None,
        },
    );
    // A downstream consumer of the empty value: typed EmptyOperand
    // failure, not a poison and not a panic.
    let (doc, consumer) = fixture::insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: sub,
            b: big,
            declare: None,
        },
    );
    let ev = run(&doc, None, false);
    match &ev.value(sub).expect("empty is a VALUE").payload {
        ValuePayload::Boolean(BooleanValue::Empty) => {}
        other => panic!("expected typed empty, got {}", other.kind_name()),
    }
    match ev.nodes.get(&consumer) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::EmptyOperand { input } => assert_eq!(*input, sub),
            other => panic!("expected EmptyOperand, got {other:?}"),
        },
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn split_evaluates_both_parts_role_tagged() {
    use editor_core::{Datum, DatumValue, Node, SplitSide};
    let doc = ProfileDoc::empty_derived("m4_pr2_eval");
    let (doc, prof) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        )),
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile: prof,
            distance: len(2.0),
        },
    );
    // The tool: a datum plane z = 0.5 (normal +z).
    let (doc, plane) = fixture::insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [fixture::scl(0.0), fixture::scl(0.0), fixture::scl(2.0)],
        }),
    );
    let (doc, split_node) = fixture::insert(
        doc,
        Node::Split {
            target: cube,
            tool: plane,
        },
    );
    let ev = run(&doc, None, false);
    // The datum evaluated with a NORMALIZED normal.
    match &ev.value(plane).unwrap().payload {
        ValuePayload::Datum(DatumValue::Plane { normal, .. }) => {
            assert_eq!((normal.x, normal.y, normal.z), (0.0, 0.0, 1.0));
        }
        other => panic!("expected datum, got {}", other.kind_name()),
    }
    match &ev.value(split_node).unwrap().payload {
        ValuePayload::Split { above, below } => {
            let (SplitSide::Body(above), SplitSide::Body(below)) = (above, below) else {
                panic!("both sides carry material");
            };
            assert_eq!(mass_properties(above).unwrap().volume, 2.0 * 2.0 * 1.5);
            assert_eq!(mass_properties(below).unwrap().volume, 2.0 * 2.0 * 0.5);
            for b in [above, below] {
                assert_eq!(validate(b), Ok(()));
                assert_eq!(validate_closed(b), Ok(()));
            }
        }
        other => panic!("expected split value, got {}", other.kind_name()),
    }
}
