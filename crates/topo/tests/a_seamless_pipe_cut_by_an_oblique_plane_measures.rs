//! **A seamless pipe cut by an oblique plane measures.** A cylinder's
//! wall between a floor circle and an oblique roof ellipse, with no
//! seam: one face whose two loops each wind the cylinder once, built
//! through public Euler doors alone (no constructor states a seamless
//! band — the kernel's writers split one at a seam), at `f64` and at
//! `Interval` by the same operator sequence. The wall carries an
//! ellipse, so it takes the certified quadrature, and the solid
//! measures against its closed form: volume `π·r²·h` and area
//! `2π·r·h + π·r² + π·r²/cos θ`, `h` the roof's height on the axis.
//!
//! The poses run in four frames — the world's, one off its origin, and
//! two with tilted axes, one a kilometre out (review R1 of PR 4839) —
//! and every tolerance is derived, not chosen:
//!
//! - **The pads** are bounded by the reporting target the wall
//!   converged to: `width(flux) ≤ 3·1024ε·area`, so the volume's
//!   half-width is at most `512ε·A_wall` and the area's at most
//!   `1536ε·A_wall/r` (the flux is `r` times the area plus a position
//!   term read at a point).
//! - **At `Interval`** the analytic value, itself an enclosure, lies in
//!   the reported enclosure widened by its pad, with no slack at all.
//! - **At `f64`** only the closed-form terms round: the caps' fluxes and
//!   the wall's position term, each a sum of a few products no larger
//!   than the body's distance from the origin times its area. They are
//!   held to `8u·R·S` on the volume and `8u·S` on the area, `u` the
//!   unit roundoff, `R` the farthest point's distance and `S` the area.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point3, Real, Tol, Vec3};
use topo::validate::{validate_closed, validate_geometric};
use topo::{AtRestPolicy, Body, FaceSurface, MefSite, MevSite, Rechart, mass_properties};

/// An orthonormal frame: origin and axes, in world coordinates.
#[derive(Clone, Copy)]
struct Frame {
    o: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}

impl Frame {
    /// The frame at `o` whose `z` is the unit `axis`.
    fn at(o: [f64; 3], axis: [f64; 3]) -> Self {
        let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let z = axis.map(|c| c / n);
        let t = if z[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        let d = t[0] * z[0] + t[1] * z[1] + t[2] * z[2];
        let x = [t[0] - d * z[0], t[1] - d * z[1], t[2] - d * z[2]];
        let n = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        let x = x.map(|c| c / n);
        let y = [
            z[1] * x[2] - z[2] * x[1],
            z[2] * x[0] - z[0] * x[2],
            z[0] * x[1] - z[1] * x[0],
        ];
        Self { o, x, y, z }
    }

    fn world(&self, (a, b, c): (f64, f64, f64)) -> [f64; 3] {
        core::array::from_fn(|i| self.o[i] + self.x[i] * a + self.y[i] * b + self.z[i] * c)
    }

    fn p<T: Real>(&self, at: (f64, f64, f64)) -> Point3<T> {
        let w = self.world(at);
        Point3::new(T::from_f64(w[0]), T::from_f64(w[1]), T::from_f64(w[2]))
    }

    fn v<T: Real>(&self, (a, b, c): (f64, f64, f64)) -> Vec3<T> {
        let w: [f64; 3] = core::array::from_fn(|i| self.x[i] * a + self.y[i] * b + self.z[i] * c);
        Vec3::new(T::from_f64(w[0]), T::from_f64(w[1]), T::from_f64(w[2]))
    }
}

/// One pipe: radius `r`, floor at `z0` along the frame's axis, the roof
/// `h` above it on the axis, tilted by `theta` about the frame's `y`.
#[derive(Clone, Copy, Debug)]
struct Pose {
    r: f64,
    z0: f64,
    h: f64,
    theta: f64,
}

impl Pose {
    fn volume(self) -> f64 {
        PI * self.r * self.r * self.h
    }

    fn wall(self) -> f64 {
        TAU * self.r * self.h
    }

    fn area(self) -> f64 {
        self.wall() + PI * self.r * self.r * (1.0 + 1.0 / self.theta.cos())
    }

