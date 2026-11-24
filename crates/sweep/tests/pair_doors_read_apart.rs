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

/// The volume of `body`.
fn volume(what: &str, body: &AtRestBody<f64>) -> f64 {
    topo::test_support::pinned_volume(what, body, tol()).volume
}

/// What `a ∪ b` builds, its solids and its volume, or its refusal.
fn union_of(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<(usize, f64), BooleanError> {
    match topo::union(a, b, tol())? {
        BooleanResult::Body(out) => Ok((out.body.solids().count(), volume("the union", &out.body))),
        _ => panic!("a union of solids is a body"),
    }
}

/// Whether `built` is `solids` solids of the volume `expected`, the two
/// operands side by side.
fn side_by_side(built: &Result<(usize, f64), BooleanError>, solids: usize, expected: f64) -> bool {
    matches!(*built, Ok((n, v)) if n == solids && (v - expected).abs() <= 1e-9 * expected)
}

/// The rod `r ≤ 0.5` about `y` over `y ∈ [0, 1]`.
fn rod() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// The rod moved along the cap's edge `x = 0`, `z = 1`, `gap` clear of
/// it on the diagonal of `−x` and `+z` (inside it where negative).
fn rod_by_the_edge(gap: f64) -> AtRestBody<f64> {
    let off = (0.5 + gap) * FRAC_1_SQRT_2;
    finished(
        "the rod",
        moved(&rod(), None, Vec3::new(-off, 0.5, 1.0 + off)),
        tol(),
    )
}

/// The cap's box stood on its side, its cap facing `+x` at `x`.
fn stood_at(name: &str, cap: &Body<f64>, x: f64) -> AtRestBody<f64> {
    // `(x, y, z) ↦ (z, y, −x)` stands the cap at `x = 1`.
    finished(
        name,
        moved(
            cap,
            Some((Vec3::unit_y(), FRAC_PI_2)),
            Vec3::new(x - 1.0, -1.0, 1.25),
        ),
        tol(),
    )
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
/// lies along the cap's edge `x = 0`, `z = 1`, `gap` clear of it on the
/// diagonal of `−x` and `+z` ([`rod_by_the_edge`]). No world axis and no
/// face normal of either operand parts the cap from the rod's faces;
/// the offset from the rod's axis to the cap's nearest edge does
/// (`depth_gap::toward`), and the union is the box and the rod side by
/// side, `4 + π/4`. At gap 0 the edge lies along a ruling of the wall,
/// and 1e-3 inside it the rod overlaps the cap: each door refuses the
/// Approx cap, and the overlap never builds as two solids. Red where a
/// pair door reads only the anchors' axis and the operands' planar
/// normals, and where it parts a pair that meets.
#[test]
fn a_spline_cap_beside_a_rods_wall_unions_until_it_touches() {
    let apart = 4.0 + core::f64::consts::FRAC_PI_4;
    for (name, cap) in caps() {
        let cap = finished(name, cap, tol());
        for gap in [0.04, 1e-3] {
            let built = union_of(&cap, &rod_by_the_edge(gap));
            assert!(
                side_by_side(&built, 2, apart),
                "{name} with the rod {gap} clear: {built:?}"
            );
        }
        let e = union_of(&cap, &rod_by_the_edge(0.0)).expect_err("the cap's edge lies on the wall");
        assert!(refused_on_the_cap(name, &e), "{name} touching: {e:?}");
    }
    let (name, cap) = caps().into_iter().nth(1).unwrap();
    let e = union_of(&finished(name, cap, tol()), &rod_by_the_edge(-1e-3))
        .expect_err("the rod overlaps the cap");
    assert!(refused_on_the_cap(name, &e), "{name} overlapped: {e:?}");
}

/// **The lumps of a bar cut through a ball union with a spline cap
/// between them.** The box ([`caps`]) stands on its side between the
/// [`lumps`], its cap facing `+x` at `x = 0.5` and at `x = 0.83`, short
/// of the `+x` patch's rim at `x ≈ 0.88`. Along `x` the patch holds
/// neither pole, so it reaches no further than its rim
/// (`cover::face_cover_toward`), and the NURBS cap parts from it: the
/// union is the three solids, the lumps' volume and the box's 4. The
/// `Approx` cap at `x = 0.92` cuts the patch, and the operand gate
/// refuses it there. Short of the rim, it passes the gate as the NURBS
/// cap does;
/// the sphere extent scan then refuses it, since the ball the patches
/// leave holds it
/// (`work/orbit/sphere-extent-scan-refuses-an-approx-face-the-ball-holds-but-no-patch-reaches.md`).
/// Red where a pair door reads a sphere face by its window alone, and
/// where it parts a cap that cuts the patch.
#[test]
fn the_lumps_of_a_bar_through_a_ball_union_with_a_spline_cap_between_them() {
    let lumps = lumps();
    let apart = volume("the lumps", &lumps) + 4.0;
    for (name, cap) in caps() {
        for x in [0.5, 0.83] {
            let built = union_of(&lumps, &stood_at(name, &cap, x));
            let what = format!("{name} at x = {x}");
            if name == "the NURBS cap" {
                assert!(side_by_side(&built, 3, apart), "{what}: {built:?}");
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
                    "{what}: past the operand gate, the extent scan's ball refuses until \
                     sphere-extent-scan-refuses-an-approx-face-the-ball-holds-but-no-patch-reaches \
                     is fixed: {e:?}"
                );
            }
        }
    }
    let (name, cap) = caps().into_iter().nth(1).unwrap();
    let e = union_of(&lumps, &stood_at(name, &cap, 0.92)).expect_err("the cap cuts the patch");
    assert!(
        refused_on_the_cap(name, &e),
        "{name} cutting the patch: {e:?}"
    );
}

/// Whether `split` is the split gate's refusal of a sphere face.
fn refuses_the_sphere<T>(split: &Result<T, topo::SplitError>) -> bool {
    matches!(
        split,
        Err(topo::SplitError::Reduce(
            topo::SplitReduceError::CurvedBooleanUnsupported {
                kind: geom::SurfaceKind::Sphere,
                ..
            }
        ))
    )
}

/// The plane through `c·n̂`, normal to `n`.
fn plane_at(c: f64, n: Vec3<f64>) -> topo::SplitPlane<f64> {
    let n = n.normalize();
    topo::test_support::split_plane(Point3::origin() + n * c, n, tol())
}

/// **A split between the lumps clears their sphere patches, and one
/// through a patch's held pole does not.** The [`lumps`] split by the
/// plane `x = c`: at `c = 0.5` and `c = 0` the plane meets neither lump,
/// and each patch reaches no further along `x` than its rim
/// (`x ≈ ±0.88`), since it holds neither pole. At `c = 0.92` the plane
/// cuts the `+x` patch, and the split's gate refuses the sphere. The
/// unit ball poled along `z` is one sphere face bounded by its seam
/// meridian in `y = 0`, which holds both poles `±n̂` of `n ∝ (0.3, 1, 0)`
/// inside it. Cut by the planes `n̂·p = ±0.5`, which cross the ball, the
/// gate refuses the sphere: the seam stays within 0.29 of the centre
/// along `n̂`, and the face's reach is the whole sphere's at both ends. Red
/// where the gate reads a sphere face by its window alone, and where it
/// cuts a face to its boundary at an end whose pole the face holds.
#[test]
fn a_split_between_the_lumps_clears_their_sphere_patches() {
    let lumps = lumps();
    for c in [0.5, 0.0] {
        let split = topo::split(&lumps, &plane_at(c, Vec3::unit_x()), tol());
        assert!(split.is_ok(), "the plane x = {c}: {:?}", split.err());
    }
    let split = topo::split(&lumps, &plane_at(0.92, Vec3::unit_x()), tol());
    assert!(
        refuses_the_sphere(&split),
        "the plane x = 0.92 cuts the patch: {:?}",
        split.err()
    );
    let ball = finished(
        "the ball",
        sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
        tol(),
    );
    for c in [-0.5, 0.5] {
        let split = topo::split(&ball, &plane_at(c, Vec3::new(0.3, 1.0, 0.0)), tol());
        assert!(
            refuses_the_sphere(&split),
            "the plane at {c} cuts the ball, whose face holds the pole: {:?}",
            split.err()
        );
    }
}
