//! **A bound carried as a margin's allowance decides as the lifted
//! spelling did, and encloses what it stands for** (the crate README,
//! DL5.2; [`Allowance`]).
//!
//! Each door that takes an `f64` bound is driven against the spelling it
//! replaces, the bound lifted through [`Real::from_computed`] and the
//! margin formed in `T`, over margins placed on and either side of the
//! band's two edges, at `f64`, `Interval` (a point reading and a wide
//! one), `Sym<f64>` and `Dual<f64>`. The verdict (the sign, or the kind
//! of refusal) must be the one the lifted spelling gave. At `Interval`
//! the applied allowance must enclose the exact `r · scale − less` over
//! the reading's ends, read in exact rationals. At `Sym` no identity is
//! proved over a margin that carries a bound, as none was over the lifted
//! one.

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use super::*;
use crate::interval::Interval;
use crate::real::{Bounds, Real};
use crate::sym::{Sym, SymBudget, with_session};
use crate::test_support::fixture_lever;
use crate::{Dual, k_stats};

#[path = "../../tests/exact_q/mod.rs"]
mod exact_q;
use exact_q::Q;

const ZERO: f64 = 1e-9;
const ESCALATE: f64 = 1e-8;

fn band() -> Band {
    Band::new(ZERO, ESCALATE).unwrap()
}

/// The margins each row places: on, and an ulp either side of, both
/// band edges on both sides, the origin, and two far from the band.
fn margins() -> Vec<f64> {
    let mut out = vec![0.0, -0.0, 1e-3, -1e-3, 0.5 * ZERO, 3.0 * ZERO];
    for edge in [ZERO, ESCALATE] {
        for m in [edge, edge.next_up(), edge.next_down()] {
            out.push(m);
            out.push(-m);
        }
    }
    out
}

/// The allowances a row is formed with: `(scale, less)`.
fn allowances() -> Vec<(f64, f64)> {
    vec![
        (1.0, 0.0),
        (1.0, 2.5e-9),
        (1.0, -4e-9),
        (0.5, 0.0),
        (3.0, 1e-9),
        (1e3, -7e-9),
        (1e-3, 0.0),
    ]
}

/// A verdict as a comparable word: the sign, or which refusal.
fn word(r: Result<Decided, Indeterminate>) -> String {
    match r {
        Ok(d) => format!("{:?}", d.sign),
        Err(e) if e.margin.is_invalid() => "invalid".into(),
        Err(e) => format!("undecided(terminal {})", e.terminal_sliver),
    }
}

fn verdict_of<T: Decide, A: MarginTerms>(m: Margin<T, A>) -> String {
    word(k_stats::decide_reported("allowance_rows", m, band()))
}

