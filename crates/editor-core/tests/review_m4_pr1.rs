//! ADVERSARIAL REVIEW suite for M4 PR 1 (PR #81) — falsification
//! probes R1–R8 per the review charter. Each test names its
//! assignment; witnesses of EXPECTED gaps (pre-fix-pass) are marked.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::float_cmp)]

use editor_core::{
    Dimension, DocEdit, DocParam, EditError, Expr, ParamEnv, ParamName, RecipeNodeId, SlotId, eval,
    eval_count,
};
use geom_core::Tol;

// v4: `Doc<P>` requires `P: ProfilePayload` (defaults = the retired
// opaque behavior), which a foreign `&str` cannot implement here — a
// transparent local newtype carries the same test payloads.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Fake(&'static str);
impl editor_core::ProfilePayload for Fake {}
type Doc = editor_core::Doc<Fake>;
type Edit = DocEdit<Fake>;

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
        let a = d.apply(e, Tol::witness()).unwrap();
        minted.extend(a.record.minted);
        d = a.doc;
    }
    (d, minted)
}

/// R1 — replay reproduces the doc BIT-identically under adversarial
/// floats (-0.0, subnormals, last-ulp pairs) with delete churn
/// interleaved. (Fix pass: NaN payloads are no longer constructible —
/// door 1 refuses non-finite literals — so the adversarial set is the
/// full FINITE bit-hazard menagerie.)
#[test]
fn r1_replay_bit_identity_adversarial() {
    let ulp1 = f64::from_bits(0x3FF0000000000001); // 1.0 + 1 ulp
    let adversarial = [
        -0.0,
        0.0,
        5e-324, // smallest subnormal
        -5e-324,
        f64::MIN_POSITIVE - 5e-324, // largest subnormal neighborhood
        1.0,
        ulp1,
        -ulp1,
        f64::MAX,
        -f64::MAX,
    ];
    let mut log: Vec<Edit> = vec![Edit::SetDocParam {
        name: ParamName::new("neg_zero"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: -0.0,
        },
    }];
    let mut doc = Doc::empty_derived("review_m4_pr1", Tol::witness())
        .apply(&log[0], Tol::witness())
        .unwrap()
        .doc;
    let mut minted = Vec::new();
    for &v in &adversarial {
        let e = point_edit(len(v));
        let a = doc.apply(&e, Tol::witness()).unwrap();
        minted.push(a.record.minted.unwrap());
        doc = a.doc;
        log.push(e);
    }
    // Interleaved deletes (including the newest id) then more inserts.
    for &d in &[minted[3], minted[7], minted[9]] {
        let e = Edit::DeleteNode { id: d };
        doc = doc.apply(&e, Tol::witness()).unwrap().doc;
        log.push(e);
    }
    let e = point_edit(len(-0.0));
    let a = doc.apply(&e, Tol::witness()).unwrap();
    // D3: ids strictly increase even after deleting the highest one.
    assert!(a.record.minted.unwrap() > *minted.iter().max().unwrap());
    doc = a.doc;
    log.push(e);
    // Re-insert a last-ulp carrier after the churn — replay must
    // carry its exact bits through the re-minted id stream.
    let e = point_edit(len(-ulp1));
    doc = doc.apply(&e, Tol::witness()).unwrap().doc;
    log.push(e);
    let replayed = Doc::replay(doc.id(), &log, Tol::witness()).unwrap();
    assert_bit_identical(&replayed, &doc);
    // The crate's own bit-semantic comparator agrees (fix pass).
    assert!(replayed.bit_eq(&doc), "Doc::bit_eq on replay");
}

