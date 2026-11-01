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
    wall_n(body, kind, 0)
}
fn wall_n(body: &Body<f64>, kind: fn(&Surface<f64>) -> bool, n: usize) -> (FaceKey, Frame) {
    let (face, frame) = body
        .faces()
        .filter_map(|(k, f)| {
            let s = body.get_surface(f.surface)?;
            kind(s).then(|| (k, Frame::of(s)))
        })
        .nth(n)
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
    let f = body.get_face(face).unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("the outer loop is a cycle")
    };
    let p = [
        frame.at(us.0, ss.0),
        frame.at(us.1, ss.0),
        frame.at(us.1, ss.1),
        frame.at(us.0, ss.1),
    ]
    .map(pt::<T>);
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

/// The honest body passes every tier-3 check; the misplaced one is
/// refused by the region arm on `face`, and by nothing else.
fn honest_then_misplaced<T: Scalar>(
    name: &str,
    honest: &Body<T>,
    misplaced: &Body<T>,
    face: FaceKey,
) {
    assert_eq!(
        topo::validate_geometric_structural(honest, tol()),
        Ok(()),
        "[{name}] the window in the face validates"
    );
    let errors = topo::validate_geometric_structural(misplaced, tol()).unwrap_err();
    assert!(
        errors.iter().all(|e| matches!(
            e,
            ValidationError::CycleOffFaceSide { face: f, .. } if *f == face
        )),
        "[{name}] only the region arm refuses, on the wall: {errors:?}"
    );
    assert!(
        !errors.is_empty(),
        "[{name}] the misplaced window is refused"
    );
}


// ===================== review probes (delta 6163) =====================

fn revolved_by<T: Scalar>(pts: &[(f64, f64, f64)], rev: Revolution<T>) -> Option<Body<T>> {
    let meridian = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b)))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![meridian]).validate(tol()).ok()?;
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(T::zero(), T::zero()),
            dir: Vec2::new(T::zero(), T::one()),
        },
        rev,
        tol(),
    )
    .ok()
    .map(|b| b.body)
}

fn short<E: core::fmt::Debug>(r: &Result<Option<topo::FaceContainment>, E>) -> String {
    match r {
        Ok(Some(topo::FaceContainment::In)) => "In".into(),
        Ok(Some(topo::FaceContainment::Out)) => "Out".into(),
        Ok(None) => "None".into(),
        Ok(Some(o)) => format!("{o:?}"),
        Err(e) => {
            let s = format!("{e:?}");
            let cut = s.find(|c: char| c == ' ' || c == '{' || c == '(').unwrap_or(s.len());
            let mut out = format!("E:{}", &s[..cut]);
            if let Some(i) = s.find("name: \"") {
                let rest = &s[i + 7..];
                out.push(':');
                out.push_str(&rest[..rest.find('"').unwrap_or(0)]);
            }
            out
        }
    }
}

type Win = ((f64, f64), (f64, f64));

