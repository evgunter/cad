//! **The viewer's clearance consumer**: a landing whose `f64` run holds
//! a clearance requirement owes the certifying scalar's answer, taken at
//! rest on its own seam and settled into the landing; the row reads the
//! requirement as an equation and that answer beside it; the witness
//! pair is marked from the run the picture is drawn from; and the
//! create door writes each shape of bound.
//!
//! The scene is two 20 × 20 × 10 mm blocks side by side along `x`,
//! 10 mm apart (`test_support::two_blocks_apart`), so every verdict has
//! a number worked out by hand.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::cell::RefCell;
use std::rc::Rc;

use pncad::analysis::{ClearanceOutcome, clearances_at_nominal};
use pncad::document::{CancelToken, ClearanceBounds, Formula, Node, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use viewer::evalseam::{CertifyDone, CertifyRequest, CertifyService};
use viewer::generation::Generation;
use viewer::props::Notation;
use viewer::session::{ClearedSeat, DocSession, SessionOp};
use viewer::test_support::{CLEARANCE_GAP as GAP, CLEARANCE_SIDE as SIDE, len, two_blocks_apart};
use viewer::tree::{Certified, Readout, TreeRow};

/// Millimetres, so every bound reads as written.
const IN_MM: Notation = Notation {
    length: MM,
    ..Notation::DEFAULT
};

/// **The scene, landed** ([`two_blocks_apart`]) under `certifier`,
/// answering the session and each block's extrude.
fn scene(
    tol: Tol,
    certifier: Option<Box<dyn CertifyService>>,
) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (doc, [left, right], _) = two_blocks_apart(tol);
    let session = DocSession::inline(doc, tol);
    let mut session = match certifier {
        Some(certifier) => session.with_certifier(certifier),
        None => session,
    };
    session.pump();
    (session, left, right)
}

/// The world placement that draws `block`.
fn drawn(session: &DocSession, block: RecipeNodeId) -> RecipeNodeId {
    let [placement] = viewer::world::placements_of(session.committed_doc(), block)[..] else {
        panic!("each block is placed once");
    };
    placement
}

/// `clearance(left, right)` within `bounds`, both seats bodies.
fn requirement(
    left: RecipeNodeId,
    right: RecipeNodeId,
    bounds: ClearanceBounds<Formula>,
) -> SessionOp {
    SessionOp::AddClearance {
        a: ClearedSeat::Body(left.into()),
        b: ClearedSeat::Body(right.into()),
        bounds,
    }
}

/// Insert `op`, answering the node it made.
fn add(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    outcome.made.expect("the requirement is a node")
}

/// What the recording seam saw.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Seen {
    Submit(Generation),
    Cancel,
}

/// **A certify seam the row drives by hand**: it records every submit
/// and cancel, holds the last request, and answers only what the row
/// queues.
#[derive(Default, Clone)]
struct Recording {
    seen: Rc<RefCell<Vec<Seen>>>,
    asked: Rc<RefCell<Option<CertifyRequest>>>,
    ready: Rc<RefCell<Vec<CertifyDone>>>,
}

impl CertifyService for Recording {
    fn submit(&mut self, request: CertifyRequest) {
        self.seen
            .borrow_mut()
            .push(Seen::Submit(request.generation));
        *self.asked.borrow_mut() = Some(request);
    }

    fn cancel(&mut self) {
        self.seen.borrow_mut().push(Seen::Cancel);
    }

    fn poll(&mut self) -> Option<CertifyDone> {
        self.ready.borrow_mut().pop()
    }

    fn busy(&self) -> bool {
        !self.ready.borrow().is_empty()
    }
}

impl Recording {
    /// The answer the certifying scalar gives the request in hand, at
    /// the request's own tolerance.
    fn answer(&self) -> CertifyDone {
        let request = self
            .asked
            .borrow()
            .clone()
            .expect("a request was submitted");
        CertifyDone {
            generation: request.generation,
            answers: clearances_at_nominal(
                &request.doc,
                request.resolver,
                &CancelToken::new(),
                request.tol,
            ),
        }
    }

    fn submits(&self) -> Vec<Generation> {
        self.seen
            .borrow()
            .iter()
            .filter_map(|seen| match seen {
                Seen::Submit(generation) => Some(*generation),
                Seen::Cancel => None,
            })
            .collect()
    }
}

/// The clearance row `id` reads, out of the session's tree.
fn cleared(session: &DocSession, id: RecipeNodeId) -> viewer::tree::Cleared {
    let rows: Vec<TreeRow> = session.tree_rows();
    let row = rows
        .into_iter()
        .find(|row| row.id == id)
        .expect("the requirement has a row");
    match row.readout {
        Some(Readout::Clearance(cleared)) => cleared,
        other => panic!("a clearance row reads as one: {other:?} ({:?})", row.status),
    }
}

