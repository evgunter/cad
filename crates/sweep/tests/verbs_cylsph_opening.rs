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
/// **BOTH halves of the payload are read off the bodies, and each half
/// carries a stated non-vacuity guard.** The pierce door exists for a
/// LINE carrier definitely crossing a CYLINDER wall, so a refusal here
/// is either the carrier's fault or the face's, and the row has to name
/// which:
///
/// - the EDGE is operand A's, and its certified carrier is a **Line** —
///   the cylinder's SEAM, which refutes the guess this row was first
///   written with (a rim circle). Discriminating, and measured so: A
///   carries SIX edges, of which FOUR are the rim Circles and only two
///   are seam Lines, and the guard below pins that the circles are on
///   offer.
/// - the FACE is operand B's, and its surface kind is **Sphere**.
///
/// So the carrier is one the door HAS an arm for and the face is not:
/// what refuses is the FACE kind. That is one reason, not two, and it
/// is not a germ-pair join.
///
/// **What the Sphere half can and cannot catch, measured rather than
/// assumed.** B's face roster is UNIFORM — two faces, both the ball's
/// sphere wall — and body face keys are slotmap indices that COLLIDE
/// across bodies, so this assertion cannot catch a door that named a
/// different face; measured directly, substituting a face key drawn
/// from A leaves the row green. What it does catch is a door reaching
/// into a roster with no sphere in it, and A is exactly that (two
/// Planes and two Cylinders), which the second guard pins. That is
/// strictly more than the assertions it replaced: those read Display
/// SUBSTRINGS that are static literals present in EVERY
/// `CurvedPierceUnsupported` message whatever the payload said, so
/// they pinned nothing about this pose and are gone.
#[test]
fn the_coaxial_union_refuses_at_the_curved_pierce_door() {
    for (label, c, s) in fixture() {
        let err = topo::union(&c, &s, Tol::witness())
            .expect_err("a coaxial cyl×sphere crossing has no crossing lane");
        let BooleanError::CurvedPierceUnsupported {
            operand,
            edge,
            face,
            ..
        } = err
        else {
            panic!("{label}: expected the curved pierce door, got {err:?}");
        };
        // Operand A is the cylinder, and the edge it names is its SEAM.
        assert_eq!(format!("{operand:?}"), "A", "{label}");
        assert_eq!(
            topo::query::edge_carrier_kind(&c, edge),
            Some(topo::CurveKind::Line),
            "{label}: the refusal names a non-line carrier"
        );
        // Non-vacuity for that half: the rim circles ARE on offer, so
        // "Line" is a choice the door made and not the only kind there.
        assert_eq!(
            c.edges()
                .filter(|&(k, _)| {
                    topo::query::edge_carrier_kind(&c, k) == Some(topo::CurveKind::Circle)
                })
                .count(),
            4,
            "{label}: the Line pin only says something while rim circles are on offer"
        );
        // The face is the BALL's, and it is the sphere wall — the half
        // the door has no arm for.
        assert_eq!(
            topo::query::face_surface_kind(&s, face),
            Some(geom_brep::SurfaceKind::Sphere),
            "{label}: the refusal does not name a sphere face of operand B"
        );
        // Non-vacuity for THIS half, exactly as far as it goes (see the
        // doc): operand A has no sphere face at all, so a door that
        // reached into A's roster could not answer Sphere.
        assert!(
            c.faces().all(|(k, _)| {
                topo::query::face_surface_kind(&c, k) != Some(geom_brep::SurfaceKind::Sphere)
            }),
            "{label}: operand A has a sphere face, so the Sphere pin says nothing"
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
///
/// **This row pins `CurvedPairUnsupported`, NOT
/// `CurvedBooleanUnsupported`** — a torus operand is stopped at the
/// pair/kind gate and never reaches the germ-pair join dispatch. The
/// other variant's text is pinned by
/// [`the_join_dispatchs_refusal_says_what_it_actually_wires`] below,
/// which had to construct a different error to get at it.
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

/// **The germ-pair JOIN dispatch's refusal says what that dispatch
/// actually wires** — the corrected clause, read off a CONSTRUCTED
/// `CurvedBooleanUnsupported`, which no row in the tree did before.
///
/// The clause it replaces was created by this unit's own refusal-text
/// sweep and was measured FALSE: it said the join dispatch wires
/// `(Sphere, Sphere)` and a declared-coaxial `(Cylinder, Sphere)`. It
/// does not. `join::join_germ_pair`'s match has three arms —
/// `(Plane, Plane)`, `(Plane, Sphere) | (Plane, Cylinder)` and the
/// mirror of the second — and its catch-all is the site that raises
/// THIS variant, so a sphere pair or a cyl×sphere germ reaches the
/// catch-all exactly like a cone or torus one. What IS wider is
/// `join::pair_section_frame`, a different dispatch answering a
/// different question: it names a section frame (a centre and an axis
/// for the rotational facing test), never a seam lane. Both Displays
/// now say that, and neither contradicts the other.
///
/// **The operand here is a NURBS wall, deliberately.** The variant is
/// per-KIND and its Display carries no per-site branch, so any body
/// that raises it serves. A NURBS wall is a construction that DOES
/// reach it through the public `union` door — measured, by this row.
/// What is NOT available is the germ pose the corrected clause is
/// about: the cyl×sphere and sphere×sphere crossings are stopped two
/// layers above (the rows at the top of this file are that
/// measurement), so the join dispatch's catch-all cannot be reached
/// end to end for them. No claim is made that a NURBS wall is the ONLY
/// construction that reaches this variant — the error has several raise
/// sites (`sectors.rs`, `vtxfac.rs`, `recl.rs`, `reduce.rs` beside
/// `join.rs`) and this row measured one of them, not all.
#[test]
fn the_join_dispatchs_refusal_says_what_it_actually_wires() {
    let a = cyl(1.0, -2.0, 2.0);
    let mut b = cyl(1.0, -0.5, 0.5);
    let (face, _) = b.faces().next().unwrap();
    b.set_face_surface(
        face,
        topo::FaceSurface::New(geom::Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::placeholder(),
        ))),
    )
    .unwrap();
    let err = topo::union(&a, &b, Tol::witness())
        .expect_err("a NURBS wall has no crossing layer in this build");
    assert!(
        matches!(err, BooleanError::CurvedBooleanUnsupported { .. }),
        "expected the crossing-layer refusal, got {err:?}"
    );
    let msg = format!("{err}");
    // The corrected clause: what the JOIN dispatch wires, and that the
    // catch-all is not cone/torus-only.
    assert!(
        msg.contains("germ-pair JOIN dispatch's catch-all"),
        "the refusal no longer names the dispatch it is raised from: {msg}"
    );
    assert!(
        msg.contains("(Plane, Plane), (Plane, Cylinder) and (Plane, Sphere) only"),
        "the refusal does not state what that dispatch wires: {msg}"
    );
    assert!(
        msg.contains("(Sphere, Sphere) or (Cylinder, Sphere) germ reaches the catch-all"),
        "the refusal still reads as cone/torus-only: {msg}"
    );
    // And the distinction from the WIDER dispatch beside it, which is
    // what the false clause conflated it with.
    assert!(
        msg.contains("SECTION-FRAME dispatch"),
        "the refusal drops the dispatch the false clause confused it with: {msg}"
    );
    assert!(
        msg.contains("a frame is not a join arm"),
        "the refusal drops why a wider frame dispatch moves nothing: {msg}"
    );
    // The two Displays must AGREE, which is the half that was broken:
    // `CurvedPairUnsupported` said "(Plane, Cylinder) and (Plane,
    // Sphere) only" while this one implied four wired pairs.
    let pair_msg = format!(
        "{}",
        BooleanError::CurvedPairUnsupported {
            op: None,
            operand: topo::Operand::A,
            face,
            kind: geom_brep::SurfaceKind::Torus,
            other_face: face,
            other_kind: geom_brep::SurfaceKind::Plane,
        }
    );
    assert!(
        pair_msg.contains(
            "germ-pair JOIN dispatch wires (Plane, Plane), (Plane, Cylinder) \
             and (Plane, Sphere) only, mirrors included"
        ),
        "the sibling refusal contradicts this one: {pair_msg}"
    );
    assert!(
        pair_msg.contains("names a frame, never a join arm"),
        "the sibling refusal drops the frame/join distinction: {pair_msg}"
    );
}
