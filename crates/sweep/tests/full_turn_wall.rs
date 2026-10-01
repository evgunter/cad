//! **A face that ALONE wraps its carrier's azimuth gets a verdict, and
//! a face that only reads a whole turn does not.**
//!
//! The main fixture is a drilled BEAD: the lens `ρ = r` (the bore line)
//! and the arc of the radius-`R` circle through `ρ = R`, revolved about
//! the sketch's `y` axis. A full turn gives two faces, each the whole
//! turn of its carrier: a bore CYLINDER with a self-mated seam and a
//! SPHERE zone. Their two shared rim circles are the whole boundary.
//!
//! - Both doors ([`topo::curved_face_containment`],
//!   [`topo::point_in_solid`]) place a point on either carrier by its
//!   height (latitude) window alone, at every pose and scale the oracle
//!   row sweeps.
//! - A revolve a hair short of a turn reads a window just under a
//!   period, and its gap is `Out` at both doors.
//! - A sphere face that reads a whole turn but does not wrap alone (a
//!   zone merged with half a cap) is not served as a full turn.
//! - A face with a window narrower than a turn is not served as one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom::Surface;
use geom_core::{OrthoFrame, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, FaceContainment, FaceKey, SolidContainment, curved_face_containment, point_in_solid,
};

/// The sketch frame a bead is revolved in: origin `o`, sketch axes
/// `u` (radial at azimuth 0) and `v` (the revolve axis), and `w = u × v`.
struct Pose {
    o: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
}

impl Pose {
    /// The world frame at `o`, or a frame tilted off every world axis.
    fn new(tilted: bool, o: Point3<f64>) -> Self {
        let (u, v) = if tilted {
            (Vec3::new(0.7, 0.3, -0.2), Vec3::new(-0.1, 0.8, 0.55))
        } else {
            (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))
        };
        let f = OrthoFrame::gram_schmidt(o, u, v, "full_turn_wall", band()).unwrap();
        Self {
            o,
            u: f.u().get(),
            v: f.v().get(),
            w: f.w().get(),
        }
    }

    /// The point at radius `rho` from the axis, height `y`, azimuth `a`.
    fn at(&self, rho: f64, y: f64, a: f64) -> Point3<f64> {
        self.o + self.u * (rho * a.cos()) + self.v * y + self.w * (rho * a.sin())
    }

    fn plane(&self) -> SketchPlane<f64> {
        SketchPlane::from_frame(
            OrthoFrame::gram_schmidt(self.o, self.u, self.v, "full_turn_wall", band()).unwrap(),
        )
    }
}

/// The sphere radius `big_r`, bore radius `r` bead, swept through `turn`.
fn bead(pose: &Pose, big_r: f64, r: f64, turn: Revolution<f64>) -> Body<f64> {
    let h = (big_r * big_r - r * r).sqrt();
    // The arc from (r, −h) to (r, h) through ρ = R turns CCW about the
    // origin by 2φ, φ = atan(h/r): bulge tan(φ/2).
    let phi = (h / r).atan();
    let lp = bulge_loop(vec![
        (Point2::new(r, -h), (phi / 2.0).tan()),
        (Point2::new(r, h), 0.0),
    ]);
    let vp = Profile::new(pose.plane(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, turn, Tol::witness()).unwrap().body
}

/// The one face of `body` on a cylinder (`cyl`) or a sphere.
fn the_face(body: &Body<f64>, cyl: bool) -> FaceKey {
    let found: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Cylinder { .. }) => cyl,
            Some(Surface::Sphere { .. }) => !cyl,
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(found.len(), 1, "one face per carrier: {found:?}");
    found[0]
}

/// Azimuths clear of the seam at 0, and the fractions of the bore's
/// half-height and of the half-turn the oracle row places points at.
const AZIMUTHS: [f64; 4] = [0.7, 2.0, 4.1, 5.5];
const HEIGHTS: [f64; 4] = [0.2, 0.9, 1.1, 1.4];
const POLARS: [f64; 4] = [0.2, 0.6, 1.6, 2.9];

