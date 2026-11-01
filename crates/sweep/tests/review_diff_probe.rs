//! **One region per face, off the plane** (tier 3; `topo::one_region`),
//! from an outside consumer's seat: each body is a solid of revolution
//! built by `revolve`, with windows planted on its curved wall through
//! the public Euler doors (`mev`, `mef`, `kemr`). A window's plug face
//! keeps the shell closed, so the only thing wrong with a misplaced one
//! is where its ring lies, and no other check sees it: the hole's area
//! is subtracted from a face it is not in.
//!
//! Every row is a pair: the window placed in the face validates, and the
//! same window placed off it refuses `CycleOffFaceSide` on that face. The
//! sphere and torus rows run at `f64` and again at the certified
//! `Interval`, from the same `f64` frame, since a reading the doors settle
//! at one scalar can stay in band at the other.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point2, Point3, Real, SpanLocate, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::AtRestPolicy;
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, LoopBoundary, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

/// The scalars a row builds at: `f64`, and the certified `Interval`.
trait Scalar: Decide + AtRestPolicy + SpanLocate + Bounds + core::fmt::Debug {}
impl<T: Decide + AtRestPolicy + SpanLocate + Bounds + core::fmt::Debug> Scalar for T {}

fn pt<T: Real>(p: Point3<f64>) -> Point3<T> {
    p.map(T::from_f64)
}

fn vc<T: Real>(v: Vec3<f64>) -> Vec3<T> {
    v.map(T::from_f64)
}

/// The loop `(x, y, bulge)` of the `xy` sketch plane revolved a full turn
/// about `y`, at `T`.
fn revolved<T: Scalar>(pts: &[(f64, f64, f64)]) -> Body<T> {
    let meridian = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b)))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .expect("the meridian profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(T::zero(), T::zero()),
            dir: Vec2::new(T::zero(), T::one()),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// A polygon of the `xy` sketch plane revolved a full turn about `y`.
fn revolved_polygon(pts: &[(f64, f64)]) -> Body<f64> {
    revolved(&pts.iter().map(|&(x, y)| (x, y, 0.0)).collect::<Vec<_>>())
}

/// The unit ball: the half-disc right of `y` revolved about it.
fn revolved_disc<T: Scalar>() -> Body<T> {
    revolved(&[(0.0, -1.0, 1.0), (0.0, 1.0, 0.0)])
}

/// The donut: the unit disc about `(2, 0)` revolved about `y`.
fn donut<T: Scalar>() -> Body<T> {
    revolved(&[(1.0, 0.0, 1.0), (3.0, 0.0, 1.0)])
}

/// A surface of revolution's frame, and its point at azimuth `u` and
/// meridian parameter `s`: a cylinder's height, a cone's slant, a
/// sphere's angle from its south pole, a torus's tube angle from the
/// middle of the face ([`wall`]), so that on every kind `s` runs along
/// the meridian one way and the window's corners turn one way.
#[derive(Clone, Copy)]
struct Frame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    e1: Vec3<f64>,
    kind: Kind,
    /// Added to every `s`: the tube angle of the torus face's middle.
    shift: f64,
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
            } => (
                center,
                axis,
                u_ref,
                Kind::Torus {
                    major: major_radius,
                    minor: minor_radius,
                },
            ),
            _ => panic!("a surface of revolution"),
        };
        Self {
            origin,
            axis,
            e1,
            kind,
            shift: 0.0,
        }
    }

    fn radial(&self, u: f64) -> Vec3<f64> {
        let e2 = self.axis.cross(self.e1);
        self.e1 * u.cos() + e2 * u.sin()
    }

    /// The parallel at `s`: its height along the axis and its radius.
    fn parallel(&self, s: f64) -> (f64, f64) {
        let s = s + self.shift;
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
    /// toward increasing azimuth, or back from `u1` to `u0`, at `T`.
    fn arc<T: Scalar>(&self, s: f64, (u0, u1): (f64, f64), forward: bool) -> EdgeCurveSpec<T> {
        let (h, rho) = self.parallel(s);
        let (axis, u_ref) = if forward {
            (self.axis, self.e1)
        } else {
            (-self.axis, self.radial(u1))
        };
        let (t0, t1) = if forward { (u0, u1) } else { (0.0, u1 - u0) };
        circle_arc(self.origin + self.axis * h, axis, rho, u_ref, (t0, t1))
    }

    /// The meridian at azimuth `u` from `s0` to `s1`, at `T`: a ruling, or
    /// a sphere's great-circle arc, or a torus's tube circle.
    fn meridian<T: Scalar>(&self, u: f64, (s0, s1): (f64, f64)) -> EdgeCurveSpec<T> {
        let (t0, t1) = (s0 + self.shift, s1 + self.shift);
        let r = self.radial(u);
        match self.kind {
            Kind::Cylinder { .. } | Kind::Cone { .. } => {
                EdgeCurveSpec::line_between(pt(self.at(u, s0)), pt(self.at(u, s1)))
            }
            Kind::Sphere { radius } => circle_arc(
                self.origin,
                (-self.axis).cross(r),
                radius,
                -self.axis,
                (t0, t1),
            ),
            Kind::Torus { major, minor } => circle_arc(
                self.origin + r * major,
                r.cross(self.axis),
                minor,
                r,
                (t0, t1),
            ),
        }
    }
}

