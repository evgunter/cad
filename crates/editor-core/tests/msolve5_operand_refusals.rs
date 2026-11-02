//! **The at-rest gate on a mate whose pose does not evaluate.**
//!
//! A mate's `on` is a pose read off the copy it places: for an
//! instance, a selection of the instance's own output. A selection that
//! resolves nothing, or a tie, is the selection's own refusal, met
//! where the placement evaluates: the placement refuses with it
//! (`MateFault::SideUnevaluated`), and the gather refuses the product
//! at that placement before any mate is read. A placement whose part
//! does not resolve refuses on its own, in the resolver's voice, and
//! what reads it is poisoned through it.
//!
//! **The kind question is not asked here, and no row for it belongs
//! here.** A face read is authored as a `SitedFace` over a `FaceName`,
//! so "what does this name denote" is decided by the type: `SitedFace`'s
//! own `compile_fail` row, `edit_one_predicate`'s load-door row and
//! `test_assembly_author.py`'s measure it.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;
use editor_core::Dimension;

use editor_core::{
    CapEnd, Coord, DocumentId, EntityKind, Entry, EvalOptions, Evaluation, Formula, MateFault,
    MateRef, MateSide, Node, NodeErrorKind, NodeResult, NodeStanding, PartFault, ProductError,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedFace, StableName, TargetAxis,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, run};
use geom_core::Tol;

// ---- the scene ----

/// A `w x w x h` box as a whole part document, and its body.
fn box_part(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

/// The slab is wide enough that the block seated on it stands clear
/// of its edges.
const BASE_WIDTH: f64 = 3.0;
const BASE_HEIGHT: f64 = 1.0;
const TOP_HEIGHT: f64 = 3.0;
const SPACING: f64 = 5.0;

/// The U-shaped outline, in the plane's own coordinates: a `4 x 2`
/// bar with a `3 x 1` notch cut in from its left end.
const U_OUTLINE: [(f64, f64); 8] = [
    (2.0, 1.0),
    (6.0, 1.0),
    (6.0, 3.0),
    (2.0, 3.0),
    (2.0, 2.5),
    (5.0, 2.5),
    (5.0, 1.5),
    (2.0, 1.5),
];

/// A `4 x 4 x 4` box with a U-shaped channel subtracted through its
/// middle (`z` from 1 to 3, clear of both caps): the cutter's tongue
/// leaves a side face in two fragments under ONE name, so the part's
/// product — and the instance's table — holds a TIED face row. The
/// subtract is the body the part places in its world.
fn slotted_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = box_part(label, 4.0, 4.0);
    let (doc, p) = insert(doc, Node::Profile(fixture::desc(vec![U_OUTLINE.to_vec()])));
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(2.0),
        },
    );
    let (doc, b) = fixture::slid(doc, body, b, [0.0, 0.0, 1.0]);
    insert(
        doc,
        Node::Subtract {
            from: body.into(),
            tool: b.into(),
        },
    )
}

struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    base_body: RecipeNodeId,
}

