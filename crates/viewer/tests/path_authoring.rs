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

use crate::common;

use common::{insert, len, shape};
use pncad::document::{Doc, ValuePayload};
use pncad::document::{LoopProgram, ProgramArcData, ProgramStep, ProgramTarget};
use pncad::geom_core::Point2;
use pncad::geom_core::Tol;
use pncad::profile::{
    ArcData, ArcMode, ReplayErrorKind, SketchPlane, Step, Target, TargetKind, TipState, Verb,
};
use viewer::session::{DocSession, ProfileShape, Refusal, SessionOp};
use viewer::sketch::{self, Notation, PreviewError, admits_at, preview};

/// The flattening tolerance the rows read at — a tenth of a
/// millimetre, fine enough that a circle's points land on it to well
/// inside the assertions below.
const CHORD: f64 = 1.0e-4;

/// A display tolerance coarse enough that an arc of ordinary size
/// sags less than it over its whole sweep, so `arc_points` answers the
/// one-segment floor. One row needs that arm and nothing else here
/// does; it is a δ the caller chooses, not a property of the geometry.
const COARSE_CHORD: f64 = 1.0e-1;

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("path-start", tol), tol)
}

/// A point of the sketch frame, in metres.
fn pt(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A closed square, authored the way the form does: bind the entry,
/// three legs, then a leg that targets the start.
fn square(side: f64) -> ProfileShape {
    ProfileShape::Path {
        steps: vec![
            Step::At(pt(0.0, 0.0)),
            Step::LineTo(Target::Point(pt(side, 0.0))),
            Step::LineTo(Target::Point(pt(side, side))),
            Step::LineTo(Target::Point(pt(0.0, side))),
            Step::LineTo(Target::Start),
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
    let plane = common::xy_frame_in(&mut session);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane,
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
            Step::At(pt(-radius, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(radius, 0.0)),
                b: 1.0,
            }),
            Step::LineTo(Target::Start),
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
        steps: vec![Step::At(pt(0.0, 0.0)), Step::Tangent],
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
    // The sentence is about the step the author wrote, so it names the
    // verb the way they wrote it — `profile::Verb`'s own `Display`,
    // which is the word the row's combo offers — and not the variant
    // identifier, which is what the table-COORDINATE sentence renders
    // (`profile`'s `ReplayError`).
    let said = refusal.to_string();
    assert!(said.contains("tangent"), "{said}");
    assert!(!said.contains("Tangent"), "{said}");

    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    let out = session.perform(SessionOp::AddProfile {
        plane,
        loops: vec![shape(&template)],
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Edit(_))),
        "the door refuses it too: {:?}",
        out.refusal,
    );
    assert_eq!(
        session.committed_doc().order(),
        &[plane][..],
        "and nothing landed — the frame the profile would have named is \
         all the document holds",
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
            Step::At(pt(0.0, 0.0)),
            Step::LineTo(Target::Point(pt(0.01, 0.0))),
            Step::LineTo(Target::Point(pt(0.01, 0.01))),
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
    let plane = common::xy_frame_in(&mut session);
    let out = session.perform(SessionOp::AddProfile {
        plane,
        loops: vec![shape(&template)],
    });
    assert!(
        matches!(out.refusal, Some(Refusal::Edit(_))),
        "the door still refuses a chain that does not close: {:?}",
        out.refusal,
    );
    assert_eq!(
        session.committed_doc().order(),
        &[plane][..],
        "and nothing landed — the frame the profile would have named is \
         all the document holds",
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
        steps: vec![Step::At(pt(0.0, 0.0)), Step::Angle(0.0)],
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
    let plane = common::xy_frame_in(&mut session);
    let out = session.perform(SessionOp::AddProfile {
        plane,
        loops: overlapping.iter().map(shape).collect(),
    });
    assert!(out.refusal.is_some(), "the door refuses what it drew");
}

/// **Every verb lowers, at every arc mode and every target form.**
///
/// The census the form's vocabulary deserves, keyed on the KERNEL's
/// own lists: each verb the transition table declares is taken at the
/// step the form starts it as (`sketch::fresh_step`) and put through
/// the door that mints its `Expr` slots, so a starting step the lift
/// refuses — a placeholder that is not a finite literal, a
/// complete-loop verb filed as a chain step — is caught here rather
/// than at somebody's first click. The same goes for each arc mode
/// inside `arc_to` and each target form inside `line_to` and a bulge
/// arc. The WALK is not the subject: each step is lowered alone and is
/// not asked to be a legal chain. Which DIMENSION each argument is
/// minted at is the lift's own table, and
/// `a_path_authored_in_millimetres_remembers_its_notation` below is the
/// row that reads it back.
#[test]
fn every_authoring_verb_lowers_to_its_recorded_step() {
    let mut steps: Vec<Step<f64>> = Verb::ALL
        .iter()
        .map(|&verb| sketch::fresh_step(verb))
        .collect();
    steps.extend(
        ArcMode::ALL
            .iter()
            .map(|&mode| Step::ArcTo(sketch::fresh_arc(mode))),
    );
    for &kind in TargetKind::ALL {
        steps.push(Step::LineTo(sketch::fresh_target(kind)));
        steps.push(Step::ArcTo(ArcData::Bulge {
            target: sketch::fresh_target(kind),
            b: 0.5,
        }));
    }
    for step in steps {
        let verb = step.verb();
        let program = shape(&ProfileShape::Path { steps: vec![step] });
        // A verb the lift misfiles is caught as a count: the complete-
        // loop verbs become their own program forms, and every other
        // verb one step of a chain.
        match (verb, &program) {
            (Verb::Circle, LoopProgram::Circle { .. })
            | (Verb::CircleSplit, LoopProgram::CircleSplit { .. }) => {}
            (_, LoopProgram::Chain(lowered)) => assert_eq!(lowered.len(), 1, "{verb}"),
            (verb, program) => panic!("{verb} lowered to {program:?}"),
        }
    }
}

/// **A path takes the form's notation at every argument that has
/// one**: its lengths remember millimetres and its angles degrees, and
/// a dimensionless argument is written as any dimensionless literal
/// is, because it has one spelling.
///
/// The notation is written over whatever arguments the lifted program
/// reports holding, so a row that only read one length would not
/// notice an angle left canonical, or a bulge given a unit.
#[test]
fn a_path_authored_in_millimetres_remembers_its_notation() {
    let mm = Notation {
        length: pncad::quantity::MM,
        angle: pncad::quantity::DEG,
    };
    let path = ProfileShape::Path {
        steps: vec![
            Step::At(pt(0.0, 0.001)),
            Step::Angle(0.5),
            Step::Toward { dx: 1.0, dy: 0.0 },
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(0.01, 0.0)),
                b: 0.5,
            }),
        ],
    };
    let LoopProgram::Chain(lowered) = sketch::loop_program(&path, mm).expect("finite literals")
    else {
        panic!("a chain lowers to a chain");
    };
    let written = |expr: &pncad::document::Expr| expr.display_unit().map(|unit| unit.symbol());
    let [
        ProgramStep::At([x, y]),
        ProgramStep::Angle(theta),
        ProgramStep::Toward { dx, dy },
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point([tx, ty]),
            b,
        }),
    ] = lowered.as_slice()
    else {
        panic!("the steps lower one for one: {lowered:?}");
    };
    for length in [x, y, tx, ty] {
        assert_eq!(written(length), Some("mm"));
    }
    assert_eq!(written(theta), Some("deg"));
    // A dimensionless argument is written the one way a dimensionless
    // literal is, whatever the form's notation says.
    let plain = pncad::document::Expr::literal(1.0, pncad::document::Dimension::Scalar)
        .expect("a finite scalar");
    for scalar in [dx, dy, b] {
        assert_eq!(written(scalar), written(&plain));
    }
}

