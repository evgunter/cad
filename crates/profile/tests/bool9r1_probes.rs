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
//!    semicircle, a signed zero, then reversed) — at `f64` and at
//!    the interval scalar, where `from_f64` is not the identity.
//! 2. **The widened lift on loops the unit's rows do not reach**: the
//!    all-tangent loop at every seam rotation, a two-arc circle, and a
//!    closing joint collinear by less than the band.
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
//! that bulge gone the door derives nothing and carries every stored
//! field through `f`, so these rows hold it against the source table
//! itself: the identity at `f64`, and each field's own `from_f64` at
//! the interval scalar.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Real, Tol};
use profile::{
    Fidelity, LiftOutcome, Open, ProfileLoop, RawLoop, Segment, Start, Step, Target, lift_checked,
    test_support::bulge_loop,
};

/// A table only the fixture helpers can spell: arcs of both signs, a
/// semicircle, a signed zero, then the whole thing reversed.
fn awkward() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, -0.0), 0.3),
        (Point2::new(2.0, 0.0), -0.5),
        (Point2::new(2.0, 2.0), 1.0),
        (Point2::new(1.0, 3.0), 0.0),
        (Point2::new(0.0, 2.0), 0.123_456_789_012_3),
    ])
    .reversed()
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
    vs.join(" ")
}

// ------------------------------------------------------------------
// The materialization door
// ------------------------------------------------------------------

/// At `f64` the door is the identity, bit for bit — including the
/// signed zero and the reversed order.
#[test]
fn r1_embed_is_the_identity_at_f64_bit_for_bit() {
    let src = awkward();
    let door: ProfileLoop<f64> = src.map_scalar(<f64 as Real>::from_f64);
    for (i, (d, w)) in door.vertices().iter().zip(src.vertices()).enumerate() {
        assert_eq!(d.x.to_bits(), w.x.to_bits(), "vertex {i} x vs the source");
        assert_eq!(d.y.to_bits(), w.y.to_bits(), "vertex {i} y vs the source");
        assert_eq!(
            format!("{:?}", door.segments()[i]),
            format!("{:?}", src.segments()[i]),
            "segment {i} vs the source"
        );
    }
    // `reversed` keeps vertex 0 in place, so the signed zero is at 0.
    assert_eq!(
        door.vertices()[0].y.to_bits(),
        (-0.0f64).to_bits(),
        "the signed zero travels"
    );
    assert_eq!(table(&door), table(&src));
}

/// The door is total: a poisoned table crosses unexamined, bit for
/// bit.
#[test]
fn r1_embed_is_total_on_a_poisoned_table() {
    let src = bulge_loop(vec![
        (Point2::new(f64::INFINITY, 0.0), f64::NAN),
        (Point2::new(1.0, f64::NEG_INFINITY), -0.0),
    ]);
    let door: ProfileLoop<f64> = src.map_scalar(<f64 as Real>::from_f64);
    assert_eq!(table(&door), table(&src));
    assert!(
        matches!(door.segments()[0], Segment::Arc(arc) if arc.sweep.is_nan()),
        "a NaN bulge lowers to an arc of NaN sweep, and it crosses as one"
    );
}

/// At the interval scalar `from_f64` is an embedding, not the
/// identity, so this is where a door that chose a different conversion
/// than each field's own, or carried a field to another's place, would
/// show: every vertex coordinate and every arc field is the point
/// interval of its own stored bits.
#[test]
fn r1_embed_carries_each_field_at_interval() {
    use geom_core::Interval;
    let src = awkward();
    let door: ProfileLoop<Interval> = src.map_scalar(Interval::from_f64);
    let point = |x: f64| format!("{:?}", Interval::from_f64(x));
    let got = |x: &Interval| format!("{x:?}");
    for (i, (d, s)) in door.vertices().iter().zip(src.vertices()).enumerate() {
        assert_eq!(
            (got(&d.x), got(&d.y)),
            (point(s.x), point(s.y)),
            "vertex {i}"
        );
    }
    for (i, (d, s)) in door.segments().iter().zip(src.segments()).enumerate() {
        match (d, s) {
            (Segment::Line, Segment::Line) => {}
            (Segment::Arc(d), Segment::Arc(s)) => assert_eq!(
                [&d.centre.x, &d.centre.y, &d.radius, &d.sweep].map(got),
                [s.centre.x, s.centre.y, s.radius, s.sweep].map(point),
                "segment {i}'s arc"
            ),
            (d, s) => panic!("segment {i} crossed as another kind: {d:?} from {s:?}"),
        }
    }
}

// ------------------------------------------------------------------
// The widened lift
// ------------------------------------------------------------------

/// The unit's stadium: two straight sides, two semicircular ends,
/// every joint constructed tangent, authored through the lattice.
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

/// The all-tangent loop seamed at EVERY rotation: the closing leg is
/// alternately an arc and a straight side, the first leg alternately
/// a straight side and an arc, and each lifts at rotation 0 with the
/// arrival carrying joint 0's tangency.
///
/// Fidelity is NOT asserted bit-identical: the unit's two rows hold
/// bit-identity at seams 0 and 1, where the lift's derived `line(len)`
/// legs re-run the very computation that authored the vertices; at
/// seam 2 the same leg is derived off an arc arrival instead and lands
/// value-equal (F10's class). The per-seam worst is printed.
#[test]
fn r1_the_all_tangent_loop_lifts_at_every_seam() {
    let source = stadium();
    let n = source.vertices().len();
    for r in 0..n {
        let reseamed: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new((0..n).map(|k| {
            let j = (k + r) % n;
            (source.vertices()[j], source.segments()[j])
        }));
        match lift_checked(&reseamed, Tol::witness()) {
            LiftOutcome::Lifted {
                program,
                rotation,
                fidelity,
                worst_ulps,
                ..
            } => {
                assert_eq!(rotation, 0, "seam {r}: a tangent seam never rotates");
                println!("seam {r}: {fidelity:?} worst_ulps {worst_ulps}: {program:?}");
                assert!(
                    matches!(
                        program.last(),
                        Some(
                            Step::TangentArcTo(Target::StartArriving)
                                | Step::ContinueTo(Target::StartArriving)
                        )
                    ),
                    "seam {r}: the arrival carries the tangency: {program:?}"
                );
            }
            other => panic!("seam {r}: the stadium must lift: {other:?}"),
        }
    }
}

/// A two-arc circle: both joints are one carrier continuing, so both
/// are tangent, and the lift reads it as the closed carrier it is — the
/// `circle` form, seamed at its +x pole.
#[test]
fn r1_a_two_arc_circle_lifts_as_the_circle_form() {
    let circle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 1.0),
        (Point2::new(2.0, 0.0), 1.0),
    ]);
    match lift_checked(&circle, Tol::witness()) {
        LiftOutcome::Lifted {
            program, rotation, ..
        } => {
            assert_eq!(rotation, 1, "the circle form seams at the +x pole");
            assert!(
                matches!(program.as_slice(), [Step::Circle { .. }]),
                "{program:?}"
            );
        }
        other => panic!("the two-arc circle must lift: {other:?}"),
    }
}

/// A closing joint collinear to within 1e-13 m — inside the band, so
/// validation decides it one carrier continuing, a tangent joint. The
/// lift spells `continue_to`, the driver accepts within the band, and
/// the replayed table is the source bit for bit: the lift moves nothing
/// and re-adjudicates nothing.
#[test]
fn r1_a_closing_joint_collinear_within_the_band_lifts_bit_identical() {
    let loop_ = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 0.5 + 1e-13),
    ]);
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
