//! The raw door's demotion, at the two seams it moved.
//!
//! The census rows in `raw_door_census.rs` say what a shipped build
//! cannot mint. These say what the two things that replaced the door
//! actually produce: the materialization door's table, and the lift's
//! program for a loop whose every joint is declared.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Real, Tol};
use profile::{
    Fidelity, LiftOutcome, Open, ProfileLoop, ProfileVertex, RawLoop, Start, Step, Target,
    lift_checked,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The stadium: two straight sides and two semicircular ends, every
/// joint declared tangent, authored through the lattice.
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

// ------------------------------------------------------------------
// The materialization door
// ------------------------------------------------------------------

/// **`map` at `f64` is the identity, bit for bit** — the receipt the
/// two production sites that call it stand on.
///
/// The door crosses every coordinate through `T::from_f64`. At `f64`
/// that is the identity function, so this row measures the WALK — the
/// index order, the bulge carried with its own vertex, the declared
/// joints travelling unchanged — rather than the arithmetic. A walk
/// that dropped a joint or shifted a bulge by one index would still
/// produce a plausible loop, and this is what says it does not.
#[test]
fn the_materialization_door_reproduces_the_table_bit_for_bit() {
    let source = stadium();
    let crossed: ProfileLoop<f64> = source.map(<f64 as Real>::from_f64);

    assert_eq!(crossed.vertices().len(), source.vertices().len());
    for (i, (a, b)) in source
        .vertices()
        .iter()
        .zip(crossed.vertices().iter())
        .enumerate()
    {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits(), "vertex {i} x");
        assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits(), "vertex {i} y");
        assert_eq!(a.bulge().to_bits(), b.bulge().to_bits(), "vertex {i} bulge");
    }
    assert_eq!(
        crossed.tangent_joints(),
        source.tangent_joints(),
        "the declarations travel"
    );
}

/// The door carries a HAND-BUILT table too, including one the lattice
/// would refuse — because that is what a persisted or fixture table can
/// hold, and crossing scalars is not the place to re-adjudicate it.
#[test]
fn the_materialization_door_does_not_re_adjudicate_the_table() {
    let odd: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
    ])
    .with_tangent_joints(vec![7]);
    let crossed: ProfileLoop<f64> = odd.map(<f64 as Real>::from_f64);
    assert_eq!(crossed.tangent_joints(), [7]);
    assert_eq!(crossed.vertices().len(), 3);
}

// ------------------------------------------------------------------
// The seam the entry cannot declare
// ------------------------------------------------------------------

/// **Every joint declared, and the loop lifts.** `.at(p)` declares
/// nothing, so the lift used to have no seam for this loop at all; the
/// closing TARGET declares joint 0 instead, and the arrival is where
/// the declaration rides.
/// **`BitIdentical` here is SEAM-SELECTED, and that is the honest
/// reading of it** (R1 n4). This loop is bit-identical at seams 0 and
/// 1, where the lift's derived `line(len)` legs re-run the very
/// computation that authored the vertices; rotate the same loop to seam
/// 2 or 3 and the leg is derived off an arc arrival instead and lands
/// `ValueEqual` — F10's class, not a defect and not a regression.
/// `bool9r1_probes::r1_the_all_declared_loop_lifts_at_every_seam` walks
/// all four and prints each. So this row's `BitIdentical` is a claim
/// about THIS SEAM of this loop, never about the widening.
#[test]
fn the_all_tangent_stadium_lifts_at_the_declared_seam() {
    let loop_ = stadium();
    assert_eq!(loop_.tangent_joints().len(), loop_.vertices().len());

    match lift_checked(&loop_, Tol::witness()) {
        LiftOutcome::Lifted {
            program,
            rotation,
            fidelity,
            worst_ulps,
            ..
        } => {
            assert_eq!(rotation, 0, "no rotation can move a declared seam");
            assert_eq!(fidelity, Fidelity::BitIdentical);
            assert_eq!(worst_ulps, 0);
            assert!(
                matches!(
                    program.last(),
                    Some(Step::TangentArcTo(Target::StartArriving))
                ),
                "{program:?}"
            );
        }
        other => panic!("the stadium must lift: {other:?}"),
    }
}

