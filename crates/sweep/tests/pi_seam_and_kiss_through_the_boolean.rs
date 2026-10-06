//! **What a π seam between distinct carriers, a cap abutting a tube on
//! its rim, and a coaxial kiss, do through the boolean.** The declared
//! seam and the transverse rim build; every other row is a refusal,
//! pinned by kind.
//!
//! - **The sphere-capped tube** (cylinder `z ∈ [0, h]` ∪ hemisphere of
//!   the same radius on its top cap): the two walls meet G1 along the
//!   rim circle, wedge π. Its walls declared a `Seam` and its discs
//!   `Rest`, the union BUILDS in both orders: the tube and the half
//!   ball, the rim minted `TangentIntersection` — the same body as the
//!   capsule revolved from one profile with its joint authored, which
//!   needs no declaration. Undeclared it stops at the crossing layer,
//!   naming the recourse; declared `Tangent` it is contradicted by its
//!   aligned senses and steered to the seam; declared `Rest` or a
//!   continuation it is contradicted on carrier kind.
//! - **A seam declared where there is none**: the transverse dome is
//!   contradicted, the cone is outside the declaration inventory, and a
//!   ball seated in its bore has no locus to verify one along.
//! - **A cap abutting the tube on its rim, its end discs declared
//!   `Rest`**, answers by its corner. A transverse 45° spherical dome
//!   BUILDS, its seams aligned with the tube's or not: each rim circle
//!   lies on the other operand's wall, and its parents are decided
//!   distinct from that wall, so it is an ON event, and the two rims
//!   are one seam of the zip. A tube revolved rather than extruded, and
//!   an inverted dome in the tube's place (a lens), build the same way.
//!   Undeclared, the coincident discs refuse. A rim offset in band of
//!   the partner's wall escalates.
//! - **A tube ending on a ball, or on a torus's 45° latitude**,
//!   undeclared: the rim lies inside the partner's face rather than on
//!   its boundary, and passes the crossing layer the same way; the union
//!   stops in the join. A same-radius stacked cylinder
//!   stops there on its own rim, whose parent shares the partner's
//!   carrier. A cone frustum is refused earlier, at the operand gate,
//!   on its kind.
//! - **Tube ∪ ball** (overlapping, ball centred on the top cap) stops at
//!   the crossing layer.
//! - **The stadium** (slab ∪ cylinder whose wall the slab's top and
//!   bottom are tangent to along a ruling). Undeclared it stops at the
//!   crossing layer; declared `Tangent` the witness lane contradicts it,
//!   because its outward normals AGREE; declared a `Seam` it is
//!   contradicted too, because the ruling runs through the rod wall's
//!   interior: the wall does not END at the line, so no seam runs along
//!   it (`seam_locus_no_edge`). The D-bar on the slab, whose half-rod
//!   wall does end there, verifies as a line seam and builds.
//! - **The dodge plate** (a plate whose outline leaves a line on one
//!   side and then reaches round to the other): the line seam is read
//!   where the faces leave the line, not off the boundary.
//! - **The kiss** (a ball seated in a bore of its own radius): the
//!   equator touches the bore wall, wedge 2π. Undeclared it stops at
//!   the crossing layer; declared `Tangent` or a `Seam` the class is
//!   refused — the two faces share no boundary circle, so the rim
//!   routing never runs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::seam_pairs::meeting;
use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick, finished, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{
    AtRestBody, Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    ContactClass, FaceKey, FacePairDeclaration,
};

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

fn declared(
    a: &[FaceKey],
    b: &[FaceKey],
    class: impl Into<BooleanCoincidence> + Copy,
) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    for &fa in a {
        for &fb in b {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, class));
        }
    }
    d
}

/// The union both ways round, each with its declarations' sides
/// swapped to match, and both refusals.
fn union_both_orders(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    fa: &[FaceKey],
    fb: &[FaceKey],
    class: Option<BooleanCoincidence>,
) -> [BooleanError; 2] {
    let run = |x: &AtRestBody<f64>, y: &AtRestBody<f64>, fx: &[FaceKey], fy: &[FaceKey]| {
        let r = match class {
            None => topo::union(x, y, Tol::witness()),
            Some(c) => topo::union_with(x, y, &meeting(x, y, declared(fx, fy, c)), Tol::witness()),
        };
        match r {
            Err(e) => e,
            Ok(BooleanResult::Body(_)) => panic!("{class:?}: the union built a body"),
            Ok(BooleanResult::Empty) => panic!("{class:?}: the union came back empty"),
        }
    };
    [run(a, b, fa, fb), run(b, a, fb, fa)]
}

/// A z-axis cylinder of radius `r` from `z0`, length `len`, through the
/// extrude door.
fn rod_z(r: f64, z0: f64, len: f64) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
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
    let tol = Tol::witness();
    let at0 = revolved_about_y(profile, Revolution::Full, tol);
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let turned = topo::transform_rigid(&at0, &turn, tol).unwrap();
    let mut cap =
        topo::transform_rigid(&turned, &Affine3::translation(Vec3::new(0.0, 0.0, H)), tol).unwrap();
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

