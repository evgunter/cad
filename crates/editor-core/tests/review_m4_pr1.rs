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
