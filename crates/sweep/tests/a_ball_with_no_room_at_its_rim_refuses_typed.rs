//! **A rim's ball that does not exist is refused as that, never read.**
//!
//! A curved rim's rolling ball has its centre where the two supports'
//! `r`-offsets cross, and past some radius they do not. That radius is
//! decided under `fillet3_ball_room` when the link resolves, so no arm
//! takes the root of a negative number and no later predicate — the
//! clearance screen above all — is handed the poison such a root
//! would carry. These rows ladder each curved-rim family the arms
//! cover through and past its ball's room, and read every refusal's
//! margins: none is a non-value.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRefusal, Filleted, fillet_edges};
use sweep::test_support::{
    dome, domed_cavity, finished, hemisphere_on_flat_base, lantern, one_edge_rim_at,
    revolved_about_y, rim_arcs_at,
};
use topo::{Body, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

/// **The lentil**: the solid between two unit spheres centred at
/// `(0, ∓0.6)`, bored on-axis at `0.6`, whose equator at radius `0.8`
/// is a CONVEX sphere×sphere rim — two circle traces in the meridian.
/// The offset circles `1 − r` about centres `1.2` apart cross while
/// `r < 0.4`.
fn lentil() -> Body<f64> {
    let bulge = (0.96f64.acos() / 4.0).tan();
    revolved_about_y(
        vec![
            (Point2::new(0.6, -0.2), bulge),
            (Point2::new(0.8, 0.0), bulge),
            (Point2::new(0.6, 0.2), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The radii every family is laddered through: from one that clears
/// every fixture here to past each one's ball.
const LADDER: [f64; 12] = [
    0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.4, 0.5, 0.6, 0.75, 0.9, 1.2,
];

/// Every margin a refusal carries, read off its `Debug` form: a
/// non-value reading prints as `MarginDiag(Invalid)` wherever it is
/// nested, so the read covers every variant without naming one.
fn assert_every_margin_is_a_reading(what: &str, r: f64, out: &Result<Filleted<f64>, BlendRefusal>) {
    if let Err(BlendRefusal { error, .. }) = out {
        assert!(
            !format!("{error:?}").contains("MarginDiag(Invalid)"),
            "{what} at r = {r}: a refusal carries a non-value margin: {error_at}\n  {error:?}",
            error_at = error.render(geom_brep::recourse::Reading::Build)
        );
    }
}

/// The ladder on one rim: every refusal's margins are readings, and the
/// first radius the ball has no room at, if the family reaches one
/// before another predicate answers, is `room_from`'s.
fn ladder<F>(what: &str, body: &F, rim: &[EdgeKey], room_from: Option<f64>)
where
    F: topo::FinishedBody<Scalar = f64>,
{
    let mut first_no_room = None;
    for r in LADDER {
        let out = fillet_edges(body, rim, r, tol());
        assert_every_margin_is_a_reading(what, r, &out);
        if let Err(BlendRefusal {
            error: BlendError::BallDoesNotFit { margin, radius, .. },
            ..
        }) = &out
        {
            assert_eq!(margin.predicate, "fillet3_ball_room", "{what} at r = {r}");
            assert_ne!(
                margin.sign,
                Sign::Positive,
                "{what} at r = {r}: refused on room"
            );
            assert_eq!(*radius, r, "{what}: the refusal names the request");
            first_no_room.get_or_insert(r);
        }
    }
    assert_eq!(
        first_no_room, room_from,
        "{what}: the first radius on the ladder whose ball has no room"
    );
}

/// **Plane × sphere, the ball inside the sphere** (the dome and the
/// pole-touching hemisphere, both on the plane through the sphere's
/// centre): the room is `R − 2r`, gone at `r = R/2`.
#[test]
fn a_plane_sphere_rim_refuses_its_ball_past_half_the_sphere() {
    let d = finished("dome", dome(1.0, tol()), tol());
    let rim = one_edge_rim_at(&d, 1.0, 0.0);
    ladder("dome equator", &d, &[rim], Some(0.5));
    let h = finished("hemisphere", hemisphere_on_flat_base(1.0, tol()), tol());
    let rim = rim_arcs_at(&h, 1.0, 0.0);
    ladder("hemisphere equator", &h, &rim, Some(0.5));
}

/// **Plane × sphere in the fold's fourth quadrant** — a concave rim
/// against a pocket, the ball in the void inside a sphere of radius
/// `0.5` and on the floor through its centre: room `0.5 − 2r`, gone at
/// `r = 0.25`.
#[test]
fn a_pockets_floor_rim_refuses_its_ball_past_half_the_pocket() {
    let c = finished("cavity", domed_cavity(tol()), tol());
    let rim = rim_arcs_at(&c, 0.5, 0.3);
    ladder("cavity floor", &c, &rim, Some(0.25));
}

/// **Sphere × sphere, through the sheet reduction**: the lentil's
/// equator, two circle traces, whose room is gone at `r = 0.4`.
#[test]
fn a_sphere_sphere_rim_refuses_its_ball_where_the_offset_circles_part() {
    let l = finished("lentil", lentil(), tol());
    let rim = one_edge_rim_at(&l, 0.8, 0.0);
    ladder("lentil equator", &l, &[rim], Some(0.4));
}

/// **Sphere × cone through the sheet reduction, and plane × sphere
/// through its own arm, on one body**: the lantern's shoulder and neck.
/// The neck is a plane × sphere rim on the plane through the belly's
/// centre, gone at `r = 0.5`. The shoulder is a line and a circle — the
/// unit belly about the origin and the cone `x + y = 1.4` in the
/// meridian — whose room `|1 − r| − |1.4 − r√2|/√2` lasts past where
/// the spine folds and the belly's curvature runs out, and is gone on
/// this ladder at `r = 1.2`.
#[test]
fn a_lanterns_rims_refuse_their_balls_where_the_offset_traces_part() {
    let lan = finished("lantern", lantern(tol()), tol());
    let neck = rim_arcs_at(&lan, 1.0, 0.0);
    ladder("lantern neck", &lan, &neck, Some(0.5));
    let shoulder = rim_arcs_at(&lan, 0.8, 0.6);
    ladder("lantern shoulder", &lan, &shoulder, Some(1.2));
}
