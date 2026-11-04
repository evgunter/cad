//! **An edge's iso family is read over its own span.** The joinable
//! vertex check (`topo::boolean::edge_join`) asks for the family of each
//! edge's carrier over the edge's interval, which a spindle sheet needs
//! (its meridian circle leaves the sheet past its apexes). Every other
//! kind's family is the carrier's whatever part of it an edge holds, so
//! over a partial span it reads as over the carrier's own interval.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{IsoFamily, chart_iso_family};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

#[test]
fn a_partial_edge_reads_the_family_its_whole_carrier_does() {
    let (o, z, x) = (
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let cylinder = Surface::Cylinder {
        origin: o,
        axis: z,
        radius: 1.0,
        u_ref: x,
    };
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: PI / 6.0,
        u_ref: x,
    };
    let sphere = Surface::Sphere {
        center: o,
        radius: 1.0,
        axis: z,
        u_ref: x,
    };
    let torus = Surface::Torus {
        center: o,
        axis: z,
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x,
    };
    let rim = |h: f64, rho: f64| Curve3::circle(Point3::new(0.0, 0.0, h), z, rho, x);
    let (s30, c30) = (PI / 6.0).sin_cos();
    let rows: [(
        &str,
        Curve3<f64>,
        &Surface<f64>,
        (f64, f64),
        (f64, f64),
        IsoFamily,
    ); 7] = [
        (
            "a cylinder ruling",
            Curve3::line(Point3::new(1.0, 0.0, 0.0), z),
            &cylinder,
            (0.0, 1.0),
            (0.3, 2.5),
            IsoFamily::U,
        ),
        (
            "a cylinder rim",
            rim(0.4, 1.0),
            &cylinder,
            (0.0, TAU),
            (0.5, 2.0),
            IsoFamily::V,
        ),
        (
            "a cone generator",
            geom::pole_form::cone_generator(o, z, PI / 6.0, x),
            &cone,
            (0.0, 1.0),
            (0.4, 1.7),
            IsoFamily::U,
        ),
        (
            "a cone rim",
            rim(c30, s30),
            &cone,
            (0.0, TAU),
            (1.0, 3.0),
            IsoFamily::V,
        ),
        (
            "a sphere meridian",
            geom::pole_form::sphere_meridian(o, 1.0, z, x),
            &sphere,
            (0.0, TAU),
            (0.2, 1.3),
            IsoFamily::U,
        ),
        (
            "a sphere parallel",
            rim(0.6, 0.8),
            &sphere,
            (0.0, TAU),
            (0.5, 2.2),
            IsoFamily::V,
        ),
        (
            "a torus meridian",
            Curve3::circle(
                Point3::new(2.0, 0.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                0.5,
                x,
            ),
            &torus,
            (0.0, TAU),
            (0.3, 2.9),
            IsoFamily::U,
        ),
    ];
    for (what, carrier, surface, own, part, family) in rows {
        let whole = chart_iso_family(&carrier, own, surface, band());
        let edge = chart_iso_family(&carrier, part, surface, band());
        assert_eq!(whole, Ok(Some(family)), "{what}: its whole carrier");
        assert_eq!(edge, whole, "{what}: over {part:?}");
    }
}
