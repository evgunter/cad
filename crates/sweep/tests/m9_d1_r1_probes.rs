//! M9-D1 review probes (R1): the pole export is a construction fact.
//!
//! Ground truth in every probe is derived from the AUTHORED profile
//! geometry (canonical vertex 0 = lexicographic least point), never
//! from the construction under test: the export must land each
//! canonical index on the body vertex at that authored point, in
//! forward AND reversed authored orientation, for full revolves
//! (swept traversal always REVERSED), θ > 0 partials (reversed) and
//! θ < 0 partials (forward) — the traversal family the rejected
//! direction-inference shape failed on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::FRAC_PI_2;

use geom_core::Tol;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use revolve_common::*;
use sweep::{Revolution, Revolved, revolve};

/// The ball meridian, authored CCW: (0,−1) —arc(bulge 1)→ (0,1)
/// —axis line→ close. Canonical vertex 0 is (0,−1) (lex least).
fn ball_ccw() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])
}

/// The SAME ball meridian authored in the reversed (CW) vertex order:
/// (0,1) —arc(bulge −1, still through (1,0))→ (0,−1) —axis line→
/// close. Canonicalization must undo this; the poles must still land
/// south-at-0.
fn ball_cw() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 1.0), -1.0),
        ProfileVertex::new(p2(0.0, -1.0), 0.0),
    ])
}

/// y-coordinate of an exported pole vertex.
fn pole_y(t: &Revolved<f64>, l: usize, v: usize) -> f64 {
    let k = t.poles[l][v].unwrap_or_else(|| panic!("pole ({l},{v}) not exported"));
    t.body
        .get_point(t.body.get_vertex(k).unwrap().point)
        .unwrap()
        .y
}

fn assert_ball_poles(t: &Revolved<f64>) {
    assert_eq!(t.poles.len(), 1);
    assert_eq!(t.poles[0].len(), 2);
    // Canonical 0 = (0,−1) south, 1 = (0,1) north — authored ground
    // truth, the swap an off-by-one would fail.
    assert!(
        (pole_y(t, 0, 0) + 1.0).abs() < 1e-12,
        "canonical 0 is south"
    );
    assert!(
        (pole_y(t, 0, 1) - 1.0).abs() < 1e-12,
        "canonical 1 is north"
    );
    assert_ne!(t.poles[0][0], t.poles[0][1]);
}

#[test]
fn full_ball_reversed_authoring_exports_the_same_canonical_poles() {
    for lp in [ball_ccw(), ball_cw()] {
        let t = revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap();
        assert_all_tiers(&t.body);
        assert_eq!(counts(&t.body), (2, 2, 2, 0));
        assert_ball_poles(&t);
    }
}

#[test]
fn partial_ball_both_sweep_directions_export_the_same_canonical_poles() {
    // θ > 0 sweeps the REVERSED canonical chain; θ < 0 the forward
    // one. Both authored orientations, both directions.
    for theta in [FRAC_PI_2, -FRAC_PI_2] {
        for lp in [ball_ccw(), ball_cw()] {
            let t = revolve(
                &validated(vec![lp]),
                axis_y(),
                Revolution::Partial(theta),
                Tol::witness(),
            )
            .unwrap();
            // Tiers 1-2 only: the natural wedge's band face has no
            // rims and non-coplanar meridians, so tier 3's sphere
            // volume classification refuses typed (pre-existing
            // props_band_coplanar gap, recorded in the review).
            assert_eq!(topo::validate(&t.body), Ok(()));
            assert_eq!(topo::validate_closed(&t.body), Ok(()));
            assert_ball_poles(&t);
            // The pole is ONE body vertex serving both chains: the
            // wedge has exactly 2 vertices on the axis (plus none
            // elsewhere — the meridian has no off-axis vertex).
            assert_eq!(t.body.vertices().count(), 2);
        }
    }
}

/// Subdivided axis run: (0,−1) —arc→ (0,1) —line→ (0,0) —line→ close.
/// Canonical: v0=(0,−1), v1=(0,1), v2=(0,0). The FULL revolve deletes
/// the run's interior vertex (0,0); the export must say so with None
/// while both tips stay Some.
#[test]
fn full_subdivided_axis_run_exports_tips_and_omits_the_interior() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    // Same V2 E2 F2 ball; the interior axis vertex does not exist.
    assert_eq!(counts(&t.body), (2, 2, 2, 0));
    assert!((pole_y(&t, 0, 0) + 1.0).abs() < 1e-12);
    assert!((pole_y(&t, 0, 1) - 1.0).abs() < 1e-12);
    assert_eq!(
        t.poles[0][2], None,
        "deleted interior run vertex must export None"
    );
}

/// The same profile PARTIALLY revolved keeps the interior axis vertex
/// alive: THREE poles, every one exported at its canonical index.
#[test]
fn partial_subdivided_axis_run_exports_all_three_poles() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    // Tiers 1-2 only (same pre-existing tier-3 wedge gap as above).
    assert_eq!(topo::validate(&t.body), Ok(()));
    assert_eq!(topo::validate_closed(&t.body), Ok(()));
    assert!((pole_y(&t, 0, 0) + 1.0).abs() < 1e-12);
    assert!((pole_y(&t, 0, 1) - 1.0).abs() < 1e-12);
    assert!(pole_y(&t, 0, 2).abs() < 1e-12);
    assert_eq!(t.body.vertices().count(), 3);
}

/// Mixed on/off-axis (the dome): (0,0) —line→ (1,0) —quarter arc→
/// (0,1) —axis line→ close. Off-axis vertex (1,0) must export None
/// (it is addressed through rims); both poles Some.
#[test]
fn full_mixed_profile_exports_poles_only_at_pinned_vertices() {
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), b),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    // Canonical v0 = (0,0), v1 = (1,0), v2 = (0,1).
    assert!(pole_y(&t, 0, 0).abs() < 1e-12);
    assert_eq!(
        t.poles[0][1], None,
        "off-axis vertex must not export a pole"
    );
    assert!((pole_y(&t, 0, 2) - 1.0).abs() < 1e-12);
    assert!(
        t.rims[0][1].is_some(),
        "off-axis vertex is addressed through its rim"
    );
}
