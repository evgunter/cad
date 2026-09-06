//! **The mate solve reports the evaluation's own refusal.**
//!
//! A mate whose reference is placed by a pattern copy or a transform
//! used to refuse `DanglingHead` for every way the pose failed to
//! derive — a slot that does not evaluate, a direction of no definite
//! length, an explicit rule — about a head that resolves and a placer
//! that exists. The doc's justification was that the placer node
//! names the cause in its own voice; it cannot, because the mate
//! fault POISONS the document and that node evaluates to `Poisoned`
//! instead.
//!
//! These rows measure the replacement. `MateFault::PlacerRefused`
//! carries the evaluation layer's own typed refusal UNALTERED, and
//! the headline rows compare it against what the same node's own
//! evaluation raises on the TWIN document — the same recipe with the
//! mate left out, where nothing poisons it. `DanglingHead` keeps
//! exactly its two causes.
//!
//! Every row goes through ordinary doors: `DocEdit::InsertNode`,
//! `solve_document`, `evaluate`, `apply`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::{
    Alignment, Axis3, AxisSense, CapEnd, ContactClass, Datum, DocEdit, DocumentId, EditError,
    EvalOptions, Expr, Frame, MateFault, MateFrame, MatePrimitive, Node, NodeErrorKind, NodeResult,
    PatternKind, ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef, SlotId, StableName,
    solve_document,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{ang, in_copy, insert, len, on_frame, run, scl, step, xform};
use geom_core::Tol;

// ---- the scene ----

/// A `1 x 1 x 1` block, as a whole part document.
fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// The seat every row's mate declares.
fn seat(a: StableName, b: StableName) -> Node<ProfileProgram> {
    let frame = |origin: [f64; 3], axis: [f64; 3]| MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    };
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// What every row builds: a part instance, whatever places it, a
/// second instance, and a mate onto the placed body.
///
/// `twin` is the same document with the mate left OUT, and its node
/// ids are the mated document's because the mate is inserted last —
/// so the node the twin evaluates is the node the mate names.
struct Scene {
    doc: ProfileDoc,
    twin: ProfileDoc,
    placer: RecipeNodeId,
    mate: RecipeNodeId,
    store: Arc<PartStore>,
}

impl Scene {
    /// The evaluation options every row runs through.
    fn opts(&self) -> EvalOptions {
        EvalOptions {
            resolver: Some(Arc::clone(&self.store) as Arc<dyn editor_core::PartResolver>),
            ..EvalOptions::default()
        }
    }

    /// The fault the solve records for the mate.
    fn fault(&self) -> MateFault {
        solve_document(&self.doc, Tol::witness())
            .fault(self.mate)
            .cloned()
            .expect("the placer refuses")
    }

    /// **What a node's OWN evaluation raises**, on the twin — the
    /// comparison A1 is about, rendered through `Debug`, which is the
    /// structural reading of a [`NodeErrorKind`]: the kernel refusals
    /// it carries unaltered have no equality of their own.
    fn own_refusal_of(&self, node: RecipeNodeId) -> String {
        let ev = run(&self.twin, &self.opts());
        match ev.result(node) {
            Some(NodeResult::Failed(err)) => format!("{:?}", err.kind),
            other => panic!("node {node:?} must fail on its own on the twin: {other:?}"),
        }
    }

    /// The refusal the placer itself raises on the twin.
    fn own_refusal(&self) -> String {
        self.own_refusal_of(self.placer)
    }

    /// The mated document's own row for the placer — `Poisoned`, by
    /// the mate fault, which is why the cause cannot be read there.
    fn placer_row(&self) -> String {
        let ev = run(&self.doc, &self.opts());
        format!("{:?}", ev.result(self.placer))
    }
}

/// A scene whose placer is a PATTERN of `kind` at `count`, mated onto
/// copy `i` — the name carries the `Instance(i)` qualifier the walk
/// consumes.
fn patterned(label: &str, kind: PatternKind, count: i64, i: u32) -> Scene {
    build(label, |doc, legs| {
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs,
                count: Expr::count(count),
                kind,
            },
        );
        let name = in_copy(pattern, i, in_part(legs, CapEnd::End));
        (doc, pattern, name, Vec::new())
    })
    .0
}

/// A slot the edit door admits and the evaluator refuses: a count
/// promoted out of the exactly-representable range. (An unbound
/// parameter cannot be used — `InsertNode` refuses it.)
fn unevaluable() -> Expr {
    Expr::count_to_scalar(Expr::count(1 << 40)).expect("a count promotes to a scalar")
}

