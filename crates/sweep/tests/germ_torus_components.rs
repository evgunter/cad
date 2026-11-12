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
fn stop(got: &Result<BooleanResult<f64>, BooleanError>, kind: Option<geom::SurfaceKind>) -> String {
    use geom_brep::PropsError;
    use topo::{MassPropsError, ValidationError};
    // The refused face's own chart, read right after its boolean.
    let chart = match kind {
        Some(geom::SurfaceKind::Torus) => "torus",
        Some(geom::SurfaceKind::Sphere) => "sphere",
        Some(geom::SurfaceKind::Cylinder) => "cylinder",
        Some(geom::SurfaceKind::Cone) => "cone",
        Some(geom::SurfaceKind::Plane) => "plane",
        _ => "unread",
    };
    match got {
        Ok(r) => format!("body {:?}", r.body().map(|b| b.kind)),
        Err(BooleanError::ResultInvalid { findings: errors }) => match errors.as_slice() {
            [ValidationError::VolumeUncomputable { source, .. }] => match source {
                MassPropsError::Face {
                    source: PropsError::QuadratureUnsupported { what },
                    ..
                } if what.starts_with("a trimmed face with no volume lane on its chart") => {
                    format!("{chart} trim")
                }
                MassPropsError::RingOnCurvedFace { .. } => format!("ring on a {chart} face"),
                other => format!("volume {other:?}"),
            },
            other => format!("result {other:?}"),
        },
        Err(BooleanError::GermSectionSpiric { .. }) => "spiric".to_owned(),
        Err(BooleanError::CurvedPairUnsupported {
            site: topo::PairRefusalSite::InteriorLoopGuard,
            kind,
            other_kind,
            ..
        }) => format!("loop guard {kind:?} × {other_kind:?}"),
        Err(BooleanError::GermSectionUnbuilt {
            why:
                topo::SectionUnbuilt::Carry(geom_brep::CarryRefusal::March(
                    geom_brep::SsiError::TubeStraddles { .. },
                )),
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

/// A run that builds one seamed solid.
const BUILDS: &str = "body Some(Seamed)";

/// A run the interior-loop guard refuses on a torus × torus pair.
const TORUS_PAIR: &str = "loop guard Torus × Torus";

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
/// (`work/secttorus/the-torus-doors-tube-straddles-transverse-poses-at-a-coarse-band.md`).
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
        // Each run's refused face is read right after it
        // (`topo::boolean::refused_face_kind`).
        let kind = |r: &Result<BooleanResult<f64>, BooleanError>| {
            r.as_ref().err().and_then(topo::boolean::refused_face_kind)
        };
        let (ab, ab_marches) = marching(|| topo::union(&a, &b, Tol::witness()));
        let ab_kind = kind(&ab);
        let (ba, ba_marches) = marching(|| topo::union(&b, &a, Tol::witness()));
        let ba_kind = kind(&ba);
        let (got, marches) = (
            [stop(&ab, ab_kind), stop(&ba, ba_kind)],
            [ab_marches, ba_marches],
        );
        if got != *want {
            off.push(format!("placement {i}: {got:?}"));
        }
        if marches != [1, 1] {
            off.push(format!("placement {i}: marches by op {marches:?}"));
        }
        for (order, r) in [("A ∪ B", &ab), ("B ∪ A", &ba)] {
            if let Some(why) = union_floor(r, &a, &b) {
                off.push(format!("placement {i}, {order}: {why}"));
            }
        }
    }
    assert!(off.is_empty(), "{off:#?}");
}

/// **A union that builds lies between its operands' volumes**:
/// `max(V_a, V_b) ≤ V ≤ V_a + V_b`, each certified volume's pad held
/// against the bound. A sanity floor, not a truth: these poses have no
/// independent volume
/// (`work/spiricarm/the-general-pose-torus-germ-unions-that-build-have-no-volume-truth.md`).
pub(crate) fn union_floor(
    r: &Result<BooleanResult<f64>, BooleanError>,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
) -> Option<String> {
    let tol = Tol::witness();
    let body = r.as_ref().ok()?.body()?;
    let settle = || topo::test_support::pinned_resolution(tol);
    let v = topo::mass_properties(&body.body, tol, settle())
        .ok()?
        .enclosure;
    let va = a.measure(settle()).ok()?.enclosure;
    let vb = b.measure(settle()).ok()?.enclosure;
    let (lo, hi) = (v.volume - v.volume_pad, v.volume + v.volume_pad);
    let floor = (va.volume - va.volume_pad).max(vb.volume - vb.volume_pad);
    let ceiling = va.volume + va.volume_pad + vb.volume + vb.volume_pad;
    (hi < floor || lo > ceiling).then(|| {
        format!(
            "the union's volume {} ± {} lies outside [{floor}, {ceiling}]",
            v.volume, v.volume_pad
        )
    })
}

/// **A drum against a moved donut passes the germ mint at every
/// placement.** Each germ on the wall × tube pair takes its site's
/// foot on the carried section, the general placements' (1, 2, 3, 5)
/// listed by the march and the square walls' (0, 4) counted; every run
/// builds past the mint, its chorded walls and torus face measured through
/// the rotational lane, but placement 3, which stops at the cap plane's
/// spiric (`GermSectionSpiric`). Placement 5 divides the drum's wall,
/// holding a ring, by a spline chord, and re-homes the ring by the path.
/// No body here is held to a volume of its own
/// (`work/spiricarm/the-general-pose-torus-germ-unions-that-build-have-no-volume-truth.md`).
#[test]
fn a_drum_against_a_moved_donut_passes_the_germ_mint() {
    row(
        drum,
        &[0, 1, 2, 3, 4, 5],
        &[
            [BUILDS, BUILDS],
            [BUILDS, BUILDS],
            [BUILDS, BUILDS],
            ["spiric", "spiric"],
            [BUILDS, BUILDS],
            [BUILDS, BUILDS],
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
/// builds, the ball's chorded sphere face and the donut's torus face
/// measured through the rotational lane.
#[test]
fn a_ball_round_the_donuts_axis_passes_the_germ_mint() {
    row(ball, &[5], &[[BUILDS, BUILDS]], &[]);
}

/// **Two donuts in general pose** (placements 1, 2, 3): the section the
/// march's, each germ's component its site's foot. Each passes the join
/// and the result gate, its torus faces measured through the rotational
/// lane: placement 2 builds, and placements 1 and 3 stop past the gate at
/// the crossings path's interior-loop guard, whose section certificate
/// has no torus × torus arm
/// (`work/secttorus/two-donuts-in-general-pose-stop-at-the-interior-loop-guard.md`).
#[test]
fn two_donuts_in_general_pose_pass_the_germ_mint() {
    row(
        donut,
        &[1, 2, 3],
        &[
            [TORUS_PAIR, TORUS_PAIR],
            [BUILDS, BUILDS],
            [TORUS_PAIR, TORUS_PAIR],
        ],
        &[
            (1e-6, 1, ["pierce curvature"; 2]),
            (1e-6, 2, ["tube straddles"; 2]),
        ],
    );
}