/// **The seam is asked exactly when a landing holds a clearance**: a
/// document of two blocks lands and asks nothing; the requirement's
/// landing asks once, under its own generation, and its row reads
/// certifying until the answer comes.
///
/// Red if `land` submits for every landing, or for none.
#[test]
fn a_landing_asks_the_certify_seam_only_when_it_holds_a_clearance() {
    let tol = Tol::witness();
    let seam = Recording::default();
    let (mut session, left, right) = scene(tol, Some(Box::new(seam.clone())));
    assert!(
        seam.submits().is_empty(),
        "a landing with no clearance owes no certified answer: {:?}",
        seam.seen.borrow()
    );
    let id = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.005))),
    );
    session.pump();
    assert_eq!(
        seam.submits(),
        vec![session.generation()],
        "the requirement's landing asks once, under its own generation"
    );
    assert_eq!(cleared(&session, id).answer, Certified::Certifying);
    assert_eq!(cleared(&session, id).state(), viewer::tree::CERTIFYING);
}

/// **A newer request cancels the older job, and its answer is dropped**:
/// the edit that replaces a landing cancels the seam, the landing's row
/// says certifying stopped, the new landing's reads certifying, and an
/// answer for the replaced landing settles nothing — until the new
/// landing's own answer lands.
///
/// Red if the next request leaves the job running, if the stopped row
/// still reads certifying, or if an answer for an older generation
/// settles into the newer landing.
#[test]
fn a_newer_request_cancels_the_certify_job_and_its_late_answer_settles_nothing() {
    let tol = Tol::witness();
    let seam = Recording::default();
    let (mut session, left, right) = scene(tol, Some(Box::new(seam.clone())));
    let id = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.005))),
    );
    session.pump();
    let stale = seam.answer();
    seam.seen.borrow_mut().clear();
    // Any edit replaces the landing: a label on the requirement.
    let outcome = session.perform(SessionOp::SetLabel {
        node: id,
        label: Some(pncad::document::Label::new("gap").expect("a label")),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        seam.seen.borrow().first(),
        Some(&Seen::Cancel),
        "the request that replaces the landing cancels its job first"
    );
    assert_eq!(
        cleared(&session, id).answer,
        Certified::Stopped,
        "the landing still drawn says nothing is certifying it"
    );
    session.pump();
    assert_eq!(cleared(&session, id).answer, Certified::Certifying);
    assert_ne!(stale.generation, session.generation(), "the premise");
    seam.ready.borrow_mut().push(stale);
    session.pump();
    assert_eq!(
        cleared(&session, id).answer,
        Certified::Certifying,
        "an answer for the landing replaced settles nothing"
    );
    let fresh = seam.answer();
    seam.ready.borrow_mut().push(fresh);
    session.pump();
    assert_eq!(
        cleared(&session, id).answer,
        Certified::Answered(ClearanceOutcome::Holds),
        "the landing's own answer settles"
    );
}

