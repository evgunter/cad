//! **A loft's v-parameters are a function of its section SET**: every
//! spelling of the same sections — which vertex each loop is written
//! from, which way its placement is rolled by one of its own
//! symmetries — builds the bit-identical body.
//!
//! Each fixture is chosen so that the strips of one section set carry
//! DIFFERENT chord shares (a flared or off-centre middle section), so a
//! parameterization read off any one strip moves with the spelling and
//! these rows go red.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_core::{Affine3, Mat3, Tol, Vec3};
use sweep::skin::{LoftGeometry, Section, loft_geometry, loft_parameters};
use sweep::test_support::stacked_at;
use sweep::{Lofted, loft_body};

use crate::common::quad;

/// `pts` written from vertex `start` — the same loop, spelled from a
/// different corner.
fn spelled_from(pts: [(f64, f64); 4], start: usize) -> Section {
    quad(core::array::from_fn(|i| pts[(i + start) % 4]))
}

fn same_bits(a: &NurbsSurface<f64>, b: &NurbsSurface<f64>) -> bool {
    a.knots_u().knots() == b.knots_u().knots()
        && a.knots_v().knots() == b.knots_v().knots()
        && a.weights() == b.weights()
        && a.control().len() == b.control().len()
        && a.control()
            .iter()
            .zip(b.control())
            .all(|(p, q)| p.x == q.x && p.y == q.y && p.z == q.z)
}

/// The loft's walls at its own parameters, and its body.
fn build(sections: &[Section], places: &[Affine3<f64>]) -> (LoftGeometry, Lofted<f64>) {
    let tol = Tol::witness();
    let params = loft_parameters(sections, places, 2, tol).expect("the sections parameterize");
    let geometry = loft_geometry(sections, places, 2, &params, tol).expect("the sections skin");
    let lofted = loft_body::<f64>(sections, places, 2, tol).expect("the loft builds");
    assert_eq!(
        lofted.section_params, params,
        "the body's read-back is the parameters `loft_parameters` answered"
    );
    (geometry, lofted)
}

/// Asserts that `other` is `base` with every loop's walls shifted by
/// `shift` — wall `j` of `other` is wall `j + shift` of `base`, bit for
/// bit — and that the two bodies read back the same parameters.
fn assert_same_loft(
    base: &(LoftGeometry, Lofted<f64>),
    other: &(LoftGeometry, Lofted<f64>),
    shift: usize,
    what: &str,
) {
    assert_eq!(
        other.1.section_params, base.1.section_params,
        "{what}: the section parameters must not move with the spelling"
    );
    for (l, (walls, base_walls)) in other.0.walls.iter().zip(&base.0.walls).enumerate() {
        let n = base_walls.len();
        for (j, wall) in walls.iter().enumerate() {
            assert!(
                same_bits(wall, &base_walls[(j + shift) % n]),
                "{what}: wall [{l}][{j}] must be wall [{l}][{}] of the base spelling, bit \
                 for bit",
                (j + shift) % n
            );
        }
    }
}

/// The corpus's non-uniform prism (square, flared trapezoid, square at
/// z = 0, 1, 3), every section written from each of its four corners.
/// The flare moves only the two corners of the first edge, so the four
/// strips carry three different chord shares and a parameterization
/// read off one strip would follow the start corner.
#[test]
fn a_loft_spelled_from_each_corner_builds_the_same_body() {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let flared = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let places = stacked_at(&[0.0, 1.0, 3.0]);
    let spelled = |start| {
        build(
            &[
                spelled_from(square, start),
                spelled_from(flared, start),
                spelled_from(square, start),
            ],
            &places,
        )
    };
    let base = spelled(0);
    for start in 1..4 {
        assert_same_loft(
            &base,
            &spelled(start),
            start,
            &format!("written from corner {start}"),
        );
    }
}

/// A placement rolled by a quarter turn about its own normal — a
/// symmetry of the square it carries, so every section's ring is the
/// same set of world points, only its start corner moved. The quarter
/// turn is the exact column permutation, so the world points are the
/// same BITS too and the row can ask for bit-identity. The middle
/// section is larger and off-centre, so its strips' shares differ.
#[test]
fn a_loft_whose_sections_are_rolled_by_their_own_symmetry_builds_the_same_body() {
    let square = |h: f64| quad([(-h, -h), (h, -h), (h, h), (-h, h)]);
    let sections = [square(1.0), square(1.6), square(1.0)];
    let centres = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.5, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 3.0),
    ];
    let quarter = Mat3::from_cols(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let placed = |rolled: bool| -> Vec<Affine3<f64>> {
        centres
            .iter()
            .map(|c| {
                let linear = if rolled { quarter } else { Mat3::identity() };
                Affine3::from_parts(linear, *c)
            })
            .collect()
    };
    let base = build(&sections, &placed(false));
    // The rolled frame carries sketch vertex `j` to the world point the
    // unrolled frame carries vertex `j + 1` to, so rolled wall `j` is
    // unrolled wall `j + 1`.
    assert_same_loft(
        &base,
        &build(&sections, &placed(true)),
        1,
        "every section rolled a quarter turn",
    );
}
