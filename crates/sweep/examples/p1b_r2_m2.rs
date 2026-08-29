//! **PCURVE P-1b, reviewer R2 — MAJOR-2 measured.**
//!
//! `replace_face.rs`'s retirement text says the two *"not a rigid
//! translation"* refusals are "unreachable for a body AT REST, because
//! tier 3's transience fence refuses a scaffold there", and the tour
//! row demonstrates the premise by showing its fixtures carry no
//! scaffolding description.
//!
//! That premise is about the SCAFFOLD arm. This unit ADDED a second
//! arm — `carried_declaration()` — raising the SAME variant for a chart
//! image whose authority is `Declared`, which is exactly what the
//! fence's conversion produces from the pushforwards that used to raise
//! the retired arm.
//!
//! Fixture: the revolved ball. Its angle-π meridian is a declared image
//! in the sphere's own chart (the unit's own `revolve_ball` row asserts
//! it), and a sphere's offset is a homothety, so `transport_curve`
//! answers `delta = None`.
//!
//! Prints; asserts nothing. Evidence for a review, not a gate.

// A reviewer's evidence binary, not a library door: it fails LOUDLY on
// any unexpected state, which is the point. Same allowance the test
// targets carry, for the same reason.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Tol, Vec2};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn main() {
    // The half disc: semicircle (0,-1) -> (1,0) -> (0,1), closed on the
    // axis; revolved a full turn about y. (`revolve_ball`'s fixture.)
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
        ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the half disc validates");
    let ball: Body<f64> = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the ball revolves")
    .body;

    // Premise 1 — the one the tour row proves: nothing scaffolded.
    let scaffolds = ball
        .edges()
        .filter(|(_, e)| {
            ball.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_)))
        })
        .count();

    // Premise 2 — the one it does NOT state: is anything Declared?
    let declared = ball
        .edges()
        .filter(|(_, e)| {
            ball.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| c.authority().is_declared())
        })
        .count();

    println!(
        "[M2] ball at rest: {} edges | scaffold descriptions = {scaffolds} | declared = {declared}",
        ball.edges().count()
    );
    println!(
        "[M2] tier3 at rest: {:?}",
        topo::validate_geometric(&ball, Tol::witness()).is_ok()
    );

    // Both bands share ONE sphere key, so the chart moves as a group.
    let Some(sphere_key) = ball.faces().find_map(|(_, f)| {
        matches!(ball.get_surface(f.surface), Some(Surface::Sphere { .. })).then_some(f.surface)
    }) else {
        println!("[M2] no sphere chart; nothing to measure");
        return;
    };
    let group: Vec<FaceKey> = ball
        .faces()
        .filter(|(_, f)| f.surface == sphere_key)
        .map(|(k, _)| k)
        .collect();
    println!("[M2] sphere chart worn by {} face(s)", group.len());

    let mut body = ball.clone();
    match topo::replace_faces_offset(&mut body, &group, 0.05, 1e-9, band(), Tol::witness()) {
        Err(topo::ReplaceFaceError::CarrierLaneUnsupported { what, .. }) => {
            println!("[M2] REFUSED CarrierLaneUnsupported: {what}");
            println!(
                "[M2] declared-arm? {}",
                if what.contains("declared") {
                    "YES"
                } else {
                    "no"
                }
            );
        }
        Err(e) => println!("[M2] refused through another door: {e:?}"),
        Ok(()) => println!("[M2] the offset SUCCEEDED — the declared arm did not fire"),
    }
}
