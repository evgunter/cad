//! R1 review E2E — a fused arc-fillet-arc tangency authored HERE, not
//! the rocker eye, driven through replay at both lanes.
//!
//! The unit's fixtures are the rocker eye (carriers about (∓½, 0)
//! through (0, −√3⁄2), radius 1) and the vesica lens. This row authors
//! a different one: carriers about (∓3⁄2, 0) through (0, −2), so the
//! carrier radius is exactly 5/2 — a 3-4-5 triangle, every coordinate
//! and the radius exactly representable, so the squared-radius rule is
//! not merely approximately exact here. The anchor is again one of the
//! two carrier intersections, so the derived corner list contains it
//! bitwise and the incoming advance gate measures a sweep from a point
//! to itself: the class's live shape, at coordinates the unit never ran.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::tol;
use geom_core::{Point2, Real};
use profile::{ArcData, ArcSweep, Center, Open, ProfileLoop, ReplayError, Step, Target, replay};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn embed_step<T: Real>(step: &Step<f64>) -> Step<T> {
    fn pt<T: Real>(p: Point2<f64>) -> Point2<T> {
        Point2::new(T::from_f64(p.x), T::from_f64(p.y))
    }
    fn tgt<T: Real>(t: Target<f64>) -> Target<T> {
        match t {
            Target::Start => Target::Start,
            Target::StartArriving => Target::StartArriving,
            Target::Point(p) => Target::Point(pt(p)),
        }
    }
    fn spec<T: Real>(s: ArcData<f64>) -> ArcData<T> {
        match s {
            ArcData::Radius { r, side } => ArcData::Radius {
                r: T::from_f64(r),
                side,
            },
            ArcData::Bulge { target, b } => ArcData::Bulge {
                target: tgt(target),
                b: T::from_f64(b),
            },
            ArcData::Via { q, target } => ArcData::Via {
                q: pt(q),
                target: tgt(target),
            },
            ArcData::Center { c, winding, target } => ArcData::Center {
                c: pt(c),
                winding,
                target: tgt(target),
            },
            ArcData::Sweep { r, side, angle } => ArcData::Sweep {
                r: T::from_f64(r),
                side,
                angle: T::from_f64(angle),
            },
            ArcData::ArcLen { r, side, len } => ArcData::ArcLen {
                r: T::from_f64(r),
                side,
                len: T::from_f64(len),
            },
        }
    }
    match *step {
        Step::At(p) => Step::At(pt(p)),
        Step::Angle(theta) => Step::Angle(T::from_f64(theta)),
        Step::Toward { dx, dy } => Step::Toward {
            dx: T::from_f64(dx),
            dy: T::from_f64(dy),
        },
        Step::Tangent => Step::Tangent,
        Step::Cusp => Step::Cusp,
        Step::Turn(delta) => Step::Turn(T::from_f64(delta)),
        Step::Line(len) => Step::Line(T::from_f64(len)),
        Step::LineTo(t) => Step::LineTo(tgt(t)),
        Step::ContinueTo(t) => Step::ContinueTo(tgt(t)),
        Step::ArcTo(s) => Step::ArcTo(spec(s)),
        Step::TangentArcTo(t) => Step::TangentArcTo(tgt(t)),
        Step::ArcContinue(p) => Step::ArcContinue(pt(p)),
        Step::Fillet { radius } => Step::Fillet {
            radius: T::from_f64(radius),
        },
        Step::FilletArc { radius, spec: s } => Step::FilletArc {
            radius: T::from_f64(radius),
            spec: spec(s),
        },
        Step::ArcFillet { spec: s, radius } => Step::ArcFillet {
            spec: spec(s),
            radius: T::from_f64(radius),
        },
        Step::ArcFilletArc {
            spec: s,
            radius,
            spec2,
        } => Step::ArcFilletArc {
            spec: spec(s),
            radius: T::from_f64(radius),
            spec2: spec(spec2),
        },
        Step::FarEndTo(p) => Step::FarEndTo(pt(p)),
        Step::CloseTo => Step::CloseTo,
        Step::Circle { centre, radius } => Step::Circle {
            centre: pt(centre),
            radius: T::from_f64(radius),
        },
        Step::CircleSplit {
            centre,
            radius,
            n,
            phase,
        } => Step::CircleSplit {
            centre: pt(centre),
            radius: T::from_f64(radius),
            n,
            phase: T::from_f64(phase),
        },
    }
}

