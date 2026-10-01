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
use topo::splitting::{SplitPart, SplitPlane, split};
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
/// Two lune bores, one per half, and a control inside the polygon.
const HOLES: [(f64, f64); 3] = [(1.6, 0.8), (-1.6, 0.8), (0.8, 0.4)];

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
    let hole = [(cx - BORE, cy, 1.0), (cx + BORE, cy, 1.0)];
    extruded(SketchPlane::xy(), &[outer, &hole], 1.0)
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
                let plane = SplitPlane {
                    origin: Point3::new(0.0, 0.0, 0.5),
                    normal: Vec3::new(nx, 0.0, 0.0),
                };
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
