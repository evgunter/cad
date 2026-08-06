//! Review probe (review/margin, PR #213): executable demonstration of
//! the laundered comparand at `editor-core/src/eval/wire.rs`
//! (`revolve_axis_dir_in_plane`): `dir.dot(n)` with `dir` UNIT (wire.rs
//! `unit()` normalizes every datum axis) is a SINE — dimensionless —
//! wrapped in `Length::of` against the linear metre band. This is the
//! audit's class-(c) shape (bare sine vs length band), the same defect
//! family `bool_plane_parallel` etc. were FIXED for, and the same
//! function's OTHER dimensionless comparand (`|θ|−τ`, radians) was
//! honestly flagged as F14. The probe shows the executed consequence:
//! the margin is scale-BLIND — the same tilt classifies identically at
//! 1 mm and 10 m model scale, while the out-of-plane point deviation
//! the gate is supposed to bound grows ×10⁴ across the band.
//!
//! ADOPTED as ledger row F15's pin (PR #213 fix pass): the shipped
//! site now rides `decide_flagged(.., "F15")` — the finding lane, no
//! `Length` constructed — and this probe pins the scale-blindness
//! F15's own unit will retire (the honest form levers the sine at the
//! profile's radial extent, kernel-side). The probe replicates the
//! site's arithmetic locally, so it stays green across that fix too:
//! it asserts the DEFECT's shape, not the site's door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::decide;
use geom_core::{Band, Length, Sign, Vec3};

/// The wire.rs site's exact arithmetic, replicated: a datum axis tilted
/// out of the sketch plane by angle `theta`, gated as the site gates it.
fn site_margin(theta: f64) -> f64 {
    let n = Vec3::new(0.0, 0.0, 1.0_f64);
    // A unit axis direction tilted by theta out of the plane (wire.rs
    // normalizes every datum direction through `unit()`).
    let dir = Vec3::new(theta.cos(), 0.0, theta.sin());
    dir.dot(n) // = sin(theta), dimensionless
}

#[test]
fn revolve_axis_dir_in_plane_margin_is_scale_blind() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    // A tilt whose SINE sits below the band's zero threshold: the site
    // classifies the axis as in-plane...
    let theta = 5e-10_f64;
    let m = site_margin(theta);
    let verdict = decide("probe_axis_dir_in_plane", Length::of(m), band).unwrap();
    assert_eq!(verdict, Sign::Zero, "the sine passes as coincident-zero");
    // ...and the margin is bit-identical whatever the model scale is,
    // because no length enters the comparand at all:
    assert_eq!(site_margin(theta), m, "no scale can appear: it is a sine");
    // But the quantity the ratified convention says must be compared —
    // the point deviation the tilt induces (θ·r, D4 ¶1) — crosses the
    // whole band as the profile's radial extent moves 1 mm → 10 m:
    let dev_small = m * 1e-3; // 5e-13 m: genuinely coincident
    let dev_large = m * 10.0; // 5e-9 m: ABOVE band.zero — not coincident
    assert!(dev_small < band.zero());
    assert!(dev_large > band.zero());
    // So at 10 m extent the gate declares "in plane" an axis whose
    // out-of-plane deviation the same band refuses — the class-(c)
    // defect, reachable through `Length::of` (laundered, not flagged).
}
