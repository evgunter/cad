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

/// The honest body passes every tier-3 check; the misplaced one is
/// refused by the region arm on `face`, and by nothing else.
fn honest_then_misplaced(name: &str, honest: &Body<f64>, misplaced: &Body<f64>, face: FaceKey) {
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
    let band = revolved(&[(s, c), (2.0 * s, 2.0 * c), (0.0, 2.0 * c), (0.0, c)]);
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
    let cap = revolved(&[(0.0, 0.0), (2.0 * s, 2.0 * c), (0.0, 2.0 * c)]);
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
    let mut band = revolved(&[
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
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
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
    let ball = revolved_disc();
    let (face, frame) = wall(&ball, is_sphere);
    let mut honest = ball.clone();
    plant(&mut honest, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut honest, face, frame, (0.1, 0.9), (1.0, 2.0));
    let mut misplaced = ball;
    plant(&mut misplaced, face, frame, (-0.9, -0.1), (1.0, 2.0));
    plant(&mut misplaced, face, frame, (-0.7, -0.3), (1.3, 1.7));
    honest_then_misplaced("sphere rings", &honest, &misplaced, face);
}

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

/// **A torus face's essential pair**: each of the donut's two faces is
/// the band between two parallels, which wind the azimuth once each,
/// opposite ways (the lift's integers, `cycle_winding`), and the donut
/// validates. The same face with one parallel's stored joint element
/// moved two periods reads two cycles wound the same way, which bound no
/// band, and the region arm names it; nothing in the face's geometry
/// changed, so the row pins that the arm reads the lift's integers.
#[test]
fn a_torus_face_reads_its_essential_cycles_as_one_pair() {
    let honest = donut();
    assert_eq!(
        topo::validate_geometric_structural(&honest, tol()),
        Ok(()),
        "the donut validates"
    );
    let (face, cycles) = honest
        .faces()
        .find_map(|(face, f)| {
            matches!(honest.get_surface(f.surface), Some(Surface::Torus { .. })).then(|| {
                let cycles = topo::surface_boundary::surface_cycles(&honest, face).unwrap();
                (face, cycles)
            })
        })
        .expect("a torus face");
    let windings: Vec<i64> = cycles
        .iter()
        .map(|c| {
            let w = topo::surface_boundary::cycle_winding(&honest, face, c).unwrap();
            i64::from(w.deck.u)
        })
        .collect();
    assert_eq!(windings.len(), 2, "two parallels bound the face");
    assert_eq!(windings[0], -windings[1], "wound opposite: {windings:?}");
    assert_ne!(windings[0], 0, "each winds the azimuth: {windings:?}");
    let mut misread = honest.clone();
    let h = cycles[0][0];
    let element = misread.joint(h).expect("a minted joint");
    let turned = topo::Deck {
        u: element.deck().u - 2 * windings[0] as i32,
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
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
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
/// read now through the doors): a window face 10 µm tall, 1 cm from the
/// ball's south pole, holds neither pole and lies in its lune, and the
/// body validates. The pole's side of the sliver is the sphere door's
/// reading at the pole, which no chart polygon approximates.
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
    let mut ball = revolved_disc();
    let (face, frame) = wall(&ball, is_sphere);
    plant(&mut ball, face, frame, (-0.5, 0.5), (0.01, 0.01 + height));
    assert_eq!(
        topo::validate_geometric_structural(&ball, tol()),
        Ok(()),
        "the sliver beside the pole validates"
    );
}
