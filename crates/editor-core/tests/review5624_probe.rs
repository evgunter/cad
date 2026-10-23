//! Reviewer probes for PR 5624 (not for merge as-is).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::fixture::{insert, len};
use editor_core::{
    Evaluation, LoopProgram, NamedCell, NamedCoincidence, Node, ProfileDoc, ProfileProgram,
    Proof, RecipeNodeId, Rung, SplitHalf, VarId, coincide,
};
use geom_core::Tol;
use topo::Relation;

fn rows(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<NamedCoincidence> {
    ev.value(node)
        .map(|v| v.coincidences.to_vec())
        .unwrap_or_default()
}

fn proofs(doc: &ProfileDoc, node: RecipeNodeId, rows: &[NamedCoincidence]) -> Vec<Proof> {
    coincide::prove(
        doc,
        &[(node, rows)],
        None,
        Tol::witness(),
        &editor_core::CancelToken::new(),
    )
    .expect("an uncanceled door")
    .pop()
    .expect("one node asked")
}

fn port(doc: &ProfileDoc, split: RecipeNodeId, half: SplitHalf) -> VarId {
    doc.output(split, half.port()).expect("a split's port")
}

fn a_crossing(cell: &NamedCell) -> bool {
    matches!(cell, NamedCell::Entity { name, .. }
        if format!("{name:?}").contains("CrossingVertex"))
}

/// Per-row census of the door over the corpus, for a base/head diff.
#[test]
#[ignore = "probe"]
fn probe_door_rows_over_the_corpus() {
    for cd in crate::corpus::documents() {
        let ev = run(&cd.doc);
        let nodes: Vec<(RecipeNodeId, &[NamedCoincidence])> = ev
            .order
            .iter()
            .filter_map(|&node| ev.value(node).map(|v| (node, &*v.coincidences)))
            .filter(|(_, rows)| !rows.is_empty())
            .collect();
        let proofs = coincide::prove(
            &cd.doc,
            &nodes,
            None,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("an uncanceled door");
        for ((node, rows), proofs) in nodes.iter().zip(&proofs) {
            for (i, (row, proof)) in rows.iter().zip(proofs).enumerate() {
                let how = match proof {
                    Proof::Structural(rung) => format!("{rung:?}"),
                    Proof::Unproven { residual, recourse } => {
                        format!("unproven/{:?}/{recourse:?}", residual.replay)
                    }
                };
                eprintln!(
                    "ROW {} {node:?} {i} {:?} {:?} {how}",
                    cd.name, row.relation, row.site
                );
            }
        }
    }
}

/// The upper half of a split block, placed (by literal values) before
/// it is unioned with the slab whose top is the tool: the crossing's
/// construction carries the placement's map, the slab's top does not,
/// so rung 1 must not lay the crossing on the top by construction.
#[test]
fn probe_a_placed_half_is_not_on_the_unplaced_face() {
    let doc = ProfileDoc::empty_derived("probe-placed-half", Tol::witness());
    let (doc, slab) = block(doc, (-1.0, 2.0), (-1.0, 2.0), 0.5);
    let (doc, x) = block_on(doc, slab, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let tool = crate::fixture::plane_on(&doc, slab, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(doc, Node::Split { target: x.into(), tool });
    let upper = port(&doc, split, SplitHalf::Above);
    let (doc, moved) = crate::fixture::against(
        doc,
        slab,
        upper,
        &editor_core::Frame::translation([0.25, 0.0, 0.0]),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![moved.into(), slab.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let proven = proofs(&doc, union, &got);
    let mut n = 0;
    for (row, proof) in got.iter().zip(&proven) {
        eprintln!("PLACED {:?} {:?} {proof:?}", row.relation, row.cells);
        if row.relation == Relation::OnCarrier && row.cells.iter().any(a_crossing) {
            n += 1;
            assert_ne!(
                *proof,
                Proof::Structural(Rung::SameConstruction),
                "a placed crossing read on the unplaced top by construction: {row:?}"
            );
        }
    }
    assert!(n > 0, "the premise: crossing rows exist: {got:?}");
}

/// An extruded disc split by a plane off its axis along y, so each cap
/// circle's upper arc is crossed twice; its halves reunited. Every row
/// should prove at rung 1 if ranks and senses align per crossing.
#[test]
fn probe_an_arc_crossed_twice_reunites() {
    let doc = ProfileDoc::empty_derived("probe-arc-twice", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).unwrap()],
            ids: Vec::new(),
        }),
    );
    let (doc, cyl) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, wall) = block_on(doc, cyl, (-1.0, 1.0), (-1.0, 0.2), 0.0, 1.0);
    let tool = crate::fixture::plane_on(&doc, wall, [0.0, 0.2, 0.0], [0.0, 1.0, 0.0]);
    let (doc, split) = insert(doc, Node::Split { target: cyl.into(), tool });
    let (a, b) = (
        port(&doc, split, SplitHalf::Above),
        port(&doc, split, SplitHalf::Below),
    );
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let got = rows(&ev, union);
    let proven = proofs(&doc, union, &got);
    let mut crossings = 0;
    let mut bad = Vec::new();
    for (row, proof) in got.iter().zip(&proven) {
        eprintln!("ARC {:?} {:?} {proof:?}", row.relation, row.cells);
        if row.cells.iter().any(a_crossing) {
            crossings += 1;
        }
        if *proof != Proof::Structural(Rung::SameConstruction) {
            bad.push((row.clone(), proof.clone()));
        }
    }
    eprintln!("ARC crossings rows {crossings}, not rung-1 {}", bad.len());
    assert!(crossings >= 4, "premise: {got:?}");
    assert!(bad.iter().all(|(_, p)| matches!(p, Proof::Structural(_))), "{bad:?}");
}
