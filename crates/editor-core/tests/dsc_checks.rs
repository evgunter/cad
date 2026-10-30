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
use test_utils::refusal::tagged;

use std::collections::BTreeMap;

use editor_core::{
    Advisory, CancelToken, CheckEvidence, CheckFinding, CheckId, CheckKind, ChecksConfig,
    ChecksError, ChecksReport, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, Severity,
    enforce_checks, run_checks, subject_body,
};
use fixture::{ang, insert, len, on_frame, square};
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

/// An extruded square: half-width `h` centered at `(cx, 0)` on its own
/// xy, extruded `dz` up.
fn slab(doc: ProfileDoc, cx: f64, h: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(dz),
        },
    )
}

/// [`slab`] placed against `root`, `z0` up its own z: the placement.
fn slab_on(
    doc: ProfileDoc,
    root: RecipeNodeId,
    cx: f64,
    h: f64,
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, b) = slab(doc, cx, h, dz);
    fixture::slid(doc, root, b, [0.0, 0.0, z0])
}

/// Two disjoint unit cubes, deliberately united and placed: one copy,
/// one body, two components. Answers the placement, the checks'
/// subject.
fn disjoint_union() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-disjoint", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 1.0);
    let (doc, b) = slab_on(doc, a, 3.0, 0.5, 0.0, 1.0);
    let (doc, result) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    fixture::place(doc, result)
}

/// `A ∖ B` with `B` strictly inside `A`: the void birth — one
/// component, two shells.
fn voided() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-voided", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 1.5, 3.0);
    let (doc, b) = slab_on(doc, a, 0.0, 0.5, 1.0, 1.0);
    let (doc, result) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    );
    fixture::place(doc, result)
}

fn checks(doc: &editor_core::ProfileDoc, cfg: &ChecksConfig) -> ChecksReport {
    let ev = run(doc);
    run_checks(doc, &ev, cfg, Tol::witness()).expect("checks run over a completed evaluation")
}

#[test]
fn disjoint_union_is_one_finding() {
    let (doc, placement) = disjoint_union();
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            subject: editor_core::FindingSubject::Output {
                placement,
                output_ix: 0
            },
            evidence: CheckEvidence::Connectedness {
                actual: 2,
                expected: 1,
            },
            quiet: None,
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
    let (doc, placement) = disjoint_union();
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((placement, 0), 2)]),
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
/// subjects — and its placement.
fn annihilated() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-annihilated", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 1.0);
    let (doc, b) = slab_on(doc, a, 3.0, 0.5, 0.0, 1.0);
    let (doc, result) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    // An empty body has no geometry for a mate to read, so its
    // placement is a loose copy: placed, in no product.
    insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(result)),
    )
}

#[test]
fn stale_expectation_on_a_vanished_body() {
    // Expectations are two-directional: the entry names a placement whose
    // boolean annihilated, so NO subject consumes it — the entry is
    // stale, and staleness is a finding, never a silent ignore.
    let (doc, placement) = annihilated();
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((placement, 0), 1)]),
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            subject: editor_core::FindingSubject::Output {
                placement,
                output_ix: 0
            },
            evidence: CheckEvidence::StaleExpectation { expected: 1 },
            quiet: None,
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
fn stale_expectation_on_a_nonexistent_placement() {
    // The key names no placement output at all (wrong id): same staleness,
    // attributed at the entry's own key.
    let (doc, placement) = disjoint_union();
    let ghost = RecipeNodeId::new(placement.0.ordinal() + 999, placement.0.digest());
    let cfg = ChecksConfig {
        expected_components: BTreeMap::from([((placement, 0), 2), ((ghost, 0), 1)]),
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            subject: editor_core::FindingSubject::Output {
                placement: ghost,
                output_ix: 0,
            },
            evidence: CheckEvidence::StaleExpectation { expected: 1 },
            quiet: None,
        }]
    );
}

