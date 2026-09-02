//! **The whole profile vocabulary, authored headlessly**: the PATHS
//! verb set as a form's currency (`viewer::sketch`), lowered to the
//! document's recorded programs, replayed for the picture a form
//! shows, and driven through the real `AddProfile` door to a solid.
//!
//! # What the preview rows are actually claiming
//!
//! `sketch::preview` runs the SAME ladder the edit door runs on
//! commit — lower, resolve, replay, validate — and the point of the
//! rows below is that the two cannot disagree: a walk the door
//! refuses is a walk the preview refuses, in the same words, and a
//! chain that previews is a chain the door accepts. That is why the
//! ill-typed row asserts against BOTH surfaces rather than trusting
//! one of them.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use common::{insert, len, shape};
use pncad::document::{Doc, ValuePayload};
use pncad::geom_core::Tol;
use pncad::profile::{ArcSide, ArcSweep, SketchPlane, TipState, Verb};
use viewer::session::{DocSession, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{ArcSpec, PathStep, PathTarget, PreviewError, preview};

/// The flattening tolerance the rows read at — a tenth of a
/// millimetre, fine enough that a circle's points land on it to well
/// inside the assertions below.
const CHORD: f64 = 1.0e-4;

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("path-start", tol), tol)
}

/// A closed square, authored the way the form does: bind the entry,
/// three legs, then a leg that targets the start.
fn square(side: f64) -> ProfileShape {
    ProfileShape::Path {
        steps: vec![
            PathStep::At([0.0, 0.0]),
            PathStep::LineTo(PathTarget::Point([side, 0.0])),
            PathStep::LineTo(PathTarget::Point([side, side])),
            PathStep::LineTo(PathTarget::Point([0.0, side])),
            PathStep::LineTo(PathTarget::Start),
        ],
    }
}

/// **A chain previews as the polygon it spells**, and the same chain
/// goes through the creation door to a real body.
///
/// The preview and the commit are the two consumers of one lowering;
/// a row that only checked the picture would not notice a chain the
/// door refuses, and one that only checked the door would not notice
/// a picture drawn from different numbers.
#[test]
fn a_line_chain_previews_and_authors_the_same_square() {
    let tol = Tol::witness();
    let side = 0.02;
    let drawn = preview(SketchPlane::xy(), &[square(side)], tol, CHORD).expect("the square closes");
    assert!(drawn.invalid.is_none(), "{:?}", drawn.invalid);
    assert_eq!(drawn.loops.len(), 1);
    // Four corners and no subdivision: a straight leg has no sag to
    // answer for, so the flattener adds nothing between its ends.
    assert!(
        drawn.loops[0].closed,
        "the square's chain closes on its own"
    );
    assert_eq!(
        drawn.loops[0].points,
        vec![[0.0, 0.0], [side, 0.0], [side, side], [0.0, side]],
    );

    let mut session = session(tol);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![shape(&square(side))],
        },
    );
    let extrude = insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: len(0.01),
        },
    );
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    assert!(
        matches!(
            &eval.value(extrude).expect("the extrude evaluated").payload,
            ValuePayload::Body(_)
        ),
        "a chain-authored profile extrudes like any other",
    );
}

