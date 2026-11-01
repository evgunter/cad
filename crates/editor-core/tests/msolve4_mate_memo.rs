//! **A placement's memo key carries its bundle's answer.**
//!
//! A placement folds its bundle where it evaluates, so its value is the
//! copy at its solved pose in its space, or a typed refusal when the
//! fold refused. Its key is its reads' keys: an edit to the bundle, or
//! to a placement its target reads, moves it; an edit elsewhere does
//! not. A key of the placement's recipe payload alone would serve last
//! evaluation's answer into this one — an unedited placement reading
//! `Ok` in the evaluation whose refusal reaches it, or carrying a space
//! an edit elsewhere has since changed.
//!
//! Every row goes through ordinary doors — `DocEdit::AddConstraint`,
//! `DocEdit::SetConstraints`, `DocEdit::RemoveConstraint`, `evaluate` with the
//! previous evaluation as the prior — and reads the answer off the
//! evaluation and off `solve_document`, never off the key function.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Formula;

use std::sync::Arc;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, CapEnd, Coord, DocEdit, DocumentId, EvalOptions, Evaluation, MateFault, MateRef,
    Node, NodeErrorKind, NodeResult, Operand, ProfileDoc, RecipeNodeId, SitedFace, TargetAxis,
    evaluate,
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
        },
    )
}

// ---- the scene ----

/// The base slab's height and width, and the blocks' width and height.
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 4.0;
const BLOCK_WIDTH: f64 = 1.0;
const BLOCK_HEIGHT: f64 = 2.0;

