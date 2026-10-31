//! **The torus germ row's placements through the germ mint**: a drum, a
//! ball and a donut against a moved donut, at the placements of
//! `torus-germ-pairs-have-no-section-frame` whose germs the mint refused
//! (`Untold` or `Refused(RoutesToGeneralRung)`), each run under ∪ in both
//! member orders. A germ's component is its site's foot on the carried
//! section, so each passes the mint and stops at a later door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

/// The row's six placements: B turned by `angle` about `axis` through
/// the origin, then moved by `t`.
#[allow(clippy::approx_constant)] // the row's second angle is a hair short of a right one
fn placements() -> [(f64, Vec3<f64>, Vec3<f64>); 6] {
    [
        (0.0, Vec3::unit_x(), Vec3::new(0.45, 0.1, 0.05)),
        (1.570_796_3, Vec3::unit_x(), Vec3::new(0.3, 0.2, 0.1)),
        (0.7, Vec3::unit_x(), Vec3::new(0.1, 0.35, 0.2)),
        (
            1.1,
            Vec3::new(0.3, 1.0, 0.2).normalize(),
            Vec3::new(0.42, -0.15, 0.3),
        ),
        (0.4, Vec3::unit_y(), Vec3::new(0.6, 0.0, 0.0)),
        (
            0.9,
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(0.05, 0.1, 0.02),
        ),
    ]
}

fn placed(b: &Body<f64>, (angle, axis, t): (f64, Vec3<f64>, Vec3<f64>)) -> AtRestBody<f64> {
    let r = Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    let moved = topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::from_parts(r.linear, r.translation + t),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap();
    finished("the moved shape", moved, Tol::witness())
}