/// The thin row (the review's P9 fixture, promoted): a slab whose `V/A`
/// sits INSIDE the ambiguity band — built relative to the run's ε AND
/// its K, so the row holds at every matrix point and at every
/// `CAD_AMBIGUITY_K`. For a unit-square slab of thickness `(1 + K)·ε`,
/// `V/A ≈ dz/2 = ((1 + K)/2)·ε`, strictly inside `(ε, K·ε)` for every
/// K > 1. Its one shell is its solid's outer shell by arity, so the
/// count reads one component, the one expected: no finding.
#[test]
fn a_thin_slab_counts_as_one_component() {
    let tol = Tol::witness();
    let dz = (1.0 + tol.k()) * tol.eps();
    let doc = ProfileDoc::empty_derived("dsc-checks-thin", Tol::witness());
    let (doc, thin) = slab(doc, 0.0, 0.5, dz);
    let (doc, _root) = fixture::place(doc, thin);
    let report = checks(&doc, &ChecksConfig::default());
    assert!(report.findings.is_empty(), "{report}");
}

/// The void side: a 3 m box holding a unit-square cavity `(1 + K)·ε`
/// thick, its `V/A` in band on the void side. A ray aimed at one of the
/// cavity's faces reads it a cavity off its two faces, a definite pair
/// (D10, Booleans), so the union builds one solid of two shells, and
/// the count reads one component: no finding.
///
/// The sheet is what a unit cube cavity leaves when a box filling all
/// but its top `(1 + K)·ε` is united into it.
#[test]
fn a_thin_void_shell_is_a_cavity_and_the_count_reads_one() {
    let tol = Tol::witness();
    let t = (1.0 + tol.k()) * tol.eps();
    let doc = ProfileDoc::empty_derived("dsc-checks-thin-void", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 1.5, 3.0);
    let (doc, b) = slab_on(doc, a, 0.0, 0.5, 1.0, 1.0);
    let (doc, hollow) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
        },
    );
    // The filler a unit tall at first, so the union evaluates and its
    // copy stands in the world; then raised to leave the cavity `t`
    // thin over it.
    let (doc, filler) = slab(doc, 0.0, 0.8, 1.0);
    let (doc, c) = fixture::slid(doc, a, filler, [0.0, 0.0, 0.5]);
    let (doc, root) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![hollow.into(), c.into()]),
        },
    );
    let (doc, _placement) = fixture::place(doc, root);
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::SetParam {
            node: filler,
            slot: editor_core::SlotId::Distance,
            value: len(1.5 - t).into(),
            fresh: Vec::new(),
        },
    );
    let report = checks(&doc, &ChecksConfig::default());
    assert!(report.findings.is_empty(), "{report}");
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
    let editor_core::FindingSubject::Output {
        placement: at,
        output_ix,
    } = finding.subject
    else {
        panic!("a connectedness finding is about a placement output")
    };
    let (body, contacts) = subject_body(&ev, at, output_ix)
        .expect("the attribution resolves against the evaluation it came from");
    // The flagged body IS the disjoint union: the two shells the
    // finding counted (their grouping into solids is the kernel's
    // business, not pinned here).
    assert_eq!(body.body().shells().count(), 2);
    // The subject's DECLARATIONS travel with it, so the tier-3′ gate
    // reached through an attribution asks about the same body the
    // producer minted. This union carries no records (no member holds one,
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
    assert!(subject_body(&ev, at, 7).is_none());
}

// ---------------------------------------------------------------------
// The at-rest resident
// ---------------------------------------------------------------------
//
// Claims pinned below:
// - two placements overlapping are ONE interference finding naming BOTH, and
//   the same two placements moved apart are clean;
// - placements that merely TOUCH are the lint's contact, never this
//   resident's finding;
// - one placement's own multi-solid body is NOT this resident's subject;
// - `Off` is visibly skipped, independently of the other resident;
// - `Error` refuses a loud overlap at `enforce_checks` and nowhere else;
// - the report is deterministic across runs.

/// Two slabs placed as two SEPARATE copies (no boolean joining them),
/// `b` centered at `cx`: the two placements, the shape a document
/// grows when a body is placed beside what it was meant to join.
fn two_placements(cx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dsc-checks-two-roots", Tol::witness());
    let (doc, a) = slab(doc, 0.0, 0.5, 1.0);
    let (doc, b) = slab(doc, cx, 0.5, 1.0);
    let (doc, a) = fixture::place(doc, a);
    let (doc, b) = fixture::place(doc, b);
    (doc, a, b)
}

