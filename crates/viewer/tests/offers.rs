//! **What a failure offers, through the session** (`viewer::recourse`):
//! a check's Commit is one click and one undo and quiets its finding; a
//! Lead seats the assertion tool with what the finding determines; a
//! run that is not the committed document's offers no document edit;
//! and every offer is filled from what the kernel minted, never from a
//! finding's raw cell names.
//!
//! The scenes are two 20 × 20 × 10 mm blocks along `x`, each placed into
//! the world once: touching, an at-rest coincidence of the two walls the
//! lint reports until an assertion says it; or overlapping, an
//! interference.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{
    AssertionRelation, CheckEvidence, CheckFinding, Doc, MeasureVerb, Node, ProfileProgram,
    RecipeNodeId, Speaker,
};
use pncad::geom_core::Tol;
use viewer::combine::AssertionTool;
use viewer::recourse::{Failure, Lead, Offer, REEVALUATE, offers};
use viewer::session::{AssertedClaim, AssertedSeat, DocSession, SessionOp};
use viewer::test_support::{
    CLEARANCE_SIDE as SIDE, finding_offers, inserted, len, placed, rectangle,
};

/// **Two blocks along `x`, the second's left wall at `x = at`**, each
/// placed into the world once: the session, landed.
pub(crate) fn blocks(tol: Tol, at: f64) -> DocSession {
    let mut doc = Doc::<ProfileProgram>::empty_derived("two-blocks", tol);
    for x in [0.0, at] {
        let (with, profile) = inserted(&doc, rectangle([x, 0.0], SIDE, SIDE), tol);
        let (with, block) = inserted(
            &with,
            Node::Extrude {
                profile: profile.into(),
                distance: len(0.01),
            },
            tol,
        );
        let (with, _) = placed(&with, block, tol);
        doc = with;
    }
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    session
}

/// The landed report's findings, with what each offers, while the run
/// is the committed document's.
fn offered(session: &DocSession) -> Vec<(CheckFinding, Vec<Offer>)> {
    let report = session.checks().expect("the registry ran");
    assert!(report.pending.is_empty(), "the inline door settled");
    let offers = finding_offers(session.current()).expect("the landed run is the committed one");
    report.findings.iter().cloned().zip(offers).collect()
}

/// Perform `op`, landing it: its outcome's node.
fn performed(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    outcome.made.expect("an assertion is a node")
}

/// The loud findings of `session` that `matches` picks, by the subject
/// and evidence kind they are about.
fn loud(session: &DocSession, matches: impl Fn(&CheckFinding) -> bool) -> Vec<CheckFinding> {
    session
        .checks()
        .expect("the registry ran")
        .findings
        .iter()
        .filter(|f| matches(f) && f.quiet.is_none())
        .cloned()
        .collect()
}

fn is_coincidence(f: &CheckFinding) -> bool {
    matches!(f.evidence, CheckEvidence::UnprovenCoincidence { .. })
}

fn is_interference(f: &CheckFinding) -> bool {
    matches!(f.evidence, CheckEvidence::Interference { .. })
}

/// **Every Commit a coincidence offers is the kernel's own assertion,
/// one click and one undo, and the finding goes quiet** — the shape of
/// the kernel's law (`every_offered_assertion_quiets`), run through the
/// viewer's door. For each loud coincidence that offers a Commit:
/// - the op is `AssertQuieting` over exactly the value the finding
///   carries, so no seat is built from its raw cell names (which are
///   keyed by the `⊔` that gathered them and would not resolve);
/// - performed, it is one history step, and the checks run again report
///   the row quiet under the assertion it made;
/// - one undo is the document it started from, and the row loud again.
///
/// The touching walls carry such a claim, so the loop is not vacuous.
///
/// Red if a Commit is filled from anything but the carried value, if it
/// takes two undos, or if it does not quiet its row.
#[test]
fn a_coincidences_commit_is_the_kernels_assertion_one_undo_and_quiets_it() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE);
    let mut checked = 0;
    for (finding, offers) in offered(&session) {
        let CheckEvidence::UnprovenCoincidence { quieting, .. } = &finding.evidence else {
            continue;
        };
        let commits: Vec<_> = offers
            .iter()
            .filter_map(|offer| match offer {
                Offer::Commit(commit) => Some(commit.op.clone()),
                Offer::Lead(_) | Offer::Unoffered(_) => None,
            })
            .collect();
        let Some(carried) = quieting.as_ref().filter(|_| finding.quiet.is_none()) else {
            assert!(
                commits.is_empty(),
                "a finding with no claim commits nothing"
            );
            continue;
        };
        assert_eq!(
            commits,
            [SessionOp::AssertQuieting {
                quieting: carried.clone()
            }],
            "the one Commit is the finding's own value"
        );
        let before = session.committed_doc().clone();
        let steps = session.history().len();
        let loud_before = loud(&session, is_coincidence).len();
        let made = performed(&mut session, commits[0].clone());
        assert_eq!(session.history().len(), steps + 1, "one action");
        assert!(
            session
                .checks()
                .expect("checked")
                .findings
                .iter()
                .any(|f| is_coincidence(f) && f.quiet == Some(made)),
            "the row is quiet under the assertion the Commit made"
        );
        assert_eq!(
            loud(&session, is_coincidence).len(),
            loud_before - 1,
            "and it alone went quiet"
        );
        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
        session.pump();
        assert!(session.committed_doc().bit_eq(&before), "one undo");
        assert_eq!(
            loud(&session, is_coincidence).len(),
            loud_before,
            "undone, the row is loud again"
        );
        checked += 1;
    }
    assert!(checked >= 1, "the walls' contact offers its claim");
}

