//! **The boolean's pair doors and the split's gate part what the census
//! parts.** The census parts two solids' faces along directions their
//! own geometry aims (an axis toward the other face's nearest boundary
//! point, an arc's chord, the line between two axes or centres) and
//! reads a sphere patch no further than its rim at a pole it does not
//! hold (`cross_solid_apart`). The operand gate, the NURBS extent test
//! and the split's carrier gate refuse a face kind only through a pair
//! or a plane it may meet, and read the same directions and the same
//! reach (`topo::separating::faces_apart`, `cover::face_cover_toward`).
//!
//! The partner the boolean does not serve is a box whose top cap wears a
//! plain NURBS chart (the extent test's kind) or a certified `Approx`
//! (off the operand gate's roster), set beside a rod's wall or between
//! the two lumps a bar leaves cut through a ball.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use geom_core::{Affine3, Point2, Point3, RigidMotion, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::approx::{box_with_approx_cap, box_with_bumped_cap};

fn tol() -> Tol {
    Tol::witness()
}

/// `body` turned by `angle` about `axis` through the origin, if
/// given, then moved by `by`.
fn moved(body: &Body<f64>, turn: Option<(Vec3<f64>, f64)>, by: Vec3<f64>) -> Body<f64> {
    let body = match turn {
        Some((axis, angle)) => topo::transform_rigid(
            body,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(Point3::origin(), axis, angle),
                tol(),
            ),
        )
        .unwrap(),
        None => body.clone(),
    };
    topo::transform_rigid(&body, &RigidMotion::translation(by).unwrap()).unwrap()
}

