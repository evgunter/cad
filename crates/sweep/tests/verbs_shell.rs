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
use topo::{Body, FaceKey, LoopBoundary, ShellError, ShellRole};

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
/// the boolean's ring-run test uses. The name has **three** owners, not
/// two — `topo::validate`, `topo::boolean::join`, and
/// `topo::merge_faces`' role normalization — which is exactly why a
/// silent prefix filter would be the wrong shape here. Allowing it by
/// name keeps the claim exact.
///
/// **What this pin does and does not cover.** It reads the log for
/// `bool_`-prefixed predicates, which is the crossing pipeline's own
/// vocabulary; it does NOT cover the `ssi_*` or `tangent_locus_*`
/// families, which carry their own prefixes. The claim is "the
/// boolean's machinery did not run", and the marching stack is reached
/// only through that machinery, so the coverage of SSI is by
/// composition rather than by the filter — stated rather than implied.
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
    // The vessel is TALL so the caps clear each other: at `h = 6` the
    // two cap planes are 6 m apart and a 1.2 m wall needs 2.4, so the
    // clearance gate (which runs first, and rightly) passes and the
    // refusal under test is the one the row is about.
    let e = topo::shell(&vessel(1.0, 6.0), 1.2, FIT_TOL, band(), Tol::witness())
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

/// **A chart with two orientations has no "inward".** `shell` reads a
/// chart's inward direction from its faces' orientation bit, and a
/// chart is moved as ONE by the group door — so a surface worn by
/// faces of OPPOSITE sense has no single direction to move it by.
///
/// Nothing structural forbids that body: `step-import`'s adoption
/// shares surface keys outright, so the sharing is not always a
/// revolve's two co-oriented wall bands. This row builds the
/// configuration the honest way available from outside the kernel —
/// sharing the outer wall's chart onto the inner wall, whose sense is
/// the opposite — and pins that the verb decides it rather than reading
/// the first face's bit and hoping.
#[test]
fn a_mixed_sense_chart_refuses_typed() {
    let mut body = tube(0.6, 1.0, 2.0);
    let cyl = |b: &Body<f64>, r: f64| -> FaceKey {
        b.faces()
            .find(|(_, f)| {
                matches!(b.get_surface(f.surface), Some(geom::Surface::Cylinder { radius, .. })
                    if (*radius - r).abs() < 1e-9)
            })
            .map(|(k, _)| k)
            .unwrap_or_else(|| panic!("no r = {r} wall"))
    };
    let (outer, inner) = (cyl(&body, 1.0), cyl(&body, 0.6));
    assert_ne!(
        body.get_face(outer).unwrap().sense,
        body.get_face(inner).unwrap().sense,
        "the tube's two walls face opposite ways, which is the point"
    );
    let shared = body.get_face(outer).unwrap().surface;
    body.set_face_surface(inner, topo::FaceSurface::Shared(shared))
        .expect("the attach-layer door shares a live key");

    let e = topo::shell(&body, 0.1, FIT_TOL, band(), Tol::witness())
        .expect_err("a mixed-sense chart has no single inward");
    assert!(
        matches!(e, ShellError::ChartSenseMixed { .. }),
        "expected the chart-sense gate, got {e}"
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
        ("box", boxy(2.0, 3.0, 4.0), 0.25),
        ("vessel", vessel(1.0, 2.0), 0.2),
        ("tube", tube(0.6, 1.0, 2.0), 0.1),
    ];
    for (name, body, t) in cases {
        // Counts are PRINTED, never spelled into the label: a hand
        // label drifts from the fixture (this row's first version said
        // "vessel (4 faces)" while printing 6) and a drifted label is
        // the kind of number that gets copied into an issue.
        let faces = body.faces().count();
        let charts = {
            let mut k: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
            k.sort_by_key(|s| format!("{s:?}"));
            k.dedup();
            k.len()
        };
        let start = Instant::now();
        let hollow =
            topo::shell(&body, t, FIT_TOL, band(), Tol::witness()).expect("the fixture shells");
        let build = start.elapsed();
        let start = Instant::now();
        topo::validate_geometric(&hollow, Tol::witness()).expect("valid");
        let validate = start.elapsed();
        println!(
            "[shell cost] {name}: {faces} operand faces / {charts} charts -> {} result faces; \
             build {build:?}, one tier-3 validation {validate:?}",
            hollow.faces().count(),
        );
    }
}

// ---------------------------------------------------------------------
// The Klein bottle's walls, re-authored
// ---------------------------------------------------------------------

