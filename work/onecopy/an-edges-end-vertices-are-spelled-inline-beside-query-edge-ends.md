---
id: an-edges-end-vertices-are-spelled-inline-beside-query-edge-ends
kind: issue
title: An edge's two end vertices are spelled inline at about seventeen topo sites beside query::edge_ends and Body::edge_vertices
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [6327]
---

Found by the sweep of `the-curve-pair-units-follow-ups-from-its-review` item 1, which moved `carrier_cross::face_boundary` onto `query::edge_ends`.

An edge's two vertices, `he_plus`'s start then `he_minus`'s, have three homes and many inline spellings:
- homes: `query::edge_ends` (linked, panics on a torn link), `Body::edge_vertices` (`body.rs`, `Option`), and `boolean/reduce.rs` `edge_ends` (with the points, `BooleanError`);
- inline, the shape `[e.he_plus, e.he_minus].map(.. .start)` or `(start(edge.he_plus), start(edge.he_minus))`: `lineage.rs` (`ends`), `boolean/edge_join.rs`, `boolean/reduce.rs` (four more sites: `starts`, two `(a, c)`, `ends`), `boolean/overlap.rs` (`(at_t0, at_t1)`), `merge_faces.rs`, and test rows in `census.rs` (five) and `boolean/offer_rows.rs` (two).

Sweep shape: `grep -rnE "he_plus.*he_minus" crates/topo/src | grep -i "start\|vert"`. It cannot see a site that reads `he_plus`'s start and `next`'s start (the `query::edge_ends` spelling itself, re-spelled), so a second pass on `\.next.*\.start` is owed.

Fix: one home per lookup tier (a linked walk that panics, an argument lookup that refuses), the inline sites repointed, and the three homes reduced to those tiers.

