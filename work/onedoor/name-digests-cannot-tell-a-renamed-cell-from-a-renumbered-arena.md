---
id: name-digests-cannot-tell-a-renamed-cell-from-a-renumbered-arena
kind: issue
title: Name digests cannot tell a name that moved to another cell from an arena that renumbered, so every key-only move is settled by a hand dump
status: open
opened: 2026-11-13
priority: P2
---


Found by PR 6910's review (2026-11-13).

The corpus name digests feed each entity's arena key and slot version:
- `crates/editor-core/tests/lib_g16_corpus_name_digests.rs:27-45`;
- `perf2_name_keying_differential.rs:79-106`;
- `crates/editor-core/tests/fixture/digest.rs:117-121`;
- the seat4, seat7 and seat8 document digests.

So they go red alike when a name comes to denote another cell, which is a defect, and when an unrelated build-order change renumbers the arena, which is not.

**Every key-only move has been settled by a throwaway hand dump that reads each cell by its geometry.** There have been at least three:
- seat4's earlier re-bless note;
- PR 6505 on seat7's `corner_table`;
- PR 6910, after PR 6832.

The cited tell, `m10_p_fence`'s ids-masked row, covers geometry, not which cell a name denotes. So the claim "every name denotes what it denoted" lives only in comments, with no row that goes red when it stops being true.

**The fix: pin a key-free word beside the keyed one.** For each name, pin the geometric reading of the cell it denotes. Use the reading PR 6910's lane and reviewer both built:
- a vertex by its point;
- an edge by its directed ends and curve;
- a face by its sense, carrier and its loops as canonically rotated cycles of signed edge readings.

Then a key-only move turns only the keyed word red, and the key-free word proves the names held. A rename turns both red. Re-blessing the keyed word needs no human dump, and the comments recording dumps can go.

The class covers `lib_g16`, `perf2` and the seat4, seat7 and seat8 document digests.
