//! A carrier's weights held in the scalar `T` (C6).
//!
//! At an enclosure scalar a carrier stands for a family, so a weight
//! may be an enclosure wider than a point, as a pole may. What is
//! structure is decided once, for every member: every weight is
//! strictly positive (the convex-combination licence every hull bound
//! stands on), certified over the values it stands for. A weight
//! positive for some members and not others refuses typed, carrying
//! its enclosure as the margin a narrower box lifts
//! ([`SplineError::WeightSignUndecided`]).
//!
//! Where every weight is a point, the `f64` vector read off them is
//! still reachable, as C6's interim: [`Weights::weights_f64_interim`],
//! which refuses loud on weights wider than a point. A carrier's door
//! admits only point weights for now
//! ([`SplineError::CarrierWeightNotAPoint`]): its knot algebra and
//! certificates read that vector.

use super::knots::{SplineError, check_weight_value};
use super::locate::SpanLocate;
use crate::Real;

/// A carrier's weights, held in `T`, each certified strictly positive
/// and finite for every value it stands for (module docs).
#[derive(Clone, Debug)]
pub struct Weights<T> {
    values: Vec<T>,
    /// The `f64` vector, where every weight is a point: the values'
    /// value channel, C6's interim.
    point: Option<Vec<f64>>,
}

impl<T: SpanLocate> Weights<T> {
    /// The weights `values`, each certified for every value it stands
    /// for. Where every one is a point, the `f64` vector is read off
    /// them through [`SpanLocate::point_value`].
    ///
    /// # Errors
    ///
    /// At the first weight that is not certified. A point weight
    /// refuses as [`check_weight_value`] does. A weight wider than a
    /// point has its sign decided first:
    /// - [`SplineError::NonPositiveWeight`] where it is positive for no
    ///   member (poison reads as NaN; an enclosure whose top is not above
    ///   zero reports that top);
    /// - [`SplineError::WeightSignUndecided`] where it is positive for
    ///   some members and not others, carrying its enclosure, which may
    ///   reach `−∞`;
    /// - then [`SplineError::NonFiniteWeight`] where it is positive for
    ///   every member and reaches `+∞`.
    pub fn new(values: Vec<T>) -> Result<Self, SplineError> {
        let mut point: Option<Vec<f64>> = Some(Vec::with_capacity(values.len()));
        for (index, &w) in values.iter().enumerate() {
            if let Some(v) = w.point_value() {
                check_weight_value(index, v)?;
                if let Some(p) = point.as_mut() {
                    p.push(v);
                }
                continue;
            }
            point = None;
            let Some((lo, hi)) = w.structure_bracket() else {
                return Err(SplineError::NonPositiveWeight {
                    index,
                    weight: f64::NAN,
                });
            };
            if !w.structure_positive() {
                return Err(if hi > 0.0 {
                    SplineError::WeightSignUndecided {
                        index,
                        margin: crate::MarginDiag::enclosure(lo, hi),
                    }
                } else {
                    SplineError::NonPositiveWeight { index, weight: hi }
                });
            }
            if !w.is_bounded() {
                return Err(SplineError::NonFiniteWeight { index, weight: hi });
            }
        }
        Ok(Self { values, point })
    }

    /// The first weight wider than a point, where there is one.
    pub fn first_wide(&self) -> Option<usize> {
        self.values.iter().position(|w| w.point_value().is_none())
    }

    /// The one plain constant every weight is ([`constant_weight`] over
    /// [`SpanLocate::plain_constant`]), or `None`: a weight that carries
    /// an enclosure, a derivative or a symbol is no constant, whatever
    /// its value channel reads.
    pub fn constant(&self) -> Option<f64> {
        let plain: Option<Vec<f64>> = self.values.iter().map(|w| w.plain_constant()).collect();
        constant_weight(&plain?)
    }
}

/// **Whether a weight vector cancels out of the rational basis**: the
/// one value every weight holds, compared on the bits against the
/// first, or `None` where two differ or there are none.
///
/// A constant `c` cancels exactly, `Rᵢ = Nᵢ·c / Σₖ Nₖ·c = Nᵢ`, so a
/// carrier whose weights answer `Some(c)` IS the polynomial on its
/// control points, read as Cartesian, for every `c`, `1` among them.
/// Every "is this carrier polynomial" question in the kernel is this
/// one; the carriers ask it through their own `constant_weight`.
#[must_use]
pub fn constant_weight(weights: &[f64]) -> Option<f64> {
    let (&first, rest) = weights.split_first()?;
    rest.iter()
        .all(|w| w.to_bits() == first.to_bits())
        .then_some(first)
}

