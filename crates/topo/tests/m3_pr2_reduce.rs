//! M3 PR 2 acceptance: `split_reduce` — reduction + neighborhood
//! classification on the Fig. 14.2-analogue notched block, the two
//! rule-(b) discriminating fixtures (tangent-edge, touching-wedge), the
//! rule-(a) mirror pair, and the ch. 14 mirror-check sites, all on
//! asymmetric solids. Runs at every ε row (coordinates are small
//! integers — dyadic, definite at 1e-6/1e-9/1e-12) and, behind the
//! feature, on the interval lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Band, Point3, Vec3};
use topo::{
    Body, PlaneSide, SplitPlane, SplitReduceError, VertexKey, classify_neighborhood, split_reduce,
};

/// The split plane y = 1, Above = +y (the fixtures' tool).
fn plane_y1<T: geom_core::Decide>() -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(1.0), T::from_f64(0.0)),
    }
}

/// Fig. 14.2 analogue (profile in x–y, extruded along z, split y = 1):
/// a block whose top carries a flat notch floor exactly ON the plane
/// (coplanar-artifact faces, rule (a)) and a V-notch whose tip edge
/// lies exactly ON the plane (the tangent-edge rule-(b) site); the
/// Above side is three disconnected prisms. Deliberately asymmetric.
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0), // → crossing on segment (8,0)→(8,2) at (8,1)
    (7.0, 1.0), // ON (flat notch floor starts)
    (6.0, 1.0), // ON (flat notch floor ends; floor face is IN the plane)
    (5.0, 2.0), // peak
    (4.0, 1.0), // ON — the V-notch tip (tangent edge)
    (3.0, 2.0), // peak
    (0.0, 2.0), // → crossing on closing segment (0,2)→(0,0) at (0,1)
];

/// The touching-wedge fixture: the notched block mirrored through the
/// plane (y ↦ 2 − y, order reversed to stay CCW) — material above, the
/// V tip touches y = 1 from below: the BOB rule-(b) site.
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0), // ON — the Λ tip (touching wedge)
    (5.0, 0.0),
    (6.0, 1.0), // ON (flat notch ceiling — face IN the plane)
    (7.0, 1.0), // ON
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// All vertices of `body` at exactly (x, y, z) (bitwise — fixture
/// coordinates are exact; a reduced body holds coincident null-edge
/// copies at ON positions).
fn vertices_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> Vec<VertexKey> {
    body.vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .map(|(k, _)| k)
        .collect()
}

/// The unique vertex of `body` at (x, y, z).
fn vertex_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> VertexKey {
    let hits = vertices_at(body, x, y, z);
    assert_eq!(hits.len(), 1, "expected one vertex at ({x},{y},{z})");
    hits[0]
}

