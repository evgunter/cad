//! **E4's door is OPEN** — the type-level statement that a dual is an
//! evaluation scalar.
//!
//! `docs/ERROR-DESIGN.md` **E4** states the sensitivity mechanism as
//! *"∂m/∂pᵢ = evaluate the recipe at `Dual<f64>` with pᵢ seeded"*, and
//! `docs/DUAL-DESIGN.md` (DL1–DL6) ratifies what a dual has to satisfy
//! to walk through [`editor_core::eval::evaluate`]'s door: the memo's
//! [`editor_core::eval::ContentBits`] feeding BOTH channels (DL2), and
//! the scalar-policy seam that makes certified validation structurally
//! absent at a dual (`topo::AtRestPolicy`, DL3). This suite pins that
//! `Dual64` — and, under the `interval` feature, `Dual<Interval>` —
//! satisfies **every** bound `evaluate` asks for, so the door's state
//! is a compiler fact rather than prose.
//!
//! # What each row detects, and what none of them does
//!
//! 1. **The positive rows.** `Dual64` (and `DualInterval`) satisfy the
//!    literal bound set. Red the day any term of that set loses its
//!    `Dual` impl — `ContentBits for Dual` reverted, `Bounds for Dual`
//!    reverted, the DL3 policy's generic dual impl narrowed.
//! 2. **The drift detector**
//!    ([`evaluate_asks_for_nothing_outside_the_literal`]). The literal
//!    in row 1 is hand-copied and can go stale on its own: a term
//!    added to `evaluate`'s bound that `f64` meets and `Dual` does not
//!    would leave every row green while the suite's name became false.
//!    This row names `evaluate` itself from a generic function, so it
//!    is checked at its own definition and goes red on exactly that
//!    drift. Three functions make up this row: one bridge against
//!    `evaluate`'s literal where-clause, a second against the NAMED
//!    set `EvalScalar` (the two are restated ten lines apart in
//!    `eval/mod.rs` and can drift from each other), and a third
//!    proving the converse inclusion — so the last two together pin
//!    set EQUALITY rather than one direction.
//!
//! **What none of these detects**: a term ADDED to `evaluate`'s bound
//! that every scalar here satisfies — invisible and harmless until a
//! scalar stops satisfying it, which is the moment the rows above go
//! red. Nor does anything here execute `evaluate`; the RUNTIME half of
//! the door — a full corpus build at `Dual64`, value channel
//! bit-identical to `f64` — is `m10_di_dual_corpus.rs`.
//!
//! **It deliberately does not build E4.** No seeding surface, no
//! sensitivity API, no stackup reporting — those are M10-4's. What DL2
//! already settles about them lives at the `ContentBits for Dual` impl:
//! the seed rides the tangent bits, so the memo cannot serve one
//! parameter's pass from another's.
//!
//! The certification half of the dual ruling is pinned where it lives:
//! `geom-core/src/dual.rs`'s `compile_fail` doctests keep
//! `CertifiedEnclosure for Dual` impossible (DL1 — a dual never
//! certifies).
//!
//! The scan record this suite's predecessor kept honest — S44's D1
//! block in `docs/SMELL-SCAN-2026-08.md` — carries the dated LANDED
//! note for the flip; a change to what this suite claims owes that
//! record a matching note, as the flip itself did.

use geom_core::Dual64;
use geom_core::predicate::Decide;

/// Every bound [`editor_core::eval::evaluate`] requires, hand-written —
/// a literal restatement, and therefore a claim that can go stale on
/// its own. The bridge rows below are what keep it honest.
fn requires_every_evaluate_bound<T>()
where
    T: Decide
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + editor_core::eval::ContentBits
        + editor_core::analysis::AxisScalar,
{
}

/// **The drift detector.** Generic, so the compiler checks it HERE, at
/// the definition, for every `T` satisfying the literal above — and its
/// body names [`editor_core::eval::evaluate`], whose own where-clause
/// must therefore be implied by that set. A term added to `evaluate`'s
/// bound and not to this list stops this function compiling, no matter
/// which scalars do or do not satisfy the new term.
///
/// An `f64` instantiation cannot do this job: `EvalScalar` is a blanket
/// impl over the same set, so `f64` clearing it re-proves nothing about
/// drift. The bound has to be generic and the call has to be `evaluate`.
fn evaluate_asks_for_nothing_outside_the_literal<T>()
where
    T: Decide
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + editor_core::eval::ContentBits
        + editor_core::analysis::AxisScalar,
{
    let _ = editor_core::eval::evaluate::<T>;
}

/// The same bridge against the NAMED set rather than against
/// `evaluate`'s where-clause. `evaluate` now names `EvalScalar` rather
/// than restating its terms, so the restatement that can drift is the
/// one in THIS file: this row proves `T: EvalScalar` from it, so a term
/// added to `EvalScalar` and not to the list above breaks it.
fn the_literal_is_the_whole_eval_scalar_set<T>()
where
    T: Decide
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + editor_core::eval::ContentBits
        + editor_core::analysis::AxisScalar,
{
    requires_the_whole_eval_scalar_set::<T>();
}

/// The full `EvalScalar` set, named once so the bridges on either side
/// of it have something to prove into and out of.
fn requires_the_whole_eval_scalar_set<T: editor_core::eval::EvalScalar>() {}

/// The converse direction, so the pair pins set EQUALITY rather than
/// one inclusion: anything `EvalScalar` admits satisfies the literal
/// above, so the literal has not grown a term `evaluate` does not ask
/// for.
fn the_literal_asks_for_nothing_evaluate_does_not<T: editor_core::eval::EvalScalar>() {
    requires_every_evaluate_bound::<T>();
}

/// **E4's door is open.** A `Dual64` meets every bound `evaluate` asks
/// for: `evaluate::<Dual64>` is a well-typed function of this library.
#[test]
fn dual64_meets_every_evaluate_bound() {
    requires_every_evaluate_bound::<Dual64>();
    // Instantiated so the drift detector is monomorphised somewhere; it
    // is already checked generically at its definition, which is the
    // part that matters.
    evaluate_asks_for_nothing_outside_the_literal::<Dual64>();
    the_literal_is_the_whole_eval_scalar_set::<Dual64>();
}

/// The derivative-enclosure instantiation (DL1's third use): the
/// generic impls open the same door for `Dual<Interval>` under the
/// `interval` feature, with nothing scalar-specific added.
#[cfg(feature = "interval")]
#[test]
fn dual_interval_meets_every_evaluate_bound() {
    requires_every_evaluate_bound::<geom_core::DualInterval>();
}

/// `f64` clears both directions of the set-equality pair, so the rows
/// above are measuring a bound a real scalar can satisfy.
#[test]
fn f64_clears_both_directions_of_the_set_equality() {
    requires_every_evaluate_bound::<f64>();
    the_literal_asks_for_nothing_evaluate_does_not::<f64>();
}