/// **`face`'s plane slid `h` along its own outward normal and flipped**:
/// a target a face rests against at a standoff of `h`.
fn standoff(face: SitedFace, h: f64) -> Operand {
    let plane = fixture::plane(face);
    fixture::flip(Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by: len(h),
    })))
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
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &opts);
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
    /// The base's top cap.
    fn base_top(&self) -> SitedFace {
        fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End))
    }

    /// A block's cap at `end`.
    fn block_cap(&self, block: RecipeNodeId, end: CapEnd) -> SitedFace {
        fixture::head_at(block, in_part(block, self.block_body, end))
    }

    /// Writes `mate` as `block`'s bundle, each freedom it leaves set
    /// ([`fixture::mated`]), answering the mate's address.
    fn add(
        &mut self,
        block: RecipeNodeId,
        mate: editor_core::Mate<Formula>,
        values: &[(Coord, Formula)],
    ) -> MateRef {
        let (doc, id) = fixture::mated(self.doc.clone(), block, mate, values);
        self.doc = doc;
        id
    }

    /// `block`'s bottom cap resting on the base's top cap `h` above it,
    /// square to the base: a plane mate and the constant `0` on each
    /// freedom it leaves.
    fn rest_on_base(&mut self, block: RecipeNodeId, h: f64) -> MateRef {
        let mate = fixture::mate(
            fixture::plane(self.block_cap(block, CapEnd::Start)),
            standoff(self.base_top(), h),
        );
        self.add(block, mate, &[])
    }

    /// `block` seated on the base's top cap with its origin over
    /// `(x, y)` and its axes the base's: the rest and its three values,
    /// which pin it.
    fn seat_on_base(&mut self, block: RecipeNodeId, x: f64, y: f64) -> MateRef {
        let mate = fixture::mate(
            fixture::plane(self.block_cap(block, CapEnd::Start)),
            standoff(self.base_top(), 0.0),
        );
        self.add(
            block,
            mate,
            &[
                (Coord::Slide(TargetAxis::X), len(x)),
                (Coord::Slide(TargetAxis::Y), len(y)),
            ],
        )
    }

    /// `upper`'s bottom cap seated on `lower`'s top cap, flush, in
    /// `upper`'s bundle: pinned in `lower`'s space.
    fn stack(&mut self, lower: RecipeNodeId, upper: RecipeNodeId) -> MateRef {
        let rest = fixture::rest(
            self.block_cap(upper, CapEnd::Start),
            self.block_cap(lower, CapEnd::End),
        );
        self.add(upper, rest, &[])
    }

    /// Adds `mate` to `block`'s bundle past the bundle rule — the state
    /// a doctored save leaves, which the fold refuses — answering its
    /// address.
    fn add_past_the_door(
        &mut self,
        block: RecipeNodeId,
        mate: editor_core::Mate<Formula>,
    ) -> MateRef {
        let (doc, id) = editor_core::test_support::beside_past_the_bundle_rule(
            &self.doc,
            block,
            mate,
            Tol::witness(),
        );
        self.doc = doc;
        MateRef::new(block, id)
    }

    /// A second rest of `block` on the base `h` above it, past the
    /// door: a plane mate beside the rest the bundle holds.
    fn second_rest(&mut self, block: RecipeNodeId, h: f64) -> MateRef {
        let mate = fixture::mate(
            fixture::plane(self.block_cap(block, CapEnd::Start)),
            standoff(self.base_top(), h),
        );
        self.add_past_the_door(block, mate)
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
/// one. The document-wide `reused` counter cannot say WHICH node, which
/// is the whole question here.
fn reused_value(earlier: &Evaluation<f64>, later: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    match (earlier.value(id), later.value(id)) {
        (Some(a), Some(b)) => Arc::ptr_eq(&a.carry.contacts, &b.carry.contacts),
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

/// **An overconstraint landing in an already-evaluated bundle faults
/// its placement, and the row carries the fold's own blame.**
///
/// Two rests on one bundle: the first leaves the block free on the cap
/// (a loose copy, evaluated `Ok`); the second, beside it past the door
/// (the state a doctored save leaves), fixes the normal the first
/// already fixes, and the fold refuses the bundle naming both.
///
/// *Breaks if* the placement's key omits its bundle, so the second
/// evaluation serves the first's `Ok`, or the row's fault and the
/// solve's blame disagree.
#[test]
fn an_overconstraint_faults_the_placement_that_evaluated_before_it() {
    let mut s = scene("msolve4-overconstraint");
    let held = s.rest_on_base(s.top_a, 0.0);
    // The first constraint lands and EVALUATES: the memo now holds an
    // `Ok` for the placement.
    let first = s.eval(None);
    assert_ok(&first, s.top_a, "the placement of one rest");

    // A second rest on the same bundle, a metre up.
    let added = s.second_rest(s.top_a, 1.0);
    let second = s.eval(Some(&first));

    let poses = solve(&s.doc, &s.opts, Tol::witness());
    let row = row_fault(&second, s.top_a, "the placement");
    assert!(
        matches!(
            &row,
            MateFault::Overconstrained { placement, constraints, .. }
                if *placement == s.top_a && *constraints == vec![held.id, added.id]
        ),
        "expected the bundle's overconstraint naming both constraints, got {row:?}"
    );
    // Blame and row agree — the consistency the whole unit is for.
    assert_eq!(
        Some(&row),
        poses.fault(s.top_a),
        "the solve's blame and the row's own result disagree"
    );
}

/// **A refusal upstream poisons the unedited placement pinned to it.**
///
/// A block stacked on a loose block evaluates `Ok` in the lower
/// block's space; an overconstraint then lands in the LOWER block's
/// bundle. The upper block's bundle is untouched, and its target reads
/// the refused copy: it is poisoned through it, and not served from
/// the memo.
///
/// *Breaks if* the upper block's key omits its target's, so the memo
/// serves its `Ok` over a refused lower block.
#[test]
fn an_upstream_refusal_poisons_the_unedited_placement_pinned_to_it() {
    let mut s = scene("msolve4-chain");
    s.rest_on_base(s.top_a, 0.0);
    s.stack(s.top_a, s.top_b);
    let first = s.eval(None);
    assert_ok(&first, s.top_b, "the upper block");
    assert_eq!(
        s.doc.node_space(s.top_b),
        s.doc.node_space(s.top_a),
        "the premise: the upper block lives in the lower block's space"
    );

    let offender = s.second_rest(s.top_a, 1.0);
    let second = s.eval(Some(&first));

    let offence = row_fault(&second, s.top_a, "the lower block");
    assert!(
        matches!(&offence, MateFault::Overconstrained { constraints, .. } if constraints.contains(&offender.id)),
        "the lower block carries the overconstraint: {offence:?}"
    );
    assert!(
        matches!(
            second.result(s.top_b),
            Some(NodeResult::Poisoned { through }) if *through == s.top_a
        ),
        "the upper block is poisoned through the refused lower block: {:?}",
        second.result(s.top_b)
    );
    let poses = solve(&s.doc, &s.opts, Tol::witness());
    assert_eq!(
        (poses.fault(s.top_b), poses.space(s.top_b)),
        (None, None),
        "the upper block's own bundle refused nothing, and it has no answer"
    );
    assert!(
        !reused_value(&first, &second, s.top_b),
        "the unedited upper block must not be served from the memo"
    );
}

/// **The reverse direction: removing the overconstraint lets the
/// faulted placement evaluate `Ok` again.**
///
/// A faulted node is never memoized — the memo serves `Ok` priors
/// only — so this direction does not lean on the key; it is here so a
/// key that moved the wrong way strands the repair as visibly as the
/// break.
///
/// *Breaks if* the repaired placement still reads the refusal.
#[test]
fn removing_the_overconstraint_returns_the_faulted_placement_to_ok() {
    let mut s = scene("msolve4-repair");
    s.rest_on_base(s.top_a, 0.0);
    let added = s.second_rest(s.top_a, 1.0);
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
        "the repaired document evaluates the placement on its own fold",
    );
    assert!(
        solve(&s.doc, &s.opts, Tol::witness())
            .fault(s.top_a)
            .is_none(),
        "and the solve records no blame against it"
    );
}

/// **A move of the body an unedited placement is mated to reaches its
/// value through the memo, and moves its content key.**
///
/// The upper block is stacked on the lower block, standing where its
/// body stands; seating the lower block on the base moves it, and the
/// upper block with it. The upper block's bundle never changes.
///
/// *Breaks if* the upper block is served from the memo where it stood.
#[test]
fn a_moved_target_reaches_an_unedited_placements_value() {
    let mut s = scene("msolve4-space");
    s.stack(s.top_a, s.top_b);
    let first = s.eval(None);
    let before = solve(&s.doc, &s.opts, Tol::witness())
        .relative(s.top_b)
        .expect("the upper block is placed");

    s.seat_on_base(s.top_a, 1.0, 1.0);
    let second = s.eval(Some(&first));

    let after = solve(&s.doc, &s.opts, Tol::witness())
        .relative(s.top_b)
        .expect("the upper block is placed");
    assert_ne!(
        before.translation, after.translation,
        "the premise: the unedited placement moved with its target"
    );
    assert!(
        !reused_value(&first, &second, s.top_b),
        "the unedited placement is not served from the memo"
    );
    assert_ne!(
        key(&first, s.top_b, "before"),
        key(&second, s.top_b, "after"),
        "and its content key moved with the answer"
    );
}

/// **A placement whose payload AND whose reads are unchanged is still
/// reused** — the key carries an answer, not a nonce.
///
/// *Breaks if* a placement's key carries anything that differs between
/// two evaluations of one document.
#[test]
fn an_unchanged_placement_is_reused() {
    let mut s = scene("msolve4-reuse");
    s.seat_on_base(s.top_a, 1.0, 1.0);
    let first = s.eval(None);
    let second = s.eval(Some(&first));

    assert_eq!(
        key(&first, s.top_a, "the first evaluation"),
        key(&second, s.top_a, "the second"),
        "an unedited placement over an unchanged document keys identically"
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
///
/// *Breaks if* a refusal in one space re-runs a placement of another.
#[test]
fn a_placement_in_another_space_is_untouched_by_a_refusal() {
    let mut s = scene("msolve4-per-placement");
    // Space one: two blocks stacked, nowhere near the base.
    s.stack(s.top_b, s.top_c);
    // Space two: a block resting on the base.
    s.rest_on_base(s.top_a, 0.0);
    let first = s.eval(None);
    for (id, what) in [(s.top_c, "the far stack"), (s.top_a, "the near block")] {
        assert_ok(&first, id, what);
    }

    // An overconstraint in the near block's bundle only.
    s.second_rest(s.top_a, 1.0);
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
