//! **A ball cut by one plane, through the public doors** (issue
//! 1250).
//!
//! The face a single planar cut leaves on a ball is bounded by one rim
//! circle and nothing else, with the pole interior to it. A chart
//! singularity in a face's closure is a vertex of it, so tier 3 refuses
//! the statement (`ValidationError::ChartSingularityNotAVertex`) and
//! import normalises it; the Euler doors still state it, and the closed
//! form reads it by Gauss–Bonnet: the rim has a pole on each side, so
//! its loop encodes no side, and the face measures under its bit. These
//! rows read the result from outside: assemble the body through the
//! Euler doors, hold it to all three tiers, and weigh it against the
//! spherical-cap closed form `πh²(3R − h)/3`.
//!
//! Every volume here is a CLOSED FORM end to end — `volume_pad` is
//! exactly zero, so no row is served by a certified enclosure whose
//! midpoint happens to land.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::chart_singularity::Singularity;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::Body;
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MevSite};

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
/// an edge between two vertices; Gauss–Bonnet integrates both spans,
/// which is one of the things the rows below check by getting the area
/// right.
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
                    pair: SurfacePair::new(s_seed, s_made),
                    selector: geom_brep::CurveSelector::Witness(witness),
                },
                ..spec
            };
            body.set_edge_curve(edge, spec, tol).unwrap();
        } else {
            body.describe_at_rest(edge, tol).unwrap();
        }
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// Validate at all three tiers and return the closed-form volume,
/// asserting that no part of it came from an enclosure (its pad is zero:
/// every face took a closed form, rounding and all).
///
/// Tier 3 refuses each sphere face for the pole inside it, `poles[i]`
/// for the `i`-th sphere face in arena order. The rim's two half-arcs
/// meet at two vertices nothing else does, so the body is construction
/// state until the join takes them back into one closed edge: tier 3
/// refuses those two vertices too (check 11), and nothing else. The
/// half-arcs stay, because their spans summing back to `2π` is part of
/// what these rows check.
fn tier_checked_volume(name: &str, body: &Body<f64>, poles: &[Singularity]) -> f64 {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{name}: tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{name}: tier 2");
    let spheres = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(face, _)| face);
    let inside = spheres.zip(poles).map(|(face, &point)| {
        topo::ValidationError::ChartSingularityNotAVertex {
            face,
            point,
            lies: topo::SingularityLies::Interior,
        }
    });
    assert_eq!(
        topo::validate_geometric(body, tol),
        Err(inside
            .chain(
                body.vertices()
                    .map(|(vertex, _)| topo::ValidationError::JoinableVertexAtRest { vertex })
            )
            .collect()),
        "{name}: tier 3 refuses each sphere face's pole and the rim's two vertices, and \
         nothing else"
    );
    // Raw door: the measured body is construction state whose joinable vertices tier 3 refuses (JoinableVertexAtRest).
    let props = topo::test_support::measure_unvalidated(
        body,
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap_or_else(|e| panic!("{name}: {e:?}"));
    assert_eq!(
        props.enclosure.volume_pad, 0.0,
        "{name}: a closed form needs no pad"
    );
    assert_eq!(props.enclosure.area_pad, 0.0, "{name}: nor does its area");
    props.enclosure.volume
}

/// The spherical cap of height `h` off a unit ball.
fn cap_volume(h: f64) -> f64 {
    core::f64::consts::PI * h * h * (3.0 - h) / 3.0
}

/// **The cap a plane cuts off a ball weighs `πh²(3R − h)/3`** — the
/// solid above the cut, whose spherical face is one rim circle with
/// the north pole interior to it, and whose planar face is the disc.
#[test]
fn a_ball_cut_by_one_plane_weighs_the_spherical_cap() {
    for z in [0.5_f64, 0.0, -0.5, 0.9] {
        // The disc caps the solid from BELOW, so its outward normal
        // points down.
        let body = cut_ball(z, unit_sphere(), Some(cut_plane(z, false)));
        let got = tier_checked_volume(&format!("cap above z={z}"), &body, &[Singularity::North]);
        let want = cap_volume(1.0 - z);
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "cap above z={z}: volume {got} vs closed form {want}"
        );
    }
}

