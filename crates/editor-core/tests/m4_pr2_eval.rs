//! M4 PR 2 acceptance (spec D7): the die document EVALUATES —
//! doc-param recompute counting, poisoning, cancelation, the typed
//! empty boolean, and split's both-parts value.
//!
//! **Two rows retired here by the 2026-08-13 test-time audit**, each
//! naming the gate that now owns its claim; the retirement notes sit
//! where the rows were, below.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanValue, CancelToken, EvalOptions, EvalOutcome, Evaluation, NodeResult, ProfileDoc,
    SlotId, ValuePayload, evaluate,
};
use fixture::{die, len};
use geom_core::Tol;
use topo::{Body, mass_properties, validate, validate_closed};

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, parallel: bool) -> Evaluation<f64> {
    let opts = EvalOptions {
        parallel,
        ..EvalOptions::default()
    };
    evaluate::<f64>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

/// The final boolean body of an evaluation.
fn final_body(ev: &Evaluation<f64>, id: editor_core::RecipeNodeId) -> &Body<f64> {
    match &ev.value(id).expect("final node evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("expected boolean body, got {}", other.kind_name()),
    }
}

// **RETIRED (2026-08-13 test-time audit): `die_evaluates_to_the_exact\
// _oracle`.** It built `fixture::die()`, ran it cold, and asserted
// `n_nodes == 77`, `outcome == Completed`, `order.len() == 77`,
// `(recomputed, reused) == (77, 0)`, `volume == 7.8359375`,
// `surface_area == 26.625`, `validate == Ok`, `validate_closed == Ok`.
//
// The gate that owns those claims is `m4_pr8_corpus.rs` over corpus
// document `die` (`tests/corpus/die.rs`), which is the SAME document —
// it calls this crate's `fixture::die()` verbatim, "so the corpus and
// the PR 2/5/6 acceptance rows can never drift apart" — carrying
// `MassPin { volume: 7.8359375, area: Some(26.625) }`:
//
// * `every_document_evaluates_green` asserts no failed/poisoned node,
//   `outcome == Completed`, `order.len() == d.len()` and
//   `(recomputed, reused) == (d.len(), 0)` — the cold-evaluation half,
//   over every corpus document rather than this one;
// * `exact_mass_pins_hold` asserts `validate == Ok`,
//   `validate_closed == Ok`, `volume == 7.8359375` EXACTLY and
//   `surface_area == 26.625` EXACTLY (the retired row wrote the area
//   as `24.0 + 21.0 * 4.0 * 0.25 * DEPTH`, which is that number), plus
//   a floor on how many documents still carry a pin at all.
//
// The one assertion that is a literal 77 rather than the document's own
// length stays in this file: `cancelation_returns_a_typed_partial_\
// result` asserts `ev.order.len() == 77` and `(77, 0)` on the same
// document, and `poisoning_hits_descendants_only_and_is_walkable`
// asserts `77 - 3`. So the die's node count is still pinned here.

// **RETIRED (2026-08-13 test-time audit): `incremental_edit_recomputes\
// _only_the_downstream_cone`.** It moved the +z pip's Transform x from
// 1.0 to 0.5 and asserted `outcome == Completed`,
// `(recomputed, reused) == (2, 75)`, that the volume was unchanged, and
// then a 4-way D9 cross-check: sequential-memo, sequential-scratch,
// parallel-scratch and parallel-memo all produce a bit-identical final
// body, with the parallel memo run also at `(2, 75)`.
//
// Two gates own the claim between them:
//
// * the COUNTING half — `m4_pr8_corpus::incremental_recompute_reuses\
//   _the_cone_complement`, which for every corpus document bumps a
//   mid-DAG slot and asserts `(recomputed, reused)` equals
//   `(cone.len(), total - cone.len())` where the cone is derived
//   independently from the recipe DAG (`corpus::cone`), not from the
//   evaluator, plus that the bumped document still evaluates green.
//   A hardcoded `(2, 75)` on one edit of one document is a single cell
//   of that; and the die itself is one of the documents it runs, bumped
//   on `pz_extrude`. The literal `(2, 75)` for a `pz_transform` slot
//   edit ALSO survives verbatim, in `review_m4_pr2::edit_back_restores\
//   _bit_identical_bodies`;
// * the BIT-IDENTITY half — `review_m4_pr2::four_way_schedule_memo\
//   _identity_on_rich_doc`, which runs the same four evaluations
//   (seq/par × scratch/memo) and compares `eval_digest`s: not one final
//   body but EVERY node's result kind, content key, `through` target
//   and full body fingerprint, split sides and pattern instances
//   included, over the 14-node `rich_doc` (diamond, circular pattern,
//   revolve, split, and a poisoned subgraph — a wider node vocabulary
//   than the die's Profile/Extrude/Transform/Declare/Subtract).
//   Memo-after-an-EDIT bit-identity is separately owned by
//   `review_m4_pr2::edit_back_restores_bit_identical_bodies` (edit,
//   re-evaluate, edit back against the stale memo, final body must be
//   bit-identical to the original scratch run) and by
//   `operand_swap_never_reuses_the_prior_subtract` (a memo run over an
//   edited doc must bit-match its own scratch run).
//
// **What is lost, stated:** the PARALLEL schedule is no longer
// exercised on a memo run whose prior belongs to a DIFFERENT (pre-edit)
// document — `four_way_schedule_memo_identity_on_rich_doc`'s memo runs
// take the same document's own prior, and the edited-doc memo rows
// above are sequential. The parallel scheduler is still gated on
// scratch and same-doc-memo digests there, and on verdict logs by
// `m4_pr4_diff::parallel_schedule_preserves_verdict_logs`.

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
        .apply(
            &editor_core::DocEdit::SetDocParam {
                name: editor_core::ParamName::new("pip_depth"),
                value: editor_core::DocParam::Continuous {
                    dim: editor_core::Dimension::Length,
                    value: 0.0625,
                },
            },
            Tol::witness(),
        )
        .unwrap()
        .doc;
    let memo = run(&edited, Some(&full), false);
    assert_eq!((memo.recomputed, memo.reused), (48, 29));
    let vol = mass_properties(final_body(&memo, d.final_node), Tol::witness())
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
        .apply(
            &editor_core::DocEdit::SetParam {
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
            },
            Tol::witness(),
        )
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
    let ev = evaluate::<f64>(&d.doc, None, &cancel, &opts, Tol::witness());
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
    let full = evaluate::<f64>(
        &d.doc,
        Some(&ev),
        &CancelToken::new(),
        &opts2,
        Tol::witness(),
    );
    assert_eq!(full.outcome, EvalOutcome::Completed);
    assert_eq!((full.recomputed, full.reused), (77, 0));
}

#[test]
fn disjoint_subtract_to_empty_is_a_typed_success() {
    use editor_core::{BooleanOp, Node};
    // A 1×1×1 cube inside a 3×3×3 cube: inner ∖ outer = ∅.
    let doc = ProfileDoc::empty_derived("m4_pr2_eval", Tol::witness());
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
    let doc = ProfileDoc::empty_derived("m4_pr2_eval", Tol::witness());
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
            assert_eq!(
                mass_properties(above, Tol::witness()).unwrap().volume,
                2.0 * 2.0 * 1.5
            );
            assert_eq!(
                mass_properties(below, Tol::witness()).unwrap().volume,
                2.0 * 2.0 * 0.5
            );
            for b in [above, below] {
                assert_eq!(validate(b), Ok(()));
                assert_eq!(validate_closed(b), Ok(()));
            }
        }
        other => panic!("expected split value, got {}", other.kind_name()),
    }
}