/// The scene builder both shapes share: two part documents, the
/// placed instance, the placer with the name the mate reads it by,
/// the capping instance, and the mate.
fn build<F>(label: &str, place: F) -> (Scene, Vec<RecipeNodeId>)
where
    F: FnOnce(
        ProfileDoc,
        RecipeNodeId,
    ) -> (ProfileDoc, RecipeNodeId, StableName, Vec<RecipeNodeId>),
{
    let mut store = PartStore::new();
    let leg = store.insert(block(&format!("{label}-leg")), Tol::witness());
    let top = store.insert(block(&format!("{label}-top")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
    let (doc, placer, name, extra) = place(doc, legs);
    let (doc, cap) = insert(doc, Node::instantiate_part(top));
    let twin = doc.clone();
    let mut node = seat(name, in_part(cap, CapEnd::Start));
    if let Node::Mate { a, .. } = &mut node {
        // The reference is read AT the placer: that operand is what
        // puts the placer on the walk's chain.
        *a = SitedRef::new(placer, a.name.clone());
    }
    let (doc, mate) = step(doc, DocEdit::InsertNode { node });
    (
        Scene {
            doc,
            twin,
            placer,
            mate: mate.expect("the mate mints"),
            store: Arc::new(store),
        },
        extra,
    )
}

/// The refusal a `PlacerRefused` carries, and the node it names.
fn carried(fault: &MateFault) -> (RecipeNodeId, String) {
    match fault {
        MateFault::PlacerRefused { placer, error, .. } => (*placer, format!("{:?}", error.kind())),
        other => panic!("expected PlacerRefused, got {other:?}"),
    }
}

// ---- A1: the cause, in the placer's own words ----

/// **The finding's own document.** A pattern direction of `1e200` has
/// a LENGTH that overflows to infinity. The mate names copy 1; the
/// solve refuses `PlacerRefused` naming the pattern and carrying
/// `NonFiniteDirection { role: "pattern direction" }` — the SAME kind
/// the pattern's own evaluation raises on the unpoisoned twin — while
/// the pattern's row in the mated document reads `Poisoned`, so the
/// mate's fault is the only place the cause appears.
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
    let f = scene.fault();
    let (placer, kind) = carried(&f);
    assert_eq!(placer, scene.placer, "the fault names the pattern: {f:?}");
    assert_eq!(
        kind,
        scene.own_refusal(),
        "the carried refusal is the one the placer's own evaluation raises"
    );
    assert!(
        kind.contains("NonFiniteDirection") && kind.contains("pattern direction"),
        "and it is the direction door's own: {kind}"
    );
    assert!(
        f.to_string().contains("pattern direction"),
        "the prose says which vector: {f}"
    );
    assert!(
        scene.placer_row().contains("Poisoned"),
        "the placer's own row cannot state it: {}",
        scene.placer_row()
    );
}

/// A DECIDED-ZERO direction is the other half of the same door, and
/// it is the case the retired doc listed as a dangling head by name.
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
    let f = scene.fault();
    let (_, kind) = carried(&f);
    assert_eq!(kind, scene.own_refusal(), "the twin raises the same kind");
    assert!(
        kind.contains("DegenerateDirection") && kind.contains("pattern direction"),
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
            spacing: Expr::mul(len(1e200), scl(1e200)).expect("length times scalar is a length"),
        },
        4,
        1,
    );
    let f = scene.fault();
    let (_, kind) = carried(&f);
    assert_eq!(kind, scene.own_refusal(), "the twin raises the same kind");
    assert!(
        kind.contains("Expr") && kind.contains(&format!("{:?}", SlotId::Spacing)),
        "the refusal names the slot it read: {kind}"
    );
}

/// A TRANSFORM on the chain refuses in its own role word: the same
/// door, a different vector.
#[test]
fn a1_a_transform_with_a_non_finite_axis_names_its_axis() {
    let (scene, _) = build("msolve3-transform", |doc, legs| {
        let (doc, moved) = insert(doc, xform(legs, [0.0, 0.0, 0.0], [1e200, 0.0, 0.0], 0.5));
        // A transform mints no name segment: the reference is the
        // part's own face, read AT the transform.
        (doc, moved, in_part(legs, CapEnd::End), Vec::new())
    });
    let f = scene.fault();
    let (placer, kind) = carried(&f);
    assert_eq!(placer, scene.placer, "{f:?}");
    assert_eq!(kind, scene.own_refusal(), "the twin raises the same kind");
    assert!(
        kind.contains("NonFiniteDirection") && kind.contains("transform rotation axis"),
        "{kind}"
    );
}

