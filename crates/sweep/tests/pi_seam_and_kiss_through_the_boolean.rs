//! **What a π seam between distinct carriers, a cap abutting a tube on
//! its rim, and a coaxial kiss, do through the boolean.** One row
//! builds — the transverse rim — and every other is a refusal, pinned by
//! kind.
//!
//! - **The sphere-capped tube** (cylinder `z ∈ [0, h]` ∪ hemisphere of
//!   the same radius on its top cap): the two walls meet G1 along the
//!   rim circle, wedge π. Undeclared it stops at the crossing layer;
//!   declared `Tangent` across the rim the routing classifies the seam
//!   and refuses `RimSeamNotDeclarable`; declared `Rest` it is
//!   contradicted on carrier kind.
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
//!   stops in the join. The G1
//!   hemisphere stops at the crossing layer on an edge leaving the rim,
//!   which grazes the partner's wall; a same-radius stacked cylinder
//!   stops there on its own rim, whose parent shares the partner's
//!   carrier. A cone frustum is refused earlier, at the operand gate,
//!   on its kind.
//! - **Tube ∪ ball** (overlapping, ball centred on the top cap) stops at
//!   the crossing layer.
//! - **The stadium** (slab ∪ cylinder whose wall the slab's top and
//!   bottom are tangent to along a ruling): a plane×cylinder π seam.
//!   Undeclared it stops at the crossing layer; declared `Tangent` the
//!   witness lane contradicts it, because its outward normals AGREE.
//! - **The kiss** (a ball seated in a bore of its own radius): the
//!   equator touches the bore wall, wedge 2π. Undeclared it stops at
//!   the crossing layer; declared `Tangent` the class is refused — the
//!   two faces share no boundary circle, so the rim routing never runs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::SurfaceKind;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick, revolved_about_y};
use sweep::{Extrusion, Revolution, extrude};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanResult, ContactClass, FaceKey,
    FacePairDeclaration,
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

