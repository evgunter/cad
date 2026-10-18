//! **A placement whose mate reads a failing placer's copy points at
//! the placer, and the placer states the cause.**
//!
//! A mate whose target is read off a pattern copy reads that node's
//! value: the placer is upstream of the placement, so when the placer
//! refuses — a slot that does not evaluate, a direction of no definite
//! length, a degenerate step — the placement is poisoned through it,
//! and the placer's own row states its refusal in its own words,
//! exactly as its evaluation raises it on the TWIN document (the same
//! recipe with the mate left out). The nearest failed node is named.
//!
//! A target that stops resolving after the mate is in — a copy past a
//! shrunk count, a placer deleted — is the placement's own refusal
//! where it evaluates: the pose does not evaluate
//! (`MateFault::SideUnevaluated`), or its read is unresolved.
//!
//! Every row goes through ordinary doors: `DocEdit::InsertNode`,
//! `evaluate`, `apply`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    CapEnd, ContactClass, DocEdit, DocumentId, EditError, EvalOptions, Formula,
    Frame, MateFault, MateRef, MateSide, Node, NodeErrorClass, NodeErrorKind, NodeResult,
    OperandSlot, PatternKind, ProfileDoc, RecipeNodeId, SlotId, StableName,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{ang, in_copy, insert, insert_pinned, len, on_frame, run, scl, solve, step};
use geom_core::Tol;

// ---- the scene ----

/// A `1 x 1 x 1` block, as a whole part document, and its body.
fn block(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// What every row builds: a part instance pinned to the world, whatever
/// places a copy of it, a second instance, and a mate in the second
/// instance's bundle resting its start cap on the placed copy's face.
///
/// `twin` is the same document with the mate left OUT, and its node
/// ids are the mated document's because the mate is added last — so
/// the node the twin evaluates is the node the mate reads.
struct Scene {
    doc: ProfileDoc,
    twin: ProfileDoc,
    placer: RecipeNodeId,
    mate: MateRef,
    store: Arc<PartStore>,
}

impl Scene {
    /// The evaluation options every row runs through.
    fn opts(&self) -> EvalOptions {
        options(&self.store)
    }

    /// **The failed node the mated placement is poisoned through.**
    fn poisoned_through(&self) -> RecipeNodeId {
        match run(&self.doc, &self.opts()).result(self.mate.placement) {
            Some(NodeResult::Poisoned { through }) => *through,
            other => panic!("the placement is poisoned through what it reads: {other:?}"),
        }
    }

    /// **What a node's OWN evaluation raises**, on the twin, rendered
    /// through `Debug`, which is the structural reading of a
    /// [`NodeErrorKind`]: the kernel refusals it carries unaltered have
    /// no equality of their own; and its class.
    fn own_refusal_of(&self, node: RecipeNodeId) -> (String, NodeErrorClass) {
        let ev = run(&self.twin, &self.opts());
        match ev.result(node) {
            Some(NodeResult::Failed(err)) => (format!("{:?}", err.kind), err.kind.class()),
            other => panic!("node {node:?} must fail on its own on the twin: {other:?}"),
        }
    }

    /// What `node`'s row in the MATED document fails with, when it
    /// fails in its own right.
    fn mated_refusal_of(&self, node: RecipeNodeId) -> Option<String> {
        match run(&self.doc, &self.opts()).result(node) {
            Some(NodeResult::Failed(err)) => Some(format!("{:?}", err.kind)),
            _ => None,
        }
    }

    /// **The placement points at `failed` and `failed` states the
    /// cause**: the placement is poisoned through it, and its own row
    /// in the mated document is the twin's word for word. Answers the
    /// refusal and its class.
    fn points_at(&self, failed: RecipeNodeId, label: &str) -> (String, NodeErrorClass) {
        assert_eq!(
            self.poisoned_through(),
            failed,
            "{label}: the placement names the node that refused"
        );
        let (kind, class) = self.own_refusal_of(failed);
        assert_eq!(
            self.mated_refusal_of(failed).as_deref(),
            Some(kind.as_str()),
            "{label}: the node's own row in the mated document states the twin's refusal"
        );
        (kind, class)
    }
}

/// The options that resolve `store`'s parts.
fn options(store: &Arc<PartStore>) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::clone(store) as Arc<dyn editor_core::PartResolver>),
        ..EvalOptions::default()
    }
}

/// A scene whose placer is a PATTERN of `kind` at `count`, mated onto
/// copy `i` — the name carries the `Instance(i)` qualifier.
fn patterned(label: &str, kind: PatternKind<Formula>, count: i64, i: u32) -> Scene {
    build(label, |doc, legs, leg_body| {
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs.into(),
                count: Formula::count(count),
                kind,
            },
        );
        let name = in_copy(pattern, i, in_part(legs, leg_body, CapEnd::End));
        (doc, pattern, name, Vec::new())
    })
    .0
}

