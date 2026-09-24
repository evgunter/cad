//! **The notation a recorded path leg was authored in**, carried
//! across the one crossing where a recording becomes a document:
//! `LoopProgram::from_recorded_with_notation`.
//!
//! A `profile::Step<f64>` is canonical metres and radians and stays so
//! — D6 keeps dimensional types out of kernel-internal code and G1
//! layering puts `quantity` above `profile` — so a leg authored as
//! `25 mm` reached the document as `0.025` with nothing saying what it
//! was written in, while the same vertex authored through the polygon
//! door read back `25 mm`. The first row below is that disagreement;
//! every row after it is the door that ends it.
//!
//! **Two doors onto the address.** `set` takes the step index, and a
//! hand-written index is a second description of the recording that
//! can disagree with it silently — an off-by-one landing on a step
//! that carries the same role is accepted. `set_after` takes the
//! recording instead and writes its last step, so the leg is the
//! recorder's own count. Every row here that authors a leg writes
//! through `set_after`, and the trap the addressed door leaves open
//! is pinned as behaviour rather than described.
//!
//! **Three rows keep `set`, each for a reason the derived door does
//! not cover.** `a_hand_counted_index_that_is_off_by_one_lands_on_the
//! _wrong_leg` needs a hand index because the acceptance it pins is
//! what a hand index buys. `a_unit_must_measure_what_its_role_holds`
//! authors no recording at all, so there is nothing for `set_after`
//! to derive an index from. `a_notation_entry_off_the_program_refuses`
//! pins sentences only a hand-written index can provoke — a step past
//! the end, and a role the step named does not carry.
//!
//! **What these rows pin, and where the claim is stated.** The reading
//! is written once, on `RecordedNotation`'s own rustdoc — the notation
//! is presentation metadata under DESIGN.md D6, it is keyed by the
//! document's own `(step, StepArg)` address, and it is consumed at the
//! lift rather than stored. Here it is executed: the read-back, the
//! bit-blindness of identity and of evaluation, the round trip through
//! the SAVED TEXT and back, both halves of the address, and the
//! refusals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/expr.rs",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/src/persist/",
    "crates/editor-core/src/program.rs",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use editor_core::{
    DocEdit, DocumentId, EvalOptions, ExprPath, LoopProgram, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RecordedNotation, RecordedProgramError, SlotId, StepArg, ValuePayload, load,
    save,
};
use geom_core::{Point2, Tol};
use profile::{Open, Start, Step};

/// Every document below is a frame and then the profile drawn on it.
const PLANE: RecipeNodeId = RecipeNodeId(0);
const PROFILE: RecipeNodeId = RecipeNodeId(1);

/// The leg this suite is about: step 1's target, the corner a path
/// author writes as `line_to((25 mm, 0 mm))`.
///
/// **This is the suite's READ address, and it is a hand count on
/// purpose.** The rows below author their notation through
/// `RecordedNotation::set_after`, which derives the index from the
/// recording, and then read the document back here — so the write and
/// the read are two independent descriptions of which leg was meant,
/// and a derivation that picked a different step is a failure rather
/// than a shared mistake.
const LEG: u32 = 1;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A square of side `side` METRES, recorded through the path algebra
/// exactly as a caller writes it: `At(p0)`, three `LineTo`s, the
/// closer — with `roles` of the FIRST leg written in millimetres at
/// the leg itself, through the derived door.
///
/// This is where a caller writing a notation stands: holding the path
/// it has just extended, not a finished recording it has to count
/// through. `set_after` takes the recording so far and writes the
/// verb that made it, so the notation in the pair below names the leg
/// the line above authored.
fn square_authored(side: f64, roles: &[StepArg]) -> (Vec<Step<f64>>, RecordedNotation) {
    let t = Tol::witness();
    let mut n = RecordedNotation::new();
    let path = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(side, 0.0), t)
        .expect("a leg of a square");
    for &arg in roles {
        n.set_after(path.recorded(), arg, quantity::MM.def())
            .expect("mm measures a length, and a target coordinate is one");
    }
    let program = path
        .line_to(p2(side, side), t)
        .expect("a leg of a square")
        .line_to(p2(0.0, side), t)
        .expect("a leg of a square")
        .line_to(Start, t)
        .expect("the square closes")
        .program;
    (program, n)
}

/// The same square with no notation anywhere.
fn square(side: f64) -> Vec<Step<f64>> {
    square_authored(side, &[]).0
}

/// `25 mm`, written down: the square whose first leg a caller authored
/// in millimetres, both coordinates. The value in the recording is the
/// canonical `0.025`, because that is what a recording holds.
fn in_millimetres() -> (Vec<Step<f64>>, RecordedNotation) {
    square_authored(0.025, &[StepArg::TargetX, StepArg::TargetY])
}

