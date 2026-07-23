//! ADVERSARIAL REVIEW suite for M4 PR 1 (PR #81) — falsification
//! probes R1–R8 per the review charter. Each test names its
//! assignment; witnesses of EXPECTED gaps (pre-fix-pass) are marked.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::float_cmp)]

use editor_core::{
    Dimension, DocEdit, DocParam, EditError, Expr, ParamEnv, ParamName, RecipeNodeId, SlotId, eval,
    eval_count,
};

type Doc = editor_core::Doc<&'static str>;
type Edit = DocEdit<&'static str>;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

/// Insert a datum point whose x-component is `x` (bit-exact carrier).
fn point_edit(x: Expr) -> Edit {
    DocEdit::InsertNode {
        node: editor_core::Node::Datum(editor_core::Datum::Point {
            position: [x, len(0.0), len(0.0)],
        }),
    }
}

fn apply_all(doc: Doc, edits: &[Edit]) -> (Doc, Vec<RecipeNodeId>) {
    let mut d = doc;
    let mut minted = Vec::new();
    for e in edits {
        let a = d.apply(e).unwrap();
        minted.extend(a.record.minted);
        d = a.doc;
    }
    (d, minted)
}

/// R1 — replay reproduces the doc BIT-identically under adversarial
/// floats (-0.0, subnormals, last-ulp pairs, NaN payload) with delete
/// churn interleaved. NaN literal is constructible today (R6 door 1
/// open pre-fix-pass), so replay must carry it bit-faithfully too.
#[test]
fn r1_replay_bit_identity_adversarial() {
    let ulp1 = f64::from_bits(0x3FF0000000000001); // 1.0 + 1 ulp
    let nan_payload = f64::from_bits(0x7FF80000DEADBEEF);
    let adversarial = [
        -0.0,
        0.0,
        5e-324,          // smallest subnormal
        -5e-324,
        f64::MIN_POSITIVE - 5e-324, // largest subnormal neighborhood
        1.0,
        ulp1,
        -ulp1,
        f64::MAX,
        nan_payload,     // door 1 open today: literal(NaN) accepted
    ];
    let mut log: Vec<Edit> = vec![Edit::SetDocParam {
        name: ParamName::new("neg_zero"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: -0.0,
        },
    }];
    let mut doc = Doc::empty().apply(&log[0]).unwrap().doc;
    let mut minted = Vec::new();
    for &v in &adversarial {
        let e = point_edit(len(v));
        let a = doc.apply(&e).unwrap();
        minted.push(a.record.minted.unwrap());
        doc = a.doc;
        log.push(e);
    }
    // Interleaved deletes (including the newest id) then more inserts.
    for &d in &[minted[3], minted[7], minted[9]] {
        let e = Edit::DeleteNode { id: d };
        doc = doc.apply(&e).unwrap().doc;
        log.push(e);
    }
    let e = point_edit(len(-0.0));
    let a = doc.apply(&e).unwrap();
    // D3: ids strictly increase even after deleting the highest one.
    assert!(a.record.minted.unwrap() > *minted.iter().max().unwrap());
    doc = a.doc;
    log.push(e);
    // Re-insert a NaN carrier so the FINAL doc holds one (the earlier
    // one was deleted in the churn) — replay must carry its bits.
    let e = point_edit(len(nan_payload));
    doc = doc.apply(&e).unwrap().doc;
    log.push(e);
    let replayed = Doc::replay(&log).unwrap();
    assert_bit_identical(&replayed, &doc);
}

