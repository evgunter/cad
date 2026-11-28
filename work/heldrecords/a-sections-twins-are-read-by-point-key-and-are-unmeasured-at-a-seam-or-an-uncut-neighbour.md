---
id: a-sections-twins-are-read-by-point-key-and-are-unmeasured-at-a-seam-or-an-uncut-neighbour
kind: issue
title: A section cell's twins are re-derived by point key, and the twin rule is unmeasured on a periodic seam, on a boundary section with no twin, and on several edges over one point pair
status: open
opened: 2026-11-28
priority: P2
cost: M
---


Found by the review of PR 7832 (`heldrecords/interior-minted`, the
probes on `heldrecords/review-interior-minted-3b9e71`).

## What

- **The rule.** `Incidence::chords` (`crates/topo/src/boolean/ops.rs:4447`)
  keeps a section cell in a divided face's interior only where each of
  its twins bounds only the face's fragments. `Incidence::of`
  (`ops.rs:4476`) takes as twins every clone cell of the same kind on
  the same point keys.
- **Read by point key, not from the op's records.** The op already
  records which cells are copies of which: the null-edge copy lists
  (`crate::lineage::NullEdgeCopies`, `Ledger.copies`,
  `crates/topo/src/boolean/mod.rs:2773`) and `Descendants::copies_of`
  (`ops.rs:4028`). The twin set re-derives those pairs from points
  instead, so it also takes in the operand's own uncut cells on the
  same points: 1150 of the 2546 excluding twins on ci are of that kind
  (review probe `TWINROOT`). This is the review's Q1.
- **Not measured:**
  - a section along a periodic face's seam, where the twin across the
    seam is a cell of the same face;
  - a boundary section with no twin, where the neighbouring face is
    uncut, so the twin rule keeps it interior. The review's
    `PROBE_TWIN1PANIC` never fired on ci or the slow set, so no test
    reaches it;
  - several edges over one point pair (a multi-edge), whose twin sets
    merge by points into one.

## Done when

Twins are read from the op's copy records, or each of the three cases
above has a row that pins what the rule does there.