/// The arc over `(t0, t1)` of the circle `(center, axis, radius, u_ref)`,
/// at `T`.
fn circle_arc<T: Scalar>(
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
    (t0, t1): (f64, f64),
) -> EdgeCurveSpec<T> {
    let carrier = Curve3::Circle {
        center: pt(center),
        axis: vc(axis),
        radius: T::from_f64(radius),
        u_ref: vc(u_ref),
    };
    EdgeCurveSpec::arc_of_circle(carrier, T::from_f64(t0), T::from_f64(t1)).expect("an arc")
}

/// The face of `body` on the surface `kind` picks, and its frame turned
/// to the face: azimuth `u = 0` is the middle of the face's rim, since
/// `revolve` may make a wall of more than one face and a row's windows
/// are placed within `±π/2` of that middle; a cylinder's heights run from
/// the face's lowest vertex; a sphere lune's `u = 0` is a quarter turn to
/// the face's side of its meridian; a torus face's `s = 0` is the middle
/// of its tube span, a quarter turn to the face's side of its rim.
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
    let sense = if body.get_face(face).unwrap().sense {
        1.0
    } else {
        -1.0
    };
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
            let normal = match frame.kind {
                Kind::Torus { major, .. } => {
                    let w = at - frame.origin;
                    let radial = w - frame.axis * w.dot(frame.axis);
                    at - (frame.origin + radial / radial.norm() * major)
                }
                _ => at - frame.origin,
            };
            matches!(curve.carrier(), Curve3::Circle { .. })
                .then(|| (at, normal.cross(tangent) * sense))
        })
        .expect("a circle edge");
    let w = rim - frame.origin;
    let mut u = w.dot(frame.axis.cross(frame.e1)).atan2(w.dot(frame.e1));
    let mut shift = 0.0;
    match frame.kind {
        // A sphere's circle edge may be a meridian bounding a lune.
        Kind::Sphere { .. } => {
            let along = frame.axis.cross(frame.radial(u));
            if side.dot(along).abs() > 0.5 * side.norm() {
                u += core::f64::consts::FRAC_PI_2 * side.dot(along).signum();
            }
        }
        Kind::Torus { major, .. } => {
            let radial = frame.radial(u);
            let v = w.dot(frame.axis).atan2(w.dot(radial) - major);
            let along = frame.axis * v.cos() - radial * v.sin();
            shift = v + core::f64::consts::FRAC_PI_2 * side.dot(along).signum();
        }
        Kind::Cylinder { .. } | Kind::Cone { .. } => {}
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
            shift,
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

fn is_torus(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Torus { .. })
}

/// **A window planted on `face`**, at `T`: the chart rectangle azimuth
/// `us` × meridian `ss` of the `f64` frame `frame`, its two parallels'
/// arcs and its two meridians, a plug face filling it and its loop a ring
/// of `face`. Grown off `face`'s outer loop wherever the rectangle lies:
/// the Euler doors read no geometry. Returns the plug.
fn plant<T: Scalar>(
    body: &mut Body<T>,
    face: FaceKey,
    frame: Frame,
    us: (f64, f64),
    ss: (f64, f64),
) -> FaceKey {
    plant_wound(body, face, frame, us, ss, false)
}

