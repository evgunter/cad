//! Review probes for the closed-chain junction record (R1 lane).
//!
//! The junction record is checked against the BODY's own edge
//! endpoints rather than against the link's `start`/`end` fields, on
//! every closed rim the unit's fixtures mint and on chain shapes the
//! unit's suite does not build: five- and six-arc rims on every door
//! and material side, closed rims whose links are lines AND arcs (a
//! stadium and a rounded square), two closed rims in one request, and
//! request orders that are neither a rotation nor a reversal.
//!
//! The fixtures' material sides are read off the body: each body's
//! volume is pinned at its closed form and every link's convexity
//! verdict is read off the walk.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::approx::band;
use geom_core::{Point2, Tol};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::battery::{Chain, ChainClosure, Convexity};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, BlendRefusal};
use sweep::test_support::{
    bored_block_of_arcs, boss_of_arcs, circle_arcs_at_z, disc_of_arcs, pocket_of_arcs,
    walked_chains, wedge_fill,
};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, VertexKey, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.5;
const RHO: f64 = 0.1;
const L: f64 = 2.0;

/// The plane–cylinder corner torus at wall radius `big_r`, from the
/// HOMED Pappus oracle: the meridian corner `(big_r, 0)` and the two
/// generators leaving it — the cap plane, toward the axis (`inward`,
/// a disc's rim or a pocket's floor) or away from it, and the wall
/// downward — a right angle either way.
fn corner_fill(big_r: f64, rho: f64, inward: bool) -> f64 {
    let along_cap = if inward { (-1.0, 0.0) } else { (1.0, 0.0) };
    wedge_fill((big_r, 0.0), along_cap, (0.0, -1.0), rho)
}

/// An edge's two end vertices, read off the BODY (`he_plus` start and
/// end), not off any link record.
fn body_ends(body: &Body<f64>, e: EdgeKey) -> (VertexKey, VertexKey) {
    let ed = body.get_edge(e).unwrap();
    let s = body.get_half_edge(ed.he_plus).unwrap().start;
    let t = body.half_edge_end(ed.he_plus).unwrap();
    (s, t)
}

/// The vertices of the body at which exactly two REQUESTED edges meet
/// — the junctions the walk owes, computed from the body alone.
fn expected_junctions(body: &Body<f64>, requested: &[EdgeKey]) -> Vec<(VertexKey, Vec<EdgeKey>)> {
    let mut at: Vec<(VertexKey, Vec<EdgeKey>)> = Vec::new();
    for &e in requested {
        let (s, t) = body_ends(body, e);
        for v in if s == t { vec![s] } else { vec![s, t] } {
            match at.iter_mut().find(|(k, _)| *k == v) {
                Some((_, es)) => es.push(e),
                None => at.push((v, vec![e])),
            }
        }
    }
    at.into_iter().filter(|(_, es)| es.len() == 2).collect()
}

/// Every junction of every chain names, as `arriving` and `leaving`,
/// exactly the two requested edges the BODY says meet at its vertex;
/// and every such vertex is a junction of exactly one chain.
fn assert_pairing_is_the_bodys(
    body: &Body<f64>,
    requested: &[EdgeKey],
    chains: &[Chain<f64>],
    what: &str,
) {
    let expected = expected_junctions(body, requested);
    let mut seen = 0;
    for chain in chains {
        let links: Vec<_> = chain.links().collect();
        for j in &chain.junctions {
            let (ea, el) = (links[j.arriving()].edge, links[j.leaving()].edge);
            for (role, e) in [("arriving", ea), ("leaving", el)] {
                let (s, t) = body_ends(body, e);
                assert!(
                    s == j.vertex || t == j.vertex,
                    "{what}: the {role} edge {e:?} of junction {:?} does not touch it on the body ({s:?}→{t:?})",
                    j.vertex
                );
            }
            assert_ne!(ea, el, "{what}: two distinct links at a junction");
            let (_, want) = expected
                .iter()
                .find(|(v, _)| *v == j.vertex)
                .unwrap_or_else(|| {
                    panic!(
                        "{what}: junction {:?} is not a two-edge vertex of the request",
                        j.vertex
                    )
                });
            assert!(
                want.contains(&ea) && want.contains(&el),
                "{what}: junction {:?} pairs {ea:?},{el:?}; the body has {want:?} there",
                j.vertex
            );
            seen += 1;
        }
    }
    assert_eq!(
        seen,
        expected.len(),
        "{what}: one junction per two-edge vertex"
    );
}