/// **The Checks window says the coincidence once**: where its Commit is
/// drawn, the row's sentence leaves the assertion out of its recourse
/// and the control's label is the claim; where the run is not the
/// committed document's, no control is drawn and the sentence says the
/// assertion in words.
///
/// Red if the row prints the recourse it draws as a control, or drops
/// the words when it draws none.
#[test]
fn a_coincidence_drawn_with_its_commit_is_not_said_twice() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE);
    // The session's own report and landed document: a clone of the
    // report is another value, and `Current` offers on its own only.
    fn row(session: &DocSession) -> viewer::frame::CheckRow {
        let (landed, _) = session.landed_pair().expect("landed");
        viewer::frame::check_rows(
            session.checks().expect("checked"),
            landed,
            session.current(),
        )
        .into_iter()
        .find(|row| {
            row.sentence
                .to_string()
                .contains("rests on one carrier against")
        })
        .expect("the walls' contact has a row")
    }
    let drawn = row(&session);
    let [Offer::Commit(commit)] = &drawn.offers[..] else {
        panic!("the contact offers its claim: {:?}", drawn.offers)
    };
    assert!(
        commit.label.starts_with("Assert Gap("),
        "the control is the claim: {}",
        commit.label
    );
    assert!(
        !drawn.sentence.to_string().contains("assert Gap("),
        "the sentence beside the control does not say it again: {}",
        drawn.sentence
    );
    // A run outstanding: the row is the landed one's, said whole.
    let again = session.perform(SessionOp::Reevaluate);
    assert!(again.refusal.is_none(), "{:?}", again.refusal);
    assert!(session.current().is_none(), "the run is outstanding");
    let words = row(&session);
    assert!(words.offers.is_empty(), "an outstanding run offers nothing");
    assert!(
        words.sentence.to_string().contains("assert Gap("),
        "with no control, the assertion is said: {}",
        words.sentence
    );
    let others = "Recourse: build the cells as one construction";
    assert!(
        drawn.sentence.to_string().contains(others) && words.sentence.to_string().contains(others),
        "the acts no control takes are said either way: {}",
        drawn.sentence
    );
}

/// **A run that is not the committed document's offers no document
/// edit**: after an edit and before its run lands, no finding, product
/// fault or tree row offers anything, and the Checks rows draw no
/// control; re-evaluating is offered whatever the run. Once the run
/// lands, the offers are back, read off it.
///
/// Red if any offer is read off the older run.
#[test]
fn a_stale_run_offers_no_document_editing_op() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE);
    assert!(
        offered(&session)
            .iter()
            .any(|(_, offers)| !offers.is_empty()),
        "the premise: the landed run offers"
    );
    let commit = offered(&session)
        .into_iter()
        .find_map(|(_, offers)| match offers.into_iter().next()? {
            Offer::Commit(commit) => Some(commit.op),
            Offer::Lead(_) | Offer::Unoffered(_) => None,
        })
        .expect("a Commit");
    let outcome = session.perform(commit);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.current().is_none(), "the edit's run is outstanding");
    assert_eq!(finding_offers(session.current()), None, "no finding offers");
    assert!(
        viewer::test_support::product_commits(session.current()).is_empty(),
        "no product fault offers"
    );
    assert!(
        session.tree_rows().iter().all(|row| row.offers.is_empty()),
        "no tree row offers"
    );
    let (landed, _) = session.landed_pair().expect("the older run is still shown");
    let rows = viewer::frame::check_rows(
        session.checks().expect("its checks"),
        landed,
        session.current(),
    );
    assert!(
        rows.iter().all(|row| row.offers.is_empty()),
        "the Checks rows draw no control"
    );
    let [Offer::Commit(again)] = &offers(Failure::Canceled).drawn[..] else {
        panic!("a cancel offers re-evaluating")
    };
    assert_eq!(
        (again.label.as_str(), &again.op),
        (REEVALUATE, &SessionOp::Reevaluate)
    );
    session.pump();
    assert!(session.current().is_some(), "landed, the run is current");
    assert!(
        finding_offers(session.current()).is_some(),
        "and offers are read off it"
    );
}

