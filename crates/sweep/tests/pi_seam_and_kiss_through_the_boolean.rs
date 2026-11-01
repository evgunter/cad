//! **What a π seam between distinct carriers, a cap abutting a tube on
//! its rim, and a coaxial kiss, do through the boolean.** The seam and
//! the transverse rims build; every other row is a refusal, pinned by
//! kind.
//!
//! - **The sphere-capped tube** (cylinder `z ∈ [0, h]` ∪ hemisphere of
//!   the same radius on its top cap): the two walls meet G1 along the
//!   rim circle, wedge π. The boolean verifies the walls' seam by its
//!   witness, and the union BUILDS in both orders: the tube and the
//!   half ball, the rim minted `TangentIntersection` — the same body as
//!   the capsule revolved from one profile with its joint authored.
//!   With the hemisphere turned about the axis it builds the same body;
//!   a hair from aligned, the tube's seam ruling keeps the covered line
//!   rung's door.
//! - **A cap abutting the tube on its rim** answers by its corner. A
//!   transverse 45° spherical dome BUILDS, its seams aligned with the
//!   tube's or not: each rim circle lies on the other operand's wall,
//!   and its parents are decided distinct from that wall, so it is an
//!   ON event, and the two rims are one seam of the zip. A tube revolved
//!   rather than extruded, and an inverted dome in the tube's place (a
//!   lens), build the same way. A rim offset in band of the partner's
//!   wall escalates; offset inside the zero band, it answers alike in
//!   both member orders.
//! - **A tube ending on a ball, or on a torus's 45° latitude**: the rim
//!   lies inside the partner's face rather than on its boundary, and
//!   passes the crossing layer the same way; the union stops in the
//!   join. A same-radius stacked cylinder continues the tube's carrier
//!   and builds. A cone frustum on the rim builds as the dome does: its
//!   base rim lies on the cone and on the tube's wall at once, and the
//!   carrier ladder decides the cone distinct from the wall.
//! - **Tube ∪ ball** (overlapping, ball centred on the top cap) stops at
//!   the crossing layer.
//! - **The stadium** (a slab whose top and bottom are tangent to a
//!   rod's wall along a ruling, the rod half inside it) answers every op.
//!   The D-bar on the slab, whose half-rod wall ends at the line,
//!   verifies as a line seam and builds.
//! - **The dodge plate** (a plate whose outline leaves a line on one
//!   side and then reaches round to the other): the line seam is read
//!   where the faces leave the line, not off the boundary.
//! - **The kiss** (a ball seated in a bore of its own radius): the
//!   equator touches the bore wall, wedge 2π, and it stops at the
//!   crossing layer.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick, finished, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, FaceKey};

const R: f64 = 1.0;
const H: f64 = 2.0;

fn faces_of(b: &Body<f64>, kind: SurfaceKind) -> Vec<FaceKey> {
    b.faces()
        .filter(|(_, f)| b.get_surface(f.surface).map(geom::Surface::kind) == Some(kind))
        .map(|(k, _)| k)
        .collect()
}

fn planes_at_z(b: &Body<f64>, z: f64) -> Vec<FaceKey> {
    b.faces()
        .filter(|(_, f)| {
            matches!(
                b.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-9 && normal.z.abs() > 0.5
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The union both ways round, and both refusals.
fn union_both_orders(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [BooleanError; 2] {
    let run = |x: &AtRestBody<f64>, y: &AtRestBody<f64>| match topo::union(x, y, Tol::witness()) {
        Err(e) => e,
        Ok(BooleanResult::Body(_)) => panic!("the union built a body"),
        Ok(BooleanResult::Empty) => panic!("the union came back empty"),
    };
    [run(a, b), run(b, a)]
}

/// A z-axis cylinder of radius `r` from `z0`, length `len`, through the
/// extrude door.
fn rod_z(r: f64, z0: f64, len: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    finished("the rod", rod, tol)
}

/// A revolved cap standing on `z = H`: the profile (sketch x radial,
/// sketch y axial from the cap's base) revolved, turned onto `+z`,
/// lifted, and its base disc's two revolve halves merged (the boolean
/// refuses a non-maximal operand).
fn cap_on_the_tube(profile: Vec<(Point2<f64>, f64)>) -> AtRestBody<f64> {
    placed_on_the_tube(revolved_about_y(profile, Revolution::Full, Tol::witness()))
}

/// [`cap_on_the_tube`]'s placement of a cap built about `+y`.
fn placed_on_the_tube(at0: Body<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let turned =
        topo::transform_rigid(&at0, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    let mut cap = topo::transform_rigid(
        &turned,
        &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, H)).unwrap(),
        tol,
    )
    .unwrap();
    cap.merge_coplanar_faces(tol).unwrap();
    finished("the cap", cap, tol)
}

/// A solid hemisphere of radius [`R`] standing on `z = H`.
fn hemisphere_on_the_cap() -> AtRestBody<f64> {
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    cap_on_the_tube(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), bulge),
        (Point2::new(0.0, R), 0.0),
    ])
}

/// A 45° cone frustum standing on `z = H`: base radius [`R`], top
/// radius `R / 2` at height `R / 2`. Its wall meets the tube's along
/// the rim circle at a 45° corner, not tangentially.
fn frustum_on_the_cap() -> AtRestBody<f64> {
    cap_on_the_tube(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), 0.0),
        (Point2::new(R / 2.0, R / 2.0), 0.0),
        (Point2::new(0.0, R / 2.0), 0.0),
    ])
}

/// A spherical cap of radius `√2·R` standing on `z = H`, its centre on
/// the axis `R` below the cap's base, so the base circle is the tube's
/// rim and the sphere meets the tube's wall there at 45°.
fn dome_on_the_cap() -> AtRestBody<f64> {
    let bulge = (core::f64::consts::FRAC_PI_4 / 4.0).tan();
    cap_on_the_tube(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), bulge),
        (Point2::new(0.0, (2.0_f64.sqrt() - 1.0) * R), 0.0),
    ])
}

fn is_pierce(e: &BooleanError) -> bool {
    matches!(e, BooleanError::CurvedPierceUnsupported { .. })
}

/// The tube and a cap, unioned both ways round.
fn unions_both_ways(
    tube: &AtRestBody<f64>,
    cap: &AtRestBody<f64>,
) -> [Result<BooleanResult<f64>, BooleanError>; 2] {
    let tol = Tol::witness();
    [topo::union(tube, cap, tol), topo::union(cap, tube, tol)]
}

/// The tube and a cap whose walls meet the tube's along the rim in a
/// seam: the union builds both ways round.
fn seam_builds(label: &str, tube: &AtRestBody<f64>, cap: &AtRestBody<f64>) {
    for (order, r) in unions_both_ways(tube, cap).into_iter().enumerate() {
        assert!(
            matches!(r, Ok(BooleanResult::Body(_))),
            "{label}, order {order}: the seam builds: {r:?}"
        );
    }
}

