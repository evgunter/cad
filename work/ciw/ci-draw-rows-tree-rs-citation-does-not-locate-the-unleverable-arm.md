---
id: ci-draw-rows-tree-rs-citation-does-not-locate-the-unleverable-arm
kind: issue
title: The closed ci-draw row's tree.rs citation names a different arm of the same match
status: closed
opened: 2026-09-15
refs: [ci-draw-can-hide-a-compile-break-on-main]
closed: 2026-09-20
---


Found by CHROME's `chrome/band-refusal-badging` lane while measuring
which `tree.rs` citations its own change moved. This one it did **not**
move: it was already off on `origin/main` before that branch existed.

**Read the disposition paragraph first.** The row this is about is
CLOSED, and the honest answer may be to do nothing.

## The citation

`work/ciw/ci-draw-can-hide-a-compile-break-on-main.md` (closed
2026-09-07), under "Note for whoever takes it":

> `main` compiles at this lane today: the arm is present at
> `crates/viewer/src/tree.rs:325`
> (`| MateFault::Unleverable { mate, .. } => vec![*mate]`).

At the merge base this lane branched from, `tree.rs:325` is
`| MateFault::ClassNotAdmitted { mate }` — a different alternative of
the same or-pattern. The `Unleverable` alternative the row quotes is
several lines below it, and the parenthesised quote is the thing that
makes this legible at all: the row named its subject **as well as** its
line, so a reader can still find it. That is the citation rule working,
in a row written before the rule was stated.

The whole or-pattern reaches one `=> vec![*mate]`, so "the arm" is
loosely true of any line in the group. What is not true is that `:325`
is the line the parentheses quote.

## Why it is small, and filed anyway

It costs a reader seconds, and the quoted text rescues it. It is filed
because the alternative was to leave it inside another program's row as
an appendix line, which dies when that row closes and never reaches
CIW. This is the cheap end of a class both CHROME and VIEW measured
this week; the expensive end is on MSOLVE's slate
(`work/msolve/memo-key-rows-tree-rs-citation-now-lands-on-the-opposite-claim`),
where a citation resolves to a real line showing something else
entirely.

## Disposition

`ci-draw-can-hide-a-compile-break-on-main` is CLOSED and its own text
says *"This issue is about the hole, not about the instance, and the
instance is closed"* — so the citation is decoration on a note about a
thing the row is explicitly not about. Options, and **the owner picks**:

- **Leave it**, and close this row citing that sentence. A closed row's
  incidental line number costs nobody anything, and this lane would not
  argue.
- Drop the number and keep the quoted arm, which is what was carrying
  the meaning anyway.

Nothing is asked of `crates/viewer/src/tree.rs`, which is correct
today, and nothing of CIW's live board.

Split out of `work/view/tree-rs-header-growth-moved-five-cited-subjects.md`
at the CHROME orchestrator's direction rather than left in that row's
appendix (`work/README.md`: a residue disclosed inside another item's
prose is invisible to the re-homing sweep).

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)

## Closed (2026-09-20)

Taken as a drive-by, on the row's own second option — *"drop the number
and keep the quoted arm, which is what was carrying the meaning
anyway."* `work/ciw/ci-draw-can-hide-a-compile-break-on-main.md`'s
"Note for whoever takes it" now names the file without a line number
and keeps the quoted arm, with one parenthesis recording why the
number went. The closed row it edits stays a record of the tree it was
written against; what changed is a pointer inside it that pointed
wrong, which is not the same thing.

This is the shape `work/README.md`'s new "The tracker is not
comprehensive" clause is about: a one-line edit that cost a file, a
header, a lint run and a reader's attention to schedule.
