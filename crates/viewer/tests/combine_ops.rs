//! **Combining bodies, replayed headlessly** (GAUTH-4): the Phase-B op
//! vocabulary driving the real `DocSession` with no renderer — the
//! boolean / split / transform / pattern doors with their typed
//! refusals, the tool values that feed them, and the rule that only
//! one modal tool is open.
//!
//! # The two boxes every geometric row is about
//!
//! [`two_boxes`] authors an overlapping pair through the creation
//! vocabulary alone (rectangle profile → extrude, and a second one
//! placed by [`SessionOp::AddTransform`]): a 40 × 20 × 10 mm block and
//! a 20 × 10 × 6 mm block offset so that they overlap in a
//! 5 × 10 × 6 mm brick and share NO plane. The constants below are
//! that arrangement's closed form, and every boolean row asserts
//! against them rather than against a recorded number — a row goes red
//! when the kernel's answer stops being the arithmetic one, which is
//! the only thing worth pinning here.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{
    Axis3, BooleanOp, Datum, Dimension, Doc, DocEdit, Expr, Node, PatternKind, RecipeNodeId, SlotId,
};
use pncad::document::{BooleanValue, SplitSide};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::profile::SketchPlane;
use viewer::combine::{
    BooleanTool, CombineToolError, CombineToolEvent, PatternForm, PatternRuleSpec, PatternTool,
    Seat, SplitTool, TransformTool, seat_line,
};
use viewer::session::{
    DatumSpec, DocSession, NodeKindWanted, ProfileShape, Refusal, Selection, SessionOp,
};
use viewer::tools::{ToolKind, Tools};

/// The big block: 40 × 20 × 10 mm, extruded up from the sketch plane.
const A: [f64; 3] = [0.04, 0.02, 0.01];
/// The small block: 20 × 10 × 6 mm, before it is placed.
const B: [f64; 3] = [0.02, 0.01, 0.006];
/// Where the small block is placed — chosen so the two overlap in a
/// brick and no face of one is coplanar with a face of the other.
const OFFSET: [f64; 3] = [0.025, 0.002, 0.002];
/// The volume of the overlap brick: 5 × 10 × 6 mm.
const OVERLAP: f64 = 0.005 * 0.01 * 0.006;

/// A session over a throwaway document.
fn session(tol: Tol) -> DocSession {
    DocSession::inline(Doc::empty_derived("combine-start", tol), tol)
}

/// Perform one op that must commit exactly one insert, answering the
/// id of the node it minted.
fn insert(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "exactly one committed edit");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::InsertNode { .. })
    ));
    *session
        .committed_doc()
        .order()
        .last()
        .expect("the insert landed")
}

/// A box of `size`, authored through the creation doors: one
/// rectangle profile on world XY, one extrude.
fn boxed(session: &mut DocSession, size: [f64; 3]) -> RecipeNodeId {
    let profile = insert(
        session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Rectangle {
                width: size[0],
                height: size[1],
            }],
        },
    );
    insert(
        session,
        SessionOp::AddExtrude {
            profile,
            distance: size[2],
        },
    )
}

/// The two overlapping blocks (module docs), the second placed by a
/// transform. Answers the session and the two BODY nodes, in the order
/// the module's constants name them.
fn two_boxes(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = session(tol);
    let a = boxed(&mut session, A);
    let raw_b = boxed(&mut session, B);
    let b = insert(
        &mut session,
        SessionOp::AddTransform {
            input: raw_b,
            translation: OFFSET,
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: 0.0,
        },
    );
    (session, a, b)
}

/// The evaluated volume of a node's single body, with the seam pumped.
fn body_volume(session: &mut DocSession, node: RecipeNodeId, tol: Tol) -> f64 {
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let body = match &eval.value(node).expect("the node evaluated").payload {
        ValuePayload::Body(body) => body.clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body.clone(),
        other => panic!("expected a body, got {other:?}"),
    };
    pncad::topo::mass_properties(&body, tol)
        .expect("mass properties")
        .volume
}

