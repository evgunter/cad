---
id: topo-integration-tests-hand-write-partial-body-unchanged-walks
kind: issue
title: topo's integration tests hand-write four body-unchanged walks, three of them partial, that a new arena or field passes silently
status: open
opened: 2026-09-30
priority: P3
---



## What

Found by `work/topo/deep-snapshot-is-written-three-times.md`'s receipt
sweep. That row gives `crates/topo/src/` one deep body snapshot,
`fixtures::deep_snapshot`, which has its own row
(`fixtures::tests::deep_snapshot_sees_every_arena_and_provenance_record`):
a new record in any of the ten arenas or seven provenance maps moves it.
`fixtures` is `cfg(test)` in the lib, so the integration crates under
`crates/topo/tests/` cannot call it. Four of them write their own walk
and compare bodies by it. None has a row of its own, and three walk
only part of the body, so a change to what they skip passes:

- `box_with_hole.rs`, `snapshot` (~`:379`), compared in
  `holed_box_lineage_is_deterministic` (~`:446`). A full copy of the
  in-crate snapshot through the public API: ten arenas, `Debug`
  payloads, provenance. An arena added to `Body` has to be added here
  by hand, and nothing reds if it is not.
- `split_edge_pcurve_rows.rs`, `deep` (~`:88`), compared in
  `the_carried_rows_are_the_mint_passs_rows_byte_for_byte` (~`:206`).
  Ten arenas plus the pcurve rows, but it lists fields by hand rather
  than through `Debug`: a face line carries `outer`, `rings` and
  `surface` and drops `sense` and `shell`, and a vertex line carries
  `point` and `emanating`. It reads no provenance.
- `review_m4_pr2_transform.rs`, `arena_dump` (~`:309`), compared in the
  same-map-twice determinism row (~`:124`), and the `topo_dump` closure
  (~`:286`), compared as "topology records must be bit-stable"
  (~`:302`). `arena_dump` walks points, curves, surfaces, edges and
  faces. It skips solids, shells, loops, half-edges, vertices and
  provenance. `topo_dump` walks edges and faces only.
- `void_door.rs`, `reading` (~`:232`), compared as "the body is
  untouched on both" in `a_wrong_destination_arity_refuses_typed`
  (~`:343`) and in `one_destination_through_either_door_reads_identically`
  (~`:270`). It walks shells, faces (`shell`, `sense`, surface value)
  and vertex points. It skips solids, loops, half-edges, edges, curves
  and provenance, so a refusal that mutates any of those passes.

Three more integration files compare whole-body `Debug`
(`format!("{body:?}")`) instead: `review_m2_pr3.rs`, `review_m3_pr1.rs`
and `review_m3_pr5.rs`. That form is derived over every field of
`Body`, so it is not this class.

## Shape to consider

The four could share one public-API walk under `tests/common/`, with a
row of its own like the in-crate one's. Where a walk is partial on
purpose (a transform moves geometry, so `topo_dump` compares topology
only), it can say what it skips and why. Otherwise it can read the whole
body.
