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
//! - **The crossing coaxial pose gets through the crossing layer and
//!   refuses at the germ frame, `GermFrameUnsupported`.** The cylinder's
//!   SEAM LINE crossing the ball's SPHERE face used to be the door (a
//!   line × sphere pair with no root lane); the crossing layer has that
//!   lane now, so the pose reaches `boolean::join::cs_pair_frame`, which
//!   names a frame only for a DECLARED-coaxial pair — coaxiality is
//!   never inferred — and keeps `NoArm` for this undeclared one.
//! - **A non-crossing coaxial pose — a ball wholly inside a wider
//!   cylinder — refuses at `FallbackExtentUnsupported`** instead, the
//!   containment fallback's curved-extent scan. It is reachable
//!   precisely because no crossing is found first.
//!
//! Every row below pins the refusal as a MEASUREMENT rather than as a
//! target. What would move the first is a coaxiality declaration the
//! frame can read (`work/wire/axis-shaped-identity-channel.md`); what
//! would move the second is a cyl×sphere seam lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
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
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -r), 1.0),
        (Point2::new(0.0, r), 0.0),
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

/// **THE OPENING MEASUREMENT.** The crossing coaxial union gets past
/// the CROSSING layer in both poses and dies at the GERM FRAME, with the
/// same typed variant, naming the cylinder and the sphere.
///
/// The crossing layer used to stop it: operand A's seam LINE crossing
/// the ball's SPHERE face had no root lane. With the line × sphere
/// roots the seam pierces, and the germ pair this crossing mints is a
/// cylinder×sphere pair whose frame `cs_pair_frame` names only under a
/// coaxiality DECLARATION, which no caller can pass yet.
#[test]
fn the_coaxial_union_refuses_at_the_germ_frame() {
    for (label, c, s) in fixture() {
        let err = topo::union(&c, &s, Tol::witness())
            .expect_err("an undeclared coaxial cyl×sphere pair has no frame");
        let BooleanError::GermFrameUnsupported { a_kind, b_kind, .. } = err else {
            panic!("{label}: expected the germ frame door, got {err:?}");
        };
        assert_eq!(
            (a_kind, b_kind),
            (
                geom_brep::SurfaceKind::Cylinder,
                geom_brep::SurfaceKind::Sphere
            ),
            "{label}: the germ pair is the cylinder's wall and the ball's sphere"
        );
    }
}

/// **Both poses take the same door**: a row that greened only on the
/// direct pose would be hiding a pose-dependent answer, so the twin is
/// asserted to the same variant.
#[test]
fn both_poses_take_the_same_door() {
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
    assert_eq!(doors[0], "GermFrameUnsupported", "{doors:?}");
}

