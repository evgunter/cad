//! **The rim door's own contracts** ([`topo::query::rim_of`]), on
//! bodies this crate can build: what the door returns for a rim a
//! chart seam split, in what order, and what it refuses on.
//!
//! The corpus rows — a revolve's seam-split rims, the repaired pole
//! body, a partial revolve's open rim, the end-to-end carve — live in
//! `sweep/tests/rim_of_rows.rs`, because their producers are `sweep`'s
//! and this crate is below them. What is HERE is what a body assembled
//! through the Euler doors can state exactly: one circle, two arcs, two
//! surfaces, and the same circle read with one surface on both sides.
//!
//! The fixture is a spherical cap closed by its own disc: a unit sphere
//! cut at `z = 1/2`, the rim circle split into two half-arcs between
//! the sphere and the disc.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::query::rim_of;
use topo::{
    Body, CurveKind, DanglingRef, EdgeKey, EntityId, FaceSurface, MefSite, MevSite, RimBreak,
    RimError, query,
};

use crate::common;

/// A point of the rim circle at azimuth `theta`.
fn point_at(theta: f64) -> Point3<f64> {
    let (s, c) = theta.sin_cos();
    Point3::new(rim_r() * c, rim_r() * s, RIM_Z)
}

/// The rim's latitude, and the radius that follows on the unit sphere.
const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn rim_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, RIM_Z),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn rim_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// [`rim_circle`] wound the other way: parameter `t` is at azimuth `-t`.
fn rim_circle_reversed() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, RIM_Z),
        axis: Vec3::new(0.0, 0.0, -1.0),
        radius: rim_r(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **The half-built cap**: the sphere face alone, with the rim's first
/// half-arc out and back inside it — so that arc has the sphere on BOTH
/// sides, which is what a chart-seam meridian looks like to the door.
fn half_built() -> (Body<f64>, EdgeKey) {
    let tol = Tol::witness();
    let r = rim_r();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(r, 0.0, RIM_Z), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense: true,
        },
    )
    .unwrap();
    let made = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(-r, 0.0, RIM_Z),
            EdgeCurveSpec::arc_of_circle(rim_circle(), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    (body, made.edge)
}

/// **The closed cap**: [`half_built`] with the rim's second half-arc
/// added, splitting the loop and minting the disc that closes it. The
/// rim is now two arcs of ONE circle between TWO surfaces — the
/// seam-split shape, stated exactly.
fn capped() -> (Body<f64>, EdgeKey, EdgeKey) {
    let tol = Tol::witness();
    let (mut body, first) = half_built();
    let e = body.get_edge(first).unwrap();
    let (he1, he2) = (e.he_minus, e.he_plus);
    let made = body
        .mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::arc_of_circle(
                rim_circle(),
                core::f64::consts::PI,
                core::f64::consts::TAU,
            )
            .unwrap(),
            FaceSurface::New {
                surface: rim_plane(),
                sense: true,
            },
            tol,
        )
        .unwrap();
    (body, first, made.edge)
}

/// **Either arc names the whole rim, in one order up to rotation.**
///
/// Both directions of the claim in one row, because they are one fact:
/// the door returns every arc of the rim and only those; it starts at
/// the seed; and the two seeds' answers are rotations of each other.
/// Repeated calls agree (D9).
#[test]
fn either_arc_names_the_whole_rim_and_the_two_answers_are_rotations() {
    let (body, a, b) = capped();

    let from_a = rim_of(&body, a).expect("an arc of a closed rim names it");
    let from_b = rim_of(&body, b).expect("and so does the other arc");
    assert_eq!(from_a, vec![a, b], "the seed first, then what follows it");
    assert_eq!(from_b, vec![b, a], "a rotation of the same cycle");

    assert_eq!(rim_of(&body, a).unwrap(), from_a, "same body, same answer");
    assert_eq!(from_a.len(), 2, "not vacuous: the rim really is split");
}

/// **A seam meridian refuses `CoSurface`.** The half-built cap's arc is
/// the same circle, on the same body, with the same certified carrier —
/// and one surface on both sides, which is the whole of what separates
/// a chart seam from a rim. The payload names that surface.
#[test]
fn one_surface_on_both_sides_refuses_co_surface() {
    let (body, only) = half_built();
    let face = query::all_faces(&body)[0];
    let surface = body.get_face(face).unwrap().surface;
    assert_eq!(
        rim_of(&body, only),
        Err(RimError::CoSurface {
            edge: only,
            surface
        }),
        "a co-surface arc is refused, and the refusal names the surface"
    );
}

