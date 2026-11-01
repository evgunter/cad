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
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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

/// **P09, a cone band less a window outside it** (CONEARM's probe, the
/// cone of half-angle ½ at the apex): the band over slant `[1, 2]` with a
/// window over slant `[3, 3.3]` and an azimuth span of 1 (the probe's
/// `[1, 2]`, centred on the wall, [`wall`]). The honest twin's
/// window is over slant `[1.3, 1.7]`.
#[test]
fn a_cone_band_refuses_a_window_beyond_its_rim() {
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let band = revolved_polygon(&[(s, c), (2.0 * s, 2.0 * c), (0.0, 2.0 * c), (0.0, c)]);
    let (face, frame) = wall(&band, is_cone);
    let mut honest = band.clone();
    plant(&mut honest, face, frame, (-0.5, 0.5), (1.3, 1.7));
    let mut misplaced = band;
    plant(&mut misplaced, face, frame, (-0.5, 0.5), (3.0, 3.3));
    honest_then_misplaced("P09", &honest, &misplaced, face);
}

/// **P47, a cone cap with a window outside it**: the cap from the apex
/// to the rim at slant 2, with a window over slant `[3, 3.5]` and an
/// azimuth span of 1. The honest twin's is over slant `[1, 1.5]`.
#[test]
fn a_cone_cap_refuses_a_window_beyond_its_rim() {
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let cap = revolved_polygon(&[(0.0, 0.0), (2.0 * s, 2.0 * c), (0.0, 2.0 * c)]);
    let (face, frame) = wall(&cap, is_cone);
    let mut honest = cap.clone();
    plant(&mut honest, face, frame, (-0.5, 0.5), (1.0, 1.5));
    let mut misplaced = cap;
    plant(&mut misplaced, face, frame, (-0.5, 0.5), (3.0, 3.5));
    honest_then_misplaced("P47", &honest, &misplaced, face);
}

/// **P49, a window face holding a hole outside it**: on the band over
/// slant `[0.5, 2.5]`, the plug of the window slant `[1, 2]` × azimuth
/// `[-0.5, 0.5]` (centred on the wall) holds a hole over slant
/// `[1.2, 1.4]` and azimuth `[1, 1.5]`, beside it as the probe's `[3, 3.5]`
/// is beside `[1, 2]`. The honest twin's hole is over azimuth `[-0.3, 0]`.
#[test]
fn a_cone_window_refuses_a_hole_beside_it() {
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let mut band = revolved_polygon(&[
        (0.5 * s, 0.5 * c),
        (2.5 * s, 2.5 * c),
        (0.0, 2.5 * c),
        (0.0, 0.5 * c),
    ]);
    let (wall_face, frame) = wall(&band, is_cone);
    let window = plant(&mut band, wall_face, frame, (-0.5, 0.5), (1.0, 2.0));
    let mut honest = band.clone();
    plant(&mut honest, window, frame, (-0.3, 0.0), (1.2, 1.4));
    let mut misplaced = band;
    plant(&mut misplaced, window, frame, (1.0, 1.5), (1.2, 1.4));
    honest_then_misplaced("P49", &honest, &misplaced, window);
}

/// **A ring inside another ring on a cylinder**: two windows on a
/// cylinder's wall, the second inside the first. Each alone validates.
#[test]
fn a_cylinder_wall_refuses_a_ring_inside_another() {
    let tube = revolved_polygon(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, is_cylinder);
    let mut honest = tube.clone();
    plant(&mut honest, face, frame, (-0.9, -0.1), (0.5, 1.5));
    plant(&mut honest, face, frame, (0.1, 0.9), (0.5, 1.5));
    let mut misplaced = tube;
    plant(&mut misplaced, face, frame, (-0.9, -0.1), (0.5, 1.5));
    plant(&mut misplaced, face, frame, (-0.7, -0.3), (0.8, 1.2));
    honest_then_misplaced("cylinder rings", &honest, &misplaced, face);
    assert_eq!(
        region_findings(&misplaced).len(),
        1,
        "one ring lies in the other, and the other outside the first"
    );
}