/// The document a lifted program reaches: a frame, then the profile.
fn doc_of(program: LoopProgram) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("edit-recorded-notation"), Tol::witness());
    for edit in edits_of(program) {
        doc = doc
            .apply(&edit, Tol::witness(), &editor_core::RefusingReach)
            .expect("the fixture document is legal")
            .doc;
    }
    doc
}

/// The same document as its edit log, for the save/load row.
fn edits_of(program: LoopProgram) -> [DocEdit<ProfileProgram>; 2] {
    [
        DocEdit::InsertNode {
            node: fixture::xy_frame(),
        },
        DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: PLANE,
                loops: vec![program],
            }),
        },
    ]
}

fn slot(step: u32, arg: StepArg) -> ExprPath {
    ExprPath {
        node: PROFILE,
        slot: SlotId::Profile {
            loop_: 0,
            step,
            arg,
        },
        path: Vec::new(),
    }
}

/// What a reader asking the document what one argument says gets back:
/// the canonical value, and the notation it was written in.
fn read_back(doc: &ProfileDoc, step: u32, arg: StepArg) -> (f64, &'static str) {
    let Some(e) = doc.expr_at(&slot(step, arg)) else {
        panic!("the document addresses ({step}, {arg:?})")
    };
    let Some(v) = e.literal_value() else {
        panic!("a recorded argument is a literal")
    };
    let Some(u) = e.display_unit() else {
        panic!("a literal always names its notation")
    };
    (v, u.symbol())
}

/// Every addressable argument of a lifted program, with the value and
/// the notation the document reads back for it.
///
/// Enumerated through `LoopProgram::step_args` — the program's OWN walk
/// over its roles, the same one the slot doors answer at — so a row
/// built on this covers every argument the program has. A hand-written
/// list of roles would cover the ones the author thought of, which is
/// how a mutant remapping a role the suite never names survives.
///
/// It also asserts what it enumerated: every address `step_args` hands
/// back holds an expression, so a role the walk claims and the document
/// cannot answer is a failure here rather than a silently shorter list.
fn arg_bits(program: LoopProgram) -> Vec<(u32, StepArg, Option<f64>, Option<&'static str>)> {
    let addresses = program.step_args();
    let doc = doc_of(program);
    addresses
        .into_iter()
        .map(|(step, arg)| {
            let Some(e) = doc.expr_at(&slot(step, arg)) else {
                panic!("step_args names ({step}, {arg:?}), so the document addresses it")
            };
            (
                step,
                arg,
                e.literal_value(),
                e.display_unit().map(|u| u.symbol()),
            )
        })
        .collect()
}

/// The replayed loop's vertices, bit for bit — what "one geometry"
/// means where two documents are compared.
fn vertex_bits(doc: &ProfileDoc) -> Vec<(u64, u64)> {
    let ev = fixture::run(doc, &EvalOptions::default());
    let Some(v) = ev.value(PROFILE) else {
        panic!("the profile evaluates")
    };
    let ValuePayload::Profile(pv) = &v.payload else {
        panic!("a profile payload")
    };
    pv.validated.loops()[0]
        .vertices()
        .iter()
        .map(|vx| (vx.pos().x.to_bits(), vx.pos().y.to_bits()))
        .collect()
}

// ------------------------------------------------------------------
// The disagreement, and the door that ends it
// ------------------------------------------------------------------

/// **The row this unit exists for.** A leg authored in millimetres
/// reads back millimetres through the document.
///
/// RED before the notation door: the same recording lifted to a literal
/// whose display unit was the canonical metre, so `25 mm` came back as
/// `0.025 m` — the polygon door's `25 mm` and the path door's `0.025 m`
/// being two spellings of one authoring that disagreed.
///
/// The VALUE is canonical either way and this row says so: what the
/// notation changes is what the document says the number was written
/// in, never the number.
#[test]
fn a_leg_authored_in_millimetres_reads_back_millimetres() {
    let (steps, notation) = in_millimetres();
    let program = LoopProgram::from_recorded_with_notation(&steps, &notation)
        .expect("a square with a notation lifts");
    let doc = doc_of(program);
    assert_eq!(
        read_back(&doc, LEG, StepArg::TargetX),
        (0.025, "mm"),
        "the leg's x reads back in the notation it was authored in"
    );
    assert_eq!(read_back(&doc, LEG, StepArg::TargetY), (0.0, "mm"));
}

