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
//!    then reversed) — at `f64` and at
//!    the interval scalar, where `from_f64` is not the identity.
//! 2. **The widened lift on loops the unit's rows do not reach**: the
//!    all-declared loop at every seam rotation, a two-arc circle with
//!    both joints declared, and a declared closing joint whose
//!    declaration is false by less than the band.
//!
//! **Adopted into the unit's branch with one mechanical re-aim, kept
//! otherwise verbatim (authorship the reviewer's).** The door these rows
//! exercise is spelled `ProfileLoop::map_scalar` at the merged head, not
//! `embed`: R2-Q1 ruled that `embed` was a fourth private word for what
//! `Point2`/`Vec2`/`Affine3`/`SketchPlane` all call `map` — a leaf's
//! name, which a loop's lift then took under the scalar-lift
//! convention. Every `src.embed()` here is
//! `src.map_scalar(<f64 as Real>::from_f64)` or
//! `src.map_scalar(Interval::from_f64)`; nothing else about what these
//! rows measure has changed.
//!
//! **Second adoption note (the `origin/main` merge).** `embed_profile`
//! no longer exists: main's `ValidatedProfile::lift_onto` moved
//! editor-core's crossing onto the VALIDATED form, and the free
//! function these rows name was deleted with it. The walk is kept here
//! verbatim anyway — it is still one of the two walks the door was
//! claimed to reproduce, the claim was made when both existed, and a
//! row that measures a retired walk against the door that replaced it
//! is exactly as informative now as it was then. `loft.rs::end_profile`
//! is the surviving production caller.
//!
//! **Third adoption note (the stored bulge retired).** Both walks
//! re-lowered each segment from the bulge the loop kept beside it; with
//! that bulge gone the door carries every stored field through `f` and
//! derives nothing, so the one walk left to measure it against is that
//! field-by-field embedding, written out by hand (`field_walk`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Arc2, Point2, Real, Tol};
use profile::{
    Fidelity, LiftOutcome, Open, ProfileLoop, RawLoop, Segment, Start, Step, Target, lift_checked,
    test_support::bulge_loop,
};

/// A table only the fixture helpers can spell: arcs of both signs, a
/// semicircle, a signed zero, declarations duplicated and out of
/// order, then the whole thing reversed (so the joints are remapped).
fn awkward() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, -0.0), 0.3),
        (Point2::new(2.0, 0.0), -0.5),
        (Point2::new(2.0, 2.0), 1.0),
        (Point2::new(1.0, 3.0), 0.0),
        (Point2::new(0.0, 2.0), 0.123_456_789_012_3),
    ])
    .with_tangent_joints(vec![4, 1, 1, 0])
    .reversed()
}

/// Every stored field of the table carried through `from_f64` one at a
/// time — the vertices, each segment's kind and an arc's centre, radius
/// and sweep — written out by hand beside the door.
fn field_walk<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    <ProfileLoop<T> as RawLoop<T>>::new(lp.vertices().iter().zip(lp.segments()).map(
        |(vx, segment)| {
            let segment = match *segment {
                Segment::Line => Segment::Line,
                Segment::Arc(arc) => Segment::Arc(Arc2 {
                    centre: Point2::new(T::from_f64(arc.centre.x), T::from_f64(arc.centre.y)),
                    radius: T::from_f64(arc.radius),
                    sweep: T::from_f64(arc.sweep),
                }),
            };
            (Point2::new(T::from_f64(vx.x), T::from_f64(vx.y)), segment)
        },
    ))
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// The table as text: every scalar through its `Debug`, which for
/// `f64` distinguishes the two zeros and for the interval scalar
/// prints both bounds.
fn table<T: Real + std::fmt::Debug>(lp: &ProfileLoop<T>) -> String {
    let vs: Vec<String> = lp
        .vertices()
        .iter()
        .zip(lp.segments())
        .map(|(v, s)| format!("{:?},{:?};{:?}", v.x, v.y, s))
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
    let door: ProfileLoop<f64> = src.map_scalar(<f64 as Real>::from_f64);
    let walk = field_walk::<f64>(&src);
    for (i, (d, w)) in door.vertices().iter().zip(walk.vertices()).enumerate() {
        assert_eq!(d.x.to_bits(), w.x.to_bits(), "vertex {i} x vs the walk");
        assert_eq!(d.y.to_bits(), w.y.to_bits(), "vertex {i} y vs the walk");
        assert_eq!(
            format!("{:?}", door.segments()[i]),
            format!("{:?}", walk.segments()[i]),
            "segment {i} vs the walk"
        );
    }
    // `reversed` keeps vertex 0 in place, so the signed zero is at 0.
    assert_eq!(
        door.vertices()[0].y.to_bits(),
        (-0.0f64).to_bits(),
        "the signed zero travels"
    );
    assert_eq!(table(&door), table(&walk));
}

/// The door is total: a poisoned table crosses unexamined, bit for
/// bit, exactly as the former walks carried it.
#[test]
fn r1_embed_is_total_on_a_poisoned_table() {
    let src = bulge_loop(vec![
        (Point2::new(f64::INFINITY, 0.0), f64::NAN),
        (Point2::new(1.0, f64::NEG_INFINITY), -0.0),
    ])
    .with_tangent_joints(vec![usize::MAX]);
    let door: ProfileLoop<f64> = src.map_scalar(<f64 as Real>::from_f64);
    assert_eq!(table(&door), table(&field_walk::<f64>(&src)));
    assert!(
        matches!(door.segments()[0], Segment::Arc(arc) if arc.sweep.is_nan()),
        "a NaN bulge lowers to an arc of NaN sweep, and it crosses as one"
    );
    assert_eq!(door.tangent_joints(), [usize::MAX]);
}

/// At the interval scalar `from_f64` is an embedding, not the
/// identity, so this is where a door that chose a different
/// conversion than the sites it replaced would show.
#[test]
fn r1_embed_is_both_former_walks_at_interval() {
    use geom_core::Interval;
    let src = awkward();
    let door: ProfileLoop<Interval> = src.map_scalar(Interval::from_f64);
    assert_eq!(
        table(&door),
        table(&field_walk::<Interval>(&src)),
        "door vs the walk"
    );
    // And the coordinates are point intervals of the stored bits.
    let table_scalar = |x: &dyn std::fmt::Debug| format!("{x:?}");
    for (d, s) in door.vertices().iter().zip(src.vertices()) {
        assert_eq!(table_scalar(&d.x), format!("{:?}", Interval::from_f64(s.x)));
    }
    for (d, s) in door.segments().iter().zip(src.segments()) {
        if let (Segment::Arc(d), Segment::Arc(s)) = (d, s) {
            assert_eq!(
                table_scalar(&d.sweep),
                format!("{:?}", Interval::from_f64(s.sweep))
            );
            assert_eq!(
                table_scalar(&d.radius),
                format!("{:?}", Interval::from_f64(s.radius))
            );
        }
    }
}

// ------------------------------------------------------------------
// The widened lift
// ------------------------------------------------------------------

/// The unit's stadium: two straight sides, two semicircular ends,
/// every joint declared, authored through the lattice.
fn stadium() -> ProfileLoop<f64> {
    let t = Tol::witness();
    Open.at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .tangent()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start.arrives_tangent(), t)
        .expect("the stadium closes")
        .loop_
        .into_loop()
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
        let reseamed: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new((0..n).map(|k| {
            let j = (k + r) % n;
            (source.vertices()[j], source.segments()[j])
        }))
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
    let circle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 1.0),
        (Point2::new(2.0, 0.0), 1.0),
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
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 0.5 + 1e-13),
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