/// R1 WITNESS — `Doc: PartialEq` (which the crate's own replay test
/// and `diff` rely on) is float-semantic, NOT bit-semantic: a 0.0 vs
/// -0.0 payload difference is INVISIBLE to both, and a NaN-carrying
/// doc reports UNEQUAL to its bit-identical replay. Replay itself is
/// bit-faithful (r1_replay_bit_identity_adversarial); the CLAIM
/// verification tooling is what conflates.
#[test]
fn r1_partialeq_and_diff_conflate_signed_zero_and_nan() {
    let (pos, _) = apply_all(Doc::empty_derived("review_m4_pr1", Tol::witness()), &[point_edit(len(0.0))]);
    let (neg, _) = apply_all(
        Doc::empty_derived("review_m4_pr1", Tol::witness()),
        &[point_edit(len(-0.0))],
    );
    // Bitwise the docs DIFFER…
    let vp = eval::<f64>(
        pos.node(pos.order()[0])
            .unwrap()
            .expr(SlotId::Origin(editor_core::Axis3::X))
            .unwrap(),
        &pos.param_env(),
    )
    .unwrap();
    let vn = eval::<f64>(
        neg.node(neg.order()[0])
            .unwrap()
            .expr(SlotId::Origin(editor_core::Axis3::X))
            .unwrap(),
        &neg.param_env(),
    )
    .unwrap();
    assert_ne!(vp.to_bits(), vn.to_bits(), "bits differ");
    // PartialEq stays IEEE-semantic (documented)…
    assert_eq!(pos, neg, "PartialEq conflates -0.0/0.0 (by design)");
    // …but the fix pass made diff and bit_eq BIT-semantic: the
    // 0.0→-0.0 payload change is DETECTED (diff is the future
    // SetTolerance-audit substrate and must not be bit-blind).
    let d = pos.diff(&neg);
    assert_eq!(
        d.nodes,
        vec![editor_core::NodeChange::Changed(pos.order()[0])],
        "diff detects the signed-zero change"
    );
    assert!(!pos.bit_eq(&neg), "bit_eq distinguishes -0.0/0.0");
    // NaN can no longer enter a document at all (door 1): the
    // conflation hazard for NaN is gone at the source.
    assert_eq!(
        Expr::literal(f64::NAN, Dimension::Length).unwrap_err(),
        editor_core::DimensionError::NonFiniteLiteral
    );
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
    assert_eq!(
        Expr::atan2(ang(1.0), ang(2.0)).unwrap().dim(),
        Dimension::Angle
    );
    assert_eq!(
        Expr::atan2(scl(1.0), scl(2.0)).unwrap().dim(),
        Dimension::Angle
    );
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
    let doc = Doc::empty_derived("review_m4_pr1", Tol::witness())
        .apply(&Edit::SetDocParam {
            name: ParamName::new("q"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 2.0,
            },
        }, Tol::witness())
        .unwrap()
        .doc;
    let res = doc.apply(&point_edit(expr), Tol::witness());
    assert!(
        matches!(res, Err(EditError::DocParamDimensionMismatch { .. })),
        "got {res:?}"
    );
}

/// R2/R6 REGRESSION (fixed) — `eval` of `CountToScalar(i64::MIN)`
/// used to PANIC (`i64::abs` overflow in the old ±2^53 guard). The
/// ruled fix routes promotion through `i32::try_from` +
/// `f64::from(i32)`: total, exact, and i64::MIN is the same typed
/// error as any other out-of-range count — never a panic.
#[test]
fn r2_count_to_scalar_i64_min_is_typed_error_not_panic() {
    let env = ParamEnv::<f64>::default();
    for n in [
        i64::MIN,
        i64::MAX,
        i64::from(i32::MAX) + 1,
        i64::from(i32::MIN) - 1,
    ] {
        let e = Expr::count_to_scalar(Expr::count(n)).unwrap();
        let outcome = std::panic::catch_unwind(|| eval::<f64>(&e, &env));
        let r = outcome.expect("must never panic");
        assert_eq!(
            r,
            Err(editor_core::EvalError::CountToScalarOutOfRange(n)),
            "typed refusal for {n}"
        );
    }
    // Boundary values promote exactly.
    for n in [i64::from(i32::MIN), i64::from(i32::MAX)] {
        let e = Expr::count_to_scalar(Expr::count(n)).unwrap();
        #[allow(clippy::cast_precision_loss)] // |n| ≤ 2^31: exact
        let expected = n as f64;
        assert_eq!(eval::<f64>(&e, &env).unwrap(), expected);
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
    let a = Doc::empty_derived("review_m4_pr1", Tol::witness()).apply(&ins, Tol::witness()).unwrap();
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
        }, Tol::witness())
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
        }, Tol::witness())
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
    let a = Doc::empty_derived("review_m4_pr1", Tol::witness()).apply(&ins, Tol::witness()).unwrap();
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
        }, Tol::witness())
        .unwrap()
        .doc;
    check(&d);
    // (c) SetParam a DIFFERENT slot on the same node.
    let d = d
        .apply(&Edit::SetParam {
            node: id,
            slot: SlotId::Origin(Axis3::Z),
            expr: len(6.0),
        }, Tol::witness())
        .unwrap()
        .doc;
    check(&d);
}