/// R1 WITNESS — `Doc: PartialEq` (which the crate's own replay test
/// and `diff` rely on) is float-semantic, NOT bit-semantic: a 0.0 vs
/// -0.0 payload difference is INVISIBLE to both, and a NaN-carrying
/// doc reports UNEQUAL to its bit-identical replay. Replay itself is
/// bit-faithful (r1_replay_bit_identity_adversarial); the CLAIM
/// verification tooling is what conflates.
#[test]
fn r1_partialeq_and_diff_conflate_signed_zero_and_nan() {
    let (pos, _) = apply_all(Doc::empty(), &[point_edit(len(0.0))]);
    let (neg, _) = apply_all(Doc::empty(), &[point_edit(len(-0.0))]);
    // Bitwise the docs DIFFER…
    let vp = eval::<f64>(
        pos.node(pos.order()[0]).unwrap().expr(SlotId::Origin(editor_core::Axis3::X)).unwrap(),
        &pos.param_env(),
    )
    .unwrap();
    let vn = eval::<f64>(
        neg.node(neg.order()[0]).unwrap().expr(SlotId::Origin(editor_core::Axis3::X)).unwrap(),
        &neg.param_env(),
    )
    .unwrap();
    assert_ne!(vp.to_bits(), vn.to_bits(), "bits differ");
    // …but PartialEq and diff both say "identical" (witnessed gap).
    assert_eq!(pos, neg, "PartialEq conflates -0.0/0.0 (witness)");
    assert!(pos.diff(&neg).is_empty(), "diff blind to -0.0 (witness)");
    // NaN: a doc is UNEQUAL to its own bit-identical replay.
    let e = point_edit(len(f64::NAN));
    let (nan_doc, _) = apply_all(Doc::empty(), std::slice::from_ref(&e));
    let replayed = Doc::replay(std::slice::from_ref(&e)).unwrap();
    assert_bit_identical(&replayed, &nan_doc);
    assert_ne!(nan_doc, replayed, "PartialEq rejects NaN==NaN (witness)");
    let d = nan_doc.diff(&replayed);
    assert!(!d.is_empty(), "diff reports a phantom change (witness)");
}

/// R2 — dimension smuggling via nested promotion / synthetic
/// dimensionless-ness / chained Mul-Div. Every arm here must refuse.
#[test]
fn r2_dimension_smuggling_probes() {
    use editor_core::DimensionError as DE;
    let c = || Expr::count(3);
    // Nested promotion: CountToScalar(CountToScalar(c)) — inner is
    // Scalar, outer demands Count.
    let inner = Expr::count_to_scalar(c()).unwrap();
    assert!(matches!(
        Expr::count_to_scalar(inner.clone()),
        Err(DE::NotCount { .. })
    ));
    // Synthetic dimensionless: Length × promoted Count → Length (ok),
    // then × Length must still refuse (no laundering through chains).
    let l_times_promoted = Expr::mul(len(2.0), inner.clone()).unwrap();
    assert_eq!(l_times_promoted.dim(), Dimension::Length);
    assert!(matches!(
        Expr::mul(l_times_promoted.clone(), len(1.0)),
        Err(DE::MulNeedsScalar { .. })
    ));
    // Div chain: (Length/Scalar) is Length; dividing BY it refused.
    let l_div_s = Expr::div(len(1.0), scl(2.0)).unwrap();
    assert_eq!(l_div_s.dim(), Dimension::Length);
    assert!(matches!(
        Expr::div(scl(1.0), l_div_s.clone()),
        Err(DE::DivNeedsScalarDivisor { .. })
    ));
    // Div by raw Count and by promoted-Count-… : raw refused loudly;
    // promoted IS Scalar so it passes (correct: explicit promotion).
    assert!(matches!(
        Expr::div(len(1.0), c()),
        Err(DE::CountNeedsExplicitPromotion { .. })
    ));
    assert!(Expr::div(len(1.0), inner.clone()).is_ok());
    // Count division is NOT closed (spec lists add/sub/mul/min/max).
    assert!(Expr::div(c(), c()).is_err());
    // min/max cross-dimension, including Count vs Scalar.
    assert!(Expr::min(len(1.0), ang(1.0)).is_err());
    assert!(Expr::max(c(), scl(1.0)).is_err());
    // atan2 edges: mixed Length/Angle refused; Count/Count refused
    // (needs explicit promotion); Angle/Angle and Scalar/Scalar both
    // ACCEPTED under deviation 4's "any shared continuous dimension".
    assert!(Expr::atan2(len(1.0), ang(1.0)).is_err());
    assert!(matches!(
        Expr::atan2(c(), c()),
        Err(DE::CountNeedsExplicitPromotion { .. })
    ));
    assert_eq!(Expr::atan2(ang(1.0), ang(2.0)).unwrap().dim(), Dimension::Angle);
    assert_eq!(Expr::atan2(scl(1.0), scl(2.0)).unwrap().dim(), Dimension::Angle);
    // Neg is dimension-transparent: Neg(Length) still refuses ×Length.
    let neg_l = Expr::neg(len(1.0));
    assert!(Expr::mul(neg_l, len(1.0)).is_err());
    // Trig on promoted Count refused (Scalar, not Angle).
    assert!(Expr::sin(inner).is_err());
}

