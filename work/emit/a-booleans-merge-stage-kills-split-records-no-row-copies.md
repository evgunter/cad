---
id: a-booleans-merge-stage-kills-split-records-no-row-copies
kind: issue
title: The boolean's merge stage kills edges with their SplitEdge records and no naming row copies them, as the join stage's now does
status: closed
opened: 2026-10-14
priority: P3
cost: M
closed: 2026-10-14
branch: emit/remap-run-refuses
---


## What

Found by the sweep behind
`a-booleans-same-step-split-and-join-loses-the-childs-lineage.md`
(closed). The pattern was an output-stage kill that drops a
`Provenance::SplitEdge` record a naming reader walks.

`topo::boolean::ops::finish_output` kills edges in two stages, and only
one keeps the records:
- **the join** (`edge_join::join_stage`) copies each killed edge's
  record into `BooleanNaming::joined_lineage`;
- **the merge** (`merge_coplanar_faces_unjoined`) kills the edges
  between the faces it merges, and no row copies their records.

**Why it might matter.** Suppose an edge this step split is killed by
the merge, and its child survives. The child's own record still names
the dead parent. But a walk that has to climb past that parent
(`borders::Obstacles::up_lineage`, falling back to `split_from`) stops
there, because the parent was minted this step and so was never live
at the end of an earlier step.

Not measured: no fixture is known to reach it.

**Other joins outside the boolean**, each with its own naming:
`Body::join_edges_within`'s callers in `shell.rs` and `replace_face.rs`.
Their records are not copied either. The Split copies its own
(`SplitNaming::joined_lineage`).

## Next

- Probe whether any piece edge's lineage passes through an edge the
  merge killed in the same step: count the hops `up_lineage` ends on
  for want of a record, across the editor-core suite.
- If any do, copy the record as the join stage now does.

## Closed

**Measured** (`b41d2fe38f`, full editor-core suite, an uncommitted
probe that noted each record an output-stage kill dropped and checked
every lineage walk's last key against them). The merge dropped a
record 1,134 times, 711 of them with a live child under the dead
parent. Walks that ended at such a parent:
- `emit_topo::chase_edge_to_table`, 342 times. Every one changed the
  outcome: the chase stopped short of the operand table, so the edge
  went to `chord_kind` and was named a seam (`Cross`); followed through
  the dropped record it reaches a named operand edge, a piece of whose
  line it is (N2). Tests: `crossing_slots` and the corpus-wide and
  union-order rows that evaluate it, `emit_crossing_sense`, `docm8`.
- `borders::Obstacles::up_lineage`, 20,872 times. Followed on, the
  walk met its stop 6 times, all in
  `union_member_orders_decide_alike` (the probe matched a key's value
  across bodies, so these may be another body's). With the fix that
  test passes, and the only corpus table that moved is the one the
  chase explains.
- `chase_split_lineage`: none (the Split copies its own).
- `Body::split_root`'s callers in `topo` (`props::run_edges`,
  `boolean::join::along_edge_conic`): reached; filed, below.

The shell's join left a live child under a killed parent 4 times
(`emit_shell`'s three-point rim test). No naming walk reads the
shell's lineage: `emit_shell` names a joined edge through
`edge_joins`, not the records. The offset doors' join
(`replace_face.rs`) dropped no record in the suite.

**Fixed** (records only): `finish_output` takes `split_records` before
the merge and writes each edge the merge killed into
`BooleanNaming::joined_lineage` through `record_joined_lineage`, which
now takes the killed edges rather than the joins.
`emit_topo::KilledSplits` reads `joined_lineage` into its arena hops,
so the boolean's chase steps through output-stage kills as it does
through discards; `borders.rs` already read the table.

**What moved:** `crossing_slots` only. A stretch of the first cavity's
bottom rim, kept where the second slot's flush floor merged the
stretch beside it away, was a seam of the cavity's wall and the slot's
floor; it is a piece of the rim. The floor corner where the slot's rim
meets that wall moved from a touch (`Crossing`, `Inside`) to a
crossing of the two rims, as the other three such corners are named.
Re-pinned: `lib_g16_corpus_name_digests` and
`perf2_name_keying_differential` (that row; the persisted text held),
`seat4_verb_lowering` (that digest), `name_words_corpus` (the full total
411,498 → 411,652 words, `SAID_DIGEST`), and
`emit_crossing_sense::a_rim_on_the_other_cavitys_floor_meets_its_rim_where_it_meets_its_wall`
(four rim crossings, no touch). New:
`topo` `boolean_discards::an_edge_split_and_merged_away_in_one_step_records_its_parent`,
which fails with the merge's copy removed.

**Filed:** `work/origin/body-split-root-stops-at-a-parent-an-output-stage-killed.md`
(`split_root` reads the body alone; the shell's dropped records reach
only those readers).
