//! Joining order: the lexicographic null-edge sort (ch. 14 §14.7.1)
//! with a **TOTAL comparator** — the ε-banded `comp` sort of the book
//! (a non-transitive comparator used as a sort key, the notes' flagged
//! robustness landmine) is engineered out per the M3 plan/synthesis.
//!
//! The comparator classifies coordinate differences against the
//! **exact-order band** — `Band::new(f64::from_bits(1),
//! f64::from_bits(2))`, profile's canonical-form device: the open
//! interior (min-subnormal, 2·min-subnormal) contains no representable
//! `f64`, so at `f64` the comparison is exact and total (a Zero means
//! bit-identical or exact ±min-subnormal neighbors — coincident
//! null-edge copies land here by construction, bitwise). At the
//! interval scalar an enclosure straddling the hairline escalates
//! honestly (the replay contract: both lanes must sort identically or
//! the interval lane refuses typed — a silent order divergence would
//! break the key-for-key replay model).
//!
//! Ties (all three coordinates Zero — distinct null edges at one
//! point, e.g. the two tip-vertex runs of the Fig. 14.2 notch) keep
//! **insertion order** (the reduction's deterministic discovery order):
//! the sort is stable, and the topological neighbor criterion
//! disambiguates the join partners at coincident positions.

use geom_core::{Band, BandError, Decide, Indeterminate, Point3, Sign};

use crate::validate::decide;

/// The exact-order band (module docs; identical to profile's).
///
/// # Errors
///
/// [`BandError`] is structurally impossible for these constants; typed
/// through all the same (no panic paths in operator code).
pub(super) fn exact_band() -> Result<Band, BandError> {
    Band::new(f64::from_bits(1), f64::from_bits(2))
}

/// Total lexicographic comparison of two points through the
/// exact-order band — the **`split_join_order_x`/`_y`/`_z`**
/// predicates. `Less`/`Greater` are definite; `Equal` means all three
/// coordinate pairs are bit-level ties.
///
/// # Errors
///
/// [`Indeterminate`] only on the interval lane (an enclosure pair
/// whose difference straddles the hairline without being an exact
/// tie).
pub(super) fn lex_cmp<T: Decide>(
    p: &Point3<T>,
    q: &Point3<T>,
    exact: Band,
) -> Result<core::cmp::Ordering, Indeterminate> {
    use core::cmp::Ordering;
    for (name, a, b) in [
        ("split_join_order_x", p.x, q.x),
        ("split_join_order_y", p.y, q.y),
        ("split_join_order_z", p.z, q.z),
    ] {
        match decide(name, a - b, exact)? {
            Sign::Negative => return Ok(Ordering::Less),
            Sign::Positive => return Ok(Ordering::Greater),
            Sign::Zero => {}
        }
    }
    Ok(core::cmp::Ordering::Equal)
}

/// Stable insertion sort of `items` by a fallible comparator on their
/// sort points (null-edge counts are small; the quadratic sweep is the
/// documented posture, like mesh's CDT note). Returns the sorted
/// permutation of indices into `items`.
///
/// # Errors
///
/// The first comparator escalation, verbatim.
pub(super) fn sort_indices_by_point<T: Decide>(
    points: &[Point3<T>],
    exact: Band,
) -> Result<Vec<usize>, Indeterminate> {
    let mut order: Vec<usize> = (0..points.len()).collect();
    // Insertion sort: stable, deterministic, fallible-comparator-safe.
    for i in 1..order.len() {
        let mut j = i;
        while j > 0 {
            let (a, b) = (order[j - 1], order[j]);
            if lex_cmp(&points[b], &points[a], exact)? == core::cmp::Ordering::Less {
                order.swap(j - 1, j);
                j -= 1;
            } else {
                break;
            }
        }
    }
    Ok(order)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    /// Totality and stability: exact ties keep insertion order; the
    /// order is by (x, y, z) lexicographic value.
    #[test]
    fn lex_sort_total_and_stable() {
        let exact = exact_band().unwrap();
        let pts = [
            p3(2.0, 0.0, 0.0),
            p3(1.0, 5.0, 0.0),
            p3(1.0, 5.0, 0.0), // bit-identical tie with index 1
            p3(1.0, -1.0, 3.0),
            p3(-7.0, 9.0, 9.0),
        ];
        let order = sort_indices_by_point(&pts, exact).unwrap();
        assert_eq!(order, vec![4, 3, 1, 2, 0]);
    }

    /// The comparator is exact: values one ULP apart order strictly
    /// (an ε-banded comparator would call them equal — the engineered-
    /// out fragility).
    #[test]
    fn one_ulp_apart_orders_strictly() {
        let exact = exact_band().unwrap();
        let a = p3(1.0, 0.0, 0.0);
        let b = p3(f64::from_bits(1.0f64.to_bits() + 1), 0.0, 0.0);
        assert_eq!(lex_cmp(&a, &b, exact).unwrap(), core::cmp::Ordering::Less);
        assert_eq!(
            lex_cmp(&b, &a, exact).unwrap(),
            core::cmp::Ordering::Greater
        );
        assert_eq!(lex_cmp(&a, &a, exact).unwrap(), core::cmp::Ordering::Equal);
    }
}