/// **The same rim traversed the other way is the rest of the ball.**
/// Nothing about the two solids differs except which way their
/// spherical face walks the cut circle: the cap's rim runs `+u` and
/// its interior is above, this one's runs `−u` and its interior is
/// below, down to the south pole.
#[test]
fn the_other_traversal_of_the_same_rim_weighs_the_rest_of_the_ball() {
    let ball = 4.0 * core::f64::consts::PI / 3.0;
    for z in [0.5_f64, 0.0, -0.5, 0.9] {
        // Now the DISC is the seed face (rim `+u`, outward normal up)
        // and the sphere is the `−u` face.
        let body = cut_ball(z, cut_plane(z, true), Some(unit_sphere()));
        let got = tier_checked_volume(&format!("ball below z={z}"), &body, &[Singularity::South]);
        let want = ball - cap_volume(1.0 - z);
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "ball below z={z}: volume {got} vs closed form {want}"
        );
    }
}

/// **A closed sphere split by ONE rim circle into two rim-only caps
/// weighs `4πR³/3`.** Each cap measures under its own bit, and the two
/// closed forms sum to the ball.
#[test]
fn a_sphere_split_into_two_rim_only_caps_weighs_the_ball() {
    for z in [0.5_f64, 0.0, -0.75] {
        let body = cut_ball(z, unit_sphere(), None);
        let got = tier_checked_volume(
            &format!("two caps at z={z}"),
            &body,
            &[Singularity::North, Singularity::South],
        );
        let want = 4.0 * core::f64::consts::PI / 3.0;
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "two caps at z={z}: volume {got} vs 4π/3 {want}"
        );
    }
}

/// **A pole whose distance to a face's boundary is in band is
/// undecided, at rest as at import**: a unit ball split by a great
/// circle that passes `3ε` from both poles, inside the band `(ε, Kε)`.
/// Each face holds one pole off its boundary and has the other just
/// outside, and neither reading is a vertex, an edge or a side: tier 3
/// refuses all four as undecided distances, carrying the sized ending
/// ([`geom_brep::chart_singularity::ON_BOUNDARY`]), among whatever else
/// it reports of a cut left on its scaffolding (no chart lane describes
/// an arc that close to the pole at rest). Mapping an undecided reading to no finding
/// passes a pole a finer tolerance would place inside a face.
#[test]
fn a_pole_in_band_of_a_great_circle_cut_is_undecided_on_both_faces() {
    use geom_brep::chart_singularity::Undecided;
    let tol = Tol::witness();
    let delta = 3.0 * tol.eps();
    // The great circle whose normal makes `n·z = sin δ`: it passes `δ`
    // from each pole.
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(delta.cos(), 0.0, delta.sin()),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let (a, b) = (Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, -1.0, 0.0));
    let pi = core::f64::consts::PI;
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense: true,
        },
    )
    .unwrap();
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim.clone(), 0.0, pi).unwrap(),
            tol,
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Chords {
                he1: first.he_minus,
                he2: first.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(rim, pi, core::f64::consts::TAU).unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    let _ = made;
    let errors = topo::validate_geometric(&body, tol).unwrap_err();
    let mut undecided: Vec<(topo::FaceKey, Singularity)> = Vec::new();
    for e in &errors {
        match e {
            topo::ValidationError::ChartSingularityUndecided {
                face,
                point,
                reading: Undecided::Boundary(_),
            } => {
                assert!(
                    e.to_string().contains("tighten the tolerance below"),
                    "the sized ending: {e}"
                );
                undecided.push((*face, *point));
            }
            topo::ValidationError::ChartSingularityNotAVertex { .. }
            | topo::ValidationError::ChartSingularityUndecided { .. } => {
                panic!("only undecided distances: {e:?}")
            }
            _ => {}
        }
    }
    let faces: Vec<topo::FaceKey> = body.faces().map(|(k, _)| k).collect();
    let mut want: Vec<(topo::FaceKey, Singularity)> = faces
        .iter()
        .flat_map(|&f| [(f, Singularity::North), (f, Singularity::South)])
        .collect();
    let key = |p: &(topo::FaceKey, Singularity)| {
        (
            faces.iter().position(|f| *f == p.0),
            p.1 == Singularity::South,
        )
    };
    undecided.sort_by_key(key);
    want.sort_by_key(key);
    assert_eq!(undecided, want, "{errors:?}");
}
