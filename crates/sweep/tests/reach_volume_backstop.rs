//! **The boolean's volume backstop measures curved trims.** The
//! backstop compares the result's volume with its operands' and so
//! must measure all three bodies. These rows are booleans whose bodies
//! carry a cylinder wall trimmed by an ellipse — a boss on a tilted
//! sketch plane, an oblique cut through a rod — which only the certified
//! quadrature encloses: each builds, certifies at rest, and has its
//! analytic volume. A wall the property layer cannot measure at all
//! refuses `VolumeUnmeasured`, carrying the property layer's own
//! refusal. Wrong results planted on quadrature-measured bodies refuse
//! down to the resolution the backstop states, and a dual runs no
//! backstop at all (DL3).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::three_arc;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, extruded, sketch_from_axes};
use topo::{AtRestOutcome, Body, BooleanError, BooleanErrorKind, BooleanOp, MassPropsError};

fn tol() -> Tol {
    Tol::witness()
}

/// The sketch plane through `o`, turned `theta` about the world `x`.
fn tilted_about_x(o: Point3<f64>, theta: f64) -> SketchPlane<f64> {
    sketch_from_axes(
        o,
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, theta.cos(), theta.sin()),
        tol(),
    )
}

fn body_of(r: Result<topo::BooleanResult<f64>, BooleanError>, what: &str) -> Body<f64> {
    r.unwrap_or_else(|e| panic!("{what}: the boolean refuses: {e:?}"))
        .body()
        .expect("a body remains")
        .body
        .clone()
}

/// The result certifies at rest and has `expect` for its volume.
fn assert_sound(r: Result<topo::BooleanResult<f64>, BooleanError>, expect: f64, what: &str) {
    let body = body_of(r, what);
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let v = topo::mass_properties(&body, tol()).unwrap().volume;
    assert!(
        (v - expect).abs() < 1e-9,
        "{what}: volume {v} against the analytic {expect}"
    );
}

const BOSS_R: f64 = 0.25;

/// The base, `[−1, 1]² × [0, 1]`.
fn base() -> Body<f64> {
    brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol())
}

/// A radius-[`BOSS_R`] cylinder of height 1 on the sketch plane through
/// `(0, 0, z0)` turned `theta` about `x`.
fn boss(theta: f64, z0: f64) -> Body<f64> {
    extruded(
        tilted_about_x(Point3::new(0.0, 0.0, z0), theta),
        vec![three_arc(Point2::new(0.0, 0.0), BOSS_R, 0.0)],
        1.0,
        tol(),
    )
}

/// `base ∪ boss(θ, 1)`: the half of the cap below `z = 1` sinks into
/// the base by the ungula `⅔·r³·tan θ`.
fn standing_union(theta: f64) -> f64 {
    let r = BOSS_R;
    4.0 + PI * r * r - 2.0 / 3.0 * r * r * r * theta.tan()
}

/// `base ∪ boss(θ, 0.5)`. In the boss's own frame (`a` along `x`, `b`
/// across the tilt, `t` up its axis) a point of the cap disc stays in
/// the base up to `t = min(1, (0.5 − b·sin θ) / cos θ)`. Over the whole
/// disc the unclipped length integrates to `π·r²·0.5 / cos θ`; past
/// `b* = (0.5 − cos θ) / sin θ` the top cap itself dips under `z = 1`,
/// and the segment `b < b*` (distance `d = −b*` from the centre, area
/// `r²·acos(d/r) − d·√(r² − d²)`, first moment `−⅔·(r² − b*²)^{3/2}`)
/// gives back what the clip removes.
fn penetrating_union(theta: f64) -> f64 {
    let r = BOSS_R;
    let (s, c) = theta.sin_cos();
    let mut inside = PI * r * r * 0.5 / c;
    let b_star = (0.5 - c) / s;
    if b_star > -r {
        let d = -b_star;
        let segment = r * r * (d / r).acos() - d * (r * r - d * d).sqrt();
        inside -=
            (0.5 / c - 1.0) * segment + 2.0 / 3.0 * (s / c) * (r * r - b_star * b_star).powf(1.5);
    }
    4.0 + PI * r * r - inside
}

