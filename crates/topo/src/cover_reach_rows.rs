//! **The face reaches read the face's cover**: rows a reading of the
//! boundary alone fails ([`super::face_reach_from`],
//! [`super::face_range_along`], [`super::face_axial_range`],
//! [`super::cone_face_reach`]), and a ruled face with no bounding loop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

/// The farthest the boundary alone stands from `at`: each boundary
/// edge's cover's far distance, what a boundary-only reach reads.
fn boundary_far(body: &crate::Body<f64>, face: crate::entity::FaceKey, at: Point3<f64>) -> f64 {
    let cover = super::boundary_cover(body, body.proven_face(face)).unwrap();
    cover.distance_range(&geom::cover::Site::Point(at)).1
}

/// **A sphere cap past a hemisphere reaches its rim point's antipode**
/// (GERM's worked case). The unit sphere's cap above latitude −30°,
/// read from a rim point: its rim levers `2ρ ≈ 1.73`, and the cap holds
/// the rim point's antipode, 2 away. The face's cover reads 2.
#[test]
fn a_sphere_cap_past_a_hemisphere_reaches_its_rims_antipode() {
    let z = -0.5_f64;
    let (body, face) = crate::test_support_fixtures::sphere_cap(z, Tol::witness());
    let rim = Point3::new((1.0 - z * z).sqrt(), 0.0, z);
    let far = super::face_reach_from(&body, face, rim, band())
        .unwrap()
        .length();
    let edge = boundary_far(&body, face, rim);
    assert!(
        edge < 1.8,
        "the fixture: the rim alone reaches {edge} from its point"
    );
    assert!(
        far >= 2.0 - 1e-12,
        "the cap reaches {far} from its rim point, short of the antipode's 2"
    );
}

/// **A sphere cap's axial range reaches its pole**: the cap above
/// `z = −0.5` reaches the north pole, 1.5 up the axis from its rim,
/// where its boundary reaches nothing.
#[test]
fn a_sphere_caps_axial_range_reaches_its_pole() {
    let z = -0.5_f64;
    let (body, face) = crate::test_support_fixtures::sphere_cap(z, Tol::witness());
    let rim = Point3::new((1.0 - z * z).sqrt(), 0.0, z);
    let (below, above) = super::face_axial_range(&body, face, rim, Vec3::unit_z(), band()).unwrap();
    assert!(
        above >= 1.5 - 1e-12 && below <= 1e-12,
        "the cap reaches {below} below and {above} above its rim, not its pole's 1.5"
    );
}

/// **A cone cap holding its apex reaches it along the axis**: the cap
/// of half-angle 1/2 below its rim at `z = 1` holds the apex, which no
/// boundary point reaches. Its window from the apex runs from 0 to 1,
/// and the plane×cone reach from a rim point reaches the apex below it.
#[test]
fn a_cone_cap_holding_its_apex_reaches_it_along_the_axis() {
    let (body, face) = crate::test_support_fixtures::cone_cap(0.5, 1.0, Tol::witness());
    let cone = body.proven_face(face).surface().clone();
    let (lo, hi) =
        super::face_range_along(&body, face, Point3::origin(), Vec3::unit_z(), band()).unwrap();
    assert!(
        lo <= 1e-12 && (hi - 1.0).abs() <= 1e-9,
        "the cap's window from its apex is [{lo}, {hi}], not [0, 1]"
    );
    let rim = Point3::new(0.5_f64.tan(), 0.0, 1.0);
    let geom_brep::Reach::Face { below, .. } =
        super::cone_face_reach(&body, face, rim, (&cone, None), band()).unwrap()
    else {
        panic!("a cone face's reach is a face's")
    };
    assert!(
        below >= 1.0 - 1e-9,
        "the reach from the rim runs {below} down the axis, short of the apex"
    );
}

/// **A ruled face whose outer loop is a lone vertex has no cover**: the
/// face is the whole carrier, unbounded, and its lone vertex bounds
/// nothing. A reading that hulled the vertex would answer one point.
#[test]
fn a_ruled_face_on_a_lone_vertex_has_no_cover() {
    for surface in [
        geom::Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        },
        geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        geom::Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        },
    ] {
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 0.0, 0.0), true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
                surface: surface.clone(),
                sense: true,
            },
        )
        .unwrap();
        let got = super::face_cover(&body, seed.face, band());
        assert!(
            matches!(&got, Err(why) if why
                .carrier()
                .unknowns()
                .iter()
                .all(|&(_, what)| what == geom::cover::Unknown::Unbounded)),
            "{:?}: a lone-vertex face's cover, got {got:?}",
            surface.kind()
        );
    }
}
