//! Review probes for PR 5817 (not for merge): adversarial pairs for
//! arm 1's offset directions and held sphere reach. Every pair that
//! truly touches or overlaps must not read `Ok(())`; apart pairs print.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body, ContactRecords, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

fn census(body: &AtRestBody<f64>) -> Result<(), Vec<ValidationError>> {
    topo::validate_pseudomanifold(body, &ContactRecords::default(), tol())
}

fn short(r: &Result<(), Vec<ValidationError>>) -> String {
    match r {
        Ok(()) => "OK".into(),
        Err(e) => format!("ERR x{} first={:?}", e.len(), e.first().map(|e| format!("{e:?}").chars().take(160).collect::<String>())),
    }
}

fn rev(verts: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    sweep::test_support::revolved_about_y(verts, sweep::Revolution::Full, tol())
}

fn graft(mut a: Body<f64>, b: &Body<f64>, what: &str) -> AtRestBody<f64> {
    topo::graft_disjoint(&mut a, b).expect("grafts");
    finished(what, a, tol())
}

/// Report a row; `meets` pairs must refuse.
fn row(failures: &mut Vec<String>, label: &str, meets: bool, body: &AtRestBody<f64>) {
    let r = census(body);
    println!("PROBE {label} meets={meets}: {}", short(&r));
    if meets && r.is_ok() {
        failures.push(label.to_string());
    }
}

fn rod() -> Body<f64> {
    rev(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.5, 0.0), 0.0),
        (Point2::new(0.5, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ])
}