/// Notched-block acceptance: classes, crossing insertion, and the
/// null-edge census — the whole reduction end to end.
#[test]
fn notched_block_reduction() {
    let fixture = prism::<f64>(NOTCHED, 1.0);
    let operand_counts = (
        fixture.body.vertices().count(),
        fixture.body.edges().count(),
        fixture.body.faces().count(),
    );
    let red = split_reduce(&fixture.body, &plane_y1()).unwrap();

    // Functional: the operand is untouched.
    assert_eq!(
        operand_counts,
        (
            fixture.body.vertices().count(),
            fixture.body.edges().count(),
            fixture.body.faces().count(),
        )
    );

    // ON set: 6 structural (P3, P4, P6 × bottom/top) + 4 inserted
    // crossings; every original vertex classified.
    assert_eq!(red.on_vertices.len(), 10);
    for (i, &(x, y)) in NOTCHED.iter().enumerate() {
        for (vs, z) in [(&fixture.bottom, 0.0), (&fixture.top, 1.0)] {
            let side = red.sides[vs[i]];
            let expect = match y {
                0.0 => PlaneSide::Below,
                1.0 => PlaneSide::On,
                2.0 => PlaneSide::Above,
                _ => unreachable!(),
            };
            assert_eq!(side, expect, "corner {i} at ({x},{y},{z})");
        }
    }

    // Crossing insertion (mirror site: split_edge argument pair /
    // parameter interpolation): the four crossings land bitwise at
    // y = 1 on the x = 0 and x = 8 rims — each position holds the
    // crossing vertex (a member of the ON set) plus its one null-edge
    // copy (ON by construction, not a member).
    for (x, z) in [(8.0, 0.0), (8.0, 1.0), (0.0, 0.0), (0.0, 1.0)] {
        let hits = vertices_at(&red.body, x, 1.0, z);
        assert_eq!(hits.len(), 2, "at ({x},1,{z})");
        assert!(hits.iter().all(|&w| red.sides[w] == PlaneSide::On));
        assert_eq!(
            hits.iter().filter(|w| red.on_vertices.contains(w)).count(),
            1
        );
    }

    // Null-edge census: 2 per V-tip vertex (the adjudicated AOA→BELOW
    // separates the two slant runs), 1 per flat-notch corner, 1 per
    // crossing vertex; none dangling in this fixture.
    assert_eq!(red.null_edges.len(), 12);
    assert!(red.null_edges.iter().all(|r| !r.dangling));
    let minted_at = |v: VertexKey| red.null_edges.iter().filter(|r| r.at_vertex == v).count();
    for z in [0.0, 1.0] {
        assert_eq!(minted_at(vertex_at(&fixture.body, 4.0, 1.0, z)), 2); // V tip
        assert_eq!(minted_at(vertex_at(&fixture.body, 7.0, 1.0, z)), 1);
        assert_eq!(minted_at(vertex_at(&fixture.body, 6.0, 1.0, z)), 1);
    }

    // F9 orientation-as-data invariant (replaces the book's he1/he2
    // encoding): every half-edge leaving an above_end ends Above or
    // ON; every one leaving a below_end ends Below or ON.
    for record in &red.null_edges {
        assert_eq!(record.attr.below_end, record.at_vertex);
        assert_eq!(red.sides[record.attr.above_end], PlaneSide::On);
        for (v, banned) in [
            (record.attr.above_end, PlaneSide::Below),
            (record.attr.below_end, PlaneSide::Above),
        ] {
            let anchor = red.body.get_vertex(v).unwrap().emanating.unwrap();
            for he in red.body.vertex_orbit(anchor).unwrap() {
                let end = red.body.half_edge_end(he).unwrap();
                assert_ne!(red.sides[end], banned, "orbit of {v:?}");
            }
        }
    }

    // Determinism (D9): a second run reproduces the records exactly.
    let red2 = split_reduce(&fixture.body, &plane_y1()).unwrap();
    assert_eq!(red.null_edges, red2.null_edges);
    assert_eq!(red.on_vertices, red2.on_vertices);
}

/// The classified entries of `vertex`, bucketed by where each orbit
/// edge ends: (classes of edges ending at `other`, classes of the
/// remaining edge entries, classes of bisector duplicates).
fn bucket_entries(
    body: &Body<f64>,
    plane: &SplitPlane<f64>,
    vertex: VertexKey,
    other: VertexKey,
) -> (Vec<PlaneSide>, Vec<PlaneSide>, Vec<PlaneSide>) {
    let (sides, _) = topo::vertex_sides(body, plane).unwrap();
    let entries =
        classify_neighborhood(body, plane, &sides, vertex, Band::linear().unwrap()).unwrap();
    let (mut to_other, mut edges, mut dups) = (vec![], vec![], vec![]);
    for e in &entries {
        match e.kind {
            topo::SectorEntryKind::WideBisector => dups.push(e.class),
            topo::SectorEntryKind::Edge => {
                if body.half_edge_end(e.he).unwrap() == other {
                    to_other.push(e.class);
                } else {
                    edges.push(e.class);
                }
            }
        }
    }
    (to_other, edges, dups)
}