/// The unit's closed fixtures, N = 2…6 on the disc and 2…5 elsewhere,
/// each with its rim station and its expected convexity and volume.
fn fixtures() -> Vec<(String, Body<f64>, f64, Convexity, f64)> {
    let mut v = Vec::new();
    for n in 2..=6 {
        v.push((
            format!("{n}-arc disc"),
            disc_of_arcs(n, R, 1.0, tol()),
            1.0,
            Convexity::Convex,
            PI * R * R,
        ));
    }
    for n in 2..=5 {
        v.push((
            format!("{n}-arc bore"),
            bored_block_of_arcs(n, L, 1.0, R, tol()),
            1.0,
            Convexity::Convex,
            L * L - PI * R * R,
        ));
        v.push((
            format!("{n}-arc boss"),
            boss_of_arcs(n, L, R, 1.0, 2.0, tol()),
            L,
            Convexity::Concave,
            L * L * L + PI * R * R * (1.0 + 2.0 - L),
        ));
        v.push((
            format!("{n}-arc pocket"),
            pocket_of_arcs(n, L, R, 1.5, tol()),
            1.5,
            Convexity::Concave,
            L * L * L - PI * R * R * (L - 1.5),
        ));
    }
    v
}

/// Request orders the suite does not use: every rotation, the
/// reversal, and an interleave (`0, 2, 4, …, 1, 3, …`) so the seed's
/// neighbours are not its request neighbours.
fn orders(arcs: &[EdgeKey]) -> Vec<Vec<EdgeKey>> {
    let n = arcs.len();
    let mut out: Vec<Vec<EdgeKey>> = (0..n)
        .map(|k| {
            let mut o = arcs.to_vec();
            o.rotate_left(k);
            o
        })
        .collect();
    out.push(arcs.iter().rev().copied().collect());
    let mut inter: Vec<EdgeKey> = arcs.iter().step_by(2).copied().collect();
    inter.extend(arcs.iter().skip(1).step_by(2).copied());
    out.push(inter);
    out
}

/// **The pairing is the body's incidence**, read independently of the
/// link record, over every fixture at N up to 6 and every request
/// order including the interleave.
#[test]
fn r1_every_junction_pairs_exactly_the_two_requested_edges_the_body_has_at_its_vertex() {
    for (name, body, z, _, _) in fixtures() {
        let arcs = circle_arcs_at_z(&body, z);
        for order in orders(&arcs) {
            let chains = walked_chains(&body, &order, RHO, band());
            assert_eq!(chains.len(), 1, "{name}: one chain");
            assert_eq!(chains[0].closure, ChainClosure::Closed, "{name}");
            assert_pairing_is_the_bodys(&body, &order, &chains, &name);
        }
    }
}

/// **Material side and volume off the body**: each fixture's volume is
/// its closed form (so the boolean built what the name says) and every
/// link's convexity verdict is the side the fixture claims.
#[test]
fn r1_fixture_volumes_and_convexities_are_the_bodys() {
    for (name, body, z, side, v0) in fixtures() {
        let props = mass_properties(&body, tol()).unwrap();
        assert_eq!(props.volume_pad, 0.0, "{name}");
        assert!(
            (props.volume - v0).abs() < 1e-12,
            "{name}: V₀ = {} against {v0}",
            props.volume
        );
        let arcs = circle_arcs_at_z(&body, z);
        let chains = walked_chains(&body, &arcs, RHO, band());
        for l in chains[0].links() {
            assert_eq!(l.convexity, side, "{name}: {:?}", l.edge);
        }
    }
}

