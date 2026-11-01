//! **A box along a fillet's line–arc junction.** `A` is the square
//! `[−1, 1]²` with its `(+, +)` corner rounded by a quarter circle
//! `r = 0.2` about `(0.8, 0.8)`, extruded over `z ∈ [0, 1]`. Each box has
//! a face lying along one of `A`'s straight sides up to the junction
//! where that side meets the arc tangentially, and past it: along
//! `y = 1` through `(0.8, 1)`, and, mirrored, along `x = 1` through
//! `(1, 0.8)`. The box's face is coplanar with the side on one side of
//! the junction's ruling and tangent to the arc's wall on the other, so
//! the ruling is where `A`'s material changes from a coincident face to
//! a tangent one. The box spans `A`'s height, ends flush with its top
//! or floor, stands past both, or lies between them, so the junction's
//! ends meet the box at a vertex pair, inside its face, or not at all.
//!
//! Two neighbours put the box face along a ruling with a different face
//! on its far side: a concave arc, so both flankers are tangent to the
//! box face and curve to opposite sides of it (an inflection), and a
//! straight side falling away from it, so one flanker is tangent and the
//! other leaves the face at first order.
//!
//! Every row runs ∪, ∩ and both differences in both operand orders, at
//! rest and turned, each read through `differential::outcome` (tiers 2
//! and 3′, the certificate, the legal operand, the closed-form volume)
//! and meshed, or refuses as an open row pins it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_8, PI};

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanResult, EdgeInFaceGap};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::poses::torax_pose;

fn tol() -> Tol {
    Tol::witness()
}

/// The fillet's radius.
const R: f64 = 0.2;

/// The area the rounding takes from the square's corner.
const CORNER: f64 = R * R - PI * R * R / 4.0;

/// `body` moved by `pose`, finished.
fn posed(what: &str, body: topo::Body<f64>, pose: &Affine3<f64>) -> AtRestBody<f64> {
    let body =
        topo::transform_rigid(&body, &geom_core::test_support::rigid(pose, tol()), tol()).unwrap();
    finished(what, body, tol())
}

/// The bulge loop `chain` extruded over `z ∈ [0, 1]`, moved by `pose`.
fn prism(what: &str, chain: Vec<(Point2<f64>, f64)>, pose: &Affine3<f64>) -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![bulge_loop(chain)])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: 1.0,
        side: ExtrudeSide::Along,
    };
    posed(what, extrude(&profile, depth, tol()).unwrap().body, pose)
}

/// The rounded prism `A`: `4 − CORNER`.
fn rounded(pose: &Affine3<f64>) -> AtRestBody<f64> {
    let chain = vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
        (Point2::new(1.0, 1.0 - R), FRAC_PI_8.tan()),
        (Point2::new(1.0 - R, 1.0), 0.0),
        (Point2::new(-1.0, 1.0), 0.0),
    ];
    prism("the rounded prism", chain, pose)
}

/// A run's verdict: `SOUND` at its closed form, its body meshed.
fn holds(row: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    if let Ok(BooleanResult::Body(b)) = &r {
        let m = mesh::tessellate(&b.body, 5e-3, tol()).unwrap_or_else(|e| panic!("{row}: {e:?}"));
        mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{row}: mesh {e:?}"));
    }
    let line = outcome(r, want, tol());
    assert!(line.starts_with("OK SOUND"), "{row}: {line}");
}

/// The box `y ∈ [0.5, 1]`, `x ∈ [0.5, 1.5]` over `z`, along `A`'s side
/// `y = 1`, or its mirror in `x = y` along `x = 1`, against `A`, both
/// moved by `pose`: every op in both orders. The box is `0.5` across
/// its section, of which `A` holds `0.25 − CORNER`.
fn along_the_junction(pose: &Affine3<f64>, at: &str) {
    let a = rounded(pose);
    for z in [
        (0.0, 1.0),
        (0.0, 0.5),
        (0.5, 1.0),
        (-0.5, 1.0),
        (0.0, 1.5),
        (-0.5, 1.5),
        (0.25, 0.75),
    ] {
        let (h, held) = (z.1 - z.0, f64::min(z.1, 1.0) - f64::max(z.0, 0.0));
        for (side, x, y) in [
            ("y = 1", (0.5, 1.5), (0.5, 1.0)),
            ("x = 1", (0.5, 1.0), (0.5, 1.5)),
        ] {
            let b = posed("the box", brick(x, y, z, tol()), pose);
            let v = (4.0 - CORNER, 0.5 * h, (0.25 - CORNER) * held);
            for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
                holds(&format!("{at}, along {side}, z ∈ {z:?}: {op}"), r, want);
            }
        }
    }
}

/// **At rest.**
#[test]
fn a_box_along_the_junction_at_rest() {
    along_the_junction(&Affine3::translation(Vec3::new(0.0, 0.0, 0.0)), "at rest");
}

