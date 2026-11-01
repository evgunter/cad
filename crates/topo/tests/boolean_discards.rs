//! `BooleanNaming::discards`: every face a boolean discards, with the
//! stretches along which it bordered a kept face — the record the naming
//! layer reads to tell a split face's pieces apart by the walls between
//! them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{Body, BooleanBody, BooleanResult, Cell, EdgeKey, FaceKey, VertexKey, subtract};

/// A settled stretch: each result edge it lies along, with the faces on
/// the edge's two sides; empty for a stretch whose ends fused, or lie
/// inside a merged face, or that no live edge joins. An end the zip split
/// per cone lies at each of its copies (`Settler::settle_cones`), and the
/// stretch is read between every pair of copies, as `borders.rs` reads
/// it: each edge found lies along the stretch's locus, so the discard's
/// region meets it.
pub(crate) type Stretch = Vec<(EdgeKey, [FaceKey; 2])>;

/// The result edges each discard's bordered stretch settles onto, and
/// the kept faces on either side of each.
pub(crate) fn bordered_edges(out: &BooleanBody<f64>) -> Vec<Vec<Stretch>> {
    let body: &Body<f64> = &out.body;
    let settler = out.naming.settler(body);
    let settle = |v: VertexKey| {
        settler
            .settle_cones(v)
            .expect("every end settles on a live cell")
    };
    let face_of = |he| body.face_of_half_edge(he).unwrap();
    let mut by_ends: BTreeMap<(VertexKey, VertexKey), Stretch> = BTreeMap::new();
    for (k, e) in body.edges() {
        let s = body.get_half_edge(e.he_plus).unwrap().start;
        let t = body.half_edge_end(e.he_plus).unwrap();
        by_ends
            .entry((s.min(t), s.max(t)))
            .or_default()
            .push((k, [face_of(e.he_plus), face_of(e.he_minus)]));
    }
    out.naming
        .discards
        .iter()
        .map(|d| {
            d.bordered
                .iter()
                .chain(&d.bordered_through)
                .map(|&(u, w)| {
                    let (us, ws) = (settle(u), settle(w));
                    let mut found = Vec::new();
                    for (&u, &w) in us.iter().flat_map(|u| ws.iter().map(move |w| (u, w))) {
                        let between = match (u, w) {
                            (Cell::Vertex(u), Cell::Vertex(w)) => {
                                by_ends.get(&(u.min(w), u.max(w)))
                            }
                            // An end inside a joined edge or a merged
                            // face ends no edge.
                            _ => None,
                        };
                        if let Some(es) = between {
                            found.extend(es.iter().copied());
                        } else if let Some(e) = out.naming.stretch_through_joins(body, (u, w))
                            && let Some(d) = body.get_edge(e)
                        {
                            found.push((e, [face_of(d.he_plus), face_of(d.he_minus)]));
                        }
                    }
                    found
                })
                .collect()
        })
        .collect()
}

/// A slot through a plate cuts its top into two pieces. The discarded
/// strip of the top borders both of them, one stretch each, and each
/// stretch settles onto a seam edge of the result.
#[test]
fn a_through_slot_records_the_strip_it_discards_between_two_kept_pieces() {
    let tol = Tol::witness();
    let plate = finished(
        "plate",
        brick::<f64>((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), tol),
        tol,
    );
    let slot = finished(
        "slot",
        brick::<f64>((1.4, 1.6), (-1.0, 3.0), (-1.0, 3.0), tol),
        tol,
    );
    let BooleanResult::Body(out) = subtract(&plate, &slot, tol).expect("the slot subtracts") else {
        panic!("a slotted plate is not empty");
    };
    let root = |f: FaceKey| out.naming.face_fragments_a.root(f);
    // Operand face → the result faces descending from it (A keys are
    // the result's on this path).
    let mut pieces: BTreeMap<FaceKey, BTreeSet<FaceKey>> = BTreeMap::new();
    for (f, _) in out.body.faces() {
        if out.naming.graft_faces.iter().all(|&(_, d)| d != f) {
            pieces.entry(root(f)).or_default().insert(f);
        }
    }
    let seams: BTreeSet<EdgeKey> = out.naming.seam_edges.iter().copied().collect();
    let edges = bordered_edges(&out);
    assert!(
        out.naming
            .discards
            .iter()
            .any(|d| d.operand == topo::Operand::B),
        "the slot's faces outside the plate are discarded too: {:?}",
        out.naming.discards
    );
    let mut dividers = 0;
    for (d, stretches) in out.naming.discards.iter().zip(&edges) {
        if d.operand != topo::Operand::A {
            continue;
        }
        for s in stretches {
            assert!(
                !s.is_empty(),
                "every stretch the slot's discards border settles on a live edge"
            );
            for (e, _) in s {
                assert!(seams.contains(e), "a bordered stretch is a seam edge");
            }
        }
        let held = pieces.get(&root(d.face)).cloned().unwrap_or_default();
        let bordered: BTreeSet<FaceKey> = stretches
            .iter()
            .flatten()
            .flat_map(|(_, fs)| fs.iter().copied())
            .filter(|f| held.contains(f))
            .collect();
        if held.len() == 2 {
            assert_eq!(
                bordered, held,
                "a strip between two pieces of its face borders both of them"
            );
            dividers += 1;
        }
    }
    // The top, the bottom and the two long sides are each cut in two.
    assert_eq!(
        dividers, 4,
        "four plate faces lose a middle strip to the slot"
    );
}

