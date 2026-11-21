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
//! the reading's ends, read in exact rationals. At `Sym` the verdict
//! word includes whether the decision was proved, so a door proves
//! exactly what its lifted spelling proved: a form's zero through a
//! product with a bound, nothing through a subtracted one, nothing over a
//! bound alone, whatever its value.

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use super::*;
use crate::interval::Interval;
use crate::real::{Bounds, Real};
use crate::sym::{Sym, SymBudget, with_session};
use crate::test_support::fixture_lever;
use crate::{Dual, k_stats};

use crate::spline::compose::exact_q::Q;

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

/// A verdict as a comparable word: the sign and whether the symbolic
/// tier proved it, or which refusal.
fn word(r: Result<Decided, Indeterminate>) -> String {
    match r {
        Ok(d) => format!("{:?} proven {}", d.sign, d.margin.proven()),
        Err(e) if e.margin.is_invalid() => "invalid".into(),
        Err(e) => format!("undecided(terminal {})", e.terminal_sliver),
    }
}

fn verdict_of<T: Decide, A: MarginTerms>(m: Margin<T, A>) -> String {
    word(k_stats::decide_reported("allowance_rows", m, band()))
}

/// One scalar's rows: for each margin, allowance and door, the new
/// spelling's verdict against the lifted one's. `at` builds the reading
/// at the scalar; `zero` is a reading that is zero as a form (at `Sym`,
/// `x − x`), so a product's proof is pinned too.
fn rows<T: Decide>(label: &str, at: impl Fn(f64) -> T, zero: impl Fn() -> T) {
    let lifted = |x: f64| T::from_computed(x);
    for target in margins() {
        for (scale, less) in allowances() {
            // The reading whose margin lands at `target` (to rounding).
            let r = (target + less) / scale;
            let what = format!("{label}: target {target:e}, scale {scale:e}, less {less:e}");
            assert_eq!(
                verdict_of(Margin::of(at(r)).less(less)),
                verdict_of(Margin::of(at(r) - lifted(less))),
                "less, {what}"
            );
            assert_eq!(
                verdict_of(Margin::of(at(r)).less_floored(less)),
                verdict_of(Margin::of((at(r) - lifted(less)).max(T::zero()))),
                "less_floored, {what}"
            );
            let r = target / scale;
            assert_eq!(
                verdict_of(Margin::levered_at(at(r), fixture_lever(scale))),
                verdict_of(Margin::levered(at(r), fixture_lever(lifted(scale)))),
                "levered_at, {what}"
            );
            assert_eq!(
                verdict_of(Margin::bound_levered(r, fixture_lever(at(scale)))),
                verdict_of(Margin::levered(lifted(r), fixture_lever(at(scale)))),
                "bound_levered, {what}"
            );
            assert_eq!(
                verdict_of(Margin::metered_bound(r, InfSpeed::new(at(scale)))),
                verdict_of(Margin::metered(lifted(r), InfSpeed::new(at(scale)))),
                "metered_bound, {what}"
            );
        }
        assert_eq!(
            verdict_of(Margin::<T>::bound(target)),
            verdict_of(Margin::of(lifted(target))),
            "{label}: bound {target:e}"
        );
        assert_eq!(
            verdict_of(Margin::<T>::bound_of(Margin::of(target))),
            verdict_of(Margin::of(lifted(target))),
            "{label}: bound_of {target:e}"
        );
    }
    // A reading that is zero as a form, through each door, the bound
    // exactly zero or one included: what the lifted spelling proved.
    for b in [0.0, -0.0, 1.0, 2.5] {
        let what = format!("{label}: a form-zero reading, bound {b:e}");
        assert_eq!(
            verdict_of(Margin::of(zero()).less(b)),
            verdict_of(Margin::of(zero() - lifted(b))),
            "less, {what}"
        );
        assert_eq!(
            verdict_of(Margin::of(zero()).less_floored(b)),
            verdict_of(Margin::of((zero() - lifted(b)).max(T::zero()))),
            "less_floored, {what}"
        );
        assert_eq!(
            verdict_of(Margin::levered_at(zero(), fixture_lever(b))),
            verdict_of(Margin::levered(zero(), fixture_lever(lifted(b)))),
            "levered_at, {what}"
        );
        assert_eq!(
            verdict_of(Margin::bound_levered(b, fixture_lever(zero()))),
            verdict_of(Margin::levered(lifted(b), fixture_lever(zero()))),
            "bound_levered, {what}"
        );
        assert_eq!(
            verdict_of(Margin::metered_bound(b, InfSpeed::new(zero()))),
            verdict_of(Margin::metered(lifted(b), InfSpeed::new(zero()))),
            "metered_bound, {what}"
        );
        assert_eq!(
            verdict_of(Margin::<T>::bound(b)),
            verdict_of(Margin::of(lifted(b))),
            "bound, {what}"
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
    rows::<f64>("f64", |x| x, || 0.0);
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_interval() {
    rows::<Interval>("interval point", Interval::from_computed, Interval::zero);
    // A reading wide enough to straddle an edge: the allowance widens
    // it exactly as the lifted operand's interval arithmetic did.
    rows::<Interval>(
        "interval wide",
        |x| {
            let w = 0.25 * ZERO;
            Interval::from_bounds(x - w, x + w)
        },
        Interval::zero,
    );
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_a_dual() {
    rows::<Dual<f64>>("dual", |x| Dual::new(x, 1.0), || Dual::new(0.0, 1.0));
}

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 128,
    }
}

#[test]
fn an_allowance_decides_as_the_lifted_spelling_did_at_sym() {
    let ((), _) = with_session(budget(), || {
        rows::<Sym<f64>>("sym", Sym::from_computed, || {
            let x = Sym::<f64>::from_f64(0.3);
            x - x
        });
    });
}

fn proven(m: Result<Decided, Indeterminate>) -> bool {
    m.unwrap().margin.proven()
}

/// **A bound that is exactly zero is still a bound**: no `Zero` is
/// proved over it, through any door that takes one, as none was over the
/// lifted unknown.
#[test]
fn an_exact_zero_bound_proves_nothing_at_sym() {
    let ((bound, less, of), _) = with_session(budget(), || {
        let x = Sym::<f64>::from_f64(0.3);
        let zero = x - x;
        (
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::<Sym<f64>>::bound(0.0),
                band(),
            )),
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::of(zero).less(0.0),
                band(),
            )),
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::<Sym<f64>>::bound_of(Margin::of(0.0)),
                band(),
            )),
        )
    });
    assert!(
        !bound && !less && !of,
        "bound {bound}, less {less}, bound_of {of}"
    );
}