impl<T> Weights<T> {
    /// Weights whose certification is ALREADY established: a carrier's
    /// structural door (its `from_validated_parts`), carrying a
    /// validated carrier's point weights under a map that keeps them,
    /// or a knot-algebra plan's weights, every one of which the plan
    /// admitted. `channel` is `values`' value channel, one per weight.
    #[must_use]
    pub fn from_point_parts(values: Vec<T>, channel: Vec<f64>) -> Self {
        debug_assert_eq!(
            values.len(),
            channel.len(),
            "Weights::from_point_parts: one channel value per weight"
        );
        Self {
            values,
            point: Some(channel),
        }
    }

    /// The weights as held.
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// The count.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether there are none.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Whether every weight is a point, so that
    /// [`Self::weights_f64_interim`] answers.
    pub fn is_point(&self) -> bool {
        self.point.is_some()
    }

    /// **C6's interim**: the `f64` vector read off these weights, where
    /// every one is a point. Weights wider than a point have none, and a
    /// reader reaching for it there is a kernel bug, refused loud with
    /// its location. Counted by `scripts/gates/knots-f64-interim.sh`.
    ///
    /// # Panics
    ///
    /// On weights that are not all points.
    #[track_caller]
    #[allow(clippy::panic)]
    pub fn weights_f64_interim(&self) -> &[f64] {
        // A `match`, not `unwrap_or_else`: `Location::caller()` inside a
        // closure names this file, not the reader.
        match &self.point {
            Some(p) => p,
            None => panic!(
                "a reader at {} took the f64 vector of weights that are not all points \
                 (C6: weights are their held values)",
                core::panic::Location::caller()
            ),
        }
    }

    /// The parts: the held values and, where every one is a point,
    /// their `f64` vector.
    pub fn into_parts(self) -> (Vec<T>, Option<Vec<f64>>) {
        (self.values, self.point)
    }
}

