---
id: ratified-is-asserted-across-viewer-src-and-some-was-never-ratified
kind: issue
title: The word ratified is asserted about wording across crates/viewer/src, and at least one escalation is agent-written
status: open
opened: 2026-09-20
priority: P3
cost: E
---

Opened by the style review of #2960 with one confirmed member. **It is
a census before it is a fix**, and the population must be re-derived
against the tree by whoever takes it — the instrument is below, and the
count in this file is evidence only as of its filing.

## The class

A doc comment that calls something **ratified** is doing work no other
sentence does: it tells the next lane that changing this needs an
`[ev]` PR rather than a judgement (`CLAUDE.md`, the merge rules). A
claim of that kind is only as good as the decision it points at, and
the failure mode is an **escalation** — the decision ratifies a
BEHAVIOUR, and the doc that cites it treats the WORDING as ratified
too. That reads as design when it is a lane's own sentence, and it
freezes what nobody froze.

## The confirmed member

`crates/viewer/src/session/refuse.rs`, `Refusal::affordance`:

> "Dragging an expression-driven dimension → refuse, with an
> affordance" is a ratified micro-decision **whose WORDING is part of
> the decision**, so it is composed once and every surface that shows
> it … calls this.

`crates/viewer/GUI-DESIGN.md`'s Micro-decisions (G4) says, in full:

> Dragging an expression-driven dimension refuses, with an affordance
> offering to edit the expression.

**G4 ratifies the behaviour and states no sentence.** *"driven by an
expression over X (currently 0.01) — edit the expression?"* is this
crate's own composition. The one-home rule the doc draws from it is
right and is worth keeping on its own merits — two copies of any
sentence drift — but the reason given for it is a ratification that
does not exist, and `Display for Refusal` repeats the escalation
(*"the affordance arm's wording is a RATIFIED decision of this
layer's"*).

The consequence is live rather than academic: a lane asked to reword
that sentence today would read those two comments, conclude the change
needs Ev, and either stall or open an `[ev]` PR for a decision Ev never
made.

## The instrument, for whoever re-derives the population

`grep -rn 'ratified\|RATIFIED\|Ratified' crates/viewer/src`, and for
each hit: name the document and clause it points at, read that clause,
and say whether the clause decides what the comment says it decides.
`crates/viewer/GUI-DESIGN.md` and `docs/DESIGN.md` are the only two
documents that can ratify anything; a comment citing neither, or citing
a `crates/<crate>/README.md` page, is citing a page whose own
ratification has to be checked the way `CLAUDE.md` says — `git log -S`
for the sentence, and no commit means no ratification.

**What that pattern cannot match**: a doc that asserts a decision is
settled without using the word — *"the ruling"*, *"decided"*, *"Ev's
call"*, *"not re-litigated"* — and a comment that is silent about
having escalated, which is most of them. The word is a starting point,
not the property.

## Not this row

Whether the affordance's wording SHOULD be ratified is a separate
question and a good one; this row is about the tree asserting that it
already is.
