//! **`fillet3_radius_headroom` is read on the ball's side of each
//! support**: a bend limits the radius only where it turns toward the
//! ball. Each curved kind is pinned on both sides, at a radius past the
//! bend's own, through `battery::radius_headroom` with the side chosen by
//! the convexity verdict against the face's stored sense — the one place
//! the battery reads it. The keyhole witness (a ball OUTSIDE a hole's
//! wall) runs end to end in `review_band_ruled_ring_probes`.
//!
//! The table replays at `Interval`, and one row certifies a radius BOX:
//! across `r ∈ [1.2, 1.25]` on a rod the ball inside it refuses on every
//! point of the box and the ball outside it passes on every point.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom::Surface;
use geom_core::{Band, Bounds, Decide, Interval, Point2, Point3, Real, Sign};
use sweep::blend::BlendError;
use sweep::blend::battery::{Convexity, radius_headroom};
use sweep::test_support::{corners, revolved_about_y_at};
use topo::{AtRestPolicy, Body, FaceKey};

/// The convexity verdict that puts the ball on the `inner` side of a
/// face of stored sense `sense` (`inner`: the side the chart normal
/// points away from) — the inverse of `Convexity::ball_side`.
fn putting_ball(sense: bool, inner: bool) -> Convexity {
    let c = if sense == inner {
        Convexity::Convex
    } else {
        Convexity::Concave
    };
    assert_eq!(
        c.ball_side(sense),
        inner,
        "the verdict puts the ball where asked"
    );
    c
}

/// The faces of `body` whose surface `pick` accepts.
fn faces_where<T: Real>(body: &Body<T>, pick: impl Fn(&Surface<T>) -> bool) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| body.get_surface(f.surface).is_some_and(&pick))
        .map(|(k, _)| k)
        .collect()
}

/// A face of `body` whose surface `pick` accepts. A revolve that
/// touches the axis mints a kind's wall in more than one face, all on
/// the one surface with the one sense, which is all the predicate reads
/// off a face.
fn face_where<T: Real>(body: &Body<T>, what: &str, pick: impl Fn(&Surface<T>) -> bool) -> FaceKey {
    let hits = faces_where(body, pick);
    let first = hits.first().copied();
    first.unwrap_or_else(|| panic!("{what}: no such face"))
}

/// A rectangle `[0.5, 1] × [0, 1]` turned about the y axis: an outer
/// wall of radius 1 (material inside it) and a bore of radius 0.5
/// (material outside it).
fn sleeve<T: Decide + AtRestPolicy>() -> Body<T> {
    revolved_about_y_at(
        corners(&[(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)]),
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
}

/// A disc of radius `t` about `(big, 0)` turned about the y axis: a
/// ring torus of major radius `big` and tube radius `t`.
fn ring<T: Decide + AtRestPolicy>(big: f64, t: f64) -> Body<T> {
    let v = |x: f64, b: f64| (Point2::new(T::from_f64(x), T::zero()), T::from_f64(b));
    revolved_about_y_at(
        vec![v(big + t, 1.0), v(big - t, 1.0)],
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
}

/// A ball of radius 1 cut at its equator and again at 45° latitude —
/// one sphere zone wall, material inside it.
fn zone<T: Decide + AtRestPolicy>() -> Body<T> {
    let a45 = core::f64::consts::FRAC_1_SQRT_2;
    let bulge = (core::f64::consts::FRAC_PI_4 / 4.0).tan();
    let v = |x: f64, y: f64, b: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b));
    revolved_about_y_at(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, bulge),
            v(a45, a45, 0.0),
            v(0.0, a45, 0.0),
        ],
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
}

/// A frustum `(0,0)→(1,0)→(0.5,1)→(0,1)`: one cone wall, material
/// inside it.
fn frustum<T: Decide + AtRestPolicy>() -> Body<T> {
    revolved_about_y_at(
        corners(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
        sweep::Revolution::Full,
        geom_core::Tol::witness(),
    )
}

/// One row: on `face`, at `p`, with the ball on the `inner` side, at
/// radius `r`, the predicate passes (`want: None`) or refuses with the
/// closed-form margin `want`.
struct Row<'a> {
    what: &'a str,
    inner: bool,
    r: f64,
    want: Option<f64>,
}

