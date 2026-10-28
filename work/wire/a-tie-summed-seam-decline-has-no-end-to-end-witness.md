---
id: a-tie-summed-seam-decline-has-no-end-to-end-witness
kind: issue
title: After PR 5876 nothing end-to-end witnesses the tie-summed decline, or a tied pair seam group's own count
status: closed
opened: 2026-10-28
priority: P3
cost: E
closed: 2026-10-28
---

Filed by the WIRE orchestrator from the PR 5876 review (MINOR).

## What

- PR 5876 retired `resolve_group_membership::a_seam_group_tied_parents_share_is_not_counted`. That test was the end-to-end witness of `Group::tie_summed` → `None` → decline.
  - Its replacement, `a_tied_parents_seams_are_bare_so_no_fragment_of_them_vanishes`, never reaches that rung.
  - The replacement's second assertion (nothing vanishes) follows from its first (every tie-citing seam is bare), so it cannot go red on its own.
- The path is still live through `GroupRecord::record_by_name` for pair seam vertices and joined sets: `crates/editor-core/src/names/emit_topo.rs` around lines 2561 and 3267.
- Nothing witnesses the new per-candidate count a tied pair seam group records, which `GroupResized` reads.

## Done when

- A fixture reaches a `record_by_name` tie-summed group (a pair seam vertex or a joined set) and the rung declines on it. The witness is red if that group is recorded as counted.
- A tied pair seam group's own count is pinned at `GroupResized`.
- The redundant assertion in the replacement test either goes or is made independent.

## Done in `wire/tie-summed-decline-witnessed`

- **A tied pair seam group's own count is pinned.**
  `resolve_group_membership::a_tied_parents_seam_group_counts_its_own_pieces`
  forks each tied prong into two tines (`tied_tines_cut`), so each bar
  wall seams each tied cap in two chords. Every vanished fragment reads
  `GroupResized { was: 2, now: 0 }`. With `name_boolean_edges` keyed by
  the two names alone again, it reads `was: 4`.
- **The redundant assertion is gone.** `vanished` keeps only
  `Fragment`-tailed rows, so once every tie-citing seam is bare nothing
  can vanish. The test is now `a_tied_parents_seams_are_bare`.

## Why the decline witness was not built

The decline cannot be reached from either `record_by_name` site. The
rung (`resolve::group_resized`) returns at `fragment_base(name)?` for a
bare name before it reads the record, and neither site leaves a
`Fragment`-tailed row under its base:

- **Pair seam vertices** (`name_boolean_vertices`, the last loop). Every
  member goes through `defer::mint_candidates`. A lone one is `put` and
  several are pushed to the tie lane, and either way it lands under
  the bare base.
- **Joined sets** (`name_boolean_edges`, the `joined_sets` loop). The
  group is an `EdgeGroup` with `Lone::Whole`, and `name_edge_pieces`
  gives a lone edge the bare base. A group needs two result edges with
  one `joined_cover` set before it is qualified. On a straight or
  non-periodic carrier that cannot happen. Each member of the set
  overlaps both edges, so the member covering the first edge's start
  runs past that edge's end and holds it whole, which makes the first
  edge `JoinedCover::One`. A closed carrier escapes this argument:
  two arcs that overlap at both ends could give one set two result
  edges. That case is unbuilt. The review tried 7 disc-minus-slot
  variants and got no `Merged` edge. Still untried is a lateral face
  merged across two coaxial coincident cylinders. If such a tied set
  were reached, its rung would decline when it need not. The set is
  keyed by operand edges, so its members are one candidate's. But
  `record_by_name(.., from_tie)` stores `tie_summed = true`, so
  `line_counts` (`groups.rs`) returns `None` for it.

Measured as well. The editor-core suite ran without its interval and
m10 slow rows, about 2,700 tests, with `record_by_name` and
`group_reading`'s `sizes() == None` branch instrumented. There were
1,248 tied `record_by_name` calls, every one a lone seam vertex
(`n = 1`), and `sizes()` was never `None`.

So `Group::tie_summed` is never read by a rung that answers. Proposed,
not done (src): record joined sets with `record(.., Parent::Elsewhere)`,
since they are keyed per candidate already. That is also the fix for the
closed-carrier case above. `Group::tie_summed`'s doc (`groups.rs`) is
false at the joined-set site: it says tied parents' members may share
the group, but that group is keyed per candidate. Fix the doc with the
proposal. Then retire
`record_by_name` and `tie_summed`, or keep them with
`group_resized`'s doc saying the rung never reaches such a group.
`emit_union`'s one call passes `tied = false` and is
`wire-unionrecord`'s ground.

## Closed (2026-10-28, `wire/joined-sets-record-per-candidate`)

The decline this row wanted witnessed no longer exists:
`GroupRecord::record_by_name`, `Group::tie_summed` and the `None`
count are deleted, so no group the emitter records is a tie's sum.

- **Joined sets** (`name_boolean_edges`, the `joined_sets` loop) record
  with `GroupRecord::record(.., Parent::Elsewhere)`. The set is keyed by
  the operand edges themselves, so a tied set's group is one
  candidate's. The closed-carrier case above would answer with its own
  count instead of declining, if it can be built (below).
- **Pair seam vertices** (`name_boolean_vertices`, the last loop) record
  nothing. They are always minted bare (`defer::mint_candidates`), so
  the rung returned at `fragment_base` before reading them. Their
  group was keyed by name, so recording it with `record` would have
  stored a tie's sum as one parent's count.

Over the editor-core `ci` profile (2,784 tests), before and after,
every `Emitted` table hashes the same (1,733 emissions). The only
change in the group records is that 5,150 seam-vertex groups are gone:
4,790 `Crossing`, 216 `EdgeCrossing` and 2 `Seam`, all of count 1, plus
142 tied `Crossing` groups that read `None`. No joined set was tied.

The closed-carrier witness is still unbuilt, and no fixture was tried.
Building one needs two result edges on one circle that no member arc
holds whole, with every member arc overlapping both. That means two
vertices on the circle that survive the output stage's join. The
operand whose feature keeps such a vertex has its own rim broken
there, so its arcs overlap one result edge only. A crossing the
boolean makes removes one side of the rim. So the shape looks
unreachable from a pair boolean, but this is argued, not proved. The
lead the brief named, a lateral face merged across two coaxial
coincident cylinders, was not tried. With the tie-summed path deleted,
nothing is left for such a witness to tell apart, so it is not
scheduled.
