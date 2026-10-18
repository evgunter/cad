//! **A placement's fold decides on that placement's log.**
//!
//! A placement folds its bundle where it evaluates, so every decision
//! the fold makes — a table split adding one mate to what the rest
//! left, a membership check, a value's chart — is that placement's,
//! recorded on its own row, and on no other's. What the copies its
//! mates read decide about themselves is theirs. That nothing is
//! recorded outside a node is checked for every mate document a suite
//! evaluates through `fixture::run`; the rows here pin WHICH node, the
//! memo, and the lever and `Part`-index rows beside them.
//!
//! Every row goes through ordinary doors — `DocEdit::InsertNode`,
//! `DocEdit::AddConstraint`, `DocEdit::AddValue`, `evaluate` with or
//! without a prior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CancelToken, CapEnd, ContactClass, Coord, DocEdit, DocumentId, EvalOptions, Evaluation,
    Formula, Mate, MateRef, NameRef, Node, NodeErrorKind, NodeResult, Operand, PatternKind,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedFace, SitedRef, SlideAxis, StableName, Target, Value,
    WorldRead, evaluate,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::{ang, in_copy, insert, len, on_frame, scl};
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
            side: ExtrudeSide::Along,
        },
    )
}

/// **`name`, an entity of a part's body, as `instance` wears it**:
/// under the placement the part's world copies the body by, inside the
/// instance's `InPart` wrapper.
fn worn(instance: RecipeNodeId, body: RecipeNodeId, name: StableName) -> StableName {
    let placed = StableName {
        kind: name.kind,
        node: placement_of(body),
        path: vec![RoleSeg::Placed {
            of: NameRef::new(name),
        }],
    };
    StableName {
        kind: placed.kind,
        node: instance,
        path: vec![RoleSeg::InPart { of: placed.into() }],
    }
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

/// A base slab and a block, each pinned to the world, a linear pattern
/// of the block, and two blocks with empty bundles for the constraints
/// to place: `cap`, seated on the pattern's copy 1, and `other`, seated
/// on the base.
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
    /// The pattern's copy 1, projected out of its instances by a
    /// `Part`: a pose is read off one body, and a pattern's value is
    /// many.
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
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(base_ref), &opts);
    let (doc, block) = fixture::insert_pinned(doc, Node::instantiate_part(block_ref), &opts);
    let (doc, other) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, cap) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: block.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, copy1) = insert(
        doc,
        Node::Part {
            of: pattern.into(),
            select: editor_core::PartSelect::Instance(Formula::count(1)),
        },
    );
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
        copy1,
    }
}