/// `left` and `right` agree to within a relative tolerance the planar
/// boolean rows can hold.
fn near(left: f64, right: f64) -> bool {
    ((left - right) / right).abs() < 1e-9
}

/// A body's volume, with the closed form it must match.
fn assert_volume(session: &mut DocSession, node: RecipeNodeId, want: f64, tol: Tol) {
    let got = body_volume(session, node, tol);
    assert!(near(got, want), "volume {got} vs {want}");
}

/// **The acceptance row**: a two-body boolean authored from nothing,
/// evaluated, saved, reloaded and re-evaluated.
#[test]
fn a_two_body_union_authors_evaluates_saves_and_reloads() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let union = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a,
            b,
        },
    );
    // `declare: None` is the door's authored value, not a default the
    // op could have carried differently.
    assert!(matches!(
        session.committed_doc().node(union),
        Some(Node::Boolean {
            op: BooleanOp::Union,
            declare: None,
            ..
        })
    ));
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];
    let volume = body_volume(&mut session, union, tol);
    assert!(near(volume, va + vb - OVERLAP), "union volume {volume}");

    let dir = common::tempdir("gauth4-union");
    let path = dir.join("union.pncad");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none(),
        "save"
    );
    let authored = session.committed_doc().clone();
    assert!(
        session.perform(SessionOp::Open(path)).refusal.is_none(),
        "open"
    );
    assert!(
        session.committed_doc().bit_eq(&authored),
        "the reloaded document is the authored one, bit for bit"
    );
    let reloaded = body_volume(&mut session, union, tol);
    assert_eq!(
        volume.to_bits(),
        reloaded.to_bits(),
        "same solid after reload"
    );
}

/// **Difference is not commutative, and the door's operand order is
/// what decides it**: the same two picks in the two orders author two
/// different solids, each the arithmetic one.
#[test]
fn subtraction_is_not_commutative_in_the_authored_order() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);
    let va = A[0] * A[1] * A[2];
    let vb = B[0] * B[1] * B[2];

    let a_minus_b = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a,
            b,
        },
    );
    let b_minus_a = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a: b,
            b: a,
        },
    );
    let first = body_volume(&mut session, a_minus_b, tol);
    let second = body_volume(&mut session, b_minus_a, tol);
    assert!(near(first, va - OVERLAP), "a ∖ b volume {first}");
    assert!(near(second, vb - OVERLAP), "b ∖ a volume {second}");
    assert!(
        (first - second).abs() > OVERLAP,
        "the two orders are genuinely different solids"
    );

    // The intersection is the overlap brick either way — the operand
    // order is data for subtraction and for nothing else here.
    let meet = insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Intersect,
            a,
            b,
        },
    );
    assert_volume(&mut session, meet, OVERLAP, tol);
}

/// Both operand seats want a body, and the two seats want DIFFERENT
/// nodes.
#[test]
fn the_boolean_door_refuses_a_non_body_seat_and_a_self_boolean() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let a = boxed(&mut session, A);
    let profile = insert(
        &mut session,
        SessionOp::AddProfile {
            plane: SketchPlane::xy(),
            loops: vec![ProfileShape::Circle {
                centre: [0.0, 0.0],
                radius: 0.01,
            }],
        },
    );
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0; 3],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    // A profile, a datum and an id the document never held are each
    // "not a body" at either seat.
    for wrong in [profile, plane, RecipeNodeId(999)] {
        for (x, y) in [(wrong, a), (a, wrong)] {
            let refused = session.perform(SessionOp::AddBoolean {
                op: BooleanOp::Union,
                a: x,
                b: y,
            });
            assert!(
                matches!(
                    refused.refusal,
                    Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body })
                        if node == wrong
                ),
                "{:?}",
                refused.refusal
            );
        }
    }
    // One body in both seats: the DAG would take it, the door does
    // not.
    let refused = session.perform(SessionOp::AddBoolean {
        op: BooleanOp::Subtract,
        a,
        b: a,
    });
    assert!(
        matches!(refused.refusal, Some(Refusal::SelfBoolean { node }) if node == a),
        "{:?}",
        refused.refusal
    );
    // And the kind gate speaks FIRST: two profiles in both seats is
    // reported as "that is not a body", the fact a user can act on.
    let refused = session.perform(SessionOp::AddBoolean {
        op: BooleanOp::Subtract,
        a: profile,
        b: profile,
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind {
                wanted: NodeKindWanted::Body,
                ..
            })
        ),
        "{:?}",
        refused.refusal
    );
}

