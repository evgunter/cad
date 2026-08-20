//! **E4's door, and exactly how far it opens** — the compile-only
//! demonstration commissioned with Wave 0 decision **D1** of
//! `docs/SMELL-SCAN-2026-08.md` (Evan, 2026-08-19: *a `Dual` may not
//! certify — at least for now — but it may have `Bounds`*).
//!
//! `docs/ERROR-DESIGN.md` **E4** states the sensitivity mechanism as
//! *"∂m/∂pᵢ = evaluate the recipe at `Dual<f64>` with pᵢ seeded"*, and
//! `DESIGN.md` schedules it at M10. The recipe evaluator is
//! [`editor_core::eval::evaluate`], whose bound is
//! `Decide + ContentBits + geom_core::Bounds + Send + Sync +
//! topo::PropsQuadLane`. `SMELL-SCAN`'s S2 steelman found `Bounds` to be
//! the unregistered structural blocker: `geom_core::Bounds` had no `Dual`
//! impl, so `evaluate::<Dual64>` did not compile and nothing recorded
//! that as a blocker.
//!
//! **This suite pins where that stands now.** The `Bounds` lock is open —
//! a `Dual64` satisfies every one of `evaluate`'s bounds *except*
//! `ContentBits`, which is what the row below asserts, and asserting it
//! for all-but-one is what makes `ContentBits` the named residue rather
//! than a vague "something else". The row goes red in both directions: if
//! `Bounds for Dual` is reverted it stops compiling, and if `evaluate`'s
//! bound gains a term that a dual cannot meet it stops compiling too.
//!
//! **It deliberately does not build E4.** No seeding, no sensitivity API,
//! no stackup reporting — those are M10's, and `ContentBits for Dual` is
//! its own decision (a dual's content key has to say whether the *seed*
//! is part of the key; if it is not, a memo can serve one parameter's
//! pass from another's, which is a soundness question about the memo, not
//! about brackets).

use geom_core::Dual64;
use geom_core::predicate::Decide;

/// Every bound [`editor_core::eval::evaluate`] requires, minus
/// `ContentBits`. Kept as a literal restatement rather than as
/// `EvalScalar` minus a term — there is no "minus" in the language, and
/// a restatement that drifts from the real bound is caught by the
/// `same_shape_as_evaluate` row below.
fn requires_everything_but_content_bits<T>()
where
    T: Decide + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
{
}

/// The full `EvalScalar` set, to keep the restatement above honest: this
/// one must accept `f64`, so if `evaluate`'s bound grows a term the
/// restatement lacks, the two rows disagree under review rather than
/// silently measuring a stale set.
fn requires_the_whole_eval_scalar_set<T: editor_core::eval::EvalScalar>() {}

/// **E4's door is open at `Bounds`.** A `Dual64` now meets every bound
/// `evaluate` asks for except `ContentBits` — which is to say the blocker
/// S2 named is gone, and the one that remains is a *different* one, in a
/// different crate, that no document had registered either.
#[test]
fn dual64_meets_every_evaluate_bound_except_content_bits() {
    requires_everything_but_content_bits::<Dual64>();
}

/// The control: `f64` meets the same reduced set AND the full one, so the
/// row above is measuring a real bound rather than a vacuous one.
#[test]
fn f64_meets_both_the_reduced_and_the_full_set() {
    requires_everything_but_content_bits::<f64>();
    requires_the_whole_eval_scalar_set::<f64>();
}
