//! **The at-rest gate on a mate read below a placer.**
//!
//! `P(T(top))` with the mate read AT `T`: the solve places it and the
//! product gathers. A mate side is a selection of the body it is read
//! in, and the gate mints on the world copy of that body; no placement
//! reads `T` — the pattern's copies are the world — so the mate is no
//! statement about the product, and the seat it would have declared is
//! an unattributed contact. A side whose selection resolves nothing, or
//! a tie, is the selection's own refusal at evaluation.
//!
//! The control (the same document read AT the pattern, with the
//! instance spelling) mints as it did.
//!
//! **The kind question is not asked here, and no row for it belongs
//! here.** A mate side is authored as a `SitedFace` over a `FaceName`,
//! so "what does this name denote" is decided by the type: `SitedFace`'s
//! own `compile_fail` row, `edit_one_predicate`'s load-door row and
//! `test_assembly_author.py`'s measure it.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CapEnd, ContactClass, DocumentId, EntityKind, Entry,
    EvalOptions, Evaluation, Formula, LeverRefusal, MateFault, MateFrame, MatePrimitive, MateRef,
    Node, NodeErrorKind, NodeResult, NodeStanding, PartSelect, PatternKind, ProductError,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedFace, Space, StableName, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{gate, in_copy, insert, len, on_frame, run, scl, solve, xform};
use geom_core::Tol;

// ---- the scene ----

/// **The copy of `body` a node over an instance carries** — `node`'s
/// copies keyed as the instance keys the copy of its part's `body`
/// ([`fixture::resolver::copy_key`]): what a seat reading one body
/// reads of a transform over the instance.
fn one_copy(node: RecipeNodeId, body: RecipeNodeId) -> editor_core::BodyRead<Formula> {
    editor_core::BodyRead::keyed(
        editor_core::Operand::Node(node),
        fixture::resolver::copy_key(body),
    )
}

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
            side: ExtrudeSide::Along,
        },
    )
}

/// The slab is wide enough that the block seated on it stands clear
/// of its edges, and the pattern's spacing puts every other copy
/// clear of the slab altogether — the rows are about the copy the
/// mate names, and a sibling resting uninvited is an undeclared
/// contact the gate is right to refuse.
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
/// product — and every table above it — holds a TIED face row. The
/// subtract is the body the part places in its world.
fn slotted_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = box_part(label, 4.0, 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![U_OUTLINE.to_vec()],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    insert(
        doc,
        Node::Subtract {
            from: body.into(),
            tool: b.into(),
        },
    )
}

/// **The world the product is** (A10): beside the instances, which are
/// placements of their own, each of `bodies` placed, then each
/// `(pattern, count)`'s copies placed through a `Part` per copy.
fn world(doc: ProfileDoc, bodies: &[RecipeNodeId], patterns: &[(RecipeNodeId, i64)]) -> ProfileDoc {
    let doc = fixture::place_all(doc, bodies);
    patterns.iter().fold(doc, |doc, &(pattern, count)| {
        (0..count).fold(doc, |doc, i| {
            let (doc, copy) = insert(
                doc,
                Node::Part {
                    of: pattern.into(),
                    select: editor_core::PartSelect::Instance(Formula::count(i)),
                },
            );
            fixture::place(doc, copy).0
        })
    })
}

struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top: RecipeNodeId,
    base_body: RecipeNodeId,
    top_body: RecipeNodeId,
    xf: RecipeNodeId,
    pattern: RecipeNodeId,
}

/// The scene up to `T(top)`, over the given top part: `base` (the slab)
/// pinned to the world, `top` (the block) with an empty bundle, which
/// the seat's constraint fills, and `T(top)` lifted well clear of the
/// slab; with each instance's part body.
fn lifted(label: &str, top_part: (ProfileDoc, RecipeNodeId)) -> Scene {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        box_part(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) = store.insert_part(top_part, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    Scene {
        doc,
        opts,
        base,
        top,
        base_body,
        top_body,
        xf,
        pattern: xf,
    }
}

/// The two-copy linear pattern over `T(top)`.
fn patterned(s: Scene) -> Scene {
    let (doc, pattern) = insert(
        s.doc,
        Node::Pattern {
            input: s.xf.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(SPACING),
            },
        },
    );
    Scene { doc, pattern, ..s }
}

