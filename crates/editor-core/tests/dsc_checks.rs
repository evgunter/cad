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
    enforce_checks, run_checks, subject_body,
};
use fixture::{desc, insert, len, square};
use geom_core::Tol;
use topo::ShellClassifyError;

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

/// An annihilated boolean: the intersection of two DISJOINT slabs —
/// an honest ∅ result (F8: `Empty` is a typed success) denoting zero
/// subjects.
fn annihilated() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-annihilated", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 0.0, 1.0);
    let (doc, b) = slab(doc, 3.0, 0.5, 0.0, 1.0);
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Intersect,
            a,
            b,
            declare: None,
        },
    )
}

#[test]
fn stale_expectation_on_a_vanished_body() {
    // Expectations are two-directional: the entry names a root whose
    // boolean annihilated, so NO subject consumes it — the entry is
    // stale, and staleness is a finding, never a silent ignore.
    let (doc, root) = annihilated();
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((root, 0), 1)]),
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            root,
            output_ix: 0,
            evidence: CheckEvidence::StaleExpectation { expected: 1 },
        }]
    );
    let rendered = report.findings[0].to_string();
    assert!(rendered.contains("has no subject"), "{rendered}");
    assert!(rendered.contains("expected_components"), "{rendered}");
}

#[test]
fn annihilation_without_a_stated_expectation_is_clean() {
    // The DEFAULT expectation binds only existing subjects: a
    // legitimate ∅ result with nothing stated about it stays clean.
    let (doc, _root) = annihilated();
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.findings, vec![]);
}

#[test]
fn stale_expectation_on_a_nonexistent_root() {
    // The key names no root output at all (wrong id): same staleness,
    // attributed at the entry's own key.
    let (doc, root) = disjoint_union();
    let ghost = RecipeNodeId(root.0 + 999);
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((root, 0), 2), ((ghost, 0), 1)]),
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            root: ghost,
            output_ix: 0,
            evidence: CheckEvidence::StaleExpectation { expected: 1 },
        }]
    );
}

/// The escalation row (the review's P9 fixture, promoted): a slab
/// whose `V/A` sits INSIDE the ambiguity band — built relative to the
/// run's ε so the row holds at every ε row. `V/A ≈ dz/2 = 5ε` for a
/// unit-square slab of thickness `10ε` (band `(ε, Kε)`, K = 10).
#[test]
fn in_band_shell_escalates_typed_never_guessed() {
    let tol = Tol::witness();
    let dz = 10.0 * tol.eps();
    let doc = ProfileDoc::empty_derived("dsc-checks-thin", Tol::witness());
    let (doc, root) = slab(doc, 0.0, 0.5, 0.0, dz);
    let report = checks(&doc, &ChecksConfig::default());
    // Exactly one finding: the typed escalation. NEVER a counted
    // verdict — an in-band orientation is not guessed to a side (F6).
    assert_eq!(report.findings.len(), 1, "{report}");
    let finding = &report.findings[0];
    assert_eq!((finding.root, finding.output_ix), (root, 0));
    let CheckEvidence::Escalated {
        source: ShellClassifyError::Escalated { source, .. },
    } = &finding.evidence
    else {
        panic!("expected the typed in-band escalation, got: {finding}");
    };
    // The escalation names the funnel site it came from.
    assert_eq!(source.predicate, Some("chk_shell_volume_sign"));
    // The rendered story names the margin data and the check's own
    // recourse — not the funnel's generic declare-the-coincidence
    // menu, and no kernel arena key.
    let rendered = finding.to_string();
    assert!(rendered.contains("chk_shell_volume_sign"), "{rendered}");
    assert!(rendered.contains("thicken or remove"), "{rendered}");
    assert!(!rendered.contains("declare"), "{rendered}");
    assert!(!rendered.contains("ShellKey"), "{rendered}");
}

#[test]
fn a_findings_attribution_resolves_to_its_subject() {
    // The door from a finding back to the flagged body: the same
    // enumeration run_checks walks.
    let (doc, _root) = disjoint_union();
    let ev = run(&doc);
    let report =
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).expect("checks run");
    let finding = &report.findings[0];
    let body = subject_body(&ev, finding.root, finding.output_ix)
        .expect("the attribution resolves against the evaluation it came from");
    // The flagged body IS the disjoint union: the two shells the
    // finding counted (their grouping into solids is the kernel's
    // business, not pinned here).
    assert_eq!(body.shells().count(), 2);
    // An attribution with no subject (a stale expectation's shape)
    // resolves to None, not to a wrong body.
    assert!(subject_body(&ev, finding.root, 7).is_none());
}

