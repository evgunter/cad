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
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
        Kind::Cone { .. } | Kind::Sphere { .. } => frame.origin,
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
/// The unit ball: the half-disc right of `y` revolved about it.
fn revolved_disc() -> Body<f64> {
    let meridian = bulge_loop(vec![
        (Point2::new(0.0, -1.0), 1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .expect("the half-disc profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the half-disc revolves")
    .body
}

/// The donut: the unit disc about `(2, 0)` revolved about `y`.
fn donut() -> Body<f64> {
    let meridian = bulge_loop(vec![
        (Point2::new(1.0, 0.0), 1.0),
        (Point2::new(3.0, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .expect("the disc profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the disc revolves")
    .body
}


// ======================= REVIEW PROBES (PR 6163) =======================

fn tag(r: &Result<Option<topo::FaceContainment>, topo::ContainError>) -> String {
    use topo::FaceContainment as F;
    match r {
        Ok(Some(F::In)) => "I".into(),
        Ok(Some(F::Out)) => "O".into(),
        Ok(Some(F::OnEdge(_))) => "E".into(),
        Ok(Some(F::OnVertex(_))) => "V".into(),
        Ok(None) => "-".into(),
        Err(_) => "x".into(),
    }
}

/// Sample points across each curved face's surface, print the door's answers.
fn dump(name: &str, body: &Body<f64>) {
    let band = geom_core::Band::linear(tol()).unwrap();
    for (fi, (face, f)) in body.faces().enumerate() {
        let s = body.get_surface(f.surface).unwrap().clone();
        let pts: Vec<Point3<f64>> = match s {
            Surface::Torus { center, axis, major_radius, minor_radius, u_ref } => {
                let e2 = axis.cross(u_ref);
                let mut v = Vec::new();
                for i in 0..40 { for j in 0..40 {
                    let u = -3.1 + 6.2 * f64::from(i) / 40.0;
                    let w = -3.1 + 6.2 * f64::from(j) / 40.0;
                    let rad = u_ref * u.cos() + e2 * u.sin();
                    v.push(center + rad * (major_radius + minor_radius * w.cos()) + axis * (minor_radius * w.sin()));
                }}
                v
            }
            Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => {
                let fr = Frame::of(&s);
                let (lo, hi) = match fr.kind {
                    Kind::Cylinder { .. } => (-1.0, 3.0),
                    Kind::Cone { .. } => (0.05, 4.0),
                    Kind::Sphere { .. } => (0.02, 3.12),
                };
                let mut v = Vec::new();
                for i in 0..48 { for j in 0..48 {
                    let u = -3.1 + 6.2 * f64::from(i) / 48.0;
                    let ss = lo + (hi - lo) * f64::from(j) / 47.0;
                    v.push(fr.at(u, ss));
                }}
                v
            }
            _ => continue,
        };
        let line: String = pts.iter().map(|&q| tag(&topo::curved_face_containment(body, face, q, band))).collect();
        println!("DUMP {name} face#{fi} {line}");
    }
}

fn bodies() -> Vec<(String, Body<f64>)> {
    let mut out = Vec::new();
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let band = revolved(&[(s, c), (2.0 * s, 2.0 * c), (0.0, 2.0 * c), (0.0, c)]);
    let (face, frame) = wall(&band, is_cone);
    let mut b = band.clone(); plant(&mut b, face, frame, (-0.5, 0.5), (1.3, 1.7)); out.push(("cone-band-honest".into(), b));
    let mut b = band.clone(); plant(&mut b, face, frame, (-0.5, 0.5), (3.0, 3.3)); out.push(("cone-band-P09".into(), b));
    let cap = revolved(&[(0.0, 0.0), (2.0 * s, 2.0 * c), (0.0, 2.0 * c)]);
    let (face, frame) = wall(&cap, is_cone);
    let mut b = cap.clone(); plant(&mut b, face, frame, (-0.5, 0.5), (1.0, 1.5)); out.push(("cone-cap-honest".into(), b));
    let mut b = cap.clone(); plant(&mut b, face, frame, (-0.5, 0.5), (3.0, 3.5)); out.push(("cone-cap-P47".into(), b));
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, is_cylinder);
    let mut b = tube.clone(); plant(&mut b, face, frame, (-0.9, -0.1), (0.5, 1.5)); plant(&mut b, face, frame, (0.1, 0.9), (0.5, 1.5)); out.push(("tube-honest".into(), b));
    let mut b = tube.clone(); plant(&mut b, face, frame, (-0.9, -0.1), (0.5, 1.5)); plant(&mut b, face, frame, (-0.7, -0.3), (0.8, 1.2)); out.push(("tube-nested".into(), b));
    let ball = revolved_disc();
    let (face, frame) = wall(&ball, is_sphere);
    let mut b = ball.clone(); plant(&mut b, face, frame, (-0.9, -0.1), (1.0, 2.0)); plant(&mut b, face, frame, (0.1, 0.9), (1.0, 2.0)); out.push(("ball-honest".into(), b));
    let mut b = ball.clone(); plant(&mut b, face, frame, (-0.9, -0.1), (1.0, 2.0)); plant(&mut b, face, frame, (-0.7, -0.3), (1.3, 1.7)); out.push(("ball-nested".into(), b));
    let mut b = ball.clone(); plant(&mut b, face, frame, (-0.5, 0.5), (0.01, 0.2)); out.push(("ball-nearpole".into(), b));
    out.push(("donut".into(), donut()));
    out
}

#[test]
fn review_door_dump() {
    for (name, b) in bodies() { dump(&name, &b); }
}

// ---- HEAD ONLY ----
fn region_only(body: &Body<f64>) -> Vec<String> {
    match topo::validate_geometric_structural(body, tol()) {
        Ok(()) => Vec::new(),
        Err(errors) => errors.into_iter().map(|e| format!("{e:?}")).collect(),
    }
}

#[test]
fn review_no_false_refusal() {
    let (eps, k) = (tol().eps(), tol().k());
    let near = 40.0 * k * eps;
    let mut cases: Vec<(String, Box<dyn Fn() -> Body<f64>>)> = Vec::new();
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    for g in [1e-2, 1e-4, near] {
        cases.push((format!("cone band window gap {g:e}"), Box::new(move || {
            let mut b = revolved(&[(s, c), (2.0 * s, 2.0 * c), (0.0, 2.0 * c), (0.0, c)]);
            let (face, frame) = wall(&b, is_cone);
            plant(&mut b, face, frame, (-0.5, 0.5), (1.0 + g, 2.0 - g)); b })));
        cases.push((format!("tube two windows gap {g:e}"), Box::new(move || {
            let mut b = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
            let (face, frame) = wall(&b, is_cylinder);
            plant(&mut b, face, frame, (-0.9, -g / 2.0), (0.5, 1.5));
            plant(&mut b, face, frame, (g / 2.0, 0.9), (0.5, 1.5)); b })));
        cases.push((format!("tube window near rims gap {g:e}"), Box::new(move || {
            let mut b = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
            let (face, frame) = wall(&b, is_cylinder);
            plant(&mut b, face, frame, (-0.9, 0.9), (g, 2.0 - g)); b })));
        cases.push((format!("tube hole in plug gap {g:e}"), Box::new(move || {
            let mut b = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
            let (face, frame) = wall(&b, is_cylinder);
            let plug = plant(&mut b, face, frame, (-0.9, 0.9), (0.5, 1.5));
            plant(&mut b, plug, frame, (-0.9 + g, 0.9 - g), (0.5 + g, 1.5 - g)); b })));
        cases.push((format!("ball window near lune sides gap {g:e}"), Box::new(move || {
            let mut b = revolved_disc();
            let (face, frame) = wall(&b, is_sphere);
            plant(&mut b, face, frame, (-1.5 + g, 1.5 - g), (1.0, 2.0)); b })));
        cases.push((format!("ball hole in plug gap {g:e}"), Box::new(move || {
            let mut b = revolved_disc();
            let (face, frame) = wall(&b, is_sphere);
            let plug = plant(&mut b, face, frame, (-1.0, 1.0), (1.0, 2.0));
            plant(&mut b, plug, frame, (-1.0 + g, 1.0 - g), (1.0 + g, 2.0 - g)); b })));
    }
    for w in [1e-2, 1e-3] {
        cases.push((format!("thin cone band {w:e}"), Box::new(move || {
            let mut b = revolved(&[(s, c), ((1.0 + w) * s, (1.0 + w) * c), (0.0, (1.0 + w) * c), (0.0, c)]);
            let (face, frame) = wall(&b, is_cone);
            plant(&mut b, face, frame, (-0.5, 0.5), (1.0 + 0.25 * w, 1.0 + 0.75 * w)); b })));
    }
    for s0 in [1e-2, 1e-3, 1e-4] {
        cases.push((format!("ball window near S pole {s0:e}"), Box::new(move || {
            let mut b = revolved_disc();
            let (face, frame) = wall(&b, is_sphere);
            plant(&mut b, face, frame, (-0.5, 0.5), (s0, 2.0 * s0)); b })));
        cases.push((format!("ball window near N pole {s0:e}"), Box::new(move || {
            let mut b = revolved_disc();
            let (face, frame) = wall(&b, is_sphere);
            let pi = core::f64::consts::PI;
            plant(&mut b, face, frame, (-0.5, 0.5), (pi - 2.0 * s0, pi - s0)); b })));
    }
    let mut bad = 0;
    println!("eps={eps:e} k={k}");
    std::panic::set_hook(Box::new(|_| {}));
    for (name, build) in cases {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| build())) {
            Err(_) => println!("PROBE {name}: SKIP (does not build)"),
            Ok(b) => {
                let errs = region_only(&b);
                let region = errs.iter().filter(|e| e.starts_with("CycleOffFaceSide") || e.starts_with("CycleSideUndecided") || e.starts_with("EssentialCycles")).count();
                if region > 0 { bad += 1; }
                println!("PROBE {name}: region={region} all={}", errs.len());
            }
        }
    }
    let _ = std::panic::take_hook();
    assert_eq!(bad, 0, "region arm refused an honest body");
}

mod interval_probe {
    use super::*;
    use geom_core::{Interval, Real};
    fn pt<T: Real>(p: Point3<f64>) -> Point3<T> { Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)) }
    fn vc<T: Real>(p: Vec3<f64>) -> Vec3<T> { Vec3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)) }

    fn disc<T: Real + geom_core::Decide>() -> Body<T> where T: topo::props::AtRestPolicy {
        let meridian = bulge_loop::<T>(vec![
            (Point2::new(T::from_f64(0.0), T::from_f64(-1.0)), T::from_f64(1.0)),
            (Point2::new(T::from_f64(0.0), T::from_f64(1.0)), T::from_f64(0.0)),
        ]);
        let profile = Profile::new(SketchPlane::<T>::xy(), vec![meridian]).validate(tol()).unwrap();
        revolve(&profile, RevolveAxis { origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)), dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)) }, Revolution::Full, tol()).unwrap().body
    }

    /// The f64 `plant` replayed at `T` from the f64 frame's specs.
    pub fn plant_t<T>(body: &mut Body<T>, face: FaceKey, frame: Frame, us: (f64, f64), ss: (f64, f64)) -> FaceKey
    where T: geom_core::Decide + topo::props::AtRestPolicy + geom_core::SpanLocate {
        let f = body.get_face(face).unwrap();
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else { panic!() };
        let p = [frame.at(us.0, ss.0), frame.at(us.1, ss.0), frame.at(us.1, ss.1), frame.at(us.0, ss.1)].map(pt::<T>);
        let arc = |s: f64, fwd: bool| {
            let (h, rho) = frame.parallel(s);
            let (axis, u_ref) = if fwd { (frame.axis, frame.e1) } else { (-frame.axis, frame.radial(us.1)) };
            let carrier = Curve3::Circle { center: pt(frame.origin + frame.axis * h), axis: vc(axis), radius: T::from_f64(rho), u_ref: vc(u_ref) };
            let (t0, t1) = if fwd { us } else { (0.0, us.1 - us.0) };
            EdgeCurveSpec::arc_of_circle(carrier, T::from_f64(t0), T::from_f64(t1)).unwrap()
        };
        let mer = |u: f64| {
            let carrier = Curve3::Circle { center: pt(frame.origin), axis: vc((-frame.axis).cross(frame.radial(u))), radius: T::from_f64(1.0), u_ref: vc(-frame.axis) };
            EdgeCurveSpec::arc_of_circle(carrier, T::from_f64(ss.0), T::from_f64(ss.1)).unwrap()
        };
        let strut = body.mev_line(MevSite::Fan { he1: first, he2: first }, p[0], tol()).unwrap();
        let fan = |he| MevSite::Fan { he1: he, he2: he };
        let a = body.mev(fan(strut.he_minus), p[1], arc(ss.0, true), tol()).unwrap();
        let b = body.mev(fan(a.he_minus), p[2], mer(us.1), tol()).unwrap();
        let c = body.mev(fan(b.he_minus), p[3], arc(ss.1, false), tol()).unwrap();
        let close = body.mef(MefSite::Chords { he1: a.he_plus, he2: c.he_minus }, mer(us.0), FaceSurface::Inherit, tol()).unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        for edge in [a.edge, b.edge, c.edge, close.edge] { body.describe_at_rest(edge, tol()).unwrap(); }
        topo::mint_pcurves(body, tol()).unwrap();
        close.face
    }

    fn run<T>(name: &str) where T: geom_core::Decide + geom_core::Bounds + topo::props::AtRestPolicy + geom_core::SpanLocate {
        let ball64 = revolved_disc();
        let (face64, frame) = wall(&ball64, is_sphere);
        let idx = ball64.faces().position(|(k, _)| k == face64).unwrap();
        for (us, ss) in [((-0.5, 0.5), (1.0, 2.0)), ((-0.5, 0.5), (0.3, 0.6)), ((-1.0, 1.0), (0.5, 2.6))] {
            let mut ball: Body<T> = disc::<T>();
            let face = ball.faces().nth(idx).unwrap().0;
            assert_eq!(topo::validate_geometric_structural(&ball, tol()), Ok(()), "{name} plain ball");
            plant_t(&mut ball, face, frame, us, ss);
            let r = topo::validate_geometric_structural(&ball, tol());
            println!("IPROBE {name} window {us:?}x{ss:?}: {r:?}");
        }
    }

    fn nested<T>(name: &str) where T: geom_core::Decide + geom_core::Bounds + topo::props::AtRestPolicy + geom_core::SpanLocate {
        let ball64 = revolved_disc();
        let (face64, frame) = wall(&ball64, is_sphere);
        let idx = ball64.faces().position(|(k, _)| k == face64).unwrap();
        let mut ball: Body<T> = disc::<T>();
        let face = ball.faces().nth(idx).unwrap().0;
        plant_t(&mut ball, face, frame, (-0.9, -0.1), (1.0, 2.0));
        plant_t(&mut ball, face, frame, (-0.7, -0.3), (1.3, 1.7));
        println!("IPROBE {name} nested: {:?}", topo::validate_geometric_structural(&ball, tol()));
        let mut ball: Body<T> = disc::<T>();
        plant_t(&mut ball, face, frame, (-0.5, 0.5), (0.1, 0.4));
        plant_t(&mut ball, face, frame, (-0.5, 0.5), (2.7, 3.0));
        println!("IPROBE {name} two near poles: {:?}", topo::validate_geometric_structural(&ball, tol()));
    }

    #[test]
    fn review_interval_nested() {
        nested::<f64>("f64");
        nested::<Interval>("interval");
    }

    #[test]
    fn review_interval_ball_window() {
        run::<f64>("f64");
        run::<Interval>("interval");
    }
}