/// **A ring inside another ring on a sphere**: two windows on a ball's
/// face, the second inside the first. Two windows apart validate.
#[test]
fn a_sphere_face_refuses_a_ring_inside_another() {
    let ball = revolved_disc::<f64>();
    let (face, frame) = wall(&ball, is_sphere);
    let mut honest = ball.clone();
    plant(&mut honest, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut honest, face, frame, (0.1, 0.9), (1.0, 2.0));
    let mut misplaced = ball;
    plant(&mut misplaced, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut misplaced, face, frame, (-0.7, -0.3), (1.3, 1.7));
    honest_then_misplaced("sphere rings", &honest, &misplaced, face);
}

/// **A torus face's essential pair**: each of the donut's two faces is
/// the band between two parallels, which wind the azimuth once each,
/// opposite ways (the lift's integers, `surface_cycle_windings`), and the donut
/// validates. The same face with one parallel's stored joint element
/// moved two periods reads two cycles wound the same way, which bound no
/// band, and the region arm names it; nothing in the face's geometry
/// changed, so the row pins that the arm reads the lift's integers.
#[test]
fn a_torus_face_reads_its_essential_cycles_as_one_pair() {
    let honest = donut::<f64>();
    assert_eq!(
        topo::validate_geometric_structural(&honest, tol()),
        Ok(()),
        "the donut validates"
    );
    let (face, cycles) = honest
        .faces()
        .find_map(|(face, f)| {
            matches!(honest.get_surface(f.surface), Some(Surface::Torus { .. })).then(|| {
                let cycles = topo::surface_boundary::surface_cycle_windings(&honest, face).unwrap();
                (face, cycles)
            })
        })
        .expect("a torus face");
    let windings: Vec<i32> = cycles
        .iter()
        .map(|(_, w)| w.expect("every joint is minted").deck.u)
        .collect();
    assert_eq!(windings.len(), 2, "two parallels bound the face");
    assert_eq!(windings[0], -windings[1], "wound opposite: {windings:?}");
    assert_ne!(windings[0], 0, "each winds the azimuth: {windings:?}");
    let mut misread = honest.clone();
    let h = cycles[0].0[0];
    let element = misread.joint(h).expect("a minted joint");
    let turned = topo::Deck {
        u: element.deck().u - 2 * windings[0],
        ..element.deck()
    };
    misread.attach_joint(h, topo::JointElement::Shift(turned));
    let errors = topo::validate_geometric_structural(&misread, tol()).unwrap_err();
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::EssentialCyclesUnpaired { face: f, essential: 2 } if *f == face
        )),
        "the region arm names the face: {errors:?}"
    );
}

/// **A cycle read in band escalates typed** (D4 ¶1): on a cylinder wall,
/// a second window whose every corner stands `3ε` off the first one's
/// corners along both chart directions, so each corner's distance to the
/// first window's boundary is in the sliver band (between ε and K·ε).
/// No vertex of either window is placed against the other, and the arm
/// refuses `CycleSideUndecided` carrying the margin, never a pass.
#[test]
fn a_cycle_read_in_band_escalates_with_its_margin() {
    let tube = revolved_polygon(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, is_cylinder);
    let d = 3.0 * tol().eps();
    let mut body = tube;
    plant(&mut body, face, frame, (-0.6, 0.6), (0.5, 1.5));
    plant(
        &mut body,
        face,
        frame,
        (-0.6 - d, 0.6 + d),
        (0.5 - d, 1.5 + d),
    );
    let errors = topo::validate_geometric_structural(&body, tol()).unwrap_err();
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::CycleSideUndecided { face: f, source, .. }
                if *f == face && !source.margin.is_invalid()
        )),
        "the region arm escalates on the wall: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::CycleOffFaceSide { .. })),
        "no side is read off a margin in band: {errors:?}"
    );
}

