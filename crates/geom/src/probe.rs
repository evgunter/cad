//! SCRATCH, branch `oneround/conic-harmonic-probe`: never merges.
//!
//! The switch the conic-harmonic measurement flips, and its log.
//! Variant 0 is the branch as it stands; 1 takes a first harmonic's
//! constant from the stored point and leaves the solve; 2 also poses the
//! root about the stored point.

use std::cell::{Cell, RefCell};

/// The process's default variant: `$CONIC_PROBE_MODE`, else 0. M4 runs
/// whole suites under mode 6: every conic, centre-held ones too, read
/// through the scratch polynomial (first-degree roots as the quadratic
/// in `tan(θ/2)` about the stored point; a sphere's or a wall's residual
/// as the offsets jet).
fn default_mode() -> u8 {
    static MODE: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *MODE.get_or_init(|| {
        std::env::var("CONIC_PROBE_MODE")
            .ok()
            .and_then(|m| m.parse().ok())
            .unwrap_or(0)
    })
}

/// M4's record of a door's call: appended to
/// `$CONIC_PROBE_LOG/<pid>.log` as `test \t door \t input \t output`.
pub fn record(door: &str, input: impl FnOnce() -> String, output: impl FnOnce() -> String) {
    use std::io::Write;
    let Ok(dir) = std::env::var("CONIC_PROBE_LOG") else {
        return;
    };
    let name = std::thread::current().name().unwrap_or("?").to_owned();
    let path = format!("{dir}/{}.log", std::process::id());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{name}\t{door}\t{}\t{}", input(), output());
    }
}

thread_local! {
    static VARIANT: Cell<u8> = Cell::new(default_mode());
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
    let dir = |num: T, den: T| (two * num * den, den.powi(2) - num.powi(2));
    let solve = |a2: T, b: T, c: T| {
        let disc = (b.powi(2) - a2 * c).max(T::zero()).sqrt();
        let w = T::zero() - (b + b.select_le_zero(T::zero() - disc, disc));
        // t₁ = w/a2, t₂ = c/w; (sin, cos) ∝ (2·num·den, den² − num²).
        [dir(w, a2), dir(c, w)]
    };
    if (kc.abs() + ks.abs()).is_exact_zero_in_every_channel() {
        // Held at its centre: `h + p·cos θ + q·sin θ`, the quadratic
        // `(h − p)·t² + 2q·t + (h + p)` in `t = tan(θ/2)`.
        return solve(h - p, q, h + p);
    }
    let (pp, qq) = (p * kc + q * ks, q * kc - p * ks);
    let turn = |(y, x): (T, T)| (y * kc + x * ks, x * kc - y * ks);
    solve(h - two * pp, qq, h).map(turn)
}