/// Every torus face of `at_f64` (built at f64) and the same bodies at T,
/// with windows planted on torus face `nth`, then dump containment over a grid.
fn dump<T: Scalar>(name: &str, at_f64: &Body<f64>, body_t: &Body<T>, nth: usize, wins: &[Win]) {
    let band = geom_core::Band::linear(tol()).unwrap();
    let (face64, frame) = wall_n(at_f64, is_torus, nth);
    let face = same_face(at_f64, face64, body_t);
    let mut body = body_t.clone();
    for &(us, ss) in wins {
        plant(&mut body, face, frame, us, ss);
    }
    let tori: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface).map(|s| s.clone()), Some(_)))
        .filter(|(_, f)| {
            let s = format!("{:?}", body.get_surface(f.surface).unwrap());
            s.starts_with("Torus")
        })
        .map(|(k, _)| k)
        .collect();
    let n = 36;
    let mut counts = std::collections::BTreeMap::new();
    for (fi, &f) in tori.iter().enumerate() {
        for i in 0..n {
            for j in 0..n {
                let u = -core::f64::consts::PI + (i as f64 + 0.37) * 2.0 * core::f64::consts::PI / n as f64;
                let s = -core::f64::consts::PI + (j as f64 + 0.61) * 2.0 * core::f64::consts::PI / n as f64;
                let q = pt::<T>(frame.at(u, s));
                let r = topo::curved_face_containment(&body, f, q, band);
                let k = short(&r);
                *counts.entry(k.clone()).or_insert(0) += 1;
                println!("D2|{name}|{}|f{fi}|{i}|{j}|{k}", core::any::type_name::<T>());
            }
        }
        // points near window edges
        for &(us, ss) in wins {
            for du in [-0.01, 0.0, 0.003, 0.5, 1.0, 1.02] {
                for (sk, s) in [(0, ss.0 + 0.01), (1, 0.5 * (ss.0 + ss.1)), (2, ss.1 - 0.01), (3, ss.1 + 0.02), (4, ss.0 - 0.05), (5, ss.1 + 0.05)] {
                    let u = us.0 + du * (us.1 - us.0);
                    let q = pt::<T>(frame.at(u, s));
                    let r = topo::curved_face_containment(&body, f, q, band);
                    let k = short(&r);
                    *counts.entry(k.clone()).or_insert(0) += 1;
                    println!("D2|{name}|{}|f{fi}|w{du}|{sk}|{k}", core::any::type_name::<T>());
                }
            }
        }
    }
    println!("D2SUM|{name}|{}|{counts:?}", core::any::type_name::<T>());
}

fn dump_both(name: &str, pts: &[(f64, f64, f64)], rev: Option<f64>, nth: usize, wins: &[Win]) {
    let mk64 = || match rev {
        None => revolved_by::<f64>(pts, Revolution::Full),
        Some(a) => revolved_by::<f64>(pts, Revolution::Partial(a)),
    };
    let mki = || match rev {
        None => revolved_by::<geom_core::Interval>(pts, Revolution::Full),
        Some(a) => revolved_by::<geom_core::Interval>(pts, Revolution::Partial(geom_core::Interval::from_f64(a))),
    };
    let (Some(b64), Some(bi)) = (mk64(), mki()) else {
        println!("D2SKIP|{name}");
        return;
    };
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        dump::<f64>(name, &b64, &b64, nth, wins);
        dump::<geom_core::Interval>(name, &b64, &bi, nth, wins);
    }));
    if ok.is_err() {
        println!("D2PANIC|{name}");
    }
}

#[test]
fn review_d2_torus_door_grid() {
    let donut = [(1.0, 0.0, 1.0), (3.0, 0.0, 1.0)];
    let thin = [(1.8, 0.0, 1.0), (2.2, 0.0, 1.0)];
    let fat = [(0.2, 0.0, 1.0), (2.2, 0.0, 1.0)];
    let high = [(1.0, 0.7, 1.0), (3.0, 0.7, 1.0)];
    let halfdisc = [(1.0, 0.0, 1.0), (3.0, 0.0, 0.0)];
    let p = core::f64::consts::PI;
    let probe = |shift: f64| (1.0 - shift, 2.0 - shift);
    let _ = probe;
    let cases: Vec<(&str, &[(f64, f64, f64)], Option<f64>, usize, Vec<Win>)> = vec![
        ("donut", &donut, None, 0, vec![]),
        ("donut_in", &donut, None, 1, vec![]),
        ("thin", &thin, None, 0, vec![]),
        ("fat", &fat, None, 0, vec![]),
        ("high", &high, None, 1, vec![]),
        ("halfdisc", &halfdisc, None, 0, vec![]),
        ("partial4", &donut, Some(4.0), 0, vec![]),
        ("partial1.5", &donut, Some(1.5), 0, vec![]),
        ("halfdisc_partial", &halfdisc, Some(2.5), 0, vec![]),
        ("win1", &donut, None, 0, vec![((-0.3, 0.3), (-0.4, 0.4))]),
        ("win2", &donut, None, 0, vec![((-0.5, -0.1), (-0.5, 0.5)), ((0.1, 0.5), (-0.5, 0.5))]),
        ("win2_in", &donut, None, 1, vec![((-0.5, -0.1), (-0.5, 0.5)), ((0.1, 0.5), (-0.5, 0.5))]),
        ("winnest", &donut, None, 0, vec![((-0.5, 0.5), (-0.5, 0.5)), ((-0.2, 0.2), (-0.2, 0.2))]),
        ("winoff", &donut, None, 0, vec![((-0.3, 0.3), (p - 0.4, p + 0.4))]),
        ("win_edge", &donut, None, 0, vec![((-0.5, 0.5), (1.2, 1.5)), ((-0.5, 0.5), (-1.5, -1.2))]),
        ("win_partial", &donut, Some(4.0), 0, vec![((0.2, 0.6), (-0.5, 0.5))]),
        ("stacked", &donut, None, 0, vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.5, 0.5), (0.1, 1.0))]),
        ("stacked_in", &donut, None, 1, vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.5, 0.5), (0.1, 1.0))]),
    ];
    for (name, pts, rev, nth, wins) in cases {
        dump_both(name, pts, rev, nth, &wins);
    }
}