/// **A sliver beside a pole passes** (PR 6054's near-pole false refusal,
/// on a body): a window face 10 µm tall, 1 cm from the ball's south pole,
/// holds neither pole and lies in its lune, its plug and the lune both
/// read by the region arm and the chart-singularity arm, and the body
/// validates.
#[test]
fn a_sliver_beside_a_pole_passes() {
    let (height, eps) = (1e-5, tol().eps());
    if height <= 2.0 * tol().k() * eps {
        println!(
            "SKIPPED at ε {eps:e}: a {height:e} edge is within twice the band, and the window \
             does not build"
        );
        return;
    }
    let mut ball = revolved_disc::<f64>();
    let (face, frame) = wall(&ball, is_sphere);
    plant(&mut ball, face, frame, (-0.5, 0.5), (0.01, 0.01 + height));
    assert_eq!(
        topo::validate_geometric_structural(&ball, tol()),
        Ok(()),
        "the sliver beside the pole validates"
    );
}

/// A window's chart rectangle: its azimuth span and its meridian span.
type Window = ((f64, f64), (f64, f64));

/// **Contractible rings on a torus face, at `T`**: on the donut's face
/// (its band between two parallels), a window in the face validates; the
/// same window moved half a tube turn, off the face, is refused, as is a
/// window nested inside another; two windows side by side validate. Each
/// contractible cycle is read against the other alone and against the
/// essential pair, and the essential pair against each window alone
/// (`topo::one_region`).
fn torus_rings<T: Scalar>(scalar: &str) {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let donut = donut::<T>();
    let face = same_face(&at_f64, face64, &donut);
    let honest = |windows: &[Window]| {
        let mut body = donut.clone();
        for &(us, ss) in windows {
            plant(&mut body, face, frame, us, ss);
        }
        body
    };
    let one = honest(&[((-0.3, 0.3), (-0.4, 0.4))]);
    let off = honest(&[(
        (-0.3, 0.3),
        (core::f64::consts::PI - 0.4, core::f64::consts::PI + 0.4),
    )]);
    honest_then_misplaced(
        &format!("{scalar}: torus ring off the face"),
        &one,
        &off,
        face,
    );
    // The review's probe (PR 6163, `review_probe::windows_at_interval`):
    // azimuth `[-0.5, -0.1]` and `[0.1, 0.5]` over tube angle `[1, 2]`,
    // which this face spans.
    let probe = (1.0 - frame.shift, 2.0 - frame.shift);
    assert!(
        probe.0 > -core::f64::consts::FRAC_PI_2 && probe.1 < core::f64::consts::FRAC_PI_2,
        "[{scalar}] the probe's windows lie on the face: {probe:?}"
    );
    let windows = honest(&[((-0.5, -0.1), probe), ((0.1, 0.5), probe)]);
    assert_eq!(
        topo::validate_geometric_structural(&windows, tol()),
        Ok(()),
        "[{scalar}] the review's two windows apart validate"
    );
    let apart = honest(&[((-0.5, -0.1), (-0.5, 0.5)), ((0.1, 0.5), (-0.5, 0.5))]);
    let nested = honest(&[((-0.5, 0.5), (-0.5, 0.5)), ((-0.2, 0.2), (-0.2, 0.2))]);
    honest_then_misplaced(
        &format!("{scalar}: torus ring in a ring"),
        &apart,
        &nested,
        face,
    );
    assert_eq!(
        region_findings(&nested).len(),
        1,
        "[{scalar}] one ring lies in the other, and the other outside the first"
    );
}

#[test]
fn a_torus_face_refuses_a_ring_off_it_or_inside_another() {
    torus_rings::<f64>("f64");
}

/// **The same at the certified scalar.** Two windows apart on the torus
/// face were refused here once (`CycleSideUndecided`,
/// `bool_torus_region_component` over `[-1.38, 2.37]`): a meridian leg
/// from one window's corner has a root at its own start on the plane of
/// the other's parallel, and the torus door unwound it onto the branch of
/// a whole turn about `π`, whose cut that root straddles
/// ([`windows_on_an_azimuth_grid`] states the cause and the fix).
#[test]
fn a_torus_face_refuses_a_ring_off_it_or_inside_another_at_the_certified_scalar() {
    torus_rings::<geom_core::Interval>("interval");
}

