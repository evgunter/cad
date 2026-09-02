//! **The opening measurement** for the coaxial cylinder×sphere lane,
//! and the differential rows that say what this unit's arms did and did
//! not move.
//!
//! The fixture is the one the pair is named for: a sphere threaded on a
//! cylinder — centre on the cylinder's axis, `R > r` so the two walls
//! genuinely cross, in two circles at `z = ±√((R−r)(R+r))` well inside
//! the cylinder's own extent. Every body here is authored through the
//! public extrude/revolve doors.
//!
//! **What the measurement found, and it was not presumed — including
//! the parts that refuted the first guess.** Three doors were named as
//! candidates. Two of them are reachable for this pair, one is not, and
//! WHICH one a pose takes turns on whether the walls cross:
//!
//! - **The crossing coaxial pose refuses at `CurvedPierceUnsupported`**
//!   — the CROSSING layer, `boolean::reduce`'s curved-face arm. The
//!   edge it names is operand A's SEAM LINE, and the face is the ball's
//!   SPHERE face. The pierce door exists for a LINE carrier crossing a
//!   CYLINDER wall, so it is the FACE kind that refuses this one, not
//!   the carrier — which is the SPHSPH reading exactly, on the other
//!   operand order.
//! - **A non-crossing coaxial pose — a ball wholly inside a wider
//!   cylinder — refuses at `FallbackExtentUnsupported`** instead, the
//!   containment fallback's curved-extent scan. It is reachable
//!   precisely because no crossing is found first.
//! - **The germ frame is reached by neither.** Both doors sit above it,
//!   so no cylinder×sphere germ pair is ever offered to
//!   `pair_section_frame` in this build.
//!
//! This unit's section and frame arms move none of that, and every row
//! below pins the refusal as a MEASUREMENT rather than as a target.
//! What would move the first is a pierce lane for a curved FACE; what
//! would move the second is a cyl×sphere seam lane. Neither is this
//! unit's work.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError};

/// The cylinder: a circle of radius `r` at the origin, extruded along
/// world Z from `z0` to `z1`. Its axis is Z.
fn cyl(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(z1 - z0), tol)
        .unwrap()
        .body
}

/// A radius-`r` ball at `centre`, poles on world Y (the pip corpus's
/// constructor chart — the same one SPHSPH measured on).
fn ball_at(r: f64, centre: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -r), 1.0),
        ProfileVertex::new(Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    topo::transform_rigid(&ball, &Affine3::translation(centre), Tol::witness()).unwrap()
}

/// The re-posed twin's map, off every axis plane.
fn twin_map() -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(0.3, -0.2, 0.7),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    ) * Affine3::translation(Vec3::new(0.11, 0.23, -0.37))
}

fn posed(b: &Body<f64>) -> Body<f64> {
    topo::transform_rigid(b, &twin_map(), Tol::witness()).unwrap()
}

