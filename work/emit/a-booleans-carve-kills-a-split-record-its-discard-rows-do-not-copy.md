---
id: a-booleans-carve-kills-a-split-record-its-discard-rows-do-not-copy
kind: issue
title: The boolean's carve kills an edge with its SplitEdge record outside every discard row's chains, so the chase names its live child a seam
status: open
opened: 2026-10-14
priority: P3
cost: M
---



## What

Found by the blind-spot pass behind
`a-booleans-merge-stage-kills-split-records-no-row-copies.md` (closed),
which copied the merge stage's dropped records. That pass recorded, per
body, every `SplitEdge` record any kill dropped (`kev`, `kef`, `kemr`,
and `splitting::finish`'s bulk kill), and checked where each lineage
walk in the naming layer stopped.

`emit_topo::chase_edge_to_table` still stops at a dead parent once in
the editor-core suite (`boss_flush_offer::the_flush_boss_unions`).
The parent (`EdgeKey(13v1)`) was killed by `splitting::finish::carve`
under `boolean::finish::setopfinish`: the boolean's carve of discarded
material. No `DiscardRow::boundary_chains` holds its hop, and the
output stage does not see it. Its child (`EdgeKey(14v1)`) lives, so
the chase returns the dead key, the edge goes to `chord_kind`, and it
is named a seam (`Cross`). Followed through the dropped record, the
lineage reaches operand A's edge `EdgeKey(10v1)`, which A's table
names, so by N2 the edge is a piece of that line.

The other walks: `borders::Obstacles::up_lineage` ended at a dead key
with an uncopied record 6,049 times across the union tests, and,
followed on through every dropped record, never met its stop (68
union and emit tests). So no `Borders` set moves. `chase_split_lineage`
ended at none.

## Next

- Find why `carve` kills this edge outside the chains `discard.rs`
  writes. It may be interior to the discarded region, or a chain may
  stop short of it. Either copy its hop into the boundary chains or add
  a carve-killed lineage beside `BooleanNaming::joined_lineage`, which
  `KilledSplits` already reads.
- `discard.rs` is fenced while PR 5107 works there, so this waits for
  it or goes through it.
- Re-pin what moves: the boss's corpus row, if it has one, and the
  name digests.