/// **The form's starting step for every verb is the verb it was asked
/// for.** `sketch::fresh_step` is exhaustive on `Verb`, which holds
/// that every verb HAS a starting step; this holds that each one is
/// the right verb's, which a match arm copied from its neighbour would
/// break while still compiling.
#[test]
fn every_verbs_starting_step_names_that_verb() {
    for &verb in Verb::ALL {
        assert_eq!(sketch::fresh_step(verb).verb(), verb);
    }
    for &mode in ArcMode::ALL {
        assert_eq!(sketch::fresh_arc(mode).mode(), mode);
    }
    for &kind in TargetKind::ALL {
        assert_eq!(sketch::fresh_target(kind).kind(), kind);
    }
}

/// **The declared straight continuation and the declared seam arrival
/// author through the form** — the two things it could not say while
/// it held a copy of the kernel's step rather than the step itself.
///
/// A square whose left side is cut at its midpoint and continued with
/// `continue_to`, and whose entry sits mid-way along the bottom side,
/// so the closing leg arrives continuing the side the entry leaves
/// along: a TANGENT seam, which is refused unless the target declares
/// it.
#[test]
fn continue_to_and_the_declared_arrival_author_through_the_door() {
    let tol = Tol::witness();
    let steps = vec![
        Step::At(pt(0.005, 0.0)),
        Step::LineTo(Target::Point(pt(0.01, 0.0))),
        Step::LineTo(Target::Point(pt(0.01, 0.01))),
        Step::LineTo(Target::Point(pt(0.0, 0.01))),
        Step::LineTo(Target::Point(pt(0.0, 0.005))),
        Step::ContinueTo(Target::Point(pt(0.0, 0.0))),
        Step::LineTo(Target::StartArriving),
    ];
    let drawn = preview(
        SketchPlane::xy(),
        &[ProfileShape::Path {
            steps: steps.clone(),
        }],
        tol,
        CHORD,
    )
    .expect("the declared seam previews");
    assert!(drawn.invalid.is_none(), "{:?}", drawn.invalid);
    assert!(drawn.loops[0].closed);

    // The same seam UNDECLARED is the refusal whose sentence names the
    // declaration — the one a person using this form now can act on.
    let mut undeclared = steps.clone();
    undeclared[6] = Step::LineTo(Target::Start);
    let refusal = preview(
        SketchPlane::xy(),
        &[ProfileShape::Path { steps: undeclared }],
        tol,
        CHORD,
    )
    .expect_err("an undeclared tangent seam is refused");
    assert!(
        matches!(
            &refusal,
            PreviewError::Geometry { step: 6, rendered, .. } if rendered.contains("arrives_tangent")
        ),
        "{refusal}",
    );

    let mut session = session(tol);
    let plane = common::xy_frame_in(&mut session);
    insert(
        &mut session,
        SessionOp::AddProfile {
            plane,
            loops: vec![shape(&ProfileShape::Path { steps })],
        },
    );
}

