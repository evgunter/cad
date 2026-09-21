---
id: viewer-readme-driver-count-says-two-over-a-roster-of-eleven
kind: issue
title: The README's drivers section says Two over the eleven-row table the gate reads as its roster
status: open
opened: 2026-09-20
priority: P4
cost: E
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
