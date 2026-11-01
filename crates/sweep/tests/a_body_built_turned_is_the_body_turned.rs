//! **A body built turned is the body turned** (D10 `[placement.frame]`:
//! a minted reference direction is a function of the inputs, not of the
//! axes). The same constructions are built on the world's three
//! coordinate planes — `xy`, `yz` and `zx`, which are the one sketch
//! carried by the two cyclic turns of the axes — and each turned body is
//! turned back. A signed permutation of the axes is exact in `f64`, and
//! every input here is a small dyadic, so the turn moves no bit of the
//! arithmetic: what comes back must be the `xy` body, every plane's
//! `u_ref` included, and its STEP text byte for byte, `ref_direction`s
//! included.
//!
//! The turns are the ones `Vec3::orthonormal_basis` is not covariant
//! under: it compares the normal's components against the axes, so a
//! plane whose reference it picked comes back turned by a quarter or a
//! half. Each construction reaches one minting site that once did:
//!
//! - a prism's caps (`swept::cap_plane`) and side walls
//!   (`extrude::side_surface`), each a `newell_plane`;
//! - the section faces of a one-segment circle's cylinder split across
//!   its axis, whose one-corner loops are charted by
//!   `section_loops::chord_u_ref`'s fallback;
//! - the union of two balls offset along the sketch normal, whose
//!   sphere groups the boolean re-charts about their centre line with
//!   seams turned to the pair's own (`boolean/ops.rs`, `pair_seam`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Surface;
use geom_core::{Affine3, Arc2, Mat3, Point2, Point3, RigidMotion, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use step_export::{StepOptions, step_string};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;
use topo::splitting::split;

/// A computing frame: the sketch plane the constructions are built on,
/// its frame's axes placed at any origin (`axes`), and the turn that
/// carries its bodies back to `xy`'s coordinates.
struct Frame {
    name: &'static str,
    plane: SketchPlane<f64>,
    axes: fn(Point3<f64>) -> RigidMotion<f64>,
    back: Affine3<f64>,
}

fn turn(cols: [[f64; 3]; 3]) -> Affine3<f64> {
    let [a, b, c] = cols.map(|[x, y, z]| Vec3::new(x, y, z));
    Affine3::from_parts(Mat3::from_cols(a, b, c), Vec3::zero())
}

/// `xy`, and the two cyclic turns: `zx` carries `(x, y, z)` to
/// `(y, z, x)`, `yz` to `(z, x, y)`; each `back` is its inverse.
fn frames() -> [Frame; 3] {
    [
        Frame {
            name: "xy",
            plane: SketchPlane::xy(),
            axes: RigidMotion::axes_xy,
            back: turn([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
        },
        Frame {
            name: "zx",
            plane: SketchPlane::zx(),
            axes: RigidMotion::axes_zx,
            back: turn([[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]),
        },
        Frame {
            name: "yz",
            plane: SketchPlane::yz(),
            axes: RigidMotion::axes_yz,
            back: turn([[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]),
        },
    ]
}

fn extruded(
    plane: SketchPlane<f64>,
    loops: Vec<ProfileLoop<f64>>,
    depth: f64,
    tol: Tol,
) -> Body<f64> {
    let profile = Profile::new(plane, loops).validate(tol).unwrap();
    let along = Extrusion::Distance {
        depth,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, along, tol).unwrap().body
}

/// A pentagon whose walls face along the axes and between them, extruded.
fn prism(plane: SketchPlane<f64>, tol: Tol) -> Body<f64> {
    let p = |x, y| (Point2::new(x, y), 0.0);
    let pentagon = bulge_loop(vec![
        p(0.0, 0.0),
        p(2.0, 0.0),
        p(3.0, 1.0),
        p(1.0, 2.0),
        p(0.0, 1.0),
    ]);
    extruded(plane, vec![pentagon], 1.5, tol)
}

/// The one-segment unit circle (its vertex at sketch `(1, 0)`) extruded
/// by 2 and split across its axis at height 1: each piece's section
/// face is bounded by a loop of one corner, where the plane crosses the
/// wall's seam.
fn split_cylinder(plane: SketchPlane<f64>, tol: Tol) -> [Body<f64>; 2] {
    let circle = RawLoop::new([(
        Point2::new(1.0, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep: TAU,
        }),
    )]);
    let body = extruded(plane, vec![circle], 2.0, tol);
    let body = topo::test_support::finished("the cylinder", body, tol);
    let normal = plane.normal().get();
    let cut = topo::test_support::split_plane(plane.origin() + normal, normal, tol);
    let r = split(&body, &cut, tol).expect("the cut answers");
    [r.below, r.above].map(|side| topo::Body::clone(side.body().expect("material on each side")))
}

/// Every construction, labelled, built on `plane`.
fn constructions(plane: SketchPlane<f64>, tol: Tol) -> Vec<(&'static str, Body<f64>)> {
    let [below, above] = split_cylinder(plane, tol);
    vec![
        ("the prism", prism(plane, tol)),
        ("the split cylinder's lower piece", below),
        ("the split cylinder's upper piece", above),
    ]
}

/// A ball of radius `r` about the sketch point `c` (`c[2]` along the
/// normal) in `frame`: a semicircle on the sketch plane through `c`,
/// revolved fully about the sketch's `y` direction through `c`.
fn ball(frame: &Frame, r: f64, c: [f64; 3], tol: Tol) -> topo::AtRestBody<f64> {
    let at = frame.plane.normal().get() * c[2];
    let plane = SketchPlane::from_frame((frame.axes)(Point3::origin() + at));
    let lp = bulge_loop(vec![
        (Point2::new(c[0], c[1] - r), 1.0),
        (Point2::new(c[0], c[1] + r), 0.0),
    ]);
    let profile = Profile::new(plane, vec![lp]).validate(tol).unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(c[0], c[1]),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = revolve(&profile, axis, Revolution::Full, tol).unwrap().body;
    topo::test_support::finished("the ball", body, tol)
}

/// A ball: its radius and its centre in sketch coordinates.
type Ball = (f64, [f64; 3]);

/// Ball pairs offset along the sketch normal, so their centre line
/// crosses both balls' polar axes: equal radii, unequal radii, and a
/// centre line through the origin.
const BALL_PAIRS: [(&str, Ball, Ball); 3] = [
    (
        "equal balls",
        (1.0, [0.5, 0.25, 0.0]),
        (1.0, [0.5, 0.25, 1.0]),
    ),
    (
        "unequal balls",
        (1.0, [0.5, 0.25, 0.0]),
        (0.75, [0.5, 0.25, 1.0]),
    ),
    (
        "balls on a line through the origin",
        (1.0, [0.0, 0.0, -0.5]),
        (1.0, [0.0, 0.0, 0.5]),
    ),
];

/// Every surface carrier, rendered by `{:?}` (shortest round-trip, so
/// equal text is equal bits, signed zeros included), in arena order.
fn surfaces(body: &Body<f64>) -> Vec<String> {
    body.surfaces().map(|(_, s)| format!("{s:?}")).collect()
}

/// Each plane carrier's `(origin, normal, u_ref)`, in arena order.
fn plane_frames(body: &Body<f64>) -> Vec<[[f64; 3]; 3]> {
    body.surfaces()
        .filter_map(|(_, s)| match s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Some([
                [origin.x, origin.y, origin.z],
                normal.to_array(),
                u_ref.to_array(),
            ]),
            _ => None,
        })
        .collect()
}

/// Bit identity, so a signed zero, which the STEP text writes, counts.
fn same_bits(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3]) -> bool {
    a.iter()
        .flatten()
        .zip(b.iter().flatten())
        .all(|(x, y)| x.to_bits() == y.to_bits())
}

#[test]
fn every_construction_built_on_a_turned_plane_is_the_construction_turned() {
    let tol = Tol::witness();
    let [home, turned @ ..] = frames();
    let at_home = constructions(home.plane, tol);
    for frame in turned {
        let back = geom_core::test_support::rigid(&frame.back, tol);
        for ((what, want), (_, built)) in at_home.iter().zip(constructions(frame.plane, tol)) {
            let got = topo::transform_rigid(&built, &back, tol).unwrap();
            let (want_planes, got_planes) = (plane_frames(want), plane_frames(&got));
            assert!(
                !want_planes.is_empty(),
                "{what}: the construction mints no plane, so the row reads nothing"
            );
            assert_eq!(
                got_planes.len(),
                want_planes.len(),
                "{what}: the plane count moved"
            );
            for (i, (g, w)) in got_planes.iter().zip(&want_planes).enumerate() {
                assert!(
                    same_bits(g, w),
                    "{what} built on {} and turned back: plane {i}'s (origin, normal, u_ref) \
                     moved with the computing axes\n  got  {g:?}\n  want {w:?}",
                    frame.name
                );
            }
            let options = StepOptions::default();
            let (got, want) = (
                step_string(
                    &topo::test_support::finished("the turned body", got, tol),
                    &options,
                    tol,
                )
                .unwrap(),
                step_string(
                    &topo::test_support::finished("the home body", want.clone(), tol),
                    &options,
                    tol,
                )
                .unwrap(),
            );
            if let Some((g, w)) = got.lines().zip(want.lines()).find(|(g, w)| g != w) {
                panic!(
                    "{what} built on {} and turned back: the STEP text differs at\n  got  {g}\n  want {w}",
                    frame.name
                );
            }
            assert_eq!(
                got.lines().count(),
                want.lines().count(),
                "{what} built on {} and turned back: the STEP text differs in length",
                frame.name
            );
        }
    }
}

#[test]
fn a_ball_union_built_on_a_turned_plane_is_the_union_turned() {
    let tol = Tol::witness();
    let [home, turned @ ..] = frames();
    let union = |frame: &Frame| -> Vec<Body<f64>> {
        BALL_PAIRS
            .iter()
            .map(|(what, (ra, ca), (rb, cb))| {
                let (a, b) = (ball(frame, *ra, *ca, tol), ball(frame, *rb, *cb, tol));
                match topo::union(&a, &b, tol) {
                    Ok(topo::BooleanResult::Body(bb)) => (*bb.body).clone(),
                    other => panic!("{what} on {}: the union is a body: {other:?}", frame.name),
                }
            })
            .collect()
    };
    let at_home = union(&home);
    for frame in turned {
        let back = geom_core::test_support::rigid(&frame.back, tol);
        for (((what, ..), want), built) in BALL_PAIRS.iter().zip(&at_home).zip(union(&frame)) {
            let got = topo::transform_rigid(&built, &back, tol).unwrap();
            let (want, got) = (surfaces(want), surfaces(&got));
            assert!(
                want.iter().any(|s| s.starts_with("Sphere")),
                "{what}: the union holds no sphere, so the row reads nothing"
            );
            assert_eq!(got.len(), want.len(), "{what}: the surface count moved");
            for (i, (g, w)) in got.iter().zip(&want).enumerate() {
                assert_eq!(
                    g, w,
                    "{what} united on {} and turned back: surface {i} moved with the \
                     computing axes",
                    frame.name
                );
            }
        }
    }
}