/// **A cancelled evaluation leaves its row saying certifying stopped**,
/// never `certifying…`: the request that replaced the landing stopped
/// its certify job, and the cancel means no landing comes to certify
/// again — so the row says what is true, and how to resume.
///
/// Red if `request_eval` leaves the stopped landing reading certifying.
#[test]
fn a_cancelled_evaluation_leaves_its_row_saying_certifying_stopped() {
    let tol = Tol::witness();
    let seam = Recording::default();
    let (mut session, left, right) = scene(tol, Some(Box::new(seam.clone())));
    let id = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.005))),
    );
    session.pump();
    let outcome = session.perform(SessionOp::SetLabel {
        node: id,
        label: Some(pncad::document::Label::new("gap").expect("a label")),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let cancelled = session.perform(SessionOp::CancelEvaluation);
    assert!(cancelled.refusal.is_none(), "{:?}", cancelled.refusal);
    session.pump();
    assert!(!session.certifying(), "the premise: nothing is certifying");
    let row = cleared(&session, id);
    assert_eq!(row.state(), "not certified");
    assert_eq!(row.sentence(), Some(viewer::tree::CERTIFY_STOPPED));
}

/// **Each verdict reads as its equation**, the seats in the document's
/// own words — each block as the document speaks it, by its label — and
/// the bound in the working notation, then the certified state and the
/// pair that decided it, worked out from the gap:
///
/// - `≥ 5 mm` holds: every pair is 10 mm apart or more;
/// - `≥ 20 mm` is violated by a pair 10 mm apart;
/// - `≤ 20 mm` holds, proved by a pair 10 mm apart;
/// - `≤ 5 mm` is exceeded: every pair is farther apart;
/// - `= 20 mm` is violated, its lower end, by a pair 10 mm apart;
/// - the band `5 mm ≤ … ≤ 20 mm` holds, proved by a pair 10 mm apart.
///
/// Red if a row reads a raw read id, re-spells the relation, drops the
/// pair's distance, or reads the `f64` third state once certified.
#[test]
fn each_certified_verdict_reads_as_its_equation_in_the_documents_words() {
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let said = |id| session.doc().spoken(id).to_string();
    let (l, r) = (said(left), said(right));
    assert!(l.contains("\"left\""), "the premise: {l}");
    let cases = [
        (
            ClearanceBounds::AtLeast(len(0.005)),
            format!("clearance({l}, {r}) ≥ 5 mm"),
            "Holds",
            None,
        ),
        (
            ClearanceBounds::AtLeast(len(0.02)),
            format!("clearance({l}, {r}) ≥ 20 mm"),
            "Violated",
            Some("a violating pair 10 mm apart"),
        ),
        (
            ClearanceBounds::AtMost(len(0.02)),
            format!("clearance({l}, {r}) ≤ 20 mm"),
            "Within",
            Some("a pair 10 mm apart lies within it"),
        ),
        (
            ClearanceBounds::AtMost(len(0.005)),
            format!("clearance({l}, {r}) ≤ 5 mm"),
            "Exceeded",
            Some("every pair is farther apart than the upper bound"),
        ),
        (
            ClearanceBounds::Equal(len(0.02)),
            format!("clearance({l}, {r}) = 20 mm"),
            "Violated",
            Some("a violating pair 10 mm apart"),
        ),
        (
            ClearanceBounds::Within {
                lo: len(0.005),
                hi: len(0.02),
            },
            format!("5 mm ≤ clearance({l}, {r}) ≤ 20 mm"),
            "Within",
            Some("a pair 10 mm apart lies within it"),
        ),
    ];
    let ids: Vec<RecipeNodeId> = cases
        .iter()
        .map(|(bounds, ..)| add(&mut session, requirement(left, right, bounds.clone())))
        .collect();
    session.pump();
    for (id, (_, equation, state, detail)) in ids.into_iter().zip(cases) {
        let row = cleared(&session, id);
        assert_eq!(row.equation(IN_MM), equation);
        assert_eq!(row.state(), state, "{equation}: {:?}", row.answer);
        assert_eq!(row.detail(IN_MM).as_deref(), detail, "{equation}");
        assert_eq!(
            row.sentence(),
            None,
            "{equation}: a decided verdict has no refusal line"
        );
    }
}

/// **A face seat reads as its face**, said by the document: the left
/// block's wall facing the right one, `clearance(<that face>, <right>)`.
///
/// Red if a face seat reads as its selection's raw id.
#[test]
fn a_face_seat_reads_as_the_face_the_document_names() {
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let face = facing_wall(&session, left);
    let id = add(
        &mut session,
        SessionOp::AddClearance {
            a: ClearedSeat::Face(face.clone()),
            b: ClearedSeat::Body(right.into()),
            bounds: ClearanceBounds::AtLeast(len(0.005)),
        },
    );
    session.pump();
    let row = cleared(&session, id);
    let doc = session.doc();
    let wall = pncad::document::Speaker::of(doc)
        .name(&face.name)
        .to_string();
    assert_eq!(
        row.equation(IN_MM),
        format!("clearance({wall}, {}) ≥ 5 mm", doc.spoken(right)),
    );
    assert_eq!(row.state(), "Holds", "{:?}", row.answer);
}

/// The left block's wall at `x = SIDE`, the one facing the right block,
/// as the picture draws it: on the block's world copy.
fn facing_wall(session: &DocSession, block: RecipeNodeId) -> viewer::session::FaceSelection {
    let (_, eval) = session.landed_pair().expect("the scene landed");
    let copy = drawn(session, block);
    let sel = pncad::select::Selector::of(
        pncad::select::NamePat::of_kind(pncad::select::EntityKind::Face)
            .seg(pncad::select::SegPat::tag(pncad::select::SegTag::Lateral)),
    );
    pncad::select::select(eval, copy, &sel)
        .into_iter()
        .find(|name| {
            pncad::select::face_frame(eval, copy, name)
                .is_ok_and(|pose| (pose.origin.x - SIDE).abs() < 1e-9 && pose.axis.x.abs() > 0.5)
        })
        .map(|name| viewer::session::FaceSelection {
            name,
            node: copy,
            body: 0,
        })
        .expect("the block has a wall at x = SIDE")
}

/// **A refusal reads as its sentence and its recourse**, under the
/// equation: a band whose lower end is above its upper is the kernel's
/// `EmptyBand`, refused at evaluation — the form writes it as typed —
/// and a negative bound its `NotADistance`.
///
/// Red if the row drops the refusal's sentence, or re-checks the bound
/// itself rather than reading the kernel's refusal.
#[test]
fn a_refused_bound_reads_as_the_kernels_sentence_and_recourse() {
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let reversed = add(
        &mut session,
        requirement(
            left,
            right,
            ClearanceBounds::Within {
                lo: len(0.02),
                hi: len(0.005),
            },
        ),
    );
    let negative = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(-0.001))),
    );
    session.pump();
    for (id, name) in [(reversed, "empty_band"), (negative, "not_a_distance")] {
        let row = cleared(&session, id);
        let Certified::Answered(ClearanceOutcome::Refused {
            name: got,
            sentence,
        }) = &row.answer
        else {
            panic!("{name}: the kernel refuses the bound: {:?}", row.answer);
        };
        assert_eq!(*got, name);
        assert_eq!(row.state(), "Refused");
        assert_eq!(
            row.sentence(),
            Some(sentence.as_str()),
            "the sentence is drawn as said"
        );
        assert!(
            sentence.contains("Recourse:"),
            "{name} says what to do: {sentence}"
        );
    }
}

