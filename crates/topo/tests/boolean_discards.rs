//! `BooleanNaming::discards`: every face a boolean discards, with the
//! stretches along which it bordered a kept face — the record the naming
//! layer reads to tell a split face's pieces apart by the walls between
//! them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::common;

use common::brick;
use geom_core::Tol;
use topo::{Body, BooleanBody, BooleanResult, EdgeKey, FaceKey, VertexKey, subtract};

/// A settled stretch: the result edge and the faces on its two sides.
pub(crate) type Stretch = Option<(EdgeKey, [FaceKey; 2])>;

/// The result edge each discard's bordered stretch settles onto, and the
/// kept faces on either side of it; `None` for a stretch whose ends
/// fused or that no live edge joins.
pub(crate) fn bordered_edges(out: &BooleanBody<f64>) -> Vec<Vec<Stretch>> {
    let body: &Body<f64> = &out.body;
    let fused = out
        .naming
        .fused_into()
        .expect("the zip's fusions form no cycle");
    let settle = |v: VertexKey| fused.get(&v).copied().unwrap_or(v);
    let face_of = |he| body.face_of_half_edge(he).unwrap();
    let mut by_ends: BTreeMap<(VertexKey, VertexKey), (EdgeKey, [FaceKey; 2])> = BTreeMap::new();
    for (k, e) in body.edges() {
        let s = body.get_half_edge(e.he_plus).unwrap().start;
        let t = body.half_edge_end(e.he_plus).unwrap();
        by_ends.insert(
            (s.min(t), s.max(t)),
            (k, [face_of(e.he_plus), face_of(e.he_minus)]),
        );
    }
    out.naming
        .discards
        .iter()
        .map(|d| {
            d.bordered
                .iter()
                .map(|&(u, w)| {
                    let (u, w) = (settle(u), settle(w));
                    by_ends.get(&(u.min(w), u.max(w))).copied()
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
    let plate = brick::<f64>((0.0, 3.0), (0.0, 2.0), (0.0, 1.0), tol);
    let slot = brick::<f64>((1.4, 1.6), (-1.0, 3.0), (-1.0, 3.0), tol);
    let BooleanResult::Body(out) = subtract(&plate, &slot, tol).expect("the slot subtracts") else {
        panic!("a slotted plate is not empty");
    };
    let rows: BTreeMap<FaceKey, FaceKey> = out.naming.face_fragments_a.iter().copied().collect();
    let root = |mut f: FaceKey| {
        while let Some(&p) = rows.get(&f).filter(|p| **p != f) {
            f = p;
        }
        f
    };
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
            let (e, _) =
                s.expect("every stretch the slot's discards border settles on a live edge");
            assert!(seams.contains(&e), "a bordered stretch is a seam edge");
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
