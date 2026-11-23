//! **A body built on a rod's end cap glues as its rim reads.** Each
//! fixture's walls both end on the cap's rim (or the inner rim of a
//! pipe), and the glue door's verdict on every pair of faces sharing that
//! rim is pinned (`topo::boolean::rim_door`): a junction where the
//! routing reads the π seam, the walls leave on opposite sides and both
//! bodies' own rim edges are convex creases; the tangency the door read
//! before the junction lane where one of those edges is concave; no glue
//! where the walls leave on one side or meet at a cusp; no tangency at a
//! corner. Then the unions: the closed form where one builds, sound at
//! tier 3 and 3′, or a typed refusal, never a wrong body.
//!
//! The fixtures are the two reviews' probes of PR 7432 and the rod rows
//! of `a_rod_unions_with_what_ends_on_its_rim`, rewritten as rows. A
//! mutant of the junction's three reads (routing, departures, convex
//! rim edges) turns a door row red where the union's outcome alone
//! would not (`probes/heldrecords-elbow/README.md`, the mutant table).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::SurfaceKind::{Cone, Cylinder, Sphere};
use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::boolean::RimDoor;
use topo::{AtRestBody, BooleanResult};

fn body(what: &str, verts: Vec<(Point2<f64>, f64)>, rev: Revolution<f64>) -> AtRestBody<f64> {
    finished(
        what,
        revolved_about_y(verts, rev, Tol::witness()),
        Tol::witness(),
    )
}

fn p(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

/// The bulge of a quarter circle.
fn q() -> f64 {
    (PI / 8.0).tan()
}

/// The rod: radius 1, `y ∈ [−2, 0]`.
fn rod() -> AtRestBody<f64> {
    body(
        "rod",
        vec![
            p(0.0, -2.0, 0.0),
            p(1.0, -2.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.0, 0.0, 0.0),
        ],
        Revolution::Full,
    )
}

/// The unit ball's half above `y = 0`.
fn half_ball() -> AtRestBody<f64> {
    body(
        "half ball",
        vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)],
        Revolution::Full,
    )
}

/// The rod with a flange 0.3 thick and 1.5 wide on top: its wall ends on
/// the rim `r = 1, y = 0` at a CONCAVE edge, the flange's underside.
fn flanged_rod() -> AtRestBody<f64> {
    body(
        "flanged rod",
        vec![
            p(0.0, -2.0, 0.0),
            p(1.0, -2.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.5, 0.0, 0.0),
            p(1.5, 0.3, 0.0),
            p(0.0, 0.3, 0.0),
        ],
        Revolution::Full,
    )
}

/// The unit ball's half below `y = 0`, hanging in the rod's mouth.
fn bowl() -> AtRestBody<f64> {
    body(
        "bowl",
        vec![p(0.0, -1.0, q()), p(1.0, 0.0, 0.0), p(0.0, 0.0, 0.0)],
        Revolution::Full,
    )
}

/// A pipe: radii 0.6 to 1, `y ∈ [−2, 0]`.
fn pipe() -> AtRestBody<f64> {
    body(
        "pipe",
        vec![
            p(0.6, -2.0, 0.0),
            p(1.0, -2.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.6, 0.0, 0.0),
        ],
        Revolution::Full,
    )
}

/// The ball of radius 0.6's upper half, on the pipe's inner rim.
fn small_half_ball() -> AtRestBody<f64> {
    body(
        "small half ball",
        vec![p(0.0, 0.0, 0.0), p(0.6, 0.0, q()), p(0.0, 0.6, 0.0)],
        Revolution::Full,
    )
}

/// A frustum: radius 1 at `y = 0` to 0.8 at `y = 1`.
fn frustum() -> AtRestBody<f64> {
    body(
        "frustum",
        vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(0.8, 1.0, 0.0),
            p(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
    )
}

/// The half ball revolved a quarter turn: its sphere ends on a quarter
/// of the rim.
fn quarter_ball() -> AtRestBody<f64> {
    body(
        "quarter ball",
        vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)],
        Revolution::Partial(PI / 2.0),
    )
}

/// **The door's row**: every pair of `a`'s and `b`'s faces that share a
/// rim and whose kinds are `walls` reads `want`, there is at least one,
/// and every other pair reads no tangency (a corner).
#[track_caller]
fn door(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    walls: (geom::SurfaceKind, geom::SurfaceKind),
    want: RimDoor,
) {
    let rows = topo::boolean::rim_door(a, b, Tol::witness());
    let (w, rest): (Vec<_>, Vec<_>) = rows.into_iter().partition(|(kinds, _)| *kinds == walls);
    assert!(!w.is_empty(), "{what}: the walls share the rim");
    assert!(
        w.iter().all(|(_, d)| *d == want),
        "{what}: every wall pair reads {want:?}: {w:?}"
    );
    assert!(
        rest.iter().all(|(_, d)| *d == RimDoor::NotTangent),
        "{what}: every other pair meets at a corner: {rest:?}"
    );
}

