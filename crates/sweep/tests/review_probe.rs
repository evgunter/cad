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

/// The region arm's findings in `body`'s tier-3 report.
fn region_findings(body: &Body<f64>) -> Vec<ValidationError> {
    match topo::validate_geometric_structural(body, tol()) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .into_iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::CycleOffFaceSide { .. }
                        | ValidationError::CycleSideUndecided { .. }
                        | ValidationError::EssentialCyclesUnpaired { .. }
                )
            })
            .collect(),
    }
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

fn report(name: &str, body: &Body<f64>) {
    let r = topo::validate_geometric_structural(body, tol());
    println!("[{name}] eps {} -> {:?}", tol().eps(), r);
}

#[test]
fn probe_torus_rings() {
    let d = donut();
    let (face, frame) = torus_face(&d);
    report("donut", &d);
    // find where the face is: try both v ranges
    for (lab, vs) in [("top", (1.2, 1.9)), ("bottom", (-1.9, -1.2))] {
        let mut b = d.clone();
        plant(&mut b, face, frame, (-0.3, 0.3), vs);
        report(&format!("one window {lab}"), &b);
    }
    // nested on top and bottom
    for (lab, vs, inner) in [("top", (1.0, 2.0), (1.3, 1.7)), ("bottom", (-2.0, -1.0), (-1.7, -1.3))] {
        let mut b = d.clone();
        plant(&mut b, face, frame, (-0.5, 0.5), vs);
        plant(&mut b, face, frame, (-0.2, 0.2), inner);
        report(&format!("nested {lab}"), &b);
        let mut b = d.clone();
        plant(&mut b, face, frame, (-0.5, -0.1), vs);
        plant(&mut b, face, frame, (0.1, 0.5), vs);
        report(&format!("apart {lab}"), &b);
        let mut b = d.clone();
        plant(&mut b, face, frame, (-0.5, 0.1), vs);
        plant(&mut b, face, frame, (-0.1, 0.5), (vs.0+0.2, vs.1 + 0.2));
        report(&format!("overlapping {lab}"), &b);
    }
}

#[test]
fn probe_cylinder_overlap_and_far() {
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, |s| matches!(s, Surface::Cylinder { .. }));
    // overlapping windows: one corner of the second inside the first
    let mut b = tube.clone();
    plant(&mut b, face, frame, (-0.9, -0.1), (0.5, 1.5));
    plant(&mut b, face, frame, (-0.3, 0.5), (1.0, 1.8));
    report("cyl overlapping", &b);
    // window beyond the face's height
    let mut b = tube.clone();
    plant(&mut b, face, frame, (-0.3, 0.3), (2.5, 2.8));
    report("cyl above", &b);
    // window in another face's azimuth
    let mut b = tube.clone();
    plant(&mut b, face, frame, (2.8, 3.3), (0.5, 1.5));
    report("cyl other azimuth", &b);
    // thin band window near rim
    let mut b = tube.clone();
    plant(&mut b, face, frame, (-0.3, 0.3), (2.0 - 1e-4, 2.0 - 5e-5));
    report("cyl ring near rim", &b);
    let mut b = tube.clone();
    plant(&mut b, face, frame, (-1.2, 1.2), (1.0, 1.0 + 2e-5));
    report("cyl thin band window", &b);
}

#[test]
fn probe_sphere_near_pole() {
    let ball = { 
        let meridian = bulge_loop(vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)]);
        let profile = Profile::new(SketchPlane::xy(), vec![meridian]).validate(tol()).unwrap();
        revolve(&profile, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, Revolution::Full, tol()).unwrap().body
    };
    let (face, frame) = wall(&ball, |s| matches!(s, Surface::Sphere { .. }));
    for (lab, ss) in [("near south", (1e-3, 2e-3)), ("near north", (3.1405, 3.1410)), ("mid", (1.0,2.0)), ("tiny", (1.5, 1.5+2e-6))] {
        let mut b = ball.clone();
        plant(&mut b, face, frame, (-0.5, 0.5), ss);
        report(&format!("sphere {lab}"), &b);
    }
    // window in the other lune
    let mut b = ball.clone();
    plant(&mut b, face, frame, (2.8, 3.4), (1.0, 2.0));
    report("sphere other lune", &b);
}

mod at_interval {
    use geom_core::{Decide, Interval, Point2, Point3, Real, Tol, Vec3};
    use sweep::test_support::{ball_poled_z_at, revolved_about_y_at};
    use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
    use topo::BooleanResult;

