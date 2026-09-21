//! R2 review probes for RING-1: two axes the ungating pin
//! (`interval_type_default_build.rs`) leaves undrawn in a DEFAULT build.
//!
//! Both rows are ε-free — every band is a literal [`Band::new`] and no
//! row reads the global `Tolerance` — so they read identically at every
//! eps row, like the suite they sit beside.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Real};

/// The pin asserts only that the product's enclosure CONTAINS the true
/// value. That assertion is monotone in the wrong direction: it gets
/// easier as the enclosure degrades, and `[-inf, inf]` passes it. This
/// row goes red when the arithmetic stops being TIGHT — the property
/// an enclosure scalar exists to have.
#[test]
fn the_default_build_product_enclosure_is_tight_and_not_merely_containing() {
    let x = Interval::from_f64(1e-3);
    let sq = x * x;
    let width = Bounds::hi(sq) - Bounds::lo(sq);
    // One outward step each way is the whole budget a correctly
    // rounded backend needs at a point argument.
    let budget = 4.0 * f64::EPSILON * 1e-6;
    assert!(
        width <= budget,
        "point-squared enclosure [{}, {}] is {width} wide; a tight one is <= {budget}",
        Bounds::lo(sq),
        Bounds::hi(sq)
    );
}

/// `repr_bits` returns a TRIPLE `(lo, hi, decoration)`, and the pin's
/// `bit_identity` row only ever varies the endpoints — so a default
/// build whose arm dropped the decoration word would still pass it.
/// This row varies the decoration alone: two enclosures with identical
/// endpoints, one of them the clamped image of a domain violation,
/// must not be bit-identical descriptions.
#[test]
fn the_bit_identity_channel_separates_decoration_in_a_default_build() {
    let clamped = Interval::from_bounds(-1.0, 4.0).sqrt();
    let clean = Interval::from_bounds(Bounds::lo(clamped), Bounds::hi(clamped));
    assert_eq!(
        (Bounds::lo(clamped), Bounds::hi(clamped)),
        (Bounds::lo(clean), Bounds::hi(clean)),
        "the two enclosures must agree on endpoints for this row to be about decoration"
    );
    assert_eq!(
        geom_core::bit_identity::eq_bits(&clamped, &clean),
        Some(false),
        "same endpoints, different decoration: not the same description"
    );
}
