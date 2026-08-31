//! BOOL-2 R1 review probes — adversarial rows against the ray×cone arm
//! (PR 1425, issue 1011's cone half). Each probe attacks a claim the
//! PR body makes, by execution through public doors only.
//!
//! Attack map:
//! - generator-parallel schedule rays (the `bool_ray_cone_lead` Zero
//!   site): a cone whose half-angle equals the angle between the first
//!   TWO schedule members and its axis, so both graze and the third
//!   answers — the retry recovers rather than answering wrong;
//! - lines through the apex (every such ray has an exactly zero
//!   discriminant, since `G` restricted to the line is `A(t − t₀)²`);
//! - rim-grazing rays (trim-boundary Zero on the slant window);
//! - azimuth-window and slant-window edge straddles at offsets my own,
//!   not the suite's `away()`;
//! - the arm on a body 20× smaller than the doors suite's unit bodies
//!   (the `away()` clamp is the ROWS' crutch; the arm itself must be
//!   scale-honest);
//! - the near-apex interior, where the incidence lever (the local
//!   radius) is small;
//! - an e2e through public doors: revolve → union → tessellate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Point3, Tol, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use revolve_common::*;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid};

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

fn pis(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    point_in_solid(body, q, band(), Tol::witness()).unwrap()
}

/// The doors suite's full cone: base disc radius 1 at y = 0, apex at
/// (0, 1, 0), half-angle π/4.
fn cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The doors suite's frustum: base radius 1 at y = 0, top radius 0.5
/// at y = 1; virtual apex (0, 2, 0).
fn frustum() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.5, 1.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The doors suite's quarter cone (swept quadrant x > 0, z < 0).
fn quarter_cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// **A cone whose generators run along the first two schedule rays.**
/// Apex at the origin, axis (1, 1, 0)/√2, half-angle π/4: the x-axis
/// and the y-axis are generators, so schedule members [1,0,0] and
/// [0,1,0] are both generator-PARALLEL — `A = (d·â)² − cos²α` is zero
/// to f64 rounding, five-plus orders inside the band — and the arm must
/// graze both and answer with the third member, never guess a parity
/// from a degenerate quadratic.
fn tilted_cone() -> Body<f64> {
    // Profile in the XY sketch plane: apex (0,0) on the axis, (1,0) a
    // base-rim point (on the x-axis generator), (0.5,0.5) on the axis.
    // Edge (1,0)→(0.5,0.5) is perpendicular to the axis (a disc);
    // edge (0.5,0.5)→(0,0) lies on the axis (omitted).
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.5, 0.5)]);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(1.0, 1.0).normalize(),
    };
    revolve(&validated(vec![lp]), axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// Generator-parallel leading coefficient: the first two schedule rays
/// graze on the tilted cone, and the door still answers — through the
/// third — on all three verdicts. This is the `bool_ray_cone_lead`
/// Zero site's reachability row: red if the degenerate quadratic is
/// ANSWERED (a linear root would count one crossing where the arm
/// cannot certify a pair) rather than retried.
#[test]
fn probe_generator_parallel_schedule_rays_graze_and_the_door_recovers() {
    let body = tilted_cone();
    assert_all_tiers(&body);

    // Interior: strictly inside the half-angle and short of the cap.
    for q in [Point3::new(0.5, 0.3, 0.05), Point3::new(0.3, 0.25, -0.02)] {
        assert_eq!(pis(&body, q), SolidContainment::In, "interior {q:?}");
    }
    // Exterior: outside the half-angle, and beyond the cap plane.
    for q in [
        Point3::new(0.9, -0.3, 0.0),
        Point3::new(0.8, 0.8, 0.0),
        Point3::new(-0.2, -0.2, 0.1),
    ] {
        assert_eq!(pis(&body, q), SolidContainment::Out, "exterior {q:?}");
    }
    // ON the wall: a base-rim generator point (0.5, 0, 0) is on the
    // x-axis generator, and (0, 0.5, 0) on the y-axis one.
    for q in [Point3::new(0.5, 0.0, 0.0), Point3::new(0.0, 0.5, 0.0)] {
        assert_eq!(pis(&body, q), SolidContainment::OnBoundary, "on-wall {q:?}");
    }
}

/// A schedule ray whose LINE passes through the apex has `G` restricted
/// to the line equal to `A(t − t_apex)²` — an exactly zero
/// discriminant, whatever the direction. The door must graze it and
/// recover through the next member, and the recovery must not misread
/// the mirror nappe it then meets.
#[test]
fn probe_rays_through_the_apex_graze_and_recover() {
    let body = cone();
    // Apex height, z = 0: schedule ray 1 ([1,0,0]) runs through the
    // apex from every one of these. All are free space.
    for q in [
        Point3::new(0.35, 1.0, 0.0),
        Point3::new(-0.5, 1.0, 0.0),
        Point3::new(-2.0, 1.0, 0.0),
    ] {
        assert_eq!(
            pis(&body, q),
            SolidContainment::Out,
            "apex-height free space {q:?}"
        );
    }
}

/// Near-apex, both sides, at my own offsets (2e-3, not the suite's
/// `away()`): laterally beside the apex is Out, just below it inside
/// the material is In — the row where the incidence margin's lever
/// (the local radius) is at its smallest.
#[test]
fn probe_the_apex_neighbourhood_at_small_levers() {
    let body = cone();
    // FOUND BY THIS PROBE, and load-bearing here: near the apex the
    // ray×cone discriminant scales as the SQUARE of the distance to
    // the apex, so the honest-escalation halo around the apex has
    // radius ~√(K·ε·v_ext) — a √ε length, NOT the ε-order band the
    // doors suite's `away()` comment reasons from ("escalate is
    // within an order or two of eps"). A fixed 2e-3 offset sits
    // inside that halo at the ε = 1e-6 row and the door (correctly,
    // honestly) refuses `RayExhausted` instead of answering. The
    // offset here therefore scales as √ε; at K = 1000 the doors
    // suite's own near-apex row fails the same way at default ε.
    let d = (1e6 * Tol::witness().get().eps).sqrt().clamp(2e-3, 0.05);
    assert_eq!(
        pis(&body, Point3::new(d, 1.0, 0.0)),
        SolidContainment::Out,
        "laterally beside the apex"
    );
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0 - 1.5 * d, 0.0)),
        SolidContainment::In,
        "on the axis just below the apex"
    );
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0 + d, 0.0)),
        SolidContainment::Out,
        "on the axis just above the apex (mirror side)"
    );
}