/// **An arc leg is flattened, not straightened.** A half circle
/// authored as one `arc_to` bulge leg comes back as a run of points
/// every one of which is on the carrier, to within the chord
/// tolerance the caller asked for.
#[test]
fn an_arc_leg_flattens_onto_its_own_carrier() {
    let tol = Tol::witness();
    let radius = 0.01;
    // A half turn: b = tan(θ/4) = tan(π/4) = 1 over the diameter.
    let template = ProfileShape::Path {
        steps: vec![
            PathStep::At([-radius, 0.0]),
            PathStep::ArcTo(ArcSpec::Bulge {
                target: PathTarget::Point([radius, 0.0]),
                b: 1.0,
            }),
            PathStep::LineTo(PathTarget::Start),
        ],
    };
    let drawn = preview(
        SketchPlane::xy(),
        std::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect("the half disc closes");
    let points = &drawn.loops[0].points;
    assert!(
        points.len() > 8,
        "a half circle is subdivided, not chorded: {} points",
        points.len(),
    );
    for point in points {
        let from_centre = (point[0] * point[0] + point[1] * point[1]).sqrt();
        assert!(
            (from_centre - radius).abs() <= CHORD,
            "{point:?} is {from_centre} from the centre, not {radius}",
        );
    }
    // **Which way it bulges is the convention, so it is asserted.**
    // A positive bulge travels COUNTERCLOCKWISE, and counterclockwise
    // from (-r, 0) is the way that goes DOWN: the tangent there is
    // -y, the carrier's centre is to the left of it, and the arc's
    // own midpoint is (0, -r). An arc drawn through (0, +r) instead
    // would be the same carrier travelled the other way — the one
    // mistake a bulge flattener makes, and the one this pins.
    assert!(
        points.iter().all(|p| p[1] <= CHORD),
        "the arc stays on one side of its chord",
    );
    let bottom = points
        .iter()
        .copied()
        .fold(f64::INFINITY, |lowest, p| lowest.min(p[1]));
    assert!(
        (bottom + radius).abs() <= CHORD,
        "the arc reaches its own midpoint at {bottom}, not {}",
        -radius,
    );
}

/// **An ill-typed walk refuses, and the preview and the commit door
/// refuse the same one.**
///
/// `tangent` needs an incoming carrier to be tangent TO; at a plain
/// bound point there is none, and the lattice says so. The preview
/// names the tip's state and the verb; the door refuses the edit.
#[test]
fn an_illegal_walk_refuses_at_the_preview_and_at_the_door() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![PathStep::At([0.0, 0.0]), PathStep::Tangent],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("a tangent off a plain point is ill-typed");
    assert!(
        matches!(
            refusal,
            PreviewError::Transition {
                loop_: 0,
                step: 1,
                state: TipState::PlainPoint,
                verb: Some(Verb::Tangent),
            }
        ),
        "{refusal}",
    );

    let mut session = session(tol);
    let out = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: vec![shape(&template)],
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Edit(_))),
        "the door refuses it too: {:?}",
        out.refusal,
    );
    assert!(
        session.committed_doc().order().is_empty(),
        "and nothing landed",
    );
}

/// **A chain that has not closed yet DRAWS, and is still not
/// committable.**
///
/// The two halves are the point. A path is written one step at a time,
/// so refusing to draw it until the last step lands is a preview that
/// arrives when it is no longer needed — the chain is therefore
/// replayed under a provisional close and marked open, and the legs
/// that were authored are exactly the legs that come back. What does
/// NOT move is the door: a program that does not close is not a loop,
/// and the edit refuses it as it always did.
#[test]
fn an_unclosed_chain_draws_its_authored_legs_and_still_refuses_at_the_door() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            PathStep::At([0.0, 0.0]),
            PathStep::LineTo(PathTarget::Point([0.01, 0.0])),
            PathStep::LineTo(PathTarget::Point([0.01, 0.01])),
        ],
    };
    let drawn = preview(
        SketchPlane::xy(),
        std::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect("an unfinished chain still draws what it has");
    assert_eq!(drawn.loops.len(), 1);
    assert!(
        !drawn.loops[0].closed,
        "the chain has no closing verb, and the preview says so",
    );
    assert!(drawn.has_open_chain());
    // The authored vertices, and ONLY those: the provisional
    // `line_to Start` contributes no point of its own, so the polyline
    // is the three legs the person wrote.
    assert_eq!(
        drawn.loops[0].points,
        vec![[0.0, 0.0], [0.01, 0.0], [0.01, 0.01]],
    );
    // An unfinished chain is not a profile, so there is no validation
    // verdict to report about it.
    assert!(drawn.invalid.is_none(), "{:?}", drawn.invalid);

    let mut session = session(tol);
    let out = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: vec![shape(&template)],
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Edit(_))),
        "the door still refuses a chain that does not close: {:?}",
        out.refusal,
    );
    assert!(
        session.committed_doc().order().is_empty(),
        "and nothing landed",
    );
}

