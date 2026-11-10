---
id: an-edges-end-vertices-are-spelled-inline-beside-query-edge-ends
kind: issue
title: About fifty test sites read an edge's end as half_edge_end(e.he_plus) beside Body::edge_vertices
status: closed
opened: 2026-11-03
priority: P2
cost: M
refs: [6327, 6552]
closed: 2026-11-10
pr: 6710
branch: onecopy/edge-ends-turns
---

Found by the sweep of `the-curve-pair-units-follow-ups-from-its-review` item 1, which moved `carrier_cross::face_boundary` onto `query::edge_ends`.

## Narrowed (2026-11-06, `onecopy/topo-handles`)

An edge's two end vertices now have one home per lookup tier:
- `Body::edge_vertices` (`crates/topo/src/body.rs`, public): `he_plus`'s start and its end, read as `Body::half_edge_end` reads it (`next`'s start); `None` for a stale key;
- `query::edge_ends` (`crates/topo/src/query.rs`, crate-private): the same reading through `query::half_edge_ends`, each hop a link, so a miss panics (D2 row 4).

The one reading of a half-edge's end is `next`'s start, not the twin's start: tier 1 checks `end(he_plus) = start(he_minus)` against it (`Body::half_edge_end`'s docs).

The inline spellings in topo, the boolean `reduce.rs` and `attach.rs` helpers, mesh's `chords::edge_vertices`, editor-core's `names::emit::edge_ends`, step-export's writer and every `[e.he_plus, e.he_minus].map(..start..)` / `start(e.he_plus) .. start(e.he_minus)` statement outside topo were repointed. The gate `scripts/gates/topology-reads-through-topo.sh` holds those two shapes in every root.

## What remains

The `next` spelling, `body.half_edge_end(e.he_plus)` (usually beside `get_half_edge(e.he_plus).start`), at 52 test sites across 44 files: `crates/sweep/tests` (most, e.g. `wedge_end_doors.rs`, `verbs_arms1_annulus.rs`, `verbs_rim_closed_lever.rs`, `blend6_verb_vocab.rs`, `common/{approx,cavity,certificates,differential}.rs`), `crates/topo/tests` (`boolean_discards.rs`, `geometric_cube.rs`, `fixture/m7_8.rs`, `readback_sense_kind.rs`, ...), `crates/editor-core/tests/fixture/mod.rs`, `crates/mesh/tests/common/mod.rs`, `crates/sweep/examples/near_tangent_census_probe.rs`, `demos/tour/tests/common/rim_select.rs`. Sweep: `grep -rnP "half_edge_end\(\s*[\w.()]*\.he_plus\s*\)" crates demos tools | grep -v ^crates/topo/src`. The plain two-`let` pairs were ported. What is left reads one end inside an expression (`Some(start) == body.half_edge_end(e.he_plus)`, `p(body.half_edge_end(d.he_plus).unwrap())`), and each port needs the edge key in scope.

Inside `crates/topo/src` the same call is mostly an Euler operator's test asserting a half-edge's own end, which is not this class.

The gate cannot hold this shape: by text, `half_edge_end(e.he_plus)` is also a half-edge's own end. Once the sites are ported, deprecating the pair spelling is the remaining lever.

## Closed by `onecopy/edge-ends-turns`

All 52 sites read `Body::edge_vertices(key)` (44 files). The sweep's grep
now finds none outside `crates/topo/src`. Where a site held only the
`Edge`, its key was taken from where the edge was fetched (an
`edges()` pair, a `get_edge(k)`), so no site needed a new lookup. A
paired `get_half_edge(e.he_plus).start` beside it folded into the same
call. The closed-edge checks that compared `Some(start) ==
half_edge_end(..)` now compare both ends of one `edge_vertices`, which
differs only on a stale `next`, which no test body has.

Not done: deprecating the pair spelling. `half_edge_end(e.he_plus)` is
also a half-edge's own end by text, and inside `crates/topo/src` that
is its use (an Euler operator's tests asserting a half-edge's end), so
the call stays and no gate can hold the edge reading of it.
