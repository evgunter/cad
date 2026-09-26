---
id: the-edges-at-a-vertex-are-spelled-per-sweep-suite
kind: issue
title: The edges (or faces) meeting a vertex are read through its emanating orbit, written out per sweep suite
status: closed
opened: 2026-09-26
closed: 2026-09-26
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

## Closed (2026-09-26, PR: batch 6)

**The home is two `&self` doors on `topo::Body`, beside
`vertex_orbit`**: `Body::edges_of_vertex` and `Body::faces_of_vertex`,
one private engine (`orbit_projection`) under both. The face and
SURFACE variants are one door and a projection — a caller wanting the
distinct surfaces maps each face to `Face::surface` and dedups, since
two faces can wear one chart — and the edge variant is its own door,
because an edge is a different projection of the same half-edge.

**The contract, as the rustdoc states it**:

- **Dedup order is part of the answer**: orbit order from the vertex's
  stored `emanating` half-edge, each entity at its FIRST reach. The door
  does not sort — a sorted list cannot give the orbit order back, and
  every production FACE consumer (`blend::admit`, `offset_together`)
  reads orbit order. A caller that wants a set sorts at the site.
- **Empty is not refusal** (`faces_of_solid`'s rule): a lone vertex
  answers `Some(vec![])`; a stale key or a broken orbit answers `None`,
  and so does a face projection that does not resolve.
- `None` is the only refusal, so a caller naming which hop failed keeps
  its own walk.

Pinned by `body::tests::the_vertex_doors_project_the_orbit_once_each_in_orbit_order`
— against the raw orbit rather than a sorted set, over the strut cube's
root (a face reached twice) and tip, the `mvfs` lone vertex, a stale
key, a broken orbit and an orphaned loop. `source_walk::DOORS_MEASURED`
counts `&mut` doors only and does not move (its guard rows green). No
`gated_to!` marker names one of the old homes' FILES; four name their
directory, `crates/sweep/src/blend/` — `review_chamfer_r1_probes`,
`review_d2_adv_probes`, `review_verbs_rim_lever_probes`,
`verbs_rim_r1_probes`, all four in the reach control's red set below —
and none of them named `crates/topo/src/body.rs`, where the walk now
lives. All four now do (`gated-suite-paths.sh` and `ci-filter.py
--selftest` green).

**The census, re-taken at `032999ff2`** (`git grep vertex_orbit`, no
path argument; then every hit read past its walk):

| site | disposition |
| --- | --- |
| `sweep/src/blend/battery.rs` `vertex_edges` | folded; `cap_incidence` order-free, `classify` sorts at the site so the supports' normals keep their key order into the independence determinant |
| `sweep/src/blend/surgery.rs` `vertex_edges_of` | folded at all five call sites; the four that sorted still sort, the two redundant `dedup`s go (the door dedups) |
| `sweep/src/blend/build.rs` `vertex_faces` | folded; `admit.rs` and `open/planar.rs` prose re-pointed |
| `topo/src/offset_together.rs` `faces_at_vertex` | its walk folded; it stays as the one-line mapping of `None` to the module's entity-agnostic `ReplaceFaceError::Corrupt`, which `body::tests::the_walk_consumers_keep_their_own_refusal` pins |
| `topo/src/boolean/rest.rs` `incident_faces` | **kept**: its refusal names the hop (three `desync` messages) and a pierce-ring vertex with no fan contributes its host face — both things the door cannot say |
| `fillet_h5_hostless_rim::valence`, `ladder_split_key::incident`, `review_ladder_split_key_r1_probes::incident` | deleted; callers read `edges_of_vertex` (order-free uses) |
| `review_ladder_split_key_r2_probes::meridian_at`, `verbs_arms3` (sorts: it compares to `mouth`'s arcs), `review_arms3_r1_probes`, `blend1_r1_probes` | folded |
| `shell7_dump::faces_at`, `shell8_dump::faces_at` | deleted; the dumps sort at the site so their printed lines do not move |
| `shell7_common::distinct_surfaces_at`, `sf2a_r1`, `sf2a_r1_head::worst_incidence`, `sf2b_head::corner_forms` | folded onto `faces_of_vertex` plus the surface or plane projection. `sf2a_r1` and `sf2b_head` dropped their `emanating` guard, so a lone vertex now counts as valence 0 (a `0` histogram bucket, a `0 []` corner form) where it was skipped (`sf2a_r1_head`'s maximum reads nothing from one); their fixtures are closed solids with none, so nothing they print or assert moves |
| `editor-core/tests/emit_union_flush_names.rs` `faces_at` (arena scan of half-edges starting at `v` and their mates' loops) | folded: on a manifold body the same SET, and both callers read it as a set |
| `fillet_h5_r2_probes`, `verbs_f7_r2_probes` ×2, `topo/src/offset_axial.rs` `corner_arms`, `offset_together`'s corner arm lengths, `boolean/sectors.rs`, `splitting/neighborhood.rs`, `merge_faces.rs` ×2, `shell.rs::valence`, `boolean/rest.rs` ×3 more, `boolean/insert.rs` | **not members**: each reads the orbit's half-edges one by one (a length, a sector, a tangent, a mutation mid-walk), not a deduped set |
| `review_m2_pr5::valence` (half-edges starting at `v`, counted by arena scan) | **not a member**: a half-edge count, independent of the orbit by construction |
| `s49_census_jurisdiction::walls_by_vertex` (cylinder faces split by whether a loop holds a half-edge starting at `v`) | folded: `faces_of_vertex` then a partition of the cylinder faces, each side in arena order as before |
| `band_ruled_d_hole` (~:93, any half-edge starting at `v` in a RING loop) | **not a member**: it asks which LOOP a half-edge lies in, which no face projection answers |
| `editor-core/src/names/emit.rs` `vertex_edges`, `topo/src/census.rs` `vertex_faces`, `topo/src/review_m1_pr4.rs` `vertex_faces` | **not members**: whole-body incidence INDEXES built in one arena pass, not a per-vertex walk |

**What the instruments could not see**: a walk spelled without
`vertex_orbit` or `emanating` (second pass: `\.start == (v|vertex|…)`
arena scans, which found `emit_union_flush_names`'s member and the three
non-members above); a macro-assembled walk.

**Behaviour on corrupt bodies: unchanged.** The doors answer an empty
list for a vertex with no emanating half-edge; the old blend helpers
answered `None` (`.emanating?`). At every blend site the vertex is a
link end, a corner or a rim crossing, so edges meet it and an empty fan
is corruption. One private helper, `blend::build::fan_at`, folds the
empty answer back into the door's `None`, and all eight blend calls go
through it, so each keeps its merge-base refusal (read at `032999ff2`):
surgery's chain-end corner (~:589, `BodyNotIntact` "a chain end's
vertex orbit does not walk"), `resolve_seam_split_rim`,
`refresh_annulus_seams`, `resolve_annulus` and `rim_phase` (each
`BodyNotIntact` "a rim vertex's edge orbit"), battery `classify`
(`Indeterminate`), battery `cap_incidence` (`None`), and
`CornerFaces::admit` (`BodyNotIntact` "a corner's face orbit does not
walk") — none of them a geometric verdict with a recourse attached.
Without the helper they read `unbuilt_chain` or a valence-0
`NEdgeVertex` carrying `RunOutStopAtVertex`.

**No `sweep` test can build that body**: a fan whose vertex holds
`emanating: None` needs `Body::get_vertex_mut`, which is `pub(crate)`
in `topo`, and `with_entity_removed_for_tests` removes entities rather
than clearing a field. So the helper is pinned by its own row
(`blend::build::tests::an_empty_fan_refuses_like_a_broken_orbit`) and
measured by plant over the 953 rows of every module the reach control
below reached:

| plant in `fan_at` | red |
| --- | --- |
| R1 an empty fan passes as `Some(vec![])` (the pre-fix behaviour) | **1**: the helper's own row. No body in the suite reaches the empty case, which is what makes it corruption-only |
| R2 every answer refused (reach control) | 317 (sweep lib 8, sweep `all` 256, editor-core 53) |

**Measured by plant** (harness restores the file's pre-plant bytes and
checks the tree hash; C1 over all of `topo` and `sweep` plus
`editor-core`'s blend/fillet/chamfer/round/emit_union rows outside the
interval lane; Q1–Q4 over every module C1 reached, 952 rows):

| plant in `orbit_projection` | direction argued | red |
| --- | --- | --- |
| C1 `panic!` at entry (reach control) | no answer satisfies it | **539** (topo lib 4, sweep lib 7, sweep `all` 494, editor-core 34) |
| Q1 dedup dropped | every repeat kept: a longer answer wherever an edge or face is reached twice | 58 |
| Q2 answer reversed | order only | **2**: the pinning row, and `editor-core` `seat4_verb_lowering::the_blend_documents_evaluate_to_their_committed_digests` |
| Q3 a lone vertex refuses | the empty answer becomes `None` | 1 (the pinning row) |
| Q4 a face projection that does not resolve is skipped | relaxes the refusal: the probe is a guard of the refusal, and reds only where a row asserts it | 2 (the pinning row, `the_walk_consumers_keep_their_own_refusal`) |

Q2 is why order is in the contract: a committed blend-document digest
moves when the faces around a corner arrive in another order, so orbit
order is load-bearing downstream and the door keeps it rather than
sorting. The fold kept it too — the digest row is green at the head.