/// **An edge the boolean split and its own join killed keeps its
/// parent** (`BooleanNaming::killed_lineage`): two bricks overlapping
/// along x, flush on their other four sides, union into one brick. Each
/// operand's long edges are cut where the other's end face crosses
/// them, and the join makes each long edge of the result again, killing
/// pieces the cut minted, whose birth records die with them.
#[test]
fn an_edge_split_and_joined_away_in_one_step_records_its_parent() {
    let tol = Tol::witness();
    let a: topo::AtRestBody<f64> =
        finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
    let b: topo::AtRestBody<f64> =
        finished("b", brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol), tol);
    let out = match topo::union(&a, &b, tol) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    };
    let naming = &out.naming;
    let gone: BTreeSet<EdgeKey> = naming.edge_joins.iter().map(|j| j.gone).collect();
    // The join's entries; the merge's own are the next test's.
    let joined: Vec<(&EdgeKey, &EdgeKey)> = naming
        .killed_lineage
        .iter()
        .filter(|(g, _)| gone.contains(g))
        .collect();
    assert!(
        !joined.is_empty(),
        "a piece the cut minted is joined away: {:?}",
        naming.edge_joins
    );
    assert_eq!(out.body.edges().count(), 12, "the union is one brick");
    for (g, from) in joined {
        assert!(out.body.get_edge(*g).is_none(), "{g:?} is dead");
        // The parent, followed through any parent the join also
        // killed, lies on the result edge `g` was joined into: the long
        // edge the cut divided.
        let mut up = *from;
        let mut hops = 0;
        while let Some(&p) = naming.killed_lineage.get(&up) {
            up = p;
            hops += 1;
            assert!(
                hops <= naming.killed_lineage.len(),
                "{g:?}'s lineage cycles"
            );
        }
        assert_eq!(
            naming.joined_edge(up),
            naming.joined_edge(*g),
            "{g:?}'s parent {from:?} lies on the edge {g:?} was joined into"
        );
    }
}

/// **An edge the boolean split and its own merge killed keeps its
/// parent** (`BooleanNaming::killed_lineage`): a pocket is cut into a
/// plate, then a slot whose floor is flush with the pocket's runs out
/// through the pocket's wall. The slot cuts the pocket's bottom rim
/// under that wall; the stretch between the slot's sides lies between
/// the two floors, which the merge makes one face, killing the stretch
/// with its birth record while the stretch past it lives on.
#[test]
fn an_edge_split_and_merged_away_in_one_step_records_its_parent() {
    let tol = Tol::witness();
    let plate: topo::AtRestBody<f64> =
        finished("plate", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol);
    let pocket = finished(
        "pocket",
        brick((1.0, 3.0), (1.0, 3.0), (0.5, 2.0), tol),
        tol,
    );
    let slot = finished("slot", brick((2.0, 5.0), (1.5, 2.5), (0.5, 2.0), tol), tol);
    let pocketed = match subtract(&plate, &pocket, tol) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the pocket cuts: {other:?}"),
    };
    let out = match subtract(&pocketed.body, &slot, tol) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the slot cuts: {other:?}"),
    };
    let naming = &out.naming;
    let gone: BTreeSet<EdgeKey> = naming.edge_joins.iter().map(|j| j.gone).collect();
    let merged: Vec<(EdgeKey, EdgeKey)> = naming
        .killed_lineage
        .iter()
        .filter(|(g, _)| !gone.contains(g))
        .map(|(&g, &f)| (g, f))
        .collect();
    assert!(
        !merged.is_empty(),
        "a rim stretch the slot cut lies between the merged floors: {:?}",
        naming.killed_lineage
    );
    for (g, from) in merged {
        assert!(out.body.get_edge(g).is_none(), "{g:?} is dead");
        // Its parent, followed through any parent the output stage also
        // killed, is a live edge: the rim stretch the merge kept.
        let mut up = from;
        let mut hops = 0;
        while let Some(&p) = naming.killed_lineage.get(&up) {
            up = p;
            hops += 1;
            assert!(
                hops <= naming.killed_lineage.len(),
                "{g:?}'s lineage cycles"
            );
        }
        let live = naming.joined_edge(up);
        let edge = out
            .body
            .get_edge(live)
            .unwrap_or_else(|| panic!("{g:?}'s lineage reaches a live edge, not {up:?}"));
        // It lies on the pocket's bottom rim under the wall the slot runs
        // through: both ends on the line x = 3, z = 0.5.
        let ends = [
            out.body.get_half_edge(edge.he_plus).unwrap().start,
            out.body.half_edge_end(edge.he_plus).unwrap(),
        ];
        let points: BTreeMap<VertexKey, _> = out.body.vertex_points().collect();
        for v in ends {
            let p = points[&v];
            assert!(
                (p.x - 3.0).abs() < 1e-9 && (p.z - 0.5).abs() < 1e-9,
                "{g:?}'s lineage reaches {live:?}, which ends at {p:?}, off the pocket's rim"
            );
        }
    }
}