/// D1/D3: tier-3 findings over window configurations.
fn findings<T: Scalar>(name: &str, at_f64: &Body<f64>, body_t: &Body<T>, nth: usize, wins: &[Win]) {
    let (face64, frame) = wall_n(at_f64, is_torus, nth);
    let face = same_face(at_f64, face64, body_t);
    let mut body = body_t.clone();
    for &(us, ss) in wins {
        plant(&mut body, face, frame, us, ss);
    }
    let all = topo::validate_geometric_structural(&body, tol());
    let summary: Vec<String> = match &all {
        Ok(()) => vec!["OK".into()],
        Err(es) => es
            .iter()
            .map(|e| {
                let s = format!("{e:?}");
                let cut = s.find(|c: char| c == ' ' || c == '{' || c == '(').unwrap_or(s.len());
                let mut out = s[..cut].to_string();
                if let Some(i) = s.find("name: \"") {
                    let rest = &s[i + 7..];
                    out.push(':');
                    out.push_str(&rest[..rest.find('"').unwrap_or(0)]);
                }
                out
            })
            .collect(),
    };
    println!("D1|eps={}|{name}|{}|shift={:.3}|{summary:?}", tol().eps(), core::any::type_name::<T>(), frame.shift);
}

#[test]
fn review_d1_windows() {
    let p = core::f64::consts::PI;
    let d64 = donut::<f64>();
    let di = donut::<geom_core::Interval>();
    let cases: Vec<(&str, usize, Vec<Win>)> = vec![
        ("probe_apart", 0, vec![((-0.5, -0.1), (0.5, 1.5)), ((0.1, 0.5), (0.5, 1.5))]),
        ("close", 0, vec![((-0.5, -0.001), (-0.5, 0.5)), ((0.001, 0.5), (-0.5, 0.5))]),
        ("close_1e4", 0, vec![((-0.5, -0.0001), (-0.5, 0.5)), ((0.0001, 0.5), (-0.5, 0.5))]),
        ("near_rim", 0, vec![((-0.5, -0.1), (1.0, 1.55)), ((0.1, 0.5), (1.0, 1.55))]),
        ("near_rim2", 0, vec![((-0.5, -0.1), (-1.565, -1.0)), ((0.1, 0.5), (1.0, 1.565))]),
        ("stacked_v", 0, vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.5, 0.5), (0.1, 1.0))]),
        ("three", 0, vec![((-0.9, -0.5), (-0.5, 0.5)), ((-0.3, 0.3), (-0.5, 0.5)), ((0.5, 0.9), (-0.5, 0.5))]),
        ("far_apart", 0, vec![((-0.3, 0.3), (-0.5, 0.5)), ((2.8, 3.4), (-0.5, 0.5))]),
        ("inner_apart", 1, vec![((-0.5, -0.1), (-0.5, 0.5)), ((0.1, 0.5), (-0.5, 0.5))]),
        ("inner_one", 1, vec![((-0.3, 0.3), (-0.4, 0.4))]),
        // misplaced (D3)
        ("BAD_off", 0, vec![((-0.3, 0.3), (p - 0.4, p + 0.4))]),
        ("BAD_off_beside", 0, vec![((-0.5, -0.1), (-0.5, 0.5)), ((0.1, 0.5), (p - 0.5, p + 0.5))]),
        ("BAD_nest", 0, vec![((-0.5, 0.5), (-0.5, 0.5)), ((-0.2, 0.2), (-0.2, 0.2))]),
        ("BAD_nest_rim", 0, vec![((-0.5, 0.5), (1.0, 1.55)), ((-0.2, 0.2), (1.2, 1.4))]),
        ("BAD_nest_inner", 1, vec![((-0.5, 0.5), (-0.5, 0.5)), ((-0.2, 0.2), (-0.2, 0.2))]),
        ("BAD_off_inner", 1, vec![((-0.3, 0.3), (p - 0.4, p + 0.4))]),
        ("BAD_three_nest", 0, vec![((-0.9, 0.9), (-0.9, 0.9)), ((-0.5, 0.5), (-0.5, 0.5)), ((-0.2, 0.2), (-0.2, 0.2))]),
    ];
    for (name, nth, wins) in cases {
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            findings::<f64>(name, &d64, &d64, nth, &wins);
            findings::<geom_core::Interval>(name, &d64, &di, nth, &wins);
        }));
        if r.is_err() {
            println!("D1PANIC|{name}");
        }
    }
}