/// The certified carrier of `edge` in `body`.
fn carrier_of(body: &Body<f64>, edge: topo::EdgeKey) -> geom::Curve3<f64> {
    body.get_curve_geom(body.get_edge(edge).expect("a live edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified edge")
        .carrier()
        .clone()
}

/// The tube's volume plus the dome's spherical cap: height
/// `(√2 − 1)·R` on a sphere of radius `√2·R`.
fn tube_and_dome_volume() -> f64 {
    let rise = (2.0_f64.sqrt() - 1.0) * R;
    PI * R * R * H + PI * rise * rise * (3.0 * 2.0_f64.sqrt() * R - rise) / 3.0
}

/// The tube's volume plus the half ball on its top cap.
fn tube_and_half_ball_volume() -> f64 {
    PI * R * R * H + 2.0 / 3.0 * PI * R.powi(3)
}

/// The edges of `b` described `TangentIntersection`.
fn tangent_intersections(b: &Body<f64>) -> Vec<topo::EdgeKey> {
    b.edges()
        .filter(|(_, e)| {
            matches!(
                b.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(|c| c.description()),
                Some(geom_brep::EdgeDescription::TangentIntersection { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The sphere-capped tube builds**, its walls a seam and its discs
/// resting on each other, in both member orders. The seam is verified
/// along the rim circle (the `Tangent` witness lane, B's sense
/// reversed), the material wedge routes it to the smooth seam, and the
/// cover it gives lets each edge leaving the rim record its end there:
/// the tube's rulings lie outside the sphere, the cap's meridians
/// inside the tube's carrier.
///
/// The body is the tube and the half ball, exactly: the two discs
/// removed as interior, the rim minted once as two `TangentIntersection`
/// semicircles, and nothing else of either operand touched. It is the
/// same body, census and volume, as the capsule revolved from ONE
/// profile with its joint authored
/// ([`a_g1_joint_authored_inside_one_profile_is_the_seams_union`]).
#[test]
fn the_sphere_capped_tube_builds_as_a_seam() {
    let tol = Tol::witness();
    let (tube, hemi) = (rod_z(R, 0.0, H), hemisphere_on_the_cap());
    let want = tube_and_half_ball_volume();
    for (label, x, y) in [("tube ∪ cap", &tube, &hemi), ("cap ∪ tube", &hemi, &tube)] {
        let r = topo::union(x, y, tol);
        let body = match &r {
            Ok(BooleanResult::Body(b)) => b.body.clone(),
            other => panic!("{label}: builds: {other:?}"),
        };
        let (v, c, records) = built(label, r);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: the tube and the half ball: {v} vs {want}"
        );
        assert_eq!(
            c,
            (5, 8, 5, 1),
            "{label}: two walls, two sphere faces, the floor disc"
        );
        assert_eq!(records, [0; 6], "{label}: no contact survives a union");
        let rims = tangent_intersections(&body);
        assert_eq!(
            rims.len(),
            2,
            "{label}: the rim's two semicircles: {rims:?}"
        );
        for k in rims {
            let geom::Curve3::Circle { center, radius, .. } = carrier_of(&body, k) else {
                panic!("{label}: the rim is a circle");
            };
            assert!(
                (center - Point3::new(0.0, 0.0, H)).norm() < 1e-12 && (radius - R).abs() < 1e-12,
                "{label}: on the rim at z = H: {center:?}, {radius}"
            );
        }
    }
    // ∖ and ∩ stop before any crossing, at the fallback's containment
    // read of the sphere's section against the disc: filed as
    // `a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent`.
    for (op, r) in [
        ("tube ∖ cap", topo::subtract(&tube, &hemi, tol)),
        ("cap ∖ tube", topo::subtract(&hemi, &tube, tol)),
        ("tube ∩ cap", topo::intersect(&tube, &hemi, tol)),
    ] {
        assert!(
            matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
            "{op}: {r:?}"
        );
    }
}

/// `body` turned about the axis by `deg` degrees.
fn turned(body: &AtRestBody<f64>, deg: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), deg.to_radians());
    finished(
        "the turned operand",
        topo::transform_rigid(body, &geom_core::test_support::rigid(&spin, tol), tol).unwrap(),
        tol,
    )
}

/// The tube and the half ball on its top cap, as a closed form: inside
/// (`true`) or outside, or `None` within `1e-6` of its boundary.
fn in_tube_and_half_ball(p: Point3<f64>) -> Option<bool> {
    let radial = (p.x * p.x + p.y * p.y).sqrt();
    let tube = (radial - R).max(-p.z).max(p.z - H);
    let ball = ((p - Point3::new(0.0, 0.0, H)).norm() - R).max(H - p.z);
    let d = tube.min(ball);
    (d.abs() > 1e-6).then_some(d < 0.0)
}

/// **The sphere-capped tube builds with its hemisphere turned about the
/// axis**, in both member orders and with the pair spun. Turned, each rim semicircle of one
/// operand ends inside a rim arc of the other, so the rim is four arcs.
/// Each tube seam ruling ends on the rim beside a sphere face it does
/// not touch, and that face's reach over its azimuth window clears the
/// pair. The body is the tube and the half ball at every turn, by
/// volume and by point membership against the closed form. ∖ and ∩ stop
/// at the fallback extent at every turn, as they do aligned
/// (`a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent`).
#[test]
fn the_sphere_capped_tube_builds_with_its_hemisphere_turned() {
    let tol = Tol::witness();
    let hemisphere = hemisphere_on_the_cap();
    let want = tube_and_half_ball_volume();
    let lattice: Vec<Point3<f64>> = (0..5)
        .flat_map(|i| (0..5).flat_map(move |j| (0..7).map(move |k| (i, j, k))))
        .map(|(i, j, k)| {
            let step = |n: i32, lo: f64, hi: f64, of: i32| {
                lo + (hi - lo) * (f64::from(n) + 0.37) / f64::from(of)
            };
            Point3::new(
                step(i, -1.3, 1.3, 5),
                step(j, -1.3, 1.3, 5),
                step(k, -0.3, H + 1.3, 7),
            )
        })
        .collect();
    // `(the pair's spin, the hemisphere's turn on the tube)`, degrees.
    let poses = [
        (0.0, 7.0),
        (0.0, 30.0),
        (0.0, 45.0),
        (0.0, 90.0),
        (0.0, 173.0),
        (45.0, 30.0),
        (70.0, 90.0),
        (0.0, 0.0),
    ];
    for (spin, deg) in poses {
        let tube = turned(&rod_z(R, 0.0, H), spin);
        let hemi = turned(&hemisphere, spin + deg);
        let rims = if deg == 0.0 {
            (5, 8, 5, 1)
        } else {
            (5, 10, 7, 1)
        };
        for (order, x, y) in [("tube ∪ cap", &tube, &hemi), ("cap ∪ tube", &hemi, &tube)] {
            let label = format!("spun {spin}°, turned {deg}°, {order}");
            let r = topo::union(x, y, tol);
            let body = match &r {
                Ok(BooleanResult::Body(b)) => b.body.clone(),
                other => panic!("{label}: builds: {other:?}"),
            };
            let (v, c, records) = built(&label, r);
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the tube and the half ball: {v} vs {want}"
            );
            assert_eq!(c, rims, "{label}: F, E, V, shells");
            assert_eq!(records, [0; 6], "{label}: no contact survives a union");
            let mut decided = 0;
            for &q in &lattice {
                let Some(inside) = in_tube_and_half_ball(q) else {
                    continue;
                };
                decided += 1;
                let got = topo::point_in_solid(&body, q, Band::linear(tol).unwrap(), tol)
                    .unwrap_or_else(|e| panic!("{label}: membership at {q:?}: {e:?}"));
                let want = if inside {
                    topo::SolidContainment::In
                } else {
                    topo::SolidContainment::Out
                };
                assert_eq!(got, want, "{label}: membership at {q:?}");
            }
            assert!(decided > 150, "{label}: the lattice reads {decided} points");
        }
        for (op, r) in [
            ("tube ∖ cap", topo::subtract(&tube, &hemi, tol)),
            ("cap ∖ tube", topo::subtract(&hemi, &tube, tol)),
            ("tube ∩ cap", topo::intersect(&tube, &hemi, tol)),
            ("cap ∩ tube", topo::intersect(&hemi, &tube, tol)),
        ] {
            assert!(
                matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
                "spun {spin}°, turned {deg}°, {op}: {r:?}"
            );
        }
    }
}

/// **Turned within a hair of aligned**, the sphere-capped tube answers
/// by how far its rim vertices sit apart. Inside the zero band it is the
/// aligned body. Where the rim vertices sit in the sliver band apart,
/// the glue door reads the shared rim and escalates the seam cover's
/// order there, in both operand orders. From there to about
/// `√(70·ε·R)` apart, the tube's seam ruling, whose top
/// end lies on the hemisphere's rim just outside one sphere face, keeps
/// the covered line rung's door against that face
/// (`a-covered-line-ending-just-off-the-face-keeps-the-door`): the
/// face's reach clears the ruling by `R·(1 − cos θ)`, which is inside
/// the sweep's pad. Further apart the reach clears the pair, and it
/// builds.
#[test]
fn the_sphere_capped_tube_turned_within_a_hair_of_aligned() {
    enum Expect {
        Builds((usize, usize, usize, usize)),
        RulingDoor,
        /// The glue door's rim routing escalates.
        RimDoor,
    }
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (zero, escalate) = (band.zero(), band.escalate());
    let (tube, hemisphere) = (rod_z(R, 0.0, H), hemisphere_on_the_cap());
    let want = tube_and_half_ball_volume();
    // How far apart the rim vertices sit. The seam cover's decisions
    // read it, or half of it; the reach clears the ruling by
    // `apart² / 2R`, against the sweep's pad `escalate + 2·zero` on each
    // side.
    for (apart, expect) in [
        (0.01 * zero, Expect::Builds((5, 8, 5, 1))),
        (2.0 * (zero * escalate).sqrt(), Expect::RimDoor),
        (5.0 * escalate, Expect::RulingDoor),
        ((R * escalate).sqrt(), Expect::RulingDoor),
        (-(R * escalate).sqrt(), Expect::RulingDoor),
        (
            (2000.0 * R * escalate).sqrt(),
            Expect::Builds((5, 10, 7, 1)),
        ),
    ] {
        let deg = (apart / R).to_degrees();
        let hemi = turned(&hemisphere, deg);
        for (order, x, y, tube_is) in [
            ("tube ∪ cap", &tube, &hemi, topo::Operand::A),
            ("cap ∪ tube", &hemi, &tube, topo::Operand::B),
        ] {
            let label = format!("turned {deg}°, {order}");
            let r = topo::union(x, y, tol);
            match expect {
                Expect::Builds(rims) => {
                    let (v, c, _) = built(&label, r);
                    assert!(
                        (v - want).abs() <= 1e-12 * want,
                        "{label}: the tube and the half ball: {v} vs {want}"
                    );
                    assert_eq!(c, rims, "{label}: F, E, V, shells");
                }
                Expect::RimDoor => {
                    let Err(BooleanError::Escalated { diag, .. }) = &r else {
                        panic!("{label}: the rim routing escalates: {r:?}");
                    };
                    assert_eq!(
                        diag.predicate,
                        Some("seam_cover_order"),
                        "{label}: the seam cover's order: {r:?}"
                    );
                }
                Expect::RulingDoor => {
                    let Err(BooleanError::CurvedPierceUnsupported { operand, edge, .. }) = r else {
                        panic!("{label}: the covered line rung's door: {r:?}");
                    };
                    assert_eq!(operand, tube_is, "{label}: an edge of the tube");
                    assert!(
                        matches!(carrier_of(&tube, edge), geom::Curve3::Line { .. }),
                        "{label}: the tube's seam ruling"
                    );
                }
            }
        }
    }
}

/// **The rim is the seam the boolean verifies**, in both member orders:
/// the fixture is the half ball standing on the tube's top cap, and the
/// union builds ([`the_sphere_capped_tube_builds_as_a_seam`]).
#[test]
fn the_sphere_capped_tube_is_its_seam() {
    let tube = rod_z(R, 0.0, H);
    let hemi = hemisphere_on_the_cap();
    let v = topo::test_support::pinned_volume("the measured body", &hemi, Tol::witness()).volume;
    let half_ball = 2.0 / 3.0 * PI * R.powi(3);
    assert!(
        (v - half_ball).abs() <= 1e-12 * half_ball,
        "the hemisphere fixture is the half ball above the cap: {v} vs {half_ball}"
    );
    let top = hemi
        .points()
        .map(|(_, p)| p.z)
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (top - (H + R)).abs() < 1e-12,
        "it stands on z = H: top at {top}"
    );
    let cyl = faces_of(&tube, SurfaceKind::Cylinder);
    let sph = faces_of(&hemi, SurfaceKind::Sphere);
    let (cap_t, cap_h) = (planes_at_z(&tube, H), planes_at_z(&hemi, H));
    assert_eq!((cap_t.len(), cap_h.len()), (1, 1), "one cap disc each");

    assert_eq!((cyl.len(), sph.len()), (2, 2), "two wall faces each");
    seam_builds("hemisphere", &tube, &hemi);
}

#[test]
fn a_dome_abutting_on_the_rim_at_a_transverse_corner_builds() {
    let tol = Tol::witness();
    let tube = rod_z(R, 0.0, H);
    let dome = dome_on_the_cap();
    let rise = (2.0_f64.sqrt() - 1.0) * R;
    let dome_v = PI * rise * rise * (3.0 * 2.0_f64.sqrt() * R - rise) / 3.0;
    let v = topo::test_support::pinned_volume("the measured body", &dome, tol).volume;
    assert!(
        (v - dome_v).abs() <= 1e-12 * dome_v,
        "the dome fixture is the spherical cap on the rim: {v} vs {dome_v}"
    );
    // Turned a twelfth of a turn about the axis, the dome's seams miss
    // the tube's, so each rim runs along two of the other's arcs.
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), PI / 6.0);
    let turned = finished(
        "the turned dome",
        topo::transform_rigid(&dome, &geom_core::test_support::rigid(&turn, tol), tol).unwrap(),
        tol,
    );
    // The same tube revolved: its wall's seam is a meridian ruling.
    let revolved = cap_on_the_tube(vec![
        (Point2::new(0.0, -H), 0.0),
        (Point2::new(R, -H), 0.0),
        (Point2::new(R, 0.0), 0.0),
        (Point2::new(0.0, 0.0), 0.0),
    ]);
    let want = tube_and_dome_volume();
    // Each rim's two semicircles are one seam each; turned, each runs
    // along two of the other's arcs, splitting at their seam vertices.
    for (label, tube, cap, want_census) in [
        ("dome", &tube, &dome, (5, 8, 5, 1)),
        ("dome turned", &tube, &turned, (5, 10, 7, 1)),
        ("revolved tube", &revolved, &dome, (5, 8, 5, 1)),
    ] {
        for (order, r) in unions_both_ways(tube, cap).into_iter().enumerate() {
            let b = match r {
                Ok(BooleanResult::Body(b)) => b.body,
                other => panic!("{label}, order {order}: the union builds: {other:?}"),
            };
            topo::validate_geometric(&b, tol)
                .unwrap_or_else(|e| panic!("{label}, order {order}: tier 3: {e:?}"));
            let v = topo::test_support::pinned_volume("the measured body", &b, tol).volume;
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}, order {order}: the tube and the cap: {v} vs {want}"
            );
            assert_eq!(
                census(&b),
                want_census,
                "{label}, order {order}: F, E, V, shells"
            );
            assert!(
                planes_at_z(&b, H).is_empty(),
                "{label}, order {order}: the abutting discs are consumed"
            );
            assert_eq!(
                (
                    faces_of(&b, SurfaceKind::Cylinder).is_empty(),
                    faces_of(&b, SurfaceKind::Sphere).is_empty(),
                ),
                (false, false),
                "{label}, order {order}: the tube's wall meets the dome's at the rim"
            );
        }
    }
}