/// **Two faults on one node, and the same winner on both roads.** A
/// transform whose axis is degenerate AND whose angle does not
/// evaluate: the node's slot order decides which refusal is reported,
/// and the mate road must not report a different one from the node's
/// own evaluation.
#[test]
fn a1_two_faults_on_one_placer_pick_the_same_winner() {
    let (scene, _) = build("msolve3-two-faults", |doc, legs| {
        let mut t = xform(legs, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.5);
        if let Node::Transform { rotation_angle, .. } = &mut t {
            *rotation_angle = Expr::mul(ang(1e200), scl(1e200)).expect("angle times scalar");
        }
        let (doc, moved) = insert(doc, t);
        (doc, moved, in_part(legs, CapEnd::End), Vec::new())
    });
    let f = scene.fault();
    let (_, kind) = carried(&f);
    assert_eq!(
        kind,
        scene.own_refusal(),
        "the two roads read the node's slots in one order"
    );
}

/// **A circular rule whose `axis` operand is a PLANE datum.** The
/// operand-kind question belongs to the pattern's own wiring, so the
/// refusal is the pattern's and its `found` word is the value family
/// the evaluation would report — compared against the twin, so the
/// recipe-side reading of that family cannot drift from the payload
/// one.
#[test]
fn a1_a_circular_rule_over_a_plane_datum_refuses_the_operand() {
    let (scene, _) = build("msolve3-circular-plane", |doc, legs| {
        let (doc, plane) = insert(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(0.0), len(0.0), len(0.0)],
                normal: [scl(0.0), scl(0.0), scl(1.0)],
            }),
        );
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs,
                count: Expr::count(4),
                kind: PatternKind::Circular {
                    axis: plane,
                    step: ang(0.5),
                },
            },
        );
        let name = in_copy(pattern, 1, in_part(legs, CapEnd::End));
        (doc, pattern, name, vec![plane])
    });
    let f = scene.fault();
    let (placer, kind) = carried(&f);
    assert_eq!(placer, scene.placer, "the pattern's wiring refuses: {f:?}");
    assert_eq!(kind, scene.own_refusal(), "word for word with the twin's");
    assert!(
        kind.contains("WrongOperand") && kind.contains("datum axis") && kind.contains("\"datum\""),
        "{kind}"
    );
}

/// The same operand question over a BODY — the second value family a
/// circular rule's axis is authored as by mistake. It is here so the
/// recipe-side family word is pinned against the evaluation's for
/// more than one answer.
#[test]
fn a1_a_circular_rule_over_a_body_refuses_the_operand() {
    let (scene, _) = build("msolve3-circular-body", |doc, legs| {
        // A body where an axis datum belongs — a second node, because
        // one input may not be the same node twice.
        let (doc, body) = insert(doc, xform(legs, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0));
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs,
                count: Expr::count(4),
                kind: PatternKind::Circular {
                    axis: body,
                    step: ang(0.5),
                },
            },
        );
        let name = in_copy(pattern, 1, in_part(legs, CapEnd::End));
        (doc, pattern, name, Vec::new())
    });
    let f = scene.fault();
    let (_, kind) = carried(&f);
    assert_eq!(kind, scene.own_refusal(), "word for word with the twin's");
    assert!(
        kind.contains("WrongOperand") && kind.contains("\"body\""),
        "{kind}"
    );
}

/// **A slot of the axis DATUM is reported at the datum.** The pattern
/// has no `Direction(X)` slot, so naming the pattern here would blame
/// a node for a slot it does not carry: the refusal names the node
/// whose evaluation raised it, which is the node an author goes and
/// fixes. On the twin the pattern is merely `Poisoned` through that
/// datum, which is why the mate's fault is the only place the cause
/// is legible.
#[test]
fn a1_an_axis_datums_slot_refusal_is_reported_at_the_datum() {
    let (scene, extra) = build("msolve3-circular-datum-slot", |doc, legs| {
        let (doc, axis) = insert(
            doc,
            Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: [unevaluable(), scl(0.0), scl(1.0)],
            }),
        );
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs,
                count: Expr::count(4),
                kind: PatternKind::Circular {
                    axis,
                    step: ang(0.5),
                },
            },
        );
        let name = in_copy(pattern, 1, in_part(legs, CapEnd::End));
        (doc, pattern, name, vec![axis])
    });
    let datum = extra[0];
    let f = scene.fault();
    let (placer, kind) = carried(&f);
    assert_eq!(
        placer, datum,
        "the datum raised it, so the datum is named: {f:?}"
    );
    assert_eq!(
        kind,
        scene.own_refusal_of(datum),
        "word for word with the datum's own"
    );
    assert!(
        kind.contains(&format!("{:?}", SlotId::Direction(Axis3::X))),
        "{kind}"
    );
    assert!(
        f.to_string().contains(&format!("node {}", datum.0)),
        "and the message names that node: {f}"
    );
}

