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
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MevSite, Rechart};
use topo::validate::{validate_closed, validate_geometric};
use topo::{AtRestPolicy, Body, mass_properties};

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
                selector: geom_brep::CurveSelector::Witness(witness),
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
    let mp = mass_properties(
        &topo::test_support::finished("the measured body", body.clone(), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap_or_else(|e| panic!("{at}: the pipe does not measure: {e}"));
    let eps = tol.eps();
    let (vpad, apad) = (
        512.0 * eps * pose.wall() * (1.0 + 1e-6),
        1536.0 * eps * pose.wall() / pose.r * (1.0 + 1e-6),
    );
    assert!(
        mp.enclosure.volume_pad > 0.0 && mp.enclosure.volume_pad <= vpad,
        "{at}: the wall's volume pad {:e} is positive and within the target's {vpad:e}",
        mp.enclosure.volume_pad
    );
    assert!(
        mp.enclosure.area_pad > 0.0 && mp.enclosure.area_pad <= apad,
        "{at}: the wall's area pad {:e} is positive and within the target's {apad:e}",
        mp.enclosure.area_pad
    );
    mp.enclosure
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

/// Every pose converges at `Interval`, at every ε. The pipe a kilometre
/// out with its floor 100 m up its tilted axis once ran its wall out of
/// budget at ε = 1e-12 (width 3.3e-9 against 1.024e-9, rounds 13), read
/// about the world origin; each face is read about a vertex of its
/// shell (`topo::props`' `shell_reference`), and it converges.
#[test]
fn a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval() {
    for (o, axis) in FRAMES {
        let f = Frame::at(o, axis);
        for pose in POSES {
            let at = format!("frame {o:?} axis {axis:?}, {pose:?}, Interval");
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

/// The document door's default whole-query budget
/// (`editor_core::clearance::DEFAULT_MAX_CELL_PAIRS`).
const BUDGET: usize = 65_536;

/// A witness: two faces, the closest pair of on-face stations, and
/// their distance.
#[derive(Clone, Copy, Debug)]
struct Witness {
    a: topo::FaceKey,
    b: topo::FaceKey,
    pa: Point3<f64>,
    d: f64,
}

/// The injectivity question over `faces` of `body` (every face where
/// `None`), at the default depth and `budget` cell pairs (the document
/// door's default is [`BUDGET`]). Where `witnessed`, a candidate pair is a witness where the
/// closest pair of its cells' on-face stations, evaluated afresh, is
/// decided at distance zero; otherwise none is offered, so a
/// coincidence refuses rather than reads `Violated`.
fn injectivity(
    body: &Body<f64>,
    faces: Option<Vec<topo::FaceKey>>,
    witnessed: bool,
    budget: usize,
    tol: Tol,
) -> topo::clearance::BoundReport<Witness> {
    use geom_core::k_stats::decide;
    use geom_core::{Band, Margin, Sign};
    use topo::clearance::{Budgets, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which};
    let band = Band::linear(tol).unwrap();
    let faces = faces.unwrap_or_else(|| body.faces().map(|(k, _)| k).collect());
    let side = Side { body, faces };
    let windows = side.windows(Which::First).expect("every face has a window");
    let bound = ClearanceBound::StrictlyPositive;
    let sweep = Sweep {
        bound,
        band,
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: budget,
        },
    };
    let surface = |f: topo::FaceKey| {
        body.get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let report = sweep.run(&windows, &windows, true, |candidate| {
        if !witnessed {
            return Err("no witness is offered".to_owned());
        }
        let (fa, fb) = candidate.faces();
        let (sa, sb) = (surface(fa), surface(fb));
        let (la, lb) = candidate.on_face_stations();
        let mut best: Option<Witness> = None;
        for &(au, av) in &la {
            let pa = sa.eval(au, av);
            for &(bu, bv) in &lb {
                let d = (pa - sb.eval(bu, bv)).norm();
                if best.is_none_or(|w| d < w.d) {
                    best = Some(Witness {
                        a: fa,
                        b: fb,
                        pa,
                        d,
                    });
                }
            }
        }
        let w = best.ok_or("no station")?;
        match decide(bound.predicate(), Margin::of(w.d - bound.c()), band) {
            Ok(Sign::Negative | Sign::Zero) => Ok(w),
            _ => Err(format!("the closest station pair is {} apart", w.d)),
        }
    });
    assert!(report.receipt.holds(), "{:?}", report.receipt);
    report
}

/// **A curved junction certifies across the departure arm.** The pipe's
/// wall meets its oblique roof along an ellipse: on the wall's chart
/// that is no coordinate line, and on the roof's plane no straight one.
/// Each face is presented by the region on its side of the ellipse's
/// chart curve, and the other face's carrier separates them — in every
/// frame and pose, the wall–roof and wall–floor pairs each certified
/// across faces at least once. Red if the curved presentation is
/// refused, or reads the wrong side of the curve so that no separator
/// takes opposite signs.
#[test]
fn a_curved_junction_certifies_across_the_departure_arm() {
    use topo::clearance::BoundVerdict;
    let tol = Tol::witness();
    for (o, axis) in FRAMES {
        let f = Frame::at(o, axis);
        for pose in POSES {
            let body = pipe::<f64>(pose, &f, tol);
            let report = injectivity(&body, None, true, BUDGET, tol);
            println!(
                "[curvedep] pipe {o:?} {axis:?} {pose:?}: {:?} {:?}",
                report.verdict, report.receipt
            );
            assert!(
                matches!(report.verdict, BoundVerdict::Holds),
                "the pipe in frame {o:?} {axis:?}, {pose:?}, is embedded: {:?} {:?}",
                report.verdict,
                report.receipt
            );
            let faces: Vec<topo::FaceKey> = body.faces().map(|(k, _)| k).collect();
            let wall = faces
                .iter()
                .copied()
                .find(|&k| {
                    matches!(
                        body.get_surface(body.get_face(k).unwrap().surface).unwrap(),
                        Surface::Cylinder { .. }
                    )
                })
                .unwrap();
            for &cap in faces.iter().filter(|&&k| k != wall) {
                let pair = (wall.min(cap), wall.max(cap));
                assert!(
                    report.across.get(&pair).is_some_and(|&n| n > 0),
                    "the wall and each cap are certified across: {:?}",
                    report.across
                );
            }
        }
    }
}

/// **The pipe mitred at its roof onto a second cylinder**: the wall and
/// floor of `pose`, and in place of the roof's plane a band of the OTHER
/// circular cylinder that holds the roof's ellipse — the wall's own
/// cylinder mirrored through the roof's plane — from the ellipse to a
/// circle `drop` along its axis past the ellipse's vertex, closed by a
/// plane. The two cylinders touch at the ellipse's two points on the
/// line `x = 0` of the roof's plane, and meet again along a second
/// ellipse, in the plane `z = h + x·cot θ`.
///
/// - `drop < 0` hangs the band ABOVE the roof: an elbow, embedded, the
///   band and the wall on opposite sides of the roof's plane.
/// - `drop > 0` folds it back BELOW the roof, through the wall along the
///   second ellipse, a crossing that runs into the junction at the two
///   points the cylinders touch.
fn mitred(pose: Pose, f: &Frame, drop: f64, tol: Tol) -> Body<f64> {
    let band_sense = drop < 0.0;
    let Pose { r, z0, h, theta } = pose;
    let (s, c) = theta.sin_cos();
    let n = (s, 0.0, c);
    let major = (c, 0.0, -s);
    // The mirror of the wall's axis through the roof: `z − 2(z·n)n`.
    let a2 = (-2.0 * c * s, 0.0, 1.0 - 2.0 * c * c);
    let circle = Curve3::Circle {
        center: f.p((0.0, 0.0, z0)),
        axis: f.v((0.0, 0.0, 1.0)),
        radius: r,
        u_ref: f.v((1.0, 0.0, 0.0)),
    };
    let ellipse = Curve3::Ellipse {
        center: f.p((0.0, 0.0, z0 + h)),
        axis: -f.v::<f64>(n),
        semi_u: r / c,
        semi_v: r,
        u_ref: f.v(major),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(f.p((r, 0.0, z0)), true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Cylinder {
                    origin: f.p((0.0, 0.0, 0.0)),
                    axis: f.v((0.0, 0.0, 1.0)),
                    radius: r,
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
    let start = (r, 0.0, z0 + h - r * s / c);
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: cap_b.he_plus,
                he2: cap_b.he_plus,
            },
            f.p(start),
            tol,
        )
        .unwrap();
    // The band's cylinder: the wall's mirrored through the roof, about
    // the axis through the roof's centre, its reference ruling through
    // the ellipse's vertex.
    let o2 = (0.0, 0.0, z0 + h);
    let w = (start.0 - o2.0, start.1 - o2.1, start.2 - o2.2);
    let along = w.0 * a2.0 + w.1 * a2.1 + w.2 * a2.2;
    let radial = (w.0 - along * a2.0, w.1 - along * a2.1, w.2 - along * a2.2);
    let rn = (radial.0 * radial.0 + radial.1 * radial.1 + radial.2 * radial.2).sqrt();
    let u2 = (radial.0 / rn, radial.1 / rn, radial.2 / rn);
    let band = body
        .mef(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_minus,
            },
            rim(cyl, ellipse),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // Lifts `RechartUnvouched`: the ellipse lies on the mirrored cylinder
    // by construction. It keeps its image on the wall's chart, since the
    // cylinders touch at two of its points, where no intersection
    // description certifies.
    body.set_face_surface_unvouched_for_tests(
        band.face,
        FaceSurface::New {
            surface: Surface::Cylinder {
                origin: f.p(o2),
                axis: f.v(a2),
                radius: r,
                u_ref: f.v(u2),
            },
            sense: band_sense,
        },
    )
    .unwrap();
    body.set_face_surfaces_describing(
        vec![Rechart::new(
            Surface::Plane {
                origin: f.p((0.0, 0.0, z0)),
                normal: -f.v::<f64>((0.0, 0.0, 1.0)),
                u_ref: f.v((1.0, 0.0, 0.0)),
            },
            cap_b.face,
            true,
        )],
        &[],
        tol,
    )
    .unwrap();
    let floor = body.get_face(cap_b.face).unwrap().surface;
    let mirror = body.get_face(band.face).unwrap().surface;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(cyl, floor),
            selector: geom_brep::CurveSelector::Witness(f.p((-r, 0.0, z0))),
        },
        carrier: circle,
        param_start: 0.0,
        param_end: TAU,
    };
    body.set_edge_curve(cap_b.edge, spec, tol).unwrap();
    // The band's far rim: the circle `drop` along the mirrored axis past
    // the ellipse's vertex, closed by a plane.
    let foot = (
        start.0 + drop * a2.0,
        start.1 + drop * a2.1,
        start.2 + drop * a2.2,
    );
    let centre = (
        o2.0 + (along + drop) * a2.0,
        o2.1 + (along + drop) * a2.1,
        o2.2 + (along + drop) * a2.2,
    );
    let lower = Curve3::Circle {
        center: f.p(centre),
        axis: f.v(a2),
        radius: r,
        u_ref: f.v(u2),
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: band.he_minus,
                he2: band.he_minus,
            },
            f.p(foot),
            tol,
        )
        .unwrap();
    let cap_l = body
        .mef(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_minus,
            },
            rim(mirror, lower.clone()),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    body.set_face_surfaces_describing(
        vec![Rechart::new(
            Surface::Plane {
                origin: f.p(centre),
                normal: f.v(a2),
                u_ref: f.v(u2),
            },
            cap_l.face,
            false,
        )],
        &[],
        tol,
    )
    .unwrap();
    let plane = body.get_face(cap_l.face).unwrap().surface;
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(mirror, plane),
            selector: geom_brep::CurveSelector::Witness(f.p((
                centre.0 - r * u2.0,
                centre.1 - r * u2.1,
                centre.2 - r * u2.2,
            ))),
        },
        carrier: lower,
        param_start: 0.0,
        param_end: TAU,
    };
    body.set_edge_curve(cap_l.edge, spec, tol).unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// The faces of `body` on a cylinder: the wall and the band.
fn walls(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(
                body.get_surface(body.get_face(k).unwrap().surface).unwrap(),
                Surface::Cylinder { .. }
            )
        })
        .collect()
}

