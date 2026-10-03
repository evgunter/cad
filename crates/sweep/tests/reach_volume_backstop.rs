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
//!
//! Every size a row plants or checks is spelled in the run's ε, and a
//! row whose verdict changes with ε says where and why.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::three_arc;
use geom_core::{Point2, Point3, Tol, Vec3, tolerance::DEFAULT_EPS};
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

/// The result certifies at rest and has `expect` for its volume: the
/// analytic volume lies in the enclosure the property layer certifies,
/// and the enclosure's midpoint is within the model's resolution of
/// it, ε over the body's surface area.
fn assert_sound(r: Result<topo::BooleanResult<f64>, BooleanError>, expect: f64, what: &str) {
    let body = body_of(r, what);
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let p = topo::mass_properties(&body, tol()).unwrap();
    let miss = (p.volume - expect).abs();
    assert!(
        miss <= p.volume_pad,
        "{what}: the analytic {expect} is outside the enclosure {} ± {:e}",
        p.volume,
        p.volume_pad
    );
    assert!(
        miss <= tol().eps() * p.surface_area,
        "{what}: the midpoint {} misses the analytic {expect} by more than ε over the area {}",
        p.volume,
        p.surface_area
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

/// **The order-dependent gap.** At 0.5 rad and the default ε the
/// penetrating boss builds as `boss ∪ base` but not as `base ∪ boss`:
/// there the quadrature's own convergence test on one wall face lands
/// inside its band and escalates, so the backstop cannot measure the
/// result
/// (`work/quad/quadrature-convergence-test-escalates-instead-of-refining`).
/// Pinned at that outcome; the refusal says what ran out, and offers no
/// coincidence to declare. Where that margin lands is a matter of ε:
/// at 1e-6 and 1e-12 it lands outside the band and both orders measure.
#[test]
fn a_boss_at_half_a_radian_measures_in_one_operand_order() {
    let theta = 0.5f64;
    assert_sound(
        topo::union(&boss(theta, 0.5), &base(), tol()),
        penetrating_union(theta, BOSS_R),
        "boss ∪ base at 0.5 rad",
    );
    let base_first = topo::union(&base(), &boss(theta, 0.5), tol());
    if tol().eps() != DEFAULT_EPS {
        assert_sound(
            base_first,
            penetrating_union(theta, BOSS_R),
            "base ∪ boss at 0.5 rad off the default ε",
        );
        return;
    }
    let err = base_first.expect_err("the known gap");
    let BooleanError::VolumeUnmeasured {
        operand: None,
        source:
            MassPropsError::Face {
                source: geom_brep::props::PropsError::Escalated { cause, .. },
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

/// **A notched wall measures.** An extruded half-disk (the `x = 0`
/// line from `(0, 4)` to the origin, back along the radius-2 arc about
/// `(0, 2)`, height 1) minus the box `[1.5, 2.5]² × [0.5, 2.5]`: the box
/// notches the curved wall from its top rim, along two rulings and an
/// arc. The wall's chart Green form measures it in closed form, so the
/// backstop passes and the result is the half-disk less the notch: its
/// floor, `∫ (√(4 − s²) − 1.5) ds` over `|s| ≤ ½`, times the half metre
/// it sinks.
#[test]
fn a_notched_wall_measures_in_closed_form() {
    let half = bulge_loop(vec![
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 0.0), 1.0),
    ]);
    let half_disk: Body<f64> = extruded(SketchPlane::xy(), vec![half], 1.0, tol());
    let notch = brick((1.5, 2.5), (1.5, 2.5), (0.5, 2.5), tol());
    let body = body_of(
        topo::subtract(&half_disk, &notch, tol()),
        "the notched half-disk",
    );
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("the notched half-disk does not certify at rest: {e:?}"));
    let p = topo::mass_properties(&body, tol()).unwrap();
    let floor = 0.5 * 3.75f64.sqrt() + 4.0 * 0.25f64.asin() - 1.5;
    let expect = 2.0 * PI - 0.5 * floor;
    assert_eq!(p.volume_pad, 0.0, "closed-form faces only");
    assert!(
        (p.volume - expect).abs() <= 1e-12 * expect,
        "the notched half-disk: {} against the analytic {expect}",
        p.volume
    );
}

/// **A curved closed-form tie passes on the faces' own enclosures.** One
/// rod (`r = 0.7`, height 0.9, about `(0.1, 0.2)`) built from three arcs
/// started at 45° and at 60°: the same solid, whose `f64` sums round
/// 8.9e-16 m³ apart (1.38544236023309786 against …875). The backstop
/// re-derives each curved face's closed form at the interval scalar,
/// and only the faces' own enclosures, not their midpoints with the
/// fold rounded outward, hold the tie: collapsed to its midpoint, each
/// face's interval no longer covers the other start's sum and the
/// intersect refuses.
#[test]
fn a_curved_closed_form_tie_passes_on_the_faces_own_enclosures() {
    use crate::common::operands::three_arc_cylinder;
    let rod = |first: f64| three_arc_cylinder(Point2::new(0.1, 0.2), 0.7, 0.0, 0.9, first);
    let (low, high) = (rod(45.0), rod(60.0));
    let volume = |b: &Body<f64>| topo::mass_properties(b, tol()).unwrap().volume;
    assert!(
        volume(&low) < volume(&high),
        "the two starts' sums round apart: {} vs {}",
        volume(&low),
        volume(&high)
    );
    for (op, operand, result) in [
        (BooleanOp::Intersect, &low, &high),
        (BooleanOp::Subtract, &low, &high),
        (BooleanOp::Union, &high, &low),
    ] {
        assert_eq!(
            planted(op, operand, operand, result),
            Ok(AtRestOutcome::Validated),
            "{op:?}"
        );
    }
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

/// **What the last round leaves open on the oblique rod pair**, metered:
/// the boundary displacement a zero margin between two such bodies at
/// scale `s` stays open by once every face has run its last round.
/// Measured at scale 1: each body's last-round half-width is ≈ 2.2e-10
/// m³ of rule remainder whatever ε, plus ≈ 0.55·ε m³; two of them over
/// the pair's ≈ 25 m² of boundary meter ≈ 1.8e-11 m (the ε share,
/// ≈ 0.04·ε, is left out). The remainder scales with the volume and the
/// lever with the area, so the range grows linearly with `s`.
fn last_round_open(s: f64) -> f64 {
    1.8e-11 * s
}

/// Whether the backstop's last round decides a zero margin on the pair
/// at scale `s`: what it leaves open meters inside the band's
/// escalation edge `K·ε`. Beyond it the gate refuses `VolumeUndecided`.
fn last_round_decides(s: f64) -> bool {
    last_round_open(s) < tol().k() * tol().eps()
}

/// **Wrong results planted on quadrature-measured bodies.** `A` is the
/// oblique rod; a subtraction's "result" that is the same rod fattened
/// by a relative `δ` is too big by `≈ 5.5·δ` m³, a boundary displacement
/// of `δ·r`, so `δ` is spelled in ε. A fattening refuses — by refinement
/// past the reporting target where the reporting enclosures alone
/// cannot decide — down to the open range the last round every lane
/// runs leaves, about `4.5e-10 + 1.1·ε` m³ on this pair: `100·ε` clears
/// it at every ε, and `ε` does where the range meters inside the band.
/// There a fattening below it, and the right answer with it, is below
/// the model's resolution and is accepted. Where the range meters
/// beyond the band (ε = 1e-12, [`last_round_decides`]) the gate cannot
/// tell the right answer from a fattening the model resolves, and every
/// plant below the range refuses `VolumeUndecided`.
#[test]
fn planted_subtraction_results_refuse_down_to_the_stated_resolution() {
    let eps = tol().eps();
    let a = oblique_rod(0.0, 0.5);
    let b = oblique_cutter(0.0);
    let fattened = |delta: f64| {
        planted(
            BooleanOp::Subtract,
            &a,
            &b,
            &oblique_rod(0.0, 0.5 * (1.0 + delta)),
        )
    };
    for delta in [1e-3f64, 100.0 * eps] {
        assert_eq!(
            fattened(delta),
            Err(BooleanErrorKind::ResultVolumeImplausible),
            "a result fattened by δ = {delta:e} refuses"
        );
    }
    let (right, eps_fat, below) = (
        planted(BooleanOp::Subtract, &a, &b, &a),
        fattened(eps),
        fattened(0.01 * eps),
    );
    if last_round_decides(1.0) {
        assert_eq!(
            right,
            Ok(AtRestOutcome::Validated),
            "the right answer passes"
        );
        assert_eq!(
            eps_fat,
            Err(BooleanErrorKind::ResultVolumeImplausible),
            "a result fattened by δ = ε refuses"
        );
        assert_eq!(
            below,
            Ok(AtRestOutcome::Validated),
            "a fattening by δ = ε/100 is below the stated resolution"
        );
    } else {
        for (verdict, what) in [
            (right, "the right answer"),
            (eps_fat, "a fattening by δ = ε"),
            (below, "a fattening by δ = ε/100"),
        ] {
            assert_eq!(
                verdict,
                Err(BooleanErrorKind::VolumeUndecided),
                "{what} is inside the last round's open range, which meters beyond the band"
            );
        }
    }
}

/// The union side: `U = base ∪ boss(0.2, 0.5)` against a union whose
/// boss is short by a third of the two reporting enclosures' summed
/// half-width — so it refuses only through refinement. The radius that
/// short comes from the analytic volume's slope in `r`.
#[test]
fn a_planted_union_result_short_by_less_than_the_reporting_pads_refuses() {
    let theta = 0.2;
    let u = body_of(topo::union(&base(), &boss(theta, 0.5), tol()), "U");
    let pad_u = topo::mass_properties(&u, tol()).unwrap().volume_pad;
    let h = 1e-4;
    let slope =
        (penetrating_union(theta, BOSS_R + h) - penetrating_union(theta, BOSS_R - h)) / (2.0 * h);
    let short_boss = extruded(
        tilted_about_x(Point3::new(0.0, 0.0, 0.5), theta),
        vec![three_arc(
            Point2::new(0.0, 0.0),
            BOSS_R - 2.0 * pad_u / 3.0 / slope,
            0.0,
        )],
        1.0,
        tol(),
    );
    let short = body_of(topo::union(&base(), &short_boss, tol()), "the short union");
    let (pu, ps) = (
        topo::mass_properties(&u, tol()).unwrap(),
        topo::mass_properties(&short, tol()).unwrap(),
    );
    assert!(
        pu.volume - ps.volume < pu.volume_pad + ps.volume_pad,
        "the fixture's premise: the short union's midpoint {} is inside U's enclosures' reach ({} ± {:e}, ± {:e})",
        ps.volume,
        pu.volume,
        pu.volume_pad,
        ps.volume_pad
    );
    assert_eq!(
        planted(BooleanOp::Union, &u, &base(), &short),
        Err(BooleanErrorKind::ResultVolumeImplausible)
    );
}

/// **The enclosure arithmetic's direction.** The rod cut at 20° from a
/// circle and from three arcs are one solid, `3.5·π·r²`, and their
/// quadratures walk different edges, so their midpoints differ inside
/// both enclosures (at the default ε the three-arc rod's lands 4.5e-9 m³
/// lower). Planting the one with the higher midpoint as the result of
/// subtracting from the other puts the margin's midpoint below zero
/// while the true margin is zero: the backstop must not call it
/// implausible. It passes where the last round decides a zero margin
/// on this pair, and refuses `VolumeUndecided` where it cannot
/// ([`last_round_decides`]). A gate that read the midpoint, or
/// subtracted the enclosures' width instead of adding it, refuses it as
/// implausible.
#[test]
fn a_result_equal_in_truth_but_larger_at_the_midpoint_passes() {
    let cutter = oblique_cutter(0.0);
    let arcs = body_of(
        topo::subtract(
            &extruded(
                SketchPlane::xy(),
                vec![three_arc(Point2::new(0.0, 0.0), 0.5, 90.0)],
                4.0,
                tol(),
            ),
            &cutter,
            tol(),
        ),
        "the three-arc rod",
    );
    let circle = oblique_rod(0.0, 0.5);
    let (vc, va) = (
        topo::mass_properties(&circle, tol()).unwrap().volume,
        topo::mass_properties(&arcs, tol()).unwrap().volume,
    );
    assert!(
        vc != va,
        "the fixture's premise: the two rods' midpoints differ ({vc}, {va})"
    );
    let (lower, higher) = if vc > va {
        (&arcs, &circle)
    } else {
        (&circle, &arcs)
    };
    let expect = if last_round_decides(1.0) {
        Ok(AtRestOutcome::Validated)
    } else {
        Err(BooleanErrorKind::VolumeUndecided)
    };
    assert_eq!(planted(BooleanOp::Subtract, lower, &cutter, higher), expect);
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
    let f: Vec<_> = real.points().map(|(_, p)| bits(p.to_array())).collect();
    assert!(!f.is_empty());
    assert_eq!(d, f, "the dual's points are the f64 run's, in arena order");
}

/// [`oblique_rod`] at `x0 = 0` with every length scaled by `s`.
fn scaled_oblique_rod(s: f64, r: f64) -> Body<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), r * s, tol()).unwrap();
    let rod = extruded(SketchPlane::xy(), vec![disc.into()], 4.0 * s, tol());
    body_of(
        topo::subtract(&rod, &scaled_cutter(s), tol()),
        "the scaled oblique rod",
    )
}