/// R4 (RULED, spec D3 carve-out) — `StableName.node` is a REFERENCE,
/// not a DAG edge (`Declare::inputs()` stays empty), so:
/// (1) DeleteNode of a node referenced ONLY by a Declare's pairs is
///     ACCEPTED → the Declare strands (N5 dangling semantics: loud
///     `NodeGone` at resolution, `Rebind` repairs — documented on
///     `Node::Declare`);
/// (2) InsertNode of a Declare naming a node that does not EXIST at
///     edit time is a TYPED REFUSAL (a never-existed id is a typo,
///     caught at the best-diagnostics door).
#[test]
fn r4_stablename_node_refs_escape_ref_validation() {
    use editor_core::{EntityKind, Node, StableName};
    let (doc, ids) = apply_all(
        Doc::empty_derived("review_m4_pr1", Tol::witness()),
        &[point_edit(len(1.0))], // the node the name will denote
    );
    let target = ids[0];
    let declare = |node| Edit::InsertNode {
        node: Node::declare_rest(vec![(
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
        )]),
    };
    let a = doc.apply(&declare(target), Tol::witness()).unwrap();
    let declare_id = a.record.minted.unwrap();
    // (1) Delete the named node — ACCEPTED despite the live Declare.
    let after = a.doc.apply(&Edit::DeleteNode { id: target }, Tol::witness());
    let after = after.expect("WITNESS: delete of name-referenced node accepted");
    assert!(after.doc.node(target).is_none());
    // The Declare survives, holding a stale id.
    match after.doc.node(declare_id).unwrap() {
        Node::Declare { pairs } => {
            assert_eq!(pairs[0].0.0.node, target, "stale RecipeNodeId held");
        }
        n => panic!("expected Declare, got {n:?}"),
    }
    // (2) Insert a Declare naming an id that never existed: REFUSED
    // (fix pass, ruled carve-out).
    let phantom = RecipeNodeId(9999);
    let res = Doc::empty_derived("review_m4_pr1", Tol::witness()).apply(&declare(phantom), Tol::witness());
    match res {
        Err(EditError::DeclareNamesMissingNode { name }) => {
            assert_eq!(name.node, phantom, "refusal names the typo'd id");
        }
        other => panic!("phantom StableName.node must be refused, got {other:?}"),
    }
    // Contrast: a DAG-edge ref to the same phantom is refused.
    let res2 = Doc::empty_derived("review_m4_pr1", Tol::witness()).apply(&Edit::InsertNode {
        node: Node::Extrude {
            profile: phantom,
            distance: len(1.0),
        },
    }, Tol::witness());
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
        Doc::empty_derived("review_m4_pr1", Tol::witness()),
        &[Edit::InsertNode {
            node: Node::Profile(Fake("p")),
        }],
    );
    let a = doc
        .apply(&Edit::InsertNode {
            node: Node::Extrude {
                profile: ids[0],
                distance: len(1.0),
            },
        }, Tol::witness())
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
    }, Tol::witness());
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
    let doc = Doc::empty_derived("review_m4_pr1", Tol::witness())
        .apply(&Edit::SetDocParam {
            name: name.clone(),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.5,
            },
        }, Tol::witness())
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
    }, Tol::witness());
    assert!(
        matches!(flip, Err(EditError::DocParamDimensionMismatch { .. })),
        "got {flip:?}"
    );
    // Kind flip Continuous→Count under a reference: also refused.
    let kind_flip = doc.apply(&Edit::SetDocParam {
        name: name.clone(),
        value: DocParam::Count { value: 2 },
    }, Tol::witness());
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
        }, Tol::witness())
        .unwrap();
    assert!(!ok.record.structural);
    // An UNREFERENCED param may flip freely (nothing to strand).
    let free = ok
        .doc
        .apply(&Edit::SetDocParam {
            name: ParamName::new("unused"),
            value: DocParam::Count { value: 1 },
        }, Tol::witness())
        .unwrap();
    assert!(
        free.record.structural,
        "Count doc-param set flags structural"
    );
}