#[test]
fn review_d1_stacked_detail() {
    let d64 = donut::<f64>();
    let di = donut::<geom_core::Interval>();
    let band = geom_core::Band::linear(tol()).unwrap();
    for (name, wins) in [
        ("stacked", vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.5, 0.5), (0.1, 1.0))]),
        ("stacked_narrow", vec![((-0.1, 0.1), (-1.0, -0.1)), ((-0.1, 0.1), (0.1, 1.0))]),
        ("stacked_offset", vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.4, 0.6), (0.1, 1.0))]),
        ("stacked_small", vec![((-0.5, 0.5), (-0.5, -0.1)), ((-0.5, 0.5), (0.1, 0.5))]),
        ("diag", vec![((-0.5, -0.1), (-1.0, -0.1)), ((0.1, 0.5), (0.1, 1.0))]),
    ] {
        let (face64, frame) = wall_n(&d64, is_torus, 0);
        let face = same_face(&d64, face64, &di);
        let mut body = di.clone();
        let mut body64 = d64.clone();
        for &(us, ss) in &wins {
            plant(&mut body, face, frame, us, ss);
            plant(&mut body64, face64, frame, us, ss);
        }
        println!("== {name} f64: {:?}", topo::validate_geometric_structural(&body64, tol()).map_err(|e| e.len()));
        match topo::validate_geometric_structural(&body, tol()) {
            Ok(()) => println!("== {name} interval OK"),
            Err(es) => {
                for e in es {
                    println!("== {name} interval ERR {e:?}");
                }
            }
        }
        // uncut door at each window corner (vertex) and window centre, on the wall
        for &(us, ss) in &wins {
            for (u, s) in [(us.0, ss.0), (us.1, ss.1), (0.5 * (us.0 + us.1), ss.0), (0.5*(us.0+us.1), 0.5*(ss.0+ss.1))] {
                let q = pt::<geom_core::Interval>(frame.at(u, s));
                println!("   uncut door at ({u},{s}): {}", short(&topo::curved_face_containment(&body, face, q, band)));
            }
        }
    }
}

#[test]
fn review_d1_azimuth_line() {
    let d64 = donut::<f64>();
    let di = donut::<geom_core::Interval>();
    let band = geom_core::Band::linear(tol()).unwrap();
    let (face64, frame) = wall_n(&d64, is_torus, 0);
    let face = same_face(&d64, face64, &di);
    let mut body = di.clone();
    plant(&mut body, face, frame, (-0.3, 0.3), (-0.4, 0.4));
    for (u, s) in [(-0.3, -0.45), (-0.3, -1.0), (-0.3, 0.6), (-0.3 + 1e-6, -1.0), (-0.3 + 1e-3, -1.0), (-0.31, -1.0), (0.0, -1.0)] {
        let q = pt::<geom_core::Interval>(frame.at(u, s));
        let r = topo::curved_face_containment(&body, face, q, band);
        let s2 = format!("{r:?}");
        let pred = s2.find("predicate").map(|i| &s2[i..(i + 50).min(s2.len())]).unwrap_or("");
        println!("AZ ({u},{s}) {} {pred}", short(&r));
    }
}
