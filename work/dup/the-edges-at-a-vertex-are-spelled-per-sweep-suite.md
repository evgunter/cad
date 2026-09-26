---
id: the-edges-at-a-vertex-are-spelled-per-sweep-suite
kind: issue
title: The edges (or faces) meeting a vertex are read through its emanating orbit, written out per sweep suite
status: open
opened: 2026-09-26
priority: P1
cost: D
---


## Finding

- **Where**: `crates/sweep/tests` and `crates/sweep/src/blend` — the
  hit list below.
- **Confidence**: sure; every hit read past its `emanating` line.
- **Raised by**: the `dup/sweep-topo-drain` lane, 2026-09-26, in the
  second pass `solid-of-vertex-is-hand-spelled-twice-in-sweep-tests-beside-solidowners`
  asked for (every other `emanating` read in the suites, read to see
  whether it climbs to a solid).

None of those reads climbs to a solid. Most of them spell a different
walk, one the row did not name: **the edges (or faces, or surfaces)
meeting a vertex**, read as `get_vertex(v).emanating` → `vertex_orbit`
→ each half-edge's `.edge` (or face), sorted and deduplicated. `topo`
has `Body::vertex_orbit` and no door above it.

| site | what it reads |
| --- | --- |
| `fillet_h5_hostless_rim.rs` `valence` (~:103) | edges, sorted, deduped |
| `ladder_split_key.rs` `incident` (~:64) | the same body, byte for byte |
| `review_ladder_split_key_r1_probes.rs` `incident` (~:43) | the same body, byte for byte |
| `review_ladder_split_key_r2_probes.rs` `meridian_at` (~:72) | the same walk, filtered to the one non-rim edge |
| `verbs_arms3.rs` (~:297) | the same walk, inline |
| `review_arms3_r1_probes.rs` (~:401) | the same walk, inline, over every vertex |
| `blend1_r1_probes.rs` (~:323) | the same walk, inline, per rim crossing |
| `shell7_dump.rs` `faces_at` (~:38), `shell8_dump.rs` `faces_at` (~:21) | faces, sorted, deduped — two copies |
| `shell7_common.rs` `distinct_surfaces_at` (~:131) | surface keys, counted |
| `sf2a_r1.rs` (~:258), `sf2a_r1_head.rs` (~:84), `sf2b_head.rs` (~:158) | faces' plane normals or surface keys, per vertex |
| `fillet_h5_r2_probes.rs` (~:135), `verbs_f7_r2_probes.rs` (~:49, ~:114) | the orbit's half-edges read one by one, not as a set — not members |

The same walk is in production too, in BLEND's `crates/sweep/src/blend`:
`battery.rs` `vertex_edges` (~:1418) and `surgery.rs` `vertex_edges_of`
(~:831) are the edge form twice over, and `build.rs` `vertex_faces`
(~:226) the face form, all `Option`-returning. So the class spans
`src` and `tests`, and a door would serve both.

The remaining `emanating` reads in `crates/*/tests` are an anchor for
`mev_null` (`mesh` `review_m3_pr1_mesh`, `sweep` `review_m3_pr1_sweep`,
`topo` `m3_pr4_boolean`, `review_m3_pr1` ×2), orbit valence asserts
(`topo` `box_with_hole`, `cube_by_hand`, `m3_pr2_reduce`,
`review_m3_pr2`), a vertex's owning SHELL (`topo` `m3_pr3_split`
~:300 — shell granularity, which `SolidOwners` does not answer), a
dump (`topo` `split_edge_pcurve_rows`), and the vertex arm of
`shell8_common::deep_dump`, which that PR routed to `SolidOwners`.

## What a taker owes

A home, then the fold. With three production members beside the
test-side ones, the candidate home is a `&self` door on `topo::Body`
beside `vertex_orbit` (TOPO's ground, announced by seam) rather than a
test-side reader; whether the face and surface variants are the same
door with a projection or separate doors is the decision, and a
`&self` accessor adds nothing to `source_walk`'s mutation-door census.

**Why P1 and not P4.** The test-side copies alone would be P4. The
two `crates/sweep/src/blend` edge helpers are two implementations of
one underlying logic in production, which is the band `work/README.md`
puts at P1. It is cost `D`, not `E`: whether the face and surface
variants are one door with a projection or separate doors is a design
decision the taker makes before any fold.