/// And an argument the author wrote no notation for reads back the
/// canonical unit — the notation is per ARGUMENT, not per program.
///
/// Step 2's corner is the same 25 mm of geometry as step 1's and is
/// addressed one step over; nothing about the mm on step 1 reaches it.
#[test]
fn an_argument_with_no_notation_reads_back_the_canonical_unit() {
    let (steps, notation) = in_millimetres();
    let program = LoopProgram::from_recorded_with_notation(&steps, &notation)
        .expect("a square with a notation lifts");
    let doc = doc_of(program);
    assert_eq!(
        read_back(&doc, 2, StepArg::TargetX),
        (0.025, "m"),
        "the neighbouring leg was written with no notation"
    );
    assert_eq!(read_back(&doc, 0, StepArg::PointX), (0.0, "m"));
}

/// A recording with NO notation lifts to exactly the program
/// `from_recorded` mints — argument for argument, bit for bit.
///
/// This is what lets the two doors be one door with a default, and it
/// is the row that would red if the notation pass touched a program it
/// was given nothing to say about.
#[test]
fn an_empty_notation_lifts_to_the_program_from_recorded_mints() {
    let steps = square(0.025);
    let plain = LoopProgram::from_recorded(&steps).expect("lifts");
    let empty =
        LoopProgram::from_recorded_with_notation(&steps, &RecordedNotation::new()).expect("lifts");
    assert!(
        RecordedNotation::new().is_empty(),
        "the default notation writes nothing down"
    );
    let bits = arg_bits(plain.clone());
    assert_eq!(
        bits.len(),
        8,
        "the square's addressable arguments: the opening point's two \
         coordinates and a target pair per LineTo that names a point — \
         the closer names `Start` and carries none"
    );
    assert_eq!(bits, arg_bits(empty.clone()));
    assert!(
        plain == empty,
        "and the two programs are bit-identical as programs"
    );
}

// ------------------------------------------------------------------
// Identity: the unit is presentation metadata (D6)
// ------------------------------------------------------------------

/// Two recordings of one leg — `25 mm` and `0.025 m` — are the same
/// program and evaluate to one geometry.
///
/// `ProfileProgram`'s `PartialEq` is the bit comparator, and
/// `Expr::bit_eq` excludes the display unit, so the notation cannot
/// reach the recorder's replay identity: a program is compared and
/// re-run by what it computes, never by what it says it was written in.
#[test]
fn two_notations_of_one_leg_are_one_program_and_one_geometry() {
    let (steps, notation) = in_millimetres();
    let millimetres = LoopProgram::from_recorded_with_notation(&steps, &notation)
        .expect("the mm recording lifts");
    let metres = LoopProgram::from_recorded(&steps).expect("the m recording lifts");
    let (doc_mm, doc_m) = (doc_of(millimetres.clone()), doc_of(metres.clone()));
    assert_eq!(
        (
            read_back(&doc_mm, LEG, StepArg::TargetX).1,
            read_back(&doc_m, LEG, StepArg::TargetX).1,
        ),
        ("mm", "m"),
        "the two recordings really do say different things about their notation"
    );
    let a = ProfileProgram {
        plane: PLANE,
        loops: vec![millimetres.clone()],
    };
    let b = ProfileProgram {
        plane: PLANE,
        loops: vec![metres.clone()],
    };
    assert!(a == b, "bit equality is blind to the notation");
    assert_eq!(
        vertex_bits(&doc_mm),
        vertex_bits(&doc_m),
        "and both replay to the same vertices, bit for bit"
    );
}

/// The notation survives save and load, as every literal's does — the
/// round trip a recorded program needs before a stored notation is
/// worth writing.
#[test]
fn a_recorded_notation_round_trips_through_save_and_load() {
    let (steps, notation) = in_millimetres();
    let program = LoopProgram::from_recorded_with_notation(&steps, &notation).expect("lifts");
    let base = ProfileDoc::empty(DocumentId::derive("edit-recorded-notation"), Tol::witness());
    let edits = edits_of(program.clone());
    let text = save(
        &base,
        &editor_core::LoggedEdit::bare_all(&edits),
        Tol::witness(),
    )
    .expect("the log saves");
    // The STORED FORM, before any load: a save that dropped the symbol
    // and a load that re-derived it from the dimension would satisfy
    // every assertion below, and would lose the notation the moment a
    // file was read by anything else.
    assert!(
        text.contains("mm"),
        "the persisted text names the notation; got:\n{text}"
    );
    let plain = save(
        &base,
        &editor_core::LoggedEdit::bare_all(&edits_of(
            LoopProgram::from_recorded(&square(0.025)).expect("lifts"),
        )),
        Tol::witness(),
    )
    .expect("the log saves");
    assert!(
        !plain.contains("mm"),
        "and a recording with no notation stores none"
    );
    assert_ne!(text, plain, "so the two recordings save to different text");

    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert_eq!(
        read_back(&loaded.doc, LEG, StepArg::TargetX),
        (0.025, "mm"),
        "the leg still says it was written in millimetres"
    );
    assert!(
        loaded.doc.bit_eq(&doc_of(program)),
        "and the loaded document is the document that was saved"
    );
}

