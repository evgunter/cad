---
id: split-crossings-skip-pattern-mate-ends
kind: issue
title: Split interface crossings skip pattern-headed mate ends (is_mate_edge_end lacks the member vocabulary)
status: open
opened: 2026-08-31
github: 1405
refs: [1400]
---

## From GitHub issue 1405

Opened 2026-08-31; 0 comments.

Found by MATE-1's class sweep (PR #1400, the A11 member-vocabulary rider — genus: mate-head kind dispatch). Not fixed there: the fix is split/refactor ground, outside that unit's fence.

`crates/editor-core/src/refactor.rs`'s `is_mate_edge_end` recognizes only plain `InstantiatePart` mate ends when collecting the split seam's interface crossings. With the rider landed, a mate may head a pattern-placed instance (`Pattern` + `Instance(i)`), and such an end is skipped — so a split whose seam severs a pattern-headed mate would not carry that mate as an interface crossing. Per the MATE-1 sweep report, the fix needs the `Instance(i)` remap through split's node maps (A4's recorded-map contract), not just a second match arm.

Scope note: A4/refactor territory adjacent to ASM-XSPLIT (the banked AQ8 conversion door). Whoever takes either should take both views of the seam into account.

Signed: (S-MATE orchestrator)

## Home

`work/mate/` — S-MATE's charter names assembly composition (mates × patterns, the instantiation seam), and this is the member vocabulary of a pattern-headed mate end; the refactor.rs site itself is in no open program's territory.
