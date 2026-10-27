//! **Is a hairline lune reachable from a certified door?** (review
//! probe for the rim-free wedge, issue 542.)
//!
//! The sphere arm decides a lune's pole junction against a cusp
//! (`props_sphere_loop_cusp`): the chord `2·sin(δ/2)` between one
//! meridian's tangent at the pole and the other's reversed, for
//! meridians `δ` apart, levered by the SPHERE's radius `R`. `revolve`
//! gates its angle on `r_max·|θ|` and `r_max·(τ − |θ|)`, where `r_max` is the whole
//! PROFILE's radial extent. The two levers are the same number only
//! for the bare ball; a profile carrying anything at ten times the
//! sphere's radius opens a window of angles that `revolve` calls
//! definite and the flux lane calls coincident.
//!
//! The body below is that profile: a flat ring of outer radius `L`
//! with a hemispherical dimple of radius `r` on its axis (the
//! rectangle minus the half-disc), so `r_max = L` and the dimple's
//! wall is a rimless sphere band. The window row refuses the
//! coincident pair typed as the cusp it is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point2, Tol};
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
        (Point2::new(0.0, -2.0 * r), 0.0),
        (Point2::new(l, -2.0 * r), 0.0),
        (Point2::new(l, 2.0 * r), 0.0),
        (Point2::new(0.0, 2.0 * r), 0.0),
        // The dimple's meridian: a semicircle DOWN the axis bulging
        // into the material side, so the wall is a rimless sphere
        // band whose two meridians are the sweep's end copies.
        (Point2::new(0.0, r), -1.0),
        (Point2::new(0.0, -r), 0.0),
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
/// is being read as a lune (the meridians are a quarter turn apart).
#[test]
fn the_dimpled_ring_measures_its_closed_form_at_a_quarter_turn() {
    let l = 1.0;
    let theta = core::f64::consts::FRAC_PI_2;
    let vp = validated(vec![dimpled_ring(l)]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&t.body),
            Tol::witness(),
        ),
        Tol::witness(),
    )
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
/// arrives as a pair of meridians on one half-plane within the band.
/// Each meridian, levered at its faces' extent, is the sphere wall's
/// and end wall's `Intersection`; the coincidence is the props door's
/// reading at the sphere's radius, not the edges'.
/// `mass_properties` — the public props door — refuses it typed under
/// `props_sphere_loop_cusp` (the loop turns back at the poles),
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
        "the props door's sphere lever must read the meridians' pole junction as a cusp"
    );
    let vp = validated(vec![dimpled_ring(l)]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
    // Each meridian is levered at its faces' extent (`topo::wedge_reach`),
    // the end wall's `r_max` across, not at the sphere's: the sphere wall
    // meets each end wall at a definite corner, so both store the faces'
    // `Intersection`, where a meridian levered at its own length read a
    // declared `OnPair`. The cusp is the props door's question alone.
    let meridians: Vec<_> = t
        .body
        .edges()
        .filter_map(|(_, e)| {
            let c = t.body.get_curve_geom(e.curve)?.certified()?;
            matches!(c.carrier(), geom::Curve3::Circle { radius, .. } if (radius - R_SPHERE).abs() < 1e-12)
                .then(|| c.description().clone())
        })
        .collect();
    assert_eq!(
        meridians.len(),
        2,
        "the dimple's wall is bounded by two meridians"
    );
    assert!(
        meridians
            .iter()
            .all(|d| matches!(d, geom_brep::EdgeDescription::Intersection { .. })),
        "each meridian is its faces' corner: {meridians:?}"
    );
    let tier3 = topo::validate_geometric(&t.body, Tol::witness());
    assert!(
        tier3.is_err(),
        "the full tier-3 walk refuses the hairline body"
    );
    println!("[bool5r2] tier 3 on the hairline ring: {tier3:?}");
    // Raw door: the measured body is a body tier 3 cannot measure (VolumeUncomputable), whose typed refusal is the subject.
    let got = topo::test_support::measure_unvalidated(
        &t.body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .map(|m| m.enclosure.volume);
    assert!(
        matches!(
            &got,
            Err(topo::MassPropsError::Face {
                source: geom_brep::PropsError::SphereLoop {
                    what: "props_sphere_loop_cusp"
                },
                ..
            })
        ),
        "the props door refuses the coincident pair typed, got {got:?}"
    );
}