/// **A ring inside another ring on a sphere, at the certified scalar**:
/// the f64 row's pair, read at `Interval`.
#[test]
fn a_sphere_face_refuses_a_ring_inside_another_at_the_certified_scalar() {
    let at_f64 = revolved_disc::<f64>();
    let (face64, frame) = wall(&at_f64, is_sphere);
    let ball = revolved_disc::<geom_core::Interval>();
    let face = same_face(&at_f64, face64, &ball);
    let mut honest = ball.clone();
    plant(&mut honest, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut honest, face, frame, (0.1, 0.9), (1.0, 2.0));
    let mut misplaced = ball;
    plant(&mut misplaced, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut misplaced, face, frame, (-0.7, -0.3), (1.3, 1.7));
    honest_then_misplaced("interval: sphere rings", &honest, &misplaced, face);
}

/// **A sense-flipped face is check 6's, reported once**: the cylinder
/// wall holding two windows apart, its sense bit flipped, puts each of its
/// cycles off its own side of the others. Check 6 refuses the flip
/// (`CurvedSenseInverted`), and the region arm skips the face it refused
/// rather than naming every cycle again.
#[test]
fn a_sense_flipped_wall_is_refused_by_check_6_alone() {
    let tube = revolved_polygon(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, is_cylinder);
    let mut body = tube;
    plant(&mut body, face, frame, (-0.9, -0.1), (0.5, 1.5));
    plant(&mut body, face, frame, (0.1, 0.9), (0.5, 1.5));
    assert_eq!(
        topo::validate_geometric_structural(&body, tol()),
        Ok(()),
        "the honest wall validates"
    );
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    let errors = topo::validate_geometric_structural(&flipped, tol()).unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == face)),
        "check 6 refuses the flip: {errors:?}"
    );
    assert_eq!(
        region_findings(&flipped),
        Vec::new(),
        "the region arm does not read the face check 6 refused"
    );
}

/// **A torus cycle with no stored joint element has no winding**: the
/// donut's face with one parallel's element taken away reads that cycle's
/// winding as `None`, and the region arm exempts the face (check 8
/// reports the missing element), never reading the gap as the identity.
/// Read as the identity, the parallel would be contractible, and the face
/// would hold one essential cycle, refused `EssentialCyclesUnpaired`.
#[test]
fn a_torus_cycle_missing_a_joint_element_is_not_read_as_contractible() {
    let mut donut = donut::<f64>();
    let (face, _) = wall(&donut, is_torus);
    let cycles = topo::surface_boundary::surface_cycle_windings(&donut, face).unwrap();
    let h = cycles[0].0[0];
    donut.detach_joint(h).expect("a minted joint");
    let cycles = topo::surface_boundary::surface_cycle_windings(&donut, face).unwrap();
    assert_eq!(
        cycles[0].1, None,
        "the cycle missing an element has no winding"
    );
    assert!(
        cycles[1].1.is_some(),
        "the other parallel keeps its winding"
    );
    assert_eq!(
        region_findings(&donut),
        Vec::new(),
        "the region arm exempts the face, refusing nothing"
    );
}

/// **Windows on an azimuth grid, at `T`**: two windows over one azimuth
/// span and disjoint tube spans, `(-1, -0.1)` and `(0.1, 1)` about the
/// donut face's middle, validate; so does every point of the face on a
/// window's meridian line off the window, which the uncut door places in
/// the face. At `Interval` both were once unread: a parallel leg from a
/// point on a meridian edge's plane has a root at its own start, and the
/// torus door unwound its roots onto the branch of a whole turn about
/// `π`, whose cut that root straddles, so the root's enclosure was the
/// whole turn and every parallel path from the point was set aside
/// (`bool_torus_region_span` over about `[-1.5, 1.4]`). A leg whose start
/// lies on the plane now has its roots unwound onto no branch
/// (`first_harmonic_leg_roots`); every other leg reads them as before.
fn windows_on_an_azimuth_grid<T: Scalar>(scalar: &str) {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let donut = donut::<T>();
    let face = same_face(&at_f64, face64, &donut);
    let mut stacked = donut.clone();
    plant(&mut stacked, face, frame, (-0.5, 0.5), (-1.0, -0.1));
    plant(&mut stacked, face, frame, (-0.5, 0.5), (0.1, 1.0));
    assert_eq!(
        topo::validate_geometric_structural(&stacked, tol()),
        Ok(()),
        "[{scalar}] two windows stacked in tube angle validate"
    );
    let mut one = donut;
    plant(&mut one, face, frame, (-0.3, 0.3), (-0.4, 0.4));
    let band = geom_core::Band::linear(tol()).unwrap();
    for (u, s) in [(-0.3, -1.0), (-0.3, -0.45), (-0.3, 0.6), (0.3, -1.0)] {
        assert_eq!(
            topo::curved_face_containment(&one, face, pt(frame.at(u, s)), band),
            Ok(Some(topo::FaceContainment::In)),
            "[{scalar}] the point ({u}, {s}) on the window's meridian line, off it"
        );
    }
}