/// [`plant`], the window's corners walked the other way round when
/// `as_outer`: the ring left on `face` is then wound as an outer loop, so
/// it bounds the window's rectangle on the face's side rather than off it.
fn plant_wound<T: Scalar>(
    body: &mut Body<T>,
    face: FaceKey,
    frame: Frame,
    us: (f64, f64),
    ss: (f64, f64),
    as_outer: bool,
) -> FaceKey {
    let f = body.get_face(face).unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the outer loop is a cycle")
    };
    let corners = if as_outer {
        [(us.1, ss.0), (us.0, ss.0), (us.0, ss.1), (us.1, ss.1)]
    } else {
        [(us.0, ss.0), (us.1, ss.0), (us.1, ss.1), (us.0, ss.1)]
    };
    let p = corners.map(|(u, s)| pt::<T>(frame.at(u, s)));
    // The four sides: from corner 0 to 1, 1 to 2 and 2 to 3 in walk order,
    // and the closing chord's curve from corner 0 to 3 (each curve starts
    // where its edge does).
    let sides = if as_outer {
        [
            frame.arc(ss.0, us, false),
            frame.meridian(us.0, ss),
            frame.arc(ss.1, us, true),
            frame.meridian(us.1, ss),
        ]
    } else {
        [
            frame.arc(ss.0, us, true),
            frame.meridian(us.1, ss),
            frame.arc(ss.1, us, false),
            frame.meridian(us.0, ss),
        ]
    };
    let [s01, s12, s23, s30] = sides;
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
    let a = body.mev(fan(strut.he_minus), p[1], s01, tol()).unwrap();
    let b = body.mev(fan(a.he_minus), p[2], s12, tol()).unwrap();
    let c = body.mev(fan(b.he_minus), p[3], s23, tol()).unwrap();
    let close = body
        .mef(
            MefSite::Chords {
                he1: a.he_plus,
                he2: c.he_minus,
            },
            s30,
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    for edge in [a.edge, b.edge, c.edge, close.edge] {
        body.describe_at_rest(edge, tol()).unwrap();
    }
    topo::mint_pcurves(body, tol()).unwrap();
    assert!(
        !body.get_face(face).unwrap().rings.is_empty(),
        "the window is a ring of the wall"
    );
    close.face
}

/// `face` of the `f64` body `at_f64`, in `body`: the face in the same arena
/// place, since both were built by the same doors in the same order.
fn same_face<T: Scalar>(at_f64: &Body<f64>, face: FaceKey, body: &Body<T>) -> FaceKey {
    let k = at_f64.faces().position(|(f, _)| f == face).unwrap();
    body.faces().nth(k).unwrap().0
}

/// The region arm's findings in `body`'s tier-3 report.
fn region_findings<T: Scalar>(body: &Body<T>) -> Vec<ValidationError> {
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

// ===================== review differential probe =====================
use geom_core::{Band, Interval};
use std::io::Write as _;

fn revolved_partial<T: Scalar>(pts: &[(f64, f64, f64)], angle: f64) -> Option<Body<T>> {
    let meridian = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b)))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![meridian]).validate(tol()).unwrap();
    revolve(
        &profile,
        RevolveAxis { origin: Point2::new(T::zero(), T::zero()), dir: Vec2::new(T::zero(), T::one()) },
        Revolution::Partial(T::from_f64(angle)),
        tol(),
    )
    .ok()
    .map(|r| r.body)
}