/// **The lattice, not the form, decides where a verb is offered** —
/// and the kernel's complete-loop verbs are offered only where a loop
/// can begin.
#[test]
fn a_complete_loop_verb_is_admitted_only_at_the_entry() {
    let tol = Tol::witness();
    let circle = sketch::fresh_step(Verb::Circle);
    let entry = sketch::tip_state_at(&[circle], 0, tol);
    assert_eq!(entry, Some(TipState::Entry));
    assert!(admits_at(entry, Verb::Circle).is_ok());
    let chain = [sketch::fresh_step(Verb::At), circle];
    let state = sketch::tip_state_at(&chain, 1, tol);
    assert_eq!(
        admits_at(state, Verb::Circle),
        Err(TipState::PlainPoint),
        "a circle cannot follow a verb",
    );
}

/// **An arc-spec verb is offered wherever its row is, and lands in a
/// form its row takes.** The combo used to judge `arc_to` by one fixed
/// starting spec — `Radius`, which no `arc_to` row admits — so it was
/// greyed at every tip. At a leg end (a `tangent_arc_to`'s end, the
/// middle of a `B`) it is offered and starts endpoint-full; over a
/// bound direction it starts endpoint-free; and either way the chain
/// it lands in replays without a lattice refusal at that step.
#[test]
fn an_arc_spec_verb_starts_in_a_form_its_row_takes() {
    let tol = Tol::witness();
    let leg_end = vec![
        Step::At(pt(0.0, 0.0)),
        Step::Angle(0.0),
        Step::TangentArcTo(Target::Point(pt(0.0, 0.01))),
    ];
    let directed = vec![Step::At(pt(0.0, 0.0)), Step::Angle(0.0)];
    for (prefix, want) in [
        (&leg_end, TipState::DirectedPoint),
        (&directed, TipState::DirectedPlain),
    ] {
        let at = prefix.len();
        let state = sketch::tip_state_at(prefix, at, tol);
        assert_eq!(state, Some(want));
        for verb in [
            Verb::ArcTo,
            Verb::FilletArc,
            Verb::ArcFillet,
            Verb::ArcFilletArc,
        ] {
            assert!(admits_at(state, verb).is_ok(), "{verb} at {want:?}");
            let mut chain = prefix.clone();
            chain.push(sketch::fresh_step_at(verb, state));
            let refused = pncad::profile::replay(&chain, tol).err().filter(|e| {
                e.step == at && matches!(e.kind, ReplayErrorKind::Transition { verb: Some(_), .. })
            });
            assert!(refused.is_none(), "{verb} at {want:?}: {refused:?}");
        }
    }
    let Step::ArcTo(spec) = sketch::fresh_step_at(Verb::ArcTo, Some(TipState::DirectedPoint))
    else {
        unreachable!("fresh_step_at names the verb it was asked for");
    };
    assert_eq!(spec.mode(), ArcMode::Bulge);
    let Step::ArcTo(spec) = sketch::fresh_step_at(Verb::ArcTo, Some(TipState::DirectedPlain))
    else {
        unreachable!("fresh_step_at names the verb it was asked for");
    };
    assert_eq!(spec.mode(), ArcMode::Sweep);
}