/// A split's two seats want different kinds, and what it authors cuts.
#[test]
fn the_split_door_takes_a_body_and_a_datum_plane() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    let cut = 0.004;
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, cut],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 0.0, 1.0],
            },
        },
    );
    // The tool seat wants a PLANE — an axis datum and a body are both
    // refused there — and the target seat wants a body.
    for wrong in [axis, body] {
        let refused = session.perform(SessionOp::AddSplit {
            target: body,
            tool: wrong,
        });
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Plane })
                    if node == wrong
            ),
            "{:?}",
            refused.refusal
        );
    }
    let refused = session.perform(SessionOp::AddSplit {
        target: plane,
        tool: plane,
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body }) if node == plane
        ),
        "{:?}",
        refused.refusal
    );

    let split = insert(
        &mut session,
        SessionOp::AddSplit {
            target: body,
            tool: plane,
        },
    );
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Split { above, below } =
        &eval.value(split).expect("the split evaluated").payload
    else {
        panic!("a split evaluates to its two sides");
    };
    let volume_of = |side: &SplitSide<f64>| match side {
        SplitSide::Body(body) => {
            pncad::topo::mass_properties(body, tol)
                .expect("mass properties")
                .volume
        }
        SplitSide::Empty => 0.0,
    };
    let (up, down) = (volume_of(above), volume_of(below));
    let face = A[0] * A[1];
    assert!(near(up, face * (A[2] - cut)), "above the cut: {up}");
    assert!(near(down, face * cut), "below the cut: {down}");
}

/// A split's OUTPUT is two bodies, so it is not a body seat's pick —
/// the door says so rather than letting the operand mismatch surface
/// after the edit lands. The same is true of a pattern's instances.
#[test]
fn several_bodies_are_not_one_body_at_a_seat() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, 0.004],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    let split = insert(
        &mut session,
        SessionOp::AddSplit {
            target: body,
            tool: plane,
        },
    );
    let pattern = insert(
        &mut session,
        SessionOp::AddPattern {
            input: body,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        },
    );
    for wrong in [split, pattern] {
        for op in [
            SessionOp::AddBoolean {
                op: BooleanOp::Union,
                a: wrong,
                b: body,
            },
            SessionOp::AddTransform {
                input: wrong,
                translation: [0.0; 3],
                rotation_axis: [0.0, 0.0, 1.0],
                rotation_angle: 0.0,
            },
            SessionOp::AddSplit {
                target: wrong,
                tool: plane,
            },
        ] {
            let refused = session.perform(op);
            assert!(
                matches!(
                    refused.refusal,
                    Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Body })
                        if node == wrong
                ),
                "{:?}",
                refused.refusal
            );
        }
    }
}

/// The transform door places a body rigidly, with literal slots the
/// property panel then owns.
#[test]
fn the_transform_door_places_a_body_with_literal_slots() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    let placed = insert(
        &mut session,
        SessionOp::AddTransform {
            input: body,
            translation: OFFSET,
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: core::f64::consts::FRAC_PI_2,
        },
    );
    // A rigid placement preserves the volume, and the slots it landed
    // with are the numbers the form held.
    assert_volume(&mut session, placed, A[0] * A[1] * A[2], tol);
    let doc = session.committed_doc();
    for (axis, want) in [
        (Axis3::X, OFFSET[0]),
        (Axis3::Y, OFFSET[1]),
        (Axis3::Z, OFFSET[2]),
    ] {
        let expr = doc
            .node(placed)
            .and_then(|node| node.expr(SlotId::Translation(axis)))
            .expect("the translation slot is there");
        assert_eq!(expr.literal_value(), Some(want));
        assert_eq!(expr.dim(), Dimension::Length);
    }
    let angle = doc
        .node(placed)
        .and_then(|node| node.expr(SlotId::RotationAngle))
        .expect("the angle slot is there");
    assert_eq!(angle.dim(), Dimension::Angle);

    // A non-finite field never reaches the document: the literal door
    // refuses it, and nothing is recorded.
    let states = session.history().len();
    let refused = session.perform(SessionOp::AddTransform {
        input: body,
        translation: [f64::NAN, 0.0, 0.0],
        rotation_axis: [0.0, 0.0, 1.0],
        rotation_angle: 0.0,
    });
    assert!(
        matches!(refused.refusal, Some(Refusal::Dimension(_))),
        "{:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), states, "a refusal records nothing");
}