/// **A lens**: the dome on an inverted dome of the same rim, its discs
/// resting on each other. Both rims lie on the other's sphere, at a 90° corner.
/// It builds with the two domes' seams aligned, and turned: each dome rim
/// semicircle then runs along parts of two bowl arcs, and the reduction
/// splits it at the bowl's rim vertex between them.
#[test]
fn a_lens_of_two_domes_builds() {
    let tol = Tol::witness();
    let dome = dome_on_the_cap();
    let rise = (2.0_f64.sqrt() - 1.0) * R;
    let bowl = cap_on_the_tube(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (
            Point2::new(0.0, -rise),
            (core::f64::consts::FRAC_PI_4 / 4.0).tan(),
        ),
        (Point2::new(R, 0.0), 0.0),
    ]);
    let volume = |b: &Body<f64>| {
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(b), tol),
            tol,
        )
        .volume
    };
    let want = 2.0 * volume(&dome);
    assert!(
        (volume(&bowl) - volume(&dome)).abs() <= 1e-12 * want,
        "the bowl is the dome mirrored"
    );
    for (order, r) in unions_both_ways(&bowl, &dome).into_iter().enumerate() {
        let b = match r {
            Ok(BooleanResult::Body(b)) => b.body,
            other => panic!("order {order}: the lens builds: {other:?}"),
        };
        topo::validate_geometric(&b, tol)
            .unwrap_or_else(|e| panic!("order {order}: tier 3: {e:?}"));
        assert_eq!(census(&b), (4, 6, 4, 1), "order {order}: F, E, V, shells");
        assert!(
            (volume(&b) - want).abs() <= 1e-12 * want,
            "order {order}: the two domes: {} vs {want}",
            volume(&b)
        );
        assert!(
            planes_at_z(&b, H).is_empty(),
            "order {order}: the discs are consumed"
        );
    }
    for angle in [PI / 7.0, PI / 2.0] {
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), angle);
        let turned = finished(
            "the turned dome",
            topo::transform_rigid(&dome, &geom_core::test_support::rigid(&turn, tol), tol).unwrap(),
            tol,
        );
        for (order, r) in unions_both_ways(&bowl, &turned).into_iter().enumerate() {
            let label = format!("turned {angle}, order {order}");
            let (v, c, k) = built(&label, r);
            assert_eq!(k, [0; 6], "{label}: no contact records");
            // Each dome's two rim vertices and pole: minimal.
            assert_eq!(c, (4, 8, 6, 1), "{label}: F, E, V, shells");
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the two domes: {v} vs {want}"
            );
        }
    }
}