/// **A mitre between two cylinders certifies across its ellipse.** The
/// elbow of [`mitred`]: the wall and the band meet on an ellipse that
/// is no coordinate line of either chart, so each is presented by the
/// region on its side of the ellipse's chart curve; through the two
/// points where the cylinders touch no carrier separates them, and the
/// graph of the ellipse or its own plane does. Holds over every face in
/// two frames, the wall–band pair certified across. Red if the curved
/// presentation is refused, or if a run that stops on a column end at
/// the ellipse's join does not take the next piece (the join's cells
/// then split to the floor).
#[test]
fn a_mitre_between_two_cylinders_certifies_across_its_ellipse() {
    use topo::clearance::BoundVerdict;
    let tol = Tol::witness();
    for (o, axis) in [FRAMES[0], FRAMES[2]] {
        let f = Frame::at(o, axis);
        let body = mitred(POSES[0], &f, -1.5, tol);
        let report = injectivity(&body, None, true, BUDGET, tol);
        println!(
            "[curvedep] mitre {o:?} {axis:?}: {:?} {:?}",
            report.verdict, report.receipt
        );
        assert!(
            matches!(report.verdict, BoundVerdict::Holds),
            "the elbow in frame {o:?} {axis:?} is embedded: {:?} {:?}",
            report.verdict,
            report.receipt
        );
        let [a, b] = walls(&body)[..] else {
            panic!("a wall and a band")
        };
        assert!(
            report
                .across
                .get(&(a.min(b), a.max(b)))
                .is_some_and(|&n| n > 0),
            "the wall and the band are certified across: {:?}",
            report.across
        );
    }
}

