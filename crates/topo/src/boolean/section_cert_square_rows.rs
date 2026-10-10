//! **The square-wall arm's rows**: a torus against a cylinder whose
//! axis stands square to the torus's — a radial hole through the tube,
//! and every other class that pose takes. Each row's answer is written
//! down from the geometry (its comment says where each component lies)
//! and checked against the traced section (`square_search`'s oracle) as
//! well.
//!
//! Every row names the mutant that turns it red.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::section_cert_rows::{p, v};
use super::square_search::{Class, Ring, Wall, class_of, mismatches, resolution, trace, window};
use super::*;
use geom_core::Vec3;

/// A torus about `z` at the origin.
fn ring(big_r: f64, r: f64) -> Ring {
    Ring {
        c: p(0.0, 0.0, 0.0),
        a: Vec3::unit_z(),
        b1: Vec3::unit_x(),
        big_r,
        r,
    }
}

/// The wall along `d` (square to `z`, or nearly) whose axis crosses the
/// meridian plane `x = 0` at `(y, z) = (e, z0)`, stored from `x = 1.3`.
fn wall_along(d: Vec3<f64>, (e, z0, rc): (f64, f64, f64)) -> Wall {
    let d = d.normalize();
    Wall {
        o: p(0.0, e, z0) + d * 1.3,
        d,
        e1: d.orthonormal_basis().0,
        rc,
    }
}

/// The wall along `x`.
fn wall(trace_k: (f64, f64, f64)) -> Wall {
    wall_along(Vec3::unit_x(), trace_k)
}

fn classified(ring: &Ring, wall: &Wall) -> Section<f64> {
    super::section_cert_rows::classify(&ring.surface(), &wall.surface())
}

/// The arm's classes against `want` (`(essential on the torus, on the
/// wall)` per component), its `single` against one component, the
/// swapped order against the same with the flags exchanged, and the
/// traced section against all of it.
fn answers(what: &str, ring: &Ring, wall: &Wall, want: &[Class]) {
    let sec = classified(ring, wall);
    let Section::Components { parts, single } = &sec else {
        panic!("{what}: not classified: {sec:?}");
    };
    let mut got: Vec<Class> = parts.iter().map(class_of).collect();
    got.sort_unstable();
    let mut want = want.to_vec();
    want.sort_unstable();
    assert_eq!(got, want, "{what}: the classes");
    assert_eq!(*single, want.len() == 1, "{what}: single");
    let back = super::section_cert_rows::classify(&wall.surface(), &ring.surface());
    assert!(
        super::square_search::swapped_agrees(&sec, &back),
        "{what}: the swapped order: {back:?}"
    );
    // The search's resolution, capped: a row's strip spans enough cells
    // at 2000 about the wall.
    let traced = trace(
        ring,
        wall,
        window(ring, wall),
        resolution(ring, wall).min(2000),
        401,
    )
    .expect("the window holds the torus");
    let bad = mismatches(ring, wall, &sec, Some(&traced));
    assert!(bad.is_empty(), "{what}: {bad:#?}");
}

const NULL: Class = (false, false);
const ON_WALL: Class = (false, true);
const ON_TORUS: Class = (true, false);

/// **Radial holes through the tube, refused on reach before this arm,
/// now answered** (the donut, `R = 2`, `r = 0.5`, a wall along `x`).
///
/// - Through both walls, on the axis or off it: the wall's trace `K`
///   stands between the tube circles, inside the strip, so both sheets
///   lie over all of `K`: four loops, one where the hole leaves each
///   wall of the tube on each side, each about the hole.
/// - Biting the inner side (`K` across `T₊`'s inner half): one loop
///   through the two folds, where the bite is, and the outer sheet whole
///   over `K`, the two loops where the wall leaves through the outer
///   side far along.
/// - Biting the outer side (`K` across `T₊`'s outer half): the one loop
///   of the bite, null on both, single.
///
/// Mutants: drop one root of a tube crossing (`square_folds`), and every
/// bite row loses or garbles its fold loop; flag the lifts null on the
/// wall, and the through rows go red.
#[test]
fn radial_holes_through_a_tube_answer() {
    for (what, ring, k, want) in radial_rows() {
        answers(what, &ring, &wall(k), &want);
    }
}

