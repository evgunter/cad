//! The sphere chart's containment arm: the `[azimuth] × [latitude]`
//! rectangle, at both doors.
//!
//! A sphere face is served when every boundary edge is a chart iso-line
//! — a latitude rim or a meridian great circle — because that face is
//! then exactly the rectangle its boundary pins. What it is NOT served
//! by is an axial level: the latitude window is carried as the exact
//! `(axial, radial)` pair of each extreme, and every margin against it
//! is `R sin Δv`, an arc length. The rows below put probes a hair off a
//! POLE on purpose, where an axial lever collapses: at `v = 1e-7` the
//! axial separation from the pole is `R(1 − cos v) ≈ 5e-15`, under every
//! ε this repo runs, while the arc length is `R sin v ≈ 1e-7`.
//!
//! Probes are placed **in the chart's own frame**, read off the surface,
//! rather than in world coordinates: which way a revolve sweeps and
//! where it puts its seam are the constructor's business, and a row
//! that hard-codes them is testing the constructor.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{PointInSolidError, SolidContainment, point_in_solid};
use topo::{Body, FaceContainment, FaceKey};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A unit-radius sphere band swept through `turn` radians about world
/// Y. Its sphere face's boundary is two meridian great-circle arcs
/// meeting at the two poles: the iso-line class, with both latitude
/// extremes AT a pole and no constraint on either side.
fn lune(turn: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(geom_core::Point2::new(0.0, -1.0), 1.0),
        ProfileVertex::new(geom_core::Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: geom_core::Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, turn, Tol::witness()).unwrap().body
}

