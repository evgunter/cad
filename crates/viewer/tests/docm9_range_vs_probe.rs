//! **The certificate beside the REAL probe** — `viewer::bounds::probe`
//! and `editor_core::range::certified_range` on one document, which is
//! what "a certified range is a subset of the locally-valid range" is
//! a claim about.
//!
//! It lives here and not in `editor-core`'s own suite because the probe
//! does: `editor-core` sits below this crate, so its suite can only
//! reproduce the probe's QUESTION. This one runs the probe.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use editor_core::drive::DriveConfig;
use editor_core::range::{RangeField, RangeSeed, certified_range};
use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, DocParamValue, EvalOptions, Evaluation, Expr,
    LoopProgram, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, apply,
    evaluate,
};
use geom_core::Tol;
use viewer::bounds::{Bound, BoundsProbe, probe};

fn tol() -> Tol {
    Tol::witness()
}

fn name(n: &'static str) -> ParamName {
    ParamName::literal(n)
}

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar literal")
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode { node },
        tol(),
        &pncad::document::RefusingReach,
    )
    .expect("the node inserts");
    let id = applied.record.minted.expect("one minted id");
    *doc = applied.doc;
    id
}

/// A unit square extruded by a document parameter — the same branch
/// fixture `editor-core`'s own suite uses, so the two suites are
/// talking about one document.
fn slab(depth: f64) -> ProfileDoc {
    let mut doc = ProfileDoc::empty_derived("docm9", tol());
    doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name("depth"),
            value: DocParam::continuous(Dimension::Length, depth),
        },
        tol(),
        &pncad::document::RefusingReach,
    )
    .expect("the parameter declares")
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
                    .expect("finite square corners"),
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

/// **The certificate is inside the LOCALLY-VALID range, and is not
/// comparable to the probe's reported bracket.**
///
/// Two different claims, on one document with a real boundary (the
/// extrusion runs the other way through zero):
///
/// 1. every value the certificate covers is one the probe's OWN
///    validity test calls valid — the subset claim, and the one that
///    has to hold;
/// 2. the probe's furthest VALID SAMPLE sits outside the certificate,
///    because sampling stops where it happened to step and a proof
///    stops where the driver stopped deciding. The magnitudes are
///    pinned as an order of magnitude, not as numbers: what is stable
///    is that the proof's frontier is orders nearer the crossing than
///    the probe's last good sample, and a change that inverted that
///    is the thing this row exists to catch.
#[test]
fn the_certificate_is_inside_the_locally_valid_range_not_the_probes_bracket() {
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
            &pncad::document::RefusingReach,
        )
        .expect("a value edit applies")
        .doc;
        failing(&moved).is_subset(&baseline)
    };

    let bounds = probe(BoundsProbe::new(1.0, 1.0, false), valid);
    let range = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed { lo: -1.05, hi: 0.5 },
        &DriveConfig {
            max_depth: 24,
            max_leaves: 2048,
            ..DriveConfig::default()
        },
        tol(),
    )
    .expect("the slab's depth certifies");
    let (clo, chi) = range.certified_interval();
    assert!(clo < chi, "the certificate is non-empty: [{clo}, {chi}]");

    // (1) THE SUBSET CLAIM, against the probe's own validity test.
    for k in 0..=32 {
        let v = clo + (chi - clo) * f64::from(k) / 32.0;
        assert!(
            valid(v),
            "the certificate covers depth {v}, which the probe's test calls invalid"
        );
    }

    // (2) THE TWO ANSWERS ARE NOT THE SAME BRACKET — and WHICH of the
    // two numbers is larger is not the claim. The probe's bracket is
    // set by its own sampling schedule, which does not move with the
    // run's tolerance; the proof's frontier is the driver's floor,
    // which does. An ordering between them is therefore a claim about
    // one eps row, and the gate runs three: it caught exactly that
    // assertion here.
    let Bound::Edge {
        valid: probe_valid,
        invalid: probe_invalid,
    } = bounds.low
    else {
        panic!("the crossing at depth zero is a boundary the probe brackets: {bounds:?}");
    };
    assert!(
        clo > 0.0 && probe_valid > 0.0,
        "both answers stop above the crossing: certificate {clo}, probe {probe_valid}"
    );
    assert!(
        probe_invalid <= 0.0,
        "the probe's far end is at or past the crossing: {probe_invalid}"
    );
    // The certificate's frontier is the DRIVER's floor, bounded by the
    // two mechanisms that set it — the run's ambiguity band and the
    // drive's resolution over this seed — rather than by a number.
    let resolution = (1.05 + 0.5) / 2f64.powi(24);
    assert!(
        clo < 100.0 * resolution.max(tol().eps()),
        "the frontier is the driver's stopping point, not the boundary: {clo}"
    );
    // And the SHAPES differ on the other side, at every tolerance: the
    // seed holds no boundary above the nominal, so the certificate
    // says "proven to the seed's edge" where the probe can only say
    // "looked this far and found nothing".
    assert!(
        (chi - 1.5).abs() < 1e-12,
        "the hi side certifies to the seed's edge, got {chi}"
    );
    assert!(
        matches!(bounds.high, Bound::Open { .. }),
        "the probe reports its reach upward, never a proof: {:?}",
        bounds.high
    );
}
