---
id: viewer-readme-driver-count-says-two-over-a-roster-of-eleven
kind: issue
title: The README's drivers section says Two over the eleven-row table the gate reads as its roster
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-21
branch: vdoc/readme-counts
---



Pre-existing, found by the review of `vnews`'s
`tone-is-a-value-in-frame-and-a-comment-in-two-panes` and filed across
the fence: nothing in that unit caused it, and nothing in it depends on
the count being right.

`crates/viewer/README.md`, `### The drivers` (~`:308-310`) opens:

> **Two, as the rule says, and the second is split for size.** This
> table is the roster, not a summary of one: `viewer-module-kinds.sh`
> reads it, requires every module in it to declare `driver` in its own
> header, and refuses a `driver` declaration on a module the table does
> not list.

The table under it has **eleven** rows — `session`, `app`, `pane`,
`pane::create`, `pane::features`, `pane::profile`, `pane::properties`,
`pane::view`, `pane::viewport`, `widgets`, `gpu`.

**Why this is worse than an ordinary stale count.** The sentence and
the table disagree about what the section IS. The next sentence says
the table is *the roster* and that the gate reads it, so eleven is the
operative number and `Two` is a claim about a rule the roster does not
implement — `widgets` and `gpu` are not "the second driver split for
size", they are separate drivers. A reader who takes `Two` at face
value and then adds a twelfth row *as an amendment here* is doing
exactly what the paragraph tells them to do, against a count that says
they cannot.

**The fix is a decision, not an edit**: either the prose states the
roster's real shape (two ROLES — session and app — with the app driver
drawn across nine modules, if that is the true reading), or the
ratified *"exactly two drivers"* rule is amended. The gate settles what
the tree does; it does not settle which sentence was meant.

This is the paragraph the tone unit's dependency-direction argument
leans on for *a driver may name any vocabulary, and no vocabulary may
name a driver or the toolkit* — that clause is elsewhere in the section
and is unaffected, but a reader checking the argument meets this
contradiction on the way.

## Closed — the prose states the roster's shape; the ratified rule is untouched (#vdoc/readme-counts)

**The fork this row named, taken on its first branch.** The row offered
two: state the roster's real shape, or amend the ratified *"There are
exactly two [drivers]"* rule in `## Module boundaries`. The first is
taken. Nothing this PR writes changes what that clause decides, so it
is not an Ev PR: the clause counts DRIVERS and the table is a roster of
MODULES, and the only defect was a sentence that let a reader take one
for the other.

**The enumeration rule, written at the sentence.** `### The drivers`
now says the table counts modules, and gives the scan that produces the
same population from the other side:

    rg --files-with-matches '^//! Module kind: \*\*driver\*\*' crates/viewer/src | wc -l

which prints the number rather than leaving it to be read off a list.
**Eleven**, run exactly as printed on the merged tree at `f45df59dc5`.
Eleven files carry the declaration — `session`, `app`, `pane`,
`pane::create`, `pane::features`, `pane::profile`, `pane::properties`,
`pane::view`, `pane::viewport`, `widgets`, `gpu` — one per table row.
`scripts/gates/viewer-module-kinds.sh` holds the two sides to each
other and says so in its own OK line: *"11 drivers match
crates/viewer/README.md's own table of 11 rows"* (exit 0, run here).

**Two is still true of the drivers** and the paragraph now says which
number answers to which question: two drivers, the app one drawn across
ten of the eleven modules, which `### The app driver, split for size`
already stated six hundred lines further down and nothing near the
table pointed at.
