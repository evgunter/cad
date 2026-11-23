//! **Intersection of axis-aligned letter prisms builds in every operand
//! order.** The tour's silhouette letters at natural proportions,
//! all inside the block `[0, 2] × [0, 3] × [0, 3]`, whose walls and caps
//! lie flush with one another on several planes: `H` (an xy sketch
//! extruded 3 along +z), `T` (yz, 2 along +x) and `C` (zx, 3 along +y),
//! and the two-operand pair `A` and `Z` (xy and yz, each 2 deep).
//! Intersection commutes, so every nesting of the three and both orders
//! of the two build the same closed-form volume, sound (tiers 2 and 3′,
//! the at-rest certificate, a legal operand), through the plain
//! booleans.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::AtRestBody;

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

/// `loops` (the outer counter-clockwise, any inner clockwise) on
/// `plane`, extruded `depth` along its normal.
fn letter(
    what: &str,
    plane: SketchPlane<f64>,
    loops: &[&[(f64, f64)]],
    depth: f64,
) -> AtRestBody<f64> {
    let loops = loops
        .iter()
        .map(|pts| bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect()))
        .collect();
    let profile = Profile::new(plane, loops).validate(tol()).unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

fn assert_intersect_sound(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    let line = outcome(topo::intersect(a, b, tol()), want, tol());
    assert!(line.starts_with("OK SOUND"), "{what}: {line}");
}

fn intersected(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    let r = topo::intersect(a, b, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let bb = r.body().unwrap_or_else(|| panic!("{what}: empty"));
    bb.body.clone()
}

/// `H ∩ T ∩ C` is 11/4 by z-slab: over `z ∈ [0, ½]` the stem band of
/// `T` across `H`'s bar (area 1), over `[½, 5⁄2]` that band inside `C`'s
/// spine (¼), over `[5⁄2, 3]` all of `H` (7⁄2). `H ∩ T` is 17/4,
/// `H ∩ C` 13/2 and `T ∩ C` 4.
#[test]
fn h_t_c_intersect_in_every_nesting() {
    let h = letter(
        "H",
        SketchPlane::xy(),
        &[&[
            (0.0, 0.0),
            (0.5, 0.0),
            (0.5, 1.25),
            (1.5, 1.25),
            (1.5, 0.0),
            (2.0, 0.0),
            (2.0, 3.0),
            (1.5, 3.0),
            (1.5, 1.75),
            (0.5, 1.75),
            (0.5, 3.0),
            (0.0, 3.0),
        ]],
        3.0,
    );
    let t = letter(
        "T",
        SketchPlane::yz(),
        &[&[
            (1.25, 0.0),
            (1.75, 0.0),
            (1.75, 2.5),
            (3.0, 2.5),
            (3.0, 3.0),
            (0.0, 3.0),
            (0.0, 2.5),
            (1.25, 2.5),
        ]],
        2.0,
    );
    let c = letter(
        "C",
        SketchPlane::zx(),
        &[&[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.5, 2.0),
            (2.5, 0.5),
            (0.5, 0.5),
            (0.5, 2.0),
            (0.0, 2.0),
        ]],
        3.0,
    );
    for (what, a, b, want) in [
        ("H ∩ T", &h, &t, 4.25),
        ("T ∩ H", &t, &h, 4.25),
        ("H ∩ C", &h, &c, 6.5),
        ("C ∩ H", &c, &h, 6.5),
        ("T ∩ C", &t, &c, 4.0),
        ("C ∩ T", &c, &t, 4.0),
    ] {
        assert_intersect_sound(what, a, b, want);
    }
    let (ht, hc, tc) = (
        intersected("H ∩ T", &h, &t),
        intersected("H ∩ C", &h, &c),
        intersected("T ∩ C", &t, &c),
    );
    for (what, a, b) in [
        ("(H ∩ T) ∩ C", &ht, &c),
        ("C ∩ (H ∩ T)", &c, &ht),
        ("(H ∩ C) ∩ T", &hc, &t),
        ("T ∩ (H ∩ C)", &t, &hc),
        ("(T ∩ C) ∩ H", &tc, &h),
        ("H ∩ (T ∩ C)", &h, &tc),
    ] {
        assert_intersect_sound(what, a, b, 2.75);
    }
}

/// The tour's `az` letters: `A` with its counter, `Z` across it;
/// `A ∩ Z` is 38627/14336 in either order.
#[test]
fn a_and_z_intersect_in_either_order() {
    let a = letter(
        "A",
        SketchPlane::xy(),
        &[
            &[
                (0.0, 0.0),
                (0.625, 0.0),
                (0.8125, 1.0),
                (1.1875, 1.0),
                (1.375, 0.0),
                (2.0, 0.0),
                (1.125, 2.5),
                (0.875, 2.5),
            ],
            &[(1.0, 2.0), (1.09375, 1.4375), (0.90625, 1.4375)],
        ],
        2.0,
    );
    let z = letter(
        "Z",
        SketchPlane::yz(),
        &[&[
            (0.0, 0.0),
            (2.5, 0.0),
            (2.5, 0.4375),
            (0.75, 0.4375),
            (2.5, 1.5625),
            (2.5, 2.0),
            (0.0, 2.0),
            (0.0, 1.5625),
            (1.75, 1.5625),
            (0.0, 0.4375),
        ]],
        2.0,
    );
    let want = 38627.0 / 14336.0;
    assert_intersect_sound("A ∩ Z", &a, &z, want);
    assert_intersect_sound("Z ∩ A", &z, &a, want);
}