/// A unit ball pierced by a tiny flat disc at the SOUTH pole, so its
/// sphere face's one LATITUDE RIM sits at polar angle `pi - u_r` — a
/// hair off that pole, and genuinely distinct from it.
///
/// This is the fixture the §7 planted red needs and the lune cannot be:
/// a lune's two latitude-window ends are BOTH poles, so both resolve to
/// `None` and the latitude-margin list is EMPTY — every verdict it
/// reports comes from the azimuth cosine alone, which is why mutating
/// the latitude lever leaves it green. Here `lat_hi` is the rim, the
/// probes straddle it, and the latitude margin is the only thing
/// deciding them.
///
/// The rim is placed at the SOUTH pole rather than the north so the
/// spherical arc that carries it is nearly a full semicircle rather
/// than a `u_r`-long sliver: the fixture has to be near-polar in the
/// PREDICATE's coordinates, not near-degenerate in the constructor's.
/// (`Profile::validate` refuses a revolve rim closer than ~1e-4 rad to
/// the axis, which is the floor this construction has to clear;
/// `u_r = 1e-3` and `1e-2` both clear it comfortably.)
fn ball_with_a_near_polar_rim(u_r: f64) -> Body<f64> {
    let (rho, h) = (u_r.sin(), -u_r.cos());
    let lp = ProfileLoop::new(vec![
        // On the axis, at the rim's own height: the flat disc's centre.
        ProfileVertex::new(geom_core::Point2::new(0.0, h), 0.0),
        // Out to the rim, then the long spherical arc to the north pole
        // (included angle `pi - u_r`, so nothing here is a sliver).
        ProfileVertex::new(
            geom_core::Point2::new(rho, h),
            ((core::f64::consts::PI - u_r) / 4.0).tan(),
        ),
        ProfileVertex::new(geom_core::Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: geom_core::Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn sphere_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, fd)| {
            matches!(
                body.get_surface(fd.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The face's own chart frame: `(centre, radius, polar axis, seam, the
/// seam's quadrature partner)`.
fn chart(body: &Body<f64>, face: FaceKey) -> (Point3<f64>, f64, Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let fd = body.get_face(face).unwrap();
    let Some(&geom::Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    }) = body.get_surface(fd.surface)
    else {
        panic!("a sphere face")
    };
    (center, radius, axis, u_ref, axis.cross(u_ref))
}

/// The point at chart `(azimuth, polar angle)`, scaled off the carrier
/// by `k` (1.0 = on it).
fn at(
    (c, r, axis, u, v): (Point3<f64>, f64, Vec3<f64>, Vec3<f64>, Vec3<f64>),
    az: f64,
    polar: f64,
    k: f64,
) -> Point3<f64> {
    c + (axis * polar.cos() + (u * az.cos() + v * az.sin()) * polar.sin()) * (r * k)
}

/// The swept quarter's own azimuths, in the chart's frame: one the face
/// definitely contains and one it definitely does not.
const IN_AZ: f64 = 0.4;
const OUT_AZ: f64 = -1.2;

/// The face-level door answers on a lune's sphere face in both
/// directions, all the way to the POLE — the latitude window's own
/// edge, and the place an axial lever cannot see.
#[test]
fn the_face_door_answers_on_a_lune_from_pole_to_pole() {
    let body = lune(Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 1, "a partial revolve mints one sphere band");
    let (f, ch, b) = (faces[0], chart(&body, faces[0]), band());
    let near = 1000.0 * Tol::witness().get().eps;
    for polar in [
        near,
        0.05,
        0.5,
        1.5,
        2.6,
        3.09,
        core::f64::consts::PI - near,
    ] {
        assert_eq!(
            topo::curved_face_containment(&body, f, at(ch, IN_AZ, polar, 1.0), b).unwrap(),
            Some(FaceContainment::In),
            "inside the swept quarter at polar angle {polar}"
        );
        assert_eq!(
            topo::curved_face_containment(&body, f, at(ch, OUT_AZ, polar, 1.0), b).unwrap(),
            Some(FaceContainment::Out),
            "outside the swept quarter at polar angle {polar}"
        );
    }
    // A point definitely off the CARRIER is definitely off the face,
    // whatever the chart window says about its direction — the carrier
    // test runs before the parameter-domain trim, as it must.
    for k in [0.5, 1.5] {
        assert_eq!(
            topo::curved_face_containment(&body, f, at(ch, IN_AZ, 0.7, k), b).unwrap(),
            Some(FaceContainment::Out),
            "off the carrier at k={k}"
        );
    }
}

/// The near-polar row #893's item 1 asks for, in the direction this
/// unit owes: two probes a hair off the SAME pole, on genuinely
/// different meridians, decided definitely and DIFFERENTLY.
///
/// The scale is band-relative on both sides, which is what makes the
/// row a statement about the lever rather than about one ε. A probe at
/// polar angle `k·eps` sits `R·k·eps` from the pole in ARC LENGTH —
/// definite for `k` well past the escalation width — and
/// `R(1 − cos) ≈ R(k·eps)²/2` from it AXIALLY, which for these `k` is
/// orders of magnitude INSIDE the zero band. An axial lever would call
/// both probes the pole and decide Zero; the arc-length lever decides
/// them apart. Both facts are asserted here, so the row cannot quietly
/// stop testing what it says it tests.
#[test]
fn near_polar_probes_stay_definite_where_an_axial_lever_collapses() {
    let body = lune(Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let (f, b) = (sphere_faces(&body)[0], band());
    let ch = chart(&body, f);
    let eps = Tol::witness().get().eps;
    for k in [100.0_f64, 1000.0] {
        let polar = k * eps;
        assert!(polar > 10.0 * eps, "the arc-length lever is definite");
        assert!(
            1.0 - polar.cos() < eps,
            "the axial lever has collapsed into the zero band at k={k}"
        );
        assert_eq!(
            topo::curved_face_containment(&body, f, at(ch, IN_AZ, polar, 1.0), b).unwrap(),
            Some(FaceContainment::In),
            "inside the quarter at polar angle {polar}"
        );
        assert_eq!(
            topo::curved_face_containment(&body, f, at(ch, OUT_AZ, polar, 1.0), b).unwrap(),
            Some(FaceContainment::Out),
            "outside the quarter at polar angle {polar}"
        );
    }
}

/// **The §7 planted red: the latitude margin is what decides, and the
/// axial lever cannot.** The row above is a statement about the sphere
/// chart's reach; this one is a statement about the LEVER CHOICE, and
/// it is the row that goes red if the choice is changed.
///
/// The distinction matters because the lune fixture cannot make it: a
/// lune's latitude window has a pole at BOTH ends, both ends resolve to
/// `None`, and `point_on_sphere_in_face` collects an EMPTY
/// latitude-margin list — every verdict it reports there comes from the
/// azimuth cosine window, so re-spelling `latitude_sine` as the axial
/// difference `(h_a − h_b)/R` (which is exactly #893's defective lever)
/// leaves the row green. Measured: with that mutation in place the whole
/// workspace suite stayed green.
///
/// Here the window's far end is a RIM at polar angle `pi − u_r`, and the
/// probes straddle it by `delta` in ARC LENGTH, with no azimuth question
/// anywhere near its own boundary. Both levers are computed in the open
/// below, so the row states its own premise rather than assuming it:
///
/// * arc lever `R|sin(v_hi − v_here)| = R sin(delta)`, definite when
///   `delta >= K*eps`;
/// * axial lever `R|cos v_here − cos v_hi| ~= R*u_r*delta`, inside the
///   zero band when `u_r*delta <= eps`.
///
/// Both hold simultaneously whenever `u_r <= eps/delta <= 1/K`, i.e. for
/// every rim within ~0.1 rad of a pole; the two rows below take a
/// factor-of-ten (`u_r = 1e-3`, `delta = 100 eps`) and a
/// factor-of-three (`u_r = 1e-2`, `delta = 30 eps`) safety margin on
/// both sides, and both hold at every eps this repo runs because both
/// sides scale with eps. `1e-2` is the largest polar angle pinned; the
/// binding ceiling is the band's own `K`, not the construction.
#[test]
fn the_latitude_margin_decides_at_a_near_polar_rim_where_the_axial_lever_cannot() {
    let (b, eps) = (band(), Tol::witness().get().eps);
    let (zero, escalate) = (b.zero(), b.escalate());
    for (u_r, k) in [(1e-3_f64, 100.0_f64), (1e-2, 30.0)] {
        let delta = k * eps;
        // The premise, stated in the open at this eps.
        assert!(
            delta.sin() >= escalate,
            "u_r={u_r}: the arc lever must be definite ({} < {escalate})",
            delta.sin()
        );
        let axial = (u_r.cos() - (u_r + delta).cos()).abs();
        assert!(
            axial <= zero,
            "u_r={u_r}: the axial lever must have collapsed ({axial} > {zero})"
        );
        let body = ball_with_a_near_polar_rim(u_r);
        let faces = sphere_faces(&body);
        assert_eq!(faces.len(), 2, "a full revolve mints two half-bands");
        let rim_polar = core::f64::consts::PI - u_r;
        for az in [0.7_f64, 2.4, 3.8, 5.5] {
            // Inside the rim (nearer the north pole): exactly one band
            // owns the azimuth, and it must say so DEFINITELY.
            let verdicts: Vec<_> = faces
                .iter()
                .map(|&f| {
                    topo::curved_face_containment(
                        &body,
                        f,
                        at(chart(&body, f), az, rim_polar - delta, 1.0),
                        b,
                    )
                    .unwrap()
                })
                .collect();
            assert_eq!(
                verdicts
                    .iter()
                    .filter(|v| **v == Some(FaceContainment::In))
                    .count(),
                1,
                "u_r={u_r}, az={az}: one band contains the probe just inside the rim, \
                 got {verdicts:?}"
            );
            // Past the rim, in the disc's own latitude range: BOTH bands
            // are definitely out, and nothing here is a graze.
            for &f in &faces {
                assert_eq!(
                    topo::curved_face_containment(
                        &body,
                        f,
                        at(chart(&body, f), az, rim_polar + delta, 1.0),
                        b
                    )
                    .unwrap(),
                    Some(FaceContainment::Out),
                    "u_r={u_r}, az={az}: past the rim is definitely outside"
                );
            }
        }
    }
}

/// The whole ball's two half-bands are the same class — meridian-bounded
/// lunes, half a period each — and the face door now answers on them
/// too, each for its own half. Exactly one band contains any given
/// off-seam point.
#[test]
fn the_balls_two_bands_each_answer_for_their_own_half() {
    let body = lune(Revolution::Full);
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 2, "a full revolve mints two half-bands");
    let b = band();
    for az in [0.3_f64, 1.9, 2.9, 4.1, 5.7] {
        let verdicts: Vec<_> = faces
            .iter()
            .map(|&f| {
                topo::curved_face_containment(&body, f, at(chart(&body, f), az, 1.1, 1.0), b)
                    .unwrap()
            })
            .collect();
        assert_eq!(
            verdicts
                .iter()
                .filter(|v| **v == Some(FaceContainment::In))
                .count(),
            1,
            "azimuth {az}: exactly one band contains it, got {verdicts:?}"
        );
    }
}

/// The SOLID door on a trimmed sphere body: a lune classifies its own
/// interior and its own boundary, where before the whole body was
/// refused as a partial sphere face.
///
/// The EXTERIOR is a different question and it stops one door further
/// on, for a reason that has nothing to do with the chart: a ray from
/// outside a quarter ball can miss the body entirely, and the verdict
/// is then the at-infinity side, read off the body's signed volume —
/// which the closed-form props lane will not certify for a rimless
/// band whose meridians lie on two different great circles. That
/// refusal is pinned here in its honest form, naming the volume rather
/// than reporting a healthy body as broken.
#[test]
fn the_solid_door_answers_inside_a_trimmed_sphere_body() {
    let body = lune(Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let (t, b) = (Tol::witness(), band());
    let ch = chart(&body, sphere_faces(&body)[0]);
    assert_eq!(
        point_in_solid(&body, at(ch, IN_AZ, 1.0, 0.5), b, t).unwrap(),
        SolidContainment::In,
        "inside the swept quarter, well within the ball"
    );
    assert_eq!(
        point_in_solid(&body, at(ch, IN_AZ, 1.0, 1.0), b, t).unwrap(),
        SolidContainment::OnBoundary,
        "on the sphere face"
    );
    let err = point_in_solid(&body, at(ch, OUT_AZ, 1.0, 0.5), b, t)
        .expect_err("the at-infinity side needs a volume props will not certify");
    assert!(
        matches!(err, PointInSolidError::VolumeUncertified),
        "{err:?}"
    );
    let msg = err.to_string();
    assert!(msg.contains("HEALTHY"), "{msg}");
    assert!(msg.contains("hardcodes"), "{msg}");
}

/// A trimmed sphere face with a RIM is the other half of the class, and
/// it is the one every cut sphere produces. A cap body — a full revolve
/// of a partial arc — has a rimmed sphere band, which the props lane
/// does certify, so the solid door answers in every direction.
#[test]
fn the_solid_door_answers_around_a_rimmed_sphere_band() {
    // A spherical CAP: a radial segment out from the axis at y = 1/2, a
    // 60-degree arc of the unit circle up to the north pole, and the
    // axis back down. No joint is tangent, so nothing needs declaring.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(geom_core::Point2::new(0.0, 0.5), 0.0),
        ProfileVertex::new(
            geom_core::Point2::new((0.75_f64).sqrt(), 0.5),
            (core::f64::consts::PI / 12.0).tan(),
        ),
        ProfileVertex::new(geom_core::Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: geom_core::Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    let (t, b) = (Tol::witness(), band());
    let ch = chart(&body, sphere_faces(&body)[0]);
    assert_eq!(
        point_in_solid(&body, at(ch, 0.4, 0.4, 0.95), b, t).unwrap(),
        SolidContainment::In,
        "just inside the cap's own spherical wall"
    );
    assert_eq!(
        point_in_solid(&body, at(ch, 0.4, 0.4, 1.05), b, t).unwrap(),
        SolidContainment::Out,
        "just outside it"
    );
    assert_eq!(
        point_in_solid(&body, at(ch, 0.4, 2.5, 0.95), b, t).unwrap(),
        SolidContainment::Out,
        "the same radius at a latitude the cap does not reach"
    );
}

/// The refusal that remains says what the chart NEEDS, not what the
/// pcurve lane once lacked. The retired blocker — "`chart_mints` is
/// false for sphere charts" — has been false since the analytic-chart
/// completion and must not come back.
#[test]
fn the_refusal_names_the_class_it_needs() {
    let msg = PointInSolidError::PartialSphereFace {
        face: FaceKey::default(),
    }
    .to_string();
    for want in [
        "latitude rim",
        "meridian great circle",
        "POLE strictly inside",
        "azimuth jumps by",
        "Recourse",
    ] {
        assert!(msg.contains(want), "missing {want:?}: {msg}");
    }
    assert!(!msg.contains("chart_mints"), "{msg}");
}
