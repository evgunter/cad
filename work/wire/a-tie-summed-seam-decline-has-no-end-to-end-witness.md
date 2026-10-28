---
id: a-tie-summed-seam-decline-has-no-end-to-end-witness
kind: issue
title: After PR 5876 nothing end-to-end witnesses the tie-summed decline, or a tied pair seam group's own count
status: dispatched
opened: 2026-10-28
priority: P3
cost: E
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