mod torus_probe {
    use super::*;
    #[derive(Clone, Copy)]
    struct Tor { c: Point3<f64>, ax: Vec3<f64>, e1: Vec3<f64>, big: f64, small: f64 }
    impl Tor {
        fn rad(&self, u: f64) -> Vec3<f64> { self.e1 * u.cos() + self.ax.cross(self.e1) * u.sin() }
        fn at(&self, u: f64, v: f64) -> Point3<f64> { self.c + self.rad(u) * (self.big + self.small * v.cos()) + self.ax * (self.small * v.sin()) }
        fn par(&self, v: f64, us: (f64, f64), fwd: bool) -> EdgeCurveSpec<f64> {
            let center = self.c + self.ax * (self.small * v.sin());
            let radius = self.big + self.small * v.cos();
            let (axis, u_ref) = if fwd { (self.ax, self.e1) } else { (-self.ax, self.rad(us.1)) };
            let (t0, t1) = if fwd { us } else { (0.0, us.1 - us.0) };
            EdgeCurveSpec::arc_of_circle(Curve3::Circle { center, axis, radius, u_ref }, t0, t1).unwrap()
        }
        fn mer(&self, u: f64, vs: (f64, f64)) -> EdgeCurveSpec<f64> {
            let r = self.rad(u);
            EdgeCurveSpec::arc_of_circle(Curve3::Circle { center: self.c + r * self.big, axis: r.cross(self.ax), radius: self.small, u_ref: r }, vs.0, vs.1).unwrap()
        }
    }
    fn plant_t(body: &mut Body<f64>, face: FaceKey, t: Tor, us: (f64, f64), vs: (f64, f64)) -> FaceKey {
        let f = body.get_face(face).unwrap();
        let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else { panic!() };
        let p = [t.at(us.0, vs.0), t.at(us.1, vs.0), t.at(us.1, vs.1), t.at(us.0, vs.1)];
        let strut = body.mev_line(MevSite::Fan { he1: first, he2: first }, p[0], tol()).unwrap();
        let fan = |he| MevSite::Fan { he1: he, he2: he };
        let a = body.mev(fan(strut.he_minus), p[1], t.par(vs.0, us, true), tol()).unwrap();
        let b = body.mev(fan(a.he_minus), p[2], t.mer(us.1, vs), tol()).unwrap();
        let c = body.mev(fan(b.he_minus), p[3], t.par(vs.1, us, false), tol()).unwrap();
        let close = body.mef(MefSite::Chords { he1: a.he_plus, he2: c.he_minus }, t.mer(us.0, vs), FaceSurface::Inherit, tol()).unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        for edge in [a.edge, b.edge, c.edge, close.edge] { body.describe_at_rest(edge, tol()).unwrap(); }
        topo::mint_pcurves(body, tol()).unwrap();
        close.face
    }