/// [`oblique_cutter`] at `x0 = 0` with every length scaled by `s`.
fn scaled_cutter(s: f64) -> Body<f64> {
    let square = bulge_loop(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x * s, y * s), 0.0))
            .collect(),
    );
    extruded(
        tilted_about_x(Point3::new(0.0, 0.0, 3.5 * s), 20f64.to_radians()),
        vec![square],
        2.0 * s,
        tol(),
    )
}

/// **The last round fails loud.** What the last round leaves open grows
/// with the body ([`last_round_open`]), so on a large enough body it is
/// larger than the model's resolution. The scales are spelled in ε so
/// each is the same model at every ε: `10¹³·ε` (10⁴ at the default ε,
/// the rod 40 km tall) and `10¹²·ε`. At the first, a result fattened by
/// δ = 1e-11 is too big by a `0.22·δ·s` = 22ε boundary displacement and
/// its sign is still open at the last round: the backstop refuses
/// `VolumeUndecided` rather than accept it. The same holds of a CORRECT
/// result at the second, whose open range meters 18ε, beyond the band
/// too: the gate cannot tell it from a wrong one, and says so
/// (`work/quad/quadrature-interval-floor-grows-with-the-body-past-the-band`).
#[test]
fn an_open_sign_beyond_the_band_at_the_last_round_refuses() {
    let eps = tol().eps();
    let s = 1e13 * eps;
    let (a, b) = (scaled_oblique_rod(s, 0.5), scaled_cutter(s));
    assert_eq!(
        planted(
            BooleanOp::Subtract,
            &a,
            &b,
            &scaled_oblique_rod(s, 0.5 * (1.0 + 1e-11))
        ),
        Err(BooleanErrorKind::VolumeUndecided),
        "22ε too big at scale {s:e}"
    );
    let s = 1e12 * eps;
    let (a, b) = (scaled_oblique_rod(s, 0.5), scaled_cutter(s));
    assert_eq!(
        planted(BooleanOp::Subtract, &a, &b, &a),
        Err(BooleanErrorKind::VolumeUndecided),
        "the right answer at scale {s:e} is undecidable at the last round"
    );
    let err = topo::test_support::volume_backstop(BooleanOp::Subtract, &a, &b, &a, tol())
        .expect_err("undecided");
    assert!(err.to_string().contains("vol(A ∖ B) ≤ vol(A)"), "{err}");
}