fn judge<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    p: [f64; 3],
    rows: &[Row<'_>],
    band: Band,
) {
    let sense = body.get_face(face).unwrap().sense;
    let p = Point3::new(T::from_f64(p[0]), T::from_f64(p[1]), T::from_f64(p[2]));
    for row in rows {
        let conv = putting_ball(sense, row.inner);
        let got = radius_headroom(body, face, conv, p, T::from_f64(row.r), band);
        let side = if row.inner { "inner" } else { "outer" };
        match (row.want, got) {
            (None, Ok(())) => {}
            (Some(want), Err(BlendError::RadiusHeadroom { margin, .. })) => {
                assert_eq!(
                    margin.sign,
                    Sign::Negative,
                    "{}, {side}: definite",
                    row.what
                );
                // An f64 run reads the point margin back; an interval
                // run's reading is the enclosure, whose sign is above.
                if let Some(m) = margin.reading.diagnostic_f64_for_error_text().value() {
                    assert!(
                        (m - want).abs() < 1e-12,
                        "{}, {side}, r = {}: margin {m} vs (1 − r/arm)·r = {want}",
                        row.what,
                        row.r
                    );
                }
            }
            (want, got) => panic!(
                "{}, {side}, r = {}: wanted {}, got {got:?}",
                row.what,
                row.r,
                want.map_or("a pass".to_string(), |w| format!("a refusal at {w}"))
            ),
        }
    }
}

/// `(1 − r/arm)·r`.
fn headroom(r: f64, arm: f64) -> f64 {
    r - r * r / arm
}

fn the_table<T: Decide + Bounds + AtRestPolicy>(band: Band) {
    let is_cyl = |want: f64| move |s: &Surface<T>| matches!(s, Surface::Cylinder { radius, .. } if (radius.lo() - want).abs() < 1e-12);
    let s = sleeve::<T>();
    // A rod's wall: a ball inside it (a convex crease on the rod) is
    // limited at R; a ball outside it (a concave crease at its foot) is
    // not, at any radius.
    let wall = face_where(&s, "the sleeve's outer wall", is_cyl(1.0));
    judge(
        &s,
        wall,
        [1.0, 0.5, 0.0],
        &[
            Row {
                what: "a rod's wall",
                inner: true,
                r: 1.2,
                want: Some(headroom(1.2, 1.0)),
            },
            Row {
                what: "a rod's wall",
                inner: true,
                r: 0.9,
                want: None,
            },
            Row {
                what: "a rod's wall",
                inner: false,
                r: 1.2,
                want: None,
            },
        ],
        band,
    );
    // A bore: the keyhole's case. A ball outside it (in the material,
    // a convex crease) is not limited; a ball inside it is, at R.
    let bore = face_where(&s, "the sleeve's bore", is_cyl(0.5));
    judge(
        &s,
        bore,
        [0.5, 0.5, 0.0],
        &[
            Row {
                what: "a bore",
                inner: false,
                r: 0.6,
                want: None,
            },
            Row {
                what: "a bore",
                inner: true,
                r: 0.6,
                want: Some(headroom(0.6, 0.5)),
            },
        ],
        band,
    );

    let z = zone::<T>();
    let sphere = face_where(&z, "the zone's sphere", |s| {
        matches!(s, Surface::Sphere { .. })
    });
    let (c30, s30) = (0.75f64.sqrt(), 0.5);
    judge(
        &z,
        sphere,
        [c30, s30, 0.0],
        &[
            Row {
                what: "a sphere",
                inner: true,
                r: 1.2,
                want: Some(headroom(1.2, 1.0)),
            },
            Row {
                what: "a sphere",
                inner: false,
                r: 1.2,
                want: None,
            },
        ],
        band,
    );

    // A cone at ρ = 0.75: the circumferential bend turns toward the
    // axis (the ρ bound), the generator is straight.
    let f = frustum::<T>();
    let cone = face_where(&f, "the frustum's wall", |s| {
        matches!(s, Surface::Cone { .. })
    });
    judge(
        &f,
        cone,
        [0.75, 0.5, 0.0],
        &[
            Row {
                what: "a cone",
                inner: true,
                r: 0.9,
                want: Some(headroom(0.9, 0.75)),
            },
            Row {
                what: "a cone",
                inner: false,
                r: 0.9,
                want: None,
            },
        ],
        band,
    );

    // A ring torus (R = 1, tube 0.25): inside the tube the tube bend
    // limits at 0.25; outside it the circumferential bend of the inner
    // equator does, at R − r = 0.75 — so 0.3 passes outside, where an
    // unsided read (min(0.25, 0.75)) refused it.
    let t = ring::<T>(1.0, 0.25);
    let torus = face_where(&t, "the ring", |s| matches!(s, Surface::Torus { .. }));
    judge(
        &t,
        torus,
        [1.25, 0.0, 0.0],
        &[
            Row {
                what: "a ring torus",
                inner: true,
                r: 0.3,
                want: Some(headroom(0.3, 0.25)),
            },
            Row {
                what: "a ring torus",
                inner: true,
                r: 0.2,
                want: None,
            },
            Row {
                what: "a ring torus",
                inner: false,
                r: 0.3,
                want: None,
            },
            Row {
                what: "a ring torus",
                inner: false,
                r: 0.8,
                want: Some(headroom(0.8, 0.75)),
            },
        ],
        band,
    );
    // A fat ring (R = 0.4, tube 0.25): the inner equator bends harder
    // than the tube, but away from a ball inside the tube — so 0.2
    // passes inside and refuses outside, at R − r = 0.15.
    let fat = ring::<T>(0.4, 0.25);
    let torus = face_where(&fat, "the fat ring", |s| matches!(s, Surface::Torus { .. }));
    judge(
        &fat,
        torus,
        [0.65, 0.0, 0.0],
        &[
            Row {
                what: "a fat torus",
                inner: true,
                r: 0.2,
                want: None,
            },
            Row {
                what: "a fat torus",
                inner: false,
                r: 0.2,
                want: Some(headroom(0.2, 0.15)),
            },
        ],
        band,
    );

    // A plane bends toward neither side.
    let base = face_where(
        &f,
        "the frustum's base",
        |s| matches!(s, Surface::Plane { origin, .. } if origin.y.lo().abs() < 1e-12),
    );
    judge(
        &f,
        base,
        [0.5, 0.0, 0.0],
        &[
            Row {
                what: "a plane",
                inner: true,
                r: 10.0,
                want: None,
            },
            Row {
                what: "a plane",
                inner: false,
                r: 10.0,
                want: None,
            },
        ],
        band,
    );
}

