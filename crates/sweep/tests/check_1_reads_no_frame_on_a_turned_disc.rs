//! **Check 1 reads no frame on a disc a boxed turn moves**
//! (`work/onehome/check-1-reads-a-turning-frames-length-off-its-bracket.md`,
//! closed by `carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`).
//! A disc of radius 5 mm and depth 5 mm, built at `Interval` on the
//! `xy` plane and moved by a turn about `y` of `30° ± δ`. Its frames
//! are `geom::Frame`s, rigid by their type, and at an enclosure scalar
//! they enclose the rotation they hold, so check 1 has no frame margin
//! to read: the moved disc's finish carries no datum finding at any `δ`
//! from `1°` to `10⁻¹⁰°`. Before, the axis's length read
//! `±r·sin 2θ·δ` off its bracket, first order in the box, and check 1
//! decided only below about `10⁻⁵°`. What stands behind check 1 on the
//! same disc, the edges' and pcurves' readings at first order, is
//! `a-body-moved-by-a-boxed-turn-refuses-its-edges-and-pcurves-at-first-order-in-the-box`;
//! this row reads only check 1. Red where the frame margins are restored,
//! read off the entries.

#![allow(clippy::expect_used)]

use core::f64::consts::TAU;

use geom_core::{Arc2, Interval, Point2, Real, RigidMotion, Tol, UnitVec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::ValidationError;

const RADIUS: f64 = 5e-3;

fn disc() -> topo::Body<Interval> {
    let tol = Tol::witness();
    let c = Interval::from_f64;
    let rim = RawLoop::new([(
        Point2::new(c(RADIUS), c(0.0)),
        Segment::Arc(Arc2 {
            centre: Point2::new(c(0.0), c(0.0)),
            radius: c(RADIUS),
            sweep: c(TAU),
        }),
    )]);
    let profile = Profile::new(SketchPlane::xy(), vec![rim])
        .validate(tol)
        .expect("the rim is a profile");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: c(RADIUS),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the disc extrudes")
    .body
    .into_body()
}

/// Check 1's datum findings: a surface's or an edge carrier's datum
/// outside its convention or undecided against it.
fn check_1(e: &ValidationError) -> bool {
    matches!(
        e,
        ValidationError::UnrepresentableSurfaceDatum { .. }
            | ValidationError::SurfaceDatumUndecided { .. }
            | ValidationError::UnrepresentableCurveDatum { .. }
            | ValidationError::CurveDatumUndecided { .. }
    )
}

#[test]
fn a_disc_moved_by_a_boxed_turn_meets_no_frame_finding_at_check_1() {
    let tol = Tol::witness();
    let flat = disc();
    let theta = 30f64.to_radians();
    for k in 0..=10 {
        let delta = 10f64.powi(-k).to_radians();
        let angle = Interval::from_bounds(theta - delta, theta + delta);
        let turn = RigidMotion::rotation(UnitVec3::<Interval>::exact_y(), angle).expect("a turn");
        let moved = topo::transform_rigid(&flat, &turn).expect("the disc moves");
        let findings: Vec<ValidationError> = match topo::finish(moved, tol) {
            Ok(_) => Vec::new(),
            Err(not) => not
                .as_slice()
                .iter()
                .filter(|e| check_1(e))
                .cloned()
                .collect(),
        };
        assert!(
            findings.is_empty(),
            "δ 1e-{k}°: check 1 reads a datum finding: {findings:?}"
        );
    }
}
