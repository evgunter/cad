//! **A target on the departure ray is the straight segment, at both
//! scalars.** `tangent_arc_to` and `arc_onto` share one kernel, which
//! DECIDES whether the target lies on the departure line. A target
//! authored on an oblique director's ray sits a sub-band `across` off
//! the rounded unit ray, so the line has to come from that decision
//! rather than from an exact zero — an arc built off the residue is a
//! ~1e16 carrier at f64 and an unbounded enclosure at Interval.
//!
//! A counterexample search over random directors and leg lengths: at
//! f64 the leg is stored as a line and the triangle validates; at
//! Interval it is a line too, or the reading escalates.
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/onto.rs",
    "crates/profile/src/path/program.rs",
    "crates/profile/tests/common/",
];

use crate::common::{tol, try_replay_at};
use geom_core::{Interval, Point2};
use profile::{
    ConstructedLoop, ConstructedProfile, PathErrorKind, ReplayError, ReplayErrorKind, Segment,
    SketchPlane, Step, Target,
};

/// Whether segment 0 of `steps`' loop is the straight segment, at `T`;
/// `None` when the replay escalated.
fn first_leg_is_a_line<T: profile::ArcCarrierScalar>(
    steps: &[Step<f64>],
) -> Result<Option<bool>, String> {
    match try_replay_at::<T>(steps) {
        Ok(lp) => {
            validates(&lp)?;
            Ok(Some(matches!(lp.segments()[0], Segment::Line)))
        }
        Err(ReplayError {
            kind: ReplayErrorKind::Path(e),
            ..
        }) if e.kind() == PathErrorKind::Escalated => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn validates<T: profile::ArcCarrierScalar>(lp: &ConstructedLoop<T>) -> Result<(), String> {
    ConstructedProfile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(tol())
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[test]
fn a_target_on_an_oblique_departure_ray_is_a_line_at_both_scalars() {
    let mut rng = test_utils::fuzz::start("tangent_arc_on_the_ray");
    for _ in 0..test_utils::fuzz::scaled(200) {
        let theta = rng.range(0.0, core::f64::consts::TAU);
        let scale = rng.log_uniform(0.1, 10.0);
        let (dx, dy) = (scale * theta.cos(), scale * theta.sin());
        let len = rng.log_uniform(0.01, 100.0);
        let n = (dx * dx + dy * dy).sqrt();
        let p = Point2::new(len * dx / n, len * dy / n);
        // A right triangle: the leg on the ray, a quarter turn, home.
        let q = Point2::new(p.x - len * dy / n, p.y + len * dx / n);
        let tail = [Step::LineTo(Target::Point(q)), Step::LineTo(Target::Start)];
        let at = Step::At(Point2::new(0.0, 0.0));
        let toward = Step::Toward { dx, dy };
        for (verb, steps) in [
            (
                "tangent_arc_to",
                [at, toward, Step::TangentArcTo(Target::Point(p))],
            ),
            ("arc_onto", [at, Step::ArcOnto(Target::Point(p)), toward]),
        ] {
            let steps = [steps.as_slice(), &tail].concat();
            let what = format!(
                "{verb} along ({dx:e}, {dy:e}) to {p:?}; {}",
                test_utils::fuzz::replay()
            );
            match first_leg_is_a_line::<f64>(&steps) {
                Ok(Some(true)) => {}
                other => panic!("{what}, f64: {other:?}, not the stored straight segment"),
            }
            match first_leg_is_a_line::<Interval>(&steps) {
                Ok(Some(true) | None) => {}
                other => panic!("{what}, Interval: {other:?}, neither a line nor an escalation"),
            }
        }
    }
}
