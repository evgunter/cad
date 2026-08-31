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
    rimmed_ball(u_r, Revolution::Full)
}

/// The same construction at an arbitrary sweep: a unit ball whose one
/// latitude RIM sits at polar angle `pi - u_r`, swept through `turn`.
/// A PARTIAL sweep also mints the two meridian seam PLANES, whose
/// surface keys the planted rows below need for an adjacency-coherent
/// `Intersection` description.
fn rimmed_ball(u_r: f64, turn: Revolution<f64>) -> Body<f64> {
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
    revolve(&vp, axis, turn, Tol::witness()).unwrap().body
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
    // ONE planted point, inside a window with a measured floor at both
    // ends (the doc above). Everything below scales with eps.
    let (u_r, k) = (1e-2_f64, 30.0_f64);
    {
        let delta = k * eps;
        // The premise, stated in the open at this eps.
        assert!(
            delta.sin() >= escalate,
            "the arc lever must be definite ({} < {escalate})",
            delta.sin()
        );
        let axial = (u_r.cos() - (u_r + delta).cos()).abs();
        assert!(
            axial <= zero,
            "the axial lever must have collapsed ({axial} > {zero})"
        );
        // The constructive floor, measured rather than asserted: the
        // profile's radial segment meets the spherical arc at a
        // `carrier_line_circle` margin of about `u_r^2/2`, so a rim
        // nearer the pole than `sqrt(2*K*eps)` is refused as an
        // undeclared near-tangency. `1e-2` clears it at every eps the
        // suite runs (1e-6 -> 4.5e-3, 1e-9 -> 1.4e-4, 1e-12 -> 4.5e-6).
        assert!(
            u_r.powi(2) / 2.0 > escalate,
            "the fixture must clear its own construction floor"
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
                "az={az}: one band contains the probe just inside the rim, got {verdicts:?}"
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
                    "az={az}: past the rim is definitely outside"
                );
            }
        }
    }
}

