---
id: borders-records-a-join-twice-and-reads-lineage-three-ways
kind: issue
title: borders.rs spells a join's seam handover twice (eager seams extension and lazy joined_from chase) and reads split lineage from three sources
status: open
opened: 2026-10-13
priority: P4
cost: E
---


## What

These are style findings from the PR 4949 review, recorded and not gating.

**`borders.rs`:**
- **One fact spelled twice.** "A join makes the gone edge's seams the kept edge's" is written eagerly, by extending each earlier discard's `seams` through `joined_edge`, and lazily, by the `joined_from` chase. A mutation shows neither alone covers what both do, and no comment says which case each owns.
- **Three lineage sources.** Split lineage comes from `body.edge_provenance_of` on the final body, from `split_from` over each step's live edges, and from `split_from` over the discard chains. For a union the first is subsumed by the second, so the two-level match is left over from the old dead-split design.
- **Duplicate seam entries.** The seam extension pushes a `kept` edge twice when two seams join into it, and its `contains` filter is O(n²).
- **Silent cross-chain cycles.** Cycle detection is per chain, so a loop that crosses chains is absorbed by `seen` rather than refused.

**`emit_union.rs`, `group_member_edges`:** the `From` arm and the new arm that lies within a member edge are near-duplicates. They differ only in `least_edge` against `least_within`.

**Tests:** `emit_union_flush_names` keeps an empty `KNOWN_REFUSING`, with its machinery, where `emit_union_rim_piece_ranks` deleted its admissions outright. One shape should be picked.

## Since (2026-10-19, `emit/borders-closure`)

The `joined_from` chase is now documented as closure's channel for a
point the output stage joined away (`Obstacles::joined_from`). Without
it, four rows of the six naming suites go red
(`two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`
among them). The eager `seams` extension through `joined_edge` is
unchanged, so the two spellings of the handover remain.