/// **Both doors against the closed-form bead**, at two poses and two
/// scales: every verdict the doors give is the one the geometry gives.
#[test]
fn both_doors_answer_the_bead_by_its_height_window() {
    let b = band();
    for tilted in [false, true] {
        for scale in [1.0, 100.0] {
            let pose = Pose::new(tilted, Point3::new(0.3 * scale, -1.1 * scale, 2.0 * scale));
            let (big_r, r) = (scale, 0.5 * scale);
            let hr = (big_r * big_r - r * r).sqrt();
            let body = bead(&pose, big_r, r, Revolution::Full);
            assert_eq!(topo::validate_geometric(&body, Tol::witness()), Ok(()));
            let (wall, zone) = (the_face(&body, true), the_face(&body, false));
            let tag = format!("tilted {tilted}, scale {scale}");
            for a in AZIMUTHS {
                for k in HEIGHTS {
                    let want = if k < 1.0 {
                        FaceContainment::In
                    } else {
                        FaceContainment::Out
                    };
                    for y in [k * hr, -k * hr] {
                        assert_eq!(
                            curved_face_containment(&body, wall, pose.at(r, y, a), b).unwrap(),
                            Some(want),
                            "bore at height {y}, azimuth {a} ({tag})"
                        );
                    }
                }
                for pol in POLARS {
                    let (y, rho) = (big_r * pol.cos(), big_r * pol.sin());
                    let want = if rho > r {
                        FaceContainment::In
                    } else {
                        FaceContainment::Out
                    };
                    assert_eq!(
                        curved_face_containment(&body, zone, pose.at(rho, y, a), b).unwrap(),
                        Some(want),
                        "zone at polar {pol}, azimuth {a} ({tag})"
                    );
                }
                for (rho, y, want) in [
                    (0.75, 0.1, SolidContainment::In),
                    (0.25, 0.1, SolidContainment::Out),
                    (0.75, 0.9, SolidContainment::Out),
                    (1.1, 0.0, SolidContainment::Out),
                    (0.5, 0.3, SolidContainment::OnBoundary),
                ] {
                    assert_eq!(
                        point_in_solid(
                            &body,
                            pose.at(rho * scale, y * scale, a),
                            b,
                            Tol::witness()
                        )
                        .unwrap(),
                        want,
                        "solid at ρ {rho}, y {y}, azimuth {a} ({tag})"
                    );
                }
            }
        }
    }
}

/// **A revolve a hair short of a turn is not a full turn.** Its faces
/// do not wrap alone (the two end planes bound them), so their windows
/// trim, and the `δ = 10⁻³` gap is `Out` at both doors while the swept
/// side is `In`.
///
/// The window is `τ − δ` wide, so its cosine margin at these points is
/// second order in `δ` — about `r·(δ/2)·(δ/10)` at the nearest one,
/// `2.5·10⁻⁸` m. Where the run's band cannot resolve that (the coarse ε
/// rows), the row asserts only that no door answers the WRONG side; a
/// no-verdict there is the doors' honest remainder.
#[test]
fn a_near_full_revolves_gap_is_out_at_both_doors() {
    let b = band();
    let delta = 1e-3;
    let resolved = 0.5 * (delta / 2.0) * (delta / 10.0) > 20.0 * Tol::witness().get().eps;
    let pose = Pose::new(false, Point3::new(0.0, 0.0, 0.0));
    let body = bead(
        &pose,
        1.0,
        0.5,
        Revolution::Partial(core::f64::consts::TAU - delta),
    );
    let (wall, zone) = (the_face(&body, true), the_face(&body, false));
    let rho = (1.0f64 - 0.04).sqrt();
    // The gap is the azimuths (τ − δ, τ) in the sweep's sense, which the
    // pose's frame reads as (0, δ); its far side, −δ/2, is swept.
    for (a, inside) in [
        (delta / 2.0, false),
        (delta / 10.0, false),
        (-delta / 2.0, true),
        (2.0, true),
    ] {
        let (want, wrong) = if inside {
            (FaceContainment::In, FaceContainment::Out)
        } else {
            (FaceContainment::Out, FaceContainment::In)
        };
        let (solid, solid_wrong) = if inside {
            (SolidContainment::In, SolidContainment::Out)
        } else {
            (SolidContainment::Out, SolidContainment::In)
        };
        let bore = curved_face_containment(&body, wall, pose.at(0.5, 0.2, a), b);
        let zone_v = curved_face_containment(&body, zone, pose.at(rho, 0.2, a), b);
        let in_solid = point_in_solid(&body, pose.at(0.75, 0.1, a), b, Tol::witness());
        if resolved {
            assert_eq!(bore.unwrap(), Some(want), "bore at azimuth {a}");
            assert_eq!(zone_v.unwrap(), Some(want), "zone at azimuth {a}");
            assert_eq!(in_solid.unwrap(), solid, "solid at azimuth {a}");
        } else {
            assert!(
                !matches!(bore, Ok(Some(v)) if v == wrong),
                "bore at azimuth {a}: {bore:?}"
            );
            assert!(
                !matches!(zone_v, Ok(Some(v)) if v == wrong),
                "zone at azimuth {a}: {zone_v:?}"
            );
            assert!(
                !matches!(in_solid, Ok(v) if v == solid_wrong),
                "solid at azimuth {a}: {in_solid:?}"
            );
        }
    }
}

