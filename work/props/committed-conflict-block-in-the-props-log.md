---
id: committed-conflict-block-in-the-props-log
kind: issue
title: work/props/log.md carries a committed conflict block
status: open
opened: 2026-09-21
priority: P4
cost: E
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
