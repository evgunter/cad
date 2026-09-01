//! **Issue 896, the construction-door half of the fixture question:
//! can any body reach `walk::loop_polygon`'s pole guard through the
//! revolve door?** Measured: no — and these rows pin the bars that
//! say so, so the day one of them opens the route question is re-asked
//! instead of silently changing.
//!
//! The candidate body is an annular sphere band (profile: sphere arc
//! from latitude 0.5 up to a top vertex `rho` off the axis, closed
//! away from the axis), the shape whose top rim junction would sit
//! ~`rho` from the sphere chart's UNDECLARED pole. For the junction to
//! enter the guard's band, `rho <= eps` is forced (the junction's
//! distance to the pole is at least the rim radius). What the door
//! does instead, measured at all three suite ε legs:
//!
//! * `rho = 0.9ε` — the vertex certifies ON the axis and, isolated on
//!   an off-axis profile, refuses `NonManifoldAxisContact`.
//! * `rho = 5ε` — inside the K band: `SliverRadius`, the
//!   `axis_vertex_radius` certification's indeterminate arm.
//! * larger `rho` — the pcurve certification lane (`MapResidual`,
//!   `pcurve_loop_continuity`, `LoopDiscontinuity` shapes at 20ε …
//!   1e6·ε depending on the leg) keeps refusing until the rim is
//!   macroscopic; `rho = 0.1` m builds and meshes watertight at every
//!   leg, with the guard quiet — the nearest-the-pole body the door
//!   admits, and the body the guard's red-first unit row
//!   (`walk::tests::a_rim_junction_inside_the_pole_band_trips_the_guard`)
//!   reuses.
//!
//! The middle regime is deliberately NOT pinned: which pcurve shape
//! refuses is that lane's own business and varies by leg. What this
//! suite claims is the two certified axis bars and the endpoints —
//! refusal inside the band, a clean quiet mesh far outside it.
//!
//! The import door's measurement is `step-import/tests/poleguard.rs`;
//! the residual route (direct Euler-operator assembly, which no
//! certification lane fronts) is recorded at the guard itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::*;
use geom_core::Tol;
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::{Revolution, RevolveError, revolve};
use topo::Body;

/// The annular band profile with its top vertex `rho` off the axis,
/// on the unit sphere about the origin.
fn band_body(rho: f64) -> Result<Body<f64>, RevolveError> {
    let (h, rc) = (0.5f64.sin(), 0.5f64.cos());
    let yt = (1.0 - rho * rho).sqrt();
    let bulge = ((yt.atan2(rho) - h.atan2(rc)) / 4.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(rc, h), bulge),
        ProfileVertex::new(p2(rho, yt), 0.0),
        ProfileVertex::new(p2(0.3, 1.3), 0.0),
        ProfileVertex::new(p2(1.1, 0.9), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .map(|r| r.body)
}

/// Inside the band the door refuses at the two certified axis bars —
/// pinned by variant, identical at every suite ε (the bars scale with
/// it).
#[test]
fn a_rim_inside_the_pole_band_refuses_at_the_certified_axis_bars() {
    let eps = Tol::witness().get().eps;
    let pinned = band_body(0.9 * eps).map(|_| ());
    assert!(
        matches!(pinned, Err(RevolveError::NonManifoldAxisContact { .. })),
        "a vertex certified on-axis but isolated must refuse NonManifoldAxisContact; \
         got {pinned:?}"
    );
    let sliver = band_body(5.0 * eps).map(|_| ());
    assert!(
        matches!(sliver, Err(RevolveError::SliverRadius { .. })),
        "a vertex radius inside the K band must refuse SliverRadius; got {sliver:?}"
    );
}

/// Far outside the band the same shape builds and meshes watertight,
/// and the guard beside `pole_v` stays quiet — the door's admitted
/// remainder never enters the band it would fire on.
#[test]
fn the_nearest_admitted_rim_meshes_watertight_with_the_guard_quiet() {
    let body = band_body(0.1).expect("rho = 0.1 m clears every certified bar");
    topo::validate_geometric(&body, Tol::witness()).expect("tier 3");
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("meshes");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}