/// The boundary-adjacent probes §7 asks for, on both boundary classes:
/// exactly AT a meridian and exactly AT a latitude rim.
///
/// The verdict there is not a graze but a NAMED incidence — the shared
/// boundary walk runs before the chart trim and answers `OnEdge(e)`
/// with the edge a split would cut, or `OnVertex` at a corner. The row
/// pins that, and pins that the edge named is the one the probe is
/// actually on, so a boundary answer cannot degrade into a coin-flip
/// interior verdict without the row noticing.
#[test]
fn probes_on_the_boundary_land_on_the_boundary_at_a_rim_and_at_a_meridian() {
    let b = band();
    // At a MERIDIAN: the lune's own two seam edges, at several
    // latitudes between the poles.
    let body = lune(Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let f = sphere_faces(&body)[0];
    let ch = chart(&body, f);
    let meridians: Vec<_> = boundary_edges(&body, f, true);
    assert_eq!(meridians.len(), 2, "a lune is bounded by two meridians");
    for az in [0.0_f64, core::f64::consts::FRAC_PI_2] {
        for polar in [0.05_f64, 0.9, 2.2, 3.0] {
            let v = topo::curved_face_containment(&body, f, at(ch, az, polar, 1.0), b).unwrap();
            let Some(FaceContainment::OnEdge(e)) = v else {
                panic!("on the meridian boundary at az={az}, polar={polar}: {v:?}");
            };
            assert!(
                meridians.contains(&e),
                "az={az}: names a meridian, got {e:?}"
            );
        }
    }
    // At a RIM: the rimmed ball's own latitude edge. Each band is asked
    // at the azimuths it owns, so the answer is its own boundary rather
    // than the other band's exterior.
    let rimmed = rimmed_ball(1.0, Revolution::Full);
    let rim_polar = core::f64::consts::PI - 1.0;
    let mut owned_seen = 0usize;
    for &f in &sphere_faces(&rimmed) {
        let ch = chart(&rimmed, f);
        let rims = boundary_edges(&rimmed, f, false);
        for az in [0.7_f64, 2.4, 3.8, 5.5] {
            let inside =
                topo::curved_face_containment(&rimmed, f, at(ch, az, rim_polar - 0.1, 1.0), b)
                    .unwrap();
            if inside != Some(FaceContainment::In) {
                continue; // the other band's azimuth
            }
            owned_seen += 1;
            let v =
                topo::curved_face_containment(&rimmed, f, at(ch, az, rim_polar, 1.0), b).unwrap();
            let Some(FaceContainment::OnEdge(e)) = v else {
                panic!("on the owning band's own rim at az={az}: {v:?}");
            };
            assert!(rims.contains(&e), "az={az}: names a rim, got {e:?}");
        }
    }
    assert_eq!(owned_seen, 4, "every azimuth is owned by exactly one band");
}

/// The face's boundary edges of one iso class: meridian great circles
/// (`meridian`) or latitude rims.
fn boundary_edges(body: &Body<f64>, face: FaceKey, meridian: bool) -> Vec<topo::EdgeKey> {
    let fd = body.get_face(face).unwrap();
    let axis = match body.get_surface(fd.surface) {
        Some(&geom::Surface::Sphere { axis, .. }) => axis,
        _ => panic!("a sphere face"),
    };
    let topo::LoopBoundary::Cycle { first } = body.get_loop(fd.outer).unwrap().boundary else {
        panic!("a cycle")
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .filter_map(|he| {
            let ed = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            let c = body
                .get_curve_geom(ed.curve)
                .and_then(topo::null::CurveGeom::certified)?;
            let geom::Curve3::Circle { axis: n, .. } = *c.carrier() else {
                return None;
            };
            ((n.dot(axis).abs() < 1e-9) == meridian).then(|| body.get_half_edge(he).unwrap().edge)
        })
        .collect()
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

// ---------------------------------------------------------------------
// The §7 class-remainder rows.
//
// Every row below plants a face the chart rectangle must REFUSE, and
// each is planted at the layer that can actually produce it. Three of
// the four are unreachable through a sweep constructor — a revolve mints
// iso-bounded sphere faces and nothing else, which is the whole point of
// the class — so they are built by re-attaching one datum of a revolved
// body through a PUBLIC door (`set_edge_curve`, `set_face_surface`,
// `kef`, `kfmrh`), never by hand-editing an arena. Each row states which
// door it used and pins the refusal at BOTH containment doors, because
// the two doors reach the trim by different routes.
// ---------------------------------------------------------------------

/// **The §7 pole-in-edge-interior refusal (#723's premise), planted.**
/// A meridian boundary edge whose span contains a POLE strictly inside
/// takes the face out of the chart class: latitude stops being monotone
/// along the edge, so no fold over boundary levels can see the face's
/// own extreme, and the azimuth image stops being a constant-azimuth
/// iso-line (it jumps by pi at the pole).
///
/// Planted by re-spanning ONE seam meridian the long way round — same
/// carrier circle, same two vertices, the arc that goes over the far
/// pole instead of the near side. The control is the same body one call
/// earlier, which answers definitely; the only difference between the
/// two is whether the pole is a vertex or interior to an edge.
#[test]
fn a_meridian_edge_with_a_pole_strictly_inside_refuses_at_both_doors() {
    let (b, t) = (band(), Tol::witness());
    let base = rimmed_ball(1.0, Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let f = sphere_faces(&base)[0];
    let ch = chart(&base, f);
    let (on, inside) = (at(ch, 0.4, 2.0, 1.0), at(ch, 0.4, 2.0, 0.5));
    // Control: the iso-bounded original is served at both doors.
    assert_eq!(
        topo::curved_face_containment(&base, f, on, b).unwrap(),
        Some(FaceContainment::In)
    );
    assert_eq!(
        point_in_solid(&base, inside, b, t).unwrap(),
        SolidContainment::In
    );

    let mut planted = base.clone();
    let (edge, carrier, plane_key) = seam_meridian(&planted, f);
    let geom::Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = carrier
    else {
        panic!("a meridian great circle")
    };
    // The SAME circle, traversed the other way: its seam still sits at
    // the pole, so the long arc pole -> far pole -> rim is the forward
    // one. (The attach door certifies a forward, sub-period span:
    // measured, a decreasing span refuses `IntervalNotForward` and a
    // span past a full turn refuses `WindingExceeded`, so the reversal
    // is not a convenience — it is the only spelling the door takes.)
    let long = geom::Curve3::Circle {
        center,
        axis: Vec3::zero() - axis,
        radius,
        u_ref,
    };
    let width = core::f64::consts::PI + 1.0;
    planted
        .set_edge_curve(
            edge,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1: base.get_face(f).unwrap().surface,
                    s2: plane_key,
                    witness: long.eval(width * 0.5),
                },
                carrier: long,
                param_start: 0.0,
                param_end: width,
            },
            t,
        )
        .expect("the long-way meridian arc certifies: same carrier, same endpoints");

    assert_eq!(
        topo::curved_face_containment(&planted, f, on, b).unwrap(),
        None,
        "the face door reports the honest remainder"
    );
    let err = point_in_solid(&planted, inside, b, t).expect_err("out of the chart class");
    assert!(
        matches!(err, PointInSolidError::PartialSphereFace { .. }),
        "{err:?}"
    );
    assert!(err.to_string().contains("POLE strictly inside"), "{err}");
}

/// **The §7 non-iso-bounded refusal, planted**, with its two-tolerance
/// twin. A boundary circle that is neither a latitude rim (axis parallel
/// to the polar axis) nor a meridian great circle (axis perpendicular,
/// centred at the sphere centre) is a face the `[azimuth] x [latitude]`
/// rectangle does not describe, and the trim says so.
///
/// Planted by tilting the disc face's PLANE and re-attaching the shared
/// rim edge as that plane's section of the sphere — the same two
/// vertices, a genuinely tilted circle through them. (A `Chart`
/// description will not carry it: measured, the attach door refuses
/// `ChartImageUnavailable { chart: "sphere", carrier: "circle" }` for a
/// general circle, which is the same fact the PR body states about a
/// meridian through a pole, met from the other side.)
///
/// The twin is the same construction at a tilt of `3*eps` radians, where
/// the class margin `|n x a|*r` lands in the ambiguity band and the door
/// ESCALATES on its own named predicate instead of guessing a class.
#[test]
fn a_boundary_circle_in_neither_iso_class_refuses_and_escalates_in_band() {
    let (b, t) = (band(), Tol::witness());
    let eps = t.get().eps;
    for (tilt, what) in [(0.5_f64, "definite"), (3.0 * eps, "in-band")] {
        let mut planted = rimmed_ball(1.0, Revolution::Partial(core::f64::consts::FRAC_PI_2));
        let f = sphere_faces(&planted)[0];
        let sph_key = planted.get_face(f).unwrap().surface;
        let ch = chart(&planted, f);
        let (edge, rim, _) = rim_edge(&planted, f);
        let (p0, p1) = (rim.eval(0.0), rim.eval(core::f64::consts::FRAC_PI_2));
        // The pencil of planes through the rim edge's two endpoints,
        // parameterised by the tilt away from the latitude plane.
        let e1 = Vec3::new(0.0, 1.0, 0.0);
        let e2 = (p1 - p0).normalize().cross(e1).normalize();
        let m = e1 * tilt.cos() + e2 * tilt.sin();
        let s0 = m.dot(p0 - Point3::origin());
        let c = Point3::origin() + m * s0;
        let rad = (1.0 - s0 * s0).sqrt();
        let u = (p0 - c) * (1.0 / rad);
        let t1 = (p1 - c).dot(m.cross(u)).atan2((p1 - c).dot(u));
        let tilted = geom::Curve3::Circle {
            center: c,
            axis: m,
            radius: rad,
            u_ref: u,
        };
        // The disc face carries the tilted circle's own plane, so the
        // pair the description names IS the edge's adjacent pair.
        let disc = flat_disc(&planted);
        let plane_key = planted
            .set_face_surface(
                disc,
                topo::FaceSurface::New(geom::Surface::Plane {
                    origin: c,
                    normal: m,
                    u_ref: u,
                }),
            )
            .unwrap();
        planted
            .set_edge_curve(
                edge,
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        s1: sph_key,
                        s2: plane_key,
                        witness: tilted.eval(t1 * 0.5),
                    },
                    carrier: tilted,
                    param_start: 0.0,
                    param_end: t1,
                },
                t,
            )
            .expect("a plane section of the sphere certifies");
        let (on, inside) = (at(ch, 0.4, 2.0, 1.0), at(ch, 0.4, 2.0, 0.5));
        if what == "definite" {
            assert_eq!(
                topo::curved_face_containment(&planted, f, on, b).unwrap(),
                None,
                "a tilted boundary circle is outside the served class"
            );
            let err = point_in_solid(&planted, inside, b, t).expect_err("out of the class");
            assert!(
                matches!(err, PointInSolidError::PartialSphereFace { .. }),
                "{err:?}"
            );
        } else {
            // The two-tolerance twin: the same question one band-width
            // away escalates, naming the predicate that could not
            // decide the class.
            let err = topo::curved_face_containment(&planted, f, on, b)
                .expect_err("the class margin is in the ambiguity band");
            assert!(
                format!("{err:?}").contains("bool_sphere_iso_rim"),
                "the escalation names its own predicate: {err:?}"
            );
        }
    }
}

