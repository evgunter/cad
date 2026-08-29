//! **The replay driver at scalars other than `f64`.**
//!
//! [`profile::replay`] is generic over `ArcCarrierScalar` (`Decide +
//! Bounds`) and type-checks at every evaluation scalar, but the whole
//! shipped corpus instantiates it at `f64` alone. A generic path with
//! one instantiation is a compile-time claim, not an exercised one:
//! every `from_f64`, every band read, every `Bounds` read inside the
//! fused fillet family is unmeasured off the value lane until something
//! runs there.
//!
//! These rows run the verb-coverage corpus — the same closed chains
//! `path_program.rs` uses to prove every declared verb has a replayed
//! arm — through the driver at `Dual64` and at `Interval`, and pin what
//! each lane owes the f64 lane:
//!
//! - **`Dual64`**: the value channel is bit-identical to `f64`. `Dual`
//!   delegates every value operation exactly and a derivative never
//!   influences a branch, so a differing bit is a real divergence, not
//!   a tolerance question. The tangent channel of a constant-seeded
//!   replay is identically zero — which is exactly the seam a profile
//!   parameter lift exists to open, so it is pinned as the STATE, not
//!   as a desideratum.
//! - **`Interval`**: every emitted coordinate and bulge ENCLOSES the
//!   f64 lane's answer. Containment, not equality, is the interval
//!   lane's contract; a lane that certifies a box excluding the f64
//!   answer is unsound.
//!
//! Neither row is a structure-selection claim. The corpus is authored
//! macroscopically, so every discrete decision inside replay decides
//! definitely at both lanes here; the hairline cases where they need
//! not agree are the guided path's business, not this suite's.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{coverage_corpus, tol};
use geom_core::{Dual64, Point2, Real};
use profile::{ArcData, ProfileLoop, ReplayError, Step, Target, replay};

