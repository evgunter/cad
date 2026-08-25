//! The check registry and its first resident (DISCIPLINES-DESIGN DS6;
//! LONGTERM-IDEAS I1(0b)): `run_checks` / `enforce_checks` over real
//! evaluated documents.
//!
//! Claims pinned here:
//! - a deliberately disjoint union is ONE finding
//!   (`actual 2, expected 1`), and stating `expected = 2` in the
//!   config clears it — the resident's acknowledgment mechanism;
//! - an interior void is NOT a component (`A ∖ B`, `B` strictly
//!   inside: no finding at the default expectation) — the load-bearing
//!   void case;
//! - severity changes only what is ACCEPTED: `Warn` and `Error`
//!   produce the identical report, `Error` refuses at
//!   `enforce_checks` (the ONLY refusing path), `Off` is VISIBLY
//!   skipped;
//! - the report is deterministic (D9): two runs, identical reports.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::collections::BTreeMap;

use editor_core::{
    BooleanOp, CancelToken, CheckEvidence, CheckFinding, CheckId, CheckKind, ChecksConfig,
    ChecksReport, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, Severity,
    enforce_checks, run_checks,
};
use fixture::{desc, insert, len, square};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// An extruded square: half-width `h` centered at `(cx, 0)` on the
/// z = `z0` plane, extruded `dz` up.
fn slab(doc: ProfileDoc, cx: f64, h: f64, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![square(cx, 0.0, h)],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(dz),
        },
    )
}

/// Two disjoint unit cubes, deliberately united: one root, one body,
/// two components.
fn disjoint_union() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-disjoint", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 0.0, 1.0);
    let (doc, b) = slab(doc, 3.0, 0.5, 0.0, 1.0);
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    )
}

/// `A ∖ B` with `B` strictly inside `A`: the void birth — one
/// component, two shells.
fn voided() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-voided", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 1.5, 0.0, 3.0);
    let (doc, b) = slab(doc, 0.0, 0.5, 1.0, 1.0);
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b,
            declare: None,
        },
    )
}

fn checks(doc: &ProfileDoc, cfg: &ChecksConfig) -> ChecksReport {
    let ev = run(doc);
    run_checks(doc, &ev, cfg, Tol::witness()).expect("checks run over a completed evaluation")
}

#[test]
fn disjoint_union_is_one_finding() {
    let (doc, root) = disjoint_union();
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.skipped, Vec::<CheckId>::new());
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            root,
            output_ix: 0,
            evidence: CheckEvidence::Connectedness {
                actual: 2,
                expected: 1,
            },
        }]
    );
    // The finding tells one story with its recourse (the Display
    // contract): the count, the expectation, and the acknowledgment
    // mechanism by name.
    let rendered = report.findings[0].to_string();
    assert!(rendered.contains("check connectedness"), "{rendered}");
    assert!(
        rendered.contains("2 disconnected component(s) where 1 was expected"),
        "{rendered}"
    );
    assert!(rendered.contains("expected_components"), "{rendered}");
}

#[test]
fn stated_expectation_clears_the_finding() {
    let (doc, root) = disjoint_union();
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((root, 0), 2)]),
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(report.findings, vec![]);
    assert_eq!(report.skipped, vec![]);
}

#[test]
fn interior_void_is_not_a_component() {
    let (doc, _root) = voided();
    let report = checks(&doc, &ChecksConfig::default());
    // One Outer shell + one Void shell = ONE component: clean at the
    // default expectation, at this and every ε row (the count is
    // exact; only the per-shell orientation read is decided, and both
    // shells here are decisively signed).
    assert_eq!(report.findings, vec![]);
}

#[test]
fn severity_error_refuses_at_enforce_only() {
    let (doc, _root) = disjoint_union();
    let warn = ChecksConfig::default();
    let error = ChecksConfig {
        connectedness: Severity::Error,
        ..ChecksConfig::default()
    };
    // The severity knob changes NOTHING about what is found (DS3):
    // identical reports at Warn and Error.
    let report_warn = checks(&doc, &warn);
    let report_error = checks(&doc, &error);
    assert_eq!(report_warn, report_error);
    // Warn passes enforcement; Error refuses with the findings.
    assert!(enforce_checks(&report_warn, &warn).is_ok());
    let refusal = enforce_checks(&report_error, &error).unwrap_err();
    assert_eq!(refusal.findings, report_error.findings);
    assert!(
        refusal
            .to_string()
            .contains("1 check finding(s) at Error severity"),
        "{refusal}"
    );
}

#[test]
fn off_is_visibly_skipped() {
    let (doc, _root) = disjoint_union();
    let off = ChecksConfig {
        connectedness: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &off);
    assert_eq!(report.findings, vec![]);
    // "Not checked" is an answer the report carries, distinct from
    // "checked and fine".
    assert_eq!(report.skipped, vec![CheckId::Connectedness]);
    assert!(report.to_string().contains("skipped"), "{report}");
    // Off never reaches enforcement: nothing was found, nothing
    // refuses.
    assert!(enforce_checks(&report, &off).is_ok());
}

#[test]
fn reports_are_deterministic() {
    let (doc, _root) = disjoint_union();
    let a = checks(&doc, &ChecksConfig::default());
    let b = checks(&doc, &ChecksConfig::default());
    assert_eq!(a, b);
}

#[test]
fn connectedness_is_labeled_certified() {
    // The honesty label (DS6): the count is a theorem, and the label
    // says so — a heuristic resident must never be dressed as this.
    assert_eq!(CheckId::Connectedness.kind(), CheckKind::Certified);
}
