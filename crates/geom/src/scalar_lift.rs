//! Test-side scalar lifts for point and vector data.
//!
//! The two halves' unit tests both need "the same `f64` geometry, read
//! at another scalar" to check derivative-vs-dual consistency and
//! enclosure containment. The per-variant *ladders* that do that are
//! S33's territory and stay where they are; these four converters are
//! not ladders — they are dimension- and kind-blind, and one copy each
//! is enough for the whole crate.

use geom_core::{Dual, Dual64, Point3, Vec3};

/// A point as `Dual64` constants — no variable component, so only the
/// evaluation parameter carries a derivative.
pub(crate) fn dual_point(p: Point3<f64>) -> Point3<Dual64> {
    Point3::new(
        Dual::constant(p.x),
        Dual::constant(p.y),
        Dual::constant(p.z),
    )
}

/// A vector as `Dual64` constants (see [`dual_point`]).
pub(crate) fn dual_vec(v: Vec3<f64>) -> Vec3<Dual64> {
    Vec3::new(
        Dual::constant(v.x),
        Dual::constant(v.y),
        Dual::constant(v.z),
    )
}

/// A point as degenerate intervals — each coordinate its own exact
/// bracket, so the lift adds no width of its own.
///
/// The `#[cfg(test)]` below is redundant — `lib.rs` already declares
/// this whole module test-only — and is written anyway: it is what
/// makes the `interval` gate legible as the "test-only code" case of
/// `scripts/check-interval-cfg-additive.py`'s rule to a checker that
/// reads this file without its parent declaration.
///
/// **The redundancy is load-bearing, and in the safe direction.** The
/// attribute that satisfies the gate is the same attribute that makes
/// the claim true, so deleting it as "redundant" turns CI **red**
/// rather than quietly making a gated item production-reachable.
#[cfg(feature = "interval")]
#[cfg(test)]
pub(crate) fn interval_point(p: Point3<f64>) -> Point3<geom_core::Interval> {
    use geom_core::{Interval, Real};
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

/// A vector as degenerate intervals (see [`interval_point`]); the
/// double gate is [`interval_point`]'s.
#[cfg(feature = "interval")]
#[cfg(test)]
pub(crate) fn interval_vec(v: Vec3<f64>) -> Vec3<geom_core::Interval> {
    use geom_core::{Interval, Real};
    Vec3::new(
        Interval::from_f64(v.x),
        Interval::from_f64(v.y),
        Interval::from_f64(v.z),
    )
}