/// **A boss on a definitely tilted sketch plane, unioned onto a box**,
/// standing on the top face and penetrating it; its wall meets the top
/// face along an ellipse. At 0.8 and 1.0 rad the penetrating boss's
/// top cap dips under the top face too.
#[test]
fn a_tilted_boss_unions_onto_a_box() {
    for theta in [4e-3f64, 0.2] {
        assert_sound(
            topo::union(&base(), &boss(theta, 1.0), tol()),
            standing_union(theta),
            &format!("boss tilted {theta} rad, standing"),
        );
    }
    for theta in [4e-3f64, 0.2, 0.8, 1.0] {
        assert_sound(
            topo::union(&base(), &boss(theta, 0.5), tol()),
            penetrating_union(theta),
            &format!("boss tilted {theta} rad, penetrating"),
        );
    }
}

/// **The order-dependent gap.** At 0.5 rad the penetrating boss builds
/// as `boss ∪ base` but not as `base ∪ boss`: there the quadrature's own
/// convergence test on one wall face lands inside its band and
/// escalates, so the backstop cannot measure the result
/// (`work/quad/quadrature-convergence-test-escalates-instead-of-refining`).
/// Pinned at that outcome; the refusal says what ran out, and offers no
/// coincidence to declare.
#[test]
fn a_boss_at_half_a_radian_measures_in_one_operand_order() {
    let theta = 0.5f64;
    assert_sound(
        topo::union(&boss(theta, 0.5), &base(), tol()),
        penetrating_union(theta),
        "boss ∪ base at 0.5 rad",
    );
    let err = topo::union(&base(), &boss(theta, 0.5), tol()).expect_err("the known gap");
    let BooleanError::VolumeUnmeasured {
        operand: None,
        source:
            MassPropsError::Face {
                source: geom_brep::props::PropsError::Escalated { cause },
                ..
            },
    } = &err
    else {
        panic!("base ∪ boss at 0.5 rad: {err:?}");
    };
    assert_eq!(cause.predicate, Some("props_quad_converged"), "{err:?}");
    let text = err.to_string();
    assert!(
        text.contains("converged") && !text.contains("declare"),
        "the refusal names the quadrature's convergence and no coincidence: {text}"
    );
}

/// The rod: `r = 0.5` about the vertical through `(x0, 0)`, over
/// `z ∈ [0, 4]`, an extruded circle.
fn rod(x0: f64, r: f64) -> Body<f64> {
    let disc = profile::circle(Point2::new(x0, 0.0), r, tol()).unwrap();
    extruded(SketchPlane::xy(), vec![disc.into()], 4.0, tol())
}