/// **The non-coaxial transversal pose still refuses in the crossing
/// layer** — the SSI lane is untouched by this unit. The crossing that
/// used to keep the pierce door is certified now by the circle ×
/// cylinder root lane (`topo::boolean::circle_cylinder`), so the door
/// is the next one in, a pierce whose sector side the wall's curvature
/// swamps (`work/reach/slab-cut-cylinder-refuses-sector-side.md`), in
/// both poses.
#[test]
fn a_transversal_pose_stops_at_the_sector_side_in_both_poses() {
    let c = cyl(1.0, -2.0, 2.0);
    let s = ball_at(1.5, Vec3::new(0.6, 0.0, 0.0));
    for (label, c, s) in [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ] {
        let err = topo::union(&c, &s, Tol::witness()).expect_err("no sector-side lane");
        assert!(
            matches!(err, BooleanError::CurvedSectorSideUnsupported { .. }),
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
/// single door: the crossing pose takes the germ frame and the
/// non-crossing pose takes the scan.
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

/// **A torus operand passes the pair gate and refuses typed at the
/// crossing layer.** The torus is on the union's KIND roster, so the
/// cylinder×torus union is no longer the gate's to refuse. The
/// cylinder's rim circles genuinely cross the tube (the ring passes
/// through the cylinder's end caps at `(0, 0, ±2)`), and that
/// circle×torus crossing has a root lane (`topo::boolean::circle_torus`),
/// as the OTHER direction's does — a circle of the torus against the
/// cylinder's wall, the circle × cylinder lane
/// (`topo::boolean::circle_cylinder`). What refuses is a pierce's
/// sector side, which the wall's curvature swamps
/// (`work/reach/slab-cut-cylinder-refuses-sector-side.md`) — never a
/// body.
///
/// The pair gate's own sentence is pinned on a cone, the kind it still
/// refuses (`review_m3_pr4::curved_face_gate_witness`); the germ-pair
/// join dispatch's text by
/// [`the_join_dispatchs_refusal_says_what_it_actually_wires`] below.
#[test]
fn a_torus_operand_passes_the_pair_gate_and_refuses_at_the_crossing_layer() {
    let torus = {
        // A torus operand reaches the pair/kind refusal, which is the
        // door that carries the fitted-chord sentence.
        let lp = bulge_loop(vec![
            (Point2::new(2.0, -0.3), 1.0),
            (Point2::new(2.0, 0.3), 1.0),
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
    let a = cyl(1.0, -2.0, 2.0);
    // The refusal names no edge, so the sweep's trace says which events
    // it took: a circle of the torus (B) on the cylinder's wall (A).
    let (_, ba) = topo::sweep_traces(
        &a,
        &torus,
        topo::SweepStrategy::Realized,
        None,
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("the sweep refused: {e:?}"));
    let torus_circle_on_wall = ba.accepted.iter().any(|(e, f)| {
        let circle = torus
            .get_edge(*e)
            .and_then(|e| torus.get_curve_geom(e.curve))
            .and_then(topo::CurveGeom::certified)
            .is_some_and(|c| matches!(c.carrier(), topo::Curve3::Circle { .. }));
        let wall = matches!(
            a.get_face(*f).and_then(|f| a.get_surface(f.surface)),
            Some(topo::Surface::Cylinder { .. })
        );
        circle && wall
    });
    assert!(
        torus_circle_on_wall,
        "a torus circle meets the cylinder's wall"
    );
    let err = topo::union(&a, &torus, Tol::witness())
        .expect_err("a pierce's sector side is not certified against the wall's bend");
    assert!(
        matches!(err, BooleanError::CurvedSectorSideUnsupported { .. }),
        "expected the pierce's sector-side door, got {err:?}"
    );
    let msg = format!("{err}");
    assert!(
        msg.contains("where an edge pierces a curved face") && msg.contains("Recourse:"),
        "the refusal names the pierce and ends on its recourse: {msg}"
    );
}

/// **The germ-pair JOIN dispatch's refusal says what that dispatch
/// actually wires** — the corrected clause, read off a CONSTRUCTED
/// `CurvedBooleanUnsupported`, which no row in the tree did before.
///
/// The clause it replaces was created by this unit's own refusal-text
/// sweep and was measured FALSE: it said the join dispatch wires
/// `(Sphere, Sphere)` and a declared-coaxial `(Cylinder, Sphere)`. It
/// does not. `join::bool_connect`'s match has three arms —
/// `(Plane, Plane)`, `(Plane, Sphere) | (Plane, Cylinder)` and the
/// mirror of the second — and its catch-all is the site that raises
/// THIS variant, so a sphere pair or a cyl×sphere germ reaches the
/// catch-all exactly like a cone or torus one. What IS wider is
/// `join::pair_section_frame`, a different dispatch answering a
/// different question: it names a section frame (a centre and an axis
/// for the rotational facing test), never a seam lane. The wired pairs
/// are stated once, in `topo`'s `meeting_recourse`, and every refusal
/// that names them renders that one sentence, so there is no second
/// Display left to disagree with.
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
    // Lifts both refusals: the relabelled face is the join dispatch's input.
    b.set_face_surface_stranding_for_tests(
        face,
        topo::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(geom::NurbsSurface::placeholder())),
            sense: true,
        },
    )
    .unwrap();
    let err = topo::union(&a, &b, Tol::witness())
        .expect_err("a NURBS wall has no crossing layer in this build");
    assert!(
        matches!(err, BooleanError::CurvedBooleanUnsupported { .. }),
        "expected the crossing-layer refusal, got {err:?}"
    );
    let msg = format!("{err}");
    // What the JOIN dispatch wires, stated as the recourse: a plane
    // face against a plane, cylinder or sphere face — so the sentence
    // does not read as cone/torus-only, and does not claim the wider
    // SECTION-FRAME dispatch's pairs as join arms.
    let wired = "they meet only where a plane face meets a plane, cylinder or sphere face";
    assert!(
        msg.contains(wired),
        "the refusal does not state what that dispatch wires: {msg}"
    );
}