    fn tube_at<T: Decide + topo::AtRestPolicy>(x: T, a: T, z0: T, len: T) -> topo::Body<T> {
        let tol = Tol::witness();
        let o = T::zero();
        let circle = profile::circle(Point2::new(x, o), a, tol).unwrap();
        let plane = profile::SketchPlane::xy_at(Point3::new(o, o, z0));
        let p = profile::Profile::new(plane, vec![circle.into()]).validate(tol).unwrap();
        extrude(&p, Extrusion::Distance { depth: len, side: ExtrudeSide::Along }, tol).unwrap().body
    }

    fn run<T: Decide + topo::AtRestPolicy + core::fmt::Debug + geom_core::Bounds + geom_core::CertifiedBounds>(lab: &str, a: topo::Body<T>, b: topo::Body<T>) {
        let tol = Tol::witness();
        let a = topo::AtRestBody::validate(a, tol).expect("operand a");
        let b = topo::AtRestBody::validate(b, tol).expect("operand b");
        for (op, r) in [("sub", topo::subtract(&a, &b, tol)), ("int", topo::intersect(&a, &b, tol)), ("uni", topo::union(&a, &b, tol))] {
            match r.as_ref().map(BooleanResult::body) {
                Ok(Some(bb)) => {
                    let v = topo::validate_geometric(&bb.body, tol);
                    println!("[{lab} {op}] {} faces -> {:?}", bb.body.faces().count(), v.as_ref().map_err(|e| e.iter().map(|e| format!("{e:?}").chars().take(160).collect::<String>()).collect::<Vec<_>>()));
                }
                other => println!("[{lab} {op}] no body: {:?}", other.map(|_| ()).map_err(|e| format!("{e:?}").chars().take(900).collect::<String>())),
            }
        }
    }

    fn cases<T: Decide + topo::AtRestPolicy + core::fmt::Debug + geom_core::Bounds + geom_core::CertifiedBounds>(f: impl Fn(f64) -> T, scal: &str) {
        let tol = Tol::witness();
        let z = || Vec3::new(f(0.0), f(0.0), f(0.0));
        run(&format!("{scal} ball-through-tube"), ball_poled_z_at(f(2.0), z(), tol), tube_at(f(0.0), f(0.5), f(-3.0), f(6.0)));
        run(&format!("{scal} ball-offaxis-tube"), ball_poled_z_at(f(2.0), z(), tol), tube_at(f(0.7), f(0.4), f(-3.0), f(6.0)));
        run(&format!("{scal} ball-near-pole-thin-tube"), ball_poled_z_at(f(2.0), z(), tol), tube_at(f(0.05), f(0.02), f(-3.0), f(6.0)));
        let donut = || revolved_about_y_at(vec![(Point2::new(f(1.0), f(0.0)), f(1.0)), (Point2::new(f(3.0), f(0.0)), f(1.0))], Revolution::Full, tol);
        run(&format!("{scal} donut-tube"), donut(), tube_at(f(0.0), f(0.3), f(-4.0), f(8.0)));
        run(&format!("{scal} donut-offtube"), donut(), tube_at(f(0.5), f(0.3), f(-4.0), f(8.0)));
    }

    #[test]
    fn probe_multi_cycle_curved_faces_at_f64_and_interval() {
        cases(|x| x, "f64");
        cases(Interval::from_f64, "interval");
    }
}

mod windows_at_interval {
    use super::*;
    use geom_core::Real;
    use geom_core::Interval;
    type I = Interval;
    fn iv(x: f64) -> I { Interval::from_f64(x) }
    fn p3(p: Point3<f64>) -> Point3<I> { Point3::new(iv(p.x), iv(p.y), iv(p.z)) }
    fn v3(p: Vec3<f64>) -> Vec3<I> { Vec3::new(iv(p.x), iv(p.y), iv(p.z)) }
    fn tol() -> Tol { Tol::witness() }