/// One carve, checked: one band, tier-3 valid, `V₁ − V₀` at the
/// closed form.
fn carve(body: &Body<f64>, arcs: &[EdgeKey], signed: f64, what: &str) {
    let v0 = mass_properties(body, tol()).unwrap().volume;
    let out =
        fillet_edges(body, arcs, RHO, tol()).unwrap_or_else(|e| panic!("{what}: carves, got {e}"));
    assert_eq!(out.band_faces.len(), 1, "{what}: one band");
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
    let p1 = mass_properties(&out.body, tol()).unwrap();
    assert_eq!(p1.volume_pad, 0.0, "{what}");
    let moved = p1.volume - v0;
    assert!(
        (moved - signed).abs() < 1e-13,
        "{what}: V₁ − V₀ = {moved} against {signed}"
    );
}

/// **Five- and six-arc rims carve on every door and side** — the
/// suite carves N ≤ 4 on the ladder and the concave annulus.
#[test]
fn r1_five_and_six_arc_rims_carve_on_both_doors_and_both_sides() {
    for n in [5, 6] {
        let disc = disc_of_arcs(n, R, 1.0, tol());
        carve(
            &disc,
            &circle_arcs_at_z(&disc, 1.0),
            -corner_fill(R, RHO, true),
            &format!("{n}-arc disc"),
        );
        let bore = bored_block_of_arcs(n, L, 1.0, R, tol());
        carve(
            &bore,
            &circle_arcs_at_z(&bore, 1.0),
            -corner_fill(R, RHO, false),
            &format!("{n}-arc bore"),
        );
        let boss = boss_of_arcs(n, L, R, 1.0, 2.0, tol());
        carve(
            &boss,
            &circle_arcs_at_z(&boss, L),
            corner_fill(R, RHO, false),
            &format!("{n}-arc boss"),
        );
        let pocket = pocket_of_arcs(n, L, R, 1.5, tol());
        carve(
            &pocket,
            &circle_arcs_at_z(&pocket, 1.5),
            corner_fill(R, RHO, true),
            &format!("{n}-arc pocket"),
        );
    }
}

/// **Two closed rims in one request** — both rims of a three-arc
/// cylinder: two closed chains, each paired by the body's incidence,
/// and the carve removes two corner tori.
#[test]
fn r1_both_rims_of_a_three_arc_cylinder_walk_into_two_closed_chains_and_carve() {
    let body = disc_of_arcs(3, R, 1.0, tol());
    let mut all = circle_arcs_at_z(&body, 1.0);
    all.extend(circle_arcs_at_z(&body, 0.0));
    assert_eq!(all.len(), 6);
    // Interleave the two rims' arcs so neither chain is contiguous in
    // the request.
    let inter: Vec<EdgeKey> = [0, 3, 1, 4, 2, 5].iter().map(|&i| all[i]).collect();
    let chains = walked_chains(&body, &inter, RHO, band());
    assert_eq!(chains.len(), 2, "two closed chains");
    for c in &chains {
        assert_eq!(c.closure, ChainClosure::Closed);
        assert_eq!(c.junctions.len(), 3);
    }
    assert_pairing_is_the_bodys(&body, &inter, &chains, "both rims");
    let v0 = mass_properties(&body, tol()).unwrap().volume;
    let out = fillet_edges(&body, &inter, RHO, tol())
        .unwrap_or_else(|e| panic!("both rims carve, got {e}"));
    assert_eq!(out.band_faces.len(), 2);
    validate_geometric(&out.body, tol()).unwrap();
    let v1 = mass_properties(&out.body, tol()).unwrap().volume;
    assert!(
        (v1 - v0 + 2.0 * corner_fill(R, RHO, true)).abs() < 1e-13,
        "{}",
        v1 - v0
    );
}

