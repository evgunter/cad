//! R1 review probes for CERT-6 (the A2 area gauge). NOT for merge —
//! these live on the reviewer's probe branch only.
//!
//! The E2E the brief requires: author a body through the PUBLIC door
//! (`sweep::loft_body` + `topo::props::mass_properties`) with debug
//! assertions ON, and confirm the gauge is silent on an honest face.
//! The corruption half is a separate run against a locally-patched
//! `props/quad.rs` (recorded in the probe branch's log).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::quad;
use geom_core::Tol;
use geom_core::{Affine3, Vec3};
use sweep::{Section, loft_body};

/// A body of my own — NOT the spec's shape (iii): five sections, two
/// non-affine pairs, a deliberate 8x scale ramp so the walls' cells
/// are strongly non-uniform, and an off-axis translation so no wall
/// is an iso-plane of the global frame.
fn r1_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let s = |k: f64| {
        quad([
            (-k, -k),
            (1.0 * k, -0.6 * k),
            (0.8 * k, 1.2 * k),
            (-1.3 * k, 0.9 * k),
        ])
    };
    let sections = vec![s(1.0), s(2.0), s(0.5), s(4.0), s(1.0)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.3, -0.2, 1.0)),
        Affine3::translation(Vec3::new(-0.4, 0.5, 2.0)),
        Affine3::translation(Vec3::new(0.1, 0.1, 3.0)),
        Affine3::translation(Vec3::new(0.0, -0.3, 4.0)),
    ];
    (sections, places)
}

/// E2E, honest face: the gauge must be SILENT. With
/// `debug-assertions = true` (dev and, per the profile, release), a
/// fire is a panic, so reaching the end of this test IS the reading.
#[test]
fn r1_e2e_gauge_is_silent_on_an_honest_body_through_the_public_door() {
    let (sections, places) = r1_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("r1 loft builds");
    let m = topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
    // Report the honest widths so the run is a measurement, not just a
    // non-panic.
    println!(
        "R1-E2E volume {} +/- {} | area {} +/- {} | rel {:e}",
        m.volume,
        m.volume_pad,
        m.surface_area,
        m.area_pad,
        2.0 * m.area_pad / m.surface_area
    );
    println!(
        "R1BITS eps={:?} area_pad_bits={:016x} surface_bits={:016x}",
        std::env::var("CAD_TOLERANCE_EPS").ok(),
        m.area_pad.to_bits(),
        m.surface_area.to_bits()
    );
    assert!(m.area_pad > 0.0, "an honest curved wall pays a width");
    assert!(
        debug_assertions_are_on(),
        "this probe is only a reading when the assert is compiled in"
    );
}

/// Assert that this build really does compile `debug_assert!` in —
/// otherwise the silence above is vacuous.
fn debug_assertions_are_on() -> bool {
    cfg!(debug_assertions)
}

/// The same body at the OTHER scalar lane. `Interval` is the second
/// `Decide` substrate; the gauge reads the same `RingInterval`
/// enclosure either way, so this is the cross-lane half of the E2E.
#[cfg(feature = "interval")]
#[test]
fn r1_e2e_gauge_is_silent_at_the_interval_lane() {
    use geom_core::Interval;
    let (sections, places) = r1_sections();
    let lofted =
        loft_body::<Interval>(&sections, &places, 2, Tol::witness()).expect("r1 loft builds");
    let m = topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
    println!(
        "R1-E2E(interval) area_pad {:?} surface {:?}",
        m.area_pad, m.surface_area
    );
}