/// R5 — `apply` purity, bit-checked: the input doc is bitwise
/// untouched by successful AND failing applies; repeated application
/// of the same edit is deterministic (results bit-identical).
#[test]
fn r5_apply_pure_and_deterministic_bitwise() {
    let (doc, ids) = apply_all(
        Doc::empty_derived("review_m4_pr1", Tol::witness()),
        &[point_edit(len(-0.0)), point_edit(len(5e-324))],
    );
    let snapshot = doc.clone();
    // Successful apply: input untouched.
    let e = Edit::SetParam {
        node: ids[0],
        slot: SlotId::Origin(editor_core::Axis3::Y),
        expr: len(f64::from_bits(0x3FF0000000000001)),
    };
    let a1 = doc.apply(&e, Tol::witness()).unwrap();
    assert_bit_identical(&doc, &snapshot);
    // Failing apply: input untouched (delete of a referenced node —
    // build an extrude on a profile to get a refusal).
    let bad = doc.apply(&Edit::DeleteNode {
        id: RecipeNodeId(424_242),
    }, Tol::witness());
    assert!(bad.is_err());
    assert_bit_identical(&doc, &snapshot);
    // Determinism: same edit, same input → bit-identical outputs,
    // including the minted-id stream.
    let a2 = doc.apply(&e, Tol::witness()).unwrap();
    assert_bit_identical(&a1.doc, &a2.doc);
    assert_eq!(a1.record, a2.record);
    // eval purity in (expr, params): repeated evals bit-identical.
    let expr = Expr::div(len(0.1), scl(0.3)).unwrap();
    let env = ParamEnv::<f64>::default();
    let (v1, v2) = (
        eval::<f64>(&expr, &env).unwrap(),
        eval::<f64>(&expr, &env).unwrap(),
    );
    assert_eq!(v1.to_bits(), v2.to_bits());
}

/// R6 (RULED, fixed) — both non-finite doors are CLOSED:
/// door 1: literal(NaN/inf) and SetDocParam(NaN/inf) are typed
///         refusals at construction/edit time;
/// door 2: a non-finite RESULT (inf from 1/0, NaN from 0/0, overflow
///         to inf) is a typed `NonFiniteResult` at the eval boundary
///         — poison flows through values mid-tree per kernel policy,
///         but never OUT of `eval` as Ok.
#[test]
fn r6_nonfinite_doors_closed() {
    use editor_core::{DimensionError, EvalError};
    let env = ParamEnv::<f64>::default();
    // Door 2: pole and indeterminate-form conduits refused.
    assert_eq!(
        eval::<f64>(&Expr::div(len(1.0), scl(0.0)).unwrap(), &env),
        Err(EvalError::NonFiniteResult),
        "1/0"
    );
    assert_eq!(
        eval::<f64>(&Expr::div(len(0.0), scl(0.0)).unwrap(), &env),
        Err(EvalError::NonFiniteResult),
        "0/0"
    );
    // Arithmetic overflow to inf from finite literals: also refused.
    assert_eq!(
        eval::<f64>(&Expr::mul(len(f64::MAX), scl(2.0)).unwrap(), &env),
        Err(EvalError::NonFiniteResult),
        "overflow"
    );
    // Mid-tree poison that CANCELS still refuses at the boundary
    // check only if the FINAL value is non-finite: (1/0) flows into
    // min(inf, 1) = 1 → finite → Ok (poison-flows-through-values).
    let cancelled = Expr::min(Expr::div(len(1.0), scl(0.0)).unwrap(), len(1.0)).unwrap();
    assert_eq!(
        eval::<f64>(&cancelled, &env),
        Ok(1.0),
        "finite final value passes"
    );
    // Door 1: construction and edit-time injection refused, typed.
    assert_eq!(
        Expr::literal(f64::NAN, Dimension::Length).unwrap_err(),
        DimensionError::NonFiniteLiteral
    );
    assert_eq!(
        Expr::literal(f64::NEG_INFINITY, Dimension::Angle).unwrap_err(),
        DimensionError::NonFiniteLiteral
    );
    for poison in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let res = Doc::empty_derived("review_m4_pr1", Tol::witness()).apply(&Edit::SetDocParam {
            name: ParamName::new("poison"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: poison,
            },
        }, Tol::witness());
        assert_eq!(
            res.unwrap_err(),
            EditError::NonFiniteDocParam {
                name: ParamName::new("poison")
            },
            "SetDocParam({poison})"
        );
    }
}

