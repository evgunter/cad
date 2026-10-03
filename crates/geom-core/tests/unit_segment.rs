//! **`KnotVector::unit_segment` at its boundary.** The Bézier
//! constructor is total over its argument type, so the only claims
//! left to pin are that it builds exactly the vector
//! [`KnotVector::clamped`] would validate at that degree — no
//! substitution at the smallest degree, none above it — and that the
//! degree it reports is the degree it was asked for. The other half of
//! the boundary, that degree 0 is not a spellable argument, is the
//! `compile_fail` row on the constructor's own doc.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroUsize;
use geom_core::spline::KnotVector;

#[test]
fn the_bezier_vector_is_exactly_the_clamped_vector_of_its_degree() {
    for degree in 1..=6usize {
        let d = NonZeroUsize::new(degree).unwrap();
        let got = KnotVector::unit_segment(d);
        let mut knots = vec![0.0; degree + 1];
        knots.extend(core::iter::repeat_n(1.0, degree + 1));
        let want = KnotVector::clamped(knots, degree).unwrap();
        assert_eq!(got.degree(), degree, "degree {degree}: reported degree");
        assert_eq!(got.knots(), want.knots(), "degree {degree}: knots");
        assert_eq!(
            got.control_count(),
            degree + 1,
            "degree {degree}: one Bézier segment"
        );
        assert_eq!(got.domain(), (0.0, 1.0), "degree {degree}: unit domain");
        assert_eq!(
            got.interior_knots().count(),
            0,
            "degree {degree}: single span"
        );
    }
}

/// The smallest representable degree is `NonZeroUsize::MIN`, and it is
/// the degree-1 vector — not a clamp of anything smaller, because
/// nothing smaller can be asked for.
#[test]
fn the_smallest_degree_is_one_by_type() {
    let got = KnotVector::unit_segment(NonZeroUsize::MIN);
    assert_eq!(got.degree(), 1);
    assert_eq!(got.knots(), &[0.0, 0.0, 1.0, 1.0]);
}
