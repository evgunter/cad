//! The certified [`Interval`] scalar's own doors, pinned at the crate
//! that owns them: the scalar is nameable, constructible and
//! **decides** in the default build — the only build there is — with
//! nothing above `geom-core` in this suite's graph, so a row that goes
//! red here is the scalar's defect and not a lane impl's.
//!
//! Deliberately ε-free — every band here is a literal [`Band::new`] and
//! no row reads the global `Tolerance`, so the suite reads identically
//! at every eps row and needs no process of its own.
//!
//! The rows are the scalar's doors: the type and its
//! arithmetic, the [`Decide`] impl in both its answers, the
//! `bit_identity` arm in both its channels (endpoints and decoration)
//! and the [`SpanLocate`] impl — plus [`DualInterval`], the
//! dual-over-interval alias. Containment is the contract and tightness
//! is the quality, so the arithmetic is pinned on both. What the
//! `SpanLocate` row pins is that impl's BEHAVIOUR, not the `Sealed`
//! marker it needs: the crate's own `SpanLocate` impl requires the
//! marker, so a build without it is not a build at all.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{
    Band, Bounds, CertifiedEnclosure, Decide, Dual, DualInterval, Interval, Real, Sign,
};

/// Construction, arithmetic and the certified decision door. The band
/// is 1e-9/1e-8 and the values are orders away from it in
/// each direction, so no row here is a tolerance claim.
#[test]
fn interval_constructs_decides_and_encloses_in_a_default_build() {
    let band = Band::new(1e-9, 1e-8).unwrap();

    let x = Interval::from_f64(1e-3);
    assert_eq!(Bounds::lo(x), 1e-3, "from_f64 is the point enclosure");
    assert_eq!(Bounds::hi(x), 1e-3, "from_f64 is the point enclosure");
    assert_eq!(x.sign_within(band), Ok(Sign::Positive));
    assert_eq!(Interval::from_f64(0.0).sign_within(band), Ok(Sign::Zero));
    assert_eq!(
        Interval::from_f64(-1e-3).sign_within(band),
        Ok(Sign::Negative)
    );

    // The arithmetic encloses: the true 2e-6 lies inside the product's
    // endpoints, which outward rounding may widen but never lose.
    // Containment is the contract; tightness is a separate row below.
    let prod = x * Interval::from_f64(2e-3);
    assert!(
        Bounds::lo(prod) <= 2e-6 && 2e-6 <= Bounds::hi(prod),
        "[{}, {}] must enclose 2e-6",
        Bounds::lo(prod),
        Bounds::hi(prod)
    );

    // An enclosure straddling both thresholds has no certifiable sign —
    // the escalation the enclosure scalar exists to produce.
    assert!(
        Interval::from_bounds(-1e-3, 1e-3)
            .sign_within(band)
            .is_err(),
        "an enclosure spanning zero cannot certify a sign"
    );
}

/// The `repr_bits` arm for `Interval` is compiled, so the identity
/// channel answers `Some` at this scalar. A missing arm falls through
/// to `None` — the answer a channel-less scalar gives — and that is the
/// value this row would catch.
#[test]
fn the_bit_identity_channel_reads_intervals_in_a_default_build() {
    let a = Interval::from_f64(2.0);
    let b = Interval::from_f64(2.0);
    let c = Interval::from_f64(3.0);
    assert_eq!(geom_core::bit_identity::eq_bits(&a, &b), Some(true));
    assert_eq!(geom_core::bit_identity::eq_bits(&a, &c), Some(false));
}

/// The [`SpanLocate`] impl is compiled, and it is the
/// interval-natured one: an enclosure straddling the interior knot
/// overlaps BOTH spans, where a point scalar's locator answers one.
/// That behaviour is what this row pins; the `Sealed` marker beneath
/// it cannot be pinned separately, since the crate does not compile
/// without it.
#[test]
fn interval_locates_a_span_range_in_a_default_build() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    let straddling = Interval::from_bounds(0.5, 1.5).locate_spans(&knots);
    assert_eq!(
        (straddling.first.index(), straddling.last.index()),
        (2, 3),
        "[0.5, 1.5] straddles the knot at 1.0"
    );
    let inside = Interval::from_f64(0.5).locate_spans(&knots);
    assert_eq!((inside.first.index(), inside.last.index()), (2, 2));
}

/// `DualInterval` — the dual-over-interval alias Q1 ratifies — names a
/// type in a default build, and its two channels carry their own
/// enclosures.
#[test]
fn dual_interval_is_nameable_in_a_default_build() {
    let d: DualInterval = Dual::new(Interval::from_f64(2.0), Interval::from_f64(1.0));
    assert_eq!(Bounds::lo(d.value), 2.0);
    assert_eq!(Bounds::hi(d.deriv), 1.0);
}

/// The refusing half of the decision door is compiled too: a
/// domain-clamped enclosure (`sqrt([-1, 4])` clamps to `[0, 2]`, a
/// plausible bracket) is refused on its DECORATION, not on its
/// endpoints — `MarginDiag::Invalid`, where the straddling row above
/// refuses with `MarginDiag::Enclosure`. A build that compiled the
/// arithmetic but not the poison channel would answer `Positive`.
#[test]
fn a_domain_clamp_refuses_on_the_decoration_in_a_default_build() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let clamped = Interval::from_bounds(-1.0, 4.0).sqrt();
    assert!(
        Bounds::lo(clamped) <= 0.0 && Bounds::hi(clamped) >= 2.0,
        "the clamped bracket is still a sound bracket of [0, 2]"
    );
    let refused = clamped.sign_within(band).unwrap_err();
    assert!(
        matches!(refused.margin, geom_core::MarginDiag::Invalid),
        "a clamped enclosure refuses on its decoration: {refused:?}"
    );
    assert_eq!(
        clamped.certified_bracket(),
        None,
        "the certified door refuses the same enclosure"
    );
    assert_eq!(
        Interval::from_bounds(1.0, 4.0).sqrt().certified_bracket(),
        Some((1.0, 2.0)),
        "an in-domain sqrt certifies its exact bracket"
    );
}

/// Containment gets EASIER as an enclosure degrades — `[0, 1]`
/// contains 1e-6 — so the enclosing assertion in the first row can
/// catch a lost bracket and never a lost TIGHTNESS. This row bounds the width of the
/// square from above instead. It squares through [`Real::powi`], the
/// idiom `scripts/gates/interval-square-allowlist.sh` requires of a
/// square (the operator treats the two factors as independent, which
/// costs a zero-straddling enclosure its nonnegative lower bound), and
/// the backend pads at most 1 ulp per arithmetic endpoint
/// (`interval.rs`, "Tightness is a quality"), so 4 ulp of 1e-6 passes
/// a tight answer and fails a widened one.
#[test]
fn the_square_enclosure_is_ulp_tight_in_a_default_build() {
    let sq = Interval::from_f64(1e-3).powi(2);
    let width = Bounds::hi(sq) - Bounds::lo(sq);
    let ulp = f64::EPSILON * 1e-6;
    assert!(
        width <= 4.0 * ulp,
        "[{}, {}] is {width} wide, more than 4 ulp ({ulp}) of 1e-6",
        Bounds::lo(sq),
        Bounds::hi(sq)
    );
}

/// `repr_bits` is a TRIPLE — endpoints and a decoration word — and the
/// endpoint row above varies only the first two, so an arm that dropped
/// the decoration would still pass it. This row varies the decoration
/// alone: the clamped image of a domain violation and a clean enclosure
/// with the same endpoints are not the same description.
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
