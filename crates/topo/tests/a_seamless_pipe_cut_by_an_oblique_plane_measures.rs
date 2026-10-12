//! **A seamless pipe cut by an oblique plane measures.** The unit
//! cylinder's wall between a floor circle and an oblique roof ellipse,
//! with no seam: one face whose two loops each wind the cylinder once,
//! built through public Euler doors alone (no constructor states a
//! seamless band — the kernel's writers split one at a seam). The
//! wall carries an ellipse, so it takes the certified quadrature, and
//! the solid measures against its closed form: volume `π·r²·h` and
//! area `2π·r·h + π·r² + π·r²/cos θ`, `h` the roof's height on the
//! axis.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::{Point3, Tol, Vec3};
use topo::validate::{validate_closed, validate_geometric};
use topo::{Body, FaceSurface, MefSite, MevSite, Rechart, mass_properties};

/// A whole-turn rim on the wall, described by its chart until the cap
/// it bounds exists.
fn rim(cyl: topo::SurfaceKey, carrier: Curve3<f64>) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Chart {
            surface: cyl,
            image: None,
            wrap: false,
            declared: None,
        },
        carrier,
        param_start: 0.0,
        param_end: TAU,
    }
}

/// The pipe of radius `r` on the `z` axis from its floor at `z0` up to a
/// roof tilted by `theta` about `y`, `h` above the floor on the axis.
fn pipe(r: f64, z0: f64, h: f64, theta: f64, tol: Tol) -> Body<f64> {
    let (s, c) = theta.sin_cos();
    let n = Vec3::new(s, 0.0, c);
    let major_dir = Vec3::new(c, 0.0, -s);
    let circle = Curve3::Circle {
        center: Point3::new(0.0, 0.0, z0),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let ellipse = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, z0 + h),
        axis: -n,
        major: r / c,
        minor: r,
        u_ref: major_dir,
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(r, 0.0, z0), true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: Point3::origin(),
                    axis: Vec3::unit_z(),
                    radius: r,
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
    let cap_b = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            rim(cyl, circle.clone()),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: cap_b.he_plus,
                he2: cap_b.he_plus,
            },
            Point3::new(r, 0.0, z0 + h - r * s / c),
            tol,
        )
        .unwrap();
    let cap_t = body
        .mef(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_minus,
            },
            rim(cyl, ellipse.clone()),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    body.set_face_surfaces_describing(
        vec![
            Rechart::new(
                Surface::Plane {
                    origin: Point3::new(0.0, 0.0, z0),
                    normal: -Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                cap_b.face,
                true,
            ),
            Rechart::new(
                Surface::Plane {
                    origin: Point3::new(0.0, 0.0, z0 + h),
                    normal: n,
                    u_ref: major_dir,
                },
                cap_t.face,
                true,
            ),
        ],
        &[],
        tol,
    )
    .unwrap();
    for (cap, carrier, witness) in [
        (&cap_b, circle, Point3::new(-r, 0.0, z0)),
        (&cap_t, ellipse, Point3::new(-r, 0.0, z0 + h + r * s / c)),
    ] {
        let plane = body.get_face(cap.face).unwrap().surface;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(cyl, plane),
                witness,
            },
            carrier,
            param_start: 0.0,
            param_end: TAU,
        };
        body.set_edge_curve(cap.edge, spec, tol).unwrap();
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

#[test]
fn a_seamless_pipe_cut_by_an_oblique_plane_measures_in_its_closed_form() {
    let tol = Tol::witness();
    let pi = core::f64::consts::PI;
    for (r, z0, h, theta) in [
        (1.0, 0.0, 2.0, 0.4),
        (1.0, 0.0, 2.0, -1.0),
        (0.3, 0.0, 0.5, 0.7),
        (1.0, 100.0, 2.0, 0.4),
    ] {
        let at = format!("r {r}, floor {z0}, h {h}, tilt {theta}");
        let body = pipe(r, z0, h, theta, tol);
        assert_eq!(validate_closed(&body), Ok(()), "{at}: tier 2");
        assert_eq!(validate_geometric(&body, tol), Ok(()), "{at}: tier 3");
        let mp = mass_properties(&body, tol)
            .unwrap_or_else(|e| panic!("{at}: the pipe does not measure: {e}"));
        let volume = pi * r * r * h;
        let area = 2.0 * pi * r * h + pi * r * r + pi * r * r / theta.cos();
        assert!(
            mp.volume_pad > 0.0 && mp.area_pad > 0.0,
            "{at}: the wall is a quadrature face, so its enclosure has width: {mp:?}"
        );
        assert!(
            mp.volume_pad < 1e-3 * volume && mp.area_pad < 1e-3 * area,
            "{at}: the enclosure is tight enough to say something: {mp:?}"
        );
        // The closed-form caps add their own f64 rounding.
        let slack = 64.0 * f64::EPSILON;
        assert!(
            (mp.volume - volume).abs() <= mp.volume_pad + slack * volume * (1.0 + z0.abs()),
            "{at}: volume {} ± {} against π·r²·h = {volume}",
            mp.volume,
            mp.volume_pad
        );
        assert!(
            (mp.surface_area - area).abs() <= mp.area_pad + slack * area,
            "{at}: area {} ± {} against {area}",
            mp.surface_area,
            mp.area_pad
        );
    }
}