/// R2 — a single expression referencing one param under TWO
/// contradictory dimensions is CONSTRUCTIBLE (param() trusts its
/// caller); both `apply` and `eval` must catch it downstream.
#[test]
fn r2_contradictory_param_dims_caught_downstream() {
    let p_scl = Expr::param(ParamName::new("q"), Dimension::Scalar);
    let p_len = Expr::param(ParamName::new("q"), Dimension::Length);
    // mul(Scalar, Length) → Length: constructible with BOTH refs.
    let expr = Expr::mul(p_scl, p_len).unwrap();
    // eval: whichever binding "q" has, one ref mismatches — typed.
    let mut env: ParamEnv<f64> = ParamEnv::default();
    env.bindings.insert(
        ParamName::new("q"),
        editor_core::ParamValue::Continuous {
            dim: Dimension::Length,
            value: 2.0,
        },
    );
    assert!(matches!(
        eval::<f64>(&expr, &env),
        Err(editor_core::EvalError::ParamDimensionMismatch { .. })
    ));
    // apply: a slot carrying the contradiction is refused whichever
    // dimension the doc table declares.
    let doc = Doc::empty()
        .apply(&Edit::SetDocParam {
            name: ParamName::new("q"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 2.0,
            },
        })
        .unwrap()
        .doc;
    let res = doc.apply(&point_edit(expr));
    assert!(
        matches!(res, Err(EditError::DocParamDimensionMismatch { .. })),
        "got {res:?}"
    );
}

/// R2/R6 WITNESS (BUG) — `eval` of `CountToScalar(i64::MIN)` PANICS
/// (`n.abs()` overflows) instead of returning the typed
/// `CountToScalarInexact`: the ±2^53 guard calls `i64::abs` before
/// range-checking. Fail-loud by accident, not by contract — a typed-
/// error discipline break (and in release-without-overflow-checks,
/// `abs` would WRAP negative and the guard would PASS i64::MIN into
/// an inexact cast).
#[test]
fn r2_count_to_scalar_i64_min_panics_instead_of_typed_error() {
    let e = Expr::count_to_scalar(Expr::count(i64::MIN)).unwrap();
    let env = ParamEnv::<f64>::default();
    let outcome = std::panic::catch_unwind(|| eval::<f64>(&e, &env));
    match outcome {
        Err(_) => eprintln!("WITNESS: eval(CountToScalar(i64::MIN)) panicked"),
        Ok(r) => assert!(
            matches!(r, Err(editor_core::EvalError::CountToScalarInexact(_))),
            "if it no longer panics it must be the typed error; got {r:?}"
        ),
    }
}

