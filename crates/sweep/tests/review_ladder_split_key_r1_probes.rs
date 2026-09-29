//! **Reviewer probes: the ladder rim phase's split key through the
//! doors the unit's census did not carve.**
//!
//! The unit's census instrumented every ladder carve the `sweep` suite
//! runs and read the meridian direction off it; its reading of WHAT
//! fixes that direction is "a REVOLVE-minted cap seam runs pole to rim,
//! a BOOLEAN-minted pip seam runs rim to pole". These rows carve the
//! bodies that reading did not cover, and read the direction off
//! `he_plus` the same way, so the reading is measured rather than
//! generalized from the doors the tree happens to ship.
//!
//! - The pip and the boss cut into (grown out of) a slab's UNDERSIDE —
//!   the spec's Phase 1 door (a), which the census made unnecessary and
//!   therefore never ran. The ball is the same `ball_poled_z`, the
//!   slab's face is the other one, and the surviving half of the seam
//!   is the other half.
//! - The dome rim carved on a body that is itself a blend OUTPUT (the
//!   boss's ladder, then its top annulus, the sequential recourse the
//!   mixed-pair refusal names), with the naming walk run on each carve
//!   rather than on the first only.
//!
//! Every row ends at `assert_naming_totality`, whose direction (b) is
//! the one the unit moves: a retirement names a key the caller handed
//! in.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceKind;
use geom_core::{Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, ball_poled_z, boss, cube, realized, rim_arcs_at,
};
use topo::boolean::BooleanOp;
use topo::query::{self, SurfaceKindSet};
use topo::{Body, EdgeKey, VertexKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The rim's vertices, in no particular order.
fn rim_vertices(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<VertexKey> {
    let mut out: Vec<VertexKey> = arcs
        .iter()
        .flat_map(|e| {
            let ed = body.get_edge(*e).unwrap();
            [ed.he_plus, ed.he_minus]
                .into_iter()
                .map(|h| body.get_half_edge(h).unwrap().start)
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Per rim vertex: whether the meridian dropping into the cap has its
/// `he_plus` STARTING there. `split_edge` keeps the parent key on the
/// child carrying `start(he_plus)`, so `true` is "the rim-side piece of
/// the split keeps the source key" and `false` is "it is minted".
fn rim_side_keeps_the_source_key(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<bool> {
    rim_vertices(body, arcs)
        .into_iter()
        .map(|v| {
            let meridians: Vec<EdgeKey> = body
                .edges_of_vertex(v)
                .unwrap()
                .into_iter()
                .filter(|e| !arcs.contains(e))
                .collect();
            let [m] = meridians[..] else {
                panic!("a ladder rim vertex drops exactly one meridian into the cap")
            };
            let hp = body.get_edge(m).unwrap().he_plus;
            body.get_half_edge(hp).unwrap().start == v
        })
        .collect()
}

/// The plane×sphere rim of a slab with one ball booleaned into a face.
fn plane_sphere_rim(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect()
}

const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;

/// **The boolean door with the surviving half of the seam swapped**: a
/// pip cut into the slab's UNDERSIDE and a boss grown out of it, both
/// from the same `ball_poled_z` the top-face fixtures use.
///
/// The census read the pip's rim-to-pole direction off the top-face
/// fixtures and named the BOOLEAN the thing that fixes it. What
/// actually fixes it is which half of the ball's seam survives the
/// boolean: under the slab the pip keeps the OTHER half and its
/// meridians run pole to rim, so the boolean door reaches BOTH
/// orientations and the rim-side piece of this pip's split is minted.
/// Both carves are naming-total.
#[test]
fn r1_a_pip_and_a_boss_on_the_slabs_underside_carve_naming_total() {
    for (op, name, rim_side_keeps) in [
        (BooleanOp::Subtract, "the underside pip", false),
        (BooleanOp::Union, "the underside boss", true),
    ] {
        let slab = cube(1.0, tol());
        // The ball's centre below `z = 0` by `PIP_R - PIP_H`, so the
        // cap standing proud of the slab's underside is `PIP_H` deep:
        // the top-face fixture's arrangement, mirrored.
        let ball = ball_poled_z(PIP_R, Vec3::new(0.5, 0.5, -(PIP_R - PIP_H)), tol());
        let source = realized(op, &slab, &ball, tol());
        let rim = plane_sphere_rim(&source);
        assert_eq!(rim.len(), 2, "{name}: the rim is two arcs across the seam");
        let keeps = rim_side_keeps_the_source_key(&source, &rim);
        assert_eq!(
            keeps.len(),
            2,
            "{name}: two rim vertices, one per seam crossing"
        );
        assert_eq!(
            keeps,
            vec![rim_side_keeps; keeps.len()],
            "{name}: the direction is the SURVIVING half of the ball's seam, not the \
             door — the underside pip keeps the half whose `he_plus` ENDS at the rim \
             vertex and the underside boss the half whose `he_plus` STARTS there"
        );
        let out = fillet_edges(&source, &rim, 0.02, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name} is tier-3 valid, got {e:?}"));
        assert_naming_totality(&source, &out, &rim, name);
    }
}

/// **A ladder carve whose SOURCE body is itself a blend output.**
///
/// The boss's dome rim (a ladder) and its top outer rim (a hostless
/// annulus) share a support, so one call refuses them with the
/// sequential recourse; taking that recourse makes the second call's
/// source a body carrying this crate's own mints. Both carves are
/// walked, so a retirement naming a key the FIRST carve minted — which
/// is still a source key for the second, and is not one for the first —
/// is caught in whichever call records it.
#[test]
fn r1_the_bosss_sequential_recourse_is_naming_total_at_both_steps() {
    for up in [true, false] {
        let name = if up { "the boss" } else { "the dimple" };
        let mut source = boss(up, tol());
        source
            .merge_coplanar_faces(tol())
            .expect("the pole-split caps repair");
        let ladder = rim_arcs_at(&source, 0.5, 1.0);
        assert_eq!(ladder.len(), 2, "{name}: the dome rim is two arcs");
        let first = fillet_edges(&source, &ladder, 0.1, tol())
            .unwrap_or_else(|e| panic!("{name}: the dome rim carves, got {e:?}"));
        assert_naming_totality(&source, &first, &ladder, &format!("{name}: the dome rim"));
        let mid = first.body.clone();
        let annulus = rim_arcs_at(&mid, 1.0, 1.0);
        assert!(
            !annulus.is_empty(),
            "{name}: the top outer rim survives the first carve"
        );
        let second = fillet_edges(&mid, &annulus, 0.1, tol())
            .unwrap_or_else(|e| panic!("{name}: the top rim carves second, got {e:?}"));
        validate_geometric(&second.body, tol())
            .unwrap_or_else(|e| panic!("{name}: the composed body is tier-3 valid, got {e:?}"));
        assert_naming_totality(&mid, &second, &annulus, &format!("{name}: the top rim"));
    }
}
