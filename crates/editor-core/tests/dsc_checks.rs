//! The check registry and its residents (DISCIPLINES-DESIGN DS6;
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

use crate::fixture;

use std::collections::BTreeMap;

use editor_core::{
    Advisory, BooleanOp, CancelToken, CheckEvidence, CheckFinding, CheckId, CheckKind,
    ChecksConfig, ChecksReport, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, Severity,
    enforce_checks, run_checks, subject_body,
};
use fixture::{ang, insert, len, on_frame, scl, square};
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
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, h)],
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
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
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
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
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
    assert_eq!(
        report.skipped,
        vec![CheckId::Connectedness, CheckId::ChartCoherence]
    );
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
/// run's ε AND its K, so the row holds at every matrix point and at
/// every `CAD_AMBIGUITY_K`. For a unit-square slab of thickness
/// `(1 + K)·ε`, `V/A ≈ dz/2 = ((1 + K)/2)·ε`, which is strictly inside
/// `(ε, K·ε)` for every K > 1 — the only K the tolerance accepts.
/// Pinned at `10ε` the row was a claim about K = 10: at any K below 3
/// the same slab is DEFINITE and the escalation it asserts never
/// happens.
#[test]
fn in_band_shell_escalates_typed_never_guessed() {
    let tol = Tol::witness();
    let dz = (1.0 + tol.k()) * tol.eps();
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
    let (body, contacts) = subject_body(&ev, finding.root, finding.output_ix)
        .expect("the attribution resolves against the evaluation it came from");
    // The flagged body IS the disjoint union: the two shells the
    // finding counted (their grouping into solids is the kernel's
    // business, not pinned here).
    assert_eq!(body.shells().count(), 2);
    // The subject's DECLARATIONS travel with it, so the tier-3′ gate
    // reached through an attribution asks about the same body the
    // producer minted. This union declares nothing (`declare: None`,
    // and its operands are three metres apart), so the honest claim
    // here is that the empty set is what arrived — not that the pair
    // is populated. The case where a non-empty set is the difference
    // between passing and refusing is a carried D-1 record set, which
    // needs a store and a referenced document; it is pinned at the
    // Python boundary instead
    // (`test_checks.py::TestSubjectBodyCarriesItsDeclarations`), which
    // is where the narrowing was measured.
    assert_eq!(*contacts, topo::ContactRecords::default());
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
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
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
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
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

/// A legitimately disjoint multi-solid body draws no separation
/// finding.
///
/// **What this does NOT prove**, said here because the row's first
/// name claimed it did: `disjoint_union` is ONE root denoting ONE
/// SOLID with two outer shells, not two solids, so the pair walk emits
/// no same-subject pair for it at any separation and the guard in
/// `checks::separation` never executes. (An earlier draft of this note
/// said the two solids were "3.0 apart" so `certify` granted — also
/// true of the shells' boxes, but not the mechanism: there is no pair
/// here to certify. A boolean union of separated operands yields one
/// multi-SHELL solid; the document-layer shape that yields a
/// multi-SOLID body is an instantiated part.) This row goes green
/// against a build with the guard deleted, which is how a deletion of
/// it once survived a full local suite. The guard's own row is
/// `asm_r2b_assembly::two_solids_of_one_subject_are_skipped_by_the_guard_not_by_geometry`,
/// where the two solids are COINCIDENT and nothing but the guard can
/// keep the resident quiet. What this row still pins is worth pinning
/// on its own: a body that is deliberately several solids is not
/// noise, and the resident stays out of the connectedness resident's
/// subject.
#[test]
fn a_deliberately_disjoint_body_draws_no_separation_finding() {
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
        separation: Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.skipped,
        vec![CheckId::ChartCoherence, CheckId::Separation]
    );
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
    assert_eq!(
        report.skipped,
        vec![CheckId::Connectedness, CheckId::ChartCoherence]
    );
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].check, CheckId::Separation);

    // **And `Off` means the subject is never derived**, which is the
    // half a document that gathers cannot show: a document whose
    // gather REFUSES still reports, because with the only
    // subject-reading resident off there is nothing to gather for.
    // Two `Transform`s of one extrude are two roots whose name rows
    // collide in the product table.
    let doc = ProfileDoc::empty_derived("dsc-checks-collide", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let moved = |doc, dx: f64| {
        insert(
            doc,
            Node::Transform {
                input: extrude,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        )
    };
    let (doc, _) = moved(doc, 3.0);
    let (doc, _) = moved(doc, 6.0);
    let ev = run(&doc);
    assert!(
        editor_core::product_recorded(&doc, &ev, Tol::witness()).is_err(),
        "the premise: this document's gather refuses"
    );
    let off = ChecksConfig {
        separation: Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &off, Tol::witness())
        .expect("with the subject-reading resident off, no gather is attempted");
    assert_eq!(
        report.skipped,
        vec![CheckId::ChartCoherence, CheckId::Separation]
    );
    // …and with it on, the same document refuses on the subject.
    assert!(
        run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness()).is_err(),
        "the control: the resident that reads the subject is what needs one"
    );
}