/// R3 WITNESS — ExprPath is POSITIONAL within its slot: replacing an
/// ANCESTOR of the referent (or the whole slot) with a same-shape
/// expression makes an old path resolve to a DIFFERENT subexpression
/// with no error and no way to detect it (no generation/version on
/// the slot). D5 only *claims* stability under edits to other
/// expressions/sibling subtrees, so this is out-of-claim — but PR 5's
/// GeomSource must not assume same-slot edits are detectable.
#[test]
fn r3_ancestor_replace_silently_repoints_exprpath() {
    use editor_core::{Axis3, ExprPath};
    // Slot: x = 1.0 + 2.0; path [1] refers to the literal 2.0.
    let e0 = Expr::add(len(1.0), len(2.0)).unwrap();
    let ins = DocEdit::InsertNode {
        node: editor_core::Node::Datum(editor_core::Datum::Point {
            position: [e0, len(0.0), len(0.0)],
        }),
    };
    let a = Doc::empty().apply(&ins).unwrap();
    let id = a.record.minted.unwrap();
    let path = ExprPath {
        node: id,
        slot: SlotId::Origin(Axis3::X),
        path: vec![1],
    };
    let before = eval::<f64>(a.doc.expr_at(&path).unwrap(), &a.doc.param_env()).unwrap();
    assert_eq!(before, 2.0);
    // Replace the ANCESTOR (whole slot, path []) with 5.0 + 7.0.
    let replaced = a
        .doc
        .apply(&Edit::SetExpression {
            path: ExprPath {
                node: id,
                slot: SlotId::Origin(Axis3::X),
                path: vec![],
            },
            expr: Expr::add(len(5.0), len(7.0)).unwrap(),
        })
        .unwrap()
        .doc;
    // The old path still RESOLVES — to a different subexpression.
    let after = eval::<f64>(replaced.expr_at(&path).unwrap(), &replaced.param_env()).unwrap();
    assert_eq!(after, 7.0, "silent re-point (witness): 2.0 became 7.0");
    // Arity-shrinking ancestor replace: old path now dangles as None
    // (detectable, but an Option, not a typed error).
    let shrunk = replaced
        .apply(&Edit::SetExpression {
            path: ExprPath {
                node: id,
                slot: SlotId::Origin(Axis3::X),
                path: vec![],
            },
            expr: len(9.0),
        })
        .unwrap()
        .doc;
    assert!(shrunk.expr_at(&path).is_none(), "dangles as None, untyped");
}

/// R3 — the D5 CLAIM itself: a referent survives (a) edits to other
/// expressions, (b) replacement of a SIBLING subtree via SetExpression,
/// (c) SetParam on a different slot of the same node — bit-checked.
#[test]
fn r3_referent_survives_out_of_claim_edits_bitwise() {
    use editor_core::{Axis3, ExprPath};
    let marker = f64::from_bits(0x3FF00000000000AB); // recognizable bits
    let e0 = Expr::add(len(marker), len(2.0)).unwrap();
    let ins = DocEdit::InsertNode {
        node: editor_core::Node::Datum(editor_core::Datum::Point {
            position: [e0, len(0.0), len(0.0)],
        }),
    };
    let a = Doc::empty().apply(&ins).unwrap();
    let id = a.record.minted.unwrap();
    let referent = ExprPath {
        node: id,
        slot: SlotId::Origin(Axis3::X),
        path: vec![0],
    };
    let check = |d: &Doc| {
        let v = eval::<f64>(d.expr_at(&referent).unwrap(), &d.param_env()).unwrap();
        assert_eq!(v.to_bits(), marker.to_bits(), "referent bits");
    };
    check(&a.doc);
    // (a) edit ANOTHER node entirely.
    let (d, _) = apply_all(a.doc, &[point_edit(len(4.0))]);
    check(&d);
    // (b) replace the SIBLING subtree [1] of the same slot.
    let d = d
        .apply(&Edit::SetExpression {
            path: ExprPath {
                node: id,
                slot: SlotId::Origin(Axis3::X),
                path: vec![1],
            },
            expr: Expr::mul(scl(3.0), len(8.0)).unwrap(),
        })
        .unwrap()
        .doc;
    check(&d);
    // (c) SetParam a DIFFERENT slot on the same node.
    let d = d
        .apply(&Edit::SetParam {
            node: id,
            slot: SlotId::Origin(Axis3::Z),
            expr: len(6.0),
        })
        .unwrap()
        .doc;
    check(&d);
}

