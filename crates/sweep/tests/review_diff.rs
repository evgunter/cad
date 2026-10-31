//! **One region per face, off the plane** (tier 3; `topo::one_region`),
//! from an outside consumer's seat: each body is a solid of revolution
//! built by `revolve`, with windows planted on its curved wall through
//! the public Euler doors (`mev`, `mef`, `kemr`). A window's plug face
//! keeps the shell closed, so the only thing wrong with a misplaced one
//! is where its ring lies, and no other check sees it: the hole's area
//! is subtracted from a face it is not in.
//!
//! Every row is a pair: the window placed in the face validates, and the
//! same window placed off it refuses `CycleOffFaceSide` on that face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, LoopBoundary, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

/// The polygon `pts` of the `xy` sketch plane revolved a full turn
/// about `y`.
fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let meridian = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .expect("the meridian profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// A surface of revolution's frame, and its point at azimuth `u` and
/// meridian parameter `s`: a cylinder's height, a cone's slant, a
/// sphere's angle from its south pole, so that on every kind `s` runs
/// along the axis and the window's corners turn one way.
#[derive(Clone, Copy)]
struct Frame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    e1: Vec3<f64>,
    kind: Kind,
}

#[derive(Clone, Copy)]
enum Kind {
    Cylinder { radius: f64 },
    Cone { half_angle: f64 },
    Sphere { radius: f64 },
    Torus { major: f64, minor: f64 },
}

impl Frame {
    fn of(surface: &Surface<f64>) -> Self {
        let (origin, axis, e1, kind) = match *surface {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => (origin, axis, u_ref, Kind::Cylinder { radius }),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => (apex, axis, u_ref, Kind::Cone { half_angle }),
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => (center, axis, u_ref, Kind::Sphere { radius }),
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => (center, axis, u_ref, Kind::Torus { major: major_radius, minor: minor_radius }),
            _ => panic!("a surface of revolution"),
        };
        Self {
            origin,
            axis,
            e1,
            kind,
        }
    }

    fn radial(&self, u: f64) -> Vec3<f64> {
        let e2 = self.axis.cross(self.e1);
        self.e1 * u.cos() + e2 * u.sin()
    }

    /// The parallel at `s`: its height along the axis and its radius.
    fn parallel(&self, s: f64) -> (f64, f64) {
        match self.kind {
            Kind::Cylinder { radius } => (s, radius),
            Kind::Cone { half_angle } => (s * half_angle.cos(), s * half_angle.sin()),
            Kind::Sphere { radius } => (-radius * s.cos(), radius * s.sin()),
            Kind::Torus { major, minor } => (minor * s.sin(), major + minor * s.cos()),
        }
    }

    fn at(&self, u: f64, s: f64) -> Point3<f64> {
        let (h, rho) = self.parallel(s);
        self.origin + self.axis * h + self.radial(u) * rho
    }

    /// The arc of the parallel at `s` from azimuth `u0` to `u1`, run
    /// toward increasing azimuth, or back from `u1` to `u0`.
    fn arc(&self, s: f64, (u0, u1): (f64, f64), forward: bool) -> EdgeCurveSpec<f64> {
        let (h, rho) = self.parallel(s);
        let (axis, u_ref) = if forward {
            (self.axis, self.e1)
        } else {
            (-self.axis, self.radial(u1))
        };
        let carrier = Curve3::Circle {
            center: self.origin + self.axis * h,
            axis,
            radius: rho,
            u_ref,
        };
        let (t0, t1) = if forward { (u0, u1) } else { (0.0, u1 - u0) };
        EdgeCurveSpec::arc_of_circle(carrier, t0, t1).expect("an arc")
    }

    /// The meridian at azimuth `u` from `s0` to `s1`: a ruling, or a
    /// sphere's great-circle arc.
    fn meridian(&self, u: f64, (s0, s1): (f64, f64)) -> EdgeCurveSpec<f64> {
        match self.kind {
            Kind::Cylinder { .. } | Kind::Cone { .. } => {
                EdgeCurveSpec::line_between(self.at(u, s0), self.at(u, s1))
            }
            Kind::Sphere { radius } => {
                let carrier = Curve3::Circle {
                    center: self.origin,
                    axis: (-self.axis).cross(self.radial(u)),
                    radius,
                    u_ref: -self.axis,
                };
                EdgeCurveSpec::arc_of_circle(carrier, s0, s1).expect("a meridian arc")
            }
            Kind::Torus { major, minor } => {
                let carrier = Curve3::Circle {
                    center: self.origin + self.radial(u) * major,
                    axis: self.radial(u).cross(self.axis),
                    radius: minor,
                    u_ref: self.radial(u),
                };
                EdgeCurveSpec::arc_of_circle(carrier, s0, s1).expect("a torus meridian arc")
            }
        }
    }
}

