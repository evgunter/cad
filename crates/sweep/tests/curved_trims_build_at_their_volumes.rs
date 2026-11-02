//! **Booleans of curved trims build at their volumes.** These rows are
//! booleans whose bodies carry a cylinder wall trimmed by an ellipse — a
//! boss on a tilted sketch plane, an oblique cut through a rod — which
//! only the certified quadrature encloses: each builds, certifies at
//! rest, and has its analytic volume. The volume relations
//! (`topo::test_support::volume_relations`) flag wrong results planted
//! on these quadrature-measured bodies and pass the right one, and a
//! dual builds on the `f64` bits.
//!
//! Every size a row plants or checks is spelled in the run's ε, and a
//! row whose verdict changes with ε says where and why.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::three_arc;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, extruded, finished, sketch_from_axes};
use topo::test_support::volume_relations;
use topo::{AtRestBody, BooleanError, BooleanOp};

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

fn body_of(r: Result<topo::BooleanResult<f64>, BooleanError>, what: &str) -> AtRestBody<f64> {
    r.unwrap_or_else(|e| panic!("{what}: the boolean refuses: {e:?}"))
        .body()
        .expect("a body remains")
        .body
        .clone()
}

/// The result certifies at rest and has `expect` for its volume: the
/// analytic volume lies in the enclosure the property layer certifies,
/// and the enclosure's midpoint is within the model's resolution of
/// it, ε over the body's surface area.
fn assert_sound(r: Result<topo::BooleanResult<f64>, BooleanError>, expect: f64, what: &str) {
    let body = body_of(r, what);
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let p =
        topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol())).unwrap();
    let miss = (p.enclosure.volume - expect).abs();
    assert!(
        miss <= p.enclosure.volume_pad,
        "{what}: the analytic {expect} is outside the enclosure {} ± {:e}",
        p.enclosure.volume,
        p.enclosure.volume_pad
    );
    assert!(
        miss <= tol().eps() * p.enclosure.surface_area,
        "{what}: the midpoint {} misses the analytic {expect} by more than ε over the area {}",
        p.enclosure.volume,
        p.enclosure.surface_area
    );
}

const BOSS_R: f64 = 0.25;

/// The smallest tilt the rows call definite. Tilting the boss sags its
/// cap by `r·(1 − cos θ)` across the radius, which is the size the
/// union's coincidence test reads at the rim, so the sag is spelled in
/// ε: `2000·ε`, about 4e-3 rad at the default ε.
fn definite_tilt() -> f64 {
    (1.0 - 2000.0 * tol().eps() / BOSS_R).acos()
}

/// The base, `[−1, 1]² × [0, 1]`.
fn base() -> AtRestBody<f64> {
    finished(
        "the base",
        brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    )
}

/// A radius-[`BOSS_R`] cylinder of height 1 on the sketch plane through
/// `(0, 0, z0)` turned `theta` about `x`.
fn boss(theta: f64, z0: f64) -> AtRestBody<f64> {
    let boss = extruded(
        tilted_about_x(Point3::new(0.0, 0.0, z0), theta),
        vec![three_arc(Point2::new(0.0, 0.0), BOSS_R, 0.0)],
        1.0,
        tol(),
    );
    finished("the boss", boss, tol())
}

/// `base ∪ boss(θ, 1)`: the half of the cap below `z = 1` sinks into
/// the base by the ungula `⅔·r³·tan θ`.
fn standing_union(theta: f64) -> f64 {
    let r = BOSS_R;
    4.0 + PI * r * r - 2.0 / 3.0 * r * r * r * theta.tan()
}

/// `base ∪ boss(θ, 0.5)` for a boss of radius `r`. In the boss's own frame (`a` along `x`, `b`
/// across the tilt, `t` up its axis) a point of the cap disc stays in
/// the base up to `t = min(1, (0.5 − b·sin θ) / cos θ)`. Over the whole
/// disc the unclipped length integrates to `π·r²·0.5 / cos θ`; past
/// `b* = (0.5 − cos θ) / sin θ` the top cap itself dips under `z = 1`,
/// and the segment `b < b*` (distance `d = −b*` from the centre, area
/// `r²·acos(d/r) − d·√(r² − d²)`, first moment `−⅔·(r² − b*²)^{3/2}`)
/// gives back what the clip removes.
fn penetrating_union(theta: f64, r: f64) -> f64 {
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
    for theta in [definite_tilt(), 0.2] {
        assert_sound(
            topo::union(&base(), &boss(theta, 1.0), tol()),
            standing_union(theta),
            &format!("boss tilted {theta} rad, standing"),
        );
    }
    for theta in [definite_tilt(), 0.2, 0.8, 1.0] {
        assert_sound(
            topo::union(&base(), &boss(theta, 0.5), tol()),
            penetrating_union(theta, BOSS_R),
            &format!("boss tilted {theta} rad, penetrating"),
        );
    }
}

/// **A boss at half a radian measures in either operand order.** The
/// penetrating boss at 0.5 rad is one solid as `boss ∪ base` and as
/// `base ∪ boss`, in a different arena order, and both measure at its
/// analytic volume.
#[test]
fn a_boss_at_half_a_radian_measures_in_either_operand_order() {
    let theta = 0.5f64;
    assert_sound(
        topo::union(&boss(theta, 0.5), &base(), tol()),
        penetrating_union(theta, BOSS_R),
        "boss ∪ base at 0.5 rad",
    );
    assert_sound(
        topo::union(&base(), &boss(theta, 0.5), tol()),
        penetrating_union(theta, BOSS_R),
        "base ∪ boss at 0.5 rad",
    );
}