/// The coaxial fixture, direct and re-posed: `r = 1`, `R = 1.5`, the
/// sphere centred on the axis at the cylinder's mid-height.
fn fixture() -> [(&'static str, Body<f64>, Body<f64>); 2] {
    let c = cyl(1.0, -2.0, 2.0);
    let s = ball_at(1.5, Vec3::new(0.0, 0.0, 0.0));
    [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ]
}

/// **THE OPENING MEASUREMENT.** The crossing coaxial union dies at the
/// CROSSING layer, in both poses, with the same typed variant — not at
/// the germ frame, which sits below this door and is never reached.
///
/// The carrier kind is pinned too, because it is what names whose work
/// the row waits on, and it refuted the guess this row was first
/// written with: the edge is the cylinder's SEAM LINE, not a rim
/// circle. The pierce door exists for a LINE carrier against a CYLINDER
/// WALL, so what refuses here is the FACE — a sphere. That is one
/// reason, not two, and it is not a germ-pair join.
#[test]
fn the_coaxial_union_refuses_at_the_curved_pierce_door() {
    for (label, c, s) in fixture() {
        let err = topo::union(&c, &s, Tol::witness())
            .expect_err("a coaxial cyl×sphere crossing has no crossing lane");
        let BooleanError::CurvedPierceUnsupported { operand, edge, .. } = err else {
            panic!("{label}: expected the curved pierce door, got {err:?}");
        };
        // Operand A is the cylinder, and the edge it names is a rim.
        assert_eq!(format!("{operand:?}"), "A", "{label}");
        let Some(topo::CurveGeom::Certified(carrier)) =
            c.get_edge(edge).and_then(|e| c.get_curve_geom(e.curve))
        else {
            panic!("{label}: the named edge has no certified curve");
        };
        assert!(
            matches!(carrier.carrier(), topo::Curve3::Line { .. }),
            "{label}: the refusal names a non-line carrier"
        );
        // The face is the ball's, and the message names the clause that
        // actually applies to this pose, so a reader lands on the right
        // fact rather than on the carrier clause beside it.
        let msg = format!("{err}");
        assert!(msg.contains("sphere face"), "{label}: {msg}");
        assert!(
            msg.contains("LINE carrier definitely crossing a CYLINDER wall"),
            "{label}: {msg}"
        );
    }
}

/// **The declared arm changes nothing about the union, and this row is
/// the receipt.** `cylinder_sphere_section` and the germ-frame arm both
/// landed in this unit; the union's door did not move, because the door
/// is two layers above them. A row that greened only on the direct pose
/// would be hiding a pose-dependent answer, so the twin is asserted to
/// the same variant.
///
/// What WOULD move it: a pierce lane that takes a Circle carrier
/// against a curved face. Not this unit.
#[test]
fn the_section_and_frame_arms_do_not_move_the_unions_door() {
    let doors: Vec<String> = fixture()
        .into_iter()
        .map(|(_, c, s)| {
            let err = topo::union(&c, &s, Tol::witness()).expect_err("still refused");
            format!("{err:?}")
                .split(|ch: char| !ch.is_alphanumeric())
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    assert_eq!(doors[0], doors[1], "the two poses take different doors");
    assert_eq!(doors[0], "CurvedPierceUnsupported", "{doors:?}");
}

/// **The non-coaxial transversal pose still marches** — the SSI lane is
/// untouched by this unit, and the union's door for it is the same
/// pierce door. That is the honest statement that the exact arm did not
/// narrow anything it was not supposed to.
#[test]
fn a_transversal_pose_keeps_its_door_too() {
    let c = cyl(1.0, -2.0, 2.0);
    let s = ball_at(1.5, Vec3::new(0.6, 0.0, 0.0));
    for (label, c, s) in [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ] {
        let err = topo::union(&c, &s, Tol::witness()).expect_err("no crossing lane");
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "{label}: {err:?}"
        );
    }
}

/// **The second reachable door, and the row that refuted "a contained
/// ball just answers".** A ball wholly inside a wider cylinder has no
/// crossing at all, so the pipeline falls through to the containment
/// fallback — and the fallback's curved-extent scan refuses
/// `FallbackExtentUnsupported`, naming the cyl×sphere seam lane. It
/// cannot answer even here, because the ball's certified extent meets
/// the wall face's BOX and a box overlap is a MAY, not a DOES.
///
/// This is what makes the opening measurement a table rather than a
/// single door: the crossing pose takes the pierce, the non-crossing
/// pose takes the scan, and the germ frame takes neither. Only the
/// third would have been this unit's to move.
#[test]
fn a_contained_ball_refuses_at_the_curved_extent_scan() {
    let c = cyl(2.0, -2.0, 2.0);
    let s = ball_at(0.5, Vec3::new(0.0, 0.0, 0.0));
    for (label, c, s) in [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ] {
        let err = topo::union(&c, &s, Tol::witness())
            .expect_err("the contained pose cannot certify its nearness");
        let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
            panic!("{label}: expected the extent scan's refusal, got {err:?}");
        };
        assert!(
            what.contains("cyl×sphere seam lane is not wired"),
            "{label}: {what}"
        );
    }
}

/// **The deferred fitted-chord join window's door, named with its
/// payload** (the `verbs_shell` precedent). The window is deliberately
/// NOT built by this unit, and the sentence that says so is still TRUE:
/// the pair refusal names the missing azimuth-window analog, and
/// nothing in this unit gives it one. What would retire it is a
/// `run_azimuth_window` / `chart_pcurve` analog for a cylinder×sphere
/// fitted chord — a consumer the coaxial arms do not need and do not
/// provide.
///
/// The row reads the Display text off a constructed error rather than
/// off the source, so a rewrite that dropped the sentence reds here.
#[test]
fn the_deferred_join_windows_door_still_names_itself() {
    let torus = {
        // A torus operand reaches the pair/kind refusal, which is the
        // door that carries the fitted-chord sentence.
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(2.0, -0.3), 1.0),
            ProfileVertex::new(Point2::new(2.0, 0.3), 1.0),
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap();
        let axis = RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        revolve(&vp, axis, Revolution::Full, Tol::witness())
            .unwrap()
            .body
    };
    let err = topo::union(&cyl(1.0, -2.0, 2.0), &torus, Tol::witness())
        .expect_err("a torus operand has no wired arm");
    let msg = format!("{err}");
    assert!(
        msg.contains("cyl×sphere") && msg.contains("window"),
        "the deferred window's door stopped naming itself: {msg}"
    );
}