    #[test]
    fn review_torus_rings() {
        let d = donut();
        let band = geom_core::Band::linear(tol()).unwrap();
        let (face, t) = d.faces().find_map(|(k, f)| match *d.get_surface(f.surface)? {
            Surface::Torus { center, axis, major_radius, minor_radius, u_ref } => Some((k, Tor { c: center, ax: axis, e1: u_ref, big: major_radius, small: minor_radius })),
            _ => None,
        }).unwrap();
        // pick v where the face is: test centres
        let mut inside = None; let mut outside = None;
        for v in [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0] {
            let r = topo::curved_face_containment(&d, face, t.at(0.3, v), band);
            println!("TPROBE v={v} {:?}", r);
            match r { Ok(Some(topo::FaceContainment::In)) => { inside.get_or_insert(v); } Ok(Some(topo::FaceContainment::Out)) => { outside.get_or_insert(v); } _ => {} }
        }
        let (vi, vo) = (inside.unwrap(), outside.unwrap());
        let mut honest = d.clone();
        plant_t(&mut honest, face, t, (0.2, 0.8), (vi - 0.3, vi + 0.3));
        println!("TPROBE honest: {:?}", topo::validate_geometric_structural(&honest, tol()));
        let mut off = d.clone();
        plant_t(&mut off, face, t, (0.2, 0.8), (vo - 0.3, vo + 0.3));
        println!("TPROBE off-face ring: {:?}", topo::validate_geometric_structural(&off, tol()));
        let mut nested = d.clone();
        plant_t(&mut nested, face, t, (0.2, 0.8), (vi - 0.3, vi + 0.3));
        plant_t(&mut nested, face, t, (0.3, 0.7), (vi - 0.15, vi + 0.15));
        println!("TPROBE nested rings: {:?}", topo::validate_geometric_structural(&nested, tol()));
        let mut two = d.clone();
        plant_t(&mut two, face, t, (0.2, 0.8), (vi - 0.3, vi + 0.3));
        plant_t(&mut two, face, t, (1.2, 1.8), (vi - 0.3, vi + 0.3));
        println!("TPROBE two rings apart: {:?}", topo::validate_geometric_structural(&two, tol()));
    }
}