/// **A declared joint whose leaving segment closes the loop straight.**
/// After `.tangent()` the only straight verb is `.line(len)`, which
/// never closes — so this was a wall. It is not a wall: the straight
/// leg off a declared joint IS `continue_to`, which declares its own
/// joint and does close.
#[test]
fn a_declared_joint_closing_straight_lifts_as_the_continuation() {
    let source = stadium();
    let n = source.vertices().len();
    // The same stadium re-seamed one vertex on, so the CLOSING leg is a
    // straight side rather than an end cap. Pure reindexing: every
    // stored bit survives, which is what lets the row below ask for
    // bit-identity.
    let reseamed: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(
        (0..n).map(|k| source.vertices()[(k + 1) % n]).collect(),
    )
    .with_tangent_joints((0..n).collect());

    match lift_checked(&reseamed, Tol::witness()) {
        LiftOutcome::Lifted {
            program,
            fidelity,
            worst_ulps,
            ..
        } => {
            assert_eq!(fidelity, Fidelity::BitIdentical);
            assert_eq!(worst_ulps, 0);
            assert!(
                matches!(
                    program.last(),
                    Some(Step::ContinueTo(Target::StartArriving))
                ),
                "{program:?}"
            );
        }
        other => panic!("the re-seamed stadium must lift: {other:?}"),
    }
}

/// The same wall with a SHARP seam still in the loop, so the
/// all-declared arm is not what answers: the last joint is declared and
/// its leaving segment is the straight one that closes.
///
/// A triangle with one side subdivided at a genuinely collinear
/// vertex — the declaration is true, so the driver has nothing to
/// refuse. `.tangent()` then had nowhere to go (`.line(len)` never
/// closes); `continue_to(Start)` closes it, and the seam itself stays
/// undeclared, so the target is the plain `Start`.
#[test]
fn a_declared_joint_closing_straight_lifts_beside_a_sharp_seam() {
    let loop_: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(1.0, 1.0),
        p2(0.5, 0.5),
    ])
    .with_tangent_joints(vec![3]);

    match lift_checked(&loop_, Tol::witness()) {
        LiftOutcome::Lifted {
            program,
            fidelity,
            rotation,
            ..
        } => {
            assert_eq!(rotation, 0, "joint 0 is sharp and is the seam");
            assert_eq!(fidelity, Fidelity::BitIdentical);
            assert!(
                matches!(program.last(), Some(Step::ContinueTo(Target::Start))),
                "{program:?}"
            );
        }
        other => panic!("the closing continuation must lift: {other:?}"),
    }
}

/// **What the retired structural wall becomes when the declaration is
/// FALSE**: the driver's refusal, in the driver's words.
///
/// This is the fixture the lift census used to pin
/// `DeclaredJointBeforeClosingLine` on — and its joint 2 is a right
/// angle, so "declared tangent" is untrue of it. The lift's contract is
/// that geometric walls belong to the binders: it spells the natural
/// program and `continue_to` measures the target against the departing
/// ray, naming the miss. A structural refusal could only ever have said
/// "no spelling"; this says what is wrong with the loop.
#[test]
fn a_false_declaration_at_the_closing_joint_is_the_drivers_refusal() {
    let loop_: ProfileLoop<f64> = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.3),
        ProfileVertex::new(p2(1.0, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])
    .with_tangent_joints(vec![2]);

    match lift_checked(&loop_, Tol::witness()) {
        LiftOutcome::ReplayRefused { error, .. } => {
            assert!(
                format!("{error}").contains("ray"),
                "the target is off the departing ray: {error}"
            );
        }
        other => panic!("expected the driver's wall: {other:?}"),
    }
}

/// The undeclared seam is untouched: a loop with a sharp joint still
/// rotates onto it and closes with the PLAIN `Start`, so the new
/// spelling is reached by the declaration and never by default.
#[test]
fn an_undeclared_seam_still_closes_plain() {
    let square: ProfileLoop<f64> =
        RawLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    match lift_checked(&square, Tol::witness()) {
        LiftOutcome::Lifted {
            program, fidelity, ..
        } => {
            assert_eq!(fidelity, Fidelity::BitIdentical);
            assert!(
                matches!(program.last(), Some(Step::LineTo(Target::Start))),
                "{program:?}"
            );
        }
        other => panic!("a square must lift: {other:?}"),
    }
}