/// **A form-zero reading levered at a bound arm is proved**, as the
/// lifted arm's product proved it.
#[test]
fn a_levered_form_zero_at_sym_is_proved_as_the_lifted_arm_was() {
    let ((lifted, levered), _) = with_session(budget(), || {
        let x = Sym::<f64>::from_f64(0.3);
        let zero = x - x;
        (
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::levered(zero, fixture_lever(Sym::from_computed(2.5))),
                band(),
            )),
            proven(k_stats::decide_reported(
                "allowance_rows",
                Margin::levered_at(zero, fixture_lever(2.5)),
                band(),
            )),
        )
    });
    assert!(lifted, "the lifted arm's product is proved zero");
    assert_eq!(
        levered, lifted,
        "levered_at proves what the lifted arm proved"
    );
}

/// **At `Interval` the applied bound encloses the exact margin**:
/// `r · scale` and `r − less` for `r` at either end of the reading, in
/// exact rationals, lie in the enclosure the decision reads, over
/// readings, scales of either sign and magnitude and subtracted bounds.
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
    let bounds = [1.0, 0.1, -0.7, 1.0 / 3.0, 1e-20, 3e15, 1e-30, 2.5e16];
    let encloses = |got: Interval, exact: &dyn Fn(f64) -> Q, (lo, hi): (f64, f64), what: &str| {
        let (glo, ghi) = (got.lo(), got.hi());
        for end in [exact(lo), exact(hi)] {
            assert!(
                end.cmp_f64(glo) != core::cmp::Ordering::Less
                    && end.cmp_f64(ghi) != core::cmp::Ordering::Greater,
                "{what} over [{lo:e}, {hi:e}]: [{glo:e}, {ghi:e}] misses {:e}",
                end.to_f64_report()
            );
        }
    };
    for (lo, hi) in readings {
        let reading = Interval::from_bounds(lo, hi);
        for b in bounds {
            let scaled = Margin::levered_at(reading, fixture_lever(b));
            encloses(
                scaled.allowance.apply(scaled.reading),
                &|r| Q::from_f64(r).mul(&Q::from_f64(b)),
                (lo, hi),
                &format!("· {b:e}"),
            );
            let less = Margin::of(reading).less(b);
            encloses(
                less.allowance.apply(less.reading),
                &|r| Q::from_f64(r).sub(&Q::from_f64(b)),
                (lo, hi),
                &format!("− {b:e}"),
            );
        }
    }
}

/// **A dual's scaled bound scales its tangent, and a subtracted one
/// leaves it**: the bound is not a function of the inputs.
#[test]
fn a_duals_tangent_is_scaled_by_a_bound_and_not_shifted() {
    let scaled = Margin::levered_at(Dual::new(0.25, 2.0), fixture_lever(4.0));
    let got = scaled.allowance.apply(scaled.reading);
    assert_eq!((got.value, got.deriv), (1.0, 8.0), "scaled");
    let less = Margin::of(Dual::new(0.25, 2.0)).less(0.5);
    let got = less.allowance.apply(less.reading);
    assert_eq!(
        (got.value, got.deriv),
        (-0.25, 2.0),
        "shifted value, tangent kept"
    );
}
