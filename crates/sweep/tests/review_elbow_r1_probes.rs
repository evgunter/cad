//! **Review probes for the rim-junction glue door (PR 7432, R1)**: bodies
//! built on a rod's end cap whose faces end on the cap's rim, in poses
//! the routing-first door has to keep honest — a π flare, a cusp flange,
//! a rim shared over half its length, a ball whose lower hemisphere
//! leaves the rim into the rod's own material, and a bowl hanging in the
//! rod's mouth. Each row: the union builds the closed-form volume, sound
//! at tier 3 and 3′, or refuses typed; never a wrong body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult};

fn body(what: &str, verts: Vec<(Point2<f64>, f64)>, rev: Revolution<f64>) -> AtRestBody<f64> {
    finished(what, revolved_about_y(verts, rev, Tol::witness()), Tol::witness())
}

/// The rod: radius 1, `y ∈ [−2, 0]`.
fn rod() -> AtRestBody<f64> {
    body(
        "rod",
        vec![
            (Point2::new(0.0, -2.0), 0.0),
            (Point2::new(1.0, -2.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
    )
}

const Q: f64 = PI / 8.0;

/// What the union of `a` and `b` (both orders) answers: `Ok(volume)` for
/// one sound solid, else the refusal or the failed check, in words.
fn outcome(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Result<f64, String>; 2] {
    [(a, b), (b, a)].map(|(x, y)| match topo::union(x, y, Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            let solids = bb.body.solids().count();
            let t3 = topo::validate_geometric(&bb.body, Tol::witness());
            let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness());
            if solids != 1 || t3.is_err() || t3p.is_err() {
                return Err(format!("BUILT solids {solids} t3 {t3:?} t3' {t3p:?}"));
            }
            Ok(topo::test_support::pinned_volume("u", &bb.body, Tol::witness()).volume)
        }
        Ok(other) => Err(format!("BUILT non-body {other:?}")),
        Err(e) => Err(format!("REFUSED {:?}: {e}", e.kind())),
    })
}

/// A row: a wrong volume or an unsound body is red; a typed refusal is
/// printed and passes (the probe's job is to find a WRONG verdict).
#[track_caller]
fn never_wrong(what: &str, b: &AtRestBody<f64>, want: f64) {
    for (i, r) in outcome(&rod(), b).into_iter().enumerate() {
        println!("PROBE {what} order {i}: {r:?} want {want}");
        match r {
            Ok(v) => assert!((v - want).abs() <= 1e-9 * want, "{what}: volume {v} vs {want}"),
            Err(e) => assert!(e.starts_with("REFUSED"), "{what}: {e}"),
        }
    }
}

/// A trumpet flare: its torus wall leaves the rim G1, curving away from
/// the rod's axis (κ_rel definite all round). π, opposite departures.
#[test]
fn r1_a_rod_unions_with_a_trumpet_flare() {
    let flare = body(
        "flare",
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), -Q.tan()),
            (Point2::new(2.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
    );
    never_wrong("flare", &flare, 2.0 * PI + PI * (14.0 / 3.0 - PI));
}

/// A flange whose torus wall leaves the rim with its outward normal
/// OPPOSED to the rod's: a cusp (0/2π). The two solids share only the rim.
#[test]
fn r1_a_rod_unions_with_a_cusp_flange() {
    let flange = body(
        "flange",
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(3.0, 0.0), 0.0),
            (Point2::new(3.0, 1.0), 0.0),
            (Point2::new(2.0, 1.0), Q.tan()),
        ],
        Revolution::Full,
    );
    never_wrong("flange", &flange, 2.0 * PI + PI * (13.0 / 3.0 + PI));
}

/// A half ball turned half way: its sphere face ends on half the rim.
#[test]
fn r1_a_rod_unions_with_a_half_turn_half_ball() {
    let half = body(
        "half-turn half ball",
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), Q.tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(PI / 2.0),
    );
    never_wrong("quarter-turn half ball", &half, 2.0 * PI + PI / 6.0);
}

/// A whole ball centred on the cap: its upper hemisphere leaves the rim
/// G1 away from the rod (π), its lower one into the rod's own material
/// (same departure). The union is the rod and the upper half ball.
#[test]
fn r1_a_rod_unions_with_a_ball_centred_on_its_cap() {
    let ball = body(
        "ball",
        vec![
            (Point2::new(0.0, -1.0), Q.tan()),
            (Point2::new(1.0, 0.0), Q.tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
    );
    never_wrong("ball", &ball, 2.0 * PI + 2.0 / 3.0 * PI);
}

/// A bowl hanging in the rod's mouth: the lower half ball, inside the
/// rod, its sphere ending on the rim (same departure). The union is the
/// rod.
#[test]
fn r1_a_rod_unions_with_a_bowl_in_its_mouth() {
    let bowl = body(
        "bowl",
        vec![
            (Point2::new(0.0, -1.0), Q.tan()),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
    );
    never_wrong("bowl", &bowl, 2.0 * PI);
}
