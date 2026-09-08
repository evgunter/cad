//! BOOL-9 review probes (R1).
//!
//! Two subjects the unit's own rows leave unmeasured:
//!
//! 1. **The materialization door against the two walks it replaced.**
//!    `ProfileLoop::embed` stands in for `loft.rs::end_profile`'s and
//!    `anchor.rs::embed_profile`'s former per-site walks. The unit's
//!    receipt compares `embed` at `f64` with its own input; these rows
//!    compare it with BOTH former walks, restated here verbatim, on a
//!    table the lattice would not author (arcs of both signs, a
//!    semicircle, a signed zero, duplicate and unordered declarations,
//!    then reversed) — at `f64` and, under `--features interval`, at
//!    the interval scalar, where `from_f64` is not the identity.
//! 2. **The widened lift on loops the unit's rows do not reach**: the
//!    all-declared loop at every seam rotation, a two-arc circle with
//!    both joints declared, and a declared closing joint whose
//!    declaration is false by less than the band.
//!
//! **Adopted into the unit's branch with one mechanical re-aim, kept
//! otherwise verbatim (authorship the reviewer's).** The door these rows
//! exercise is spelled `ProfileLoop::map` at the merged head, not
//! `embed`: R2-Q1 ruled that `embed` was a fourth private word for what
//! `Point2`/`Vec2`/`Affine3`/`SketchPlane` all call `map`. Every
//! `src.embed()` here is `src.map(<f64 as Real>::from_f64)` or
//! `src.map(Interval::from_f64)`; nothing else about what these rows
//! measure has changed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Real, Tol};
use profile::{
    Fidelity, LiftOutcome, Open, ProfileLoop, ProfileVertex, RawLoop, Start, Step, Target,
    lift_checked,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A table only the fixture door can spell: arcs of both signs, a
/// semicircle, a signed zero, declarations duplicated and out of
/// order, then the whole thing reversed (so the joints are remapped).
fn awkward() -> ProfileLoop<f64> {
    <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, -0.0), 0.3),
        ProfileVertex::new(p2(2.0, 0.0), -0.5),
        ProfileVertex::new(p2(2.0, 2.0), 1.0),
        ProfileVertex::new(p2(1.0, 3.0), 0.0),
        ProfileVertex::new(p2(0.0, 2.0), 0.123_456_789_012_3),
    ])
    .with_tangent_joints(vec![4, 1, 1, 0])
    .reversed()
}

