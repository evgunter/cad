//! **A sphere against a torus or a cone, certified on the no-crossings
//! path by the section pass.**
//!
//! With no crossing anywhere, the fallback's vertex probe is sound only
//! when every face pair is certified disjoint. The sphere extent scan
//! did that for a sphere's pairs, and refused every torus or cone face
//! its ball's box met: its only certificates are box separation and the
//! plane escape, and neither kind is ever an escape face. The section
//! certificate answers the question the scan was asking — with no
//! event anywhere, "every component cleared" is disjointness — so those
//! pairs are now the section pass's, and the scan steps past them.
//!
//! - **Sphere × torus** answers where the certificate clears: a ball in
//!   the donut's hole, a ball around the whole donut, a ball inside the
//!   tube. Each union is checked against its closed-form volume and
//!   `point_in_solid`. A ball scraping the tube in an oval interior to
//!   both faces refuses R-loop, typed as the fallback's extent refusal
//!   naming the torus face.
//! - **Sphere × cone** is examined by the pass, and refuses on reach
//!   until the certificate's cone rows land: the rows run the path's
//!   two certificates directly, since the operand gate keeps every cone
//!   pair off the operations.
//!
//! The mutant is the scan stepping past the pair without the pass
//! taking it up: then nothing examines the pair, the scraping ball's
//! union comes back a valid body that counts the lens twice, and the
//! cone rows certify a pair no one looked at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use revolve_common::{axis_y, validated};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, BooleanError, SolidContainment};

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

/// A ball of radius `r` centred at `c`, poled along `z`: its seam
/// meridians lie in the plane `y = c.y`. Centred on the donut's axis,
/// each is a circle coaxial with the donut, which the crossing layer's
/// torus residual bounds exactly; a `y`-poled ball's meridians pass
/// through that axis, where the bound is honestly infinite and the
/// circle rung refuses before the no-crossings path is reached.
fn ball_poled_z(r: f64, c: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let turn = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::unit_x(), PI / 2.0);
    let placed = topo::transform_rigid(&ball, &turn, Tol::witness()).unwrap();
    topo::transform_rigid(&placed, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// The donut: `R = 2`, `r = 1/2` about `y`. Its seam meridians lie in
/// the plane `z = 0`, and its two parallels at `y = ±1/2`.
fn donut() -> Body<f64> {
    revolve(
        &validated(vec![revolve_common::donut_profile()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// `2π²Rr²`.
const DONUT_VOLUME: f64 = PI * PI;

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r * r * r
}

fn volume(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, Tol::witness())
        .expect("the volume integrates")
        .volume
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: {got} against the closed form {want}"
    );
}

fn pis(b: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    topo::point_in_solid(b, q, band, Tol::witness()).unwrap()
}

/// The union's one body, its volume against `want`, and each named
/// point's containment.
fn union_answers(
    a: &Body<f64>,
    b: &Body<f64>,
    want: f64,
    points: &[(Point3<f64>, SolidContainment)],
    what: &str,
) {
    for (x, y, order) in [(a, b, "a ∪ b"), (b, a, "b ∪ a")] {
        let r =
            topo::union(x, y, Tol::witness()).unwrap_or_else(|e| panic!("{what}, {order}: {e:?}"));
        let body = &r.body().expect("a union is never empty").body;
        close(volume(body), want, &format!("{what}, {order}"));
        for &(q, c) in points {
            assert_eq!(pis(body, q), c, "{what}, {order}: {q:?}");
        }
    }
}

/// **A ball in the donut's hole** (`r = 1` at the origin; the hole's
/// radius is `3/2`). The ball's box meets the donut's inner faces, so
/// the scan refused; the certificate finds no component (W0). The union
/// is the two solids side by side.
#[test]
fn a_ball_in_the_donuts_hole_unions_by_its_closed_form() {
    union_answers(
        &donut(),
        &ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0)),
        DONUT_VOLUME + ball_volume(1.0),
        &[
            (Point3::new(0.0, 0.0, 0.0), SolidContainment::In),
            (Point3::new(0.0, 0.3, 2.0), SolidContainment::In),
            (Point3::new(1.25, 0.0, 0.0), SolidContainment::Out),
            (Point3::new(0.0, 0.0, 3.0), SolidContainment::Out),
        ],
        "ball in the hole",
    );
}

/// **A ball around the whole donut** (`r = 3 > R + r`): no component
/// again, and the union is the ball.
#[test]
fn a_ball_around_the_donut_unions_to_the_ball() {
    union_answers(
        &donut(),
        &ball_poled_z(3.0, Vec3::new(0.0, 0.0, 0.0)),
        ball_volume(3.0),
        &[
            (Point3::new(0.0, 0.0, 0.0), SolidContainment::In),
            (Point3::new(0.0, 0.0, 2.0), SolidContainment::In),
            (Point3::new(2.9, 0.0, 0.0), SolidContainment::In),
            (Point3::new(0.0, 3.1, 0.0), SolidContainment::Out),
        ],
        "ball around the donut",
    );
}

