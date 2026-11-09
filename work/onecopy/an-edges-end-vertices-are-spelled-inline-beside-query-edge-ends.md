---
id: an-edges-end-vertices-are-spelled-inline-beside-query-edge-ends
kind: issue
title: About fifty test sites read an edge's end as half_edge_end(e.he_plus) beside Body::edge_vertices
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [6327, 6552]
---

Found by the sweep of `the-curve-pair-units-follow-ups-from-its-review` item 1, which moved `carrier_cross::face_boundary` onto `query::edge_ends`.

## Narrowed (2026-11-06, `onecopy/topo-handles`)

An edge's two end vertices now have one home per lookup tier:
- `Body::edge_vertices` (`crates/topo/src/body.rs`, public): `he_plus`'s start and its end, read as `Body::half_edge_end` reads it (`next`'s start); `None` for a stale key;
- `query::edge_ends` (`crates/topo/src/query.rs`, crate-private): the same reading through `query::half_edge_ends`, each hop a link, so a miss panics (D2 row 4).

The one reading of a half-edge's end is `next`'s start, not the twin's start: tier 1 checks `end(he_plus) = start(he_minus)` against it (`Body::half_edge_end`'s docs).

The inline spellings in topo, the boolean `reduce.rs` and `attach.rs` helpers, mesh's `chords::edge_vertices`, editor-core's `names::emit::edge_ends`, step-export's writer and every `[e.he_plus, e.he_minus].map(..start..)` / `start(e.he_plus) .. start(e.he_minus)` statement outside topo were repointed. The gate `scripts/gates/topology-reads-through-topo.sh` holds those two shapes in every root.

## What remains

The `next` spelling, `body.half_edge_end(e.he_plus)` (usually beside `get_half_edge(e.he_plus).start`), at 54 test sites across 46 files: `crates/sweep/tests` (most, e.g. `wedge_end_doors.rs`, `verbs_arms1_annulus.rs`, `verbs_rim_closed_lever.rs`, `blend6_verb_vocab.rs`, `common/{approx,cavity,certificates,differential}.rs`), `crates/topo/tests` (`boolean_discards.rs`, `geometric_cube.rs`, `fixture/m7_8.rs`, `readback_sense_kind.rs`, ...), `crates/editor-core/tests/fixture/mod.rs`, `crates/mesh/tests/common/mod.rs`, `crates/sweep/examples/near_tangent_census_probe.rs`, `demos/tour/tests/common/rim_select.rs`. Sweep: `grep -rnP "half_edge_end\(\s*[\w.()]*\.he_plus\s*\)" crates demos tools | grep -v ^crates/topo/src`. The plain two-`let` pairs were ported. What is left reads one end inside an expression (`Some(start) == body.half_edge_end(e.he_plus)`, `p(body.half_edge_end(d.he_plus).unwrap())`), and each port needs the edge key in scope.

Inside `crates/topo/src` the same call is mostly an Euler operator's test asserting a half-edge's own end, which is not this class.

The gate cannot hold this shape: by text, `half_edge_end(e.he_plus)` is also a half-edge's own end. Once the sites are ported, deprecating the pair spelling is the remaining lever.
