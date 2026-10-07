//! **A loft's v-parameters are a function of its section SET**: every
//! spelling of the same sections — which vertex each loop is written
//! from, which sense it runs, which way its placement is rolled by one
//! of its own symmetries — builds bit-identical parameters and walls.
//! (The cap planes are fitted in the authored vertex order, so they
//! can differ by ulps:
//! `work/carve/a-loft-caps-plane-is-summed-in-the-authored-vertex-order.md`.)
//!
//! Each fixture is chosen so that the strips of one section set carry
//! DIFFERENT chord shares (a flared or off-centre middle section), so a
//! parameterization read off any one strip moves with the spelling and
//! these rows go red.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::NurbsSurface;
use geom_core::{Affine3, Mat3, Point2, Tol, Vec3};
use profile::{RawLoop, test_support::bulge_loop};
use sweep::skin::{LoftGeometry, Section, loft_geometry, loft_parameters};
use sweep::test_support::stacked_at;
use sweep::{Lofted, ProfileLoop, loft_body};

use crate::common::quad;

/// `pts` written from vertex `start` — the same loop, spelled from a
/// different corner.
fn spelled_from(pts: [(f64, f64); 4], start: usize) -> Section {
    quad(core::array::from_fn(|i| pts[(i + start) % 4]))
}

/// `pts` as one polygon loop written from vertex `start`, and in the
/// opposite sense when `reversed`.
fn poly(pts: &[(f64, f64)], start: usize, reversed: bool) -> ProfileLoop<f64> {
    let n = pts.len();
    let mut order: Vec<(f64, f64)> = (0..n).map(|i| pts[(i + start) % n]).collect();
    if reversed {
        order[1..].reverse();
    }
    ProfileLoop::polygon(order.into_iter().map(|(x, y)| Point2::new(x, y)))
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

/// Asserts that `walls` is `base` cyclically shifted, bit for bit, at
/// SOME shift — the spelling decides which wall comes first, nothing
/// else.
fn assert_cyclic_shift(
    base: &[std::sync::Arc<NurbsSurface<f64>>],
    walls: &[std::sync::Arc<NurbsSurface<f64>>],
    what: &str,
) {
    let n = base.len();
    assert_eq!(walls.len(), n, "{what}: wall count");
    assert!(
        (0..n).any(|shift| (0..n).all(|j| same_bits(&walls[j], &base[(j + shift) % n]))),
        "{what}: the walls must be the base spelling's walls, cyclically shifted, bit for bit"
    );
}

/// The per-section mean is a function of the multiset of rows, bit for
/// bit: an irregular seven-gon, every section scaled and nudged
/// differently and placed off-axis, so the rows' shares all differ and
/// a mean summed in arrival order rounds differently for each start
/// vertex. Red without the per-section sort.
#[test]
fn parameters_are_order_free_bit_for_bit() {
    let base: Vec<(f64, f64)> = (0..7)
        .map(|i| {
            let th = f64::from(i) * TAU / 7.0;
            let r = 1.0 + 0.137 * (f64::from(i) * 1.7).sin();
            (r * th.cos(), r * th.sin())
        })
        .collect();
    let zs = [0.0, 0.31, 0.77, 1.9, 2.05, 3.3];
    #[allow(clippy::cast_precision_loss)]
    let section = |k: usize, start| {
        let s = 1.0 + 0.21 * (k as f64 * 1.3).sin();
        let pts: Vec<(f64, f64)> = base
            .iter()
            .enumerate()
            .map(|(i, (x, y))| {
                (
                    x * s + 0.05 * (k * i) as f64 % 0.13,
                    y * s * 0.03f64.mul_add(k as f64, 1.0),
                )
            })
            .collect();
        vec![poly(&pts, start, false)]
    };
    #[allow(clippy::cast_precision_loss)]
    let places: Vec<Affine3<f64>> = zs
        .iter()
        .enumerate()
        .map(|(k, z)| Affine3::translation(Vec3::new(0.09 * k as f64, -0.04 * (k * k) as f64, *z)))
        .collect();
    let p = |start| {
        loft_parameters(
            &(0..6).map(|k| section(k, start)).collect::<Vec<_>>(),
            &places,
            3,
            Tol::witness(),
        )
        .expect("the seven-gons parameterize")
    };
    let p0 = p(0);
    for start in 1..7 {
        let q = p(start);
        assert!(
            q.iter().zip(&p0).all(|(a, b)| a.to_bits() == b.to_bits()),
            "written from vertex {start}: {q:?} against {p0:?}"
        );
    }
}

/// An outer loop and two off-centre holes, the middle section's outer
/// flared and its holes moved, every loop re-spelled independently
/// (start vertex and sense): the parameters are bit-identical and each
/// loop's walls are the base spelling's, cyclically shifted. The
/// parameters are the outer loop's, so reshaping a hole leaves them —
/// and every outer wall — exactly where they were.
#[test]
fn a_loft_with_holes_is_the_same_under_every_spelling_and_its_holes_do_not_move_v() {
    let rect = |cx: f64, cy: f64, h: f64| {
        [
            (cx - h, cy - h),
            (cx + h, cy - h),
            (cx + h, cy + h),
            (cx - h, cy + h),
        ]
    };
    let outer = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)];
    let flared = [(-2.5, -2.0), (2.5, -2.0), (2.0, 2.0), (-2.0, 2.0)];
    let holes = |k: usize, b: f64| match k {
        1 => (rect(-0.6, 0.5, 0.35), rect(1.0, -0.7, b)),
        _ => (rect(-0.8, 0.3, 0.3), rect(0.9, -0.5, 0.25)),
    };
    let places = stacked_at(&[0.0, 1.0, 3.0]);
    let sections = |start: usize, reversed: bool, b: f64| -> Vec<Section> {
        (0..3)
            .map(|k| {
                let (a, h) = holes(k, b);
                vec![
                    poly(if k == 1 { &flared } else { &outer }, start, reversed),
                    poly(&a, (start + 1) % 4, !reversed),
                    poly(&h, (start + 2) % 4, reversed),
                ]
            })
            .collect()
    };
    let base = build(&sections(0, false, 0.25), &places);
    for start in 0..4 {
        for reversed in [false, true] {
            let what = format!("loops written from {start} (reversed {reversed})");
            let other = build(&sections(start, reversed, 0.25), &places);
            assert_eq!(
                other.1.section_params, base.1.section_params,
                "{what}: the parameters must not move with the spelling"
            );
            for (l, walls) in other.0.walls.iter().enumerate() {
                assert_cyclic_shift(&base.0.walls[l], walls, &format!("{what}, loop {l}"));
            }
        }
    }
    let reshaped = build(&sections(0, false, 0.4), &places);
    assert_eq!(
        reshaped.1.section_params, base.1.section_params,
        "a hole is interpolated at the outer loop's parameters and has no say in them"
    );
    for (j, wall) in reshaped.0.walls[0].iter().enumerate() {
        assert!(
            same_bits(wall, &base.0.walls[0][j]),
            "reshaping a hole moved outer wall {j}"
        );
    }
}

