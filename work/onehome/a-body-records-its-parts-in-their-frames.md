---
id: a-body-records-its-parts-in-their-frames
kind: issue
title: Stage 2 of fork 288: the body gets a frame table and geometry reads go through one aligning door
status: open
opened: 2026-11-18
priority: P2
refs: [7079, 7099, the-graft-holds-a-placed-copy-twice, vertex-on-edge-is-answered-from-the-claim-not-from-coordinates]
---

Filed by the ONEHOME orchestrator: stage 2 of fork 288 (PR 7079,
approved 2026-11-18). `[placement.frame]` now reads "a body records
each of its parts in the frame it was built in, and that record is
the part's data".

**Precondition (Ev's, in PR 7079).** Stage 1 lands first: every
kernel door over two bodies takes placed operands, and one aggregate
door (PR 7099 is its first). Then the whole-Boolean numbers at boxed
motions go back to Ev before this starts. The cross-placement probe
(`onehome/cross-placement-probe`, not merged) could only show the
readings; it showed carrier readings bit-identical, classification
8/8 against 0/8 at wide boxes, and the whole Boolean refusing first
in data the up-front move wrote.

**The unit.** The body gets a frame table (a tree; relative motions
through the nearest shared frame). About 630–860 geometry reads move
behind one aligning door, module by module, behind a temporary
accessor. A Boolean's result keeps each operand's survivors bit for
bit under its own frame. Estimated 9–30 PRs depending on staging.