/// **A chain whose provisional close is itself ill-typed reports the
/// ORIGINAL refusal.**
///
/// `angle` binds a direction and leaves the position pending, and no
/// `line_to` is well-typed there — so the close this module appends to
/// draw an unfinished chain cannot be walked either. The refusal a
/// reader gets is the end-of-program one, about the program they
/// wrote, never one about a step nobody authored.
#[test]
fn an_unclosable_chain_reports_the_refusal_for_the_program_that_was_written() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![PathStep::At([0.0, 0.0]), PathStep::Angle(0.0)],
    };
    let refusal = preview(
        SketchPlane::xy(),
        std::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("a bound direction with no position cannot be closed");
    assert!(
        matches!(
            refusal,
            PreviewError::Transition {
                loop_: 0,
                step: 2,
                verb: None,
                ..
            }
        ),
        "{refusal}",
    );
}

/// **A preview that replays but does not validate still draws.**
///
/// Two loops that cross are geometry a person needs to LOOK at to see
/// what is wrong with them, so the refusal rides beside the picture
/// rather than replacing it. The commit door still refuses the edit.
#[test]
fn an_invalid_profile_is_drawn_with_its_refusal_beside_it() {
    let tol = Tol::witness();
    let overlapping = vec![
        ProfileShape::Circle {
            centre: [0.0, 0.0],
            radius: 0.01,
        },
        ProfileShape::Circle {
            centre: [0.015, 0.0],
            radius: 0.01,
        },
    ];
    let drawn = preview(SketchPlane::xy(), &overlapping, tol, CHORD).expect("both loops replay");
    assert_eq!(drawn.loops.len(), 2, "both are drawn");
    assert!(
        drawn.invalid.is_some(),
        "two crossing loops are not a profile",
    );

    let mut session = session(tol);
    let out = session.perform(SessionOp::AddProfile {
        plane: SketchPlane::xy(),
        loops: overlapping.iter().map(shape).collect(),
    });
    assert!(out.refusal.is_some(), "the door refuses what it drew");
}

