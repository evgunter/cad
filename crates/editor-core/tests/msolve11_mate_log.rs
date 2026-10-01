//! **The mate solve's decisions, on the log of the mate whose answer
//! each decided.**
//!
//! The solve runs once per evaluation, before any node's frame opens,
//! and keeps every decision it makes for exactly one mate: a decision
//! about a mate's own datum is that mate's, a decision the fold makes
//! adding a mate to its pair's intersection is the added mate's, and a
//! decision about the pair once folded is the pair's first mate's.
//! Each mate's node splices what was kept for it into its own frame.
//! That every decision is on exactly one mate's log is checked for
//! every mate document a suite evaluates through `fixture::run`
//! (`fixture::solve_decisions_have_one_home`); the rows here pin WHICH
//! mate, the memo, and the lever and `Part`-index rows beside them.
//!
//! Every row goes through ordinary doors — `DocEdit::InsertNode`,
//! `DocEdit::DeleteNode`, `evaluate` with or without a prior.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions,
    Evaluation, Expr, MateFrame, MatePrimitive, Node, NodeResult, PatternKind, ProfileDoc,
    ProfileProgram, RecipeNodeId, SitedFace, evaluate,
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
            profile,
            distance: len(h),
        },
    )
}

/// A base slab and two blocks, instantiated, with a linear pattern of
/// the first block standing between it and the mates read at it.
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    base: RecipeNodeId,
    base_body: RecipeNodeId,
    block: RecipeNodeId,
    other: RecipeNodeId,
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
    let (doc, other) = insert(doc, Node::instantiate_part(block_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: block,
            count: Expr::count(2),
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
        block_body,
        pattern,
    }
}

impl Scene {
    /// The base's top cap, read at the base.
    fn base_top(&self) -> SitedFace {
        fixture::head_at(self.base, in_part(self.base, self.base_body, CapEnd::End))
    }

    /// Copy 1 of the patterned block's bottom cap, read at the pattern.
    fn copy_bottom(&self) -> SitedFace {
        fixture::head_at(
            self.pattern,
            in_copy(
                self.pattern,
                1,
                in_part(self.block, self.block_body, CapEnd::Start),
            ),
        )
    }

    /// The other block's bottom cap, read at that block.
    fn other_bottom(&self) -> SitedFace {
        fixture::head_at(
            self.other,
            in_part(self.other, self.block_body, CapEnd::Start),
        )
    }

    /// Inserts `node` through the door, levered by the scene's parts.
    fn add(&mut self, node: Node<ProfileProgram>) -> RecipeNodeId {
        let reach = editor_core::mate_reach::<f64>(&self.opts, Tol::witness());
        let (doc, id) = fixture::step_with(self.doc.clone(), DocEdit::InsertNode { node }, &reach);
        self.doc = doc;
        id.unwrap()
    }

    /// Deletes `id` through the door, levered by the scene's parts.
    fn delete(&mut self, id: RecipeNodeId) {
        let reach = editor_core::mate_reach::<f64>(&self.opts, Tol::witness());
        let (doc, _) = fixture::step_with(self.doc.clone(), DocEdit::DeleteNode { id }, &reach);
        self.doc = doc;
    }

    /// A rest of copy 1's bottom cap on the base's top cap.
    fn rest(&mut self) -> RecipeNodeId {
        let node = seat(
            self.base_top(),
            self.copy_bottom(),
            (2.0, 2.0),
            MatePrimitive::PlanarRest { offset: 0.0 },
            None,
        );
        self.add(node)
    }

    /// A clocked coaxial pin of copy 1 on the base, on the rest's axis.
    fn pin(&mut self) -> RecipeNodeId {
        let node = seat(
            self.base_top(),
            self.copy_bottom(),
            (2.0, 2.0),
            MatePrimitive::Coaxial,
            Some(0.0),
        );
        self.add(node)
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

/// A seat of `b`'s bottom cap on `a`'s top cap at `(x, y)`.
fn seat(
    a: SitedFace,
    b: SitedFace,
    (x, y): (f64, f64),
    primitive: MatePrimitive,
    clocking: Option<f64>,
) -> Node<ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored([x, y, BASE_HEIGHT], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            b: MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            primitive,
            sense: AxisSense::Opposed,
            clocking,
        },
    }
}

/// The verdict log a mate's row carries, as `(predicate, sign)` pairs.
fn log(ev: &Evaluation<f64>, mate: RecipeNodeId) -> Vec<geom_core::k_stats::Verdict> {
    match ev.result(mate) {
        Some(NodeResult::Ok(v)) => v.verdicts.to_vec(),
        other => panic!("mate {} did not evaluate: {other:?}", mate.0),
    }
}

/// The frame ladder's decisions over one mate's two authored sides —
/// the first thing the solve decides about a mate's own datum.
const OWN_FRAMES: [&str; 4] = [
    "frame_point_at_aim",
    "frame_point_at_roll_offset",
    "frame_point_at_aim",
    "frame_point_at_roll_offset",
];