/// Embeds a resolved `f64` step into any scalar, coordinate by
/// coordinate through `Real::from_f64` — the same exact embedding the
/// parameter environment and the profile lift use.
///
/// Exhaustive over the step vocabulary and over the arc-spec modes, so
/// a verb the transition table gains breaks this file at compile rather
/// than silently dropping out of the off-`f64` rows.
fn embed_step<T: Real>(step: &Step<f64>) -> Step<T> {
    fn pt<T: Real>(p: Point2<f64>) -> Point2<T> {
        Point2::new(T::from_f64(p.x), T::from_f64(p.y))
    }
    fn tgt<T: Real>(t: Target<f64>) -> Target<T> {
        match t {
            Target::Start => Target::Start,
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
        Step::Turn(delta) => Step::Turn(T::from_f64(delta)),
        Step::Line(len) => Step::Line(T::from_f64(len)),
        Step::LineTo(t) => Step::LineTo(tgt(t)),
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

/// One corpus row's program, embedded and replayed at `T`.
fn replay_at<T: profile::ArcCarrierScalar>(program: &[Step<f64>]) -> ProfileLoop<T> {
    try_replay_at(program)
        .unwrap_or_else(|e| panic!("the corpus program refused at the lifted scalar: {e}"))
}

fn try_replay_at<T: profile::ArcCarrierScalar>(
    program: &[Step<f64>],
) -> Result<ProfileLoop<T>, ReplayError<T>> {
    let embedded: Vec<Step<T>> = program.iter().map(embed_step).collect();
    replay(&embedded, tol())
}

/// The `Dual64` instantiation: same structure, same value bits, zero
/// tangent.
///
/// The zero tangent is the load-bearing observation, not an incidental
/// one: a constant-seeded replay carries no derivative anywhere, which
/// is precisely why a `Dual` seed on a profile dimension propagates
/// nothing today. This row records that state at the driver.
#[test]
fn the_corpus_replays_at_dual_with_bit_identical_values() {
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let base = replay_at::<f64>(&closed.program);
        let dual = replay_at::<Dual64>(&closed.program);
        assert_eq!(
            base.vertices().len(),
            dual.vertices().len(),
            "row {i}: vertex count"
        );
        for (k, (a, b)) in base.vertices().iter().zip(dual.vertices()).enumerate() {
            assert_eq!(
                a.pos().x.to_bits(),
                b.pos().x.value.to_bits(),
                "row {i} vertex {k}: x value channel"
            );
            assert_eq!(
                a.pos().y.to_bits(),
                b.pos().y.value.to_bits(),
                "row {i} vertex {k}: y value channel"
            );
            assert_eq!(
                a.bulge().to_bits(),
                b.bulge().value.to_bits(),
                "row {i} vertex {k}: bulge value channel"
            );
            for (what, d) in [
                ("x", b.pos().x.deriv),
                ("y", b.pos().y.deriv),
                ("bulge", b.bulge().deriv),
            ] {
                assert_eq!(
                    d, 0.0,
                    "row {i} vertex {k}: {what} tangent — a constant-seeded \
                     replay carries no derivative"
                );
            }
        }
        let (mut want, mut got) = (
            base.tangent_joints().to_vec(),
            dual.tangent_joints().to_vec(),
        );
        want.sort_unstable();
        got.sort_unstable();
        assert_eq!(want, got, "row {i}: declared tangent joints");
    }
}

/// The `Interval` instantiation: the f64 answer lies inside every
/// emitted enclosure, and the declared structure is the same.
///
/// One corpus row does not reach that comparison at all, and the
/// census below is deliberately a census rather than a skip — see
/// [`exactly_one_corpus_row_escalates_at_interval`] for which row, why,
/// and why the wall is the honest answer rather than a defect this
/// suite papers over.
#[cfg(feature = "interval")]
#[test]
fn the_corpus_replays_at_interval_and_encloses_the_f64_lane() {
    use geom_core::{Bounds, Interval};
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let base = replay_at::<f64>(&closed.program);
        let Ok(iv) = try_replay_at::<Interval>(&closed.program) else {
            continue;
        };
        assert_eq!(
            base.vertices().len(),
            iv.vertices().len(),
            "row {i}: vertex count"
        );
        for (k, (a, b)) in base.vertices().iter().zip(iv.vertices()).enumerate() {
            for (what, exact, enc) in [
                ("x", a.pos().x, b.pos().x),
                ("y", a.pos().y, b.pos().y),
                ("bulge", a.bulge(), b.bulge()),
            ] {
                assert!(
                    enc.lo() <= exact && exact <= enc.hi(),
                    "row {i} vertex {k}: the {what} enclosure [{}, {}] excludes the \
                     f64 lane's {exact}",
                    enc.lo(),
                    enc.hi()
                );
            }
        }
        let (mut want, mut got) = (base.tangent_joints().to_vec(), iv.tangent_joints().to_vec());
        want.sort_unstable();
        got.sort_unstable();
        assert_eq!(want, got, "row {i}: declared tangent joints");
    }
}

/// **The finding this suite was written to surface.** Exactly one
/// corpus row cannot replay at `Interval`, and its wall is an
/// ENCLOSURE-QUALITY artifact of the shared angular helpers, not a
/// hairline geometry it would be honest to call ambiguous.
///
/// The row is the rocker eye: one fused `ArcFilletArc` step whose two
/// carriers are circles about (∓½, 0) through (0, −√3⁄2). That anchor
/// is itself one of the pair's two intersections, so the derived corner
/// list contains the anchor exactly — by design, and bitwise, since the
/// squared-radius rule makes a derived corner reproduce an authored
/// one. The incoming advance gate therefore measures the signed swept
/// angle from the anchor TO the anchor: exactly `0.0` at `f64`, which
/// classifies definitely Zero, discards that corner, and keeps the
/// other one. That is the gate working.
///
/// At `Interval` the two angular coordinates are `atan2` enclosures of
/// two separately-rounded points, so their difference straddles zero
/// with nonzero width — and the swept-angle reduction then passes it
/// through TWO successive `floor`-based period reductions (`x mod τ`,
/// then the signed fold `x − τ⌊x/τ + ½⌋`). A box straddling a period
/// boundary makes the first `floor` span an integer, widening the
/// reduction toward the whole period; the second one spans an integer
/// on the widened box in turn. A true value of ~0 comes out as the full
/// `[−τ, τ]`, which no band can classify — hence the escalation.
///
/// Nothing here is fixed by tightening a tolerance, and the obvious
/// reformulation (fold the RAW difference in one reduction, or take
/// `atan2` of the cross/dot) changes the `f64` expression and so the
/// `f64` bits — which the exact-fit rows depend on, by the standing
/// argument in `signed_swept`'s own docs. The wall is therefore
/// recorded and pinned rather than repaired here, and the row is a
/// genuine, corpus-supplied indeterminate case: the shape a guided lane
/// pass must abort on rather than quietly keep the nominal structure.
#[cfg(feature = "interval")]
#[test]
fn exactly_one_corpus_row_escalates_at_interval() {
    use geom_core::Interval;
    use profile::{PathError, ReplayErrorKind, Verb};
    let mut escalated: Vec<(usize, Vec<Verb>)> = Vec::new();
    for (i, closed) in coverage_corpus().into_iter().enumerate() {
        let verbs: Vec<Verb> = closed.program.iter().map(Step::verb).collect();
        let Err(e) = try_replay_at::<Interval>(&closed.program) else {
            continue;
        };
        match e.kind {
            ReplayErrorKind::Path(PathError::Escalated { ref source }) => assert_eq!(
                source.predicate,
                Some("path_corner_advance_arc"),
                "row {i}: the escalation must name the angular advance gate"
            ),
            ref other => panic!("row {i}: unexpected refusal class at Interval: {other:?}"),
        }
        assert_eq!(e.step, 0, "row {i}: the fused step is the whole program");
        escalated.push((i, verbs));
    }
    assert_eq!(
        escalated.len(),
        1,
        "the interval-lane wall is a ONE-row census; it moved: {escalated:?} \
         — a row that joined it is a new instance of the reduction-widening \
         class (see this test's rustdoc), and a row that left it means the \
         angular helpers changed and the finding needs re-stating"
    );
    assert_eq!(
        escalated[0].1,
        vec![Verb::ArcFilletArc],
        "the escalating row is the one-step fused eye"
    );
}
