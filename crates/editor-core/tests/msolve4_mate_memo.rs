//! **A placement's memo key carries the solve's answer.**
//!
//! A placement's value carries the solve's answer for its bundle: the
//! copy at its solved pose in its space, or a typed refusal when the
//! solve faulted it. The solve runs afresh on every evaluation, so a
//! key of the placement's recipe payload alone would serve last
//! evaluation's answer into this one — an unedited placement reading
//! `Ok` in the very evaluation whose fault reaches it, or carrying a
//! space an edit elsewhere has since changed.
//!
//! Every row here goes through ordinary doors — `DocEdit::AddConstraint`,
//! `DocEdit::RemoveConstraint`, `evaluate` with the previous evaluation
//! as the prior — and reads the answer off the evaluation and off
//! `solve_document`, never off the key function.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions,
    Evaluation, Mate, MateFault, MateFrame, MatePrimitive, MateRef, Node, NodeErrorKind,
    NodeResult, ProfileDoc, RecipeNodeId, SitedFace, Space, evaluate,
};
use fixture::{insert, len, on_frame, solve, step};
use geom_core::Tol;

// ---- substrate ----

/// The extrude in a one-block part document, and the wrapped name of
/// one of its cap faces — the shared assembly substrate
/// (`fixture::resolver`), not a stub authored here.
use fixture::resolver::{PartStore, in_part, with_resolver};

