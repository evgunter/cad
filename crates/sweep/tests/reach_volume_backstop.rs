//! **The boolean's volume backstop measures curved trims.** The
//! backstop compares the result's volume with its operands' and so
//! must measure all three bodies. These rows are booleans whose bodies
//! carry a cylinder wall trimmed by an ellipse — a boss on a tilted
//! sketch plane, an oblique cut through a rod — which only the certified
//! quadrature encloses: each builds, certifies at rest, and has its
//! analytic volume. A wall the property layer cannot measure at all
//! refuses `VolumeUnmeasured`, carrying the property layer's own
//! refusal, never a kernel-invariant sentence about a body that is
//! neither planar nor corrupt.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::three_arc;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, extruded, sketch_from_axes};
use topo::{Body, BooleanError, MassPropsError};

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

/// The result certifies at rest and has `expect` for its volume.
fn assert_sound(
    r: Result<topo::BooleanResult<f64>, BooleanError>,
    expect: f64,
    what: &str,
) -> Body<f64> {
    let out = r.unwrap_or_else(|e| panic!("{what}: the boolean refuses: {e:?}"));
    let body = out.body().expect("a body remains").body.clone();
    topo::validate_geometric_certificate(&body, tol())
        .unwrap_or_else(|e| panic!("{what}: the result does not certify at rest: {e:?}"));
    let v = topo::mass_properties(&body, tol()).unwrap().volume;
    assert!(
        (v - expect).abs() < 1e-9,
        "{what}: volume {v} against the analytic {expect}"
    );
    body
}

/// **A boss on a definitely tilted sketch plane, unioned onto a box.**
/// The base is `[−1, 1]² × [0, 1]`; the boss is a radius-`r` cylinder
/// of height 1 whose sketch plane passes through `(0, 0, z0)` turned
/// `θ` about `x`, so its wall meets the box's top face along an
/// ellipse.
///
/// - Standing on the top face (`z0 = 1`): the half of the cap below
///   `z = 1` sinks into the base by the ungula `⅔·r³·tan θ`, so the
///   union is `4 + π·r² − ⅔·r³·tan θ`.
/// - Penetrating it (`z0 = 0.5`): the boss's axis reaches `z = 1` at
///   `0.5 / cos θ`, and a cylinder between a square cut and an oblique
///   one holds `π·r²` times its axial length, so the union is
///   `4 + π·r²·(1 − 0.5 / cos θ)`.
#[test]
fn a_tilted_boss_unions_onto_a_box() {
    let r: f64 = 0.25;
    let base = brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
    for theta in [4e-3f64, 0.2] {
        for (z0, expect) in [
            (1.0, 4.0 + PI * r * r - 2.0 / 3.0 * r * r * r * theta.tan()),
            (0.5, 4.0 + PI * r * r * (1.0 - 0.5 / theta.cos())),
        ] {
            let boss = extruded(
                tilted_about_x(Point3::new(0.0, 0.0, z0), theta),
                vec![three_arc(Point2::new(0.0, 0.0), r, 0.0)],
                1.0,
                tol(),
            );
            assert_sound(
                topo::union(&base, &boss, tol()),
                expect,
                &format!("boss tilted {theta} rad from z = {z0}"),
            );
        }
    }
}

/// The rod: `r = 0.5` about `z` over `z ∈ [0, 4]`, an extruded circle.
fn rod() -> Body<f64> {
    let disc = profile::circle(Point2::new(0.0, 0.0), 0.5, tol()).unwrap();
    extruded(SketchPlane::xy(), vec![disc.into()], 4.0, tol())
}

/// **An untilted rod minus an obliquely tilted box.** The box is
/// extruded 2 from the square `[−1, 1]²` on the sketch plane through
/// `(0, 0, 3.5)` turned 20° about `x`, so it removes everything of
/// the rod above that plane and trims its wall by an ellipse. The
/// plane passes through the rod's axis, so what is left is
/// `3.5·π·r²`.
#[test]
fn a_rod_cut_by_an_oblique_box_keeps_its_volume() {
    let square = bulge_loop(
        [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let cutter = extruded(
        tilted_about_x(Point3::new(0.0, 0.0, 3.5), 20f64.to_radians()),
        vec![square],
        2.0,
        tol(),
    );
    assert_sound(
        topo::subtract(&rod(), &cutter, tol()),
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

/// **A dual holds no certified quadrature** (DL1), so at `Dual64` the
/// tilted boss's union measures in closed form, and its ellipse-trimmed
/// wall refuses there: the backstop answers `VolumeUnmeasured` for the
/// result, carrying that face's refusal. The `f64` evaluation of the
/// same recipe, which the dual rides beside, is the row above.
#[test]
fn a_dual_measures_the_tilted_boss_in_closed_form_and_says_so() {
    use geom_core::{Dual, Dual64};
    let c = Dual::constant;
    let theta = 0.2f64;
    let base = brick::<Dual64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol());
    let plane = sketch_from_axes(
        Point3::new(c(0.0), c(0.0), c(1.0)),
        Vec3::new(c(1.0), c(0.0), c(0.0)),
        Vec3::new(c(0.0), c(theta.cos()), c(theta.sin())),
        tol(),
    );
    let boss = extruded(
        plane,
        vec![three_arc(Point2::new(0.0, 0.0), 0.25, 0.0).map_scalar(c)],
        c(1.0),
        tol(),
    );
    let err = topo::union(&base, &boss, tol()).expect_err("the dual cannot measure the wall");
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
