---
id: a-family-holds-a-name-table-per-member
kind: issue
title: A family holds a name table, a Carry and a results-map clone per member, where a frame and an index tuple would do
status: open
priority: P2
cost: H
opened: 2026-11-02
---


From the `a-family-has-no-member-ceiling` fork (fork-log row 235, PR 6287): both designers found the measured ~17.5 KB per member (PR 6267's ring, debug build) to be scaffolding, not geometry; read from the code, not profiled.

- `name_family` (`crates/editor-core/src/names/emit.rs`) builds a flat family table holding every member's names again, beside the per-member tables.
- Each member's `NodeValue` holds its own name table (`Member { at, of }` over the source's names), derivable from the member's index tuple and the source's table.
- Each member holds its own `Carry`.
- The members are held again as the family's `Instances`.
- `overlay` (`eval/members.rs`) clones the whole results map once per member: O(members × nodes), transient.

[repetition] says copies of one body are built once and mapped; a placement member's intrinsic data is a frame and an index tuple. Acceptance: the ring measurement (`g_index_families`), per-member bytes and time before and after, with no value changed. This lowers the constant; it does not replace the member budget.