// ------------------------------------------------------------------
// The carrier forms
// ------------------------------------------------------------------

/// A carrier form authors ONE step, numbered 0, and takes its notation
/// at that step like any other — the radius in millimetres, the phase
/// in degrees.
#[test]
fn a_carrier_form_takes_its_notation_at_step_zero() {
    let t = Tol::witness();
    let circle = profile::circle(p2(0.0, 0.0), 0.025, t).expect("a circle");
    let mut n = RecordedNotation::new();
    // A carrier's whole recording is its one step, so the derived
    // door addresses step 0 without the author saying so.
    n.set_after(&circle.program, StepArg::Radius, quantity::MM.def())
        .expect("a radius is a length");
    let program =
        LoopProgram::from_recorded_with_notation(&circle.program, &n).expect("the circle lifts");
    let doc = doc_of(program);
    assert_eq!(read_back(&doc, 0, StepArg::Radius), (0.025, "mm"));
    assert_eq!(
        read_back(&doc, 0, StepArg::CenterX),
        (0.0, "m"),
        "the centre was written with no notation"
    );
}

// ------------------------------------------------------------------
// The address: one role, and the roles outside the point-shaped verbs
// ------------------------------------------------------------------

/// One role of a pair takes the notation ALONE — the address is per
/// role, not per step.
///
/// `in_millimetres` writes BOTH `TargetX` and `TargetY` of the leg, so
/// every row above it survives a lift that swapped the two roles on
/// its way through. Writing one of the pair is what gives the role half
/// of the address its own tension.
#[test]
fn one_role_of_a_pair_takes_the_notation_alone() {
    let (steps, n) = square_authored(0.025, &[StepArg::TargetX]);
    let program = LoopProgram::from_recorded_with_notation(&steps, &n).expect("the square lifts");
    let doc = doc_of(program);
    assert_eq!(read_back(&doc, LEG, StepArg::TargetX), (0.025, "mm"));
    assert_eq!(
        read_back(&doc, LEG, StepArg::TargetY),
        (0.0, "m"),
        "the sibling role of the same step keeps the canonical unit"
    );
}

/// An angle role and a length role on ONE program, neither of them a
/// coordinate.
///
/// Every other row addresses `PointX/Y`, `TargetX/Y`, `CenterX` and
/// `Radius` — six of the twenty-nine roles [`StepArg`] declares, all
/// reached through the point-shaped verbs. A `line`/`turn` square is
/// authored in the other half of the vocabulary: a scalar leg length
/// and a scalar corner, so a lift that mapped one role to a neighbour
/// of the same dimension has somewhere to be caught.
#[test]
fn an_angle_role_and_a_length_role_on_one_program() {
    let t = Tol::witness();
    let l = 0.025;
    let q = std::f64::consts::FRAC_PI_2;
    let mut n = RecordedNotation::new();
    let path = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .expect("a departure direction")
        .line(l, t)
        .expect("the first side");
    n.set_after(path.recorded(), StepArg::Length, quantity::MM.def())
        .expect("a leg length is a length");
    let path = path.turn(q, t).expect("a corner");
    n.set_after(path.recorded(), StepArg::TurnVal, quantity::DEG.def())
        .expect("a turn is an angle");
    let steps = path
        .line(l, t)
        .expect("the second side")
        .turn(q, t)
        .expect("a corner")
        .line(l, t)
        .expect("the third side")
        .line_to(Start, t)
        .expect("the square closes")
        .program;
    let program = LoopProgram::from_recorded_with_notation(&steps, &n).expect("the square lifts");
    let doc = doc_of(program);
    assert_eq!(read_back(&doc, 2, StepArg::Length), (l, "mm"));
    assert_eq!(
        read_back(&doc, 3, StepArg::TurnVal),
        (q, "deg"),
        "the turn reads back in degrees and the VALUE stays canonical radians"
    );
    assert_eq!(
        read_back(&doc, 4, StepArg::Length),
        (l, "m"),
        "the next side was written with no notation"
    );
}

// ------------------------------------------------------------------
// The two doors onto the address: derived, and hand-written
// ------------------------------------------------------------------

