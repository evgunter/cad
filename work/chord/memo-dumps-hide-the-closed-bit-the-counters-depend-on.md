---
id: memo-dumps-hide-the-closed-bit-the-counters-depend-on
kind: issue
title: PatchMemo's and PickMemo's Debug print hit/miss counters without the closed bit that says which picture the counters describe
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Raised by CENSUS-DEBUG
(`work/census/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`),
which destructured both walks. Disclosed rather than fixed: that unit's
spec forbids widening a dump's output, and adding a field is a widening.

## The defect

`PatchMemo` (`crates/mesh/src/memo.rs`) declares `closed` as *"Whether
the last picture was closed and nothing has started the next: the
counters below then describe the closed picture, and the next use
zeroes them."* The fields it qualifies are `hits` and `misses`, both
printed.

Its `Debug` prints `entries`, `picture`, `hits` and `misses` and does
not print `closed`. So a dump reading `hits: 12, misses: 3` does not
say whether those describe the picture being built or the one just
finished — **which is exactly the question `closed` exists to answer**,
and the one a reader of a memo dump is asking.

`PickMemo` (`crates/editor-core/src/resolve/pick.rs`) re-spells the
same machinery and has the same omission over four counters
(`node_hits`, `node_misses`, `table_hits`, `table_misses`); its own
`closed` doc says *"As [`PatchMemo`]'s"*. Held together here because
the bit and its argument are one design, not two;
`work/perf/fnv-digest-and-memo-machinery-copies.md` holds the
consolidation of the machinery itself.

## What CENSUS-DEBUG did instead

Both walks now destructure `Self`, so `closed` binds to `_` and the
omission is a decision a reader can see and the compiler still forces,
and both end in `finish_non_exhaustive()` rather than `finish()` — a
`finish` claims every field is shown, and these do not show `closed`.
The tie is there; what is open is whether the bit should be shown.

## The call

Printing it is one `.field("closed", closed)` per walk and an
`E0027`-held destructure is already in place, so the cost is the output
change and whatever reads these dumps. Nothing in the tree asserts on
either dump's text today (`assert_send::<PickMemo>()` is the only
mention). If it is shown, both walks return to `finish()`.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.
