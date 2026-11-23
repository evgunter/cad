//! SCRATCH, branch `oneround/conic-harmonic-probe`: never merges.
//!
//! The switch the conic-harmonic measurement flips, and its log.
//! Variant 0 is the branch as it stands; 1 takes a first harmonic's
//! constant from the stored point and leaves the solve; 2 also poses the
//! root about the stored point.

use std::cell::{Cell, RefCell};

thread_local! {
    static VARIANT: Cell<u8> = const { Cell::new(0) };
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// The variant in force on this thread.
#[must_use]
pub fn variant() -> u8 {
    VARIANT.with(Cell::get)
}

/// `f` run under variant `v`.
pub fn with_variant<R>(v: u8, f: impl FnOnce() -> R) -> R {
    let old = VARIANT.with(|c| c.replace(v));
    let out = f();
    VARIANT.with(|c| c.set(old));
    out
}

/// A line for the probe's log.
pub fn log(line: String) {
    LOG.with(|l| l.borrow_mut().push(line));
}

/// The log so far, emptied.
#[must_use]
pub fn take_log() -> Vec<String> {
    LOG.with(|l| core::mem::take(&mut *l.borrow_mut()))
}

/// **The two roots of `h + p·(cos θ − k_c) + q·(sin θ − k_s)`** for a
/// unit pair `(k_c, k_s)`, posed about the stored point: with
/// `φ = θ − θ₀`, `h − 2P·sin²(φ/2) + Q·sin φ`, a quadratic in
/// `tan(φ/2)` solved without cancellation. Each root as the pair
/// `(sin θ, cos θ)` up to a positive scale.
#[must_use]
pub fn offset_roots<T: geom_core::Real>(h: T, p: T, q: T, (kc, ks): (T, T)) -> [(T, T); 2] {
    let two = T::from_f64(2.0);
    let (pp, qq) = (p * kc + q * ks, q * kc - p * ks);
    let (a2, c) = (h - two * pp, h);
    let disc = (qq.powi(2) - a2 * c).max(T::zero()).sqrt();
    let w = T::zero() - (qq + qq.select_le_zero(T::zero() - disc, disc));
    // t₁ = w/a2, t₂ = c/w; (sin φ, cos φ) ∝ (2·num·den, den² − num²).
    let dir = |num: T, den: T| (two * num * den, den.powi(2) - num.powi(2));
    let turn = |(y, x): (T, T)| (y * kc + x * ks, x * kc - y * ks);
    [turn(dir(w, a2)), turn(dir(c, w))]
}