/// The donut: `R = 0.6`, `r = 0.2`, revolved about `y`.
fn donut() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.6, -0.2), 1.0), (Point2::new(0.6, 0.2), 1.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The ball: radius `0.5` about the origin, poled on `y`.
fn ball() -> Body<f64> {
    ball_poled_y(0.5, Vec3::new(0.0, 0.0, 0.0), Tol::witness())
}

/// The drum: radius `0.5` about `z`, `z ∈ [−1, 1]`.
fn drum() -> Body<f64> {
    cyl(0.5, 1.0)
}

fn at_rest(b: &Body<f64>) -> AtRestBody<f64> {
    finished("the shape", b.clone(), Tol::witness())
}

/// **Where a run stops**, in a word the rows pin: each a door's own
/// typed refusal past the germ mint, or a body.
fn stop(got: &Result<BooleanResult<f64>, BooleanError>) -> String {
    use geom_brep::PropsError;
    use topo::{MassPropsError, ValidationError};
    match got {
        Ok(r) => format!("body {:?}", r.body().map(|b| b.kind)),
        Err(BooleanError::ResultInvalid { errors }) => match errors.as_slice() {
            [ValidationError::VolumeUncomputable { source, .. }] => match source {
                MassPropsError::Face {
                    source: PropsError::QuadratureUnsupported { what },
                    ..
                } if what.starts_with(
                    "curved-cut face half-edge carries a pcurve with no harmonic form",
                ) =>
                {
                    "projected".to_owned()
                }
                MassPropsError::Face {
                    source: PropsError::QuadratureUnsupported { what },
                    ..
                } if what.starts_with("conic trim on a cone/sphere/torus chart") => {
                    "conic trim".to_owned()
                }
                MassPropsError::RingOnCurvedFace { .. } => "ring on a curved face".to_owned(),
                other => format!("volume {other:?}"),
            },
            other => format!("result {other:?}"),
        },
        Err(BooleanError::GermSectionSpiric { .. }) => "spiric".to_owned(),
        Err(BooleanError::GermSectionUnbuilt {
            why: topo::SectionUnbuilt::March(geom_brep::SsiError::TubeStraddles { .. }),
            ..
        }) => "tube straddles".to_owned(),
        Err(BooleanError::Escalated {
            decision: topo::BooleanDecision::PierceCurvature,
            ..
        }) => "pierce curvature".to_owned(),
        Err(BooleanError::JoinCarrierUnsupported {
            carrier: geom::CurveKind::Nurbs,
            ..
        }) => "spline join".to_owned(),
        Err(BooleanError::CurvedBooleanUnsupported {
            operand,
            kind: geom::SurfaceKind::Torus,
            ..
        }) => format!("curved join {operand:?}"),
        Err(e) => format!("{e:?}"),
    }
}

/// `run`'s answer, and how many marches it ran
/// ([`geom_brep::implicit_pair_marches`]).
fn marching<R>(run: impl FnOnce() -> R) -> (R, usize) {
    let before = geom_brep::implicit_pair_marches();
    let got = run();
    (got, geom_brep::implicit_pair_marches() - before)
}

/// `a` against the donut moved to each of `places`, under ∪ in both
/// member orders (`A ∪ B`, then `B ∪ A`), against `want` by placement at
/// the compiled ε, each run marching its one marched pair once (the
/// section table carries a pair once per operation, for the germ mint,
/// the join and the certificate alike); `other_eps` names the
/// placements that stop elsewhere at another ε row, measured: at `1e-6`
/// a march's tube straddles
/// (`work/sectrung/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`).
fn row(
    a: fn() -> Body<f64>,
    places: &[usize],
    want: &[[&str; 2]],
    other_eps: &[(f64, usize, [&str; 2])],
) {
    let eps = Tol::witness().eps();
    let mut off = Vec::new();
    for (&i, want) in places.iter().zip(want) {
        let want = other_eps
            .iter()
            .find(|&&(e, k, _)| e == eps && k == i)
            .map_or(want, |(_, _, w)| w);
        let (a, b) = (at_rest(&a()), placed(&donut(), placements()[i]));
        let (ab, ab_marches) = marching(|| topo::union(&a, &b, Tol::witness()));
        let (ba, ba_marches) = marching(|| topo::union(&b, &a, Tol::witness()));
        let (got, marches) = ([stop(&ab), stop(&ba)], [ab_marches, ba_marches]);
        if got != *want {
            off.push(format!("placement {i}: {got:?}"));
        }
        if marches != [1, 1] {
            off.push(format!("placement {i}: marches by op {marches:?}"));
        }
    }
    assert!(off.is_empty(), "{off:#?}");
}

/// **A drum against a moved donut passes the germ mint at every
/// placement.** Each germ on the wall × tube pair takes its site's
/// foot on the carried section, the general placements' (1, 2, 3, 5)
/// listed by the march and the square walls' (0, 4) counted; every run
/// stops past the mint at a door's own refusal: the result gate's volume
/// on a face its fitted chords bound
/// (`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`)
/// or a conic trim on the torus chart, or the cap plane's spiric
/// (`GermSectionSpiric`). Placement 5 divides the drum's wall, holding a
/// ring, by a spline chord, and re-homes the ring by the path.
#[test]
fn a_drum_against_a_moved_donut_passes_the_germ_mint() {
    row(
        drum,
        &[0, 1, 2, 3, 4, 5],
        &[
            ["conic trim", "projected"],
            ["projected", "projected"],
            ["projected", "conic trim"],
            ["spiric", "spiric"],
            ["conic trim", "conic trim"],
            ["projected", "conic trim"],
        ],
        &[
            (1e-6, 2, ["tube straddles"; 2]),
            (1e-6, 3, ["tube straddles"; 2]),
            (1e-6, 5, ["tube straddles"; 2]),
        ],
    );
}

/// **A ball cutting both tube circles of its meridian** (placement 5):
/// two curves about the torus axis, counted, which no plane or meridian
/// tells apart; each germ's component is its site's foot, and the run
/// stops at the result gate's volume on a conic trim of the torus chart.
#[test]
fn a_ball_round_the_donuts_axis_passes_the_germ_mint() {
    row(ball, &[5], &[["conic trim", "conic trim"]], &[]);
}

/// **Two donuts in general pose** (placements 1, 2, 3): the section the
/// march's, each germ's component its site's foot. Each passes the join
/// and stops at the result gate's volume: a conic trim on the torus
/// chart, or a ring left on a curved face (`RingOnCurvedFace`).
#[test]
fn two_donuts_in_general_pose_pass_the_germ_mint() {
    row(
        donut,
        &[1, 2, 3],
        &[
            ["ring on a curved face", "conic trim"],
            ["conic trim", "conic trim"],
            ["conic trim", "conic trim"],
        ],
        &[
            (1e-6, 1, ["pierce curvature"; 2]),
            (1e-6, 2, ["tube straddles"; 2]),
        ],
    );
}
