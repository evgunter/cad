//! The §6.1 point-projection policy both halves of the crate
//! implement (The NURBS Book §6.1, pp. 229–234).
//!
//! Projecting a point onto a NURBS **curve** and onto a NURBS
//! **surface** are two Newton iterations of different dimension — one
//! orthogonality condition against two, a scalar derivative against a
//! 2×2 Jacobian — so the iterations themselves live in
//! [`crate::curves::projection`] and [`crate::surfaces::projection`].
//! What they share is this: one iteration budget, one seed schedule,
//! one pair of acceptance thresholds, and one rule for reading a
//! bracketed scalar as structure. Those are declared here, once, and
//! neither half may hold a private copy — the halves ARE one policy,
//! and a second declaration is how a policy becomes two.
//!
//! Each half's module docs state that policy's dimension-specific
//! form: what "a seed per span" means over a span versus over a span
//! cell, what the acceptance conditions test, and what the returned
//! residuals are honest about.

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
/// Written `lo + ½(hi − lo)` rather than `½(lo + hi)` so that at `f64`
/// (where `lo` = `hi` = the value) it is bitwise the identity, with no
/// overflow at the representable extremes. A poisoned bracket yields
/// NaN, which loses every `<` comparison in the sweeps and breaks a
/// Newton loop into the typed refusal — poison never selects.
pub(crate) fn mid<T: Bounds>(v: T) -> f64 {
    let (lo, hi) = (v.lo(), v.hi());
    lo + 0.5 * (hi - lo)
}