/// A box extruded 2 from the square `x0 + [−1, 1] × [−1, 1]` on the
/// sketch plane through `(x0, 0, 3.5)` turned 20° about `x`.
fn oblique_cutter(x0: f64) -> Body<f64> {
    let square = bulge_loop(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    extruded(
        tilted_about_x(Point3::new(x0, 0.0, 3.5), 20f64.to_radians()),
        vec![square],
        2.0,
        tol(),
    )
}

/// The rod of radius `r` about `(x0, 0)` cut obliquely at `z = 3.5`.
fn oblique_rod(x0: f64, r: f64) -> Body<f64> {
    body_of(
        topo::subtract(&rod(x0, r), &oblique_cutter(x0), tol()),
        "the oblique rod",
    )
}

/// **An untilted rod minus an obliquely tilted box.** The box removes
/// everything of the rod above the plane through its axis at
/// `z = 3.5`, trimming the wall by an ellipse, so `3.5·π·r²` is left.
#[test]
fn a_rod_cut_by_an_oblique_box_keeps_its_volume() {
    assert_sound(
        topo::subtract(&rod(0.0, 0.5), &oblique_cutter(0.0), tol()),
        3.5 * PI * 0.25,
        "the rod cut obliquely at z = 3.5",
    );
}

/// **A wall the property layer has no measurement for.** An extruded
/// half-disk (the `x = 0` line from `(0, 4)` to the origin, back along
/// the radius-2 arc about `(0, 2)`, height 1) minus the box
/// `[1.5, 2.5]² × [0.5, 2.5]`: the box notches the curved wall from
/// its top rim, along two rulings and an arc, and a notched wall is
/// neither an iso-rectangle (the closed form) nor conic-trimmed (the
/// quadrature). The backstop cannot measure the result, so it refuses
/// naming the result and carrying the face's own refusal
/// (`work/props/a-notched-cylinder-wall-has-no-volume-measurement`).
#[test]
fn a_notched_wall_refuses_as_unmeasured() {
    let half = bulge_loop(vec![
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 0.0), 1.0),
    ]);
    let half_disk: Body<f64> = extruded(SketchPlane::xy(), vec![half], 1.0, tol());
    let notch = brick((1.5, 2.5), (1.5, 2.5), (0.5, 2.5), tol());
    let err = topo::subtract(&half_disk, &notch, tol()).expect_err("the notched wall refuses");
    assert!(
        matches!(
            err,
            BooleanError::VolumeUnmeasured {
                operand: None,
                source: MassPropsError::Face {
                    source: geom_brep::props::PropsError::NotIsoRectangle { .. },
                    ..
                },
            }
        ),
        "{err:?}"
    );
}

/// The backstop's verdict on a planted result.
fn planted(
    op: BooleanOp,
    a: &Body<f64>,
    b: &Body<f64>,
    result: &Body<f64>,
) -> Result<AtRestOutcome, BooleanErrorKind> {
    topo::test_support::volume_backstop(op, a, b, result, tol()).map_err(|e| e.kind())
}

/// **Wrong results planted on quadrature-measured bodies.** `A` is the
/// oblique rod, whose reporting enclosure is ±9.2e-7 m³ wide; a
/// subtraction's "result" that is the same rod fattened by a relative
/// `δ` is too big by `≈ 5.5·δ` m³. Above the resolution the backstop
/// states (about 1.5e-9 m³ on this pair, the quadrature's interval
/// floor at its last shared round) each refuses — by refinement past
/// the reporting target where the reporting enclosures alone could not
/// decide — and below it the fattening is accepted.
#[test]
fn planted_subtraction_results_refuse_down_to_the_stated_resolution() {
    let a = oblique_rod(0.0, 0.5);
    let b = oblique_cutter(0.0);
    assert_eq!(
        planted(BooleanOp::Subtract, &a, &b, &a),
        Ok(AtRestOutcome::Validated),
        "the right answer passes"
    );
    for delta in [1e-3f64, 1e-7, 1e-9] {
        assert_eq!(
            planted(
                BooleanOp::Subtract,
                &a,
                &b,
                &oblique_rod(0.0, 0.5 * (1.0 + delta))
            ),
            Err(BooleanErrorKind::ResultVolumeImplausible),
            "a result fattened by δ = {delta:e} refuses"
        );
    }
    assert_eq!(
        planted(
            BooleanOp::Subtract,
            &a,
            &b,
            &oblique_rod(0.0, 0.5 * (1.0 + 1e-11))
        ),
        Ok(AtRestOutcome::Validated),
        "a fattening of ≈ 5.5e-11 m³ is below the stated resolution"
    );
}

