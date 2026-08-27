//! **`shell` at the body**: sealed and opened.
//!
//! The sealed arm is checked against closed forms (a box and a solid of
//! revolution both have exact volumes and areas), against the shell
//! roles (`Outer` + `Void`, decided rather than declared), and — the
//! structural claim that matters — against the verdict log: the general
//! crossing pipeline must not run, which is a fact about what the verb
//! DID, not a sentence about what it meant to do.
//!
//! The opened arm is checked the same way, plus the shape of the rim:
//! one shell, one annular face, its ring on the designated face's own
//! surface.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, ShellError, ShellRole};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The fit tolerance the door's NURBS lane would use; unread on every
/// fixture here, all of which are analytic.
const FIT_TOL: f64 = 1e-6;

/// A `w x d x h` box at the origin.
fn boxy(w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(w, 0.0), 0.0),
        ProfileVertex::new(p2(w, d), 0.0),
        ProfileVertex::new(p2(0.0, d), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a rectangle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a rectangle extrudes")
        .body
}

/// **The vessel**: a rectangular meridian revolved a full turn about
/// the `y` axis — a solid cylinder of radius `r` and height `h`,
/// bounded by one cylinder wall and two planar caps. The perf fixture.
fn vessel(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r, 0.0), 0.0),
        ProfileVertex::new(p2(r, h), 0.0),
        ProfileVertex::new(p2(0.0, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the meridian is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// A tube: the annular meridian revolved a full turn — the curved
/// two-shell shape the STEP gate is recorded on.
fn tube(ri: f64, ro: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(ri, 0.0), 0.0),
        ProfileVertex::new(p2(ro, 0.0), 0.0),
        ProfileVertex::new(p2(ro, h), 0.0),
        ProfileVertex::new(p2(ri, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the annular meridian is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the annular meridian revolves")
    .body
}

/// The planar face whose origin sits at height `y` (the caps).
fn plane_face_at(body: &Body<f64>, y: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - y).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no z = {y} cap"))
}

// ---------------------------------------------------------------------
// The sealed arm
// ---------------------------------------------------------------------

/// **The sealed shelled box.** Two shells in one solid, roles DECIDED
/// (not declared), and the volume is the closed form of the wall.
#[test]
fn a_sealed_shelled_box_is_an_outer_and_a_void() {
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let hollow = topo::shell(&boxy(w, d, h), t, FIT_TOL, band(), Tol::witness())
        .expect("a box thicker than twice the wall shells");

    assert_eq!(
        topo::validate_geometric(&hollow, Tol::witness()),
        Ok(()),
        "tier 3 on the sealed thin box"
    );
    assert_eq!(hollow.shells().count(), 2, "outer boundary plus cavity");
    assert_eq!(hollow.solids().count(), 1, "one solid, two shells");

    let roles = topo::classify_shells(&hollow, Tol::witness()).expect("the shells classify");
    let mut kinds: Vec<ShellRole> = roles.iter().map(|c| c.role).collect();
    kinds.sort_by_key(|r| format!("{r:?}"));
    assert_eq!(
        kinds,
        vec![ShellRole::Outer, ShellRole::Void],
        "one outer boundary, one cavity"
    );

    let props = topo::mass_properties(&hollow, Tol::witness()).expect("a planar body's props");
    let want = w * d * h - (w - 2.0 * t) * (d - 2.0 * t) * (h - 2.0 * t);
    assert!(
        (props.volume - want).abs() <= 1e-12,
        "the wall's volume is outer minus inner: got {}, want {want}",
        props.volume
    );
}

/// **The structural claim: no crossing machinery runs.** The sealed
/// shell is the boolean's degenerate no-crossing arm, and this row is
/// the FACT rather than the intention — the verdict log records every
/// decided predicate, and none of the crossing pipeline's may appear.
///
/// The fixture is deliberately boolean-ADMISSIBLE (planes throughout),
/// so a reroute through `subtract` would genuinely run the pipeline and
/// this row would see it.
///
/// **`bool_ring_run_winding` is allowed, and naming it is the point.**
/// The verb ends in a tier-3 validation, and the validator's own
/// planar-boundary check decides that predicate against the same margin
/// the boolean's ring-run test uses — one name, two owners
/// (`topo::validate` and `topo::boolean::join`). Allowing it by name
/// keeps the claim exact: nothing SSI, census or classification ran.
/// A silent `bool_`-prefix filter would have hidden that the shared
/// name exists at all.
const VALIDATOR_SHARED: &str = "bool_ring_run_winding";

#[test]
fn shell_runs_no_intersection_machinery() {
    let body = boxy(2.0, 3.0, 4.0);
    start_verdict_log();
    let hollow = topo::shell(&body, 0.25, FIT_TOL, band(), Tol::witness()).expect("it shells");
    let verdicts = take_verdict_log();
    assert!(!verdicts.is_empty(), "the verb decided something");
    let crossing: Vec<&'static str> = verdicts
        .iter()
        .map(|v| v.predicate)
        .filter(|p| p.starts_with("bool_") && *p != VALIDATOR_SHARED)
        .collect();
    assert!(
        crossing.is_empty(),
        "the sealed shell must not run the crossing pipeline; it decided {crossing:?}"
    );
    assert_eq!(hollow.shells().count(), 2);
}

/// **The sealed shelled vessel** — a solid of revolution, so the wall
/// is a cylinder pair and the caps are annuli. Closed forms both.
#[test]
fn a_sealed_shelled_vessel_matches_its_closed_form() {
    let (r, h, t) = (1.0, 2.0, 0.2);
    let hollow =
        topo::shell(&vessel(r, h), t, FIT_TOL, band(), Tol::witness()).expect("the vessel shells");
    assert_eq!(
        topo::validate_geometric(&hollow, Tol::witness()),
        Ok(()),
        "tier 3 on the sealed vessel"
    );
    assert_eq!(hollow.shells().count(), 2);

    let props = topo::mass_properties(&hollow, Tol::witness()).expect("the vessel's props");
    let want = core::f64::consts::PI * (r * r * h - (r - t) * (r - t) * (h - 2.0 * t));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "wall volume: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}

// ---------------------------------------------------------------------
// The opened arm
// ---------------------------------------------------------------------

/// **The opened shelled box: a cup.** One shell (the cavity fused into
/// the boundary through the rim), the designated face now annular, its
/// ring on its own surface, tier-3 valid, and the volume is the same
/// closed form as the sealed wall PLUS the lid the opening removed —
/// which is the whole content of "the wall thickness shows".
#[test]
fn an_opened_shelled_box_is_a_closed_thin_solid_with_a_rim() {
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = boxy(w, d, h);
    let top = plane_face_at(&body, h);
    let cup = topo::shell_open(&body, t, &[top], FIT_TOL, band(), Tol::witness())
        .expect("a box opens at its top");

    assert_eq!(
        topo::validate_geometric(&cup, Tol::witness()),
        Ok(()),
        "tier 3 on the cup"
    );
    assert_eq!(
        cup.shells().count(),
        1,
        "the rim fuses the cavity into the boundary — genus rises, nothing opens"
    );
    let rim = cup
        .get_face(top)
        .expect("the designated face survives as the rim");
    assert_eq!(rim.rings.len(), 1, "the rim carries exactly one ring");

    let props = topo::mass_properties(&cup, Tol::witness()).expect("a planar body's props");
    // The cup is the sealed wall plus the lid that is no longer there:
    // equivalently, the box minus a cavity that runs to the top.
    let want = w * d * h - (w - 2.0 * t) * (d - 2.0 * t) * (h - t);
    assert!(
        (props.volume - want).abs() <= 1e-12,
        "cup volume: got {}, want {want}",
        props.volume
    );
}

/// Opening TWO opposite faces gives a tube — one shell still, two rims,
/// and the closed form says so.
#[test]
fn opening_two_faces_gives_two_rims_and_one_shell() {
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = boxy(w, d, h);
    let (top, bottom) = (plane_face_at(&body, h), plane_face_at(&body, 0.0));
    let tubey = topo::shell_open(&body, t, &[top, bottom], FIT_TOL, band(), Tol::witness())
        .expect("a box opens at both caps");
    assert_eq!(
        topo::validate_geometric(&tubey, Tol::witness()),
        Ok(()),
        "tier 3 on the two-ended tube"
    );
    assert_eq!(tubey.shells().count(), 1);
    let props = topo::mass_properties(&tubey, Tol::witness()).expect("props");
    let want = w * d * h - (w - 2.0 * t) * (d - 2.0 * t) * h;
    assert!(
        (props.volume - want).abs() <= 1e-12,
        "tube volume: got {}, want {want}",
        props.volume
    );
}

// ---------------------------------------------------------------------
// The planted reds
// ---------------------------------------------------------------------

/// A non-positive wall is not a thin solid.
#[test]
fn a_nonpositive_thickness_refuses_typed() {
    for t in [0.0_f64, -0.1] {
        let e = topo::shell(&boxy(2.0, 3.0, 4.0), t, FIT_TOL, band(), Tol::witness())
            .expect_err("a non-positive wall must not build");
        assert!(
            matches!(e, ShellError::Thickness { .. }),
            "t = {t}: expected the thickness gate, got {e}"
        );
    }
}

/// **The reach.** A wall thicker than the vessel's own radius collapses
/// the cylinder onto its axis — the offset door's realized-radius floor
/// refuses, and that refusal IS the containment evidence's own decide.
#[test]
fn a_wall_past_the_reach_refuses_typed() {
    let e = topo::shell(&vessel(1.0, 2.0), 1.5, FIT_TOL, band(), Tol::witness())
        .expect_err("a wall past the radius collapses the wall");
    assert!(
        matches!(
            e,
            ShellError::Face {
                error: ref b,
                ..
            } if matches!(
                **b,
                topo::ReplaceFaceError::Offset {
                    error: geom_brep::OffsetError::RadiusFloor { .. },
                    ..
                }
            )
        ),
        "expected the radius floor carried up through the face gate, got {e}"
    );
}

/// The designation gates: stale, repeated, exhaustive, disconnecting,
/// and curved.
#[test]
fn the_open_face_designation_gates_refuse_typed() {
    let body = boxy(2.0, 3.0, 4.0);
    let top = plane_face_at(&body, 4.0);
    let bottom = plane_face_at(&body, 0.0);
    let t = 0.25;

    let e = topo::shell_open(&body, t, &[top, top], FIT_TOL, band(), Tol::witness())
        .expect_err("a face designated twice");
    assert!(
        matches!(e, ShellError::OpenFaceRepeated { face } if face == top),
        "got {e}"
    );

    let all: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let e = topo::shell_open(&body, t, &all, FIT_TOL, band(), Tol::witness())
        .expect_err("every face designated");
    assert!(
        matches!(e, ShellError::OpenFacesExhaustShell { .. }),
        "got {e}"
    );

    // The four walls, leaving the two caps as the remainder: they touch
    // nothing, so the boundary falls into two components.
    let walls: Vec<FaceKey> = all
        .iter()
        .copied()
        .filter(|f| *f != top && *f != bottom)
        .collect();
    let e = topo::shell_open(&body, t, &walls, FIT_TOL, band(), Tol::witness())
        .expect_err("the remainder is disconnected");
    assert!(
        matches!(e, ShellError::OpenFacesDisconnect { components: 2, .. }),
        "got {e}"
    );

    // A curved designation: its rim would be a curved face with a ring.
    let v = vessel(1.0, 2.0);
    let wall = v
        .faces()
        .find(|(_, f)| {
            matches!(
                v.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    let e = topo::shell_open(&v, 0.2, &[wall], FIT_TOL, band(), Tol::witness())
        .expect_err("a curved rim has no closed-form reading");
    assert!(
        matches!(
            e,
            ShellError::OpenFaceRingUnsupported {
                kind: geom_brep::SurfaceKind::Cylinder,
                ..
            }
        ),
        "got {e}"
    );
}

// ---------------------------------------------------------------------
// The STEP gate, recorded
// ---------------------------------------------------------------------

/// **The standing gate, on a shelled body.** A curved two-shell solid
/// refuses STEP export: the outward/void classifier is a planarity
/// identity with no curved counterpart. Recorded here so the day it is
/// retired this row says so.
#[test]
fn a_curved_two_shell_shell_refuses_step_export() {
    let hollow = topo::shell(&tube(0.6, 1.0, 2.0), 0.1, FIT_TOL, band(), Tol::witness())
        .expect("the tube shells");
    assert_eq!(hollow.shells().count(), 2);
    let e = step_export::step_string(
        &hollow,
        &step_export::StepOptions {
            product_name: "shelled tube".into(),
            ..Default::default()
        },
        Tol::witness(),
    )
    .expect_err("a curved two-shell body has no classifier");
    assert!(
        matches!(
            e,
            step_export::StepExportError::CurvedShellClassification { .. }
        ),
        "expected the standing curved-two-shell gate, got {e}"
    );
}

// ---------------------------------------------------------------------
// The measurement (#1019)
// ---------------------------------------------------------------------

/// **The shell verb's own cost, measured rather than asserted** —
/// `#1019`'s named fixture, and the place the `O(n²)` whole-body pcurve
/// mint banked at OFF-D PR-1 either shows up or does not.
///
/// `#[ignore]` because a timing row is a measurement, not a gate: it
/// asserts nothing about wall-clock (which is not a property of the
/// kernel), and a CI runner's numbers are not the numbers to record.
/// Run it explicitly, both profiles, and put what it prints in the
/// issue:
///
/// ```text
/// cargo test -p sweep --test all -- verbs_shell::the_shell_cost --ignored --nocapture
/// cargo test --release -p sweep --test all -- verbs_shell::the_shell_cost --ignored --nocapture
/// ```
#[test]
#[ignore = "a measurement, not a gate — see the doc comment"]
fn the_shell_cost_is_measured_not_asserted() {
    use std::time::Instant;
    let cases: Vec<(&str, Body<f64>, f64)> = vec![
        ("box (6 faces, 6 charts)", boxy(2.0, 3.0, 4.0), 0.25),
        ("vessel (4 faces, 3 charts)", vessel(1.0, 2.0), 0.2),
        ("tube (8 faces, 6 charts)", tube(0.6, 1.0, 2.0), 0.1),
    ];
    for (name, body, t) in cases {
        let faces = body.faces().count();
        let start = Instant::now();
        let hollow =
            topo::shell(&body, t, FIT_TOL, band(), Tol::witness()).expect("the fixture shells");
        let build = start.elapsed();
        let start = Instant::now();
        topo::validate_geometric(&hollow, Tol::witness()).expect("valid");
        let validate = start.elapsed();
        println!(
            "[shell cost] {name}: {faces} operand faces -> {} result faces; \
             build {build:?}, one tier-3 validation {validate:?}",
            hollow.faces().count(),
        );
    }
}