#[test]
fn a_half_ball_on_the_rods_cap_glues_a_junction() {
    door(
        "half ball",
        &rod(),
        &half_ball(),
        (Cylinder, Sphere),
        RimDoor::Junction,
    );
}

#[test]
fn a_quarter_turn_half_ball_glues_a_junction_on_its_quarter_of_the_rim() {
    door(
        "quarter ball",
        &rod(),
        &quarter_ball(),
        (Cylinder, Sphere),
        RimDoor::Junction,
    );
}

/// The flange's underside closes the rod's wedge at the rim ABOVE π, so
/// the half ball's sphere leaves the rim inside the rod's material: no
/// junction. The door reads the rim as before the junction lane, a
/// tangency along its turn.
#[test]
fn a_half_ball_on_a_flanged_rod_is_no_junction_and_glues_as_before() {
    door(
        "flanged",
        &flanged_rod(),
        &half_ball(),
        (Cylinder, Sphere),
        RimDoor::Tangent,
    );
}

/// The bowl's sphere leaves the rim on the rod's own side: no junction,
/// and the routing leaves it unglued.
#[test]
fn a_bowl_in_the_rods_mouth_is_not_glued() {
    door(
        "bowl",
        &rod(),
        &bowl(),
        (Cylinder, Sphere),
        RimDoor::Unrouted,
    );
}

/// The small half ball's outward normal opposes the pipe's inner wall's
/// across the inner rim: a cusp, not glued.
#[test]
fn a_half_ball_on_a_pipes_inner_rim_is_a_cusp_and_not_glued() {
    door(
        "cusp",
        &pipe(),
        &small_half_ball(),
        (Cylinder, Sphere),
        RimDoor::Unrouted,
    );
}

/// The frustum's cone meets the rod's wall at a corner all round.
#[test]
fn a_frustum_on_the_rods_cap_meets_it_at_a_corner() {
    door(
        "frustum",
        &rod(),
        &frustum(),
        (Cylinder, Cone),
        RimDoor::NotTangent,
    );
}

/// **The flanged rod and the half ball union to the closed form**, on
/// the tangency the door read before the junction lane: the rod and its
/// flange, and the half ball's cap above the flange's top.
#[test]
fn a_half_ball_on_a_flanged_rod_unions_to_its_closed_form() {
    let t = 0.3;
    let h = 1.0 - t;
    let want = 2.0 * PI + PI * 1.5 * 1.5 * t + PI * h * h * (3.0 - h) / 3.0;
    for r in outcome(&flanged_rod(), &half_ball()) {
        let v = r.unwrap_or_else(|e| panic!("the flanged union builds: {e}"));
        assert!((v - want).abs() <= 1e-9 * want, "volume {v} against {want}");
    }
}

/// What the union of `a` and `b` answers in both orders: `Ok(volume)` for
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
            let at_rest = topo::test_support::finished("the union", bb.body, Tol::witness());
            Ok(topo::test_support::pinned_volume("the union", &at_rest, Tol::witness()).volume)
        }
        Ok(other) => Err(format!("BUILT non-body {other:?}")),
        Err(e) => Err(format!("REFUSED {:?}: {e}", e.kind())),
    })
}

/// A union row: a wrong volume or an unsound body is red; a typed
/// refusal passes (the row's job is to find a WRONG body).
#[track_caller]
fn never_wrong(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    for (i, r) in outcome(a, b).into_iter().enumerate() {
        match r {
            Ok(v) => assert!(
                (v - want).abs() <= 1e-9 * want,
                "{what} order {i}: volume {v} vs {want}"
            ),
            Err(e) => assert!(e.starts_with("REFUSED"), "{what} order {i}: {e}"),
        }
    }
}