impl Scene {
    /// The base's top cap, read at the base.
    fn base_top(&self) -> SitedFace {
        fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End))
    }

    /// Copy 1's lateral edge, as an axis read at the `Part` that
    /// projects it.
    fn copy_edge(&self) -> Operand {
        fixture::axis(SitedRef::new(
            self.copy1,
            in_copy(
                self.pattern,
                1,
                worn(self.block, self.block_body, self.edge.clone()),
            ),
        ))
    }

    /// A block's bottom cap, read at the block.
    fn bottom(&self, block: RecipeNodeId) -> SitedFace {
        fixture::head_at(block, in_part(block, self.block_body, CapEnd::Start))
    }

    /// A block's lateral edge, as an axis read at the block.
    fn block_edge(&self, block: RecipeNodeId) -> Operand {
        fixture::axis(SitedRef::new(
            block,
            worn(block, self.block_body, self.edge.clone()),
        ))
    }

    /// Inserts `node` through the door.
    fn insert(&mut self, node: editor_core::AuthoredNode) -> RecipeNodeId {
        let (doc, id) = insert(self.doc.clone(), node);
        self.doc = doc;
        id
    }

    /// Adds `mate` to `block`'s bundle through the door.
    fn add(&mut self, block: RecipeNodeId, mate: Mate<Formula>) -> MateRef {
        let (doc, id) = fixture::constrain_at(self.doc.clone(), block, mate);
        self.doc = doc;
        id
    }

    /// Sets the freedom `coord` of `mate` to `by`.
    fn value(&mut self, mate: MateRef, coord: Coord, by: Formula) {
        self.doc = editor_core::apply(
            &self.doc,
            &DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .unwrap_or_else(|e| panic!("the value is admitted: {e}"))
        .doc;
    }

    /// The cap seated on copy 1: its lateral edge on the copy's (one
    /// coaxial mate), slid `BLOCK_HEIGHT` along it so its bottom cap
    /// meets the copy's top cap, and its spin set — pinned.
    fn seat_cap(&mut self) {
        let coaxial = self.add(
            self.cap,
            fixture::mate(
                self.block_edge(self.cap),
                self.copy_edge(),
                ContactClass::Rest,
            ),
        );
        self.value(coaxial, Coord::Slide(SlideAxis::Z), len(BLOCK_HEIGHT));
        self.value(coaxial, Coord::Spin, ang(0.0));
    }

    /// The other block seated on the base with its origin over
    /// `(x, y)`: one rest and its three values.
    fn seat_other(&mut self, x: f64, y: f64) -> MateRef {
        let rest = self.add(
            self.other,
            fixture::rest(self.bottom(self.other), self.base_top(), ContactClass::Rest),
        );
        self.value(rest, Coord::Slide(SlideAxis::X), len(x));
        self.value(rest, Coord::Slide(SlideAxis::Y), len(y));
        self.value(rest, Coord::Spin, ang(0.0));
        rest
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

/// The decisions a pattern makes about its own rule.
const PATTERN_RULE: [&str; 2] = ["eval_direction_norm", "pattern_spacing"];

/// **Each decision is on the node that made it.** Two placements: the
/// cap, held by a coaxial mate against copy 1 of a pattern (read at a
/// `Part` that projects it) and its slide and spin, and the other
/// block, held by a rest and three values against the base.
///
/// - Each placement's chart (`mate_chart_reference_antipode`) is on its
///   own log.
/// - The other block's two slides run in its rest's plane, and the line
///   the first leaves is minted there (`mate_axes_parallel`): on its
///   log, and not on the cap's, whose one slide runs along its axis.
/// - The pattern's rule — its direction and the sign of its spacing —
///   is decided where the pattern evaluates, on its own log, and on
///   neither placement's though the cap reads its copy through it.
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
    let (cap_log, other_log, pattern_log) = (names(s.cap), names(s.other), names(s.pattern));
    let on = |got: &[&str], p: &str| got.contains(&p);
    for (what, got) in [("cap", &cap_log), ("other block", &other_log)] {
        assert!(
            on(got, "mate_chart_reference_antipode"),
            "the {what} charts its values on its own log: {got:?}"
        );
    }
    assert!(
        on(&other_log, "mate_axes_parallel") && !on(&cap_log, "mate_axes_parallel"),
        "the plane slide's line is the other block's: {other_log:?}, cap {cap_log:?}"
    );
    for p in PATTERN_RULE {
        assert!(
            on(&pattern_log, p),
            "the pattern decides its {p}: {pattern_log:?}"
        );
        for (what, got) in [("cap", &cap_log), ("other block", &other_log)] {
            assert!(
                !on(got, p),
                "the pattern's {p} is on no placement's log, the {what}'s included: {got:?}"
            );
        }
    }
}

// ---- the memo ----

/// **Whether `later` served `id` from `earlier`** rather than running
/// its op again: a memo hit returns the prior value's contact records,
/// the same allocation, and every op that runs mints a fresh one.
fn reused_value(earlier: &Evaluation<f64>, later: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    match (earlier.value(id), later.value(id)) {
        (Some(a), Some(b)) => std::sync::Arc::ptr_eq(&a.contacts, &b.contacts),
        _ => false,
    }
}

/// **A reused placement carries the log a fresh evaluation of it
/// carries.** The cap's bundle is a coaxial mate against a pattern
/// copy and its slide and spin values, so its log holds the values'
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
        fresh_log
            .iter()
            .any(|v| v.predicate == "mate_chart_reference_antipode"),
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

/// The other block's bottom cap on a plane of the world `z = h` up,
/// facing down: a plane mate whose target is the world.
fn on_the_world(s: &Scene, h: f64) -> Mate<Formula> {
    Mate {
        on: fixture::plane(s.bottom(s.other)),
        to: Target::World(
            WorldRead::through(PoseRole::Xy)
                .slide(WorldRead::through(PoseRole::Z), len(h))
                .flip(),
        ),
        class: ContactClass::Rest,
    }
}

/// **A lever out of the format's range refuses at its one door,
/// through the fold.** A rest against the base at a standoff of
/// `1e154` m is a finite datum, and so is its distance from the
/// target's origin; the lever the bundle forms over it — the copy's
/// reach plus that distance — has a sixteenfold square that overflows,
/// and a plane mate beside it would have the fold decide the bundle's
/// angular splits over that lever. The fold forms the lever through
/// `Arm::of`, which refuses it typed — against the bundle's first
/// mate, carrying both halves and the range recourse — so no predicate
/// is handed it, and no escalation names the parallelism predicate.
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
            ContactClass::Rest,
        ),
    );
    s.add(s.other, on_the_world(&s, BASE_HEIGHT));
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
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    let (doc, rest) = fixture::at_the_door_of(
        &s.doc,
        &reach,
        s.other,
        fixture::mate(
            fixture::plane(s.bottom(s.other)),
            standoff(s.base_top(), 1e200),
            ContactClass::Rest,
        ),
    )
    .unwrap_or_else(|(at, fault)| panic!("the door admits the rest, got {at:?}: {fault}"));
    s.doc = doc;
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

