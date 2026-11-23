//! **A finding carries the assertion that quiets it** (DESIGN
//! `[one-way.places]`, "A refusal's recourse"; D10 `[assertions.quiet]`).
//!
//! The law, both ways. Every loud unproven-coincidence finding that
//! carries an assertion ([`editor_core::Quieting`]) refuses at `Error`,
//! its edits apply, every member of the assertion they author holds, and
//! the checks run again report the same row quiet under it. And every
//! loud finding that carries none says no assertion, never refuses, and
//! is quieted by no assertion written in the rule's language that holds
//! at every member. The law is held over the corpus and over each
//! fixture of `atrest_operation_rows_quiet`, whose rows cover the kinds
//! of row the rule says.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, run};
use crate::corpus;
use crate::fixture::{self, insert};
use editor_core::{
    AssertionRelation, AssertionVerdict, BodyRead, CheckEvidence, CheckFinding, CheckId,
    ChecksConfig, Coord, Evaluation, Formula, MeasurePrimitive, MeasureVerb, NamedCell,
    NamedCoincidence, Node, Operand, ProfileDoc, Quieting, RecipeNodeId, RoleSeg, Severity,
    Speaker, StableName, TargetAxis, ValuePayload, VarKind, enforce_checks,
};
use geom_core::Tol;

/// `doc`'s checks at the default configuration.
pub(crate) fn checks(doc: &ProfileDoc) -> editor_core::ChecksReport {
    checks_of(doc, &run(doc))
}

