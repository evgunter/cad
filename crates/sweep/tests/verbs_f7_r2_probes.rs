//! R2 review probes, ADOPTED (PR #1131), sweep side. Written against a
//! GATE EXEMPTION that was withdrawn on their evidence; what shipped
//! was the dangling-seam pruning in `merge_coplanar_faces`. The full
//! revolve now builds the pole-touching cap whole, by the route probe 2
//! found, so the rows here pin that it does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{
    Body, BooleanOp, FaceKey, HalfEdgeKey, boolean_reduce, validate_closed, validate_geometric,
};

fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

fn cone() -> Body<f64> {
    let vp = validated(vec![triangle()]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn face_of(b: &Body<f64>, he: HalfEdgeKey) -> FaceKey {
    let l = b.get_half_edge(he).unwrap().parent_loop;
    b.get_loop(l).unwrap().face
}

fn is_plane(b: &Body<f64>, f: FaceKey) -> bool {
    matches!(
        b.get_surface(b.get_face(f).unwrap().surface),
        Some(geom::Surface::Plane { .. })
    )
}

/// Dumps every valence-2 vertex whose two edges separate the same face
/// pair — i.e. every site the withdrawn `reduce::pole_split_cap` fired
/// at, which is also where the shipped repair looks — and says
/// whether the pair is planar.
fn dump_poles(b: &Body<f64>, label: &str) {
    for (vk, v) in b.vertices() {
        let Some(em) = v.emanating else { continue };
        let Some(orbit) = b.vertex_orbit(em) else {
            continue;
        };
        if orbit.len() != 2 {
            continue;
        }
        let pairs: Vec<(FaceKey, FaceKey)> = orbit
            .iter()
            .map(|h| {
                let e = b.get_edge(b.get_half_edge(*h).unwrap().edge).unwrap();
                let (x, y) = (face_of(b, e.he_plus), face_of(b, e.he_minus));
                if x <= y { (x, y) } else { (y, x) }
            })
            .collect();
        if pairs[0] == pairs[1] {
            println!(
                "{label}: vertex {vk:?} valence 2, both edges separate {:?} (planar pair = {})",
                pairs[0],
                is_plane(b, pairs[0].0) && is_plane(b, pairs[0].1)
            );
        }
    }
}

/// A brick that straddles the cone, so the boolean is a real cut.
fn brick_operand() -> Body<f64> {
    sweep::test_support::brick((-0.5, 0.5), (-0.5, 0.5), (0.0, 0.4), Tol::witness())
}

/// The edges whose two sides are distinct PLANAR faces on one surface
/// key: the pole-split cap's two diameter halves, where it exists.
fn planar_same_key(b: &Body<f64>) -> usize {
    b.edges()
        .filter(|(_, e)| {
            let (fp, fm) = (face_of(b, e.he_plus), face_of(b, e.he_minus));
            fp != fm
                && is_plane(b, fp)
                && b.get_face(fp).map(|f| f.surface) == b.get_face(fm).map(|f| f.surface)
        })
        .count()
}

/// PROBE 1 — the plain analytic CONE from `revolve` carries its base
/// disc WHOLE: the full revolve builds a plane wall as one face, with
/// no pole at its centre and no meridian across it, by the route probe
/// 2 found (`kef` one meridian, `kev` the other with the pole). So no
/// planar same-key pair exists for the F7 door to refuse, and the union
/// with a straddling brick runs.
#[test]
fn r2_cone_carries_its_cap_whole() {
    let c = cone();
    assert_eq!(counts(&c), (3, 4, 3, 0));
    assert_eq!(planar_same_key(&c), 0, "no pole-split cap");
    dump_poles(&c, "cone");
    let res = boolean_reduce(BooleanOp::Union, &c, &brick_operand(), Tol::witness());
    assert!(
        !matches!(res, Err(topo::BooleanError::NonMaximalFaces { .. })),
        "the F7 door is open on the cone as built: {:?}",
        res.map(|_| "Ok")
    );
}

// ---------------------------------------------------------------
// ACCEPTANCE (VERBS/F7, added to R2's fixtures): the pole positive.
// ---------------------------------------------------------------

/// **The unit's headline acceptance, now at the constructor.** A full
/// revolve's axis-touching planar cap was two half-discs sharing the
/// halves of the disc's DIAMETER, meeting at the pole, and
/// `merge_coplanar_faces` repaired it to ONE face bounded by its rim
/// alone. The revolve builds that face directly, so the repair finds
/// nothing to do and the body is valid as built.
#[test]
fn f7_the_cone_cap_is_one_face_as_built() {
    let tol = Tol::witness();
    let mut c = cone();
    let (f0, v0) = (c.faces().count(), c.vertices().count());
    let outcome = c.merge_coplanar_faces(tol).expect("the merge runs");
    assert!(outcome.groups.is_empty(), "no cap group to merge");
    assert_eq!((c.faces().count(), c.vertices().count()), (f0, v0));
    assert_eq!(planar_same_key(&c), 0, "the cap is maximal as built");
    assert_eq!(validate_closed(&c), Ok(()), "tier 2");
    assert_eq!(validate_geometric(&c, tol), Ok(()), "tier 3");
}