/// `P(T(top))` over the given top part.
fn scene_with(label: &str, top_part: (ProfileDoc, RecipeNodeId)) -> Scene {
    patterned(lifted(label, top_part))
}

/// `P(T(top))` over the plain block.
fn scene(label: &str) -> Scene {
    scene_with(label, box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT))
}

/// `T(top)` over the plain block, with NO pattern: `T` is a root.
fn scene_no_pattern(label: &str) -> Scene {
    lifted(label, box_part(&format!("{label}-top"), 1.0, TOP_HEIGHT))
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

/// A `Rest` mate seating `b`'s bottom cap on `a`'s top cap by frame
/// coincidence, both frames in their member's own part coordinates
/// and both axes outward, so the block stands ON the slab.
fn seat(a: SitedFace, b: SitedFace) -> editor_core::Mate<Formula> {
    fixture::mate(
        a,
        b,
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored(
                [1.0, 1.0, BASE_HEIGHT],
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            b: MateFrame::authored(
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [1.0, 0.0, 0.0],
                geom_core::Tol::witness(),
            )
            .expect("a definite frame"),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    )
}

/// `mate` added to the bundle of `top`, the block it seats — the copy
/// its `b` side reads through whatever placer stands between — and
/// answered by its address.
fn mated(
    doc: ProfileDoc,
    top: RecipeNodeId,
    mate: editor_core::Mate<Formula>,
) -> (ProfileDoc, MateRef) {
    fixture::constrain_at(doc, top, mate)
}

// ---- A1: the issue's document ----

/// **The issue's own document.** The mate is read at `T`, below the
/// pattern that places it again. The solve places it (`Determining`),
/// the product gathers, and no placement reads `T`, so the mate has no
/// world copy and mints nothing.
#[test]
fn the_issues_document_mints_nothing_for_a_side_read_below_the_pattern() {
    let s = scene("msolve5-a1");
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let b = crate::fixture::head_at(s.xf, in_part(s.top, s.top_body, CapEnd::Start));
    let doc = world(s.doc, &[], &[(s.pattern, 2)]);
    let (doc, mate) = mated(doc, s.top, seat(a, b));

    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate.placement).is_none(),
        "the solve places a mate read at the transform: {:?}",
        poses.fault(mate.placement)
    );
    assert_eq!(
        poses.space(mate.placement),
        Some(Space::World),
        "the seat pins the block against the slab, which is on the world"
    );
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "the product gathers"
    );

    crate::fixture::assert_mints_nothing(&doc, &ev, mate);
}

// ---- the control: read AT the pattern ----

/// The same document read AT `P` with the `Instance(0, top/…)`
/// spelling holds at the gate: copy 0 IS `T(top)`, and the pattern's
/// row is a product root's own row.
#[test]
fn read_at_the_pattern_with_the_instance_spelling_the_gate_holds() {
    let s = scene("msolve5-control");
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let b = crate::fixture::head_at(
        s.pattern,
        in_copy(s.pattern, 0, in_part(s.top, s.top_body, CapEnd::Start)),
    );
    let doc = world(s.doc, &[], &[(s.pattern, 2)]);
    let (doc, mate) = mated(doc, s.top, seat(a, b));
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate.placement).is_none(),
        "{:?}",
        poses.fault(mate.placement)
    );
    let ev = run(&doc, &s.opts);
    assert!(
        gate(&doc, &ev).is_ok(),
        "read at the root, the gate holds: {:?}",
        gate(&doc, &ev)
    );
}

// ---- a pass-through root over the pattern ----

