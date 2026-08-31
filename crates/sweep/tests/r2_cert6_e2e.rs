//! **R2 review probe for CERT-6 (PR 1366): the body-level e2e.**
//!
//! My own loft (not the shape (iii) fixture) through the public door
//! `loft_body` + `topo::props::mass_properties`, with debug
//! assertions on: an honest body certifies with the A2 gauge silent,
//! at both scalar lanes. The corruption half of the exercise is the
//! env-driven plant in `area_midpoint_taylor` (`R2_AREA_WIDEN`,
//! committed on this probe branch only) — with it set, THESE tests
//! are the panic witnesses.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::quad;
use geom_core::Tol;
use geom_core::{Affine3, Vec3};
use sweep::{Section, loft_body};

/// Squares at z = 0 and z = 2 with a kite-ish quad at z = 1 — a
/// non-affine middle, so the walls are genuinely curved degree-2
/// skins (my own sections, not the acceptance fixture's).
fn r2_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let kite = [(-1.5, -1.0), (1.5, -1.0), (0.7, 1.0), (-0.7, 1.0)];
    let sections = vec![quad(square), quad(kite), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    (sections, places)
}

#[test]
fn r2_own_loft_certifies_with_the_gauge_live() {
    let (sections, places) = r2_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("loft builds");
    let m = topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
    println!(
        "R2 e2e loft: volume {} ± {}, area {} ± {}",
        m.volume, m.volume_pad, m.surface_area, m.area_pad
    );
    assert!(m.surface_area > 0.0 && m.area_pad.is_finite() && m.area_pad > 0.0);
}

#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::interval::Interval;

    #[test]
    fn r2_own_loft_certifies_at_interval() {
        let (sections, places) = r2_sections();
        let lofted =
            loft_body::<Interval>(&sections, &places, 2, Tol::witness()).expect("loft builds");
        let m =
            topo::props::mass_properties(&lofted.body, Tol::witness()).expect("mass properties");
        assert!(geom_core::Bounds::hi(m.area_pad) > 0.0);
    }
}
