//! **The sup of one harmonic pair**: `sup_t ‖a·cos t + b·sin t‖` in
//! closed form, for a vector pair ([`harmonic_sup`]) and a scalar one
//! ([`harmonic_sup_scalar`]) — the one home of both, read wherever a
//! bound meets one `cos t`/`sin t` pair (check 4's envelope terms and
//! class gates in `pcurve_cache`, the off-sphere bound in
//! `sphere_circle`, the cone conic's least reach in `implicit`).
//!
//! Each closed form is the sup exactly, in ℝ. Every step is one of the
//! scalar's own operations, so at `Interval` the result encloses the
//! sup; the squares go through `norm_squared` and `powi(2)`, whose
//! enclosures stay non-negative, so a pair whose enclosure straddles
//! zero never asks `sqrt` of a negative and poisons nothing. At `f64`
//! the result carries its own rounding, as every `f64` bound does; a
//! pair below `√(f64::MIN_POSITIVE)` squares into the subnormals and
//! loses relative precision there, as `norm` does.

use geom_core::{Real, Vec3};

/// `sup_t ‖a·cos t + b·sin t‖`, exactly: the largest singular value of
/// `[a b]`, `√(m + √(q² + (a·b)²))` with `m = (‖a‖² + ‖b‖²)/2` and
/// `q = (‖a‖² − ‖b‖²)/2`.
pub(crate) fn harmonic_sup<T: Real>(a: Vec3<T>, b: Vec3<T>) -> T {
    #[cfg(test)]
    if let Some(mutant) = mutant::vector(a, b) {
        return mutant;
    }
    let two = T::one() + T::one();
    let (aa, bb) = (a.norm_squared(), b.norm_squared());
    let (m, q) = ((aa + bb) / two, (aa - bb) / two);
    (m + (q.powi(2) + a.dot(b).powi(2)).sqrt()).sqrt()
}

/// `sup_t |a·cos t + b·sin t| = √(a² + b²)`, exactly.
pub(crate) fn harmonic_sup_scalar<T: Real>(a: T, b: T) -> T {
    #[cfg(test)]
    if let Some(mutant) = mutant::scalar(a, b) {
        return mutant;
    }
    (a.powi(2) + b.powi(2)).sqrt()
}

/// Wrong forms of the two helpers, one of which a test build takes in
/// place of the closed form when `PCERT_HARMONIC_SUM_MUTANT` names it
/// at compile time: `vector` / `scalar` the coefficient sum this module
/// replaced (an over-read), `vector-max` / `scalar-max` the larger
/// coefficient, `vector-rms` / `scalar-rms` the root mean square,
/// `vector-inner` the vector form without its inner root, `vector-dot`
/// it without `a·b` (each an under-read), and `square-mul` both forms
/// squaring by `x * x`. The rows that refuse them are in
/// `pcurve_cache::harmonic_sup_rows`. The branch sits in the helpers
/// because the rows reach them only through the production paths, and
/// `cfg(test)` compiles it out of every other build; the read is
/// `option_env!` because `scripts/gates/no-ambient-env.sh` forbids a
/// runtime one in `crates/*/src`.
#[cfg(test)]
mod mutant {
    use geom_core::{Real, Vec3};

    const NAME: Option<&str> = option_env!("PCERT_HARMONIC_SUM_MUTANT");

    pub(super) fn vector<T: Real>(a: Vec3<T>, b: Vec3<T>) -> Option<T> {
        let two = T::one() + T::one();
        let (aa, bb, ab) = (a.norm_squared(), b.norm_squared(), a.dot(b));
        let (m, q) = ((aa + bb) / two, (aa - bb) / two);
        match NAME? {
            "vector" => Some(a.norm() + b.norm()),
            "vector-max" => Some(a.norm().max(b.norm())),
            "vector-rms" => Some(m.sqrt()),
            "vector-inner" => Some((m + q.powi(2) + ab.powi(2)).sqrt()),
            "vector-dot" => Some((m + q.abs()).sqrt()),
            "square-mul" => Some((m + (q * q + ab * ab).sqrt()).sqrt()),
            _ => None,
        }
    }

    pub(super) fn scalar<T: Real>(a: T, b: T) -> Option<T> {
        let two = T::one() + T::one();
        match NAME? {
            "scalar" => Some(a.abs() + b.abs()),
            "scalar-max" => Some(a.abs().max(b.abs())),
            "scalar-rms" => Some(((a.powi(2) + b.powi(2)) / two).sqrt()),
            "square-mul" => Some((a * a + b * b).sqrt()),
            _ => None,
        }
    }
}