/// **The derived door writes the leg the author had just recorded.**
///
/// `set_after` takes the recording so far and addresses its LAST step,
/// so the index is the recorder's own count and never the author's.
/// The chain is three legs and the notation is written after leg two,
/// which is step 2 because the entry verb is step 0 — the arithmetic
/// nobody performs here.
///
/// RED before the derived door in the only way it can be: there was no
/// door, and the author wrote `set(2, …)` after counting.
#[test]
fn the_derived_door_writes_the_leg_the_author_had_just_recorded() {
    let t = Tol::witness();
    let mut n = RecordedNotation::new();
    let path = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(0.025, 0.0), t)
        .expect("the first leg");
    let path = path.line_to(p2(0.025, 0.025), t).expect("the second leg");
    for arg in [StepArg::TargetX, StepArg::TargetY] {
        n.set_after(path.recorded(), arg, quantity::MM.def())
            .expect("mm measures a length, and a target coordinate is one");
    }
    assert_eq!(
        n.get(2, StepArg::TargetX).map(|u| u.symbol()),
        Some("mm"),
        "the notation landed on the leg just recorded"
    );
    assert_eq!(
        n.get(1, StepArg::TargetX),
        None,
        "and on no other leg — the first leg was recorded before the write"
    );
    assert_eq!(n.len(), 2, "the two roles written, and nothing else");
    let steps = path.line_to(Start, t).expect("the chain closes").program;
    let lifted = LoopProgram::from_recorded_with_notation(&steps, &n)
        .expect("the chain with a notation lifts");
    let doc = doc_of(lifted);
    assert_eq!(read_back(&doc, 2, StepArg::TargetX), (0.025, "mm"));
    assert_eq!(read_back(&doc, 2, StepArg::TargetY), (0.025, "mm"));
    assert_eq!(
        read_back(&doc, 1, StepArg::TargetX),
        (0.025, "m"),
        "leg one is the same 25 mm of geometry and was written with no notation"
    );
}

/// **The trap the derived door closes, in the direction that tells.**
///
/// A hand count that is off by one lands on a step carrying the SAME
/// role, so the lift has nothing to refuse: the program is minted, the
/// unit is on the wrong leg, and nobody is told. This row asserts the
/// silent acceptance rather than a refusal, because that is the
/// behaviour `set` has and keeps — the addressed door stays for the
/// callers whose index is derived from the program (the viewer's
/// sketch notation walks `LoopProgram::step_args`), so the trap is
/// still expressible and the recourse is `set_after`, not a check.
#[test]
fn a_hand_counted_index_that_is_off_by_one_lands_on_the_wrong_leg() {
    let t = Tol::witness();
    let steps = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(0.025, 0.0), t)
        .expect("the first leg")
        .line_to(p2(0.025, 0.025), t)
        .expect("the second leg")
        .line_to(Start, t)
        .expect("the chain closes")
        .program;
    // The author meant the second leg and counted the LEGS, not the
    // steps — the entry verb is a step too.
    let mut n = RecordedNotation::new();
    n.set(1, StepArg::TargetX, quantity::MM.def())
        .expect("mm measures a length");
    let lifted = LoopProgram::from_recorded_with_notation(&steps, &n)
        .expect("the miscount is ACCEPTED: step 1 carries a target x of its own");
    let doc = doc_of(lifted);
    assert_eq!(
        read_back(&doc, 1, StepArg::TargetX),
        (0.025, "mm"),
        "the unit landed on the leg the author did not mean"
    );
    assert_eq!(
        read_back(&doc, 2, StepArg::TargetX),
        (0.025, "m"),
        "and the leg they did mean reads back the canonical unit"
    );
}

/// The derived door refuses a recording with no last step: there is
/// nothing for it to write against, and it will not invent step 0.
///
/// The refusal is its own arm rather than
/// `NotationOffProgram { step: 0, … }`, because the caller named no
/// step for that sentence to be about — and step 0 of a recording
/// whose entry verb lacks the role is a live, different mistake
/// (`a_notation_entry_off_the_program_refuses` writes exactly it).
#[test]
fn the_derived_door_refuses_a_recording_with_no_last_step() {
    let mut n = RecordedNotation::new();
    let refused = n
        .set_after(&[], StepArg::TargetX, quantity::MM.def())
        .expect_err("nothing has been recorded");
    assert_eq!(
        refused,
        RecordedProgramError::NotationBeforeAnyStep {
            arg: StepArg::TargetX
        }
    );
    assert_eq!(
        refused.to_string(),
        "the notation names the target x of the step just recorded, and nothing has been \
         recorded yet"
    );
    assert!(n.is_empty(), "and nothing was written down");
}

/// The derived door asks the same question about the unit that the
/// addressed one does, and says so in the lift's vocabulary: a wrong
/// QUANTITY is refused where the caller writes it, before any program
/// exists.
#[test]
fn the_derived_door_refuses_a_unit_that_does_not_measure_the_role() {
    let t = Tol::witness();
    let path = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(0.025, 0.0), t)
        .expect("the first leg");
    let mut n = RecordedNotation::new();
    let refused = n
        .set_after(path.recorded(), StepArg::TargetX, quantity::DEG.def())
        .expect_err("a target coordinate is a length");
    assert_eq!(
        refused.to_string(),
        "a recorded literal was refused: the display unit measures angle but the literal is length"
    );
    assert!(n.is_empty(), "and nothing was written down");
}