/// Rim-grazing rays: from apex/base-rim height the first schedule ray
/// meets the frustum's carrier exactly at the slant window's bound —
/// a trim-boundary Zero, which must graze and retry, and the retry
/// must land Out (free space), never OnBoundary.
#[test]
fn probe_rim_grazing_rays_retry_and_answer_out() {
    let body = frustum();
    // Top-rim height (y = 1): ray 1 hits the carrier at x = ±0.5,
    // exactly the window's top bound. Base-rim height (y = 0): at
    // x = ±1, exactly the bottom bound (and the ray lies IN the base
    // plane besides).
    for q in [Point3::new(-2.0, 1.0, 0.0), Point3::new(-2.0, 0.0, 0.0)] {
        assert_eq!(pis(&body, q), SolidContainment::Out, "rim-height ray {q:?}");
    }
    // Control: the rim itself is ON the boundary.
    assert_eq!(
        pis(&body, Point3::new(0.5, 1.0, 0.0)),
        SolidContainment::OnBoundary,
        "the top rim"
    );
}

/// Slant-window edge straddle on the frustum's carrier, at offsets my
/// own: just inside the window is boundary, just past it is free
/// space.
#[test]
fn probe_slant_window_edge_straddle() {
    let body = frustum();
    let d = 5e-3;
    let on = |y: f64| {
        let rho = 1.0 - y / 2.0;
        Point3::new(rho, y, 0.0)
    };
    assert_eq!(pis(&body, on(1.0 - d)), SolidContainment::OnBoundary);
    assert_eq!(pis(&body, on(-0.0 + d)), SolidContainment::OnBoundary);
    assert_eq!(pis(&body, on(1.0 + d)), SolidContainment::Out);
    assert_eq!(pis(&body, on(-d)), SolidContainment::Out);
}