    /// The farthest point of the pipe from the world origin, bounded.
    fn reach(self, f: &Frame) -> f64 {
        let o = (f.o[0] * f.o[0] + f.o[1] * f.o[1] + f.o[2] * f.o[2]).sqrt();
        let lean = self.r * self.theta.tan().abs();
        o + self.z0.abs() + self.h + lean + self.r
    }
}

/// A whole-turn rim on the wall, described by its chart until the cap
/// it bounds exists.
fn rim<T: Real>(cyl: topo::SurfaceKey, carrier: Curve3<T>) -> EdgeCurveSpec<T> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Chart {
            surface: cyl,
            image: None,
            wrap: false,
            declared: None,
        },
        carrier,
        param_start: T::zero(),
        param_end: T::from_f64(TAU),
    }
}

/// The pipe of `pose` in frame `f`, at the scalar `T`.
fn pipe<T: Decide + AtRestPolicy>(pose: Pose, f: &Frame, tol: Tol) -> Body<T> {
    let Pose { r, z0, h, theta } = pose;
    let (s, c) = theta.sin_cos();
    let n = (s, 0.0, c);
    let major = (c, 0.0, -s);
    let circle = Curve3::Circle {
        center: f.p((0.0, 0.0, z0)),
        axis: f.v((0.0, 0.0, 1.0)),
        radius: T::from_f64(r),
        u_ref: f.v((1.0, 0.0, 0.0)),
    };
    let ellipse = Curve3::Ellipse {
        center: f.p((0.0, 0.0, z0 + h)),
        axis: -f.v::<T>(n),
        semi_u: T::from_f64(r / c),
        semi_v: T::from_f64(r),
        u_ref: f.v(major),
    };
    let mut body = Body::<T>::new();
    let seed = body.mvfs(f.p((r, 0.0, z0)), true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: f.p((0.0, 0.0, 0.0)),
                    axis: f.v((0.0, 0.0, 1.0)),
                    radius: T::from_f64(r),
                    u_ref: f.v((1.0, 0.0, 0.0)),
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
            f.p((r, 0.0, z0 + h - r * s / c)),
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
                    origin: f.p((0.0, 0.0, z0)),
                    normal: -f.v::<T>((0.0, 0.0, 1.0)),
                    u_ref: f.v((1.0, 0.0, 0.0)),
                },
                cap_b.face,
                true,
            ),
            Rechart::new(
                Surface::Plane {
                    origin: f.p((0.0, 0.0, z0 + h)),
                    normal: f.v(n),
                    u_ref: f.v(major),
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
        (&cap_b, circle, f.p((-r, 0.0, z0))),
        (&cap_t, ellipse, f.p((-r, 0.0, z0 + h + r * s / c))),
    ] {
        let plane = body.get_face(cap.face).unwrap().surface;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(cyl, plane),
                witness,
            },
            carrier,
            param_start: T::zero(),
            param_end: T::from_f64(TAU),
        };
        body.set_edge_curve(cap.edge, spec, tol).unwrap();
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

const FRAMES: [([f64; 3], [f64; 3]); 4] = [
    ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
    ([3.0, -2.0, 5.0], [0.0, 0.0, 1.0]),
    ([1.5, 2.5, -0.5], [1.0, 2.0, 3.0]),
    ([1e3, -7e2, 4e2], [0.3, -0.2, 0.9]),
];

const POSES: [Pose; 5] = [
    Pose {
        r: 1.0,
        z0: 0.0,
        h: 2.0,
        theta: 0.4,
    },
    Pose {
        r: 1.0,
        z0: 0.0,
        h: 2.0,
        theta: -1.0,
    },
    Pose {
        r: 0.3,
        z0: 0.0,
        h: 0.5,
        theta: 0.7,
    },
    Pose {
        r: 1.0,
        z0: 100.0,
        h: 2.0,
        theta: 0.4,
    },
    Pose {
        r: 1.0,
        z0: 0.0,
        h: 30.0,
        theta: 1.5,
    },
];