// ---- one home per decision ----

/// **Each decision is on the mate whose answer it decided.** Three
/// mates: a rest and a clocked pin on ONE pair (the base and copy 1 of
/// a pattern, so the pair has a static left factor to derive), and a
/// clocked coincidence on another pair.
///
/// - Each mate's log opens with its own two frames.
/// - The fold's intersection is decided while the PIN is added to what
///   the rest left (`mate_axis_normal_perpendicular`, then the
///   membership checks), so it is the pin's and not the rest's.
/// - The pair's left factor — the pattern's direction, derived for
///   the pair once its mates are folded (`eval_direction_norm`) — is
///   the pair's first mate's: the rest's, the mate an UNDER on this
///   pair would name, and not the pin's.
/// - The rider on the coincidence (`mate_clocking_redundant`) is its
///   own mate's.
#[test]
fn each_decision_is_on_the_log_of_the_mate_whose_answer_it_decided() {
    let mut s = scene("msolve11-home");
    let rest = s.rest();
    let pin = s.pin();
    let coincidence = s.add(seat(
        s.base_top(),
        s.other_bottom(),
        (6.0, 6.0),
        MatePrimitive::FrameCoincidence,
        Some(0.0),
    ));
    let ev = fixture::run(&s.doc, &s.opts);
    let names =
        |mate| -> Vec<&'static str> { log(&ev, mate).iter().map(|v| v.predicate).collect() };
    let (rest_log, pin_log, coincidence_log) = (names(rest), names(pin), names(coincidence));
    for (what, got) in [
        ("rest", &rest_log),
        ("pin", &pin_log),
        ("coincidence", &coincidence_log),
    ] {
        assert!(
            got.starts_with(&OWN_FRAMES),
            "the {what}'s log opens with its own frames: {got:?}"
        );
    }
    let on = |got: &[&str], p: &str| got.contains(&p);
    assert!(
        on(&pin_log, "mate_axis_normal_perpendicular") && on(&pin_log, "mate_member_axis_fixed"),
        "the intersection is the added mate's: {pin_log:?}"
    );
    assert!(
        !rest_log
            .iter()
            .any(|p| p.starts_with("mate_member_") || p.starts_with("mate_axis")),
        "the first mate's log holds no intersection: {rest_log:?}"
    );
    assert_eq!(
        rest_log.last(),
        Some(&"eval_direction_norm"),
        "the pair's left factor is the first mate's, decided after the fold: {rest_log:?}"
    );
    assert!(
        !on(&pin_log, "eval_direction_norm") && !on(&coincidence_log, "eval_direction_norm"),
        "the left factor is on no other mate's log: {pin_log:?} {coincidence_log:?}"
    );
    assert_eq!(
        coincidence_log[OWN_FRAMES.len()..],
        ["mate_clocking_redundant"],
        "the rider is its own mate's"
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

/// **A reused mate carries the log a fresh evaluation of it carries.**
/// The pin is the pair's SECOND mate, so its log holds the fold's
/// intersection with the rest. Deleting the rest and adding it back
/// makes the pin the pair's first mate: its answer — a determining
/// mate, unfaulted — and its key are what they were, so the memo
/// serves it, while what the solve decides about it is now its own
/// frames and the pair's left factor. The served row carries that,
/// exactly as the row of an evaluation with no prior does.
#[test]
fn a_reused_mate_carries_the_log_a_fresh_one_does() {
    let mut s = scene("msolve11-memo");
    let rest = s.rest();
    let pin = s.pin();
    let first = s.eval(None);
    s.delete(rest);
    let _rest = s.rest();
    let later = s.eval(Some(&first));
    let fresh = s.eval(None);
    assert!(
        reused_value(&first, &later, pin),
        "the pin's answer and key did not move, so the memo serves it"
    );
    assert_ne!(
        log(&first, pin),
        log(&fresh, pin),
        "the solve decides something else about the pin now"
    );
    assert_eq!(
        log(&later, pin),
        log(&fresh, pin),
        "the served pin carries this run's decisions about it"
    );
    let escalations = |ev: &Evaluation<f64>| match ev.result(pin) {
        Some(NodeResult::Ok(v)) => v.escalations.to_vec(),
        other => panic!("the pin did not evaluate: {other:?}"),
    };
    assert_eq!(escalations(&later), escalations(&fresh));
}

// ---- the lever ----

/// The `Unleverable` refusal a mate carries, panicking on any other.
fn unleverable(fault: &editor_core::MateFault) -> &editor_core::LeverRefusal {
    match fault {
        editor_core::MateFault::Unleverable { refusal, .. } => refusal,
        other => panic!("expected an unleverable mate, got {other:?}"),
    }
}

/// **A lever out of the format's range refuses at its one door,
/// through the solve.** A rest authored at an offset of `1e300` m is
/// a finite datum whose lever is finite too, and a pin beside it on
/// the same pair would have the fold decide the pair's angular splits
/// over that lever: a sine levered by `1e300` is a vector whose
/// squared length overflows unless the sine is exactly zero. The fold forms each mate's lever through
/// `Arm::of`, which refuses it typed — the rest, the first mate whose
/// lever is formed, carrying both halves and the range recourse — so
/// no predicate is handed it, and no escalation names the parallelism
/// predicate.
#[test]
fn a_lever_out_of_range_refuses_typed_through_the_solve() {
    let mut s = scene("msolve11-range-solve");
    let rest = s.add(seat(
        s.base_top(),
        s.other_bottom(),
        (2.0, 2.0),
        MatePrimitive::PlanarRest { offset: 1e300 },
        None,
    ));
    let pin = s.add(seat(
        s.base_top(),
        s.other_bottom(),
        (2.0, 2.0),
        MatePrimitive::Coaxial,
        Some(0.0),
    ));
    let ev = fixture::run(&s.doc, &s.opts);
    let fault = match ev.result(rest) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the rest to fail, got {other:?}"),
    };
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
    for id in [rest, pin] {
        let escalations = match ev.result(id) {
            Some(NodeResult::Ok(v)) => v.escalations.to_vec(),
            Some(NodeResult::Failed(e)) => e.escalations.to_vec(),
            _ => Vec::new(),
        };
        assert!(
            escalations.is_empty(),
            "mate {}: no predicate decided over the lever: {escalations:?}",
            id.0
        );
    }
}

/// **The same door, through the edit door.** A coincidence's rider is
/// the one row of the table that levers a decision of a mate alone, so
/// the insert door forms that mate's lever — through the same door —
/// and refuses one whose datum is a frame `1e200` m from its part's
/// origin, a distance whose own square overflows.
#[test]
fn a_lever_out_of_range_refuses_typed_at_the_edit_door() {
    let s = scene("msolve11-range-door");
    let node = Node::Mate {
        a: s.base_top(),
        b: s.other_bottom(),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored([1e200, 0.0, BASE_HEIGHT], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            b: MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: Some(0.0),
        },
    };
    let reach = editor_core::mate_reach::<f64>(&s.opts, Tol::witness());
    let err = s
        .doc
        .apply(&DocEdit::InsertNode { node }, Tol::witness(), &reach)
        .expect_err("the door refuses the lever");
    let editor_core::EditError::MateRefused { fault, .. } = err else {
        panic!("expected MateRefused, got {err:?}");
    };
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
/// `k = 0`, where the mate is inserted, and overflows the count's
/// range at `k = 1`.
#[test]
fn a_parts_index_that_does_not_evaluate_is_refused_at_the_part() {
    use editor_core::{Dimension, DocParam, DocParamValue, ParamName, PartSelect};
    let mut s = scene("msolve11-part-index");
    let k = ParamName::from_static("k");
    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::SetDocParam {
            name: k.clone(),
            value: DocParam::Count { value: 0 },
        },
    );
    s.doc = doc;
    let index = Expr::add(
        Expr::mul(
            Expr::param(k.clone(), Dimension::Count),
            Expr::count(i64::MAX),
        )
        .unwrap(),
        Expr::count(1),
    )
    .unwrap();
    let part = s.add(Node::Part {
        of: s.pattern,
        select: PartSelect::Instance(index),
    });
    let read_at_part = fixture::head_at(
        part,
        in_copy(s.pattern, 1, in_part(s.block, s.block_body, CapEnd::Start)),
    );
    let mate = s.add(seat(
        s.base_top(),
        read_at_part,
        (2.0, 2.0),
        MatePrimitive::FrameCoincidence,
        None,
    ));
    let ev = fixture::run(&s.doc, &s.opts);
    assert!(
        matches!(ev.result(mate), Some(NodeResult::Ok(_))),
        "the index selects the named copy at k = 0: {:?}",
        ev.result(mate)
    );

    let (doc, _) = fixture::step(
        s.doc.clone(),
        DocEdit::SetDocParamValue {
            name: k,
            value: DocParamValue::Count(1),
        },
    );
    s.doc = doc;
    let ev = fixture::run(&s.doc, &s.opts);
    let fault = match ev.result(mate) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        other => panic!("expected the mate to fail, got {other:?}"),
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
        "the `Part`'s own row states the refusal, so the mate points there"
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
        count: Expr::count(70_000),
        kind: PatternKind::Linear {
            direction: [scl(0.0), scl(1.0), scl(0.0)],
            spacing: len(3.0),
        },
    };
    let inner = s.add(wide(s.other));
    let outer = s.add(wide(inner));
    let part = s.add(Node::Part {
        of: outer,
        select: PartSelect::Instance(Expr::count(0)),
    });
    let b = fixture::head_at(
        part,
        in_copy(
            outer,
            69_999,
            in_copy(inner, 69_999, in_part(s.other, s.block_body, CapEnd::Start)),
        ),
    );
    let fault = fixture::door_refusal(
        &s.doc,
        seat(
            s.base_top(),
            b,
            (2.0, 2.0),
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
