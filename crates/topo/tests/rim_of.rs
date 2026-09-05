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
//! cut at `z = 1/2`, the rim circle stated ONCE and split into two
//! half-arcs, so the two arcs' stored carriers are the same value bit
//! for bit and the door's exact match has nothing to round.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::query::rim_of;
use topo::{Body, CurveKind, EdgeKey, EntityId, FaceSurface, MefSite, MevSite, RimError, query};

use crate::common;

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// A point of the rim circle at azimuth `theta`.
fn point_at(theta: f64) -> Point3<f64> {
    let (s, c) = theta.sin_cos();
    p3(rim_r() * c, rim_r() * s, RIM_Z)
}

/// The rim's latitude, and the radius that follows on the unit sphere.
const RIM_Z: f64 = 0.5;

fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn rim_plane() -> Surface<f64> {
    Surface::Plane {
        origin: p3(0.0, 0.0, RIM_Z),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn rim_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius: rim_r(),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// **The half-built cap**: the sphere face alone, with the rim's first
/// half-arc out and back inside it — so that arc has the sphere on BOTH
/// sides, which is what a chart-seam meridian looks like to the door.
fn half_built() -> (Body<f64>, EdgeKey) {
    let tol = Tol::witness();
    let r = rim_r();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p3(r, 0.0, RIM_Z)).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    let made = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p3(-r, 0.0, RIM_Z),
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
            FaceSurface::New(rim_plane()),
            tol,
        )
        .unwrap();
    (body, first, made.edge)
}

/// **Either arc names the whole rim, in one order up to rotation.**
///
/// Both directions of the claim in one row, because they are one fact:
/// the door returns every arc of the rim and only those; it starts at
/// the seed; it runs the seed carrier's positive parameter direction
/// (the second arc is the one that continues from the seed's `he_plus`
/// END, which is where that parameter increases to); and the two seeds'
/// answers are rotations of each other. Repeated calls agree (D9).
#[test]
fn either_arc_names_the_whole_rim_and_the_two_answers_are_rotations() {
    let (body, a, b) = capped();

    let from_a = rim_of(&body, a).expect("an arc of a closed rim names it");
    let from_b = rim_of(&body, b).expect("and so does the other arc");
    assert_eq!(from_a, vec![a, b], "the seed first, then what follows it");
    assert_eq!(from_b, vec![b, a], "a rotation of the same cycle");

    // The order is the seed carrier's, not the arena's: arc two starts
    // (or ends) where arc one's `he_plus` ends, and `he_plus`-forward IS
    // increasing carrier parameter.
    let ends = |k: EdgeKey| {
        let e = body.get_edge(k).unwrap();
        (
            body.get_half_edge(e.he_plus).unwrap().start,
            body.half_edge_end(e.he_plus).unwrap(),
        )
    };
    let (_, a_end) = ends(a);
    let (b_start, b_end) = ends(b);
    assert!(
        b_start == a_end || b_end == a_end,
        "the next arc continues from where the seed's parameter runs to"
    );

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

/// **A closed chain that leaves matched arcs unused is not one rim.**
///
/// The guard R2's mutant found unrowed: dropping
/// `ordered.len() == matched.len()` from the walk left the whole tree
/// green, because nothing in the corpus has two components on one
/// circle between one surface pair. This body does. Two 2-cycles —
/// `a`, `b` across `V0`/`V1` and `c`, `d` across `V2`/`V3` — sit on
/// the SAME stored circle (bit for bit: one `rim_circle()` value
/// throughout) between the SAME two surface KEYS
/// ([`FaceSurface::Shared`] is what lets a second component reuse
/// them). The walk from `a` closes after two arcs with two more still
/// matched, and the door refuses rather than handing back the half it
/// happened to walk.
#[test]
fn a_chain_that_closes_leaving_matched_arcs_unused_refuses() {
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
    let seed = body.mvfs(point_at(t2)).unwrap();
    body.set_face_surface(seed.face, FaceSurface::Shared(sphere))
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
            FaceSurface::Shared(plane),
            tol,
        )
        .unwrap()
        .edge;

    let all: BTreeSet<EdgeKey> = [a, b, c, d].into_iter().collect();
    for (seed, closes_with) in [(a, [a, b]), (c, [c, d])] {
        match rim_of(&body, seed) {
            Err(RimError::NotOneRim { arcs, gap }) => {
                assert_eq!(
                    arcs.iter().copied().collect::<BTreeSet<_>>(),
                    all,
                    "all four arcs matched: one circle, one surface pair"
                );
                let unused: Vec<EdgeKey> = arcs
                    .iter()
                    .copied()
                    .filter(|k| !closes_with.contains(k))
                    .collect();
                assert_eq!(
                    unused.len(),
                    2,
                    "the walk closed after {closes_with:?} with {unused:?} still matched"
                );
                assert!(gap.is_finite(), "the payload names a real parameter: {gap}");
            }
            other => panic!("a second component on one circle is not one rim, got {other:?}"),
        }
    }
}

/// **A straight edge is not an arc, and a dangling key is not intact.**
/// Two payload shapes in one row: the kind the seed carries, and the
/// entity that could not be read.
#[test]
fn a_line_and_a_dangling_key_refuse_typed() {
    let brick = common::prism_z::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 0.0, 1.0);
    let body = &brick.body;
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
        Err(RimError::NotIntact(EntityId::Edge(gone))),
        "a key naming nothing is an intactness fault, never a panic"
    );
}