/// A mate read at a `Part { Instance(0) }` root over the pattern
/// holds: a pass-through root's rows are verbatim — the `Part`
/// projects the pattern's `Instance(0)` rows under their own heads —
/// so the product's table answers and the operand is never asked.
#[test]
fn a_mate_read_at_a_part_root_over_the_pattern_holds() {
    let s = scene("msolve5-part-root");
    let (doc, part) = insert(
        s.doc,
        Node::Part {
            of: s.pattern.into(),
            select: PartSelect::Instance(Formula::count(0)),
        },
    );
    let doc = world(doc, &[part], &[]);
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let b = crate::fixture::head_at(
        part,
        in_copy(s.pattern, 0, in_part(s.top, s.top_body, CapEnd::Start)),
    );
    let (doc, mate) = mated(doc, s.top, seat(a, b));
    let poses = solve(&doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(mate.placement).is_none(),
        "{:?}",
        poses.fault(mate.placement)
    );
    let ev = run(&doc, &s.opts);
    assert!(
        gate(&doc, &ev).is_ok(),
        "read at a pass-through root, the gate holds: {:?}",
        gate(&doc, &ev)
    );
}

// ---- A2: the order of the two questions ----

/// A mate read at `T` naming a face `top` does not have refuses: the
/// side's selection resolves nothing in the body it reads, so the
/// block whose bundle holds the mate refuses with it.
#[test]
fn a_name_the_operand_does_not_spell_stays_vanished() {
    let s = scene("msolve5-vanished");
    // The block's part has no node 99 — its body is `s.top_body` — so
    // no face of `top` wears this spelling: at `T`, at the pattern,
    // or anywhere.
    const NO_SUCH_PART_NODE: RecipeNodeId = RecipeNodeId::new(0, 99);
    assert_ne!(NO_SUCH_PART_NODE, s.top_body);
    let nowhere = StableName {
        kind: EntityKind::Face,
        node: s.top,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: NO_SUCH_PART_NODE,
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let b = crate::fixture::head_at(s.xf, nowhere);
    let doc = world(s.doc, &[], &[(s.pattern, 2)]);
    let (doc, mate) = mated(doc, s.top, seat(a, b));
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            ev.node_error(mate.placement).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::SelectResolve { .. })
        ),
        "the side's selection refuses: {:?}",
        ev.node_error(mate.placement)
    );
    // The constraint is the block's, and the block is a placement of
    // the product: its side's refusal is the block's, and the gather
    // refuses there before any mate is read.
    assert_refused_at_the_block(&doc, &ev, s.top);
}

/// The side's refusal is the block's: the gather refuses at the
/// block's placement, failed, before any mate is read.
fn assert_refused_at_the_block(doc: &ProfileDoc, ev: &Evaluation<f64>, top: RecipeNodeId) {
    let err = gate(doc, ev).expect_err("the block's refusal refuses the product");
    assert!(
        matches!(
            &err,
            AssemblyError::Product(e)
                if matches!(**e, ProductError::Root(NodeStanding::Failed { node }) if node == top)
        ),
        "{err:?}"
    );
}

// ---- ties: the product decides its own ----

/// A TIED face is the side's selection's to refuse, wherever it is
/// read: at `T` below the pattern, and AT the pattern with the instance
/// spelling. The mate refuses with it, and the refusal is the block's,
/// whose bundle holds the mate: the gather refuses at the block.
#[test]
fn a_tied_face_refuses_at_the_side_wherever_it_is_read() {
    let s = scene_with("msolve5-tied-face", slotted_part("msolve5-tied-face-top"));
    let ev0 = run(&s.doc, &s.opts);
    let (tied, width) = tied_row(&ev0, s.xf, EntityKind::Face);
    assert_eq!(tied.node, s.top, "the tie is worn by the instance");
    assert_eq!(width, 2);
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let placed = world(s.doc, &[], &[(s.pattern, 2)]);
    for (at, b) in [
        ("T", crate::fixture::head_at(s.xf, tied.clone())),
        (
            "the pattern",
            crate::fixture::head_at(s.pattern, in_copy(s.pattern, 0, tied.clone())),
        ),
    ] {
        let (doc, mate) = mated(placed.clone(), s.top, seat(a.clone(), b));
        let ev = run(&doc, &s.opts);
        assert!(
            matches!(
                ev.node_error(mate.placement).map(|e| &e.kind),
                Some(editor_core::NodeErrorKind::SelectResolve { .. })
            ),
            "read at {at}: a tie is never broken by picking: {:?}",
            ev.node_error(mate.placement)
        );
        assert_refused_at_the_block(&doc, &ev, s.top);
    }
}