/// R8 — Interval instantiation over a representative expression set:
/// every enclosure must contain the f64 result; the zero-divisor Div
/// must come out POISONED (empty/NaI or decoration-demoted), never a
/// confident finite enclosure. (Source sweep: editor-core contains no
/// `x*x` self-multiplication anywhere — checked by grep, noted in the
/// review report; powi discipline is geom-core's.)
#[cfg(feature = "interval")]
#[test]
fn r8_interval_lane_representative_and_zero_divisor() {
    use editor_core::eval;
    use geom_core::Interval;
    let env_f = ParamEnv::<f64>::default();
    let env_i = ParamEnv::<Interval>::default();
    let cases: Vec<Expr> = vec![
        Expr::add(len(0.1), len(0.2)).unwrap(),
        Expr::mul(scl(3.0), Expr::div(len(1.0), scl(7.0)).unwrap()).unwrap(),
        Expr::sin(ang(std::f64::consts::FRAC_PI_6)).unwrap(),
        Expr::atan2(len(1.0), len(2.0)).unwrap(),
        Expr::min(ang(1.0), Expr::atan2(scl(1.0), scl(1.0)).unwrap()).unwrap(),
        Expr::max(len(-0.0), len(0.0)).unwrap(),
        Expr::mul(Expr::count_to_scalar(Expr::count(21)).unwrap(), len(0.002)).unwrap(),
        Expr::neg(Expr::sub(len(1.0), len(f64::from_bits(0x3FF0000000000001))).unwrap()),
    ];
    for (i, e) in cases.iter().enumerate() {
        let vf = eval::<f64>(e, &env_f).unwrap();
        let vi = eval::<Interval>(e, &env_i).unwrap();
        let (lo, hi, dec) = vi.repr_bits();
        let (lo, hi) = (f64::from_bits(lo), f64::from_bits(hi));
        assert!(dec >= 2, "case {i}: decoration {dec} (poisoned?)");
        assert!(
            lo <= vf && vf <= hi,
            "case {i}: enclosure [{lo:e},{hi:e}] misses f64 {vf:e}"
        );
    }
    // Zero-containing divisor (fix pass): the certified lane's
    // empty/Trv poison is REFUSED at the eval boundary — the same
    // typed door as the f64 lane's inf/NaN, never a confident (or
    // any) enclosure.
    let div0 = Expr::div(len(1.0), scl(0.0)).unwrap();
    assert!(
        matches!(
            eval::<Interval>(&div0, &env_i),
            Err(editor_core::EvalError::NonFiniteResult)
        ),
        "interval 1/[0,0] refused at the boundary"
    );
    // NaN literal can no longer enter ANY lane (door 1).
    assert!(Expr::literal(f64::NAN, Dimension::Length).is_err());
}

