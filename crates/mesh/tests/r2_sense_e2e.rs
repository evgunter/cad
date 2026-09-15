//! **The sense bit through the public doors, end to end.** A revolved
//! unit ball, its inside-out twin and the same ball with ONE band
//! reversed, each measured by `topo::props::mass_properties` and
//! judged by `topo::validate::validate_geometric` — the two doors a
//! user reaches for, above the five doors this program re-typed.
//!
//! The one-band twin is what makes the rimless sphere band's `sense`
//! arm observable from outside the kernel at all: that arm carries the
//! one orientation fact a curved face's boundary does not encode, so
//! reversing one band moves a measurement rather than only a stored
//! bit.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::Tol;
use topo::props::mass_properties;
use topo::validate::{ValidationError, validate_geometric};

/// **One body, three orientations, two doors** — one row, because all
/// three twins are clones of one revolve and nextest is
/// process-per-test (`memories/test-suite-cost`); every assertion
/// names the twin it speaks for.
///
/// What it pins:
///
/// - the ball as revolved measures `+4/3·π` and validates;
/// - the inside-out twin measures `−4/3·π` and is refused
///   `NegativeVolume` — the +V invariant seeing a whole-body flip;
/// - **the one-band twin measures exactly `0.0`, and the MEASURING
///   door does not refuse it.** The two hemispheres' radial terms
///   cancel term for term, so `mass_properties` answers a
///   plausible-looking number for a body that encloses nothing. Tier 3
///   is what refuses it, and what it raises is `LaminaWedge` — the
///   seam meridians reading as conformal contact — not an orientation
///   verdict. Both halves of that are a finding, recorded on PROPS'
///   slate (`work/props/m6-sense-gate-recorded-residuals.md`).
#[test]
fn a_reversed_band_measures_zero_and_is_caught_only_by_tier_three() {
    let ball = common::ball();
    let keys: Vec<_> = ball.faces().map(|(k, _)| k).collect();
    let exact = 4.0 / 3.0 * std::f64::consts::PI;

    let mp = mass_properties(&ball, Tol::witness()).expect("the ball measures");
    assert!(
        (mp.volume - exact).abs() <= 1e-12 * exact,
        "the revolved unit ball encloses 4/3·π: {}",
        mp.volume
    );
    assert!(
        validate_geometric(&ball, Tol::witness()).is_ok(),
        "the ball as revolved is a valid solid"
    );

    let mut inside_out = ball.clone();
    for &k in &keys {
        let s = inside_out.get_face(k).unwrap().sense;
        inside_out.set_face_sense(k, !s).unwrap();
    }
    let mp = mass_properties(&inside_out, Tol::witness()).expect("the twin measures too");
    assert!(
        (mp.volume + exact).abs() <= 1e-12 * exact,
        "reversing every face negates the enclosed volume: {}",
        mp.volume
    );
    let errs = validate_geometric(&inside_out, Tol::witness())
        .expect_err("an inside-out solid is not valid");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume)),
        "the +V invariant names a whole-body flip: {errs:?}"
    );

    let mut one_band = ball.clone();
    let s = one_band.get_face(keys[0]).unwrap().sense;
    one_band.set_face_sense(keys[0], !s).unwrap();
    let mp = mass_properties(&one_band, Tol::witness())
        .expect("and the measuring door does not refuse this one either");
    assert_eq!(
        mp.volume, 0.0,
        "one reversed band cancels the other's radial term exactly"
    );
    let errs =
        validate_geometric(&one_band, Tol::witness()).expect_err("tier 3 is what catches it");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "the signal a user gets names conformal contact, not an inverted face: {errs:?}"
    );
}