/// `base` (the slab) pinned to the world and `top` (the given part)
/// with an empty bundle, which the seat's constraints fill.
fn scene(label: &str, top_part: (ProfileDoc, RecipeNodeId)) -> Scene {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        box_part(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, _) = store.insert_part(top_part, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    Scene {
        doc,
        opts,
        base,
        top,
        base_body,
    }
}

/// The first TIED row of `kind` in the node's table.
fn tied_row(ev: &Evaluation<f64>, node: RecipeNodeId, kind: EntityKind) -> (StableName, usize) {
    fixture::table(ev, node)
        .iter()
        .find_map(|(n, e)| match e {
            Entry::Tied(c) if n.kind == kind => Some((n.clone(), c.len())),
            Entry::Tied(_) | Entry::Unique(_) => None,
        })
        .unwrap_or_else(|| panic!("node {} holds a tied {kind:?} row", node.0))
}

/// **`top`'s face `bottom` seated on `base`'s top cap at `(1, 1)`**, its
/// axes the slab's: a rest and the three values that pin it, in `top`'s
/// bundle. Answers the rest's address.
fn seat(
    doc: ProfileDoc,
    top: RecipeNodeId,
    base_top: SitedFace,
    bottom: SitedFace,
) -> (ProfileDoc, MateRef) {
    fixture::mated(
        doc,
        top,
        fixture::rest(bottom, base_top),
        &[
            (Coord::Slide(TargetAxis::X), len(1.0)),
            (Coord::Slide(TargetAxis::Y), len(1.0)),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    )
}

/// The placement `mate` refuses with its `on` pose's own refusal, and
/// that refusal is the selection's.
fn assert_on_selection_refuses(ev: &Evaluation<f64>, mate: MateRef, what: &str) {
    let refusal = match ev.result(mate.placement) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => match &**fault {
                MateFault::SideUnevaluated {
                    mate: m,
                    side: MateSide::On,
                    refusal,
                } if *m == mate => refusal.kind(),
                other => panic!("{what}: expected the copy's pose unevaluated, got {other:?}"),
            },
            other => panic!("{what}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{what}: expected the placement to fail, got {other:?}"),
    };
    assert!(
        matches!(refusal, NodeErrorKind::SelectResolve { .. }),
        "{what}: the pose's refusal is its selection's: {refusal:?}"
    );
}

/// The side's refusal is the block's: the gather refuses the product's
/// `⊔`, poisoned through the block's failed placement, before any mate
/// is read.
fn assert_refused_at_the_block(doc: &ProfileDoc, ev: &Evaluation<f64>, top: RecipeNodeId) {
    let err = editor_core::product_recorded(doc, ev, Tol::witness())
        .map(|_| ())
        .expect_err("the block's refusal refuses the product");
    assert!(
        matches!(
            &err,
            e
                if matches!(*e, ProductError::Read(NodeStanding::Poisoned { through, .. }) if through == top)
        ),
        "{err:?}"
    );
}

// ---- a name the copy does not carry ----

/// **A face name the copy does not carry refuses at evaluation, as the
/// selection's refusal.** The block's `on` names a face of a part node
/// the part does not have, so its plane is no value: the block's
/// placement refuses with the selection's own refusal, and the gather
/// refuses the product there.
///
/// *Breaks if* the placement evaluates over a pose it could not read,
/// or the door is asked to decide what only the evaluation knows.
#[test]
fn a_name_the_copy_does_not_carry_refuses_at_its_placement() {
    let s = scene(
        "msolve5-vanished",
        box_part("msolve5-vanished-top", 1.0, TOP_HEIGHT),
    );
    // The block's part has no node 99, so no face of `top` wears this
    // spelling.
    const NO_SUCH_PART_NODE: RecipeNodeId = RecipeNodeId::new(0, 99);
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: NO_SUCH_PART_NODE,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    };
    let base_top = fixture::head_at(s.base, in_part(s.base, s.base_body, CapEnd::End));
    let (doc, mate) = seat(s.doc, s.top, base_top, fixture::head_at(s.top, nowhere));
    let ev = run(&doc, &s.opts);
    assert_on_selection_refuses(&ev, mate, "a vanished name");
    assert_refused_at_the_block(&doc, &ev, s.top);
}

// ---- a tie ----

/// **A TIED face is the selection's to refuse.** The block's own table
/// ties a side face of the slotted part; a seat whose `on` reads it
/// refuses at the block's placement with the selection's refusal — a
/// tie is never broken by picking — and the gather refuses there.
///
/// *Breaks if* a tie resolves to one of its fragments.
#[test]
fn a_tied_face_refuses_at_its_placement() {
    let s = scene("msolve5-tied-face", slotted_part("msolve5-tied-face-top"));
    let ev0 = run(&s.doc, &s.opts);
    let (tied, width) = tied_row(&ev0, s.top, EntityKind::Face);
    assert_ne!(
        tied.node, s.top,
        "the tie is the part's name, which the instance's copy carries verbatim"
    );
    assert_eq!(width, 2);
    let base_top = fixture::head_at(s.base, in_part(s.base, s.base_body, CapEnd::End));
    let (doc, mate) = seat(s.doc, s.top, base_top, fixture::head_at(s.top, tied));
    let ev = run(&doc, &s.opts);
    assert_on_selection_refuses(&ev, mate, "a tied face");
    assert_refused_at_the_block(&doc, &ev, s.top);
}