/// **The §7 full-period-azimuth row**, and the one place the two
/// containment doors deliberately disagree. A sphere face that attains
/// EVERY azimuth has no azimuth window to be excluded by: the ray lane
/// serves it (the latitude window still describes it exactly), while
/// the face door keeps its typed frontier and answers `None`.
///
/// Planted by `kef` on one of a full ball's two seam meridians, which
/// merges the two half-bands into a single face whose boundary walk
/// carries a whole turn. The differential IS the row: the same body,
/// the same face, `None` at one door and a definite verdict at the
/// other, which is only possible if the trim returned a rectangle whose
/// azimuth half is `None` — `face_geo` would have refused
/// `PartialSphereFace` for any other reason the trim declines.
#[test]
fn a_full_period_azimuth_window_is_served_by_the_ray_lane_and_refused_by_the_face_door() {
    let (b, t) = (band(), Tol::witness());
    let mut planted = rimmed_ball(1.0, Revolution::Full);
    let seam = seam_meridian(&planted, sphere_faces(&planted)[0]).0;
    let he = planted.get_edge(seam).unwrap().he_plus;
    planted
        .kef(he)
        .expect("the two half-bands merge into one face");
    let faces = sphere_faces(&planted);
    assert_eq!(faces.len(), 1, "one sphere face spanning the whole period");
    let (f, ch) = (faces[0], chart(&planted, faces[0]));
    assert_eq!(
        topo::curved_face_containment(&planted, f, at(ch, 0.4, 2.0, 1.0), b).unwrap(),
        None,
        "the face door keeps its typed frontier at a full period"
    );
    assert_eq!(
        point_in_solid(&planted, at(ch, 0.4, 2.0, 0.5), b, t).unwrap(),
        SolidContainment::In,
        "the ray lane serves it — every azimuth is in the face"
    );
    assert_eq!(
        point_in_solid(&planted, at(ch, 0.4, 2.0, 1.5), b, t).unwrap(),
        SolidContainment::Out
    );
}

