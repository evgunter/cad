//! **A placement's fold decides on that placement's log.**
//!
//! A placement folds its bundle where it evaluates, so every decision
//! the fold makes — a table split adding one mate to what the rest
//! left, a membership check, a value's chart — is that placement's,
//! recorded on its own row, and on no other's. What the copies its
//! mates read decide about themselves is theirs. That nothing is
//! recorded outside a node is checked for every mate document a suite
//! evaluates through `fixture::run`; the rows here pin WHICH node, the
//! memo, and the lever and member-index rows beside them.
//!
//! Every row goes through ordinary doors — `DocEdit::InsertNode`,
//! `DocEdit::SetConstraints`, `evaluate` with or
//! without a prior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, CapEnd, Coord, DocEdit, DocumentId, EvalOptions, Evaluation, Formula, Mate,
    MateRef, Node, NodeErrorKind, NodeResult, Operand, ProfileDoc, RecipeNodeId, RoleSeg,
    SitedFace, SitedRef, StableName, TargetAxis, VarId, evaluate,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, insert, len, on_frame};
use geom_core::Tol;

// ---- the scene ----

const BASE_WIDTH: f64 = 9.0;
const BASE_HEIGHT: f64 = 1.0;
const BLOCK_WIDTH: f64 = 1.0;
const BLOCK_HEIGHT: f64 = 2.0;

/// A `w x w x h` block, as a whole part document, and its body.
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

/// **`face`'s plane slid `h` along its own outward normal and
/// flipped**: a target a face rests against at a standoff of `h`.
fn standoff(face: SitedFace, h: f64) -> Operand {
    let plane = fixture::plane(face);
    fixture::flip(Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by: len(h),
    })))
}

/// A base slab and a block, each pinned to the world, a row of the
/// block in the world, and two blocks with empty bundles for the
/// constraints to place: `cap`, seated on the row's member 1, and
/// `other`, seated on the base.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    base_body: RecipeNodeId,
    block: RecipeNodeId,
    other: RecipeNodeId,
    cap: RecipeNodeId,
    block_body: RecipeNodeId,
    /// The block part's lateral edge at its first profile vertex, in
    /// the part's own spelling.
    edge: StableName,
    pattern: RecipeNodeId,
    /// The row's index.
    k: VarId,
    /// The row's member 1, copied out of its family by a read of
    /// `pattern[1]`: a pose is read off one body, and a family's value
    /// is many.
    copy1: RecipeNodeId,
}

fn scene(label: &str) -> Scene {
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (block_doc, block_part_body) = slab(&format!("{label}-block"), BLOCK_WIDTH, BLOCK_HEIGHT);
    let edge = fixture::ename(
        block_part_body,
        RoleSeg::LateralEdge(fixture::vpiece(&block_doc, block_part_body, 0, 0)),
    );
    let (block_ref, block_body) = store.insert_part((block_doc, block_part_body), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &opts);
    let (doc, block) =
        fixture::insert_pinned(doc, crate::fixture::mated_instance(block_ref), &opts);
    let (doc, other) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, cap) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, k, pattern) = crate::fixture::linear(doc, block, [1.0, 0.0, 0.0], 3.0, 2);
    let (doc, copy1) = fixture::member(doc, pattern, 1);
    Scene {
        doc,
        opts,
        base,
        base_body,
        block,
        other,
        cap,
        block_body,
        edge,
        pattern,
        k,
        copy1,
    }
}

