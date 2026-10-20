//! **A tangent arc whose target is nearly behind the tip keeps its end
//! vertex.** `tangent_arc_to` and `arc_onto` share one kernel, which
//! stores the sweep as `4·atan(X)`, X the half-angle tangent of the
//! tip's departure against the chord. As the target goes behind the
//! tip the turn nears 2π, and the
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
    ArcData, ConstructedLoop, ConstructedProfile, PathError, ProfileError, ReplayError,
    ReplayErrorKind, Segment, SketchPlane, Step, Target,
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

/// The `f64` replay of `steps`, or `None` where the door's read-back
/// refused it on a scene that cannot resolve the read: an arc of radius
/// `r`, whose read's rounding at that magnitude (16 ulps for its
/// landing, 8 for a `Via` arc's through-point) reaches the band,
/// refuses loudly as the scene-resolution sibling or escalates in the
/// band, naming the read. Any other refusal, or either of those on a
/// scene that resolves the read, fails the row.
///
/// Which rows refuse is pinned at the ε rows the gate runs
/// ([`REFUSES_PAST_RESOLUTION`]); at any other ε either answer stands.
fn f64_replay_or_unresolvable(
    steps: &[Step<f64>],
    r: f64,
    what: &str,
) -> Option<ConstructedLoop<f64>> {
    let eps = tol().get().eps;
    let pinned = REFUSES_PAST_RESOLUTION
        .iter()
        .find(|(e, _)| *e == eps)
        .map(|(_, rows)| rows.contains(&what));
    let refusal = match try_replay_at::<f64>(steps) {
        Ok(lp) => {
            assert_ne!(
                pinned,
                Some(true),
                "{what}: pinned to refuse at ε = {eps:e}, built"
            );
            return Some(lp);
        }
        Err(e) => e,
    };
    assert_ne!(
        pinned,
        Some(false),
        "{what}: pinned to build at ε = {eps:e}, refused: {refusal}"
    );
    let (eps, k) = (tol().get().eps, tol().get().k);
    let ulps = |read: &str| {
        if read.starts_with("path_arc_through_point")
            || read.starts_with("path_arc_start_on_carrier")
        {
            Some(profile::READ_BACK_RIM_ULPS)
        } else if read.starts_with("path_arc_landing") || read.starts_with("path_arc_branch") {
            Some(profile::READ_BACK_LANDING_ULPS)
        } else {
            None
        }
    };
    let unresolvable =
        |read: &str, scale: f64| ulps(read).is_some_and(|u| u * scale * ULP >= k * eps);
    let answered = match &refusal.kind {
        ReplayErrorKind::Path(PathError::ArcMissBelowSceneResolution {
            predicate, scale, ..
        }) => unresolvable(predicate, *scale),
        ReplayErrorKind::Path(PathError::Escalated { source }) => {
            source.predicate.is_some_and(|read| unresolvable(read, r))
        }
        _ => false,
    };
    assert!(answered, "{what}: f64 replay refused: {refusal}");
    None
}

/// **The rows whose `f64` replay the door refuses, per ε row the gate
/// runs**: each a sound arc on a scene that cannot resolve the door's
/// read of it. Every row not listed builds at that ε.
const REFUSES_PAST_RESOLUTION: &[(f64, &[&str])] = &[
    (
        1e-9,
        &[
            "Via, R 1.37e7",
            "Via, R 1.37e8",
            "arc_onto, R 1.37e7",
            "arc_onto, R 1.37e8",
            "tangent_arc_to, R 1.37e7",
            "tangent_arc_to, R 1.37e8",
        ],
    ),
    (1e-6, &[]),
    (
        1e-12,
        &[
            "Center, R 1.37e5",
            "Center, R 1.37e6",
            "Center, R 1e6, turn 1e-4",
            "Via, R 1.36e4",
            "Via, R 1.37e7",
            "Via, R 1.37e8",
            "Via, R 1e4, turn 1e-2",
            "Via, R 1e4, turn 1e-3",
            "Via, R 1e6, turn 1e-4",
            "arc_onto, R 1.37e7",
            "arc_onto, R 1.37e8",
            "tangent_arc_to, R 1.37e7",
            "tangent_arc_to, R 1.37e8",
        ],
    ),
];

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

