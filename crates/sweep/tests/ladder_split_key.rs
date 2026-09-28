//! **The ladder rim phase retires only SOURCE keys, whichever way the
//! meridian runs.**
//!
//! `Body::split_edge` hands the parent key to the child that carries
//! `start(he_plus)`: split a meridian `m` at a rim vertex `v` and the
//! piece still touching `v` keeps `m`'s key when `m`'s `he_plus` STARTS
//! at `v`, and is a FRESH key when `he_plus` ENDS there. The ladder's
//! rim phase kills that piece at every non-closure vertex, so which way
//! the meridian runs decides whether the key it retires is one the
//! caller handed in.
//!
//! **Neither DOOR decides the orientation.** What decides it is which
//! end of the seam's own stored direction the rim vertex sits at, and
//! every door mints both: a boolean pip cut into a slab's top runs
//! rim-to-pole and one cut into its underside runs pole-to-rim
//! (`review_ladder_split_key_r2_probes::r2_a_pip_cut_from_below_runs_pole_to_rim_through_the_boolean_door`),
//! a revolve's pole-touching cap seam runs pole-to-rim and so does the
//! `slab ∪ ball` boss's surviving union piece
//! (`review_ladder_split_key_r2_probes::r2_the_h4_boss_and_pip_are_naming_total`
//! — every revolve or union boss in this tree that reaches the ladder
//! runs pole-to-rim; the boolean pip's two sides are the one door here
//! that shows both orientations), and the extruded two-arc rims are
//! minted fresh-key through the extrude door
//! (`review_ladder_split_key_r2_probes::r2_the_extruded_two_arc_ladders_are_naming_total_whichever_way_their_seams_run`).
//!
//! This suite carries one fixture of each orientation:
//!
//! - the repaired BOSS's dome rim, whose cap seams run pole-to-rim, so
//!   the rim-side piece of each split is minted;
//! - the pipped DIE's pip rim, whose seam runs rim-to-pole, so the
//!   rim-side piece keeps the source key.
//!
//! Both carve to a tier-3-valid solid whose every retirement names a
//! key the caller handed in.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::SurfaceKind;
use geom_core::{Tol, Vec3};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, ball_poled_z, boss, cube, plane_sphere_external_cut, realized,
    rim_arcs_at,
};
use topo::boolean::BooleanOp;
use topo::query::{self, SurfaceKindSet};
use topo::{Body, EdgeKey, VertexKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The boss after the repair every boolean consumer runs: its flat top
/// is one plane annulus and the dome rim is that annulus's ring, which
/// is what routes the rim to the LADDER door.
fn repaired_boss(up: bool) -> Body<f64> {
    let mut b = boss(up, tol());
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
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

/// **Which end of each rim meridian's stored direction the rim vertex
/// sits at**, per rim vertex: `true` when the meridian's `he_plus`
/// STARTS at the rim vertex, `false` when it ends there.
///
/// **What this pins is the FIXTURE, not `split_edge`'s rule.** Reading
/// `he_plus` and then naming the answer after the retention rule (the
/// parent survives as the first child, `start(he_plus)` → the new
/// vertex) restates that rule rather than measuring it, so a rule that
/// moved would leave the rows below green and quietly witnessing
/// nothing. The rule itself is measured — by splitting each meridian on
/// a CLONE and comparing which piece kept the key — at
/// `review_ladder_split_key_r2_probes::r2_the_split_rule_is_measured_by_splitting_not_read_off_prose`,
/// which agrees with this reading on both orientations. What the rows
/// here add is that these two bodies carry the two orientations, so a
/// fixture whose meridian direction changed goes red here instead of
/// quietly ceasing to witness the minted-piece case.
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

/// **The witness: a ladder rim whose every meridian runs POLE TO RIM.**
///
/// `test_support::boss`'s dome is a revolve of a pole-touching profile,
/// and each of its cap seams has the rim vertex at the END of its
/// stored direction: `he_plus` starts at the pole. `split_edge`
/// therefore keeps the source key on the piece AWAY from the rim and
/// mints the piece the rim phase kills. The carve is tier-3 valid at
/// its closed form, and every retirement it records names a key the
/// caller handed in.
///
/// Both material sides: the boss's dome rim is concave and its dimple
/// twin's is convex, one form and two signs.
#[test]
fn the_boss_dome_rims_meridians_run_pole_to_rim_and_the_carve_is_naming_total() {
    let form = plane_sphere_external_cut(0.5, 0.1);
    for (up, adds) in [(true, true), (false, false)] {
        let name = if up {
            "the boss dome rim"
        } else {
            "the dimple rim"
        };
        let source = repaired_boss(up);
        let arcs = rim_arcs_at(&source, 0.5, 1.0);
        assert_eq!(arcs.len(), 2, "{name}: the rim is two arcs across the seam");
        let keeps = rim_side_keeps_the_source_key(&source, &arcs);
        assert_eq!(
            keeps,
            vec![false; keeps.len()],
            "{name}: every meridian's he_plus ENDS at its rim vertex, so the rim-side \
             piece of each split is a fresh key — this is what the fixture witnesses"
        );
        let before = mass_properties(&source, tol()).unwrap();
        let out = fillet_edges(&source, &arcs, 0.1, tol())
            .unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name} is tier-3 valid, got {e:?}"));
        let after = mass_properties(&out.body, tol()).unwrap();
        assert_eq!(after.volume_pad, 0.0, "{name}: closed-form faces only");
        let want = if adds { form } else { -form };
        let measured = after.volume - before.volume;
        assert!(
            (measured - want).abs() <= 1e-12 * want.abs().max(1.0),
            "{name}: the carve's volume delta is the hand form (measured {measured}, \
             closed form {want})"
        );
        assert_naming_totality(&source, &out, &arcs, name);
    }
}

/// A die with one pip cut into its top: the pip rim is a plane–sphere
/// closed chain whose two arcs meet at the ball's seam, and it takes
/// the ladder door.
fn pipped_die() -> (Body<f64>, Vec<EdgeKey>) {
    const PIP_R: f64 = 0.09;
    const PIP_H: f64 = 0.05;
    let die = cube(1.0, tol());
    let pip = ball_poled_z(PIP_R, Vec3::new(0.5, 0.5, 1.0 + (PIP_R - PIP_H)), tol());
    let pipped = realized(BooleanOp::Subtract, &die, &pip, tol());
    let rim: Vec<EdgeKey> = query::all_edges(&pipped)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                &pipped,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect();
    (pipped, rim)
}

/// **The other orientation, on the same walk**: this pip's seam
/// meridian runs RIM to pole, so `split_edge` keeps the source key on
/// the piece the rim phase kills and the retirement it records is that
/// source key. The pip's DOOR does not decide that — the same boolean
/// with the ball under the slab runs the other way (the module docs
/// cite the row) — it is where the rim vertex sits on the seam's own
/// stored direction.
///
/// It is here beside the witness because the witness's claim is a
/// comparison: a reading of `he_plus` that answered "minted" everywhere
/// would leave that row green for the wrong reason, and this one red.
#[test]
fn the_pip_rims_meridians_run_rim_to_pole_and_the_carve_is_naming_total() {
    let (source, rim) = pipped_die();
    assert_eq!(
        rim.len(),
        2,
        "the pip rim is two arcs across the ball's seam"
    );
    let keeps = rim_side_keeps_the_source_key(&source, &rim);
    assert_eq!(
        keeps,
        vec![true; keeps.len()],
        "the pip rim: every meridian's he_plus STARTS at its rim vertex, so the rim-side \
         piece of each split keeps the source key"
    );
    let out = fillet_edges(&source, &rim, 0.05, tol())
        .unwrap_or_else(|e| panic!("the pip rim carves, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("the pip rim is tier-3 valid, got {e:?}"));
    assert_naming_totality(&source, &out, &rim, "the pip rim");
}