// The Klein bottle's own numbers (`demos/tour/src/klein.rs`): the
// tube's spine radius, the wall thickness spelled into both offsets of
// every run by hand, the loop arc's spine radius, and one arc's sweep.
const KLEIN_R: f64 = 0.25;
const KLEIN_WALL: f64 = 0.05;
const KLEIN_RLOOP: f64 = 1.20;
const KLEIN_SWEEP_IN: f64 = 0.5 * core::f64::consts::PI;

/// A circle profile loop of radius `r` — two semicircular arcs, the
/// spelling `profile::circle` produces.
fn circle_loop(r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(-r, 0.0), 1.0),
        ProfileVertex::new(p2(r, 0.0), 1.0),
    ])
}

/// Klein's elbow, revolved about the loop-arc axis exactly as the demo
/// does — built from whichever cross-section loops it is handed.
fn klein_elbow(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("the elbow's cross-section validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(KLEIN_RLOOP, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        Revolution::Partial(-KLEIN_SWEEP_IN),
        Tol::witness(),
    )
    .expect("the elbow revolves")
    .body
}

/// **The `r ± t/2` wall pair — and the wall that stops it retiring.**
///
/// Klein's `elbow` spells the thickness twice: `circle(R + WALL/2)` for
/// the outer wall and `circle(R − WALL/2)` for the inner, revolved as
/// an annulus. The natural spelling is one circle and a `shell_open`
/// call — revolve the DISC, hollow it, and rim the two end caps where
/// the elbow meets the rest of the bottle. That is the "paid once per
/// wall" debt the demo's own findings list records.
///
/// **It does not retire in this unit, and this row is why.** A partial
/// revolve of a disc gives a TORUS wall and two PLANAR meridian end
/// caps, so every rim is `plane × torus` — and the C5 table has no arm
/// for that pair. The face-replacement door refuses
/// `NeighborPairUnroutable` naming it, and the shell verb carries that
/// refusal up unchanged. **Widening the route to chase a green would be
/// the wrong unit's work**; the refusal is the honest boundary, so this
/// row pins it by name.
///
/// **What would retire it**, concretely: a `plane × torus` section
/// arm. The configuration these rims need is the easy one — a plane
/// CONTAINING the torus axis cuts it in two circles, closed form, no
/// marching — and it is the same shape as the `plane_cylinder_section`
/// / `plane_cone_section` doors that already exist. Every other klein
/// wall pair is revolved too, so the same arm (plus `cone × cylinder`
/// for the flare) is what the whole debt waits on.
///
/// The comparison this row would make once that lands: topology exactly
/// equal, stored radii within one ulp (the two spellings reach the
/// inner radius by different routes — `R − WALL/2` against
/// `(R + WALL/2) − WALL`), volume within `1e-12`. Under the demo rule
/// the contract is **naturalness, not byte-identity**: one radius
/// instead of two, and the wall stops being a number the author has to
/// keep consistent across two call sites.
#[test]
fn the_klein_wall_pair_waits_on_a_plane_torus_route() {
    // The hand construction still builds, unchanged — the debt is real
    // and the demo is not broken, it is just paid by hand.
    let by_hand = klein_elbow(vec![
        circle_loop(KLEIN_R + KLEIN_WALL / 2.0),
        circle_loop(KLEIN_R - KLEIN_WALL / 2.0),
    ]);
    assert_eq!(
        topo::validate_geometric(&by_hand, Tol::witness()),
        Ok(()),
        "the hand-built elbow is what the demo ships"
    );

    let solid = klein_elbow(vec![circle_loop(KLEIN_R + KLEIN_WALL / 2.0)]);
    let caps: Vec<FaceKey> = solid
        .faces()
        .filter(|(_, f)| {
            matches!(
                solid.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(caps.len(), 2, "a partial revolve has two meridian end caps");

    let e = topo::shell_open(&solid, KLEIN_WALL, &caps, FIT_TOL, band(), Tol::witness())
        .expect_err("plane x torus has no route arm");
    assert!(
        matches!(
            e,
            ShellError::Face { ref error, .. } if matches!(
                **error,
                topo::ReplaceFaceError::NeighborPairUnroutable {
                    kind: geom_brep::SurfaceKind::Plane,
                    other_kind: geom_brep::SurfaceKind::Torus,
                    ..
                }
            )
        ),
        "expected the C5 refusal naming (plane, torus), got {e}"
    );

    // The sealed arm stops at the same wall, on the same edges — the
    // blocker is the rim pair, not the opening.
    let sealed = topo::shell(&solid, KLEIN_WALL, FIT_TOL, band(), Tol::witness())
        .expect_err("the sealed arm meets the same pair");
    assert!(matches!(sealed, ShellError::Face { .. }), "got {sealed}");
}

// ---------------------------------------------------------------------
// The rim on a solid of revolution (#1082)
// ---------------------------------------------------------------------

/// Every planar face whose plane sits at station `y` — the CHART, which
/// on a full revolve is what a cap is worn by.
fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
fn rings_of(body: &Body<f64>) -> usize {
    body.faces().map(|(_, f)| f.rings.len()).sum()
}

/// The Euler–Poincaré genus, parity-checked before halving.
fn genus_of(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    let chi = v - e + f - rings_of(body) as i64;
    assert!(chi % 2 == 0, "v - e + f - r = {chi} is ODD");
    body.shells().count() as i64 - chi / 2
}

/// **The AXIS-TOUCHING cap: one rim annulus, and it meshes.**
///
/// A full revolve of an axis-touching meridian wears its cap on two
/// half-disc faces that meet at the axis apex, and the cavity
/// counterpart's boundary is the same shape one wall in. Gluing that
/// counterpart on as a ring would put it ON the designated boundary —
/// the #1082 class. The rim is instead built on the chart as ONE
/// region, so the mouth comes back as a single annulus carrying a
/// single ring, the cup is genus 0 exactly as `topo::shell`'s docs say,
/// and the CDT accepts it.
#[test]
fn a_revolved_cap_opens_to_one_annular_rim() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.4, 0.05);
    let body = vessel(r, h);
    let chart = plane_chart_at_y(&body, h);
    assert_eq!(chart.len(), 2, "a full revolve's cap is two half-discs");
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol).expect("the drum opens");

    assert_eq!(topo::validate_geometric(&cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1, "the rim fuses the cavity in");
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (1, 0),
        "one rim annulus with one ring, and a cup is genus 0"
    );
    let mouth: Vec<FaceKey> = plane_chart_at_y(&cup, h);
    assert_eq!(mouth.len(), 1, "the two half-discs became ONE rim face");

    for delta in [1e-2, 1e-3, 2e-4] {
        mesh::tessellate(&cup, delta, tol)
            .unwrap_or_else(|e| panic!("the rim must triangulate at delta = {delta}, got {e:?}"));
    }
    let props = topo::mass_properties(&cup, tol).expect("props");
    let want = core::f64::consts::PI * (r * r * h - (r - t) * (r - t) * (h - t));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "drum cup volume: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}

/// **The ANNULAR cap: TWO disjoint rim annuli**, which is a face SPLIT
/// and not a ring placement — the second shape of the #1082 class. A
/// full revolve of a CLOSED off-axis meridian closes its own seam, so
/// the mouth is one slit annular face; its cavity counterpart is a
/// smaller annulus sitting strictly inside it, and the material between
/// them is two disjoint rings of wall. Both are built: the hole is
/// promoted to its own rim face before the glue and collects the
/// designated face's own hole after it.
#[test]
fn an_annular_cap_opens_to_two_disjoint_rims() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.30, 0.50, 0.40, 0.05);
    let body = tube(ri, ro, h);
    let chart = plane_chart_at_y(&body, h);
    assert_eq!(
        chart.len(),
        1,
        "a closed off-axis meridian closes its own seam, so this cap is ONE face"
    );
    let cup = topo::shell_open(&body, t, &chart, FIT_TOL, band(), tol).expect("the tube opens");

    assert_eq!(topo::validate_geometric(&cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1);
    assert_eq!(
        (rings_of(&cup), genus_of(&cup)),
        (2, 1),
        "two rim annuli, one ring each; the bore keeps the cup genus 1"
    );
    // The two rims, named by the radii they run between.
    let mut radii: Vec<(f64, f64)> = plane_chart_at_y(&cup, h)
        .into_iter()
        .map(|k| {
            let f = cup.get_face(k).expect("rim face");
            assert_eq!(f.rings.len(), 1, "each rim is an annulus");
            let radius = |lk| {
                let LoopBoundary::Cycle { first } = cup.get_loop(lk).expect("loop").boundary else {
                    panic!("an empty rim loop")
                };
                let he = cup.loop_cycle(first).expect("cycle")[0];
                let e = cup
                    .get_edge(cup.get_half_edge(he).expect("he").edge)
                    .expect("edge");
                match cup
                    .get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .expect("carrier")
                    .carrier()
                {
                    geom::Curve3::Circle { radius, .. } => *radius,
                    other => panic!("a rim bounded by {other:?}"),
                }
            };
            (radius(f.rings[0]), radius(f.outer))
        })
        .collect();
    radii.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert_eq!(radii.len(), 2, "two rim faces on the mouth plane");
    for (got, want) in radii.iter().zip([(ri, ri + t), (ro - t, ro)]) {
        assert!(
            (got.0 - want.0).abs() < 1e-12 && (got.1 - want.1).abs() < 1e-12,
            "rim between {got:?}, want {want:?}"
        );
    }

    for delta in [1e-2, 1e-3, 2e-4] {
        mesh::tessellate(&cup, delta, tol)
            .unwrap_or_else(|e| panic!("the rims must triangulate at delta = {delta}, got {e:?}"));
    }
    let props = topo::mass_properties(&cup, tol).expect("props");
    let want = core::f64::consts::PI
        * ((ro * ro - ri * ri) * h - ((ro - t).powi(2) - (ri + t).powi(2)) * (h - t));
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "tube cup volume: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}

/// **The validator's net, shown firing** — tier 3's check 9, on the
/// exact anatomy the rim construction above removes.
///
/// The wrong body #1082 named cannot be built through `shell_open` any
/// more, so it is built here through the PUBLIC doors the verb used to
/// compose: shell it sealed, lift the cavity's counterpart chart onto
/// the designated one (`replace_faces_offset`, the same distance the
/// verb derives from the two planes), then `kfmrh` each pair straight
/// on — skipping the step that makes the chart one region. That is the
/// old construction exactly, and what it mints is a ring standing on
/// its own face's outer loop.
///
/// Both contact shapes are covered, one per fixture, because the two
/// arms of the check are independent: the axis-touching cap shares a
/// VERTEX position (the apex both loops own), while the annular cap
/// shares none and is caught only by an EDGE running along an edge
/// (the radial seam, whose counterpart sits inside it).
#[test]
fn a_ring_standing_on_its_outer_loop_refuses_at_tier_3() {
    let tol = Tol::witness();
    let t = 0.05;
    for (what, body, y, want_vertex) in [
        ("an axis-touching cap", vessel(0.5, 0.4), 0.4, true),
        ("an annular cap", tube(0.30, 0.50, 0.40), 0.40, false),
    ] {
        let mut sealed = topo::shell(&body, t, FIT_TOL, band(), tol).expect("the sealed shell");
        let mouth = plane_chart_at_y(&sealed, y);
        let counterpart = plane_chart_at_y(&sealed, y - t);
        assert_eq!(
            mouth.len(),
            counterpart.len(),
            "{what}: the counterpart chart mirrors the designated one"
        );
        // The verb's own lift distance: read off the two planes rather
        // than negated from the way in.
        let plane_of =
            |b: &Body<f64>, f: FaceKey| match b.get_surface(b.get_face(f).expect("face").surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => (*origin, *normal),
                other => panic!("{what}: a non-planar cap: {other:?}"),
            };
        let (o_from, n_from) = plane_of(&sealed, counterpart[0]);
        let (o_onto, _) = plane_of(&sealed, mouth[0]);
        let back = (o_onto - o_from).dot(n_from);
        topo::replace_faces_offset(&mut sealed, &counterpart, back, FIT_TOL, band(), tol)
            .expect("the counterpart chart lifts onto the mouth plane");
        for (&rim, &source) in mouth.iter().zip(&counterpart) {
            sealed.kfmrh(rim, source).expect("the raw glue");
        }
        let errors = topo::validate_geometric(&sealed, tol)
            .expect_err("a ring standing on its outer loop must refuse");
        let contacts: Vec<&topo::ValidationError> = errors
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::RingMeetsOuter { .. }))
            .collect();
        assert!(
            !contacts.is_empty(),
            "{what}: tier 3 must name the ring-vs-outer contact; got {errors:?}"
        );
        let vertex_arm = contacts.iter().any(|e| {
            matches!(
                e,
                topo::ValidationError::RingMeetsOuter {
                    contact: topo::RingContact::Vertex { .. },
                    ..
                }
            )
        });
        let edge_arm = contacts.iter().any(|e| {
            matches!(
                e,
                topo::ValidationError::RingMeetsOuter {
                    contact: topo::RingContact::Edge { .. },
                    ..
                }
            )
        });
        if want_vertex {
            assert!(
                vertex_arm,
                "{what}: the apex both loops own is a VERTEX contact; got {contacts:?}"
            );
        } else {
            assert!(
                edge_arm && !vertex_arm,
                "{what}: no shared vertex here — the seam is an EDGE contact; got {contacts:?}"
            );
        }
    }
}
