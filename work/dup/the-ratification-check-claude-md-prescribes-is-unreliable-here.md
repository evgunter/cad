---
id: the-ratification-check-claude-md-prescribes-is-unreliable-here
kind: issue
title: The provenance check CLAUDE.md prescribes is unreliable in this checkout, in two measured ways
status: closed
opened: 2026-09-19
closed: 2026-09-19
---


## Finding

- **Where**: `CLAUDE.md`, the paragraph *"Check that Ev ever agreed,
  before you wait for Ev"*, which prescribes
  `git log -S'<the sentence>' -- <file>` as the check.
- **Importance**: high — it is the procedure that decides whether text
  binds, and it is wrong in a direction that manufactures authority
- **Confidence**: sure, both failures measured
- **Raised by**: the S-DUP orchestrator, 2026-09-19, out of
  `the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files`

**This is for Ev.** `CLAUDE.md` is project instruction text; a lane
should not rewrite the rule it is checked against. Filed rather than
fixed.

## Failure 1 — the shallow clone

This checkout carries **149 shallow grafts over 18,910 commits**. At a
graft boundary every file reads as newly added, so `git log -S` returns
a long list of parentless commits, any of which can be mistaken for the
commit that wrote the sentence. Path-scoping does not fix it.

Measured: this program cited `ff000b52e` — a `github-actions[bot]`
render commit — as the author of ratified memory text. The real commit
was `3a272fefb`, 2026-09-04. `ff000b52e` is one graft of 149, not a
special case.

**What works**: `git log --all --format=… -- <path>` read oldest-first,
then read the actual diff. `--all` is not optional here.

## Failure 2 — `-S` is line-shaped

`git log -S` matches a literal string, so a sentence that **wraps
across two `//!` lines is invisible to it**, exactly as it is to a
single-line grep.

Measured: `git log -S"nothing here is a protected class"` returns
nothing; `git log -S"protected class"` finds `928909d97`. The
prescribed check is a line-shaped instrument with the same defect as
the grep it exists to check, and this program hit that defect three
separate times on one row's census before hitting it on the
provenance check.

## Why the direction matters

Both failures fail toward **manufacturing authority**, not toward
losing it. A wrapped sentence returns no commit, which reads as *"no
ratification turns up"* — and CLAUDE.md's own instruction on that
result is *"there is none — proceed"*. A graft returns a bot commit,
which reads as provenance. Neither says "I could not tell".

This program has now twice attributed to Ev something he did not write
(`the-pr-17-promotion-attribution-is-half-checked`, and a truncated
justification read as a prohibition), so the failure is not
hypothetical.

## What would settle it

A sentence in `CLAUDE.md` naming `--all`, the oldest-first read, and a
short phrase rather than a whole sentence as the needle — plus, if Ev
wants it mechanical, a `scripts/` helper that does the three steps and
prints the author, so the check is one command rather than a
convention. **The wording is Ev's to choose.**

## Closed (2026-09-19) — Ev approved the note, and it landed

Ev's instruction: *"maybe add a single-sentence (or less) note at the
right spot saying to watch out for a shallow checkout? (don't say that
it **is** shallow since this will sometimes be false)"*.

`CLAUDE.md`'s ratification paragraph now reads, after the `git log -S`
command:

> Pass `--all` and use a short phrase rather than a whole sentence:
> `-S` is literal and line-shaped, so a wrapped sentence returns
> nothing, and in a shallow checkout every file reads as added at a
> graft.

Conditional, as Ev asked — it warns without asserting that this
checkout is one. Both failures are named in the clause that prescribes
the remedy, so a reader meets them where the command is.

**The defect demonstrated itself while this row was being closed.** To
answer Ev's *"which line"* about the sibling row's attribution, I ran
`grep -rn "PR #17 thread"` and got **one** hit. The row said seven. A
multiline scan returns **seven**: six carriers wrap the phrase across
two `//!` lines and are invisible to the line-shaped search — the exact
failure written into `CLAUDE.md` minutes earlier, committed by the
person who had just written the warning.

That is the fourth time this sitting a line-shaped instrument
undercounted a `//!` class, and the first time it happened to someone
who had the warning open in front of them. **Knowing an instrument's
blind spot does not make you run the other instrument.** What does is
the habit the program now has: when a count from a grep disagrees with
a count from a row, the grep is the suspect.