/// The scene builder both shapes share: two part documents, the
/// placed instance pinned to the world, the placer with the name the
/// mate reads it by, the capping instance, and the mate. `place` is
/// handed the placed instance and its part's body.
fn build<F>(label: &str, place: F) -> (Scene, Vec<RecipeNodeId>)
where
    F: FnOnce(
        ProfileDoc,
        RecipeNodeId,
        RecipeNodeId,
    ) -> (ProfileDoc, RecipeNodeId, StableName, Vec<RecipeNodeId>),
{
    let mut store = PartStore::new();
    let (leg, leg_body) = store.insert_part(block(&format!("{label}-leg")), Tol::witness());
    let (top, top_body) = store.insert_part(block(&format!("{label}-top")), Tol::witness());
    let store = Arc::new(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, legs) = insert_pinned(doc, Node::instantiate_part(leg), &options(&store));
    let (doc, placer, name, extra) = place(doc, legs, leg_body);
    let (doc, cap) = insert(doc, fixture::mated_instance(top));
    let twin = doc.clone();
    // The target is read AT the placer: that read is what puts the
    // placer upstream of the placement.
    let mate = fixture::rest(
        fixture::head(in_part(cap, top_body, CapEnd::Start)),
        fixture::head_at(placer, name),
        ContactClass::Rest,
    );
    let (doc, mate) = fixture::constrain_at(doc, cap, mate);
    (
        Scene {
            doc,
            twin,
            placer,
            mate,
            store,
        },
        extra,
    )
}

// ---- A1: the cause, in the placer's own words ----