/// The face of `body` on the surface `kind` picks, and its frame with
/// its azimuth turned so that `u = 0` is the middle of the face's rim:
/// `revolve` may make a wall of more than one face, and a row's windows
/// are placed within `±π/2` of that middle.
fn wall(body: &Body<f64>, kind: fn(&Surface<f64>) -> bool) -> (FaceKey, Frame) {
    let (face, frame) = body
        .faces()
        .find_map(|(k, f)| {
            let s = body.get_surface(f.surface)?;
            kind(s).then(|| (k, Frame::of(s)))
        })
        .expect("the wall");
    let LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the outer loop is a cycle")
    };
    // The middle of a circle edge of the outer loop, and the face's side
    // of it there (`N × τ`, `N` the outward normal).
    let (rim, side) = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .find_map(|h| {
            let half = body.get_half_edge(h)?;
            let edge = body.get_edge(half.edge)?;
            let curve = body.get_curve_geom(edge.curve)?.certified()?;
            let (t0, t1) = curve.params();
            let t = 0.5 * (t0 + t1);
            let tangent = curve.carrier().deriv(t) * if h == edge.he_plus { 1.0 } else { -1.0 };
            let at = curve.carrier().eval(t);
            let sense = if body.get_face(face)?.sense {
                1.0
            } else {
                -1.0
            };
            matches!(curve.carrier(), Curve3::Circle { .. })
                .then(|| (at, (at - frame.origin).cross(tangent) * sense))
        })
        .expect("a circle edge");
    let w = rim - frame.origin;
    let mut u = w.dot(frame.axis.cross(frame.e1)).atan2(w.dot(frame.e1));
    // A sphere's circle edge may be a meridian bounding a lune: the
    // lune's middle is a quarter turn to the face's side of it.
    if let (Kind::Sphere { .. }, Some(meridian)) = (frame.kind, {
        let along = frame.axis.cross(frame.radial(u));
        (side.dot(along).abs() > 0.5 * side.norm()).then_some(along)
    }) {
        u += core::f64::consts::FRAC_PI_2 * side.dot(meridian).signum();
    }
    // A cylinder's heights from its face's lowest vertex along its axis;
    // a cone's slants from its apex.
    let low = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|h| {
            let v = body
                .get_vertex(body.get_half_edge(h).unwrap().start)
                .unwrap();
            (*body.get_point(v.point).unwrap() - frame.origin).dot(frame.axis)
        })
        .fold(f64::INFINITY, f64::min);
    let origin = match frame.kind {
        Kind::Cylinder { .. } => frame.origin + frame.axis * low,
        Kind::Cone { .. } | Kind::Sphere { .. } | Kind::Torus { .. } => frame.origin,
    };
    (
        face,
        Frame {
            origin,
            e1: frame.radial(u),
            ..frame
        },
    )
}

fn is_cylinder(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cylinder { .. })
}

fn is_cone(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cone { .. })
}

fn is_sphere(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Sphere { .. })
}

/// **A window planted on `face`**: the chart rectangle azimuth `us` ×
/// meridian `ss`, its two parallels' arcs and its two rulings, a plug
/// face filling it and its loop a ring of `face`. Grown off `face`'s
/// outer loop wherever the rectangle lies: the Euler doors read no
/// geometry. Returns the plug.
fn plant(
    body: &mut Body<f64>,
    face: FaceKey,
    frame: Frame,
    us: (f64, f64),
    ss: (f64, f64),
) -> FaceKey {
    let f = body.get_face(face).unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the outer loop is a cycle")
    };
    let p = [
        frame.at(us.0, ss.0),
        frame.at(us.1, ss.0),
        frame.at(us.1, ss.1),
        frame.at(us.0, ss.1),
    ];
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            p[0],
            tol(),
        )
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let a = body
        .mev(fan(strut.he_minus), p[1], frame.arc(ss.0, us, true), tol())
        .unwrap();
    let b = body
        .mev(fan(a.he_minus), p[2], frame.meridian(us.1, ss), tol())
        .unwrap();
    let c = body
        .mev(fan(b.he_minus), p[3], frame.arc(ss.1, us, false), tol())
        .unwrap();
    let close = body
        .mef(
            MefSite::Chords {
                he1: a.he_plus,
                he2: c.he_minus,
            },
            frame.meridian(us.0, ss),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    for edge in [a.edge, b.edge, c.edge, close.edge] {
        body.describe_at_rest(edge, tol()).unwrap();
    }
    topo::mint_pcurves(body, tol()).unwrap();
    let plug = close.face;
    assert!(
        !body.get_face(face).unwrap().rings.is_empty(),
        "the window is a ring of the wall"
    );
    plug
}

