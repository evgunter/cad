//! R2 review probes, ADOPTED (PR #1131), sweep side. Written against a
//! GATE EXEMPTION that was withdrawn on their evidence; what ships is
//! the collinear-seam repair in `merge_coplanar_faces`. The pole
//! acceptance row at the foot of this file is the positive pole of
//! that differential.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{
    Body, BooleanOp, FaceKey, HalfEdgeKey, boolean_reduce, validate, validate_closed,
    validate_geometric,
};

fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
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
    use profile::{Profile, SketchPlane};
    use sweep::{Extrusion, extrude};
    let loop_ = ProfileLoop::polygon([p2(-0.5, -0.5), p2(0.5, -0.5), p2(0.5, 0.5), p2(-0.5, 0.5)]);
    let vp = Profile::new(SketchPlane::xy(), vec![loop_])
        .validate(Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(0.4), Tol::witness())
        .unwrap()
        .body
}

/// PROBE 1 — the plain analytic CONE from `revolve` carries the
/// pole-split cap (its base disc). Pre-PR this made every such body an
/// illegal boolean operand; post-PR the F7 door should be open.
#[test]
fn r2_cone_carries_the_pole_split_cap() {
    let c = cone();
    assert_eq!(counts(&c), (4, 6, 4, 0));
    dump_poles(&c, "cone");
    let b = brick_operand();
    let res = boolean_reduce(BooleanOp::Union, &c, &b, Tol::witness());
    match &res {
        Ok(_) => println!("R2-P1 union(cone, brick) => Ok — F7 door open"),
        Err(e) => println!("R2-P1 union(cone, brick) => {e:?}"),
    }
}

/// PROBE 2 — **the deviation's load-bearing premise, attacked.** The
/// PR argues there is "no maximal one-face form for this pair to be
/// measured against" and that demanding one "refuses a body for not
/// having a shape nothing can build", enumerating only two single-face
/// routes (merge one meridian ⇒ valence-1, tier-2 banned; merge both
/// ⇒ isolated vertex, `MergedFaceRoleAmbiguous`).
///
/// A third route exists and uses only public Euler ops: `kef` one
/// meridian (valence 1 is a TRANSIENT, and tier 2 is an at-rest rule),
/// then `kev` the resulting strut, which takes the pole with it. The
/// cap is then ONE face bounded by the two rim arcs. If this validates
/// at tier 2, "nothing can build it" is false.
#[test]
fn r2_one_face_cap_via_kef_then_kev() {
    let mut c = cone();
    // Locate the planar pole: a valence-2 vertex whose two edges
    // separate the same PLANAR pair.
    let mut found = None;
    for (vk, v) in c.vertices() {
        let Some(em) = v.emanating else { continue };
        let Some(orbit) = c.vertex_orbit(em) else {
            continue;
        };
        if orbit.len() != 2 {
            continue;
        }
        let e0 = c.get_edge(c.get_half_edge(orbit[0]).unwrap().edge).unwrap();
        let (f1, f2) = (face_of(&c, e0.he_plus), face_of(&c, e0.he_minus));
        if f1 != f2 && is_plane(&c, f1) && is_plane(&c, f2) {
            found = Some((vk, orbit[0], orbit[1]));
            break;
        }
    }
    let (pole, h0, h1) = found.expect("the cone's base disc has a planar pole");
    println!("R2-P2 planar pole = {pole:?}");

    // kef the FIRST meridian: kills that edge and one half-disc face.
    let kef_res = c.kef(h0);
    println!("R2-P2 kef => {:?}", kef_res.map(|_| "ok"));
    println!("R2-P2 after kef counts = {:?}", counts(&c));
    println!("R2-P2 after kef validate(tier1) = {:?}", validate(&c));
    println!(
        "R2-P2 after kef validate_closed(tier2) = {:?}",
        validate_closed(&c)
    );

    // kev the surviving meridian, killing the pole: pass the half
    // whose END is the pole, i.e. the mate of h1.
    let mate = c.mate(h1).expect("mate");
    let kev_res = c.kev(mate);
    println!("R2-P2 kev => {:?}", kev_res.map(|_| "ok"));
    println!("R2-P2 after kev counts = {:?}", counts(&c));
    println!("R2-P2 tier1  = {:?}", validate(&c));
    println!("R2-P2 tier2  = {:?}", validate_closed(&c));
    println!(
        "R2-P2 tier3  = {:?}",
        validate_geometric(&c, Tol::witness())
    );
    dump_poles(&c, "cone-after-kef-kev");
    let b = brick_operand();
    println!(
        "R2-P2 union(one-face-cap cone, brick) => {:?}",
        boolean_reduce(BooleanOp::Union, &c, &b, Tol::witness()).map(|_| "Ok")
    );
    let _ = Point3::new(0.0, 0.0, 0.0);
}

// ---------------------------------------------------------------
// ACCEPTANCE (VERBS/F7, added to R2's fixtures): the pole positive.
// ---------------------------------------------------------------

/// **The unit's headline acceptance.** A full revolve's axis-touching
/// planar cap is two half-discs sharing the two halves of the disc's
/// DIAMETER, meeting at the pole — a vertex interior to one straight
/// carrier. `merge_coplanar_faces` must now repair exactly that: the
/// pair becomes ONE face bounded by its rim alone, the pole vertex
/// goes with the seam, and the body stays valid.
///
/// The fixture is R2's own `cone()` (a triangle revolved about the
/// axis it touches). What is asserted is the repair, not the trigger:
/// face and vertex counts drop by the merge's own arithmetic, and the
/// result validates closed and geometric.
#[test]
fn f7_pole_split_cap_repairs_to_one_face() {
    let tol = Tol::witness();
    let mut c = cone();
    dump_poles(&c, "cone-before");
    let planar_same_key = |b: &Body<f64>| {
        b.edges()
            .filter(|(_, e)| {
                let (fp, fm) = (face_of(b, e.he_plus), face_of(b, e.he_minus));
                fp != fm
                    && is_plane(b, fp)
                    && b.get_face(fp).map(|f| f.surface) == b.get_face(fm).map(|f| f.surface)
            })
            .count()
    };
    let (f0, v0, seams0) = (c.faces().count(), c.vertices().count(), planar_same_key(&c));
    let outcome = c.merge_coplanar_faces(tol);
    println!(
        "[f7-accept] cone before: faces={f0} vertices={v0} planar-same-key-edges={seams0}; \
         merge={:?}",
        outcome.as_ref().map(|o| o.groups.len())
    );
    let outcome = outcome.expect("the pole-split cap repairs");
    assert_eq!(outcome.groups.len(), 1, "one cap group merged");
    let (f1, v1, seams1) = (c.faces().count(), c.vertices().count(), planar_same_key(&c));
    println!("[f7-accept] cone after:  faces={f1} vertices={v1} planar-same-key-edges={seams1}");
    assert_eq!(f1, f0 - 1, "the two half-discs became ONE face");
    assert_eq!(v1, v0 - 1, "the pole vertex went with the seam");
    assert_eq!(
        seams1, 0,
        "no planar same-key pair survives — the cap is maximal"
    );
    assert_eq!(validate_closed(&c), Ok(()), "tier 2");
    assert_eq!(validate_geometric(&c, tol), Ok(()), "tier 3");
}
