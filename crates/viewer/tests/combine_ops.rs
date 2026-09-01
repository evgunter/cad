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

use common::{body_volume, insert, near};
use pncad::document::SplitSide;
use pncad::document::{
    Axis3, BooleanOp, Datum, Dimension, Doc, Expr, LoopProgram, Node, NodeError, NodeErrorKind,
    NodeResult, PatternKind, ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::ValuePayload;
use pncad::profile::SketchPlane;
use viewer::combine::{BooleanTool, PatternTool, SplitTool, TransformTool, denotes_body};
use viewer::pick::PickKinds;
use viewer::seats::{Seat, SeatError, SeatEvent, seat_line};
use viewer::session::{
    DatumSpec, DocSession, NodeKindWanted, PatternRuleSpec, ProfileShape, Refusal, Selection,
    SessionOp,
};
use viewer::tools::{ToolKind, Tools};
use viewer::tree::{self, RowStatus};

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

/// A split node's two sides, by volume, with the seam pumped — an
/// empty side reading as zero.
fn split_volumes(session: &mut DocSession, split: RecipeNodeId, tol: Tol) -> (f64, f64) {
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
    (volume_of(above), volume_of(below))
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
    let (up, down) = split_volumes(&mut session, split, tol);
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

/// Every door that takes an existing BODY refuses mid-gesture and
/// records nothing when it refuses — the creation vocabulary's rule,
/// held by GAUTH-4's four arms and GAUTH-5's two.
///
/// The list below is hand-written, which is the reason it is worth
/// saying what it is a list OF: every `SessionOp` arm whose seat is a
/// node that must already exist. A door added to that family and not
/// added here is a door with no gesture row and no
/// nothing-was-recorded row, which is exactly how the blend doors
/// first shipped.
#[test]
fn a_refusal_at_any_body_seated_door_leaves_no_history_state() {
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
    // Real edge names for the two blend doors: their seat refusals are
    // about the TARGET, so the selection has to be the kind of thing a
    // user would actually have picked rather than an empty vector that
    // could refuse for its own reason.
    let edges = {
        session.pump();
        let eval = session.evaluation().expect("the inline seam landed");
        let edges = pncad::select::all_edges(eval, body);
        assert!(!edges.is_empty(), "the box has edges");
        edges
    };
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
        SessionOp::AddFillet {
            target: body,
            radius: 0.001,
            selection: edges.clone(),
        },
        SessionOp::AddChamfer {
            target: body,
            distance: 0.001,
            selection: edges.clone(),
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

    // The SEAT refusals record nothing either — a wrong-kind pick at
    // any of the four doors, and the boolean's two-operands-are-one
    // arm.
    for op in [
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: plane,
            b: body,
        },
        SessionOp::AddBoolean {
            op: BooleanOp::Subtract,
            a: body,
            b: body,
        },
        SessionOp::AddSplit {
            target: plane,
            tool: plane,
        },
        SessionOp::AddSplit {
            target: body,
            tool: body,
        },
        SessionOp::AddTransform {
            input: plane,
            translation: [0.0; 3],
            rotation_axis: [0.0, 0.0, 1.0],
            rotation_angle: 0.0,
        },
        SessionOp::AddPattern {
            input: plane,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: [1.0, 0.0, 0.0],
                spacing: 0.05,
            },
        },
        SessionOp::AddPattern {
            input: body,
            count: 2,
            rule: PatternRuleSpec::Circular {
                axis: body,
                step: 1.0,
            },
        },
        SessionOp::AddFillet {
            target: plane,
            radius: 0.001,
            selection: edges.clone(),
        },
        SessionOp::AddChamfer {
            target: plane,
            distance: 0.001,
            selection: edges.clone(),
        },
    ] {
        let refused = session.perform(op);
        assert!(
            matches!(
                refused.refusal,
                Some(Refusal::WrongNodeKind { .. } | Refusal::SelfBoolean { .. })
            ),
            "{:?}",
            refused.refusal
        );
        assert!(refused.committed.is_empty(), "and commits nothing");
    }
    assert_eq!(session.history().len(), states, "nothing was recorded");
    assert!(session.committed_doc().bit_eq(&doc), "and nothing changed");
}

/// **The authored path admits a non-positive count and the node
/// refuses it**, typed, on its own badge — the division of labour the
/// door states: the form declines to offer a count below one, and the
/// op vocabulary is not narrowed to what the form offers, because the
/// property panel can write the same number into the slot afterwards.
#[test]
fn a_non_positive_count_refuses_at_the_node_not_at_the_door() {
    let tol = Tol::witness();
    let mut session = session(tol);
    let body = boxed(&mut session, A);
    for count in [0, -3] {
        let pattern = insert(
            &mut session,
            SessionOp::AddPattern {
                input: body,
                count,
                rule: PatternRuleSpec::Linear {
                    direction: [1.0, 0.0, 0.0],
                    spacing: 0.05,
                },
            },
        );
        session.pump();
        let eval = session.evaluation().expect("the inline seam landed");
        assert!(
            eval.value(pattern).is_none(),
            "a pattern of {count} instances does not evaluate to a value"
        );
        let badge = tree::rows(session.committed_doc(), Some(eval))
            .into_iter()
            .find(|row| row.id == pattern)
            .map(|row| row.status);
        let Some(RowStatus::Failed { message }) = badge else {
            panic!("the tree badge carries the node's own refusal: {badge:?}");
        };
        assert!(
            message.contains("count"),
            "and the refusal names the count: {message}"
        );
    }
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
        Err(SeatError::Empty {
            seat: Seat::OperandA
        })
    ));
    boolean.pick(a);
    assert!(matches!(
        boolean.op(BooleanOp::Union),
        Err(SeatError::Empty {
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
            [SeatEvent::PickLost {
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
        Err(SeatError::Empty {
            seat: Seat::SplitPlane
        })
    ));

    let mut transform = TransformTool::new();
    assert!(matches!(
        transform.op([0.0; 3], [0.0, 0.0, 1.0], 0.0),
        Err(SeatError::Empty {
            seat: Seat::TransformBody
        })
    ));
    transform.pick(b);
    let events = transform.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [SeatEvent::PickLost {
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
        pattern.linear_op(3, [1.0, 0.0, 0.0], 0.05),
        Ok(SessionOp::AddPattern { count: 3, .. })
    ));
    assert!(matches!(
        pattern.circular_op(3, 1.0),
        Err(SeatError::Empty {
            seat: Seat::PatternAxis
        })
    ));
    pattern.clear();
    assert_eq!((pattern.input(), pattern.axis()), (None, None));
}

/// Which tools are actually holding state, read through the per-tool
/// accessors rather than through `open_kind`.
///
/// `open_kind` is a PRIORITY SCAN: it answers with the first tool it
/// finds open, so it cannot see a second one left behind it, and in
/// half of the ordered pairs that is exactly where a leftover would
/// be. The exclusivity row asserts on this instead.
///
/// The array is `ToolKind::ALL`-wide and indexed by `ordinal`, so a
/// tool added to the set widens it here and the exclusivity row keeps
/// covering every pair without a count written out twice.
fn open_flags(tools: &Tools) -> [bool; ToolKind::ALL.len()] {
    [
        tools.mate().is_some(),
        tools.revolve().is_some(),
        tools.boolean().is_some(),
        tools.split().is_some(),
        tools.transform().is_some(),
        tools.pattern().is_some(),
        tools.blend().is_some(),
    ]
}

/// **One modal tool at a time**, and the rule is the tool set's rather
/// than each activation site's: opening any tool closes every other,
/// whichever was open.
#[test]
fn only_one_modal_tool_is_open_at_a_time() {
    let mut tools = Tools::new();
    assert_eq!(
        open_flags(&tools),
        [false; ToolKind::ALL.len()],
        "nothing is open to start"
    );
    for opened in ToolKind::ALL {
        for previous in ToolKind::ALL {
            tools.open(previous);
            tools.open(opened);
            assert_eq!(
                tools.open_kind(),
                Some(opened),
                "opening {opened:?} over {previous:?}"
            );
            let mut want = [false; ToolKind::ALL.len()];
            want[opened.ordinal()] = true;
            assert_eq!(
                open_flags(&tools),
                want,
                "opening {opened:?} over {previous:?} leaves ONLY {opened:?} holding state"
            );
        }
    }
    tools.close();
    assert_eq!(
        open_flags(&tools),
        [false; ToolKind::ALL.len()],
        "close empties every seat"
    );
}

/// `ToolKind::ALL` is a hand-written list, and `ordinal` is the
/// exhaustive match beside it: this row reads the two against each
/// other, so a variant added to the enum and forgotten in the list
/// fails here rather than quietly narrowing every sweep that iterates
/// it (this suite's exclusivity row included).
#[test]
fn every_tool_kind_is_listed_in_all() {
    let mut seen = vec![false; ToolKind::ALL.len()];
    for kind in ToolKind::ALL {
        let at = kind.ordinal();
        assert!(at < seen.len(), "{kind:?} ordinal {at} is off the end");
        assert!(!seen[at], "{kind:?} shares an ordinal with another kind");
        seen[at] = true;
    }
    assert!(seen.into_iter().all(|hit| hit), "every ordinal is listed");
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
    assert!(tools.feed(&picks).is_empty(), "every pick landed");
    assert_eq!(tools.open_kind(), None);

    tools.open(ToolKind::Boolean);
    assert!(tools.feed(&picks).is_empty(), "every pick landed");
    let boolean = tools.boolean().expect("the boolean tool is open");
    assert_eq!((boolean.a(), boolean.b()), (Some(a), Some(b)));

    // Opening another tool starts it EMPTY — the picks belonged to
    // the tool that held them.
    tools.open(ToolKind::Pattern);
    assert_eq!(tools.pattern().and_then(|tool| tool.input()), None);
    assert!(tools.feed(&picks[..1]).is_empty(), "the pick landed");
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
    // `session.doc()` — the SHOWN document, which is what the
    // application hands this call every frame. It is the committed one
    // except under a gesture, and a tool holding picks under a gesture
    // must still see a delete the gesture's scratch does not contain.
    let notices = tools.reconcile(session.doc(), session.landed_pair());
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

/// **A tool closes on the COMMIT, not on the click**, and the rule is
/// asked of the OPEN TOOL rather than of the op alone: an op a
/// different tool authors — or one no tool does — closes nothing.
#[test]
fn a_tool_closes_on_its_own_committed_edit() {
    let edits = [
        (
            ToolKind::Boolean,
            SessionOp::AddBoolean {
                op: BooleanOp::Union,
                a: RecipeNodeId(1),
                b: RecipeNodeId(2),
            },
        ),
        (
            ToolKind::Split,
            SessionOp::AddSplit {
                target: RecipeNodeId(1),
                tool: RecipeNodeId(2),
            },
        ),
        (
            ToolKind::Transform,
            SessionOp::AddTransform {
                input: RecipeNodeId(1),
                translation: [0.0; 3],
                rotation_axis: [0.0, 0.0, 1.0],
                rotation_angle: 0.0,
            },
        ),
        (
            ToolKind::Pattern,
            SessionOp::AddPattern {
                input: RecipeNodeId(1),
                count: 2,
                rule: PatternRuleSpec::Linear {
                    direction: [1.0, 0.0, 0.0],
                    spacing: 0.05,
                },
            },
        ),
        (
            ToolKind::Revolve,
            SessionOp::AddRevolve {
                profile: RecipeNodeId(1),
                axis: RecipeNodeId(2),
                angle: 1.0,
            },
        ),
    ];
    let mut tools = Tools::new();
    for (author, op) in &edits {
        for open in ToolKind::ALL {
            tools.open(open);
            assert_eq!(
                tools.commits_open_tool(op),
                open == *author,
                "{op:?} closes {author:?} and nothing else"
            );
        }
    }
    // A creation op no modal tool authors closes nothing, whatever is
    // open — the mate tool included, which closes at its own click.
    for op in [
        SessionOp::AddExtrude {
            profile: RecipeNodeId(1),
            distance: 0.01,
        },
        SessionOp::Undo,
    ] {
        for open in ToolKind::ALL {
            tools.open(open);
            assert!(!tools.commits_open_tool(&op), "{op:?} with {open:?} open");
        }
    }
    tools.close();
    for (_, op) in &edits {
        assert!(
            !tools.commits_open_tool(op),
            "with nothing open there is nothing to close"
        );
    }
}

/// **The body seat tracks the evaluator's operand door**, and this row
/// is what holds that rather than the prose at
/// [`viewer::combine::denotes_body`].
///
/// The check drives the REAL door: each candidate node is fed to a
/// `Node::Transform`, whose single-body operand IS `body_operand`, and
/// the transform's own failure is the verdict — `WrongOperand` means
/// the door refused the candidate as an operand, and anything else
/// (including a failure of the transform's own, such as the NURBS
/// placement frontier a lofted body meets) means it did not. Two
/// directions matter and both are asserted: a kind the seat admits
/// that the door refuses is a lie the user meets after the edit lands,
/// and a kind the seat refuses that the door would take is silent
/// capability loss.
///
/// **The named exception**: `Sweep` is admitted by the seat and
/// evaluates to nothing at all — it is the curved-solid frontier, so
/// its own node fails and the transform is POISONED rather than
/// refused. That is asserted here in the same shape, so the day the
/// frontier moves this row notices.
///
/// **What it does not reach**, stated rather than implied: four of the
/// eighteen node kinds are absent. `Mate`, `Measure` and `Assertion`
/// need substrate this row does not build (a solved assembly, a
/// measured expression) and are all answered `false` by the seat;
/// `InstantiatePart` is answered `true` and needs a resolver with a
/// sibling document on disk, so its evaluates-to-a-body path is
/// exercised by the assembly suites instead. The fourteen that ARE
/// here include every kind whose classification is load-bearing for
/// this unit.
#[test]
fn the_body_seat_tracks_the_evaluators_operand_door() {
    let tol = Tol::witness();
    let mut doc = Doc::empty_derived("operand-door", tol);
    // The substrate every candidate is built out of.
    let (next, profile) = common::inserted(&doc, common::square(0.02), tol);
    doc = next;
    let (next, profile_b) = common::inserted(
        &doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::from_frame(
                pncad::geom_core::Point3::new(0.0, 0.0, 0.01),
                pncad::geom_core::Vec3::unit_x(),
                pncad::geom_core::Vec3::unit_y(),
            ),
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (0.02, 0.0), (0.02, 0.02), (0.0, 0.02)])
                    .expect("finite corners"),
            ],
        }),
        tol,
    );
    doc = next;
    let (next, ring) = common::inserted(
        &doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::circle(0.05, 0.0, 0.01).expect("finite circle")],
        }),
        tol,
    );
    doc = next;
    let (next, axis) = common::inserted(
        &doc,
        Node::Datum(Datum::Axis {
            origin: [common::len(0.0), common::len(0.0), common::len(0.0)],
            direction: [common::scl(0.0), common::scl(1.0), common::scl(0.0)],
        }),
        tol,
    );
    doc = next;
    let (next, plane) = common::inserted(
        &doc,
        Node::Datum(Datum::Plane {
            origin: [common::len(0.0), common::len(0.0), common::len(0.005)],
            normal: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
        }),
        tol,
    );
    doc = next;
    let (next, body) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    );
    doc = next;
    let (next, other) = common::inserted(
        &doc,
        Node::Transform {
            input: body,
            translation: [common::len(0.01), common::len(0.002), common::len(0.002)],
            rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
        },
        tol,
    );
    doc = next;

    // A real edge of the box, for the two blends: an empty blend
    // selection is an AUTHORING refusal, so a minimal fillet has to
    // name an edge, and the shipped `all_edges` door is where a user
    // would get one too.
    // ALL of them: a blend of one box edge terminates at a trivalent
    // corner whose other two edges were not requested, which the
    // kernel refuses by name — the whole-body blend is the shape a
    // minimal instance has to take.
    let edges = {
        let mut probe = DocSession::inline(doc.clone(), tol);
        probe.pump();
        let eval = probe.evaluation().expect("the inline seam landed");
        let edges = pncad::select::all_edges(eval, body);
        assert!(!edges.is_empty(), "the box has edges");
        edges
    };

    // One candidate per node kind this row can build, with the seat's
    // answer beside it. The seat's answer is READ, never restated: a
    // kind that moves sides moves in one place.
    let candidates: Vec<(&str, Node<ProfileProgram>)> = vec![
        (
            "datum",
            Node::Datum(Datum::Point {
                position: [common::len(0.0), common::len(0.0), common::len(0.0)],
            }),
        ),
        ("profile", common::square(0.01)),
        (
            "extrude",
            Node::Extrude {
                profile,
                distance: common::len(0.004),
            },
        ),
        (
            "revolve",
            Node::Revolve {
                profile: ring,
                axis,
                angle: Expr::literal(core::f64::consts::TAU, Dimension::Angle).expect("finite"),
            },
        ),
        (
            "boolean",
            Node::Boolean {
                op: BooleanOp::Union,
                a: body,
                b: other,
                declare: None,
            },
        ),
        (
            "transform",
            Node::Transform {
                input: body,
                translation: [common::len(0.0), common::len(0.0), common::len(0.0)],
                rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
            },
        ),
        (
            "split",
            Node::Split {
                target: body,
                tool: plane,
            },
        ),
        (
            "pattern",
            Node::Pattern {
                input: body,
                count: Expr::count(2),
                kind: PatternKind::Linear {
                    direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                    spacing: common::len(0.05),
                },
            },
        ),
        (
            "loft",
            Node::Loft {
                profiles: vec![profile, profile_b],
                v_degree: Expr::count(1),
            },
        ),
        (
            "fillet",
            Node::fillet(body, common::len(0.001), edges.clone()),
        ),
        (
            "chamfer",
            Node::chamfer(body, common::len(0.001), edges.clone()),
        ),
        (
            "placed union",
            Node::PlacedUnion {
                input: body,
                count: Some(Expr::count(2)),
                kind: PatternKind::Linear {
                    direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                    spacing: common::len(0.05),
                },
            },
        ),
        ("declare", Node::Declare { pairs: Vec::new() }),
        (
            "sweep",
            Node::Sweep {
                profile,
                path: profile_b,
                stations: Expr::count(8),
                v_degree: Expr::count(3),
            },
        ),
    ];

    for (name, node) in candidates {
        let admitted = denotes_body(&node);
        let (with_candidate, candidate) = common::inserted(&doc, node, tol);
        let (with_probe, probe) = common::inserted(
            &with_candidate,
            Node::Transform {
                input: candidate,
                translation: [common::len(0.0), common::len(0.0), common::len(0.0)],
                rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
            },
            tol,
        );
        let mut session = DocSession::inline(with_probe, tol);
        session.pump();
        let eval = session.evaluation().expect("the inline seam landed");
        let refused_as_operand = matches!(
            eval.result(probe).and_then(NodeResult::error),
            Some(NodeError {
                kind: NodeErrorKind::WrongOperand { .. },
                ..
            })
        );
        if name == "sweep" {
            // The one exception, asserted rather than assumed: the
            // seat admits it, and the door never gets to answer
            // because the sweep itself is the frontier.
            assert!(admitted, "the seat admits a sweep");
            assert!(
                eval.result(candidate).and_then(NodeResult::error).is_some(),
                "the sweep node fails on its own"
            );
            assert!(
                !refused_as_operand,
                "and the transform is poisoned, not told the sweep is not a body"
            );
            continue;
        }
        if admitted {
            assert!(
                eval.value(candidate).is_some(),
                "a {name} evaluates to a value: {:?}",
                eval.result(candidate).and_then(NodeResult::error)
            );
            // The probe may still fail for a reason of its OWN — a
            // rigid transform of a lofted NURBS body is its own
            // frontier — and that is not this row's business. What is
            // asserted is that the operand door did not answer "that
            // is not a body" about a kind the seat admits.
            assert!(
                !refused_as_operand,
                "the seat admits a {name}, so the operand door must not refuse one"
            );
        } else {
            assert!(
                refused_as_operand,
                "the seat refuses a {name}; the operand door must refuse it too, \
                 or the seat is losing a capability the kernel has: {:?}",
                eval.result(probe).and_then(NodeResult::error)
            );
        }
    }
}

