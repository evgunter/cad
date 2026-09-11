//! M6-3 adversarial review probes (Legs A/F + deviation 3): the
//! ReversedStacking refusal door (fired by NO shipped test), and the
//! revolve-vs-tube minor-radius drift the rider retires (measured,
//! not assumed).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{LoftError, Revolution, RevolveAxis, loft_body, revolve};

use crate::common;
use common::quad;
use geom_core::Tol;

/// Deviation 3's substituted behavior, executed: sections stacking
/// definitely AGAINST the base normal refuse
/// `LoftError::ReversedStacking` typed — no shipped test fires this
/// door (the probe is that test).
#[test]
fn probe_reversed_stacking_refuses_typed() {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    // Stacking DOWN the base normal: the shape (iii) fixture with the
    // z-translations negated.
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, -1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, -2.0)),
    ];
    match loft_body::<f64>(&sections, &places, 2, Tol::witness()) {
        Err(LoftError::ReversedStacking) => {}
        other => panic!("expected ReversedStacking, got {other:?}"),
    }
}

/// The zero-displacement limit refuses as degenerate, not reversed
/// (the trilean's Zero arm — the other side of the same decide).
#[test]
fn probe_coincident_stacking_refuses_degenerate() {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(square), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::identity(),
        Affine3::identity(),
    ];
    match loft_body::<f64>(&sections, &places, 2, Tol::witness()) {
        Err(LoftError::DegenerateStacking | LoftError::Skin(_)) => {}
        other => panic!("expected a degenerate refusal, got {other:?}"),
    }
}

/// The 56-ulp story's substance, measured: the revolve door's
/// reconstructed torus minor radius vs the intended one, on the
/// m6_tube fixture's own donut parameters. The tube door's is
/// bit-exact by pin (`m6_tube`); here the REVOLVE drift is measured
/// so the "retired for this door" claim has an executed magnitude
/// behind it.
#[test]
fn probe_measure_revolve_minor_radius_drift() {
    let (major, minor) = (2.0f64, 0.5f64);
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(major - minor, 0.0), 1.0),
        ProfileVertex::new(Point2::new(major + minor, 0.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let rev = revolve::<f64>(&vp, axis, Revolution::Full, Tol::witness())
        .expect("the revolve donut builds")
        .body;
    let rev_minor = rev
        .faces()
        .find_map(|(_, f)| match rev.get_surface(f.surface) {
            Some(geom::Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .expect("revolve donut stores a torus");
    let drift_ulps = (rev_minor.to_bits() as i64)
        .wrapping_sub(minor.to_bits() as i64)
        .abs();
    println!("revolve minor-radius drift: {drift_ulps} ulps ({rev_minor:e} vs {minor:e})");
    // No assertion on the magnitude (the revolve's own business, per
    // the m6_tube note); the probe records it.
}
