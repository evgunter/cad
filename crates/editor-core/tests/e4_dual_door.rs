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
//! **This suite pins where that stands now** — that a `Dual64` satisfies
//! every one of `evaluate`'s bounds *except* `ContentBits`, so that
//! `ContentBits` is the **named** residue rather than a vague remainder.
//! The residue is issue **#687**, so the suite has to survive the day it
//! closes; the three rows below are what makes it do that.
//!
//! # What each row detects, and what none of them does
//!
//! 1. **The positive row.** `Dual64` satisfies the literal
//!    all-but-`ContentBits` set. Red the day `Bounds for Dual` is
//!    reverted, or any other term of that set loses its `Dual` impl.
//! 2. **The drift detector**
//!    ([`evaluate_asks_for_nothing_outside_the_literal_plus_content_bits`]).
//!    The literal in row 1 is hand-copied, and the earlier version of
//!    this file had no way to notice it going stale: a new term in
//!    `evaluate`'s bound that `f64` meets and `Dual` does not would have
//!    left every row green while the suite's own name became false —
//!    all-but-one silently becoming all-but-two. This row names
//!    `evaluate` itself from a generic function, so it is checked at its
//!    own definition and goes red on exactly that drift. Three functions
//!    make up this row: one bridge against `evaluate`'s literal
//!    where-clause, a second against the NAMED set `EvalScalar` (the two
//!    are restated ten lines apart in `eval/mod.rs` and can drift from
//!    each other), and a third proving the converse inclusion — so the
//!    last two together pin set EQUALITY rather than one direction.
//! 3. **The negative row**, which is a `compile_fail,E0277` doctest on
//!    [`editor_core::eval::ContentBits`] rather than a `#[test]` here —
//!    stable Rust cannot assert "`T` does **not** implement `Tr`" from
//!    inside a test binary, and the doctest is the shape `geom-core`'s
//!    `dual.rs` already uses for the `CertifiedEnclosure` half of this
//!    same ruling. It goes red the day `ContentBits for Dual` lands,
//!    which is the day this suite's name and S44's record need
//!    rewriting.
//!
//! **What none of these detects**: a term ADDED to `evaluate`'s bound
//! that a dual *does* satisfy — that is invisible here and harmless,
//! since it changes neither the residue nor the name. Nor does anything
//! here execute `evaluate`; the whole suite is a type-level statement.
//!
//! **It deliberately does not build E4.** No seeding, no sensitivity API,
//! no stackup reporting — those are M10's, and `ContentBits for Dual`
//! (#687) is its own decision (a dual's content key has to say whether
//! the *seed* is part of the key; if it is not, a memo can serve one
//! parameter's pass from another's, which is a soundness question about
//! the memo, not about brackets).

use geom_core::Dual64;
use geom_core::predicate::Decide;

/// Every bound [`editor_core::eval::evaluate`] requires, minus
/// `ContentBits`. A hand-written literal — there is no "minus" in the
/// language — and therefore a claim that can go stale on its own. The
/// three bridge rows below are what keep it honest.
fn requires_everything_but_content_bits<T>()
where
    T: Decide + geom_core::Bounds + Send + Sync + topo::PropsQuadLane + sweep::fillet::FilletLane,
{
}

/// **The drift detector.** Generic, so the compiler checks it HERE, at
/// the definition, for every `T` satisfying the literal above plus
/// `ContentBits` — and its body names [`editor_core::eval::evaluate`],
/// whose own where-clause must therefore be implied by that set. A term
/// added to `evaluate`'s bound and not to this list stops this function
/// compiling, no matter which scalars do or do not satisfy the new term.
///
/// An `f64` instantiation cannot do this job: `EvalScalar` is a blanket
/// impl over the same set, so `f64` clearing it re-proves nothing about
/// drift. The bound has to be generic and the call has to be `evaluate`.
fn evaluate_asks_for_nothing_outside_the_literal_plus_content_bits<T>()
where
    T: Decide
        + geom_core::Bounds
        + Send
        + Sync
        + topo::PropsQuadLane
        + editor_core::eval::ContentBits
        + sweep::fillet::FilletLane,
{
    let _ = editor_core::eval::evaluate::<T>;
}

/// The same bridge against the NAMED set rather than against `evaluate`'s
/// literal where-clause. `EvalScalar` and `evaluate` restate the same list
/// ten lines apart in `eval/mod.rs` and can drift from each other, so both
/// are pinned: this row proves `T: EvalScalar` from the restatement alone,
/// so a term added to `EvalScalar` and not to the literal breaks it.
fn the_reduced_set_plus_content_bits_is_the_whole_eval_scalar_set<T>()
where
    T: Decide
        + geom_core::Bounds
        + Send
        + Sync
        + topo::PropsQuadLane
        + editor_core::eval::ContentBits
        + sweep::fillet::FilletLane,
{
    requires_the_whole_eval_scalar_set::<T>();
}

/// The full `EvalScalar` set, named once so the bridges on either side of
/// it have something to prove into and out of.
fn requires_the_whole_eval_scalar_set<T: editor_core::eval::EvalScalar>() {}

/// The converse direction, so the pair pins set EQUALITY rather than one
/// inclusion: anything `EvalScalar` admits satisfies the literal above, so
/// the literal has not grown a term `evaluate` does not ask for.
fn the_literal_asks_for_nothing_evaluate_does_not<T: editor_core::eval::EvalScalar>() {
    requires_everything_but_content_bits::<T>();
}

/// **E4's door is open at `Bounds`.** A `Dual64` now meets every bound
/// `evaluate` asks for except `ContentBits` — which is to say the blocker
/// S2 named is gone, and the one that remains (#687) is a *different*
/// one, in a different crate, that no document had registered either.
#[test]
fn dual64_meets_every_evaluate_bound_except_content_bits() {
    requires_everything_but_content_bits::<Dual64>();
    // Instantiated so the drift detector is monomorphised somewhere; it
    // is already checked generically at its definition, which is the
    // part that matters.
    evaluate_asks_for_nothing_outside_the_literal_plus_content_bits::<f64>();
    the_reduced_set_plus_content_bits_is_the_whole_eval_scalar_set::<f64>();
}

/// `f64` clears both directions of the set-equality pair, so the rows
/// above are measuring a bound a real scalar can satisfy.
#[test]
fn f64_clears_both_directions_of_the_set_equality() {
    requires_everything_but_content_bits::<f64>();
    the_literal_asks_for_nothing_evaluate_does_not::<f64>();
}
