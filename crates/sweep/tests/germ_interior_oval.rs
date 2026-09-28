//! **A curved face that meets the other operand only in a closed loop
//! interior to both faces, while crossings exist elsewhere.**
//!
//! Nothing on the crossings path sees such a loop: no edge event marks
//! it, the join cuts nothing along it, and face-region propagation
//! carries each face's side across it. On main before the guard both
//! fixtures below came back as VALID bodies that were wrong — the
//! overlap counted twice under ∪ and dropped under ∩ and ∖. The guard
//! (`ops::interior_loop_verdict`) refuses them typed, and these rows
//! pin that, the controls it must leave answering, and the one correct
//! answer it gives up.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::{axis_y, validated};
use sweep::{Revolution, revolve};
use topo::Body;

/// A C-shaped profile in the `xz` plane, extruded symmetrically in `y`.
fn bracket_xz(pts: &[(f64, f64)], half_y: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, z)| Point2::new(x, z)));
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_z(), -Vec3::unit_y()),
        Vec3::new(0.0, half_y, 0.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the bracket profile validates");
    sweep::extrude(
        &vp,
        sweep::Extrusion::Distance(2.0 * half_y),
        Tol::witness(),
    )
    .expect("the bracket extrudes")
    .body
}

fn half_donut() -> Body<f64> {
    let vp = validated(vec![revolve_common::donut_profile()]);
    revolve(
        &vp,
        axis_y(),
        Revolution::Partial(std::f64::consts::PI),
        Tol::witness(),
    )
    .expect("the half donut revolves")
    .body
}

fn torus_bracket() -> Body<f64> {
    bracket_xz(
        &[
            (1.95, -0.1),
            (2.05, -0.1),
            (2.05, 0.8),
            (3.0, 0.8),
            (3.0, -2.45),
            (-1.0, -2.45),
            (-1.0, -2.8),
            (3.2, -2.8),
            (3.2, 1.0),
            (1.95, 1.0),
        ],
        0.3,
    )
}

fn dome() -> Body<f64> {
    let t = (std::f64::consts::PI / 8.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), t),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let mut up = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the dome revolves")
        .body;
    up.merge_coplanar_faces(Tol::witness())
        .expect("the split base disc merges");
    up
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Vec3::new(0.0, 0.0, z.0),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .expect("the box profile validates");
    sweep::extrude(&vp, sweep::Extrusion::Distance(z.1 - z.0), Tol::witness())
        .expect("the box extrudes")
        .body
}

/// The cap direction: azimuth 90° off the dome's seam, latitude 30°.
fn cap_dir() -> Vec3<f64> {
    Vec3::new(0.0, 0.5, 0.75_f64.sqrt())
}

fn dome_bracket() -> Body<f64> {
    let top = boxed((-0.6, 0.6), (0.9, 1.5), (-0.6, 0.6));
    let u = cap_dir();
    let e2 = Vec3::new(0.0, 0.75_f64.sqrt(), -0.5);
    let foot = topo::transform_rigid(
        &boxed((-0.55, 0.55), (-0.6, 1.2), (0.95, 1.3)),
        &Affine3::from_parts(
            Mat3::from_cols(Vec3::unit_x(), e2, u),
            Vec3::new(0.0, 0.0, 0.0),
        ),
        Tol::witness(),
    )
    .expect("the foot tilts");
    let r = topo::union(&top, &foot, Tol::witness()).expect("the bracket's two boxes union");
    r.body().expect("non-empty").body.clone()
}

