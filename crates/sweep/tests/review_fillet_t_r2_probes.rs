//! **FILLET-T review probes (lane r2, PR 1943).**
//!
//! `corner_arcs_are_minted_in_seeded_edge_order` — an always-on fence
//! on what `D325` actually changed. `CornerLinks::sorted` returns
//! `(seed, rest)`, and the corner fusion mints the SEED's arc first and
//! the rest after; the birth record's `arcs` rows are that mint order,
//! seen from outside the crate. The row goes red if the seed ever stops
//! being the lowest-keyed incident link or if `rest` stops being
//! ordered — the two properties the three call sites read and that
//! nothing else asserts. It passes at the merge base too (the `Vec` was
//! ascending there), so it is a fence, not a differential.
//!
//! This lane's other row, `bitdump_ruled_band`, has moved into
//! `bitdump.rs` beside the corpus it belongs to; it lived here only
//! because a review lane may not edit the suite it is differentiating.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::test_support::cube;
use topo::{EdgeKey, VertexKey, query};

fn tol() -> Tol {
    Tol::witness()
}

/// **The corner fusion mints one arc per incident link, seed first, and
/// the seed is the lowest-keyed link.** Read from outside the crate
/// through the birth record: `naming.arcs` rows are pushed in mint
/// order, each naming the source link edge its arc belongs to, so per
/// corner vertex those edges must come out strictly ascending.
#[test]
fn corner_arcs_are_minted_in_seeded_edge_order() {
    let body = cube(1.0, tol());
    let out = fillet_edges(&body, &query::all_edges(&body), 0.15, tol()).unwrap();
    let rec = out.naming.as_ref().expect("the surgery records its births");
    let mut per_corner: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
    for (_, vertex, source_edge) in &rec.arcs {
        per_corner.entry(*vertex).or_default().push(*source_edge);
    }
    assert_eq!(
        per_corner.len(),
        8,
        "the die's eight corners each fuse from their own incidence list"
    );
    for (vertex, edges) in &per_corner {
        assert_eq!(
            edges.len(),
            3,
            "a trivalent corner mints one arc per incident link at {vertex:?}"
        );
        let mut ascending = edges.clone();
        ascending.sort_unstable();
        assert_eq!(
            edges, &ascending,
            "the arcs at {vertex:?} are minted in ascending source-edge order — the seed \
             (lowest key) first, then the rest"
        );
        assert_eq!(
            edges
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            edges.len(),
            "no incident link is walked twice at {vertex:?}"
        );
    }
}