/// **A rim lying inside the partner's face** passes the crossing layer
/// and builds: a tube standing on a torus's 45° latitude (`R = 2`,
/// `r = 1`, the tube's radius `2 + 1/√2` from `z = 1/√2`) unions in both
/// member orders to the torus and the tube less the torus's cap above
/// the tube's floor, which the tube holds whole (`2πR·(π/4 − 1/2)`),
/// valid at tier 3 and 3′. A tube ending on a ball builds
/// (`a_tube_ending_on_a_ball.rs`).
#[test]
fn a_rim_inside_the_partners_face_passes_the_crossing_layer() {
    let tol = Tol::witness();
    let at0 = revolved_about_y(
        vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut torus =
        topo::transform_rigid(&at0, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    torus.merge_coplanar_faces(tol).unwrap();
    let torus = finished("the torus", torus, tol);
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let tube = rod_z(2.0 + s, s, 2.0);
    let cap = 2.0 * PI * 2.0 * (PI / 4.0 - 0.5);
    let want = 4.0 * PI * PI + PI * (2.0 + s).powi(2) * 2.0 - cap;
    for (order, r) in [
        topo::union(&tube, &torus, tol),
        topo::union(&torus, &tube, tol),
    ]
    .into_iter()
    .enumerate()
    {
        let (v, ..) = built(&format!("order {order}"), r);
        assert!(
            (v - want).abs() <= want * 1e-9_f64.max(1e3 * tol.eps()),
            "order {order}: the union's volume {v} against {want}"
        );
    }
}

/// A rim whose radius is in band of the partner's wall lies on it
/// neither exactly nor definitely off it: the run escalates.
#[test]
fn a_rim_in_band_of_the_partners_wall_escalates() {
    let band = Band::linear(Tol::witness()).unwrap();
    let tube = rod_z(R + (band.zero() + band.escalate()) / 2.0, 0.0, H);
    let dome = dome_on_the_cap();
    for (order, r) in unions_both_ways(&tube, &dome).into_iter().enumerate() {
        assert!(
            matches!(r, Err(BooleanError::Escalated { .. })),
            "order {order}: an in-band rim escalates: {:?}",
            r.err()
        );
    }
}

/// **A rim offset by `k` zero bands** (the tube's radius `R + k·zero`),
/// unioned with the dome both ways round: each member
/// order answers alike at every offset.
///
/// - `k ∈ {0, ±¼, ±½}`: both build, the census of the exact abutment,
///   and each order's volume within `slack(k)` of the closed form, the
///   two orders within `slack(k)` of each other. The rim keeps the
///   first operand's circle (the copy the join keeps), and the merge
///   certifies the second operand's row against it: on the tube's wall
///   in one order, on the dome's sphere in the other. Each row's
///   envelope reads the offset once, so both clear the zero band. The
///   kept rim stands off the other operand's surface by `|k|·zero`,
///   and the volume moves with it: `slack(k) = (|k|·zero/R + 64ε)·V`.
///   Measured at the default ε, order 0 sits at `0.3` of the first term
///   and order 1 at `0.6`; at ε = 1e-12, order 0 sits at `0` (the fan
///   reads the kept rim on its own circle's sphere) and order 1 at about
///   `0.6`. A `1e-9` relative error in either reds the row at the
///   default ε and at 1e-12. At a negative `k` the dome's disc overhangs
///   the tube's by a lamina of no volume.
/// - `k = ±0.9`: the dome's meridian meets the tube's wall `√2·|k|·zero`
///   along its arc from the rim, and the crossing layer escalates on that
///   arc length, both orders
///   (`work/cleave/boolean-in-span-readings-of-grazing-roots-are-levered-by-arc-length.md`).
/// - `k = ±1.1, ±2`: in band, both orders escalate. Which question
///   escalates first follows the operand walked first.
#[test]
fn a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let dome = dome_on_the_cap();
    let rim_radius = |b: &Body<f64>| -> Vec<f64> {
        b.edges()
            .filter_map(|(e, _)| match carrier_of(b, e) {
                geom::Curve3::Circle { center, radius, .. } if (center.z - H).abs() < 1e-9 => {
                    Some(radius)
                }
                _ => None,
            })
            .collect()
    };
    let dome_rim = rim_radius(&dome)[0];
    let cap = tube_and_dome_volume() - PI * R * R * H;
    for k in [0.0, 0.25, -0.25, 0.5, -0.5, 0.9, -0.9, 1.1, -1.1, 2.0, -2.0] {
        let tube = rod_z(R + k * band.zero(), 0.0, H);
        let tube_rim = rim_radius(&tube)[0];
        let want = PI * tube_rim * tube_rim * H + cap;
        let slack = (k.abs() * band.zero() / R + 64.0 * f64::EPSILON) * want;
        let mut built = Vec::new();
        for (order, r) in unions_both_ways(&tube, &dome).into_iter().enumerate() {
            let label = format!("k = {k}, order {order}");
            if (-0.5..=0.5).contains(&k) {
                let b = match r {
                    Ok(BooleanResult::Body(b)) => b.body,
                    other => panic!("{label}: the union builds: {other:?}"),
                };
                topo::validate_geometric(&b, tol)
                    .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
                assert_eq!(census(&b), (5, 8, 5, 1), "{label}: F, E, V, shells");
                let kept = [tube_rim, dome_rim][order];
                assert_eq!(
                    rim_radius(&b),
                    vec![kept; 2],
                    "{label}: the rim keeps the first operand's circle"
                );
                built.push((
                    label,
                    topo::test_support::pinned_volume("the measured body", &b, tol).volume,
                ));
                continue;
            }
            let e = r.err().unwrap_or_else(|| panic!("{label}: refuses"));
            match k {
                0.9 | -0.9 => assert!(
                    matches!(
                        &e,
                        BooleanError::Escalated { diag, .. }
                            if diag.predicate == Some("bool_wall_root_in_span")
                    ),
                    "{label}: the meridian's root, by arc length: {e:?}"
                ),
                _ => assert!(
                    matches!(e, BooleanError::Escalated { .. }),
                    "{label}: in band: {e:?}"
                ),
            }
        }
        for (label, volume) in &built {
            assert!(
                (volume - want).abs() <= slack,
                "{label}: the tube and the dome: {volume} vs {want}, slack {slack:e}"
            );
        }
        if let [(_, v0), (_, v1)] = built.as_slice() {
            assert!(
                (v0 - v1).abs() <= slack,
                "k = {k}: the member orders' volumes {v0} and {v1}, slack {slack:e}"
            );
        }
    }
}

/// **A cap abutting the tube on its rim glues a seam or a
/// continuation, or meets it at a corner**: the hemisphere's wall meets
/// the tube's in a seam, the stacked tube's continues its carrier, and
/// the boolean verifies each. The frustum's base rim lies on the cone
/// and on the tube's wall at once, at a 45° corner: the carrier ladder
/// decides the cone distinct from the tube's wall, so the rim is an ON
/// event, and the union is the tube and the frustum
/// (`π·R²·H + π·(R/2)/3·(R² + R²/2 + R²/4)`), its abutting discs
/// consumed: two discs, two half-band walls each.
#[test]
fn a_cap_abutting_on_the_rim_glues_a_seam_or_a_continuation_undeclared() {
    let tube = rod_z(R, 0.0, H);
    let hemi = hemisphere_on_the_cap();
    // The hemisphere is G1 at the rim (wedge π): its wall meets the
    // tube's in a seam, which the boolean verifies by its witness.
    seam_builds("hemisphere", &tube, &hemi);
    // The stacked cylinder continues the tube's carrier: its own rim
    // lies on the tube's wall, but its parent wall is that carrier, so
    // the ladder decides it no distinct parent and the door stands. Its
    // walls are one carrier by margin, and the union glues them as the
    // continuation they are.
    let stacked = rod_z(R, H, 1.0);
    for (order, x, y) in [(0, &tube, &stacked), (1, &stacked, &tube)] {
        // The join matches the rim's two semicircles as arcs, and the
        // four wall faces stay unmerged (a curved continuation's merge
        // is skipped).
        let (v, c, k) = built(
            &format!("stacked, order {order}"),
            topo::union(x, y, Tol::witness()),
        );
        assert_eq!(k, [0; 6], "stacked, order {order}: no contact records");
        let want = PI * R * R * (H + 1.0);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "stacked, order {order}: the two tubes: {v} vs {want}"
        );
        assert_eq!(c, (6, 10, 6, 1), "stacked, order {order}: F, E, V, shells");
    }
    let cone = frustum_on_the_cap();
    for (order, x, y) in [(0, &tube, &cone), (1, &cone, &tube)] {
        let (v, c, k) = built(
            &format!("frustum, order {order}"),
            topo::union(x, y, Tol::witness()),
        );
        let want = PI * R * R * H + PI * (R / 2.0) / 3.0 * (R * R + R * R / 2.0 + R * R / 4.0);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "frustum, order {order}: the tube and the frustum: {v} vs {want}"
        );
        assert_eq!(c, (6, 10, 6, 1), "frustum, order {order}: F, E, V, shells");
        assert_eq!(k, [0; 6], "frustum, order {order}: no contact records");
    }
}

#[test]
fn the_tube_and_an_overlapping_ball_refuse_at_the_crossing_layer() {
    let tube = rod_z(R, 0.0, H);
    let ball = finished(
        "the ball",
        ball_poled_z(R, Vec3::new(0.0, 0.0, H), Tol::witness()),
        Tol::witness(),
    );
    for e in union_both_orders(&tube, &ball) {
        // A ruling's touch of the ball, decided, keeps the pierce door; a
        // meridian's touch of the wall, read in the band's gap, escalates
        // on the circle × cylinder roots, at the edge and the wall.
        assert!(
            is_pierce(&e)
                || matches!(
                    e,
                    BooleanError::CurvedPierceEscalated {
                        decision: topo::BooleanDecision::ArcCylinderRoots,
                        ..
                    }
                ),
            "tube ∪ ball: {e:?}"
        );
    }
}

/// **The stadium: a slab whose top and bottom end on a rod's rulings.**
/// Each is tangent to the rod along its ruling with their outward
/// normals agreeing, so the glue door glues the pair `Tangent`; the
/// slab's top edge runs along the rod wall's interior there, an
/// edge–sector site the tangent lump resolves. Every op answers at its
/// closed form, tier 3 clean, the union in both orders: the rod's half
/// past the slab's end is `π/8` of the rod's `π/4`.
#[test]
fn the_stadium_answers_every_op() {
    let tol = Tol::witness();
    let slab: AtRestBody<f64> = finished(
        "the slab",
        brick((-1.0, 2.0), (0.0, 1.0), (-0.5, 0.5), tol),
        tol,
    );
    let lp = profile::circle(Point2::new(2.0, 0.0), 0.5, tol).unwrap();
    // Sketch normal onto +y: the rod's axis is the line x = 2, z = 0.
    let plane = profile::SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), -PI / 2.0),
        geom_core::Tol::witness(),
    ));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let rod = finished("the rod", rod, tol);
    let half = PI / 8.0;
    for (what, got, want) in [
        ("slab ∪ rod", topo::union(&slab, &rod, tol), 3.0 + half),
        ("rod ∪ slab", topo::union(&rod, &slab, tol), 3.0 + half),
        ("slab ∖ rod", topo::subtract(&slab, &rod, tol), 3.0 - half),
        ("rod ∖ slab", topo::subtract(&rod, &slab, tol), half),
        ("slab ∩ rod", topo::intersect(&slab, &rod, tol), half),
    ] {
        let Ok(BooleanResult::Body(r)) = got else {
            panic!("{what}: {got:?}")
        };
        assert_eq!(
            topo::validate_geometric(&r.body, tol),
            Ok(()),
            "{what}: tier 3"
        );
        let v = topo::test_support::pinned_volume("the measured body", &r.body, tol).volume;
        assert!((v - want).abs() < 1e-9, "{what}: volume {v} vs {want}");
    }
}

/// **A line seam that IS one: the D-bar on the slab.** The half-rod
/// whose flat lies on the slab's side face (`Rest`) and whose wall
/// starts at the slab's top and bottom edges leaves each tangent ruling
/// on the side away from the slab, so its walls verify as seams against
/// the slab's top and bottom in both orders, and the union builds: the slab with its `+x` side bent round into a half
/// cylinder, tier 3 and 3′ clean, its volume the slab's plus the half
/// rod's, census (6, 12, 8, 1).
#[test]
fn a_d_bar_on_the_slab_builds_as_a_line_seam() {
    let tol = Tol::witness();
    let slab: AtRestBody<f64> = finished(
        "the slab",
        brick((-1.0, 2.0), (0.0, 1.0), (-0.5, 0.5), tol),
        tol,
    );
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(2.0, -0.5), 1.0),
        (Point2::new(2.0, 0.5), 0.0),
    ]);
    let plane = profile::SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), -PI / 2.0),
        geom_core::Tol::witness(),
    ));
    let p = profile::Profile::new(plane, vec![lp])
        .validate(tol)
        .unwrap();
    let dbar = extrude(
        &p,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    let dbar = finished("the D-bar", dbar, tol);
    let half_rod = PI * 0.25 / 2.0;
    let v = topo::test_support::pinned_volume("the measured body", &dbar, tol).volume;
    assert!(
        (v - half_rod).abs() <= 1e-12,
        "the half rod: {v} vs {half_rod}"
    );
    for (x, y) in [(&slab, &dbar), (&dbar, &slab)] {
        let label = "slab ∪ D-bar";
        let (v, c, _) = built(label, topo::union(x, y, tol));
        let want = 3.0 + half_rod;
        assert!((v - want).abs() <= 1e-12, "{label}: {v} vs {want}");
        assert_eq!(
            c,
            (6, 12, 8, 1),
            "{label}: the slab with one wall bent round"
        );
    }
}