#[test]
fn windows_on_an_azimuth_grid_validate() {
    windows_on_an_azimuth_grid::<f64>("f64");
}

#[test]
fn windows_on_an_azimuth_grid_validate_at_the_certified_scalar() {
    windows_on_an_azimuth_grid::<geom_core::Interval>("interval");
}

/// **Windows a hair apart in tube angle, at `T`**: two windows over one
/// azimuth span, mirrored about the donut face's middle, `(-1, -g)` and
/// `(g, 1)`, validate for gaps `2g` from 2 mrad up, far clear of the band
/// at every ε. Their near parallels share one plane, which runs within
/// `r·g²/2` of the top of every meridian leg's circle, so their crossings
/// with a meridian leg are not read as that plane's roots: a near-double
/// root, whose margin falls inside the band at a coarse ε or whose root
/// slack does at a fine one, would leave the windows unread. The face's
/// seam vertices also stand at the azimuth opposite the windows' middle,
/// where an edge's angle lies at the cut of its span's branch.
fn windows_a_hair_apart<T: Scalar>(scalar: &str) {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let donut = donut::<T>();
    let face = same_face(&at_f64, face64, &donut);
    for g in [0.001, 0.003, 0.01] {
        let mut body = donut.clone();
        plant(&mut body, face, frame, (-0.5, 0.5), (-1.0, -g));
        plant(&mut body, face, frame, (-0.5, 0.5), (g, 1.0));
        assert_eq!(
            topo::validate_geometric_structural(&body, tol()),
            Ok(()),
            "[{scalar}] two windows {} rad apart in tube angle validate (ε {})",
            2.0 * g,
            tol().eps()
        );
    }
}

#[test]
fn torus_windows_a_hair_apart_in_tube_angle_validate() {
    windows_a_hair_apart::<f64>("f64");
}

#[test]
fn torus_windows_a_hair_apart_in_tube_angle_validate_at_the_certified_scalar() {
    windows_a_hair_apart::<geom_core::Interval>("interval");
}

/// **A torus window read in band escalates typed, at `T`**: on the donut's
/// face, a second window whose every corner stands `3ε` off the first
/// one's corners along both chart directions (the tube's radius is 1, so
/// a tube angle is an arc length), so no vertex of either is placed
/// against the other, and the arm refuses `CycleSideUndecided` carrying
/// its margin, never a pass.
fn a_torus_window_read_in_band<T: Scalar>(scalar: &str) {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let mut body = donut::<T>();
    let face = same_face(&at_f64, face64, &body);
    let d = 3.0 * tol().eps();
    plant(&mut body, face, frame, (-0.3, 0.3), (-0.4, 0.4));
    plant(
        &mut body,
        face,
        frame,
        (-0.3 - d, 0.3 + d),
        (-0.4 - d, 0.4 + d),
    );
    let errors = topo::validate_geometric_structural(&body, tol()).unwrap_err();
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::CycleSideUndecided { face: f, source, .. }
                if *f == face && !source.margin.is_invalid()
        )),
        "[{scalar}] the region arm escalates on the face: {errors:?}"
    );
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::CycleOffFaceSide { .. })),
        "[{scalar}] and reads no side: {errors:?}"
    );
}

#[test]
fn a_torus_window_read_in_band_escalates_with_its_margin() {
    a_torus_window_read_in_band::<f64>("f64");
}

#[test]
fn a_torus_window_read_in_band_escalates_with_its_margin_at_the_certified_scalar() {
    a_torus_window_read_in_band::<geom_core::Interval>("interval");
}