/// A non-finite field refuses at the lowering, before anything is
/// replayed — the literal constructors' one door, and the only thing
/// this layer judges.
#[test]
fn a_non_finite_field_refuses_at_the_lowering() {
    let template = ProfileShape::Path {
        steps: vec![Step::At(pt(f64::NAN, 0.0))],
    };
    let refusal = preview(
        SketchPlane::xy(),
        std::slice::from_ref(&template),
        Tol::witness(),
        CHORD,
    )
    .expect_err("NaN is not a coordinate");
    assert!(matches!(refusal, PreviewError::Lowering(_)), "{refusal}",);
}

/// **An arc whose radius is not a number refuses, rather than being
/// drawn at coordinates that are not numbers.**
///
/// A bulge of `1e-320` is a finite literal — `Expr::literal` accepts
/// it, and `widgets::named_scalar` is an ordinary field a person types
/// it into — so nothing upstream of the flattener has a reason to
/// refuse. What it makes is `theta = 4e-320`, `sin(theta/2)` of the
/// same order, and a radius of `inf`; every point along such an arc is
/// `±inf` or a `NaN`. `arc_points` answers `None` for it and
/// `sketch::flatten` turns that into this refusal.
///
/// **The two-vertex version of this loop never gets here**, which is
/// worth the sentence because it is the shape the defect was first
/// written down in: an arc straight across a chord and a closing leg
/// back makes the seam reverse onto itself, and the driver refuses it
/// as an undeclared cusp two steps earlier. It takes a third vertex
/// for the arc to reach the flattener at all.
#[test]
fn an_arc_whose_radius_is_not_a_number_refuses_at_the_preview() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(0.0, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(0.01, 0.0)),
                b: 1.0e-320,
            }),
            Step::LineTo(Target::Point(pt(0.005, 0.01))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("an arc of infinite radius has no drawable shape");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 0
            }
        ),
        "{refusal}",
    );
}

/// **An arc whose CENTRE overflows refuses too, and it is a different
/// arm from the row above.**
///
/// Here the radius is an ordinary finite number — about `5e306` for
/// this chord — so `arc_points` answers `Some(256)` and refuses
/// nothing. What is not a number is the centre: the chord's own
/// midpoint is `(1.6e308 + 1.5e308) / 2`, which overflows on its way
/// to a value that would have been representable, and the arc's
/// points are all `centre + radius·(cos, sin)`.
///
/// So `radius` does not carry the whole frame. It carries the
/// apothem — `apothem = ±radius·cos(θ/2)` — but not the midpoint the
/// apothem is measured from, and this row is the population that
/// distinction produces. Deleting `flatten`'s `centre`/`start` check
/// leaves the row above green and reds this one.
#[test]
fn an_arc_whose_centre_overflows_refuses_at_the_preview() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(1.6e308, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(1.5e308, 0.0)),
                b: 1.0,
            }),
            Step::LineTo(Target::Point(pt(1.55e308, 1.0e307))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("an arc about a centre that is not a point has no drawable shape");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 0
            }
        ),
        "{refusal}",
    );
}

/// **The refusal is a refusal and not a shortened loop**, which is
/// what `flatten`'s own doc claims and what nothing else here would
/// notice. Skipping the segment instead would leave `preview`
/// answering `Ok` with a loop drawn along a leg its author never
/// wrote — a straight chord standing in for an arc — and every row
/// above would stay green through it.
#[test]
fn an_undrawable_arc_is_refused_and_not_skipped() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(0.0, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(0.01, 0.0)),
                b: 1.0e-320,
            }),
            Step::LineTo(Target::Point(pt(0.005, 0.01))),
            Step::LineTo(Target::Start),
        ],
    };
    let drawn = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    );
    assert!(
        drawn.is_err(),
        "the preview drew a loop whose arc it could not flatten",
    );
}