/// **The §7 ringed-sphere-face row.** A face with a ring is outside the
/// class at both doors and for the same reason at both: the rectangle
/// its outer boundary pins says nothing about the hole.
///
/// Planted by `kfmrh`, which re-homes the flat disc's loop as a RING of
/// the sphere face — the one public door that puts a ring on a curved
/// face at all.
#[test]
fn a_ringed_sphere_face_refuses_at_both_doors() {
    let (b, t) = (band(), Tol::witness());
    let mut planted = rimmed_ball(1.0, Revolution::Partial(core::f64::consts::FRAC_PI_2));
    let f = sphere_faces(&planted)[0];
    let ch = chart(&planted, f);
    let disc = flat_disc(&planted);
    planted
        .kfmrh(f, disc)
        .expect("the disc's loop re-homes as a ring");
    assert_eq!(planted.get_face(f).unwrap().rings.len(), 1);
    assert_eq!(
        topo::curved_face_containment(&planted, f, at(ch, 0.4, 2.0, 1.0), b).unwrap(),
        None
    );
    let err = point_in_solid(&planted, at(ch, 0.4, 2.0, 0.5), b, t).expect_err("ringed");
    assert!(
        matches!(err, PointInSolidError::PartialSphereFace { .. }),
        "{err:?}"
    );
    assert!(err.to_string().contains("ring"), "{err}");
}