// ---- an operand that is not live never reaches the gate ----

/// **A part that does not resolve refuses at its instance, and what
/// reads it is poisoned.** The top part lives in another store: the
/// block's instance fails in the resolver's own voice, a placed copy
/// of it and a family over that are poisoned, and every copy of a
/// member a placement copies is poisoned too. The gather refuses the
/// document at the first failed placement — the block — before any
/// mate is read.
///
/// *Breaks if* the block's failure is anything but its part's, or the
/// gather reads past a failed placement.
#[test]
fn a_poisoned_operand_never_reaches_the_gate() {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        box_part("msolve5-poisoned-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    // The top part lives in ANOTHER store: unresolvable here.
    let mut elsewhere = PartStore::new();
    let (top_ref, top_body) = elsewhere.insert_part(
        box_part("msolve5-poisoned-top", 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve5-poisoned"), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // A copy of the top's copy, lifted by values, which read no
    // geometry of a copy that does not evaluate.
    let (doc, xf) = insert(
        doc,
        crate::fixture::stood(editor_core::BodyRead::plain(editor_core::Operand::Node(
            top,
        ))),
    );
    let doc = fixture::pin(
        doc,
        xf,
        &editor_core::Frame::translation([0.0, 0.0, 10.0]),
        &opts,
    );
    let (doc, _, pattern) = crate::fixture::linear(doc, xf, [1.0, 0.0, 0.0], SPACING, 2);
    // The family's members, each copied by a placement of its own
    // reading `pattern[i]`: a copy that does not evaluate cannot be
    // pinned over its geometry, so each is a loose copy, which is still
    // a placement the gather reads.
    let mut doc = doc;
    let mut copies = Vec::new();
    for i in 0..2 {
        let (next, placed) = fixture::member(doc, pattern, i);
        doc = next;
        copies.push(placed);
    }
    let base_top = fixture::head_at(base, in_part(base, base_body, CapEnd::End));
    let (doc, _) = seat(
        doc,
        top,
        base_top,
        fixture::head_at(top, in_part(top, top_body, CapEnd::Start)),
    );
    let ev = run(&doc, &opts);
    assert!(
        matches!(
            ev.result(top),
            Some(NodeResult::Failed(e)) if matches!(
                &e.kind,
                NodeErrorKind::Part {
                    fault: PartFault::Unresolved { .. },
                    ..
                }
            )
        ),
        "the instance fails in the resolver's own voice: {:?}",
        ev.result(top)
    );
    for (node, what) in [(xf, "the lifted copy"), (pattern, "the family")]
        .into_iter()
        .chain(copies.iter().map(|&c| (c, "a placed copy")))
    {
        assert!(
            matches!(ev.result(node), Some(NodeResult::Poisoned { .. })),
            "{what} is poisoned: {:?}",
            ev.result(node)
        );
    }
    assert!(
        solve_fault_free(&doc, &opts, top),
        "the block's refusal is its part's, so no bundle refused"
    );
    // The block is a member of the product's `⊔`, failed on its part,
    // so the gather refuses the `⊔` poisoned through THAT placement,
    // before any reference is read.
    let err = editor_core::product_recorded(&doc, &ev, Tol::witness())
        .map(|_| ())
        .expect_err("the gather refuses");
    assert!(
        matches!(
            &err,
            e
                if matches!(*e, ProductError::Read(NodeStanding::Poisoned { through, .. }) if through == top)
        ),
        "the gather refuses at the failed placement before any reference is read: {err:?}"
    );
}

/// Whether the solve records no bundle refusal for `placement`.
fn solve_fault_free(doc: &ProfileDoc, opts: &EvalOptions, placement: RecipeNodeId) -> bool {
    fixture::solve(doc, opts, Tol::witness())
        .fault(placement)
        .is_none()
}