/// **The mitre folded back through the wall reads `Violated`.** The
/// fold of [`mitred`], the wall against the band: they touch on the
/// ellipse and cross along the second one, which runs into it. The
/// witness is a coincidence of the two faces off the shared ellipse,
/// where the two cells no longer touch and distance decides them: pinned
/// at least `0.05` off it, against `0.58` measured at ε = 1e-9 and
/// `0.115` at 1e-12 (nearer the point the cylinders touch) — a 2.3×
/// margin at the smallest ε. Red if a departure
/// certificate discharges a wall–band cell pair whose cells hold the
/// crossing — a presentation that reads less than a face's part of its
/// cell, or a certificate that ignores it.
#[test]
fn the_mitre_folded_back_through_the_wall_reads_violated() {
    use topo::clearance::BoundVerdict;
    let tol = Tol::witness();
    let f = Frame::at(FRAMES[0].0, FRAMES[0].1);
    let body = mitred(POSES[0], &f, 1.5, tol);
    let faces = walls(&body);
    let report = injectivity(&body, Some(faces.clone()), true, BUDGET, tol);
    println!("[curvedep] fold: {:?} {:?}", report.verdict, report.receipt);
    match report.verdict {
        BoundVerdict::Violated(w) => {
            assert!(
                faces.contains(&w.a) && faces.contains(&w.b) && w.a != w.b,
                "the witness is the wall against the band: {w:?}"
            );
            assert!(
                w.d <= 10.0 * tol.eps(),
                "a coincidence, at the run's tolerance: {w:?}"
            );
            // The roof's plane holds the shared ellipse: a point that far
            // off it is that far off the ellipse.
            let Pose { z0, h, theta, .. } = POSES[0];
            let (s, c) = theta.sin_cos();
            let off = (w.pa.x * s + (w.pa.z - z0 - h) * c).abs();
            assert!(
                off >= 0.05,
                "the witness is off the shared ellipse, where the cells no longer \
                 touch: {off} from the roof's plane, {w:?}"
            );
        }
        other => panic!("the band crosses the wall: {other:?} {:?}", report.receipt),
    }
}

