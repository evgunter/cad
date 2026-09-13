---
id: placement-rs-frame-carries-51-doc-lines-over-a-7-line-struct
kind: issue
title: placement.rs's Frame carries ~51 lines of type doc over a 7-line struct - the accumulation the nine-paragraphs row left open, now concentrated rather than spread
status: open
opened: 2026-09-13
refs: [2375, 2475]
---


## Finding

The live residue of
`placement-rs-states-its-exactness-rule-in-nine-paragraphs`, given its
own file at the moment the parent closed, because `work/README.md` is
explicit that a residue disclosed inside a closing row's prose *"reads
as a record of work done, not as an open thread"* and dies with it.

That row carried three findings. **S6** (one rule, nine homes) and
**S8** (`#[must_use]` inconsistency) were discharged by PR 2475 with
evidence. **S9**, the ratio, was not, and the parent row says so in as
many words: *"the remaining answer is fewer doors, not less prose."*

What PR 2475 changed about S9 is its **shape, not its size**. Before, the
accumulation was spread — nine paragraphs, none the authority for any
other, no single diff ever seeing the total. After, it is concentrated:
`Frame`'s type doc is about **51 lines above a 7-line struct** (the
`# Exactness` section plus the pre-existing rationale), and the file
grew 430 → 569. Concentrated is better — one place to read, one place to
edit, and the rule now has an authority — but it is not smaller, and the
PR was honest that it is not.

From the style review of PR 2475 (S4, confidence `likely`), which read
all 458 lines end to end under Q8; the original S9 was `unsure`, the
reviewer's, from the same kind of read of PR 2375.

## Why this row is deliberately narrow

The parent's title asks whether a 60% prose ratio is a defect, and
nobody has stated what would make the file better. That is not
actionable and a row that asks it would sit open forever.

**This row asks one answerable question**: is 51 lines of type doc over
a two-field `Copy` struct the right shape, given that every paragraph in
it is individually load-bearing and several are bit-exactness arguments
this project wants written down? The parent's own answer points at
**fewer doors** — `frame-linear-generic-door-has-no-consumers` is one
door already decided for deletion — so part of the measurement will move
without anyone writing prose at all. Re-measure after that lands before
concluding anything.

## What this row is not

It is not "delete the prose", for the same reason the parent was not.
Every paragraph is defensible, the `# Exactness` home is an improvement
that cost lines, and `memories/output-stability-as-justification.md`
fences the `bit-identical` vocabulary specifically. A unit that shortens
the file by deleting a bit-exactness argument has made the codebase
worse and passed every gate.