/// The datum's DIRECTION, decided: the vector is the datum's, so the
/// refusal is reported at the datum and carries the datum's role word.
#[test]
fn a1_an_axis_datums_degenerate_direction_is_reported_at_the_datum() {
    let (scene, extra) = build("msolve3-circular-datum-zero", |doc, legs| {
        let (doc, axis) = insert(
            doc,
            Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: [scl(0.0), scl(0.0), scl(0.0)],
            }),
        );
        let (doc, pattern) = insert(
            doc,
            Node::Pattern {
                input: legs,
                count: Expr::count(4),
                kind: PatternKind::Circular {
                    axis,
                    step: ang(0.5),
                },
            },
        );
        let name = in_copy(pattern, 1, in_part(legs, CapEnd::End));
        (doc, pattern, name, vec![axis])
    });
    let datum = extra[0];
    let f = scene.fault();
    let (placer, kind) = carried(&f);
    assert_eq!(placer, datum, "{f:?}");
    assert_eq!(
        kind,
        scene.own_refusal_of(datum),
        "word for word with the datum's own"
    );
    assert!(
        kind.contains("DegenerateDirection") && kind.contains("datum axis direction"),
        "{kind}"
    );
}

/// **The explicit rule, measured where it can be reached.** A
/// `Node::Pattern` carries its count as a slot, so an explicit
/// placement list is a second spelling of the same number and the
/// EDIT door refuses it — no document the solve reads can carry one,
/// and `derived_offset`'s `PlacementRule(CountSpelling)` arm is the
/// backstop for a hand-built one. This row pins the door that keeps
/// it unreachable rather than claiming to exercise the arm.
#[test]
fn an_explicit_pattern_rule_never_reaches_the_solve() {
    let mut store = PartStore::new();
    let leg = store.insert(block("msolve3-explicit-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("msolve3-explicit"), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
    let refused = editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Pattern {
                input: legs,
                count: Expr::count(2),
                kind: PatternKind::Explicit(vec![
                    Frame::IDENTITY,
                    Frame::translation([2.0, 0.0, 0.0]),
                ]),
            },
        },
        Tol::witness(),
    )
    .expect_err("a pattern spells its count once");
    assert!(
        matches!(refused, EditError::PlacementRuleMismatch { .. }),
        "{refused:?}"
    );
    let _ = store;
}

// ---- what stays a dangling head ----

/// A copy index at the pattern's count names a copy that does not
/// exist — still `DanglingHead`, at the pattern.
#[test]
fn an_index_at_the_count_is_still_a_dangling_head() {
    let scene = patterned(
        "msolve3-past-count",
        PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
        2,
        2,
    );
    let f = scene.fault();
    assert!(
        matches!(&f, MateFault::DanglingHead { head, .. } if *head == scene.placer),
        "{f:?}"
    );
}

/// A stranded operand stops the walk — still `DanglingHead`, naming
/// the node the walk stopped at.
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
    let (doc, _) = step(scene.doc, DocEdit::DeleteNode { id: scene.placer });
    let f = solve_document(&doc, Tol::witness())
        .fault(scene.mate)
        .cloned()
        .expect("the stranded mate refuses");
    assert!(
        matches!(&f, MateFault::DanglingHead { head, .. } if *head == scene.placer),
        "{f:?}"
    );
}

// ---- the rider ----

/// **A degenerate placement axis refuses naming the AXIS.** The
/// constructor decides the axis through the direction door, so an
/// author who builds a frame and sets it reads the vector they
/// authored — not the frame the constructor would have built.
#[test]
fn the_placement_axis_refuses_in_its_own_voice() {
    let mut store = PartStore::new();
    let part = store.insert(block("msolve3-rider-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("msolve3-rider"), Tol::witness());
    let (doc, instance) = insert(doc, Node::instantiate_part(part));

    // The natural spelling: build the frame, set it, one error type.
    let set = |axis: [f64; 3]| -> Result<ProfileDoc, EditError> {
        let frame = Frame::rotate_then_translate(axis, 0.5, [1.0, 2.0, 3.0], fixture::band())?;
        Ok(editor_core::apply(
            &doc,
            &DocEdit::SetPlacement {
                node: instance,
                frame,
            },
            Tol::witness(),
        )?
        .doc)
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
    let _ = store;
}
