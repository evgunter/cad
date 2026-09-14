//! R1 review probe: the REAL sampling probe (`viewer::bounds::probe`)
//! run beside `editor_core::range::certified_range` on one document,
//! which is what C4 is a claim about. The unit's own row reproduces the
//! probe's QUESTION inside `editor-core`; this one runs the probe.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use editor_core::range::{RangeField, RangeSeed, RangeSide, certified_range};
use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, DocParamValue, EvalOptions, Evaluation, Expr,
    LoopProgram, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, apply,
    evaluate,
};
use editor_core::drive::DriveConfig;
use geom_core::Tol;
use viewer::bounds::{Bound, BoundsProbe, probe};

fn tol() -> Tol {
    Tol::witness()
}
fn name(n: &str) -> ParamName {
    ParamName::new(n)
}
fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite")
}
fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}

fn slab(depth: f64) -> ProfileDoc {
    let mut doc = ProfileDoc::empty_derived("r1", tol());
    doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name("depth"),
            value: DocParam::continuous(Dimension::Length, depth),
        },
        tol(),
    )
    .expect("declare")
    .doc;
    let f = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [lit(0.0), lit(0.0), lit(0.0)],
            u: [scalar(1.0), scalar(0.0), scalar(0.0)],
            v: [scalar(0.0), scalar(1.0), scalar(0.0)],
        }),
    );
    let p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: f,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                    .expect("finite"),
            ],
        }),
    );
    insert(
        &mut doc,
        Node::Extrude {
            profile: p,
            distance: Expr::param(name("depth"), Dimension::Length),
        },
    );
    doc
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol()).expect("insert");
    let id = applied.record.minted.expect("one minted id");
    *doc = applied.doc;
    id
}

fn failing(doc: &ProfileDoc) -> BTreeSet<RecipeNodeId> {
    let ev: Evaluation<f64> = evaluate(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    );
    ev.nodes
        .iter()
        .filter(|(_, r)| matches!(r, NodeResult::Failed(_)))
        .map(|(id, _)| *id)
        .collect()
}

#[test]
fn r1_the_real_probe_beside_the_certificate() {
    let doc = slab(1.0);
    let baseline = failing(&doc);
    let valid = |v: f64| {
        let moved = apply(
            &doc,
            &DocEdit::SetDocParamValue {
                name: name("depth"),
                value: DocParamValue::Continuous(v),
            },
            tol(),
        )
        .expect("edit")
        .doc;
        failing(&moved).is_subset(&baseline)
    };
    let b = probe(BoundsProbe::new(1.0, 1.0, false), valid);
    println!("REAL PROBE: origin={} low={:?} high={:?} samples={}", b.origin, b.low, b.high, b.samples);

    let cr = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed { lo: -2.0, hi: 0.5 },
        &DriveConfig {
            max_depth: 24,
            max_leaves: 2048,
            ..DriveConfig::default()
        },
        tol(),
    )
    .expect("certifies");
    println!(
        "CERTIFICATE: interval={:?} lo_is_bound={} hi_is_bound={}",
        cr.certified_interval(),
        cr.lo().is_bound(),
        cr.hi().is_bound()
    );
    if let RangeSide::DecisionFlip { within, .. } | RangeSide::NewFailure { within, .. } = cr.lo() {
        println!(
            "lo within (absolute) = ({}, {})",
            cr.absolute(within.0),
            cr.absolute(within.1)
        );
    }
    // The subset claim against the REAL probe's own bracket.
    let (clo, chi) = cr.certified_interval();
    match b.low {
        Bound::Edge { valid: v, invalid } => {
            println!("probe low edge: valid={v} invalid={invalid}");
            assert!(clo >= invalid, "certificate {clo} reaches past the probe's nearest invalid {invalid}");
        }
        Bound::Open { probed } => println!("probe low open at {probed}"),
    }
    match b.high {
        Bound::Edge { valid: v, invalid } => {
            println!("probe high edge: valid={v} invalid={invalid}");
            assert!(chi <= invalid);
        }
        Bound::Open { probed } => println!("probe high open at {probed}"),
    }
}
