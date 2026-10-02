//! **A hole in the lune follows its half when a face is divided.**
//!
//! A disc of radius 2 with a bore of radius 0.15 centred at
//! `(±1.6, 0.8)`, divided along `x = 0`. Each half's outer loop is arcs
//! plus the straight chord over three or more vertices, and the bore
//! sits in the lune between an arc and the polygon through those
//! vertices: outside the polygon, inside the half-disc. Ring re-homing
//! reads the run on its own carriers, so the bore moves with its half;
//! read off the polygon it stayed on the other half's cap and the
//! divided shell refused as torn.
//!
//! Built through the public doors: a holed `Profile`, `extrude`, then
//! `split` and the Boolean. The control row puts the bore inside the
//! polygon, where both readings agree.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::sketch_at;
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, split};
use topo::{Body, BooleanResult};

const BORE: f64 = 0.15;
/// A profile loop, as `(x, y, bulge)` vertices.
type Loop = &'static [(f64, f64, f64)];

/// A two-arc disc and a four-arc one: the second puts vertices of its
/// own on the chord's ends, the first has the split mint them.
const DISCS: [(&str, Loop); 2] = [
    ("two-arc disc", &[(2.0, 0.0, 1.0), (-2.0, 0.0, 1.0)]),
    (
        "four-arc disc",
        &[
            (2.0, 0.0, 0.414_213_562_373_095_03),
            (0.0, 2.0, 0.414_213_562_373_095_03),
            (-2.0, 0.0, 0.414_213_562_373_095_03),
            (0.0, -2.0, 0.414_213_562_373_095_03),
        ],
    ),
];
/// Two lune bores, one per half; one whose anchor vertex `(1.2, 0.8)`
/// lies ON the polygon's edge `x + y = 2` (the polygon walk's
/// `OnBoundary`, though the anchor is in the half-disc's interior); and
/// a control inside the polygon.
const HOLES: [(f64, f64); 4] = [(1.6, 0.8), (-1.6, 0.8), (1.35, 0.8), (0.8, 0.4)];

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(plane: SketchPlane<f64>, loops: &[&[(f64, f64, f64)]], h: f64) -> Body<f64> {
    let loops = loops
        .iter()
        .map(|lp| bulge_loop(lp.iter().map(|&(x, y, b)| (Point2::new(x, y), b)).collect()))
        .collect();
    let profile = Profile::new(plane, loops)
        .validate(tol())
        .expect("a valid profile");
    extrude(&profile, Extrusion::Distance(h), tol())
        .expect("the profile extrudes")
        .body
}

fn bored_disc(outer: &[(f64, f64, f64)], (cx, cy): (f64, f64)) -> Body<f64> {
    bored_disc_n(outer, &[(cx, cy)])
}

fn bored_disc_n(outer: &[(f64, f64, f64)], centres: &[(f64, f64)]) -> Body<f64> {
    let holes: Vec<[(f64, f64, f64); 2]> = centres
        .iter()
        .map(|&(cx, cy)| [(cx - BORE, cy, 1.0), (cx + BORE, cy, 1.0)])
        .collect();
    let mut loops: Vec<&[(f64, f64, f64)]> = vec![outer];
    loops.extend(holes.iter().map(|h| &h[..]));
    extruded(SketchPlane::xy(), &loops, 1.0)
}

/// The slab `x ∈ [x0, x1]` standing through the disc.
fn slab(x0: f64, x1: f64) -> Body<f64> {
    let rect = [
        (x0, -3.0, 0.0),
        (x1, -3.0, 0.0),
        (x1, 3.0, 0.0),
        (x0, 3.0, 0.0),
    ];
    extruded(sketch_at(-0.5), &[&rect], 2.0)
}

/// The half-disc's volume, less the bore when the half holds it.
fn half_volume(holds_bore: bool) -> f64 {
    2.0 * PI - if holds_bore { PI * BORE * BORE } else { 0.0 }
}

/// Tier 3 passes and the volume says which half holds the bore.
fn assert_half(row: &str, part: &Body<f64>, holds_bore: bool) {
    if let Err(errs) = topo::validate_geometric(part, tol()) {
        panic!("{row}: tier 3 refused the half: {errs:?}");
    }
    let v = topo::mass_properties(part, tol())
        .unwrap_or_else(|e| panic!("{row}: mass properties refused: {e:?}"))
        .volume;
    let want = half_volume(holds_bore);
    assert!(
        (v - want).abs() < 1e-9,
        "{row}: volume {v}, want {want} (bore held: {holds_bore})"
    );
}

#[test]
fn a_split_carries_a_lune_bore_with_its_half() {
    for (disc, outer) in DISCS {
        for hole in HOLES {
            for nx in [1.0, -1.0] {
                let row = format!("{disc}, bore at {hole:?}, plane normal x = {nx}");
                let plane = topo::test_support::split_plane(
                    Point3::new(0.0, 0.0, 0.5),
                    Vec3::new(nx, 0.0, 0.0),
                );
                let result = split(&bored_disc(outer, hole), &plane, tol())
                    .unwrap_or_else(|e| panic!("{row}: split refused: {e:?}"));
                let above_holds = (hole.0 > 0.0) == (nx > 0.0);
                for (side, part, holds) in [
                    ("above", &result.above, above_holds),
                    ("below", &result.below, !above_holds),
                ] {
                    let SplitPart::Body(part) = part else {
                        panic!("{row}: the {side} side is empty");
                    };
                    assert_half(&format!("{row}, {side}"), part, holds);
                }
            }
        }
    }
}

