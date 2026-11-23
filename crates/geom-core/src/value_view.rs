//! **A geometry without its tangent** (DL5.2's place row, the crate
//! README, "Reading a bracket where nothing certifies").
//!
//! A place is an `f64` the code chose to evaluate at. It is handed to an
//! evaluator as an `f64`, and the evaluator answers at a scalar that
//! carries no tangent ([`Real::Value`]): `geom`'s place-taking
//! evaluators exist only there. A geometry held at a dual is first read
//! without its tangent, through the family it belongs to
//! ([`OverScalar`]), into a [`ValueView`]: the geometry itself where its
//! scalar carries no tangent, a copy at a dual.

use core::ops::Deref;

use crate::real::Real;

/// **A geometry type over a scalar**, as a family: `At<S>` is the type
/// at scalar `S`, and [`OverScalar::map`] carries one to another through
/// a map of every scalar it holds. It is how [`Real::view`] names the
/// type it reads without its tangent while `geom_core` does not know it.
pub trait OverScalar {
    /// The type at scalar `S`.
    type At<S: Real>;

    /// `x` with `f` applied to every scalar it holds, its structure
    /// carried over. A view is bit-identical to what a dual's value part
    /// computes only if `f` reaches every scalar and nothing else
    /// changes.
    fn map<S: Real, U: Real>(x: &Self::At<S>, f: impl Fn(S) -> U) -> Self::At<U>;
}

/// **A value without its tangent** ([`Real::view`]): the value itself
/// where its scalar carries none, a copy otherwise. Either way it reads
/// as the type at [`Real::Value`].
#[derive(Debug)]
pub enum ValueView<'a, X> {
    /// The value itself: its scalar carries no tangent.
    Same(&'a X),
    /// A copy with every tangent removed.
    Stripped(X),
}

impl<X> Deref for ValueView<'_, X> {
    type Target = X;

    fn deref(&self) -> &X {
        match self {
            Self::Same(x) => x,
            Self::Stripped(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OverScalar, ValueView};
    use crate::real::{Bounds, Real};
    use crate::{Dual64, DualInterval, Interval};

    /// One `Vec` of scalars, mapped elementwise.
    struct Row;

    impl OverScalar for Row {
        type At<S: Real> = Vec<S>;

        fn map<S: Real, U: Real>(x: &Vec<S>, f: impl Fn(S) -> U) -> Vec<U> {
            x.iter().map(|&s| f(s)).collect()
        }
    }

    /// Edge floats: signed zeros, a subnormal, the largest, a
    /// non-dyadic fraction, the infinities and NaN.
    const EDGES: [f64; 9] = [
        0.0,
        -0.0,
        f64::MIN_POSITIVE / 4.0,
        f64::MAX,
        0.1,
        -2.5,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ];

    #[test]
    fn a_view_borrows_where_the_scalar_carries_no_tangent_and_strips_one() {
        let plain: Vec<f64> = EDGES.to_vec();
        assert!(
            matches!(f64::view::<Row>(&plain), ValueView::Same(v) if core::ptr::eq(v, &plain)),
            "f64 borrows"
        );
        let enclosed: Vec<Interval> = EDGES.iter().map(|&x| Interval::from_f64(x)).collect();
        assert!(
            matches!(Interval::view::<Row>(&enclosed), ValueView::Same(v) if core::ptr::eq(v, &enclosed)),
            "Interval borrows"
        );
        #[cfg(feature = "probe")]
        {
            let probes: Vec<crate::Probe> = EDGES.iter().map(|&x| crate::Probe(x)).collect();
            assert!(
                matches!(crate::Probe::view::<Row>(&probes), ValueView::Same(v) if core::ptr::eq(v, &probes)),
                "the probe borrows"
            );
        }
        let dual: Vec<Dual64> = EDGES.iter().map(|&x| Dual64::new(x, 3.0)).collect();
        let view = Dual64::view::<Row>(&dual);
        assert!(
            matches!(view, ValueView::Stripped(_)),
            "a dual's view is a copy"
        );
        for (d, v) in dual.iter().zip(view.iter()) {
            assert_eq!(
                d.value.to_bits(),
                v.to_bits(),
                "the value part, bit for bit"
            );
            assert_eq!(
                d.without_tangent().to_bits(),
                v.to_bits(),
                "the scalar read is the same"
            );
        }
        let wide = DualInterval::new(Interval::from_bounds(1.0, 2.0), Interval::from_f64(1.0));
        let row = vec![wide];
        let stripped = DualInterval::view::<Row>(&row);
        assert_eq!(
            (stripped[0].lo(), stripped[0].hi()),
            (1.0, 2.0),
            "Dual<Interval>'s value channel is its enclosure"
        );
    }

    #[test]
    fn a_sym_strips_node_for_node_and_borrows_where_its_numeric_channel_does() {
        use crate::sym::with_session;
        use crate::{Sym, SymBudget};
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let ((), _) = with_session(budget, || {
            let a = Sym::<Dual64>::opaque(Dual64::new(1.5, 3.0));
            let c = a * Sym::<Dual64>::from_f64(2.0) + a;
            let v = c.without_tangent();
            assert!(v.node() == c.node(), "the node is kept");
            assert_eq!(
                v.value().to_bits(),
                c.value().value.to_bits(),
                "the numeric value, bit for bit"
            );
            assert!(
                Sym::<f64>::from_f64(2.0).node() == Sym::<Dual64>::from_f64(2.0).node(),
                "a node does not depend on the numeric scalar under it"
            );
            let row = vec![a, c];
            let view = <Sym<Dual64> as Real>::view::<Row>(&row);
            assert!(
                matches!(view, ValueView::Stripped(_)),
                "over a dual a Sym's view is a copy"
            );
            assert!(
                row.iter()
                    .zip(view.iter())
                    .all(|(s, v)| s.node() == v.node()),
                "node for node"
            );
            let plain = vec![Sym::<f64>::from_f64(1.0)];
            assert!(
                matches!(
                    <Sym<f64> as Real>::view::<Row>(&plain),
                    ValueView::Same(v) if core::ptr::eq(v, &plain)
                ),
                "over f64 a Sym's view borrows"
            );
        });
    }
}