/// The top-rim edges of an extruded body: both ends at station `h`.
fn top_rim(body: &Body<f64>, h: f64) -> Vec<EdgeKey> {
    let z_of = |v: VertexKey| body.get_point(body.get_vertex(v).unwrap().point).unwrap().z;
    body.edges()
        .map(|(k, _)| k)
        .filter(|&k| {
            let (s, t) = body_ends(body, k);
            (z_of(s) - h).abs() < 1e-12 && (z_of(t) - h).abs() < 1e-12
        })
        .collect()
}

/// Every joint of `vs` is tangent (line into arc into line), declared
/// as the profile door requires.
fn extruded(vs: Vec<(Point2<f64>, f64)>, h: f64) -> Body<f64> {
    let joints: Vec<usize> = (0..vs.len()).collect();
    let pf = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vs).with_tangent_joints(joints)],
    )
    .validate(tol())
    .unwrap();
    extrude(&pf, Extrusion::Distance(h), tol()).unwrap().body
}

/// A stadium: two lines and two semicircles, CCW.
fn stadium(a: f64, r: f64) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    extruded(
        vec![v(-a, -r, 0.0), v(a, -r, 1.0), v(a, r, 0.0), v(-a, r, 1.0)],
        1.0,
    )
}

/// A rounded square of half-side `a` and corner radius `c`, CCW: four
/// lines and four quarter arcs.
fn rounded_square(a: f64, c: f64) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    let q = (PI / 8.0).tan();
    extruded(
        vec![
            v(-(a - c), -a, 0.0),
            v(a - c, -a, q),
            v(a, -(a - c), 0.0),
            v(a, a - c, q),
            v(a - c, a, 0.0),
            v(-(a - c), a, q),
            v(-a, a - c, 0.0),
            v(-a, -(a - c), q),
        ],
        1.0,
    )
}

/// **Closed rims whose links are lines AND arcs** — the stadium's
/// four-link rim and the rounded square's eight-link rim — pair by
/// the body's incidence in every request order; and what the public
/// door says about them is recorded.
#[test]
fn r1_mixed_line_and_arc_closed_rims_pair_by_the_bodys_incidence() {
    for (name, body, n) in [
        ("stadium", stadium(0.5, 0.5), 4),
        ("rounded square", rounded_square(1.0, 0.3), 8),
    ] {
        let rim = top_rim(&body, 1.0);
        assert_eq!(rim.len(), n, "{name}: the top rim's links");
        for order in orders(&rim) {
            let chains = walked_chains(&body, &order, RHO, band());
            assert_eq!(chains.len(), 1, "{name}: one chain");
            assert_eq!(chains[0].closure, ChainClosure::Closed, "{name}");
            assert_eq!(chains[0].junctions.len(), n, "{name}");
            assert_pairing_is_the_bodys(&body, &order, &chains, name);
        }
        // The door refuses the mixed rim as a chain it has no band for
        // (measured: `UnsupportedChain`, "a closed chain's blend is not
        // a torus") — and NOT as `ChainNotG1`: the pairing judged every
        // line-into-arc junction between the two links that meet there.
        match fillet_edges(&body, &rim, RHO, tol()) {
            Err(BlendRefusal {
                error: BlendError::UnsupportedChain { .. },
                ..
            }) => {}
            other => panic!(
                "{name}: the mixed rim refuses at the door, not at a junction; got {other:?}"
            ),
        }
    }
}