/// **A seam lets a rim lying on the partner's carrier reach its lane.**
/// A puck of radius `a`, height `r`, and the quarter-disc ring revolved
/// round it, of minor radius `r`: the puck's top meets the ring's torus
/// G1 along its top circle (`Seam`), the walls are one cylinder
/// (`Rest`), the bottoms one plane (continuation). The puck's rim
/// lies identically on the torus, an ON event the circle root door
/// places (`LiesOn`); the seam's cover must not read it as a touch
/// first. The union is the rounded puck: census (3, 4, 3, 1) and the
/// Pappus volume, in both orders.
#[test]
fn a_puck_and_its_rounding_ring_build_as_a_seam() {
    let tol = Tol::witness();
    let (a, r) = (1.0_f64, 0.25_f64);
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let ring = revolved_about_y(
        vec![
            (Point2::new(a, 0.0), 0.0),
            (Point2::new(a + r, 0.0), q),
            (Point2::new(a, r), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut ring =
        topo::transform_rigid(&ring, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    ring.merge_coplanar_faces(tol).unwrap();
    let ring = finished("the ring", ring, tol);
    let puck = rod_z(a, 0.0, r);
    let want =
        PI * a * a * r + (PI * r * r / 4.0) * core::f64::consts::TAU * (a + 4.0 * r / (3.0 * PI));
    for (label, x, y) in [("puck ∪ ring", &puck, &ring), ("ring ∪ puck", &ring, &puck)] {
        let (v, c, _) = built(label, topo::union(x, y, tol));
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: {v} vs Pappus {want}"
        );
        assert_eq!(c, (3, 3, 2, 1), "{label}: top disc, torus, floor");
        // The join closes the torus's rim, and its survivor stays on the
        // seam strut: a vertex of a closed edge, but not conventional.
        let Ok(BooleanResult::Body(bb)) = topo::union(x, y, tol) else {
            panic!("{label}: builds")
        };
        let b = &bb.body;
        let closed: Vec<_> = b
            .edges()
            .filter_map(|(_, e)| {
                let v = b.get_half_edge(e.he_plus)?.start;
                (b.get_half_edge(e.he_minus)?.start == v).then_some(v)
            })
            .collect();
        assert!(!closed.is_empty(), "{label}: a closed edge");
        assert!(
            closed.iter().all(|&v| !topo::is_conventional_vertex(b, v)),
            "{label}: every closed edge's vertex keeps a strut"
        );
    }
}

/// **A rod in a bore refuses at the crossing layer at all but two
/// turns.** A rod of radius 0.5 seated in the bore of radius 1 of a
/// revolved tube, touching along a ruling, at thirty turns of the
/// touching ruling: pinned per outcome, and each union that builds is
/// the tube and the rod, tier 3 clean. The two that build are the turns
/// where a seam lies on the touching ruling (the tube's and the rod's at
/// turn 0, the rod's at turn 15): the seam splits the other's rim circle
/// where it touches, and nothing splits it at the other turns. Its covered ends
/// lie on the certified side, so this row holds with or without the
/// cover's side gate; the gate is carried by the unit row
/// `a_cover_admits_an_off_point_only_on_its_certified_side`, which is
/// the only place a certificate's wrong side is reachable.
#[test]
fn a_rod_in_a_bore_refuses_at_the_crossing_layer() {
    let tol = Tol::witness();
    let tube = revolved_about_y(
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0, 2.0), 0.0),
            (Point2::new(1.0, 2.0), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let tube = finished(
        "the tube",
        topo::transform_rigid(&tube, &geom_core::test_support::rigid(&turn, tol), tol).unwrap(),
        tol,
    );
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for k in 0..30 {
        let th = f64::from(k) * core::f64::consts::TAU / 30.0;
        let at = Affine3::translation(Vec3::new(0.5 * th.cos(), 0.5 * th.sin(), 0.0));
        let rod = finished(
            "the rod",
            topo::transform_rigid(
                &rod_z(0.5, -0.5, 1.0),
                &geom_core::test_support::rigid(&at, tol),
                tol,
            )
            .unwrap(),
            tol,
        );
        let key = match topo::union(&tube, &rod, tol) {
            Ok(BooleanResult::Body(r)) => {
                assert_eq!(
                    topo::validate_geometric(&r.body, tol),
                    Ok(()),
                    "turn {k}: tier 3"
                );
                let v = topo::test_support::pinned_volume("the measured body", &r.body, tol).volume;
                let want = 6.0 * PI + PI / 4.0;
                assert!((v - want).abs() < 1e-9, "turn {k}: volume {v} vs {want}");
                "built".to_owned()
            }
            Ok(BooleanResult::Empty) => panic!("turn {k}: empty"),
            Err(e) => format!("{:?}", e.kind()),
        };
        *kinds.entry(key).or_default() += 1;
    }
    assert_eq!(
        kinds,
        [
            ("CurvedPierceUnsupported".to_owned(), 28),
            ("built".to_owned(), 2)
        ]
        .into_iter()
        .collect(),
    );
}

#[test]
fn a_ball_seated_in_its_own_bore_refuses_at_the_crossing_layer() {
    let tol = Tol::witness();
    let block: AtRestBody<f64> = finished(
        "the block",
        brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let bored = match topo::subtract(&block, &rod_z(R, -3.0, 6.0), tol).unwrap() {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("the bored block is not empty"),
    };
    let ball = finished(
        "the ball",
        ball_poled_z(R, Vec3::new(0.0, 0.0, 0.0), tol),
        tol,
    );
    for e in union_both_orders(&bored, &ball) {
        // The ball's meridian circles touch the bore wall at their two
        // equator points: a tangency, which the circle × cylinder lane's
        // ladder escalates on its own decision, at the edge and the wall,
        // when its margin lands in the band's gap, and which keeps the
        // pierce door otherwise.
        assert!(
            is_pierce(&e)
                || matches!(
                    e,
                    BooleanError::CurvedPierceEscalated {
                        decision: topo::BooleanDecision::ArcCylinderRoots,
                        ..
                    }
                ),
            "the crossing layer's refusal: {e:?}"
        );
    }
}

/// **An arc lying in a curved face refuses `EdgeInFace` (`Curved`)**
/// where its two faces both leave it into the face's material: the
/// material parts it into two coincident arcs
/// (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
/// The bowl a sunk dome leaves meets the tube's wall so along the rim's
/// arcs, and a rod's top rim on the dome's sphere meets the sphere so.
fn refuses_the_rims_slit(label: &str, r: Result<BooleanResult<f64>, BooleanError>) {
    assert!(
        matches!(
            r,
            Err(BooleanError::EdgeInFace {
                gap: topo::EdgeInFaceGap::Curved,
                ..
            })
        ),
        "{label}: refuses the rim's slit: {:?}",
        r.map(|_| ())
    );
}

/// A spherical cap's volume: height `h` on a sphere of radius `rho`.
fn cap_volume(rho: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * rho - h) / 3.0
}

/// `(faces, edges, vertices, shells)` of a body.
fn census(b: &Body<f64>) -> (usize, usize, usize, usize) {
    (
        b.faces().count(),
        b.edges().count(),
        b.vertices().count(),
        b.shells().count(),
    )
}

/// What a boolean that builds is: its volume, its census, and the
/// contact records it carries as `[v-v, v-f, curve, patch]` counts.
type Built = (f64, (usize, usize, usize, usize), [usize; 6]);

/// The body of a boolean that builds, at tier 3 and 3′.
fn built(label: &str, r: Result<BooleanResult<f64>, BooleanError>) -> Built {
    let tol = Tol::witness();
    let bb = match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{label}: builds: {other:?}"),
    };
    let b = &bb.body;
    topo::validate_geometric(b, tol).unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
    topo::validate_pseudomanifold(b, &bb.contacts, tol)
        .unwrap_or_else(|e| panic!("{label}: tier 3′: {e:?}"));
    let band = geom_core::Band::linear(tol).unwrap();
    assert_eq!(
        topo::joinable_vertices(b, band).unwrap(),
        vec![],
        "{label}: maximal edges"
    );
    let c = &bb.contacts;
    (
        topo::test_support::pinned_volume("the measured body", b, tol).volume,
        census(b),
        [
            c.vertex_vertex().count(),
            c.vertex_face().count(),
            c.vertex_edge().count(),
            c.edge_edge().count(),
            c.edge_face().count(),
            c.face_face().count(),
        ],
    )
}

/// **A dome sunk into the tube**: the dome lowered by `dz`
/// puts its base disc inside the tube and its rim circle on the tube's
/// wall. The tube's two wall faces are bounded by seam rulings at
/// `±x`, where the rim's own vertices are, so each rim semicircle lies
/// wholly inside one face (certificate (a)) and its ends are recorded.
/// The union is the tube plus the dome's cap above `z = H`.
#[test]
fn a_dome_sunk_into_the_tube_builds_undeclared() {
    let tol = Tol::witness();
    let tube = rod_z(R, 0.0, H);
    let rho = 2.0_f64.sqrt() * R;
    for dz in [-1e-3, -0.3] {
        let lift = Affine3::translation(Vec3::new(0.0, 0.0, dz));
        let dome = finished(
            "the sunk dome",
            topo::transform_rigid(
                &dome_on_the_cap(),
                &geom_core::test_support::rigid(&lift, tol),
                tol,
            )
            .unwrap(),
            tol,
        );
        let want = PI * R * R * H + cap_volume(rho, rho - R + dz);
        for (order, r) in [
            topo::union(&tube, &dome, tol),
            topo::union(&dome, &tube, tol),
        ]
        .into_iter()
        .enumerate()
        {
            let label = format!("dz = {dz}, order {order}");
            let (v, c, k) = built(&label, r);
            assert_eq!(k, [0; 6], "{label}: no contact records");
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the tube and the cap above it: {v} vs {want}"
            );
            assert_eq!(c, (6, 10, 7, 1), "{label}: F, E, V, shells");
        }
    }
}

/// [`rod_z`]'s tube from `z = 0` to [`H`], its wall in four quarter
/// faces: seam rulings at `±x` and `±y`.
fn quartered_tube() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(R, 0.0), q),
        (Point2::new(0.0, R), q),
        (Point2::new(-R, 0.0), q),
        (Point2::new(0.0, -R), q),
    ]);
    let p = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: H,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    finished("the quartered tube", rod, tol)
}

