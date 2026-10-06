//! **A closed chain's junctions are judged against the links that
//! touch them.** A chain's junction carries the two links the walk
//! found incident to it (`battery::Junction`), and the G1 check reads
//! those — so a closed rim of N ≥ 3 arcs, whose junction list in walk
//! order is rotated against its ring order, is judged at every
//! junction between the two arcs that meet there and carves, where a
//! positional pairing read one far-end tangent per junction and
//! refused `ChainNotG1`. A two-arc rim cannot tell the two pairings
//! apart (both links touch both vertices), and two arcs was the only
//! closed rim any suite built.
//!
//! The fixtures are the plane–cylinder closed rims `extrude` mints
//! from a circle authored as N arcs, on both material sides and
//! through both closed-rim doors: a disc's raised rim and a pocket's
//! floor rim sit in their host's OUTER cycle (the annulus with strut
//! crossings), a through-bore's cap rim and a boss's foot rim sit in a
//! RING of their host (the ladder). Every carve is graded against the
//! homed Pappus oracle `test_support::wedge_fill` at the meridian
//! corner — a right angle between the cap plane and the wall — with
//! `volume_pad == 0` on both sides.
//!
//! **What each row pins, and what it does not.** The walk's RECORD and
//! the check's READ of it are two things, and a mutant can break one
//! without the other:
//!
//! - `every_junction_of_every_walked_chain_touches_both_its_links` —
//!   the record: over discs N = 2…5 and bores, bosses and pockets
//!   N = 2…4, each walked from every seed rotation and once in reversed
//!   order, and the open cube chain — every junction's `arriving` and
//!   `leaving` link has the vertex as an end, and the two are
//!   consecutive in walk order. Red under a walk that records the
//!   merge base's positional pairs; GREEN under a check that mis-reads
//!   a correct record, which is the carve rows' job to catch.
//! - `n_arc_discs_carve_the_annulus_at_the_convex_closed_form`
//!   (N = 2…5),
//!   `n_arc_pocket_floors_carve_the_annulus_at_the_concave_closed_form`
//!   and `n_arc_bores_and_boss_feet_carve_the_ladder_at_their_closed_forms`
//!   (N = 2…4) — the read: the carve succeeds at the closed form only if
//!   the check judged each junction between its own two links. N = 2 in
//!   each is the control every other suite builds; one N past each
//!   range is `review_closed_chain_junctions_r2_probes`'.
//! - `an_open_three_link_chain_refuses_chain_g1_at_its_first_junction`
//!   — the open case, which no pairing ever broke: three cube edges in a
//!   row, one junction at each inner vertex between exactly the two
//!   links that meet there, and the 90° refusal at the FIRST junction is
//!   the verdict that pairing owes.
//! - `a_self_closed_link_counts_its_vertex_twice` — the walk's
//!   incidence: a self-closed link beside one other link at its vertex
//!   is a corner, not a junction.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom_core::Tol;
use sweep::blend::BlendError;
use sweep::blend::battery::{BlendRequest, Chain, ChainClosure, run_battery};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, cube, disc_of_arcs, dome, one_edge_rim_at,
    pocket_of_arcs, resolved_links, walked_chains, walked_links, wedge_fill,
};
use topo::{Body, EdgeKey, VertexKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// The wall radius every fixture here is authored at, meters.
const R: f64 = 0.5;
/// The fillet radius, meters.
const RHO: f64 = 0.1;
/// The block side of the bored, bossed and pocketed fixtures.
const L: f64 = 2.0;

/// **The volume a plane–cylinder fillet of radius [`RHO`] moves at a
/// wall of radius [`R`]**, from the homed Pappus oracle: the meridian
/// corner `(R, 0)` and the two generators leaving it — the cap plane,
/// toward the axis when `inward` (a disc's rim, a pocket's floor: the
/// region lies in `r ∈ [R−ρ, R]`) or away from it (a bore's cap rim, a
/// boss's foot: `r ∈ [R, R+ρ]`), and the wall downward — a right angle
/// either way. A convex rim REMOVES this volume, a concave one ADDS it;
/// the caller supplies the sign.
fn corner_fill(inward: bool) -> f64 {
    let along_cap = if inward { (-1.0, 0.0) } else { (1.0, 0.0) };
    wedge_fill((R, 0.0), along_cap, (0.0, -1.0), RHO)
}

/// `(vertices, edges, faces)`.
fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// A volume whose closed-form inventory is checked: every face of
/// these bodies is a plane, a cylinder or the band's torus, so the pad
/// is exactly zero.
fn volume(body: &Body<f64>, what: &str) -> f64 {
    let props = mass_properties(body, tol()).expect("mass properties compute");
    assert_eq!(props.volume_pad, 0.0, "{what}: closed-form faces only");
    props.volume
}

/// Three consecutive edges of the unit cube's top face: an OPEN
/// three-link chain with a junction at each of its two inner vertices
/// (two requested links meet there) and a free end at each outer one.
fn three_top_edges_in_a_row(body: &Body<f64>) -> Vec<EdgeKey> {
    let z_of = |v: VertexKey| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
    let top: Vec<EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .all(|&he| (z_of(body.get_half_edge(he).unwrap().start) - 1.0).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(top.len(), 4, "the top face has four edges");
    // Three of a 4-cycle's edges are consecutive whichever one is left out.
    top[..3].to_vec()
}

/// One carve through the public door, checked the same way for every
/// fixture: one band, tier-3 valid, the `+N, +N+1, +1` census delta,
/// and `V₁ − V₀` equal to `signed` — the oracle with the material
/// side's sign — to well inside the agreement measured (~1e-15 on
/// volumes of order 1–10).
fn carve_and_check(body: &Body<f64>, arcs: &[EdgeKey], signed: f64, what: &str) {
    let n = arcs.len();
    let v0 = volume(body, what);
    let c0 = census(body);
    let out = fillet_edges(body, arcs, RHO, tol())
        .unwrap_or_else(|e| panic!("{what}: the whole rim carves, got {e}"));
    assert_eq!(out.band_faces.len(), 1, "{what}: one band");
    assert!(
        out.blend_faces.is_empty() && out.corner_faces.is_empty(),
        "{what}: a closed rim has no ends to blend or corner"
    );
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: tier-3 valid, got {e:?}"));
    let c1 = census(&out.body);
    assert_eq!(
        (c1.0 - c0.0, c1.1 - c0.1, c1.2 - c0.2),
        (n, n + 1, 1),
        "{what}: one vertex and one edge per crossing, one more edge, one band face"
    );
    let moved = volume(&out.body, what) - v0;
    assert!(
        (moved - signed).abs() < 1e-13,
        "{what}: V₁ − V₀ = {moved} against wedge_fill's {signed}"
    );
}

/// **The pairing invariant, on the walk's RECORD.** Every junction of
/// every chain the walk builds names two links that both have the
/// junction's vertex as an end, and the two are consecutive in walk
/// order (the wrap-around pairs the last link with the first). Read
/// off the record before any predicate: on a closed rim of N arcs from
/// every seed rotation and in reversed request order — `walk_chains`
/// seeds from the first requested link, so the rotation moves the
/// closing junction — and on the open cube chain, which the G1 check
/// refuses but the walk pairs.
///
/// Under a walk that records the merge base's pairs (the closing
/// vertex first, junction `i` against links `i` and `i+1`) every
/// N ≥ 3 rim here fails the incidence assertion while the two-arc rims
/// and the open chain pass — the blindness the merge base's corpus
/// had. Under a check that mis-reads a CORRECT record positionally this
/// row stays green: it pins the record, and the carve rows pin that
/// the check reads it.
#[test]
fn every_junction_of_every_walked_chain_touches_both_its_links() {
    let mut closed: Vec<(String, Body<f64>, f64)> = Vec::new();
    for n in 2..=5 {
        closed.push((format!("{n}-arc disc"), disc_of_arcs(n, R, 1.0, tol()), 1.0));
    }
    for n in 2..=4 {
        closed.push((
            format!("{n}-arc bore"),
            bored_block_of_arcs(n, L, 1.0, R, tol()),
            1.0,
        ));
        closed.push((
            format!("{n}-arc boss"),
            boss_of_arcs(n, L, R, 1.0, 2.0, tol()),
            L,
        ));
        closed.push((
            format!("{n}-arc pocket"),
            pocket_of_arcs(n, L, R, 1.5, tol()),
            1.5,
        ));
    }
    let mut walks = 0;
    for (name, body, z) in &closed {
        let arcs = circle_arcs_at_z(body, *z);
        let n = arcs.len();
        assert!(n >= 2, "{name}: a rim of arcs");
        let mut orders: Vec<Vec<EdgeKey>> = (0..n)
            .map(|k| {
                let mut o = arcs.clone();
                o.rotate_left(k);
                o
            })
            .collect();
        orders.push(arcs.iter().rev().copied().collect());
        for order in orders {
            let chains = walked_chains(body, &order, RHO, band());
            assert_eq!(chains.len(), 1, "{name}: one closed chain");
            let chain = &chains[0];
            assert_eq!(chain.closure, ChainClosure::Closed, "{name}");
            assert_eq!(chain.junctions.len(), n, "{name}: one junction per arc");
            assert_incident(chain, name);
            walks += 1;
        }
    }
    let cube = cube(1.0, tol());
    let edges = three_top_edges_in_a_row(&cube);
    let chains = walked_chains(&cube, &edges, RHO, band());
    assert_eq!(chains.len(), 1, "one open chain");
    assert!(matches!(chains[0].closure, ChainClosure::Open { .. }));
    assert_eq!(chains[0].junctions.len(), 2, "two inner vertices");
    assert_incident(&chains[0], "the open cube chain");
    // Thirteen closed fixtures, each walked from every seed and once
    // reversed: N + 1 walks per fixture.
    assert_eq!(
        walks,
        (2..=5).map(|n| n + 1).sum::<usize>() + 3 * (2..=4).map(|n| n + 1).sum::<usize>()
    );
}

/// The incidence assertion itself, on one chain.
fn assert_incident(chain: &Chain<f64>, name: &str) {
    let links: Vec<_> = chain.links().collect();
    let n = links.len();
    for j in &chain.junctions {
        for (role, ix) in [("arriving", j.arriving()), ("leaving", j.leaving())] {
            let l = links[ix];
            assert!(
                l.start == j.vertex || l.end == j.vertex,
                "{name}: the {role} link {:?} of junction {:?} does not touch it ({:?}→{:?})",
                l.edge,
                j.vertex,
                l.start,
                l.end
            );
        }
        assert_eq!(
            j.leaving(),
            (j.arriving() + 1) % n,
            "{name}: a junction's links are consecutive in walk order"
        );
    }
}

/// **The disc's raised rim — the annulus with strut crossings,
/// convex.** N = 3, 4, 5 carve at the closed form; N = 2 is the
/// control.
#[test]
fn n_arc_discs_carve_the_annulus_at_the_convex_closed_form() {
    for n in 2..=5 {
        let body = disc_of_arcs(n, R, 1.0, tol());
        let arcs = circle_arcs_at_z(&body, 1.0);
        assert_eq!(arcs.len(), n, "the raised rim is the N arcs authored");
        carve_and_check(&body, &arcs, -corner_fill(true), &format!("{n}-arc disc"));
    }
}

/// **The pocket's floor rim — the annulus with strut crossings,
/// CONCAVE.** The floor is the tool cap's plane whose outer cycle is
/// the rim; the band ADDS the corner torus inside the wall.
#[test]
fn n_arc_pocket_floors_carve_the_annulus_at_the_concave_closed_form() {
    for n in 2..=4 {
        let body = pocket_of_arcs(n, L, R, 1.5, tol());
        let arcs = circle_arcs_at_z(&body, 1.5);
        assert_eq!(arcs.len(), n, "the floor rim is the N arcs authored");
        carve_and_check(&body, &arcs, corner_fill(true), &format!("{n}-arc pocket"));
    }
}

/// **The ladder, on both material sides.** A through-bore's cap rim
/// (convex — a 90° material wedge outside the wall) and a boss's foot
/// rim (concave) both sit in a RING of their planar host, so both
/// route to the ladder with cylinder half-band mates; the bore REMOVES
/// the corner torus outside the wall and the boss ADDS it.
#[test]
fn n_arc_bores_and_boss_feet_carve_the_ladder_at_their_closed_forms() {
    for n in 2..=4 {
        let bore = bored_block_of_arcs(n, L, 1.0, R, tol());
        let arcs = circle_arcs_at_z(&bore, 1.0);
        assert_eq!(arcs.len(), n, "the bore's cap rim is the N arcs authored");
        carve_and_check(&bore, &arcs, -corner_fill(false), &format!("{n}-arc bore"));

        let boss = boss_of_arcs(n, L, R, 1.0, 2.0, tol());
        let arcs = circle_arcs_at_z(&boss, L);
        assert_eq!(
            arcs.len(),
            n,
            "the boss's foot rim is the N arcs the top cuts"
        );
        carve_and_check(&boss, &arcs, corner_fill(false), &format!("{n}-arc boss"));
    }
}

/// **The open case.** Three cube edges in a row walk into one open
/// chain whose two junctions are its inner vertices, each between
/// exactly the two links that meet there; and the battery's verdict on
/// it is the one those pairs owe — each junction a definite turn, so
/// chain G1 breaks the chain there, and the first chain end the corner
/// predicate reaches refuses as the turn at the FIRST junction — not a
/// verdict on a far-end tangent. No pairing the tree has had broke the
/// open case; the pin here is the verdict.
#[test]
fn an_open_three_link_chain_refuses_a_turn_at_its_first_junction() {
    let body = cube(1.0, tol());
    let edges = three_top_edges_in_a_row(&body);
    let chains = walked_chains(&body, &edges, RHO, band());
    let chain = &chains[0];
    let links: Vec<_> = chain.links().collect();
    let ChainClosure::Open { head, tail } = chain.closure else {
        panic!("an open chain")
    };
    let inner: Vec<VertexKey> = chain.junctions.iter().map(|j| j.vertex).collect();
    assert!(
        !inner.contains(&head) && !inner.contains(&tail),
        "junctions are the inner vertices"
    );
    for (i, j) in chain.junctions.iter().enumerate() {
        assert_eq!((j.arriving(), j.leaving()), (i, i + 1));
        let shared = [links[i].start, links[i].end]
            .into_iter()
            .find(|v| *v == links[i + 1].start || *v == links[i + 1].end);
        assert_eq!(
            shared,
            Some(j.vertex),
            "the junction is the vertex the two links share"
        );
    }
    match run_battery(
        &BlendRequest {
            body: &body,
            edges,
            size: RHO,
        },
        band(),
    ) {
        Err(BlendError::UnsupportedCorner {
            vertex,
            corner: sweep::blend::CornerConfig::Turn,
            ..
        }) => {
            assert_eq!(
                vertex, chain.junctions[0].vertex,
                "refused at the first junction"
            );
        }
        other => panic!("box edges meet at 90°, got {other:?}"),
    }
}

/// **A self-closed link arrives at its one vertex and leaves it, so it
/// counts there twice.** The dome's equator rim `s` is one edge whose
/// two ends are one vertex `v`. Beside it the walk is handed a second
/// link `o` whose start is rewired to `v` — the incidence a non-seam
/// edge ending at a closed rim's vertex would have, which no body the
/// tree builds carries (a rim's vertex there meets only its walls'
/// co-surface seams, which refuse before the walk). `v` then holds
/// three link-ends, a corner: `s` walks alone into a closed chain with
/// no junction and `o` into an open one ending at `v`. A walk counting
/// `s` once reads `v` as a two-link junction and returns ONE closed
/// chain with two junctions at `v` and a free far end — red here on
/// the chain count. Two self-closed links on one vertex are four ends,
/// a corner too: red on the same count under the single count, which
/// joins them into a closed figure-eight. One self-closed link alone
/// is the control: a closed chain with no junction either way.
#[test]
fn a_self_closed_link_counts_its_vertex_twice() {
    let body = dome(1.0, tol());
    let rims = [
        one_edge_rim_at(&body, 1.0, 0.0),
        one_edge_rim_at(&body, 0.5, 0.0),
    ];
    let [s, other] = <[_; 2]>::try_from(resolved_links(&body, &rims, RHO, band())).unwrap();
    let v = s.start;
    assert_eq!(s.end, v, "the equator rim is self-closed");
    assert_ne!(other.start, v, "the second rim has a vertex of its own");

    let alone = walked_links(vec![s.clone()]);
    assert_eq!(alone.len(), 1, "a lone self-closed link: one chain");
    assert_eq!(
        alone[0].closure,
        ChainClosure::Closed,
        "a lone self-closed link closes"
    );
    assert!(
        alone[0].junctions.is_empty(),
        "a lone self-closed link has no junction"
    );

    let mut o = other.clone();
    o.start = v;
    o.end = other.start;
    let far = o.end;
    let chains = walked_links(vec![s.clone(), o.clone()]);
    assert_eq!(
        chains.len(),
        2,
        "a self-closed link and one other at its vertex: a corner, two chains"
    );
    for chain in &chains {
        assert!(
            chain.junctions.is_empty(),
            "no junction at a corner: {:?}",
            chain.junctions
        );
    }
    let closure_of = |e: EdgeKey| {
        chains
            .iter()
            .find(|c| c.links().any(|l| l.edge == e))
            .map(|c| c.closure)
            .unwrap()
    };
    assert_eq!(
        closure_of(s.edge),
        ChainClosure::Closed,
        "the self-closed link closes alone"
    );
    match closure_of(o.edge) {
        ChainClosure::Open { head, tail } => {
            let mut ends = [head, tail];
            ends.sort();
            let mut want = [v, far];
            want.sort();
            assert_eq!(
                ends, want,
                "the other link's chain ends at the corner and its free end"
            );
        }
        ChainClosure::Closed => panic!("the other link's chain has a free end, so it is open"),
    }

    let mut t = other;
    t.end = v;
    t.start = v;
    let pair = walked_links(vec![s, t]);
    assert_eq!(
        pair.len(),
        2,
        "two self-closed links on one vertex: a corner, two chains"
    );
    for chain in &pair {
        assert_eq!(
            chain.closure,
            ChainClosure::Closed,
            "each self-closed link closes alone"
        );
        assert!(
            chain.junctions.is_empty(),
            "no junction at a corner: {:?}",
            chain.junctions
        );
    }
}
