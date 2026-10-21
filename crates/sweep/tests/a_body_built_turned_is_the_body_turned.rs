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
//!   `section_loops::chord_u_ref`'s fallback.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Surface;
use geom_core::{Affine3, Arc2, Mat3, Point2, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use step_export::{StepOptions, step_string};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;
use topo::splitting::split;

/// A computing frame: the sketch plane the constructions are built on,
/// and the turn that carries its bodies back to `xy`'s coordinates.
struct Frame {
    name: &'static str,
    plane: SketchPlane<f64>,
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
            back: turn([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
        },
        Frame {
            name: "zx",
            plane: SketchPlane::zx(),
            back: turn([[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]),
        },
        Frame {
            name: "yz",
            plane: SketchPlane::yz(),
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
    [r.below, r.above].map(|side| (*side.body().expect("material on each side")).clone())
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
                step_string(&got, &options, tol).unwrap(),
                step_string(want, &options, tol).unwrap(),
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
