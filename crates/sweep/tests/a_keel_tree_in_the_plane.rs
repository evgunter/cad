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
use crate::common::keel_tree::{branching, deep, grooved};
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::Body;
use topo::splitting::plane_section;

fn tol() -> Tol {
    Tol::witness()
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
    splits_sound_with_one_square(
        "two grooves end to end",
        &grooved(deep::<f64>(tol()), tol()),
    );
}

#[test]
fn a_branching_keel_tree_in_the_plane_splits_sound_and_sections_with_no_spur() {
    splits_sound_with_one_square(
        "three grooves at one vertex",
        &grooved(branching::<f64>(tol()), tol()),
    );
}