/// `sweep/src/loft.rs::end_profile`'s walk before the unit, verbatim.
fn loft_walk<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    ProfileLoop::new(
        lp.vertices()
            .iter()
            .map(|v| ProfileVertex::new(v.pos().map(T::from_f64), T::from_f64(v.bulge())))
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// `editor-core/src/eval/anchor.rs::embed_profile`'s walk before the
/// unit, verbatim.
fn anchor_walk<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    ProfileLoop::new(
        lp.vertices()
            .iter()
            .map(|vx| {
                ProfileVertex::new(
                    Point2::new(T::from_f64(vx.pos().x), T::from_f64(vx.pos().y)),
                    T::from_f64(vx.bulge()),
                )
            })
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// The table as text: every scalar through its `Debug`, which for
/// `f64` distinguishes the two zeros and for the interval scalar
/// prints both bounds.
fn table<T: Real + std::fmt::Debug>(lp: &ProfileLoop<T>) -> String {
    let vs: Vec<String> = lp
        .vertices()
        .iter()
        .map(|v| format!("{:?},{:?};{:?}", v.pos().x, v.pos().y, v.bulge()))
        .collect();
    format!("{} | {:?}", vs.join(" "), lp.tangent_joints())
}

// ------------------------------------------------------------------
// The materialization door
// ------------------------------------------------------------------

/// At `f64` the door is both former walks, bit for bit — including
/// the signed zero, the duplicate declaration and the reversed order.
#[test]
fn r1_embed_is_both_former_walks_at_f64_bit_for_bit() {
    let src = awkward();
    assert_eq!(
        src.tangent_joints(),
        [1, 4, 4, 0],
        "reversal remapped the joints"
    );
    let door: ProfileLoop<f64> = src.map(<f64 as Real>::from_f64);
    let loft = loft_walk::<f64>(&src);
    let anchor = anchor_walk::<f64>(&src);
    for (i, ((d, l), a)) in door
        .vertices()
        .iter()
        .zip(loft.vertices())
        .zip(anchor.vertices())
        .enumerate()
    {
        assert_eq!(
            d.pos().x.to_bits(),
            l.pos().x.to_bits(),
            "vertex {i} x vs loft"
        );
        assert_eq!(
            d.pos().y.to_bits(),
            l.pos().y.to_bits(),
            "vertex {i} y vs loft"
        );
        assert_eq!(
            d.bulge().to_bits(),
            l.bulge().to_bits(),
            "vertex {i} bulge vs loft"
        );
        assert_eq!(
            d.pos().x.to_bits(),
            a.pos().x.to_bits(),
            "vertex {i} x vs anchor"
        );
        assert_eq!(
            d.pos().y.to_bits(),
            a.pos().y.to_bits(),
            "vertex {i} y vs anchor"
        );
        assert_eq!(
            d.bulge().to_bits(),
            a.bulge().to_bits(),
            "vertex {i} bulge vs anchor"
        );
    }
    // `reversed` keeps vertex 0 in place, so the signed zero is at 0.
    assert_eq!(
        door.vertices()[0].pos().y.to_bits(),
        (-0.0f64).to_bits(),
        "the signed zero travels"
    );
    assert_eq!(table(&door), table(&loft));
    assert_eq!(table(&door), table(&anchor));
}

/// The door is total: a poisoned table crosses unexamined, bit for
/// bit, exactly as the former walks carried it.
#[test]
fn r1_embed_is_total_on_a_poisoned_table() {
    let src = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(f64::INFINITY, 0.0), f64::NAN),
        ProfileVertex::new(p2(1.0, f64::NEG_INFINITY), -0.0),
    ])
    .with_tangent_joints(vec![usize::MAX]);
    let door: ProfileLoop<f64> = src.map(<f64 as Real>::from_f64);
    assert_eq!(table(&door), table(&loft_walk::<f64>(&src)));
    assert!(door.vertices()[0].bulge().is_nan());
    assert_eq!(door.tangent_joints(), [usize::MAX]);
}

/// At the interval scalar `from_f64` is an embedding, not the
/// identity, so this is where a door that chose a different
/// conversion than the sites it replaced would show.
#[cfg(feature = "interval")]
#[test]
fn r1_embed_is_both_former_walks_at_interval() {
    use geom_core::Interval;
    let src = awkward();
    let door: ProfileLoop<Interval> = src.map(Interval::from_f64);
    let loft = loft_walk::<Interval>(&src);
    let anchor = anchor_walk::<Interval>(&src);
    assert_eq!(table(&door), table(&loft), "door vs loft.rs's walk");
    assert_eq!(table(&door), table(&anchor), "door vs anchor.rs's walk");
    // And the coordinates are point intervals of the stored bits.
    for (d, s) in door.vertices().iter().zip(src.vertices()) {
        assert_eq!(
            table_scalar(d.pos().x),
            format!("{:?}", Interval::from_f64(s.pos().x))
        );
        assert_eq!(
            table_scalar(d.bulge()),
            format!("{:?}", Interval::from_f64(s.bulge()))
        );
    }
}

#[cfg(feature = "interval")]
fn table_scalar<T: std::fmt::Debug>(x: T) -> String {
    format!("{x:?}")
}

// ------------------------------------------------------------------
// The widened lift
// ------------------------------------------------------------------