impl<T: Real> Weights<T> {
    /// The same weights at another scalar under a LIFT `f`, one that
    /// keeps each weight's point (the `map_scalar` contract,
    /// `geom`'s `scalar_lift` module docs): `f` applied to each, the
    /// `f64` vector carried over.
    #[must_use]
    pub fn lift<U: Real>(&self, f: impl Fn(T) -> U) -> Weights<U> {
        Weights {
            values: self.values.iter().map(|w| f(*w)).collect(),
            point: self.point.clone(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Weights, constant_weight};

    /// **Constancy is on the bits, and hands back the constant.** Any
    /// constant answers itself, `1` and `2` alike; one ulp of spread,
    /// or no weights at all, answers `None`. Through the held values, a
    /// point enclosure is its constant and a wide one is none.
    #[test]
    fn constancy_is_bitwise_against_the_first_weight_and_names_the_constant() {
        assert_eq!(constant_weight(&[1.0; 3]), Some(1.0));
        assert_eq!(constant_weight(&[2.0; 3]), Some(2.0));
        assert_eq!(constant_weight(&[0.5]), Some(0.5));
        assert_eq!(constant_weight(&[2.0, 2.0, 2.0f64.next_up()]), None);
        assert_eq!(constant_weight(&[1.0, 0.5, 1.0]), None);
        assert_eq!(constant_weight(&[]), None);
        let iv = crate::Interval::from_bounds;
        let held = |w: Vec<crate::Interval>| Weights::new(w).unwrap().constant();
        assert_eq!(held(vec![iv(2.0, 2.0); 3]), Some(2.0));
        assert_eq!(held(vec![iv(2.0, 2.0), iv(1.5, 2.5), iv(2.0, 2.0)]), None);
        assert_eq!(Weights::new(vec![3.0f64; 2]).unwrap().constant(), Some(3.0));
    }
    use crate::Interval;
    use crate::spline::SplineError;

    /// **A weight's sign is decided for every member (C6)**: a wide
    /// weight wholly above zero is held, with no `f64` vector; one that
    /// reaches zero or below for some members refuses undecided,
    /// carrying its enclosure; a narrower box that clears zero lifts
    /// that refusal; one below zero for every member refuses as not
    /// positive. Red where a straddling weight is held, or refused
    /// without its margin.
    #[test]
    fn a_weight_some_members_take_nonpositive_refuses_undecided_and_a_narrower_box_lifts_it() {
        let iv = Interval::from_bounds;
        let pt = |x: f64| iv(x, x);
        let held = Weights::new(vec![pt(1.0), iv(0.5, 2.0), pt(1.0)]).expect("held");
        assert!(!held.is_point(), "no f64 vector names a wide weight");
        match Weights::new(vec![pt(1.0), iv(-0.25, 0.5), pt(1.0)]) {
            Err(SplineError::WeightSignUndecided { index, margin }) => {
                assert_eq!(index, 1);
                let crate::ErrorTextReading::Enclosure { lo, hi } =
                    margin.diagnostic_f64_for_error_text()
                else {
                    panic!("an enclosure margin, got {margin:?}");
                };
                assert_eq!((lo, hi), (-0.25, 0.5));
            }
            other => panic!("a straddling weight was held: {other:?}"),
        }
        assert!(
            Weights::new(vec![pt(1.0), iv(0.125, 0.5), pt(1.0)]).is_ok(),
            "a narrower box clear of zero lifts it"
        );
        match Weights::new(vec![pt(1.0), iv(-0.5, 0.0), pt(1.0)]) {
            Err(SplineError::NonPositiveWeight { index, weight }) => {
                assert_eq!((index, weight), (1, 0.0));
            }
            other => panic!("a weight positive for no member: {other:?}"),
        }
        // The boundaries: a member at exactly zero is not positive, from
        // either zero; a weight reaching `−∞` is decided by its sign
        // first, and `+∞` past a positive bottom is not finite.
        for (lo, hi) in [(0.0, 0.5), (-0.0, 0.5), (f64::NEG_INFINITY, 5.0)] {
            match Weights::new(vec![pt(1.0), iv(lo, hi), pt(1.0)]) {
                Err(SplineError::WeightSignUndecided { index: 1, .. }) => {}
                other => panic!("[{lo}, {hi}]: a member not positive: {other:?}"),
            }
        }
        match Weights::new(vec![pt(1.0), iv(f64::NEG_INFINITY, -1.0), pt(1.0)]) {
            Err(SplineError::NonPositiveWeight { index, weight }) => {
                assert_eq!((index, weight), (1, -1.0));
            }
            other => panic!("[-inf, -1]: positive for no member: {other:?}"),
        }
        match Weights::new(vec![pt(1.0), iv(1.0, f64::INFINITY), pt(1.0)]) {
            Err(SplineError::NonFiniteWeight { index: 1, .. }) => {}
            other => panic!("[1, inf]: not finite: {other:?}"),
        }
    }

    /// **Point weights keep the `f64` rules, and their vector**: at `f64`,
    /// zero, negative and NaN refuse as not positive and `+∞` as not
    /// finite (at `Interval` a point `+∞` is poison, which refuses as not
    /// positive); a wide weight positive for every member and reaching
    /// `+∞` refuses as not finite. Point weights read through their `f64`
    /// vector; wide ones refuse the read loud, naming the reader.
    #[test]
    fn point_weights_keep_the_f64_rules_and_wide_ones_refuse_the_f64_read() {
        use test_utils::panic_capture::caught;
        for (w, want) in [
            (0.0, "positive"),
            (-1.0, "positive"),
            (f64::NAN, "positive"),
            (f64::INFINITY, "finite"),
        ] {
            let got = Weights::new(vec![1.0, w]);
            let ok = matches!(
                (&got, want),
                (
                    Err(SplineError::NonPositiveWeight { index: 1, .. }),
                    "positive"
                ) | (Err(SplineError::NonFiniteWeight { index: 1, .. }), "finite")
            );
            assert!(ok, "{w}: {got:?}");
        }
        match Weights::new(vec![
            Interval::from_bounds(1.0, 1.0),
            Interval::from_bounds(1.0, f64::INFINITY),
        ]) {
            Err(SplineError::NonFiniteWeight { index: 1, .. }) => {}
            other => panic!("an unbounded weight: {other:?}"),
        }
        let point = Weights::new(vec![0.5, 2.0]).unwrap();
        assert_eq!(point.weights_f64_interim(), &[0.5, 2.0]);
        let wide = Weights::new(vec![Interval::from_bounds(0.5, 2.0)]).unwrap();
        let line = line!() + 2;
        let read = caught(|| {
            wide.weights_f64_interim();
        });
        let message = read.expect("a wide carrier's f64 weights were read");
        let at = format!("a reader at {}:{line}:", file!());
        assert!(message.contains(&at), "{message:?} does not name {at}");
    }
}
