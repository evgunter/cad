---
id: tracker-file-line-citations-measured
kind: issue
title: the tracker's file:line citations, measured: 1,508 in open rows across 22 programs, and 96% already name their subject beside the number
status: open
opened: 2026-09-09
---

Measurement offered to `work/code-quality/doc-line-citations-rot-silently`,
which already owns this class and already names the repair. Filed here
rather than written into that row because a stale citation in another
program's slate is routed to its owner and never edited across the
fence (`work/meta/program.md`'s `keep_out`). **Nothing here asks for a
unit.** It is a number that row does not currently have.

## Why this is worth less than it first looks

Ev, 2026-09-09: a drifted line number is cheap to recover from —
`git blame` on the citing file gives the commit it was written at, and
the cited line is whatever it was at that hash. So the cost of the
class is **a lookup, not a wrong answer**. The measurement below reads
as alarming and mostly is not; it is offered as sizing for a tidying
that row already proposes, not as evidence of a defect.

## The measurement

Every `<path>:<line>` citation in every `work/**/*.md` whose
front-matter `status:` is `open`, with bare filenames resolved against
`git ls-files` when the basename is unique. Taken 2026-09-09.

**1,508 citations, in 317 open rows, across 22 programs.**

| check | count |
|---|---|
| file does not exist, or the line is past EOF | **31 (2.1%)** |
| basename ambiguous, so not checkable this way | 204 |
| file exists and the line is in range | 1,273 |

The largest slates are `code-quality` (376), `view` (257), `tcost`
(141), `issues` (92) and `props` (91).

**2.1% is everything a line-number checker can see.** VIEW's three
hand-sweeps over its own slate found roughly three quarters of the
citations they touched pointing at the wrong subject — 20 of 25, then
26 of 29, then 27 of 36, each time with most already wrong at that
sweep's own merge base. Those are all in the "file exists, line in
range" column. So the gap between 2.1% and ~75% is the part that is
invisible to any check over line numbers, which is the argument
against building one.

## The figure that matters for the repair

That row proposes replacing citations with a symbol-anchored form
"where the line number adds nothing". **That is cheap here, because the
symbols are already written.**

Counting a citation as *anchored* when a backticked identifier or a
quoted phrase appears within one line of it:

| | count |
|---|---|
| anchored | **1,446 (96%)** |
| unanchored | 62 (4%) |

Per program it runs 90–100%. The convention in practice is already
"name the subject and give the line" —

    `session.rs:1656` `add_boolean(op, a: RecipeNodeId, b: RecipeNodeId)`
    `session.rs:1028-1030` ("Same rule as a no-move commit")
    `BooleanOp` is declared at `crates/topo/src/boolean/mod.rs:138`

so adopting the symbol-anchored form is mostly **deleting a redundant
number**, not authoring a new anchor. The 4% unanchored are the rows
where a decision is actually needed.

## One thing this measurement independently confirms

That row's disposition rule — *touch only citations a reader would
follow as live pointers, not narrative entries, whose staleness is
ordinary history* — was rediscovered from the other end by VIEW's
sweeps, which found three shapes that must not be repointed: a citation
whose SUBJECT is gone (repointing invents one), one into a file the
sweeping change never touched, and a pasted tool transcript dated to a
SHA, where rewriting the number would make a true record false. The
third is that row's "narrative entry" exactly.

## Not proposed

A gate. The 2.1% figure says a mechanical check over line numbers would
red on almost nothing that matters, and the recovery cost above says
the rest is not worth a build. If anything is worth doing it is the
tidying that row already describes, at whatever pace suits it.

## Method

The counts are reproducible from the tree at `0762714fd`: iterate
`work/**/*.md`, keep rows whose `status:` is `open`, match
`<path>:<line>`, resolve bare basenames through `git ls-files`, and for
the anchoring figure look for a backticked identifier or a quoted
phrase within one line of the citation. The regex sees neither a
citation written as prose without a line, nor one inside a fenced
block; an earlier pass of it reported 49% broken by counting the
repo's bare-filename shorthand as a missing file, which is the reason
the resolution step is described here rather than assumed.