/// Azimuth-window edge straddle on the quarter cone's carrier: the
/// same radius and height, ±0.02 rad across the fan wall's meridian.
#[test]
fn probe_azimuth_window_edge_straddle() {
    let body = quarter_cone();
    let on = |phi: f64| {
        let (s, c) = phi.sin_cos();
        Point3::new(0.5 * c, 0.5, 0.5 * s)
    };
    // The swept quadrant is x > 0, z < 0: physical azimuth (−π/2, 0).
    assert_eq!(pis(&body, on(-0.02)), SolidContainment::OnBoundary);
    assert_eq!(
        pis(&body, on(-core::f64::consts::FRAC_PI_2 + 0.02)),
        SolidContainment::OnBoundary
    );
    assert_eq!(pis(&body, on(0.02)), SolidContainment::Out);
    assert_eq!(
        pis(&body, on(-core::f64::consts::FRAC_PI_2 - 0.02)),
        SolidContainment::Out
    );
    // ON the mirror nappe at an azimuth INSIDE the swept window: only
    // the nappe posture separates this from the face.
    let (s, c) = (-0.8_f64).sin_cos();
    assert_eq!(
        pis(&body, Point3::new(0.3 * c, 1.3, 0.3 * s)),
        SolidContainment::Out,
        "the trimmed face's mirror sheet, azimuth in-window"
    );
}

/// The ARM is scale-honest even though the doors suite's probe-offset
/// clamp is stated for unit bodies: a cone 20× smaller classifies
/// interior/exterior/boundary at offsets proportionate to ITS size.
#[test]
fn probe_a_small_cone_classifies_at_its_own_scale() {
    let s = 0.05;
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(s, 0.0), p2(0.0, s)]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_all_tiers(&body);
    assert_eq!(
        pis(&body, Point3::new(0.2 * s, 0.3 * s, 0.15 * s)),
        SolidContainment::In
    );
    assert_eq!(
        pis(&body, Point3::new(0.9 * s, 0.9 * s, 0.0)),
        SolidContainment::Out
    );
    let (y, phi) = (0.4 * s, 0.7_f64);
    let (sn, cs) = phi.sin_cos();
    let rho = s - y;
    assert_eq!(
        pis(&body, Point3::new(rho * cs, y, rho * sn)),
        SolidContainment::OnBoundary
    );
    assert_eq!(
        pis(&body, Point3::new(0.0, s, 0.0)),
        SolidContainment::OnBoundary,
        "the small cone's apex"
    );
}

/// The frustum's virtual apex still escalates typed when approached
/// from a DIFFERENT body scale — the exhaustion is the geometry's, not
/// the fixture's.
#[test]
fn probe_small_frustum_virtual_apex_still_exhausts() {
    let s = 0.05;
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(s, 0.0), p2(0.5 * s, s), p2(0.0, s)]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let err = point_in_solid(
        &body,
        Point3::new(0.0, 2.0 * s, 0.0),
        band(),
        Tol::witness(),
    )
    .unwrap_err();
    assert!(
        matches!(err, PointInSolidError::RayExhausted),
        "expected RayExhausted at the virtual apex, got {err:?}"
    );
}

/// E2E through public doors: revolve mints the cone body, the boolean
/// walks the containment door the arm serves, and the result
/// tessellates watertight.
#[test]
fn probe_e2e_revolve_union_tessellate() {
    let a = quarter_cone();
    let lp = ProfileLoop::polygon([p2(5.0, 0.0), p2(6.0, 0.0), p2(6.0, 1.0), p2(5.0, 1.0)]);
    let plane = SketchPlane::new(geom_core::Affine3::translation(Vec3::new(0.0, 0.0, -1.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let b = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let out = topo::union(&a, &b, Tol::witness()).expect("the cone-bearing union assembles");
    let result = out.body().expect("non-empty");
    assert_eq!(topo::validate_closed(&result.body), Ok(()));
    let mesh = mesh::tessellate(&result.body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// Reproduction of the PR's out-of-unit finding, pinned OUTSIDE the
/// green suite (`#[ignore]`): the PLANAR arm misreads the interior of
/// a revolved half-disc cap. Body: a rectangle revolved about the
/// y-axis — a cylinder, no cone anywhere. Truth at (0.3, 0, 0.2) (on
/// the base cap, ρ ≈ 0.36 < 0.5) is OnBoundary; the door answers Out.
/// This row asserts the MISREAD so it goes red the day the
/// `splitting/` fix lands — delete it then.
#[test]
#[ignore = "documents another unit's defect (planar half-disc cap misread); run explicitly"]
fn probe_planar_cap_misread_reproduction() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(0.5, 0.0), p2(0.5, 0.4), p2(0.0, 0.4)]);
    let body = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let got = pis(&body, Point3::new(0.3, 0.0, 0.2));
    assert_eq!(
        got,
        SolidContainment::Out,
        "the misread reproduced by the PR body; if this is now OnBoundary the \
         splitting/ defect is fixed and this probe should be deleted"
    );
}