    fn circ(c: Curve3<f64>, t0: f64, t1: f64) -> EdgeCurveSpec<I> {
        let Curve3::Circle { center, axis, radius, u_ref } = c else { panic!() };
        EdgeCurveSpec::arc_of_circle(Curve3::Circle { center: p3(center), axis: v3(axis), radius: iv(radius), u_ref: v3(u_ref) }, iv(t0), iv(t1)).expect("an arc")
    }
    fn arc(fr: &Frame, s: f64, (u0, u1): (f64, f64), forward: bool) -> EdgeCurveSpec<I> {
        let (h, rho) = fr.parallel(s);
        let (axis, u_ref) = if forward { (fr.axis, fr.e1) } else { (-fr.axis, fr.radial(u1)) };
        let (t0, t1) = if forward { (u0, u1) } else { (0.0, u1 - u0) };
        circ(Curve3::Circle { center: fr.origin + fr.axis * h, axis, radius: rho, u_ref }, t0, t1)
    }
    fn meridian(fr: &Frame, u: f64, (s0, s1): (f64, f64)) -> EdgeCurveSpec<I> {
        match fr.kind {
            Kind::Sphere { radius } => circ(Curve3::Circle { center: fr.origin, axis: (-fr.axis).cross(fr.radial(u)), radius, u_ref: -fr.axis }, s0, s1),
            Kind::Torus { major, minor } => circ(Curve3::Circle { center: fr.origin + fr.radial(u) * major, axis: fr.radial(u).cross(fr.axis), radius: minor, u_ref: fr.radial(u) }, s0, s1),
            _ => EdgeCurveSpec::line_between(p3(fr.at(u, s0)), p3(fr.at(u, s1))),
        }
    }
    fn plant_iv(body: &mut Body<I>, face: FaceKey, fr: Frame, us: (f64, f64), ss: (f64, f64)) {
        let f = body.get_face(face).unwrap();
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else { panic!() };
        let p = [fr.at(us.0, ss.0), fr.at(us.1, ss.0), fr.at(us.1, ss.1), fr.at(us.0, ss.1)].map(p3);
        let strut = body.mev_line(MevSite::Fan { he1: first, he2: first }, p[0], tol()).unwrap();
        let fan = |he| MevSite::Fan { he1: he, he2: he };
        let a = body.mev(fan(strut.he_minus), p[1], arc(&fr, ss.0, us, true), tol()).unwrap();
        let b = body.mev(fan(a.he_minus), p[2], meridian(&fr, us.1, ss), tol()).unwrap();
        let c = body.mev(fan(b.he_minus), p[3], arc(&fr, ss.1, us, false), tol()).unwrap();
        let close = body.mef(MefSite::Chords { he1: a.he_plus, he2: c.he_minus }, meridian(&fr, us.0, ss), FaceSurface::Inherit, tol()).unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        for edge in [a.edge, b.edge, c.edge, close.edge] { body.describe_at_rest(edge, tol()).unwrap(); }
        topo::mint_pcurves(body, tol()).unwrap();
    }
    fn rep(lab: &str, b: &Body<I>) {
        let r = topo::validate_geometric_structural(b, tol());
        println!("[iv {lab}] -> {:?}", r.map_err(|e| e.iter().map(|e| format!("{e:?}").chars().take(900).collect::<String>()).collect::<Vec<_>>()));
    }

