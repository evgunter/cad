//! `Point3` and `Vec3` from three `f64` literals, over whichever
//! scalar the caller is working in.
//!
//! **Why generic and not two helpers.** The suites spelled these three
//! ways — `Point3::new(x, y, z)` at `f64`, `Point3::new(T::from_f64(x),
//! …)` under a `T: Real` bound, and `Point3::new(Probe(x), …)` for the
//! k-stats probe scalar. Those are not three functions with one name:
//! `<f64 as Real>::from_f64` is the identity (pinned by
//! `from_f64_is_exact_identity` in `geom-core/src/real.rs`) and
//! `<Probe as Real>::from_f64` IS `Probe(x)`
//! (`geom-core/src/k_stats.rs`), so all three are `T::from_f64` under
//! three impls. `Real` is the tree's own name for that, and writing it
//! out is what makes the sameness checkable instead of assumed.
//!
//! **What this module does NOT absorb.** The `RingInterval` triples —
//! `[RingInterval::point(x), …]`, spelled `p` in four suites and `p3`
//! in one — are not a `Point3` and do not go through `Real`; they are
//! `shared::ring`. `offset_mint.rs`'s `fn p(out, pt)` appends a point's
//! BITS to a digest and only shares the letter.

use geom_core::{Point3, Real, Vec3};

/// A point from three `f64` literals, embedded in the caller's scalar.
pub(crate) fn p3<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}

/// A vector from three `f64` literals, embedded in the caller's scalar.
pub(crate) fn v3<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}
