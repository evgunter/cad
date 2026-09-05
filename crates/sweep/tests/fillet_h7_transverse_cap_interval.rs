//! The ruled band at the CERTIFIED scalar (feature `interval`) — the
//! interval twin of `fillet_h7_transverse_cap`'s rod row and of its
//! `fillet3_cap_transverse` trio.
//!
//! The rod with a flat is built through the same public doors at
//! `Interval`, both creases carve, the result is tier-3 valid with a
//! closed-form inventory (`volume_pad == 0`), and the volume enclosure
//! BRACKETS the prism closed form `V₀ − 2·A_section·L` narrowly enough
//! to be a claim. The predicate's three arms are exercised at `Interval`
//! too: a transverse cap is Zero, an oblique one refuses typed, and an
//! in-band one escalates naming the predicate.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Interval, Real, Tol, Vec3};
use sweep::blend::BlendError;
use sweep::blend::battery::cap_transverse;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{ROD_FLAT, ROD_L, ROD_R, rod_creases, rod_section_cut, rod_with_flat_at};
use topo::{VertexKey, mass_properties, validate_geometric};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// An enclosure must contain its truth AND be a claim.
fn assert_brackets(got: Interval, truth: f64, what: &str) {
    assert!(
        got.lo() <= truth && truth <= got.hi(),
        "{what}: the enclosure must contain the truth: {got:?} vs {truth}"
    );
    assert!(
        got.hi() - got.lo() < 1e-9,
        "{what}: the enclosure must be a claim, not a shrug: width {}",
        got.hi() - got.lo()
    );
}

#[test]
fn the_rod_carves_at_the_certified_scalar_and_brackets_the_prism_closed_form() {
    let tol = Tol::witness();
    let r = 0.1;
    let source = rod_with_flat_at::<Interval>(tol);
    let creases = rod_creases(&source);
    assert_eq!(creases.len(), 2, "two ruling creases");
    let p0 = mass_properties(&source, tol).expect("interval props");
    assert_eq!(p0.volume_pad, 0.0, "the source is closed-form");

    let out = fillet_edges(&source, &creases, iv(r), tol)
        .unwrap_or_else(|e| panic!("the ruled band carves at Interval, got {e:?}"));
    assert_eq!(out.blend_faces.len(), 2, "one band per crease");
    assert!(
        out.corner_faces.is_empty(),
        "a transverse cap is not a corner"
    );
    validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));
    let p1 = mass_properties(&out.body, tol).expect("interval props");
    assert_eq!(
        p1.volume_pad, 0.0,
        "closed-form inventory at the certified scalar too"
    );
    let cut = 2.0 * rod_section_cut(ROD_R, ROD_FLAT, r) * ROD_L;
    // The source's own enclosure is a point up to the lane's rounding;
    // the carved enclosure must bracket it less the prism.
    let v0 = 0.5 * (p0.volume.lo() + p0.volume.hi());
    assert_brackets(
        p1.volume,
        v0 - cut,
        "the carved body against the prism closed form",
    );
    assert!(
        p1.volume.hi() < p0.volume.lo(),
        "a convex band removes material, definitely: {:?} below {:?}",
        p1.volume,
        p0.volume
    );
}

/// **The two-tolerance trio for `fillet3_cap_transverse` at `Interval`**:
/// each arm is reachable and distinct at the certified scalar.
#[test]
fn cap_transverse_trio_at_the_certified_scalar() {
    let band = Band::linear(Tol::witness()).expect("a band");
    let v = VertexKey::default();
    let tau = Vec3::new(iv(0.0), iv(0.0), iv(1.0));
    let lever = iv(1.0);
    // Transverse: the cap normal IS the ruling.
    cap_transverse(v, Vec3::new(iv(0.0), iv(0.0), iv(-1.0)), tau, lever, band)
        .expect("a perpendicular cap is Zero");
    // Oblique: a definite departure refuses as the run-out it is.
    let oblique = cap_transverse(
        v,
        Vec3::new(iv(0.3), iv(0.0), iv(0.9539392014169457)),
        tau,
        lever,
        band,
    )
    .expect_err("an oblique cap refuses");
    assert!(
        matches!(oblique, BlendError::UnsupportedRunOut { .. }),
        "the oblique cap is a run-out, got {oblique:?}"
    );
    // In band: a departure between the band's zero and its escalate.
    let t = 0.5 * (band.zero() + band.escalate());
    let escalated = cap_transverse(v, Vec3::new(iv(t), iv(0.0), iv(1.0)), tau, lever, band)
        .expect_err("an in-band cap escalates");
    match escalated {
        BlendError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_cap_transverse"));
        }
        other => panic!("the in-band row must escalate, got {other:?}"),
    }
}
