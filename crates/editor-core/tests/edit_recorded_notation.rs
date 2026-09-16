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
//! **What these rows pin, and where the claim is stated.** The reading
//! is written once, on `RecordedNotation`'s own rustdoc — the notation
//! is presentation metadata under D6, it is keyed by the document's own
//! `(step, StepArg)` address, and it is consumed at the lift rather than
//! stored. Here it is executed: the read-back, the bit-blindness of
//! identity and of evaluation, the round trip through save and load, and
//! the four refusals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/program.rs",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture;

use editor_core::{
    CancelToken, DocEdit, DocumentId, EvalOptions, ExprPath, LoopProgram, Node, ProfileDoc,
    ProfileProgram, RecipeNodeId, RecordedNotation, RecordedProgramError, SlotId, StepArg,
    ValuePayload, evaluate, load, save,
};
use geom_core::{Point2, Tol};
use profile::{Open, Start, Step};

/// Every document below is a frame and then the profile drawn on it.
const PLANE: RecipeNodeId = RecipeNodeId(0);
const PROFILE: RecipeNodeId = RecipeNodeId(1);

/// The leg this suite is about: step 1's target, the corner a path
/// author writes as `line_to((25 mm, 0 mm))`.
const LEG: u32 = 1;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A square of side `side` METRES, recorded through the path algebra
/// exactly as a caller writes it: `At(p0)`, three `LineTo`s, the closer.
fn square(side: f64) -> Vec<Step<f64>> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .line_to(p2(side, 0.0), t)
        .expect("a leg of a square")
        .line_to(p2(side, side), t)
        .expect("a leg of a square")
        .line_to(p2(0.0, side), t)
        .expect("a leg of a square")
        .line_to(Start, t)
        .expect("the square closes")
        .program
}

/// `25 mm`, written down: the notation for the leg a caller authored in
/// millimetres. The value in the recording is the canonical `0.025`,
/// because that is what a recording holds.
fn in_millimetres() -> RecordedNotation {
    let mut n = RecordedNotation::new();
    for arg in [StepArg::TargetX, StepArg::TargetY] {
        n.set(LEG, arg, quantity::MM.def())
            .expect("mm measures a length, and a target coordinate is one");
    }
    n
}

/// The document a lifted program reaches: a frame, then the profile.
fn doc_of(program: LoopProgram) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("edit-recorded-notation"), Tol::witness());
    for edit in edits_of(program) {
        doc = doc
            .apply(&edit, Tol::witness())
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
fn arg_bits(program: LoopProgram) -> Vec<(u32, StepArg, Option<f64>, Option<&'static str>)> {
    let doc = doc_of(program);
    let mut out = Vec::new();
    for step in 0..4 {
        for arg in [
            StepArg::PointX,
            StepArg::PointY,
            StepArg::TargetX,
            StepArg::TargetY,
        ] {
            if let Some(e) = doc.expr_at(&slot(step, arg)) {
                out.push((
                    step,
                    arg,
                    e.literal_value(),
                    e.display_unit().map(|u| u.symbol()),
                ));
            }
        }
    }
    out
}

/// The replayed loop's vertices, bit for bit — what "one geometry"
/// means where two documents are compared.
fn vertex_bits(doc: &ProfileDoc) -> Vec<(u64, u64)> {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
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
    let program = LoopProgram::from_recorded_with_notation(&square(0.025), &in_millimetres())
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
    let program = LoopProgram::from_recorded_with_notation(&square(0.025), &in_millimetres())
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
    assert_eq!(arg_bits(plain.clone()), arg_bits(empty.clone()));
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
    let steps = square(0.025);
    let millimetres = LoopProgram::from_recorded_with_notation(&steps, &in_millimetres())
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
    let program =
        LoopProgram::from_recorded_with_notation(&square(0.025), &in_millimetres()).expect("lifts");
    let base = ProfileDoc::empty(DocumentId::derive("edit-recorded-notation"), Tol::witness());
    let edits = edits_of(program.clone());
    let text = save(&base, &edits, Tol::witness()).expect("the log saves");
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
    n.set(0, StepArg::Radius, quantity::MM.def())
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
// The refusals
// ------------------------------------------------------------------

/// A unit must measure what its role holds, and the refusal lands at
/// the door where the caller writes it rather than at the lift.
///
/// The Scalar roles are where this bites: a bulge and a director
/// component are ratios, so the only unit they admit is the
/// dimensionless row every Scalar literal already carries — which is
/// what "a scalar argument carries no notation" means, executed.
#[test]
fn a_unit_must_measure_what_its_role_holds() {
    let mut n = RecordedNotation::new();
    for (arg, unit) in [
        (StepArg::Bulge, quantity::MM.def()),
        (StepArg::DirX, quantity::DEG.def()),
        (StepArg::TargetX, quantity::DEG.def()),
        (StepArg::TurnVal, quantity::MM.def()),
    ] {
        let refused = n
            .set(0, arg, unit)
            .expect_err("the unit does not measure what the role holds");
        assert_eq!(
            refused.to_string(),
            format!(
                "the display unit measures {} but the literal is {}",
                editor_core::UnitSym::from_def(&unit).measures(),
                arg.dimension()
            )
        );
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
#[test]
fn a_notation_entry_off_the_program_refuses() {
    let steps = square(0.025);
    for (step, arg, why) in [
        (99, StepArg::TargetX, "a step past the program's end"),
        (LEG, StepArg::ViaX, "a role a LineTo does not carry"),
        (0, StepArg::TargetX, "a role the entry step does not carry"),
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
        assert_eq!(
            refused.to_string(),
            format!(
                "the notation names the {} of step {step}, which this recording has no argument at",
                arg.label()
            )
        );
    }
}