/// The body's one flat disc face — the revolve's cap, whose plane's
/// normal is the polar axis.
fn flat_disc(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, fd)| {
            matches!(
                body.get_surface(fd.surface),
                Some(geom::Surface::Plane { normal, .. }) if normal.y.abs() > 0.9
            )
        })
        .map(|(k, _)| k)
        .expect("the revolve's flat cap")
}

/// The sphere face's azimuth-0 meridian seam edge, its carrier, and the
/// seam PLANE's surface key.
fn seam_meridian(
    body: &Body<f64>,
    face: FaceKey,
) -> (topo::EdgeKey, geom::Curve3<f64>, topo::SurfaceKey) {
    boundary_pick(body, face, true)
}

fn rim_edge(
    body: &Body<f64>,
    face: FaceKey,
) -> (topo::EdgeKey, geom::Curve3<f64>, topo::SurfaceKey) {
    boundary_pick(body, face, false)
}

fn boundary_pick(
    body: &Body<f64>,
    face: FaceKey,
    meridian: bool,
) -> (topo::EdgeKey, geom::Curve3<f64>, topo::SurfaceKey) {
    let fd = body.get_face(face).unwrap();
    let axis = match body.get_surface(fd.surface) {
        Some(&geom::Surface::Sphere { axis, .. }) => axis,
        _ => panic!("sphere"),
    };
    let topo::LoopBoundary::Cycle { first } = body.get_loop(fd.outer).unwrap().boundary else {
        panic!("cycle")
    };
    for he in body.loop_cycle(first).unwrap() {
        let hed = body.get_half_edge(he).unwrap();
        let ed = body.get_edge(hed.edge).unwrap();
        let Some(c) = body
            .get_curve_geom(ed.curve)
            .and_then(topo::null::CurveGeom::certified)
        else {
            continue;
        };
        let geom::Curve3::Circle { axis: n, .. } = *c.carrier() else {
            continue;
        };
        let is_meridian = n.dot(axis).abs() < 1e-9;
        if is_meridian != meridian {
            continue;
        }
        // Only the half-edge whose he_plus starts at the pole is usable
        // for a re-span, so prefer the edge whose he_plus is in THIS
        // loop.
        if meridian && ed.he_plus != he {
            continue;
        }
        let mate = if ed.he_plus == he {
            ed.he_minus
        } else {
            ed.he_plus
        };
        let ml = body.get_half_edge(mate).unwrap().parent_loop;
        let mf = body.get_loop(ml).unwrap().face;
        let key = body.get_face(mf).unwrap().surface;
        return (hed.edge, c.carrier().clone(), key);
    }
    panic!("no such boundary edge")
}
