---
id: committed-conflict-block-in-the-props-log
kind: issue
title: work/props/log.md carries a committed conflict block
status: closed
opened: 2026-09-21
priority: P4
cost: E
closed: 2026-09-21
---


## Finding

`work/props/log.md` on `origin/main` (at `f8906fecd3`) carries a
committed conflict block: `<<<<<<< HEAD` at line 1868 and `>>>>>>>
origin/main` at line 2014, the two sides of a log merge left
unresolved. Found by LANE-1's post-merge marker grep
(`grep -rn '^<<<<<<<\|^>>>>>>>'` over the tree); the block predates
the branch (it is on the spec commit `141e4a128e`'s parent too).

It is an instance of the class
`work/ciw/committed-conflict-markers-reach-main.md`, which Ev closed
as not worth a tree-wide gate. This row is the instance, not the
class: the log is append-only narrative, so which side to keep (or
both, deduplicated) is the owning program's call, and a lane outside
PROPS should not resolve it in passing.

## What to do

Open the block, keep both halves in order or deduplicate them, and
delete the three marker lines. `python3 scripts/work.py lint` does not
read the log, so nothing mechanical reds on it meanwhile.

## Closed — repaired (2026-09-21, CHROME orchestrator)

Repaired in PR 3018 before this row was read: the three marker lines
deleted, **both narratives kept in order and unedited**, and a note on
`work/props/log.md` saying so. Nothing was chosen between the sides
because nothing had to be — a log is append-only narrative, neither
side had deleted anything, so two sessions appending at one place is a
union.

This row's reading of the class was right and mine was not: it cites
`work/ciw/committed-conflict-markers-reach-main` as **closed on Ev's
call**, and correctly files the instance without reopening the
argument. I appended a note to that closed row arguing for the gate,
having read neither its status nor its closing section; the retraction
is there.

If PROPS judges that either side was meant to replace the other, the
repair is wrong and the correction is yours — both texts are as they
were.