/// The box `[0, 2]² × [0, 1]` under a flat NURBS cap, and under a
/// certified `Approx` one.
fn caps() -> [(&'static str, Body<f64>); 2] {
    [
        ("the NURBS cap", box_with_bumped_cap(0.0).0),
        ("the Approx cap", box_with_approx_cap(0.05, 1e-9).0),
    ]
}

/// Whether `e` is the refusal the door for `name`'s kind raises on the
/// cap: the NURBS extent test, or the operand gate.
fn refused_on_the_cap(name: &str, e: &BooleanError) -> bool {
    match e {
        BooleanError::NurbsExtentUnsupported { .. } => name == "the NURBS cap",
        BooleanError::CurvedPairUnsupported {
            site: topo::PairRefusalSite::OperandGate,
            kind: geom::SurfaceKind::Approx,
            ..
        } => name == "the Approx cap",
        _ => false,
    }
}

/// The number of solids `a ∪ b` builds, or its refusal.
fn union_solids(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<usize, BooleanError> {
    match topo::union(a, b, tol())? {
        BooleanResult::Body(out) => Ok(out.body.solids().count()),
        _ => panic!("a union of solids is a body"),
    }
}

/// The two lumps the unit ball leaves of the bar
/// `[−2, 2] × [−0.2, 0.25] × [0.1, 0.4]`, each holding a patch of the
/// sphere whose carrier's reach is the whole ball's.
fn lumps() -> AtRestBody<f64> {
    let ball = finished(
        "the ball",
        sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
        tol(),
    );
    let bar = finished(
        "the bar",
        sweep::test_support::brick::<f64>((-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4), tol()),
        tol(),
    );
    let Ok(BooleanResult::Body(lumps)) = topo::subtract(&bar, &ball, tol()) else {
        panic!("bar ∖ ball builds");
    };
    lumps.body
}

/// **A spline cap beside a rod's wall unions until it touches.** The rod
/// `r ≤ 0.5` about `y` lies along the cap's edge `x = 0`, `z = 1`,
/// `gap` clear of it on the diagonal of `−x` and `+z`. No world axis
/// and no face normal of either operand parts the cap from the rod's
/// faces; the offset from the rod's axis to the cap's nearest edge does
/// (`depth_gap::toward`). At gap 0 the edge lies along a ruling of the
/// wall, and each door refuses the cap. Red where a pair door reads only
/// the anchors' axis and the operands' planar normals.
#[test]
fn a_spline_cap_beside_a_rods_wall_unions_until_it_touches() {
    let rod = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    for (name, cap) in caps() {
        let cap = finished(name, cap, tol());
        for gap in [0.04, 1e-3, 0.0] {
            let off = (0.5 + gap) * FRAC_1_SQRT_2;
            let rod = finished(
                "the rod",
                moved(&rod, None, Vec3::new(-off, 0.5, 1.0 + off)),
                tol(),
            );
            let built = union_solids(&cap, &rod);
            if gap > 0.0 {
                assert!(
                    matches!(built, Ok(2)),
                    "{name} with the rod {gap} clear: {built:?}"
                );
            } else {
                let e = built.expect_err("the cap's edge lies on the wall");
                assert!(refused_on_the_cap(name, &e), "{name} touching: {e:?}");
            }
        }
    }
}

/// **The lumps of a bar cut through a ball union with a spline cap
/// between them.** The box ([`caps`]) stands on its side between the
/// [`lumps`], its cap facing `+x` at `x = 0.5` and at `x = 0.83`, short
/// of the `+x` patch's rim at `x ≈ 0.88`. Along `x` the patch holds
/// neither pole, so it reaches no further than its rim
/// (`cover::face_cover_toward`), and the NURBS cap parts from it: the
/// union is the three solids. The `Approx` cap passes the operand gate
/// the same way; the sphere extent scan then refuses it, since the ball
/// the patches leave holds it. Red where a pair door reads a sphere face
/// by its window alone.
#[test]
fn the_lumps_of_a_bar_through_a_ball_union_with_a_spline_cap_between_them() {
    let lumps = lumps();
    for (name, cap) in caps() {
        for x in [0.5, 0.83] {
            // `(x, y, z) ↦ (z, y, −x)` stands the cap at `x = 1`.
            let b = finished(
                name,
                moved(
                    &cap,
                    Some((Vec3::unit_y(), FRAC_PI_2)),
                    Vec3::new(x - 1.0, -1.0, 1.25),
                ),
                tol(),
            );
            let built = union_solids(&lumps, &b);
            let what = format!("{name} at x = {x}");
            if name == "the NURBS cap" {
                assert!(matches!(built, Ok(3)), "{what}: {built:?}");
            } else {
                let e = built.expect_err(&what);
                assert!(
                    matches!(
                        e,
                        BooleanError::CurvedBooleanUnsupported {
                            kind: geom::SurfaceKind::Approx,
                            ..
                        }
                    ),
                    "{what}: past the operand gate, the extent scan's ball refuses: {e:?}"
                );
            }
        }
    }
}

/// **A split between the lumps clears their sphere patches.** The
/// [`lumps`] split by the plane `x = c`: at `c = 0.5` and `c = 0` the
/// plane meets neither lump, and each patch reaches no further along `x`
/// than its rim (`x ≈ ±0.88`), since it holds neither pole. At
/// `c = 0.92` the plane cuts the `+x` patch, and the split's gate
/// refuses the sphere. Red where the gate reads a sphere face by its
/// window alone.
#[test]
fn a_split_between_the_lumps_clears_their_sphere_patches() {
    let lumps = lumps();
    for c in [0.5, 0.0, 0.92] {
        let plane = topo::test_support::split_plane(
            Point3::new(c, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            tol(),
        );
        let split = topo::split(&lumps, &plane, tol());
        if c < 0.9 {
            assert!(split.is_ok(), "the plane x = {c}: {:?}", split.err());
        } else {
            assert!(
                matches!(
                    split,
                    Err(topo::SplitError::Reduce(
                        topo::SplitReduceError::CurvedBooleanUnsupported {
                            kind: geom::SurfaceKind::Sphere,
                            ..
                        }
                    ))
                ),
                "the plane x = {c} cuts the patch: {:?}",
                split.err()
            );
        }
    }
}
