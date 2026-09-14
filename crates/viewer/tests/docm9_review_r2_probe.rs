//! DOCM-9 review lane R2 — `viewer::bounds`' probe and
//! `editor_core::range::certified_range` on the same documents (C4),
//! with the SESSION's oracle (`session/probe.rs`: apply the value,
//! evaluate, `Verdict::no_worse_than` the baseline).
#![cfg(feature = "interval")]
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used, clippy::print_stdout)]

use crate::common;

use editor_core::drive::DriveConfig;
use editor_core::range::{RangeField, RangeSeed, RangeSide, certified_range};
use pncad::document::{
    CancelToken, Dimension, Doc, DocEdit, DocParam, DocParamValue, EvalOptions, Evaluation, Expr,
    Node, ParamName, ProfileProgram, RecipeNodeId, SlotId, apply, evaluate,
};
use pncad::geom_core::Tol;
use viewer::bounds::{Bound, Bounds, BoundsProbe, Verdict, probe};

fn tol() -> Tol {
    Tol::witness()
}

fn verdict(doc: &Doc<ProfileProgram>) -> Verdict {
    Verdict::of(&evaluate(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    ))
}

/// The session's oracle for a parameter, verbatim in shape.
fn probe_param(doc: &Doc<ProfileProgram>, p: &str, origin: f64, seed: f64) -> Bounds {
    let baseline = verdict(doc);
    probe(BoundsProbe::new(origin, seed, false), |candidate| {
        let edit = DocEdit::SetDocParamValue {
            name: ParamName::new(p),
            value: DocParamValue::Continuous(candidate),
        };
        match apply(doc, &edit, tol()) {
            Ok(applied) => verdict(&applied.doc).no_worse_than(&baseline),
            Err(_) => false,
        }
    })
}

/// The session's oracle for a slot.
fn probe_slot(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    slot: SlotId,
    origin: f64,
    seed: f64,
) -> Bounds {
    let baseline = verdict(doc);
    probe(BoundsProbe::new(origin, seed, false), |candidate| {
        let Ok(expr) = Expr::literal(candidate, Dimension::Length) else {
            return false;
        };
        let edit = DocEdit::SetParam { node, slot, expr };
        match apply(doc, &edit, tol()) {
            Ok(applied) => verdict(&applied.doc).no_worse_than(&baseline),
            Err(_) => false,
        }
    })
}

fn side(s: &RangeSide) -> String {
    match s {
        RangeSide::Certified { to } => format!("Certified {{ to: {to:e} }}"),
        RangeSide::NewFailure {
            certified_to,
            within,
            ..
        } => format!("NewFailure {{ certified_to: {certified_to:e}, within: {within:?} }}"),
        RangeSide::DecisionFlip {
            certified_to,
            within,
            ..
        } => format!("DecisionFlip {{ certified_to: {certified_to:e}, within: {within:?} }}"),
        RangeSide::Indeterminate {
            certified_to,
            within,
            reason,
        } => format!(
            "Indeterminate {{ certified_to: {certified_to:e}, within: {within:?}, reason: {reason:?} }}"
        ),
    }
}

fn bound(b: Bound) -> String {
    match b {
        Bound::Open { probed } => format!("Open {{ probed: {probed:e} }}"),
        Bound::Edge { valid, invalid } => format!("Edge {{ valid: {valid:e}, invalid: {invalid:e} }}"),
    }
}

/// The slab through zero (the PR's A2/A4 fixture) as a PARAMETER: the
/// probe's bracket against the certified interval.
#[test]
fn r2_probe_versus_certificate_on_the_slab_parameter() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r2-slab", tol());
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 1.0),
        },
        tol(),
    )
    .expect("declares")
    .doc;
    let (doc, profile) = common::framed_square(&doc, 1.0, tol());
    let (doc, _extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::param(ParamName::new("depth"), Dimension::Length),
        },
        tol(),
    );
    for seed in [0.1, 0.001] {
        let b = probe_param(&doc, "depth", 1.0, seed);
        println!(
            "probe seed {seed}: low {} high {} samples {}",
            bound(b.low),
            bound(b.high),
            b.samples
        );
    }
    let b = probe_param(&doc, "depth", 1.0, 0.001);
    let r = certified_range(
        &doc,
        &RangeField::Param(ParamName::new("depth")),
        RangeSeed { lo: -1.05, hi: 0.5 },
        &DriveConfig {
            max_depth: 24,
            max_leaves: 2048,
            ..DriveConfig::default()
        },
        tol(),
    )
    .expect("certifies");
    let (clo, chi) = r.certified_interval();
    println!(
        "certified: lo {} hi {} -> interval [{clo:e}, {chi:e}]",
        side(r.lo()),
        side(r.hi())
    );
    println!(
        "probe limits: [{:e}, {:e}]; certified inside the probe's LIMITS: {}",
        b.low.limit(),
        b.high.limit(),
        clo >= b.low.limit() && chi <= b.high.limit()
    );
    if let Bound::Edge { valid, invalid } = b.low {
        println!(
            "probe low bracket [{invalid:e}, {valid:e}]; certificate's lo end {clo:e} is {} the probe's furthest valid",
            if clo < valid { "BELOW" } else { "at or above" }
        );
    }
}

/// The viewer suite's own real-slot row (`the_session_probes_a_real_slots_range`):
/// an 8 mm extrude with a floor just above zero. What the certificate
/// says for the same slot.
#[test]
fn r2_probe_versus_certificate_on_the_real_slot() {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r2-slot", tol());
    let (doc, profile) = common::framed_square(&doc, 0.04, tol());
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal(0.008, Dimension::Length).expect("a length"),
        },
        tol(),
    );
    let b = probe_slot(&doc, extrude, SlotId::Distance, 0.008, 0.001);
    println!(
        "probe: low {} high {} samples {}",
        bound(b.low),
        bound(b.high),
        b.samples
    );
    let r = certified_range(
        &doc,
        &RangeField::Slot {
            node: extrude,
            slot: SlotId::Distance,
        },
        RangeSeed {
            lo: -0.0081,
            hi: 0.008,
        },
        &DriveConfig {
            max_depth: 24,
            max_leaves: 2048,
            ..DriveConfig::default()
        },
        tol(),
    )
    .expect("certifies");
    let (clo, chi) = r.certified_interval();
    println!(
        "certified: lo {} hi {} -> interval [{clo:e}, {chi:e}]",
        side(r.lo()),
        side(r.hi())
    );
    println!(
        "certified inside the probe's LIMITS: {}",
        clo >= b.low.limit() && chi <= b.high.limit()
    );
    // Is the certificate's lower end, below the probe's furthest valid sample, actually valid at f64?
    for v in [clo, clo * 0.5, 0.0] {
        let expr = Expr::literal(v, Dimension::Length).expect("a length");
        let moved = apply(&doc, &DocEdit::SetParam { node: extrude, slot: SlotId::Distance, expr }, tol())
            .expect("applies")
            .doc;
        let ev: Evaluation<f64> =
            evaluate(&moved, None, &CancelToken::new(), &EvalOptions::default(), tol());
        let failing: Vec<String> = ev
            .order
            .iter()
            .filter_map(|id| ev.node_error(*id).map(|e| format!("{}:{}", id.0, e.kind)))
            .collect();
        println!("f64 at distance {v:e}: failing {failing:?}");
    }
    // The probe reports a FLOOR (an edge). Does the certificate report a bound there?
    println!(
        "probe low is an edge: {}; certificate lo is_bound: {}",
        b.low.is_edge(),
        r.lo().is_bound()
    );
}