impl Scene {
    /// The base's top cap, read at the base.
    fn base_top(&self) -> SitedFace {
        fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End))
    }

    /// Member 1's lateral edge, as an axis read at the copy of it.
    fn copy_edge(&self) -> Operand {
        fixture::axis(SitedRef::new(
            self.copy1,
            in_copy(self.pattern, self.k, 1, self.edge.clone()),
        ))
    }

    /// A block's bottom cap, read at the block.
    fn bottom(&self, block: RecipeNodeId) -> SitedFace {
        fixture::head_at(block, in_part(block, self.block_body, CapEnd::Start))
    }

    /// A block's lateral edge, as an axis read at the block.
    fn block_edge(&self, block: RecipeNodeId) -> Operand {
        fixture::axis(SitedRef::new(block, self.edge.clone()))
    }

    /// Inserts `node` through the door.
    fn insert(&mut self, node: editor_core::AuthoredNode) -> RecipeNodeId {
        let (doc, id) = insert(self.doc.clone(), node);
        self.doc = doc;
        id
    }

    /// Writes `mate` as `block`'s bundle through the door, each freedom
    /// it leaves set to 0.
    fn add(&mut self, block: RecipeNodeId, mate: Mate<Formula>) -> MateRef {
        self.add_with(block, mate, &[])
    }

    /// Writes `mate` as `block`'s bundle through the door, the freedoms
    /// `values` names set by them and the rest to 0.
    fn add_with(
        &mut self,
        block: RecipeNodeId,
        mate: Mate<Formula>,
        values: &[(Coord, Formula)],
    ) -> MateRef {
        let (doc, id) = fixture::mated(self.doc.clone(), block, mate, values);
        self.doc = doc;
        id
    }

    /// The cap seated on copy 1: its lateral edge on the copy's (one
    /// coaxial mate), slid `BLOCK_HEIGHT` along it so its bottom cap
    /// meets the copy's top cap, and its spin set — pinned.
    fn seat_cap(&mut self) {
        self.add_with(
            self.cap,
            fixture::mate(self.block_edge(self.cap), self.copy_edge()),
            &[
                (Coord::Slide(TargetAxis::Z), len(BLOCK_HEIGHT)),
                (Coord::SPIN, Formula::zero(Dimension::Angle)),
            ],
        );
    }

    /// The other block seated on the base with its origin over
    /// `(x, y)`: one rest and its three values.
    fn seat_other(&mut self, x: f64, y: f64) -> MateRef {
        self.add_with(
            self.other,
            fixture::rest(self.bottom(self.other), self.base_top()),
            &[
                (Coord::Slide(TargetAxis::X), len(x)),
                (Coord::Slide(TargetAxis::Y), len(y)),
                (Coord::SPIN, Formula::zero(Dimension::Angle)),
            ],
        )
    }

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

/// The verdict log a node's row carries.
fn log(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<geom_core::k_stats::Verdict> {
    match ev.result(node) {
        Some(NodeResult::Ok(v)) => v.verdicts.to_vec(),
        other => panic!("node {} did not evaluate: {other:?}", node.0),
    }
}

/// The mate fault a placement's row carries, panicking on any other.
fn mate_fault(ev: &Evaluation<f64>, placement: RecipeNodeId) -> editor_core::MateFault {
    match ev.result(placement) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the placement to fail, got {other:?}"),
    }
}

// ---- one home per decision ----

/// **Each decision is on the node that made it.** Two placements: the
/// cap, held by a coaxial mate against member 1 of a row (read at a
/// copy of `row[1]`) and its slide and spin, and the other block, held
/// by a rest and three values against the base.
///
/// - Each placement's chart (`mate_chart_pole`) is on its
///   own log.
/// - Neither placement's slides decide anything: each moves along its
///   chart's own axis (`value::settle`), so no line between two slides
///   is minted (`mate_axes_parallel`) on either log.
///
/// *Breaks if* a placement's decision lands on another node's row, or a
/// placement re-decides what the copy it reads decided.
#[test]
fn each_decision_is_on_the_log_of_the_node_that_made_it() {
    let mut s = scene("msolve11-home");
    s.seat_cap();
    s.seat_other(6.0, 6.0);
    let ev = fixture::run(&s.doc, &s.opts);
    let names =
        |node| -> Vec<&'static str> { log(&ev, node).iter().map(|v| v.predicate).collect() };
    let (cap_log, other_log) = (names(s.cap), names(s.other));
    let on = |got: &[&str], p: &str| got.contains(&p);
    for (what, got) in [("cap", &cap_log), ("other block", &other_log)] {
        assert!(
            on(got, "mate_chart_pole"),
            "the {what} charts its values on its own log: {got:?}"
        );
    }
    assert!(
        !on(&other_log, "mate_axes_parallel") && !on(&cap_log, "mate_axes_parallel"),
        "no slide mints a line: {other_log:?}, cap {cap_log:?}"
    );
}

// ---- the memo ----

/// **Whether `later` served `id` from `earlier`** rather than running
/// its op again: a memo hit returns the prior value's contact records,
/// the same allocation, and every op that runs mints a fresh one.
fn reused_value(earlier: &Evaluation<f64>, later: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    match (earlier.value(id), later.value(id)) {
        (Some(a), Some(b)) => std::sync::Arc::ptr_eq(&a.carry.contacts, &b.carry.contacts),
        _ => false,
    }
}