mod revolve_probe {
    use super::*;
    fn circle(cx: f64, r: f64) -> profile::ProfileLoop<f64> {
        bulge_loop(vec![(Point2::new(cx - r, 0.0), 1.0), (Point2::new(cx + r, 0.0), 1.0)])
    }
    fn rev(loops: Vec<profile::ProfileLoop<f64>>, how: Revolution<f64>) -> Result<Body<f64>, String> {
        let profile = Profile::new(SketchPlane::xy(), loops).validate(tol()).map_err(|e| format!("{e:?}"))?;
        revolve(&profile, RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, how, tol())
            .map(|r| r.body).map_err(|e| format!("{e:?}"))
    }
    #[test]
    fn review_revolves() {
        let cases: Vec<(&str, Vec<profile::ProfileLoop<f64>>, Revolution<f64>)> = vec![
            ("disc partial 1.0", vec![circle(2.0, 1.0)], Revolution::Partial(1.0)),
            ("disc partial 3.0", vec![circle(2.0, 1.0)], Revolution::Partial(3.0)),
            ("disc partial -5.5", vec![circle(2.0, 1.0)], Revolution::Partial(-5.5)),
            ("hollow donut", vec![circle(2.0, 1.0), circle(2.0, 0.5)], Revolution::Full),
            ("hollow partial 2", vec![circle(2.0, 1.0), circle(2.0, 0.5)], Revolution::Partial(2.0)),
            ("disc w/ off-centre hole", vec![circle(2.0, 1.0), circle(2.4, 0.3)], Revolution::Full),
            ("thin tube r=1e-3", vec![circle(2.0, 1e-3)], Revolution::Full),
            ("ball partial", vec![bulge_loop(vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)])], Revolution::Partial(2.0)),
            ("ball with cavity", vec![bulge_loop(vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)]), circle(0.5, 0.2)], Revolution::Full),
            ("rect with rect hole", vec![bulge_loop(vec![(Point2::new(1.0, 0.0), 0.0), (Point2::new(3.0, 0.0), 0.0), (Point2::new(3.0, 2.0), 0.0), (Point2::new(1.0, 2.0), 0.0)]), bulge_loop(vec![(Point2::new(1.5, 0.5), 0.0), (Point2::new(1.5, 1.5), 0.0), (Point2::new(2.5, 1.5), 0.0), (Point2::new(2.5, 0.5), 0.0)])], Revolution::Full),
        ];
        let mut bad = 0;
        for (name, loops, how) in cases {
            match rev(loops, how) {
                Err(e) => println!("RPROBE {name}: does not build: {}", &e[..e.len().min(150)]),
                Ok(b) => {
                    let r = topo::validate_geometric_structural(&b, tol());
                    let s = format!("{r:?}");
                    if s.contains("CycleOffFaceSide") || s.contains("CycleSideUndecided") || s.contains("EssentialCycles") { bad += 1; }
                    println!("RPROBE {name}: {}", &s[..s.len().min(300)]);
                }
            }
        }
        assert_eq!(bad, 0);
    }
}
