//! Review probe for PR 5772: a forged glue from a sibling crate. Every
//! line below must fail to compile on `flush/nary-union-door`; it is not
//! in any test harness (add `#[path = "review_forgery/forge.rs"] mod
//! forge;` to `crates/editor-core/tests/all.rs` to check).
#![allow(dead_code, unused_imports, missing_docs)]

use topo::{BooleanInputs, GlueVerdict, Operand};

// 1. The type of a given pair, by name, at the crate root and in `boolean`.
fn a(_: topo::GluedPair<f64>) {}
fn b(_: topo::boolean::GluedPair<f64>) {}

// 2. The mint, from a kernel verdict.
fn c(v: &GlueVerdict<f64>) {
    let _ = v.given_at(Operand::A, v.a());
}

// 3. The field, through the pair door and the pair verb.
fn d() -> BooleanInputs {
    BooleanInputs {
        given: None,
        ..BooleanInputs::default()
    }
}

// 4. The crate-private door with given pairs.
fn e(x: &topo::AtRestBody<f64>, y: &topo::AtRestBody<f64>) {
    let _ = topo::boolean_op_given(
        topo::BooleanOp::Union,
        x,
        y,
        &BooleanInputs::default(),
        None,
        topo::SweepStrategy::Realized,
        geom_core::Tol::witness(),
    );
}

// 5. The pair verb's payload.
fn f() -> verbs::Boolean {
    verbs::Boolean {
        op: topo::BooleanOp::Union,
        inputs: Box::new(topo::BooleanInputs {
            given: None,
            ..Default::default()
        }),
    }
}
