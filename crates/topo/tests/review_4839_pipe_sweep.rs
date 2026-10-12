//! Reviewer probe (PR 4839). Derived from
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


struct Frame { o: [f64; 3], ex: [f64; 3], ey: [f64; 3], ez: [f64; 3] }
impl Frame {
    fn p(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
        let c = |i: usize| self.o[i] + x * self.ex[i] + y * self.ey[i] + z * self.ez[i];
        Point3::new(c(0), c(1), c(2))
    }
    fn v(&self, x: f64, y: f64, z: f64) -> Vec3<f64> {
        let c = |i: usize| x * self.ex[i] + y * self.ey[i] + z * self.ez[i];
        Vec3::new(c(0), c(1), c(2))
    }
    fn tilted(o: [f64; 3], a: f64, b: f64) -> Self {
        // rotate by a about x then b about z
        let (sa, ca) = a.sin_cos();
        let (sb, cb) = b.sin_cos();
        let rx = |v: [f64; 3]| [v[0], ca * v[1] - sa * v[2], sa * v[1] + ca * v[2]];
        let rz = |v: [f64; 3]| [cb * v[0] - sb * v[1], sb * v[0] + cb * v[1], v[2]];
        Frame { o, ex: rz(rx([1.0, 0.0, 0.0])), ey: rz(rx([0.0, 1.0, 0.0])), ez: rz(rx([0.0, 0.0, 1.0])) }
    }
}
/// The pipe of radius `r` on the `z` axis from its floor at `z0` up to a
/// roof tilted by `theta` about `y`, `h` above the floor on the axis.
fn pipe(r: f64, z0: f64, h: f64, theta: f64, f: &Frame, tol: Tol) -> Body<f64> {
    let (s, c) = theta.sin_cos();
    let n = f.v(s, 0.0, c);
    let major_dir = f.v(c, 0.0, -s);
    let circle = Curve3::Circle {
        center: f.p(0.0, 0.0, z0),
        axis: f.v(0.0, 0.0, 1.0),
        radius: r,
        u_ref: f.v(1.0, 0.0, 0.0),
    };
    let ellipse = Curve3::Ellipse {
        center: f.p(0.0, 0.0, z0 + h),
        axis: -n,
        major: r / c,
        minor: r,
        u_ref: major_dir,
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(f.p(r, 0.0, z0), true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: f.p(0.0, 0.0, 0.0),
                    axis: f.v(0.0, 0.0, 1.0),
                    radius: r,
                    u_ref: f.v(1.0, 0.0, 0.0),
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
            f.p(r, 0.0, z0 + h - r * s / c),
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
                    origin: f.p(0.0, 0.0, z0),
                    normal: -f.v(0.0, 0.0, 1.0),
                    u_ref: f.v(1.0, 0.0, 0.0),
                },
                cap_b.face,
                true,
            ),
            Rechart::new(
                Surface::Plane {
                    origin: f.p(0.0, 0.0, z0 + h),
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
        (&cap_b, circle, f.p(-r, 0.0, z0)),
        (&cap_t, ellipse, f.p(-r, 0.0, z0 + h + r * s / c)),
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
fn review_probe_pipe_sweep() {
    let tol = Tol::witness();
    let pi = core::f64::consts::PI;
    let mut fails = Vec::new();
    for theta in [1e-12f64, 1e-9, 1e-4, 0.4, -0.4, 1.2, 1.5, -1.5, 1.55, 1.565] {
        for (r, z0, h) in [(0.01f64, 1.0f64, 10.0f64), (1.0f64, 0.0f64, 2.0f64), (0.01, 3.0, 0.05), (40.0, -500.0, 120.0)] {
            for (fi, f) in [
                Frame::tilted([0.0; 3], 0.0, 0.0),
                Frame::tilted([12.0, -7.0, 3.0], 0.7, 1.9),
                Frame::tilted([1e4, 2e4, -3e3], 2.5, -0.3),
            ].iter().enumerate() {
                let at = format!("r {r}, floor {z0}, h {h}, tilt {theta}, frame {fi}");
                if (r * theta.tan()).abs() >= h { continue; } // roof must clear the floor
                let body = match std::panic::catch_unwind(|| pipe(r, z0, h, theta, f, tol)) {
                    Ok(b) => b,
                    Err(_) => { fails.push(format!("{at}: BUILD panicked")); continue; }
                };
                if let Err(e) = validate_geometric(&body, tol) { fails.push(format!("{at}: tier3 {e:?}")); continue; }
                let mp = match mass_properties(&body, tol) {
                    Ok(m) => m,
                    Err(e) => { fails.push(format!("{at}: REFUSED {e}")); continue; }
                };
                let volume = pi * r * r * h;
                let area = 2.0 * pi * r * h + pi * r * r + pi * r * r / theta.cos();
                let slack = 1e-12;
                let vok = (mp.volume - volume).abs() <= mp.volume_pad + slack * volume.abs().max(1e-30) * 1e3;
                let aok = (mp.surface_area - area).abs() <= mp.area_pad + slack * area * 1e3;
                println!("{at}: dv={:e} vpad={:e} da={:e} apad={:e} {}", mp.volume - volume, mp.volume_pad, mp.surface_area - area, mp.area_pad, if vok && aok {"ok"} else {"MISS"});
                if !(vok && aok) { fails.push(format!("{at}: MISS")); }
            }
        }
    }
    for f in &fails { println!("FAIL {f}"); }
    assert!(fails.is_empty(), "{} failures", fails.len());
}
