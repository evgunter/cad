---
id: d321-row-number-reissued
kind: issue
title: D321 was reissued: two different rows have carried the number, and the closed one is still cited by it
status: open
opened: 2026-09-04
---


## Finding

`plan.md` §"How the numbering works" says row ids are **stable and never
reused**, so a `D<N>` in prose resolves to one row for life. `D321` resolves
to two:

- the `D321` lane `T-c` closed in PR #1359 — `fillet/admit.rs`'s `include_str!`
  self-reader — still cited by that number from
  `work/code-quality/logs/SMELL-T-LOG.md:41,55,57,306,314,325,386` and
  `work/code-quality/logs/SMELL-KPW-LOG.md:451`;
- the live `work/code-quality/D321.md`, opened 2026-09-02 by CERT-N1 —
  `loft.rs`'s hand-spelled `lift_affine` — closed by T-1.

Nothing distinguishes them at a citation site. A reader following `D321` out
of either closed-track log lands on a row about a different file in a
different crate, and both rows are closed, so neither reads as obviously
wrong.

Found by lane T-1's citation sweep while closing the second one. T-1 did not
renumber: renumbering the live row would break the row it was dispatched on
and the fillet program's citations to it, and renumbering the closed one
rewrites history. It left a one-clause disambiguation in `SMELL-T-LOG.md`'s
lane-state row so a reader following the id does not fuse the two.

**Why this is a file and not a footnote.** The reuse is evidence about the
allocation procedure, not about these two rows: numbers come from per-track
blocks that are re-derived against the tree at allocation *precisely because*
"a block cannot stop a number arriving from another track" — and here a number
arrived from the track's own history, which no re-derivation against the live
tree can see, because the earlier `D321` had already closed and its file was
gone. Every block in the table is exposed to the same hole, so a second
instance is a matter of time.

## Was

`unrowed` — raised by lane T-1 (Track T) while closing the live `D321`.