#[test]
fn no_body_on_the_rods_cap_unions_to_a_wrong_solid() {
    let unit = 2.0 * PI;
    never_wrong("half ball", &rod(), &half_ball(), unit + 2.0 / 3.0 * PI);
    never_wrong("quarter ball", &rod(), &quarter_ball(), unit + PI / 6.0);
    never_wrong("bowl", &rod(), &bowl(), unit);
    never_wrong(
        "cusp",
        &pipe(),
        &small_half_ball(),
        PI * (1.0 - 0.36) * 2.0 + 2.0 / 3.0 * PI * 0.216,
    );
    never_wrong(
        "frustum",
        &rod(),
        &frustum(),
        unit + PI / 3.0 * (1.0 + 0.8 + 0.64),
    );
    // A widening frustum, 1 to 1.3: a crease of the other sign.
    let wide = body(
        "wide frustum",
        vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.3, 1.0, 0.0),
            p(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
    );
    never_wrong(
        "wide frustum",
        &rod(),
        &wide,
        unit + PI / 3.0 * (1.0 + 1.3 + 1.69),
    );
    // A whole ball centred on the cap: one sphere face through the rim.
    let ball = body(
        "ball",
        vec![p(0.0, -1.0, q()), p(1.0, 0.0, q()), p(0.0, 1.0, 0.0)],
        Revolution::Full,
    );
    never_wrong("ball", &rod(), &ball, unit + 2.0 / 3.0 * PI);
    // A trumpet flare: a torus leaving the rim G1, curving away from the
    // rod's axis.
    let flare = body(
        "flare",
        vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, -q()),
            p(2.0, 1.0, 0.0),
            p(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
    );
    never_wrong("flare", &rod(), &flare, unit + PI * (14.0 / 3.0 - PI));
    // A flange whose torus wall leaves the rim with its outward normal
    // opposed to the rod's: a cusp, the two sharing only the rim.
    let flange = body(
        "flange",
        vec![
            p(1.0, 0.0, 0.0),
            p(3.0, 0.0, 0.0),
            p(3.0, 1.0, 0.0),
            p(2.0, 1.0, q()),
        ],
        Revolution::Full,
    );
    never_wrong("flange", &rod(), &flange, unit + PI * (13.0 / 3.0 + PI));
    // A bowl hanging in a wider pipe's mouth (inner radius 1, outer 1.4):
    // the nested touch on an inner rim.
    let wide_pipe = body(
        "wide pipe",
        vec![
            p(1.0, -2.0, 0.0),
            p(1.4, -2.0, 0.0),
            p(1.4, 0.0, 0.0),
            p(1.0, 0.0, 0.0),
        ],
        Revolution::Full,
    );
    never_wrong(
        "pipe and bowl",
        &wide_pipe,
        &bowl(),
        PI * (1.96 - 1.0) * 2.0 + 2.0 / 3.0 * PI,
    );
    // A trumpet bell: a torus of minor radius 0.5 leaving the rim G1 and
    // flaring outward, so the rod's wall continued past the rim lies on
    // the bell's material side of the bell's carrier while the bell has
    // no material below the rim. Its volume, ∫₀^ρ π(1 + ρ − √(ρ² − y²))² dy,
    // in closed form: π(a²ρ − 2aπρ²/4 + ρ³·2/3) with a = 1 + ρ.
    let rho: f64 = 0.5;
    let bell = body(
        "bell",
        vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 0.0, -q()),
            p(1.0 + rho, rho, 0.0),
            p(0.0, rho, 0.0),
        ],
        Revolution::Full,
    );
    let a = 1.0 + rho;
    let bell_volume = PI * (a * a * rho - a * PI * rho * rho / 2.0 + 2.0 / 3.0 * rho.powi(3));
    never_wrong("bell", &rod(), &bell, unit + bell_volume);
}

/// The half ball under ∖ and ∩: the rod ∖ the half ball is the rod, the
/// half ball ∖ the rod is the half ball, and their ∩ is empty, or each a
/// typed refusal.
#[test]
fn the_half_ball_under_subtract_and_intersect_is_never_a_wrong_solid() {
    let (rod, half) = (rod(), half_ball());
    for (what, r, want) in [
        (
            "rod ∖ half ball",
            topo::subtract(&rod, &half, Tol::witness()),
            Some(2.0 * PI),
        ),
        (
            "half ball ∖ rod",
            topo::subtract(&half, &rod, Tol::witness()),
            Some(2.0 / 3.0 * PI),
        ),
        (
            "rod ∩ half ball",
            topo::intersect(&rod, &half, Tol::witness()),
            None,
        ),
    ] {
        match r {
            Ok(BooleanResult::Body(bb)) => {
                let at_rest = topo::test_support::finished(what, bb.body, Tol::witness());
                let v = topo::test_support::pinned_volume(what, &at_rest, Tol::witness()).volume;
                let want = want.unwrap_or_else(|| panic!("{what}: builds a body, volume {v}"));
                assert!(
                    (v - want).abs() <= 1e-9 * want,
                    "{what}: volume {v} vs {want}"
                );
            }
            Ok(_) => assert!(want.is_none(), "{what}: empty"),
            Err(_) => {}
        }
    }
}
