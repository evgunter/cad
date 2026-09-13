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

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **M** — needs a retired-id rule plus citation
disambiguation; docs only, no code. The class is a dispatch estimate
made by reading the row against the tree on 2026-09-11, not a verdict on
the finding, and a lane that finds it wrong says so in its PR. The id,
the `track:` letter where the row carries one, and the body above are
unchanged by the move.

## Re-homed to META (2026-09-12, by the CITE orchestrator)

**Overtaken on both halves by sweep 11, and what survives is META's.**

CITE held this row as a numbering-and-citation item. Sweep 11
(`docs/DOC-LEDGER.md`, 2026-09-11) swept `work/code-quality/` out of the
tree, and with it both of this row's targets:

- the **retired-id rule** it asks for has no home left. The register's
  numbering scheme is *retired, not relocated* (the ledger's words); ids
  are minted from item names now, not from per-track blocks, and the
  blocks left with the directory.
- the **citation disambiguation** it asks for inside
  `SMELL-T-LOG.md:41,55,57,306,314,325,386` and `SMELL-KPW-LOG.md:451`
  has nothing live to edit — those logs went to the archive at the
  sweep's SHA.

What survives is the third thing the row implies but never asks for, and
it is squarely META's: **a `work.py` check that an id is never
reissued.** `scripts/work.py` is META's territory, and the row's own
argument for why the check is worth having is unaffected by the sweep —
the reuse was evidence about the *allocation procedure*, and the hole it
names (a number arriving from a track's own closed history, invisible to
any re-derivation against the live tree) is a property of minting ids at
all, not of the block table that is now gone.

Moved rather than closed-and-refiled: the id and the argument are worth
keeping, and one-file-one-item says a re-home is a `git mv` plus this
record. Nothing in the body above is edited; read its `work/code-quality`
paths as of the pre-sweep tree, recoverable through the ledger.