/// R4 (deviation 6) — the `structural` flag admits FALSE POSITIVES
/// (Declare insert — pure intent metadata — flags structural) but no
/// FALSE NEGATIVE is constructible: a Count slot expression CANNOT
/// reference a continuous doc param (refused), so no continuous-
/// flagged edit can ever move a structural value. Pinned here; the
/// wider reading is safe in the conservative direction only.
#[test]
fn r4_structural_flag_false_positive_but_no_false_negative() {
    use editor_core::{Node, PatternKind};
    let cnt_param = ParamName::new("n");
    let doc = Doc::empty_derived("review_m4_pr1", Tol::witness())
        .apply(&Edit::SetDocParam {
            name: cnt_param.clone(),
            value: DocParam::Count { value: 4 },
        }, Tol::witness())
        .unwrap()
        .doc;
    let (doc, ids) = apply_all(doc, &[point_edit(len(0.0))]);
    let pattern = |count: Expr| Edit::InsertNode {
        node: Node::Pattern {
            input: ids[0],
            count,
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(0.005),
            },
        },
    };
    // Count slot referencing the Count doc param: accepted.
    let a = doc
        .apply(&pattern(Expr::param(cnt_param.clone(), Dimension::Count)), Tol::witness())
        .unwrap();
    assert!(a.record.structural);
    let pat_id = a.record.minted.unwrap();
    let doc = a.doc;
    // Attempted false negative: make the structural value depend on a
    // CONTINUOUS param. The only promotion is Count→Scalar (wrong
    // direction), and a Length-dim ref in a Count slot is refused at
    // the slot-dimension check — unrepresentable, not just unvalidated.
    let smuggle = Expr::param(ParamName::new("d_len"), Dimension::Length);
    let res = doc.apply(&Edit::SetStructuralParam {
        node: pat_id,
        slot: SlotId::Count,
        expr: smuggle,
    }, Tol::witness());
    assert!(
        matches!(res, Err(EditError::SlotDimensionMismatch { .. })),
        "got {res:?}"
    );
    // Continuous SetDocParam is flagged non-structural AND provably
    // cannot move the pattern count: value before == after.
    let n_before = eval_count(
        doc.node(pat_id).unwrap().expr(SlotId::Count).unwrap(),
        &doc.param_env::<f64>(),
    )
    .unwrap();
    let a2 = doc
        .apply(&Edit::SetDocParam {
            name: ParamName::new("other"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 9.0,
            },
        }, Tol::witness())
        .unwrap();
    assert!(!a2.record.structural);
    let n_after = eval_count(
        a2.doc.node(pat_id).unwrap().expr(SlotId::Count).unwrap(),
        &a2.doc.param_env::<f64>(),
    )
    .unwrap();
    assert_eq!(n_before, n_after);
    // FALSE POSITIVE witness: inserting a Declare (no geometry, no
    // slots, no inputs) is flagged structural under the wide reading.
    let a3 = a2
        .doc
        .apply(&Edit::InsertNode {
            node: Node::declare_rest(vec![]),
        }, Tol::witness())
        .unwrap();
    assert!(a3.record.structural, "Declare insert flags structural");
}

/// BIT-compare two docs: structure via PartialEq fields, floats via
/// to_bits (PartialEq would conflate -0.0/0.0 and reject NaN==NaN).
/// Slot expressions are compared by evaluating literal trees to f64
/// bits (the crate exposes no direct literal accessor).
fn assert_bit_identical(a: &Doc, b: &Doc) {
    // The crate's own bit-semantic comparator must agree with the
    // independent walk below (fix pass: Doc::bit_eq landed).
    assert!(a.bit_eq(b), "Doc::bit_eq");
    assert_eq!(a.order(), b.order(), "order");
    assert_eq!(a.epsilon().to_bits(), b.epsilon().to_bits(), "epsilon");
    assert_eq!(a.metadata(), b.metadata(), "metadata");
    let (pa, pb) = (a.params(), b.params());
    assert_eq!(pa.len(), pb.len(), "param count");
    for (name, p) in pa {
        match (p, pb.get(name).expect("param present")) {
            (DocParam::Continuous { dim, value }, DocParam::Continuous { dim: d2, value: v2 }) => {
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