#[test]
fn each_curved_support_limits_the_radius_only_on_the_side_it_bends_toward() {
    the_table::<f64>(band());
}

#[test]
fn the_sided_table_replays_at_interval() {
    the_table::<Interval>(band());
}

/// The read certifies over a radius BOX, and its side is what decides
/// the box: on a rod's wall the whole of `[1.2, 1.25]` refuses with the
/// ball inside and passes with it outside, and a box straddling the
/// rod's radius escalates rather than picking an answer.
#[test]
fn a_radius_box_certifies_on_each_side_of_a_rod_wall() {
    let s = sleeve::<Interval>();
    let wall = face_where(
        &s,
        "the sleeve's outer wall",
        |s| matches!(s, Surface::Cylinder { radius, .. } if (radius.lo() - 1.0).abs() < 1e-12),
    );
    let sense = s.get_face(wall).unwrap().sense;
    let p = Point3::new(1.0, 0.5, 0.0).map(Interval::from_f64);
    let read = |inner: bool, lo: f64, hi: f64| {
        radius_headroom(
            &s,
            wall,
            putting_ball(sense, inner),
            p,
            Interval::from_bounds(lo, hi),
            band(),
        )
    };
    assert!(
        matches!(
            read(true, 1.2, 1.25),
            Err(BlendError::RadiusHeadroom { .. })
        ),
        "inside the rod, every radius of the box is past R: {:?}",
        read(true, 1.2, 1.25)
    );
    read(false, 1.2, 1.25).expect("outside the rod, every radius of the box fits");
    assert!(
        matches!(read(true, 0.99, 1.01), Err(BlendError::Escalated { .. })),
        "a box straddling R inside the rod escalates: {:?}",
        read(true, 0.99, 1.01)
    );
    read(false, 0.99, 1.01).expect("outside the rod the straddling box still fits");
}
