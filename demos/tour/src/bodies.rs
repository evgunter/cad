//! The tour's bodies, each built through the public profile/sweep API.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tolerance, Vec2};
use profile::{LoopBuilder, Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

use crate::Stop;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .expect("profile validation")
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// A circle as a two-vertex closed arc carrier (bulge 1 = semicircle).
fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(cx + r, cy),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(cx - r, cy),
            bulge: 1.0,
        },
    ])
}

/// L-bracket: polyline + one fillet arc at the inner corner, extruded.
fn bracket() -> topo::Body<f64> {
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .line_to(p2(1.5, 1.0))
        .arc_to_via(p2(1.146, 1.146), p2(1.0, 1.5)) // r = 0.5 inner fillet
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    extrude(&validated(vec![lp]), Extrusion::Distance(0.75))
        .expect("extrude bracket")
        .body
}

/// Rectangular plate with two circular holes: a genus-2 extrusion.
fn plate() -> topo::Body<f64> {
    let outer = ProfileLoop::polygon([p2(-3.0, -1.5), p2(3.0, -1.5), p2(3.0, 1.5), p2(-3.0, 1.5)]);
    let holes = vec![circle(-1.5, 0.0, 0.7), circle(1.5, 0.0, 0.7)];
    let mut loops = vec![outer];
    loops.extend(holes);
    extrude(&validated(loops), Extrusion::Distance(0.6))
        .expect("extrude plate")
        .body
}

/// Solid vase: an axis-touching profile — conical base, spherical
/// belly (the arc's carrier center sits ON the revolve axis, so the
/// swept surface is a sphere zone; off-axis arc carriers would sweep
/// toroids, which M2's revolve refuses as a typed error), conical
/// flared lip — fully revolved about the y axis.
fn vase() -> topo::Body<f64> {
    // Belly arc: circle of radius 1.3 centered at (0, 0.8) — on the
    // axis — from (1.2, 0.3) through (1.3, 0.8) to (0.5, 2.0).
    let lp = LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(1.2, 0.0))
        .line_to(p2(1.2, 0.3))
        .arc_to_via(p2(1.3, 0.8), p2(0.5, 2.0))
        .line_to(p2(0.9, 2.5))
        .line_to(p2(0.0, 2.5))
        .close();
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .expect("revolve vase")
        .body
}

/// Torus (the donut): a full revolve of an off-axis circle — a closed
/// all-curved genus-1 body.
fn donut() -> topo::Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(2.0, -0.5),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(2.0, 0.5),
            bulge: 1.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .expect("revolve donut")
        .body
}

/// V-groove pulley: an off-axis polyline profile, fully revolved.
fn pulley() -> topo::Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(0.5, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 0.35),
        p2(1.45, 0.75),
        p2(2.0, 1.15),
        p2(2.0, 1.5),
        p2(0.5, 1.5),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .expect("revolve pulley")
        .body
}

/// Quarter wedge: a partial (90°) revolve — wedge caps, arc rims.
fn wedge() -> topo::Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    )
    .expect("revolve wedge")
    .body
}

/// The six stops, in tour order.
pub fn stops() -> Vec<Stop> {
    vec![
        Stop {
            name: "bracket",
            story: "L-bracket with a filleted inner corner (polyline + tangent arc profile)",
            ops: "LoopBuilder (line_to/arc_to_via) -> Profile::validate -> extrude(Distance)",
            delta: 1e-2,
            seamed: false,
            note: None,
            body: bracket(),
        },
        Stop {
            name: "plate",
            story: "plate with two circular holes — genus 2 (each hole: 2 rings, wall band)",
            ops: "polygon outer + two closed arc-carrier holes -> extrude(Distance)",
            delta: 1e-2,
            seamed: false,
            note: None,
            body: plate(),
        },
        Stop {
            name: "vase",
            story: "solid vase — axis-touching profile, spherical belly zone + conical lip",
            ops: "LoopBuilder line/arc_to_via -> revolve(axis y, Full); sphere/cone/plane faces",
            delta: 2e-2,
            seamed: false,
            note: None,
            body: vase(),
        },
        Stop {
            name: "donut",
            story: "torus — off-axis circle revolved; closed all-curved body, genus 1",
            ops: "two-vertex bulge-1 circle -> revolve(axis y, Full)",
            delta: 8e-2,
            seamed: false,
            note: None,
            body: donut(),
        },
        Stop {
            name: "pulley",
            story: "V-groove pulley — off-axis polyline revolved; cones, cylinders, annuli",
            ops: "7-gon profile -> revolve(axis y, Full), genus 1",
            delta: 2e-2,
            seamed: false,
            note: None,
            body: pulley(),
        },
        Stop {
            name: "wedge",
            story: "quarter wedge — partial 90 degree revolve with planar wedge caps",
            ops: "rectangle -> revolve(axis y, Partial(pi/2))",
            delta: 1e-2,
            seamed: false,
            note: None,
            body: wedge(),
        },
    ]
}

/// The tour's coda: the kernel is fail-loud — a self-intersecting
/// (bowtie) profile is refused with a typed error, not a crash or a
/// silently broken solid.
pub fn finale_fail_loud() {
    println!("\n== finale: fail-loud ==");
    println!("   a bowtie (self-intersecting) profile is refused before any sweep runs:");
    let bowtie = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 2.0), p2(2.0, 0.0), p2(0.0, 2.0)]);
    match Profile::new(SketchPlane::xy(), vec![bowtie]).validate(Tolerance::get()) {
        Ok(_) => panic!("bowtie profile unexpectedly validated"),
        Err(e) => println!("   typed rejection: {e:?}"),
    }
}
