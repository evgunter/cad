//! **The checks stop** (DISCIPLINES-DESIGN DS6; LONGTERM-IDEAS
//! I1(0b)): the advisory-check registry driven the way a user would —
//! author a recipe, evaluate it, ask `run_checks` for a report, read
//! the report.
//!
//! Three documents, chosen to say what the connectedness check IS:
//!
//! - a deliberate two-cube DISJOINT union — the finding fires
//!   (`actual 2, expected 1`): a stray component usually means a
//!   boolean that did not reach its operand;
//! - the same document with the disjointness ACKNOWLEDGED
//!   (`expected_components = 2`) — clean: expected disconnection is
//!   data, not a warning to live with;
//! - a subtract whose tool is strictly interior (the void birth) —
//!   clean: a CAVITY is boundary, not a component, so a voided body
//!   counts 1 at the default expectation.
//!
//! Narration-only (no render stop): the subject is a REPORT, and its
//! `Display` is the picture. Every door used here is `pncad::…` (the
//! tour's standing invariant).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use pncad::document::{
    BooleanOp, CancelToken, ChecksConfig, Dimension, DocEdit, DocumentId, EvalOptions, Evaluation,
    Expr, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, Severity, apply,
    enforce_checks, evaluate, run_checks,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::profile::SketchPlane;

/// Inserts a node and returns its minted id.
fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

/// An extruded square: half-width `h` centered at `(cx, 0)` on the
/// z = `z0` sketch plane, extruded `dz` up.
fn slab(doc: &mut ProfileDoc, cx: f64, h: f64, z0: f64, dz: f64, tol: Tol) -> RecipeNodeId {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, z0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let corners = vec![(cx - h, -h), (cx + h, -h), (cx + h, h), (cx - h, h)];
    let profile = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::polygon(corners).expect("finite corners")],
        }),
        tol,
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            // **Authored CANONICALLY, deliberately** — this scene and
            // `heatsink` are the default half of the units exhibit,
            // against `ring` (millimetres and half-turns) and
            // `diefillet` (millimetres and degrees). `Expr::literal`
            // stores the canonical row rather than nothing, so the
            // panel opens on `m` because the document SAYS `m`, not
            // because a reader had to pick a fallback.
            distance: Expr::literal(dz, Dimension::Length).unwrap(),
        },
        tol,
    )
}

/// Builds and evaluates a one-boolean document; returns the document,
/// the boolean root, and the evaluation.
fn boolean_doc(
    label: &str,
    op: BooleanOp,
    a: (f64, f64, f64, f64),
    b: (f64, f64, f64, f64),
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, Evaluation<f64>) {
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let a = slab(&mut doc, a.0, a.1, a.2, a.3, tol);
    let b = slab(&mut doc, b.0, b.1, b.2, b.3, tol);
    let root = insert(
        &mut doc,
        Node::Boolean {
            op,
            a,
            b,
            declare: None,
        },
        tol,
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    (doc, root, ev)
}

/// This scene's recipe, as a document the GUI can open: the two-cube
/// DISJOINT union the narration checks first.
///
/// The same `boolean_doc` the narration walks — a reader who opens
/// this in the viewer is looking at the document the connectedness
/// finding was raised against.
pub fn gallery_document(tol: Tol) -> ProfileDoc {
    boolean_doc(
        "pncad-demo-checks-disjoint",
        BooleanOp::Union,
        (0.0, 0.5, 0.0, 1.0),
        (3.0, 0.5, 0.0, 1.0),
        tol,
    )
    .0
}

pub fn narration(tol: Tol) {
    // (a) Two unit cubes three units apart, deliberately united into
    // one body: the finding fires at the default expectation.
    let (doc, root, ev) = boolean_doc(
        "pncad-demo-checks-disjoint",
        BooleanOp::Union,
        (0.0, 0.5, 0.0, 1.0),
        (3.0, 0.5, 0.0, 1.0),
        tol,
    );
    let cfg = ChecksConfig::default();
    let report = run_checks(&doc, &ev, &cfg, tol).expect("checks run");
    println!("   a two-cube DISJOINT union, checked at the default expectation:");
    println!("   {}", report);
    assert_eq!(report.findings.len(), 1);

    // The severity knob changes only what is ACCEPTED, and only at
    // the enforcement door the CALLER places: the same report refuses
    // at Error, passes at the default Warn.
    let strict = ChecksConfig {
        connectedness: Severity::Error,
        ..ChecksConfig::default()
    };
    assert!(enforce_checks(&report, &cfg).is_ok());
    match enforce_checks(&report, &strict) {
        Ok(()) => panic!("Error severity refuses at enforce_checks"),
        Err(refusal) => println!("   at Severity::Error, enforce_checks refuses: {refusal}"),
    }

    // (b) The same document with the disjointness stated as data.
    let acknowledged = ChecksConfig {
        expected_components: BTreeMap::from([((root, 0), 2)]),
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &acknowledged, tol).expect("checks run");
    println!("   the same document, disjointness ACKNOWLEDGED (expected_components = 2):");
    println!("   {}", report);
    assert!(report.findings.is_empty());

    // (c) The void birth: A ∖ B with B strictly inside. Two shells,
    // ONE component — a cavity is boundary, not a component.
    let (doc, _root, ev) = boolean_doc(
        "pncad-demo-checks-voided",
        BooleanOp::Subtract,
        (0.0, 1.5, 0.0, 3.0),
        (0.0, 0.5, 1.0, 1.0),
        tol,
    );
    let report = run_checks(&doc, &ev, &ChecksConfig::default(), tol).expect("checks run");
    println!("   a subtract with the tool strictly interior (outer shell + void shell):");
    println!("   {}", report);
    assert!(report.findings.is_empty());
}