/// The report's at-rest findings: a contact between two placed copies
/// is the census's row, which the `unproven-coincidence` check reports
/// instead.
fn at_rest(report: &editor_core::ChecksReport) -> Vec<CheckFinding> {
    report
        .findings
        .iter()
        .filter(|f| f.check == CheckId::AtRest)
        .cloned()
        .collect()
}

#[test]
fn overlapping_placements_are_one_interference_finding_naming_both() {
    // Half over each other: the census decides the two copies' material
    // overlaps, and the check bounds the overlap.
    let (doc, a, b) = two_placements(0.5);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
    let found = at_rest(&report);
    let [finding] = found.as_slice() else {
        panic!("one interference finding: {report}")
    };
    let CheckEvidence::Interference { finding: overlap } = &finding.evidence else {
        panic!("an interference finding: {finding:?}")
    };
    let var = |n| doc.output(n, 0).expect("a placement's output");
    assert_eq!((overlap.a.var, overlap.b.var), (var(a), var(b)));
    assert!(finding.quiet.is_none() && overlap.is_loud());
    assert!(
        matches!(overlap.overlap, editor_core::Overlap::Bounded { .. }),
        "{overlap:?}"
    );
    // It says both copies in its own sentence.
    let rendered = finding.to_string();
    assert!(rendered.contains("overlap"), "{rendered}");
}

#[test]
fn placements_moved_apart_are_clean() {
    // Same two placements, far apart: the census finds no pair at all.
    let (doc, _, _) = two_placements(3.0);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.skipped, vec![CheckId::ChartCoherence]);
}

#[test]
fn touching_placements_are_the_lints_contact_and_no_at_rest_finding() {
    // Face-to-face at x = 0.5: disjoint interiors, shared boundary. The
    // census records the contact as a row, the lint reports it, and the
    // at-rest check reports nothing of it.
    let (doc, _, _) = two_placements(1.0);
    let report = checks(&doc, &ChecksConfig::default());
    assert_eq!(at_rest(&report), Vec::new(), "{report}");
    let rests = report
        .findings
        .iter()
        .filter(|f| {
            matches!(
                &f.evidence,
                CheckEvidence::UnprovenCoincidence { row, .. }
                    if row.relation == topo::Relation::SameOpposite
            )
        })
        .count();
    assert_eq!(rests, 1, "one rest, the lint's: {report}");
}

