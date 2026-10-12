//! **`Real::sum_rounded_once` at every scalar**: a function of the
//! terms' multiset, the interval's ends enclosing the exact sum and
//! never wider than the sequential sum's, the dual's channels each the
//! base scalar's, and the symbolic lane's value the plain lane's, with
//! a DAG that re-values to it.

use geom_core::exact::{Rounding, sum_rounded};
use geom_core::sym::with_session;
use geom_core::{Bounds, Dual64, Interval, ParamSymbol, Real, Sym, SymBudget};

test_utils::gated_to![
    "crates/geom-core/src/exact.rs",
    "crates/geom-core/src/real.rs"
];

/// Terms whose sequential sums round apart by order: magnitudes across
/// twenty binades and both signs.
fn terms(rng: &mut test_utils::fuzz::Rng) -> Vec<f64> {
    (0..2 + rng.below(12))
        .map(|_| {
            let x = rng.log_uniform(1e-10, 1e10);
            if rng.below(2) == 0 { x } else { -x }
        })
        .collect()
}

/// Counterexample search: at the interval scalar, every permutation
/// gives the same ends, the ends enclose the exact sum of the lower
/// (upper) ends, and they are never wider than the sequential sum's.
#[test]
fn the_interval_sum_is_order_free_enclosing_and_no_wider() {
    let mut rng = test_utils::fuzz::start("sum_rounded_once::interval");
    for _ in 0..test_utils::fuzz::scaled(2000) {
        let lo = terms(&mut rng);
        let xs: Vec<Interval> = lo
            .iter()
            .map(|&l| Interval::from_bounds(l, l + rng.range(0.0, 1e-3) * l.abs()))
            .collect();
        let sum = Interval::sum_rounded_once(&xs);
        let sequential = xs.iter().fold(Interval::zero(), |a, &x| a + x);
        let los: Vec<f64> = xs.iter().map(|x| x.lo()).collect();
        let his: Vec<f64> = xs.iter().map(|x| x.hi()).collect();
        assert_eq!(
            (sum.lo().to_bits(), sum.hi().to_bits()),
            (
                sum_rounded(&los, Rounding::Down).to_bits(),
                sum_rounded(&his, Rounding::Up).to_bits()
            ),
            "the ends are the exact sums rounded outward; {}",
            test_utils::fuzz::replay()
        );
        assert!(
            sequential.lo() <= sum.lo() && sum.hi() <= sequential.hi(),
            "never wider than the sequential sum: {sum:?} against {sequential:?}; {}",
            test_utils::fuzz::replay()
        );
        let mut turned = xs.clone();
        turned.reverse();
        let k = rng.below(turned.len());
        turned.rotate_left(k);
        let again = Interval::sum_rounded_once(&turned);
        assert_eq!(
            (again.lo().to_bits(), again.hi().to_bits()),
            (sum.lo().to_bits(), sum.hi().to_bits()),
            "order-free; {}",
            test_utils::fuzz::replay()
        );
    }
}

/// Poison propagates as addition's does: NaI over empty over a value.
#[test]
fn the_interval_sum_propagates_poison_as_addition_does() {
    let one = Interval::from_f64(1.0);
    // A wholly negative square root is the empty enclosure.
    let empty = Interval::from_bounds(-2.0, -1.0).sqrt();
    let nai = Interval::from_f64(f64::NAN);
    assert!(
        Interval::sum_rounded_once(&[one, nai, empty]).is_poison(),
        "NaI poisons"
    );
    assert!(
        Interval::sum_rounded_once(&[one, empty]).is_poison(),
        "empty poisons"
    );
    let none = Interval::sum_rounded_once(&[]);
    assert_eq!(
        (none.lo(), none.hi()),
        (0.0, 0.0),
        "the empty sum is the point zero"
    );
}

/// The dual's two channels are each `f64`'s rounded-once sum.
#[test]
fn the_dual_sum_is_componentwise() {
    let mut rng = test_utils::fuzz::start("sum_rounded_once::dual");
    for _ in 0..test_utils::fuzz::scaled(500) {
        let (v, d) = (terms(&mut rng), terms(&mut rng));
        let n = v.len().min(d.len());
        let xs: Vec<Dual64> = (0..n)
            .map(|i| Dual64 {
                value: v[i],
                deriv: d[i],
            })
            .collect();
        let s = Dual64::sum_rounded_once(&xs);
        assert_eq!(
            (s.value.to_bits(), s.deriv.to_bits()),
            (
                f64::sum_rounded_once(&v[..n]).to_bits(),
                f64::sum_rounded_once(&d[..n]).to_bits()
            ),
            "{}",
            test_utils::fuzz::replay()
        );
    }
}

/// The symbolic lane's value is the plain lane's bits (its DAG's
/// re-valuation is `topo`'s row, where the instrument is built).
#[test]
fn the_symbolic_sum_keeps_the_plain_lanes_bits() {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let coefficients = [1.0e8, 0.1, -1.0e8, 3.0e-9, 0.7];
    let plain = f64::sum_rounded_once(&coefficients.map(|c| c * 0.3));
    let (bits, _) = with_session(budget, || {
        let x = Sym::<f64>::param(ParamSymbol::new(7), 0.3);
        Sym::sum_rounded_once(&coefficients.map(|c| Sym::from_f64(c) * x))
            .value
            .to_bits()
    });
    assert_eq!(bits, plain.to_bits(), "the plain lane's bits");
}
