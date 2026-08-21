//! The §6.1 point-projection policy both halves of the crate
//! implement (The NURBS Book §6.1, pp. 229–234).
//!
//! Projecting a point onto a NURBS **curve** and onto a NURBS
//! **surface** are two Newton iterations of different dimension — one
//! orthogonality condition against two, a scalar derivative against a
//! 2×2 Jacobian — so the iterations themselves live in
//! [`crate::curves::projection`] and [`crate::surfaces::projection`].
//! Everything below is what the two share, and it is stated here
//! once: neither half restates it, and neither may hold a private
//! copy of a constant. Each half's module docs give this policy's
//! dimension-specific *reading* — what "a seed per span" means over a
//! span versus over a span cell, which quantities the acceptance
//! conditions test, and which residuals ride the result.
//!
//! # This is C6's f64 lane — structure machinery, with a lifted payload
//!
//! Projection *selects* a parameter (structure); it decides no
//! topology. The **selection** is `f64` with raw comparisons under
//! C6's pinning rule, deterministic per D9: fixed constants, fixed
//! seeding, fixed iteration policy, no data-dependent iteration order.
//! The certification of whatever a consumer builds from the foot point
//! is the consumer's, at its own scalar (C2).
//!
//! # `f64` structure, `T` payload
//!
//! Both halves follow the ratified **f64-structure + T-lift** pattern:
//!
//! - the seeding sweep and the Newton iteration read the payload
//!   through **bracket midpoints** ([`mid`]) and run in `f64`, so on
//!   finite arithmetic the selected parameter is bitwise what an
//!   `f64`-only iteration produces;
//! - the returned projection is then **evaluated at `T`** at that
//!   selected parameter, so the distance and the orthogonality
//!   residuals are the consumer's own scalar — an enclosure on the
//!   interval lane, which is what makes a rung-3 certificate against a
//!   NURBS operand exist there at all.
//!
//! The bound is the sole-bound `T: Bounds` the discipline reserves for
//! certification/driver code (`geom_core::real`'s scope rule): reading
//! a bracket to *select* a parameter is the driver half of that rule,
//! and nothing here decides.
//!
//! A note on what the lift does NOT claim: Newton at the interval
//! scalar would be a different algorithm (interval Newton with
//! existence tests), and this is deliberately not that. The iteration
//! is a search for structure; the honesty is entirely in the residuals
//! it reports, which are reported at the consumer's scalar.
//!
//! # Honesty (C2.1): a bad projection cannot launder a bad cache
//!
//! Newton converges to *stationary points* of the distance, so a
//! deliberately bad seed can converge to a far branch or sheet with a
//! tiny orthogonality residual and a large distance; and at a
//! degenerate parameterization point (a vanishing partial — a cusp, a
//! collapsed row of control points) the cosine condition is met with a
//! trivially-zero orthogonality residual. **Every residual rides the
//! result**, so a consumer must band them *together*: wrong
//! branch/sheet ⇒ the distance fails the band; a boundary clamp ⇒ an
//! orthogonality residual fails it. Neither half decides anything;
//! both report. Each half's docs name its own residual set and its
//! planted-fixture rows.
//!
//! # Non-convergence
//!
//! Never a best-effort answer: each half returns its own typed
//! inconclusive refusal carrying the last iterate and the last
//! residuals, so a diagnosing consumer sees where the iteration died
//! and nothing can be mistaken for a foot point.

use geom_core::Bounds;

/// Fixed Newton iteration cap (D9 — never data-dependent).
pub const PROJECT_MAX_ITERS: usize = 32;

/// Fixed seed count per span **per parameter direction** (each half's
/// module docs carry its seeding rule): a curve span gets this many
/// seeds, a surface span cell gets this many squared.
pub const PROJECT_SEEDS_PER_SPAN: usize = 8;

/// Point-coincidence and parameter-stagnation threshold, in meters
/// (the Book's ε₁ role).
pub const PROJECT_EPS_POINT: f64 = 1e-13;

/// Cosine-zero threshold, dimensionless (the Book's ε₂ role):
/// `|g| ≤ ε₂·|C′|·|C − P|` is `|cos ∠(C′, C − P)| ≤ ε₂`, and the
/// surface half tests the same quantity in both parameter directions.
pub const PROJECT_EPS_COSINE: f64 = 1e-12;

/// The bracket midpoint of a scalar — the **structure read** both
/// seeding sweeps and both Newton iterations run on.
///
/// Written `lo + ½(hi − lo)` rather than `½(lo + hi)` so that a finite
/// bracket cannot overflow at the representable extremes; at `f64`
/// (where `lo` = `hi` = the value) it is then the identity on every
/// finite input.
///
/// **It is not the identity on a non-finite one, and that is
/// load-bearing.** At `x = ±∞` the form is `∞ + 0.5·(∞ − ∞)` = NaN, so
/// an overflowed residual reaches the acceptance tests as NaN, loses
/// every comparison, and falls out to the typed inconclusive refusal
/// rather than being reported as a converged foot at infinite
/// distance. Poison behaves the same way and for the same reason:
/// poison never selects. `crates/geom/tests/curves/projection.rs`'s
/// overflow row pins that exit through the public door, with finite
/// control points.
///
/// The one other departure from the identity is `mid(-0.0) = +0.0`;
/// every consumer here either takes `.abs()` of the result or divides
/// it into a difference that is insensitive to the sign of zero.
///
/// **At a dual scalar this is where the derivative channel leaves.**
/// `Bounds for Dual` is the value channel's bracket with the tangent
/// discarded, so `mid` returns an `f64` that both halves then FREEZE
/// into the iteration and into the returned foot parameter. The value
/// channel is unaffected — it is the plain-`T` run's bit-identically
/// (D9) — but every quantity the frozen parameter feeds carries a
/// PARTIAL derivative at that parameter rather than a total one. The
/// two `Projection` types say which of their fields that reaches; the
/// defect and its dispositions are issue #874.
pub(crate) fn mid<T: Bounds>(v: T) -> f64 {
    let (lo, hi) = (v.lo(), v.hi());
    lo + 0.5 * (hi - lo)
}
