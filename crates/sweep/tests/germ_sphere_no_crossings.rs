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
//!   both faces is certified a loop inside both (R-loop), and builds
//!   through a cut-in of the ball's face whose cut the output stage
//!   closes (`topo::boolean::slit`).
//! - **Sphere × cone** is examined by the pass: a ball strictly inside
//!   the cone is certified apart, and a ball meeting the lateral face in
//!   a loop with no crossing is certified R-loop and builds through a
//!   cut-in of the ball's face, as the scraping ball does.
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
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError, BooleanOp, SolidContainment};

/// A ball of radius `r` centred at `c`, poled along `y`: its seam
/// meridians lie in the plane `z = c.z`.
fn ball_at(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
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
    let placed =
        topo::transform_rigid(&ball, &geom_core::RigidMotion::translation(c).unwrap()).unwrap();
    finished("the ball", placed, Tol::witness())
}

/// A ball of radius `r` centred at `c`, poled along `z`: its seam
/// meridians lie in the plane `y = c.y`. Centred on the donut's axis,
/// each is a circle coaxial with the donut, which the crossing layer's
/// torus residual bounds exactly; a `y`-poled ball's meridians pass
/// through that axis, where the bound is honestly infinite and the
/// circle rung refuses before the no-crossings path is reached.
fn ball_poled_z(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let turn = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::unit_x(), PI / 2.0);
    let placed = topo::transform_rigid(
        &ball,
        &geom_core::test_support::rigid(&turn, Tol::witness()),
    )
    .unwrap();
    let placed =
        topo::transform_rigid(&placed, &geom_core::RigidMotion::translation(c).unwrap()).unwrap();
    finished("the z-poled ball", placed, Tol::witness())
}