fn declared(a: &[FaceKey], b: &[FaceKey], class: ContactClass) -> BooleanDeclarations {
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
    class: Option<ContactClass>,
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

#[test]
fn the_sphere_capped_tube_refuses_at_every_door() {
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
            is_pierce(&e),
            "undeclared: the crossing layer's refusal: {e:?}"
        );
    }
    for e in union_both_orders(&tube, &hemi, &cap_t, &cap_h, Some(ContactClass::Rest)) {
        assert!(
            is_pierce(&e),
            "the cap discs declared Rest: still the crossing layer: {e:?}"
        );
    }
    for e in union_both_orders(&tube, &hemi, &cyl, &sph, Some(ContactClass::Tangent)) {
        assert!(
            matches!(e, BooleanError::RimSeamNotDeclarable { .. }),
            "the walls declared Tangent: the rim routes to the seam: {e:?}"
        );
    }
    for e in union_both_orders(&tube, &hemi, &cyl, &sph, Some(ContactClass::Rest)) {
        let BooleanError::ContactContradicted { margin, .. } = &e else {
            panic!("the walls declared Rest: contradicted: {e:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("carrier_kind"),
            "on carrier kind: {e}"
        );
    }
    for e in union_both_orders(&tube, &hemi, &cap_t, &cap_h, Some(ContactClass::Tangent)) {
        assert!(
            matches!(e, BooleanError::ContactContradicted { .. }),
            "the cap discs declared Tangent: one plane, contradicted: {e:?}"
        );
    }
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
    for (label, tube, cap) in [
        ("dome", &tube, &dome),
        ("dome turned", &tube, &turned),
        ("revolved tube", &revolved, &dome),
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
fn a_cap_abutting_on_the_rim_refuses_at_a_graze_or_on_its_own_carrier() {
    let tube = rod_z(R, 0.0, H);
    let cap_t = planes_at_z(&tube, H);
    let hemi = hemisphere_on_the_cap();
    let cap_h = planes_at_z(&hemi, H);
    // The hemisphere is G1 at the rim (wedge π): the edges LEAVING the
    // rim — the tube's rulings, the hemisphere's meridians — graze the
    // other operand's wall there, and a graze needs a declaration or
    // structure. The refused edge is never the rim circle itself.
    for class in [None, Some(ContactClass::Rest)] {
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
    let stacked = rod_z(R, H, 1.0);
    let cap_s = planes_at_z(&stacked, H);
    for class in [None, Some(ContactClass::Rest)] {
        let [ab, ba] = union_both_orders(&tube, &stacked, &cap_t, &cap_s, class);
        assert!(
            is_pierce(&ab),
            "stacked, discs {class:?}, order 0: the crossing layer: {ab:?}"
        );
        let BooleanError::CurvedPierceUnsupported { edge, face, .. } = ba else {
            panic!("stacked, discs {class:?}, order 1: the crossing layer: {ba:?}");
        };
        assert!(
            matches!(
                carrier_of(&stacked, edge),
                geom::Curve3::Circle { center, .. } if (center.z - H).abs() < 1e-12
            ),
            "stacked, discs {class:?}, order 1: the cap's own rim keeps the door: {:?}",
            carrier_of(&stacked, edge)
        );
        assert!(
            faces_of(&tube, SurfaceKind::Cylinder).contains(&face),
            "stacked, discs {class:?}, order 1: against the tube's wall"
        );
    }
    // With its walls declared `Rest` too, the stacked pair is
    // contradicted at the declaration door: one cylinder with aligned
    // senses is a continuation, and aligned senses contradict `Rest` at
    // every door. It builds once the `Continuation` seat does
    // (`work/tang/pi-seam-between-two-operands-has-no-declaration.md`;
    // `work/reach/cosurface-disjoint-curved-walls-refuse.md`).
    let (wt, ws) = (
        faces_of(&tube, SurfaceKind::Cylinder),
        faces_of(&stacked, SurfaceKind::Cylinder),
    );
    for (order, x, y, dx, dy, fx, fy) in [
        (0, &tube, &stacked, &cap_t, &cap_s, &wt, &ws),
        (1, &stacked, &tube, &cap_s, &cap_t, &ws, &wt),
    ] {
        let mut d = declared(dx, dy, ContactClass::Rest);
        d.coincident_faces
            .extend(declared(fx, fy, ContactClass::Rest).coincident_faces);
        let err = topo::union_with(x, y, &d, Tol::witness())
            .expect_err("aligned walls declared Rest are a false claim");
        let BooleanError::ContactContradicted { margin, .. } = &err else {
            panic!("stacked, walls declared Rest, order {order}: contradicted: {err:?}");
        };
        assert_eq!(
            margin.predicate,
            Some("contact_rest_senses_opposed"),
            "stacked, order {order}: on the sense bit"
        );
    }
    let cone = frustum_on_the_cap();
    let cap_c = planes_at_z(&cone, H);
    for class in [None, Some(ContactClass::Rest)] {
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
    for e in union_both_orders(&slab, &rod, &flats, &walls, Some(ContactClass::Tangent)) {
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
    for e in union_both_orders(&bored, &ball, &bore, &sph, Some(ContactClass::Tangent)) {
        assert!(
            matches!(
                e,
                BooleanError::UnsupportedDeclarationClass {
                    class: ContactClass::Tangent
                }
            ),
            "declared Tangent: the class refusal: {e:?}"
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

/// The body of a boolean that builds, at tier 3, with its volume and
/// its census.
fn built(
    label: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
) -> (f64, (usize, usize, usize, usize)) {
    let tol = Tol::witness();
    let b = match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{label}: builds: {other:?}"),
    };
    topo::validate_geometric(&b, tol).unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
    (topo::mass_properties(&b, tol).unwrap().volume, census(&b))
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
            let (v, c) = built(&label, r);
            assert!(
                (v - want).abs() <= 1e-12 * want,
                "{label}: the tube and the cap above it: {v} vs {want}"
            );
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
    for (label, r, want, census) in [
        (
            "t ∪ d",
            topo::union_with(&tall, &dome, &none, tol),
            tube + above,
            (6, 12, 9, 1),
        ),
        (
            "d ∪ t",
            topo::union_with(&dome, &tall, &none, tol),
            tube + above,
            (6, 12, 9, 1),
        ),
        (
            "t ∖ d",
            topo::subtract_with(&tall, &dome, &none, tol),
            tube - inside,
            (7, 14, 10, 1),
        ),
        (
            "d ∖ t",
            topo::subtract_with(&dome, &tall, &none, tol),
            above,
            (3, 4, 3, 1),
        ),
        (
            "t ∩ d",
            topo::intersect_with(&tall, &dome, &none, tol),
            inside,
            (4, 6, 4, 1),
        ),
        (
            "d ∩ t",
            topo::intersect_with(&dome, &tall, &none, tol),
            inside,
            (4, 6, 4, 1),
        ),
    ] {
        let (v, c) = built(label, r);
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{label}: the closed form: {v} vs {want}"
        );
        assert_eq!(c, census, "{label}: F, E, V, shells");
    }
}