/// **A dome sunk into the tube and turned about the axis builds every
/// op but `t ∖ d`**: each rim semicircle lies on the tube's wall and
/// crosses a seam ruling mid-arc, passing from one wall face to the
/// next. The reduction splits the rim where it crosses the ruling, and
/// each half lies inside one face.
///
/// The rows: the two-face tube turned so that no rim vertex is on a
/// ruling (`1/12`, `1/8`, `1/5` of a turn); the four-face tube at the
/// same turns, where each semicircle crosses TWO rulings; and the
/// four-face tube at `0` and `1/4` of a turn, where the rim's vertices
/// sit on two rulings and each semicircle crosses the one between. Each
/// at both sink depths, every op in both member orders. Where a
/// semicircle crosses two rulings, its second crossing is met against
/// the next wall face, not by reading the first split's fragment against
/// the same face again: that re-reading is not pinned here.
///
/// The closed forms: the dome's sphere (radius `√2·R`, its rim at
/// `z = H + dz`) holds a cap of height `√2·R − R + dz` above the tube's
/// top, so `d ∖ t` is that cap, `t ∩ d` is the dome less it, and `t ∪ d`
/// adds it to the tube.
///
/// `t ∖ d` touches itself along the rim, as the unturned dome's does
/// (`a_tube_through_the_domes_base_builds_every_op_undeclared`): the
/// pocket the dome leaves meets the wall along the whole rim, the
/// dome's faces at each rim arc leaving it into the wall's material, so
/// each arc is two coincident arcs and `t ∖ d` refuses `EdgeInFace`
/// (`Curved`, [`refuses_the_rims_slit`]). The record carriage it pinned
/// (v-v at the rulings, v-f off them, v-e and e-e where the join took a
/// split vertex) is owed back with the curved slit form
/// (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
#[test]
fn a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared() {
    type Censuses = [(usize, usize, usize, usize); 3];
    let tol = Tol::witness();
    let rho = 2.0_f64.sqrt() * R;
    let (tube, quartered) = (rod_z(R, 0.0, H), quartered_tube());
    // `[∪, d ∖ t, ∩]`; `t ∖ d` refuses ([`refuses_the_rims_slit`]).
    let off: Censuses = [(6, 10, 7, 1), (3, 4, 3, 1), (4, 6, 4, 1)];
    let four: Censuses = [(8, 16, 11, 1), (3, 4, 3, 1), (4, 6, 4, 1)];
    let rows = [
        ("two walls", &tube, 1.0 / 12.0, off),
        ("two walls", &tube, 1.0 / 8.0, off),
        ("two walls", &tube, 1.0 / 5.0, off),
        ("four walls", &quartered, 1.0 / 12.0, four),
        ("four walls", &quartered, 1.0 / 8.0, four),
        ("four walls", &quartered, 1.0 / 5.0, four),
        ("four walls", &quartered, 0.0, four),
        ("four walls", &quartered, 1.0 / 4.0, four),
    ];
    for dz in [-1e-3, -0.3] {
        let above = cap_volume(rho, rho - R + dz);
        let inside = cap_volume(rho, rho - R) - above;
        let t = PI * R * R * H;
        let lift = Affine3::translation(Vec3::new(0.0, 0.0, dz));
        let sunk = topo::transform_rigid(
            &dome_on_the_cap(),
            &geom_core::test_support::rigid(&lift, tol),
            tol,
        )
        .unwrap();
        for (walls, tube, turn, census) in rows {
            let spin =
                Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 2.0 * PI * turn);
            let d = finished(
                "the turned dome",
                topo::transform_rigid(&sunk, &geom_core::test_support::rigid(&spin, tol), tol)
                    .unwrap(),
                tol,
            );
            refuses_the_rims_slit(
                &format!("{walls}, dz = {dz}, turn {turn}: t ∖ d"),
                topo::subtract(tube, &d, tol),
            );
            for (op, r, want, census, contacts) in [
                (
                    "t ∪ d",
                    topo::union(tube, &d, tol),
                    t + above,
                    census[0],
                    [0; 6],
                ),
                (
                    "d ∪ t",
                    topo::union(&d, tube, tol),
                    t + above,
                    census[0],
                    [0; 6],
                ),
                (
                    "d ∖ t",
                    topo::subtract(&d, tube, tol),
                    above,
                    census[1],
                    [0; 6],
                ),
                (
                    "t ∩ d",
                    topo::intersect(tube, &d, tol),
                    inside,
                    census[2],
                    [0; 6],
                ),
                (
                    "d ∩ t",
                    topo::intersect(&d, tube, tol),
                    inside,
                    census[2],
                    [0; 6],
                ),
            ] {
                let label = format!("{walls}, dz = {dz}, turn {turn}: {op}");
                let (v, c, k) = built(&label, r);
                assert!(
                    (v - want).abs() <= 1e-12 * want,
                    "{label}: the closed form: {v} vs {want}"
                );
                assert_eq!(c, census, "{label}: F, E, V, shells");
                assert_eq!(
                    k, contacts,
                    "{label}: [v-v, v-f, v-e, e-e, e-f, f-f] records"
                );
            }
        }
    }
}

/// **A tube poking through the dome's base**: the tube runs
/// to `z = 2.2` and the dome stands on `z = H`, so the dome's rim lies on
/// the tube's wall inside that face (certificate (a)) and the tube's top
/// disc cuts the dome. Every op but `t ∖ d` builds at its closed form:
/// the dome above the disc is a spherical cap of height `1 + √2 − 2.2`.
///
/// `t ∖ d` touches itself along the rim: the bowl the dome leaves in
/// the tube meets the tube's wall along the rim's two semicircles, whose
/// ends rest on the wall's seam ruling (v-e), and the dome's faces at
/// each leave it into the wall's material. Each semicircle is two
/// coincident arcs, which refuses `EdgeInFace` (`Curved`,
/// [`refuses_the_rims_slit`]); the v-e record carriage this row held is
/// owed back with the curved slit form
/// (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
#[test]
fn a_tube_through_the_domes_base_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let tall = rod_z(R, 0.0, 2.2);
    let dome = dome_on_the_cap();
    let rho = 2.0_f64.sqrt() * R;
    let above = cap_volume(rho, R + rho - 2.2);
    let inside = cap_volume(rho, rho - R) - above;
    let tube = PI * R * R * 2.2;
    refuses_the_rims_slit("t ∖ d", topo::subtract(&tall, &dome, tol));
    for (label, r, want, census, contacts) in [
        (
            "t ∪ d",
            topo::union(&tall, &dome, tol),
            tube + above,
            (6, 10, 7, 1),
            [0; 6],
        ),
        (
            "d ∪ t",
            topo::union(&dome, &tall, tol),
            tube + above,
            (6, 10, 7, 1),
            [0; 6],
        ),
        (
            "d ∖ t",
            topo::subtract(&dome, &tall, tol),
            above,
            (3, 4, 3, 1),
            [0; 6],
        ),
        (
            "t ∩ d",
            topo::intersect(&tall, &dome, tol),
            inside,
            (4, 6, 4, 1),
            [0; 6],
        ),
        (
            "d ∩ t",
            topo::intersect(&dome, &tall, tol),
            inside,
            (4, 6, 4, 1),
            [0; 6],
        ),
    ] {
        let (v, c, k) = built(label, r);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: the closed form: {v} vs {want}"
        );
        assert_eq!(c, census, "{label}: F, E, V, shells");
        assert_eq!(
            k, contacts,
            "{label}: [v-v, v-f, v-e, e-e, e-f, f-f] records"
        );
    }
}

/// **A G1 joint authored inside one profile** is the structural form of
/// the seam. The capsule revolved from one
/// profile — the tube's side, then a quarter arc tangent to it, the
/// profile deciding the joint tangent from its carriers — builds with its
/// joint minted `TangentIntersection`, and it is the seam's union: the same census and the same volume.
#[test]
fn a_g1_joint_authored_inside_one_profile_is_the_seams_union() {
    let tol = Tol::witness();
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), 0.0),
        (Point2::new(R, H), bulge),
        (Point2::new(0.0, H + R), 0.0),
    ]);
    let pr = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let at0 = sweep::revolve(&pr, axis, Revolution::Full, tol)
        .unwrap()
        .body;
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut capsule =
        topo::transform_rigid(&at0, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    capsule.merge_coplanar_faces(tol).unwrap();
    topo::validate_geometric(&capsule, tol).unwrap_or_else(|e| panic!("tier 3: {e:?}"));
    let v = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(&capsule), tol),
        tol,
    )
    .volume;
    let want = tube_and_half_ball_volume();
    assert!((v - want).abs() <= 1e-12 * want, "{v} vs {want}");
    assert_eq!(census(&capsule), (5, 8, 5, 1), "the seam union's census");
    assert_eq!(
        tangent_intersections(&capsule).len(),
        2,
        "the authored joint, minted as two semicircles"
    );
}

/// A quarter rod along `x ∈ [0, 1]`, radius 0.5, axis on `y = 0,
/// z = 0.5`: its wall's lowest ruling is the line `y = z = 0`, and its
/// flat on `y = 0` faces away from the side it lies on (`side` −1 or +1
/// in `y`).
fn quarter_rod(side: f64, tol: Tol) -> AtRestBody<f64> {
    quarter_rod_of(side, 1.0, tol)
}