/// The refusal renders as a sentence naming the loop, the vertex and
/// what has no answer — the vocabulary the other preview refusals use,
/// so a form showing this one shows the same kind of thing it shows
/// for an ill-typed walk.
#[test]
fn an_undrawable_arcs_refusal_says_which_vertex_and_why() {
    let refusal = PreviewError::Unflattenable {
        loop_: 2,
        vertex: 7,
    };
    let sentence = refusal.to_string();
    assert!(sentence.contains("loop 2"), "{sentence}");
    assert!(sentence.contains("vertex 7"), "{sentence}");
    assert!(sentence.contains("not a number"), "{sentence}");
}

/// **A vertex the replay put past the top of the exponent range
/// refuses, and no arc is involved anywhere.**
///
/// Every literal here is a finite number and the chain has no bulge
/// at all: `At` at `1e308`, a direction, and a leg of `1e308` along
/// it, whose far end is the sum of the two. `replay` accepts it and
/// hands back a loop whose second vertex is at `inf`.
///
/// That is the population the arc guards cannot reach — they are
/// under a `bulge == 0.0` `continue`, so a polygon passes all of them
/// without ever being asked — and what the flattener used to do with
/// it was emit the point and report success. The vertex the refusal
/// names is the one whose own position is not a place, which is `1`
/// and not the `0` both arc rows name.
#[test]
fn a_vertex_past_the_exponent_range_refuses_at_the_preview() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(1.0e308, 0.0)),
            Step::Toward { dx: 1.0, dy: 0.0 },
            Step::Line(1.0e308),
            Step::LineTo(Target::Point(pt(0.0, 1.0e307))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("a vertex that is not a place has no drawable loop around it");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 1
            }
        ),
        "{refusal}",
    );
}

/// **An arc whose frame is finite and whose far side is not refuses
/// too — a third arm, and the one a frame check cannot see.**
///
/// The two rows above refuse on the frame: a radius that is not a
/// number, a centre that is not a point. Here all four frame values
/// are ordinary — radius about `5.05e307`, centre about
/// `(1.29e308, 0)`, a finite sweep and start — and the arc is major
/// enough to carry its own far side past the top of the range, so
/// nine of the 256 points it draws are at `inf`.
///
/// A guard on the frame is therefore not a guard on the points, which
/// is what makes this the population rather than a fourth instance:
/// the question is asked where a coordinate is MINTED.
#[test]
fn an_arcs_far_side_past_the_range_refuses_though_its_frame_is_finite() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(8.0e307, -1.0e307)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(8.0e307, 1.0e307)),
                b: 10.0,
            }),
            Step::LineTo(Target::Point(pt(0.0, 0.0))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("an arc whose far side is not a place has no drawable shape");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 0
            }
        ),
        "{refusal}",
    );
}

/// **A leg whose separation overflows gets no heading, and the legs
/// beside it still get theirs.**
///
/// `sketch::heading` answers `Option<[f64; 2]>`, so the type says a
/// unit vector or none. The loop here is drawn — every vertex is an
/// ordinary finite number, and `preview` hands back all three — but
/// the diagonal's `dx` and `dy` are each `1.4e308`, and their `hypot`
/// is the one value in this arithmetic that overflows. An infinite
/// length is greater than zero, so the old guard let it through and
/// each component divided by it came back `0.0`.
///
/// Both halves are asserted, because neither says anything alone: a
/// float-valued door that answered its refusal for everything would
/// pass the first, and one that answered a vector for everything
/// would pass the second. The vertex that overflows is the ONLY one
/// that refuses, and the other two answer vectors of length one.
#[test]
fn a_leg_whose_separation_overflows_gets_no_heading() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(-7.0e307, -7.0e307)),
            Step::LineTo(Target::Point(pt(7.0e307, 7.0e307))),
            Step::LineTo(Target::Point(pt(0.0, 7.0e307))),
            Step::LineTo(Target::Start),
        ],
    };
    let drawn = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect("a loop of finite vertices draws");
    let polyline = &drawn.loops[0];
    let points = &polyline.points;
    assert_eq!(points.len(), 3, "{points:?}");
    assert_eq!(
        sketch::heading(points, 0, polyline.closed),
        None,
        "a separation of 1.4e308 in each axis answered a heading",
    );
    for at in [1, 2] {
        let [dx, dy] = sketch::heading(points, at, polyline.closed)
            .unwrap_or_else(|| panic!("vertex {at} of a drawn loop has a heading"));
        let length = dx.hypot(dy);
        assert!(
            (length - 1.0).abs() < 1.0e-12,
            "vertex {at} answered [{dx}, {dy}], of length {length}",
        );
    }
}