/// **A ring wound as an outer loop, refused by the essential pair's
/// readings alone** (M5a, PR 6163's re-check): on the donut's face, a
/// window whose ring on the face is walked the other way round, so the
/// region it bounds is the window's own rectangle. The ring's vertices lie
/// on the face's side of the essential pair (between its two parallels),
/// so the ring read against the pair passes; each parallel read against
/// the ring alone lies outside the rectangle, off the face's side, and is
/// refused. Drop those two readings and the wall draws no finding. The
/// plug, its outer loop now wound as a ring, is check 6's
/// (`CurvedSenseInverted`), a different face.
fn a_ring_wound_as_an_outer_loop<T: Scalar>(scalar: &str) {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let donut = donut::<T>();
    let face = same_face(&at_f64, face64, &donut);
    let pair: Vec<topo::VertexKey> = topo::surface_boundary::surface_cycle_windings(&donut, face)
        .unwrap()
        .iter()
        .flat_map(|(cycle, _)| cycle.iter().map(|&h| donut.get_half_edge(h).unwrap().start))
        .collect();
    let mut honest = donut.clone();
    plant(&mut honest, face, frame, (-0.3, 0.3), (-0.4, 0.4));
    assert_eq!(
        topo::validate_geometric_structural(&honest, tol()),
        Ok(()),
        "[{scalar}] the window in the face validates"
    );
    let mut wound = donut;
    let plug = plant_wound(&mut wound, face, frame, (-0.3, 0.3), (-0.4, 0.4), true);
    let errors = topo::validate_geometric_structural(&wound, tol()).unwrap_err();
    let (on_wall, elsewhere): (Vec<_>, Vec<_>) = errors.iter().partition(
        |e| matches!(e, ValidationError::CycleOffFaceSide { face: f, .. } if *f == face),
    );
    assert!(
        !on_wall.is_empty()
            && on_wall.iter().all(|e| matches!(
                e,
                ValidationError::CycleOffFaceSide { at, .. } if pair.contains(at)
            )),
        "[{scalar}] the wall is refused at the pair's vertices, read against the ring: {errors:?}"
    );
    assert!(
        elsewhere
            .iter()
            .all(|e| matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == plug)),
        "[{scalar}] nothing else refuses but check 6 on the plug: {errors:?}"
    );
}

#[test]
fn a_torus_ring_wound_as_an_outer_loop_is_refused_by_the_pair() {
    a_ring_wound_as_an_outer_loop::<f64>("f64");
}

#[test]
fn a_torus_ring_wound_as_an_outer_loop_is_refused_by_the_pair_at_the_certified_scalar() {
    a_ring_wound_as_an_outer_loop::<geom_core::Interval>("interval");
}

/// **A point beside one window reads in the face at the certified
/// scalar** (PR 6163's re-check, `recheck_e1_gap_door`): one window over
/// tube angle `(-1, -0.1)` on the donut's face, and points on the face's
/// middle circle (`s = 0`) at azimuths beside and across from it. A
/// meridian leg's root on the window's parallel edge is a whole turn wide
/// at `Interval`, so its point's `ρ` cannot tell the plane's two parallels
/// apart; its azimuth, read off the point's own coordinates, lies past the
/// edge's span, which is no crossing whichever parallel it lies on.
#[test]
fn a_point_beside_one_torus_window_reads_in_the_face_at_the_certified_scalar() {
    let at_f64 = donut::<f64>();
    let (face64, frame) = wall(&at_f64, is_torus);
    let donut = donut::<geom_core::Interval>();
    let face = same_face(&at_f64, face64, &donut);
    let band = geom_core::Band::linear(tol()).unwrap();
    for windows in [
        vec![((-0.5, 0.5), (-1.0, -0.1))],
        vec![((-0.5, 0.5), (-1.0, -0.1)), ((-0.5, 0.5), (0.1, 1.0))],
    ] {
        let mut body = donut.clone();
        for &(us, ss) in &windows {
            plant(&mut body, face, frame, us, ss);
        }
        for u in [0.0, 0.2, 1.0, -0.6] {
            assert_eq!(
                topo::curved_face_containment(&body, face, pt(frame.at(u, 0.0)), band),
                Ok(Some(topo::FaceContainment::In)),
                "the point ({u}, 0) beside {windows:?}"
            );
        }
    }
}
