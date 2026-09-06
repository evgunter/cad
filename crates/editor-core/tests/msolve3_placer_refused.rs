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
    Alignment, AxisSense, CapEnd, ContactClass, Datum, DocEdit, DocumentId, EditError, EvalOptions,
    Expr, Frame, MateFault, MateFrame, MatePrimitive, Node, NodeErrorKind, NodeResult, PatternKind,
    ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef, SlotId, StableName, solve_document,
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

    /// **What the placer's OWN evaluation raises**, on the twin —
    /// the comparison A1 is about, rendered through `Debug`, which is
    /// the structural reading of a [`NodeErrorKind`]: the kernel
    /// refusals it carries unaltered have no equality of their own.
    fn own_refusal(&self) -> String {
        let ev = run(&self.twin, &self.opts());
        match ev.result(self.placer) {
            Some(NodeResult::Failed(err)) => format!("{:?}", err.kind),
            other => panic!("the twin's placer must fail on its own: {other:?}"),
        }
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
        (doc, pattern, name)
    })
}

/// The scene builder both shapes share: two part documents, the
/// placed instance, the placer with the name the mate reads it by,
/// the capping instance, and the mate.
fn build<F>(label: &str, place: F) -> Scene
where
    F: FnOnce(ProfileDoc, RecipeNodeId) -> (ProfileDoc, RecipeNodeId, StableName),
{
    let mut store = PartStore::new();
    let leg = store.insert(block(&format!("{label}-leg")), Tol::witness());
    let top = store.insert(block(&format!("{label}-top")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
    let (doc, placer, name) = place(doc, legs);
    let (doc, cap) = insert(doc, Node::instantiate_part(top));
    let twin = doc.clone();
    let mut node = seat(name, in_part(cap, CapEnd::Start));
    if let Node::Mate { a, .. } = &mut node {
        // The reference is read AT the placer: that operand is what
        // puts the placer on the walk's chain.
        *a = SitedRef::new(placer, a.name.clone());
    }
    let (doc, mate) = step(doc, DocEdit::InsertNode { node });
    Scene {
        doc,
        twin,
        placer,
        mate: mate.expect("the mate mints"),
        store: Arc::new(store),
    }
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
    let scene = build("msolve3-transform", |doc, legs| {
        let (doc, moved) = insert(doc, xform(legs, [0.0, 0.0, 0.0], [1e200, 0.0, 0.0], 0.5));
        // A transform mints no name segment: the reference is the
        // part's own face, read AT the transform.
        (doc, moved, in_part(legs, CapEnd::End))
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

/// A circular rule whose `axis` operand is a PLANE datum is not an
/// axis to turn about. The solve re-derives the rule from the recipe
/// and refuses the evaluation's own `WrongOperand`, naming the
/// operand it wanted.
#[test]
fn a1_a_circular_rule_over_a_plane_datum_refuses_the_operand() {
    let mut store = PartStore::new();
    let leg = store.insert(block("msolve3-circular-leg"), Tol::witness());
    let top = store.insert(block("msolve3-circular-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("msolve3-circular"), Tol::witness());
    let (doc, legs) = insert(doc, Node::instantiate_part(leg));
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
    let (doc, cap) = insert(doc, Node::instantiate_part(top));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat(
                in_copy(pattern, 1, in_part(legs, CapEnd::End)),
                in_part(cap, CapEnd::Start),
            ),
        },
    );
    let mate = mate.expect("the mate mints");
    let f = solve_document(&doc, Tol::witness())
        .fault(mate)
        .cloned()
        .expect("the placer refuses");
    let (placer, kind) = carried(&f);
    assert_eq!(placer, pattern, "{f:?}");
    assert!(
        kind.contains("WrongOperand") && kind.contains("datum axis"),
        "the refusal names the operand the rule wanted: {kind}"
    );
    let _ = store;
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

    set([0.0, 0.0, 1.0]).expect("a definite axis still goes through");
    let _ = store;
}