/// A square whose +x edge is an arc, the bulge different in every
/// section (a quarter turn, 200° and 120°), written from each corner in
/// both senses: the parameters are bit-identical and the walls are the
/// base spelling's, cyclically shifted.
#[test]
fn an_arc_walled_loft_is_the_same_under_every_spelling() {
    let bulges = [
        (PI / 8.0).tan(),
        50f64.to_radians().tan(),
        30f64.to_radians().tan(),
    ];
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let places = stacked_at(&[0.0, 0.8, 2.0]);
    // Segment `i` runs from corner `i` to corner `i + 1`; the arc is
    // segment 1. Reversed, segment `i` is walked from corner `i + 1`
    // with its bulge negated.
    let section = |b: f64, start: usize, reversed: bool| -> Section {
        let bulge = |i: usize| if i == 1 { b } else { 0.0 };
        let chain: Vec<(Point2<f64>, f64)> = (0..4)
            .map(|k| {
                let p = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
                if reversed {
                    let i = (start + 4 - k) % 4;
                    (p(i + 1), -bulge(i))
                } else {
                    let i = (start + k) % 4;
                    (p(i), bulge(i))
                }
            })
            .collect();
        vec![bulge_loop(chain)]
    };
    // The geometry only: this body refuses at assembly under any
    // parameterization (`work/carve/a-rational-wall-beside-an-integral-one-skins-its-shared-corner-off-unit-weight.md`).
    let spelled = |start, reversed| {
        let sections = bulges.map(|b| section(b, start, reversed));
        let params = loft_parameters(&sections, &places, 2, Tol::witness())
            .expect("the arc sections parameterize");
        loft_geometry(&sections, &places, 2, &params, Tol::witness())
            .expect("the arc sections skin")
    };
    let base = spelled(0, false);
    for start in 0..4 {
        for reversed in [false, true] {
            let what = format!("written from corner {start} (reversed {reversed})");
            let other = spelled(start, reversed);
            assert_eq!(
                other.section_params, base.section_params,
                "{what}: the parameters must not move with the spelling"
            );
            assert_cyclic_shift(&base.walls[0], &other.walls[0], &what);
        }
    }
}
