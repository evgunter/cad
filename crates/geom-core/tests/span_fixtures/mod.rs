//! The shared knot-vector spread for the `Span` suites.
//!
//! `span_newtype`, `span_basis_identity` and `span_hull_window` all need
//! the same coverage — a uniform cubic, two vectors carrying an
//! **interior multiplicity 2** (so the walk must skip an empty span),
//! the smallest legal degree, and a high degree — and each grew its own
//! copy of the list. One copy, here.
//!
//! **The ORDER is load-bearing.** `span_basis_identity`'s golden table
//! is indexed by the walk over [`vectors()`], so reordering or inserting
//! an entry shifts every row after it. Append, or regenerate that table
//! (its own docs say how).
#![allow(dead_code)] // each suite uses a subset
#![allow(unreachable_pub)] // `mod`-included by several test binaries; `pub` is how each names it

use geom_core::spline::KnotVector;

fn kv(knots: Vec<f64>, degree: usize) -> KnotVector {
    KnotVector::clamped(knots, degree).expect("valid knot vector")
}

/// Degree 3, single interior knots — the ordinary case.
pub fn uniform_cubic() -> KnotVector {
    kv(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 4.0],
        3,
    )
}

/// Degree 3 with an interior knot repeated twice, so `[0.3, 0.3]` is an
/// **empty** span — the index every walk must refuse or skip.
pub fn multiplicity_2_cubic() -> KnotVector {
    kv(
        vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 1.7, 2.0, 2.0, 2.0, 2.0],
        3,
    )
}

/// The five vectors, in the fixed order the golden table depends on.
pub fn vectors() -> Vec<(&'static str, KnotVector)> {
    vec![
        ("uniform cubic", uniform_cubic()),
        ("cubic, interior multiplicity 2", multiplicity_2_cubic()),
        ("degree 1", kv(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1)),
        (
            "degree 5",
            kv(
                vec![0.0; 6]
                    .into_iter()
                    .chain([0.4, 0.9])
                    .chain(vec![1.0; 6])
                    .collect(),
                5,
            ),
        ),
        (
            "quadratic, interior multiplicity 2, nonuniform",
            kv(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.25, 3.0, 3.0, 3.0], 2),
        ),
    ]
}