/// **A reused placement carries the log a fresh evaluation of it
/// carries.** The cap's bundle is a coaxial mate against a family's
/// member and its slide and spin values, so its log holds the values'
/// charts. A constraint added to ANOTHER placement leaves the
/// cap's reads and key as they were, so the memo serves it, and the
/// served row carries the decisions its fold recorded — the ones a
/// fresh evaluation of it records.
///
/// *Breaks if* a served placement drops or doubles its fold's log.
#[test]
fn a_reused_placement_carries_the_log_a_fresh_one_does() {
    let mut s = scene("msolve11-memo");
    s.seat_cap();
    let first = s.eval(None);
    s.seat_other(6.0, 6.0);
    let later = s.eval(Some(&first));
    let fresh = s.eval(None);
    assert!(
        reused_value(&first, &later, s.cap),
        "the cap's reads and key did not move, so the memo serves it"
    );
    let fresh_log = log(&fresh, s.cap);
    assert!(
        fresh_log.iter().any(|v| v.predicate == "mate_chart_pole"),
        "the fold charts the cap's values: {fresh_log:?}"
    );
    assert_eq!(
        log(&later, s.cap),
        fresh_log,
        "the served cap carries the decisions a fresh fold records"
    );
}

// ---- the lever ----

/// The `Unleverable` refusal a placement carries, panicking on any other.
fn unleverable(fault: &editor_core::MateFault) -> &editor_core::LeverRefusal {
    match fault {
        editor_core::MateFault::Unleverable { refusal, .. } => refusal,
        other => panic!("expected an unleverable mate, got {other:?}"),
    }
}

/// The other block's bottom cap on a floor's top `z = h` up, facing
/// down ([`fixture::floor`]): a plane mate whose target stands in the
/// world.
fn on_the_world(s: &mut Scene, h: f64) -> Mate<Formula> {
    let (doc, top, _) = fixture::floor(
        s.doc.clone(),
        vec![(Coord::Slide(editor_core::TargetAxis::Z), len(h))],
    );
    s.doc = doc;
    Mate {
        on: fixture::plane(s.bottom(s.other)),
        to: fixture::flip(top),
        orient: editor_core::Orientation::IDENTITY,
    }
}

/// **A lever out of the format's range refuses at its one door,
/// through the fold.** A rest against the base at a standoff of
/// `1e154` m is a finite datum, and so is its distance from the
/// target's origin; the lever the bundle forms over it — the copy's
/// reach plus that distance — has a sixteenfold square that overflows.
/// The fold forms the lever through `Arm::of`, which refuses it typed —
/// against the bundle's first mate, carrying both halves and the range
/// recourse — so no predicate is handed it, and nothing escalates.
///
/// *Breaks if* an angle is decided over a lever the format cannot
/// square, or the refusal drops either half.
#[test]
fn a_lever_out_of_range_refuses_typed_through_the_fold() {
    let mut s = scene("msolve11-range-solve");
    let rest = s.add(
        s.other,
        fixture::mate(
            fixture::plane(s.bottom(s.other)),
            standoff(s.base_top(), 1e154),
        ),
    );
    let ev = fixture::run(&s.doc, &s.opts);
    let fault = mate_fault(&ev, s.other);
    assert!(
        matches!(fault, editor_core::MateFault::Unleverable { mate, .. } if mate == rest),
        "the bundle's lever is named against its first mate: {fault:?}"
    );
    let editor_core::LeverRefusal::OutOfRange { parts, datum } = *unleverable(&fault) else {
        panic!("expected the range refusal, got {fault:?}");
    };
    assert!(
        parts.is_finite() && parts > 0.0,
        "the copy's reach is in hand: {parts}"
    );
    assert!(
        datum.is_finite() && datum >= 1e154,
        "the poses' own terms carry the standoff: {datum}"
    );
    let said = fault.to_string();
    assert!(said.contains(geom_core::RANGE_RECOURSE), "{said}");
    let escalations = match ev.result(s.other) {
        Some(NodeResult::Ok(v)) => v.escalations.to_vec(),
        Some(NodeResult::Failed(e)) => e.escalations.to_vec(),
        _ => Vec::new(),
    };
    assert!(
        escalations.is_empty(),
        "no predicate decided over the lever: {escalations:?}"
    );
}