/// A `wxwxh` block, as a whole part document, and its body.
fn slab(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
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

// ---- the scene ----

/// The base slab's height and width, and the blocks' width and height.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 4.0;
const BLOCK_WIDTH: f64 = 1.0;
const BLOCK_HEIGHT: f64 = 2.0;

/// A mate frame ON the base's top cap at `(x, y)`, axis along that
/// cap's OUTWARD normal.
fn base_frame(x: f64, y: f64) -> MateFrame<Formula> {
    MateFrame::authored(
        [x, y, BASE_HEIGHT],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A block's bottom-cap corner, axis along THAT cap's outward normal,
/// which points DOWN in the block's own part coordinates. The two
/// outward normals and `Opposed` are what make this a physical seat:
/// the block stands ON what it is mated to.
fn block_bottom() -> MateFrame<Formula> {
    MateFrame::authored(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A block's TOP-cap corner, axis along that cap's outward normal.
fn block_top() -> MateFrame<Formula> {
    MateFrame::authored(
        [0.0, 0.0, BLOCK_HEIGHT],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A `Rest` mate seating `b`'s frame on `a`'s.
fn seat(
    a: SitedFace,
    b: SitedFace,
    a_frame: MateFrame<Formula>,
    b_frame: MateFrame<Formula>,
    primitive: MatePrimitive,
) -> Mate<Formula> {
    fixture::mate(
        a,
        b,
        ContactClass::Rest,
        Alignment {
            a: a_frame,
            b: b_frame,
            primitive,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    )
}

/// A wide base slab pinned to the world and three blocks with empty
/// bundles, each instantiated through the resolver — the document
/// every row below edits, and the ids it edits by.
///
/// Three blocks, because the per-placement row needs TWO spaces that
/// do not touch: `base` and one block on one side, two blocks stacked
/// on each other on the other. A block no constraint pins is a loose
/// copy of its own and changes no other row's answer.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    top_a: RecipeNodeId,
    top_b: RecipeNodeId,
    top_c: RecipeNodeId,
    base_body: RecipeNodeId,
    block_body: RecipeNodeId,
}

fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (block_ref, block_body) = store.insert_part(
        slab(&format!("{label}-block"), BLOCK_WIDTH, BLOCK_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top_a) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, top_b) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, top_c) = insert(doc, fixture::mated_instance(block_ref));
    Scene {
        doc,
        opts,
        base,
        top_a,
        top_b,
        top_c,
        base_body,
        block_body,
    }
}

impl Scene {
    /// Adds `mate` to the bundle of the copy it places, answering its
    /// address.
    fn add(&mut self, mate: Mate<Formula>) -> MateRef {
        let reach = editor_core::mate_reach::<f64>(&self.opts, Tol::witness());
        let (doc, id) = fixture::constrain_with(self.doc.clone(), &reach, mate);
        self.doc = doc;
        id
    }

    /// A constraint seating `block`'s bottom cap on the base's top cap
    /// at `(x, y)`, under `primitive`, in `block`'s bundle.
    fn seat_on_base(
        &mut self,
        block: RecipeNodeId,
        x: f64,
        y: f64,
        primitive: MatePrimitive,
    ) -> MateRef {
        let mate = seat(
            fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End)),
            fixture::head_at(block, in_part(block, self.block_body, CapEnd::Start)),
            base_frame(x, y),
            block_bottom(),
            primitive,
        );
        self.add(mate)
    }

    /// A constraint seating `upper`'s bottom cap on `lower`'s top cap,
    /// in `upper`'s bundle.
    fn stack(&mut self, lower: RecipeNodeId, upper: RecipeNodeId) -> MateRef {
        let mate = seat(
            fixture::head_at(lower, in_part(lower, self.block_body, CapEnd::End)),
            fixture::head_at(upper, in_part(upper, self.block_body, CapEnd::Start)),
            block_top(),
            block_bottom(),
            MatePrimitive::FrameCoincidence,
        );
        self.add(mate)
    }

    /// Removes `mate` from its placement's bundle.
    fn remove(&mut self, mate: MateRef) {
        let (doc, _) = step(
            self.doc.clone(),
            DocEdit::RemoveConstraint {
                placement: mate.placement,
                constraint: mate.id,
            },
        );
        self.doc = doc;
    }

    /// One evaluation, optionally over a prior — the memo's own door.
    fn eval(&self, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
        evaluate::<f64>(
            &self.doc,
            prior,
            &CancelToken::new(),
            &self.opts,
            Tol::witness(),
        )
    }
}

// ---- readers ----

/// The mate fault a node's row carries, panicking on any other status.
fn row_fault(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> MateFault {
    match ev.result(id) {
        Some(NodeResult::Failed(error)) => match &error.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("{what}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{what}: expected a failed row, got {other:?}"),
    }
}

/// Asserts a node's row is `Ok`.
fn assert_ok(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) {
    assert!(
        matches!(ev.result(id), Some(NodeResult::Ok(_))),
        "{what}: expected an Ok row, got {:?}",
        ev.result(id)
    );
}

/// **Whether `later` SERVED `id` from `earlier`** rather than running
/// its op again — the memo's own witness, per node.
///
/// A memo hit returns the prior `NodeValue` cloned, so its contact
/// records are the same allocation; every op that runs mints a fresh
/// one (the empty records a mate carries are `Arc::new`'d per call, so
/// even they are a new pointer). Not the verdict log: a reused mate
/// carries this run's recording. The document-wide `reused` counter cannot say
/// WHICH node, which is the whole question here.
fn reused_value(earlier: &Evaluation<f64>, later: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    match (earlier.value(id), later.value(id)) {
        (Some(a), Some(b)) => Arc::ptr_eq(&a.contacts, &b.contacts),
        _ => false,
    }
}

/// A node's content key in this evaluation.
fn key(ev: &Evaluation<f64>, id: RecipeNodeId, what: &str) -> editor_core::ContentKey {
    ev.value(id)
        .unwrap_or_else(|| panic!("{what}: no value to key"))
        .content_key
}

// ---- the rows ----

/// **A contradiction added to an already-evaluated bundle faults its
/// placement, and the row carries the solve's own blame.**
///
/// Two coaxials on one bundle about parallel, distinct axes: the
/// first leaves the block a slide (a loose copy, evaluated `Ok`), the
/// second admits at the door — the bundle did not yet pin its copy —
/// and the fold of the two is empty.
#[test]
fn a_contradiction_faults_the_placement_that_evaluated_before_it() {
    let mut s = scene("msolve4-contradiction");
    let held = s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::Coaxial);
    // The first constraint lands and EVALUATES: the memo now holds an
    // `Ok` for the placement.
    let first = s.eval(None);
    assert_ok(&first, s.top_a, "the placement of one coaxial");

    // A second coaxial on the same bundle, a metre away: the fold
    // cannot hold both.
    let added = s.seat_on_base(s.top_a, 2.0, 1.0, MatePrimitive::Coaxial);
    let second = s.eval(Some(&first));

    let poses = solve(&s.doc, &s.opts, Tol::witness());
    let row = row_fault(&second, s.top_a, "the placement");
    assert!(
        matches!(
            row,
            MateFault::Contradictory {
                held: h,
                added: a,
                ..
            } if h == held && a == added
        ),
        "expected the bundle's contradiction naming both constraints, got {row:?}"
    );
    // Blame and row agree — the consistency the whole unit is for.
    assert_eq!(
        Some(&row),
        poses.fault(s.top_a),
        "the solve's blame and the row's own result disagree"
    );
}

/// **A refusal upstream faults the unedited placement pinned to it.**
///
/// A block stacked on a loose block evaluates `Ok` in the lower
/// block's space; a contradiction then lands in the LOWER block's
/// bundle. The upper block's bundle is untouched, and nothing in the
/// fault's words is its own; only the solve's answer for it changed.
#[test]
fn an_upstream_refusal_faults_the_unedited_placement_pinned_to_it() {
    let mut s = scene("msolve4-chain");
    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::Coaxial);
    s.stack(s.top_a, s.top_b);
    let first = s.eval(None);
    assert_ok(&first, s.top_b, "the upper block");
    assert_eq!(
        first.space(s.top_b),
        Space::Loose { root: s.top_a },
        "the premise: the upper block lives in the lower block's space"
    );

    let offender = s.seat_on_base(s.top_a, 3.0, 1.0, MatePrimitive::Coaxial);
    let second = s.eval(Some(&first));

    let offence = row_fault(&second, s.top_a, "the lower block");
    assert!(
        matches!(offence, MateFault::Contradictory { added, .. } if added == offender),
        "the lower block carries the contradiction: {offence:?}"
    );
    // The upper block is not in the fault's words at all, and it is
    // faulted all the same, with the blame the solve recorded.
    let poses = solve(&s.doc, &s.opts, Tol::witness());
    let carried = row_fault(&second, s.top_b, "the upper block");
    assert_eq!(
        Some(&carried),
        poses.fault(s.top_b),
        "the solve's blame and the upper block's row disagree"
    );
    assert!(
        !reused_value(&first, &second, s.top_b),
        "the unedited upper block must not be served from the memo"
    );
}

/// **The reverse direction: removing the contradiction lets the
/// faulted placement evaluate `Ok` again.**
///
/// A faulted node is never memoized — the memo serves `Ok` priors
/// only — so this direction was never broken. It is here because the
/// key carries the fault flag, and a key that moved the wrong way
/// would strand the repair as easily as the break.
#[test]
fn removing_the_contradiction_returns_the_faulted_placement_to_ok() {
    let mut s = scene("msolve4-repair");
    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::Coaxial);
    let added = s.seat_on_base(s.top_a, 2.0, 1.0, MatePrimitive::Coaxial);
    let broken = s.eval(None);
    assert!(
        matches!(broken.result(s.top_a), Some(NodeResult::Failed(_))),
        "the premise: the bundle refuses"
    );

    s.remove(added);
    let repaired = s.eval(Some(&broken));
    assert_ok(
        &repaired,
        s.top_a,
        "the repaired document evaluates the placement on its own solve",
    );
    assert!(
        solve(&s.doc, &s.opts, Tol::witness())
            .fault(s.top_a)
            .is_none(),
        "and the solve records no blame against it"
    );
}

/// **A space change on an unedited placement reaches its value through
/// the memo, and moves its content key.**
///
/// The upper block is stacked on a loose lower block, so it lives in
/// the lower block's space; seating the lower block on the base puts
/// both in the world's. The upper block's bundle never changes.
#[test]
fn a_space_change_on_an_unedited_placement_reaches_its_value() {
    let mut s = scene("msolve4-space");
    s.stack(s.top_a, s.top_b);
    let first = s.eval(None);
    assert_eq!(
        first.space(s.top_b),
        Space::Loose { root: s.top_a },
        "the upper block lives in the loose lower block's space"
    );

    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    let second = s.eval(Some(&first));

    assert_eq!(
        solve(&s.doc, &s.opts, Tol::witness()).space(s.top_b),
        Some(Space::World),
        "the premise: the solve moved the unedited placement into the world"
    );
    assert_eq!(
        second.space(s.top_b),
        Space::World,
        "the unedited placement's evaluation carries the new space, not the memo's old one"
    );
    assert_ne!(
        key(&first, s.top_b, "before"),
        key(&second, s.top_b, "after"),
        "and its content key moved with the answer"
    );
}

/// **A placement whose payload AND whose solve answer are unchanged is
/// still reused** — the key carries an answer, not a nonce.
#[test]
fn an_unchanged_placement_is_reused() {
    let mut s = scene("msolve4-reuse");
    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::FrameCoincidence);
    let first = s.eval(None);
    let second = s.eval(Some(&first));

    assert_eq!(
        key(&first, s.top_a, "the first evaluation"),
        key(&second, s.top_a, "the second"),
        "an unedited placement over an unchanged solve keys identically"
    );
    assert_eq!(
        second.recomputed, 0,
        "nothing re-ran over an unedited document: {:?}",
        second.order
    );
    assert_eq!(
        second.reused,
        second.order.len(),
        "every node came off the memo"
    );
}