/// **An open tool narrows what the cursor may pick to what that tool
/// can actually use**, and a tool that can use either kind narrows
/// nothing.
///
/// Two tools narrow, in opposite directions: the mate tool's alignment
/// frames come off FACE geometry, and the blend tool blends EDGES. The
/// seated tools hold node picks, which a face and an edge answer
/// equally well, so they leave the bare cursor's rule alone.
///
/// The expectation is a match rather than a comparison against one
/// named kind: a seventh tool has to state which side of this it is on
/// before the row compiles.
#[test]
fn each_tool_narrows_the_cursor_to_what_it_can_use() {
    let mut tools = Tools::new();
    assert_eq!(tools.pick_kinds(), PickKinds::Any, "the bare cursor's rule");
    for kind in ToolKind::ALL {
        tools.open(kind);
        let want = match kind {
            ToolKind::Mate => PickKinds::FacesOnly,
            ToolKind::Blend => PickKinds::EdgesOnly,
            ToolKind::Revolve
            | ToolKind::Boolean
            | ToolKind::Split
            | ToolKind::Transform
            | ToolKind::Pattern => PickKinds::Any,
        };
        assert_eq!(tools.pick_kinds(), want, "{kind:?}");
    }
    tools.close();
    assert_eq!(tools.pick_kinds(), PickKinds::Any);
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