/// **Turned** by the torax pose.
#[test]
fn a_box_along_the_junction_turned() {
    along_the_junction(&torax_pose(), "turned");
}

/// The refusal an open row pins a run at, by the box's `z` span and the
/// op; `None` where the run builds.
type Open = fn((f64, f64), &str) -> Option<Refusal>;

/// Whether a run refused as an open row pins it.
type Refusal = fn(&BooleanError) -> bool;

/// A neighbour of `A`: the body, its volume, and the part of the box's
/// section it holds.
type Neighbour = (AtRestBody<f64>, f64, f64);

/// The box `y ∈ [0.5, 1]`, `x ∈ [0.5, 1.5]` over `z` spans against the
/// neighbour `make` builds, both moved by each pose: every op in both
/// orders, SOUND, or refused as `open` pins it.
fn beside_the_junction(make: fn(&Affine3<f64>) -> Neighbour, open: Open) {
    for (at, pose) in [
        ("at rest", Affine3::translation(Vec3::new(0.0, 0.0, 0.0))),
        ("turned", torax_pose()),
    ] {
        let (a, va, section) = make(&pose);
        for z in [
            (0.0, 1.0),
            (0.0, 0.5),
            (-0.5, 1.0),
            (-0.5, 1.5),
            (0.25, 0.75),
            (0.0, 1.5),
            (0.5, 1.5),
            (0.5, 1.0),
            (-0.5, 0.5),
        ] {
            let (h, held) = (z.1 - z.0, f64::min(z.1, 1.0) - f64::max(z.0, 0.0));
            let b = posed("the box", brick((0.5, 1.5), (0.5, 1.0), z, tol()), &pose);
            let v = (va, 0.5 * h, section * held);
            for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
                let row = format!("{at}, z ∈ {z:?}: {op}");
                match open(z, op) {
                    None => holds(&row, r, want),
                    Some(refusal) => {
                        if !r.as_ref().is_err_and(refusal) {
                            panic!(
                                "{row}: the pinned refusal moved: {}",
                                outcome(r, want, tol())
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The parting's tangent-wall gap.
fn a_tangent_parting(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::EdgeInFace {
            gap: EdgeInFaceGap::Tangent,
            ..
        }
    )
}

/// **An inflection ruling.** `A`'s corner runs the fillet's convex arc
/// into `(0.8, 1)` and a concave arc about `(0.8, 1.2)` on to
/// `(0.6, 1.2)`: `4 + 1.6 R`, of which the box's section holds
/// `0.25 − CORNER`. Where the box stands past both of `A`'s ends, both
/// ends of the ruling lie inside the box's face.
#[test]
fn a_box_along_an_inflection_ruling() {
    fn make(pose: &Affine3<f64>) -> Neighbour {
        let t = FRAC_PI_8.tan();
        let chain = vec![
            (Point2::new(-1.0, -1.0), 0.0),
            (Point2::new(1.0, -1.0), 0.0),
            (Point2::new(1.0, 1.0 - R), t),
            (Point2::new(1.0 - R, 1.0), -t),
            (Point2::new(1.0 - 2.0 * R, 1.0 + R), 0.0),
            (Point2::new(-1.0, 1.0 + R), 0.0),
        ];
        let a = prism("the inflected prism", chain, pose);
        (a, 4.0 + 1.6 * R, 0.25 - CORNER)
    }
    beside_the_junction(make, |_, _| None);
}

/// **A tangent wall beside a falling side.** `A`'s corner runs the
/// fillet's arc into `(0.8, 1)`, from where a straight side falls to
/// `(−1, 0.6)`. `B ∖ A` refuses the parting's tangent-wall gap at every
/// span, open on
/// `work/parting/the-boolean-parting-refuses-its-rarer-configurations.md`.
#[test]
fn a_box_along_a_tangent_wall_beside_a_falling_side() {
    fn make(pose: &Affine3<f64>) -> Neighbour {
        let chain = vec![
            (Point2::new(-1.0, -1.0), 0.0),
            (Point2::new(1.0, -1.0), 0.0),
            (Point2::new(1.0, 1.0 - R), FRAC_PI_8.tan()),
            (Point2::new(1.0 - R, 1.0), 0.0),
            (Point2::new(-1.0, 0.6), 0.0),
        ];
        // The side's height over `x`, and the trapezoid under it over
        // `x ∈ [0.5, 0.8]`, above the box's `y = 0.5`.
        let side = |x: f64| 0.6 + 0.4 * (x + 1.0) / 1.8;
        let strip = 0.3 * ((side(0.5) - 0.5) + (side(0.8) - 0.5)) / 2.0;
        let a = prism("the prism with a falling side", chain, pose);
        (a, 4.0 - CORNER - 0.36, strip + 0.1 - CORNER)
    }
    beside_the_junction(make, |_, op| {
        (op == "B ∖ A").then_some(a_tangent_parting as Refusal)
    });
}
