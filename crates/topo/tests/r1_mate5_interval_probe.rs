//! **R1 review probe (interval lane) for MATE-5 (PR #1443).** The unit
//! pinned its gating CI point to `lane=interval`. These rows ask what
//! the cylinder arm's two scalar-sensitive gates actually do there.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::interval::Interval;
use geom_core::{Bounds, Real};

/// `wrap_band` (the full-wrap band FAST PATH's detector) admits a
/// polygon only when every vertex is an EXACT POINT: `p.x.lo() ==
/// p.x.hi()`. But `uv_b` is always the TRANSFERRED polygon
/// `(δ + σ·u, c + σ·v)` with `δ = atan2(…)`. If `atan2` of exact
/// inputs is not a point at this scalar, the fast path can never fire
/// for B, hence never at all — on the very lane the gate pinned.
#[test]
fn probe_i1_the_band_fast_paths_exactness_gate_at_interval() {
    let one = Interval::from_f64(1.0);
    let zero = Interval::from_f64(0.0);
    let delta = zero.atan2(one);
    println!("atan2(0,1) = [{:e}, {:e}]", delta.lo(), delta.hi());
    let d = 0.7_f64;
    let delta2 = Interval::from_f64(d.sin()).atan2(Interval::from_f64(d.cos()));
    println!(
        "atan2(sin .7, cos .7) = [{:e}, {:e}]  width = {:e}",
        delta2.lo(),
        delta2.hi(),
        delta2.hi() - delta2.lo()
    );
    let tau = Interval::tau();
    let k = (delta2 / tau + Interval::from_f64(0.5)).floor();
    println!("floor((delta/tau)+.5) = [{:e}, {:e}]", k.lo(), k.hi());
    // ADOPTED at the fix pass as the PIN of the disclosed lane
    // asymmetry (it was the finding's demonstration): the transferred
    // azimuth is NOT an exact point at Interval, so `wrap_band`'s C6
    // exact-point read rejects `uv_b` there and the full-wrap band
    // fast path is structurally f64-ONLY — disclosed at the arm and
    // in the PR, and the fix head gates BOTH lanes. If this width
    // ever collapses to zero, the disclosure is stale and the band
    // arm's lane story must be re-measured.
    assert!(
        delta2.lo() < delta2.hi(),
        "the interval atan2 became exact — re-measure the band fast \
         path's lane asymmetry and update its disclosure"
    );
}
