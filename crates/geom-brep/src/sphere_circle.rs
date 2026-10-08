//! **A circle against a sphere**: a closed-form bound on how far the
//! circle is from the sphere over the whole circle — the incidence term
//! of a sphere's general circle's projected row
//! (`pcurve_cache::projected`).
//!
//! # The circle against the sphere
//!
//! For the carrier `C(t) = c + a·cos t + b·sin t` and the sphere of
//! centre `s` and radius `R`, with `d = c − s`,
//!
//! ```text
//! |C(t) − s|² − R² = k₀ + k₁·cos t + k₂·sin t + k₃·cos 2t + k₄·sin 2t
//! k₀ = |d|² + (|a|² + |b|²)/2 − R²
//! k₁ = 2·d·a     k₂ = 2·d·b     k₃ = (|a|² − |b|²)/2     k₄ = a·b
//! ```
//!
//! exactly, for any `a`, `b` (the second harmonics vanish for an
//! orthonormal frame) — [`off_sphere_coefficients`]. Since
//! `| |p − s| − R | = | |p − s|² − R² | / (|p − s| + R) ≤ | |p − s|² − R² | / R`,
//! each coefficient over the lever `R` is a sound metre reading, inside
//! the sphere and out, and `Σ|kᵢ| / R` bounds the circle's distance from
//! the sphere over the whole circle ([`off_sphere_sup`]): an upper
//! bound, which the projected row's certificate reads. (The chart door's incidence
//! test decides `Off` from a lower bound instead, the sampled distance —
//! `pcurve_cache::chart_incidence`.)

use geom_core::{Real, Vec3};

/// The coefficients `[k₀, k₁, k₂, k₃, k₄]` of `|C(t) − s|² − R²` for
/// `C(t) = s + d + a·cos t + b·sin t` (module docs) — the one home of
/// that expansion.
pub(crate) fn off_sphere_coefficients<T: Real>(
    d: Vec3<T>,
    a: Vec3<T>,
    b: Vec3<T>,
    radius: T,
) -> [T; 5] {
    let two = T::from_f64(2.0);
    let (aa, bb) = (a.dot(a), b.dot(b));
    [
        d.dot(d) + (aa + bb) / two - radius.powi(2),
        two * d.dot(a),
        two * d.dot(b),
        (aa - bb) / two,
        a.dot(b),
    ]
}

/// `Σ|kᵢ| / R`: a bound in metres on `| |C(t) − s| − R |` over the
/// whole circle (module docs).
pub(crate) fn off_sphere_sup<T: Real>(coefficients: [T; 5], radius: T) -> T {
    coefficients
        .into_iter()
        .fold(T::zero(), |sum, k| sum + k.abs())
        / radius
}
