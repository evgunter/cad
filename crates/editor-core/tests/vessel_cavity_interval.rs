//! **The corpus vessel's hollow is certified embedded at `Interval`**:
//! the document's own pot (`corpus/vessel.rs`), shelled sealed and
//! opened, answers E7's self-intersection question `Holds` at `f64` and
//! at `Interval` alike.
//!
//! The document builds the belly from `ProgramArcData::Center`, which
//! lands its sphere exactly (`c = (0, 1/16)`, `r = 5/64`, point
//! intervals), where a pot authored through a bulge carries the same
//! sphere a few ulps wide. Every enclosure on the exact pot is the
//! narrower, and so the cavity's mouth-rim corner on the band, where
//! the seam meets the rim, is held by two enclosures of one point whose
//! own roundings no longer coincide: the rim line's, and the seam's end.
//! The presentation reads the side the seam bounds out to the rim
//! line's far bound, which the seam alone does not reach
//! (`topo::injective`'s `side_ranges`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::{body_of, eval, failures, vessel};
use editor_core::{EvalScalar, Node};
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Interval, Tol};
use topo::clearance::{
    BoundReport, BoundVerdict, Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
};
use topo::{Body, FaceKey};

trait Scalar: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy + EvalScalar {}
impl<T: Decide + Bounds + CertifiedBounds + topo::AtRestPolicy + EvalScalar> Scalar for T {}

/// The pot the vessel shells, and the vessel itself: the revolve node's
/// body and the document's result, evaluated at `T`.
fn pot_and_vessel<T: Scalar>() -> (Body<T>, Body<T>) {
    let d = vessel::document();
    let ev = eval::<T>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "the vessel evaluates:\n{}", bad.join("\n"));
    let pot = d
        .doc
        .ids()
        .iter()
        .copied()
        .find(|&id| matches!(d.doc.node(id), Some(Node::Revolve { .. })))
        .expect("the pot's revolve");
    (
        body_of(&ev, pot),
        body_of(&ev, d.result.expect("the shell")),
    )
}

/// The pot shelled sealed to the vessel's wall, and its cavity's faces.
fn sealed_cavity<T: Scalar>(pot: Body<T>) -> (Body<T>, Vec<FaceKey>) {
    let tol = Tol::witness();
    let finished = |what, b| sweep::test_support::finished(what, b, tol);
    let hollow = topo::shell(&finished("the pot", pot), T::from_f64(vessel::WALL), tol)
        .expect("the pot shells sealed")
        .body;
    let roles = topo::classify_shells(&finished("the hollow", hollow.clone()), tol)
        .expect("the hollow classifies");
    let [void] = roles
        .iter()
        .filter(|c| c.role == topo::ShellRole::Void)
        .map(|c| c.shell)
        .collect::<Vec<_>>()[..]
    else {
        panic!("one cavity: {roles:?}")
    };
    let faces = hollow
        .faces()
        .filter(|(_, f)| f.shell == void)
        .map(|(k, _)| k)
        .collect();
    (hollow, faces)
}

/// E7's self-intersection question over `faces` of `body`, at the
/// document door's default budget.
fn injective<T: Scalar>(body: &Body<T>, faces: Vec<FaceKey>) -> BoundReport<()> {
    let tol = Tol::witness();
    let windows = Side { body, faces }
        .windows(Which::First)
        .expect("every face has a window");
    let sweep = Sweep {
        bound: ClearanceBound::StrictlyPositive,
        band: Band::linear(tol).unwrap(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 65_536,
        },
    };
    sweep.run(&windows, &windows, true, |_| {
        Err::<(), _>("a witness here is a crossing".to_owned())
    })
}

/// Both questions at `T`: the sealed cavity's faces, the opened
/// vessel's every face.
fn both<T: Scalar>() -> [BoundReport<()>; 2] {
    let (pot, vessel) = pot_and_vessel::<T>();
    let (hollow, cavity) = sealed_cavity(pot);
    let all = vessel.faces().map(|(k, _)| k).collect();
    [injective(&hollow, cavity), injective(&vessel, all)]
}

/// **The vessel's cavity, sealed and opened, holds at both scalars,
/// refusing no pair.** Red if the side a seam bounds at the mouth-rim
/// corner must be reached by the seam's own end: at `Interval` both
/// questions then refuse `Uncertified { Departure }` across the band's
/// seam, the corner cell unpresented at every depth.
#[test]
fn the_vessels_cavity_certifies_embedded_at_both_scalars() {
    let float = both::<f64>();
    let interval = both::<Interval>();
    for (what, report) in [
        ("sealed cavity, f64", &float[0]),
        ("opened vessel, f64", &float[1]),
        ("sealed cavity, Interval", &interval[0]),
        ("opened vessel, Interval", &interval[1]),
    ] {
        println!(
            "[potdoc] {what}: {:?} {:?} cornered = {}",
            report.verdict, report.receipt, report.cornered
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "{what} is embedded: {:?}",
            report.verdict
        );
        assert_eq!(report.receipt.refused, 0, "{what}: no pair refused");
        assert!(
            report.receipt.certified > 0,
            "{what}: touching pairs certified"
        );
    }
}
