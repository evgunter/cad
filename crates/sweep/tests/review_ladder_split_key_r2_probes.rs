//! **Reviewer probes (R2) for the ladder rim phase's split-key
//! provenance** — bodies the unit's census did not carve, and the
//! split rule re-derived by SPLITTING rather than by reading prose.
//!
//! - `split_edge`'s key rule is measured: each rim meridian is split on
//!   a clone of the body and the piece still touching the rim vertex is
//!   compared against the `he_plus` reading the unit's witness rows
//!   rest on. The two agree everywhere, on both orientations.
//! - The boolean door mints BOTH orientations, not one: a pip cut from
//!   BELOW a slab's underside (a cap from the ball's far pole to the
//!   rim) runs pole-to-rim, a boss grown DOWNWARD runs rim-to-pole —
//!   the orientation is the ball's own seam direction against which
//!   hemisphere survives, not the door's.
//! - A revolve profile authored in the other traversal order mints the
//!   SAME orientation (`profile` canonicalizes traversal, so the spec's
//!   door (b) cannot reach a second one).
//! - Shipped census bodies are naming-total at the head.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_brep::SurfaceKind;
use geom_core::{Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, ball_poled_z, boss, cube, realized, revolved_about_y, rim_arcs_at,
};
use topo::boolean::BooleanOp;
use topo::query::{self, SurfaceKindSet};
use topo::{Body, EdgeKey, VertexKey, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const SLAB: f64 = 1.0;
const BALL_R: f64 = 0.09;
const CAP_H: f64 = 0.05;

/// The edges between a plane face and a sphere face — the one rim of a
/// pip or a boss cut into or grown on a slab.
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

/// The one edge at `v` that is not a rim arc: the meridian into the cap.
fn meridian_at(body: &Body<f64>, v: VertexKey, arcs: &[EdgeKey]) -> EdgeKey {
    let ms: Vec<EdgeKey> = body
        .edges_of_vertex(v)
        .unwrap()
        .into_iter()
        .filter(|e| !arcs.contains(e))
        .collect();
    let [m] = ms[..] else {
        panic!("a ladder rim vertex drops exactly one meridian into the cap, got {ms:?}")
    };
    m
}

fn touches(body: &Body<f64>, e: EdgeKey, v: VertexKey) -> bool {
    let ed = body.get_edge(e).unwrap();
    [ed.he_plus, ed.he_minus]
        .into_iter()
        .any(|h| body.get_half_edge(h).unwrap().start == v)
}

/// The `he_plus` reading the unit's rows rest on: `true` when the
/// meridian's `he_plus` STARTS at the rim vertex.
fn he_plus_starts_at(body: &Body<f64>, m: EdgeKey, v: VertexKey) -> bool {
    let hp = body.get_edge(m).unwrap().he_plus;
    body.get_half_edge(hp).unwrap().start == v
}

/// **The rule, MEASURED**: split the meridian mid-span on a clone and
/// report whether the piece still touching `v` kept the parent key.
fn rim_side_keeps_key_by_splitting(body: &Body<f64>, m: EdgeKey, v: VertexKey) -> bool {
    let mut b = body.clone();
    let ed = b.get_edge(m).unwrap().clone();
    let (t0, t1) = b
        .get_curve_geom(ed.curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    let created = b.split_edge(m, 0.5 * (t0 + t1), tol()).unwrap();
    let parent = touches(&b, m, v);
    let fresh = touches(&b, created.new_edge, v);
    assert_ne!(
        parent, fresh,
        "exactly one piece of the split touches the rim vertex"
    );
    parent
}

/// Per rim vertex: (by splitting, by `he_plus`).
fn orientations(body: &Body<f64>, arcs: &[EdgeKey]) -> Vec<(bool, bool)> {
    rim_vertices(body, arcs)
        .into_iter()
        .map(|v| {
            let m = meridian_at(body, v, arcs);
            (
                rim_side_keeps_key_by_splitting(body, m, v),
                he_plus_starts_at(body, m, v),
            )
        })
        .collect()
}

fn repaired_boss(up: bool) -> Body<f64> {
    let mut b = boss(up, tol());
    b.merge_coplanar_faces(tol())
        .expect("the pole-split caps repair");
    b
}

/// `slab ∪ ball` / `slab ∖ ball` with the ball at height `cz`.
fn slab_with(op: BooleanOp, cz: f64) -> Body<f64> {
    let ball = ball_poled_z(BALL_R, Vec3::new(0.5, 0.5, cz), tol());
    realized(op, &cube(SLAB, tol()), &ball, tol())
}

fn carve_total(name: &str, source: &Body<f64>, arcs: &[EdgeKey], r: f64) {
    let out =
        fillet_edges(source, arcs, r, tol()).unwrap_or_else(|e| panic!("{name} carves, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{name} is tier-3 valid, got {e:?}"));
    assert_naming_totality(source, &out, arcs, name);
}

/// **The `he_plus` reading and the split's own answer agree**, on the
/// unit's two witnesses: the parent key stays with the piece carrying
/// `start(he_plus)`, and nothing else decides it.
#[test]
fn r2_the_split_rule_is_measured_by_splitting_not_read_off_prose() {
    for up in [true, false] {
        let b = repaired_boss(up);
        let arcs = rim_arcs_at(&b, 0.5, 1.0);
        let o = orientations(&b, &arcs);
        assert_eq!(o.len(), 2, "the boss's dome rim has two rim vertices");
        for (split, hp) in &o {
            assert_eq!(
                split, hp,
                "boss(up={up}): the split's answer is the he_plus reading"
            );
            assert!(!split, "boss(up={up}): the rim-side piece is a FRESH key");
        }
    }
    let pip = slab_with(BooleanOp::Subtract, SLAB + (BALL_R - CAP_H));
    let arcs = plane_sphere_rim(&pip);
    let o = orientations(&pip, &arcs);
    assert_eq!(o.len(), 2);
    for (split, hp) in &o {
        assert_eq!(
            split, hp,
            "the pip: the split's answer is the he_plus reading"
        );
        assert!(split, "the pip: the rim-side piece keeps the SOURCE key");
    }
}

/// **A pip cut from BELOW the slab's underside** — the ball sits under
/// the slab and its far (top) cap is the cavity, so the surviving seam
/// piece runs from the ball's upper pole DOWN to the rim: the boolean
/// door mints the pole-to-rim orientation too. The head carves it
/// naming-total.
#[test]
fn r2_a_pip_cut_from_below_runs_pole_to_rim_through_the_boolean_door() {
    let pip = slab_with(BooleanOp::Subtract, -(BALL_R - CAP_H));
    let arcs = plane_sphere_rim(&pip);
    assert_eq!(
        arcs.len(),
        2,
        "the underside pip rim is two arcs across the seam"
    );
    let o = orientations(&pip, &arcs);
    assert_eq!(
        o,
        vec![(false, false); o.len()],
        "the underside pip: every meridian ENDS at its rim vertex, so the rim-side piece \
         is minted — through the boolean door"
    );
    carve_total("the underside pip", &pip, &arcs, 0.05);
}

/// **A boss grown DOWNWARD** — the ball's lower cap hangs under the
/// slab, so the seam piece runs from the rim DOWN to the far pole: the
/// rim-to-pole orientation on a concave (material-adding) rim. Both
/// orientations therefore sit on both material sides.
#[test]
fn r2_a_boss_grown_downward_runs_rim_to_pole_on_a_concave_rim() {
    let b = slab_with(BooleanOp::Union, BALL_R - CAP_H);
    let arcs = plane_sphere_rim(&b);
    assert_eq!(
        arcs.len(),
        2,
        "the underside boss rim is two arcs across the seam"
    );
    let o = orientations(&b, &arcs);
    assert_eq!(
        o,
        vec![(true, true); o.len()],
        "the underside boss: every meridian STARTS at its rim vertex"
    );
    carve_total("the underside boss", &b, &arcs, 0.02);
}

/// **The shipped `slab ∪ ball` boss (the H4 fixture) and the top pip
/// are naming-total at the head** — the boss is one of the census's 14
/// fresh-key rows and ran no totality walk before.
#[test]
fn r2_the_h4_boss_and_pip_are_naming_total() {
    let b = slab_with(BooleanOp::Union, SLAB - (BALL_R - CAP_H));
    let arcs = plane_sphere_rim(&b);
    let o = orientations(&b, &arcs);
    assert_eq!(
        o,
        vec![(false, false); o.len()],
        "the H4 boss: a boolean UNION's surviving seam piece runs pole-to-rim"
    );
    carve_total("the H4 boss", &b, &arcs, 0.02);
    let pip = slab_with(BooleanOp::Subtract, SLAB + (BALL_R - CAP_H));
    let arcs = plane_sphere_rim(&pip);
    carve_total("the H4 pip", &pip, &arcs, 0.02);
}

/// **A revolve profile authored the other way round mints the same
/// orientation**: `profile` canonicalizes traversal, so the spec's door
/// (b) — "the pole-touching profile in the OTHER traversal order" —
/// reaches nothing new. Pinned so the door table's answer is measured.
#[test]
fn r2_a_profile_authored_in_reverse_mints_the_same_meridian_direction() {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    // The bulge rides the segment it starts: reversed, the dome arc is
    // the leg (0,1.5) -> (0.5,1.0) and its bulge sign flips.
    let mut reversed2 = revolved_about_y(
        vec![
            (Point2::new(0.0, 1.5), -q),
            (Point2::new(0.5, 1.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    reversed2
        .merge_coplanar_faces(tol())
        .expect("the reversed boss repairs");
    let arcs = rim_arcs_at(&reversed2, 0.5, 1.0);
    assert_eq!(arcs.len(), 2, "the reversed boss's dome rim is two arcs");
    let o = orientations(&reversed2, &arcs);
    let forward = orientations(
        &repaired_boss(true),
        &rim_arcs_at(&repaired_boss(true), 0.5, 1.0),
    );
    assert_eq!(
        o, forward,
        "authoring order does not reach the other orientation: the profile canonicalizes"
    );
    carve_total("the reversed boss", &reversed2, &arcs, 0.1);
}

/// **The EXTRUDED ladders** — `n`-arc discs, bores, bosses and pockets
/// whose meridians are the extrude's seam LINES, a third door beside
/// the revolve's and the sphere boolean's. Per fixture the rule is
/// measured by splitting, the reading is uniform around the rim, and
/// the carve is naming-total on both material sides.
#[test]
fn r2_the_extruded_two_arc_ladders_are_naming_total_whichever_way_their_seams_run() {
    use sweep::test_support::{
        bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, disc_of_arcs, pocket_of_arcs,
    };
    for (name, body, z) in [
        ("two_arc_disc", disc_of_arcs(2, 0.5, 1.0, tol()), 1.0),
        (
            "two_arc_bore",
            bored_block_of_arcs(2, 2.0, 1.0, 0.5, tol()),
            1.0,
        ),
        (
            "two_arc_boss",
            boss_of_arcs(2, 2.0, 0.5, 1.0, 2.0, tol()),
            2.0,
        ),
        (
            "two_arc_pocket",
            pocket_of_arcs(2, 2.0, 0.5, 1.5, tol()),
            1.5,
        ),
    ] {
        let arcs = circle_arcs_at_z(&body, z);
        assert_eq!(arcs.len(), 2, "{name}: two arcs by authoring");
        let o = orientations(&body, &arcs);
        for (split, hp) in &o {
            assert_eq!(
                split, hp,
                "{name}: the split's answer is the he_plus reading"
            );
        }
        let first = o[0].0;
        assert!(
            o.iter().all(|(s, _)| *s == first),
            "{name}: one extrude mints its seams one way round"
        );
        println!("R2 {name}: rim-side keeps the source key = {first}");
        carve_total(name, &body, &arcs, 0.1);
    }
}
