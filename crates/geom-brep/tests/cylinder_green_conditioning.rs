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