/// **The witness is marked only from the picture's own run**: the
/// violated requirement's pair is offered while the landing it was
/// certified for is the run names are judged by, and withdrawn the
/// moment the shown document moves past it — a verdict of an older
/// document is never drawn over a newer one.
///
/// The pair's faces are named in the picture's run, on the copies that
/// draw the two blocks, and its ends span the gap.
///
/// Red if `witness` answers off a landing that no longer answers the
/// shown document, or names a face other than the facing walls.
#[test]
fn the_witness_is_offered_only_for_the_shown_documents_run() {
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let violated = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.02))),
    );
    let holds = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.005))),
    );
    session.pump();
    assert!(
        session.witness(holds).is_none(),
        "a pass has no pair to draw"
    );
    let witnessed = session.witness(violated).expect("a violation has its pair");
    let [a, b] = &witnessed.faces;
    assert_eq!(
        (a.node, b.node),
        (drawn(&session, left), drawn(&session, right)),
        "each face on the copy that draws its seat's body"
    );
    let (_, eval) = session.landed_pair().expect("landed");
    for face in [a, b] {
        assert!(
            pncad::select::face_frame(eval, face.node, &face.name).is_ok(),
            "{face:?} is a face the picture's run reads on its copy"
        );
    }
    // The copies stand where the blocks were built, so the pair's own
    // frame is the world's: one end on the left block's facing wall,
    // the other on the right block's.
    let [p, q] = witnessed.points;
    assert!(
        (p.x - SIDE).abs() < 1e-9 && (q.x - (SIDE + GAP)).abs() < 1e-9,
        "the pair spans the gap between the facing walls: {p:?} {q:?}"
    );
    // The next edit's request: the shown document moves past the
    // landing, and its pair is withdrawn until the new one is certified.
    let outcome = session.perform(SessionOp::SetLabel {
        node: holds,
        label: Some(pncad::document::Label::new("loose").expect("a label")),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        session.witness(violated).is_none(),
        "no pair is drawn over a document its run does not answer"
    );
    session.pump();
    assert!(
        session.witness(violated).is_some(),
        "the new landing's pair"
    );
}

/// **The create door writes each shape of bound**, through the
/// document's own insert door: `≥`, `≤`, `=` and the band, each the
/// clearance tool's choice, every end at the one length it is given; a
/// band's upper end is then moved at its own row.
///
/// Red if a choice writes another shape, or the band's ends swap.
#[test]
fn the_clearance_tool_writes_each_shape_of_bound() {
    use viewer::combine::{ClearanceBoundChoice, ClearanceTool};
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let wall = facing_wall(&session, left);
    let mut tool = ClearanceTool::new();
    tool.seat_face(session.committed_doc(), wall.clone());
    assert!(tool.seat_body(session.committed_doc(), right).is_none());
    for (choice, _) in ClearanceBoundChoice::ALL {
        let op = tool
            .op(choice.bounds(len(0.005)))
            .expect("both seats are held");
        let id = add(&mut session, op);
        // A band's upper end moved apart at its own row, so ends that
        // swapped would read back swapped.
        if choice == ClearanceBoundChoice::Between {
            let moved = session.perform(SessionOp::SetSlotExpression {
                node: id,
                slot: pncad::document::SlotId::ClearanceBound(pncad::document::BoundEnd::High),
                text: "20 mm".to_owned(),
            });
            assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
        }
        let doc = session.committed_doc();
        let Some(Node::Assertion(pncad::document::Claim::Clearance { a, b, bounds })) =
            doc.node(id)
        else {
            panic!("{choice:?} writes a clearance assertion");
        };
        let value = |var| match viewer::props::variable_value(doc, var) {
            Some(viewer::props::SlotValue::Continuous(v)) => v,
            other => panic!("a bound is a length: {other:?}"),
        };
        let written = bounds
            .try_map(|&var| Ok::<_, ()>(value(var)))
            .expect("lengths");
        let want = match choice {
            ClearanceBoundChoice::AtLeast => ClearanceBounds::AtLeast(0.005),
            ClearanceBoundChoice::AtMost => ClearanceBounds::AtMost(0.005),
            ClearanceBoundChoice::Equal => ClearanceBounds::Equal(0.005),
            ClearanceBoundChoice::Between => ClearanceBounds::Within {
                lo: 0.005,
                hi: 0.02,
            },
        };
        assert_eq!(written, want, "{choice:?}");
        assert_eq!(
            doc.selection(*a).map(|s| s.names.clone()),
            Some(vec![wall.name.clone()]),
            "{choice:?}: the first seat is the picked face"
        );
        assert!(
            doc.selection(*b).is_none(),
            "{choice:?}: the second seat is the body"
        );
    }
}