/// `doc`'s checks at the default configuration, over `ev`.
pub(crate) fn checks_of(doc: &ProfileDoc, ev: &Evaluation<f64>) -> editor_core::ChecksReport {
    editor_core::run_checks(
        doc,
        ev,
        &ChecksConfig::default(),
        &editor_core::Stop::default(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the checks run: {e}"))
}

/// **Whether [`enforce_checks`] refuses `finding` at `Error`.** The
/// lint's knob is an `Advisory` and spells no `Error`, so the finding is
/// read alone under the knob of a check that does: what the refusing
/// path reads of a finding is its `quiet` and its evidence, not which
/// check it is.
pub(crate) fn refuses_at_error(report: &editor_core::ChecksReport, finding: &CheckFinding) -> bool {
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
) -> (ProfileDoc, RecipeNodeId) {
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
    let assertion = edits
        .minted
        .last()
        .copied()
        .flatten()
        .unwrap_or_else(|| panic!("{what}: the last edit mints the assertion"));
    assert_eq!(
        replayed.ids().last(),
        Some(&assertion),
        "{what}: the assertion is the last node authored"
    );
    (replayed, assertion)
}

/// **Each member's verdict of `assertion` in `ev`**, whether it holds:
/// one for an assertion reaching no index.
pub(crate) fn verdicts(ev: &Evaluation<f64>, assertion: RecipeNodeId) -> Vec<bool> {
    let holds = |payload: &ValuePayload<f64>| {
        matches!(
            payload,
            ValuePayload::Assertion(AssertionVerdict::Holds { .. })
        )
    };
    let Some(value) = ev.value(assertion) else {
        return vec![false];
    };
    match &value.members {
        Some(family) => family
            .members
            .iter()
            .map(|(_, m)| holds(&m.payload))
            .collect(),
        None => vec![holds(&value.payload)],
    }
}

/// **Every assertion over `row` written in the rule's language**
/// ([`editor_core::test_support::saying`]): each measure that says the
/// row, over its two cells in either order, each cell written as a
/// selection of one of the names a person could read it by (its own, the
/// arrival's without the `⊔`'s key, a family member's by the name it
/// wraps), read as an axis or a centre where the row is one of poses; a
/// tool as its read. What the completeness half of the law authors.
fn written_in_the_rules_language(row: &NamedCoincidence) -> Vec<(MeasureVerb, [Operand; 2])> {
    let Some((verbs, poses)) = editor_core::test_support::saying(row) else {
        return Vec::new();
    };
    let [a, b] = row.cells.as_slice() else {
        return Vec::new();
    };
    let operands = |cell: &NamedCell| -> Vec<Operand> {
        let (input, name) = match cell {
            NamedCell::Tool { input, .. } => return vec![Operand::Var(*input)],
            NamedCell::Entity { input, name } => (*input, name),
            _ => return Vec::new(),
        };
        let mut names: Vec<StableName> = vec![name.clone()];
        if let [RoleSeg::From { read, of }] = name.path.as_slice()
            && *read == input
        {
            names.push((**of).clone());
        }
        for held in names.clone() {
            if let [RoleSeg::Member { of, .. }] = held.path.as_slice() {
                names.push((**of).clone());
            }
        }
        names.dedup();
        let kinds: &[Option<VarKind>] = if poses {
            &[Some(VarKind::Axis), Some(VarKind::Point)]
        } else {
            &[None]
        };
        names
            .iter()
            .flat_map(|name| {
                kinds.iter().map(move |kind| {
                    let of = Operand::Select {
                        body: Box::new(BodyRead::plain(Operand::Var(input))),
                        names: vec![name.clone()],
                    };
                    match kind {
                        None => of,
                        Some(VarKind::Axis) => {
                            Operand::Pose(Box::new(editor_core::pose::PoseDef::Axis { of }))
                        }
                        Some(_) => {
                            Operand::Pose(Box::new(editor_core::pose::PoseDef::Point { of }))
                        }
                    }
                })
            })
            .collect()
    };
    let (xs, ys) = (operands(a), operands(b));
    let mut out = Vec::new();
    for &verb in verbs {
        for x in &xs {
            for y in &ys {
                out.push((verb, [x.clone(), y.clone()]));
                out.push((verb, [y.clone(), x.clone()]));
            }
        }
    }
    out
}

/// `verb(a, b) = 0` authored in `doc` as one action, a variable read as
/// it is named first where it is unnamed; `None` where a door refuses
/// it (a pose its cell does not have, a read the door does not take).
fn author_written(
    doc: &ProfileDoc,
    verb: MeasureVerb,
    [a, b]: [Operand; 2],
) -> Option<(ProfileDoc, RecipeNodeId)> {
    let mut action = editor_core::Recording::start(doc, Tol::witness());
    for operand in [&a, &b] {
        let Operand::Var(var) = operand else {
            continue;
        };
        let unnamed = doc.var_name(*var).is_none()
            && doc.var(*var).is_some_and(|v| v.def().output().is_none());
        if unnamed {
            let name = (1..)
                .map(|n| format!("written_{n}"))
                .find(|n| doc.var_named(n).is_none())?;
            action
                .apply(editor_core::DocEdit::RenameVar {
                    var: (*var).into(),
                    name: Some(editor_core::VarName::new(name).ok()?),
                })
                .ok()?;
        }
    }
    let primitive = match verb {
        MeasureVerb::Gap => MeasurePrimitive::Gap { outer: a, inner: b },
        MeasureVerb::Distance => MeasurePrimitive::Distance { a, b },
        MeasureVerb::Angle => MeasurePrimitive::Angle { a, b },
    };
    let measured = action.measure(&[primitive]).ok()?;
    let assertion = action
        .insert(Node::Assertion(editor_core::Claim::Value {
            value: Formula::var(measured.outputs[0], editor_core::Dimension::Length),
            bound: Formula::zero(editor_core::Dimension::Length),
            relation: AssertionRelation::Equal,
        }))
        .ok()?;
    Some((action.finish().ok()?.doc, assertion))
}

/// **The law on `doc`** (the module's head): with each distinct claim
/// authored, in finding order, then every assertion in the rule's
/// language over each loud row that carries none, the checks run once
/// more. One claim can be several rows' (a family's members), and then
/// one assertion quiets each. Answers how many rows it quieted.
pub(crate) fn every_offered_assertion_quiets(what: &str, doc: &ProfileDoc) -> usize {
    let report = checks(doc);
    let by = Speaker::of(doc);
    let mut after = doc.clone();
    let mut assertions: Vec<(&Quieting, RecipeNodeId)> = Vec::new();
    let mut offered = Vec::new();
    let mut unoffered = Vec::new();
    for (found, row, quieting) in coincidences(&report) {
        let said = editor_core::Said(found, by).sentence().to_string();
        if found.quiet.is_some() {
            assert!(
                !said.contains("Recourse"),
                "{what}: a quiet finding offers nothing: {said}"
            );
            continue;
        }
        let Some(quieting) = quieting else {
            assert!(
                !said.contains("assert") && !refuses_at_error(&report, found),
                "{what}: a finding carrying no assertion offers none and never refuses: {said}"
            );
            unoffered.push((found.subject, row));
            continue;
        };
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
    let mut written = 0;
    for (_, row) in &unoffered {
        for (verb, operands) in written_in_the_rules_language(row) {
            if let Some((next, _)) = author_written(&after, verb, operands) {
                after = next;
                written += 1;
            }
        }
    }
    if offered.is_empty() && written == 0 {
        return 0;
    }
    let ev = run(&after);
    let report = checks_of(&after, &ev);
    let by = Speaker::of(&after);
    let now = coincidences(&report);
    // A family's operation reports a row once per member, each the same
    // row: every one is held to the law.
    let found_now = |subject, row: &NamedCoincidence| {
        let rows: Vec<&CheckFinding> = now
            .iter()
            .filter(|(f, r, _)| f.subject == subject && *r == row)
            .map(|(f, _, _)| *f)
            .collect();
        assert!(
            !rows.is_empty(),
            "{what}: the row is still reported: {row:?}\n{report}"
        );
        rows
    };
    for (quieting, assertion) in &assertions {
        let held = verdicts(&ev, *assertion);
        assert!(
            held.iter().all(|&holds| holds),
            "{what}: every member of an offered assertion holds, {}: {held:?}",
            quieting.said(by)
        );
    }
    for (subject, row, claim, assertion) in &offered {
        for found in found_now(*subject, row) {
            assert_eq!(
                found.quiet,
                Some(*assertion),
                "{what}: the row is quiet under the assertion authored for {claim}: {found}"
            );
            let named = editor_core::sentence!(by.node_as(*assertion, "assertion")).to_string();
            let said = editor_core::Said(found, by).sentence().to_string();
            assert!(
                said.contains(&format!("quiet: {named} says so")),
                "{what}: the quiet row names the new assertion {named}: {said}"
            );
        }
    }
    for (subject, row) in &unoffered {
        for found in found_now(*subject, row) {
            if let Some(quiet) = found.quiet {
                assert!(
                    !verdicts(&ev, quiet).iter().all(|&holds| holds),
                    "{what}: a row carrying no assertion is quieted by {}, which holds at every \
                 member: {found}",
                    editor_core::sentence!(by.node_as(quiet, "assertion"))
                );
            }
        }
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

/// `body` placed `n` times by one placement whose slide reads
/// `k = index(n)`: `pitch · k` along x, and `z + rise · k` up.
pub(crate) fn family(
    doc: ProfileDoc,
    body: RecipeNodeId,
    k: editor_core::VarId,
    pitch: f64,
    (z, rise): (f64, f64),
) -> (ProfileDoc, RecipeNodeId) {
    let along = |by: f64| Formula::mul(fixture::scalar_of(k), fixture::len(by)).unwrap();
    let up = Formula::add(fixture::len(z), along(rise)).unwrap();
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::slide(TargetAxis::X), along(pitch)),
            (Coord::slide(TargetAxis::Z), up),
        ],
    );
    let node = fixture::place_node(
        &doc,
        editor_core::BodyRead::plain(Operand::Node(body)),
        placed.bundle(),
    );
    insert(doc, node)
}

/// `n` fins standing over a unit-high base, one placement whose slide
/// reads `k = index(n)`, `pitch` apart, the fin at `k` lifted
/// `z + rise · k`: at `(1, 0)` each fin rests on the base's top, at a
/// pitch of the fin's width each rests against the next, and under it
/// they overlap. `(doc, k, fins)`, the fins' placement.
pub(crate) fn fins_at(
    n: i64,
    pitch: f64,
    (z, rise): (f64, f64),
) -> (ProfileDoc, editor_core::VarId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("offer-fins", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", n);
    let (doc, base) = block(doc, (0.0, 13.0), (0.0, 2.0), 1.0);
    let (doc, fin) = block(doc, (0.25, 0.5), (0.5, 1.5), 1.25);
    let (doc, _) = fixture::place(doc, base);
    let (doc, fins) = family(doc, fin, k, pitch, (z, rise));
    (doc, k, fins)
}

/// [`fins_at`], at rest on the base and assembled.
pub(crate) fn fins(n: i64, pitch: f64) -> ProfileDoc {
    let (doc, _, _) = fins_at(n, pitch, (1.0, 0.0));
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
    assert_eq!(
        verdicts(&run(&after), assertion),
        vec![true; 3],
        "the claim holds at every member"
    );
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

/// **A family only some of whose members rest carries no assertion.**
/// Fins climbing a stair, each lifted off the last, only the first
/// resting on the base: the one assertion that would say the first's
/// rest reaches the index, and it is violated at every other member, so
/// it is no recourse. The row names the construction alone and never
/// refuses; and no assertion holding at every member quiets it (the
/// law's completeness half).
///
/// *Breaks if* the mint offers a family assertion it read only at the
/// row's own member.
#[test]
fn a_partly_resting_family_carries_no_assertion() {
    for (pitch, rise) in [(2.0, 0.5), (0.75, 0.1)] {
        let (doc, _, _) = fins_at(3, pitch, (1.0, rise));
        let doc = editor_core::test_support::assemble_world_copies(doc, Tol::witness());
        let what = format!("fins rising {rise} a pitch of {pitch}");
        let report = checks(&doc);
        let rests = coincidences(&report);
        assert_eq!(
            rests.len(),
            1,
            "{what}: the first fin alone rests: {report}"
        );
        let (found, _, quieting) = rests[0];
        assert!(
            quieting.is_none() && !refuses_at_error(&report, found),
            "{what}: a claim violated at other members is none, and never refuses: {found}"
        );
        assert_eq!(every_offered_assertion_quiets(&what, &doc), 0, "{what}");
    }
}

/// **Two families on one index: a cap on each fin carries the one
/// assertion over both family reads.** Caps stacked on fins, both
/// placements reading `k`: the cap at `k` rests on the fin at `k`, and
/// `Gap(the fin's top off the fins, the cap's foot off the caps) = 0`
/// is one assertion per member, each saying its own pair. Each row
/// carries it, refuses at `Error` while loud, and is quiet once it is
/// authored. Held over the two families assembled as copies and over a
/// plain union of the two family reads.
///
/// *Breaks if* the mint refuses a claim whose two reads both reach the
/// index the rule reads them at.
#[test]
fn two_families_on_one_index_carry_the_assertion_over_both() {
    for union in [false, true] {
        let (doc, k, fins) = fins_at(3, 0.75, (1.0, 0.0));
        let (doc, cap) = block(doc, (0.25, 0.5), (0.5, 1.5), 0.5);
        let (doc, caps) = family(doc, cap, k, 0.75, (2.25, 0.0));
        let (doc, what) = if union {
            let (doc, _) = fixture::union_over(doc, &[fins, caps]);
            (doc, "a union of two families")
        } else {
            (
                editor_core::test_support::assemble_world_copies(doc, Tol::witness()),
                "two families assembled",
            )
        };
        let report = checks(&doc);
        let caps_rows: Vec<_> = coincidences(&report)
            .into_iter()
            .filter(|(_, row, _)| {
                row.cells
                    .iter()
                    .all(|c| matches!(c, NamedCell::Entity { name, .. } if format!("{name:?}").contains("Member")))
            })
            .collect();
        assert!(
            caps_rows.len() >= 3,
            "{what}: the premise, a cap resting on each fin: {report}"
        );
        for (found, _, quieting) in &caps_rows {
            assert!(
                quieting.is_some() && refuses_at_error(&report, found),
                "{what}: a row between two families' members carries its claim: {found}"
            );
        }
        assert!(
            every_offered_assertion_quiets(what, &doc) >= 3,
            "{what}: the caps' claims quiet their rows"
        );
    }
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

/// **The construction recourse is one sentence, a word apart**, said
/// with no claim and with one.
///
/// *Breaks if* the sentence's text drifts, a run of spaces among it.
#[test]
fn the_construction_recourse_reads_as_one_sentence() {
    use editor_core::coincide::Recourse;
    let alone = "Recourse: build the cells as one construction of the same variables, so the \
                 coincidence holds whatever their values";
    assert_eq!(Recourse::OneConstruction.words(None, Speaker::TAG), alone);
}

/// **The law over three more family shapes**: fins resting on two
/// blocks of different heights, each member on its own (a family claim
/// over either block fails at the other member); three families stacked
/// on one index, assembled and as one union; and two families on
/// unrelated indices. Each offered claim holds at every member and
/// quiets its rows, and no row offered none is quieted by an assertion
/// in the rule's language holding at every member.
///
/// *Breaks if* the mint and the rule part over a family whose members
/// rest on different cells, more than two families, or indices neither
/// within the other.
#[test]
fn the_law_holds_over_steps_stacks_and_unrelated_indices() {
    let steps = {
        let doc = ProfileDoc::empty_derived("offer-steps", Tol::witness());
        let (doc, k) = fixture::index(doc, "k", 2);
        let (doc, low) = block(doc, (0.0, 1.5), (0.0, 2.0), 1.0);
        let (doc, high) = block(doc, (1.5, 6.0), (0.0, 2.0), 1.5);
        let (doc, fin) = block(doc, (0.25, 0.5), (0.5, 1.5), 1.25);
        let (doc, _) = fixture::place(doc, low);
        let (doc, _) = fixture::place(doc, high);
        let (doc, _) = family(doc, fin, k, 2.0, (1.0, 0.5));
        editor_core::test_support::assemble_world_copies(doc, Tol::witness())
    };
    let stacked = |union: bool| {
        let (doc, k, fins) = fins_at(3, 0.75, (1.0, 0.0));
        let (doc, cap) = block(doc, (0.25, 0.5), (0.5, 1.5), 0.5);
        let (doc, caps) = family(doc, cap, k, 0.75, (2.25, 0.0));
        let (doc, hat) = block(doc, (0.25, 0.5), (0.5, 1.5), 0.25);
        let (doc, hats) = family(doc, hat, k, 0.75, (2.75, 0.0));
        if union {
            fixture::union_over(doc, &[fins, caps, hats]).0
        } else {
            editor_core::test_support::assemble_world_copies(doc, Tol::witness())
        }
    };
    let unrelated = {
        let (doc, _, _) = fins_at(3, 0.75, (1.0, 0.0));
        let (doc, j) = fixture::index(doc, "j", 3);
        let (doc, cap) = block(doc, (0.25, 0.5), (0.5, 1.5), 0.5);
        let (doc, _) = family(doc, cap, j, 0.75, (2.25, 0.0));
        editor_core::test_support::assemble_world_copies(doc, Tol::witness())
    };
    for (what, doc) in [
        ("fins on two heights", steps),
        ("three families assembled", stacked(false)),
        ("three families unioned", stacked(true)),
        ("two families on unrelated indices", unrelated),
    ] {
        let offered = every_offered_assertion_quiets(what, &doc);
        assert!(offered > 0, "{what}: offers a claim");
    }
}