/// The donut: `R = 2`, `r = 1/2` about `y`. Its seam meridians lie in
/// the plane `z = 0`, and its two parallels at `y = ±1/2`.
fn donut() -> AtRestBody<f64> {
    let donut = revolve(
        &validated(vec![revolve_common::donut_profile()]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the donut", donut, Tol::witness())
}

/// `2π²Rr²`.
const DONUT_VOLUME: f64 = PI * PI;

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r * r * r
}

fn volume(b: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the volume integrates",
        &topo::test_support::finished("the measured body", topo::Body::clone(b), Tol::witness()),
        Tol::witness(),
    )
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
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
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
/// certificate proves it a loop inside both faces (R-loop). The ball's
/// face is cut in along a meridian of its chart the oval's tube is read
/// crossing, the op re-enters on the crossings, and the output
/// stage closes the cut (`topo::boolean::slit`). Every op builds in both operand orders, with
/// no trace of the cut. The lens is the intersection: the union and the
/// differences answer the operands' closed forms with it, and it answers
/// its integral over slices across the donut's axis
/// (`sweep::test_support::lens`) within the volume claim and ten times
/// the integral's estimated error. (A result is not
/// read by `point_in_solid`, which refuses the ball's face bounded by the
/// oval: `work/inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord.md`.)
#[test]
fn a_ball_scraping_the_tube_builds_through_a_cut_in() {
    let (d, b) = (donut(), ball_at(0.3, Vec3::new(0.0, 0.0, 2.7)));
    let (vd, vb) = (DONUT_VOLUME, ball_volume(0.3));
    // The lens over slices across the donut's axis (major radius 2,
    // tube 1/2, about `y`), the ball's centre 2.7 off it at height 0.
    let (slice, ends) = sweep::test_support::lens::torus_slice(2.0, 0.5);
    let (integral, error) =
        sweep::test_support::lens::ball_in_revolved(0.3, (2.7, 0.0), (slice, &ends), 2000);
    let claim = 1e-6 * (vd + vb);
    for (x, y, order) in [(&d, &b, "d, b"), (&b, &d, "b, d")] {
        let built = |op: BooleanOp, r: Result<topo::BooleanResult<f64>, BooleanError>| {
            let label = format!("{order}, {op:?}");
            let r = r.unwrap_or_else(|e| panic!("{label}: the op builds, got {e:?}"));
            sweep::test_support::cut_trace::assert_no_cut_trace(&label, &r, [x, y]);
            let v = volume(&r.body().expect("a body").body);
            (label, v)
        };
        let lens = built(BooleanOp::Intersect, topo::intersect(x, y, Tol::witness()));
        let l = lens.1;
        assert!(
            (l - integral).abs() <= claim + 10.0 * error,
            "{}: the lens {l} against its integral {integral} ± {error}",
            lens.0
        );
        let (vx, vy) = if order == "d, b" { (vd, vb) } else { (vb, vd) };
        for ((label, v), want) in [
            (
                built(BooleanOp::Union, topo::union(x, y, Tol::witness())),
                vd + vb - l,
            ),
            (lens, l),
            (
                built(BooleanOp::Subtract, topo::subtract(x, y, Tol::witness())),
                vx - l,
            ),
            (
                built(BooleanOp::Subtract, topo::subtract(y, x, Tol::witness())),
                vy - l,
            ),
        ] {
            assert!(
                (v - want).abs() <= claim,
                "{label}: {v} against the closed forms with the lens, {want}"
            );
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
    .into_body()
}

/// The lateral ball's eight ops at ε 1e-6 (∩, ∪ and both differences,
/// the cone first, then the ball first): each escalates, typed, at the
/// pierce curvature (`BooleanDecision::PierceCurvature`), a margin of
/// `1.26e-6` read in the band `[1e-6, 1e-5]` on a ball of radius `0.05`.
/// Measured. Before the cone's cut-in, every op refused at the section
/// pass at every ε.
const LATERAL_COARSE: [&str; 8] = ["escalated PierceCurvature"; 8];

/// **A ball strictly inside the cone, and one meeting its lateral face
/// in a loop with no crossing.** Both boxes meet the cone face's, so the
/// pass takes both up. The ball inside meets the double cone nowhere
/// (W0), and the path certifies it with no re-cut. The ball on the
/// lateral face meets it in one null loop whose witness lies inside
/// both faces, with no event: R-loop, and the pass asks for one cut-in of
/// the ball's face, whose every op builds at the volumes a slice-by-slice
/// integral of the ball's part gives at ε 1e-9 and 1e-12, and escalates,
/// typed, at 1e-6 ([`LATERAL_COARSE`]). The mutant reading that loop's arc
/// of generators as the whole circle claims two essential curves, which
/// W2 clears on the apex-closed face: the certificates pass with no
/// cut-in, red.
#[test]
fn a_ball_against_a_cone_face_is_the_section_passs() {
    let cone = quarter_cone();
    let inside = ball_at(0.05, Vec3::new(0.2, 0.3, -0.2));
    for (a, b) in [(&cone, &*inside), (&*inside, &cone)] {
        assert_eq!(
            topo::test_support::no_crossings_certificates(BooleanOp::Union, a, b, Tol::witness())
                .unwrap(),
            (0, 0),
            "the ball inside is apart"
        );
    }
    let cone_face =
        |f: topo::FaceKey| matches!(cone.face_surface(f), Ok(geom::Surface::Cone { .. }));
    let pairs: Vec<String> = topo::test_support::no_crossings_section_report(
        BooleanOp::Union,
        &cone,
        &inside,
        Tol::witness(),
    )
    .unwrap()
    .into_iter()
    .filter(|(fa, _, _)| cone_face(*fa))
    .map(|(_, _, v)| v)
    .collect();
    assert!(
        !pairs.is_empty() && pairs.iter().all(|v| v == "Ok([])"),
        "the ball inside meets the double cone nowhere (W0): {pairs:?}"
    );
    // The foot `(0.45 cos φ, 0.55, 0.45 sin φ)` at `φ = −π/4`, and the
    // centre `0.045` out along the outward normal `(r̂ + ŷ)/√2`: the ball
    // meets the face in a cap of half-angle `acos 0.9 ≈ 26°`, clear of
    // its own meridians (30° off) and poles (45° off).
    let h = 0.5_f64.sqrt();
    let foot = Vec3::new(0.45 * h, 0.55, -0.45 * h);
    let centre = foot + Vec3::new(0.5, h, -0.5) * 0.045;
    // The ball on the lateral face: the pass reads its loop (R-loop) and
    // asks for one cut-in of the ball's face, nothing deferred.
    let lateral = ball_at(0.05, centre);
    for (a, b) in [(&cone, &*lateral), (&*lateral, &cone)] {
        assert_eq!(
            topo::test_support::no_crossings_certificates(BooleanOp::Union, a, b, Tol::witness())
                .unwrap(),
            (1, 0),
            "the ball on the lateral face is cut in"
        );
    }
    // Every op builds, at the volumes the ball's part of the cone gives,
    // that part integrated over slices across the cone's axis (the cone's
    // slice the disc of radius `1 − y`, the quadrant holding the ball's
    // whole slice).
    let cone = finished("the quarter cone", cone, Tol::witness());
    let (vc, vb) = (PI / 12.0, ball_volume(0.05));
    let e = (centre.x.powi(2) + centre.z.powi(2)).sqrt();
    let (integral, error) = sweep::test_support::lens::ball_in_revolved(
        0.05,
        (e, centre.y),
        (
            |y: f64| (0.0..=1.0).contains(&y).then(|| (0.0, 1.0 - y)),
            &[],
        ),
        2000,
    );
    let claim = 1e-6 * (vc + vb);
    let mut got = Vec::new();
    for (x, y, (vx, vy), order) in [
        (&cone, &lateral, (vc, vb), "the cone first"),
        (&lateral, &cone, (vb, vc), "the ball first"),
    ] {
        // Each op's volume as its certified bracket, `(volume, pad)` (a
        // ball this small does not meet the pinned resolution at a fine
        // ε), or its refusal by kind.
        let run = |op: &str, r: Result<topo::BooleanResult<f64>, BooleanError>| {
            let label = format!("{order}, {op}");
            match r {
                Ok(r) => {
                    sweep::test_support::cut_trace::assert_no_cut_trace(&label, &r, [x, y]);
                    let body = finished(
                        "the measured body",
                        r.body().expect("a body").body.clone(),
                        Tol::witness(),
                    );
                    let m = topo::mass_properties(
                        &body,
                        Tol::witness(),
                        topo::test_support::pinned_resolution(Tol::witness()),
                    )
                    .unwrap_or_else(|e| panic!("{label}: the body does not measure: {e:?}"))
                    .enclosure;
                    (label, Some((m.volume, m.volume_pad)), "built".to_owned())
                }
                Err(
                    e @ (BooleanError::JoinDesync { .. }
                    | BooleanError::ClassificationInvariant { .. }),
                ) => {
                    panic!("{label}: a defect: {e:?}")
                }
                Err(BooleanError::Escalated { decision, .. }) => {
                    (label, None, format!("escalated {decision:?}"))
                }
                Err(e) => (label, None, format!("{e:?}")),
            }
        };
        let runs = [
            run("∩", topo::intersect(x, y, Tol::witness())),
            run("∪", topo::union(x, y, Tol::witness())),
            run("x ∖ y", topo::subtract(x, y, Tol::witness())),
            run("y ∖ x", topo::subtract(y, x, Tol::witness())),
        ];
        for (label, v, outcome) in &runs {
            println!("[lateral] {label}: {outcome} {v:?}");
            got.push(outcome.clone());
        }
        let Some((l, lpad)) = runs[0].1 else {
            continue;
        };
        assert!(
            (l - integral).abs() <= claim + lpad + 10.0 * error,
            "{}: the ball's part {l} ± {lpad:e} against its integral {integral} ± {error}",
            runs[0].0
        );
        for ((label, v, _), want) in runs[1..].iter().zip([vc + vb - l, vx - l, vy - l]) {
            if let Some((v, pad)) = v {
                assert!(
                    (v - want).abs() <= claim + pad + lpad,
                    "{label}: {v} ± {pad:e} against the closed forms with the ball's part, {want}"
                );
            }
        }
    }
    // Every op builds at ε 1e-9 and 1e-12; at ε 1e-6 every op escalates,
    // typed ([`LATERAL_COARSE`]).
    let want: Vec<String> = if Tol::witness().eps() >= 1e-6 {
        LATERAL_COARSE.iter().map(|s| (*s).to_owned()).collect()
    } else {
        vec!["built".to_owned(); 8]
    };
    assert_eq!(got, want, "the ball on the lateral face: each op's outcome");
}