/// A replay's refusal on the departure line: the chord a
/// `DegenerateArcChord` reports (its enclosure's ends), and whether the replay escalates on
/// `path_collinear_target` instead; the replay's own answer when it is
/// anything else.
fn departure_line_refusal<T: profile::ArcCarrierScalar>(
    steps: &[Step<f64>],
) -> Result<(Option<(f64, f64)>, bool), String> {
    match try_replay_at::<T>(steps) {
        Err(ReplayError {
            kind: ReplayErrorKind::Path(e),
            ..
        }) => {
            let chord = match &e {
                PathError::DegenerateArcChord { chord } => Some((chord.lo(), chord.hi())),
                _ => None,
            };
            let undecided = matches!(&e, PathError::Escalated { source }
                if source.predicate == Some("path_collinear_target"));
            Ok((chord, undecided))
        }
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
/// quarter-tangent missed by 3.4e-8, 4.4e-2 and the whole chord), and
/// where the scene cannot resolve the door's landing read the replay
/// refuses on that read instead of storing the arc; at
/// `Interval` its sweep is an enclosure ~1e-14 wide whose landing holds
/// the vertex (it was 2e-11 wide, then `[−2π, 2π]`). Validation answers
/// the clearance rule: at the default ε the 1.37e8 arc is short of a
/// full circle by 3.3e-9 m, inside the band, and escalates on that
/// rather than reading `NearFullArc` wherever the replay stores it; at
/// `f64` the 1.36e4 arc, clear and inside the scene's resolution,
/// validates.
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
                    // On the line the refusal reports the chord the ends
                    // span, not a tolerance-sized one.
                    let answered = match &refusal {
                        Ok((Some((lo, hi)), _)) => {
                            p.y <= eps && *lo <= 2.7 + 1e-9 && 2.7 <= *hi + 1e-9
                        }
                        Ok((None, undecided)) => p.y > eps && *undecided,
                        Err(_) => false,
                    };
                    assert!(
                        answered,
                        "{what}: a target {:e} m off the departure line, behind the tip, \
                         refuses {refusal:?}",
                        p.y
                    );
                }
                continue;
            }
            if let Some(lp) = f64_replay_or_unresolvable(&steps, r, &what) {
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

/// **The through-point arc near a full turn**, the `Via` mode's
/// quarter-tangent read off the same kind of split: the carrier tangent to `+x` at the
/// origin with radius `r`, through its top `(0, 2r)`, to the point 2.7 m
/// of arc short of the full turn, and home by way of a vertex below
/// the chord. The stored arc lands on its end vertex within 8 ulps of
/// its radius at `f64` where the scene resolves its through-point read,
/// and past that the door refuses on that read; its `Interval` landing
/// holds the vertex.
#[test]
fn the_through_point_arc_near_a_full_turn_keeps_its_end_vertex() {
    let a = Point2::new(0.0, 0.0);
    for r in [1.36e4, 1.37e7, 1.37e8] {
        let turn = core::f64::consts::TAU - 2.7 / r;
        let b = Point2::new(r * turn.sin(), r * (1.0 - turn.cos()));
        let steps = vec![
            Step::At(a),
            Step::ArcTo(ArcData::Via {
                q: Point2::new(0.0, 2.0 * r),
                target: Target::Point(b),
            }),
            Step::LineTo(Target::Point(Point2::new(-1.35, -5.0))),
            Step::LineTo(Target::Start),
        ];
        let what = format!("Via, R {r:e}");
        // The through-point is the read the cancelling quarter-tangent
        // moves; its landing stays within 8 ulps whatever X is.
        if let Some(lp) = f64_replay_or_unresolvable(&steps, r, &what) {
            let miss = f64_miss(&lp, &what);
            assert!(
                miss <= 8.0 * r * ULP,
                "{what}: the stored arc misses its end vertex by {miss:e} m, past 8 ulps of its \
                 radius"
            );
        }
        let li = try_replay_at::<Interval>(&steps)
            .unwrap_or_else(|e| panic!("{what}: Interval replay refused: {e}"));
        let (holds, width) = interval_landing(&li, &what);
        assert!(holds, "{what}: the Interval landing misses its end vertex");
        assert!(
            width <= 64.0 * r * ULP,
            "{what}: the Interval landing is {width:e} m wide"
        );
    }
}

/// **On the departure line, by both verbs, at every ε row**: a target
/// a quarter-ε off the line 2.7 m behind the tip refuses
/// `DegenerateArcChord` reporting the 2.7 m its ends span; one a quarter-ε from the tip refuses the same
/// kind reporting a chord under ε.
#[test]
fn on_the_departure_line_the_refusal_reports_the_chord_its_ends_span() {
    let eps = tol().get().eps;
    let a = Point2::new(0.0, 0.0);
    for (verb, build) in [
        ("tangent_arc_to", forward as fn(_, _, _) -> Vec<Step<f64>>),
        ("arc_onto", onto),
    ] {
        let behind = build(a, (1.0, 0.0), Point2::new(-2.7, eps / 4.0));
        let at_tip = build(a, (1.0, 0.0), Point2::new(eps / 4.0, eps / 4.0));
        let what = format!("{verb} at eps {eps:e}");
        for (scalar, behind, at_tip) in [
            (
                "f64",
                departure_line_refusal::<f64>(&behind),
                departure_line_refusal::<f64>(&at_tip),
            ),
            (
                "Interval",
                departure_line_refusal::<Interval>(&behind),
                departure_line_refusal::<Interval>(&at_tip),
            ),
        ] {
            assert!(
                matches!(behind, Ok((Some((lo, hi)), false)) if lo <= 2.7 + 1e-9 && 2.7 <= hi + 1e-9),
                "{what}, {scalar}: a target behind the tip on the line refuses {behind:?}"
            );
            assert!(
                matches!(at_tip, Ok((Some((_, hi)), false)) if hi <= eps),
                "{what}, {scalar}: a target at the tip refuses {at_tip:?}"
            );
        }
    }
}

/// **An arc about an authored centre near a full turn** keeps its end
/// vertex: the carrier of radius `r` about `(0, r)`, counter-clockwise
/// from the origin to the point 2.7 m of arc short of the full turn,
/// home by way of a vertex below the chord. At `f64` it lands within 8
/// ulps of its radius of the end vertex (the radius-plus-apothem
/// quarter-tangent missed by 2.5e-8, 5.0e-6 and 2.1e-4, and validated)
/// where the scene resolves the door's landing read, and refuses on
/// that read where it does not; its `Interval` landing holds the
/// vertex, and its sweep stays under 1e-13 wide.
#[test]
fn the_centre_arc_near_a_full_turn_keeps_its_end_vertex() {
    let a = Point2::new(0.0, 0.0);
    for r in [1.36e4, 1.37e5, 1.37e6] {
        let turn = core::f64::consts::TAU - 2.7 / r;
        let b = Point2::new(r * turn.sin(), r * (1.0 - turn.cos()));
        let steps = vec![
            Step::At(a),
            Step::ArcTo(ArcData::Center {
                c: Point2::new(0.0, r),
                winding: profile::ArcSweep::Ccw,
                target: Target::Point(b),
            }),
            Step::LineTo(Target::Point(Point2::new(-1.35, -5.0))),
            Step::LineTo(Target::Start),
        ];
        let what = format!("Center, R {r:e}");
        if let Some(lp) = f64_replay_or_unresolvable(&steps, r, &what) {
            let miss = f64_miss(&lp, &what);
            assert!(
                miss <= 8.0 * r * ULP,
                "{what}: the stored arc misses its end vertex by {miss:e} m, past 8 ulps of \
                 its radius"
            );
        }
        // Past the scene's resolution the Interval lane cannot read the
        // authored centre's equidistance, and says so.
        let li = match try_replay_at::<Interval>(&steps) {
            Ok(li) => li,
            Err(ReplayError {
                kind: ReplayErrorKind::Path(PathError::Escalated { source }),
                ..
            }) if source.predicate == Some("path_arc_center_equidistant")
                && 64.0 * r * ULP > tol().get().eps =>
            {
                continue;
            }
            Err(e) => panic!("{what}: Interval replay refused: {e}"),
        };
        let sweep = first_arc(&li, &what).sweep;
        assert!(
            sweep.hi() - sweep.lo() <= 1e-13,
            "{what}: the Interval sweep [{:e}, {:e}] is wider than 1e-13",
            sweep.lo(),
            sweep.hi()
        );
        // The end vertex sits off the carrier by its own rounding, so the
        // landing is read against it at the f64 row's 8 ulps of R.
        let v = li.vertices();
        let end = first_arc(&li, &what).landing(v[0]);
        let near = |e: Interval, t: Interval| {
            e.lo() - 8.0 * r * ULP <= t.lo() && t.hi() <= e.hi() + 8.0 * r * ULP
        };
        assert!(
            near(end.x, v[1].x) && near(end.y, v[1].y),
            "{what}: the Interval landing is not within 8 ulps of R of its end vertex"
        );
    }
}

/// **Short arcs, by every mode that reads the home**: a tangent arc, an
/// arc through a point and an arc about a centre, each turning `θ` from
/// 1e-4 to 1e-2 on a carrier of radius `r`, its sagitta well clear of
/// the band. Each stored arc lands on its end vertex within 8 ulps of its
/// radius at `f64`, where the scene resolves the door's read of it. Here `along` is near `len`, where the home's second
/// arm `(len − along)/across` cancels: read alone, it misses by about
/// `r·θ·2⁻⁵²·8/θ²`, a thousand times the bound and more.
#[test]
fn short_arcs_keep_their_end_vertex_by_every_mode() {
    for (r, theta) in [(1e4, 1e-3), (1e4, 1e-2), (1e6, 1e-4)] {
        let a = Point2::new(0.0, 0.0);
        let at = |t: f64| Point2::new(r * t.sin(), r * (1.0 - t.cos()));
        let b = at(theta);
        let home = Point2::new(0.5 * b.x, -r * theta);
        for (mode, leg) in [
            ("tangent", None),
            (
                "Via",
                Some(ArcData::Via {
                    q: at(0.5 * theta),
                    target: Target::Point(b),
                }),
            ),
            (
                "Center",
                Some(ArcData::Center {
                    c: Point2::new(0.0, r),
                    winding: profile::ArcSweep::Ccw,
                    target: Target::Point(b),
                }),
            ),
        ] {
            let steps = match leg {
                None => vec![
                    Step::At(a),
                    Step::Toward { dx: 1.0, dy: 0.0 },
                    Step::TangentArcTo(Target::Point(b)),
                    Step::LineTo(Target::Point(home)),
                    Step::LineTo(Target::Start),
                ],
                Some(leg) => vec![
                    Step::At(a),
                    Step::ArcTo(leg),
                    Step::LineTo(Target::Point(home)),
                    Step::LineTo(Target::Start),
                ],
            };
            let what = format!("{mode}, R {r:e}, turn {theta:e}");
            if let Some(lp) = f64_replay_or_unresolvable(&steps, r, &what) {
                let miss = f64_miss(&lp, &what);
                assert!(
                    miss <= 8.0 * r * ULP,
                    "{what}: the stored arc misses its end vertex by {miss:e} m, past 8 ulps \
                     of its radius"
                );
            }
        }
    }
}
