//! M2 PR 7 acceptance: exact-B-rep mass properties vs closed forms
//! (f64 lane), plus the Pappus sanity cross-check for revolved bodies
//! and the +V tier-3 invariant on every acceptance shape.
//!
//! The closed forms are independent hand derivations (volumes/areas of
//! the acceptance solids); `topo::mass_properties` must reproduce them
//! to floating accuracy — these are closed-form-vs-closed-form
//! comparisons, not quadrature convergence, so the tolerance is tight.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2, TAU};
use profile::RawLoop;

use profile::{ProfileLoop, ProfileVertex};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, mass_properties, validate_geometric};

use geom_core::Tol;
use revolve_common::{axis_y, full_pappus_y, p2, validated};

/// Tight relative tolerance for closed-form-vs-closed-form agreement.
fn assert_close(what: &str, got: f64, expect: f64) {
    let scale = expect.abs().max(1.0);
    assert!(
        (got - expect).abs() <= 1e-12 * scale,
        "{what}: got {got}, expected {expect} (rel err {})",
        ((got - expect) / scale).abs()
    );
}

fn check(body: &Body<f64>, what: &str, volume: f64, area: f64) {
    validate_geometric(body, Tol::witness())
        .expect("acceptance body must be tier-3 valid (incl. +V)");
    let props = mass_properties(body, Tol::witness()).expect("mass properties must compute");
    assert_close(&format!("{what} volume"), props.volume, volume);
    assert_close(&format!("{what} area"), props.surface_area, area);
}

// ---- extrusions -------------------------------------------------------

#[test]
fn l_prism_matches_closed_forms() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let body = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    // Profile area 3, perimeter 8, height 1.
    check(&body, "L-prism", 3.0, 2.0 * 3.0 + 8.0);
}

#[test]
fn square_with_two_vertex_hole_matches_closed_forms() {
    // 4×4 square with a centered circular hole of radius 1, the closed
    // carrier split into two bulge-1 semicircle segments (genus 1).
    let outer = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 0.0), 1.0),
        ProfileVertex::new(p2(-1.0, 0.0), 1.0),
    ]);
    let body = extrude(
        &validated(vec![outer, hole]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    // V = (16 − π)·1; A = 2(16 − π) + 16 + 2π = 48.
    check(&body, "holed prism", 16.0 - PI, 48.0);
}

// ---- revolves ---------------------------------------------------------

#[test]
fn washer_matches_closed_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    // V = π(2² − 1²)·1 = 3π; A = 4π + 2π + 2·3π = 12π.
    check(&t.body, "washer", 3.0 * PI, 12.0 * PI);
    let pappus = full_pappus_y(&t);
    let v = mass_properties(&t.body, Tol::witness()).unwrap().volume;
    assert!(
        (v - pappus).abs() < 1e-3 * v,
        "washer Pappus cross-check: exact {v} vs quadrature {pappus}"
    );
}

#[test]
fn ball_matches_closed_forms() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "ball", 4.0 * PI / 3.0, 4.0 * PI);
    let pappus = full_pappus_y(&t);
    let v = mass_properties(&t.body, Tol::witness()).unwrap().volume;
    assert!(
        (v - pappus).abs() < 1e-3 * v,
        "ball Pappus: {v} vs {pappus}"
    );
}

#[test]
fn cone_matches_closed_forms() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    // r = h = 1: V = π/3; A = πr√(r²+h²) + πr² = π(√2 + 1).
    check(&t.body, "cone", PI / 3.0, PI * (SQRT_2 + 1.0));
    let pappus = full_pappus_y(&t);
    let v = mass_properties(&t.body, Tol::witness()).unwrap().volume;
    assert!(
        (v - pappus).abs() < 1e-3 * v,
        "cone Pappus: {v} vs {pappus}"
    );
}

#[test]
fn donut_matches_closed_forms() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, -0.5), 1.0),
        ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    // R = 2, r = 1/2: V = 2π²Rr² = π²; A = 4π²Rr = 4π².
    check(&t.body, "donut", PI * PI, 4.0 * PI * PI);
    let pappus = full_pappus_y(&t);
    let v = mass_properties(&t.body, Tol::witness()).unwrap().volume;
    assert!(
        (v - pappus).abs() < 1e-3 * v,
        "donut Pappus: {v} vs {pappus}"
    );
}

#[test]
fn partial_wedge_matches_theta_scaled_closed_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    // θ = π/2 (quarter): V = 3π/4; A = θ-scaled walls + 2 meridian
    // rectangles: (4π + 2π + 6π)/4 + 2·(1·1) = 3π + 2.
    check(&t.body, "wedge", 3.0 * PI / 4.0, 3.0 * PI + 2.0);
}

#[test]
fn axis_touching_wedge_matches_closed_forms() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    // Quarter cylinder: V = θr²h/2 = π/4; A = θrh + 2·θr²/2 + 2·1
    //                                      = π/2 + π/2 + 2.
    check(&t.body, "axis wedge", PI / 4.0, PI + 2.0);
}

#[test]
fn near_full_wedge_matches_theta_scaled_closed_forms() {
    // A non-dyadic near-full angle (past 3π/2 — the PR 6 blocker
    // family's territory, exercised here on the exact side).
    let theta = TAU - 0.01;
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .unwrap();
    let s = theta / TAU;
    check(
        &t.body,
        "near-full wedge",
        3.0 * PI * s,
        12.0 * PI * s + 2.0,
    );
}

#[test]
fn holed_wedge_matches_theta_scaled_closed_forms() {
    // The `kemr`-ring path: a wedge whose start lamina carries a hole,
    // planted through the ring rather than grown from the mvfs seed.
    // Outer square x ∈ [1, 3], y ∈ [0, 2]; hole x ∈ [1.5, 2.5],
    // y ∈ [0.5, 1.5]; θ = π/2 about +y, so r = x.
    let outer = ProfileLoop::polygon([p2(1.0, 0.0), p2(3.0, 0.0), p2(3.0, 2.0), p2(1.0, 2.0)]);
    let hole = ProfileLoop::polygon([p2(1.5, 0.5), p2(2.5, 0.5), p2(2.5, 1.5), p2(1.5, 1.5)]);
    let t = revolve(
        &validated(vec![outer, hole]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    // Pappus, θ-scaled, and every term is the hole's as well as the
    // outer's — a chain built at the wrong loop or in the wrong order
    // moves one of them. V = θ·∫∫r dA = θ·(8 − 2) = 3π. Walls =
    // θ·Σ r̄·L: outer (2 + 6 + 4 + 4)θ, hole (1.5 + 2.5 + 2 + 2)θ,
    // so 24θ = 12π. Caps: two planar laminae of area 4 − 1 = 3.
    check(&t.body, "holed wedge", 3.0 * PI, 12.0 * PI + 6.0);
}