/// The tube and a cap, unioned both ways round with the discs at
/// `z = H` declared `Rest`.
fn unions_with_discs_rest(
    tube: &AtRestBody<f64>,
    cap: &AtRestBody<f64>,
) -> [Result<BooleanResult<f64>, BooleanError>; 2] {
    let tol = Tol::witness();
    let (dt, dc) = (planes_at_z(tube, H), planes_at_z(cap, H));
    [
        topo::union_with(tube, cap, &declared(&dt, &dc, ContactClass::Rest), tol),
        topo::union_with(cap, tube, &declared(&dc, &dt, ContactClass::Rest), tol),
    ]
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

/// The walls of `x` and `y` declared under `class`, and their discs at
/// `z = H` declared `Rest`, in `x`'s operand order.
fn walls_and_discs(x: &Body<f64>, y: &Body<f64>, class: BooleanCoincidence) -> BooleanDeclarations {
    let curved = |b: &Body<f64>| {
        let mut f = faces_of(b, SurfaceKind::Cylinder);
        f.extend(faces_of(b, SurfaceKind::Sphere));
        f
    };
    let mut d = meeting(x, y, declared(&curved(x), &curved(y), class));
    d.coincident_faces.extend(
        declared(&planes_at_z(x, H), &planes_at_z(y, H), ContactClass::Rest).coincident_faces,
    );
    d
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

/// **The sphere-capped tube builds with its walls declared a `Seam`**
/// and its discs `Rest`, in both member orders. The seam is verified
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
/// ([`a_g1_joint_authored_inside_one_profile_needs_no_declaration`]).
#[test]
fn the_sphere_capped_tube_builds_with_its_walls_declared_a_seam() {
    let tol = Tol::witness();
    let (tube, hemi) = (rod_z(R, 0.0, H), hemisphere_on_the_cap());
    let want = tube_and_half_ball_volume();
    for (label, x, y) in [("tube ∪ cap", &tube, &hemi), ("cap ∪ tube", &hemi, &tube)] {
        let d = walls_and_discs(x, y, BooleanCoincidence::Seam);
        assert_eq!(
            d.coincident_faces.len(),
            5,
            "{label}: two by two walls, each pair meeting the rim, one disc pair"
        );
        let r = topo::union_with(x, y, &d, tol);
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
        assert_eq!(
            records,
            [0, 0, 0, 0],
            "{label}: no contact survives a union"
        );
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
    let d = walls_and_discs(&tube, &hemi, BooleanCoincidence::Seam);
    let e = walls_and_discs(&hemi, &tube, BooleanCoincidence::Seam);
    for (op, r) in [
        ("tube ∖ cap", topo::subtract_with(&tube, &hemi, &d, tol)),
        ("cap ∖ tube", topo::subtract_with(&hemi, &tube, &e, tol)),
        ("tube ∩ cap", topo::intersect_with(&tube, &hemi, &d, tol)),
    ] {
        assert!(
            matches!(r, Err(BooleanError::FallbackExtentUnsupported { .. })),
            "{op}: {r:?}"
        );
    }
}

/// **Every other way of stating the rim refuses, each by its own
/// name**, in both member orders. Undeclared, an edge leaving the rim
/// grazes the partner's wall and the crossing layer refuses, naming the
/// declaration as the recourse; so it does with only the discs declared.
/// A `Tangent` claim on the walls is contradicted by their aligned
/// senses (`contact_tangent_rim_seam`) and steered to the seam; `Rest`
/// and a continuation are contradicted on carrier kind, each under its
/// own type.
#[test]
fn the_sphere_capped_tube_refuses_undeclared_and_under_every_other_class() {
    let tube = rod_z(R, 0.0, H);
    let hemi = hemisphere_on_the_cap();
    let v = topo::mass_properties(&hemi, Tol::witness()).unwrap().volume;
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

    for e in union_both_orders(&tube, &hemi, &cyl, &sph, None) {
        assert!(
            is_pierce(&e) && e.to_string().contains("declare the coincidence"),
            "undeclared: the crossing layer's refusal, naming the recourse: {e:?}"
        );
    }
    for e in union_both_orders(&tube, &hemi, &cap_t, &cap_h, Some(BooleanCoincidence::REST)) {
        assert!(
            is_pierce(&e),
            "the cap discs declared Rest: still the crossing layer: {e:?}"
        );
    }
    for (x, y) in [(&tube, &hemi), (&hemi, &tube)] {
        let run = |class| topo::union_with(x, y, &walls_and_discs(x, y, class), Tol::witness());
        let e = run(BooleanCoincidence::TANGENT).expect_err("Tangent refuses");
        let BooleanError::ContactContradicted { margin, steer, .. } = &e else {
            panic!("the walls declared Tangent: contradicted: {e:?}");
        };
        assert_eq!(margin.predicate, Some("contact_tangent_rim_seam"), "{e}");
        assert_eq!(*steer, Some(topo::SEAM_STEER), "steered to the seam: {e}");
        assert!(e.to_string().contains("which is a `Seam`"), "{e}");
        let e = run(BooleanCoincidence::REST).expect_err("Rest refuses");
        let BooleanError::ContactContradicted { margin, .. } = &e else {
            panic!("the walls declared Rest: contradicted: {e:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("carrier_kind"),
            "on carrier kind: {e}"
        );
        let e = run(BooleanCoincidence::Continuation).expect_err("a continuation refuses");
        let BooleanError::ContinuationContradicted { margin, .. } = &e else {
            panic!("the walls declared a continuation: contradicted: {e:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("carrier_kind"),
            "on carrier kind: {e}"
        );
    }
    for e in union_both_orders(
        &tube,
        &hemi,
        &cap_t,
        &cap_h,
        Some(BooleanCoincidence::TANGENT),
    ) {
        assert!(
            matches!(e, BooleanError::ContactContradicted { .. }),
            "the cap discs declared Tangent: one plane, contradicted: {e:?}"
        );
    }
}

/// **A seam declared where there is none is refused.** The 45° dome on
/// the same rim meets the tube transversely: the witness lane finds the
/// normals definitely apart (`contact_tangent_parallel`) and the seam
/// is contradicted, in both orders and under every op. The 45° cone
/// frustum is refused earlier, at the declaration inventory, on its
/// kind. (The ball seated in its bore, which has no locus to verify a
/// seam along, is `a_ball_seated_in_its_own_bore_refuses_declared_or_not`.)
#[test]
fn a_seam_declared_on_a_transverse_pair_is_contradicted() {
    let tol = Tol::witness();
    let tube = rod_z(R, 0.0, H);
    let dome = dome_on_the_cap();
    let d = walls_and_discs(&tube, &dome, BooleanCoincidence::Seam);
    let e = walls_and_discs(&dome, &tube, BooleanCoincidence::Seam);
    for (op, r) in [
        ("tube ∪ dome", topo::union_with(&tube, &dome, &d, tol)),
        ("dome ∪ tube", topo::union_with(&dome, &tube, &e, tol)),
        ("tube ∖ dome", topo::subtract_with(&tube, &dome, &d, tol)),
        ("dome ∖ tube", topo::subtract_with(&dome, &tube, &e, tol)),
        ("tube ∩ dome", topo::intersect_with(&tube, &dome, &d, tol)),
    ] {
        let Err(BooleanError::SeamContradicted { margin, .. }) = &r else {
            panic!("{op}: the seam is contradicted: {r:?}");
        };
        assert_eq!(margin.predicate, Some("contact_tangent_parallel"), "{op}");
    }
    let frustum = frustum_on_the_cap();
    let cone = faces_of(&frustum, SurfaceKind::Cone);
    let cyl = faces_of(&tube, SurfaceKind::Cylinder);
    let r = topo::union_with(
        &tube,
        &frustum,
        &declared(&cyl, &cone, BooleanCoincidence::Seam),
        tol,
    );
    assert!(
        matches!(r, Err(BooleanError::InvalidDeclaration { .. })),
        "the cone is outside the declaration inventory: {r:?}"
    );
}

#[test]
fn a_dome_abutting_on_the_rim_at_a_transverse_corner_builds_with_its_discs_declared_rest() {
    let tol = Tol::witness();
    let tube = rod_z(R, 0.0, H);
    let dome = dome_on_the_cap();
    let rise = (2.0_f64.sqrt() - 1.0) * R;
    let dome_v = PI * rise * rise * (3.0 * 2.0_f64.sqrt() * R - rise) / 3.0;
    let v = topo::mass_properties(&dome, tol).unwrap().volume;
    assert!(
        (v - dome_v).abs() <= 1e-12 * dome_v,
        "the dome fixture is the spherical cap on the rim: {v} vs {dome_v}"
    );
    // Turned a twelfth of a turn about the axis, the dome's seams miss
    // the tube's, so each rim runs along two of the other's arcs.
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), PI / 6.0);
    let turned = finished(
        "the turned dome",
        topo::transform_rigid(&dome, &turn, tol).unwrap(),
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
        for (order, r) in unions_with_discs_rest(tube, cap).into_iter().enumerate() {
            let b = match r {
                Ok(BooleanResult::Body(b)) => b.body,
                other => panic!("{label}, order {order}: the union builds: {other:?}"),
            };
            topo::validate_geometric(&b, tol)
                .unwrap_or_else(|e| panic!("{label}, order {order}: tier 3: {e:?}"));
            let v = topo::mass_properties(&b, tol).unwrap().volume;
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

/// **A lens**: the dome on an inverted dome of the same rim, discs
/// declared `Rest`. Both rims lie on the other's sphere, at a 90° corner.
/// It builds with the two domes' seams aligned; turned, it refuses.
#[test]
fn a_lens_of_two_domes_builds_with_its_discs_declared_rest() {
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
    let volume = |b: &Body<f64>| topo::mass_properties(b, tol).unwrap().volume;
    let want = 2.0 * volume(&dome);
    assert!(
        (volume(&bowl) - volume(&dome)).abs() <= 1e-12 * want,
        "the bowl is the dome mirrored"
    );
    for (order, r) in unions_with_discs_rest(&bowl, &dome).into_iter().enumerate() {
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
    // Turned on the bowl, each dome rim semicircle runs along parts of
    // two bowl arcs, and the chain certificate needs both of its ends
    // paired: it keeps the door
    // (`work/tang/a-turned-lens-keeps-the-door.md`).
    for angle in [PI / 7.0, PI / 2.0] {
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), angle);
        let turned = finished(
            "the turned dome",
            topo::transform_rigid(&dome, &turn, tol).unwrap(),
            tol,
        );
        for (order, r) in unions_with_discs_rest(&bowl, &turned)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "turned {angle}, order {order}: the crossing layer: {:?}",
                r.err()
            );
        }
    }
}

/// **A rim lying inside the partner's face** passes the crossing layer:
/// a tube ending on a ball of radius `√2` (its rim on the sphere, 45° to
/// the wall), and a tube standing on a torus's 45° latitude. Each union
/// stops in the join, on a frontier that is not the crossing layer's
/// (`work/join/a-tube-ending-on-a-ball-refuses-section-loop-mixed.md`;
/// the torus × plane germ frame, `work/germ/c5-plane-torus-cone-cylinder-arms.md`).
#[test]
fn a_rim_inside_the_partners_face_passes_the_crossing_layer() {
    let tol = Tol::witness();
    let none = BooleanDeclarations::none();
    let ball = finished(
        "the ball",
        ball_poled_z(2.0_f64.sqrt(), Vec3::new(0.0, 0.0, 0.0), tol),
        tol,
    );
    let at0 = revolved_about_y(
        vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut torus = topo::transform_rigid(&at0, &turn, tol).unwrap();
    torus.merge_coplanar_faces(tol).unwrap();
    let torus = finished("the torus", torus, tol);
    let s = core::f64::consts::FRAC_1_SQRT_2;
    for (label, tube, partner) in [
        ("tube on a ball", rod_z(R, 1.0, 2.0), &ball),
        ("tube on a torus", rod_z(2.0 + s, s, 2.0), &torus),
    ] {
        for (order, r) in [
            topo::union_with(&tube, partner, &none, tol),
            topo::union_with(partner, &tube, &none, tol),
        ]
        .into_iter()
        .enumerate()
        {
            assert!(
                matches!(
                    r,
                    Err(
                        BooleanError::Join(topo::SplitJoinError::SectionLoopMixed { .. })
                            | BooleanError::GermFrameUnsupported { .. }
                    )
                ),
                "{label}, order {order}: past the crossing layer, the join's refusal: {:?}",
                r.err()
            );
        }
    }
}

/// Undeclared, the dome's disc and the tube's are one plane by value
/// only, which never glues: the coincidence refuses on that pair.
#[test]
fn a_dome_abutting_on_the_rim_undeclared_refuses_on_its_discs() {
    let tube = rod_z(R, 0.0, H);
    let dome = dome_on_the_cap();
    let (dt, dd) = (planes_at_z(&tube, H), planes_at_z(&dome, H));
    let [ab, ba] = union_both_orders(&tube, &dome, &dt, &dd, None);
    for (order, e, discs) in [(0, ab, (dt[0], dd[0])), (1, ba, (dd[0], dt[0]))] {
        let BooleanError::UndeclaredCoincidence { pair, .. } = e else {
            panic!("order {order}: the discs' undeclared coincidence: {e:?}");
        };
        assert_eq!(
            (pair[0].1, pair[1].1),
            discs,
            "order {order}: the pair is the two discs"
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
    for (order, r) in unions_with_discs_rest(&tube, &dome).into_iter().enumerate() {
        assert!(
            matches!(r, Err(BooleanError::Escalated { .. })),
            "order {order}: an in-band rim escalates: {:?}",
            r.err()
        );
    }
}

#[test]
fn a_cap_abutting_on_the_rim_refuses_at_a_graze_or_as_an_undeclared_continuation() {
    let tube = rod_z(R, 0.0, H);
    let cap_t = planes_at_z(&tube, H);
    let hemi = hemisphere_on_the_cap();
    let cap_h = planes_at_z(&hemi, H);
    // The hemisphere is G1 at the rim (wedge π): the edges LEAVING the
    // rim — the tube's rulings, the hemisphere's meridians — graze the
    // other operand's wall there, and a graze needs a declaration or
    // structure. The refused edge is never the rim circle itself.
    for class in [None, Some(BooleanCoincidence::REST)] {
        let [ab, ba] = union_both_orders(&tube, &hemi, &cap_t, &cap_h, class);
        for (order, e, x) in [(0, ab, &tube), (1, ba, &hemi)] {
            let BooleanError::CurvedPierceUnsupported { edge, .. } = e else {
                panic!("hemisphere, discs {class:?}, order {order}: the crossing layer: {e:?}");
            };
            let leaves_the_rim = match carrier_of(x, edge) {
                geom::Curve3::Line { .. } => true,
                geom::Curve3::Circle { axis, .. } => axis.z.abs() < 0.5,
                _ => false,
            };
            assert!(
                leaves_the_rim,
                "hemisphere, discs {class:?}, order {order}: a ruling or a meridian grazes: {:?}",
                carrier_of(x, edge)
            );
        }
    }
    // The stacked cylinder continues the tube's carrier: its own rim
    // lies on the tube's wall, but its parent wall is that carrier, so
    // the ladder decides it no distinct parent and the door stands.
    // The stacked cylinder continues the tube's carrier: undeclared, or
    // with only its discs declared, its walls are an undeclared
    // continuation, refused at the reduction before the crossing layer.
    let stacked = rod_z(R, H, 1.0);
    let cap_s = planes_at_z(&stacked, H);
    let (wt, ws) = (
        faces_of(&tube, SurfaceKind::Cylinder),
        faces_of(&stacked, SurfaceKind::Cylinder),
    );
    for class in [None, Some(BooleanCoincidence::REST)] {
        for (order, e) in union_both_orders(&tube, &stacked, &cap_t, &cap_s, class)
            .into_iter()
            .enumerate()
        {
            let BooleanError::UndeclaredCoincidence {
                pair: [(_, fa), (_, fb)],
                relation: topo::PlaneRelation::SameOriented,
                ..
            } = e
            else {
                panic!("stacked, discs {class:?}, order {order}: the walls' continuation: {e:?}");
            };
            let walls = if order == 0 { (&wt, &ws) } else { (&ws, &wt) };
            assert!(
                walls.0.contains(&fa) && walls.1.contains(&fb),
                "stacked, discs {class:?}, order {order}: a wall pair"
            );
        }
    }
    for (order, x, y, dx, dy, fx, fy) in [
        (0, &tube, &stacked, &cap_t, &cap_s, &wt, &ws),
        (1, &stacked, &tube, &cap_s, &cap_t, &ws, &wt),
    ] {
        let mut d = declared(dx, dy, ContactClass::Rest);
        for &fa in fx.iter() {
            for &fb in fy.iter() {
                d.coincident_faces
                    .push(FacePairDeclaration::continuation(fa, fb));
            }
        }
        // Declared, the union builds: the zip matches the rim's two
        // semicircles as arcs, and the four wall faces stay unmerged
        // (a curved continuation's merge is skipped).
        let (v, c, k) = built(
            &format!("stacked, order {order}"),
            topo::union_with(x, y, &d, Tol::witness()),
        );
        assert_eq!(k, [0; 4], "stacked, order {order}: no contact records");
        let want = PI * R * R * (H + 1.0);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "stacked, order {order}: the two tubes: {v} vs {want}"
        );
        assert_eq!(c, (6, 10, 6, 1), "stacked, order {order}: F, E, V, shells");
    }
    let cone = frustum_on_the_cap();
    let cap_c = planes_at_z(&cone, H);
    for class in [None, Some(BooleanCoincidence::REST)] {
        for e in union_both_orders(&tube, &cone, &cap_t, &cap_c, class) {
            assert!(
                matches!(
                    e,
                    BooleanError::CurvedPairUnsupported {
                        kind: SurfaceKind::Cone,
                        ..
                    }
                ),
                "frustum, discs {class:?}: the operand gate's refusal: {e:?}"
            );
        }
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
    for e in union_both_orders(&tube, &ball, &[], &[], None) {
        assert!(is_pierce(&e), "tube ∪ ball: {e:?}");
    }
}

#[test]
fn the_stadiums_plane_cylinder_seam_is_contradicted_as_a_tangent() {
    let tol = Tol::witness();
    let slab: AtRestBody<f64> = finished(
        "the slab",
        brick((-1.0, 2.0), (0.0, 1.0), (-0.5, 0.5), tol),
        tol,
    );
    let lp = profile::circle(Point2::new(2.0, 0.0), 0.5, tol).unwrap();
    // Sketch normal onto +y: the rod's axis is the line x = 2, z = 0.
    let plane = profile::SketchPlane::new(Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::unit_x(),
        -PI / 2.0,
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
    for e in union_both_orders(&slab, &rod, &[], &[], None) {
        // Which of the two the run's band lands on is ε-dependent: the
        // `B ∪ A` order escalates at the default ε on a contact-vertex
        // margin and pierces at 1e-6 and 1e-12.
        assert!(
            is_pierce(&e) || matches!(e, BooleanError::Escalated { .. }),
            "undeclared: the crossing layer's refusal: {e:?}"
        );
    }
    let mut flats = planes_at_z(&slab, 0.5);
    flats.extend(planes_at_z(&slab, -0.5));
    let walls = faces_of(&rod, SurfaceKind::Cylinder);
    for e in union_both_orders(
        &slab,
        &rod,
        &flats,
        &walls,
        Some(BooleanCoincidence::TANGENT),
    ) {
        let BooleanError::ContactContradicted { margin, .. } = &e else {
            panic!("declared Tangent: contradicted: {e:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("contact_tangent_opposed"),
            "the seam's outward normals agree, which the witness lane reads as \
             containment: {e}"
        );
    }
    // Declared a `Seam`, the lane finds the outward normals aligned,
    // and then that no boundary edge of the rod's wall runs along the
    // ruling: the ruling crosses the wall's interior, half of which
    // runs inside the slab. A face that does not end at the line
    // leaves it on no one side, so this is no seam.
    for e in union_both_orders(&slab, &rod, &flats, &walls, Some(BooleanCoincidence::Seam)) {
        let BooleanError::SeamContradicted { margin, .. } = &e else {
            panic!("declared Seam: contradicted: {e:?}");
        };
        assert_eq!(margin.predicate, Some("seam_locus_no_edge"), "{e}");
    }
}

/// The planar faces of `b` with their origin and OUTWARD normal.
fn planar_faces(b: &Body<f64>) -> Vec<(FaceKey, Point3<f64>, Vec3<f64>)> {
    b.faces()
        .filter_map(|(k, f)| match b.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                Some((k, *origin, *normal * if f.sense { 1.0 } else { -1.0 }))
            }
            _ => None,
        })
        .collect()
}

/// Every coplanar planar face pair across `x` and `y`, declared `Rest`
/// where the outward normals oppose and a continuation where they agree.
fn coplanar_pairs(x: &Body<f64>, y: &Body<f64>) -> Vec<FacePairDeclaration> {
    let mut out = Vec::new();
    for (fa, oa, na) in planar_faces(x) {
        for (fb, ob, nb) in planar_faces(y) {
            if na.cross(nb).norm() < 1e-9 && (ob - oa).dot(na).abs() < 1e-9 {
                out.push(if na.dot(nb) > 0.0 {
                    FacePairDeclaration::continuation(fa, fb)
                } else {
                    FacePairDeclaration::rest(fa, fb)
                });
            }
        }
    }
    out
}

/// **A line seam that IS one: the D-bar on the slab.** The half-rod
/// whose flat lies on the slab's side face (`Rest`) and whose wall
/// starts at the slab's top and bottom edges leaves each tangent ruling
/// on the side away from the slab, so its walls declared a `Seam`
/// against the slab's top and bottom verify in both orders, and the
/// union builds: the slab with its `+x` side bent round into a half
/// cylinder, tier 3 and 3′ clean, its volume the slab's plus the half
/// rod's, census (6, 12, 8, 1).
#[test]
fn a_d_bar_on_the_slab_builds_with_its_line_seams_declared() {
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
    let plane = profile::SketchPlane::new(Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::unit_x(),
        -PI / 2.0,
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
    let v = topo::mass_properties(&dbar, tol).unwrap().volume;
    assert!(
        (v - half_rod).abs() <= 1e-12,
        "the half rod: {v} vs {half_rod}"
    );
    let flats = |b: &Body<f64>| {
        let mut f = planes_at_z(b, 0.5);
        f.extend(planes_at_z(b, -0.5));
        f
    };
    for (x, y) in [(&slab, &dbar), (&dbar, &slab)] {
        let mut d = declared(
            &flats(x),
            &faces_of(y, SurfaceKind::Cylinder),
            BooleanCoincidence::Seam,
        );
        d.coincident_faces.extend(
            declared(
                &faces_of(x, SurfaceKind::Cylinder),
                &flats(y),
                BooleanCoincidence::Seam,
            )
            .coincident_faces,
        );
        d.coincident_faces.extend(coplanar_pairs(x, y));
        assert_eq!(
            d.coincident_faces.len(),
            5,
            "two seams, the flat, two flush ends"
        );
        let label = "slab ∪ D-bar";
        let (v, c, _) = built(label, topo::union_with(x, y, &d, tol));
        let want = 3.0 + half_rod;
        assert!((v - want).abs() <= 1e-12, "{label}: {v} vs {want}");
        assert_eq!(
            c,
            (6, 12, 8, 1),
            "{label}: the slab with one wall bent round"
        );
    }
}

/// **The same rim, the same aligned normals, and no seam: a cusp.** A
/// lower half ball hanging in the tube's mouth (the bowl), and a whole
/// ball split at its equator on the rim, each meet the tube's wall G1
/// along the rim with outward normals aligned. But the bowl and the
/// ball's lower half leave the rim DOWNWARD, as the tube's wall does,
/// so both materials lie on one side: a nested touch, wedge 0, not the
/// π seam. Declared a `Seam` each is contradicted (`seam_cusp`), in
/// both orders. Declared `Tangent`, the bowl is contradicted by its
/// aligned senses without a steer, since no declaration fits it
/// (`contact_tangent_rim_nested`).
#[test]
fn a_bowl_or_a_split_ball_in_the_tubes_mouth_is_a_cusp_not_a_seam() {
    let tol = Tol::witness();
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let tube = rod_z(R, 0.0, H);
    let bowl = cap_on_the_tube(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(0.0, -R), q),
        (Point2::new(R, 0.0), 0.0),
    ]);
    let split = cap_on_the_tube(vec![
        (Point2::new(0.0, -R), q),
        (Point2::new(R, 0.0), q),
        (Point2::new(0.0, R), 0.0),
    ]);
    let half_ball = 2.0 / 3.0 * PI * R.powi(3);
    for (label, cap, want, faces) in [
        ("bowl", &bowl, half_ball, 3),
        ("split", &split, 2.0 * half_ball, 4),
    ] {
        let v = topo::mass_properties(cap, tol).unwrap().volume;
        assert!((v - want).abs() <= 1e-12 * want, "{label}: {v} vs {want}");
        assert_eq!(census(cap).0, faces, "{label}: its faces");
    }
    let walls = |b: &Body<f64>| {
        let mut f = faces_of(b, SurfaceKind::Cylinder);
        f.extend(faces_of(b, SurfaceKind::Sphere));
        f
    };
    for (label, cap) in [("bowl", &bowl), ("split", &split)] {
        for (x, y) in [(&tube, cap), (cap, &tube)] {
            let run = |class| {
                let mut d = meeting(x, y, declared(&walls(x), &walls(y), class));
                d.coincident_faces.extend(coplanar_pairs(x, y));
                topo::union_with(x, y, &d, tol)
            };
            let r = run(BooleanCoincidence::Seam);
            let Err(BooleanError::SeamContradicted { margin, .. }) = &r else {
                panic!("{label}: declared Seam, contradicted: {r:?}");
            };
            assert_eq!(margin.predicate, Some("seam_cusp"), "{label}");
            if label == "bowl" {
                let r = run(BooleanCoincidence::TANGENT);
                let Err(BooleanError::ContactContradicted { margin, steer, .. }) = &r else {
                    panic!("bowl: declared Tangent, contradicted: {r:?}");
                };
                assert_eq!(margin.predicate, Some("contact_tangent_rim_nested"));
                assert_eq!(*steer, None, "no declaration fits a nested touch");
            }
        }
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
fn a_puck_and_its_rounding_ring_build_with_the_top_declared_a_seam() {
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
    let mut ring = topo::transform_rigid(&ring, &turn, tol).unwrap();
    ring.merge_coplanar_faces(tol).unwrap();
    let ring = finished("the ring", ring, tol);
    let puck = rod_z(a, 0.0, r);
    let want =
        PI * a * a * r + (PI * r * r / 4.0) * core::f64::consts::TAU * (a + 4.0 * r / (3.0 * PI));
    for (label, x, y) in [("puck ∪ ring", &puck, &ring), ("ring ∪ puck", &ring, &puck)] {
        let mut d = meeting(
            x,
            y,
            declared(
                &planes_at_z(x, r),
                &faces_of(y, SurfaceKind::Torus),
                BooleanCoincidence::Seam,
            ),
        );
        d.coincident_faces.extend(
            meeting(
                x,
                y,
                declared(
                    &faces_of(x, SurfaceKind::Torus),
                    &planes_at_z(y, r),
                    BooleanCoincidence::Seam,
                ),
            )
            .coincident_faces,
        );
        d.coincident_faces.extend(
            declared(
                &faces_of(x, SurfaceKind::Cylinder),
                &faces_of(y, SurfaceKind::Cylinder),
                ContactClass::Rest,
            )
            .coincident_faces,
        );
        d.coincident_faces.extend(coplanar_pairs(x, y));
        let (v, c, _) = built(label, topo::union_with(x, y, &d, tol));
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: {v} vs Pappus {want}"
        );
        assert_eq!(c, (3, 4, 3, 1), "{label}: top disc, torus, floor");
    }
}

/// **A rod in a bore, declared `Tangent`, refuses at the crossing
/// layer at every turn.** A rod of radius 0.5 seated in the bore of
/// radius 1 of a revolved tube, touching along a ruling, at thirty
/// turns of the touching ruling: pinned per refusal. Its covered ends
/// lie on the certified side, so this row holds with or without the
/// cover's side gate; the gate is carried by the unit row
/// `a_cover_admits_an_off_point_only_on_its_certified_side`, which is
/// the only place a certificate's wrong side is reachable.
#[test]
fn a_rod_in_a_bore_declared_tangent_refuses_at_the_crossing_layer() {
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
        topo::transform_rigid(&tube, &turn, tol).unwrap(),
        tol,
    );
    let bore: Vec<FaceKey> = faces_of(&tube, SurfaceKind::Cylinder)
        .into_iter()
        .filter(|&f| {
            matches!(
                tube.get_surface(tube.get_face(f).unwrap().surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (*radius - 1.0).abs() < 1e-12
            )
        })
        .collect();
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for k in 0..30 {
        let th = f64::from(k) * core::f64::consts::TAU / 30.0;
        let at = Affine3::translation(Vec3::new(0.5 * th.cos(), 0.5 * th.sin(), 0.0));
        let rod = finished(
            "the rod",
            topo::transform_rigid(&rod_z(0.5, -0.5, 1.0), &at, tol).unwrap(),
            tol,
        );
        let d = declared(
            &bore,
            &faces_of(&rod, SurfaceKind::Cylinder),
            ContactClass::Tangent,
        );
        let key = match topo::union_with(&tube, &rod, &d, tol) {
            Ok(_) => "built".to_owned(),
            Err(e) => format!("{:?}", e.kind()),
        };
        *kinds.entry(key).or_default() += 1;
    }
    assert_eq!(
        kinds,
        [
            ("CurvedBooleanUnsupported".to_owned(), 1),
            ("CurvedPierceUnsupported".to_owned(), 29)
        ]
        .into_iter()
        .collect(),
    );
}

/// **The seam's own refusals.** Two stacked rods of one radius share
/// one carrier: a seam declared on their walls is contradicted as
/// conformal. The fact names the finding (`OneCarrier`), and the margin
/// keeps the predicate the carrier ladder measured it with. A rod hovering a clear gap above a slab
/// has no tangency: the closed-form locus finds the gap
/// (`pc_parallel_gap`).
#[test]
fn a_seam_on_one_carrier_or_across_a_gap_is_contradicted() {
    let tol = Tol::witness();
    let (low, high) = (rod_z(R, 0.0, H), rod_z(R, H, H));
    let cyl = |b: &Body<f64>| faces_of(b, SurfaceKind::Cylinder);
    for (x, y) in [(&low, &high), (&high, &low)] {
        let mut d = declared(&cyl(x), &cyl(y), BooleanCoincidence::Seam);
        d.coincident_faces.extend(coplanar_pairs(x, y));
        let r = topo::union_with(x, y, &d, tol);
        let Err(BooleanError::SeamContradicted { margin, fact, .. }) = &r else {
            panic!("stacked rods: contradicted: {r:?}");
        };
        assert_eq!(*fact, Some(topo::Contradiction::OneCarrier), "{r:?}");
        assert_eq!(margin.predicate, Some("carrier_cyl_axis_parallel"), "{r:?}");
        assert!(
            r.as_ref()
                .unwrap_err()
                .to_string()
                .contains("lie on one carrier"),
            "the message says it in words"
        );
    }
    let slab: AtRestBody<f64> = finished(
        "the slab",
        brick((-1.0, 2.0), (0.0, 1.0), (-0.5, 0.5), tol),
        tol,
    );
    let lp = profile::circle(Point2::new(2.0, -1.5), 0.5, tol).unwrap();
    let plane = profile::SketchPlane::new(Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::unit_x(),
        -PI / 2.0,
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
    for (x, y, fx, fy) in [
        (&slab, &rod, planes_at_z(&slab, 0.5), cyl(&rod)),
        (&rod, &slab, cyl(&rod), planes_at_z(&slab, 0.5)),
    ] {
        let r = topo::union_with(x, y, &declared(&fx, &fy, BooleanCoincidence::Seam), tol);
        let Err(BooleanError::SeamContradicted { margin, .. }) = &r else {
            panic!("a rod above the slab: contradicted: {r:?}");
        };
        assert_eq!(margin.predicate, Some("pc_parallel_gap"), "{r:?}");
    }
}

#[test]
fn a_ball_seated_in_its_own_bore_refuses_declared_or_not() {
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
    let bore = faces_of(&bored, SurfaceKind::Cylinder);
    let sph = faces_of(&ball, SurfaceKind::Sphere);
    for e in union_both_orders(&bored, &ball, &bore, &sph, None) {
        // The ball's meridian circles touch the bore wall at their two
        // equator points: a tangency, which the circle × cylinder lane's
        // ladder escalates on its own decision when its margin lands in
        // the band's gap, and which keeps the pierce door otherwise.
        assert!(
            is_pierce(&e)
                || matches!(
                    e,
                    BooleanError::Escalated {
                        decision: topo::BooleanDecision::ArcCylinderRoots,
                        ..
                    }
                ),
            "undeclared: the crossing layer's refusal: {e:?}"
        );
    }
    for e in union_both_orders(
        &bored,
        &ball,
        &bore,
        &sph,
        Some(BooleanCoincidence::TANGENT),
    ) {
        assert!(
            matches!(
                e,
                BooleanError::UnsupportedDeclarationClass {
                    class: topo::BooleanCoincidence::TANGENT
                }
            ),
            "declared Tangent: the class refusal: {e:?}"
        );
    }
    for e in union_both_orders(&bored, &ball, &bore, &sph, Some(BooleanCoincidence::Seam)) {
        assert!(
            matches!(
                e,
                BooleanError::UnsupportedDeclarationClass {
                    class: BooleanCoincidence::Seam
                }
            ),
            "declared Seam: no rim and no closed-form locus, the class refusal: {e:?}"
        );
    }
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
type Built = (f64, (usize, usize, usize, usize), [usize; 4]);

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
    let c = &bb.contacts;
    (
        topo::mass_properties(b, tol).unwrap().volume,
        census(b),
        [
            c.vv.len(),
            c.a_on_b.len() + c.b_on_a.len(),
            c.curves.len(),
            c.patches.len(),
        ],
    )
}

/// **A dome sunk into the tube, undeclared**: the dome lowered by `dz`
/// puts its base disc inside the tube and its rim circle on the tube's
/// wall. The tube's two wall faces are bounded by seam rulings at
/// `±x`, where the rim's own vertices are, so each rim semicircle lies
/// wholly inside one face (certificate (a)) and its ends are recorded.
/// The union is the tube plus the dome's cap above `z = H`.
///
/// Turned a twelfth of a turn, each semicircle crosses a seam ruling
/// mid-arc: a crossing neither certificate places, so it keeps the door
/// (`work/tang/a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door.md`).
#[test]
fn a_dome_sunk_into_the_tube_builds_undeclared() {
    let tol = Tol::witness();
    let none = BooleanDeclarations::none();
    let tube = rod_z(R, 0.0, H);
    let rho = 2.0_f64.sqrt() * R;
    for dz in [-1e-3, -0.3] {
        let lift = Affine3::translation(Vec3::new(0.0, 0.0, dz));
        let dome = finished(
            "the sunk dome",
            topo::transform_rigid(&dome_on_the_cap(), &lift, tol).unwrap(),
            tol,
        );
        let want = PI * R * R * H + cap_volume(rho, rho - R + dz);
        for (order, r) in [
            topo::union_with(&tube, &dome, &none, tol),
            topo::union_with(&dome, &tube, &none, tol),
        ]
        .into_iter()
        .enumerate()
        {
            let label = format!("dz = {dz}, order {order}");
            let (v, c, k) = built(&label, r);
            assert_eq!(k, [0; 4], "{label}: no contact records");
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the tube and the cap above it: {v} vs {want}"
            );
            // Two valence-2 vertices stay on the tube's seam rulings at
            // the dissolved rims; minimal is (6, 10, 7)
            // (`work/tang/a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings.md`).
            assert_eq!(c, (6, 12, 9, 1), "{label}: F, E, V, shells");
        }
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), PI / 6.0);
        let turned = finished(
            "the turned dome",
            topo::transform_rigid(&dome, &turn, tol).unwrap(),
            tol,
        );
        for e in union_both_orders(&tube, &turned, &[], &[], None) {
            assert!(
                is_pierce(&e),
                "dz = {dz}, turned: the crossing layer: {e:?}"
            );
        }
    }
}

/// **A tube poking through the dome's base, undeclared**: the tube runs
/// to `z = 2.2` and the dome stands on `z = H`, so the dome's rim lies on
/// the tube's wall inside that face (certificate (a)) and the tube's top
/// disc cuts the dome. Every op builds at its closed form: the dome
/// above the disc is a spherical cap of height `1 + √2 − 2.2`.
///
/// `t ∖ d` touches itself along the rim: the bowl the dome leaves in
/// the tube meets the tube's wall along the whole rim circle. The result
/// holds two vertices at each of the circle's vertices `(±1, 0, 2)` and
/// records the touch as two v-v contacts, valid at tier 3′. The two unions keep valence-2 vertices on the seam rulings
/// (`work/tang/a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings.md`).
#[test]
fn a_tube_through_the_domes_base_builds_every_op_undeclared() {
    let tol = Tol::witness();
    let none = BooleanDeclarations::none();
    let tall = rod_z(R, 0.0, 2.2);
    let dome = dome_on_the_cap();
    let rho = 2.0_f64.sqrt() * R;
    let above = cap_volume(rho, R + rho - 2.2);
    let inside = cap_volume(rho, rho - R) - above;
    let tube = PI * R * R * 2.2;
    for (label, r, want, census, contacts) in [
        (
            "t ∪ d",
            topo::union_with(&tall, &dome, &none, tol),
            tube + above,
            (6, 12, 9, 1),
            [0; 4],
        ),
        (
            "d ∪ t",
            topo::union_with(&dome, &tall, &none, tol),
            tube + above,
            (6, 12, 9, 1),
            [0; 4],
        ),
        (
            "t ∖ d",
            topo::subtract_with(&tall, &dome, &none, tol),
            tube - inside,
            (7, 14, 10, 1),
            [2, 0, 0, 0],
        ),
        (
            "d ∖ t",
            topo::subtract_with(&dome, &tall, &none, tol),
            above,
            (3, 4, 3, 1),
            [0; 4],
        ),
        (
            "t ∩ d",
            topo::intersect_with(&tall, &dome, &none, tol),
            inside,
            (4, 6, 4, 1),
            [0; 4],
        ),
        (
            "d ∩ t",
            topo::intersect_with(&dome, &tall, &none, tol),
            inside,
            (4, 6, 4, 1),
            [0; 4],
        ),
    ] {
        let (v, c, k) = built(label, r);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: the closed form: {v} vs {want}"
        );
        assert_eq!(c, census, "{label}: F, E, V, shells");
        assert_eq!(k, contacts, "{label}: [v-v, v-f, curve, patch] records");
    }
}

/// **A G1 joint authored inside one profile needs no declaration**: it
/// is the structural form of the seam. The capsule revolved from one
/// profile — the tube's side, then a quarter arc tangent to it, the
/// joint authored in the profile's `tangent_joints` — builds with its
/// joint minted `TangentIntersection`, and it is the declared seam's
/// union: the same census and the same volume.
#[test]
fn a_g1_joint_authored_inside_one_profile_needs_no_declaration() {
    use profile::RawLoop;
    let tol = Tol::witness();
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), 0.0),
        (Point2::new(R, H), bulge),
        (Point2::new(0.0, H + R), 0.0),
    ])
    .with_tangent_joints(vec![2]);
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
    let mut capsule = topo::transform_rigid(&at0, &turn, tol).unwrap();
    capsule.merge_coplanar_faces(tol).unwrap();
    topo::validate_geometric(&capsule, tol).unwrap_or_else(|e| panic!("tier 3: {e:?}"));
    let v = topo::mass_properties(&capsule, tol).unwrap().volume;
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
    let plane = profile::SketchPlane::new(Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(k, k, k),
        core::f64::consts::TAU / 3.0,
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
/// vertex and arc midpoint reads `y ≥ 0`. A quarter rod on `y < 0`
/// tangent to the bottom along the line leaves it on the same side, a
/// cusp: declared a `Seam`, contradicted (`seam_cusp`). On `y > 0`
/// it leaves on the other side, a seam: the door verifies it, and the
/// union goes on past the declarations. The square plate, which lies on
/// `y < 0` everywhere, is the control. Both member orders.
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
        for (pose, side) in [("cusp", -1.0), ("seam", 1.0)] {
            let rod = quarter_rod(side, tol);
            for (x, y) in [(&plate, &rod), (&rod, &plate)] {
                let mut d = declared(
                    &planes_at_z(x, 0.0),
                    &faces_of(y, SurfaceKind::Cylinder),
                    BooleanCoincidence::Seam,
                );
                d.coincident_faces.extend(
                    declared(
                        &faces_of(x, SurfaceKind::Cylinder),
                        &planes_at_z(y, 0.0),
                        BooleanCoincidence::Seam,
                    )
                    .coincident_faces,
                );
                d.coincident_faces.extend(coplanar_pairs(x, y));
                let r = topo::union_with(x, y, &d, tol);
                match pose {
                    "cusp" => {
                        let Err(BooleanError::SeamContradicted { margin, .. }) = &r else {
                            panic!("{label} {pose}: contradicted: {r:?}");
                        };
                        assert_eq!(margin.predicate, Some("seam_cusp"), "{label} {pose}");
                    }
                    // Verified; the square plate and the rod are
                    // disjoint but for the line, and the union is both.
                    _ if label == "square" => {
                        let (v, _, _) = built(label, r);
                        let want = 0.25 + PI * 0.25 / 4.0;
                        assert!((v - want).abs() <= 1e-12, "{label} {pose}: {v} vs {want}");
                    }
                    // Verified; the dodge plate's outline also reaches
                    // over the rod on `y > 0`, and that crossing stops
                    // at the crossing layer, past the declarations.
                    _ => assert!(
                        matches!(r, Err(BooleanError::CurvedPierceUnsupported { .. })),
                        "{label} {pose}: verified at the door: {r:?}"
                    ),
                }
            }
        }
    }
}

