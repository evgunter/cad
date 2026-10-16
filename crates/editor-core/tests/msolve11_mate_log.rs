//! **The mate solve's decisions, on the log of the placement whose
//! bundle each decided.**
//!
//! The solve runs once per evaluation, before any node's frame opens,
//! and keeps every decision it makes for exactly one placement: a
//! decision about a constraint's own datum, a decision the fold makes
//! adding a constraint to its bundle's intersection, and a decision
//! about the bundle once folded are all that bundle's placement's.
//! Each placement's node splices what was kept for it into its own
//! frame. That every decision is on exactly one placement's log is
//! checked for every mate document a suite evaluates through
//! `fixture::run` (`fixture::solve_decisions_have_one_home`); the rows
//! here pin WHICH placement, the memo, and the lever and `Part`-index
//! rows beside them.
//!
//! Every row goes through ordinary doors — `DocEdit::InsertNode`,
//! `DocEdit::AddConstraint`, `evaluate` with or without a prior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions,
    Evaluation, Formula, Mate, MateFrame, MatePrimitive, MateRef, Node, NodeResult, PatternKind,
    ProfileDoc, RecipeNodeId, SitedFace, evaluate,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{in_copy, insert, len, on_frame, scl};
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
    cap_body: RecipeNodeId,
    block_body: RecipeNodeId,
    pattern: RecipeNodeId,
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
    let (doc, block) = insert(doc, Node::instantiate_part(block_ref));
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
    Scene {
        doc,
        opts,
        base,
        base_body,
        block,
        other,
        cap,
        cap_body: block_body,
        block_body,
        pattern,
    }
}