/// Project p onto the surface.
fn project(s: &Surface<f64>, p: Point3<f64>) -> Point3<f64> {
    match *s {
        Surface::Sphere { center, radius, .. } => {
            let w = p - center;
            center + w / w.norm() * radius
        }
        Surface::Torus { center, axis, major_radius, minor_radius, .. } => {
            let w = p - center;
            let h = w.dot(axis);
            let rad = w - axis * h;
            let c = center + rad / rad.norm() * major_radius;
            let d = p - c;
            c + d / d.norm() * minor_radius
        }
        Surface::Cylinder { origin, axis, radius, .. } => {
            let w = p - origin;
            let h = w.dot(axis);
            let rad = w - axis * h;
            origin + axis * h + rad / rad.norm() * radius
        }
        Surface::Cone { apex, axis, half_angle, .. } => {
            let w = p - apex;
            let h = w.dot(axis);
            let rad = w - axis * h;
            let rn = rad.norm();
            if rn == 0.0 { return p; }
            let gdir = (axis * half_angle.cos() + rad / rn * half_angle.sin());
            apex + gdir * w.dot(gdir).max(0.0)
        }
        _ => p,
    }
}

fn queries(body: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let s = body.get_surface(body.get_face(face).unwrap().surface).unwrap().clone();
    let fr = Frame::of(&s);
    let tau = core::f64::consts::TAU;
    let pi = core::f64::consts::PI;
    let mut out = Vec::new();
    let nu = 16;
    let (s_lo, s_hi, ns): (f64, f64, usize) = match fr.kind {
        Kind::Sphere { .. } => (0.0, pi, 9),
        Kind::Torus { .. } => (-pi, pi, 12),
        Kind::Cylinder { .. } => (-1.5, 1.5, 7),
        Kind::Cone { .. } => (0.0, 3.5, 8),
    };
    for i in 0..nu {
        let u = tau * (i as f64 + 0.37) / nu as f64;
        for j in 0..=ns {
            let sv = s_lo + (s_hi - s_lo) * j as f64 / ns as f64;
            out.push(project(&s, fr.at(u, sv)));
        }
    }
    // poles / apex and near them
    match fr.kind {
        Kind::Sphere { .. } | Kind::Cone { .. } => {
            let ends: &[f64] = if matches!(fr.kind, Kind::Sphere { .. }) { &[0.0, pi] } else { &[0.0] };
            for &e in ends {
                out.push(fr.at(0.0, e));
                for d in [1e-2, 1e-4, 1e-6, 1e-8, 1e-10, 1e-12] {
                    for k in 0..6 {
                        let u = tau * (k as f64 + 0.11) / 6.0;
                        let sv = if e == 0.0 { e + d } else { e - d };
                        out.push(project(&s, fr.at(u, sv)));
                    }
                }
            }
        }
        _ => {}
    }
    // near every edge, both sides, and vertices and their antipodes
    let LoopBoundary::Cycle { .. } = body.get_loop(body.get_face(face).unwrap().outer).unwrap().boundary else { return out };
    let f = body.get_face(face).unwrap();
    let mut loops = vec![f.outer];
    loops.extend(f.rings.iter().copied());
    for l in loops {
        let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else { continue };
        for h in body.loop_cycle(first).unwrap() {
            let half = body.get_half_edge(h).unwrap();
            let vp = *body.get_point(body.get_vertex(half.start).unwrap().point).unwrap();
            out.push(vp);
            if let Surface::Sphere { center, .. } = s {
                out.push(center + (center - vp));
            }
            if let Surface::Torus { center, axis, .. } = s {
                // the point turned a half turn about the axis
                let w = vp - center;
                let hh = axis * w.dot(axis);
                out.push(center + hh - (w - hh));
            }
            let edge = body.get_edge(half.edge).unwrap();
            let Some(curve) = body.get_curve_geom(edge.curve).and_then(|c| c.certified()) else { continue };
            let (t0, t1) = curve.params();
            for frac in [0.0, 1e-9, 0.25, 0.5, 0.75, 1.0 - 1e-9, 1.0] {
                let t = t0 + (t1 - t0) * frac;
                let at = curve.carrier().eval(t);
                let tan = curve.carrier().deriv(t);
                let nrm = match s {
                    Surface::Sphere { center, .. } => at - center,
                    _ => {
                        // finite difference normal: use projection of offset
                        let q = project(&s, at + Vec3::new(1e-3, 2e-3, 3e-3));
                        let r = project(&s, at + Vec3::new(-2e-3, 1e-3, -1e-3));
                        (q - at).cross(r - at)
                    }
                };
                let side = nrm.cross(tan);
                let sn = side.norm();
                if !(sn > 0.0) { continue; }
                let side = side / sn;
                out.push(at);
                for d in [1e-1, 1e-3, 1e-5, 1e-7, 3e-9, 1e-10, 1e-11, 1e-13] {
                    out.push(project(&s, at + side * d));
                    out.push(project(&s, at - side * d));
                }
                if let Surface::Sphere { center, .. } = s {
                    out.push(center + (center - at));
                }
            }
        }
    }
    out
}

