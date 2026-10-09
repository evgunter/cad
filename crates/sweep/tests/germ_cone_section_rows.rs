//! **The section certificate's cone rows on real bodies**, at verdict
//! level: the preview cone of `docs/GERM-VERBS-CONE-SPEC.md` §0 against
//! its fixtures, and a seamed frustum against a tilted slab.
//!
//! The rows read the certificate directly: on the crossings path with
//! the events of the cone's root lane
//! (`topo::section_report_admitting_cones`, behind `sweep-testing`), and
//! on the no-crossings path
//! (`topo::test_support::no_crossings_section_report`), each pair with
//! the cone face, its components cleared or refused with no event
//! anywhere. A W1 or W2 clearance holds whatever the crossing layer
//! finds; W3 and the no-event decision are what the pass answers when it
//! finds nothing. The bodies the ops return on these fixtures are
//! measured in `cone_operand_rows.rs`.
//!
//! The preview cone is the triangle `(0,0) (1,0) (0,1)` revolved fully
//! about `y`, merged to one cone face: apex `(0, 1, 0)`, lateral face
//! `ρ = 1 − y`, apex-closed, so it describes through the apex closure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::test_support::{brick, no_crossings_certificates, no_crossings_section_report};
use topo::{Body, BooleanError, FaceKey};

/// `polygon` (radius, height) revolved about `y`, fully (merged) or
/// through `theta`.
fn revolved(polygon: &[(f64, f64)], theta: Option<f64>) -> Body<f64> {
    let loop_ = ProfileLoop::polygon(polygon.iter().map(|&(x, y)| Point2::new(x, y)));
    let mut body = revolve(
        &validated(vec![loop_]),
        axis_y(),
        theta.map_or(Revolution::Full, Revolution::Partial),
        Tol::witness(),
    )
    .unwrap()
    .body;
    if theta.is_none() {
        body.merge_coplanar_faces(Tol::witness()).unwrap();
    }
    assert_all_tiers(&body);
    body
}

/// The preview cone.
fn cone() -> Body<f64> {
    revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], None)
}

/// A solid round pin of radius `r` about the `y` line through
/// `(x, ·, 0)`, over `y ∈ [y0, y1]`.
fn pin(r: f64, x: f64, y0: f64, y1: f64) -> Body<f64> {
    let body = revolved(&[(0.0, y0), (r, y0), (r, y1), (0.0, y1)], None);
    topo::transform_rigid(
        &body,
        &Affine3::translation(Vec3::new(x, 0.0, 0.0)),
        Tol::witness(),
    )
    .unwrap()
}

fn is_cone(body: &Body<f64>, face: FaceKey) -> bool {
    matches!(
        body.get_surface(body.get_face(face).unwrap().surface),
        Some(geom::Surface::Cone { .. })
    )
}

/// The pass's verdict on every pair of `a`'s cone face with a face of
/// `b`. Run both ways round, the verdicts agree up to which face a W2
/// or W3 names: the rule asks `F`'s face first, so a component essential
/// on both clears on whichever face is operand A.
fn cone_verdicts(a: &Body<f64>, b: &Body<f64>) -> Vec<String> {
    let sideless = |v: &str| v.replace("(F)", "").replace("(G)", "");
    let mut forward: Vec<String> = no_crossings_section_report(a, b, Tol::witness())
        .unwrap()
        .into_iter()
        .filter(|(fa, _, _)| is_cone(a, *fa))
        .map(|(_, _, v)| v)
        .collect();
    let mut back: Vec<String> = no_crossings_section_report(b, a, Tol::witness())
        .unwrap()
        .into_iter()
        .filter(|(_, fb, _)| is_cone(a, *fb))
        .map(|(_, _, v)| sideless(&v))
        .collect();
    forward.sort();
    back.sort();
    let mut plain: Vec<String> = forward.iter().map(|v| sideless(v)).collect();
    plain.sort();
    assert_eq!(plain, back, "the cone as F and as G");
    forward
}

/// Every verdict is one of `allowed`, and each of `required` occurs.
fn verdicts_are(v: &[String], allowed: &[&str], required: &[&str], what: &str) {
    assert!(!v.is_empty(), "{what}: the pass examined no cone pair");
    for x in v {
        assert!(allowed.contains(&x.as_str()), "{what}: {x} in {v:?}");
    }
    for r in required {
        assert!(v.iter().any(|x| x == r), "{what}: no {r} in {v:?}");
    }
}

const W1: &str = "Ok([Unbounded, Unbounded])";
const W2: &str = "Ok([Essential(F)])";