/// **A ball inside the tube** (`r = 1/5` on the core circle, clear of
/// every donut edge): the union is the donut.
#[test]
fn a_ball_inside_the_tube_unions_to_the_donut() {
    union_answers(
        &donut(),
        &ball_at(0.2, Vec3::new(0.0, 0.0, 2.0)),
        DONUT_VOLUME,
        &[
            (Point3::new(0.0, 0.0, 2.0), SolidContainment::In),
            (Point3::new(0.0, 0.0, 2.45), SolidContainment::In),
            (Point3::new(0.0, 0.0, 0.0), SolidContainment::Out),
        ],
        "ball inside the tube",
    );
}

/// **A ball scraping the tube's outer equator** (`r = 3/10` at
/// `(0, 0, 2.7)`): it meets the torus in one oval, interior to both
/// faces — clear of the donut's seams and parallels, and of the ball's
/// own meridians in the plane `z = 2.7`. No crossing sees it, and the
/// certificate proves it a loop inside both faces: R-loop, refused as
/// the fallback's extent refusal naming the torus face. The witness
/// point `(0, 0, 2.45)` is inside both operands, so a union that
/// answered would count that lens twice.
#[test]
fn a_ball_scraping_the_tube_refuses_as_an_interior_loop() {
    let (d, b) = (donut(), ball_at(0.3, Vec3::new(0.0, 0.0, 2.7)));
    let q = Point3::new(0.0, 0.0, 2.45);
    assert_eq!(pis(&d, q), SolidContainment::In);
    assert_eq!(pis(&b, q), SolidContainment::In);
    for (x, y, operand) in [(&d, &b, topo::Operand::A), (&b, &d, topo::Operand::B)] {
        match topo::union(x, y, Tol::witness()) {
            Err(BooleanError::FallbackExtentUnsupported {
                operand: o,
                face,
                what,
            }) => {
                assert_eq!(o, operand);
                let torus = if operand == topo::Operand::A { x } else { y };
                assert!(
                    matches!(
                        torus.get_surface(torus.get_face(face).unwrap().surface),
                        Some(geom::Surface::Torus { .. })
                    ),
                    "the refusal names the torus face"
                );
                assert!(
                    what.contains("closed loop interior to both faces"),
                    "{what}"
                );
            }
            Err(e) => panic!("refused, but not as an interior loop: {e:?}"),
            Ok(r) => panic!(
                "answered a body of volume {:?}",
                r.body().map(|b| volume(&b.body))
            ),
        }
    }
}

// -------------------------------------------------------------------
// Sphere × cone: examined by the pass.
// -------------------------------------------------------------------

/// The `revolve_cone` triangle revolved a quarter turn: apex
/// `(0, 1, 0)`, base radius `1` at `y = 0`, the quadrant `x > 0, z < 0`.
fn quarter_cone() -> Body<f64> {
    revolve(
        &validated(vec![ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
        ])]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The path's certificates refuse on reach, naming the cone face: the
/// pass examined the pair and has no cone arm yet.
fn refused_by_the_pass_on_the_cone(cone: &Body<f64>, ball: &Body<f64>, what: &str) {
    for (a, b, operand) in [
        (cone, ball, topo::Operand::A),
        (ball, cone, topo::Operand::B),
    ] {
        match topo::test_support::no_crossings_certificates(a, b, Tol::witness()) {
            Err(BooleanError::FallbackExtentUnsupported {
                operand: o,
                face,
                what: why,
            }) => {
                assert_eq!(o, operand, "{what}");
                assert!(
                    matches!(
                        cone.get_surface(cone.get_face(face).unwrap().surface),
                        Some(geom::Surface::Cone { .. })
                    ),
                    "{what}: the refusal names the cone face"
                );
                assert!(why.contains("no section classification"), "{what}: {why}");
            }
            other => panic!("{what}: {other:?}"),
        }
    }
}

/// **A ball strictly inside the cone, and one meeting its lateral face
/// in a loop with no crossing.** Both boxes meet the cone face's, so the
/// scan refused both by the cone's kind. The pass takes both up now,
/// and both refuse on reach until the certificate's cone rows land.
#[test]
fn a_ball_against_a_cone_face_is_the_section_passs() {
    let cone = quarter_cone();
    refused_by_the_pass_on_the_cone(
        &cone,
        &ball_at(0.05, Vec3::new(0.2, 0.3, -0.2)),
        "the ball inside",
    );
    // The foot `(0.45 cos φ, 0.55, 0.45 sin φ)` at `φ = −π/4`, and the
    // centre `0.045` out along the outward normal `(r̂ + ŷ)/√2`: the ball
    // meets the face in a cap of half-angle `acos 0.9 ≈ 26°`, clear of
    // its own meridians (30° off) and poles (45° off).
    let h = 0.5_f64.sqrt();
    let foot = Vec3::new(0.45 * h, 0.55, -0.45 * h);
    let centre = foot + Vec3::new(0.5, h, -0.5) * 0.045;
    refused_by_the_pass_on_the_cone(
        &cone,
        &ball_at(0.05, centre),
        "the ball on the lateral face",
    );
}