/// A plate on `z ∈ [0, 0.25]` extruded from the polygon `outline` in
/// the `xy` plane.
fn plate(outline: &[(f64, f64)], tol: Tol) -> AtRestBody<f64> {
    let lp = profile::test_support::bulge_loop(
        outline
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
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
    finished("the plate", plate, tol)
}

/// The plate's bottom against the rod's wall, declared a `Seam` in
/// `x`'s operand order, and nothing else.
fn bottom_and_wall(x: &Body<f64>, y: &Body<f64>) -> BooleanDeclarations {
    let mut d = declared(
        &planes_at_z(x, 0.0),
        &faces_of(y, SurfaceKind::Cylinder),
        BooleanCoincidence::Seam,
    );
    d.coincident_faces.extend(
        declared(
            &faces_of(x, SurfaceKind::Cylinder),
            &planes_at_z(y, 0.0),
            BooleanCoincidence::Seam,
        )
        .coincident_faces,
    );
    d
}

/// The seam's verdict on `plate` against `rod`, both member orders, as
/// the refusal's fact and label (`None` where the door verified).
fn seam_verdicts(
    plate: &AtRestBody<f64>,
    rod: &AtRestBody<f64>,
) -> Vec<Option<(Option<topo::Contradiction>, Option<&'static str>)>> {
    [(plate, rod), (rod, plate)]
        .into_iter()
        .map(
            |(x, y)| match topo::union_with(x, y, &bottom_and_wall(x, y), Tol::witness()) {
                Err(BooleanError::SeamContradicted { fact, margin, .. }) => {
                    Some((fact, margin.predicate))
                }
                _ => None,
            },
        )
        .collect()
}

/// **The side is read where the two faces touch.** A face's edge on
/// the line names its side only where the face ENDS at the line, so an
/// edge elsewhere on the same line must not stand in for it. The rod is
/// a quarter rod on `y > 0` along `x ∈ [0, 1]`; only the plate's bottom
/// and the rod's wall are declared a `Seam`, in both orders:
///
/// - the TAB plate covers `x ∈ [−0.5, 2]` across the line and has its
///   only edge on it at `x ∈ [2, 3]`: where the rod touches it, the line
///   runs through the plate's interior (`seam_locus_no_edge`);
/// - the PARTIAL plate has an edge on the line at `x ∈ [0.5, 1]` and
///   runs on across it at `x ∈ [0, 0.5]`: the edge overlaps the touch
///   and still does not cover it (`seam_locus_no_edge`);
/// - the FAR plate (`x ∈ [2, 3]`, `y < 0`) never meets the rod, on
///   either side of it (`seam_locus_untouched`).
/// - the NOTCH plate rides the line at `x ∈ [0, 0.4]` below it, leaves
///   it out through a notch opening upward on `(0.4, 0.8)`, and contains
///   it again from `x = 0.8`, where the notch's diagonal edge
///   `(0.6, −0.5) → (1, 0.5)` crosses the line with no vertex there: only
///   the crossing's own cut finds the plate's interior under the rod on
///   `[0.8, 1]` (`seam_locus_no_edge`).
#[test]
fn a_line_seam_is_read_only_where_the_faces_touch() {
    let tol = Tol::witness();
    let tab = plate(
        &[
            (-0.5, -1.0),
            (3.0, -1.0),
            (3.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (-0.5, 1.0),
        ],
        tol,
    );
    let partial = plate(
        &[
            (1.0, 0.0),
            (0.5, 0.0),
            (0.5, 1.0),
            (-0.5, 1.0),
            (-0.5, -1.0),
            (1.0, -1.0),
        ],
        tol,
    );
    let far = plate(&[(2.0, -1.0), (3.0, -1.0), (3.0, 0.0), (2.0, 0.0)], tol);
    let notch = plate(
        &[
            (0.0, 0.0),
            (0.0, -1.0),
            (1.5, -1.0),
            (1.5, 0.5),
            (1.0, 0.5),
            (0.6, -0.5),
            (0.4, -0.5),
            (0.4, 0.0),
        ],
        tol,
    );
    let no_edge = Some((
        Some(topo::Contradiction::SeamFaceRunsOn),
        Some("seam_locus_no_edge"),
    ));
    let untouched = Some((
        Some(topo::Contradiction::SeamUntouched),
        Some("seam_locus_untouched"),
    ));
    let above = quarter_rod(1.0, tol);
    for (label, body, want) in [
        ("tab", &tab, &no_edge),
        ("partial", &partial, &no_edge),
        ("notch", &notch, &no_edge),
    ] {
        assert_eq!(seam_verdicts(body, &above), vec![*want; 2], "{label}");
    }
    for side in [1.0, -1.0] {
        let rod = quarter_rod(side, tol);
        assert_eq!(
            seam_verdicts(&far, &rod),
            vec![untouched; 2],
            "far, rod on {side}"
        );
    }
}

/// **Opposite sides along one stretch, the same along another.** The
/// plate's outline rides the line `y = z = 0` at `x ∈ [0, 1]` with the
/// plate below it, and at `x ∈ [2, 3]` with the plate above, the two
/// lobes joined below the line and round past `x = 3`. A quarter rod on
/// `y > 0` along `x ∈ [0, 3]` meets the first stretch as a seam and the
/// second as a cusp: declared a `Seam`, contradicted as mixed, with its
/// fact, in both orders.
#[test]
fn a_seam_whose_faces_leave_on_both_sides_is_contradicted_as_mixed() {
    let tol = Tol::witness();
    let lobes = plate(
        &[
            (0.0, 0.0),
            (0.0, -1.0),
            (4.0, -1.0),
            (4.0, 1.0),
            (2.0, 1.0),
            (2.0, 0.0),
            (3.0, 0.0),
            (3.0, -0.5),
            (1.0, -0.5),
            (1.0, 0.0),
        ],
        tol,
    );
    let rod = quarter_rod_of(1.0, 3.0, tol);
    let mixed = Some((
        Some(topo::Contradiction::SeamSidesMixed),
        Some("seam_sides_mixed"),
    ));
    assert_eq!(seam_verdicts(&lobes, &rod), vec![mixed; 2]);
}

/// **The suite's pair filter keeps every pair that meets, turned or
/// not.** The hemisphere turned 0.3 rad about the axis: each half-wall
/// shares a stretch of the rim with BOTH half-caps, so all four wall
/// pairs meet, and the filter keeps them with the disc pair (5), as it
/// does unturned, where the cross pairs meet at the rim's vertices.
#[test]
fn the_seam_pair_filter_keeps_every_pair_that_meets() {
    let tol = Tol::witness();
    let tube = rod_z(R, 0.0, H);
    for turn in [0.0, 0.3] {
        let hemi = topo::transform_rigid(
            &hemisphere_on_the_cap(),
            &Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), turn),
            tol,
        )
        .unwrap();
        for (x, y) in [(&*tube, &hemi), (&hemi, &*tube)] {
            let d = walls_and_discs(x, y, BooleanCoincidence::Seam);
            assert_eq!(d.coincident_faces.len(), 5, "turned {turn}");
        }
    }
}