/// **A clearance's bounds are its rows**: the tool inserts contact, and
/// each end of the band is then a slot row of the requirement
/// (`SlotId::ClearanceBound`), which takes a formula. A formula over a
/// variable moves the verdict when the variable moves, and the edits
/// that wrote it save and load back to the same document.
///
/// Red if a bound is no row, if a row refuses a formula or does not
/// follow its variable, or if the slot address does not round-trip
/// through the file's edit log.
#[test]
fn a_clearances_bounds_are_rows_that_take_a_formula_and_round_trip() {
    use pncad::document::{BoundEnd, SlotId};
    use viewer::combine::ClearanceBoundChoice;
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let slots = |session: &DocSession, id| {
        viewer::props::slot_rows(session.committed_doc(), id)
            .into_iter()
            .map(|row| row.slot)
            .collect::<Vec<_>>()
    };
    let (low, high) = (
        SlotId::ClearanceBound(BoundEnd::Low),
        SlotId::ClearanceBound(BoundEnd::High),
    );
    for (choice, want) in [
        (ClearanceBoundChoice::AtLeast, vec![low]),
        (ClearanceBoundChoice::AtMost, vec![high]),
        (ClearanceBoundChoice::Equal, vec![low]),
        (ClearanceBoundChoice::Between, vec![low, high]),
    ] {
        let id = add(
            &mut session,
            requirement(left, right, choice.bounds(len(0.0))),
        );
        assert_eq!(slots(&session, id), want, "{choice:?}: one row per end");
    }

    let id = add(
        &mut session,
        requirement(left, right, ClearanceBoundChoice::AtLeast.bounds(len(0.0))),
    );
    session.pump();
    assert_eq!(cleared(&session, id).state(), "Holds", "contact holds");
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::new("gap").expect("a name"),
        value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Length, 0.005),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let written = session.perform(SessionOp::SetSlotExpression {
        node: id,
        slot: low,
        text: "gap".to_owned(),
    });
    assert!(written.refusal.is_none(), "{:?}", written.refusal);
    session.pump();
    assert_eq!(
        cleared(&session, id).state(),
        "Holds",
        "10 mm apart is at least 5 mm"
    );
    let gap = session.committed_doc().var_named("gap").expect("declared");
    let moved = session.perform(SessionOp::SetVariableText {
        var: gap,
        text: "20 mm".to_owned(),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    session.pump();
    assert_eq!(
        cleared(&session, id).state(),
        "Violated",
        "10 mm apart is not at least 20 mm"
    );

    let history = session.history();
    let edits: Vec<_> = history
        .path()
        .into_iter()
        .flat_map(|step| history.entry(step).edits().to_vec())
        .collect();
    let text = pncad::document::save(history.entry(history.root()).doc(), &edits, tol)
        .expect("a file carrying the log saves");
    assert!(
        text.contains("ClearanceBound"),
        "the log carries the bound's slot address"
    );
    let loaded = pncad::document::load(&text, tol).expect("and loads, replaying the log");
    assert!(
        loaded.doc.bit_eq(session.committed_doc()),
        "the log replays to the same document"
    );
    assert_eq!(
        loaded
            .doc
            .slot_expansion(id, low)
            .map(|formula| loaded.doc.unparse(&formula)),
        Some("gap".to_owned()),
        "the loaded bound reads the variable"
    );
}

/// **A tree pick of a body the world draws twice is refused at the
/// pick, in the tool's words**: it names no one part, so the tool seats
/// nothing and asks for the copy in the picture — rather than seating
/// the construction for the insert door to refuse in a placement's
/// words.
///
/// Red if the tool seats a body the world draws more than once.
#[test]
fn a_body_drawn_twice_is_refused_at_the_pick_in_the_tools_words() {
    use viewer::combine::{ClearanceEvent, ClearanceTool};
    let tol = Tol::witness();
    let (doc, [left, _], _) = two_blocks_apart(tol);
    let (doc, _) = viewer::test_support::placed(&doc, left, tol);
    let mut tool = ClearanceTool::new();
    let refused = tool.seat_body(&doc, left).expect("the pick is refused");
    assert!(
        matches!(refused, ClearanceEvent::SeveralCopies { copies: 2, .. }),
        "{refused:?}"
    );
    let said = pncad::document::Said(&refused, pncad::document::Speaker::of(&doc)).to_string();
    assert!(
        said.contains("\"left\"")
            && said.contains("pick a face of the copy you mean in the picture"),
        "{said}"
    );
    assert_eq!(tool.body(0), None, "the seat is untouched");
}

/// **A requirement over a pattern's members refuses in words, never
/// crashes**: a pattern seat lifts the assertion over the family, one
/// requirement per member, which the certifying lane does not decide
/// yet. The row says so with its recourse, under the requirement's own
/// name. A face of one drawn member, picked in the picture, seats that
/// member alone, and is certified.
///
/// Red if the landing hands the family to the certify door as a verdict
/// (which panicked the worker), or the row stays silent.
#[test]
fn a_requirement_over_a_patterns_members_refuses_in_words() {
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let pattern = add(
        &mut session,
        SessionOp::AddPattern {
            input: left.into(),
            count: 3,
            rule: viewer::session::PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
                spacing: len(0.05),
            },
        },
    );
    session.pump();
    let by_body = add(
        &mut session,
        requirement(pattern, right, ClearanceBounds::AtLeast(len(0.02))),
    );
    let label = pncad::document::Label::new("pattern gap").expect("a label");
    let labelled = session.perform(SessionOp::SetLabel {
        node: by_body,
        label: Some(label),
    });
    assert!(labelled.refusal.is_none(), "{:?}", labelled.refusal);
    session.pump();
    let row = cleared(&session, by_body);
    assert!(
        matches!(
            row.answer,
            Certified::Answered(ClearanceOutcome::OverMembers { .. })
        ),
        "{:?}",
        row.answer
    );
    assert_eq!(row.state(), "Refused");
    let sentence = row.sentence().expect("the refusal is said");
    assert!(
        sentence.contains("\"pattern gap\"") && sentence.contains("Recourse:"),
        "{sentence}"
    );
    // A landing whose only clearance is lifted owes no certified answer.
    let seam = Recording::default();
    let mut fresh = DocSession::inline(session.committed_doc().clone(), tol)
        .with_certifier(Box::new(seam.clone()));
    fresh.pump();
    assert!(
        seam.submits().is_empty(),
        "a family is not handed to the certify door as a verdict"
    );
    // The certify door answers it the same way, in words, rather than
    // reading the family as a verdict.
    let answers = clearances_at_nominal(session.doc(), None, &CancelToken::new(), tol)
        .expect("an uncancelled run answers");
    let door = answers
        .iter()
        .find(|answer| answer.assertion == by_body)
        .expect("one answer per clearance");
    assert!(
        matches!(&door.answer, ClearanceOutcome::OverMembers { sentence } if sentence.contains("Recourse:")),
        "{:?}",
        door.answer
    );
    // A face of the second member, picked where the picture draws it.
    let doc = session.committed_doc();
    let [drawn] = viewer::world::placements_of(doc, pattern)[..] else {
        panic!("the pattern is drawn once");
    };
    let (landed, eval) = session.landed_pair().expect("landed");
    let index = viewer::pickindex::PickIndex::build(
        landed,
        eval,
        viewer::pickindex::PictureKey::of(
            session.generation(),
            viewer::test_support::plate_delta(),
            viewer::world::Shown::none(),
        ),
        tol,
    )
    .expect("the scene indexes");
    let id = *index
        .ids_in(drawn, 1)
        .first()
        .expect("the second member is drawn");
    let member_face = index.face_of(id).expect("a drawn patch is a named face");
    let by_face = add(
        &mut session,
        SessionOp::AddClearance {
            a: ClearedSeat::Face(member_face),
            b: ClearedSeat::Body(right.into()),
            bounds: ClearanceBounds::AtLeast(len(0.02)),
        },
    );
    session.pump();
    // That face is one member's: the second, 50 mm up the pattern,
    // whose nearest corner is √(10² + 30²) ≈ 31.6 mm from the right
    // block — clear of 20 mm, where the family's first member, 10 mm
    // away, would violate it.
    let row = cleared(&session, by_face);
    assert_eq!(
        row.answer,
        Certified::Answered(ClearanceOutcome::Holds),
        "a face of a drawn member reads that member"
    );
}

