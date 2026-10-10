//! The section certificate's test kit shared by its rows and its
//! counterexample searches: the run's band, and the searches' draws.

#![allow(clippy::unwrap_used)]

use geom_core::{Band, Tol, Vec3};
use test_utils::fuzz::Rng;

/// The run's linear band.
pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A uniformly random unit vector.
pub(super) fn unit_vec(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

/// An offset from a margin's zero, relative, either sign: half the
/// draws log-uniform in `[0.03, 0.5]`, half in `[1e-12, 0.5]`, into the
/// band and through it.
pub(super) fn offset(rng: &mut Rng) -> f64 {
    let floor = if rng.unit() < 0.5 { 0.03f64 } else { 1e-12 };
    let m = (rng.range(floor.ln(), 0.5f64.ln())).exp();
    if rng.unit() < 0.5 { m } else { -m }
}
