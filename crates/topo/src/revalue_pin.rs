//! **The per-site laundering pin**: a site's computation built at
//! `Sym<f64>` twice, with one parameter at `p0` and at `p1`, its `p0`
//! DAG re-valued at `p1` (`geom_core::sym::revalue`) and compared bit
//! for bit with what the `p1` build took — the corpus row
//! (`editor-core`'s `revalue_corpus.rs`) at the scale of one site.
//!
//! The build receives the parameter's `f64` value beside its symbol,
//! so it can compute in `f64` what the kernel's numerical routine
//! computes from it (a cell of a window, a quadrature's pad), and hand
//! that to the site: a site that re-enters it as a constant re-values
//! to where `p0` put it, and is counted laundered.
#![allow(clippy::expect_used)]

use geom_core::sym::revalue::{record, revalue};
use geom_core::sym::with_session;
use geom_core::{ParamSymbol, Sym, SymBudget};

/// The scalar the pins build at.
pub(crate) type S = Sym<f64>;

/// What one pin's comparison found, over the decisions the build took
/// and the values it returned.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Revalued {
    /// Re-valued to the `p1` build's own bits.
    pub(crate) kept: usize,
    /// Reached an opaque value: no claim made at `p1`.
    pub(crate) opaque: usize,
    /// Re-valued away from the `p1` build: a constant that should have
    /// moved.
    pub(crate) laundered: usize,
}

/// `build(p, x)` at `p0` and at `p1`, where `x` is the parameter bound
/// at `p`; it returns the values to compare beside the decisions.
///
/// # Panics
///
/// When the two builds take different decisions, or return different
/// numbers of values: the move changed a branch, and the pin compares
/// nothing.
pub(crate) fn revalued(p: [f64; 2], build: impl Fn(f64, S) -> Vec<S>) -> Revalued {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    revalued_at(budget, p, build)
}

/// [`revalued`] at `budget`: at zero the tier proves nothing and every
/// decision is the numeric one, so a build reaches the arms a symbolic
/// `Zero` would have short-cut.
pub(crate) fn revalued_at(
    budget: SymBudget,
    p: [f64; 2],
    build: impl Fn(f64, S) -> Vec<S>,
) -> Revalued {
    let symbol = ParamSymbol::new(1);
    let run = |at: f64| record(|| build(at, S::param(symbol, at)));
    let ((values1, decided1), _) = with_session(budget, || {
        let (out, decided) = run(p[1]);
        let values: Vec<f64> = out.iter().map(|x| x.value).collect();
        (values, decided)
    });
    let (found, _) = with_session(budget, || {
        let (out, decided) = run(p[0]);
        let names = |d: &[geom_core::sym::revalue::Decision]| -> Vec<&'static str> {
            d.iter().map(|x| x.predicate).collect()
        };
        assert_eq!(
            names(&decided),
            names(&decided1),
            "the two builds took different decisions: the move changed a branch"
        );
        assert_eq!(
            out.len(),
            values1.len(),
            "the two builds returned different values"
        );
        let at = |q: f64| move |s: ParamSymbol| (s == symbol).then_some(q);
        let same = |x: f64, y: f64| x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan());
        let value = |d: &geom_core::sym::revalue::Decision| {
            d.value.expect("Sym<f64> values every decision")
        };
        let mut found = Revalued::default();
        let nodes = decided
            .iter()
            .map(|d| (d.node, value(d)))
            .chain(out.iter().map(|x| (x.node(), x.value)));
        let wants = decided1.iter().map(value).chain(values1.iter().copied());
        for ((node, own), want) in nodes.zip(wants) {
            assert!(
                revalue(node, &at(p[0])).is_none_or(|r| same(r, own)),
                "a DAG re-valued at p0 is not the p0 build's own bits: revalue no longer \
                 computes an op as Sym builds it"
            );
            match revalue(node, &at(p[1])) {
                None => found.opaque += 1,
                Some(r) if same(r, want) => found.kept += 1,
                Some(_) => found.laundered += 1,
            }
        }
        found
    });
    found
}