/// Tiers 2 and 3, and the pads within what the reporting target allows,
/// for one pose at the scalar `T`; returns the measurement.
fn measured<T: Decide + AtRestPolicy + CertifiedBounds>(
    pose: Pose,
    f: &Frame,
    at: &str,
) -> topo::MassProperties<T> {
    let tol = Tol::witness();
    let body = pipe::<T>(pose, f, tol);
    assert_eq!(validate_closed(&body), Ok(()), "{at}: tier 2");
    assert_eq!(validate_geometric(&body, tol), Ok(()), "{at}: tier 3");
    let mp = mass_properties(&body, tol)
        .unwrap_or_else(|e| panic!("{at}: the pipe does not measure: {e}"));
    let eps = tol.eps();
    let (vpad, apad) = (
        512.0 * eps * pose.wall() * (1.0 + 1e-6),
        1536.0 * eps * pose.wall() / pose.r * (1.0 + 1e-6),
    );
    assert!(
        mp.volume_pad > 0.0 && mp.volume_pad <= vpad,
        "{at}: the wall's volume pad {:e} is positive and within the target's {vpad:e}",
        mp.volume_pad
    );
    assert!(
        mp.area_pad > 0.0 && mp.area_pad <= apad,
        "{at}: the wall's area pad {:e} is positive and within the target's {apad:e}",
        mp.area_pad
    );
    mp
}

#[test]
fn a_seamless_pipe_cut_by_an_oblique_plane_measures_in_its_closed_form() {
    let u = f64::EPSILON / 2.0;
    for (o, axis) in FRAMES {
        let f = Frame::at(o, axis);
        for pose in POSES {
            let at = format!("frame {o:?} axis {axis:?}, {pose:?}, f64");
            let mp = measured::<f64>(pose, &f, &at);
            let (volume, area) = (pose.volume(), pose.area());
            let reach = 8.0 * u * pose.reach(&f) * area;
            assert!(
                (mp.volume - volume).abs() <= mp.volume_pad + reach,
                "{at}: volume {} ± {:e} against π·r²·h = {volume} (rounding allowance {reach:e})",
                mp.volume,
                mp.volume_pad
            );
            assert!(
                (mp.surface_area - area).abs() <= mp.area_pad + 8.0 * u * area,
                "{at}: area {} ± {:e} against {area}",
                mp.surface_area,
                mp.area_pad
            );
        }
    }
}

/// The one pose measured not to converge: at `Interval` and ε = 1e-12,
/// the pipe a kilometre out with its floor 100 m up its tilted axis
/// runs its wall out of budget (width 3.3e-9 against 1.024e-9, rounds
/// 13). The same pose converges at `f64`, and at `Interval` in the
/// world frame
/// (`work/quad/quadrature-interval-floor-grows-with-the-body-past-the-band`).
fn runs_out_of_budget_at_interval(frame: [f64; 3], pose: Pose) -> bool {
    Tol::witness().eps() <= 1e-12 && frame[0] == 1e3 && pose.z0 == 100.0
}

#[test]
fn a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval() {
    for (o, axis) in FRAMES {
        let f = Frame::at(o, axis);
        for pose in POSES {
            let at = format!("frame {o:?} axis {axis:?}, {pose:?}, Interval");
            if runs_out_of_budget_at_interval(o, pose) {
                let tol = Tol::witness();
                let got = mass_properties(&pipe::<Interval>(pose, &f, tol), tol);
                assert!(
                    matches!(
                        got,
                        Err(topo::MassPropsError::Face {
                            source: geom_brep::props::PropsError::QuadratureBudget { .. },
                            ..
                        })
                    ),
                    "{at}: want the pinned budget refusal, got {got:?}"
                );
                test_utils::vacuity::stood_down(
                    &at,
                    "its wall runs the quadrature out of budget at this ε (pinned), so its \
                     enclosure is not read",
                );
                continue;
            }
            let mp = measured::<Interval>(pose, &f, &at);
            let (r, h) = (Interval::from_f64(pose.r), Interval::from_f64(pose.h));
            let volume = Interval::pi() * r * r * h;
            let cap = Interval::pi() * r * r;
            let area = Interval::tau() * r * h + cap + cap / Interval::from_f64(pose.theta).cos();
            for (what, got, pad, exact) in [
                ("volume", mp.volume, mp.volume_pad, volume),
                ("area", mp.surface_area, mp.area_pad, area),
            ] {
                assert!(
                    got.lo() - pad <= exact.hi() && exact.lo() <= got.hi() + pad,
                    "{at}: {what} {got:?} ± {pad:e} must meet the analytic {exact:?}"
                );
            }
        }
    }
}