/// **The unit disk on the plane `z = 0` and what lies across its rim.**
/// The disk is one face; the other is the plane's region on the far side
/// of the rim (where `lamina` is false) out to the circle of radius 2,
/// the annulus, or the same disk again with its sense flipped (where
/// `lamina`), a face folded back onto the first along the rim. Returns
/// the body, the disk, and the other face.
fn across_a_rim(lamina: bool, tol: Tol) -> (Body<f64>, topo::FaceKey, topo::FaceKey) {
    let circle = |r: f64| Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(1.0, 0.0, 0.0), true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin: Point3::new(0.0, 0.0, 0.0),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    let disk = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            rim(plane, circle(1.0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    if lamina {
        let sense = body.get_face(seed.face).unwrap().sense;
        body.set_face_sense(seed.face, !sense).unwrap();
    } else {
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: disk.he_plus,
                    he2: disk.he_plus,
                },
                Point3::new(2.0, 0.0, 0.0),
                tol,
            )
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: strut.he_minus,
                he2: strut.he_minus,
            },
            rim(plane, circle(2.0)),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    (body, disk.face, seed.face)
}

/// **A curved rim folded back on itself refuses, never certified.** A
/// lamina: the unit disk and the same disk again, its sense flipped,
/// sharing the rim. The two faces coincide, so no separator takes
/// opposite strict signs on any pair of their cells and nothing
/// certifies, whatever order the sweep reads the pairs in; no witness is
/// offered, so the coincident interiors refuse. Which refusal comes
/// first is the sweep's order, so it is not pinned here: the rim pair's
/// own outcome, a cusp the sweep types `SecondOrder`, is pinned on the
/// pair itself (`injective`'s
/// `a_rim_folded_on_itself_is_a_cusp_and_across_it_certifies`). Red if
/// a cell pair of the lamina is certified, or the lamina holds.
#[test]
fn a_curved_rim_folded_on_itself_refuses_never_certified() {
    use topo::clearance::BoundVerdict;
    let tol = Tol::witness();
    let (body, disk, other) = across_a_rim(true, tol);
    let report = injectivity(&body, Some(vec![disk, other]), false, 4096, tol);
    println!(
        "[curvedep] lamina: {:?} {:?}",
        report.verdict, report.receipt
    );
    assert!(
        matches!(report.verdict, BoundVerdict::Refused(_)),
        "the lamina refuses: {:?}",
        report.verdict
    );
    assert_eq!(
        report.receipt.certified, 0,
        "nothing certified: {:?}",
        report.receipt
    );
}

/// **The disk and its annulus certify across their rim** — the lamina's
/// wedge-π complement: the rim's chart curve presents each face on its
/// side, coplanar, so no carrier and no plane separate them, and the
/// rim's graph does. Holds, the pair certified across.
#[test]
fn a_disk_and_its_annulus_certify_across_their_rim() {
    use topo::clearance::BoundVerdict;
    let tol = Tol::witness();
    let (body, disk, other) = across_a_rim(false, tol);
    let report = injectivity(&body, Some(vec![disk, other]), true, BUDGET, tol);
    println!(
        "[curvedep] annulus: {:?} {:?}",
        report.verdict, report.receipt
    );
    assert!(
        matches!(report.verdict, BoundVerdict::Holds),
        "coplanar across a rim: {:?}",
        report.verdict
    );
    assert!(
        report
            .across
            .get(&(disk.min(other), disk.max(other)))
            .is_some_and(|&n| n > 0),
        "certified across: {:?}",
        report.across
    );
}