/// R4 WITNESS (HOLE) — `StableName.node` is a `RecipeNodeId` but is
/// NOT a DAG edge (`Declare::inputs()` is empty), so:
/// (1) DeleteNode of a node referenced ONLY by a Declare's pairs is
///     ACCEPTED → the live Declare holds a stale RecipeNodeId;
/// (2) InsertNode of a Declare whose StableName points at a NEVER-
///     EXISTING node is ACCEPTED (no UnresolvedInput).
/// Spec D3 says "apply rejects unresolvable refs"; whether name refs
/// count as refs is the open question this witnesses.
#[test]
fn r4_stablename_node_refs_escape_ref_validation() {
    use editor_core::{EntityKind, Node, StableName};
    let (doc, ids) = apply_all(
        Doc::empty(),
        &[point_edit(len(1.0))], // the node the name will denote
    );
    let target = ids[0];
    let declare = |node| Edit::InsertNode {
        node: Node::Declare {
            pairs: vec![(
                StableName {
                    kind: EntityKind::Face,
                    node,
                    path: vec![],
                },
                StableName {
                    kind: EntityKind::Face,
                    node,
                    path: vec![],
                },
            )],
        },
    };
    let a = doc.apply(&declare(target)).unwrap();
    let declare_id = a.record.minted.unwrap();
    // (1) Delete the named node — ACCEPTED despite the live Declare.
    let after = a.doc.apply(&Edit::DeleteNode { id: target });
    let after = after.expect("WITNESS: delete of name-referenced node accepted");
    assert!(after.doc.node(target).is_none());
    // The Declare survives, holding a stale id.
    match after.doc.node(declare_id).unwrap() {
        Node::Declare { pairs } => {
            assert_eq!(pairs[0].0.node, target, "stale RecipeNodeId held");
        }
        n => panic!("expected Declare, got {n:?}"),
    }
    // (2) Insert a Declare naming an id that never existed.
    let phantom = RecipeNodeId(9999);
    let res = Doc::empty().apply(&declare(phantom));
    assert!(
        res.is_ok(),
        "WITNESS: phantom StableName.node accepted at insert"
    );
    // Contrast: a DAG-edge ref to the same phantom is refused.
    let res2 = Doc::empty().apply(&Edit::InsertNode {
        node: Node::Extrude {
            profile: phantom,
            distance: len(1.0),
        },
    });
    assert!(matches!(res2, Err(EditError::UnresolvedInput { .. })));
}

/// R4 — cycles via multi-edit sequences: the v1 edit vocabulary has
/// NO arm that rewires a node's inputs after insertion (SetParam /
/// SetExpression touch expression slots only; Circular pattern's axis
/// ref lives outside every slot), so A→B-then-B→A is UNCONSTRUCTIBLE
/// through the public API — the defensive WouldCycle stays unreachable.
/// This pins the exhaustive attempt.
#[test]
fn r4_cycle_unconstructible_by_any_edit_sequence() {
    use editor_core::Node;
    let (doc, ids) = apply_all(
        Doc::empty(),
        &[
            Edit::InsertNode {
                node: Node::Profile("p"),
            },
        ],
    );
    let a = doc
        .apply(&Edit::InsertNode {
            node: Node::Extrude {
                profile: ids[0],
                distance: len(1.0),
            },
        })
        .unwrap();
    let extrude = a.record.minted.unwrap();
    let doc = a.doc;
    // Forward ref to a FUTURE id (the only way to seed a cycle at
    // insert) is refused: the id isn't live yet.
    let next_would_be = RecipeNodeId(extrude.0 + 1);
    let res = doc.apply(&Edit::InsertNode {
        node: Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: extrude,
            b: next_would_be,
            declare: None,
        },
    });
    assert!(matches!(res, Err(EditError::UnresolvedInput { .. })));
    // And no edit arm can touch `Extrude.profile` afterwards: the
    // slot vocabulary for Extrude is exactly [Distance].
    let slots = doc.node(extrude).unwrap().slots();
    assert_eq!(slots, vec![SlotId::Distance]);
}

