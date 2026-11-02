//! **A run holds a member budget** (DESIGN.md Band 1, Cancelation): the
//! listing steps every family of an evaluation may take together,
//! charged at admission in the schedule's order before anything is
//! built. Running out is [`EvalOutcome::Exhausted`], the completed
//! prefix kept, the crossing family and everything after it holding no
//! result.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::fixture;
use crate::g_index_families::ring;
use editor_core::{
    CancelToken, Coord, DEFAULT_MEMBER_BUDGET, EvalOptions, EvalOutcome, Evaluation, Formula,
    ProfileDoc, RecipeNodeId, TargetAxis, evaluate,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>, opts: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn budget(member_budget: u64, parallel: bool) -> EvalOptions {
    EvalOptions {
        member_budget,
        parallel,
        ..Default::default()
    }
}

/// **Two families off one block**: `a` over `i = index(3)` and, inserted
/// after it, `b` over `j = index(5)`, each sliding a copy along `x` by
/// its index. So `a` is charged 3 listing steps and `b` 5, `a` first in
/// the schedule's order, and both stand in one level.
fn two_families() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("budget-two", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 3);
    let (doc, j) = fixture::index(doc, "j", 5);
    let (doc, cell) = block(doc, (0.0, 0.5), (0.0, 0.5), 0.5);
    let slide = |index| Formula::mul(fixture::scalar_of(index), fixture::len(1.0)).unwrap();
    let (doc, a) = fixture::family_of(doc, cell, vec![(Coord::Slide(TargetAxis::X), slide(i))]);
    let (doc, b) = fixture::family_of(doc, cell, vec![(Coord::Slide(TargetAxis::X), slide(j))]);
    (doc, a, b)
}

/// The run out at `b` under a budget of 6: `a`'s 3 steps admitted, and
/// `b`'s 5 passing the 3 left.
const OUT_AT_B: u64 = 6;

/// `ev` ran out at `b` after `a`, holding `a` and not `b`.
fn out_at_b(ev: &Evaluation<f64>, a: RecipeNodeId, b: RecipeNodeId, what: &str) {
    assert_eq!(
        ev.outcome,
        EvalOutcome::Exhausted {
            at: b,
            held: 3,
            budget: OUT_AT_B
        },
        "{what}: the run runs out at the second family, the first's 3 steps held"
    );
    assert!(
        ev.value(a).is_some(),
        "{what}: the first family is in the prefix"
    );
    assert!(
        ev.result(b).is_none(),
        "{what}: the crossing family holds no result"
    );
}

/// **A ring of `u32::MAX` copies, evaluated headless with no cancel,
/// runs out at admission** instead of building members until the
/// allocator aborts: the ring holds no result, the block it copies
/// does, on the serial walk and the level schedule alike. Red (it never
/// returns) where nothing bounds a run nobody cancels.
#[test]
fn a_huge_ring_nobody_cancels_runs_out_before_it_builds() {
    let (doc, the_block, ring) = ring(i64::from(u32::MAX));
    for parallel in [false, true] {
        let ev = run(&doc, None, &budget(DEFAULT_MEMBER_BUDGET, parallel));
        assert_eq!(
            ev.outcome,
            EvalOutcome::Exhausted {
                at: ring,
                held: 0,
                budget: DEFAULT_MEMBER_BUDGET
            },
            "parallel {parallel}: the ring runs out of the default budget"
        );
        assert!(
            ev.result(ring).is_none(),
            "parallel {parallel}: the ring holds no result"
        );
        assert!(
            ev.value(the_block).is_some(),
            "parallel {parallel}: the block before it is in the prefix"
        );
    }
}

/// **The serial walk and the level schedule run out at the same family,
/// holding the same steps**: two families in one level, the second
/// crossing. Red if the level schedule charges a level as a whole, or
/// in another order than the schedule's.
#[test]
fn both_schedules_run_out_at_the_same_family() {
    let (doc, a, b) = two_families();
    for parallel in [false, true] {
        let ev = run(&doc, None, &budget(OUT_AT_B, parallel));
        out_at_b(&ev, a, b, &format!("parallel {parallel}"));
    }
}

/// **The verdict is the document's and the budget's alone**: a run over
/// a warm memo that holds both families, and a cold run of the
/// document saved and loaded, run out where a cold run does. Red if a
/// memo-served family is not charged, or a run out enters the memo.
#[test]
fn a_warm_memo_and_a_cold_load_run_out_alike() {
    let (doc, a, b) = two_families();
    let full = run(&doc, None, &EvalOptions::default());
    assert_eq!(
        full.outcome,
        EvalOutcome::Completed,
        "the default admits both"
    );
    out_at_b(&run(&doc, None, &budget(OUT_AT_B, false)), a, b, "cold");
    let warm = run(&doc, Some(&full), &budget(OUT_AT_B, false));
    out_at_b(&warm, a, b, "warm");
    assert!(
        warm.reused > 0,
        "the warm run served its prefix from the memo"
    );
    let text = editor_core::save(&doc, &[], Tol::witness()).expect("saves");
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads").doc;
    out_at_b(
        &run(&loaded, None, &budget(OUT_AT_B, false)),
        a,
        b,
        "loaded",
    );
}

/// **Raising the budget on the same prior evaluates the family**: the
/// run that ran out is the next run's memo, and a budget that admits
/// `b` builds all five of its members. Red if running out entered a
/// node result the memo serves.
#[test]
fn raising_the_budget_over_the_run_that_ran_out_evaluates_it() {
    let (doc, a, b) = two_families();
    let out = run(&doc, None, &budget(OUT_AT_B, false));
    out_at_b(&out, a, b, "the prior");
    let raised = run(&doc, Some(&out), &budget(8, false));
    assert_eq!(raised.outcome, EvalOutcome::Completed, "8 admits 3 + 5");
    let members = raised
        .value(b)
        .and_then(|value| value.members.as_ref())
        .expect("the second family evaluates");
    assert_eq!(members.members.len(), 5, "every member of it");
}

/// **Listing steps count the outer values that bind no member**: an
/// outer `index(10^6)` over an inner index of count 0 lists no member
/// and still runs out at admission, of a budget of a thousand. Red if
/// the budget counts members.
#[test]
fn outer_values_over_empty_inner_indices_run_out() {
    let doc = ProfileDoc::empty_derived("budget-empty-inner", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 1_000_000);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(0), Some(i));
    let (doc, cell) = block(doc, (0.0, 0.5), (0.0, 0.5), 0.5);
    let slide = |index| Formula::mul(fixture::scalar_of(index), fixture::len(1.0)).unwrap();
    let (doc, grid) = fixture::family_of(
        doc,
        cell,
        vec![
            (Coord::Slide(TargetAxis::X), slide(i)),
            (Coord::Slide(TargetAxis::Y), slide(j)),
        ],
    );
    let ev = run(&doc, None, &budget(1_000, false));
    assert_eq!(
        ev.outcome,
        EvalOutcome::Exhausted {
            at: grid,
            held: 0,
            budget: 1_000
        },
        "a million outer values are a million listing steps"
    );
}
