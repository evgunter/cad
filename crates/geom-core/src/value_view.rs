//! **A place, and what is evaluated there** (DL5.2's place row, the
//! crate README, "Reading a bracket where nothing certifies").
//!
//! A place is an `f64` the code chose to evaluate at: a sample, a
//! station, a cell corner, the point a cell is subdivided at, a foot
//! that is only read. It is handed to the evaluator as an `f64`, and the
//! evaluator answers on the value channel ([`Real::Value`]), so a place
//! never becomes a scalar with a tangent and what it reads is decided
//! on, never held by a body. The geometry is put on its value channel
//! once ([`Real::view`], a type's `values()`), and evaluated there at
//! [`place`]s.

use core::ops::Deref;

use crate::real::Real;

/// **A geometry type over a scalar**, as a family: `At<S>` is the type
/// at scalar `S`, and [`OverScalar::map`] carries one to another through
/// a map of every scalar it holds. It is how [`Real::view`] names the
/// type it reads on its value channel without `geom_core` knowing it.
pub trait OverScalar {
    /// The type at scalar `S`.
    type At<S: Real>;

    /// `x` with `f` applied to every scalar it holds, its structure
    /// carried over.
    fn map<S: Real, U: Real>(x: &Self::At<S>, f: impl Fn(S) -> U) -> Self::At<U>;
}

/// **A value on its value channel** ([`Real::view`]): the value itself
/// where its scalar is its own value channel, a copy at a dual. Either
/// way it reads as the value channel's type.
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

/// **A place, at a scalar that is its own value channel**: the `f64` the
/// code chose to evaluate at, embedded exactly. The bound admits no
/// dual, so a place has no tangent by type. What it means at each
/// scalar the bound admits:
///
/// - `f64` and [`crate::Probe`]: the number itself.
/// - [`crate::Interval`]: the point `[x, x]`. The place is exactly `x`,
///   so its point encloses nothing wider; what the evaluator answers
///   encloses the geometry's exact value there.
/// - [`crate::Sym`]: the exact rational `x`, a constant. A decision over
///   what is evaluated there proves an identity at that fixed place,
///   which is what the code chose to read; it claims nothing of a place
///   that would move with the unknowns, since none is read.
#[must_use]
pub fn place<V: Real<Value = V>>(x: f64) -> V {
    V::from_f64(x)
}

#[cfg(test)]
mod tests {
    use super::{OverScalar, ValueView, place};
    use crate::real::{Bounds, Real};
    use crate::{Dual64, DualInterval, Interval};

    /// The edge floats a place can be: signed zeros, the subnormal and
    /// the largest, a non-dyadic fraction, the infinities and NaN.
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

    /// The places these sites took as `from_computed` are the same
    /// numbers at every scalar that is its own value channel and has no
    /// symbols: a moved site reads what it read.
    #[test]
    fn a_place_is_the_computed_embedding_at_f64_interval_and_the_probe() {
        for x in EDGES {
            assert_eq!(
                place::<f64>(x).to_bits(),
                f64::from_computed(x).to_bits(),
                "f64 at {x}"
            );
            let (p, c) = (place::<Interval>(x), Interval::from_computed(x));
            assert_eq!(
                (p.lo().to_bits(), p.hi().to_bits(), p.is_poison()),
                (c.lo().to_bits(), c.hi().to_bits(), c.is_poison()),
                "Interval at {x}"
            );
            #[cfg(feature = "probe")]
            {
                let (p, c) = (place::<crate::Probe>(x), crate::Probe::from_computed(x));
                assert_eq!(p.0.to_bits(), c.0.to_bits(), "Probe at {x}");
            }
        }
    }

    /// One `Vec` of scalars, mapped elementwise.
    struct Row;

    impl OverScalar for Row {
        type At<S: Real> = Vec<S>;

        fn map<S: Real, U: Real>(x: &Vec<S>, f: impl Fn(S) -> U) -> Vec<U> {
            x.iter().map(|&s| f(s)).collect()
        }
    }

    #[test]
    fn a_view_borrows_where_the_scalar_is_its_value_channel_and_strips_a_tangent() {
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
        let dual: Vec<Dual64> = EDGES.iter().map(|&x| Dual64::new(x, 3.0)).collect();
        let ValueView::Stripped(values) = Dual64::view::<Row>(&dual) else {
            panic!("a dual's view is a copy");
        };
        for (d, v) in dual.iter().zip(&values) {
            assert_eq!(
                d.value.to_bits(),
                v.to_bits(),
                "the value part, bit for bit"
            );
            assert_eq!(
                d.value().to_bits(),
                v.to_bits(),
                "`Real::value` is the same read"
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
}