/// A role the LAST step does not carry is still the lift's off-program
/// refusal — and it now names the author's own leg, because the step
/// half of the address was derived from the recording.
///
/// This is the half `set_after` does not close and does not claim to:
/// the index is certain, the role is the author's to get right.
#[test]
fn the_derived_door_still_refuses_a_role_the_last_step_does_not_carry() {
    let t = Tol::witness();
    let path = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(0.025, 0.0), t)
        .expect("the first leg");
    let path = path.line_to(p2(0.025, 0.025), t).expect("the second leg");
    let last = u32::try_from(path.recorded().len() - 1).expect("a short recording");
    let mut n = RecordedNotation::new();
    n.set_after(path.recorded(), StepArg::ViaX, quantity::MM.def())
        .expect("a via point is a length, so the notation door writes it down");
    let steps = path.line_to(Start, t).expect("the chain closes").program;
    let refused = LoopProgram::from_recorded_with_notation(&steps, &n)
        .expect_err("a LineTo carries no via point");
    assert_eq!(
        refused,
        RecordedProgramError::NotationOffProgram {
            step: last,
            arg: StepArg::ViaX
        },
        "the refusal names the leg that was just authored"
    );
    assert_eq!(last, 2, "which is the second leg of this chain");
}

// ------------------------------------------------------------------
// The refusals
// ------------------------------------------------------------------

/// A unit must measure what its role holds, and the refusal lands at
/// the door where the caller writes it rather than at the lift.
///
/// The Scalar roles are where this bites: a bulge and a director
/// component are ratios, so the only unit they admit is the
/// dimensionless row every Scalar literal already carries — which is
/// what "a scalar argument carries no notation" means, executed.
///
/// Written through `set`, and the index is arbitrary: this row is
/// about the door's predicate over (role, unit), which no recording
/// takes part in — there is nothing here for `set_after` to derive an
/// index from.
#[test]
fn a_unit_must_measure_what_its_role_holds() {
    let mut n = RecordedNotation::new();
    for (arg, unit, sentence) in [
        (
            StepArg::Bulge,
            quantity::MM.def(),
            "the display unit measures length but the literal is scalar",
        ),
        (
            StepArg::DirX,
            quantity::DEG.def(),
            "the display unit measures angle but the literal is scalar",
        ),
        (
            StepArg::TargetX,
            quantity::DEG.def(),
            "the display unit measures angle but the literal is length",
        ),
        (
            StepArg::TurnVal,
            quantity::MM.def(),
            "the display unit measures length but the literal is angle",
        ),
    ] {
        let refused = n
            .set(0, arg, unit)
            .expect_err("the unit does not measure what the role holds");
        assert_eq!(refused.to_string(), sentence);
        assert!(n.is_empty(), "and nothing was written down");
    }
    // The pairings that DO agree are written, and the Scalar row's own
    // unit is one of them.
    n.set(0, StepArg::Bulge, quantity::ONE.def())
        .expect("a ratio is written in the dimensionless row");
    n.set(0, StepArg::TurnVal, quantity::DEG.def())
        .expect("a turn is an angle");
    assert_eq!(n.len(), 2);
    assert_eq!(n.get(0, StepArg::TurnVal).map(|u| u.symbol()), Some("deg"));
    assert_eq!(n.get(0, StepArg::Radius), None);
}

/// A notation entry addressing an argument the recording has none of
/// refuses, naming the step and the role — it is not dropped.
///
/// A notation is a caller's SECOND description of a recording, so the
/// two can disagree: a step past the program's end, or a role this
/// verb does not carry. Both are the caller's own mistake and both are
/// told.
///
/// **Written through `set`** (one of the three rows that are — the
/// module header lists them), because the refusals it pins are the
/// addressed door's own: a step past the program's end is a sentence
/// only a caller who wrote an index can provoke.
/// `the_derived_door_still_refuses_a_role_the_last_step_does_not_carry`
/// covers the half the derived door reaches.
#[test]
fn a_notation_entry_off_the_program_refuses() {
    let steps = square(0.025);
    for (step, arg, why, sentence) in [
        (
            99,
            StepArg::TargetX,
            "a step past the program's end",
            "the notation names the target x of step 99, which this recording has no argument at",
        ),
        (
            LEG,
            StepArg::ViaX,
            "a role a LineTo does not carry",
            "the notation names the via x of step 1, which this recording has no argument at",
        ),
        (
            0,
            StepArg::TargetX,
            "a role the entry step does not carry",
            "the notation names the target x of step 0, which this recording has no argument at",
        ),
    ] {
        let mut n = RecordedNotation::new();
        n.set(step, arg, quantity::MM.def()).expect("a length");
        let refused = LoopProgram::from_recorded_with_notation(&steps, &n)
            .expect_err("the entry addresses nothing");
        assert_eq!(
            refused,
            RecordedProgramError::NotationOffProgram { step, arg },
            "{why}"
        );
        assert_eq!(refused.to_string(), sentence);
    }
}