/// **A distance whose own square overflows is admitted at the door
/// and refused where the placement evaluates.** The door decides
/// nothing that needs a pose, so a rest at a standoff of `1e200` m
/// enters; the fold measures the target's distance from its origin,
/// whose square overflows, and refuses the lever typed with that half
/// not finite.
///
/// *Breaks if* the door decides a lever, or the fold carries a
/// non-finite distance past `Arm::of`.
#[test]
fn a_lever_whose_distance_overflows_is_admitted_and_refused_at_evaluation() {
    let mut s = scene("msolve11-range-door");
    let placed = fixture::squared(
        &s.doc,
        fixture::mate(
            fixture::plane(s.bottom(s.other)),
            standoff(s.base_top(), 1e200),
        ),
        &[],
    );
    let applied = editor_core::apply(&s.doc, &placed.write(s.other), Tol::witness())
        .unwrap_or_else(|e| panic!("the door admits the rest: {e}"));
    let rest = MateRef::new(s.other, applied.record.minted_constraints[0]);
    s.doc = applied.doc;
    let fault = mate_fault(&fixture::run(&s.doc, &s.opts), s.other);
    assert!(
        matches!(fault, editor_core::MateFault::Unleverable { mate, .. } if mate == rest),
        "{fault:?}"
    );
    assert!(
        matches!(
            unleverable(&fault),
            editor_core::LeverRefusal::OutOfRange { datum, .. } if !datum.is_finite()
        ),
        "{fault:?}"
    );
    assert!(
        fault.to_string().contains(geom_core::RANGE_RECOURSE),
        "{fault}"
    );
}

// ---- a member read's own index ----

/// The cap resting on the member a read of the row carries, read at
/// that copy under member 1's name.
fn rest_on_part(s: &mut Scene, part: RecipeNodeId) -> MateRef {
    let read_at_part = fixture::head_at(
        part,
        in_copy(
            s.pattern,
            s.k,
            1,
            in_part(s.block, s.block_body, CapEnd::End),
        ),
    );
    s.add(s.cap, fixture::rest(s.bottom(s.cap), read_at_part))
}

/// **A member read's index that does not evaluate is refused at the
/// read**, the node the evaluation fails, and not at the healthy family
/// below it; the cap whose mate reads the copy is poisoned through it.
/// The index `which · i64::MAX + 1` reads member 1 at `which = 0`, where
/// the cap rests on it, and overflows the count's range at `which = 1`.
///
/// *Breaks if* the refusal lands on the family, or the cap evaluates
/// over a member that no longer exists.
#[test]
fn a_member_reads_index_that_does_not_evaluate_is_refused_at_the_read() {
    use editor_core::{Dimension, FreeValue, FreeVar, VarName};
    let mut s = scene("msolve11-part-index");
    let which = VarName::from_static("which");
    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::DeclareVar {
            name: which.clone(),
            def: editor_core::VarDecl::Free(FreeVar::Count { value: 0 }),
        },
    );
    s.doc = doc;
    let index = Formula::add(
        Formula::mul(
            Formula::named(which.clone(), Dimension::Count),
            Formula::count(i64::MAX),
        )
        .unwrap(),
        Formula::count(1),
    )
    .unwrap();
    let node = fixture::member_node(&s.doc, s.pattern, vec![index]);
    let part = s.insert(node);
    rest_on_part(&mut s, part);
    let ev = fixture::run(&s.doc, &s.opts);
    assert!(
        matches!(ev.result(s.cap), Some(NodeResult::Ok(_))),
        "the index reads the named member at which = 0: {:?}",
        ev.result(s.cap)
    );

    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::SetVarValue {
            var: which.into(),
            value: FreeValue::Count(1),
        },
    );
    s.doc = doc;
    let ev = fixture::run(&s.doc, &s.opts);
    assert!(
        matches!(
            ev.result(part),
            Some(NodeResult::Failed(e)) if matches!(
                e.kind,
                NodeErrorKind::Expr {
                    slot: editor_core::SlotId::Index {
                        seat: editor_core::OperandSlot::Source,
                        k: 0
                    },
                    ..
                }
            )
        ),
        "the read fails on its own index: {:?}",
        ev.result(part)
    );
    assert!(
        matches!(ev.result(s.pattern), Some(NodeResult::Ok(_))),
        "the family below it is healthy: {:?}",
        ev.result(s.pattern)
    );
    assert!(
        matches!(
            ev.result(s.cap),
            Some(NodeResult::Poisoned { through }) if *through == part
        ),
        "the cap is poisoned through the read: {:?}",
        ev.result(s.cap)
    );
}

