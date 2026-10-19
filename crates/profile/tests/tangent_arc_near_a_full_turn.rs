//! **A tangent arc whose target is nearly behind the tip keeps its end
//! vertex.** `tangent_arc_to` and `arc_onto` share one kernel, which
//! stores the sweep as 4·atan(X) with X the quarter-tangent of the
//! turn. As the target goes behind the tip the turn nears 2π, and the
//! stored arc has to land on its own end vertex there as it does
//! anywhere: within the scene's f64 resolution at `f64`, and inside an
//! enclosure that stays narrow at `Interval`.
//!
//! Where such a loop is refused, the refusal answers the clearance
//! rule: an arc short of a full circle by at most ε is `NearFullArc`,
//! one short by a clearance inside the band escalates on
//! `arc_diameter_clearance`, and one clear of the band is refused for
//! neither.
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/seg.rs",
    "crates/profile/tests/common/",
];

use crate::common::{tol, try_replay_at};
use geom_core::{Arc2, Bounds, Interval, Point2, Real};
use profile::{
    ConstructedLoop, ConstructedProfile, PathError, ProfileError, ReplayError, ReplayErrorKind,
    Segment, SketchPlane, Step, Target,
};

const ULP: f64 = f64::EPSILON;

/// The loop the row measures: a tip at `a` heading along `(cx, cy)`,
/// the tangent arc to `p`, and the chord home.
fn forward(a: Point2<f64>, (cx, cy): (f64, f64), p: Point2<f64>) -> Vec<Step<f64>> {
    vec![
        Step::At(a),
        Step::Toward { dx: cx, dy: cy },
        Step::TangentArcTo(Target::Point(p)),
        Step::LineTo(Target::Start),
    ]
}

/// The same arc authored backwards: from `p` onto `a`, arriving along
/// the reversed director, then home — `forward`'s arc, retraced.
fn onto(a: Point2<f64>, (cx, cy): (f64, f64), p: Point2<f64>) -> Vec<Step<f64>> {
    vec![
        Step::At(p),
        Step::ArcOnto(Target::Point(a)),
        Step::Toward { dx: -cx, dy: -cy },
        Step::LineTo(Target::Start),
    ]
}

fn first_arc<T: Real>(lp: &ConstructedLoop<T>, what: &str) -> Arc2<T> {
    match lp.segments()[0] {
        Segment::Arc(arc) => arc,
        Segment::Line => panic!("{what}: the leg is stored as a line"),
    }
}

/// How far the stored first arc, landed from its own start vertex,
/// ends from the next vertex.
fn f64_miss(lp: &ConstructedLoop<f64>, what: &str) -> f64 {
    let v = lp.vertices();
    (first_arc(lp, what).landing(v[0]) - v[1]).norm()
}

/// Whether the `Interval` first arc, landed from its start vertex,
/// holds the next vertex, and the wider side of that landing.
fn interval_landing(lp: &ConstructedLoop<Interval>, what: &str) -> (bool, f64) {
    let v = lp.vertices();
    let end = first_arc(lp, what).landing(v[0]);
    let holds = |e: Interval, t: Interval| e.lo() <= t.lo() && t.hi() <= e.hi();
    let width = (end.x.hi() - end.x.lo()).max(end.y.hi() - end.y.lo());
    (holds(end.x, v[1].x) && holds(end.y, v[1].y), width)
}

/// The tip at the origin heading `+x`, the target at chord 2.7 behind
/// it, offset so the carrier's radius is `r`: the row's probe.
fn behind_at_radius(r: f64) -> Point2<f64> {
    Point2::new(-2.7, 7.29 / (2.0 * r))
}

/// The clearance rule's three answers.
#[derive(Debug, PartialEq)]
enum Clearance {
    NearFull,
    Undecided,
    Clear,
}

/// The clearance `2r(1 − sin(|Δθ|/4)) = 4r·sin²(gap/8)` of an arc of
/// radius `r` short of the full turn by `gap`, against the run's band.
fn clearance_class(r: f64, gap: f64) -> (Clearance, f64) {
    let clearance = 4.0 * r * (gap / 8.0).sin().powi(2);
    let (eps, k) = (tol().get().eps, tol().get().k);
    let class = if clearance <= eps {
        Clearance::NearFull
    } else if clearance < k * eps {
        Clearance::Undecided
    } else {
        Clearance::Clear
    };
    (class, clearance)
}

