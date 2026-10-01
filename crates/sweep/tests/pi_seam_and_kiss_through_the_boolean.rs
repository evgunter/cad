//! **What a π seam between distinct carriers, and a coaxial kiss, do
//! through the boolean.** Every row is a refusal, pinned by kind.
//!
//! - **The sphere-capped tube** (cylinder `z ∈ [0, h]` ∪ hemisphere of
//!   the same radius on its top cap): the two walls meet G1 along the
//!   rim circle, wedge π. Undeclared it stops at the crossing layer;
//!   declared `Tangent` across the rim the routing classifies the seam
//!   and refuses `RimSeamNotDeclarable`; declared `Rest` it is
//!   contradicted on carrier kind.
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

use geom_brep::SurfaceKind;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
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
        .filter(|(_, f)| b.get_surface(f.surface).map(SurfaceKind::of) == Some(kind))
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

/// A solid hemisphere of radius [`R`] standing on `z = H`: the quarter
/// profile revolved, turned onto `+z`, lifted, and its base disc's two
/// revolve halves merged (the boolean refuses a non-maximal operand).
fn hemisphere_on_the_cap() -> Body<f64> {
    let tol = Tol::witness();
    let bulge = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    let at0 = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(R, 0.0), bulge),
            (Point2::new(0.0, R), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI / 2.0);
    let turned = topo::transform_rigid(&at0, &turn, tol).unwrap();
    let mut hemi =
        topo::transform_rigid(&turned, &Affine3::translation(Vec3::new(0.0, 0.0, H)), tol).unwrap();
    hemi.merge_coplanar_faces(tol).unwrap();
    hemi
}

fn is_pierce(e: &BooleanError) -> bool {
    matches!(e, BooleanError::CurvedPierceUnsupported { .. })
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
        assert!(is_pierce(&e), "undeclared: {e:?}");
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