// ---------------------------------------------------------------------
// The separation resident
// ---------------------------------------------------------------------
//
// Claims pinned below:
// - two roots occupying the same space are ONE finding naming BOTH, and
//   the same two roots moved apart are clean — the resident's whole
//   subject, and the diefillet gallery bug that motivated it;
// - roots that merely TOUCH are reported too: the certificate is
//   sufficient, not necessary, and the finding says "not certifiably
//   disjoint", never "these overlap";
// - one root's own multi-solid body is NOT this resident's subject
//   (the gather did not put those solids together);
// - `Off` is visibly skipped, independently of the other resident;
// - `Error` refuses at `enforce_checks` and nowhere else;
// - the report is deterministic across runs.

/// Two slabs as two SEPARATE product roots (no boolean joining them),
/// `b` centered at `cx`. The gather lists both as sinks, which is
/// exactly the shape a recipe grows when a feature is left dangling.
fn two_roots(cx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-two-roots", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 0.0, 1.0);
    let (doc, b) = slab(doc, cx, 0.5, 0.0, 1.0);
    (doc, a, b)
}

#[test]
fn overlapping_roots_are_one_finding_naming_both() {
    // Concentric: the two roots are the same cube twice over, which is
    // the die's blank-over-composed shape in miniature.
    let (doc, a, b) = two_roots(0.0);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.skipped, Vec::<CheckId>::new());
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Separation,
            root: a,
            output_ix: 0,
            evidence: CheckEvidence::NotSeparated {
                other_root: b,
                other_output: 0,
            },
        }]
    );
    // The finding names both roots in its own sentence, so a reader
    // never has to consult the attribution separately to know what it
    // is about.
    let rendered = report.findings[0].to_string();
    assert!(rendered.contains(&format!("root {}", a.0)), "{rendered}");
    assert!(rendered.contains(&format!("root {}", b.0)), "{rendered}");
    // And it denies the CERTIFICATE — it never claims the two overlap,
    // which the boxes do not decide.
    assert!(rendered.contains("not certifiably disjoint"), "{rendered}");
}

#[test]
fn roots_moved_apart_are_clean() {
    // Same two roots, far enough apart that the padded face boxes
    // cannot meet: the certificate is granted and nothing is reported.
    let (doc, _, _) = two_roots(3.0);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.skipped, Vec::<CheckId>::new());
}

#[test]
fn touching_roots_are_reported_as_uncertified_not_as_overlapping() {
    // Face-to-face at x = 0.5: disjoint interiors, shared boundary.
    // The box rule cannot separate them and says so — the
    // sufficient-not-necessary contract, visible.
    let (doc, a, b) = two_roots(1.0);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Separation,
            root: a,
            output_ix: 0,
            evidence: CheckEvidence::NotSeparated {
                other_root: b,
                other_output: 0,
            },
        }]
    );
}

#[test]
fn one_roots_own_disjoint_body_is_not_this_residents_subject() {
    // `disjoint_union` is ONE root carrying two solids. The gather did
    // not put them together, so the separation resident says nothing
    // about them — only the connectedness resident does.
    let (doc, _) = disjoint_union();
    let report = checks(&doc, &ChecksConfig::default());
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check == CheckId::Connectedness),
        "{report}"
    );
}

#[test]
fn separation_off_is_visibly_skipped_and_independent() {
    let (doc, _, _) = two_roots(0.0);
    let cfg = ChecksConfig {
        separation: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(report.skipped, vec![CheckId::Separation]);
    assert_eq!(report.findings, Vec::new());

    // The other direction: turning connectedness off leaves the
    // separation resident running. Before this resident existed the
    // dispatch returned early on a single `Off`, which would have
    // silently taken the second one with it.
    let cfg = ChecksConfig {
        connectedness: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(report.skipped, vec![CheckId::Connectedness]);
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].check, CheckId::Separation);
}

#[test]
fn separation_severity_changes_only_what_is_accepted() {
    let (doc, _, _) = two_roots(0.0);
    let warn = ChecksConfig::default();
    let error = ChecksConfig {
        separation: Severity::Error,
        ..ChecksConfig::default()
    };
    let (a, b) = (checks(&doc, &warn), checks(&doc, &error));
    // Identical reports: severity is a position on what is ACCEPTED,
    // never on what is found.
    assert_eq!(a, b);
    assert!(enforce_checks(&a, &warn).is_ok());
    assert!(enforce_checks(&b, &error).is_err());
}

#[test]
fn separation_findings_are_certified_and_deterministic() {
    // The honesty label: what this resident stays SILENT about is a
    // theorem (the box rule is a sound superset), so it is not a
    // heuristic dressed up.
    assert_eq!(CheckId::Separation.kind(), CheckKind::Certified);
    let (doc, _, _) = two_roots(0.0);
    let cfg = ChecksConfig::default();
    assert_eq!(checks(&doc, &cfg), checks(&doc, &cfg));
}
