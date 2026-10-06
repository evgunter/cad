//! A plane along one of a body's own faces, under the normal that left
//! the edges described in that face's chart on the ring of the null
//! face the section is promoted from. Both section faces leave the
//! chart with those edges restated, so each split answers with its true
//! volumes and closed sides.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::splitting::{SplitPlane, split};
use topo::{Body, mass_properties, validate_closed};

fn extruded(points: &[((f64, f64), f64)]) -> Body<f64> {
    let lp = bulge_loop(
        points
            .iter()
            .map(|&((x, y), b)| (Point2::new(x, y), b))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(
        &vp,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn revolved(points: &[(f64, f64)]) -> Body<f64> {
    use crate::revolve_common::{axis_y, validated};
    use profile::RawLoop;
    let lp = profile::ProfileLoop::polygon(points.iter().map(|&(x, y)| Point2::new(x, y)));
    sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Splits `body` by the plane through `origin` along `normal`, and holds
/// each side to a closed solid of the volume wanted.
fn answers(label: &str, body: Body<f64>, origin: (f64, f64), normal: (f64, f64), want: (f64, f64)) {
    let operand = sweep::test_support::finished(label, body, Tol::witness());
    let n = Vec3::new(normal.0, normal.1, 0.0);
    let plane: SplitPlane<f64> = topo::test_support::split_plane(
        Point3::new(origin.0, origin.1, 0.0),
        n / n.norm(),
        Tol::witness(),
    );
    let r = split(&operand, &plane, Tol::witness()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
    for (side, part, want) in [("above", &r.above, want.0), ("below", &r.below, want.1)] {
        let b = part
            .body()
            .unwrap_or_else(|| panic!("{label}: no {side} side"));
        assert_eq!(validate_closed(b), Ok(()), "{label}: {side} is closed");
        let v = mass_properties(b, Tol::witness()).unwrap().volume;
        assert!(
            (v - want).abs() <= 1e-9 * want.max(1.0),
            "{label}: {side} {v}, want {want}"
        );
    }
}

/// A comb of three teeth on a 5 × 1 spine, cut along the teeth's roots
/// (y = 1, three section components) and along the first tooth's side
/// (x = 1), each with the normal toward the spine's origin side.
#[test]
fn a_comb_split_along_its_roots_or_a_tooth_side_answers() {
    let comb = || {
        extruded(&[
            ((0.0, 0.0), 0.0),
            ((5.0, 0.0), 0.0),
            ((5.0, 2.0), 0.0),
            ((4.0, 2.0), 0.0),
            ((4.0, 1.0), 0.0),
            ((3.0, 1.0), 0.0),
            ((3.0, 2.0), 0.0),
            ((2.0, 2.0), 0.0),
            ((2.0, 1.0), 0.0),
            ((1.0, 1.0), 0.0),
            ((1.0, 2.0), 0.0),
            ((0.0, 2.0), 0.0),
        ])
    };
    answers("comb roots", comb(), (0.0, 1.0), (0.0, -1.0), (5.0, 3.0));
    answers("comb side", comb(), (1.0, 0.0), (-1.0, 0.0), (2.0, 6.0));
}

/// A disc (r = 2, height 0.5) with a boss (r = 0.5, height 1), revolved,
/// cut along the shoulder the boss stands on (y = 0.5, normal −y): the
/// section is the annulus between the boss and the rim.
#[test]
fn a_boss_split_along_its_shoulder_answers() {
    use std::f64::consts::PI;
    let boss = revolved(&[
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 0.5),
        (0.5, 0.5),
        (0.5, 1.5),
        (0.0, 1.5),
    ]);
    answers(
        "boss shoulder",
        boss,
        (0.0, 0.5),
        (0.0, -1.0),
        (2.0 * PI, PI / 4.0),
    );
}

/// An L (legs 3 × 1) whose inner corner carries a quarter arc of
/// r = 0.5 from (1.5, 1) to (1, 1.5), with no declared tangent joint, cut
/// along the foot's top (y = 1, normal −y).
#[test]
fn an_l_split_along_the_face_its_round_leaves_answers() {
    let q = (std::f64::consts::PI / 8.0).tan();
    let l = extruded(&[
        ((0.0, 0.0), 0.0),
        ((3.0, 0.0), 0.0),
        ((3.0, 1.0), 0.0),
        ((1.5, 1.0), q),
        ((1.0, 1.5), 0.0),
        ((1.0, 3.0), 0.0),
        ((0.0, 3.0), 0.0),
    ]);
    let below = 2.0 + std::f64::consts::PI / 16.0;
    answers("L round", l, (1.5, 1.0), (0.0, -1.0), (3.0, below));
}