/// The rod: `r = 0.5` about the vertical through `(x0, 0)`, over
/// `z ∈ [0, 4]`, an extruded circle.
fn rod(x0: f64, r: f64) -> AtRestBody<f64> {
    let disc = profile::circle(Point2::new(x0, 0.0), r, tol()).unwrap();
    finished(
        "the rod",
        extruded(SketchPlane::xy(), vec![disc.into()], 4.0, tol()),
        tol(),
    )
}

/// A box extruded 2 from the square `x0 + [−1, 1] × [−1, 1]` on the
/// sketch plane through `(x0, 0, 3.5)` turned 20° about `x`.
fn oblique_cutter(x0: f64) -> AtRestBody<f64> {
    let square = bulge_loop(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let cutter = extruded(
        tilted_about_x(Point3::new(x0, 0.0, 3.5), 20f64.to_radians()),
        vec![square],
        2.0,
        tol(),
    );
    finished("the oblique cutter", cutter, tol())
}

/// The rod of radius `r` about `(x0, 0)` cut obliquely at `z = 3.5`.
fn oblique_rod(x0: f64, r: f64) -> AtRestBody<f64> {
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

/// **A notched wall measures.** An extruded half-disk (the `x = 0`
/// line from `(0, 4)` to the origin, back along the radius-2 arc about
/// `(0, 2)`, height 1) minus the box `[1.5, 2.5]² × [0.5, 2.5]`: the box
/// notches the curved wall from its top rim, along two rulings and an
/// arc. The wall's chart Green form measures it in closed form, and the
/// result is the half-disk less the notch: its
/// floor, `∫ (√(4 − s²) − 1.5) ds` over `|s| ≤ ½`, times the half metre
/// it sinks.
#[test]
fn a_notched_wall_measures_in_closed_form() {
    let half = bulge_loop(vec![
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 0.0), 1.0),
    ]);
    let half_disk: AtRestBody<f64> = finished(
        "the half-disk",
        extruded(SketchPlane::xy(), vec![half], 1.0, tol()),
        tol(),
    );
    let notch = finished(
        "the notch",
        brick((1.5, 2.5), (1.5, 2.5), (0.5, 2.5), tol()),
        tol(),
    );
    let body = body_of(
        topo::subtract(&half_disk, &notch, tol()),
        "the notched half-disk",
    );
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("the notched half-disk does not certify at rest: {e:?}"));
    let p =
        topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol())).unwrap();
    let floor = 0.5 * 3.75f64.sqrt() + 4.0 * 0.25f64.asin() - 1.5;
    let expect = 2.0 * PI - 0.5 * floor;
    assert_eq!(p.enclosure.volume_pad, 0.0, "closed-form faces only");
    assert!(
        (p.enclosure.volume - expect).abs() <= 1e-12 * expect,
        "the notched half-disk: {} against the analytic {expect}",
        p.enclosure.volume
    );
}

/// **Wrong results planted on quadrature-measured bodies.** `A` is the
/// oblique rod; a subtraction's "result" that is the same rod fattened
/// by a relative `δ` is too big by `≈ 5.5·δ` m³, against the `≈ 37` m²
/// of the two operands' boundary the relations allow ε of displacement
/// over. Fattenings of 1e-3 and `100·ε` break `vol(A ∖ B) ≤ vol(A)`;
/// the right answer, and a fattening of `ε/100`, which a correct result
/// may carry, do not.
#[test]
fn planted_subtraction_results_break_the_volume_relations() {
    let eps = tol().eps();
    let a = oblique_rod(0.0, 0.5);
    let b = oblique_cutter(0.0);
    let fattened = |delta: f64| {
        volume_relations(
            BooleanOp::Subtract,
            &a,
            &b,
            Some(&oblique_rod(0.0, 0.5 * (1.0 + delta))),
            tol(),
        )
    };
    for delta in [1e-3f64, 100.0 * eps] {
        let got = fattened(delta);
        assert!(
            got.as_ref()
                .is_err_and(|why| why.contains("vol(A ∖ B) ≤ vol(A)")),
            "a result fattened by δ = {delta:e} breaks vol(A ∖ B) ≤ vol(A): {got:?}"
        );
    }
    let right = volume_relations(BooleanOp::Subtract, &a, &b, Some(&a), tol());
    assert!(right.is_ok(), "the right answer holds: {right:?}");
    let below = fattened(0.01 * eps);
    assert!(below.is_ok(), "a fattening by δ = ε/100 holds: {below:?}");
}

/// **A dual builds on the `f64` bits** (DL3): at `Dual64` the tilted
/// boss's union builds, and its value channel is the `f64` run's, bit
/// for bit.
#[test]
fn a_dual_builds_the_tilted_boss_on_the_f64_bits() {
    use geom_core::{Dual, Dual64};
    let c = Dual::constant;
    let theta = 0.2f64;
    let base_d = finished(
        "the dual base",
        brick::<Dual64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
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
    let boss_d = finished("the dual boss", boss_d, tol());
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
    let f: Vec<_> = real.points().map(|(_, p)| bits(p.to_array())).collect();
    assert!(!f.is_empty());
    assert_eq!(d, f, "the dual's points are the f64 run's, in arena order");
}
