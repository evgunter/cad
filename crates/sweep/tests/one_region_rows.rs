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
/// meridian parameter `s` (a cylinder's height, a cone's slant).
#[derive(Clone, Copy)]
struct Frame {
    origin: Point3<f64>,
    axis: Vec3<f64>,
    e1: Vec3<f64>,
    /// A cylinder's radius, or `None` on a cone of this half-angle.
    radius: Option<f64>,
    half_angle: f64,
}

impl Frame {
    fn of(surface: &Surface<f64>) -> Self {
        match *surface {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Self {
                origin,
                axis,
                e1: u_ref,
                radius: Some(radius),
                half_angle: 0.0,
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Self {
                origin: apex,
                axis,
                e1: u_ref,
                radius: None,
                half_angle,
            },
            _ => panic!("a surface of revolution"),
        }
    }

    fn radial(&self, u: f64) -> Vec3<f64> {
        let e2 = self.axis.cross(self.e1);
        self.e1 * u.cos() + e2 * u.sin()
    }

    /// The parallel at `s`: its height along the axis and its radius.
    fn parallel(&self, s: f64) -> (f64, f64) {
        match self.radius {
            Some(r) => (s, r),
            None => (s * self.half_angle.cos(), s * self.half_angle.sin()),
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
}

/// The face of `body` on the surface `kind` picks, and its frame.
fn wall(body: &Body<f64>, kind: fn(&Surface<f64>) -> bool) -> (FaceKey, Frame) {
    body.faces()
        .find_map(|(k, f)| {
            let s = body.get_surface(f.surface)?;
            kind(s).then(|| (k, Frame::of(s)))
        })
        .expect("the wall")
}

fn is_cylinder(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cylinder { .. })
}

fn is_cone(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Cone { .. })
}

/// **A window planted on `face`**: the chart rectangle azimuth `us` ×
/// meridian `ss`, its two parallels' arcs and its two rulings, a plug
/// face filling it and its loop a ring of `face`. Grown off `face`'s
/// outer loop wherever the rectangle lies: the Euler doors read no
/// geometry. Returns the plug.
fn plant(body: &mut Body<f64>, face: FaceKey, frame: Frame, us: (f64, f64), ss: (f64, f64)) -> FaceKey {
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
        .mev_line(MevSite::Fan { he1: first, he2: first }, p[0], tol())
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let a = body
        .mev(fan(strut.he_minus), p[1], frame.arc(ss.0, us, true), tol())
        .unwrap();
    let ruling = |q0: Point3<f64>, q1: Point3<f64>| EdgeCurveSpec::line_between(q0, q1);
    let b = body
        .mev(fan(a.he_minus), p[2], ruling(p[1], p[2]), tol())
        .unwrap();
    let c = body
        .mev(fan(b.he_minus), p[3], frame.arc(ss.1, us, false), tol())
        .unwrap();
    let plug = body
        .mef(
            MefSite::Chords {
                he1: a.he_plus,
                he2: c.he_minus,
            },
            ruling(p[0], p[3]),
            FaceSurface::Inherit,
            tol(),
        )
        .unwrap()
        .face;
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
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
    assert!(!errors.is_empty(), "[{name}] the misplaced window is refused");
}

/// **P09, a cone band less a window outside it** (CONEARM's probe, the
/// cone of half-angle ½ at the apex): the band over slant `[1, 2]` with a
/// window over slant `[3, 3.3]`, azimuth `[1, 2]`. The honest twin's
/// window is over slant `[1.3, 1.7]`.
#[test]
fn a_cone_band_refuses_a_window_beyond_its_rim() {
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let band = revolved(&[(s, c), (2.0 * s, 2.0 * c), (0.0, 2.0 * c), (0.0, c)]);
    let (face, frame) = wall(&band, is_cone);
    let mut honest = band.clone();
    plant(&mut honest, face, frame, (1.0, 2.0), (1.3, 1.7));
    let mut misplaced = band;
    plant(&mut misplaced, face, frame, (1.0, 2.0), (3.0, 3.3));
    honest_then_misplaced("P09", &honest, &misplaced, face);
}

/// **P47, a cone cap with a window outside it**: the cap from the apex
/// to the rim at slant 2, with a window over slant `[3, 3.5]`, azimuth
/// `[1, 2]`. The honest twin's is over slant `[1, 1.5]`.
#[test]
fn a_cone_cap_refuses_a_window_beyond_its_rim() {
    let (c, s) = (0.5_f64.cos(), 0.5_f64.sin());
    let cap = revolved(&[(0.0, 0.0), (2.0 * s, 2.0 * c), (0.0, 2.0 * c)]);
    let (face, frame) = wall(&cap, is_cone);
    let mut honest = cap.clone();
    plant(&mut honest, face, frame, (1.0, 2.0), (1.0, 1.5));
    let mut misplaced = cap;
    plant(&mut misplaced, face, frame, (1.0, 2.0), (3.0, 3.5));
    honest_then_misplaced("P47", &honest, &misplaced, face);
}

/// **P49, a window face holding a hole outside it**: on the band over
/// slant `[0.5, 2.5]`, the plug of the window slant `[1, 2]` × azimuth
/// `[1, 2]` holds a hole over slant `[1.2, 1.4]`, azimuth `[3, 3.5]`.
/// The honest twin's hole is over azimuth `[1.2, 1.5]`.
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
    let window = plant(&mut band, wall_face, frame, (1.0, 2.0), (1.0, 2.0));
    let mut honest = band.clone();
    plant(&mut honest, window, frame, (1.2, 1.5), (1.2, 1.4));
    let mut misplaced = band;
    plant(&mut misplaced, window, frame, (3.0, 3.5), (1.2, 1.4));
    honest_then_misplaced("P49", &honest, &misplaced, window);
}

/// **A ring inside another ring on a cylinder**: two windows on a
/// cylinder's wall, the second inside the first. Each alone validates.
#[test]
fn a_cylinder_wall_refuses_a_ring_inside_another() {
    let tube = revolved(&[(1.0, 0.0), (1.0, 2.0), (0.0, 2.0), (0.0, 0.0)]);
    let (face, frame) = wall(&tube, is_cylinder);
    let mut honest = tube.clone();
    plant(&mut honest, face, frame, (1.0, 2.0), (0.5, 1.5));
    plant(&mut honest, face, frame, (3.0, 4.0), (0.5, 1.5));
    let mut misplaced = tube;
    plant(&mut misplaced, face, frame, (1.0, 2.0), (0.5, 1.5));
    plant(&mut misplaced, face, frame, (1.2, 1.8), (0.8, 1.2));
    honest_then_misplaced("cylinder rings", &honest, &misplaced, face);
    assert_eq!(
        region_findings(&misplaced).len(),
        1,
        "one ring lies in the other, and the other outside the first"
    );
}