fn verdict<E: core::fmt::Debug>(r: &Result<Option<topo::boolean::FaceContainment>, E>) -> String {
    match r {
        Ok(Some(c)) => format!("{c:?}"),
        Ok(None) => "None".into(),
        Err(e) => {
            let d = format!("{e:?}");
            let head: String = d.chars().take_while(|c| *c != '{' && *c != '(').collect();
            let pred = d.find("predicate: Some(\"").map(|i| {
                let rest = &d[i + 17..];
                rest[..rest.find('"').unwrap_or(0)].to_string()
            }).unwrap_or_default();
            format!("ERR:{}:{}", head.trim(), pred)
        }
    }
}

fn run_body<T: Scalar>(
    name: &str,
    b64: &Body<f64>,
    bt: &Body<T>,
    scalar: &str,
    w: &mut impl std::io::Write,
) {
    for (fi, (face, _)) in b64.faces().enumerate() {
        let surf = b64.get_surface(b64.get_face(face).unwrap().surface).unwrap();
        if matches!(surf, Surface::Plane { .. }) { continue; }
        let ft = same_face(b64, face, bt);
        let qs = queries(b64, face);
        for (qi, q) in qs.iter().enumerate() {
            for eps in [1e-6, 1e-9, 1e-12] {
                let band = Band::new(eps, 10.0 * eps).unwrap();
                let r = topo::boolean::curved_face_containment(bt, ft, pt::<T>(*q), band);
                writeln!(w, "{name}|f{fi}|q{qi}|{scalar}|{eps:e}|{}", verdict(&r)).unwrap();
            }
        }
    }
}

fn build_all<T: Scalar>() -> Vec<(String, Option<Body<T>>)> {
    let pi = core::f64::consts::PI;
    let disc = [(0.0, -1.0, 1.0), (0.0, 1.0, 0.0)];
    let ring = [(1.0, 0.0, 1.0), (3.0, 0.0, 1.0)];
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let mut v: Vec<(String, Option<Body<T>>)> = Vec::new();
    v.push(("ball".into(), Some(revolved::<T>(&disc))));
    v.push(("donut".into(), Some(revolved::<T>(&ring))));
    v.push(("cyl".into(), Some(revolved::<T>(&[(0.0, -1.0, 0.0), (1.0, -1.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)]))));
    v.push(("cone".into(), Some(revolved::<T>(&[(0.0, 0.0, 0.0), (2.0 * s, 2.0 * c, 0.0), (0.0, 2.0 * c, 0.0)]))));
    v.push(("coneband".into(), Some(revolved::<T>(&[(s, c, 0.0), (2.0 * s, 2.0 * c, 0.0), (0.0, 2.0 * c, 0.0), (0.0, c, 0.0)]))));
    for deg in [1.0f64, 90.0, 180.0, 270.0, 359.0] {
        let a = deg.to_radians();
        v.push((format!("lune{deg}"), revolved_partial::<T>(&disc, a)));
        v.push((format!("donutp{deg}"), revolved_partial::<T>(&ring, a)));
        v.push((format!("conep{deg}"), revolved_partial::<T>(&[(0.0, 0.0, 0.0), (2.0 * s, 2.0 * c, 0.0), (0.0, 2.0 * c, 0.0)], a)));
        v.push((format!("cylp{deg}"), revolved_partial::<T>(&[(0.0, -1.0, 0.0), (1.0, -1.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0)], a)));
    }
    v
}