/// **`rim_of` refuses every extruded multi-arc rim** (the filed
/// finding, reproduced): the two- and three-arc discs' rim arcs are
/// stored on carriers whose centre or radius differ in bits, so the
/// bit-exact door answers `NotOneRim` from every seed — and
/// `circle_arcs_at_z` is what the unit's fixtures have to select by.
#[test]
fn r1_rim_of_refuses_the_extruded_two_and_three_arc_rims_on_carrier_bits() {
    for n in [2, 3] {
        let body = disc_of_arcs(n, R, 1.0, tol());
        let arcs = circle_arcs_at_z(&body, 1.0);
        assert_eq!(arcs.len(), n);
        let carrier = |e: EdgeKey| {
            let ed = body.get_edge(e).unwrap();
            match body
                .get_curve_geom(ed.curve)
                .unwrap()
                .certified()
                .unwrap()
                .carrier()
            {
                geom::Curve3::Circle {
                    center,
                    radius,
                    axis,
                    ..
                } => (*center, *radius, *axis),
                other => panic!("an arc, got {other:?}"),
            }
        };
        let (c0, r0, a0) = carrier(arcs[0]);
        let mut all_same_bits = true;
        for &e in &arcs[1..] {
            let (c, r, a) = carrier(e);
            let same = c.x.to_bits() == c0.x.to_bits()
                && c.y.to_bits() == c0.y.to_bits()
                && c.z.to_bits() == c0.z.to_bits()
                && r.to_bits() == r0.to_bits()
                && a.x.to_bits() == a0.x.to_bits()
                && a.y.to_bits() == a0.y.to_bits()
                && a.z.to_bits() == a0.z.to_bits();
            all_same_bits &= same;
        }
        assert!(
            !all_same_bits,
            "{n}-arc disc: the arcs' stored circles differ in bits"
        );
        for &e in &arcs {
            match topo::query::rim_of(&body, e) {
                Err(topo::query::RimError::NotOneRim { .. }) => {}
                other => panic!("{n}-arc disc: rim_of from {e:?} refuses NotOneRim, got {other:?}"),
            }
        }
    }
}

/// The rim's links in cycle order off the body, from `rim[0]`.
fn in_cycle_order(body: &Body<f64>, rim: &[EdgeKey]) -> Vec<EdgeKey> {
    let mut ordered = vec![rim[0]];
    let mut at = body_ends(body, rim[0]).1;
    while ordered.len() < rim.len() {
        let next = rim
            .iter()
            .copied()
            .find(|&e| {
                !ordered.contains(&e) && {
                    let (s, t) = body_ends(body, e);
                    s == at || t == at
                }
            })
            .unwrap();
        let (s, t) = body_ends(body, next);
        at = if s == at { t } else { s };
        ordered.push(next);
    }
    ordered
}

/// **An OPEN chain with G1 junctions** — the rounded square's top rim
/// minus one arc (seven links, six tangent junctions) and the
/// stadium's minus one line (three links, two) — walks into one open
/// chain paired by the body's incidence, and the public door refuses
/// it at a run-out END, not at any junction.
#[test]
fn r1_an_open_chain_with_tangent_junctions_pairs_by_incidence_and_refuses_at_a_run_out() {
    for (name, body) in [
        ("rounded square", rounded_square(1.0, 0.3)),
        ("stadium", stadium(0.5, 0.5)),
    ] {
        let rim = top_rim(&body, 1.0);
        let open: Vec<EdgeKey> = in_cycle_order(&body, &rim)[1..].to_vec();
        let chains = walked_chains(&body, &open, RHO, band());
        assert_eq!(chains.len(), 1, "{name}: one open chain");
        assert!(
            matches!(chains[0].closure, ChainClosure::Open { .. }),
            "{name}"
        );
        assert_eq!(chains[0].junctions.len(), open.len() - 1, "{name}");
        assert_pairing_is_the_bodys(&body, &open, &chains, name);
        // Measured: the door refuses at an END of the chain as a corner
        // it cannot classify (`UnsupportedCorner`, run-out policy named)
        // — never at a junction — so no open chain with a junction
        // carves today, which is the PR's premise for narrowing the
        // open-chain acceptance to a pairing pin.
        let ends = match chains[0].closure {
            ChainClosure::Open { head, tail } => [head, tail],
            ChainClosure::Closed => unreachable!(),
        };
        match fillet_edges(&body, &open, RHO, tol()) {
            Err(BlendRefusal {
                error: BlendError::UnsupportedCorner { vertex, .. },
                ..
            }) => assert!(
                ends.contains(&vertex),
                "{name}: refused at an end, not a junction"
            ),
            other => panic!("{name}: the open chain refuses at its run-out, got {other:?}"),
        }
    }
}