/// The unit's stadium: two straight sides, two semicircular ends,
/// every joint declared, authored through the lattice.
fn stadium() -> ProfileLoop<f64> {
    let t = Tol::witness();
    Open.at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t)
        .expect("the stadium closes")
        .loop_
}

/// The all-declared loop seamed at EVERY rotation: the closing leg is
/// alternately an arc and a straight side, the first leg alternately
/// a straight side and an arc, and each lifts at rotation 0 with the
/// arrival carrying joint 0's declaration.
///
/// Fidelity is NOT asserted bit-identical: the unit's two rows hold
/// bit-identity at seams 0 and 1, where the lift's derived `line(len)`
/// legs re-run the very computation that authored the vertices; at
/// seam 2 the same leg is derived off an arc arrival instead and lands
/// value-equal (F10's class). The per-seam worst is printed.
#[test]
fn r1_the_all_declared_loop_lifts_at_every_seam() {
    let source = stadium();
    let n = source.vertices().len();
    for r in 0..n {
        let reseamed: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(
            (0..n).map(|k| source.vertices()[(k + r) % n]).collect(),
        )
        .with_tangent_joints((0..n).collect());
        match lift_checked(&reseamed, Tol::witness()) {
            LiftOutcome::Lifted {
                program,
                rotation,
                fidelity,
                worst_ulps,
                ..
            } => {
                assert_eq!(rotation, 0, "seam {r}: a declared seam never rotates");
                println!("seam {r}: {fidelity:?} worst_ulps {worst_ulps}: {program:?}");
                assert!(
                    matches!(
                        program.last(),
                        Some(
                            Step::TangentArcTo(Target::StartArriving)
                                | Step::ContinueTo(Target::StartArriving)
                        )
                    ),
                    "seam {r}: the arrival carries the declaration: {program:?}"
                );
            }
            other => panic!("seam {r}: the stadium must lift: {other:?}"),
        }
    }
}

/// A two-arc circle with BOTH joints declared — legal data since the
/// sixth-round ruling (a declaration on carrier identity is honoured)
/// and, with a declared joint present, off the closed-carrier forms.
/// The chain form seams at 0 and closes with the tangent arrival.
#[test]
fn r1_a_two_arc_circle_with_both_joints_declared_lifts() {
    let circle = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 1.0),
        ProfileVertex::new(p2(2.0, 0.0), 1.0),
    ])
    .with_tangent_joints(vec![0, 1]);
    let outcome = lift_checked(&circle, Tol::witness());
    match &outcome {
        LiftOutcome::Lifted {
            program, rotation, ..
        } => {
            assert_eq!(*rotation, 0);
            assert!(
                matches!(
                    program.last(),
                    Some(Step::TangentArcTo(Target::StartArriving))
                ),
                "{program:?}"
            );
        }
        other => panic!("the declared two-arc circle must lift: {other:?}"),
    }
}

/// A declared closing joint whose declaration is false by 1e-13 m —
/// inside the continuation's band. The lift spells `continue_to`, the
/// driver accepts within the band, and the replayed table is the
/// source bit for bit: the lift moves nothing and re-adjudicates
/// nothing; whether the declaration is TRUE is the data gate's
/// question, asked of the same table either way.
#[test]
fn r1_a_declared_closing_joint_false_within_the_band_lifts_bit_identical() {
    let loop_ = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(1.0, 1.0),
        p2(0.5, 0.5 + 1e-13),
    ])
    .with_tangent_joints(vec![3]);
    match lift_checked(&loop_, Tol::witness()) {
        LiftOutcome::Lifted {
            program,
            fidelity,
            rotation,
            ..
        } => {
            assert_eq!(rotation, 0);
            assert_eq!(fidelity, Fidelity::BitIdentical, "{program:?}");
            assert!(
                matches!(program.last(), Some(Step::ContinueTo(Target::Start))),
                "{program:?}"
            );
        }
        other => panic!("expected the in-band continuation to lift: {other:?}"),
    }
}
