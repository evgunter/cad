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
//! - **A non-crossing pose — a ball wholly inside a wider cylinder —
//!   builds**: no crossing is found, the containment fallback runs, and
//!   the section pass certifies the sphere × wall pair apart.
//!
//! The crossing rows pin the refusal as a MEASUREMENT rather than as a
//! target. What would move them is a coaxiality declaration the frame
//! can read (`work/wire/axis-shaped-identity-channel.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError};

/// The cylinder: a circle of radius `r` at the origin, extruded along
/// world Z from `z0` to `z1`. Its axis is Z.
fn cyl(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
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
            (geom::SurfaceKind::Cylinder, geom::SurfaceKind::Sphere),
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

/// **The non-coaxial transversal pose crosses and reaches the germ
/// frame.** The crossing that used to keep the pierce door is certified
/// now by the circle × cylinder root lane
/// (`topo::boolean::circle_cylinder`), its pierce's sector side
/// certifies, and the cylinder × sphere germ pair it mints has no frame
/// off the coaxial declaration, in both poses.
#[test]
fn a_transversal_pose_reaches_the_germ_frame_in_both_poses() {
    let c = cyl(1.0, -2.0, 2.0);
    let s = ball_at(1.5, Vec3::new(0.6, 0.0, 0.0));
    for (label, c, s) in [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ] {
        let err = topo::union(&c, &s, Tol::witness()).expect_err("no off-axis cyl×sphere frame");
        assert!(
            matches!(
                err,
                BooleanError::GermFrameUnsupported {
                    a_kind: geom::SurfaceKind::Cylinder,
                    b_kind: geom::SurfaceKind::Sphere,
                    ..
                }
            ),
            "{label}: {err:?}"
        );
    }
}

/// The boolean's volume, `None` for an empty result, after tiers 1–3;
/// a refusal fails with the payload.
fn built_volume(label: &str, op: topo::BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Option<f64> {
    let tol = Tol::witness();
    let out = match op {
        topo::BooleanOp::Union => topo::union(a, b, tol),
        topo::BooleanOp::Intersect => topo::intersect(a, b, tol),
        topo::BooleanOp::Subtract => topo::subtract(a, b, tol),
    }
    .unwrap_or_else(|e| panic!("{label}: refused {e:?}"));
    let body = &out.body()?.body;
    assert_eq!(topo::validate(body), Ok(()), "{label}: tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{label}: tier 2");
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: tier 3"
    );
    Some(topo::mass_properties(body, tol).unwrap().volume)
}

/// **The non-crossing pose: a ball wholly inside a wider cylinder.**
/// No crossing exists, so the containment fallback runs, and the
/// sphere × cylinder pair is the section pass's: the ball's sphere and
/// the wall's carrier have no section at all (centred on the axis, and
/// off it), so the pass clears the pair and the vertex probe answers.
/// Every op builds in both operand orders and both poses, against
/// `π r² h` and `4πρ³/3`.
#[test]
fn a_contained_ball_builds_through_the_section_pass() {
    use core::f64::consts::PI;
    use topo::BooleanOp::{Intersect, Subtract, Union};
    let (r, h, rho) = (2.0_f64, 4.0, 0.5_f64);
    let (vc, vs) = (PI * r * r * h, 4.0 / 3.0 * PI * rho.powi(3));
    let c = cyl(r, -2.0, 2.0);
    for centre in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.9, 0.8, -1.2)] {
        let s = ball_at(rho, centre);
        for (pose, c, s) in [
            ("direct", c.clone(), s.clone()),
            ("re-posed twin", posed(&c), posed(&s)),
        ] {
            for (op, x, y, want) in [
                (Union, &c, &s, Some(vc)),
                (Union, &s, &c, Some(vc)),
                (Intersect, &c, &s, Some(vs)),
                (Intersect, &s, &c, Some(vs)),
                (Subtract, &c, &s, Some(vc - vs)),
                (Subtract, &s, &c, None),
            ] {
                let label = format!("{pose}, ball at {centre:?}, {op:?}");
                let got = built_volume(&label, op, x, y);
                match (got, want) {
                    (Some(v), Some(w)) => {
                        assert!((v - w).abs() <= 1e-9 * w, "{label}: {v} against {w}");
                    }
                    (None, None) => {}
                    _ => panic!("{label}: {got:?} against {want:?}"),
                }
            }
        }
    }
}

/// **A ball scraping the wall from inside, in an oval interior to both
/// faces.** Poled along z with its seam meridians in the plane
/// `y = 1.8`, the ball centred at `(0, 1.8, 0)` pokes through the wall
/// (`1.8 + 0.5 > 2`) without any edge of either body crossing a face:
/// the seam stays inside the cylinder, the cylinder's seam line and
/// rims stay clear of the ball. The section is one oval inside both
/// faces, which the section pass certifies and refuses (R-loop), typed
/// as the fallback's extent refusal. Were the pass not to take the pair
/// up, the vertex probe would answer for it and count the lens wrong.
#[test]
fn a_ball_scraping_the_wall_refuses_at_the_section_pass() {
    let c = cyl(2.0, -2.0, 2.0);
    let poled_z = Affine3::translation(Vec3::new(0.0, 1.8, 0.0))
        * Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(1.0, 0.0, 0.0),
            core::f64::consts::FRAC_PI_2,
        );
    let s = topo::transform_rigid(
        &ball_at(0.5, Vec3::new(0.0, 0.0, 0.0)),
        &poled_z,
        Tol::witness(),
    )
    .unwrap();
    for (pose, c, s) in [
        ("direct", c.clone(), s.clone()),
        ("re-posed twin", posed(&c), posed(&s)),
    ] {
        let err = topo::union(&c, &s, Tol::witness()).expect_err("the oval is interior");
        let BooleanError::FallbackExtentUnsupported { what, .. } = err else {
            panic!("{pose}: expected the section pass's refusal, got {err:?}");
        };
        assert!(
            what.contains("closed loop interior to both"),
            "{pose}: {what}"
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
/// (`topo::boolean::circle_cylinder`). Their pierces' sector sides
/// certify, and what refuses is the cylinder × torus germ pair, which
/// has no frame — never a body.
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
        .expect_err("a cylinder × torus germ pair has no frame");
    assert!(
        matches!(
            err,
            BooleanError::GermFrameUnsupported {
                a_kind: geom::SurfaceKind::Cylinder,
                b_kind: geom::SurfaceKind::Torus,
                ..
            }
        ),
        "expected the germ-frame door, got {err:?}"
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
    // The coaxial walls left on one carrier are declared as what the
    // detector finds them to be, so the op reaches its crossing layer.
    let found = topo::flush::find_flush_candidates(&a, &b, Tol::witness()).unwrap();
    let flush = topo::flush::declare_all(&found);
    let err = topo::union_with(&a, &b, &flush, Tol::witness())
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