/// **A sphere face that reads a whole turn but does not wrap alone is
/// not served as one.** A unit ball revolved from three arcs (south cap,
/// the ±30° zone, north cap) is six half-faces. `kef_minting` merges the
/// zone's two halves across their seam, then merges the zone with ONE
/// north-cap half across their rim arc. That face's walk reads the whole
/// turn and the latitudes from −30° to the pole, but north of +30° it
/// holds only half the azimuths: the other north half bounds it along
/// two meridians, mated to a face that is not itself.
///
/// The body is not a valid one — tier 3 reports a pcurve
/// `LoopDiscontinuity` on the merged loop — and the row says so: its
/// subject is that the face door does not answer `In` for such a face
/// at a point another face holds. No public door mints a valid body with
/// this face shape.
#[test]
fn a_zone_merged_with_half_a_cap_is_not_a_full_turn() {
    let b = band();
    let (s30, c30) = (0.5f64, 0.75f64.sqrt());
    let bulge = (core::f64::consts::FRAC_PI_3 / 4.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -1.0), bulge),
        (Point2::new(c30, -s30), bulge),
        (Point2::new(c30, s30), bulge),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let mut body = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    let pose = Pose::new(false, Point3::new(0.0, 0.0, 0.0));
    let on_sphere = |pol: f64, a: f64| pose.at(pol.sin(), pol.cos(), a);
    let sphere_faces = |body: &Body<f64>| -> Vec<FaceKey> {
        body.faces()
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .map(|(k, _)| k)
            .collect()
    };
    let holder = |body: &Body<f64>, p: Point3<f64>| -> FaceKey {
        let held: Vec<FaceKey> = sphere_faces(body)
            .into_iter()
            .filter(|&f| {
                curved_face_containment(body, f, p, b).unwrap() == Some(FaceContainment::In)
            })
            .collect();
        assert_eq!(held.len(), 1, "one face holds {p:?}: {held:?}");
        held[0]
    };
    // The half-edge on `fa`'s side of an edge between `fa` and `fb`.
    let between = |body: &Body<f64>, fa: FaceKey, fb: FaceKey, rim: bool| {
        body.edges()
            .find_map(|(_, e)| {
                let (pa, pb) = (
                    body.face_of_half_edge(e.he_plus)?,
                    body.face_of_half_edge(e.he_minus)?,
                );
                let circle = body
                    .get_curve_geom(e.curve)
                    .and_then(topo::null::CurveGeom::certified)
                    .map(|c| c.carrier().clone());
                let is_rim =
                    matches!(circle, Some(geom::Curve3::Circle { axis, .. }) if axis.y.abs() > 0.9);
                if is_rim != rim {
                    return None;
                }
                match (pa, pb) {
                    _ if (pa, pb) == (fa, fb) => Some(e.he_plus),
                    _ if (pa, pb) == (fb, fa) => Some(e.he_minus),
                    _ => None,
                }
            })
            .expect("an edge between the two faces")
    };
    // The zone's two halves, across their seam meridian.
    let (z1, z2) = (
        holder(&body, on_sphere(1.6, 1.0)),
        holder(&body, on_sphere(1.6, 4.0)),
    );
    let seam = between(&body, z1, z2, false);
    body.kef_minting(seam, Tol::witness()).unwrap();
    // The zone and the north half that holds azimuth 1.0; the merge
    // kills the face on the half-edge's side and keeps the zone.
    let zone = holder(&body, on_sphere(1.6, 1.0));
    let north = holder(&body, on_sphere(0.4, 1.0));
    let rim = between(&body, north, zone, true);
    let killed = body.kef_minting(rim, Tol::witness()).unwrap().killed_face;
    assert_eq!(killed, north, "the north half merges into the zone");
    assert!(
        topo::validate_geometric(&body, Tol::witness()).is_err(),
        "the merged body is not tier-3 valid (see the row's doc)"
    );
    let merged = zone;
    // North of +30° at azimuth 4.0 is the OTHER north half's.
    let other = on_sphere(0.4, 4.0);
    assert_ne!(
        curved_face_containment(&body, merged, other, b).unwrap(),
        Some(FaceContainment::In),
        "the merged face holds only half the azimuths north of +30°"
    );
}

/// **A face with a window narrower than a turn is not served as one.**
/// The three-arc peg's wall faces are each a third of the turn, bounded
/// by two rim arcs and two meridian lines mated to the neighbouring
/// thirds; on their carrier, a point at another third's azimuth is
/// `Out` of the face, whatever its height.
#[test]
fn a_third_of_a_turn_is_out_at_another_thirds_azimuth() {
    let b = band();
    let peg = crate::mate2_common::peg(0.0, 1.0);
    assert_eq!(topo::validate_geometric(&peg, Tol::witness()), Ok(()));
    let walls = crate::mate2_common::walls_at(&peg, 0.5);
    assert_eq!(walls.len(), 3, "three wall thirds");
    let at = |deg: f64| {
        let a = deg.to_radians();
        Point3::new(0.5 * a.cos(), 0.5 * a.sin(), 0.5)
    };
    for &wall in &walls {
        let verdicts: Vec<_> = [60.0, 180.0, 300.0]
            .iter()
            .map(|&deg| curved_face_containment(&peg, wall, at(deg), b).unwrap())
            .collect();
        let held = verdicts
            .iter()
            .filter(|v| **v == Some(FaceContainment::In))
            .count();
        let out = verdicts
            .iter()
            .filter(|v| **v == Some(FaceContainment::Out))
            .count();
        assert_eq!(
            (held, out),
            (1, 2),
            "face {wall:?} holds its own third only: {verdicts:?}"
        );
    }
}