/// **Every verb lowers.**
///
/// The census the exhaustive lowering deserves: each arm of the step
/// vocabulary is put through the door that mints its `Expr` slots, so
/// a verb that lowers to a dimension mismatch — a radius minted as an
/// angle — is caught here rather than at somebody's first click. The
/// WALK is not the subject: these steps are not a legal chain and are
/// not asked to be.
#[test]
fn every_authoring_verb_lowers_to_its_recorded_step() {
    let arc = ArcSpec::Radius {
        r: 0.01,
        side: ArcSide::Left,
    };
    let steps = vec![
        PathStep::At([0.0, 0.001]),
        PathStep::Angle(0.5),
        PathStep::Toward { dx: 1.0, dy: 0.0 },
        PathStep::Tangent,
        PathStep::Cusp,
        PathStep::Turn(0.25),
        PathStep::Line(0.01),
        PathStep::LineTo(PathTarget::Point([0.01, 0.0])),
        PathStep::ArcTo(arc),
        PathStep::ArcTo(ArcSpec::Bulge {
            target: PathTarget::Start,
            b: 0.5,
        }),
        PathStep::ArcTo(ArcSpec::Via {
            q: [0.005, 0.005],
            target: PathTarget::Point([0.01, 0.0]),
        }),
        PathStep::ArcTo(ArcSpec::Center {
            c: [0.0, 0.0],
            winding: ArcSweep::Ccw,
            target: PathTarget::Point([0.01, 0.0]),
        }),
        PathStep::ArcTo(ArcSpec::Sweep {
            r: 0.01,
            side: ArcSide::Right,
            angle: 1.0,
        }),
        PathStep::ArcTo(ArcSpec::ArcLen {
            r: 0.01,
            side: ArcSide::Left,
            len: 0.005,
        }),
        PathStep::TangentArcTo(PathTarget::Start),
        PathStep::ArcContinue([0.002, 0.002]),
        PathStep::Fillet(0.001),
        PathStep::FilletArc {
            radius: 0.001,
            spec: arc,
        },
        PathStep::ArcFillet {
            spec: arc,
            radius: 0.001,
        },
        PathStep::ArcFilletArc {
            spec: arc,
            radius: 0.001,
            spec2: arc,
        },
        PathStep::FarEndTo([0.02, 0.0]),
        PathStep::CloseTo,
    ];
    // **The census is the COMPILER's, not a number written here.**
    // `ordinal` is an exhaustive match, so a verb added to the
    // vocabulary does not compile until somebody gives it a number —
    // and the ordinals covered here have to run from 0 with no hole,
    // so a verb slotted into the middle of the list is caught the
    // moment it has one. A hand-written count caught neither.
    //
    // The residual, stated because the row cannot close it: a verb
    // given an ordinal PAST `CloseTo`'s extends a range this sweep
    // does not know the end of, and would go untested. `ordinal`'s
    // own docs carry the obligation that answers it — new verbs take
    // a number before `CloseTo`'s, and `CloseTo` stays last.
    let covered: std::collections::BTreeSet<usize> = steps.iter().map(ordinal).collect();
    let contiguous: std::collections::BTreeSet<usize> = (0..covered.len()).collect();
    assert_eq!(
        covered, contiguous,
        "the verbs covered here leave a hole: every ordinal from 0 needs a case",
    );
    assert_eq!(
        covered.len(),
        ordinal(&PathStep::CloseTo) + 1,
        "the vocabulary is bigger than this row covers — see `ordinal`'s obligation",
    );

    shape(&ProfileShape::Path { steps });
}

/// Each verb's position in the vocabulary, as an exhaustive match —
/// the census's oracle.
///
/// **A verb added to `PathStep` takes a number BEFORE `CloseTo`'s,
/// and `CloseTo` keeps the last one.** The row above reads the
/// vocabulary's size off `CloseTo` — it has no other way to know it —
/// so a verb numbered past it would be a verb the census never asks
/// about. Renumbering the arms below is free; the obligation is only
/// that `CloseTo` ends them.
fn ordinal(step: &PathStep) -> usize {
    match step {
        PathStep::At(_) => 0,
        PathStep::Angle(_) => 1,
        PathStep::Toward { .. } => 2,
        PathStep::Tangent => 3,
        PathStep::Cusp => 4,
        PathStep::Turn(_) => 5,
        PathStep::Line(_) => 6,
        PathStep::LineTo(_) => 7,
        PathStep::ArcTo(_) => 8,
        PathStep::TangentArcTo(_) => 9,
        PathStep::ArcContinue(_) => 10,
        PathStep::Fillet(_) => 11,
        PathStep::FilletArc { .. } => 12,
        PathStep::ArcFillet { .. } => 13,
        PathStep::ArcFilletArc { .. } => 14,
        PathStep::FarEndTo(_) => 15,
        PathStep::CloseTo => 16,
    }
}

/// A non-finite field refuses at the lowering, before anything is
/// replayed — the literal constructors' one door, and the only thing
/// this layer judges.
#[test]
fn a_non_finite_field_refuses_at_the_lowering() {
    let template = ProfileShape::Path {
        steps: vec![PathStep::At([f64::NAN, 0.0])],
    };
    let refusal = preview(
        SketchPlane::xy(),
        std::slice::from_ref(&template),
        Tol::witness(),
        CHORD,
    )
    .expect_err("NaN is not a coordinate");
    assert!(matches!(refusal, PreviewError::Dimension(_)), "{refusal}",);
}