// ---- the `Part`'s own index ----

/// The cap resting on the copy a `Part` over the pattern carries,
/// read at the `Part` under copy 1's name.
fn rest_on_part(s: &mut Scene, part: RecipeNodeId) -> MateRef {
    let read_at_part = fixture::head_at(
        part,
        in_copy(s.pattern, 1, in_part(s.block, s.block_body, CapEnd::End)),
    );
    s.add(
        s.cap,
        fixture::rest(s.bottom(s.cap), read_at_part, ContactClass::Rest),
    )
}

/// **A `Part`'s index that does not evaluate is refused at the
/// `Part`**, the node the evaluation fails, and not at the healthy
/// pattern below it; the cap whose mate reads the `Part` is poisoned
/// through it. The index `k · i64::MAX + 1` selects copy 1 at `k = 0`,
/// where the cap rests on it, and overflows the count's range at
/// `k = 1`.
///
/// *Breaks if* the refusal lands on the pattern, or the cap evaluates
/// over a copy that no longer exists.
#[test]
fn a_parts_index_that_does_not_evaluate_is_refused_at_the_part() {
    use editor_core::{Dimension, FreeValue, FreeVar, PartSelect, VarName};
    let mut s = scene("msolve11-part-index");
    let k = VarName::from_static("k");
    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::DeclareVar {
            name: k.clone(),
            def: editor_core::VarDecl::Free(FreeVar::Count { value: 0 }),
        },
    );
    s.doc = doc;
    let index = Formula::add(
        Formula::mul(
            Formula::named(k.clone(), Dimension::Count),
            Formula::count(i64::MAX),
        )
        .unwrap(),
        Formula::count(1),
    )
    .unwrap();
    let part = s.insert(Node::Part {
        of: s.pattern.into(),
        select: PartSelect::Instance(index),
    });
    rest_on_part(&mut s, part);
    let ev = fixture::run(&s.doc, &s.opts);
    assert!(
        matches!(ev.result(s.cap), Some(NodeResult::Ok(_))),
        "the index selects the named copy at k = 0: {:?}",
        ev.result(s.cap)
    );

    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::SetVarValue {
            var: k.into(),
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
                NodeErrorKind::Expr { slot: editor_core::SlotId::Instance, .. }
            )
        ),
        "the `Part` fails on its own index: {:?}",
        ev.result(part)
    );
    assert!(
        matches!(ev.result(s.pattern), Some(NodeResult::Ok(_))),
        "the pattern below it is healthy: {:?}",
        ev.result(s.pattern)
    );
    assert!(
        matches!(
            ev.result(s.cap),
            Some(NodeResult::Poisoned { through }) if *through == part
        ),
        "the cap is poisoned through the `Part`: {:?}",
        ev.result(s.cap)
    );
}

/// **A `Part`'s index outside its value is refused as the evaluation
/// refuses it, at the `Part`.** The index 5 over a two-copy pattern
/// selects no copy: the evaluation fails the `Part` with
/// `InstanceOutOfRange`, and the cap whose mate reads it is poisoned
/// through it rather than refused in words that name a copy that does
/// not exist.
///
/// *Breaks if* the cap carries a refusal of its own for the `Part`'s.
#[test]
fn a_parts_index_outside_its_value_is_refused_at_the_part() {
    use editor_core::PartSelect;
    let mut s = scene("msolve11-part-out-of-range");
    let part = s.insert(Node::Part {
        of: s.pattern.into(),
        select: PartSelect::Instance(Formula::count(5)),
    });
    rest_on_part(&mut s, part);
    let ev = fixture::run(&s.doc, &s.opts);
    let Some(NodeResult::Failed(failed)) = ev.result(part) else {
        panic!("the evaluation fails the `Part`: {:?}", ev.result(part));
    };
    assert!(
        matches!(
            failed.kind,
            NodeErrorKind::InstanceOutOfRange {
                index: 5,
                count: 2,
                ..
            }
        ),
        "{:?}",
        failed.kind
    );
    assert!(
        matches!(
            ev.result(s.cap),
            Some(NodeResult::Poisoned { through }) if *through == part
        ),
        "the cap is poisoned through the `Part`: {:?}",
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
    s.add(
        s.other,
        fixture::rest(s.bottom(s.other), s.base_top(), ContactClass::Rest),
    );
    let (doc, _) = editor_core::test_support::beside_past_the_bundle_rule(
        &s.doc,
        s.other,
        on_the_world(&s, BASE_HEIGHT),
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