/// A row: what it is, the torus, the wall's trace `(e, z0, rc)`, and the
/// classes of its components.
type Row = (&'static str, Ring, (f64, f64, f64), Vec<Class>);

fn radial_rows() -> Vec<Row> {
    let donut = ring(2.0, 0.5);
    [
        (
            "through both walls, on the axis",
            (0.0, 0.0, 0.2),
            &[ON_WALL; 4][..],
        ),
        (
            "through both walls, as wide as the tube allows",
            (0.0, 0.0, 0.45),
            &[ON_WALL; 4],
        ),
        (
            "through both walls, off the axis",
            (0.15, 0.1, 0.2),
            &[ON_WALL; 4],
        ),
        (
            "biting the inner side",
            (1.5, 0.0, 0.1),
            &[NULL, ON_WALL, ON_WALL],
        ),
        (
            "biting the inner side, off centre",
            (1.5, 0.2, 0.1),
            &[NULL, ON_WALL, ON_WALL],
        ),
        ("biting the outer side", (2.5, 0.0, 0.1), &[NULL]),
        (
            "biting the outer side, off centre",
            (2.5, -0.2, 0.1),
            &[NULL],
        ),
        ("along the far tube", (-2.0, 0.0, 0.2), &[ON_WALL; 2]),
    ]
    .into_iter()
    .map(|(what, k, want)| (what, donut, k, want.to_vec()))
    .collect()
}

/// **Every other class the pose takes.**
///
/// - A hole wider than the tube, through the hole of the torus
///   (`r < rc < R − r`): `K` crosses the strip on both edges, standing
///   over the parallel, so each of its flanks carries an oval from one
///   edge to the other: the tube pierces the wall four times, four loops
///   about the tube.
/// - A fat ring (`R = 1`, `r = 0.8`) and a wall through its hole, inside
///   the strip and across both tube circles' inner halves: the outer
///   sheet whole (two loops about the hole), and the inner sheet's two
///   runs between the circles, each from `T₊` to `T₋`, each around the
///   torus axis.
/// - The same ring, the wall raised across the top edge: one oval, its
///   inner-sheet runs again from `T₊` to `T₋`, around the axis.
/// - A wall so wide it is nearly the plane under the hole: two curves
///   around the axis, on the outer and the inner equator's sides.
/// - A wall through the hole cutting the tube's top: one oval per side,
///   null.
/// - The wall above the torus, and the wall beside it: nothing.
///
/// Mutant: flag a run from `T₊` to `T₋` null on the torus, and the fat
/// ring rows go red.
#[test]
fn every_class_of_a_square_wall_answers() {
    for (what, ring, k, want) in class_rows() {
        answers(what, &ring, &wall(k), &want);
    }
}

fn class_rows() -> Vec<Row> {
    let donut = ring(2.0, 0.5);
    let fat = ring(1.0, 0.8);
    [
        (
            "wider than the tube",
            &donut,
            (0.0, 0.0, 0.7),
            &[ON_TORUS; 4][..],
        ),
        (
            "wider still, through the hole",
            &donut,
            (0.0, 0.0, 1.2),
            &[ON_TORUS; 4],
        ),
        (
            "through a fat ring's hole",
            &fat,
            (0.0, 0.0, 0.5),
            &[ON_TORUS, ON_TORUS, ON_WALL, ON_WALL],
        ),
        (
            "raised across a fat ring's top",
            &fat,
            (0.0, 0.3, 0.6),
            &[ON_TORUS; 2],
        ),
        (
            "nearly the plane under the hole",
            &donut,
            (0.0, -10.0, 10.0),
            &[ON_TORUS; 2],
        ),
        (
            "cutting the tube's top",
            &donut,
            (0.0, 0.3, 0.4),
            &[NULL; 2],
        ),
        (
            "biting both outer sides at once",
            &donut,
            (0.0, 0.0, 2.3),
            &[NULL; 2],
        ),
        ("above the torus", &donut, (0.0, 3.0, 0.4), &[]),
        ("beside the torus", &donut, (3.0, 0.0, 0.2), &[]),
    ]
    .into_iter()
    .map(|(what, ring, k, want)| (what, *ring, k, want.to_vec()))
    .collect()
}

/// **Not a gate: writes the rows' poses for the mpmath oracle**, as
/// `square_search::dump_for_the_mpmath_oracle` does the search's.
#[test]
#[ignore = "writes the mpmath oracle's input; run by hand"]
fn dump_the_rows_for_the_mpmath_oracle() {
    for (what, ring, k, _) in radial_rows().into_iter().chain(class_rows()) {
        println!(
            "SQDUMP {}",
            super::square_search::dump_line(what, &ring, &wall(k))
        );
    }
}

/// **Tangencies refuse R-tan, naming the margin.** `K` tangent to the
/// strip's top or bottom edge, tangent to a tube circle from outside or
/// from inside, and through a tube circle's top point (where the fold
/// stands on neither half). Mutant: answer a decided-`Zero` margin as
/// either side, and the row goes red.
#[test]
fn square_wall_tangencies_refuse() {
    let donut = ring(2.0, 0.5);
    for (what, k, name) in [
        (
            "tangent to the top edge",
            (1.0, 0.3, 0.2),
            "section_torus_square_wall_top",
        ),
        (
            "tangent to the bottom edge",
            (1.0, -0.3, 0.2),
            "section_torus_square_wall_bottom",
        ),
        (
            "touching T₊ from outside",
            (2.7, 0.0, 0.2),
            "section_torus_square_wall_tube",
        ),
        (
            "touching T₊ from inside",
            (2.3, 0.0, 0.2),
            "section_torus_square_wall_tube",
        ),
        (
            "through T₊'s top point",
            (2.3, 0.5, 0.3),
            "section_torus_square_wall_fold_half",
        ),
    ] {
        let w = wall(k);
        for sec in [
            classified(&donut, &w),
            super::section_cert_rows::classify(&w.surface(), &donut.surface()),
        ] {
            assert!(
                matches!(sec, Section::Tangent(n) if n == name),
                "{what}: {sec:?}"
            );
        }
    }
}

/// **The square reading is the band's.** A wall tilted off square by
/// `1e-15` classifies as the square one does, witnesses on the tilted
/// carrier; one tilted by `1e-3` is no pose an arm claims and refuses on
/// reach. Mutant: decide the tilt unlevered, or read any tilt as square,
/// and a row goes red.
#[test]
fn a_square_wall_is_read_within_the_band_only() {
    let donut = ring(2.0, 0.5);
    let k = (0.15, 0.1, 0.2);
    answers(
        "tilted by 1e-15",
        &donut,
        &wall_along(v(1.0, 0.0, 1e-15), k),
        &[ON_WALL; 4],
    );
    let tilted = wall_along(v(1.0, 0.0, 1e-3), k);
    assert!(matches!(classified(&donut, &tilted), Section::Intractable));
    assert!(matches!(
        super::section_cert_rows::classify(&tilted.surface(), &donut.surface()),
        Section::Intractable
    ));
}