/// **Brick fixtures: planes cut the cone in parallels and hyperbolas.**
/// A brick's faces are axis-normal (a parallel: the ellipse's class,
/// essential, cleared W2 on the cone face, which describes through the
/// apex closure) or contain the axis direction (a hyperbola, W1). Every
/// pair clears and the certificates pass:
///
/// - P3, the apex pin (its pierces near the apex are premise S's, the
///   crossing layer's root lane, not the certificate's);
/// - P4, a brick clear of the cone with overlapping boxes;
/// - P5, the axis-normal slab;
/// - P6, a box strictly inside;
/// - P2a, a pin through the base disc.
///
/// The mutants: a parallel read unbounded clears the same, so these
/// rows pin the answer rate; the ellipse's class read as R-reach (the
/// cone arm removed) reds every row.
#[test]
fn the_preview_bricks_clear_by_w1_and_w2() {
    let cone = cone();
    for (what, b) in [
        ("P3", ((-0.02, 0.02), (0.5, 1.5), (0.02, 0.06))),
        ("P4", ((-0.1, 0.1), (0.2, 0.3), (0.9, 1.2))),
        ("P5", ((-2.0, 2.0), (0.3, 0.6), (-2.0, 2.0))),
        ("P6", ((-0.3, 0.3), (0.2, 0.4), (-0.3, 0.3))),
        ("P2a", ((-0.7, -0.5), (-0.3, 0.1), (-0.1, 0.1))),
    ] {
        let brick = brick::<f64>(b.0, b.1, b.2, Tol::witness());
        let v = cone_verdicts(&cone, &brick);
        verdicts_are(&v, &[W1, W2], &[W2], what);
        assert_eq!(
            no_crossings_certificates(&cone, &brick, Tol::witness()).unwrap(),
            0,
            "{what}"
        );
    }
}

/// **P10, the coaxial pin**: its wall meets the cone in parallels,
/// essential on both, cleared W2; its end discs cut parallels too.
#[test]
fn the_coaxial_pin_clears_by_w2() {
    let v = cone_verdicts(&cone(), &pin(0.1, 0.0, -0.5, 0.5));
    verdicts_are(
        &v,
        &[W2, "Ok([Essential(F), Essential(F)])"],
        &["Ok([Essential(F), Essential(F)])"],
        "P10",
    );
}

/// **A parallel-axis pin through the lateral face**: its wall, not
/// enclosing the cone's axis, meets each nappe in one loop — null on
/// the cone, but encircling the pin, so W2 clears it on the pin's wall,
/// which carries its seam. The mutant reading the wall's side as not
/// essential leaves the face's own loop to its witness, inside both
/// faces with no event: R-loop, red.
#[test]
fn a_parallel_pin_meets_the_cone_in_loops_cleared_on_the_pin() {
    let cone = cone();
    let pin = pin(0.1, 0.4, -0.5, 1.5);
    let v = cone_verdicts(&cone, &pin);
    verdicts_are(
        &v,
        &[W2, "Ok([Essential(G), Essential(G)])"],
        &["Ok([Essential(G), Essential(G)])"],
        "the pin",
    );
    assert_eq!(
        no_crossings_certificates(&cone, &pin, Tol::witness()).unwrap(),
        0
    );
}

/// **P9, the cone nested in a big box**: no face of the box comes near
/// the cone face's box, so the pass has no cone pair to examine.
#[test]
fn the_nested_cone_has_no_cone_pair() {
    let cone = cone();
    let big = brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (-3.0, 3.0), Tol::witness());
    assert!(cone_verdicts(&cone, &big).is_empty());
    assert_eq!(
        no_crossings_certificates(&cone, &big, Tol::witness()).unwrap(),
        0
    );
}

/// **P1, an oblique rod**: the preview's bite carrier, a cylinder along
/// `x` about `(y, z) = (0.45, 0.8)`, radius `0.3`. Cone × oblique
/// cylinder has no arm: R-reach, naming the cone face.
#[test]
fn an_oblique_rod_refuses_on_reach() {
    let cone = cone();
    let along_x = topo::transform_rigid(
        &pin(0.3, 0.0, -2.0, 2.0),
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -PI / 2.0),
        Tol::witness(),
    )
    .unwrap();
    let rod = topo::transform_rigid(
        &along_x,
        &Affine3::translation(Vec3::new(0.0, 0.45, 0.8)),
        Tol::witness(),
    )
    .unwrap();
    let v = cone_verdicts(&cone, &rod);
    assert!(v.iter().any(|x| x == "Err(Reach)"), "{v:?}");
    match no_crossings_certificates(&cone, &rod, Tol::witness()) {
        Err(BooleanError::FallbackExtentUnsupported {
            operand,
            face,
            what,
        }) => {
            assert!(what.contains("no section classification"), "{what}");
            assert!(
                operand == topo::Operand::A && is_cone(&cone, face),
                "{face:?}"
            );
        }
        other => panic!("{other:?}"),
    }
}

/// **The ellipse on a cone face with its seam, and on apex-closed
/// sectors**: a slab tilted `0.2` rad cuts a seamed frustum (the
/// trapezoid revolved fully, not merged) and the preview cone's `3π/2`
/// sector (P7's) in ellipses, each cleared W2.
#[test]
fn a_tilted_slab_cuts_ellipses_cleared_by_w2() {
    let slab = topo::transform_rigid(
        &brick::<f64>((-2.0, 2.0), (0.2, 0.3), (-2.0, 2.0), Tol::witness()),
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.25, 0.0), Vec3::unit_z(), 0.2),
        Tol::witness(),
    )
    .unwrap();
    let frustum = revolve(
        &validated(vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.5, 0.5),
            Point2::new(0.0, 0.5),
        ])]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_all_tiers(&frustum);
    for (what, body) in [
        ("the seamed frustum", frustum),
        (
            "the 3π/2 sector",
            revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], Some(1.5 * PI)),
        ),
    ] {
        let v = cone_verdicts(&body, &slab);
        verdicts_are(&v, &[W1, W2], &[W2], what);
    }
}