/// Narrow review (JOIN-1 fix pass 3): domes and bowls of several corner
/// angles, turned, on tubes, every op both orders, discs declared
/// `Rest`; every build read at tiers 2, 3′, the certificate, tier 3,
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
                topo::transform_rigid(&dome, &turn, tol).unwrap(),
                tol,
            );
            for (plabel, partner) in [
                ("tube", rod_z(R, 0.0, H)),
                ("short tube", rod_z(R, H - 0.25, 0.25)),
                ("bowl", bowl.clone()),
            ] {
                for (x, y, xl, yl) in [(&partner, &tdome, "p", "d"), (&tdome, &partner, "d", "p")] {
                    let (dx, dy) = (planes_at_z(x, H), planes_at_z(y, H));
                    let d = declared(&dx, &dy, ContactClass::Rest);
                    for (op, r) in [
                        ("union", topo::union_with(x, y, &d, tol)),
                        ("subtract", topo::subtract_with(x, y, &d, tol)),
                        ("intersect", topo::intersect_with(x, y, &d, tol)),
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
                                let v = topo::mass_properties(&bb.body, tol)
                                    .map(|m| m.volume)
                                    .unwrap_or(f64::NAN);
                                let vx = topo::mass_properties(x, tol).unwrap().volume;
                                let vy = topo::mass_properties(y, tol).unwrap().volume;
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
                        let v = topo::mass_properties(&bb.body, tol)
                            .map(|m| m.volume)
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
