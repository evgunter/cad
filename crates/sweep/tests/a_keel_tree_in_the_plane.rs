//! **A keel that is a tree of straight edges in the plane splits sound
//! and sections with no spur.** A block `0..4 × 0..4 × 0..2` less
//! V-grooves whose floors lie in the plane `z = 1`, each a prism of a V
//! section with its tip on that plane. The first groove opens through
//! the block's face `x = 0` and runs to `x = 2`; from there a groove of a
//! wider V runs on to `x = 3` (a **deep** tree: two edges end to end), and,
//! in the branching form, a third runs off along `y` to `y = 2.5` (a
//! **branching** tree: three edges at one vertex). Split by the plane
//! either way up, the material on the grooves' side joins round every
//! closed end, so the side holding the floors traces them as a tree and
//! the other as its slit: each half keeps its volume, and `plane_section`
//! reads the block's square whole, the tree's open end its fifth corner.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::halves_at_rest;
use geom_core::{Point2, Point3, RigidMotion, Tol, Vec3};
use profile::SketchPlane;
use sweep::test_support::{block, finished, prism_on};
use topo::splitting::plane_section;
use topo::{Body, BooleanResult, subtract};

fn tol() -> Tol {
    Tol::witness()
}

/// A V of half-width `w` at `z = 2.5`, its tip on `z = 1` at `tip` in the
/// sketch's first coordinate, run `len` along the sketch normal from `o`.
fn groove(frame: RigidMotion<f64>, tip: f64, w: f64, len: f64) -> Body<f64> {
    let p = |u: f64, v: f64| (Point2::new(u, v), 0.0);
    prism_on(
        SketchPlane::from_frame(frame),
        vec![p(tip - w, 2.5), p(tip, 1.0), p(tip + w, 2.5)],
        len,
        tol(),
    )
}

/// The block less each groove in turn.
fn grooved(grooves: Vec<Body<f64>>) -> Body<f64> {
    let mut body = block::<f64>(4.0, 4.0, 2.0, tol());
    for (i, g) in grooves.into_iter().enumerate() {
        body = match subtract(
            &finished("the block", body, tol()),
            &finished("a groove", g, tol()),
            tol(),
        ) {
            Ok(BooleanResult::Body(b)) => b.body.into_body(),
            other => panic!("the block less groove {i}: {other:?}"),
        };
    }
    body
}

/// Along `x`: the first groove from outside the block to `x = 2`, and a
/// wider one on to `x = 3`, both centred on `y = 1`.
fn deep() -> Vec<Body<f64>> {
    vec![
        groove(
            RigidMotion::axes_yz(Point3::new(-1.0, 0.0, 0.0)),
            1.0,
            0.3,
            3.0,
        ),
        groove(
            RigidMotion::axes_yz(Point3::new(2.0, 0.0, 0.0)),
            1.0,
            0.5,
            1.0,
        ),
    ]
}

/// [`deep`], and a groove along `y` from `y = 1` to `y = 2.5` at `x = 2`.
fn branching() -> Vec<Body<f64>> {
    let mut grooves = deep();
    // The zx frame's first coordinate is z; sketch in it with x as the
    // second, run along y.
    let p = |u: f64, v: f64| (Point2::new(u, v), 0.0);
    grooves.push(prism_on(
        SketchPlane::from_frame(RigidMotion::axes_zx(Point3::new(0.0, 1.0, 0.0))),
        vec![p(2.5, 1.6), p(1.0, 2.0), p(2.5, 2.4)],
        1.5,
        tol(),
    ));
    grooves
}

fn splits_sound_with_one_square(what: &str, body: &Body<f64>) {
    let whole = topo::test_support::pinned_volume(
        "the measured body",
        &finished("body", body.clone(), tol()),
        tol(),
    )
    .volume;
    for up in [1.0, -1.0] {
        let what = format!("{what}, the plane's normal {up} z");
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, up),
            tol(),
        );
        let halves = halves_at_rest(&what, body, &plane);
        let sum: f64 = halves
            .iter()
            .map(|h| {
                topo::test_support::pinned_volume(
                    "the measured body",
                    &finished("the measured body", h.clone(), tol()),
                    tol(),
                )
                .volume
            })
            .sum();
        assert!(
            (sum - whole).abs() < 1e-9 * whole,
            "{what}: the halves hold {sum}, the body {whole}"
        );
        let cut = plane_section(&finished("the body", body.clone(), tol()), &plane, tol())
            .unwrap_or_else(|e| panic!("{what}: the section refused: {e:?}"));
        let [region] = cut.regions.as_slice() else {
            panic!("{what}: {} regions, not one", cut.regions.len());
        };
        assert!(
            (region.area() - 16.0).abs() < 1e-9,
            "{what}: section area {}, the square's 16",
            region.area()
        );
        let points = region.outline.points();
        let near = tol().eps() * tol().k();
        let repeated = (0..points.len())
            .any(|i| (i + 1..points.len()).any(|j| points[i].distance(points[j]) <= near));
        // The square's four, and the tree's open end on `x = 0`.
        assert!(
            points.len() == 5 && !repeated,
            "{what}: the outline {points:?}, not 5 distinct corners"
        );
    }
}

#[test]
fn a_deep_keel_tree_in_the_plane_splits_sound_and_sections_with_no_spur() {
    splits_sound_with_one_square("two grooves end to end", &grooved(deep()));
}

#[test]
fn a_branching_keel_tree_in_the_plane_splits_sound_and_sections_with_no_spur() {
    splits_sound_with_one_square("three grooves at one vertex", &grooved(branching()));
}
