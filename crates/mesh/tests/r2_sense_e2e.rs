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
use topo::construction::Construct;
use topo::props::mass_properties;
use topo::validate::{ValidationError, validate_geometric};

/// **One body, three orientations, two doors** — one row, because all
/// three twins are clones of one revolve and nextest is
/// process-per-test (implementer-discipline §8); every assertion
/// names the twin it speaks for.
///
/// What it pins:
///
/// - the ball as revolved measures `+4/3·π` and validates;
/// - the inside-out twin measures `−4/3·π` and is refused
///   `NegativeVolume` — the +V invariant seeing a whole-body flip;
/// - **the one-band twin is refused by both doors, as one lamina.** Its
///   two hemispheres' radial terms would cancel term for term, a volume
///   of exactly `0.0`; the measuring door reads their seam region, whose
///   members' bits disagree, and refuses
///   `PropsError::SeamRegionSensesDisagree`, the finding tier 3 names
///   `LaminaWedge` (the seam meridians read as conformal contact). That
///   tier 3 names a lamina rather than an inverted face is still a
///   finding (`work/verdict/m6-sense-gate-recorded-residuals.md`).
#[test]
fn a_reversed_band_is_refused_as_a_lamina_by_both_doors() {
    let ball = common::ball();
    let keys: Vec<_> = ball.faces().map(|(k, _)| k).collect();
    let exact = 4.0 / 3.0 * std::f64::consts::PI;

    let mp = mass_properties(
        &topo::test_support::finished("the measured body", ball.clone(), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("the ball measures");
    assert!(
        (mp.enclosure.volume - exact).abs() <= 1e-12 * exact,
        "the revolved unit ball encloses 4/3·π: {}",
        mp.enclosure.volume
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
    // Raw door: the measured body is an inside-out body, which tier 3 refuses (NegativeVolume).
    let mp = topo::test_support::measure_unvalidated(
        &inside_out,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("the twin measures too");
    assert!(
        (mp.enclosure.volume + exact).abs() <= 1e-12 * exact,
        "reversing every face negates the enclosed volume: {}",
        mp.enclosure.volume
    );
    let errs = validate_geometric(&inside_out, Tol::witness())
        .expect_err("an inside-out solid is not valid");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume { .. })),
        "the +V invariant names a whole-body flip: {errs:?}"
    );

    let mut one_band = ball.clone();
    let s = one_band.get_face(keys[0]).unwrap().sense;
    one_band.set_face_sense(keys[0], !s).unwrap();
    // Raw door: the measured body is a lamina (a flipped band), which tier 3 refuses (LaminaWedge).
    let door = topo::test_support::measure_unvalidated(
        &one_band,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    );
    assert!(
        matches!(
            door,
            Err(topo::MassPropsError::Face {
                source: geom_brep::props::PropsError::SeamRegionSensesDisagree,
                ..
            })
        ),
        "the measuring door refuses the lamina off the seam region: {door:?}"
    );
    let errs =
        validate_geometric(&one_band, Tol::witness()).expect_err("tier 3 is what catches it");
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "the signal a user gets names conformal contact, not an inverted face: {errs:?}"
    );
}
