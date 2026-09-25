//! **Is the coplanar arm's blind spot reachable from a certified
//! door?** (review probe for the rim-free wedge arm, issue 542.)
//!
//! The rimless sphere branch decides `props_band_coplanar` on
//! `R·‖n_B × n_A‖ = R·|sin δ|` and takes `Δu = π` on a `Zero`, where
//! `R` is the SPHERE's radius. `revolve` gates its angle on
//! `r_max·|θ|` and `r_max·(τ − |θ|)`, where `r_max` is the whole
//! PROFILE's radial extent. The two levers are the same number only
//! for the bare ball; a profile carrying anything at ten times the
//! sphere's radius opens a window of angles that `revolve` calls
//! definite and the flux lane calls coincident.
//!
//! The body below is that profile: a flat ring of outer radius `L`
//! with a hemispherical dimple of radius `r` on its axis (the
//! rectangle minus the half-disc), so `r_max = L` and the dimple's
//! wall is a rimless sphere band. Adopted by the unit with the window
//! row inverted: the coplanar branch refuses the coincident pair typed
//! (`props_band_opposite`) where it measured it at `π`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::Tol;
use profile::{ProfileLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, revolve};

/// The dimple's radius, in metres.
const R_SPHERE: f64 = 0.010;

/// The ring of outer radius `l` with the hemispherical dimple of
/// radius [`R_SPHERE`] sunk into its axis: the rectangle
/// `[0, l] × [−2r, 2r]` less the half-disc of radius `r` at the
/// origin. Counter-clockwise, so the material is the ring and the
/// dimple's sphere wall faces INTO the sphere.
fn dimpled_ring(l: f64) -> ProfileLoop<f64> {
    let r = R_SPHERE;
    bulge_loop(vec![
        (p2(0.0, -2.0 * r), 0.0),
        (p2(l, -2.0 * r), 0.0),
        (p2(l, 2.0 * r), 0.0),
        (p2(0.0, 2.0 * r), 0.0),
        // The dimple's meridian: a semicircle DOWN the axis bulging
        // into the material side, so the wall is a rimless sphere
        // band whose two meridians are the sweep's end copies.
        (p2(0.0, r), -1.0),
        (p2(0.0, -r), 0.0),
    ])
}

/// Pappus for that region swept through `theta`: the rectangle's
/// `2·r·l²` less the half-disc's `2r³/3`, times the angle.
fn exact_volume(l: f64, theta: f64) -> f64 {
    let r = R_SPHERE;
    theta.abs() * (2.0 * r * l * l - 2.0 * r.powi(3) / 3.0)
}

/// **The control**: the same body at an ordinary angle measures its
/// closed form, so the fixture itself is sound and the dimple's wall
/// is being read by the wedge arm (the meridians are a quarter turn
/// apart, definitely non-coplanar).
#[test]
fn the_dimpled_ring_measures_its_closed_form_at_a_quarter_turn() {
    let l = 1.0;
    let theta = core::f64::consts::FRAC_PI_2;
    let vp = validated(vec![dimpled_ring(l)]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let v = topo::mass_properties(&t.body, Tol::witness())
        .unwrap()
        .volume;
    let want = exact_volume(l, theta);
    assert!(
        (v - want).abs() / want < 1e-12,
        "volume {v:.15e} != {want:.15e}"
    );
}

/// **The window, executed, and what stands in it.** `r_max = 100` and
/// `R = 0.01`, so the angle is definite at `revolve`'s lever
/// (`θ·r_max ≫ 10·zero`) and coincident at the sphere's
/// (`R·sin θ ≤ zero`): `revolve` builds the body and the dimple's wall
/// reaches the coplanar branch as a pair of meridians on one
/// half-plane. `mass_properties` — the public props door — refuses it
/// typed under `props_band_opposite` (the loop reverses at the poles),
/// where a lune measured at `Δu = π` would have been short by
/// `(2/3)R³(π − θ)`. The full tier-3 walk refuses too; whichever of
/// its checks reports first (the knife-edge caps' `LaminaWedge` or the
/// volume), the props door's own answer is the typed refusal, which is
/// the half `point_in_solid`'s at-infinity arm asks for.
///
/// Stated against the run's own ε rather than a literal: the row
/// asserts the window it uses is the window it computed.
#[test]
fn a_hairline_revolve_of_a_dimpled_ring_refuses_typed_at_the_props_door() {
    let zero = eps();
    let l = 100.0;
    let theta = 0.5 * zero / R_SPHERE;
    assert!(
        theta * l >= 10.0 * zero,
        "revolve must call this angle definite"
    );
    assert!(
        theta.sin() * R_SPHERE <= zero,
        "the sphere lever must call the meridian pair coincident"
    );
    let vp = validated(vec![dimpled_ring(l)]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
    let tier3 = topo::validate_geometric(&t.body, Tol::witness());
    assert!(
        tier3.is_err(),
        "the full tier-3 walk refuses the hairline body"
    );
    println!("[bool5r2] tier 3 on the hairline ring: {tier3:?}");
    let got = topo::mass_properties(&t.body, Tol::witness()).map(|m| m.volume);
    assert!(
        matches!(
            &got,
            Err(topo::MassPropsError::Face {
                source: geom_brep::PropsError::NotIsoRectangle { what },
                ..
            }) if what.starts_with("a rimless sphere face whose coplanar meridians share one half-plane")
        ),
        "the props door refuses the coincident pair typed, got {got:?}"
    );
}