/// **The finding's own document.** A pattern direction of `1e200` has
/// a LENGTH that overflows to infinity. The mate reads copy 1; the
/// placement is poisoned through the pattern, and the pattern's own
/// row states `NonFiniteDirection { role: "pattern direction" }` — the
/// SAME kind its own evaluation raises on the unpoisoned twin: the
/// mate is a constraint of the capping instance, downstream of the
/// pattern, so it changes nothing the pattern reads.
#[test]
fn a1_a_non_finite_pattern_direction_names_the_direction() {
    let scene = patterned(
        "msolve3-nonfinite",
        PatternKind::Linear {
            direction: [scl(1e200), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
        4,
        1,
    );
    let (kind, class) = scene.points_at(scene.placer, "nonfinite");
    assert!(
        class == NodeErrorClass::NonFiniteDirection && kind.contains("pattern direction"),
        "and it is the direction door's own: {kind}"
    );
}

/// A DECIDED-ZERO direction is the other half of the same door.
#[test]
fn a1_a_degenerate_pattern_direction_names_the_direction() {
    let scene = patterned(
        "msolve3-degenerate",
        PatternKind::Linear {
            direction: [scl(0.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
        4,
        1,
    );
    let (kind, class) = scene.points_at(scene.placer, "degenerate");
    assert!(
        class == NodeErrorClass::DegenerateDirection && kind.contains("pattern direction"),
        "{kind}"
    );
}

/// **A slot that does not evaluate refuses `Expr { slot, source }`,
/// and the SLOT is named** — so a reader is pointed at the spacing
/// rather than at the pattern. The spacing here overflows to
/// infinity, which is the expression evaluator's own ruled refusal.
#[test]
fn a1_a_slot_that_does_not_evaluate_names_the_slot() {
    let scene = patterned(
        "msolve3-slot",
        PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: Formula::mul(len(1e200), scl(1e200)).expect("length times scalar is a length"),
        },
        4,
        1,
    );
    let (kind, class) = scene.points_at(scene.placer, "slot");
    assert!(
        class == NodeErrorClass::Expr && kind.contains(&format!("{:?}", SlotId::Spacing)),
        "the refusal names the slot it read: {kind}"
    );
}

/// **A circular rule whose `axis` reads no axis is refused at the
/// door.** The axis slot reads an `Axis` variable, so a plane, a body
/// or a pattern is refused by kind before any evaluation or solve runs: no mate can reach a placer so
/// wired, and the evaluation's operand refusal is a defence behind the
/// door.
#[test]
fn a1_a_circular_rule_over_a_non_axis_is_refused_at_the_door() {
    use editor_core::{EditError, OperandSlot, SlotKind, VarKind};
    let (doc, body) = block("msolve3-circular");
    let plane = fixture::plane(editor_core::Operand::select(
        body,
        vec![fixture::fname(body, editor_core::RoleSeg::Cap(CapEnd::Start))],
    ));
    let (doc, array) = insert(
        doc,
        Node::Pattern {
            input: body.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    for (axis, found) in [
        (plane, VarKind::Plane),
        (editor_core::Operand::Node(body), VarKind::Body),
        (editor_core::Operand::Node(array), VarKind::Bodies),
    ] {
        let refusal = fixture::insert_refused(
            &doc,
            Node::Pattern {
                input: body.into(),
                count: Formula::count(4),
                kind: PatternKind::Circular {
                    axis: axis.into(),
                    step: ang(0.5),
                },
            },
        );
        assert!(
            matches!(
                &refusal,
                EditError::SlotVarKind {
                    slot: editor_core::SlotId::Operand(OperandSlot::Axis),
                    found: f,
                    expected: SlotKind::Is(VarKind::Axis),
                    ..
                } if *f == found
            ),
            "{found:?}: {refusal:?}"
        );
    }
}

/// **A spacing or a step the pattern refuses is the pattern's refusal.**
/// A negative or zero spacing, and a zero step or one past a full turn:
/// the placement reading a copy is poisoned through the pattern, whose
/// own row carries the kind its evaluation raises on the twin.
#[test]
fn a1_a_spacing_or_step_the_pattern_refuses_is_its_own_refusal() {
    let linear = |spacing: f64| PatternKind::Linear {
        direction: [scl(1.0), scl(0.0), scl(0.0)],
        spacing: len(spacing),
    };
    for (label, kind, class) in [
        (
            "msolve3-negative-spacing",
            Rule::Plain(linear(-2.0)),
            NodeErrorClass::NegativeSpacing,
        ),
        (
            "msolve3-zero-spacing",
            Rule::Plain(linear(0.0)),
            NodeErrorClass::DegenerateSpacing,
        ),
        (
            "msolve3-zero-step",
            Rule::Turning(0.0),
            NodeErrorClass::DegenerateStep,
        ),
        (
            "msolve3-step-past-a-turn",
            Rule::Turning(7.0),
            NodeErrorClass::FullRangeStep,
        ),
    ] {
        let scene = kind.scene(label, 1);
        let (refused, got) = scene.points_at(scene.placer, label);
        assert_eq!(got, class, "{label}: {refused}");
    }
}

/// A stepped rule [`a1_a_spacing_or_step_the_pattern_refuses_is_its_own_refusal`]
/// mates through: a linear rule as written, or a circular one at a
/// step about the axis of the placed copy's top cap.
#[derive(Clone)]
enum Rule {
    Plain(PatternKind<Formula>),
    Turning(f64),
}

impl Rule {
    /// The rule's scene at four copies, mated onto copy `i`.
    fn scene(&self, label: &str, i: u32) -> Scene {
        match self {
            Self::Plain(kind) => patterned(label, kind.clone(), 4, i),
            Self::Turning(step) => {
                build(label, |doc, legs, leg_body| {
                    let axis = fixture::axis(editor_core::SitedRef::new(
                        legs,
                        in_part(legs, leg_body, CapEnd::End),
                    ));
                    let (doc, pattern) = insert(
                        doc,
                        Node::Pattern {
                            input: legs.into(),
                            count: Formula::count(4),
                            kind: PatternKind::Circular {
                                axis: axis.into(),
                                step: ang(*step),
                            },
                        },
                    );
                    let name = in_copy(pattern, i, in_part(legs, leg_body, CapEnd::End));
                    (doc, pattern, name, Vec::new())
                })
                .0
            }
        }
    }
}

/// **The explicit rule, measured where it can be reached.** A
/// `Node::Pattern` carries its count as a slot, so an explicit
/// placement list is a second spelling of the same number and the
/// EDIT door refuses it — no document a mate reads through can carry
/// one. This row pins the door that keeps it unreachable.
#[test]
fn an_explicit_pattern_rule_never_reaches_a_mate() {
    let mut store = PartStore::new();
    let leg = store.insert(block("msolve3-explicit-leg").0, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("msolve3-explicit"), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
    let refused = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Pattern {
                input: legs.into(),
                count: Formula::count(2),
                kind: PatternKind::Explicit(vec![
                    Frame::IDENTITY,
                    Frame::translation([2.0, 0.0, 0.0]),
                ]),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("a pattern spells its count once");
    assert!(
        matches!(refused, EditError::PlacementRuleMismatch { .. }),
        "{refused:?}"
    );
    let _ = store;
}

// ---- a target that stops resolving after the mate is in ----

/// A copy index at the pattern's count names a copy that does not
/// exist. The mate reads copy 2 of a count-3 pattern that then shrinks
/// to two copies: the pattern still evaluates, and the placement's own
/// evaluation refuses — its `to` pose does not evaluate
/// (`SideUnevaluated`), carrying the read's own refusal.
#[test]
fn an_index_at_the_count_is_still_a_dangling_head() {
    let mut scene = patterned(
        "msolve3-past-count",
        PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
        3,
        2,
    );
    let (doc, _) = step(
        scene.doc,
        DocEdit::SetStructuralParam {
            node: scene.placer,
            slot: editor_core::SlotId::Count,
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
    );
    scene.doc = doc;
    assert!(
        scene.mated_refusal_of(scene.placer).is_none(),
        "the shrunk pattern evaluates"
    );
    let f = solve(&scene.doc, &scene.opts(), Tol::witness())
        .fault(scene.mate.placement)
        .cloned()
        .expect("the placement refuses the pose it cannot read");
    assert!(
        matches!(
            &f,
            MateFault::SideUnevaluated { mate, side: MateSide::To, .. } if *mate == scene.mate
        ),
        "{f:?}"
    );
}

/// A deleted placer leaves the target's read unresolved — the
/// placement's own `UnresolvedRead`, at the mate's `to` seat.
#[test]
fn a_stranded_operand_is_still_a_dangling_head() {
    let scene = patterned(
        "msolve3-stranded",
        PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
        4,
        1,
    );
    let (doc, _) = step(scene.doc.clone(), DocEdit::DeleteNode { id: scene.placer });
    match run(&doc, &scene.opts()).result(scene.mate.placement) {
        Some(NodeResult::Failed(err)) => assert!(
            matches!(
                err.kind,
                NodeErrorKind::UnresolvedRead {
                    slot: OperandSlot::Side {
                        constraint,
                        side: MateSide::To,
                    },
                    ..
                } if constraint == scene.mate.id
            ),
            "{:?}",
            err.kind
        ),
        other => panic!("the stranded placement refuses its read: {other:?}"),
    }
}

// ---- the placement axis ----

/// **A degenerate placement axis refuses naming the AXIS.** The
/// constructor decides the axis through the direction door, so an
/// author who builds a frame and pins a copy at it reads the vector
/// they authored — not the frame the constructor would have built.
#[test]
fn the_placement_axis_refuses_in_its_own_voice() {
    let mut store = PartStore::new();
    let part = store.insert(block("msolve3-placement-axis-part").0, Tol::witness());
    let opts = options(&Arc::new(store));
    let doc = ProfileDoc::empty(DocumentId::derive("msolve3-placement-axis"), Tol::witness());
    let (doc, instance) = insert(doc, fixture::mated_instance(part));

    // The natural spelling: build the frame, pin the instance to the
    // world at it, one error type.
    let set = |axis: [f64; 3]| -> Result<ProfileDoc, EditError> {
        let frame = Frame::rotate_then_translate(axis, 0.5, [1.0, 2.0, 3.0], fixture::band())?;
        Ok(fixture::pin(doc.clone(), instance, &frame, &opts))
    };

    let refused = set([0.0, 0.0, 0.0]).expect_err("a zero axis has no direction");
    let EditError::PlacementAxis { error } = &refused else {
        panic!("expected PlacementAxis, got {refused:?}");
    };
    assert!(
        matches!(
            error.kind(),
            NodeErrorKind::DegenerateDirection { role } if *role == "placement rotation axis"
        ),
        "the refusal names the axis and its role: {error:?}"
    );
    assert!(
        refused.to_string().contains("placement rotation axis"),
        "and so does its prose: {refused}"
    );

    let overflowed = set([1e200, 0.0, 0.0]).expect_err("a non-finite length has no direction");
    assert!(
        matches!(
            &overflowed,
            EditError::PlacementAxis { error }
                if matches!(error.kind(), NodeErrorKind::NonFiniteDirection { .. })
        ),
        "{overflowed:?}"
    );
    let poisoned = set([f64::NAN, 0.0, 0.0]).expect_err("a NaN axis has no direction");
    assert!(
        matches!(
            &poisoned,
            EditError::PlacementAxis { error }
                if matches!(error.kind(), NodeErrorKind::NonFiniteDirection { .. })
        ),
        "a NaN component is a non-finite LENGTH, not a zero one: {poisoned:?}"
    );

    // The BAND is the door's judgement, not a literal's: a length
    // under its zero threshold is decided zero and one over its
    // escalation threshold is a direction. Both come from the band
    // this run actually has, so the row says the same thing at every
    // eps rather than at the default one.
    let band = fixture::band();
    let under = set([band.zero() / 2.0, 0.0, 0.0])
        .expect_err("a length under the band's zero is decided zero");
    assert!(
        matches!(
            &under,
            EditError::PlacementAxis { error }
                if matches!(error.kind(), NodeErrorKind::DegenerateDirection { .. })
        ),
        "{under:?}"
    );
    set([band.escalate() * 2.0, 0.0, 0.0])
        .expect("a length clear of the band's escalation is a direction");

    set([0.0, 0.0, 1.0]).expect("a definite axis still goes through");
}
