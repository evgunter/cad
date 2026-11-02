---
id: body-split-root-stops-at-a-parent-an-output-stage-killed
kind: issue
title: Body::split_root stops at a parent an output stage killed, so props' carrier identity and the boolean join's germ lane read a cut-short lineage
status: open
opened: 2026-10-14
priority: P3
cost: M
---



## What

Found by the measurement that closed
`work/emit/a-booleans-merge-stage-kills-split-records-no-row-copies.md`.

An output stage can kill an edge that holds a `SplitEdge` record while
a child split off it lives on. The kill drops the parent's record, so
the child's lineage in the body ends at a dead key:
- the boolean's merge of coplanar faces (`boolean::ops::finish_output`);
- the shell's closing join (`shell.rs`, `join_edges_within`).

The naming readers now step past such a key through the copies in
`BooleanNaming::killed_lineage`. `Body::split_root`
(`crates/topo/src/provenance.rs`) reads the body alone, so two of its
callers in `topo` stop at the dead parent:
- `props::run_edges` (`props.rs`, the `carrier_id` stamp): two pieces
  of one edge, one of them under a killed parent, get two carrier ids,
  so a fold that should take them as one carrier declines;
- `boolean::join::along_edge_conic` (`join.rs`, its `AlongEdge` root): an
  `EdgePlane` germ lane is keyed by a root that is not the edge's.

**Measurement** (`b41d2fe38f`, full editor-core suite, an uncommitted
probe): `split_root` ended at a dead key whose record a merge or the
shell join had dropped 132,758 times from `props.rs` and 192 times from
`join.rs` (the union-order tests and `emit_shared_rim_several`). The
probe matched a dead key by its value across the whole process, and
bodies cloned from one arena share key values, so the counts are an
upper bound. The shell join left a live child under a killed parent 4
times (`names::emit_shell::tests::a_rim_cut_at_three_points_joins_to_the_set_of_its_four_pieces`);
the offset doors' join (`replace_face.rs`) dropped no record in the
suite.

What each reader does wrong with the short root is not measured. The
`props.rs` comment says the fold declines rather than mis-folds, so it
may cost a fold and nothing else.

## The sibling finishers (sweep, 2026-10-14)

None of these copies the records its kills drop:
- **The public `merge_coplanar_faces` door** (`merge_faces.rs`, merge
  then `join_edges`). `MergeCoplanarOutcome` reports
  `groups[].killed_edges` and `joins`, but not the dropped records. Its
  only callers are `census.rs` and tests, and no naming layer reads
  them.
- **The blend's closing join** (`sweep/src/blend/surgery.rs`,
  `join_edges_within`). `BlendNaming::lineage`
  (`sweep/src/blend/naming.rs`) builds `topo::Lineage` from
  `through_joins` and the blend's own division rows, and never reads a
  `SplitEdge` record. Neither does `emit_blend`. So only `split_root`'s
  readers on a later body can see the loss.
- **The shell's closing join** (`shell.rs`), measured above: no naming
  reader walks its records.

The blind-spot pass recorded the records every kill drops (`kev`,
`kef`, `kemr`, and the split's bulk kill), which covers these joins.
Across the editor-core suite it found one naming walk that stops short,
and that one is the boolean's carve
(`work/emit/a-booleans-carve-kills-a-split-record-its-discard-rows-do-not-copy.md`).
So these siblings reach only `split_root`'s body-level readers listed
above.

## Next

- Decide whether the kill should leave the record readable (a kill
  that keeps dead edges' records in a side table on the body, which
  `split_root` falls back to), or whether each reader should take the
  copies, as the naming readers do. This touches
  `the-split-edge-lineage-walk-has-four-homes.md`: the single walk that
  row asks for could take the copies as a second lookup.
- Measure the readers' outcomes with the copies followed, as the emit
  row did for the naming chase.
