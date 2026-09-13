---
id: placement-rs-frame-carries-51-doc-lines-over-a-7-line-struct
kind: issue
title: placement.rs's Frame carries ~51 lines of type doc over a 7-line struct - the accumulation the nine-paragraphs row left open, now concentrated rather than spread
status: closed
pr: 2499
opened: 2026-09-13
closed: 2026-09-12
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

## Re-measured (2026-09-12), and the re-measurement is the finding

The row asked for the counts to be retaken after
`frame-linear-generic-door-has-no-consumers` landed. They were, by
script, over four states of the file:

| state | `Frame` type doc | file | comment lines | code lines |
| --- | --- | --- | --- | --- |
| `7d5782045` (this row's subject, PR 2475's head) | **55** | 569 | 225 | 307 |
| `a2fcc48c8` (`Frame::linear` deleted, PR 2487) | **55** | 551 | 223 | 292 |
| `origin/main` at dispatch | **55** | 551 | 223 | 292 |
| this PR | **52** | 526 | 215 | 276 |

Two corrections to the row's own numbers first: the type doc was **55
lines, not ~51**, and the struct is a **4-line** declaration under
**4 lines** of field doc, not "7 lines".

**Deleting the door moved the type doc by exactly zero lines.** The
row's premise — *"part of the measurement will move without anyone
writing prose at all"* — is false, and measurably so.
`Frame::linear`'s own doc was two lines and lived on the method; the
type doc's single mention of it reflowed away at no net cost. The
whole-file comment share went 42% → 43% across that deletion, i.e.
UP, because the door was more code than prose.

That is the answer to the question this row asks. The type doc's size
does not track the door count, because it does not document the doors:
it documents one rule that five doors are cases of, which is the
concentration S6 deliberately chose. A 4-line `Copy` struct whose
CONTRACT is bit-level exactness has a doc proportional to the contract.
Fewer doors will not shrink it, and the parent's answer — *"fewer
doors, not less prose"* — does not survive its own measurement.

## The call: one specific thing moved, and the row closes

Of the 55 lines, 46 are `Frame`'s: what it stores, why a general
linear part rather than an axis–angle triple, what rigidity is not,
and the exactness rule with a pointer to where its guards live.
Every one is load-bearing and several are the bit-exactness arguments
`memories/output-stability-as-justification.md` fences.

The remaining 9 were not about `Frame` at all. They were about a
CITATION MECHANISM: that rustdoc cannot link a test function, what a
rename over in `asm2a_instantiate` would do, and that the class is
scheduled on another program's slate. The rename narration and the
slate pointer are the parts a reader of `Frame` never needs — and the
slate pointer is itself a citation that dies when `meta` closes and its
directory is deleted. Trimmed to the invariant that is owed: the name
is hand-written, no gate reads it, grep for the assertion. Net −3
lines, 55 → 52, and the class itself is still tracked where it belongs
(META's `doc-citations-no-gate-checks-rot-silently`, arm B).

Nothing else moves. This closes the row: the shape is right, and the
measurement that would have argued otherwise says the opposite of what
the row expected.
