//! **The viewer's clearance consumer**: a landing whose `f64` run holds
//! a clearance requirement owes the certifying scalar's answer, taken at
//! rest on its own seam and settled into the landing; the row reads the
//! requirement as an equation and that answer beside it; the witness
//! pair is marked from the run the picture is drawn from; and the
//! create door writes each shape of bound.
//!
//! The scene is two 20 × 20 × 10 mm blocks side by side along `x`,
//! [`GAP`] apart, so every verdict has a number worked out by hand.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::cell::RefCell;
use std::rc::Rc;

use pncad::analysis::{ClearanceOutcome, clearances_at_nominal};
use pncad::document::{
    CancelToken, ClearanceBounds, Doc, Formula, Node, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use viewer::evalseam::{CertifyDone, CertifyRequest, CertifyService};
use viewer::generation::Generation;
use viewer::props::Notation;
use viewer::session::{ClearedSeat, DocSession, SessionOp};
use viewer::test_support::{inserted, len, rectangle};
use viewer::tree::{Certified, Readout, TreeRow};

/// The blocks' side, in x and y.
const SIDE: f64 = 0.02;
/// Their height.
const HEIGHT: f64 = 0.01;
/// The gap between the two blocks' facing walls.
const GAP: f64 = 0.01;

/// Millimetres, so every bound reads as written.
const IN_MM: Notation = Notation {
    length: MM,
    ..Notation::DEFAULT
};

/// **The scene, landed**: the two blocks, each a construction labelled
/// `left` and `right` and placed into the world — two parts as they
/// stand — under `certifier`, answering the session and each block's
/// extrude.
fn scene(
    tol: Tol,
    certifier: Option<Box<dyn CertifyService>>,
) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (doc, left, right) = two_blocks(tol);
    let session = DocSession::inline(doc, tol);
    let mut session = match certifier {
        Some(certifier) => session.with_certifier(certifier),
        None => session,
    };
    for block in [left, right] {
        let outcome = session.perform(SessionOp::AddToProduct {
            node: block,
            body: None,
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
    }
    (session, left, right)
}

/// The world placement that draws `block`.
fn drawn(session: &DocSession, block: RecipeNodeId) -> RecipeNodeId {
    let [placement] = viewer::world::placements_of(session.committed_doc(), block)[..] else {
        panic!("each block is placed once");
    };
    placement
}

/// The two blocks, answering the document and each block's extrude.
fn two_blocks(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc = Doc::empty_derived("clearance-consumer", tol);
    let block = |doc: &Doc<ProfileProgram>, x: f64, label: &str| {
        let (doc, profile) = inserted(doc, rectangle([x, 0.0], SIDE, SIDE), tol);
        let (doc, solid) = inserted(
            &doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(HEIGHT),
            },
            tol,
        );
        let labelled = pncad::document::apply(
            &doc,
            &pncad::document::DocEdit::SetLabel {
                node: solid,
                label: Some(pncad::document::Label::new(label).expect("a label")),
            },
            tol,
        )
        .expect("the label lands")
        .doc;
        (labelled, solid)
    };
    let (doc, left) = block(&doc, 0.0, "left");
    let (doc, right) = block(&doc, SIDE + GAP, "right");
    (doc, left, right)
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
    /// The answer the certifying scalar gives the request in hand.
    fn answer(&self, tol: Tol) -> CertifyDone {
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
                tol,
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
/// the edit that replaces a landing cancels the seam, and an answer for
/// the replaced landing settles nothing — the row reads certifying until
/// the new landing's own answer lands.
///
/// Red if the next request leaves the job running, or if an answer for
/// an older generation settles into the newer landing.
#[test]
fn a_newer_request_cancels_the_certify_job_and_drops_its_answer() {
    let tol = Tol::witness();
    let seam = Recording::default();
    let (mut session, left, right) = scene(tol, Some(Box::new(seam.clone())));
    let id = add(
        &mut session,
        requirement(left, right, ClearanceBounds::AtLeast(len(0.005))),
    );
    session.pump();
    let stale = seam.answer(tol);
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
    session.pump();
    assert_ne!(stale.generation, session.generation(), "the premise");
    seam.ready.borrow_mut().push(stale);
    session.pump();
    assert_eq!(
        cleared(&session, id).answer,
        Certified::Certifying,
        "an answer for the landing replaced settles nothing"
    );
    let fresh = seam.answer(tol);
    seam.ready.borrow_mut().push(fresh);
    session.pump();
    assert_eq!(
        cleared(&session, id).answer,
        Certified::Answered(ClearanceOutcome::Holds),
        "the landing's own answer settles"
    );
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
/// and a negative bound its `NotADistance`. Neither sentence names a
/// node by a bare id.
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
        for node in [left, right, reversed, negative] {
            assert!(
                !sentence.contains(&format!("node {}", node)),
                "{name} names no node by a bare id: {sentence}"
            );
        }
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
/// clearance tool's choice over the form's two lengths.
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
    tool.seat_body(session.committed_doc(), right);
    let (lo, hi) = (len(0.005), len(0.02));
    for (choice, _) in ClearanceBoundChoice::ALL {
        let op = tool
            .op(choice.bounds(lo.clone(), hi.clone()))
            .expect("both seats are held");
        let id = add(&mut session, op);
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