/// **A pattern of the left block along `+y`, members 50 mm apart**,
/// with the right block standing beside the third member, 10 mm from
/// it: answering the session, the index of the picture, the node that
/// draws the pattern, and the right block.
fn member_scene(
    tol: Tol,
) -> (
    DocSession,
    viewer::pickindex::PickIndex,
    RecipeNodeId,
    RecipeNodeId,
) {
    use viewer::test_support::{inserted, rectangle};
    let block = |doc: &pncad::document::Doc<pncad::document::ProfileProgram>, x: f64, y: f64| {
        let (doc, profile) = inserted(doc, rectangle([x, y], SIDE, SIDE), tol);
        inserted(
            &doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(0.01),
            },
            tol,
        )
    };
    let doc = pncad::document::Doc::empty_derived("member-witness", tol);
    let (doc, left) = block(&doc, 0.0, 0.0);
    let (doc, right) = block(&doc, SIDE + GAP, 0.10);
    let mut session = DocSession::inline(doc, tol);
    let pattern = add(
        &mut session,
        SessionOp::AddPattern {
            input: left.into(),
            count: 3,
            rule: viewer::session::PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::Y),
                spacing: len(0.05),
            },
        },
    );
    session.pump();
    for node in [pattern, right] {
        let placed = session.perform(SessionOp::AddToProduct { node, body: None });
        assert!(placed.refusal.is_none(), "{:?}", placed.refusal);
        session.pump();
    }
    // A pattern moved into the world is drawn by its own node; a copy
    // of one, by the copy.
    let drawn = viewer::world::placements_of(session.committed_doc(), pattern)
        .first()
        .copied()
        .unwrap_or(pattern);
    let index = picture_index(&session, tol);
    (session, index, drawn, right)
}