// ------------------------------------------------------------------
// The derived index over the whole verb vocabulary
// ------------------------------------------------------------------

/// **Every verb moves the derived index by exactly one, fused verbs
/// and binders included, and the unit lands on the verb the author
/// had just written.**
///
/// The chain walks the shapes that could break the correspondence
/// between "the verb I just called" and "the step the lift numbers":
/// `arc_fillet_arc` as the entry (two arcs and a fillet in ONE step),
/// `arc_fillet(Radius)` mid-chain (fused again, with its binders
/// still to come), the re-entry pair `at`/`toward`, a `line`, and the
/// closer. `set_after` is written at each verb and the lifted
/// document is read at every address, so a verb that recorded two
/// steps or none would put every later unit on the wrong leg.
///
/// The direction row is the negative half: an argument written with
/// no notation reads back none, so the assertions above are about
/// what was written rather than about a unit the lift supplies.
#[test]
fn the_derived_index_tracks_every_verb_including_fused_ones() {
    use profile::{ArcSide, ArcSweep, Center, Radius};
    let t = Tol::witness();
    let mut n = RecordedNotation::new();

    let path = Open
        .arc_fillet_arc(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(5.0, 0.0),
            },
            0.5,
            Center {
                c: p2(0.0, 7.0),
                winding: ArcSweep::Cw,
                p: p2(0.0, 4.0),
            },
            t,
        )
        .expect("the fused entry");
    assert_eq!(path.recorded().len(), 1, "a fused verb records ONE step");
    n.set_after(path.recorded(), StepArg::Radius, quantity::MM.def())
        .expect("the fused step's fillet radius is a length");

    let path = path
        .arc_fillet(
            Radius {
                r: 3.0,
                side: ArcSide::Right,
            },
            0.3,
            t,
        )
        .expect("the mid-chain arc extension");
    assert_eq!(path.recorded().len(), 2, "and so does the fused extension");
    n.set_after(path.recorded(), StepArg::Radius, quantity::MM.def())
        .expect("its fillet radius is a length too");

    let path = path.at(p2(-2.0, 2.0), t).expect("the arrival anchor");
    assert_eq!(path.recorded().len(), 3);
    n.set_after(path.recorded(), StepArg::PointX, quantity::MM.def())
        .expect("a point coordinate is a length");

    let path = path.toward(0.0, -1.0, t).expect("the arrival direction");
    let path = path.line(1.0, t).expect("a leg");
    assert_eq!(path.recorded().len(), 5);
    n.set_after(path.recorded(), StepArg::Length, quantity::MM.def())
        .expect("a leg length is a length");

    let closed = path.line_to(Start, t).expect("the chain closes");
    let program = LoopProgram::from_recorded_with_notation(&closed.program, &n)
        .expect("the mixed chain with its notation lifts");
    let doc = doc_of(program);
    assert_eq!(
        read_back(&doc, 0, StepArg::Radius).1,
        "mm",
        "the fused entry"
    );
    assert_eq!(read_back(&doc, 1, StepArg::Radius).1, "mm", "the extension");
    assert_eq!(read_back(&doc, 2, StepArg::PointX).1, "mm", "the anchor");
    assert_eq!(read_back(&doc, 4, StepArg::Length).1, "mm", "the leg");
    assert_eq!(
        read_back(&doc, 3, StepArg::DirX),
        (0.0, ""),
        "the direction was written with no notation (a ratio's own row)"
    );
    assert_eq!(n.len(), 4, "four writes, four legs, no arithmetic anywhere");
}

/// **A `fillet(r)` binder IS the last recorded step**, so its radius
/// takes its notation through the derived door like any leg's
/// argument, and the verb after it is a different step.
///
/// A binder records a step and emits geometry only when the next verb
/// resolves it, which is the one place "the verb I just called" and
/// "the last thing that drew something" come apart. The derived door
/// follows the recording, not the drawing: after `fillet(0.2)` the
/// address is the binder's own, and `get` is read at the index the
/// author never wrote.
#[test]
fn a_fillet_binder_is_the_last_recorded_step() {
    use profile::{ArcSide, Sweep};
    let t = Tol::witness();
    let mut n = RecordedNotation::new();
    let path = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .expect("a departure")
        .arc_to(
            Sweep {
                r: 2.0,
                side: ArcSide::Left,
                angle: 0.6,
            },
            t,
        )
        .expect("an arc leg");
    let path = path.fillet(0.2, t).expect("a fillet binder");
    assert_eq!(path.recorded().len(), 4, "at, angle, arc_to, fillet");
    n.set_after(path.recorded(), StepArg::Radius, quantity::MM.def())
        .expect("a fillet radius is a length");
    assert_eq!(
        n.get(3, StepArg::Radius).map(|u| u.symbol()),
        Some("mm"),
        "the binder is step three, and the author counted nothing"
    );
    let closed = path
        .at(p2(4.0, 3.0), t)
        .expect("the anchor after the fillet")
        .toward(0.0, 1.0, t)
        .expect("the departure after it")
        .line(3.0, t)
        .expect("a leg")
        .line_to(Start, t)
        .expect("the chain closes");
    let program = LoopProgram::from_recorded_with_notation(&closed.program, &n)
        .expect("the filleted chain lifts");
    assert_eq!(read_back(&doc_of(program), 3, StepArg::Radius).1, "mm");
}

