---
id: editor-core-suites-redefine-the-name-table-helpers
kind: issue
title: Twenty-four editor-core suites still redefine the name-table and body readers fixture now has one home for
status: open
opened: 2026-09-17
---


## Finding

`crates/editor-core/tests/fixture/mod.rs` now holds one home for the
readers a name-reading suite works an evaluation with — `tol`, `table`,
`key_of`, `edge_of`, `vertex_of`, `face_of`, `count`, `point`, `ends`,
`face_vertices`, `face_edges` — and for the name-authoring shorthands
`minted`, `fname`, `ename`, `vname`, `rim_edge`, `cap_vertex`. Two
suites read them from there (`edit_ladder_rim.rs`,
`edit_ruled_carve.rs`, PR #2794). **The rest of the tree does not.**

The reviewer of PR #2794 counted the class before the move: `table`
×19, `point` ×9, `key_of` ×5, `count` ×5, `face_of` ×4, `minted` ×3,
and hand-rolled vertex names ×9. Re-measured after it, over
`crates/editor-core/tests/` less `tests/fixture/`, matching both a
`fn <name>(` definition and a `let <name> = |` closure:

- `table` — 12 files (`docm7_union_declare`, `blend5_r1_probes`,
  `m9_d1_r1_probes`, `m4_pr3_names`, `blend5_rim_support`,
  `m4_pr3_names_bool`, `blend5_r2_probes`, `wire_product_gather_tie`,
  `ring_r1_names_probe`, `m9_d1_r2_probes`, `lib_tube_r1_probes`,
  `lib_g14_split_walls`)
- `tol` — 5 files (`edit_step_segments`, `seat7_sweep_lowering`,
  `seat4_verb_lowering`, `seat8_split_lowering`, `docm9_range`)
- `count` — 4 files (`review_m5_pr9_doc_probe`, `blend5_rim_support`,
  `m5_pr10_nodes`, `m10_10_evidence_interval`)
- `point` — 2 (`switch_program_vocabulary`, `wire_rv_bytes`);
  `key_of` — 2 (`switch_program_key`, `msolve1_transform_aware`)
- a vertex name spelled out as a `StableName` literal rather than
  through a shorthand — 8 sites in 6 files (`corpus/kiss_carry`,
  `wire_entity_door`, `docm7_union_declare`, `m9_d1_r1_probes`,
  `m9_d1_r2_probes`, `m4_pr5_declare`)

Twenty-four distinct files in all. The counts differ from the
reviewer's because this pattern matches closures and definitions inside
nested modules as well; both are measurements of the same class.

## Why it matters

A copy is not merely repetition: the copies have already DIVERGED.
`edit_ladder_rim`'s `face_vertices` returned a `HashSet` and
`edit_ruled_carve`'s a `Vec` of the same walk, and every caller of
both wanted membership — so one of the two was doing work no row
asked for, and nothing said which. A reader of either file could not
tell whether the difference meant anything.

## Not swept in PR #2794, deliberately

That unit's fence is the two `edit_*` suites its rows live in. The
twenty-four files above are a different change with a different blast
radius — each copy has to be read for divergence before it is deleted,
the way these two were (byte-identical bodies measured by diff before
the move, both suites' rows green after). This row is that work.

## What a taker owes

The same discipline, file by file: diff the copy against
`fixture`'s door before deleting it, say in the PR which copies had
diverged and how, and leave a suite-local adapter where the local
spelling carries document-specific knowledge (`edit_ladder_rim`'s
`rim_edge` knows the plate's cap end and its `Rim`; it delegates rather
than duplicating). Where a door does not fit a caller, widen the door
or leave the copy and say why — a forced fit is worse than the copy.

## Territory

`crates/editor-core/tests/*` is tcost's and tint's ground; EDIT works
in it under the ladder-rim and ruled-carve units and files here beside
them.