/// The index of the picture the session's landing draws.
fn picture_index(session: &DocSession, tol: Tol) -> viewer::pickindex::PickIndex {
    let (landed, eval) = session.landed_pair().expect("landed");
    viewer::pickindex::PickIndex::build(
        landed,
        eval,
        viewer::pickindex::PictureKey::of(
            session.generation(),
            viewer::test_support::plate_delta(),
            viewer::world::Shown::none(),
        ),
        tol,
    )
    .expect("the scene indexes")
}

/// A face of `member` of the drawn pattern, as a pick in the picture
/// takes it.
fn member_face(
    index: &viewer::pickindex::PickIndex,
    drawn: RecipeNodeId,
    member: u32,
) -> viewer::session::FaceSelection {
    let id = *index
        .ids_in(drawn, member)
        .first()
        .expect("the member is drawn");
    index.face_of(id).expect("a drawn patch is a named face")
}

/// What a selected clearance row marks: the lit patches and the leg.
fn marked(
    session: &DocSession,
    index: &viewer::pickindex::PickIndex,
    requirement: RecipeNodeId,
) -> ([u32; 2], Vec<[f32; 3]>) {
    let witnessed = session
        .witness(requirement)
        .expect("a violation has its pair");
    let (mut lit, mut legs) = (
        viewer::marks::Highlight::default(),
        viewer::marks::EdgeOverlay::default(),
    );
    viewer::marks::witness(
        index,
        &session.display_view(),
        &witnessed,
        viewer::marks::WitnessLit::Selected,
        (&mut lit, &mut legs),
    );
    ([lit.selected, lit.selected_second], legs.selected)
}

/// **A face seat on a pattern member is witnessed on that member**: a
/// face of the third member against the right block, 10 mm away, is
/// violated at 50 mm, and its witness lights the very patch picked and
/// draws its leg from that member — not from the first, which the seat
/// does not read.
///
/// Red if the witness reads a seat's body as body 0 rather than as the
/// member the kernel resolved it to.
#[test]
fn a_pattern_member_face_seat_is_witnessed_on_that_member() {
    let tol = Tol::witness();
    let (mut session, index, drawn, right) = member_scene(tol);
    let face = member_face(&index, drawn, 2);
    let requirement = add(
        &mut session,
        SessionOp::AddClearance {
            a: ClearedSeat::Face(face.clone()),
            b: ClearedSeat::Body(right.into()),
            bounds: ClearanceBounds::AtLeast(len(0.05)),
        },
    );
    session.pump();
    assert!(
        matches!(
            cleared(&session, requirement).answer,
            Certified::Answered(ClearanceOutcome::Violated(_))
        ),
        "the premise: the third member is 10 mm away"
    );
    let index = picture_index(&session, tol);
    let ([lit, _], leg) = marked(&session, &index, requirement);
    let picked = index.ids_of_target(&face);
    assert_eq!(picked.len(), 1, "the picked face is drawn once");
    assert_eq!(
        lit, picked[0],
        "the lit face is the picked one, on the third member"
    );
    assert!(
        (0.10 - 1e-6..=0.12 + 1e-6).contains(&f64::from(leg[0][1])),
        "the leg starts on the third member (y in [100, 120] mm): {leg:?}"
    );
}