/// One scalar's rows: for each margin, allowance and door, the new
/// spelling's verdict against the lifted one's. `at` builds the
/// reading at the scalar.
fn rows<T: Decide>(label: &str, at: impl Fn(f64) -> T) {
    for target in margins() {
        for (scale, less) in allowances() {
            // The reading whose margin lands at `target` (to rounding).
            let r = (target + less) / scale;
            let lifted = |x: f64| T::from_computed(x);
            let what = format!("{label}: target {target:e}, scale {scale:e}, less {less:e}");
            // `less` on a margin read in `T`.
            assert_eq!(
                verdict_of(Margin::of(at(r)).less(less)),
                verdict_of(Margin::of(at(r) - lifted(less))),
                "less, {what}"
            );
            // An arm read as a bound, then a bound subtracted.
            let arm = fixture_lever(scale);
            assert_eq!(
                verdict_of(Margin::levered_at(at(r), arm).less(less)),
                verdict_of(Margin::of(at(r) * lifted(scale) - lifted(less))),
                "levered_at, {what}"
            );
            // A dimensionless bound at an arm read in `T`.
            assert_eq!(
                verdict_of(Margin::bound_levered(r, fixture_lever(at(scale)))),
                verdict_of(Margin::levered(lifted(r), fixture_lever(at(scale)))),
                "bound_levered, {what}"
            );
            // A span read as a bound, metered at a rate read in `T`.
            assert_eq!(
                verdict_of(Margin::metered_bound(r, InfSpeed::new(at(scale)))),
                verdict_of(Margin::metered(lifted(r), InfSpeed::new(at(scale)))),
                "metered_bound, {what}"
            );
        }
        // A bound with no `T` operand.
        assert_eq!(
            verdict_of(Margin::<T>::bound(target)),
            verdict_of(Margin::of(T::from_computed(target))),
            "{label}: bound {target:e}"
        );
        assert_eq!(
            verdict_of(Margin::of(target).at_scalar::<T>()),
            verdict_of(Margin::of(T::from_computed(target))),
            "{label}: at_scalar {target:e}"
        );
    }
    // Poison stays poison.
    assert_eq!(
        verdict_of(Margin::<T>::bound(f64::NAN)),
        "invalid",
        "{label}: a NaN bound"
    );
    assert_eq!(
        verdict_of(Margin::of(at(1.0)).less(f64::NAN)),
        "invalid",
        "{label}: a NaN allowance"
    );
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_f64() {
    rows::<f64>("f64", |x| x);
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_interval() {
    rows::<Interval>("interval point", Interval::from_computed);
    // A reading wide enough to straddle an edge: the allowance widens
    // it exactly as the lifted operand's interval arithmetic did.
    rows::<Interval>("interval wide", |x| {
        let w = 0.25 * ZERO;
        Interval::from_bounds(x - w, x + w)
    });
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_a_dual() {
    rows::<Dual<f64>>("dual", |x| Dual::new(x, 1.0));
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_sym() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let ((), _) = with_session(budget, || rows::<Sym<f64>>("sym", Sym::from_computed));
}

/// **At `Sym` a margin that carries a bound is never proved zero**: a
/// reading that is zero as a form decides `Zero` proved where it is the
/// whole margin, and unproved once a bound is subtracted from it, as it
/// was when the bound entered as an unknown.
#[test]
fn no_identity_is_proved_over_a_margin_that_carries_a_bound() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let proven = |m: Result<Decided, Indeterminate>| m.unwrap().margin.proven();
    let ((plain, allowed, lifted), _) = with_session(budget, || {
        let x = Sym::<f64>::from_f64(0.3);
        let zero = x - x;
        let pad = 1e-12;
        (
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::of(zero),
                band(),
            )),
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::of(zero).less(pad),
                band(),
            )),
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::of(zero - Sym::from_computed(pad)),
                band(),
            )),
        )
    });
    assert!(plain, "the form's own zero is proved");
    assert!(!allowed, "no zero is proved over a subtracted bound");
    assert_eq!(allowed, lifted, "as over the lifted bound");
}

/// **At `Interval` the applied allowance encloses the exact margin**:
/// `r · scale − less` for `r` at either end of the reading, in exact
/// rationals, lies in the enclosure the decision reads, over readings,
/// scales of either sign and magnitude and subtracted bounds whose
/// rounding the product and the difference must each widen for.
#[test]
fn an_interval_allowance_encloses_the_exact_margin() {
    let readings = [
        (0.1, 0.1),
        (-0.3, 0.7),
        (1.0 / 3.0, 2.0 / 3.0),
        (1e-300, 1e-290),
        (-1e17, 1e17 + 2.0),
        (0.0, 0.0),
    ];
    let scales = [1.0, 0.1, -0.7, 1.0 / 3.0, 1e-20, 3e15];
    let lesses = [0.0, 0.1, -1.0 / 7.0, 1e-30, 2.5e16];
    for (lo, hi) in readings {
        for scale in scales {
            for less in lesses {
                let m = Margin::levered_at(Interval::from_bounds(lo, hi), fixture_lever(scale))
                    .less(less);
                let (reading, allowance) = m.parts();
                let got = reading.allowing(allowance);
                let (glo, ghi) = (got.lo(), got.hi());
                let exact = |r: f64| {
                    Q::from_f64(r)
                        .mul(&Q::from_f64(scale))
                        .sub(&Q::from_f64(less))
                };
                for end in [exact(lo), exact(hi)] {
                    assert!(
                        end.cmp_f64(glo) != core::cmp::Ordering::Less
                            && end.cmp_f64(ghi) != core::cmp::Ordering::Greater,
                        "[{lo:e}, {hi:e}] · {scale:e} − {less:e}: [{glo:e}, {ghi:e}] misses {:e}",
                        end.to_f64_report()
                    );
                }
            }
        }
    }
}

/// **A dual's allowance scales its tangent and leaves its value the
/// `f64` margin's**: the bound is not a function of the inputs.
#[test]
fn a_duals_allowance_scales_its_tangent() {
    let m = Margin::levered_at(Dual::new(0.25, 2.0), fixture_lever(4.0)).less(0.5);
    let (reading, allowance) = m.parts();
    let got = reading.allowing(allowance);
    assert_eq!(got.value, 0.25f64.allowing(allowance), "the value channel");
    assert_eq!(got.deriv, 8.0, "the tangent, scaled and not shifted");
}
