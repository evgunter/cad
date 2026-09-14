//! SYM-6 review (r2) — the door's slack, measured at the door.
//!
//! `|a − b| ≤ tol.eps() · max(|a|, |b|, 1)` at `f64` and `Probe`, for
//! magnitudes from 0 to 1e12 and a gap of `k · eps · scale` with `k` on
//! both sides of 1; the arm an inexact witness answers is `Disputed`
//! and never `Contradicted`. `Interval`'s meet is exact and ignores the
//! parameter: two enclosures one ULP apart are `Contradicted` at every
//! eps row, where `f64` at the same two values is `Witnessed`. One
//! process per eps row.

use geom_core::sym::SymRegistration;
use geom_core::{Real, Tol};

const MAGNITUDES: [f64; 8] = [0.0, 0.3, 1.0, 1e3, 1e9, -1e9, 1e12, 3.7e11];
const K: [f64; 6] = [0.5, 0.9, 0.99, 1.01, 1.1, 2.0];

/// The slack the spec states, computed independently of the impl.
fn gap(a: f64, k: f64) -> f64 {
    k * Tol::witness().eps() * a.abs().max(1.0)
}

fn check<T: Real + Copy + core::fmt::Debug>(lane: &str, lift: impl Fn(f64) -> T) {
    let tol = Tol::witness();
    for a in MAGNITUDES {
        for k in K {
            let b = a + gap(a, k);
            // The gap the floats actually realised, since `a + gap` rounds.
            let realised = (b - a).abs() / (tol.eps() * a.abs().max(b.abs()).max(1.0));
            let fwd = lift(a).register_equal(lift(b), tol);
            let rev = lift(b).register_equal(lift(a), tol);
            println!("   [{lane}] eps={:e} a={a:e} k={k} realised k={realised:.4}: {fwd:?} / {rev:?}", tol.eps());
            assert_ne!(fwd, SymRegistration::Contradicted, "an inexact witness never answers Contradicted");
            assert_ne!(rev, SymRegistration::Contradicted, "an inexact witness never answers Contradicted");
            assert_eq!(fwd, rev, "the witness is symmetric");
            // Decide by the REALISED ratio so a rounded `a + gap` is not
            // a false red; the printed column shows how close to 1 it sat.
            if realised < 0.999 {
                assert_eq!(fwd, SymRegistration::Witnessed, "a={a:e} k={k}: inside the slack");
            } else if realised > 1.001 {
                assert_eq!(fwd, SymRegistration::Disputed, "a={a:e} k={k}: outside the slack");
            }
        }
    }
}

#[test]
fn sym6_r2_f64_slack_is_the_runs_eps_relative_and_floored() {
    check("f64", |x| x);
}

#[cfg(feature = "probe")]
#[test]
fn sym6_r2_probe_slack_is_f64s() {
    use geom_core::Probe;
    check("Probe", |x| <Probe as Real>::from_f64(x));
}

#[cfg(feature = "interval")]
#[test]
fn sym6_r2_interval_meet_is_exact_and_ignores_tol() {
    use geom_core::Interval;
    let tol = Tol::witness();
    for a in [1.0_f64, 1e9, 1e12] {
        let up = f64::from_bits(a.to_bits() + 1);
        // One ULP apart: inside every eps row's f64 slack, yet disjoint.
        assert_eq!(<f64 as Real>::register_equal(a, up, tol), SymRegistration::Witnessed);
        let point = Interval::from_bounds(a, a);
        let next = Interval::from_bounds(up, up);
        let r = point.register_equal(next, tol);
        println!("   [Interval] eps={:e} [{a:e}] vs [{up:e}]: {r:?}", tol.eps());
        assert_eq!(r, SymRegistration::Contradicted, "disjoint enclosures are a proof, at every eps");
        // Meeting enclosures a geometric amount apart in midpoint: witnessed.
        let wide = Interval::from_bounds(a * 0.9, a * 1.1);
        let other = Interval::from_bounds(a * 1.05, a * 1.2);
        assert_eq!(wide.register_equal(other, tol), SymRegistration::Witnessed);
        // Touching at one endpoint still meets.
        assert_eq!(
            Interval::from_bounds(a * 0.9, a).register_equal(Interval::from_bounds(a, a * 1.1), tol),
            SymRegistration::Witnessed
        );
    }
    for r in [
        Interval::from_bounds(0.0, 1.0).register_equal(Interval::from_bounds(2.0, 3.0), tol),
        Interval::from_bounds(1.0, 1.0).register_equal(Interval::from_bounds(1.0, 1.0), tol),
    ] {
        assert_ne!(r, SymRegistration::Disputed, "an exact witness never answers Disputed");
    }
}