/// **A member read's index outside the family is refused as the
/// evaluation refuses it, at the read.** The index 5 over a two-member
/// row reads no member: the evaluation fails the read with
/// `MemberOutOfRange`, and the cap whose mate reads it is poisoned
/// through it rather than refused in words that name a member that does
/// not exist.
///
/// *Breaks if* the cap carries a refusal of its own for the read's.
#[test]
fn a_member_reads_index_outside_its_family_is_refused_at_the_read() {
    let mut s = scene("msolve11-part-out-of-range");
    let node = fixture::member_node(&s.doc, s.pattern, vec![Formula::count(5)]);
    let part = s.insert(node);
    rest_on_part(&mut s, part);
    let ev = fixture::run(&s.doc, &s.opts);
    let Some(NodeResult::Failed(failed)) = ev.result(part) else {
        panic!("the evaluation fails the read: {:?}", ev.result(part));
    };
    assert!(
        matches!(
            &failed.kind,
            NodeErrorKind::MemberOutOfRange { input, at }
                if *input == s.pattern && at.as_slice() == [5]
        ),
        "{:?}",
        failed.kind
    );
    assert!(
        matches!(
            ev.result(s.cap),
            Some(NodeResult::Poisoned { through }) if *through == part
        ),
        "the cap is poisoned through the read: {:?}",
        ev.result(s.cap)
    );
}

// ---- the box driver ----

/// **What a box run meets on a document whose bundle is
/// overconstrained, as the driver stands today.** The bundle's refusal
/// is box-independent (FORK-S3O: decided by its families, measuring
/// nothing), so `drive::classify_replay` would meet it at every leaf.
/// The driver never gets there: its evaluations carry no resolver
/// (`drive::lane_opts`), so at the nominal witness every instance's
/// part is out of hand, the instances refuse before any bundle is
/// folded, and the run refuses up front with `WitnessDoesNotBuild`. The
/// same document under a resolver refuses `Overconstrained`, with no
/// escalation on its log.
///
/// The bundle rests the other block on the base and on a plane of the
/// world, the second beside the first past the door (the state a
/// doctored save leaves). Measured for
/// `work/msolve/a-box-independent-mate-fault-bisects-the-whole-leaf-budget.md`.
///
/// *Breaks if* the refusal escalates (it measured something), or the
/// driver's witness builds without a resolver.
#[test]
fn a_box_run_over_an_overconstrained_bundle_refuses_at_its_witness() {
    use editor_core::analysis::{AnalysisPolicy, analyzed_box};
    use editor_core::drive::{DriveConfig, DriveRefusal};
    use editor_core::range::{RangeField, RangeSeed, derive};
    use editor_core::{Dimension, FreeVar, VarName};
    let mut s = scene("msolve11-drive");
    s.add(s.other, fixture::rest(s.bottom(s.other), s.base_top()));
    let beside = on_the_world(&mut s, BASE_HEIGHT);
    let (doc, _) = editor_core::test_support::beside_past_the_bundle_rule(
        &s.doc,
        s.other,
        beside,
        Tol::witness(),
    );
    s.doc = doc;
    let w = VarName::from_static("w");
    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::DeclareVar {
            name: w.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
        },
    );
    let derived = derive(
        &doc,
        &RangeField::Param(doc.var_named(w.as_str()).expect("declared")),
        RangeSeed::symmetric(0.1),
        Tol::witness(),
    )
    .expect("the parameter boxes");
    let ev = fixture::run(&derived.doc, &s.opts);
    let Some(NodeResult::Failed(added)) = ev.result(s.other) else {
        panic!("the placement refuses: {:?}", ev.result(s.other));
    };
    assert!(
        matches!(
            &added.kind,
            NodeErrorKind::Mate(fault)
                if matches!(**fault, editor_core::MateFault::Overconstrained { .. })
        ) && added.escalations.is_empty(),
        "under a resolver the bundle refuses, measuring nothing: {:?}",
        added.kind
    );
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_depth: 3,
        max_leaves: 8,
        ..DriveConfig::default()
    };
    let refusal = editor_core::drive::drive(&derived.doc, &analyzed, &config, Tol::witness())
        .expect_err("the witness does not build");
    let DriveRefusal::WitnessDoesNotBuild { cause, .. } = &refusal else {
        panic!("expected WitnessDoesNotBuild, got {refusal:?}");
    };
    assert!(
        cause.contains("no part resolver"),
        "the driver's evaluation has no resolver, so no instance resolves: {cause}"
    );
}