// -------------------------------------------------------------------
// The crossings path: the cone sweep's events
// -------------------------------------------------------------------

/// **With the crossing layer's events, the preview pairs still clear
/// by W1 and W2**: P2a's pin through the base disc, P3's apex pin (whose
/// four pierces near the apex the cone's root lane now records), P5's
/// slab, P10's coaxial pin and a parallel pin through the lateral face,
/// each with the cone as `A` and as `B`. Every component these pairs
/// carry is unbounded or essential on a face that describes, so the
/// events change nothing: the rows pin that the crossings path reads
/// the same arms. The mutant reading the ellipse's class as R-reach
/// reds them.
#[test]
fn the_preview_pairs_clear_with_the_sweeps_events() {
    let cone = cone();
    let fixtures: Vec<(&str, Body<f64>)> = vec![
        (
            "P2a",
            brick::<f64>((-0.7, -0.5), (-0.3, 0.1), (-0.1, 0.1), Tol::witness()),
        ),
        (
            "P3",
            brick::<f64>((-0.02, 0.02), (0.5, 1.5), (0.02, 0.06), Tol::witness()),
        ),
        (
            "P5",
            brick::<f64>((-2.0, 2.0), (0.3, 0.6), (-2.0, 2.0), Tol::witness()),
        ),
        ("P10", pin(0.1, 0.0, -0.5, 0.5)),
        ("the parallel pin", pin(0.1, 0.4, -0.5, 1.5)),
    ];
    let cleared = |v: &str| {
        matches!(
            v,
            "Ok([Unbounded, Unbounded])"
                | "Ok([Essential(F)])"
                | "Ok([Essential(G)])"
                | "Ok([Essential(F), Essential(F)])"
                | "Ok([Essential(G), Essential(G)])"
        )
    };
    for (what, b) in &fixtures {
        for (x, y, cone_is_a) in [(&cone, b, true), (b, &cone, false)] {
            let v: Vec<String> = topo::section_report_admitting_cones(x, y, Tol::witness())
                .unwrap_or_else(|e| panic!("{what}: the sweep refused {e:?}"))
                .into_iter()
                .filter(|(fa, fb, _)| {
                    if cone_is_a {
                        is_cone(&cone, *fa)
                    } else {
                        is_cone(&cone, *fb)
                    }
                })
                .map(|(_, _, v)| v)
                .collect();
            assert!(!v.is_empty(), "{what}: no cone pair");
            assert!(
                v.iter().all(|x| cleared(x)),
                "{what}, cone as A {cone_is_a}: {v:?}"
            );
        }
    }
}

/// A ball of radius `r` centred at `c`, poled along `y`: its seam
/// meridians lie in the plane `z = c.z`.
fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
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
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// **W4 through the door: a ball on the quarter cone's generator
/// edge.** The ball meets the lateral face in one null loop, which the
/// edge `φ = 0` crosses: the sweep records the edge's pierces of the
/// ball, the reduction's own reading pairs them with the cone face, and
/// the certified single loop clears by W4 against each of the ball's
/// faces — with the cone as `A` and as `B`. With no events read, the same loop's witness lies inside both
/// faces and the pair refuses R-loop: the mutant that drops the door's
/// events reds the row.
#[test]
fn a_ball_across_a_generator_edge_clears_by_w4_through_the_door() {
    let quarter = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)], Some(PI / 2.0));
    let h = 0.5_f64.sqrt();
    let ball = ball_at(
        0.05,
        Vec3::new(0.45, 0.55, -0.01) + Vec3::new(h, h, 0.0) * 0.045,
    );
    for (a, b, cone_is_a) in [(&quarter, &ball, true), (&ball, &quarter, false)] {
        let v: Vec<String> = topo::section_report_admitting_cones(a, b, Tol::witness())
            .unwrap()
            .into_iter()
            .filter(|(fa, fb, _)| {
                if cone_is_a {
                    is_cone(&quarter, *fa)
                } else {
                    is_cone(&quarter, *fb)
                }
            })
            .map(|(_, _, v)| v)
            .collect();
        assert!(
            !v.is_empty() && v.iter().all(|x| x == "Ok([LoneEvented])"),
            "cone as A {cone_is_a}: {v:?}"
        );
    }
    let quiet: Vec<String> = no_crossings_section_report(&quarter, &ball, Tol::witness())
        .unwrap()
        .into_iter()
        .filter(|(fa, _, _)| is_cone(&quarter, *fa))
        .map(|(_, _, v)| v)
        .collect();
    assert!(
        quiet.iter().any(|x| x == "Err(Loop)"),
        "with no event the loop is interior: {quiet:?}"
    );
}
