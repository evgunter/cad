//! **A hollow round tube standing on a plate unions at its closed
//! form** (`work/zip/a-round-tube-standing-on-a-plate-refuses-seam-orientation`).
//!
//! The plate is `[0,3] × [0,2] × [0,1]`. The tube stands about
//! `(1.5, 1)` from `z = 0.5` to `z = 2`: its rim a two-arc circle of
//! radius `0.6` (seams at 0° and 180°), its bore a three-arc circle of
//! radius `0.4` turned by `0.3` rad, so no seam of one meets a seam of
//! the other. The plate's top meets both walls, and its section rings
//! are full circles on each.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{ProfileLoop, SketchPlane, circle_split};
use sweep::test_support::{brick, extruded, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn plate() -> AtRestBody<f64> {
    finished(
        "the plate",
        brick((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    )
}

fn tube() -> AtRestBody<f64> {
    let c = Point2::new(1.5, 1.0);
    let rim: ProfileLoop<f64> = circle_split(c, 0.6, 2, 0.0, tol()).unwrap().into();
    let bore: ProfileLoop<f64> = circle_split(c, 0.4, 3, 0.3, tol()).unwrap().into();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.5)));
    finished(
        "the tube",
        extruded(plane, vec![rim, bore], 1.5, tol()),
        tol(),
    )
}

/// The slab across the tube, `x ∈ [1.45, 1.55]`, `y ∈ [−1, 3]`, from
/// `z = 0.47` to `top`.
fn slab(top: f64) -> AtRestBody<f64> {
    finished(
        "the slab",
        brick((1.45, 1.55), (-1.0, 3.0), (0.47, top), tol()),
        tol(),
    )
}

/// The plate and the tube's annulus over `z ∈ [1, 2]`.
const PLATE_AND_TUBE: f64 = 6.0 + 0.2 * PI;

/// The area the slab's strip `|x − 1.5| < 0.05` shares with the tube's
/// annulus: the strip's chord area in the rim's disc less the bore's.
fn strip_in_annulus() -> f64 {
    let h: f64 = 0.05;
    let chord = |r: f64| 2.0 * (h * (r * r - h * h).sqrt() + r * r * (h / r).asin());
    chord(0.6) - chord(0.4)
}

fn assert_sound(what: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let r = r.unwrap_or_else(|e| panic!("{what}: builds: {e:?}"));
    let bb = r
        .body()
        .unwrap_or_else(|| panic!("{what}: leaves material"));
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
    topo::validate_geometric_certificate(&bb.body, tol())
        .unwrap_or_else(|e| panic!("{what}: the at-rest certificate: {e:?}"));
    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
    assert!(
        (v - want).abs() < 1e-9,
        "{what}: volume {v} against the closed form {want}"
    );
}

/// **The pair builds in both member orders**, through tiers 2 and 3′
/// and the at-rest certificate, at `6 + 0.2π`.
#[test]
fn a_round_tube_on_a_plate_unions_in_both_orders() {
    let (p, t) = (plate(), tube());
    assert_sound("plate ∪ tube", topo::union(&p, &t, tol()), PLATE_AND_TUBE);
    assert_sound("tube ∪ plate", topo::union(&t, &p, tol()), PLATE_AND_TUBE);
}

/// **A slab across the tube unions in all six member orders**, ending
/// inside the tube's wall height (`z = 1.03`) and at `z = 1.5`, each
/// order through the same checks at the closed form: the slab adds its
/// ends past the plate and, over the plate's top, its strip less what
/// the annulus already holds.
#[test]
fn a_slab_across_a_round_tube_on_a_plate_unions_in_every_order() {
    let (p, t) = (plate(), tube());
    for top in [1.03, 1.5] {
        let s = slab(top);
        let want = PLATE_AND_TUBE + 0.2 * (top - 0.47) + (0.2 - strip_in_annulus()) * (top - 1.0);
        let m = [("plate", &p), ("tube", &t), ("slab", &s)];
        for o in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let what = format!(
                "{} ∪ {} ∪ {}, slab top {top}",
                m[o[0]].0, m[o[1]].0, m[o[2]].0
            );
            let first = topo::union(m[o[0]].1, m[o[1]].1, tol())
                .unwrap_or_else(|e| panic!("{what}: the first pair builds: {e:?}"));
            let first = first
                .body()
                .unwrap_or_else(|| panic!("{what}: the first pair leaves material"));
            let first = finished(&what, first.body.clone().into_body(), tol());
            assert_sound(&what, topo::union(&first, m[o[2]].1, tol()), want);
        }
    }
}