/// **A vertex whose Y is past the range refuses, and its X is an
/// ordinary number** — which is the only row here that separates the
/// two coordinates.
///
/// `drawable` asks both, and every other fixture in this file carries
/// its non-finite value in `x`: the vertex row's `inf` is `(inf, 0)`
/// and the arc row's first bad point is `[inf, -4.6e306]`, so
/// weakening the predicate to `point[0].is_finite()` alone leaves the
/// whole viewer suite green. Measured, not supposed: 637 rows passed
/// under exactly that mutation. This is the row that reds it.
#[test]
fn a_vertexs_second_coordinate_is_asked_the_question_too() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(0.0, 1.0e308)),
            Step::Toward { dx: 0.0, dy: 1.0 },
            Step::Line(1.0e308),
            Step::LineTo(Target::Point(pt(1.0e307, 0.0))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        CHORD,
    )
    .expect_err("a vertex whose ordinate is not a number has no drawable loop around it");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 1
            }
        ),
        "{refusal}",
    );
}

/// **The arc FRAME's own question is load-bearing, and only a
/// one-segment arc shows it.**
///
/// Every other undrawable arc here is caught twice over: the frame
/// refuses it, and the points it would have minted are not numbers
/// either, so deleting the frame's `drawable(centre)` leaves the
/// refusal and its ordinal unchanged and no row moves. Measured — the
/// whole file stayed green under that deletion.
///
/// The arm that separates them is `arc_points` answering **one**. A
/// millimetre-scale arc far from the origin — two vertices a
/// millimetre apart at `1.6e308`, bulge `0.5` — has radius
/// `6.25e-4` and a sagitta of `2.5e-4`, so at a COARSE display
/// tolerance it genuinely needs no subdivision and the interior-point
/// loop never runs. Its `start` is `atan2` of a finite ordinate over
/// `-inf`, which is `-π` and perfectly finite. The centre is
/// `[inf, 5e-4]`, and nothing but the frame check asks. Without it the
/// arc is drawn as a straight chord — a leg the author did not write,
/// which is what this module refuses by name.
///
/// **The one-segment answer is bought with the CHORD and not with the
/// geometry**, which is why this row passes its own tolerance rather
/// than the file's. An earlier draft shrank the arc to a micron
/// instead, and the eps = 1e-6 row of the matrix refused its junction
/// at replay two steps before the flattener ever saw it: the turn
/// margin was `3.75e-7 m`, which at that tolerance is tangency. A
/// fixture whose scale is near an eps row's is a fixture about that
/// row. `chord` is the caller's own δ — `pane::viewport` passes the
/// display budget's — so asking for a coarse one is the ordinary
/// thing, and it leaves the geometry three orders of magnitude clear
/// of the coarsest eps the matrix runs.
#[test]
fn a_one_segment_arc_about_a_centre_that_is_not_a_point_refuses() {
    let tol = Tol::witness();
    let template = ProfileShape::Path {
        steps: vec![
            Step::At(pt(1.6e308, 0.0)),
            Step::ArcTo(ArcData::Bulge {
                target: Target::Point(pt(1.6e308, 1.0e-3)),
                b: 0.5,
            }),
            Step::LineTo(Target::Point(pt(1.0e307, 5.0e-4))),
            Step::LineTo(Target::Start),
        ],
    };
    let refusal = preview(
        SketchPlane::xy(),
        core::slice::from_ref(&template),
        tol,
        COARSE_CHORD,
    )
    .expect_err("an arc that draws no interior point still has a centre to be asked about");
    assert!(
        matches!(
            refusal,
            PreviewError::Unflattenable {
                loop_: 0,
                vertex: 0
            }
        ),
        "{refusal}",
    );
}