/// **An interference's Lead seats the assertion tool with the pair the
/// kernel minted, and leaves the bound to the person**: the lead carries
/// exactly the finding's opposed pair; the tool it opens holds the
/// pair's two references, outer then inner, and commits a `Gap` over
/// them; with the bound the person types below the overlap's depth it
/// quiets the finding, in one undo.
///
/// Red if the lead is built from anything but the carried pair, the
/// tool seats anything else or in another order, or the authored `Gap`
/// does not quiet the overlap.
#[test]
fn an_interferences_lead_seats_its_pair_and_the_persons_bound_quiets_it() {
    let tol = Tol::witness();
    // The blocks overlap 2 mm along x.
    let mut session = blocks(tol, SIDE - 0.002);
    let leads: Vec<_> = offered(&session)
        .into_iter()
        .filter(|(finding, _)| is_interference(finding) && finding.quiet.is_none())
        .map(|(finding, offers)| {
            let CheckEvidence::Interference { finding } = &finding.evidence else {
                unreachable!("filtered to an interference")
            };
            let carried = finding
                .quieting
                .clone()
                .expect("the overlap carries its pair");
            let [Offer::Lead(Lead::AssertGap { pair, label })] = &offers[..] else {
                panic!("the overlap offers its lead alone: {offers:?}")
            };
            assert_eq!(**pair, carried, "the lead is the finding's own pair");
            assert!(
                label.starts_with("Assert a negative Gap between "),
                "{label}"
            );
            carried
        })
        .collect();
    let [pair] = &leads[..] else {
        panic!("one loud overlap: {leads:?}")
    };

    let tool = AssertionTool::led(pair);
    let by = Speaker::of(session.committed_doc());
    let line = tool.line(by);
    for reference in &pair.references {
        let name = reference
            .name()
            .unwrap_or_else(|| panic!("a face is a cell's reference: {reference:?}"));
        let said = pncad::document::sentence!(by.name(name)).to_string();
        assert!(line.contains(&said), "the tool says {said}: {line}");
    }
    let bound = len(-0.001);
    let op = tool
        .op(AssertedClaim::Value {
            measure: MeasureVerb::Gap,
            relation: AssertionRelation::AtMost,
            bound: bound.clone(),
        })
        .expect("both seats are held");
    let [outer, inner] = pair.references.clone();
    assert_eq!(
        op,
        SessionOp::AddAssertion {
            a: AssertedSeat::Minted(outer),
            b: AssertedSeat::Minted(inner),
            claim: AssertedClaim::Value {
                measure: MeasureVerb::Gap,
                relation: AssertionRelation::AtMost,
                bound,
            },
        },
        "the tool's seats are the pair's references, outer then inner"
    );
    let steps = session.history().len();
    let made = performed(&mut session, op);
    assert_eq!(session.history().len(), steps + 1, "one action");
    assert!(
        session
            .checks()
            .expect("checked")
            .findings
            .iter()
            .any(|f| is_interference(f) && f.quiet == Some(made)),
        "the person's Gap quiets the overlap"
    );
}

/// **The same Commit taken twice authors one assertion**: the door's own
/// check (`DocSession`'s `AssertQuieting` arm, `Quieting::held_in`)
/// refuses the second, naming the assertion that already says it, and
/// records nothing; one undo then takes back the one.
///
/// Red if the second take authors a second assertion, or refuses
/// anything but `ClaimHeld` naming the first.
#[test]
fn the_same_commit_taken_twice_authors_one_assertion() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE);
    let commit = offered(&session)
        .into_iter()
        .find_map(|(_, offers)| match offers.into_iter().next()? {
            Offer::Commit(commit) => Some(commit.op),
            Offer::Lead(_) | Offer::Unoffered(_) => None,
        })
        .expect("the walls' contact commits its claim");
    let steps = session.history().len();
    let first = session.perform(commit.clone());
    assert!(first.refusal.is_none(), "{:?}", first.refusal);
    let made = first.made.expect("an assertion");
    for when in ["before its run lands", "after"] {
        let again = session.perform(commit.clone());
        assert!(
            matches!(&again.refusal,
                Some(viewer::Refusal::ClaimHeld { assertion }) if assertion.id() == made),
            "{when}: the second take names the first: {:?}",
            again.refusal
        );
        assert!(again.committed.is_empty(), "{when}: and records nothing");
        session.pump();
    }
    assert_eq!(session.history().len(), steps + 1, "one assertion");
}

