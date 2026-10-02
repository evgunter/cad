//! **The sphere flux arm's Gauss–Bonnet form** (`curved::sphere_circle_loop`),
//! held to Girard's theorem: a spherical triangle with three right
//! angles — the octant `x, y, z > 0` — has area `R²·π/2`, on a sphere
//! whose chart axis is `(1, 1, 1)/√3`, so each of its three great-circle
//! arcs is tilted against the chart (neither a rim nor a meridian) and
//! the face takes the loop form, not the iso-rectangle one.
//!
//! The flux is `σ·R·Area + c·A⃗` with `A⃗` the octant's vector area,
//! `(R²π/4)(1, 1, 1)` — a quarter disc projected on each axis plane — so
//! a centre off the origin checks the anchor term against an
//! independent number too.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::Vec3;

const R: f64 = 2.0;

fn sphere(c: Vec3<f64>) -> Surface<f64> {
    let k = 1.0 / 3.0_f64.sqrt();
    Surface::Sphere {
        center: p3(c.x, c.y, c.z),
        radius: R,
        axis: v3(k, k, k),
        u_ref: v3(1.0, -1.0, 0.0) * (1.0 / 2.0_f64.sqrt()),
    }
}

/// The octant's loop, interior on the left under the outward normal:
/// `+x → +y` in `z = 0`, `+y → +z` in `x = 0`, `+z → +x` in `y = 0`.
fn octant(c: Vec3<f64>) -> Vec<LoopEdge<f64>> {
    let arc = |axis: Vec3<f64>, u_ref: Vec3<f64>, s: u32, e: u32| {
        edge(
            Curve3::Circle {
                center: p3(c.x, c.y, c.z),
                axis,
                radius: R,
                u_ref,
            },
            0.0,
            FRAC_PI_2,
            s,
            e,
        )
    };
    let (x, y, z) = (v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0));
    vec![arc(z, x, 0, 1), arc(x, y, 1, 2), arc(y, z, 2, 0)]
}

#[test]
fn the_octant_meets_girard_and_its_vector_area() {
    let area = R * R * FRAC_PI_2;
    for (label, c) in [
        ("centred", v3(0.0, 0.0, 0.0)),
        ("off-centre", v3(1.0, 2.0, 3.0)),
    ] {
        let fc = curved_face(&sphere(c), &octant(c), true, band()).expect("the octant measures");
        let flux = R * area + (c.x + c.y + c.z) * R * R * PI / 4.0;
        assert!(
            (fc.area - area).abs() < 1e-12,
            "{label}: area {} != {area}",
            fc.area
        );
        assert!(
            (fc.flux - flux).abs() < 1e-12,
            "{label}: flux {} != {flux}",
            fc.flux
        );
        // The same loop under the reversed bit bounds the rest of the
        // sphere, facing in: its area is the complement and its radial
        // term turns over, while the traversal-read A⃗ does not.
        let fc = curved_face(&sphere(c), &octant(c), false, band()).expect("the complement");
        let rest = 4.0 * PI * R * R - area;
        let flux = -R * rest + (c.x + c.y + c.z) * R * R * PI / 4.0;
        assert!(
            (fc.area - rest).abs() < 1e-12,
            "{label}: area {} != {rest}",
            fc.area
        );
        assert!(
            (fc.flux - flux).abs() < 1e-12,
            "{label}: flux {} != {flux}",
            fc.flux
        );
    }
}

/// A loop that turns back on itself at a vertex has no turning angle
/// there, and is refused rather than integrated.
#[test]
fn a_cusp_is_refused() {
    let c = v3(0.0, 0.0, 0.0);
    // The octant's first arc, there and back: the loop closes, and at
    // both ends the departing tangent is the arriving one reversed.
    let mut cusp = octant(c);
    cusp.truncate(1);
    let back = LoopEdge {
        forward: false,
        start: 1,
        end: 0,
        ..cusp[0].clone()
    };
    cusp.push(back);
    assert_eq!(
        curved_face(&sphere(c), &cusp, true, band()).map(|_| ()),
        Err(geom_brep::props::PropsError::NotIsoRectangle {
            what: "props_sphere_loop_cusp",
        })
    );
}
