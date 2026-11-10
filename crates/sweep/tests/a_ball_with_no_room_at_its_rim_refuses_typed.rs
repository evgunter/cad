//! **A rim's ball that does not exist is refused as that, never read.**
//!
//! A curved rim's rolling ball has its centre where the two supports'
//! `r`-offsets cross, and past some radius they do not. That radius is
//! decided under `fillet3_ball_room` when the link resolves, so no arm
//! takes the root of a negative number and no later predicate — the
//! clearance screen above all — is handed the poison such a root would
//! carry. These rows ladder each curved-rim family the arms cover
//! through and past its ball's room, finely around the boundary, and
//! at every rung read the refusal against the room's closed form: no
//! refusal carries a non-value margin, every rung without room refuses
//! `BallDoesNotFit` at the closed form's margin, and no rung with room
//! does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Sign, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRefusal, fillet_edges};
use sweep::test_support::{
    boss, dome, domed_cavity, finished, hemisphere_on_flat_base, lantern, one_edge_rim_at,
    revolved_about_y, rim_arcs_at, sphere_zone,
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

/// The lantern shoulder's cone, `x + y = 1.4`, stands this far from the
/// belly's centre in the meridian.
const SHOULDER_CONE: f64 = 1.4 / core::f64::consts::SQRT_2;

/// The radii every family is laddered through, from one that clears
/// every fixture here to past the sphere's own radius, with the rungs
/// either side of `boundary` (the room's closed-form zero) added.
fn ladder_with(boundary: Option<f64>) -> Vec<f64> {
    let mut rs = vec![
        0.05, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2,
    ];
    if let Some(b) = boundary {
        rs.extend([b - 0.01, b - 0.001, b, b + 0.001, b + 0.01]);
    }
    rs.sort_by(f64::total_cmp);
    rs.dedup();
    rs
}

/// Where the room's closed form is this close to zero, the reading may
/// fall either side of the band, so the rung asserts only that it is
/// typed.
const NEAR: f64 = 1e-6;

/// The ladder on one rim against `room`, the closed form of
/// `fillet3_ball_room`'s margin at radius `r`:
///
/// - no refusal carries `MarginDiag(Invalid)`, read off its `Debug`
///   form wherever it is nested;
/// - where `room(r) < −NEAR` the rung refuses `BallDoesNotFit`, at
///   `room(r)`, decided Negative, naming the request;
/// - where `room(r) > NEAR` it does not refuse `BallDoesNotFit`.
///
/// Returns how many rungs had no room, so a family whose ladder must
/// cross its boundary can say so.
fn ladder<F>(
    what: &str,
    body: &F,
    rim: &[EdgeKey],
    boundary: Option<f64>,
    room: fn(f64) -> f64,
) -> usize
where
    F: topo::FinishedBody<Scalar = f64>,
{
    let mut no_room = 0;
    for r in ladder_with(boundary) {
        let out = fillet_edges(body, rim, r, tol());
        if let Err(BlendRefusal { error, .. }) = &out {
            assert!(
                !format!("{error:?}").contains("MarginDiag(Invalid)"),
                "{what} at r = {r}: a refusal carries a non-value margin: {error_at}\n  {error:?}",
                error_at = error.render(geom_brep::recourse::Reading::Build)
            );
        }
        let want = room(r);
        match &out {
            Err(BlendRefusal {
                error: BlendError::BallDoesNotFit { margin, radius, .. },
                ..
            }) => {
                assert!(
                    want <= NEAR,
                    "{what} at r = {r}: the room {want:e} is open, yet refused"
                );
                assert_eq!(margin.predicate, "fillet3_ball_room", "{what} at r = {r}");
                assert_eq!(*radius, r, "{what}: the refusal names the request");
                let got = margin
                    .reading
                    .diagnostic_f64_for_error_text()
                    .value()
                    .expect("an f64 reading");
                assert!(
                    (got - want).abs() < 1e-12,
                    "{what} at r = {r}: the room is the closed form {want:e}, got {got:e}"
                );
                if want < -NEAR {
                    assert_eq!(margin.sign, Sign::Negative, "{what} at r = {r}");
                }
                no_room += 1;
            }
            other => assert!(
                want > -NEAR,
                "{what} at r = {r}: no room ({want:e}), yet {}",
                match other {
                    Ok(_) => "it built".to_owned(),
                    Err(e) => e.error.render(geom_brep::recourse::Reading::Build),
                }
            ),
        }
    }
    no_room
}

/// **Plane × sphere, the ball inside the sphere** (the dome and the
/// pole-touching hemisphere, on the plane through the sphere's centre):
/// the room is `R − 2r`, gone at `r = R/2` and still falling past
/// `r = R`, where the offset sphere's radius `R − r` turns negative.
#[test]
fn a_plane_sphere_rim_refuses_its_ball_past_half_the_sphere() {
    let d = finished("dome", dome(1.0, tol()), tol());
    let rim = one_edge_rim_at(&d, 1.0, 0.0);
    let n = ladder("dome equator", &d, &[rim], Some(0.5), |r| 1.0 - 2.0 * r);
    assert!(n >= 10, "dome equator: {n} rungs past the boundary");
    let h = finished("hemisphere", hemisphere_on_flat_base(1.0, tol()), tol());
    let rim = rim_arcs_at(&h, 1.0, 0.0);
    let n = ladder("hemisphere equator", &h, &rim, Some(0.5), |r| 1.0 - 2.0 * r);
    assert!(n >= 10, "hemisphere equator: {n} rungs past the boundary");
}

/// **Plane × sphere in the fold's fourth quadrant**, plane first — a
/// concave rim against a pocket, the ball in the void inside a sphere
/// of radius `0.5` and on the floor through its centre: room
/// `0.5 − 2r`, gone at `r = 0.25`.
#[test]
fn a_pockets_floor_rim_refuses_its_ball_past_half_the_pocket() {
    let c = finished("cavity", domed_cavity(tol()), tol());
    let rim = rim_arcs_at(&c, 0.5, 0.3);
    let n = ladder("cavity floor", &c, &rim, Some(0.25), |r| 0.5 - 2.0 * r);
    assert!(n >= 10, "cavity floor: {n} rungs past the boundary");
}

/// **Plane × sphere off the centre, in both orders**: the #935 zone, a
/// sphere `R = 2` about the origin cut at `y = 1` (plane first) and at
/// `y = −0.5` (sphere first), the ball inside the sphere. The room is
/// `(2 − r) − |1 − r|` at the top, gone at `r = 1.5`, and
/// `(2 − r) − |r − 0.5|` at the bottom, gone at `r = 1.25`: a plane off
/// the centre holds more room than the hemisphere's on its far side.
#[test]
fn an_off_centre_plane_sphere_rim_refuses_its_ball_where_its_cap_runs_out() {
    let z = finished("zone", sphere_zone(0.5, Revolution::Full, tol()), tol());
    let hi = one_edge_rim_at(&z, 3f64.sqrt(), 1.0);
    let n = ladder("zone top", &z, &[hi], Some(1.5), |r| {
        (2.0 - r) - (1.0 - r).abs()
    });
    assert!(n >= 3, "zone top: {n} rungs past the boundary");
    let lo = one_edge_rim_at(&z, 3.75f64.sqrt(), -0.5);
    let n = ladder("zone bottom", &z, &[lo], Some(1.25), |r| {
        (2.0 - r) - (r - 0.5).abs()
    });
    assert!(n >= 3, "zone bottom: {n} rungs past the boundary");
}

/// **Plane × sphere with the ball outside the sphere**: the boss's dome
/// rim (concave, the ball in the void above the flat top) and the
/// dimple's (convex, the ball in the material around the pocket), on
/// the plane through the sphere's centre. The offset sphere is
/// `R + r`, so the room is `R = 0.5` at every radius and never runs
/// out.
#[test]
fn a_ball_outside_its_sphere_never_runs_out_of_room() {
    for (what, up) in [("boss dome rim", true), ("dimple rim", false)] {
        let b = finished(what, boss(up, tol()), tol());
        let rim = rim_arcs_at(&b, 0.5, 1.0);
        assert_eq!(ladder(what, &b, &rim, None, |_| 0.5), 0, "{what}");
    }
}

/// **Sphere × sphere, through the sheet reduction**: the lentil's
/// equator, two circle traces, whose offsets' secant margin is
/// `2(1 − r) − 1.2`, gone at `r = 0.4`.
#[test]
fn a_sphere_sphere_rim_refuses_its_ball_where_the_offset_circles_part() {
    let l = finished("lentil", lentil(), tol());
    let rim = one_edge_rim_at(&l, 0.8, 0.0);
    let n = ladder("lentil equator", &l, &[rim], Some(0.4), |r| 0.8 - 2.0 * r);
    assert!(n >= 10, "lentil equator: {n} rungs past the boundary");
}

/// **Sphere × cone through the sheet reduction, and plane × sphere
/// through its own arm, on one body**: the lantern's neck and shoulder.
/// The neck is a plane × sphere rim on the plane through the belly's
/// centre, gone at `r = 0.5`. The shoulder is a line and a circle — the
/// unit belly about the origin and the cone `x + y = 1.4` in the
/// meridian, [`SHOULDER_CONE`] from the centre — whose room
/// `(1 − r) − |1.4/√2 − r|` is about `0.01` while `r` is short of that
/// distance and `1.98995 − 2r` past it, gone at `r ≈ 0.994975`.
#[test]
fn a_lanterns_rims_refuse_their_balls_where_the_offset_traces_part() {
    let lan = finished("lantern", lantern(tol()), tol());
    let neck = rim_arcs_at(&lan, 1.0, 0.0);
    let n = ladder("lantern neck", &lan, &neck, Some(0.5), |r| 1.0 - 2.0 * r);
    assert!(n >= 10, "lantern neck: {n} rungs past the boundary");
    let shoulder = rim_arcs_at(&lan, 0.8, 0.6);
    let n = ladder(
        "lantern shoulder",
        &lan,
        &shoulder,
        Some((1.0 + SHOULDER_CONE) / 2.0),
        |r| (1.0 - r) - (SHOULDER_CONE - r).abs(),
    );
    assert!(n >= 4, "lantern shoulder: {n} rungs past the boundary");
}
