---
id: placement-rs-frame-carries-51-doc-lines-over-a-7-line-struct
kind: issue
title: placement.rs's Frame carries ~51 lines of type doc over a 7-line struct - the accumulation the nine-paragraphs row left open, now concentrated rather than spread
status: closed
pr: 2499
opened: 2026-09-12
closed: 2026-09-13
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

## Re-measured (2026-09-13), and the re-measurement is the finding

The row asked for the counts to be retaken after
`frame-linear-generic-door-has-no-consumers` landed. They were, by
script (`measure.py`, the lane's), over four states of the file.

**The counting rule, stated because two independent counts of this
disagreed by one and neither had stated its own.** Split the file on
newlines and DROP the empty string a trailing newline produces — that
is the whole of the disagreement; the first count in this PR read that
empty string as a blank line and reported every `code` figure one too
low. A line is COMMENT if its first non-space characters are `//` (so
`///`, `//!` and `//` all count), BLANK if it is empty or whitespace
only, and CODE otherwise. The three partition the file, and the table
below is checked to sum.

| state | `Frame` type doc | file | comment | blank | code |
| --- | --- | --- | --- | --- | --- |
| `7d5782045` (this row's subject, PR 2475's head) | **55** | 569 | 225 | 36 | 308 |
| `a2fcc48c8` (`Frame::linear` deleted, PR 2487) | **55** | 551 | 223 | 35 | 293 |
| `origin/main` at dispatch | **55** | 551 | 223 | 35 | 293 |
| this PR | **52** | 555 | 227 | 35 | 293 |

Two corrections to the row's own numbers first: the type doc was **55
lines, not ~51**, and the struct is a **4-line** declaration under
**4 lines** of field doc, not "7 lines".

**Deleting the door moved the type doc by exactly zero lines.** The
row's premise — *"part of the measurement will move without anyone
writing prose at all"* — is false, and measurably so.
`Frame::linear`'s own doc was two lines and lived on the method; the
type doc's single mention of it reflowed away at no net cost. The
file's comment share went **42.2% → 43.2% of comment+code**, or
**39.5% → 40.5% of the file** — the denominator is named because the
table carries a `file` column and either division is defensible;
both give the same direction, which is UP, because the deleted door
was more code than prose.

That is the answer to the question this row asks. The type doc's size
does not track the door count, because it does not document the doors:
it documents one rule that five doors are cases of, which is the
concentration S6 deliberately chose. A 4-line `Copy` struct whose
CONTRACT is bit-level exactness has a doc proportional to the contract.
Fewer doors will not shrink it, and the parent's answer — *"fewer
doors, not less prose"* — does not survive its own measurement.

The PR's own net effect on the file is **+4 lines**, not −3: the type
doc lost 3, and the fix pass's two near-identity fixtures and their
doc (a different row's deliverable, in this same PR) added more than
that back. The type doc — this row's actual subject — is 52.

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
lines, 55 → 52.

**The trade, recorded rather than left as an absence.** Those nine
lines were `placement.rs`'s only in-repo pointer to META's
`doc-citations-no-gate-checks-rot-silently` arm B. The class is still
tracked, and that row names the placement assertion from its side, so
nothing is lost from the tracker's view. What is lost is the thread a
reader of `placement.rs` could pull: the surviving sentence states the
hazard without naming where it is answered. That is the cost of not
writing a citation that outlives its target, and it is the right side
of the trade only because the pointer would have rotted silently — but
it IS a cost, and a reader who wants the class now has to grep `work/`
for it.

Nothing else moves. This closes the row: the shape is right, and the
measurement that would have argued otherwise says the opposite of what
the row expected.


## Closed 2026-09-13 (PR 2499) — and this row's own premise was false

**Measured, the premise this row was filed on does not hold.** It said
*"part of the measurement will move without anyone writing prose at
all"* once `Frame::linear` was deleted. Across that deletion the type
doc moved by **exactly zero lines**, and the file's comment share went
**up** — 42.2% → 43.2% of comment+code — because the door was more code
than prose. The parent row's *"fewer doors, not less prose"* does not
survive its own measurement.

Two of this row's figures were also wrong: the type doc was **55** lines,
not ~51, and the struct is a **4**-line declaration under 4 lines of
field doc, not 7. Both were the orchestrator's, and both were corrected
by the lane and then re-taken independently by the review.

**The call: 46 of the 55 lines are `Frame`'s and none of them moved**,
including every bit-exactness argument this project wants written down.
The other 9 were about a **citation mechanism** rather than about
`Frame` — rustdoc's inability to link a test function, what a rename
would do, which slate tracks the class — and were trimmed to the
invariant a reader is owed. Type doc 55 → 52.

**Two honest costs, recorded rather than left as absences.** The trimmed
lines were `placement.rs`'s only in-repo pointer to META's
citation-rot row; META still names the placement assertion from its side
so the class stays tracked, but a reader of this file now has no thread
to pull. And the PR's **net effect on the file is +4 lines, not −3** —
the fix pass's new fixtures added back more than the trim removed. The
type doc is unaffected at 52; the growth is in `mod tests`.

The counting rule is now written down with the table, after the first
pass's `code` column came out one low in every row: the trailing
newline's empty string was being counted as a blank line.