/// **Two rims on one surface pair: each seed answers its own.**
///
/// Two 2-cycles — `a`, `b` across `V0`/`V1` and `c`, `d` across
/// `V2`/`V3` — lie between the SAME two surface KEYS
/// ([`FaceSurface::Shared`] is what lets a second component reuse
/// them). Membership is the chain through the seed, so the edges of the
/// other component are another rim and not a refusal: a plane through
/// a torus, or a cylinder through a sphere, has two.
#[test]
fn two_chains_on_one_surface_pair_are_two_rims() {
    let tol = Tol::witness();
    let (mut body, a, b) = capped();
    let sphere = body.get_face(query::all_faces(&body)[0]).unwrap().surface;
    let plane = body.get_face(query::all_faces(&body)[1]).unwrap().surface;
    assert_ne!(sphere, plane, "the cap's two faces are two surfaces");

    // A second component on the same circle, at two fresh stations, on
    // the same two surface keys.
    let (t2, t3) = (
        core::f64::consts::FRAC_PI_4,
        5.0 * core::f64::consts::FRAC_PI_4,
    );
    let seed = body.mvfs(point_at(t2), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::Shared {
            key: sphere,
            sense: true,
        },
    )
    .unwrap();
    let c = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            point_at(t3),
            EdgeCurveSpec::arc_of_circle(rim_circle(), t2, t3).unwrap(),
            tol,
        )
        .unwrap()
        .edge;
    let ce = body.get_edge(c).unwrap();
    let (he1, he2) = (ce.he_minus, ce.he_plus);
    let d = body
        .mef(
            MefSite::Chords { he1, he2 },
            EdgeCurveSpec::arc_of_circle(rim_circle(), t3, t2 + core::f64::consts::TAU).unwrap(),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap()
        .edge;

    for (seed, rim) in [(a, [a, b]), (b, [b, a]), (c, [c, d]), (d, [d, c])] {
        assert_eq!(
            rim_of(&body, seed),
            Ok(rim.to_vec()),
            "the seed answers its own chain on the shared pair"
        );
    }
}

/// **Four edges of the pair at one vertex refuse `NotOneRim`, naming
/// it as a branch.** A second 2-cycle `c`, `d` is grown from the cap's
/// `V0` into the sphere face and closed on a fresh face sharing the
/// disc's surface key, so every one of `a`, `b`, `c`, `d` lies between
/// the sphere and the disc and all four meet at `V0`. No seed can pick
/// one chain there, and each refuses naming `V0`.
#[test]
fn four_edges_of_the_pair_at_one_vertex_refuse_as_a_branch() {
    let tol = Tol::witness();
    let (mut body, a, b) = capped();
    let faces = query::all_faces(&body);
    let (sphere_face, plane_face) = (faces[0], faces[1]);
    let plane = body.get_face(plane_face).unwrap().surface;
    let v0 = body
        .get_half_edge(body.get_edge(a).unwrap().he_plus)
        .unwrap()
        .start;
    let face_of = |body: &Body<f64>, he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let at_v0_on_sphere = [a, b]
        .iter()
        .flat_map(|&k| {
            let e = body.get_edge(k).unwrap();
            [e.he_plus, e.he_minus]
        })
        .find(|&he| {
            body.get_half_edge(he).unwrap().start == v0 && face_of(&body, he) == sphere_face
        })
        .expect("a sphere-side half-edge leaves V0");
    let quarter = core::f64::consts::FRAC_PI_2;
    let c = body
        .mev(
            MevSite::Fan {
                he1: at_v0_on_sphere,
                he2: at_v0_on_sphere,
            },
            point_at(quarter),
            EdgeCurveSpec::arc_of_circle(rim_circle(), 0.0, quarter).unwrap(),
            tol,
        )
        .unwrap()
        .edge;
    let ce = body.get_edge(c).unwrap();
    let (c_back, c_fwd) = (ce.he_minus, ce.he_plus);
    let d = body
        .mef(
            MefSite::Chords {
                he1: c_fwd,
                he2: c_back,
            },
            // `V0 → V2` the long way round: the opposite winding.
            EdgeCurveSpec::arc_of_circle(rim_circle_reversed(), 0.0, 3.0 * quarter).unwrap(),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap()
        .edge;

    for seed in [a, b, c, d] {
        match rim_of(&body, seed) {
            Err(RimError::NotOneRim { walked, at, how }) => {
                assert_eq!(at, v0, "from {seed:?}: the walk stops at V0");
                assert_eq!(
                    how,
                    RimBreak::Branches,
                    "from {seed:?}: four ends meet there"
                );
                assert_eq!(walked[0], seed, "the walk starts at the seed");
            }
            other => panic!("from {seed:?}: a branch is not one rim, got {other:?}"),
        }
    }
}

/// **A straight edge is not an arc, and a dangling key is not intact.**
/// Two payload shapes in one row: the kind the seed carries, and the
/// entity that could not be read.
#[test]
fn a_line_and_a_dangling_key_refuse_typed() {
    let brick = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let body = &brick;
    let edge = query::all_edges(body)[0];
    assert_eq!(
        rim_of(body, edge),
        Err(RimError::NotAnArc {
            edge,
            kind: Some(CurveKind::Line)
        }),
        "a prism's edges are chords, and the refusal says which kind"
    );

    let gone = EdgeKey::default();
    assert_eq!(
        rim_of(body, gone),
        Err(RimError::NotIntact(DanglingRef::Entity(EntityId::Edge(
            gone
        )))),
        "a key naming nothing is an intactness fault, never a panic"
    );
}