/// The pattern door spells its count STRUCTURALLY, and both parametric
/// rules author what they say.
#[test]
fn the_pattern_door_spells_its_count_structurally() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    let axis = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Axis {
                origin: [0.0; 3],
                direction: [0.0, 0.0, 1.0],
            },
        },
    );
    let linear = insert(
        &mut session,
        SessionOp::AddPattern {
            input: body,
            count: 3,
            rule: PatternRuleSpec::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        },
    );
    // The count is a Count-dimensioned slot — the structural one —
    // and the rule is the parametric variant the form offered.
    let count = session
        .committed_doc()
        .node(linear)
        .and_then(|node| node.expr(SlotId::Count))
        .expect("a pattern has a count slot");
    assert_eq!(count.dim(), Dimension::Count);
    assert!(SlotId::Count.is_structural());
    assert!(count.bit_eq(&Expr::count(3)));
    assert!(matches!(
        session.committed_doc().node(linear),
        Some(Node::Pattern {
            kind: PatternKind::Linear { .. },
            ..
        })
    ));
    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let ValuePayload::Instances(instances) =
        &eval.value(linear).expect("the pattern evaluated").payload
    else {
        panic!("a pattern evaluates to its instances");
    };
    assert_eq!(instances.len(), 3, "three instances, unfused");

    // The circular rule takes the axis from the tool's second seat,
    // and that seat wants an axis datum.
    let circular = insert(
        &mut session,
        SessionOp::AddPattern {
            input: body,
            count: 4,
            rule: PatternRuleSpec::Circular {
                axis,
                step: core::f64::consts::FRAC_PI_2,
            },
        },
    );
    assert!(matches!(
        session.committed_doc().node(circular),
        Some(Node::Pattern {
            kind: PatternKind::Circular { .. },
            ..
        })
    ));
    let refused = session.perform(SessionOp::AddPattern {
        input: body,
        count: 4,
        rule: PatternRuleSpec::Circular {
            axis: body,
            step: 1.0,
        },
    });
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::WrongNodeKind { node, wanted: NodeKindWanted::Axis }) if node == body
        ),
        "{:?}",
        refused.refusal
    );
}

/// Every combining door refuses mid-gesture and records nothing when
/// it refuses — the creation vocabulary's rule, held by the four new
/// arms too.
#[test]
fn a_refusal_at_any_combining_door_leaves_no_history_state() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0; 3],
                normal: [0.0, 0.0, 1.0],
            },
        },
    );
    let other = boxed(&mut session, B);
    let states = session.history().len();
    let doc = session.committed_doc().clone();

    // A gesture in flight closes every door here.
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: body,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none(),
        "the gesture opened"
    );
    for op in [
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: body,
            b: other,
        },
        SessionOp::AddSplit {
            target: body,
            tool: plane,
        },
        SessionOp::AddTransform {
            input: body,
            translation: [0.0; 3],
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: 0.0,
        },
        SessionOp::AddPattern {
            input: body,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        },
    ] {
        let refused = session.perform(op);
        assert!(
            matches!(refused.refusal, Some(Refusal::GestureInFlight)),
            "{:?}",
            refused.refusal
        );
    }
    assert!(session.perform(SessionOp::CancelGesture).refusal.is_none());
    assert_eq!(session.history().len(), states, "nothing was recorded");
    assert!(session.committed_doc().bit_eq(&doc), "and nothing changed");
}