/// **An arrival state's author reaches the derived door too.**
///
/// `fillet_arc` into a `Radius` arrival hands back a
/// `profile::RadiusArrival`, not a `PartialPath`: a state that holds
/// the recording and whose binders are still to come. The author has
/// just recorded the fused step whose radius they want to write, so
/// the door is there — every builder state that holds the core
/// answers `recorded()`, which is what keeps the derived index
/// available at the moment the step exists rather than one state
/// later.
///
/// Binding the arrival APPENDS: the prefix the arrival reported is
/// still the prefix of the finished program, so the index written
/// here is the index the lift reads.
#[test]
fn an_arrival_states_author_reaches_the_derived_door() {
    use profile::{ArcSide, Radius};
    let t = Tol::witness();
    let mut n = RecordedNotation::new();
    let path = Open.at(p2(0.0, 0.0)).angle(0.0, t).expect("a departure");
    let arrival = path
        .fillet_arc(
            0.25,
            Radius {
                r: 3.0,
                side: ArcSide::Left,
            },
            t,
        )
        .expect("a fillet into a radius arrival");
    assert_eq!(
        arrival.recorded().len(),
        3,
        "at, angle, fillet_arc — one step per verb, in the arrival state too"
    );
    n.set_after(arrival.recorded(), StepArg::Radius, quantity::MM.def())
        .expect("the fused step's fillet radius is a length");
    assert_eq!(
        n.get(2, StepArg::Radius).map(|u| u.symbol()),
        Some("mm"),
        "the fused step is step two, and the author counted nothing"
    );
    let prefix = format!("{:?}", arrival.recorded());

    let closed = arrival
        .at(p2(6.0, 1.0))
        .toward(0.0, 1.0, t)
        .expect("the arrival's director, which resolves the fillet")
        // The arrival leaves the tip tangent-continuous, so the
        // departure is declared rather than authored as an angle.
        .tangent()
        .line(1.0, t)
        .expect("a leg")
        .line_to(Start, t)
        .expect("the chain closes");
    assert_eq!(
        format!("{:?}", &closed.program[..3]),
        prefix,
        "binding the arrival appended; the prefix the author wrote against did not move"
    );
    let program = LoopProgram::from_recorded_with_notation(&closed.program, &n)
        .expect("the arrival chain with its notation lifts");
    assert_eq!(read_back(&doc_of(program), 2, StepArg::Radius).1, "mm");
}

/// **After the closer the recording is the finished loop's `program`
/// field**, and `set_after` over it addresses the CLOSING step.
///
/// The derived door takes a slice, so it does not care which value
/// the caller is holding — mid-chain the slice comes from
/// `recorded()`, and once the chain has closed the chain is gone and
/// what is left is `ClosedLoop::program`. Those are the two moments
/// and the two spellings, and this row pins the second: the index
/// derived from a finished program is the closer's own, which is why
/// a role the closer does not carry refuses AT that index rather than
/// at the leg before it.
#[test]
fn after_the_closer_the_recording_is_the_finished_loops_program() {
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(0.025, 0.0), t)
        .expect("a leg")
        .line_to(p2(0.025, 0.025), t)
        .expect("a leg")
        .line_to(Start, t)
        .expect("the chain closes");
    // `closed` is a `ClosedLoop`: the chain is over, so the slice is
    // the public field and the index it derives is the CLOSER's.
    let mut n = RecordedNotation::new();
    n.set_after(&closed.program, StepArg::TargetX, quantity::MM.def())
        .expect("mm measures a length");
    let refused = LoopProgram::from_recorded_with_notation(&closed.program, &n)
        .expect_err("the closing LineTo(Start) carries no target coordinate");
    assert_eq!(
        refused,
        RecordedProgramError::NotationOffProgram {
            step: 3,
            arg: StepArg::TargetX
        },
        "the derived index is the closer's own step"
    );
}
