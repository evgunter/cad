---
id: klint-memory-false-after-unsampling
kind: issue
title: memories/agent-lane-operations.md says the k-lint row is sampled; PR 1850 makes that false
status: review
opened: 2026-09-04
pr: 1855
branch: ciw/klint-memory-correction
---


## Why this is its own item and its own `[ev]` PR

`CLAUDE.md`'s git-workflow section: **"PRs that add to or change
`memories/` wait the same way"** — for Ev's sign-off — "that text is read
at the start of every session, so what goes in it is Ev's call." PR 1850
un-samples the k-lint row and originally carried the memory correction
inside it, with a note flagging the rule. That was the wrong call: the
change to `memories/` waits, so it is re-filed here, on a PR titled
`[ev]`, so it reaches Ev's channel instead of riding a merge he never
saw.

**The residue is stated rather than left implicit: from the moment PR
1850 merges until this one does, `memories/agent-lane-operations.md` is
FALSE in three places, and it is read at the start of every session.**
That is the price of the rule, and it is the right price — but it is a
price, so it is written down.

## What the memory says today, and what makes it false

Three sites, all in `memories/agent-lane-operations.md`:

1. **`:102`** — "A green job NAME can sit over a SKIPPED step — k-lint's
   demos rows are their own sampled axis (`klint_row`)." After 1850 there
   is no sampled axis. The CLASS (a green job name over a skipped step)
   survives and is the durable half; the k-lint INSTANCE closes.
2. **`:131`** — "A voided `klint=` trailer still falls back to a draw."
   There is no draw to fall back to. After 1850 a voided trailer fails
   safe on every dimension.
3. **`:234`** — "A missing k-lint row can be ASKED FOR rather than
   re-rolled for: a `CI-Config: klint=dev-probe` trailer … pin it for one
   run", and "**The same two spellings NARROW the lane and the eps rows
   rather than adding them**". After 1850 a row cannot be missing, and
   `klint=dev-probe` in a trailer REDS the classify step rather than
   pinning anything.

## The correction, and the review finding folded into it

The corrected text keeps each class and retires each instance, with the
instance date-stamped so a reader can see when it closed.

**Reviewer's finding 9, applied here rather than carried forward.** The
version PR 1850 carried said, at `:234`, "**Both spellings NARROW every
dimension rather than adding to it**" — which contradicts the two lines
directly above it, which correctly say a trailer now REDS. A reader
taking that sentence at face value would believe `CI-Config:
lane=interval` gives them a narrowed run; it gives them a red. The
sentence in this PR is:

> **The `workflow_dispatch` inputs are now the ONLY spelling that
> narrows**: a trailer may name only the whole-dimension value
> (`lane=both` / `eps=all` / `klint=all`) and reds on anything else, so
> `CI-Config: lane=interval` buys a RED, not less gate.

Also from the same finding: the 96-character line at `:237` is rewrapped
to the file's ~72 columns, and the `:234` bullet gets the same
`(2026-09-04.)` date stamp the `:102` bullet has.

## Disposition

Merge order does not matter for correctness — this PR's text is true only
once 1850 has merged, and 1850's merge is what makes the current text
false. So: merge this one as soon as Ev signs off, and prefer that to be
after 1850 rather than before.

## THE RULING (Ev, 2026-09-04)

> "for 1855 probably just delete those comments that were only there
> because of the sampling"

Taken as the sign-off `CLAUDE.md` requires, and as a direction on
*shape*: the first draft replaced each sampling passage with a longer
one explaining that sampling is gone. That is the wrong move for a file
read at the start of every session — it grows the thing every lane pays
to read, to record a fact no lane needs.

So the sampling material is **cut**, not rewritten. Net **−9 lines**
(16 added, 25 removed) against the draft's +7. Four sites:

1. The "green job NAME over a SKIPPED step" bullet keeps the class and
   the TEAPOT dual that evidences it; the `klint_row` instance and the
   "the drawn row didn't carry it" parenthetical go. The class never
   depended on sampling — any step `if:` can do it — so nothing is
   lost, and the bullet no longer teaches a mechanism that is gone.
2. The `CI-Config:` trailer bullet loses the per-dimension
   sampled/unsampled bookkeeping. One live rule replaces it: a voided
   trailer fails safe, a trailer may only ADD, and narrowing is
   `workflow_dispatch`'s alone.
3. The prose restatement loses its `(k-lint's demos rows are their own
   sampled axis)` parenthetical outright.
4. "A missing k-lint row can be ASKED FOR rather than re-rolled for"
   is deleted entire. It was advice that existed only because rows were
   drawn; with all five always running there is no missing row to ask
   for, and the `CI-Config: klint=dev-probe` spelling it taught now
   reds.

`grep -rn "sampl\|klint_row\|klint=" memories/` leaves only the three
lines in point 2, which are the intended ones.