/// R4 — SetDocParam re-declaration sweep + the flagged no-delete-arm
/// hole: a dimension flip under a referencing slot is REFUSED (sweep
/// works); a Count→Count value change passes; and since NO edit can
/// remove a param, reference stranding via deletion is impossible in
/// v1 (the hole is the absent arm, not a validation gap).
#[test]
fn r4_setdocparam_sweep_and_no_delete_arm() {
    let name = ParamName::new("d");
    let doc = Doc::empty()
        .apply(&Edit::SetDocParam {
            name: name.clone(),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.5,
            },
        })
        .unwrap()
        .doc;
    let (doc, _) = apply_all(
        doc,
        &[point_edit(Expr::param(name.clone(), Dimension::Length))],
    );
    // Dimension flip out from under the referencing slot: refused.
    let flip = doc.apply(&Edit::SetDocParam {
        name: name.clone(),
        value: DocParam::Continuous {
            dim: Dimension::Angle,
            value: 0.5,
        },
    });
    assert!(
        matches!(flip, Err(EditError::DocParamDimensionMismatch { .. })),
        "got {flip:?}"
    );
    // Kind flip Continuous→Count under a reference: also refused.
    let kind_flip = doc.apply(&Edit::SetDocParam {
        name: name.clone(),
        value: DocParam::Count { value: 2 },
    });
    assert!(matches!(
        kind_flip,
        Err(EditError::DocParamDimensionMismatch { .. })
    ));
    // Same-dimension value change: accepted, non-structural.
    let ok = doc
        .apply(&Edit::SetDocParam {
            name,
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.75,
            },
        })
        .unwrap();
    assert!(!ok.record.structural);
    // An UNREFERENCED param may flip freely (nothing to strand).
    let free = ok
        .doc
        .apply(&Edit::SetDocParam {
            name: ParamName::new("unused"),
            value: DocParam::Count { value: 1 },
        })
        .unwrap();
    assert!(free.record.structural, "Count doc-param set flags structural");
}

/// BIT-compare two docs: structure via PartialEq fields, floats via
/// to_bits (PartialEq would conflate -0.0/0.0 and reject NaN==NaN).
/// Slot expressions are compared by evaluating literal trees to f64
/// bits (the crate exposes no direct literal accessor).
fn assert_bit_identical(a: &Doc, b: &Doc) {
    assert_eq!(a.order(), b.order(), "order");
    assert_eq!(a.epsilon().to_bits(), b.epsilon().to_bits(), "epsilon");
    assert_eq!(a.metadata(), b.metadata(), "metadata");
    let (pa, pb) = (a.params(), b.params());
    assert_eq!(pa.len(), pb.len(), "param count");
    for (name, p) in pa {
        match (p, pb.get(name).expect("param present")) {
            (
                DocParam::Continuous { dim, value },
                DocParam::Continuous {
                    dim: d2,
                    value: v2,
                },
            ) => {
                assert_eq!(dim, d2, "param dim {name:?}");
                assert_eq!(value.to_bits(), v2.to_bits(), "param bits {name:?}");
            }
            (DocParam::Count { value }, DocParam::Count { value: v2 }) => {
                assert_eq!(value, v2, "count param {name:?}");
            }
            (x, y) => panic!("param kind mismatch {name:?}: {x:?} vs {y:?}"),
        }
    }
    for &id in a.order() {
        let (na, nb) = (a.node(id).unwrap(), b.node(id).unwrap());
        assert_eq!(na.inputs(), nb.inputs(), "inputs of {id:?}");
        assert_eq!(na.slots(), nb.slots(), "slots of {id:?}");
        for slot in na.slots() {
            let (ea, eb) = (na.expr(slot).unwrap(), nb.expr(slot).unwrap());
            assert_eq!(ea.dim(), eb.dim(), "slot dim {id:?}/{slot:?}");
            if slot.is_structural() {
                let (va, vb) = (
                    eval_count(ea, &a.param_env::<f64>()),
                    eval_count(eb, &b.param_env::<f64>()),
                );
                assert_eq!(va, vb, "count slot {id:?}/{slot:?}");
            } else {
                let va = eval::<f64>(ea, &a.param_env()).unwrap();
                let vb = eval::<f64>(eb, &b.param_env()).unwrap();
                assert_eq!(
                    va.to_bits(),
                    vb.to_bits(),
                    "slot bits {id:?}/{slot:?}: {va:e} vs {vb:e}"
                );
            }
        }
    }
}