/// INVARIANT: [`CheckId::ALL`] is EVERY check, in the order the
/// registry runs them — the list `ChecksConfig::needs_a_subject` folds
/// over, so a check missing from it would be a resident whose
/// subject-reading never made the registry gather.
///
/// The match is the compiler's own walk of the closed set: a new
/// variant fails to compile here until it is named, and the assertion
/// then fails until it is in `ALL`.
#[test]
fn the_registry_order_is_every_check() {
    // WHAT THIS CAN AND CANNOT DO: no test can prove a constant array
    // lists every variant of an enum. What the match below does is
    // fail to COMPILE when a variant is added, at which point its
    // position has to be written down here — and the assertion then
    // fails until `ALL` carries it. That is the walk, and it is the
    // same mechanism `ChecksConfig::severity` relies on.
    for check in CheckId::ALL {
        let position = match check {
            CheckId::Connectedness => 0,
            // Second, not last, and the reason is the registry's
            // refusal order rather than taste: a resident that reads
            // no subject must answer BEFORE `run_checks_on` can refuse
            // `ChecksError::Product` on one that does. Moved after
            // `Separation`, this resident would lose every finding on
            // exactly the documents that do not gather.
            CheckId::ChartCoherence => 1,
            CheckId::Separation => 2,
        };
        assert_eq!(
            CheckId::ALL[position],
            check,
            "{check} is not where the registry's order puts it"
        );
    }
    assert_eq!(
        CheckId::ALL.len(),
        3,
        "a variant added without a place in `ALL` is a resident the \
         registry would never gather for"
    );
    // ChecksReport's skipped list is SPACE-separated, so a check whose
    // rendered name carries a space makes two entries indistinguishable
    // from one. Red-capable: a name of "chart coherence" fires this.
    for check in CheckId::ALL {
        let name = check.to_string();
        assert!(
            !name.is_empty() && !name.contains(char::is_whitespace),
            "{name:?} is rendered into a space-separated list and must be one token"
        );
    }
    // The one resident that reads a subject is the one the registry
    // gathers for.
    assert!(!CheckId::Connectedness.reads_subject());
    assert!(!CheckId::ChartCoherence.reads_subject());
    assert!(CheckId::Separation.reads_subject());
    // The order's own invariant, stated as a predicate rather than as
    // the three indices above: every subject-less resident precedes
    // every subject-reading one.
    let first_reader = CheckId::ALL.iter().position(|c| c.reads_subject());
    let last_nonreader = CheckId::ALL.iter().rposition(|c| !c.reads_subject());
    assert!(
        first_reader > last_nonreader,
        "a subject-reading resident runs before a subject-less one, so the latter's \
         findings are lost whenever the gather refuses"
    );
}