/// A box whose edge nearest the rod's axis runs along `x = z = s`
/// over `y ∈ [-0.5, 1.5]`, past both caps: no vertex of either is
/// inside the other.
#[test]
fn probe_box_through_rod_wall() {
    let mut fails = Vec::new();
    for gap in [-1e-2, -1e-3, -1e-6, -1e-9, 0.0, 1e-12, 1e-9, 1e-6, 1e-4, 1e-3] {
        let s = (0.5 + gap) * core::f64::consts::FRAC_1_SQRT_2;
        let b = brick((s, s + 1.0), (-0.5, 1.5), (s, s + 1.0), tol());
        let body = graft(rod(), &b, "box through rod");
        row(&mut fails, &format!("rod-wall gap {gap:e}"), gap <= 0.0, &body);
    }
    // The box's edge off the diagonal: at angle 30° about the axis.
    for gap in [-1e-3, 0.0, 1e-3] {
        let (sn, cs) = 30f64.to_radians().sin_cos();
        let (x, z) = ((0.5 + gap) * cs, (0.5 + gap) * sn);
        let b = brick((x, x + 1.0), (-0.5, 1.5), (z, z + 1.0), tol());
        let body = graft(rod(), &b, "box through rod 30");
        // Nearest point of the box's corner edge to the axis is the edge.
        row(&mut fails, &format!("rod-wall 30deg gap {gap:e}"), gap <= 0.0, &body);
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}

/// A pointed cone (apex `(0, 1, 0)`, base radius 0.5 at `y = 0`) and a
/// box whose edge runs along `x = z = s` past both ends.
#[test]
fn probe_box_through_cone() {
    let mut fails = Vec::new();
    let cone = || {
        rev(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ])
    };
    for (rho, meets) in [(0.3, true), (0.05, true), (0.499, true), (0.5, true), (0.501, false), (0.52, false)] {
        let s = rho * core::f64::consts::FRAC_1_SQRT_2;
        let b = brick((s, s + 1.0), (-0.5, 1.5), (s, s + 1.0), tol());
        let body = graft(cone(), &b, "box through cone");
        row(&mut fails, &format!("cone rho {rho}"), meets, &body);
    }
    // Near the apex, a box above the base plane only: y ∈ [0.8, 1.5].
    for (rho, meets) in [(0.05, true), (0.1, true), (0.11, false)] {
        let s = rho * core::f64::consts::FRAC_1_SQRT_2;
        let b = brick((s, s + 1.0), (0.8, 1.5), (s, s + 1.0), tol());
        let body = graft(cone(), &b, "box near apex");
        row(&mut fails, &format!("cone apex rho {rho}"), meets, &body);
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}

/// A torus `R = 1, r = 0.25` about `y`, and a box in its hole whose
/// corner edges along `y` reach radius `a·√2`.
#[test]
fn probe_box_in_torus_hole() {
    let mut fails = Vec::new();
    let torus = || {
        rev(vec![(Point2::new(0.75, 0.0), 1.0), (Point2::new(1.25, 0.0), 1.0)])
    };
    for (rho, meets) in [(0.76, true), (0.751, true), (0.75, true), (0.749, false), (0.74, false), (0.7, false)] {
        let a = rho * core::f64::consts::FRAC_1_SQRT_2;
        let b = brick((-a, a), (-0.5, 0.5), (-a, a), tol());
        let body = graft(torus(), &b, "box in torus hole");
        row(&mut fails, &format!("torus hole rho {rho}"), meets, &body);
    }
    // A box beside the torus's inner wall on the diagonal, from inside
    // the hole outward: its far corner edge at radius rho, past y.
    for (rho, meets) in [(0.76, true), (0.75, true), (0.74, false)] {
        let s = rho * core::f64::consts::FRAC_1_SQRT_2;
        let b = brick((s - 0.3, s), (-0.5, 0.5), (s - 0.3, s), tol());
        let body = graft(torus(), &b, "box at torus inner wall");
        row(&mut fails, &format!("torus inner wall rho {rho}"), meets, &body);
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}

/// The unit ball and a box whose edge along `z` runs at `x = y = s`,
/// past the ball both ways, across the ball's bulge on the diagonal.
#[test]
fn probe_box_edge_across_a_ball() {
    let mut fails = Vec::new();
    for pole in ["y", "z"] {
        for (rho, meets) in [(0.9, true), (0.999, true), (1.0, true), (1.001, false), (1.05, false)] {
            let s = rho * core::f64::consts::FRAC_1_SQRT_2;
            let ball = match pole {
                "y" => sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
                _ => sweep::test_support::ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol()),
            };
            let b = brick((s, s + 1.0), (s, s + 1.0), (-2.0, 2.0), tol());
            let body = graft(ball, &b, "box edge across ball");
            row(&mut fails, &format!("ball {pole} diag rho {rho}"), meets, &body);
        }
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}

/// Sphere faces with holes, crossed by a slab near a pole the face
/// holds (or does not): the boolean cuts the holes, then a slab is
/// grafted across.
#[test]
fn probe_holed_ball_crossed_by_a_slab() {
    let mut fails = Vec::new();
    let o = Vec3::new(0.0, 0.0, 0.0);
    let balls: Vec<(&str, Body<f64>)> = vec![
        ("y", sweep::test_support::ball_poled_y(1.0, o, tol())),
        ("z", sweep::test_support::ball_poled_z(1.0, o, tol())),
    ];
    let cutters = [
        ("bar x off pole", [(-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4)]),
        ("bar x through pole", [(-2.0, 2.0), (-0.2, 0.25), (-0.1, 0.4)]),
        ("cube far side", [(-0.2, 0.2), (-2.0, -0.9), (-0.2, 0.2)]),
        ("cube at +y pole", [(-0.1, 0.1), (0.98, 2.0), (-0.1, 0.1)]),
        ("cube at +x pole", [(0.98, 2.0), (-0.1, 0.1), (-0.1, 0.1)]),
    ];
    let slabs = [
        ("slab y", [(-2.0, 2.0), (0.9, 0.95), (-2.0, 2.0)]),
        ("slab x", [(0.9, 0.95), (-2.0, 2.0), (-2.0, 2.0)]),
        ("slab y thin-cap", [(-2.0, 2.0), (0.995, 0.999), (-2.0, 2.0)]),
        ("slab x thin-cap", [(0.995, 0.999), (-2.0, 2.0), (-2.0, 2.0)]),
    ];
    for (pole, ball) in &balls {
        let ball = finished("ball", ball.clone(), tol());
        for (cname, c) in &cutters {
            let cutter = finished("cutter", brick(c[0], c[1], c[2], tol()), tol());
            let holed = match topo::subtract(&ball, &cutter, tol()) {
                Ok(topo::BooleanResult::Body(b)) => b,
                other => {
                    println!("PROBE ball {pole} ∖ {cname}: no body {:?}", other.map(|_| ()));
                    continue;
                }
            };
            for (sname, s) in &slabs {
                let slab = brick(s[0], s[1], s[2], tol());
                let body = graft(holed.body.clone().into_body(), &slab, "holed ball and slab");
                // Every slab here crosses the sphere where the holed face
                // still has material (the cutters miss the slabs' bands
                // except where noted, and the slab spans the whole ring).
                // A thin cap slab over a cutter that removes the whole
                // cap under it is ~1.3e-5 apart; every other combination
                // crosses material.
                let apart = matches!(
                    (*cname, *sname),
                    ("cube at +y pole", "slab y thin-cap")
                        | ("cube at +x pole", "slab x thin-cap")
                        | ("bar x through pole", "slab x thin-cap")
                );
                row(&mut fails, &format!("ball {pole} ∖ {cname} + {sname}"), !apart, &body);
            }
        }
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}

/// The ball poled `y` (and `z`) with the cap over `|x|, |z| ≤ 0.1`
/// removed: its rim's highest points are `(±0.1, √0.99, 0)` and
/// `(0, √0.99, ±0.1)`. A slab whose bottom sits `delta` above `√0.99`:
/// apart for `delta > 0`, touching at 0, overlapping below.
#[test]
fn probe_holed_cap_slab_at_the_rim() {
    let mut fails = Vec::new();
    let o = Vec3::new(0.0, 0.0, 0.0);
    let top = 0.99f64.sqrt();
    for pole in ["y", "z"] {
        let ball = match pole {
            "y" => sweep::test_support::ball_poled_y(1.0, o, tol()),
            _ => sweep::test_support::ball_poled_z(1.0, o, tol()),
        };
        let ball = finished("ball", ball, tol());
        let cutter = finished("cutter", brick((-0.1, 0.1), (0.98, 2.0), (-0.1, 0.1), tol()), tol());
        let Ok(topo::BooleanResult::Body(holed)) = topo::subtract(&ball, &cutter, tol()) else {
            panic!("the cap cut builds");
        };
        for delta in [1e-3, 1e-5, 1e-6, 1e-7, 1e-9, 1e-12, 0.0, -1e-9, -1e-6, -1e-4] {
            let slab = brick((-2.0, 2.0), (top + delta, top + 0.002), (-2.0, 2.0), tol());
            let body = graft(holed.body.clone().into_body(), &slab, "holed cap and slab");
            row(&mut fails, &format!("holed {pole} cap slab delta {delta:e}"), delta <= 0.0, &body);
        }
    }
    assert!(fails.is_empty(), "false apart: {fails:?}");
}