/// Whether a refusal reads the clearance as `NearFullArc`, and whether
/// it escalates on it.
fn clearance_reading(result: &Result<(), ProfileError>) -> (bool, bool) {
    let near_full = matches!(result, Err(ProfileError::NearFullArc(_)));
    let undecided = matches!(
        result,
        Err(ProfileError::Escalated { source, .. })
            if source.predicate == Some("arc_diameter_clearance")
    );
    (near_full, undecided)
}

/// A replay's refusal on the departure line: whether it is
/// `DegenerateArcChord` naming, and saying, the target behind the tip,
/// and whether
/// it escalates on `path_collinear_target`; the replay's own answer
/// when it is anything else.
fn departure_line_refusal<T: profile::ArcCarrierScalar>(
    steps: &[Step<f64>],
) -> Result<(bool, bool), String> {
    match try_replay_at::<T>(steps) {
        Err(ReplayError {
            kind: ReplayErrorKind::Path(e),
            ..
        }) => Ok((
            matches!(e, PathError::DegenerateArcChord { behind: true, .. })
                && e.to_string()
                    .contains("on the departure line behind the tip"),
            matches!(&e, PathError::Escalated { source }
                if source.predicate == Some("path_collinear_target")),
        )),
        Err(e) => Err(e.to_string()),
        Ok(_) => Err("the replay builds".to_owned()),
    }
}

fn validated<T: profile::ArcCarrierScalar>(lp: &ConstructedLoop<T>) -> Result<(), ProfileError> {
    ConstructedProfile::new(SketchPlane::xy(), vec![lp.clone()])
        .validate(tol())
        .map(|_| ())
}

/// **The row's three radii**, by both verbs. At `f64` the stored arc
/// lands within 8 ulps of its radius of its end vertex (the cancelling
/// quarter-tangent missed by 3.4e-8, 4.4e-2 and the whole chord); at
/// `Interval` its sweep is an enclosure ~1e-14 wide whose landing holds
/// the vertex (it was 2e-11 wide, then `[−2π, 2π]`). Validation answers
/// the clearance rule: at the default ε the 1.37e8 arc is short of a
/// full circle by 3.3e-9 m, inside the band, and escalates on that
/// rather than reading `NearFullArc`; at `f64` the 1.36e4 arc, clear
/// and inside the scene's resolution, validates.
#[test]
fn the_rows_three_radii_keep_their_end_vertex() {
    let a = Point2::new(0.0, 0.0);
    let (eps, k) = (tol().get().eps, tol().get().k);
    for r in [1.36e4, 1.37e7, 1.37e8] {
        let p = behind_at_radius(r);
        let gap = 2.0 * p.y.atan2(-p.x);
        let (class, clearance) = clearance_class(r, gap);
        for (verb, steps) in [
            ("tangent_arc_to", forward(a, (1.0, 0.0), p)),
            ("arc_onto", onto(a, (1.0, 0.0), p)),
        ] {
            let what = format!("{verb}, R {r:e}");
            // `p.y` is the target's offset from the departure line.
            if p.y < k * eps {
                for refusal in [
                    departure_line_refusal::<f64>(&steps),
                    departure_line_refusal::<Interval>(&steps),
                ] {
                    let (behind, undecided) = refusal.as_ref().map_or((false, false), |&pair| pair);
                    assert!(
                        if p.y <= eps { behind } else { undecided },
                        "{what}: a target {:e} m off the departure line, behind the tip, \
                         refuses {refusal:?}",
                        p.y
                    );
                }
                continue;
            }
            let lp = try_replay_at::<f64>(&steps)
                .unwrap_or_else(|e| panic!("{what}: f64 replay refused: {e}"));
            let miss = f64_miss(&lp, &what);
            assert!(
                miss <= 8.0 * r * ULP,
                "{what}: the stored arc misses its end vertex by {miss:e} m, past 8 ulps \
                 of its radius"
            );
            let result = validated(&lp);
            let (near_full, undecided) = clearance_reading(&result);
            let answered = match class {
                Clearance::NearFull => near_full,
                Clearance::Undecided => undecided,
                Clearance::Clear => !near_full && !undecided,
            };
            assert!(
                answered,
                "{what}: f64 validation {result:?} does not answer the clearance \
                 {clearance:e} m ({class:?})"
            );
            if class == Clearance::Clear && 64.0 * r * ULP <= eps {
                assert!(result.is_ok(), "{what}: f64 validation refused: {result:?}");
            }

            let li = try_replay_at::<Interval>(&steps)
                .unwrap_or_else(|e| panic!("{what}: Interval replay refused: {e}"));
            let sweep = first_arc(&li, &what).sweep;
            assert!(
                sweep.hi() - sweep.lo() <= 1e-13,
                "{what}: the Interval sweep [{:e}, {:e}] is wider than 1e-13",
                sweep.lo(),
                sweep.hi()
            );
            let (holds, width) = interval_landing(&li, &what);
            assert!(holds, "{what}: the Interval landing misses its end vertex");
            assert!(
                width <= 64.0 * r * ULP,
                "{what}: the Interval landing is {width:e} m wide"
            );
            // An enclosure at the band's edge may read either side of it.
            let iv = validated(&li);
            let (near_full, undecided) = clearance_reading(&iv);
            let answered = match class {
                Clearance::NearFull => near_full || undecided,
                Clearance::Undecided => undecided,
                Clearance::Clear => !near_full && !undecided,
            };
            assert!(
                answered,
                "{what}: Interval validation {:?} does not answer the clearance \
                 {clearance:e} m ({class:?})",
                iv.err().map(|e| e.to_string())
            );
        }
    }
}