/// **Two members' faces are witnessed where each member stands**: a
/// face of the second member against one of the third, neither in its
/// space's own frame, so the pair is decided in a member's frame. The
/// leg is drawn from the second member to the third — not 50 mm off,
/// beside the first.
///
/// Red if the witness composes the pair through the first member's
/// frame.
#[test]
fn two_pattern_members_faces_are_witnessed_where_each_stands() {
    let tol = Tol::witness();
    let (mut session, index, drawn, _) = member_scene(tol);
    let (second, third) = (member_face(&index, drawn, 1), member_face(&index, drawn, 2));
    let requirement = add(
        &mut session,
        SessionOp::AddClearance {
            a: ClearedSeat::Face(second.clone()),
            b: ClearedSeat::Face(third.clone()),
            bounds: ClearanceBounds::AtLeast(len(0.2)),
        },
    );
    session.pump();
    assert!(
        matches!(
            cleared(&session, requirement).answer,
            Certified::Answered(ClearanceOutcome::Violated(_))
        ),
        "the premise: two members are nearer than 200 mm"
    );
    let index = picture_index(&session, tol);
    let (lit, leg) = marked(&session, &index, requirement);
    assert_eq!(
        lit,
        [
            index.ids_of_target(&second)[0],
            index.ids_of_target(&third)[0]
        ],
        "both picked faces are lit"
    );
    let y = |end: [f32; 3]| f64::from(end[1]);
    assert!(
        (0.05 - 1e-6..=0.07 + 1e-6).contains(&y(leg[0]))
            && (0.10 - 1e-6..=0.12 + 1e-6).contains(&y(leg[1])),
        "the leg runs from the second member to the third: {leg:?}"
    );
}

/// **A variable a clearance bound reads focuses nothing**: an assertion
/// moves no geometry, so selecting the variable its bound slot reads
/// lights up no patch, not even through the assertion's seats
/// (`marks::drives`).
///
/// Red if `drives` counts an assertion's bound slots: the seats'
/// ancestry is then marked.
#[test]
fn a_variable_a_clearance_bound_reads_focuses_nothing() {
    use pncad::document::{BoundEnd, SlotId};
    use viewer::combine::ClearanceBoundChoice;
    let tol = Tol::witness();
    let (mut session, left, right) = scene(tol, None);
    let id = add(
        &mut session,
        requirement(left, right, ClearanceBoundChoice::AtLeast.bounds(len(0.0))),
    );
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::new("gap").expect("a name"),
        value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Length, 0.005),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let written = session.perform(SessionOp::SetSlotExpression {
        node: id,
        slot: SlotId::ClearanceBound(BoundEnd::Low),
        text: "gap".to_owned(),
    });
    assert!(written.refusal.is_none(), "{:?}", written.refusal);
    session.pump();
    let index = crate::common::index_of(
        &session,
        viewer::scene::DisplayTolerance::new(0.0005).expect("a tolerance"),
    );
    let gap = session.committed_doc().var_named("gap").expect("declared");
    let focus = viewer::marks::focus(
        &index,
        session.doc(),
        &viewer::session::Selection::Variable(gap),
    );
    assert!(
        focus.is_empty(),
        "the bound's variable moves nothing: {focus:?}"
    );
}

/// **A bound that does not evaluate names its own slot**: each shape's
/// each end, with `1 mm / z` at `z = 0` written there, fails the
/// requirement with `NodeErrorKind::Expr` at that end's
/// `SlotId::ClearanceBound`, a band's upper end at `High`.
///
/// Red if the fault names another end, or is no slot fault.
#[test]
fn a_bound_that_does_not_evaluate_names_its_own_end() {
    use pncad::document::{BoundEnd, NodeErrorKind, SlotId};
    use viewer::combine::ClearanceBoundChoice;
    let tol = Tol::witness();
    for (choice, end) in [
        (ClearanceBoundChoice::AtLeast, BoundEnd::Low),
        (ClearanceBoundChoice::AtMost, BoundEnd::High),
        (ClearanceBoundChoice::Equal, BoundEnd::Low),
        (ClearanceBoundChoice::Between, BoundEnd::Low),
        (ClearanceBoundChoice::Between, BoundEnd::High),
    ] {
        let (mut session, left, right) = scene(tol, None);
        let id = add(
            &mut session,
            requirement(left, right, choice.bounds(len(0.001))),
        );
        let declared = session.perform(SessionOp::DeclareVar {
            name: pncad::document::VarName::new("z").expect("a name"),
            value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Scalar, 0.0),
        });
        assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
        let written = session.perform(SessionOp::SetSlotExpression {
            node: id,
            slot: SlotId::ClearanceBound(end),
            text: "1 mm / z".to_owned(),
        });
        assert!(written.refusal.is_none(), "{:?}", written.refusal);
        session.pump();
        let eval = session.evaluation().expect("landed");
        let error = eval
            .result(id)
            .and_then(pncad::document::NodeResult::error)
            .expect("the bound fails");
        assert!(
            matches!(&error.kind, NodeErrorKind::Expr { slot, .. } if *slot == SlotId::ClearanceBound(end)),
            "{choice:?} {end:?}: {:?}",
            error.kind
        );
    }
}