/// **A placement's key is per placement, not per document**: when one
/// placement's answer changes, a placement in ANOTHER space —
/// untouched, and with the same answer as before — is still served
/// from the memo.
///
/// The two spaces are disjoint on purpose. A refusal reaches the
/// placements pinned to the refused one and stops there, so the far
/// stack's answer is genuinely unchanged while the near block's is
/// not; a key that carried anything document-global would re-run the
/// far stack too, and no row that only counts recomputations would
/// notice.
#[test]
fn a_placement_in_another_space_is_untouched_by_a_refusal() {
    let mut s = scene("msolve4-per-placement");
    // Space one: two blocks stacked, nowhere near the base.
    s.stack(s.top_b, s.top_c);
    // Space two: a block coaxial on the base.
    s.seat_on_base(s.top_a, 1.0, 1.0, MatePrimitive::Coaxial);
    let first = s.eval(None);
    for (id, what) in [(s.top_c, "the far stack"), (s.top_a, "the near block")] {
        assert_ok(&first, id, what);
    }

    // A contradiction in the near block's bundle only.
    s.seat_on_base(s.top_a, 2.0, 1.0, MatePrimitive::Coaxial);
    let second = s.eval(Some(&first));
    assert!(
        matches!(second.result(s.top_a), Some(NodeResult::Failed(_))),
        "the premise: the near bundle refuses"
    );

    // The far stack: same answer, same key, and SERVED — its value is
    // the prior's value, not an equal one built again.
    assert_ok(&second, s.top_c, "the far stack after the break");
    assert_eq!(
        key(&first, s.top_c, "before"),
        key(&second, s.top_c, "after"),
        "an unchanged answer keys identically"
    );
    for (id, what) in [(s.top_b, "the lower block"), (s.top_c, "the upper block")] {
        assert!(
            reused_value(&first, &second, id),
            "{what} of the untouched space must come off the memo"
        );
    }
    // And the near block did NOT: its answer changed.
    assert!(
        !reused_value(&first, &second, s.top_a),
        "the placement whose answer changed must not be served from the memo"
    );
}