/// INVARIANT: this resident cannot refuse, and the TYPE is what says
/// so — DS6 lets a check offer `error` only *iff* it ships a waiver
/// vocabulary, and this one ships none.
///
/// `ChecksConfig::separation` is an [`Advisory`], which has no `Error`
/// position to set, so `enforce_checks` cannot refuse on a separation
/// finding however the config is written. The row that used to live
/// here asserted the opposite (it set `Severity::Error` and checked
/// that `enforce_checks` refused), which is exactly the DS6 violation
/// the narrower type retired.
#[test]
fn separation_cannot_refuse_because_it_ships_no_waiver() {
    let (doc, _, _) = two_roots(0.0);
    let cfg = ChecksConfig::default();
    let report = checks(&doc, &cfg);
    assert_eq!(report.findings.len(), 1);
    assert!(
        enforce_checks(&report, &cfg).is_ok(),
        "a finding from a waiver-less resident never gates"
    );
    // Every position this knob HAS maps below `Error`.
    for advisory in [Advisory::Off, Advisory::Warn] {
        assert_ne!(advisory.severity(), Severity::Error);
    }
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

// ---------------------------------------------------------------
// The chart-coherence resident (topo::examine_chart_coherence)
// ---------------------------------------------------------------

/// A washer: a rectangle off the axis revolved a full turn — two
/// cylinder bands and two plane annuli, so the body carries
/// chart-bearing faces for the examination to read.
fn washer() -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("dsc-checks-washer", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    )
}

/// Only the chart-coherence resident, at `knob`.
fn coherence_only(knob: Advisory) -> ChecksConfig {
    ChecksConfig {
        connectedness: Severity::Off,
        separation: Advisory::Off,
        chart_coherence: knob,
        ..ChecksConfig::default()
    }
}

/// INVARIANT: the resident RUNS, it reads the kernel door, and it
/// carries BOTH of that door's lists — its findings and its unexamined
/// loops — into the report, one finding each.
///
/// The cross-check is against the door called directly on the same
/// body, so a resident that dropped either list (or read a different
/// body) reds here rather than reporting a quiet subject.
#[test]
fn the_chart_coherence_resident_carries_the_whole_kernel_report() {
    let (doc, root) = washer();
    let ev = run(&doc);
    let report = run_checks(&doc, &ev, &coherence_only(Advisory::Warn), Tol::witness())
        .expect("the examination reads the evaluation and needs no product");

    let (body, _) = subject_body(&ev, root, 0).expect("the revolve denotes one body");
    let door = topo::examine_chart_coherence(body.as_ref(), Tol::witness());

    let measured = report
        .findings
        .iter()
        .filter(|f| matches!(f.evidence, CheckEvidence::ChartCoherence { .. }))
        .count();
    let unreadable = report
        .findings
        .iter()
        .filter(|f| matches!(f.evidence, CheckEvidence::ChartCoherenceUnexamined { .. }))
        .count();
    assert_eq!(
        (measured, unreadable),
        (door.findings.len(), door.unexamined.len()),
        "the resident's two finding classes are the door's two lists, one for one"
    );
    assert!(
        report
            .findings
            .iter()
            .all(|f| f.check == CheckId::ChartCoherence && f.root == root && f.output_ix == 0),
        "every finding is attributed to the rest body it was measured on"
    );
    assert!(
        !report.skipped.contains(&CheckId::ChartCoherence),
        "a resident that RAN is not a skipped check, whatever it found"
    );
}