/// Each tool holds its picks by ROLE, refuses typed until its seats
/// are filled, and drops a pick whose node vanished — naming the seat
/// it emptied.
#[test]
fn each_combining_tool_holds_its_picks_and_survives_a_vanished_one() {
    let tol = Tol::witness();
    let (mut session, a, b) = two_boxes(tol);

    let mut boolean = BooleanTool::new();
    assert!(matches!(
        boolean.op(BooleanOp::Union),
        Err(CombineToolError::SeatEmpty {
            seat: Seat::OperandA
        })
    ));
    boolean.pick(a);
    assert!(matches!(
        boolean.op(BooleanOp::Union),
        Err(CombineToolError::SeatEmpty {
            seat: Seat::OperandB
        })
    ));
    boolean.pick(b);
    assert_eq!((boolean.a(), boolean.b()), (Some(a), Some(b)));
    // A third pick replaces the SECOND seat: the first pick is the
    // one a subtraction keeps, and it stays where it was put.
    boolean.pick(a);
    assert_eq!((boolean.a(), boolean.b()), (Some(a), Some(a)));
    boolean.pick(b);
    assert!(matches!(
        boolean.op(BooleanOp::Subtract),
        Ok(SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a: first,
            b: second,
        }) if first == a && second == b
    ));

    // Deleting the SECOND operand's node empties that seat and leaves
    // the first where it is — no promotion between roles.
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: b })
            .refusal
            .is_none()
    );
    let events = boolean.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [CombineToolEvent::PickLost {
                seat: Seat::OperandB,
                node
            }] if *node == b
        ),
        "{events:?}"
    );
    assert_eq!((boolean.a(), boolean.b()), (Some(a), None));
    // The next pick refills the empty seat rather than displacing the
    // held one.
    boolean.pick(a);
    assert_eq!((boolean.a(), boolean.b()), (Some(a), Some(a)));

    let mut split = SplitTool::new();
    split.pick(a);
    assert!(matches!(
        split.op(),
        Err(CombineToolError::SeatEmpty {
            seat: Seat::SplitPlane
        })
    ));

    let mut transform = TransformTool::new();
    assert!(matches!(
        transform.op([0.0; 3], [0.0, 0.0, 1.0], 0.0),
        Err(CombineToolError::SeatEmpty {
            seat: Seat::TransformBody
        })
    ));
    transform.pick(b);
    let events = transform.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [CombineToolEvent::PickLost {
                seat: Seat::TransformBody,
                ..
            }]
        ),
        "{events:?}"
    );
    assert_eq!(transform.input(), None);

    // The pattern tool's axis seat is required by the CIRCULAR rule
    // and by nothing else.
    let mut pattern = PatternTool::new();
    pattern.pick(a);
    assert!(matches!(
        pattern.op(
            3,
            PatternForm::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        ),
        Ok(SessionOp::AddPattern { count: 3, .. })
    ));
    assert!(matches!(
        pattern.op(3, PatternForm::Circular { step: 1.0 }),
        Err(CombineToolError::SeatEmpty {
            seat: Seat::PatternAxis
        })
    ));
    pattern.clear();
    assert_eq!((pattern.input(), pattern.axis()), (None, None));
}

/// **One modal tool at a time**, and the rule is the tool set's rather
/// than each activation site's: opening any tool closes every other,
/// whichever was open.
#[test]
fn only_one_modal_tool_is_open_at_a_time() {
    let mut tools = Tools::new();
    assert_eq!(tools.open_kind(), None);
    for opened in ToolKind::ALL {
        for previous in ToolKind::ALL {
            tools.open(previous);
            tools.open(opened);
            assert_eq!(
                tools.open_kind(),
                Some(opened),
                "opening {opened:?} over {previous:?}"
            );
        }
    }
    tools.close();
    assert_eq!(tools.open_kind(), None);
}

