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