fn top_box() -> Body<f64> {
    boxed((-0.6, 0.6), (0.9, 1.5), (-0.6, 0.6))
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

fn refuses_as_the_interior_loop_guard(
    r: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    op: topo::BooleanOp,
    kind: geom_brep::SurfaceKind,
    what: &str,
) {
    match r {
        Err(topo::BooleanError::CurvedPairUnsupported {
            op: Some(o),
            kind: k,
            ..
        }) => {
            assert_eq!((o, k), (op, kind), "{what}");
        }
        Err(e) => panic!("{what}: refused, but not by the guard: {e:?}"),
        Ok(r) => panic!(
            "{what}: answered {:?}, volume {:?}",
            r.body().map(|b| b.kind),
            r.body().map(|b| volume(&b.body))
        ),
    }
}

/// **The live case: a half donut and a bracket whose foot cuts an oval
/// off the outer equator.** The pin's crossings through the cap are the
/// only events, so the oval was never seen, and ∪ came back a valid
/// `Seamed` body of volume `π²/2 + 1.476 − 0.006` — the lens counted
/// twice. Each op now refuses at the guard.
#[test]
fn a_torus_oval_behind_crossings_elsewhere_refuses_every_op() {
    let (h, c) = (half_donut(), torus_bracket());
    for (op, r) in [
        (topo::BooleanOp::Union, topo::union(&h, &c, Tol::witness())),
        (
            topo::BooleanOp::Intersect,
            topo::intersect(&h, &c, Tol::witness()),
        ),
    ] {
        refuses_as_the_interior_loop_guard(
            r,
            op,
            geom_brep::SurfaceKind::Torus,
            "half donut and bracket",
        );
    }
}

/// **What the torus half gives up, stated.** The bracket without its
/// foot — the pin alone crossing the cap — has no loop anywhere, and its
/// union was the correct `π²/2 + 0.594 − 0.006`. Its pin still stands
/// inside the torus faces' boxes with no event between them, and the
/// guard refuses on reach, as the no-crossings extent gate does. A
/// guard that learns to answer this is a visible change here.
#[test]
fn the_pin_alone_is_the_torus_guards_conservative_refusal() {
    let pin_only = bracket_xz(
        &[
            (1.95, -0.1),
            (2.05, -0.1),
            (2.05, 0.8),
            (3.0, 0.8),
            (3.0, -2.45),
            (3.2, -2.45),
            (3.2, 1.0),
            (1.95, 1.0),
        ],
        0.3,
    );
    refuses_as_the_interior_loop_guard(
        topo::union(&half_donut(), &pin_only, Tol::witness()),
        topo::BooleanOp::Union,
        geom_brep::SurfaceKind::Torus,
        "the pin alone",
    );
}

/// **The sphere analogue: a dome and a bracket whose foot cuts a cap off
/// the dome's side, clear of its seams, while the bracket's top crosses
/// the dome's crown.** On main every op answered a valid body missing
/// the side cap: `dome ∩ bracket` came back as the crown alone, and a
/// point inside the side cap, inside both operands, read `Out` of it.
#[test]
fn a_sphere_cap_behind_crossings_elsewhere_refuses_every_op() {
    let (d, c) = (dome(), dome_bracket());
    let band = geom_core::Band::linear(Tol::witness()).expect("the run's band");
    let q = Point3::origin() + cap_dir() * 0.97;
    for (name, body) in [("dome", &d), ("bracket", &c)] {
        assert!(
            matches!(
                topo::point_in_solid(body, q, band, Tol::witness()),
                Ok(topo::SolidContainment::In)
            ),
            "the witness point is inside the {name}"
        );
    }
    for (op, r) in [
        (topo::BooleanOp::Union, topo::union(&d, &c, Tol::witness())),
        (
            topo::BooleanOp::Intersect,
            topo::intersect(&d, &c, Tol::witness()),
        ),
        (
            topo::BooleanOp::Subtract,
            topo::subtract(&d, &c, Tol::witness()),
        ),
        (
            topo::BooleanOp::Subtract,
            topo::subtract(&c, &d, Tol::witness()),
        ),
    ] {
        refuses_as_the_interior_loop_guard(
            r,
            op,
            geom_brep::SurfaceKind::Sphere,
            "dome and bracket",
        );
    }
}

/// **The sphere lane the guard must leave answering.** The bracket's top
/// box alone cuts the dome's crown at `y = 0.9`: the section circle
/// crosses the dome's seams, so the pair has events, and the other box
/// faces stand clear of the ball. The answers are the closed forms —
/// the crown cap `πh²(3r − h)/3` at `h = 0.1` and its complements —
/// valid at tier 3.
#[test]
fn the_crown_alone_still_answers_its_closed_form() {
    let (d, t) = (dome(), top_box());
    let cap = std::f64::consts::PI * 0.01 * (3.0 - 0.1) / 3.0;
    let dome_v = 2.0 * std::f64::consts::PI / 3.0;
    let box_v = 1.2 * 0.6 * 1.2;
    for (what, r, want) in [
        ("dome ∩ top", topo::intersect(&d, &t, Tol::witness()), cap),
        (
            "dome ∖ top",
            topo::subtract(&d, &t, Tol::witness()),
            dome_v - cap,
        ),
        (
            "dome ∪ top",
            topo::union(&d, &t, Tol::witness()),
            dome_v + box_v - cap,
        ),
    ] {
        let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let b = &r.body().expect("non-empty").body;
        assert_eq!(
            topo::validate_geometric(b, Tol::witness()),
            Ok(()),
            "{what}"
        );
        close(volume(b), want, what);
    }
}

/// **Whatever ∪ answers for the half donut and bracket, it does not
/// count the lens twice.** The pre-guard answer was a valid body of
/// volume `vol(H) + vol(C) − 0.006` — only the pin's overlap removed.
/// The true union is smaller by the lens, so a body at that volume (or
/// above) is the wrong answer, and a refusal is the only other outcome
/// this row accepts.
#[test]
fn the_half_donut_union_never_counts_the_lens_twice() {
    let (h, c) = (half_donut(), torus_bracket());
    let twice = volume(&h) + volume(&c) - 0.006;
    if let Ok(r) = topo::union(&h, &c, Tol::witness()) {
        let got = volume(&r.body().expect("non-empty").body);
        assert!(
            got < twice - 1e-6,
            "the union's volume {got} counts the lens twice (vol A + vol B − pin = {twice})"
        );
    }
}

/// **The corner bar: a second lens, and the torus half's per-op reach
/// is what covers it.** A `0.2`-square bar across the donut's hole, cut
/// to the length that puts all eight corners on the inner face. Its end
/// squares' edges along `y` keep a constant `ρ`, so their interiors run
/// inside the tube, and the lens between each end square and the tube
/// is bounded by no event but the corners' own contacts. A gate that
/// cleared a torus pair because it HAS events would pass it; the torus
/// half never consults events — any undeclared overlapping pair refuses
/// unless the carriers are certified apart — so whatever door the
/// pipeline reaches first, the result is never a body.
#[test]
fn the_corner_bar_never_comes_back_a_body() {
    let d = {
        let vp = validated(vec![revolve_common::donut_profile()]);
        revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
            .expect("the donut revolves")
            .body
    };
    let hw = 0.1_f64;
    let rho = 2.0 - (0.25 - hw * hw).sqrt();
    let z = (rho * rho - hw * hw).sqrt();
    let b = boxed((-hw, hw), (-hw, hw), (-z, z));
    for (what, r) in [
        ("∪", topo::union(&d, &b, Tol::witness())),
        ("∩", topo::intersect(&d, &b, Tol::witness())),
        ("∖", topo::subtract(&d, &b, Tol::witness())),
        ("∖ reversed", topo::subtract(&b, &d, Tol::witness())),
    ] {
        if let Ok(r) = r {
            panic!(
                "{what}: the corner bar came back {:?}",
                r.body().map(|x| (x.kind, volume(&x.body)))
            );
        }
    }
}