/// My own fused tangency: 3-4-5 carriers, fillet radius 2/5.
fn my_eye() -> Vec<Step<f64>> {
    let loop_ = Open
        .arc_fillet_arc(
            Center {
                c: p2(-1.5, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -2.0),
            },
            0.4,
            Center {
                c: p2(1.5, 0.0),
                winding: ArcSweep::Ccw,
                p: profile::Start,
            },
            tol(),
        )
        .unwrap();
    loop_.program.clone()
}

fn try_replay_at<T: profile::ArcCarrierScalar>(
    program: &[Step<f64>],
) -> Result<ProfileLoop<T>, ReplayError<T>> {
    let embedded: Vec<Step<T>> = program.iter().map(embed_step).collect();
    replay(&embedded, tol())
}

/// The f64 lane: my tangency replays, and the anchor-coincident corner
/// is discarded exactly as the eye's is.
#[test]
fn cert4r1_my_fused_tangency_replays_at_f64() {
    let prog = my_eye();
    let l = try_replay_at::<f64>(&prog).expect("the f64 lane must serve this profile");
    assert!(l.vertices().len() >= 2, "a fused corner emits a real loop");
}

/// The interval lane: same profile, and every emitted enclosure is a
/// hairline rather than a period. This is the consumer-visible claim —
/// before the unit, a profile of this shape came back refused.
#[cfg(feature = "interval")]
#[test]
fn cert4r1_my_fused_tangency_is_input_width_at_interval() {
    use geom_core::{Bounds, Interval};
    let prog = my_eye();
    let iv = try_replay_at::<Interval>(&prog)
        .unwrap_or_else(|e| panic!("the interval lane refused my authored tangency: {e}"));
    let f = try_replay_at::<f64>(&prog).unwrap();
    let mut widest = 0.0f64;
    for (k, (a, b)) in iv.vertices().iter().zip(f.vertices()).enumerate() {
        for (what, enc, exact) in [
            ("x", a.pos().x, b.pos().x),
            ("y", a.pos().y, b.pos().y),
            ("bulge", a.bulge(), b.bulge()),
        ] {
            let w = enc.hi() - enc.lo();
            widest = widest.max(w);
            assert!(
                enc.lo() <= exact && exact <= enc.hi(),
                "vertex {k}'s {what} enclosure [{}, {}] excludes the f64 answer {exact}",
                enc.lo(),
                enc.hi()
            );
            assert!(
                w <= 1e-12,
                "vertex {k}'s {what} enclosure is {w:e} wide — period-width, not input-width"
            );
        }
    }
    // Reported, not asserted as a band: what a consumer actually sees.
    println!("cert4r1: widest enclosure on my authored tangency = {widest:e}");
    assert!(widest > 0.0, "not all degenerate");
}

/// **What the unit's fixtures were too friendly to show.** The eye and
/// the vesica are unit-scale, and the PR reports "~1e-16 against a
/// 1e-12 ceiling". The enclosure width of a fused tangency scales with
/// the profile, so the headroom under that ceiling is a property of the
/// fixtures' size, not of the fix. This row walks the same construction
/// over four decades of scale and REPORTS the widths.
#[cfg(feature = "interval")]
#[test]
fn cert4r1_the_enclosure_width_scales_with_the_profile() {
    use geom_core::{Bounds, Interval};
    fn eye_at(s: f64) -> Vec<Step<f64>> {
        Open.arc_fillet_arc(
            Center {
                c: p2(-1.5 * s, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -2.0 * s),
            },
            0.4 * s,
            Center {
                c: p2(1.5 * s, 0.0),
                winding: ArcSweep::Ccw,
                p: profile::Start,
            },
            tol(),
        )
        .map(|l| l.program.clone())
        .unwrap_or_default()
    }
    for s in [1.0f64, 10.0, 100.0, 1000.0, 10_000.0] {
        let prog = eye_at(s);
        if prog.is_empty() {
            println!("cert4r1 scale {s:>9}: not authorable (a lever refusal, not a fold)");
            continue;
        }
        match try_replay_at::<Interval>(&prog) {
            Ok(iv) => {
                let mut widest = 0.0f64;
                for v in iv.vertices() {
                    for enc in [v.pos().x, v.pos().y, v.bulge()] {
                        widest = widest.max(enc.hi() - enc.lo());
                    }
                }
                println!("cert4r1 scale {s:>9}: widest enclosure {widest:e}");
            }
            Err(e) => println!("cert4r1 scale {s:>9}: REFUSED — {e}"),
        }
    }
}