    #[test]
    fn probe_windows_at_interval() {
        use sweep::test_support::revolved_about_y_at;
        // sphere
        let ball64 = { let m = bulge_loop(vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)]);
            let pr = Profile::new(SketchPlane::xy(), vec![m]).validate(tol()).unwrap();
            revolve(&pr, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, Revolution::Full, tol()).unwrap().body };
        let (face, fr) = wall(&ball64, |s| matches!(s, Surface::Sphere { .. }));
        let ball = revolved_about_y_at(vec![(Point2::new(iv(0.0), iv(-1.0)), iv(1.0)), (Point2::new(iv(0.0), iv(1.0)), iv(0.0))], sweep::Revolution::Full, tol());
        rep("ball", &ball);
        let cases: &[(&str, &[((f64, f64), (f64, f64))])] = &[
            ("one window", &[((-0.5, 0.5), (1.0, 2.0))]),
            ("two apart", &[((-0.9, -0.1), (1.0, 2.0)), ((0.1, 0.9), (1.0, 2.0))]),
            ("nested", &[((-0.9, -0.1), (1.0, 2.0)), ((-0.7, -0.3), (1.3, 1.7))]),
            ("near south pole", &[((-0.5, 0.5), (1e-3, 2e-3))]),
            ("near north pole", &[((-0.5, 0.5), (3.1405, 3.1410))]),
            ("two near south pole", &[((-0.5, -0.1), (1e-2, 2e-2)), ((0.1, 0.5), (1e-2, 2e-2))]),
            ("sliver beside pole", &[((-0.5, 0.5), (0.01, 0.01 + 1e-5))]),
            ("thin band", &[((-1.2, 1.2), (1.5, 1.5 + 2e-5))]),
            ("other lune", &[((2.8, 3.4), (1.0, 2.0))]),
        ];
        for (lab, ws) in cases {
            let mut b = ball.clone();
            for &(us, ss) in *ws { plant_iv(&mut b, face, fr, us, ss); }
            rep(&format!("sphere {lab}"), &b);
            let mut b64 = ball64.clone();
            for &(us, ss) in *ws { plant(&mut b64, face, fr, us, ss); }
            println!("[f64 sphere {lab}] -> {:?}", topo::validate_geometric_structural(&b64, tol()).is_ok());
        }
        // torus
        let d64 = donut();
        let (tface, tfr) = torus_face(&d64);
        let d = revolved_about_y_at(vec![(Point2::new(iv(1.0), iv(0.0)), iv(1.0)), (Point2::new(iv(3.0), iv(0.0)), iv(1.0))], sweep::Revolution::Full, tol());
        rep("donut", &d);
        let tcases: &[(&str, &[((f64, f64), (f64, f64))])] = &[
            ("one window", &[((-0.3, 0.3), (1.2, 1.9))]),
            ("two apart", &[((-0.5, -0.1), (1.0, 2.0)), ((0.1, 0.5), (1.0, 2.0))]),
            ("nested", &[((-0.5, 0.5), (1.0, 2.0)), ((-0.2, 0.2), (1.3, 1.7))]),
            ("off the face", &[((-0.3, 0.3), (-1.9, -1.2))]),
            ("near rim", &[((-0.3, 0.3), (1e-4, 2e-4))]),
        ];
        {
            let mut b = d.clone();
            plant_iv(&mut b, tface, tfr, (-0.5, -0.1), (1.0, 2.0));
            plant_iv(&mut b, tface, tfr, (0.1, 0.5), (1.0, 2.0));
            let band = geom_core::Band::linear(tol()).unwrap();
            for (u, v) in [(0.0, 1.5), (0.0, 0.5), (-0.3, 1.5), (-0.3, 0.5), (1.0, 1.5), (0.3, 2.5)] {
                let q = p3(tfr.at(u, v));
                let r = topo::curved_face_containment(&b, tface, q, band);
                println!("[iv torus full-door u={u} v={v}] -> {}", format!("{r:?}").chars().take(300).collect::<String>());
            }
            let mut b1 = d.clone();
            plant_iv(&mut b1, tface, tfr, (-0.5, -0.1), (1.0, 2.0));
            for (u, v) in [(0.3, 1.5), (-0.3, 1.5), (0.3, 0.5)] {
                let q = p3(tfr.at(u, v));
                let r = topo::curved_face_containment(&b1, tface, q, band);
                println!("[iv torus one-window full-door u={u} v={v}] -> {}", format!("{r:?}").chars().take(300).collect::<String>());
            }
        }
        for (lab, ws) in tcases {
            let mut b = d.clone();
            for &(us, ss) in *ws { plant_iv(&mut b, tface, tfr, us, ss); }
            rep(&format!("torus {lab}"), &b);
            let mut b64 = d64.clone();
            for &(us, ss) in *ws { plant(&mut b64, tface, tfr, us, ss); }
            println!("[f64 torus {lab}] -> {:?}", topo::validate_geometric_structural(&b64, tol()).map_err(|e| e.len()));
        }
    }
}

#[test]
fn probe_many_rings_timing() {
    let tube = revolved(&[(1.0, 0.0), (1.0, 4.0), (0.0, 4.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, |s| matches!(s, Surface::Cylinder { .. }));
    for n in [4usize, 8, 16, 24] {
        let mut b = tube.clone();
        let cols = 4; let rows = n / cols;
        for i in 0..cols { for j in 0..rows {
            let u0 = -1.2 + 0.6 * i as f64; let s0 = 0.2 + (3.6 / rows as f64) * j as f64;
            plant(&mut b, face, frame, (u0, u0 + 0.4), (s0, s0 + 0.5 * 3.6 / rows as f64));
        }}
        let t = std::time::Instant::now();
        let r = topo::validate_geometric_structural(&b, tol());
        let total = t.elapsed();
        println!("[timing rings={n}] validate {:?} ok={}", total, r.is_ok());
    }
}