fn donut() -> Body<f64> {
    let meridian = bulge_loop(vec![
        (Point2::new(1.0, 0.0), 1.0),
        (Point2::new(3.0, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .unwrap();
    revolve(&profile, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) },
        Revolution::Full, tol()).unwrap().body
}

fn torus_face(body: &Body<f64>) -> (FaceKey, Frame) {
    let (face, frame) = body.faces().find_map(|(k, f)| {
        let s = body.get_surface(f.surface)?;
        matches!(s, Surface::Torus { .. }).then(|| (k, Frame::of(s)))
    }).unwrap();
    // print vertices (u, v)
    let f = body.get_face(face).unwrap();
    for lk in core::iter::once(&f.outer).chain(&f.rings) {
        if let LoopBoundary::Cycle { first } = body.get_loop(*lk).unwrap().boundary {
            for h in body.loop_cycle(first).unwrap() {
                let v = body.get_vertex(body.get_half_edge(h).unwrap().start).unwrap();
                let p = *body.get_point(v.point).unwrap() - frame.origin;
                let e2 = frame.axis.cross(frame.e1);
                let u = p.dot(e2).atan2(p.dot(frame.e1));
                let rad = (p - frame.axis * p.dot(frame.axis)).norm();
                let Kind::Torus { major, .. } = frame.kind else { panic!() };
                let vv = p.dot(frame.axis).atan2(rad - major);
                println!("face {face:?} sense {} vertex u={u:.3} v={vv:.3}", f.sense);
            }
        }
    }
    (face, frame)
}

fn _report(name: &str, body: &Body<f64>) {
    let r = topo::validate_geometric_structural(body, tol());
    println!("[{name}] eps {} -> {:?}", tol().eps(), r);
}


#[test]
fn review_diff_containment() {
    let ball = {
        let m = bulge_loop(vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)]);
        let pr = Profile::new(SketchPlane::xy(), vec![m]).validate(tol()).unwrap();
        revolve(&pr, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, Revolution::Full, tol()).unwrap().body
    };
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let cone = revolved(&[(0.5 * s, 0.5 * c), (2.5 * s, 2.5 * c), (0.0, 2.5 * c), (0.0, 0.5 * c)]);
    let d = donut();
    let mut bodies: Vec<(String, Body<f64>, FaceKey, Frame, (f64, f64))> = Vec::new();
    {
        let (f, fr) = wall(&ball, |s| matches!(s, Surface::Sphere { .. }));
        let mut b = ball.clone();
        plant(&mut b, f, fr, (-0.9, -0.1), (1.0, 2.0));
        plant(&mut b, f, fr, (0.1, 0.9), (1.0, 2.0));
        plant(&mut b, f, fr, (-0.5, 0.5), (0.01, 0.01 + 1e-4));
        bodies.push(("sphere".into(), b, f, fr, (0.0, 3.14159)));
        let (f, fr) = wall(&tube, |s| matches!(s, Surface::Cylinder { .. }));
        let mut b = tube.clone();
        plant(&mut b, f, fr, (-0.9, -0.1), (0.5, 1.5));
        plant(&mut b, f, fr, (0.1, 0.9), (0.5, 1.5));
        plant(&mut b, f, fr, (-1.2, 1.2), (1.6, 1.6 + 1e-4));
        bodies.push(("cyl".into(), b, f, fr, (0.0, 2.0)));
        let (f, fr) = wall(&cone, |s| matches!(s, Surface::Cone { .. }));
        let mut b = cone.clone();
        plant(&mut b, f, fr, (-0.5, 0.5), (1.0, 2.0));
        plant(&mut b, f, fr, (0.7, 1.1), (1.0, 2.0));
        bodies.push(("cone".into(), b, f, fr, (0.5, 2.5)));
        let (f, fr) = torus_face(&d);
        let mut b = d.clone();
        plant(&mut b, f, fr, (-0.5, -0.1), (1.0, 2.0));
        plant(&mut b, f, fr, (0.1, 0.5), (1.0, 2.0));
        bodies.push(("torus".into(), b, f, fr, (0.0, 3.14159)));
    }
    let band = geom_core::Band::linear(tol()).unwrap();
    for (lab, b, f, fr, (s0, s1)) in &bodies {
        assert!(topo::validate_geometric_structural(b, tol()).is_ok(), "{lab} honest");
        for (face, _) in b.faces() {
            let _ = f;
            for i in 0..=40 {
                for j in 0..=24 {
                    let u = -3.14159 + 6.28318 * f64::from(i) / 40.0;
                    let sv = s0 + (s1 - s0) * f64::from(j) / 24.0;
                    let q = fr.at(u, sv);
                    let r = topo::curved_face_containment(b, face, q, band);
                    println!("DIFF {lab} {face:?} {i} {j} {}", match r { Ok(x) => format!("{x:?}"), Err(e) => format!("ERR {}", format!("{e:?}").split(['{', '(']).next().unwrap().to_string()) });
                }
            }
        }
    }
}