/// INVARIANT: `Off` is the configuration answer and it is VISIBLY off
/// — the one thing that tells a reader the examination did not run.
///
/// Red-capable in both directions: a resident that ran anyway would
/// leave `skipped` empty, and one whose `Off` branch reported nothing
/// at all would be indistinguishable from a clean body.
#[test]
fn chart_coherence_off_is_a_skipped_check_and_nothing_else() {
    let (doc, _) = washer();
    let ev = run(&doc);
    let report = run_checks(&doc, &ev, &coherence_only(Advisory::Off), Tol::witness())
        .expect("nothing runs, nothing refuses");
    assert_eq!(
        report.skipped,
        vec![
            CheckId::Connectedness,
            CheckId::ChartCoherence,
            CheckId::Separation
        ],
        "every Off resident is named, in registry order"
    );
    assert!(report.findings.is_empty());
}

/// INVARIANT: **a loop the DATA put out of reach and a check the
/// CONFIGURATION turned off reach the user as different things.**
///
/// `topo::CoherenceReport { findings, unexamined }` and
/// `ChecksReport { findings, skipped }` have the same shape and
/// different meanings, and folding the two would report "we chose not
/// to look" and "we could not look" as one answer. This row is what
/// reds if they are ever folded: the two reports below share not one
/// word of rendering, and neither can be spelled as the other.
#[test]
fn an_unexamined_loop_is_a_finding_never_a_skipped_check() {
    let could_not_look = ChecksReport {
        findings: vec![CheckFinding {
            check: CheckId::ChartCoherence,
            root: RecipeNodeId(3),
            output_ix: 0,
            evidence: CheckEvidence::ChartCoherenceUnexamined {
                unexamined: topo::Unexamined {
                    face: topo::FaceKey::default(),
                    r#loop: topo::LoopKey::default(),
                    why: topo::Unexaminable::NonIsoCarrier {
                        edge: topo::EdgeKey::default(),
                    },
                },
            },
        }],
        skipped: Vec::new(),
    };
    let chose_not_to = ChecksReport {
        findings: Vec::new(),
        skipped: vec![CheckId::ChartCoherence],
    };

    let data = could_not_look.to_string();
    let config = chose_not_to.to_string();
    assert!(
        data.contains("could not be examined") && data.contains("checks: 1 finding(s)"),
        "an unreadable loop is a FINDING and says why: {data}"
    );
    assert!(
        !data.contains("skipped"),
        "and it is never reported as a skipped check: {data}"
    );
    assert!(
        config.contains("checks skipped (severity Off): chart-coherence")
            && config.contains("checks: no findings"),
        "an Off check is a skipped CHECK and produces no finding: {config}"
    );
    assert!(
        !config.contains("could not be examined"),
        "and never borrows the data half's words: {config}"
    );
}

/// INVARIANT: a measurement renders its metres, its two factors and
/// the band it was judged against — the band because `metres` read
/// without it is a number without a claim.
#[test]
fn a_coherence_measurement_renders_its_length_and_its_band() {
    let finding = CheckFinding {
        check: CheckId::ChartCoherence,
        root: RecipeNodeId(4),
        output_ix: 1,
        evidence: CheckEvidence::ChartCoherence {
            finding: topo::CoherenceFinding {
                face: topo::FaceKey::default(),
                r#loop: topo::LoopKey::default(),
                edge: topo::EdgeKey::default(),
                condition: topo::CoherenceCondition::MeridianClosure {
                    vertex: topo::VertexKey::default(),
                },
                gap: std::f64::consts::PI,
                lever: 1.0e-9,
                metres: std::f64::consts::PI * 1.0e-9,
                eps: 1.0e-12,
            },
        },
    };
    let rendered = finding.to_string();
    assert!(
        rendered.contains("check chart-coherence: root 4 output 1"),
        "{rendered}"
    );
    assert!(
        rendered.contains("carrier midpoint") && rendered.contains("3.14159"),
        "the condition and the length it measured: {rendered}"
    );
    assert!(
        rendered.contains("band 1e-12"),
        "the band it was judged at: {rendered}"
    );
    assert!(
        rendered.contains("MEASUREMENT and nothing refuses on it"),
        "the recourse says what a finding is and is not: {rendered}"
    );
}
