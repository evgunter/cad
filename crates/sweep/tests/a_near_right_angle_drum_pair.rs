//! **A drum pair a hair short of square reads its pierces' sector side
//! one order down.** The drum `cyl(0.5, 1.0)` against itself, B turned
//! about `x` by `π/2 − δ` and moved by `(0.3, 0.2, 0.1)`. A's seam
//! pierces B's wall twice, and at each pierce the section leaves along
//! A's sector bisector to within the miss: the bisector departs B's
//! tangent plane at a slope of about `0.92·δ`, its rise over the arm
//! decided at the run band, while its curvature charge `s²·R/4` is
//! decided Zero. That charge's Zero ties the bound, and its jet (its
//! rise and its parting together, at the arm) decides the side, as the
//! square pose's first-order tie is decided by its parting. So both
//! member orders of the union build at the crossed cylinders' sliced
//! volume (`common::crossed_cylinders`).
//!
//! - `δ = 1e-6`: both orders build SOUND at the default band; at 1e-6
//!   the first order is in its zero band and the tie is the band's,
//!   and the body's enclosure is wider than the shared budget (pinned).
//! - `δ ≈ 2.68e-8`, the turn `1.5707963` the pose was filed at: the
//!   default band reads a sector bound's first-order rise in band
//!   (`enters_material`, 8.4e-9) and escalates; past it, nothing
//!   refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::crossed_cylinders::{self as crossed, Cylinder, Op};
use crate::common::differential::outcome;
use crate::common::germ_pair::{cyl, spin};
use core::f64::consts::FRAC_PI_2;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanDecision, BooleanError, BooleanResult, Coincide};

/// The turn the pose was filed at: `π/2` cut to eight places, the miss
/// being the pose.
#[allow(clippy::approx_constant)]
const FILED_TURN: f64 = 1.5707963;

/// The two drums, B turned `turn` about `x` and moved, with the closed
/// form of their union.
fn drums(turn: f64) -> (AtRestBody<f64>, AtRestBody<f64>, f64) {
    let tol = Tol::witness();
    let shift = Vec3::new(0.3, 0.2, 0.1);
    let b = spin(&cyl(0.5, 1.0), Vec3::unit_x(), turn);
    let b = topo::transform_rigid(
        &b,
        &geom_core::test_support::rigid(&Affine3::translation(shift), tol),
    )
    .unwrap();
    let want = crossed::volume(
        Cylinder::upright(0.0, 0.0, 0.5, (-1.0, 1.0)),
        Cylinder::upright(0.0, 0.0, 0.5, (-1.0, 1.0))
            .under(Affine3::rotation_about_axis(
                Point3::origin(),
                Vec3::unit_x(),
                turn,
            ))
            .under(Affine3::translation(shift)),
        Op::Union,
    );
    (
        finished("A", cyl(0.5, 1.0), tol),
        finished("B", b, tol),
        want,
    )
}

/// `A ∪ B` and `B ∪ A`, labelled.
type Unions = [(&'static str, Result<BooleanResult<f64>, BooleanError>); 2];

fn unions(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Unions {
    let tol = Tol::witness();
    [
        ("A ∪ B", topo::union(a, b, tol)),
        ("B ∪ A", topo::union(b, a, tol)),
    ]
}

/// Both orders build SOUND at the closed form.
fn sound(turn: f64) {
    let tol = Tol::witness();
    let (a, b, want) = drums(turn);
    for (label, r) in unions(&a, &b) {
        let line = outcome(r, want, tol);
        assert!(line.starts_with("OK SOUND"), "turn {turn}, {label}: {line}");
    }
}

/// **A microradian short of square, the charge's Zero is read by the
/// jet.** Read with the charge's Zero refused rather than tied, both
/// orders refuse `CurvedSectorSideUnsupported` on a charge of 1.05e-13
/// at the default band.
#[test]
fn a_turn_a_microradian_short_of_square_builds_at_its_sliced_volume() {
    // Pinned red at 1e-6: the union's enclosure holds the closed form and
    // is wider than the row's ceiling
    // (`work/spiricarm/main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &[1e-6],
        "a microradian short of square",
        &["A ∪ B: OK BAD", "v=2.660151792 pad=2.5e-7 want=2.660151791"],
        || sound(FRAC_PI_2 - 1e-6),
    );
}

/// **At the filed turn the default band escalates the first order**:
/// both orders escalate a sector bound's rise in band, never the
/// charge's refusal. At 1e-6 the rise is in the zero band and the union
/// builds at its sliced volume over the budget, as the microradian
/// turn's does.
#[test]
fn the_filed_turn_escalates_its_first_order_at_the_default_band() {
    let eps = Tol::witness().eps();
    if eps == 1e-12 {
        println!(
            "SKIPPED at ε {eps:e}: both orders build SOUND at the closed form, at six minutes \
             a boolean; the microradian turn's row reads the same jet"
        );
        return;
    }
    let (a, b, want) = drums(FILED_TURN);
    if eps == 1e-6 {
        for (label, r) in unions(&a, &b) {
            let line = outcome(r, want, Tol::witness());
            assert!(
                line.starts_with("OK BAD") && line.contains("want=2.660151791"),
                "{label}: builds over the budget: {line}"
            );
        }
        return;
    }
    for (label, r) in unions(&a, &b) {
        assert!(
            matches!(
                &r,
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::Coincidence(Coincide::SectorSide, _),
                    diag,
                }) if diag.predicate == Some("enters_material")
            ),
            "{label}: the sector bound's rise escalates in band, got {:?}",
            r.err()
        );
    }
}