/// Bodies with windows, built from the f64 frame.
fn windowed<T: Scalar>() -> Vec<(String, Body<T>, Body<f64>)> {
    let pi = core::f64::consts::PI;
    let mut v = Vec::new();
    let specs: Vec<(&str, fn(&Surface<f64>) -> bool, Vec<((f64, f64), (f64, f64))>)> = vec![
        ("ballwin", is_sphere, vec![((-0.5, 0.5), (1.0, 2.0))]),
        ("ballwin_antip", is_sphere, vec![((-0.5, 0.5), (1.0, 2.0)), ((pi - 0.5, pi + 0.5), (1.0, 2.0))]),
        ("ballwin_nearpole", is_sphere, vec![((-0.5, 0.5), (1e-3, 0.5))]),
        ("ballwin_nearpole_b", is_sphere, vec![((-0.5, 0.5), (3e-2, 0.5))]),
        ("ballwin_nearpole_c", is_sphere, vec![((-0.5, 0.5), (0.1, 0.5)), ((pi - 0.5, pi + 0.5), (pi - 0.5, pi - 0.1))]),
        ("ballwin_nearpole2", is_sphere, vec![((-0.5, 0.5), (1e-5, 0.3)), ((pi - 0.5, pi + 0.5), (pi - 0.3, pi - 1e-5))]),
        ("donutwin_antip", is_torus, vec![((-1.2, -0.2), (-0.4, 0.4)), ((pi - 1.2, pi - 0.2), (-0.4, 0.4))]),
        ("donutwin_antip_tube", is_torus, vec![((-0.4, 0.4), (-1.2, -0.2)), ((pi - 0.4, pi + 0.4), (pi - 1.2, pi - 0.2))]),
        ("donutwin", is_torus, vec![((-0.4, 0.4), (-0.5, 0.5))]),
    ];
    for (name, kind, wins) in specs {
        let base64: Body<f64> = if name.starts_with("ball") {
            revolved_disc::<f64>()
        } else {
            donut::<f64>()
        };
        let (face64, frame) = wall(&base64, kind);
        let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut b64 = base64.clone();
            let mut bt: Body<T> = if name.starts_with("ball") { revolved_disc::<T>() } else { donut::<T>() };
            let facet = same_face(&base64, face64, &bt);
            for &(us, ss) in &wins {
                plant(&mut b64, face64, frame, us, ss);
                plant(&mut bt, facet, frame, us, ss);
            }
            (bt, b64)
        }));
        if let Ok((bt, b64)) = built {
            v.push((name.to_string(), bt, b64));
        } else {
            eprintln!("SKIP windowed {name}");
        }
    }
    v
}

#[test]
fn review_truth_dump() {
    let path = std::env::var("REVIEW_TRUTH").unwrap();
    let mut w = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let pi = core::f64::consts::PI;
    let tau = core::f64::consts::TAU;
    let specs: Vec<(&str, Vec<((f64, f64), (f64, f64))>)> = vec![
        ("ballwin_antip", vec![((-0.5, 0.5), (1.0, 2.0)), ((pi - 0.5, pi + 0.5), (1.0, 2.0))]),
        ("ballwin_nearpole_c", vec![((-0.5, 0.5), (0.1, 0.5)), ((pi - 0.5, pi + 0.5), (pi - 0.5, pi - 0.1))]),
    ];
    for (name, wins) in specs {
        let base64 = revolved_disc::<f64>();
        let (face64, frame) = wall(&base64, is_sphere);
        let mut b = base64.clone();
        for &(us, ss) in &wins { plant(&mut b, face64, frame, us, ss); }
        let nfaces = b.faces().count();
        if name == "ballwin_antip" {
            for (fi, (face, f)) in b.faces().enumerate() {
                writeln!(w, "FACE f{fi} sense={} surf={:?}", f.sense, b.get_surface(f.surface)).unwrap();
                let mut loops = vec![f.outer];
                loops.extend(f.rings.iter().copied());
                for l in loops {
                    let LoopBoundary::Cycle { first } = b.get_loop(l).unwrap().boundary else { writeln!(w, "  loop {l:?} non-cycle").unwrap(); continue };
                    writeln!(w, "  loop {l:?}").unwrap();
                    for h in b.loop_cycle(first).unwrap() {
                        let half = b.get_half_edge(h).unwrap();
                        let edge = b.get_edge(half.edge).unwrap();
                        let vp = *b.get_point(b.get_vertex(half.start).unwrap().point).unwrap();
                        let c = b.get_curve_geom(edge.curve).and_then(|c| c.certified());
                        let desc = c.map(|c| format!("{:?} params {:?}", c.carrier(), c.params())).unwrap_or("uncertified".into());
                        writeln!(w, "    he start ({:.4},{:.4},{:.4}) plus={} {desc}", vp.x, vp.y, vp.z, h == edge.he_plus).unwrap();
                    }
                }
            }
        }
        for (fi, (face, _)) in b.faces().enumerate() {
            let surf = b.get_surface(b.get_face(face).unwrap().surface).unwrap();
            if matches!(surf, Surface::Plane { .. }) { continue; }
            let qs = queries(&b, face);
            for (qi, q) in qs.iter().enumerate() {
                let wv = *q - frame.origin;
                let h = wv.dot(frame.axis);
                let sv = (-h).clamp(-1.0, 1.0).acos();
                let e2 = frame.axis.cross(frame.e1);
                let u = wv.dot(e2).atan2(wv.dot(frame.e1));
                let inwin = wins.iter().position(|&((u0, u1), (s0, s1))| {
                    let mid = 0.5 * (u0 + u1);
                    let du = (u - mid + pi).rem_euclid(tau) - pi;
                    du.abs() < 0.5 * (u1 - u0) && sv > s0 && sv < s1
                });
                writeln!(w, "{name}|f{fi}|q{qi}|face64={}|nf={nfaces}|u={u:.6}|s={sv:.6}|win={inwin:?}|p=({:.6},{:.6},{:.6})", face == face64, q.x, q.y, q.z).unwrap();
            }
        }
    }
}