// ---- a consumer that drops the face names itself ----

/// `Intersect(T(top), far)` — a boolean whose result is EMPTY — is
/// placed over `T`. No placement reads `T`, so the mate read there has
/// no world copy and mints nothing.
#[test]
fn an_operand_under_an_empty_boolean_placement_mints_nothing() {
    let s = scene_no_pattern("msolve5-empty-root");
    let (doc, far_profile) = on_frame(
        s.doc,
        [50.0, 50.0, 50.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, far) = insert(
        doc,
        Node::Extrude {
            profile: far_profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, empty) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![one_copy(s.xf, s.top_body), far.into()]),
        },
    );
    let doc = world(doc, &[empty], &[]);
    let a = crate::fixture::head(in_part(s.base, s.base_body, CapEnd::End));
    let b = crate::fixture::head_at(s.xf, in_part(s.top, s.top_body, CapEnd::Start));
    let (doc, mate) = mated(doc, s.top, seat(a, b));
    let ev = run(&doc, &s.opts);
    assert!(
        product(&doc, &ev, Tol::witness()).is_ok(),
        "an empty root contributes nothing and the product gathers"
    );
    crate::fixture::assert_mints_nothing(&doc, &ev, mate);
    let _ = empty;
}

// ---- an operand that is not live never reaches the gate ----

/// The top part cannot be resolved, so `top`, `T` and `P` are failed
/// or poisoned — and the SOLVE, whose lever is the mated parts' own
/// extent, faults the mate with the part (`Unleverable`, carrying the
/// resolver's fault), so the block's placement fails in the mate's
/// words. What keeps the gate from reading a table that does not exist
/// is still not the solve: every live node sits under some placement,
/// so the gather's first pass refuses the document at that placement
/// before any mate is read.
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
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, xf) = insert(doc, xform(top, [0.0, 0.0, 10.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: xf.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(SPACING),
            },
        },
    );
    let doc = world(doc, &[], &[(pattern, 2)]);
    let a = crate::fixture::head(in_part(base, base_body, CapEnd::End));
    let b = crate::fixture::head_at(xf, in_part(top, top_body, CapEnd::Start));
    let (doc, mate) = mated(doc, top, seat(a, b));
    // The mate has no lever without its part: it faults in the
    // resolver's own voice, on the block's placement.
    let poses = solve(&doc, &opts, Tol::witness());
    assert_eq!(
        poses.space(mate.placement),
        None,
        "the refused block has no space"
    );
    let fault = poses
        .fault(mate.placement)
        .cloned()
        .expect("the mate faults with its part");
    assert!(
        matches!(
            &fault,
            MateFault::Unleverable { refusal, .. } if matches!(
                refusal.as_ref(),
                LeverRefusal::Reach {
                    instance,
                    refusal: editor_core::ReachRefusal::PartUnresolved { .. },
                    ..
                } if *instance == top
            )
        ),
        "{fault:?}"
    );
    let ev = run(&doc, &opts);
    assert!(
        matches!(ev.result(top), Some(NodeResult::Failed(_))),
        "the instance fails: {:?}",
        ev.result(top)
    );
    assert!(
        matches!(ev.result(xf), Some(NodeResult::Poisoned { .. })),
        "the operand is poisoned: {:?}",
        ev.result(xf)
    );
    assert!(
        matches!(ev.result(mate.placement), Some(NodeResult::Failed(e)) if matches!(&e.kind, NodeErrorKind::Mate(f) if **f == fault)),
        "the mate carries the same fault: {:?}",
        ev.result(mate.placement)
    );
    // The block is itself a placement of the product, failed in the
    // solve's words, so the gather refuses at THAT placement, before
    // the poisoned pattern copies and before any reference is read.
    let err = gate(&doc, &ev).expect_err("the gather refuses");
    assert!(
        matches!(
            &err,
            AssemblyError::Product(e)
                if matches!(**e, ProductError::Root(NodeStanding::Failed { node }) if node == top)
        ),
        "the gather refuses at the failed root before any reference is read: {err:?}"
    );
}
