//! **A ball cut by one plane, through the public doors** (issue
//! 1250).
//!
//! The face a single planar cut leaves on a ball, stated as one rim
//! circle and nothing else, holds a pole in its interior. A chart
//! singularity inside a face is a vertex of it, so this statement is
//! not one the closed form reads: the rim has a pole on each side and
//! no pole-free side, and the sphere's side reader refuses the face
//! `SenseContradicted` under either traversal — the form import must
//! normalise into half-caps meeting at the pole
//! (`work/exch/import-normalizes-the-rim-only-cap.md`). These rows
//! assemble the body through the Euler doors and read that refusal
//! from outside, at tier 3 and at the mass properties.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, FaceSurface, MefSite, MevSite};

/// The unit sphere about `+Z` at the origin.
fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The cutting plane at height `z`, with its normal pointing `up`
/// or down — the disc's OUTWARD normal, which is away from the
/// material it caps.
fn cut_plane(z: f64, up: bool) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, z),
        normal: Vec3::new(0.0, 0.0, if up { 1.0 } else { -1.0 }),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The cut circle at height `z` on the unit sphere.
fn cut_circle(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: (1.0 - z * z).sqrt(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A unit ball cut by the plane `Z = z`, as two faces sharing the cut
/// circle: `seed` carries `seed_surface` and the rim traversed `+u`,
/// `made` carries `made_surface` (or the seed's own, when `None`) and
/// the same rim traversed `−u`.
///
/// The circle is stated as two half-arcs because the Euler doors mint
/// an edge between two vertices; `du_of_rims` sums their spans back to
/// the full `2π`, which is one of the things the rows below check by
/// getting the area right.
///
/// Both edges come to REST between two faces, so each is described
/// where it rests rather than left on the scaffolding door (D3's
/// transience fence): intrinsically, as the transverse intersection of
/// the two surfaces, where the pair determines the locus; and
/// conventionally, in the one chart, where the same sphere lies on
/// both sides and determines nothing.
fn cut_ball(z: f64, seed_surface: Surface<f64>, made_surface: Option<Surface<f64>>) -> Body<f64> {
    let tol = Tol::witness();
    let r = (1.0 - z * z).sqrt();
    let (a, b) = (Point3::new(r, 0.0, z), Point3::new(-r, 0.0, z));
    let pi = core::f64::consts::PI;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: seed_surface,
            sense: true,
        },
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(cut_circle(z), 0.0, pi).unwrap(),
            tol,
        )
        .unwrap();
    let transverse = made_surface.is_some();
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(cut_circle(z), pi, core::f64::consts::TAU).unwrap(),
            made_surface.map_or(FaceSurface::Inherit, |surface| FaceSurface::New {
                surface,
                sense: true,
            }),
            tol,
        )
        .unwrap();
    let s_seed = body.get_face(seed.face).unwrap().surface;
    let s_made = body.get_face(made.face).unwrap().surface;
    for (edge, witness) in [
        (e_rim.edge, Point3::new(0.0, r, z)),
        (made.edge, Point3::new(0.0, -r, z)),
    ] {
        if transverse {
            let curve = body.get_edge(edge).unwrap().curve;
            let spec = body
                .get_curve_geom(curve)
                .unwrap()
                .certified()
                .unwrap()
                .restated_spec();
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: s_seed,
                    s2: s_made,
                    witness,
                },
                ..spec
            };
            body.set_edge_curve(edge, spec, tol).unwrap();
        } else {
            body.describe_at_rest(edge, s_seed, tol).unwrap();
        }
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// Tiers 1 and 2 pass; tier 3 refuses the volume, its sphere face
/// holding a pole, beside the rim's two vertices nothing else meets
/// (the body is construction state until the join takes the half-arcs
/// back into one closed edge, check 11); and the mass properties refuse
/// the same face by the same cause.
fn refused_for_its_pole(name: &str, body: &Body<f64>) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{name}: tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{name}: tier 2");
    let is_sphere = |face: topo::FaceKey| {
        matches!(
            body.get_face(face)
                .and_then(|f| body.get_surface(f.surface)),
            Some(Surface::Sphere { .. })
        )
    };
    let errors = topo::validate_geometric(body, tol).expect_err("tier 3 refuses");
    let (volume, rest): (Vec<_>, Vec<_>) = errors.into_iter().partition(|e| {
        matches!(e, topo::ValidationError::VolumeUncomputable { .. })
    });
    assert!(
        matches!(
            volume.as_slice(),
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    face,
                    source: geom_brep::props::PropsError::SenseContradicted,
                },
                ..
            }] if is_sphere(*face)
        ),
        "{name}: tier 3's volume: {volume:?}"
    );
    assert_eq!(
        rest,
        body.vertices()
            .map(|(vertex, _)| topo::ValidationError::JoinableVertexAtRest { vertex })
            .collect::<Vec<_>>(),
        "{name}: tier 3 refuses the rim's two vertices beside the volume"
    );
    assert!(
        matches!(
            topo::mass_properties(body, tol),
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if is_sphere(face)
        ),
        "{name}: the mass properties"
    );
}

/// **The cap a plane cuts off a ball** — the solid above the cut, whose
/// spherical face is one rim circle with the north pole interior to it,
/// and whose planar face is the disc.
#[test]
fn a_ball_cut_by_one_plane_is_refused_for_its_pole() {
    for z in [0.5_f64, 0.0, -0.5, 0.9] {
        // The disc caps the solid from BELOW, so its outward normal
        // points down.
        let body = cut_ball(z, unit_sphere(), Some(cut_plane(z, false)));
        refused_for_its_pole(&format!("cap above z={z}"), &body);
    }
}

/// **The same rim traversed the other way is the rest of the ball**,
/// holding the south pole, and is refused the same way.
#[test]
fn the_other_traversal_of_the_same_rim_is_refused_for_its_pole() {
    for z in [0.5_f64, 0.0, -0.5, 0.9] {
        // Now the DISC is the seed face (rim `+u`, outward normal up)
        // and the sphere is the `−u` face.
        let body = cut_ball(z, cut_plane(z, true), Some(unit_sphere()));
        refused_for_its_pole(&format!("ball below z={z}"), &body);
    }
}

/// **A closed sphere split by ONE rim circle into two rim-only caps**:
/// each cap holds a pole, and the volume is refused at the first.
#[test]
fn a_sphere_split_into_two_rim_only_caps_is_refused_for_its_poles() {
    for z in [0.5_f64, 0.0, -0.75] {
        let body = cut_ball(z, unit_sphere(), None);
        refused_for_its_pole(&format!("two caps at z={z}"), &body);
    }
}
