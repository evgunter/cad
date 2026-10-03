//! **The cylinder's chart Green form is conditioned like the closed
//! form it replaced.**
//!
//! A cylinder face's flux is `R²·A + origin·A⃗` with the chart area
//! `A = −Σ∮ v du` (`geom_brep::props::curved_face`'s cylinder arm). On an
//! iso-rectangle that sum is `−v₀·Δu + v₁·Δu`, which cancels
//! catastrophically when the face sits far along the axis from the
//! surface origin relative to its height: at `v₀ = 1e3`, `h = 0.0071`
//! the raw sum carried a relative error of 1.07e-11, where the product
//! `Δu·(v₁ − v₀)` it replaced carries one rounding. Reading `v` against
//! an anchor inside the face (anchor-free in exact arithmetic, because a
//! closed face's boundary has `Σ∮ du = 0`) restores the product's
//! conditioning.
//!
//! The rows hold the area and the flux of every rectangle in a grid of
//! heights, lifts, azimuth spans and radii to the product form at
//! 1e-14 relative.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face};

/// The iso-rectangle `[u0, u1] × [v0, v1]` on the radius-`r` cylinder
/// about +Z through the world origin, walked CCW about the outward
/// normal.
fn rectangle(
    r: f64,
    (u0, u1): (f64, f64),
    (v0, v1): (f64, f64),
) -> (Surface<f64>, Vec<LoopEdge<f64>>) {
    let surface = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = |v: f64, a: f64, b: f64, s: u32, e: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, v),
                axis: v3(0.0, 0.0, 1.0),
                radius: r,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            a,
            b,
            s,
            e,
        )
    };
    let ruling = |u: f64, a: f64, b: f64, s: u32, e: u32| {
        edge(
            Curve3::Line {
                origin: p3(r * u.cos(), r * u.sin(), 0.0),
                dir: v3(0.0, 0.0, 1.0),
            },
            a,
            b,
            s,
            e,
        )
    };
    let edges = vec![
        rim(v0, u0, u1, 0, 1),
        ruling(u1, v0, v1, 1, 2),
        rim(v1, u1, u0, 2, 3),
        ruling(u0, v1, v0, 3, 0),
    ];
    (surface, edges)
}

/// The face's area and flux against the product form, relative.
fn errors(r: f64, u: (f64, f64), v: (f64, f64)) -> (f64, f64) {
    let (surface, edges) = rectangle(r, u, v);
    let fc = curved_face(&surface, &edges, true, band())
        .unwrap_or_else(|e| panic!("r {r}, u {u:?}, v {v:?}: refused {e:?}"));
    let area = r * (u.1 - u.0) * (v.1 - v.0);
    let flux = r * area;
    (
        ((fc.area - area) / area).abs(),
        ((fc.flux - flux) / flux).abs(),
    )
}

/// **The witness the review measured**: a face 1000 m along the axis
/// and 7.1 mm tall.
#[test]
fn a_short_face_far_along_the_axis_keeps_the_product_conditioning() {
    let (area, flux) = errors(1.0, (0.0, 0.3), (1e3, 1e3 + 0.0071));
    assert!(
        area <= 1e-14 && flux <= 1e-14,
        "area {area:.3e}, flux {flux:.3e} relative to the product form"
    );
}

/// The grid: lifts from the origin to 1e5 m either way, heights from
/// 0.1 mm to 2 m, three azimuth spans, three radii.
#[test]
fn every_rectangle_in_the_grid_matches_the_product_form() {
    let mut worst: (f64, String) = (0.0, String::new());
    let mut rows = 0;
    for &lift in &[0.0, 1.0, -3.7, 1e3, -1e3, 1e5] {
        for &h in &[1e-4, 7.1e-3, 0.5, 2.0] {
            for &u in &[
                (0.0, 0.3),
                (-1.2, 2.9),
                (0.7, 0.7 + core::f64::consts::FRAC_PI_2),
            ] {
                for &r in &[0.01, 1.0, 40.0] {
                    let (area, flux) = errors(r, u, (lift, lift + h));
                    let e = area.max(flux);
                    if e > worst.0 {
                        worst = (e, format!("lift {lift}, h {h}, u {u:?}, r {r}"));
                    }
                    rows += 1;
                }
            }
        }
    }
    assert_eq!(rows, 216);
    assert!(
        worst.0 <= 1e-14,
        "worst relative error {:.3e} at {}",
        worst.0,
        worst.1
    );
}

