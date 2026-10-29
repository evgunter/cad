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

use super::search_kit::band;
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
    answers_within(
        what,
        ring,
        wall,
        want,
        Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: super::section_cert_rows::LEVER / 2.0,
        },
    );
}

/// [`answers`], the pair classified within `reach`.
fn answers_within(what: &str, ring: &Ring, wall: &Wall, want: &[Class], reach: Reach<f64>) {
    let sec = classify(
        &ring.surface(),
        &wall.surface(),
        reach,
        Some(geom_brep::SsiLane::marched()),
        band(),
    );
    let Section::Components { parts, single } = &sec else {
        panic!("{what}: not classified: {sec:?}");
    };
    let mut got: Vec<Class> = parts.iter().map(class_of).collect();
    got.sort_unstable();
    let mut want = want.to_vec();
    want.sort_unstable();
    assert_eq!(got, want, "{what}: the classes");
    assert_eq!(*single, want.len() == 1, "{what}: single");
    let back = classify(
        &wall.surface(),
        &ring.surface(),
        reach,
        Some(geom_brep::SsiLane::marched()),
        band(),
    );
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

/// **Tangencies refuse R-tan, on the critical gap.** `K` tangent to the
/// strip's top or bottom edge over the parallel, and tangent to a tube
/// circle from outside or from inside: each a critical value of the
/// distance from the core to the wall's axis at the wall's radius off the
/// tube's. Mutant: answer a decided-`Zero` gap as either side, and the
/// row goes red.
#[test]
fn square_wall_tangencies_refuse() {
    let donut = ring(2.0, 0.5);
    for (what, k, name) in [
        (
            "tangent to the top edge",
            (1.0, 0.3, 0.2),
            "ct_critical_gap",
        ),
        (
            "tangent to the bottom edge",
            (1.0, -0.3, 0.2),
            "ct_critical_gap",
        ),
        (
            "touching T₊ from outside",
            (2.7, 0.0, 0.2),
            "ct_critical_gap",
        ),
        (
            "touching T₊ from inside",
            (2.3, 0.0, 0.2),
            "ct_critical_gap",
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
/// carrier; one tilted by `1e-3` is no pose an arm counts, and its
/// section is the march's. Mutant: decide the tilt unlevered, or read
/// any tilt as square, and a row goes red.
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
    assert!(
        marched(&donut.surface(), &tilted.surface()),
        "tilted by 1e-3"
    );
    assert!(
        marched(&tilted.surface(), &donut.surface()),
        "tilted by 1e-3, swapped"
    );
}

/// Whether the section value reads the pair's components from the march:
/// no closed form counts them.
fn marched(f: &geom::Surface<f64>, g: &geom::Surface<f64>) -> bool {
    match geom_brep::pair_section(f, g, band()) {
        Ok(geom_brep::SectionRead::Built(s)) => s.counted().is_none(),
        other => panic!("the value builds the pair: {other:?}"),
    }
}

/// **A crossing at a tube circle's top refuses R-order, not R-tan.** `K`
/// through `T₊`'s top point `(R, r)`, square to it there: the carriers
/// cross, but the fold stands on neither half of the circle and the strip
/// crossing beside it neither over the parallel nor beyond it, so how the
/// folds pair is not read. Mutant: refuse the fold's half R-tan, and the
/// row goes red.
#[test]
fn a_crossing_at_a_tubes_top_refuses_r_order() {
    let donut = ring(2.0, 0.5);
    let w = wall((2.3, 0.5, 0.3));
    for sec in [
        classified(&donut, &w),
        super::section_cert_rows::classify(&w.surface(), &donut.surface()),
    ] {
        assert!(
            matches!(
                sec,
                Section::Unordered("section_torus_square_wall_fold_half")
            ),
            "{sec:?}"
        );
    }
}

/// **`K` tangent to a strip edge beyond the parallel answers.** `K` about
/// `(2.6, 0.2)`, radius `0.3`, touches `z = r` at `y = 2.6 > R`, where no
/// line of the wall meets the torus: the section is the one of `K` held
/// just under the line, the one null loop where `K` bites the tube's
/// outer side. Mutant: refuse every strip-edge pinch R-tan, and the row
/// goes red.
#[test]
fn a_wall_touching_a_strip_edge_beyond_the_parallel_answers() {
    answers(
        "touching z = r beyond the parallel",
        &ring(2.0, 0.5),
        &wall((2.6, 0.2, 0.3)),
        &[NULL],
    );
}

/// **The square reading's lever is the torus's ball: a tilt just inside
/// the band answers, one just outside refuses on reach.** The section
/// value reads the tilt over the ball `|x| ≤ R + r` about the torus's
/// centre, which holds every section of it, whatever reach the
/// certificate hands it: the lever is `2·(|pivot| + R + r)`, the pivot
/// the wall axis's point nearest the centre. A tilt worth `0.6` of the
/// band's Zero there reads square and answers as the untilted wall does,
/// read over a reach of `50` or of `1`; one worth `1.5` of it is no pose
/// an arm counts, read from the march. Mutants: lever the tilt by half or a millionth of that
/// (the outside row answers), or by twice it (the inside row refuses).
#[test]
fn the_square_reading_is_levered_by_the_tori_ball() {
    let donut = ring(2.0, 0.5);
    let k = (0.15, 0.1, 0.2);
    // The untilted axis passes `(0, e, z0)` along `x`, its point nearest
    // the origin; a tilt of `1e-10` and less moves it by far less than
    // the band.
    let lever = 2.0 * (f64::hypot(k.0, k.1) + 2.5);
    let tilt = |of_zero: f64| of_zero * band().zero() / lever;
    for radius in [50.0, 1.0] {
        let reach = Reach {
            centre: p(0.0, 0.0, 0.0),
            radius,
        };
        answers_within(
            "a tilt 0.6 of the band's Zero",
            &donut,
            &wall_along(v(1.0, 0.0, tilt(0.6)), k),
            &[ON_WALL; 4],
            reach,
        );
        let outside = wall_along(v(1.0, 0.0, tilt(1.5)), k);
        assert!(
            marched(&donut.surface(), &outside.surface())
                && marched(&outside.surface(), &donut.surface()),
            "a tilt 1.5 of the band's Zero"
        );
    }
}

/// `a − b` as its rounded value and the rounding's error, exactly
/// (Knuth's two-sum).
fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let bv = s - a;
    (s, (a - (s - bv)) + (-b - bv))
}

/// **A pipe laid along the core's tangent: every fold the arm answers
/// lies within the band of its true fold, or the arm refuses R-tan.** A
/// torus about `z` at a centre well off the origin, `R ∈ {1, 30, 1000}`,
/// `r = 0.3 R`, and a wall along `x` of radius `r` whose trace `K` is
/// `T₊` moved by `δ` at `30°` in `Π`, `δ` from `3ε` to `0.1 r`: `K`
/// crosses `T₊` at the angle `δ/r`, and the rounding of `K`'s centre
/// (read from operands `~R` across) moves a fold along both circles by
/// that rounding over the angle. Each answered fold witness (the
/// witnesses in `Π`, at `x = c_x`) is measured against the stored pose
/// exactly: its residuals off `K` and off `T₊`, from error-free
/// differences of the stored coordinates, solved for the step to the
/// true crossing. Static, so it also holds that some fold answers and
/// some refuses for its drift. Mutant: drop the drift's refusal, and the
/// folds at `R = 1000` stand up to `~1e4` bands off.
#[test]
fn a_pipe_along_the_cores_tangent_places_its_folds_within_the_band() {
    let eps = band().zero();
    let (mut answered, mut drifting, mut worst) = (0usize, 0usize, 0.0f64);
    for big_r in [1.0, 30.0, 1000.0] {
        let r = 0.3 * big_r;
        let c = p(0.7003 * big_r, -0.4137 * big_r, 0.2501 * big_r);
        let ring = Ring {
            c,
            a: Vec3::unit_z(),
            b1: Vec3::unit_x(),
            big_r,
            r,
        };
        for delta in [3.0 * eps, 1e3 * eps, 1e-7 * r, 1e-5 * r, 1e-3 * r, 0.1 * r] {
            let (dy, dz) = (delta * 0.75f64.sqrt(), delta * 0.5);
            let o = p(c.x + 1.3 * big_r, c.y + big_r + dy, c.z + dz);
            let wall = Wall {
                o,
                d: Vec3::unit_x(),
                e1: Vec3::unit_y(),
                rc: r,
            };
            let what = format!("R {big_r}, δ {delta:e}");
            let parts = match super::section_cert_rows::classify(&ring.surface(), &wall.surface()) {
                Section::Tangent("section_torus_square_wall_fold_drift") => {
                    drifting += 1;
                    continue;
                }
                Section::Tangent(_) => continue,
                Section::Components { parts, .. } => parts,
                other => panic!("{what}: {other:?}"),
            };
            for w in parts.iter().filter_map(|c| c.witness) {
                if w.x != c.x {
                    continue;
                }
                // `W − K`'s centre and `W − T₊`'s, in `Π`'s `(y, z)`.
                let k = [two_diff(w.y, o.y), two_diff(w.z, o.z)].map(|(h, l)| h + l);
                let (ty, ty_err) = two_diff(w.y, c.y);
                let (tz, tz_err) = two_diff(w.z, c.z);
                assert!(
                    ty >= big_r / 2.0 && ty <= 2.0 * big_r,
                    "{what}: the fold {w:?} is on T₊"
                );
                let t = [(ty - big_r) + ty_err, tz + tz_err];
                let (nk, nt) = (k[0].hypot(k[1]), t[0].hypot(t[1]));
                let (res_k, res_t) = (nk - r, nt - r);
                let (uk, ut) = ([k[0] / nk, k[1] / nk], [t[0] / nt, t[1] / nt]);
                let det = uk[0] * ut[1] - uk[1] * ut[0];
                let step = (
                    (-res_k * ut[1] + res_t * uk[1]) / det,
                    (-uk[0] * res_t + ut[0] * res_k) / det,
                );
                let drift = step.0.hypot(step.1);
                worst = worst.max(drift / eps);
                answered += 1;
                assert!(
                    drift <= eps,
                    "{what}: the fold witness {w:?} stands {drift:e} from its true fold"
                );
            }
        }
    }
    println!("[pipe] {answered} folds answered, worst at {worst:.3} bands; {drifting} refused");
    assert!(
        answered > 0 && drifting > 0,
        "{answered} folds answered, {drifting} refused for their drift"
    );
}