/// **An edge the boolean's carve kills keeps its parent**
/// (`BooleanNaming::carved_lineage`): a boss stands on a block's top,
/// its `+x` wall flush with the block's. The section splits the block's
/// top `+x` edge where the boss's walls meet it; the piece under the
/// boss lies in the material the union discards, and the carve takes it
/// with its birth record, while a piece split off it lives on. Its
/// lineage reaches the block's own edge only through the copy.
#[test]
fn an_edge_the_carve_kills_records_its_parent() {
    let tol = Tol::witness();
    let mm = 1e-3;
    let block: topo::AtRestBody<f64> = finished(
        "block",
        brick(
            (-20.0 * mm, 20.0 * mm),
            (-10.0 * mm, 10.0 * mm),
            (0.0, 10.0 * mm),
            tol,
        ),
        tol,
    );
    let boss: topo::AtRestBody<f64> = finished(
        "boss",
        brick(
            (10.0 * mm, 20.0 * mm),
            (-5.0 * mm, 5.0 * mm),
            (10.0 * mm, 14.0 * mm),
            tol,
        ),
        tol,
    );
    let out = match topo::union(&block, &boss, tol) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the boss unions: {other:?}"),
    };
    let naming = &out.naming;
    assert_eq!(
        naming.a_keys,
        topo::OperandKeys::Direct,
        "A's clone is the arena"
    );
    let carved = &naming.carved_lineage[0];
    // Each carved edge a live piece was split off.
    let parents: BTreeSet<EdgeKey> = out
        .body
        .edges()
        .filter_map(|(e, _)| match out.body.edge_provenance_of(e) {
            Some(topo::Provenance::SplitEdge { edge }) => Some(*edge),
            _ => None,
        })
        .collect();
    let held: Vec<(EdgeKey, EdgeKey)> = carved
        .iter()
        .filter(|(g, _)| parents.contains(g))
        .map(|(&g, &f)| (g, f))
        .collect();
    assert!(
        !held.is_empty(),
        "a live piece was split off an edge the carve killed: {carved:?}"
    );
    for (g, from) in held {
        assert!(out.body.get_edge(g).is_none(), "{g:?} is dead");
        // Its parent, followed through the result's records and the
        // copy, is an edge of the block.
        let mut up = from;
        let mut hops = 0;
        loop {
            up = match (out.body.edge_provenance_of(up), carved.get(&up)) {
                (Some(topo::Provenance::SplitEdge { edge }), _) => *edge,
                (_, Some(&p)) => p,
                _ => break,
            };
            hops += 1;
            assert!(
                hops <= carved.len() + out.body.edges().count(),
                "{g:?}'s lineage cycles"
            );
        }
        // The block's own edge, read in the block: its top `+x` edge,
        // both ends at x = 20, z = 10, one at each y = ±10 (mm).
        let edge = block
            .get_edge(up)
            .unwrap_or_else(|| panic!("{g:?}'s lineage reaches an edge of the block, not {up:?}"));
        let points: BTreeMap<VertexKey, _> = block.vertex_points().collect();
        let mut ys: Vec<f64> = [
            block.get_half_edge(edge.he_plus).unwrap().start,
            block.half_edge_end(edge.he_plus).unwrap(),
        ]
        .into_iter()
        .map(|v| {
            let p = points[&v];
            assert!(
                (p.x - 20.0 * mm).abs() < 1e-12 && (p.z - 10.0 * mm).abs() < 1e-12,
                "{g:?}'s lineage reaches {up:?}, which ends at {p:?}, off the block's top +x edge"
            );
            p.y
        })
        .collect();
        ys.sort_by(f64::total_cmp);
        assert!(
            (ys[0] + 10.0 * mm).abs() < 1e-12 && (ys[1] - 10.0 * mm).abs() < 1e-12,
            "{g:?}'s lineage reaches {up:?}, which spans y {ys:?}, not the whole top +x edge"
        );
    }
}