/// Rule (b), tangent-edge fixture (F4 adjudication, executable): at
/// the V-notch tip the ON tip edge is flanked ABOVE-ON-ABOVE with a
/// wide BELOW cap bisector — the adjudicated table sends it **BELOW**
/// (the book's verdict; the paper's AOA→ABOVE would merge the runs and
/// pin both Above wedges to one copy: the unrepresentable 4-face
/// edge). Consequence: two null edges, and the two slant half-edges
/// land on **different** vertex copies.
#[test]
fn tangent_edge_aoa_goes_below() {
    let fixture = prism::<f64>(NOTCHED, 1.0);
    let plane = plane_y1();
    let (tip_b, tip_t) = (
        vertex_at(&fixture.body, 4.0, 1.0, 0.0),
        vertex_at(&fixture.body, 4.0, 1.0, 1.0),
    );
    // Classification level: [slant: A, tip: ON→B, slant: A, cap dup: B].
    let (to_other, edges, dups) = bucket_entries(&fixture.body, &plane, tip_b, tip_t);
    assert_eq!(to_other, vec![PlaneSide::Below]); // the adjudicated AOA verdict
    assert_eq!(edges, vec![PlaneSide::Above, PlaneSide::Above]);
    assert_eq!(dups, vec![PlaneSide::Below]);

    // Surgery level: two runs ⇒ two copies; the slant half-edges start
    // at distinct new vertices; the tip edge keeps the old (Below-side)
    // vertices at both ends.
    let red = split_reduce(&fixture.body, &plane).unwrap();
    for tip in [tip_b, tip_t] {
        let copies: Vec<_> = red
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .map(|r| r.attr.above_end)
            .collect();
        assert_eq!(copies.len(), 2);
        assert_ne!(copies[0], copies[1]);
    }
    let tip_edge_halves: Vec<_> = red
        .body
        .edges()
        .filter_map(|(_, e)| {
            let s1 = red.body.get_half_edge(e.he_plus)?.start;
            let s2 = red.body.get_half_edge(e.he_minus)?.start;
            ((s1 == tip_b && s2 == tip_t) || (s1 == tip_t && s2 == tip_b)).then_some(())
        })
        .collect();
    assert_eq!(
        tip_edge_halves.len(),
        1,
        "tip edge stays on the old vertices"
    );
}

/// Rule (b), touching-wedge fixture (the BOB mirror): the Λ tip edge is
/// flanked BELOW-ON-BELOW with a wide ABOVE cap bisector — adjudicated
/// **ABOVE** (book; the paper's BOB→BELOW would leave the two Below
/// wedges pinned through it). Consequence: the tip edge is isolated
/// into its own run (its halves move to Above copies) and the cap
/// bisector run becomes a **dangling** null edge.
#[test]
fn touching_wedge_bob_goes_above() {
    let fixture = prism::<f64>(MIRRORED, 1.0);
    let plane = plane_y1();
    let (tip_b, tip_t) = (
        vertex_at(&fixture.body, 4.0, 1.0, 0.0),
        vertex_at(&fixture.body, 4.0, 1.0, 1.0),
    );
    let (to_other, edges, dups) = bucket_entries(&fixture.body, &plane, tip_b, tip_t);
    assert_eq!(to_other, vec![PlaneSide::Above]); // the adjudicated BOB verdict
    assert_eq!(edges, vec![PlaneSide::Below, PlaneSide::Below]);
    assert_eq!(dups, vec![PlaneSide::Above]);

    let red = split_reduce(&fixture.body, &plane).unwrap();
    for tip in [tip_b, tip_t] {
        let at_tip: Vec<_> = red
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        assert_eq!(at_tip.len(), 2);
        assert_eq!(at_tip.iter().filter(|r| r.dangling).count(), 1);
    }
    // The tip edge's halves left the original vertices for Above copies.
    let moved: Vec<_> = red
        .body
        .edges()
        .filter(|(_, e)| {
            let s1 = red.body.get_half_edge(e.he_plus).unwrap().start;
            let s2 = red.body.get_half_edge(e.he_minus).unwrap().start;
            (s1 == tip_b || s1 == tip_t) && (s2 == tip_b || s2 == tip_t)
        })
        .collect();
    assert!(
        moved.is_empty(),
        "tip edge no longer joins the old vertices"
    );
}

/// Rule (a) mirror pair, on the mirrored fixture's flat notch ceiling
/// (face IN the plane, outward −y ⇒ +n_SP ENTERS material ⇒ ABOVE) —
/// the opposite sense of the notched block's floor (outward +y ⇒
/// BELOW), which `notched_block_reduction`'s census already forces.
#[test]
fn rule_a_mirror_senses() {
    // Notched: floor face outward +y — flanking entries BELOW.
    let fx = prism::<f64>(NOTCHED, 1.0);
    let (a, b) = (
        vertex_at(&fx.body, 7.0, 1.0, 0.0),
        vertex_at(&fx.body, 6.0, 1.0, 0.0),
    );
    let (to_other, _, _) = bucket_entries(&fx.body, &plane_y1(), a, b);
    assert_eq!(to_other, vec![PlaneSide::Below]);

    // Mirrored: ceiling face outward −y — flanking entries ABOVE.
    let fx = prism::<f64>(MIRRORED, 1.0);
    let (a, b) = (
        vertex_at(&fx.body, 6.0, 1.0, 0.0),
        vertex_at(&fx.body, 7.0, 1.0, 0.0),
    );
    let (to_other, _, _) = bucket_entries(&fx.body, &plane_y1(), a, b);
    assert_eq!(to_other, vec![PlaneSide::Above]);
}