/// [`quarter_rod`] of length `len` along `x`.
fn quarter_rod_of(side: f64, len: f64, tol: Tol) -> AtRestBody<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let lp = if side < 0.0 {
        profile::test_support::bulge_loop(vec![
            (Point2::new(0.0, 0.5), 0.0),
            (Point2::new(-0.5, 0.5), q),
            (Point2::new(0.0, 0.0), 0.0),
        ])
    } else {
        profile::test_support::bulge_loop(vec![
            (Point2::new(0.0, 0.5), 0.0),
            (Point2::new(0.0, 0.0), q),
            (Point2::new(0.5, 0.5), 0.0),
        ])
    };
    // Sketch x → world y, sketch y → world z, the normal → world x.
    let k = 1.0 / 3.0_f64.sqrt();
    let plane = profile::SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(k, k, k),
            core::f64::consts::TAU / 3.0,
        ),
        geom_core::Tol::witness(),
    ));
    let p = profile::Profile::new(plane, vec![lp])
        .validate(tol)
        .unwrap();
    let rod = extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body;
    finished("the quarter rod", rod, tol)
}

/// **A line seam is read at the line, not off the boundary.** The
/// dodge plate (thickness 0.25, on `z ∈ [0, 0.25]`) has the edge
/// `(1, 0) → (0, 0)` on the line `y = z = 0`, and its outline then
/// sweeps an arc about `(1, 0.5)` of radius √1.25 through 273.4°,
/// dipping to `y ≈ −0.618` under that edge and closing above it. At the
/// line the plate's bottom leaves on `y < 0`, though every boundary
/// vertex and arc midpoint reads `y ≥ 0`. A quarter rod on `y > 0` leaves it on the other side, a seam:
/// the boolean verifies it, and the union goes on past it. The square
/// plate, which lies on `y < 0` everywhere, is the control. Both member
/// orders.
#[test]
fn a_line_seam_is_read_where_the_faces_leave_the_line() {
    let tol = Tol::witness();
    let sweep = 273.4_f64.to_radians();
    let dodge = profile::test_support::bulge_loop(vec![
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(0.0, 0.0), (sweep / 4.0).tan()),
        (
            Point2::new(
                1.0 + 1.25_f64.sqrt() * 120.0_f64.to_radians().cos(),
                0.5 + 1.25_f64.sqrt() * 120.0_f64.to_radians().sin(),
            ),
            0.0,
        ),
    ]);
    let square = profile::test_support::bulge_loop(vec![
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
    ]);
    for (label, lp) in [("dodge", dodge), ("square", square)] {
        let p = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        let plate = extrude(
            &p,
            Extrusion::Distance {
                depth: 0.25,
                side: ExtrudeSide::Along,
            },
            tol,
        )
        .unwrap()
        .body;
        let plate = finished("the plate", plate, tol);
        let rod = quarter_rod(1.0, tol);
        for (x, y) in [(&plate, &rod), (&rod, &plate)] {
            let r = topo::union(x, y, tol);
            if label == "square" {
                // The square plate and the rod are disjoint but for the
                // line, and the union is both.
                let (v, _, _) = built(label, r);
                let want = 0.25 + PI * 0.25 / 4.0;
                assert!((v - want).abs() <= 1e-12, "{label}: {v} vs {want}");
            } else {
                // The dodge plate's outline also reaches over the rod on
                // `y > 0`, and that crossing stops at the crossing layer,
                // past the seam.
                assert!(
                    matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                    "{label}: the crossing layer: {r:?}"
                );
            }
        }
    }
}

/// Narrow review (JOIN-1 fix pass 3): domes and bowls of several corner
/// angles, turned, on tubes, every op both orders; every build read at tiers 2, 3′, the certificate, tier 3,
/// volume, and as an operand.
#[test]
#[ignore]
fn narrow_review_dome_battery() {
    let tol = Tol::witness();
    let mut builds = 0;
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    let mut bad = 0;
    for &alpha_deg in &[15.0_f64, 30.0, 45.0, 60.0, 75.0, 89.0] {
        let alpha = alpha_deg.to_radians();
        let rise = R * (alpha / 2.0).tan();
        let bulge = (alpha / 4.0).tan();
        let dome = cap_on_the_tube(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(R, 0.0), bulge),
            (Point2::new(0.0, rise), 0.0),
        ]);
        let bowl = cap_on_the_tube(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.0, -rise), bulge),
            (Point2::new(R, 0.0), 0.0),
        ]);
        for &turn_deg in &[0.0_f64, 30.0, 90.0, 180.0, 7.0] {
            let turn = Affine3::rotation_about_axis(
                Point3::origin(),
                Vec3::unit_z(),
                turn_deg.to_radians(),
            );
            let tdome = finished(
                "the turned dome",
                topo::transform_rigid(&dome, &geom_core::test_support::rigid(&turn, tol), tol)
                    .unwrap(),
                tol,
            );
            for (plabel, partner) in [
                ("tube", rod_z(R, 0.0, H)),
                ("short tube", rod_z(R, H - 0.25, 0.25)),
                ("bowl", bowl.clone()),
            ] {
                for (x, y, xl, yl) in [(&partner, &tdome, "p", "d"), (&tdome, &partner, "d", "p")] {
                    for (op, r) in [
                        ("union", topo::union(x, y, tol)),
                        ("subtract", topo::subtract(x, y, tol)),
                        ("intersect", topo::intersect(x, y, tol)),
                    ] {
                        let label = format!("a{alpha_deg} t{turn_deg} {plabel} {xl}{yl} {op}");
                        match r {
                            Ok(BooleanResult::Body(bb)) => {
                                builds += 1;
                                let t2 = topo::validate_closed(&bb.body).is_ok();
                                let t3p =
                                    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
                                        .is_ok();
                                let cert =
                                    topo::validate_geometric_certificate(&bb.body, tol).is_ok();
                                let t3 = topo::validate_geometric(&bb.body, tol).is_ok();
                                let v = topo::mass_properties(
                                    &bb.body,
                                    tol,
                                    topo::test_support::pinned_resolution(tol),
                                )
                                .map(|m| m.enclosure.volume)
                                .unwrap_or(f64::NAN);
                                let vx =
                                    topo::test_support::pinned_volume("the measured body", x, tol)
                                        .volume;
                                let vy =
                                    topo::test_support::pinned_volume("the measured body", y, tol)
                                        .volume;
                                let want = match op {
                                    "union" => vx + vy,
                                    "subtract" => vx,
                                    _ => 0.0,
                                };
                                let vol_ok = (v - want).abs() <= 1e-9 * (vx + vy);
                                let operand = std::panic::catch_unwind(|| {
                                    sweep::test_support::assert_legal_operand(
                                        "probe", &bb.body, tol,
                                    )
                                })
                                .is_ok();
                                let ok = t2 && t3p && cert && t3 && vol_ok && operand;
                                if !ok {
                                    bad += 1;
                                }
                                println!(
                                    "NR {label}: BUILD t2={t2} t3p={t3p} cert={cert} t3={t3} vol={v:.12} want={want:.12} operand={operand}{}",
                                    if ok { "" } else { " BAD" }
                                );
                            }
                            Ok(BooleanResult::Empty) => {
                                println!("NR {label}: EMPTY");
                                if op != "intersect" {
                                    bad += 1;
                                }
                            }
                            Err(e) => {
                                let k = format!("{e:?}");
                                let k = k.split([' ', '(', '{']).next().unwrap().to_string();
                                println!("NR {label}: REFUSE {k}");
                                *refusals.entry(k).or_default() += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("NR SUMMARY builds={builds} bad={bad} refusals={refusals:?}");
    assert_eq!(bad, 0);
}

/// Narrow review: brick corners and tube rims on balls and domes —
/// contact vertices on curved faces, every op both orders; every build
/// read at tiers 2, 3′, the certificate, tier 3 and as an operand.
#[test]
#[ignore]
fn narrow_review_corner_on_ball_battery() {
    let tol = Tol::witness();
    let mut builds = 0;
    let mut bad = 0;
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    let s3 = 3.0_f64.sqrt();
    let s2 = 2.0_f64.sqrt();
    let mut poses: Vec<(String, AtRestBody<f64>, AtRestBody<f64>)> = Vec::new();
    for (bl, ball) in [
        ("ball z", ball_poled_z(s3, Vec3::new(0.0, 0.0, 0.0), tol)),
        (
            "ball y",
            sweep::test_support::ball_poled_y(s3, Vec3::new(0.0, 0.0, 0.0), tol),
        ),
    ] {
        let ball = finished("the ball", ball, tol);
        for (kl, x, y, z) in [
            ("out", (1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
            ("in", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            ("cross", (0.5, 1.0), (0.5, 1.0), (0.5, 2.5)),
            ("edge-out", (1.0, 2.0), (-1.0, 1.0), (1.0, 2.0)),
            ("slab", (-3.0, 3.0), (-3.0, 3.0), (1.0, 3.0)),
            ("neg", (-2.0, -1.0), (1.0, 2.0), (-1.0, 0.0)),
        ] {
            poses.push((
                format!("{bl} brick {kl}"),
                ball.clone(),
                finished("the brick", brick(x, y, z, tol), tol),
            ));
        }
        // A tube whose rim lies on the ball (radius 1 at height 1).
        for (tl, z0, len) in [
            ("tube up", 1.0, 1.0),
            ("tube down", -1.0, 2.0),
            ("tube through", 0.0, 3.0),
        ] {
            poses.push((format!("{bl} {tl}"), ball.clone(), rod_z(1.0, z0, len)));
        }
        // A ball of radius √2 at the origin: the tube rim at z = 1.
        let small = finished(
            "the small ball",
            ball_poled_z(s2, Vec3::new(0.0, 0.0, 0.0), tol),
            tol,
        );
        poses.push((
            format!("{bl} small tube"),
            small.clone(),
            rod_z(1.0, 1.0, 1.0),
        ));
    }
    for (label, p, q) in &poses {
        for (xl, x, y) in [("ab", p, q), ("ba", q, p)] {
            for (op, r) in [
                ("union", topo::union(x, y, tol)),
                ("subtract", topo::subtract(x, y, tol)),
                ("intersect", topo::intersect(x, y, tol)),
            ] {
                let label = format!("{label} {xl} {op}");
                match r {
                    Ok(BooleanResult::Body(bb)) => {
                        builds += 1;
                        let t2 = topo::validate_closed(&bb.body).is_ok();
                        let t3p =
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_ok();
                        let cert = topo::validate_geometric_certificate(&bb.body, tol).is_ok();
                        let t3 = topo::validate_geometric(&bb.body, tol).is_ok();
                        let v = topo::mass_properties(
                            &bb.body,
                            tol,
                            topo::test_support::pinned_resolution(tol),
                        )
                        .map(|m| m.enclosure.volume)
                        .unwrap_or(f64::NAN);
                        let operand = std::panic::catch_unwind(|| {
                            sweep::test_support::assert_legal_operand("probe", &bb.body, tol)
                        })
                        .is_ok();
                        let ok = t2 && t3p && cert && t3 && operand;
                        if !ok {
                            bad += 1;
                        }
                        println!(
                            "NR2 {label}: BUILD t2={t2} t3p={t3p} cert={cert} t3={t3} vol={v:.9} operand={operand}{}",
                            if ok { "" } else { " BAD" }
                        );
                    }
                    Ok(BooleanResult::Empty) => println!("NR2 {label}: EMPTY"),
                    Err(e) => {
                        let k = format!("{e:?}");
                        let k = k.split([' ', '(', '{']).next().unwrap().to_string();
                        println!("NR2 {label}: REFUSE {k}");
                        *refusals.entry(k).or_default() += 1;
                    }
                }
            }
        }
    }
    println!("NR2 SUMMARY builds={builds} bad={bad} refusals={refusals:?}");
    assert_eq!(bad, 0);
}

/// [`rod_z`] from `z = −1` to [`H`], less everything under the plane
/// `z = 0.5 + 0.2·x`: its wall faces are bounded below by an ellipse.
fn slanted_tube() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let tube = rod_z(R, -1.0, H + 1.0);
    let tilt = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), -(0.2_f64).atan());
    let under = topo::transform_rigid(
        &brick((-3.0, 3.0), (-3.0, 3.0), (-5.0, 0.0), tol),
        &geom_core::test_support::rigid(&tilt, tol),
        tol,
    )
    .unwrap();
    let under = finished(
        "the slab under the plane",
        topo::transform_rigid(
            &under,
            &geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, 0.5)).unwrap(),
            tol,
        )
        .unwrap(),
        tol,
    );
    match topo::subtract(&tube, &under, tol) {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("the slanted tube builds: {other:?}"),
    }
}

/// **A wall bounded by an ellipse builds every op but `t ∖ d`.** The
/// turned sunk dome on a tube whose bottom is cut by the plane
/// `z = 0.5 + 0.2·x`: the rim arc lying on the wall, which neither
/// certificate (a) nor (b) places, is read by the interior question,
/// whose candidates against the ellipse are the arc's meetings with
/// its plane. The rim sits above that plane, so it has none, and the
/// rim splits at the seam rulings as on the plain tube. The tube holds
/// `π·R²·(H − 0.5)` (the plane's mean height over the disc is `0.5`);
/// the dome's split, the censuses and the records are the two-face
/// rows of
/// [`a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared`],
/// whose `t ∖ d` this one's refuses as.
/// The wall's ellipse trim is measured by certified quadrature, so each
/// volume is read to the enclosure's own half-width.
#[test]
fn a_turned_rim_on_a_wall_bounded_by_an_ellipse_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let tube = slanted_tube();
    let rho = 2.0_f64.sqrt() * R;
    let t = PI * R * R * (H - 0.5);
    for dz in [-1e-3, -0.3] {
        let above = cap_volume(rho, rho - R + dz);
        let inside = cap_volume(rho, rho - R) - above;
        let lift = Affine3::translation(Vec3::new(0.0, 0.0, dz));
        let sunk = topo::transform_rigid(
            &dome_on_the_cap(),
            &geom_core::test_support::rigid(&lift, tol),
            tol,
        )
        .unwrap();
        for turn in [1.0 / 12.0, 1.0 / 5.0] {
            let spin =
                Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), 2.0 * PI * turn);
            let d = finished(
                "the turned dome",
                topo::transform_rigid(&sunk, &geom_core::test_support::rigid(&spin, tol), tol)
                    .unwrap(),
                tol,
            );
            let (whole, common) = ((6, 10, 7, 1), (4, 6, 4, 1));
            refuses_the_rims_slit(
                &format!("dz = {dz}, turn {turn}: t ∖ d"),
                topo::subtract(&tube, &d, tol),
            );
            for (op, r, want, census, contacts) in [
                (
                    "t ∪ d",
                    topo::union(&tube, &d, tol),
                    t + above,
                    whole,
                    [0; 6],
                ),
                (
                    "d ∪ t",
                    topo::union(&d, &tube, tol),
                    t + above,
                    whole,
                    [0; 6],
                ),
                (
                    "d ∖ t",
                    topo::subtract(&d, &tube, tol),
                    above,
                    (3, 4, 3, 1),
                    [0; 6],
                ),
                (
                    "t ∩ d",
                    topo::intersect(&tube, &d, tol),
                    inside,
                    common,
                    [0; 6],
                ),
                (
                    "d ∩ t",
                    topo::intersect(&d, &tube, tol),
                    inside,
                    common,
                    [0; 6],
                ),
            ] {
                let label = format!("dz = {dz}, turn {turn}: {op}");
                let pad = match &r {
                    Ok(BooleanResult::Body(bb)) => {
                        topo::test_support::pinned_volume("the measured body", &bb.body, tol)
                            .volume_pad
                    }
                    _ => 0.0,
                };
                let (v, c, k) = built(&label, r);
                assert!(
                    (v - want).abs() <= pad + 1e-12,
                    "{label}: the closed form: {v} ± {pad} vs {want}"
                );
                assert_eq!(c, census, "{label}: F, E, V, shells");
                assert_eq!(
                    k, contacts,
                    "{label}: [v-v, v-f, v-e, e-e, e-f, f-f] records"
                );
            }
        }
    }
}

