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
    CancelToken, Coord, DEFAULT_MEMBER_BUDGET, Dimension, EvalOptions, EvalOutcome, Evaluation,
    Formula, Node, ProfileDoc, RecipeNodeId, TargetAxis, evaluate,
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

/// **The level schedule charges in the schedule's order, not level by
/// level**: a placement `p` over `i = index(3)` copies a block, and an
/// extrude `c` whose depth reads `j = index(5)`, inserted after it,
/// stands a level BELOW `p` (its profile reads nothing) yet after it in
/// the schedule's order. Under a budget of 6, `p`'s 3 steps are
/// admitted and `c`'s 5 cross, on both schedules. Red if a level is
/// charged before the next is: `c`, a level earlier, would be charged
/// first and `p` would cross.
#[test]
fn a_family_a_level_down_and_later_in_order_crosses_after() {
    let doc = ProfileDoc::empty_derived("budget-levels", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 3);
    let (doc, j) = fixture::index(doc, "j", 5);
    let (doc, cell) = block(doc, (0.0, 0.5), (0.0, 0.5), 0.5);
    let slide = Formula::mul(fixture::scalar_of(i), fixture::len(1.0)).unwrap();
    let (doc, p) = fixture::family_of(doc, cell, vec![(Coord::Slide(TargetAxis::X), slide)]);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let depth = Formula::mul(
        Formula::count_to_scalar(
            Formula::add(Formula::var(j, Dimension::Count), Formula::count(1)).unwrap(),
        )
        .unwrap(),
        fixture::len(0.25),
    )
    .unwrap();
    let (doc, c) = fixture::insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let ev = run(&doc, None, &EvalOptions::default());
    let at = |id| ev.order.iter().position(|&n| n == id).unwrap();
    assert!(
        at(p) < at(c),
        "the placement precedes the extrude in the order"
    );
    for parallel in [false, true] {
        let ev = run(&doc, None, &budget(OUT_AT_B, parallel));
        out_at_b(&ev, p, c, &format!("parallel {parallel}"));
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

/// **A part of `n` copies**: a document `name` holding a row of `n`
/// copies of a block, over `k = index(n)`, its world the row.
fn row_part(name: &str, n: i64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(name, Tol::witness());
    let (doc, k) = fixture::index(doc, "k", n);
    let (doc, cell) = block(doc, (0.0, 0.5), (0.0, 0.5), 0.5);
    let slide = Formula::mul(fixture::scalar_of(k), fixture::len(1.0)).unwrap();
    fixture::family_of(doc, cell, vec![(Coord::Slide(TargetAxis::X), slide)])
}

/// A document instancing each of `parts`, in order, through a store
/// holding them, its instances, and options over that store.
fn instancing(parts: &[(&str, i64)]) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = fixture::resolver::PartStore::new();
    let mut doc = ProfileDoc::empty_derived("budget-instancing", Tol::witness());
    let mut instances = Vec::new();
    for &(name, n) in parts {
        let (part, _) = store.insert_part(row_part(name, n), Tol::witness());
        let (next, instance) = fixture::insert(doc, fixture::mated_instance(part));
        doc = next;
        instances.push(instance);
    }
    (doc, instances, fixture::resolver::with_resolver(store))
}

/// **A part is charged at its instance, warm or cold**: an instance of
/// a part of 5 copies runs out of a budget of 3 at the instance, cold
/// and over a prior that carries the part's row. Red if a carried row
/// is not charged (the warm run completes).
#[test]
fn a_carried_part_is_charged_as_an_evaluated_one() {
    let (doc, instances, opts) = instancing(&[("budget-part-five", 5)]);
    let out = EvalOutcome::Exhausted {
        at: instances[0],
        held: 0,
        budget: 3,
    };
    let three = EvalOptions {
        member_budget: 3,
        ..opts.clone()
    };
    let cold = run(&doc, None, &three);
    assert_eq!(cold.outcome, out, "cold, the instance runs out");
    let full = run(&doc, None, &opts);
    assert_eq!(
        full.outcome,
        EvalOutcome::Completed,
        "the default admits it"
    );
    let warm = run(&doc, Some(&full), &three);
    assert_eq!(warm.outcome, out, "warm, the carried row is charged alike");
}

/// **Parts share the run's one budget**: two parts of 2 and 3 copies,
/// each listing its row twice (the row, and its world's placement of
/// it), 4 and 6 steps, each fit a budget of 8 and together cross it at
/// the second instance, the first's 4 steps held. Red if each part's
/// evaluation is a run of its own under the whole budget.
#[test]
fn two_parts_that_each_fit_together_run_out() {
    let (doc, instances, opts) = instancing(&[("budget-part-two", 2), ("budget-part-three", 3)]);
    for parallel in [false, true] {
        let ev = run(
            &doc,
            None,
            &EvalOptions {
                member_budget: 8,
                parallel,
                ..opts.clone()
            },
        );
        assert_eq!(
            ev.outcome,
            EvalOutcome::Exhausted {
                at: instances[1],
                held: 4,
                budget: 8
            },
            "parallel {parallel}: the second instance crosses"
        );
        assert!(
            ev.value(instances[0]).is_some(),
            "the first is in the prefix"
        );
    }
}