/// The union side: `U = base ∪ boss(0.2, 0.5)` against a union with a
/// boss of radius `0.2499999`, short by ≈ 7.7e-8 m³ — below the two
/// reporting enclosures' summed half-width (≈ 2.4e-7 m³), so it refuses
/// only through refinement.
#[test]
fn a_planted_union_result_short_by_less_than_the_reporting_pads_refuses() {
    let u = body_of(topo::union(&base(), &boss(0.2, 0.5), tol()), "U");
    let short_boss = extruded(
        tilted_about_x(Point3::new(0.0, 0.0, 0.5), 0.2),
        vec![three_arc(Point2::new(0.0, 0.0), 0.2499999, 0.0)],
        1.0,
        tol(),
    );
    let short = body_of(topo::union(&base(), &short_boss, tol()), "the short union");
    assert_eq!(
        planted(BooleanOp::Union, &u, &base(), &short),
        Err(BooleanErrorKind::ResultVolumeImplausible)
    );
}

/// **The enclosure arithmetic's direction.** The rod cut at +20° and
/// its mirror cut at −20° have one true volume, `3.5·π·r²`, and their
/// quadrature midpoints differ: the mirror's lands 3.1e-9 m³ lower,
/// inside both enclosures (±9.2e-7 and ±3.1e-6). Planting the +20° rod
/// as the result of subtracting from the mirror puts the margin's
/// midpoint below zero by that much while the true margin is zero: the
/// backstop must pass it. A gate that read the midpoint, or subtracted
/// the enclosures' width instead of adding it, refuses here.
#[test]
fn a_result_equal_in_truth_but_larger_at_the_midpoint_passes() {
    let mirror_cutter = extruded(
        tilted_about_x(Point3::new(0.0, 0.0, 3.5), (-20f64).to_radians()),
        vec![bulge_loop(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .into_iter()
                .map(|(x, y)| (Point2::new(x, y), 0.0))
                .collect(),
        )],
        2.0,
        tol(),
    );
    let mirror = body_of(
        topo::subtract(&rod(0.0, 0.5), &mirror_cutter, tol()),
        "the mirror rod",
    );
    let a = oblique_rod(0.0, 0.5);
    let (vm, va) = (
        topo::mass_properties(&mirror, tol()).unwrap().volume,
        topo::mass_properties(&a, tol()).unwrap().volume,
    );
    assert!(
        va > vm,
        "the fixture's premise: the +20° rod's midpoint {va} is above the mirror's {vm}"
    );
    assert_eq!(
        planted(BooleanOp::Subtract, &mirror, &mirror_cutter, &a),
        Ok(AtRestOutcome::Validated)
    );
}

/// **A dual runs no backstop** (DL3): the backstop is a validation, and
/// the base-scalar run of the same recipe is the check of record. At
/// `Dual64` the tilted boss's union builds, and its value channel is
/// the `f64` run's, bit for bit.
#[test]
fn a_dual_builds_the_tilted_boss_on_the_f64_bits() {
    use geom_core::{Dual, Dual64};
    let c = Dual::constant;
    let theta = 0.2f64;
    let base_d = brick::<Dual64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
    let plane = sketch_from_axes(
        Point3::new(c(0.0), c(0.0), c(1.0)),
        Vec3::new(c(1.0), c(0.0), c(0.0)),
        Vec3::new(c(0.0), c(theta.cos()), c(theta.sin())),
        tol(),
    );
    let boss_d = extruded(
        plane,
        vec![three_arc(Point2::new(0.0, 0.0), BOSS_R, 0.0).map_scalar(c)],
        c(1.0),
        tol(),
    );
    let dual = topo::union(&base_d, &boss_d, tol()).expect("the dual builds");
    let dual = &dual.body().expect("a body").body;
    let real = body_of(
        topo::union(&base(), &boss(theta, 1.0), tol()),
        "the f64 run",
    );
    let bits = |p: [f64; 3]| p.map(f64::to_bits);
    let d: Vec<_> = dual
        .points()
        .map(|(_, p)| bits([p.x.value, p.y.value, p.z.value]))
        .collect();
    let f: Vec<_> = real.points().map(|(_, p)| bits([p.x, p.y, p.z])).collect();
    assert!(!f.is_empty());
    assert_eq!(d, f, "the dual's points are the f64 run's, in arena order");
}