impl Scene {
    /// The base's top cap, read at the base.
    fn base_top(&self) -> SitedFace {
        fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End))
    }

    /// Copy 1 of the patterned block's top cap, read at the pattern.
    fn copy_top(&self) -> SitedFace {
        fixture::head_at(
            self.pattern,
            in_copy(
                self.pattern,
                1,
                in_part(self.block, self.block_body, CapEnd::End),
            ),
        )
    }

    /// The cap block's bottom cap, read at the cap.
    fn cap_bottom(&self) -> SitedFace {
        fixture::head_at(self.cap, in_part(self.cap, self.cap_body, CapEnd::Start))
    }

    /// The other block's bottom cap, read at that block.
    fn other_bottom(&self) -> SitedFace {
        fixture::head_at(
            self.other,
            in_part(self.other, self.block_body, CapEnd::Start),
        )
    }

    /// Inserts `node` through the door, levered by the scene's parts.
    fn insert(&mut self, node: editor_core::AuthoredNode) -> RecipeNodeId {
        let reach = editor_core::mate_reach::<f64>(&self.opts, Tol::witness());
        let (doc, id) = fixture::step_with(
            self.doc.clone(),
            DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            &reach,
        );
        self.doc = doc;
        id.unwrap()
    }

    /// Adds `mate` to the bundle of the copy it places, through the
    /// door, levered by the scene's parts.
    fn add(&mut self, mate: Mate<Formula>) -> MateRef {
        let reach = editor_core::mate_reach::<f64>(&self.opts, Tol::witness());
        let (doc, id) = fixture::constrain_with(self.doc.clone(), &reach, mate);
        self.doc = doc;
        id
    }

    /// A rest of the cap's bottom cap on copy 1's top cap.
    fn rest(&mut self) -> MateRef {
        let mate = seat(
            self.copy_top(),
            self.cap_bottom(),
            (0.5, 0.5, BLOCK_HEIGHT),
            MatePrimitive::PlanarRest { offset: 0.0 },
            None,
        );
        self.add(mate)
    }

    /// A clocked coaxial pin of the cap on copy 1, on the rest's axis.
    fn pin(&mut self) -> MateRef {
        let mate = seat(
            self.copy_top(),
            self.cap_bottom(),
            (0.5, 0.5, BLOCK_HEIGHT),
            MatePrimitive::Coaxial,
            Some(0.0),
        );
        self.add(mate)
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

/// A seat of `b`'s bottom cap on `a`'s top cap at `(x, y, z)`.
fn seat(
    a: SitedFace,
    b: SitedFace,
    (x, y, z): (f64, f64, f64),
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> Mate<Formula> {
    fixture::mate(
        a,
        b,
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored(
                [x, y, z],
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
            primitive,
            sense: AxisSense::Opposed,
            clocking,
        },
    )
}

/// A seat on the base's top cap at `(x, y)`.
fn on_base(xy: (f64, f64)) -> (f64, f64, f64) {
    (xy.0, xy.1, BASE_HEIGHT)
}

/// The verdict log a placement's row carries, as `(predicate, sign)`
/// pairs.
fn log(ev: &Evaluation<f64>, placement: RecipeNodeId) -> Vec<geom_core::k_stats::Verdict> {
    match ev.result(placement) {
        Some(NodeResult::Ok(v)) => v.verdicts.to_vec(),
        other => panic!("placement {} did not evaluate: {other:?}", placement.0),
    }
}

/// The decisions over one constraint's two authored sides — each a part base
/// with one literal step, admitted at the decided rigidity door (seven
/// margins; the translation's finiteness is not a decision) before the
/// solve re-mints the composed axis — the first thing the solve decides
/// about a constraint's own datum.
const OWN_FRAMES: [&str; 16] = [
    "transform_rigid_col0_unit",
    "transform_rigid_col1_unit",
    "transform_rigid_col2_unit",
    "transform_rigid_col01_orth",
    "transform_rigid_col12_orth",
    "transform_rigid_col02_orth",
    "transform_rigid_det_plus_one",
    "mate_frame_offset_axis",
    "transform_rigid_col0_unit",
    "transform_rigid_col1_unit",
    "transform_rigid_col2_unit",
    "transform_rigid_col01_orth",
    "transform_rigid_col12_orth",
    "transform_rigid_col02_orth",
    "transform_rigid_det_plus_one",
    "mate_frame_offset_axis",
];

// ---- one home per decision ----

/// **Each decision is on the placement whose bundle it decided.**
/// Two placements: the cap, held by a rest and a clocked pin against
/// copy 1 of a pattern (so the bundle has a static left factor to
/// derive), and the other block, held by a clocked coincidence
/// against the base.
///
/// - Each placement's log opens with its first constraint's two frames.
/// - The fold's intersection is decided while the pin is added to what
///   the rest left (`mate_axis_normal_perpendicular`, then the
///   membership checks): the cap's, and on no other log.
/// - The bundle's left factor — the pattern's direction and the sign
///   of its spacing, derived once the bundle is folded
///   (`eval_direction_norm`, `pattern_spacing`) — closes the cap's log
///   and is on no other.
/// - The rider on the coincidence (`mate_clocking_redundant`) is the
///   other block's.
#[test]
fn each_decision_is_on_the_log_of_the_placement_whose_bundle_it_decided() {
    let mut s = scene("msolve11-home");
    s.rest();
    s.pin();
    s.add(seat(
        s.base_top(),
        s.other_bottom(),
        on_base((6.0, 6.0)),
        MatePrimitive::FrameCoincidence,
        Some(0.0),
    ));
    let ev = fixture::run(&s.doc, &s.opts);
    let names = |placement| -> Vec<&'static str> {
        log(&ev, placement).iter().map(|v| v.predicate).collect()
    };
    let (cap_log, other_log) = (names(s.cap), names(s.other));
    for (what, got) in [("cap", &cap_log), ("other block", &other_log)] {
        assert!(
            got.starts_with(&OWN_FRAMES),
            "the {what}'s log opens with its own frames: {got:?}"
        );
    }
    let on = |got: &[&str], p: &str| got.contains(&p);
    assert!(
        on(&cap_log, "mate_axis_normal_perpendicular") && on(&cap_log, "mate_member_axis_fixed"),
        "the intersection is the cap's: {cap_log:?}"
    );
    assert!(
        !other_log
            .iter()
            .any(|p| p.starts_with("mate_member_") || p.starts_with("mate_axis")),
        "the other block's log holds no intersection: {other_log:?}"
    );
    const LEFT_FACTOR: [&str; 2] = ["eval_direction_norm", "pattern_spacing"];
    assert!(
        cap_log.ends_with(&LEFT_FACTOR),
        "the bundle's left factor is the cap's, decided after the fold: {cap_log:?}"
    );
    for p in LEFT_FACTOR {
        assert!(
            !on(&other_log, p),
            "the left factor's {p} is on no other placement's log: {other_log:?}"
        );
    }
    assert_eq!(
        other_log[OWN_FRAMES.len()..],
        ["mate_clocking_redundant"],
        "the rider is its own placement's"
    );
}

// ---- the memo ----

/// **Whether `later` served `id` from `earlier`** rather than running
/// its op again: a memo hit returns the prior value's name table, the
/// same allocation, and every op that runs mints a fresh one.
fn reused_value(earlier: &Evaluation<f64>, later: &Evaluation<f64>, id: RecipeNodeId) -> bool {
    match (earlier.value(id), later.value(id)) {
        (Some(a), Some(b)) => std::sync::Arc::ptr_eq(&a.name_table, &b.name_table),
        _ => false,
    }
}

/// **A reused placement carries the log a fresh evaluation of it
/// carries.** The cap's bundle folds a rest and a pin against a pattern
/// copy, so its log holds frames, the fold's intersection and the
/// bundle's left factor. A constraint added to ANOTHER placement leaves
/// the cap's answer and key as they were, so the memo serves it; what
/// the solve decides about it is re-decided this run, and the served
/// row carries that, exactly as the row of an evaluation with no prior
/// does.
#[test]
fn a_reused_placement_carries_the_log_a_fresh_one_does() {
    let mut s = scene("msolve11-memo");
    s.rest();
    s.pin();
    let first = s.eval(None);
    s.add(seat(
        s.base_top(),
        s.other_bottom(),
        on_base((6.0, 6.0)),
        MatePrimitive::FrameCoincidence,
        Some(0.0),
    ));
    let later = s.eval(Some(&first));
    let fresh = s.eval(None);
    assert!(
        reused_value(&first, &later, s.cap),
        "the cap's answer and key did not move, so the memo serves it"
    );
    assert!(
        !log(&fresh, s.cap).is_empty(),
        "the solve decides something about the cap"
    );
    assert_eq!(
        log(&later, s.cap),
        log(&fresh, s.cap),
        "the served cap carries this run's decisions about it"
    );
    // The escalation channel travels with the verdicts through the same
    // splice and the same placement arm, but it cannot be pinned here:
    // an escalation in the solve refuses its placement, and only an
    // `Ok` placement is ever served from the memo.
}

// ---- the lever ----

/// The `Unleverable` refusal a placement carries, panicking on any other.
fn unleverable(fault: &editor_core::MateFault) -> &editor_core::LeverRefusal {
    match fault {
        editor_core::MateFault::Unleverable { refusal, .. } => refusal,
        other => panic!("expected an unleverable mate, got {other:?}"),
    }
}

/// **A lever out of the format's range refuses at its one door,
/// through the solve.** A rest authored at an offset of `1e300` m is
/// a finite datum whose lever is finite too, and a pin beside it in
/// the same bundle would have the fold decide the bundle's angular
/// splits over that lever: a sine levered by `1e300` is a vector whose
/// squared length overflows unless the sine is exactly zero. The fold
/// forms each constraint's lever through `Arm::of`, which refuses it
/// typed — at the rest, the constraint whose lever leaves the range,
/// carrying both halves and the range recourse — so no predicate is
/// handed it, and no escalation names the parallelism predicate.
#[test]
fn a_lever_out_of_range_refuses_typed_through_the_solve() {
    let mut s = scene("msolve11-range-solve");
    // The pin first: the door folds the bundle as it stands without
    // the constraint being added, so the rest is admitted, and the
    // lever out of range is met where the solve folds the two.
    s.add(seat(
        s.base_top(),
        s.other_bottom(),
        on_base((2.0, 2.0)),
        MatePrimitive::Coaxial,
        Some(0.0),
    ));
    let rest = s.add(seat(
        s.base_top(),
        s.other_bottom(),
        on_base((2.0, 2.0)),
        MatePrimitive::PlanarRest { offset: 1e300 },
        None,
    ));
    let ev = fixture::run(&s.doc, &s.opts);
    let fault = match ev.result(s.other) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the placement to fail, got {other:?}"),
    };
    assert!(
        matches!(fault, editor_core::MateFault::Unleverable { mate, .. } if mate == rest),
        "the rest is the constraint whose lever leaves the range: {fault:?}"
    );
    let editor_core::LeverRefusal::OutOfRange { parts, datum } = *unleverable(&fault) else {
        panic!("expected the range refusal, got {fault:?}");
    };
    assert!(
        parts.is_finite() && parts > 0.0,
        "the parts' reach is in hand: {parts}"
    );
    assert!(
        datum.is_finite() && datum >= 1e300,
        "the datum's own terms carry the offset: {datum}"
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

/// **The same door, through the edit door.** A coincidence's rider is
/// the one row of the table that levers a decision of a constraint
/// alone, so the constraint door forms that constraint's lever —
/// through the same door — and refuses one whose datum is a frame
/// `1e200` m from its part's origin, a distance whose own square
/// overflows.
#[test]
fn a_lever_out_of_range_refuses_typed_at_the_edit_door() {
    let s = scene("msolve11-range-door");
    let mate = fixture::mate(
        s.base_top(),
        s.other_bottom(),
        ContactClass::Rest,
        Alignment {
            a: MateFrame::authored(
                [1e200, 0.0, BASE_HEIGHT],
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
            clocking: Some(0.0),
        },
    );
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    let (at, fault) =
        fixture::at_the_door(&s.doc, &reach, mate).expect_err("the door refuses the lever");
    assert_eq!(at, s.other, "the door names the placement");
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

/// **A `Part`'s index that does not evaluate is refused at the
/// `Part`**, the node the evaluation fails, and not at the healthy
/// pattern below it. The index `k · i64::MAX + 1` selects copy 1 at
/// `k = 0`, where the constraint is added, and overflows the count's
/// range at `k = 1`.
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
    let read_at_part = fixture::head_at(
        part,
        in_copy(s.pattern, 1, in_part(s.block, s.block_body, CapEnd::End)),
    );
    s.add(seat(
        read_at_part,
        s.cap_bottom(),
        (0.5, 0.5, BLOCK_HEIGHT),
        MatePrimitive::FrameCoincidence,
        None,
    ));
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
    let fault = match ev.result(s.cap) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the placement to fail, got {other:?}"),
    };
    let editor_core::MateFault::PlacerRefused {
        placer, placer_row, ..
    } = fault
    else {
        panic!("expected PlacerRefused, got {fault:?}");
    };
    assert_eq!(placer, part, "the refusal names the `Part`");
    assert!(
        matches!(
            ev.result(placer),
            Some(NodeResult::Failed(e)) if matches!(
                e.kind,
                editor_core::NodeErrorKind::Expr { slot: editor_core::SlotId::Instance, .. }
            )
        ),
        "the named node is the node the evaluation fails, on its own index: {:?}",
        ev.result(placer)
    );
    assert!(
        matches!(ev.result(s.pattern), Some(NodeResult::Ok(_))),
        "the pattern below it is healthy: {:?}",
        ev.result(s.pattern)
    );
    assert_eq!(
        placer_row,
        editor_core::PlacerRow::States,
        "the `Part`'s own row states the refusal, so the placement points there"
    );
}

/// **A `Part`'s flat index that overflows the table's row width is
/// refused at the pattern it selects from**: the flat index is a row
/// of that pattern's value, and the pattern below it, whose own count
/// fits, is not the node at fault. Two patterns of `70 000` copies,
/// one directly over the other, and a `Part` selecting from the outer:
/// copy `(69 999, 69 999)` is row `69 999 · 70 000 + 69 999`, past
/// `u32`. The insert door asks the same per-reference check the solve
/// does, and nothing here is evaluated.
#[test]
fn a_parts_flat_index_past_the_row_width_is_refused_at_the_pattern_it_selects_from() {
    use editor_core::PartSelect;
    let mut s = scene("msolve11-flat-index");
    let wide = |input| Node::Pattern {
        input,
        count: Formula::count(70_000),
        kind: PatternKind::Linear {
            direction: [scl(0.0), scl(1.0), scl(0.0)],
            spacing: len(3.0),
        },
    };
    let inner = s.insert(wide(s.block.into()));
    let outer = s.insert(wide(inner.into()));
    let part = s.insert(Node::Part {
        of: outer.into(),
        select: PartSelect::Instance(Formula::count(0)),
    });
    let b = fixture::head_at(
        part,
        in_copy(
            outer,
            69_999,
            in_copy(inner, 69_999, in_part(s.block, s.block_body, CapEnd::End)),
        ),
    );
    let fault = fixture::door_refusal(
        &s.doc,
        seat(
            b,
            s.cap_bottom(),
            (0.5, 0.5, BLOCK_HEIGHT),
            MatePrimitive::FrameCoincidence,
            None,
        ),
    );
    let editor_core::MateFault::PlacerRefused { placer, .. } = fault else {
        panic!("expected PlacerRefused, got {fault:?}");
    };
    assert_eq!(
        placer, outer,
        "the refusal names the pattern the `Part` selects from"
    );
}

/// **A `Part`'s index outside its value is refused as the evaluation
/// refuses it, at the `Part`.** The index 5 over a two-copy pattern
/// selects no copy: the evaluation fails the `Part` with
/// `InstanceOutOfRange`, and the constraint's check is the same refusal at
/// the same node, stated on the `Part`'s own row, rather than a
/// disagreement whose recourse names a copy that does not exist.
#[test]
fn a_parts_index_outside_its_value_is_refused_as_the_evaluation_refuses_it() {
    use editor_core::{NodeErrorKind, PartSelect};
    let mut s = scene("msolve11-part-out-of-range");
    let part = s.insert(Node::Part {
        of: s.pattern.into(),
        select: PartSelect::Instance(Formula::count(5)),
    });
    let b = fixture::head_at(
        part,
        in_copy(s.pattern, 1, in_part(s.block, s.block_body, CapEnd::End)),
    );
    let fault = fixture::door_refusal(
        &s.doc,
        seat(
            b,
            s.cap_bottom(),
            (0.5, 0.5, BLOCK_HEIGHT),
            MatePrimitive::FrameCoincidence,
            None,
        ),
    );
    let editor_core::MateFault::PlacerRefused {
        placer,
        error,
        placer_row,
        ..
    } = &fault
    else {
        panic!("expected PlacerRefused, got {fault:?}");
    };
    assert_eq!(*placer, part, "the refusal names the `Part`");
    assert_eq!(*placer_row, editor_core::PlacerRow::States);
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
    assert_eq!(
        format!("{:?}", error.kind()),
        format!("{:?}", failed.kind),
        "the constraint carries the refusal the `Part`'s own row states"
    );
    assert!(
        !fault.to_string().contains("copy 5"),
        "no recourse names a copy that does not exist: {fault}"
    );
}

/// **A `Part` whose index refuses while the pattern below it is
/// poisoned is carried by the fault**: the pattern's instance has a
/// part that does not resolve, so the `Part`'s row reads poisoned and
/// cannot state the refusal. The placement's fault names the `Part`,
/// says its row is silent (`PlacerRow::Silent`), and carries the
/// `Part`'s own refusal, so the cause is stated where it is read.
#[test]
fn a_part_index_refusal_behind_a_poisoned_pattern_is_carried_by_the_fault() {
    use editor_core::{Dimension, FreeValue, FreeVar, PartSelect, VarName};
    let mut store = PartStore::new();
    let (base_ref, base_body) = store.insert_part(
        slab("msolve11-poisoned-base", BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    // The block's part is stored where the resolver never looks.
    let mut elsewhere = PartStore::new();
    let (block_ref, block_body) = elsewhere.insert_part(
        slab("msolve11-poisoned-block", BLOCK_WIDTH, BLOCK_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve11-poisoned"), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, block) = insert(doc, Node::instantiate_part(block_ref));
    let (doc, other) = insert(doc, fixture::mated_instance(base_ref));
    let (doc, cap) = insert(doc, fixture::mated_instance(base_ref));
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
    let mut s = Scene {
        doc,
        opts,
        base,
        base_body,
        block,
        other,
        cap,
        cap_body: base_body,
        block_body,
        pattern,
    };
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
    let read_at_part = fixture::head_at(
        part,
        in_copy(s.pattern, 1, in_part(s.block, s.block_body, CapEnd::End)),
    );
    s.add(seat(
        read_at_part,
        s.cap_bottom(),
        (0.5, 0.5, BLOCK_HEIGHT),
        MatePrimitive::FrameCoincidence,
        None,
    ));
    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::SetVarValue {
            var: k.into(),
            value: FreeValue::Count(1),
        },
    );
    s.doc = doc;
    let ev = fixture::run(&s.doc, &s.opts);
    let fault = match ev.result(s.cap) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the placement to fail, got {other:?}"),
    };
    let editor_core::MateFault::PlacerRefused {
        placer,
        error,
        placer_row,
        ..
    } = &fault
    else {
        panic!("expected PlacerRefused, got {fault:?}");
    };
    assert_eq!(*placer, part, "the refusal names the `Part`");
    assert_eq!(
        *placer_row,
        editor_core::PlacerRow::Silent,
        "the `Part`'s row cannot state the refusal, so the fault carries it"
    );
    assert!(
        matches!(
            error.kind(),
            editor_core::NodeErrorKind::Expr {
                slot: editor_core::SlotId::Instance,
                ..
            }
        ),
        "the fault carries the `Part`'s own refusal, on its index: {:?}",
        error.kind()
    );
    assert!(
        matches!(ev.result(part), Some(NodeResult::Poisoned { .. })),
        "the `Part`'s row reads poisoned, not the refusal: {:?}",
        ev.result(part)
    );
}

// ---- the contradiction's words ----

/// **Every predicate the solve refuses a contradiction on reads in
/// words**, each row pinned, and the predicate's name is left to the
/// payload. The list is every variant: the match below is exhaustive,
/// and `ALL` must hold each variant exactly once.
#[test]
fn every_refuted_predicate_reads_in_words() {
    use editor_core::mate::Refuted;
    let words = |r: Refuted| match r {
        Refuted::RotationIdentity => "the relative rotation is not the identity",
        Refuted::TranslationZero => "the relative translation is not zero",
        Refuted::TranslationAlong => "the translation leaves the shared direction",
        Refuted::AxisFixed => "the rotation moves the shared axis",
        Refuted::TranslationInPlane => "the translation leaves the shared plane",
        Refuted::PointOnAxis => "the axis point leaves the shared axis",
        Refuted::PointFixed => "the shared point moves",
        Refuted::TwoAxisReachable => "no one rotation aligns both axes",
        Refuted::ClockingRedundant => {
            "the clocking disagrees with the roll the coincidence already pins"
        }
    };
    let mut names: Vec<&str> = Refuted::ALL.iter().map(|r| r.name()).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), Refuted::ALL.len(), "each predicate once");
    for refuted in Refuted::ALL {
        assert_eq!(Refuted::of(refuted.name()), Some(refuted));
        let said = editor_core::MateFault::Contradictory {
            held: MateRef::new(
                RecipeNodeId::new(0, 3),
                editor_core::ConstraintId(editor_core::MintId::new(1, 0)),
            ),
            added: MateRef::new(
                RecipeNodeId::new(0, 3),
                editor_core::ConstraintId(editor_core::MintId::new(2, 0)),
            ),
            predicate: refuted.name(),
            clash: editor_core::Clash::Length { metres: 0.5 },
        }
        .to_string();
        assert!(
            said.contains(words(refuted)) && !said.contains(refuted.name()),
            "{}: {said}",
            refuted.name()
        );
    }
}

// ---- the box driver ----

/// **What a box run meets on a document whose bundle escalates, as the
/// driver stands today.** The escalation is on its placement's log, so
/// `drive::classify_replay` would meet it at read (2), the log, rather
/// than at read (3)'s `_ => Bisect`. The driver never gets there: its
/// evaluations carry no resolver (`drive::lane_opts`), so at the
/// nominal witness every mated part is out of hand, the placement
/// refuses `Unleverable` before the fold decides anything, and the run
/// refuses up front with `WitnessDoesNotBuild`. The same document under
/// a resolver escalates, on the placement's log. Measured for
/// `work/msolve/a-box-independent-mate-fault-bisects-the-whole-leaf-budget.md`.
#[test]
fn a_box_run_over_an_escalating_mate_refuses_at_its_witness() {
    use editor_core::analysis::{AnalysisPolicy, analyzed_box};
    use editor_core::drive::{DriveConfig, DriveRefusal};
    use editor_core::range::{RangeField, RangeSeed, derive};
    use editor_core::{Dimension, FreeVar, VarName};
    let mut s = scene("msolve11-drive");
    let eps = Tol::witness().get().eps;
    // A tilt whose levered sine lands in the band over this pair's arm.
    let tilt = 3.0 * eps / 18.216_635_077_586_883;
    for axis in [[0.0, 0.0, 1.0], [tilt, 0.0, 1.0]] {
        let mate = fixture::mate(
            s.base_top(),
            s.other_bottom(),
            ContactClass::Rest,
            Alignment {
                a: MateFrame::authored(
                    [2.0, 2.0, BASE_HEIGHT],
                    axis,
                    [0.0, 1.0, 0.0],
                    geom_core::Tol::witness(),
                )
                .expect("a definite frame"),
                b: MateFrame::authored(
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 1.0, 0.0],
                    geom_core::Tol::witness(),
                )
                .expect("a definite frame"),
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                sense: AxisSense::Opposed,
                clocking: None,
            },
        );
        s.add(mate);
    }
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
            editor_core::NodeErrorKind::Mate(fault)
                if matches!(**fault, editor_core::MateFault::Indeterminate { .. })
        ) && !added.escalations.is_empty(),
        "under a resolver the bundle escalates, on its placement's log: {:?}",
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
        "the driver's evaluation has no resolver, so the bundle cannot lever: {cause}"
    );
}