/// **A gesture's scratch preview offers nothing**: once the preview's
/// own run has landed the session is not busy, but that run is of a
/// document the history does not hold, so no finding offers an op.
///
/// Red if `DocSession::current` reads only the generations.
#[test]
fn a_previews_run_offers_no_document_editing_op() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE);
    let extrude = session
        .committed_doc()
        .ids()
        .into_iter()
        .find(|&id| matches!(session.committed_doc().node(id), Some(Node::Extrude { .. })))
        .expect("a block");
    let slot = pncad::document::SlotId::Distance;
    for op in [
        SessionOp::BeginGesture {
            node: extrude,
            slot,
        },
        SessionOp::PreviewGesture {
            node: extrude,
            slot,
            value: 0.012,
        },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    session.pump();
    assert!(!session.busy(), "the premise: the preview's run landed");
    assert!(
        session.current().is_none(),
        "a preview's run is not current"
    );
    assert_eq!(
        finding_offers(session.current()),
        None,
        "and nothing offers"
    );
}

/// **The interference row says its Gap once**: drawn with its Lead, the
/// sentence's recourse leaves the "assert a negative Gap" clause out and
/// keeps the other acts; with the run outstanding there is no control
/// and the clause is said.
///
/// Red if the interference row prints the act its Lead draws.
#[test]
fn an_interference_drawn_with_its_lead_is_not_said_twice() {
    let tol = Tol::witness();
    let mut session = blocks(tol, SIDE - 0.002);
    let row = |session: &DocSession| {
        let (landed, _) = session.landed_pair().expect("landed");
        viewer::frame::check_rows(
            session.checks().expect("checked"),
            landed,
            session.current(),
        )
        .into_iter()
        .find(|row| row.sentence.to_string().contains(" overlap"))
        .expect("the overlap has a row")
    };
    let clause = "assert a negative Gap between";
    let others = "Recourse: move the two apart";
    let drawn = row(&session);
    assert!(
        matches!(&drawn.offers[..], [Offer::Lead(Lead::AssertGap { .. })]),
        "{:?}",
        drawn.offers
    );
    let said = drawn.sentence.to_string();
    assert!(!said.contains(clause) && said.contains(others), "{said}");
    let again = session.perform(SessionOp::Reevaluate);
    assert!(again.refusal.is_none(), "{:?}", again.refusal);
    let words = row(&session);
    assert!(words.offers.is_empty());
    let said = words.sentence.to_string();
    assert!(said.contains(clause) && said.contains(others), "{said}");
}

/// **An unused node offers the product door only where it draws a
/// body**: an unused profile draws none and offers nothing; the unused
/// extrude made from it offers **Add to the product** on itself.
///
/// Red if the door is offered on a node the door would refuse.
#[test]
fn an_unused_node_offers_the_product_door_only_where_it_draws_a_body() {
    let tol = Tol::witness();
    let mut session = blocks(tol, 3.0 * SIDE);
    let profile = session.perform(SessionOp::AddProfile {
        loops: vec![
            pncad::document::LoopProgram::circle(-0.05, 0.0, 0.005).expect("a finite circle"),
        ],
    });
    assert!(profile.refusal.is_none(), "{:?}", profile.refusal);
    let profile = profile.made.expect("a profile");
    session.pump();
    let unused_of = |session: &DocSession, node: RecipeNodeId| {
        offered(session)
            .into_iter()
            .find(|(f, _)| {
                matches!(f.evidence, CheckEvidence::Unused { .. })
                    && f.subject == pncad::document::FindingSubject::Node(node)
            })
            .map(|(_, offers)| offers)
            .unwrap_or_else(|| panic!("{node:?} is reported unused"))
    };
    assert_eq!(unused_of(&session, profile), [], "a profile draws no body");
    let extrude = session.perform(SessionOp::AddExtrude {
        profile,
        distance: len(0.01),
    });
    assert!(extrude.refusal.is_none(), "{:?}", extrude.refusal);
    let extrude = extrude.made.expect("an extrude");
    session.pump();
    assert!(
        matches!(&unused_of(&session, extrude)[..], [Offer::Commit(commit)]
            if commit.op == SessionOp::AddToProduct { node: extrude, body: None }),
        "the extrude draws its body and offers the door"
    );
}