/// The open tool consumes the ordinary selection stream at the
/// vocabulary it was written against, and a closed one consumes
/// nothing.
#[test]
fn the_open_tool_consumes_the_selection_stream() {
    let tol = Tol::witness();
    let (session, a, b) = two_boxes(tol);
    let picks = vec![
        SessionOp::Select(Selection::Node(a)),
        SessionOp::Select(Selection::Node(b)),
    ];

    let mut tools = Tools::new();
    // Nothing open: the stream reaches nothing.
    tools.feed(&picks);
    assert_eq!(tools.open_kind(), None);

    tools.open(ToolKind::Boolean);
    tools.feed(&picks);
    let boolean = tools.boolean().expect("the boolean tool is open");
    assert_eq!((boolean.a(), boolean.b()), (Some(a), Some(b)));

    // Opening another tool starts it EMPTY — the picks belonged to
    // the tool that held them.
    tools.open(ToolKind::Pattern);
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), None);
    tools.feed(&picks[..1]);
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), Some(a));

    // The survival step reaches whichever tool is open, from the
    // document alone.
    let mut session = session;
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: a })
            .refusal
            .is_none()
    );
    let notices = tools.reconcile(session.committed_doc(), session.landed_pair());
    assert_eq!(notices.len(), 1, "{notices:?}");
    let sentence = notices[0].to_string();
    assert!(
        sentence.starts_with("pattern tool: "),
        "a notice names its tool: {sentence}"
    );
    assert!(
        sentence.contains("patterned body"),
        "and the seat it emptied: {sentence}"
    );
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), None);
}

/// The seat line names WHICH pick is which — the fact that decides
/// what a subtraction removes — and says so before anything is picked.
#[test]
fn the_seat_line_names_the_roles() {
    assert_eq!(
        seat_line(&[(Seat::OperandA, None), (Seat::OperandB, None)]),
        "no picks yet"
    );
    assert_eq!(
        seat_line(&[
            (Seat::OperandA, Some(RecipeNodeId(3))),
            (Seat::OperandB, None),
        ]),
        "first operand: feature 3; second operand: —"
    );
    assert_eq!(
        seat_line(&[(Seat::TransformBody, Some(RecipeNodeId(7)))]),
        "transformed body: feature 7"
    );
}

/// **A tool closes on the COMMIT, not on the click** — the rule the
/// chrome applies to the op it just performed, as a pure function of
/// the op.
#[test]
fn the_tool_edits_are_the_ones_that_close_a_tool() {
    for op in [
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: RecipeNodeId(1),
            b: RecipeNodeId(2),
        },
        SessionOp::AddSplit {
            target: RecipeNodeId(1),
            tool: RecipeNodeId(2),
        },
        SessionOp::AddTransform {
            input: RecipeNodeId(1),
            translation: [0.0; 3],
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: 0.0,
        },
        SessionOp::AddPattern {
            input: RecipeNodeId(1),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        },
        SessionOp::AddRevolve {
            profile: RecipeNodeId(1),
            axis: RecipeNodeId(2),
            angle: 1.0,
        },
    ] {
        assert!(viewer::frame::commits_a_modal_tool(&op), "{op:?}");
    }
    // A creation op no modal tool authors leaves the tools alone.
    for op in [
        SessionOp::AddExtrude {
            profile: RecipeNodeId(1),
            distance: 0.01,
        },
        SessionOp::Undo,
    ] {
        assert!(!viewer::frame::commits_a_modal_tool(&op), "{op:?}");
    }
}

/// The vocabulary a datum-plane split needs is the one the datum form
/// already authors: a plane node built by [`SessionOp::AddDatum`] is
/// what the split's tool seat takes, with no second spelling.
#[test]
fn the_split_plane_is_the_datum_form_s_own_plane() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let plane = insert(
        &mut session,
        SessionOp::AddDatum {
            datum: DatumSpec::Plane {
                origin: [0.0, 0.0, 0.001],
                normal: [0.0, 1.0, 0.0],
            },
        },
    );
    assert!(matches!(
        session.committed_doc().node(plane),
        Some(Node::Datum(Datum::Plane { .. }))
    ));
}