/// **A counterexample search over near-behind targets**: a random
/// director, tip, radius and gap short of the full turn, in both
/// windings, by both verbs. The radius and the tip stay where 64 ulps
/// of the coordinates are under ε, and the target's offset from the
/// departure line clears the band, so the stored arc owes its end
/// vertex to within ε at `f64`, and at `Interval` a landing that holds
/// the vertex and is no wider than ε.
#[test]
fn near_behind_targets_land_on_their_end_vertex_at_both_scalars() {
    let mut rng = test_utils::fuzz::start("tangent_arc_near_a_full_turn");
    let (eps, k) = (tol().get().eps, tol().get().k);
    let reach = eps / (64.0 * ULP);
    for _ in 0..test_utils::fuzz::scaled(60) {
        let heading = rng.range(0.0, core::f64::consts::TAU);
        let director = (3.0 * heading.cos(), 3.0 * heading.sin());
        let r = rng.log_uniform(reach * 1e-6, reach / 4.0);
        let a = Point2::new(
            rng.range(-reach, reach) / 4.0,
            rng.range(-reach, reach) / 4.0,
        );
        // The offset across the departure line is about r·gap²/2.
        let gap = rng.log_uniform((8.0 * k * eps / r).sqrt(), 0.5);
        let winding = if rng.below(2) == 0 { 1.0 } else { -1.0 };
        let n = director.0.hypot(director.1);
        let (ux, uy) = (director.0 / n, director.1 / n);
        let turn = core::f64::consts::TAU - gap;
        let (s, c1) = (turn.sin(), 1.0 - turn.cos());
        let p = Point2::new(
            a.x + r * (s * ux - winding * c1 * uy),
            a.y + r * (s * uy + winding * c1 * ux),
        );
        for (verb, steps) in [
            ("tangent_arc_to", forward(a, director, p)),
            ("arc_onto", onto(a, director, p)),
        ] {
            let what = format!(
                "{verb} from {a:?} along {director:?} to {p:?} (R {r:e}, gap {gap:e}); {}",
                test_utils::fuzz::replay()
            );
            let lp = try_replay_at::<f64>(&steps)
                .unwrap_or_else(|e| panic!("{what}: f64 replay refused: {e}"));
            let miss = f64_miss(&lp, &what);
            assert!(
                miss <= eps,
                "{what}: the stored arc misses its end vertex by {miss:e} m"
            );
            let li = try_replay_at::<Interval>(&steps)
                .unwrap_or_else(|e| panic!("{what}: Interval replay refused: {e}"));
            let (holds, width) = interval_landing(&li, &what);
            assert!(holds, "{what}: the Interval landing misses its end vertex");
            assert!(
                width <= eps,
                "{what}: the Interval landing is {width:e} m wide"
            );
        }
    }
}
