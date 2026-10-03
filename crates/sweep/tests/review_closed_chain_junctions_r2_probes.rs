//! **R2's probes for the closed-chain junction record.** The claim
//! under test is that a junction names the two links the BODY says meet
//! at its vertex — read here off the body's own half-edges rather than
//! off `Link::start`/`Link::end`, so a link record that disagreed with
//! the topology could not make the pairing look right.
//!
//! Three shapes the unit's own suite does not build: a rim whose links
//! are not all arcs (a bulged hexagon's extruded rim: three lines and
//! three arcs), a request carrying TWO closed chains at once
//! (both rims of one disc), and requests in scrambled order rather than
//! rotations and a reversal. Plus the open chain seeded from its MIDDLE
//! link, which is the only shape that grows the walk BACKWARD — the arm
//! that records `(vertex, next, own)` and the one the cube row's
//! forward-seeded chain never reaches.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::approx::band;
use geom_core::{Point2, Tol};
use sweep::blend::battery::{Chain, ChainClosure};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, cube, disc_of_arcs, pocket_of_arcs, prism,
    walked_chains, wedge_fill,
};
use topo::{Body, EdgeKey, VertexKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.5;
const RHO: f64 = 0.1;
const L: f64 = 2.0;

/// **An edge's two vertices, read off the body** — `he_plus`'s start
/// and its mate's start. Independent of `Link::start`/`Link::end`,
/// which is what the pairing row under review reads.
fn endpoints(body: &Body<f64>, e: EdgeKey) -> (VertexKey, VertexKey) {
    let ed = body.get_edge(e).expect("the edge is in the body");
    (
        body.get_half_edge(ed.he_plus).expect("he_plus").start,
        body.get_half_edge(ed.he_minus).expect("he_minus").start,
    )
}

/// Every junction of `chain` names exactly the two requested links the
/// BODY says meet at its vertex, and no other.
fn assert_pairing_off_the_body(
    body: &Body<f64>,
    requested: &[EdgeKey],
    chain: &Chain<f64>,
    what: &str,
) {
    let links: Vec<EdgeKey> = chain.links().map(|l| l.edge).collect();
    let n = links.len();
    let mut seen: Vec<VertexKey> = Vec::new();
    for j in &chain.junctions {
        let named = [j.arriving(), j.leaving()];
        assert!(
            named.iter().all(|&i| i < n),
            "{what}: a junction indexes a link the chain does not carry"
        );
        assert_ne!(
            j.arriving(),
            j.leaving(),
            "{what}: a junction names one link twice"
        );
        for &i in &named {
            let (a, b) = endpoints(body, links[i]);
            assert!(
                a == j.vertex || b == j.vertex,
                "{what}: link {:?} ({a:?}→{b:?}) does not touch junction {:?}",
                links[i],
                j.vertex
            );
        }
        // The converse: the requested links that touch this vertex are
        // EXACTLY the two named. A pairing that named an incident link
        // of some other chain would pass the incidence check alone.
        let mut touching: Vec<EdgeKey> = requested
            .iter()
            .copied()
            .filter(|&e| {
                let (a, b) = endpoints(body, e);
                a == j.vertex || b == j.vertex
            })
            .collect();
        touching.sort();
        let mut named_edges = vec![links[j.arriving()], links[j.leaving()]];
        named_edges.sort();
        assert_eq!(
            touching, named_edges,
            "{what}: the requested links at {:?} are not the two named",
            j.vertex
        );
        assert!(
            !seen.contains(&j.vertex),
            "{what}: {:?} is recorded as a junction twice",
            j.vertex
        );
        seen.push(j.vertex);
    }
    let want = match chain.closure {
        ChainClosure::Closed => n,
        ChainClosure::Open { .. } => n - 1,
    };
    assert_eq!(
        chain.junctions.len(),
        want,
        "{what}: one junction per adjacent pair, plus the wrap on a closed chain"
    );
}

/// **A hexagonal prism whose rim links are NOT all arcs**: a regular
/// hexagon of circumradius 1 whose alternate sides are bulged, so its
/// top rim is a closed chain of three LINES and three arcs. No suite
/// builds a closed chain of mixed link kinds.
fn mixed_rim_prism(h: f64) -> Body<f64> {
    let verts: Vec<(Point2<f64>, f64)> = (0..6)
        .map(|i| {
            let th = PI / 3.0 * f64::from(i);
            let bulge = if i % 2 == 0 { 0.2 } else { 0.0 };
            (Point2::new(th.cos(), th.sin()), bulge)
        })
        .collect();
    prism(verts, h, tol())
}

/// Every edge both of whose ends sit at station `z`.
fn edges_at_station(body: &Body<f64>, z: f64) -> Vec<EdgeKey> {
    let z_of = |v: VertexKey| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
    body.edges()
        .filter(|(k, _)| {
            let (a, b) = endpoints(body, *k);
            (z_of(a) - z).abs() < 1e-12 && (z_of(b) - z).abs() < 1e-12
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The pairing, off the body, over every shape I could build.** Rims
/// of 2…6 arcs on four fixtures, a mixed line-and-arc rim, two closed
/// chains in one request, and request orders that are neither
/// rotations nor the reversal.
#[test]
fn r2_every_junction_pairs_the_links_the_body_says_meet_there() {
    let mut cases: Vec<(String, Body<f64>, f64)> = Vec::new();
    for n in 2..=6 {
        cases.push((format!("{n}-arc disc"), disc_of_arcs(n, R, 1.0, tol()), 1.0));
    }
    for n in 2..=5 {
        cases.push((
            format!("{n}-arc bore"),
            bored_block_of_arcs(n, L, 1.0, R, tol()),
            1.0,
        ));
        cases.push((
            format!("{n}-arc boss"),
            boss_of_arcs(n, L, R, 1.0, 2.0, tol()),
            2.0,
        ));
        cases.push((
            format!("{n}-arc pocket"),
            pocket_of_arcs(n, L, R, 1.5, tol()),
            1.5,
        ));
    }
    for (name, body, z) in &cases {
        let arcs = circle_arcs_at_z(body, *z);
        let n = arcs.len();
        for order in scrambles(&arcs) {
            let chains = walked_chains(body, &order, RHO, band());
            assert_eq!(chains.len(), 1, "{name}: one closed chain");
            assert_eq!(chains[0].closure, ChainClosure::Closed, "{name}");
            assert_eq!(chains[0].link_count(), n, "{name}: every link walked");
            assert_pairing_off_the_body(body, &order, &chains[0], name);
        }
    }

    // A rim whose links are NOT all arcs: three lines and three arcs.
    let rs = mixed_rim_prism(1.0);
    let rim = edges_at_station(&rs, 1.0);
    assert_eq!(rim.len(), 6, "the hexagonal rim is six links");
    for order in scrambles(&rim) {
        let chains = walked_chains(&rs, &order, 0.05, band());
        assert_eq!(chains.len(), 1, "the mixed rim is one closed chain");
        assert_pairing_off_the_body(&rs, &order, &chains[0], "mixed line-and-arc rim");
    }

    // TWO closed chains in one request: both rims of one disc. The
    // walk's `used` set and the per-run position remap are shared
    // across them.
    let disc = disc_of_arcs(3, R, 1.0, tol());
    let mut both = circle_arcs_at_z(&disc, 1.0);
    both.extend(circle_arcs_at_z(&disc, 0.0));
    assert_eq!(both.len(), 6, "two three-arc rims");
    for order in scrambles(&both) {
        let chains = walked_chains(&disc, &order, RHO, band());
        assert_eq!(chains.len(), 2, "two closed chains");
        for c in &chains {
            assert_eq!(c.closure, ChainClosure::Closed);
            assert_pairing_off_the_body(&disc, &order, c, "two rims at once");
        }
    }
}

/// Every rotation, the reversal, and two scrambles that are neither.
fn scrambles(xs: &[EdgeKey]) -> Vec<Vec<EdgeKey>> {
    let n = xs.len();
    let mut out: Vec<Vec<EdgeKey>> = (0..n)
        .map(|k| {
            let mut o = xs.to_vec();
            o.rotate_left(k);
            o
        })
        .collect();
    out.push(xs.iter().rev().copied().collect());
    // Evens then odds, and the same read backwards: for n >= 4 neither
    // is a rotation or the reversal of the input.
    let mut interleaved: Vec<EdgeKey> = xs.iter().step_by(2).copied().collect();
    interleaved.extend(xs.iter().skip(1).step_by(2));
    out.push(interleaved.iter().rev().copied().collect());
    out.push(interleaved);
    out
}

/// **The backward arm of the walk.** An open chain seeded from a MIDDLE
/// link grows in both directions, which is the only way to reach
/// `walk_chains`' backward `joints.push((at, next, own))` and the
/// `before`-non-empty remap that renumbers every junction. The unit's
/// open row seeds from an end.
#[test]
fn r2_an_open_chain_seeded_from_its_middle_pairs_correctly() {
    // Five of the mixed rim's six links, seeded from the middle one:
    // the walk grows two links forward and two back.
    let rs = mixed_rim_prism(1.0);
    let rim = edges_at_station(&rs, 1.0);
    let whole = walked_chains(&rs, &rim, 0.05, band());
    let ring: Vec<EdgeKey> = whole[0].links().map(|l| l.edge).collect();
    let open: Vec<EdgeKey> = ring[..5].to_vec();
    let mut seeded_middle = vec![open[2]];
    seeded_middle.extend(open.iter().filter(|&&e| e != open[2]).copied());
    for (what, order) in [
        ("end-seeded", open.clone()),
        ("middle-seeded", seeded_middle),
    ] {
        let chains = walked_chains(&rs, &order, 0.05, band());
        assert_eq!(chains.len(), 1, "{what}: one open chain");
        assert!(
            matches!(chains[0].closure, ChainClosure::Open { .. }),
            "{what}: open"
        );
        assert_eq!(chains[0].link_count(), 5, "{what}: five links");
        assert_pairing_off_the_body(&rs, &order, &chains[0], what);
        assert_eq!(
            chains[0].links().next().unwrap().edge,
            open[0],
            "{what}: the chain heads at its own end link whatever the seed"
        );
    }

    // And the cube's three-edge chain, seeded from its middle link.
    let body = cube(1.0, tol());
    let top = edges_at_station(&body, 1.0);
    let three = walked_chains(&body, &top[..3], RHO, band());
    let ring: Vec<EdgeKey> = three[0].links().map(|l| l.edge).collect();
    let mid_first = vec![ring[1], ring[0], ring[2]];
    let chains = walked_chains(&body, &mid_first, RHO, band());
    assert_eq!(chains.len(), 1, "one open chain from the middle seed");
    assert_eq!(
        chains[0].links().next().unwrap().edge,
        ring[0],
        "a middle seed still heads at the chain's own first link"
    );
    assert_pairing_off_the_body(&body, &mid_first, &chains[0], "cube chain, middle-seeded");
}

/// **The carve past the suite's N.** A six-arc disc and five-arc bore,
/// boss and pocket — one N past every fixture the unit rows — carve
/// tier-3 valid at the closed form, graded against `test_support`'s
/// HOMED Pappus oracle `wedge_fill` rather than against a suite-local
/// closed form (the unit suite's own copy was retired for it).
#[test]
fn r2_rims_one_n_past_the_suite_carve_at_the_homed_oracle() {
    // The meridian half-plane corner and the two generators leaving it:
    // the cap plane and the cylinder wall, a right angle either way.
    let inward = wedge_fill((R, 0.0), (-1.0, 0.0), (0.0, -1.0), RHO);
    let outward = wedge_fill((R, 0.0), (1.0, 0.0), (0.0, -1.0), RHO);
    let cases: Vec<(String, Body<f64>, f64, f64)> = vec![
        (
            "6-arc disc".into(),
            disc_of_arcs(6, R, 1.0, tol()),
            1.0,
            -inward,
        ),
        (
            "5-arc bore".into(),
            bored_block_of_arcs(5, L, 1.0, R, tol()),
            1.0,
            -outward,
        ),
        (
            "5-arc boss".into(),
            boss_of_arcs(5, L, R, 1.0, 2.0, tol()),
            2.0,
            outward,
        ),
        (
            "5-arc pocket".into(),
            pocket_of_arcs(5, L, R, 1.5, tol()),
            1.5,
            inward,
        ),
    ];
    for (name, body, z, signed) in &cases {
        let arcs = circle_arcs_at_z(body, *z);
        let n = arcs.len();
        let p0 = mass_properties(body, tol()).expect("props");
        assert_eq!(p0.volume_pad, 0.0, "{name}: closed-form faces only");
        let v0 = p0.volume;
        let c0 = (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count(),
        );
        let out = fillet_edges(body, &arcs, RHO, tol())
            .unwrap_or_else(|e| panic!("{name}: the whole rim carves, got {e}"));
        assert_eq!(out.band_faces.len(), 1, "{name}: one band");
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{name}: tier-3 valid, got {e:?}"));
        let c1 = (
            out.body.vertices().count(),
            out.body.edges().count(),
            out.body.faces().count(),
        );
        assert_eq!(
            (c1.0 - c0.0, c1.1 - c0.1, c1.2 - c0.2),
            (n, n + 1, 1),
            "{name}: one vertex and one edge per crossing, one more edge, one band face"
        );
        let p1 = mass_properties(&out.body, tol()).expect("props");
        assert_eq!(p1.volume_pad, 0.0, "{name}: closed-form faces only after");
        assert!(
            (p1.volume - v0 - signed).abs() < 1e-13,
            "{name}: V1 - V0 = {} against wedge_fill's {signed}",
            p1.volume - v0
        );
    }
}
