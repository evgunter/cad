---
id: a-booleans-carve-kills-a-split-record-its-discard-rows-do-not-copy
kind: issue
title: The boolean's carve kills an edge with its SplitEdge record outside every discard row's chains, so the chase names its live child a seam
status: closed
closed: 2026-10-14
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
  a carve-killed lineage beside `BooleanNaming::killed_lineage`, which
  `KilledSplits` already reads.
- `discard.rs` is fenced while PR 5107 works there, so this waits for
  it or goes through it.
- Re-pin what moves: the boss's corpus row, if it has one, and the
  name digests.

## Closed

- **Why no row held it.** A discard row chains a discarded face's edge
  only where the face across is not one it counts as kept
  (`kept_across`, which reads the section faces as kept), so an edge
  the carve kills beside such a face is in no chain. In the witness
  that is `EdgeKey(13v1)`, split from operand A's `EdgeKey(10v1)` and
  killed by A's carve, with its child `EdgeKey(14v1)` live.
- **The fix, records only.** `boolean::discard::carved_lineage` copies,
  per operand, the `SplitEdge` record of every edge the carve killed
  (`edge_join::split_records` before, `record_killed_lineage` over the
  edges the carved body no longer holds). `setopfinish` takes it for A
  and B, and `BooleanNaming::carved_lineage` carries it in each
  operand's clone keys. `emit_topo::KilledSplits` reads it into the
  arena or `graft_b` by the operand's keys, as it reads the rows.
  `vtxfac.rs` is untouched.
- **Measured.** With a probe in `chase_edge_to_table`,
  `boss_flush_offer::the_flush_boss_unions` now chases `14v1` to the
  named `10v1` (it stopped at the dead `13v1`). The union's published
  edge names do not move (all 24 compared before and after): the
  collapse re-reads them off the finished body, so the fold step's
  seam spelling never reached the table. The full editor-core suite
  moves no golden. One refusal falls: the Python north-star crosslap
  union (beam A ∪ beam B), which refused `merged_chord_constituents`,
  now evaluates at its exact oracle 1.875 (toggling the copy's read
  brings the refusal back). `TestCrosslapAtTheNamingWall` became
  `TestCrosslapGlued`, and `docs/guide/north-star-audit.md` row 37
  flips NO → YES (45 of 53, 33 outright, 8 blocked).
- **Pinned.** `topo` `boolean_discards::an_edge_the_carve_kills_records_its_parent`:
  the same block and flush boss as a topo union; a live edge is split
  off an edge A's carve killed, and that edge's lineage, through the
  result's records and the copy, reaches an edge of the block.
- **Sweep.** Every `carve(` call and `edge_provenance.remove(` site in
  `topo/src`: the boolean's section path (`finish.rs`, fixed); the
  Split's two carves (`splitting/finish.rs`), whose halves keep each
  other's records (`chase_split_lineage` reads both, and found no stop
  in the source pass); the containment fallback and the sphere re-cut
  (`ops.rs`), which carve whole shells, where no kept edge's parent
  lies in a dropped shell; the Euler kills (`euler_kill.rs`,
  `euler_ring.rs`), the closed merge-stage pass's ground; the test
  fixtures in `review_m1_pr5_internal.rs` and `validate.rs`. The blind
  spot: a carve reached through a door not named `carve` (arena
  surgery by hand); `edge_provenance.remove(` lists every removal of a
  record, and every hit is above.