/// A frame-general cylinder: radius `r` about the unit `axis` through
/// `origin`, chart zero along the unit `u_ref ⊥ axis`; returns the
/// surface and a loop builder for the iso-rectangle `[u0, u1] × [v0,
/// v1]`, walked CCW (`ccw`) or CW about the outward normal.
#[allow(clippy::type_complexity)]
fn framed(
    r: f64,
    origin: [f64; 3],
    axis: [f64; 3],
    u_ref: [f64; 3],
) -> (
    Surface<f64>,
    impl Fn((f64, f64), (f64, f64), bool, u32) -> Vec<LoopEdge<f64>>,
) {
    let o = p3(origin[0], origin[1], origin[2]);
    let a = v3(axis[0], axis[1], axis[2]);
    let x = v3(u_ref[0], u_ref[1], u_ref[2]);
    let y = a.cross(x);
    let surface = Surface::Cylinder {
        origin: o,
        axis: a,
        radius: r,
        u_ref: x,
    };
    let build = move |(u0, u1): (f64, f64), (v0, v1): (f64, f64), ccw: bool, tag: u32| {
        let rim = |v: f64, s: f64, e: f64, t0: u32, t1: u32| {
            edge(
                Curve3::Circle {
                    center: o + a * v,
                    axis: a,
                    radius: r,
                    u_ref: x,
                },
                s,
                e,
                t0,
                t1,
            )
        };
        let ruling = |u: f64, s: f64, e: f64, t0: u32, t1: u32| {
            edge(
                Curve3::Line {
                    origin: o + (x * u.cos() + y * u.sin()) * r,
                    dir: a,
                },
                s,
                e,
                t0,
                t1,
            )
        };
        let k = tag;
        let ccw_loop = vec![
            rim(v0, u0, u1, k, k + 1),
            ruling(u1, v0, v1, k + 1, k + 2),
            rim(v1, u1, u0, k + 2, k + 3),
            ruling(u0, v1, v0, k + 3, k),
        ];
        if ccw {
            ccw_loop
        } else {
            vec![
                ruling(u0, v0, v1, k, k + 3),
                rim(v1, u0, u1, k + 3, k + 2),
                ruling(u1, v1, v0, k + 2, k + 1),
                rim(v0, u1, u0, k + 1, k),
            ]
        }
    };
    (surface, build)
}

/// **A tilted axis off the world origin keeps the conditioning.** The
/// axis `(1, 2, 2)/3` through `(3, −2, 5)`; a 4 mm face 40 m along it.
/// The rim levels are recovered from the carriers' centres
/// (`(c − o)·â`), which costs the rounding of that dot product — the
/// row's 1e-12 holds it; the props form adds no cancellation of its own.
#[test]
fn a_tilted_axis_off_the_origin_keeps_the_conditioning() {
    let s = 1.0 / 3.0;
    let (surface, build) = framed(
        0.7,
        [3.0, -2.0, 5.0],
        [s, 2.0 * s, 2.0 * s],
        [2.0 * s, s, -2.0 * s],
    );
    let (u, v) = ((0.2, 1.1), (40.0, 40.004));
    let fc = curved_face(&surface, &build(u, v, true, 0), true, band())
        .unwrap_or_else(|e| panic!("the tilted rectangle was refused: {e:?}"));
    let area = 0.7 * (u.1 - u.0) * (v.1 - v.0);
    assert!(
        ((fc.area - area) / area).abs() <= 1e-12,
        "area {} against the product form {area}",
        fc.area
    );
}

/// **A hole is a ring wound against its face.** A rectangle with a
/// rectangular hole: wound CW about the outward normal the hole is
/// subtracted; handed CCW, the same way as its face, it would be ADDED
/// silently, so it refuses (`props_ring_winding`).
#[test]
fn a_ring_wound_against_its_face_is_a_hole_and_one_wound_with_it_refuses() {
    let (surface, build) = framed(1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    let outer = build((0.0, 1.0), (0.0, 1.0), true, 0);
    let hole = build((0.3, 0.6), (0.3, 0.6), false, 10);
    let fc = geom_brep::props::curved_face_loops(&surface, &[&outer, &hole], true, band())
        .unwrap_or_else(|e| panic!("the holed face was refused: {e:?}"));
    let area = 1.0 - 0.09;
    assert!(
        ((fc.area - area) / area).abs() <= 1e-12,
        "area {} against {area}",
        fc.area
    );
    let reversed = build((0.3, 0.6), (0.3, 0.6), true, 10);
    let got = geom_brep::props::curved_face_loops(&surface, &[&outer, &reversed], true, band());
    assert!(
        matches!(
            got,
            Err(geom_brep::props::PropsError::NotIsoRectangle {
                what: "props_ring_winding"
            })
        ),
        "a ring wound with its face must refuse: {got:?}"
    );
}