#[test]
fn review_differential_probe() {
    let path = std::env::var("REVIEW_OUT").unwrap();
    let mut w = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let b64 = build_all::<f64>();
    let bi = build_all::<Interval>();
    for ((n, a), (_, b)) in b64.iter().zip(bi.iter()) {
        let (Some(a), Some(b)) = (a, b) else { writeln!(w, "{n}|SKIP").unwrap(); continue };
        run_body(n, a, a, "f64", &mut w);
        run_body(n, a, b, "Interval", &mut w);
    }
    let w64 = windowed::<f64>();
    let wi = windowed::<Interval>();
    for (n, a, f) in w64.iter() {
        let Some((_, b, _)) = wi.iter().find(|x| &x.0 == n) else { writeln!(w, "{n}|SKIPI").unwrap(); continue };
        run_body(n, f, a, "f64", &mut w);
        run_body(n, f, b, "Interval", &mut w);
    }
}

#[test]
fn review_point_detail() {
    // REVIEW_DETAIL="lune90:2:249,378;lune270:2:250,377"
    let spec = std::env::var("REVIEW_DETAIL").unwrap();
    let all = build_all::<f64>();
    for item in spec.split(';') {
        let mut it = item.split(':');
        let (name, fi, qs) = (it.next().unwrap(), it.next().unwrap().parse::<usize>().unwrap(), it.next().unwrap());
        let eps: f64 = it.next().map(|e| e.parse().unwrap()).unwrap_or(1e-12);
        let b = all.iter().find(|x| x.0 == name).unwrap().1.as_ref().unwrap();
        let (face, f) = b.faces().nth(fi).unwrap();
        println!("{name} f{fi} surf {:?} sense {}", b.get_surface(f.surface), f.sense);
        let LoopBoundary::Cycle { first } = b.get_loop(f.outer).unwrap().boundary else { panic!() };
        for h in b.loop_cycle(first).unwrap() {
            let half = b.get_half_edge(h).unwrap();
            let edge = b.get_edge(half.edge).unwrap();
            let vp = *b.get_point(b.get_vertex(half.start).unwrap().point).unwrap();
            let c = b.get_curve_geom(edge.curve).and_then(|c| c.certified());
            println!("  he start {:?} plus={} {:?}", vp, h == edge.he_plus, c.map(|c| (c.carrier().clone(), c.params())));
        }
        let q = queries(b, face);
        for qi in qs.split(',').map(|x| x.parse::<usize>().unwrap()) {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let r = topo::boolean::curved_face_containment(b, face, q[qi], band);
            println!("  q{qi} = {:?}\n    -> {:?}", q[qi], r);
        }
    }
}
