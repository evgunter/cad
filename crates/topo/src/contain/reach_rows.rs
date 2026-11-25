//! **A crossing's containment is read for every parameter it could be
//! stored at** ([`super::placed_within_reach`]).
//!
//! The arc is the quadratic Bézier `x(t) = 2t`, `y(t) = 4t(1 − t)·H` in
//! `z = 0`. It rises through the brick's face `y = y(¼)` at `x = ½` with
//! slope `H`, a crossing angle of about 0.57°, so every parameter whose
//! point lies within `zero / H` along `x` of the true crossing reads ON
//! the face's plane: each is a parameter a check that reads the point ON
//! the plane admits, and the stretch they fill is that check's reach.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::solved::{Checked, Reach, decide_admit};
use geom_core::{Band, Margin, Point3, Tol, Vec3};

use super::{BeyondReach, ContainError, FaceContainment, contfp_decided, placed_within_reach};
use crate::Body;
use crate::test_support::brick;

/// The arc's height parameter, and its slope where it crosses the face.
const H: f64 = 1e-2;

/// The arc's closed form at `t`.
fn arc(t: f64) -> Point3<f64> {
    Point3::new(2.0 * t, 4.0 * t * (1.0 - t) * H, 0.0)
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's band")
}

/// The length of the arc's stretch that reads ON the face's plane.
fn stretch() -> f64 {
    2.0 * band().zero() / H
}

/// The arc's point over `x`, placed on the face `y = y(¼)` of a brick
/// whose trim edge stands at `x = x1`: read as a closed form's point
/// (`reach` is `None`), or as a crossing admitted by its point reading ON
/// the plane, with `reach` along.
fn placed(
    x1: f64,
    x: f64,
    reach: Option<f64>,
) -> Result<Result<FaceContainment, BeyondReach>, ContainError> {
    let y = arc(0.25).y;
    let b: Body<f64> = brick((-0.5, x1), (y, y + 3.0), (-1.0, 1.0), Tol::witness());
    let (face, plane) = b
        .faces()
        .find_map(|(k, _)| {
            let plane = crate::face_normal::face_plane(&b, k)?;
            (plane.normal.dot(Vec3::unit_y()) < -0.5).then_some((k, plane))
        })
        .expect("the brick has the face");
    let p = arc(0.5 * x);
    let off = (p - plane.origin).dot(plane.normal);
    let reach: Option<Reach> = reach.map(|along| {
        let (admitted, ()) = decide_admit::<f64, ()>("reach_rows_on_plane", 0.5 * x, band(), |_| {
            Checked {
                residual: Margin::of(off),
                reach: along,
                witness: (),
            }
        })
        .unwrap_or_else(|why| panic!("x = {x}: the arc's point reads ON the plane: {why:?}"));
        admitted.into_parts().1
    });
    placed_within_reach(
        reach,
        band(),
        |band| contfp_decided(&b, face, plane.normal, p, band),
        |&(at, _)| {
            matches!(
                at,
                FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)
            )
            .then_some(at)
        },
    )
    .map(|placed| placed.map(|(at, _)| at))
}

/// **The containment of a shallow crossing answers one way for every
/// parameter its check admits, or refuses.** The face's trim edge stands
/// two escalation lengths past the true crossing, inside the stretch.
///
/// Each point read for itself, as a closed form's is, answers its own
/// way: `In` at the true crossing; `OnEdge` over the trim edge;
/// escalated five zero bands short of it; `Out` three escalation lengths
/// past it. Two definite topologies and a third verdict, from four
/// parameters of one crossing.
///
/// Read with the stretch as their reach, none of the four answers: the
/// point over the edge is beyond its reach (the reach is not inside the
/// band's zero, so no boundary reading is every candidate's), and the
/// other three escalate, the edge within the reach of each. With the
/// trim edge ten reaches away, every parameter of the stretch reads `In`.
#[test]
fn a_shallow_crossings_containment_answers_one_way_or_refuses() {
    let (zero, escalate) = (band().zero(), band().escalate());
    let x1 = 0.5 + 2.0 * escalate;
    let (truth, over, short, past) = (0.5, x1, x1 - 5.0 * zero, x1 + 3.0 * escalate);
    let alone = |x: f64| placed(x1, x, None);
    assert!(
        matches!(alone(truth), Ok(Ok(FaceContainment::In)))
            && matches!(alone(over), Ok(Ok(FaceContainment::OnEdge(_))))
            && alone(short).is_err()
            && matches!(alone(past), Ok(Ok(FaceContainment::Out))),
        "each point read for itself: {:?} {:?} {:?} {:?}",
        alone(truth),
        alone(over),
        alone(short),
        alone(past)
    );
    let reaching = |x: f64| placed(x1, x, Some(stretch()));
    assert!(
        matches!(reaching(over), Ok(Err(BeyondReach))),
        "over the trim edge: {:?}",
        reaching(over)
    );
    for x in [truth, short, past] {
        assert!(
            matches!(reaching(x), Err(ContainError::Escalated { .. })),
            "x = {x}: the trim edge lies within the reach: {:?}",
            reaching(x)
        );
    }
    let far = 0.5 + 10.0 * (escalate + stretch());
    for along in [-0.9, -0.4, 0.0, 0.4, 0.9] {
        let x = 0.5 + along * zero / H;
        let got = placed(far, x, Some(stretch()));
        assert!(
            matches!(got, Ok(Ok(FaceContainment::In))),
            "x = {x}: no trim edge within the reach: {got:?}"
        );
    }
}

/// **A boundary verdict holds where the reach is inside the band's
/// zero.** The point over the trim edge, admitted with a reach of a
/// thousandth of the zero band (a well-conditioned crossing's), reads
/// `OnEdge` as a closed form's does.
#[test]
fn a_crossing_reaching_less_than_the_zero_band_keeps_its_boundary_verdict() {
    let x1 = 0.5 + 2.0 * band().escalate();
    let got = placed(x1, x1, Some(1e-3 * band().zero()));
    assert!(
        matches!(got, Ok(Ok(FaceContainment::OnEdge(_)))),
        "over the trim edge: {got:?}"
    );
}
