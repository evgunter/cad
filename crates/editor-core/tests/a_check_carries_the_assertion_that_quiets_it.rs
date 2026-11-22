//! **A finding carries the assertion that quiets it** (DESIGN
//! `[one-way.places]`, "A refusal's recourse"; D10 `[assertions.quiet]`).
//!
//! The law: for every loud unproven-coincidence finding that carries an
//! assertion ([`editor_core::Quieting`]), the edits that author it
//! apply, and the checks run again report the same row quiet under the
//! new assertion. A finding that carries none says no assertion in its
//! recourse and never refuses. The law is held over the corpus and over
//! each fixture of `atrest_operation_rows_quiet`, whose rows cover the
//! kinds of row the rule says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, run};
use crate::corpus;
use crate::fixture::{self, insert};
use editor_core::{
    CheckEvidence, CheckFinding, CheckId, ChecksConfig, Coord, Formula, NamedCoincidence, Operand,
    ProfileDoc, Quieting, Severity, Speaker, TargetAxis, enforce_checks,
};
use geom_core::Tol;

fn checks(doc: &ProfileDoc) -> editor_core::ChecksReport {
    editor_core::run_checks(
        doc,
        &run(doc),
        &ChecksConfig::default(),
        &editor_core::Stop::default(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

/// Whether [`enforce_checks`] refuses `finding` at `Error`, read alone
/// under the at-rest check's knob, which spells one.
fn refuses_at_error(report: &editor_core::ChecksReport, finding: &CheckFinding) -> bool {
    let mut alone = report.clone();
    alone.findings = vec![CheckFinding {
        check: CheckId::AtRest,
        ..finding.clone()
    }];
    let cfg = ChecksConfig {
        at_rest: Severity::Error,
        ..ChecksConfig::default()
    };
    enforce_checks(&alone, &cfg).is_err()
}

/// The unproven-coincidence findings of `report`, each with its row and
/// the assertion it carries.
fn coincidences(
    report: &editor_core::ChecksReport,
) -> Vec<(&CheckFinding, &NamedCoincidence, Option<&Quieting>)> {
    report
        .findings
        .iter()
        .filter_map(|f| match &f.evidence {
            CheckEvidence::UnprovenCoincidence { row, quieting, .. } => {
                Some((f, &**row, quieting.as_ref()))
            }
            _ => None,
        })
        .collect()
}

/// `doc` after `quieting`'s edits, applied one by one as a session
/// commits them, and the assertion they author.
pub(crate) fn authored(
    what: &str,
    doc: &ProfileDoc,
    quieting: &Quieting,
) -> (ProfileDoc, editor_core::RecipeNodeId) {
    let edits = quieting
        .edits(doc, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: the offered edits apply: {e}\n{quieting:?}"));
    let replayed = edits.edits.iter().fold(doc.clone(), |doc, edit| {
        doc.apply(edit, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: an offered edit replays: {e}"))
            .doc
    });
    assert_eq!(
        replayed.ids(),
        edits.doc.ids(),
        "{what}: the edits replay to the document they produced"
    );
    assert_eq!(
        replayed.ids().last(),
        Some(&edits.assertion),
        "{what}: the assertion is the last node authored"
    );
    (replayed, edits.assertion)
}

/// **The law on `doc`**: every loud unproven-coincidence finding that
/// carries an assertion refuses at `Error` and says the claim in its
/// recourse; every one that carries none names no assertion and never
/// refuses; and with each distinct claim authored, in finding order,
/// every row that carried one is reported quiet under the assertion
/// authored for its claim, named in its sentence. One claim can be two
/// rows' (a family's members), and then one assertion quiets both.
/// Answers how many rows it quieted.
pub(crate) fn every_offered_assertion_quiets(what: &str, doc: &ProfileDoc) -> usize {
    let report = checks(doc);
    let by = Speaker::of(doc);
    let mut after = doc.clone();
    let mut assertions: Vec<(&Quieting, editor_core::RecipeNodeId)> = Vec::new();
    let mut offered = Vec::new();
    for (found, row, quieting) in coincidences(&report) {
        let said = editor_core::Said(found, by).to_string();
        let Some(quieting) = quieting else {
            assert!(
                !said.contains("assert") && !refuses_at_error(&report, found),
                "{what}: a finding carrying no assertion offers none and never refuses: {said}"
            );
            continue;
        };
        if found.quiet.is_some() {
            continue;
        }
        let claim = quieting.said(by);
        assert!(
            said.contains(&format!("assert {claim}")),
            "{what}: the recourse says the claim it carries, {claim}: {said}"
        );
        assert!(
            refuses_at_error(&report, found),
            "{what}: a loud finding an assertion could quiet refuses at Error: {said}"
        );
        let assertion = match assertions.iter().find(|(held, _)| *held == quieting) {
            Some((_, assertion)) => *assertion,
            None => {
                let (next, assertion) = authored(what, &after, quieting);
                after = next;
                assertions.push((quieting, assertion));
                assertion
            }
        };
        offered.push((found.subject, row, claim, assertion));
    }
    if offered.is_empty() {
        return 0;
    }
    let report = checks(&after);
    let by = Speaker::of(&after);
    let now = coincidences(&report);
    for (subject, row, claim, assertion) in &offered {
        let quieted: Vec<_> = now
            .iter()
            .filter(|(f, r, _)| f.subject == *subject && r == row)
            .collect();
        let [(found, _, _)] = quieted.as_slice() else {
            panic!("{what}: the row is still reported, once: {quieted:?}\n{report}")
        };
        assert_eq!(
            found.quiet,
            Some(*assertion),
            "{what}: the row is quiet under the assertion authored for {claim}: {found}"
        );
        let named = by.node_as(*assertion, "assertion").to_string();
        let said = editor_core::Said(*found, by).to_string();
        assert!(
            said.contains(&format!("quiet: {named} says so")),
            "{what}: the quiet row names the new assertion {named}: {said}"
        );
    }
    offered.len()
}

/// Corpus documents whose checks do not run at `f64`, each a union the
/// kernel refuses there: no finding of theirs is read.
const CHECKS_DO_NOT_RUN: [&str; 1] = ["cone_on_sphere"];

/// Corpus documents the law is known to bite on: each offers an
/// assertion, so a run that read none of them asserted nothing.
const OFFERS: [&str; 6] = [
    "symmetric_plate",
    "kiss_carry",
    "axis_projection",
    "blocks_on_one_floor",
    "revolves_on_one_axis",
    "shaft_and_bearing",
];

/// The law over every corpus document at an index congruent to `part`
/// mod [`PARTS`]: the corpus's checks cost minutes, so the documents
/// are dealt over that many tests.
fn the_law_over_the_corpus(part: usize) {
    let documents = corpus::documents()
        .into_iter()
        .chain(corpus::stage6_beside_the_registry());
    for (_, doc) in documents.enumerate().filter(|(i, _)| i % PARTS == part) {
        if CHECKS_DO_NOT_RUN.contains(&doc.name) {
            let refused = editor_core::run_checks(
                &doc.doc,
                &run(&doc.doc),
                &ChecksConfig::default(),
                &editor_core::Stop::default(),
                Tol::witness(),
            );
            assert!(
                refused.is_err(),
                "{}: its checks run now, so the law reads it: drop it from CHECKS_DO_NOT_RUN",
                doc.name
            );
            continue;
        }
        let quieted = every_offered_assertion_quiets(doc.name, &doc.doc);
        assert!(
            quieted > 0 || !OFFERS.contains(&doc.name),
            "{}: the document offers an assertion",
            doc.name
        );
    }
}

const PARTS: usize = 4;

/// **Every assertion the corpus's findings offer quiets its row**
/// ([`every_offered_assertion_quiets`], over [`corpus::documents`] and
/// the stage-6 documents beside it).
///
/// *Breaks if* the rule that mints the assertion and the rule that
/// matches a holding one read a row differently: a measure the rule does
/// not take, a reference read as another cell or another pose, a cell
/// order the measure reads non-zero.
#[test]
fn every_offered_assertion_across_the_corpus_quiets_its_row_part_0() {
    the_law_over_the_corpus(0);
}

/// [`every_offered_assertion_across_the_corpus_quiets_its_row_part_0`],
/// the second deal.
#[test]
fn every_offered_assertion_across_the_corpus_quiets_its_row_part_1() {
    the_law_over_the_corpus(1);
}

/// The third deal.
#[test]
fn every_offered_assertion_across_the_corpus_quiets_its_row_part_2() {
    the_law_over_the_corpus(2);
}

/// The fourth deal.
#[test]
fn every_offered_assertion_across_the_corpus_quiets_its_row_part_3() {
    the_law_over_the_corpus(3);
}

/// Every document of [`OFFERS`] and [`CHECKS_DO_NOT_RUN`] is in the
/// corpus, so neither roster holds a name the law never meets.
#[test]
fn the_laws_rosters_name_corpus_documents() {
    let names: Vec<&str> = corpus::documents()
        .into_iter()
        .chain(corpus::stage6_beside_the_registry())
        .map(|doc| doc.name)
        .collect();
    for name in OFFERS.iter().chain(&CHECKS_DO_NOT_RUN) {
        assert!(names.contains(name), "{name} is a corpus document");
    }
}

/// `n` fins standing on a base, one placement whose slide reads
/// `k = index(n)`, `pitch` apart: each fin rests on the base's top, and
/// at a pitch of the fin's width each rests against the next.
fn fins(n: i64, pitch: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("offer-fins", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", n);
    let (doc, base) = block(doc, (0.0, 13.0), (0.0, 2.0), 1.0);
    let (doc, fin) = block(doc, (0.25, 0.5), (0.5, 1.5), 1.25);
    let (doc, _) = fixture::place(doc, base);
    let slide = Formula::mul(fixture::scalar_of(k), fixture::len(pitch)).unwrap();
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::slide(TargetAxis::X), slide),
            (Coord::slide(TargetAxis::Z), fixture::len(1.0)),
        ],
    );
    let node = fixture::place_node(
        &doc,
        editor_core::BodyRead::plain(Operand::Node(fin)),
        placed.bundle(),
    );
    let (doc, _) = insert(doc, node);
    editor_core::test_support::assemble_world_copies(doc, Tol::witness())
}

/// **A family member's row carries the one assertion whose reads reach
/// the index, and that assertion quiets every member's row** (D10
/// `[assertions]`, "Families"). Three fins resting on a base: each
/// fin's rest is its own finding, each carries the same claim (the
/// fin's foot selected off the family, against the base's top), and the
/// claim authored once is one assertion per member, each quieting its
/// own fin's row.
///
/// *Breaks if* a member's cell is written as the member's own name (a
/// selection no member of the family resolves), or the rule matches a
/// family assertion's reference by the name it holds rather than the
/// name its member reads.
#[test]
fn a_family_members_row_carries_the_assertion_that_reaches_its_index() {
    let doc = fins(3, 0.75);
    let report = checks(&doc);
    let rests = coincidences(&report);
    let claims: Vec<&Quieting> = rests.iter().filter_map(|(_, _, q)| *q).collect();
    assert_eq!(
        (rests.len(), claims.len()),
        (3, 3),
        "one rest per fin, each carrying an assertion: {report}"
    );
    assert!(
        claims.iter().all(|q| *q == claims[0]),
        "every member carries the one claim: {claims:#?}"
    );
    let (after, assertion) = authored("fins", &doc, claims[0]);
    let report = checks(&after);
    let quiet: Vec<_> = coincidences(&report)
        .iter()
        .map(|(f, _, _)| f.quiet)
        .collect();
    assert_eq!(
        quiet,
        vec![Some(assertion); 3],
        "the one assertion quiets each member's row: {report}"
    );
    assert_eq!(every_offered_assertion_quiets("fins", &doc), 3);
}

/// **A row between two members of one family carries no assertion**:
/// the one assertion that says it reads `c[k]` and `c[k + 1]`, two
/// indexed reads, which the rule matches no cell by. Fins a width apart
/// rest each against the next; those rows name the construction alone
/// and never refuse, while each fin's rest on the base still carries
/// its claim.
///
/// *Breaks if* a row no authored assertion can quiet is offered one, so
/// that it refuses at `Error` with a recourse nothing satisfies.
#[test]
fn a_row_between_two_members_carries_no_assertion() {
    let doc = fins(3, 0.25);
    let report = checks(&doc);
    let between: Vec<_> = coincidences(&report)
        .into_iter()
        .filter(|(f, _, _)| {
            matches!(f.subject, editor_core::FindingSubject::Copies { a, b } if a.var == b.var)
        })
        .collect();
    assert!(
        !between.is_empty(),
        "the premise, fins resting against each other: {report}"
    );
    for (found, _, quieting) in &between {
        assert!(
            quieting.is_none() && !refuses_at_error(&report, found),
            "two members' row carries no assertion and never refuses: {found}"
        );
    }
    assert_eq!(
        every_offered_assertion_quiets("fins a width apart", &doc),
        3,
        "each fin's rest on the base still carries its claim"
    );
}
