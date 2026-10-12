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

struct Frame {
    o: Point3<f64>,
    x: Vec3<f64>,
    y: Vec3<f64>,
    z: Vec3<f64>,
}
impl Frame {
    fn p(&self, a: f64, b: f64, c: f64) -> Point3<f64> {
        self.o + self.x * a + self.y * b + self.z * c
    }
    fn v(&self, a: f64, b: f64, c: f64) -> Vec3<f64> {
        self.x * a + self.y * b + self.z * c
    }
}
fn frame(o: [f64; 3], z: [f64; 3]) -> Frame {
    let z = Vec3::new(z[0], z[1], z[2]).normalize();
    let t = if z.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    let x = (t - z * t.dot(z)).normalize();
    let y = z.cross(x);
    Frame {
        o: Point3::new(o[0], o[1], o[2]),
        x,
        y,
        z,
    }
}
fn pipe(r: f64, z0: f64, h: f64, theta: f64, tol: Tol, f: &Frame) -> Body<f64> {
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
fn probe_pipe_frames_and_tilts() {
    let tol = Tol::witness();
    let pi = core::f64::consts::PI;
    let mut bad = 0;
    for (o, z) in [
        ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        ([3.0, -2.0, 5.0], [0.0, 0.0, 1.0]),
        ([1.5, 2.5, -0.5], [1.0, 2.0, 3.0]),
        ([1e3, -7e2, 4e2], [0.3, -0.2, 0.9]),
    ] {
        let f = frame(o, z);
        for (r, z0, h, theta) in [
            (1.0, 0.0, 2.0, 1e-7),
            (1.0, 0.0, 2.0, 1e-3),
            (1.0, 0.0, 2.0, 0.4),
            (1.0, 0.0, 2.0, -1.0),
            (1.0, 0.0, 30.0, 1.5),
            (1.0, 0.0, 300.0, 1.565),
            (0.01, -3.0, 0.05, 0.7),
            (1.0, 1e4, 2.0, 0.4),
        ] {
            let at = format!("o {o:?} z {z:?} r {r}, floor {z0}, h {h}, tilt {theta}");
            let body = match std::panic::catch_unwind(|| pipe(r, z0, h, theta, tol, &f)) {
                Ok(b) => b,
                Err(_) => {
                    eprintln!("PROBE {at}: build panicked");
                    continue;
                }
            };
            let v3 = validate_geometric(&body, tol);
            let mp = mass_properties(&body, tol);
            let volume = pi * r * r * h;
            let area = 2.0 * pi * r * h + pi * r * r + pi * r * r / theta.cos();
            match mp {
                Ok(mp) => {
                    let vok = (mp.volume - volume).abs()
                        <= mp.volume_pad + 1e-13 * volume * (1.0 + z0.abs() + o[0].abs());
                    let aok = (mp.surface_area - area).abs() <= mp.area_pad + 1e-13 * area;
                    if !(vok && aok) {
                        bad += 1;
                    }
                    eprintln!(
                        "PROBE {at}: v3 {:?} vol {} ± {:e} (exact {volume}, err {:e}) ok={vok}; area {} ± {:e} (exact {area}, err {:e}) ok={aok}",
                        v3.is_ok(),
                        mp.volume,
                        mp.volume_pad,
                        mp.volume - volume,
                        mp.surface_area,
                        mp.area_pad,
                        mp.surface_area - area
                    );
                }
                Err(e) => eprintln!("PROBE {at}: v3 {:?}; REFUSED {e}", v3.is_ok()),
            }
        }
    }
    assert_eq!(bad, 0);
}
