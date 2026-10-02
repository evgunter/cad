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
//!   bottom are tangent to along a ruling): a plane×cylinder π seam.
//!   Undeclared it stops at the crossing layer; declared `Tangent` the
//!   witness lane contradicts it, because its outward normals AGREE;
//!   declared a `Seam` it verifies and stops at the curved lump site.
//! - **The kiss** (a ball seated in a bore of its own radius): the
//!   equator touches the bore wall, wedge 2π. Undeclared it stops at
//!   the crossing layer; declared `Tangent` or a `Seam` the class is
//!   refused — the two faces share no boundary circle, so the rim
//!   routing never runs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick, revolved_about_y};
use sweep::{Extrusion, Revolution, extrude};
use topo::{
    Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult, ContactClass,
    FaceKey, FacePairDeclaration,
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
    a: &Body<f64>,
    b: &Body<f64>,
    fa: &[FaceKey],
    fb: &[FaceKey],
    class: Option<BooleanCoincidence>,
) -> [BooleanError; 2] {
    let run = |x: &Body<f64>, y: &Body<f64>, fx: &[FaceKey], fy: &[FaceKey]| {
        let r = match class {
            None => topo::union(x, y, Tol::witness()),
            Some(c) => topo::union_with(x, y, &declared(fx, fy, c), Tol::witness()),
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
fn rod_z(r: f64, z0: f64, len: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let p = profile::Profile::new(plane, vec![lp.into()])
        .validate(tol)
        .unwrap();
    extrude(&p, Extrusion::Distance(len), tol).unwrap().body
}

/// A revolved cap standing on `z = H`: the profile (sketch x radial,
/// sketch y axial from the cap's base) revolved, turned onto `+z`,
/// lifted, and its base disc's two revolve halves merged (the boolean
/// refuses a non-maximal operand).
fn cap_on_the_tube(profile: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    let tol = Tol::witness();
    let at0 = revolved_about_y(profile, Revolution::Full, tol);
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let turned = topo::transform_rigid(&at0, &turn, tol).unwrap();
    let mut cap =
        topo::transform_rigid(&turned, &Affine3::translation(Vec3::new(0.0, 0.0, H)), tol).unwrap();
    cap.merge_coplanar_faces(tol).unwrap();
    cap
}

/// A solid hemisphere of radius [`R`] standing on `z = H`.
fn hemisphere_on_the_cap() -> Body<f64> {
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
fn frustum_on_the_cap() -> Body<f64> {
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
fn dome_on_the_cap() -> Body<f64> {
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
    tube: &Body<f64>,
    cap: &Body<f64>,
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
fn walls_and_discs(
    x: &Body<f64>,
    y: &Body<f64>,
    class: BooleanCoincidence,
) -> BooleanDeclarations {
    let curved = |b: &Body<f64>| {
        let mut f = faces_of(b, SurfaceKind::Cylinder);
        f.extend(faces_of(b, SurfaceKind::Sphere));
        f
    };
    let mut d = declared(&curved(x), &curved(y), class);
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
        assert_eq!(d.coincident_faces.len(), 5, "{label}: two by two walls, one disc pair");
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
        assert_eq!(records, [0, 0, 0, 0], "{label}: no contact survives a union");
        let rims = tangent_intersections(&body);
        assert_eq!(rims.len(), 2, "{label}: the rim's two semicircles: {rims:?}");
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
        assert_eq!(margin.predicate, Some("carrier_kind"), "on carrier kind: {e}");
        let e = run(BooleanCoincidence::Continuation).expect_err("a continuation refuses");
        let BooleanError::ContinuationContradicted { margin, .. } = &e else {
            panic!("the walls declared a continuation: contradicted: {e:?}");
        };
        assert_eq!(margin.predicate, Some("carrier_kind"), "on carrier kind: {e}");
    }
    for e in union_both_orders(&tube, &hemi, &cap_t, &cap_h, Some(BooleanCoincidence::TANGENT)) {
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
/// kind; and the ball seated in a bore of its own radius shares no rim
/// with it and has no closed-form locus, so the class is refused.
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
    let turned = topo::transform_rigid(&dome, &turn, tol).unwrap();
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
        let turned = topo::transform_rigid(&dome, &turn, tol).unwrap();
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
    let ball = ball_poled_z(2.0_f64.sqrt(), Vec3::new(0.0, 0.0, 0.0), tol);
    let at0 = revolved_about_y(
        vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let mut torus = topo::transform_rigid(&at0, &turn, tol).unwrap();
    torus.merge_coplanar_faces(tol).unwrap();
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
    let ball = ball_poled_z(R, Vec3::new(0.0, 0.0, H), Tol::witness());
    for e in union_both_orders(&tube, &ball, &[], &[], None) {
        assert!(is_pierce(&e), "tube ∪ ball: {e:?}");
    }
}

#[test]
fn the_stadiums_plane_cylinder_seam_is_contradicted_as_a_tangent() {
    let tol = Tol::witness();
    let slab: Body<f64> = brick((-1.0, 2.0), (0.0, 1.0), (-0.5, 0.5), tol);
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
    let rod = extrude(&p, Extrusion::Distance(1.0), tol).unwrap().body;
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
    for e in union_both_orders(&slab, &rod, &flats, &walls, Some(BooleanCoincidence::TANGENT)) {
        let BooleanError::ContactContradicted { margin, .. } = &e else {
            panic!("declared Tangent: contradicted: {e:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("contact_tangent_opposed"),
            "the seam's outward normals agree, which the witness lane reads as \
             containment: {e}"
        );
    }    // Declared a `Seam`, the same lane verifies it along the ruling with
    // the sense bit reversed. Undeclared, the rod's end discs are flush
    // continuations of the slab's sides; declared, the union stops at
    // the curved coplanar-lump site, whose second-order arm reads a
    // `Tangent` pair only (filed:
    // `a-declared-line-seam-stops-at-the-curved-lump-site`).
    let ends = |b: &Body<f64>| -> Vec<(FaceKey, f64)> {
        b.faces()
            .filter_map(|(k, f)| match b.get_surface(f.surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) if normal.y.abs() > 0.5 => {
                    Some((k, origin.y))
                }
                _ => None,
            })
            .collect()
    };
    for (x, y, fx, fy) in [(&slab, &rod, &flats, &walls), (&rod, &slab, &walls, &flats)] {
        let mut d = declared(fx, fy, BooleanCoincidence::Seam);
        for (ex, yx) in ends(x) {
            for (ey, yy) in ends(y) {
                if (yx - yy).abs() < 1e-12 {
                    d.coincident_faces.push(FacePairDeclaration::continuation(ex, ey));
                }
            }
        }
        assert_eq!(d.coincident_faces.len(), 6, "two flats by two walls, two flush ends");
        let r = topo::union_with(x, y, &d, tol);
        // The rod-first order escalates on the contact-vertex margin
        // the undeclared row's does, at an ε-dependent band.
        assert!(
            matches!(
                r,
                Err(BooleanError::CurvedBooleanUnsupported {
                    kind: SurfaceKind::Cylinder,
                    ..
                } | BooleanError::Escalated { .. })
            ),
            "declared Seam: verified, then the curved lump site: {r:?}"
        );
    }
}

#[test]
fn a_ball_seated_in_its_own_bore_refuses_declared_or_not() {
    let tol = Tol::witness();
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0), tol);
    let bored = match topo::subtract(&block, &rod_z(R, -3.0, 6.0), tol).unwrap() {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("the bored block is not empty"),
    };
    let ball = ball_poled_z(R, Vec3::new(0.0, 0.0, 0.0), tol);
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
    for e in union_both_orders(&bored, &ball, &bore, &sph, Some(BooleanCoincidence::TANGENT)) {
        assert!(
            matches!(
                e,
                BooleanError::UnsupportedDeclarationClass {
                    class: topo::BooleanCoincidence::TANGENT
                }
            ),
            "declared Tangent: the class refusal: {e:?}"
        );
    }    for e in union_both_orders(&bored, &ball, &bore, &sph, Some(BooleanCoincidence::Seam)) {
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
        let dome = topo::transform_rigid(&dome_on_the_cap(), &lift, tol).unwrap();
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
        let turned = topo::transform_rigid(&dome, &turn, tol).unwrap();
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
    let at0 = sweep::revolve(&pr, axis, Revolution::Full, tol).unwrap().body;
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