/// **A rod's rim lying on the dome's sphere across its seam meridian.**
/// A rod of radius `0.3` and length `0.6` on an axis through the
/// sphere's centre, tilted `α` off the dome's axis and turned `θ` about
/// it, its top rim on the sphere: the rim crosses the dome's revolve
/// seam, a meridian CIRCLE, mid-arc, so the split there is a
/// circle × circle meeting. The rod runs out through the dome's base
/// disc, so `a ∩ d` is the rod between its top and the plane `z = H`,
/// which cuts clean across its wall: `π·0.09·(h₀ − 1/cos α)`, `h₀` the
/// top's distance from the centre. Every op but `d ∖ a` in both member
/// orders, at tiers 3 and 3′; the volumes are read to `max(1e-9, ε)`:
/// the quadrature's reach on the tilted pieces, and at a coarse `ε` the
/// band's positional slack over less than a unit of surface. `d ∖ a`
/// leaves the rod's top rim on the sphere with the rod's top and wall
/// both going into the dome's material, two coincident arcs, and refuses
/// `EdgeInFace` (`Curved`, [`refuses_the_rims_slit`]); its edge-edge
/// records at the seam meridian are owed back with the curved slit form
/// (`work/heldrecords/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
#[test]
fn a_rod_rim_on_the_dome_across_its_seam_meridian_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let dome = dome_on_the_cap();
    let rho = 2.0_f64.sqrt() * R;
    let (r, l) = (0.3, 0.6);
    let h0 = (rho * rho - r * r).sqrt();
    let vd = cap_volume(rho, rho - R);
    let va = PI * r * r * l;
    let reach = 1e-9_f64.max(tol.eps());
    for (alpha, theta, own) in [
        (15.0_f64, 31.0_f64, 0.0_f64),
        (20.0, 20.0, 0.0),
        (15.0, 20.0, 90.0),
    ] {
        let mut rod = rod_z(r, h0 - l, l).into_body();
        for m in [
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), own.to_radians()),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), alpha.to_radians()),
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), theta.to_radians()),
            Affine3::translation(Vec3::new(0.0, 0.0, H - R)),
        ] {
            rod =
                topo::transform_rigid(&rod, &geom_core::test_support::rigid(&m, tol), tol).unwrap();
        }
        let a = finished("the tilted rod", rod, tol);
        let i = PI * r * r * (h0 - 1.0 / alpha.to_radians().cos());
        refuses_the_rims_slit(
            &format!("α {alpha}°, θ {theta}°, own {own}°: d ∖ a"),
            topo::subtract(&dome, &a, tol),
        );
        for (op, res, want, census, contacts) in [
            (
                "a ∪ d",
                topo::union(&a, &dome, tol),
                va + vd - i,
                (6, 10, 7, 1),
                [0; 6],
            ),
            (
                "d ∪ a",
                topo::union(&dome, &a, tol),
                va + vd - i,
                (6, 10, 7, 1),
                [0; 6],
            ),
            (
                "a ∖ d",
                topo::subtract(&a, &dome, tol),
                va - i,
                (4, 6, 4, 1),
                [0; 6],
            ),
            (
                "a ∩ d",
                topo::intersect(&a, &dome, tol),
                i,
                (4, 6, 4, 1),
                [0; 6],
            ),
            (
                "d ∩ a",
                topo::intersect(&dome, &a, tol),
                i,
                (4, 6, 4, 1),
                [0; 6],
            ),
        ] {
            let label = format!("α {alpha}°, θ {theta}°, own {own}°: {op}");
            let (v, c, k) = built(&label, res);
            assert!(
                (v - want).abs() <= reach,
                "{label}: the closed form: {v} vs {want}"
            );
            assert_eq!(c, census, "{label}: F, E, V, shells");
            assert_eq!(
                k, contacts,
                "{label}: [v-v, v-f, v-e, e-e, e-f, f-f] records"
            );
        }
    }
}