/// A legitimately disjoint multi-solid body draws no at-rest finding:
/// `disjoint_union` is ONE placement denoting one solid with two outer
/// shells, and the census reads nothing inside one arrival.
#[test]
fn a_deliberately_disjoint_body_draws_no_at_rest_finding() {
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

/// **A world whose gather refuses**: a cube instance pinned to the
/// world and a second placed on it by a constraint whose side names a
/// face the part does not have, so that placement fails and the gather
/// refuses it typed. Answers the document, the options that resolve its
/// part, and the failed placement.
fn failed_placement_world(id: &str) -> (ProfileDoc, EvalOptions, RecipeNodeId) {
    use editor_core::{CapEnd, EntityKind, RoleSeg, StableName};
    let mut store = fixture::resolver::PartStore::default();
    let part = ProfileDoc::empty_derived(&format!("{id}-part"), Tol::witness());
    let (part, profile) = on_frame(
        part,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (part, body) = insert(
        part,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (part_ref, body) = store.insert_part((part, body), Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let doc = ProfileDoc::empty_derived(id, Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(part_ref), &opts);
    let (doc, placed) = insert(doc, fixture::mated_instance(part_ref));
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, 99),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let mate = fixture::rest(
        fixture::head_at(
            placed,
            fixture::resolver::in_part(placed, body, CapEnd::Start),
        ),
        fixture::head_at(base, nowhere),
    );
    let (doc, _) = fixture::constrain_at(doc, placed, mate);
    (doc, opts, placed)
}

#[test]
fn at_rest_off_is_visibly_skipped_and_independent() {
    let (doc, _, _) = two_placements(0.5);
    let cfg = ChecksConfig {
        at_rest: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = checks(&doc, &cfg);
    assert_eq!(
        report.skipped,
        vec![CheckId::ChartCoherence, CheckId::AtRest]
    );
    assert_eq!(at_rest(&report), Vec::new());

    // The other direction: turning connectedness off leaves the
    // at-rest resident running. Before this resident existed the
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
    assert_eq!(at_rest(&report).len(), 1, "{report}");

    // **And `Off` means the subject is never derived**, which is the
    // half a document that gathers cannot show: a document whose
    // gather REFUSES still reports, because with the subject-reading
    // residents off there is nothing to gather for. A placement whose
    // constraint names no face fails, and the gather refuses it.
    // Connectedness reads every placement's value and would refuse the
    // failed one on its own, so it is off here too.
    let (doc, opts, _) = failed_placement_world("dsc-checks-unplaced");
    let ev = fixture::run(&doc, &opts);
    assert!(
        editor_core::product_recorded(&doc, &ev, Tol::witness()).is_err(),
        "the premise: this document's gather refuses"
    );
    let off = ChecksConfig {
        connectedness: Severity::Off,
        unproven_coincidence: Advisory::Off,
        at_rest: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &off, Tol::witness())
        .expect("with the subject-reading residents off, no gather is attempted");
    assert_eq!(
        report.skipped,
        vec![
            CheckId::Connectedness,
            CheckId::ChartCoherence,
            CheckId::UnprovenCoincidence,
            CheckId::AtRest
        ]
    );
    // …and with the at-rest check on, the same document refuses on the
    // subject.
    let on = ChecksConfig {
        at_rest: Severity::Warn,
        ..off
    };
    assert!(
        matches!(
            run_checks(&doc, &ev, &on, Tol::witness()),
            Err(ChecksError::Product { .. })
        ),
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
            // `UnprovenCoincidence`, this resident would lose every
            // finding on exactly the documents that do not gather.
            CheckId::ChartCoherence => 1,
            // Before `UnprovenCoincidence` for the same reason: it reads
            // no subject.
            CheckId::Unused => 2,
            CheckId::UnprovenCoincidence => 3,
            CheckId::AtRest => 4,
        };
        assert_eq!(
            CheckId::ALL[position],
            check,
            "{check} is not where the registry's order puts it"
        );
    }
    assert_eq!(
        CheckId::ALL.len(),
        5,
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
    assert!(CheckId::UnprovenCoincidence.reads_subject());
    assert!(CheckId::AtRest.reads_subject());
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

/// INVARIANT: the at-rest check refuses only a loud interference
/// finding, and only at `Error` (DS6: an assertion is its declarable
/// exception).
#[test]
fn at_rest_refuses_a_loud_interference_at_error_only() {
    let (doc, _, _) = two_placements(0.5);
    let warn = ChecksConfig::default();
    let report = checks(&doc, &warn);
    assert_eq!(at_rest(&report).len(), 1, "{report}");
    assert!(enforce_checks(&report, &warn).is_ok(), "Warn never gates");
    let error = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    let refused = enforce_checks(&report, &error).expect_err("a loud overlap refuses at Error");
    assert_eq!(refused.findings, at_rest(&report));
}

#[test]
fn at_rest_findings_are_certified_and_deterministic() {
    assert_eq!(CheckId::AtRest.kind(), CheckKind::Certified);
    let (doc, _, _) = two_placements(0.5);
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
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("dsc-checks-washer", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, washer) = insert(
        doc,
        fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    fixture::place(doc, washer)
}

/// Only the chart-coherence resident, at `knob`.
fn coherence_only(knob: Advisory) -> ChecksConfig {
    ChecksConfig {
        connectedness: Severity::Off,
        at_rest: Severity::Off,
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
    let (doc, placement) = washer();
    let ev = run(&doc);
    let report = run_checks(&doc, &ev, &coherence_only(Advisory::Warn), Tol::witness())
        .expect("the examination reads the evaluation and needs no product");

    let (body, _) = subject_body(&ev, placement, 0).expect("the revolve denotes one body");
    let door = topo::examine_chart_coherence(body.body(), Tol::witness());

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
            .all(|f| f.check == CheckId::ChartCoherence
                && f.subject
                    == editor_core::FindingSubject::Output {
                        placement,
                        output_ix: 0
                    }),
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
            CheckId::AtRest
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
        document: editor_core::DocumentId(1),
        findings: vec![CheckFinding {
            check: CheckId::ChartCoherence,
            subject: editor_core::FindingSubject::Output {
                placement: RecipeNodeId::new(0, tagged(3)),
                output_ix: 0,
            },
            evidence: CheckEvidence::ChartCoherenceUnexamined {
                unexamined: topo::Unexamined {
                    face: topo::FaceKey::default(),
                    r#loop: topo::LoopKey::default(),
                    why: topo::Unexaminable::NonIsoCarrier {
                        edge: topo::EdgeKey::default(),
                    },
                },
            },
            quiet: None,
        }],
        skipped: Vec::new(),
        pending: Vec::new(),
    };
    let chose_not_to = ChecksReport {
        document: editor_core::DocumentId(1),
        findings: Vec::new(),
        skipped: vec![CheckId::ChartCoherence],
        pending: Vec::new(),
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
        subject: editor_core::FindingSubject::Output {
            placement: RecipeNodeId::new(0, tagged(4)),
            output_ix: 1,
        },
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
        quiet: None,
    };
    let rendered = finding.to_string();
    assert!(
        rendered.contains("check chart-coherence: placement 000000000004 output 1"),
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
        rendered.contains("MEASUREMENT, not a refusal"),
        "the recourse says what a finding is and is not: {rendered}"
    );
}

/// **A placement tier 3 refuses is a finding, never a silent skip** (F6): the
/// connectedness check reads a finished body, so a subject the at-rest
/// gate refuses is reported `NotAtRest` with the validator's findings
/// verbatim, its rendering names the first, and `enforce_checks` at
/// `Error` refuses on it. No real document reaches the gate's refusal
/// today — every op that hands back a body gates it or builds one tier 3
/// passes — so the evaluated slab's value is replaced with the same slab
/// turned inside out (`Body::revert`, a public op), which tier 3 refuses
/// `NegativeVolume`.
#[test]
fn a_placement_tier_3_refuses_is_reported_not_at_rest() {
    use editor_core::{NodeResult, ValuePayload};
    let doc = ProfileDoc::empty_derived("dsc-checks-not-at-rest", Tol::witness());
    let (doc, slab) = slab(doc, 0.0, 0.5, 1.0);
    let (doc, placement) = fixture::place(doc, slab);
    let mut ev = run(&doc);
    let Some(NodeResult::Ok(value)) = ev.nodes.get_mut(&placement) else {
        panic!("the slab evaluates");
    };
    let ValuePayload::Body(body) = &value.payload else {
        panic!("a placed slab is a body");
    };
    let reverted = topo::Body::clone(body.body())
        .revert()
        .expect("the slab reverts");
    let errors = topo::validate_geometric(&reverted, Tol::witness())
        .expect_err("tier 3 refuses the inside-out slab");
    value.payload = ValuePayload::Body(editor_core::Framed::own(std::sync::Arc::new(reverted)));

    // The at-rest and coincidence residents read the gathered
    // product, whose own at-rest gate refuses this placement first; off, the
    // connectedness pass is the one that speaks.
    let cfg = ChecksConfig {
        connectedness: Severity::Error,
        unproven_coincidence: Advisory::Off,
        at_rest: Severity::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(&doc, &ev, &cfg, Tol::witness()).expect("the checks run");
    assert_eq!(
        report.findings,
        vec![CheckFinding {
            check: CheckId::Connectedness,
            subject: editor_core::FindingSubject::Output {
                placement,
                output_ix: 0
            },
            evidence: CheckEvidence::NotAtRest {
                errors: errors.clone()
            },
            quiet: None,
        }]
    );
    let rendered = report.findings[0].to_string();
    assert!(rendered.contains(&errors[0].to_string()), "{rendered}");
    let refusal = enforce_checks(&report, &cfg).unwrap_err();
    assert_eq!(refusal.findings, report.findings);
}