#[test]
fn a_boolean_carries_a_lune_bore_with_its_half() {
    let (_, outer) = DISCS[0];
    let (right, left) = (slab(0.0, 3.0), slab(-3.0, 0.0));
    for hole in HOLES {
        let body = bored_disc(outer, hole);
        let rows = [
            (
                "intersect x > 0",
                topo::intersect(&body, &right, tol()),
                hole.0 > 0.0,
            ),
            (
                "intersect x < 0",
                topo::intersect(&body, &left, tol()),
                hole.0 < 0.0,
            ),
            (
                "subtract x > 0",
                topo::subtract(&body, &right, tol()),
                hole.0 < 0.0,
            ),
            (
                "subtract x < 0",
                topo::subtract(&body, &left, tol()),
                hole.0 > 0.0,
            ),
        ];
        for (op, result, holds) in rows {
            let row = format!("bore at {hole:?}, {op}");
            match result {
                Ok(BooleanResult::Body(b)) => assert_half(&row, &b.body, holds),
                Ok(_) => panic!("{row}: the result is empty"),
                Err(e) => panic!("{row}: refused: {e:?}"),
            }
        }
    }
}

/// Two lune bores on one face: each is placed on its own, whether they
/// share a half or not.
#[test]
fn a_split_carries_two_lune_bores_each_with_its_half() {
    let (_, outer) = DISCS[0];
    let plane =
        topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::new(1.0, 0.0, 0.0));
    let bore = PI * BORE * BORE;
    for (pose, centres, above_bores) in [
        ("opposite halves", [(1.6, 0.8), (-1.6, -0.8)], 1.0),
        ("one half", [(1.6, 0.8), (1.6, -0.8)], 2.0),
    ] {
        let result = split(&bored_disc_n(outer, &centres), &plane, tol())
            .unwrap_or_else(|e| panic!("{pose}: split refused: {e:?}"));
        for (side, part, bores) in [
            ("above", &result.above, above_bores),
            ("below", &result.below, 2.0 - above_bores),
        ] {
            let SplitPart::Body(part) = part else {
                panic!("{pose}: the {side} side is empty");
            };
            if let Err(errs) = topo::validate_geometric(part, tol()) {
                panic!("{pose}, {side}: tier 3 refused the half: {errs:?}");
            }
            let v = topo::mass_properties(part, tol()).unwrap().volume;
            let want = 2.0 * PI - bores * bore;
            assert!(
                (v - want).abs() < 1e-9,
                "{pose}, {side}: volume {v}, want {want}"
            );
        }
    }
}

/// **An ellipse-bearing run.** The bored disc cut on the oblique plane
/// `z = 0.5 − 0.2y` leaves a lower piece whose top face is an ellipse
/// holding the bore's elliptic section as a ring; dividing that piece at
/// `x = 0` divides the elliptic face with the ring in the lune of an
/// ELLIPSE arc. Read off the polygon, the bore at `(−1.6, 0.8)` went to
/// the wrong half and the split answered `Ok`: one half carried the
/// bore's ring outside its outer loop (tier 3's `RingOutsideOuter`), the
/// other passed tier 3 with no bore in it — not the torn refusal the
/// circular cap gives.
#[test]
fn an_oblique_cut_carries_a_lune_bore_with_its_half() {
    let (_, outer) = DISCS[0];
    let oblique = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, 0.2, 1.0).normalize(),
    );
    for (cx, cy) in [(1.6, 0.8), (-1.6, 0.8), (0.8, 0.4)] {
        let SplitPart::Body(lower) = split(&bored_disc(outer, (cx, cy)), &oblique, tol())
            .unwrap_or_else(|e| panic!("bore at ({cx}, {cy}): oblique split refused: {e:?}"))
            .below
        else {
            panic!("bore at ({cx}, {cy}): the lower piece is empty");
        };
        // Each half-disc (area 2π) under z = 0.5 − 0.2y holds π (the y-term
        // integrates to zero over a half symmetric in y), less the bore's
        // column of height 0.5 − 0.2·cy on the half that holds it.
        let column = PI * BORE * BORE * (0.2f64.mul_add(-cy, 0.5));
        for nx in [1.0, -1.0] {
            let row = format!("bore at ({cx}, {cy}), plane normal x = {nx}");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 0.0, 0.25),
                Vec3::new(nx, 0.0, 0.0),
            );
            let result = split(&lower, &plane, tol())
                .unwrap_or_else(|e| panic!("{row}: split refused: {e:?}"));
            let above_holds = (cx > 0.0) == (nx > 0.0);
            for (side, part, holds) in [
                ("above", &result.above, above_holds),
                ("below", &result.below, !above_holds),
            ] {
                let SplitPart::Body(part) = part else {
                    panic!("{row}: the {side} side is empty");
                };
                if let Err(errs) = topo::validate_geometric(part, tol()) {
                    panic!("{row}, {side}: tier 3 refused the half: {errs:?}");
                }
                let v = topo::mass_properties(part, tol()).unwrap().volume;
                let want = PI - if holds { column } else { 0.0 };
                // The oblique face's quadrature lands ~1.5e-9 off.
                assert!(
                    (v - want).abs() < 1e-8,
                    "{row}, {side}: volume {v}, want {want} (bore held: {holds})"
                );
            }
        }
    }
}
