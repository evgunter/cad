---
id: frame-rs-says-the-per-subject-line-is-a-question-for-ev
kind: issue
title: frame.rs tells every reader a design question gates the per-subject line, and no such question was ever put
status: open
opened: 2026-09-20
priority: P2
cost: E
---


Filed by VNEWS's frame.rs-cluster adjudication (2026-09-20). It is the
CARRIER for group A of `work/vnews/plan.md` §Order 7 — the `frame.rs`
prose pass — because it is the only member with a consequence beyond
its own sentence, and a rider needs an id to ride.

## The sentence

`crates/viewer/src/frame.rs:656-658`, inside `frame_status`'s doc
comment:

> Making the line stop being one string — several labels, one per
> notice — is the better answer and is not this function's to give: it
> needs a value that carries several subjects, which
> `work/view/one-line-one-subject-loses-a-mixed-frames-expiry.md` owns
> and **which is a design question for Ev**.

Written by `30443532b` (2026-09-15, *"view: two levels, two marks"*),
author Claude.

## Why it is wrong, and why that matters more than a stale pointer

**No such question was ever put, and nothing gates it.** The searches
are in `work/vnews/rank-one-discards-the-frames-other-news`, which is
the canonical home for the ruling: `crates/viewer/GUI-DESIGN.md` — the
Ev-gated document — carries no clause about the status line, none of
`docs/DESIGN.md`'s Q1–Q9 is the status line, and no `[ev]` commit has
ever touched `crates/viewer/src/frame.rs`.

The sentence's cost is not that it is untrue. It is that **a reader who
meets it stops**: a design question for Ev is a wall, and the row it
points at is on a slate whose plan said *"nothing here waits on Ev"*
the whole time. This is the same propagation the adjudication found in
the tracker — an agent's doc comment read as ratified, then quoted as
ratified by two rows within four days — appearing here as a gate rather
than as a clause. `work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
is the class.

## The stale pointer is a second defect and is already owned

The same sentence cites `work/view/one-line-one-subject-…`; the row
moved to `work/vnews/` in the 2026-09-17 re-scope.
`work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`'s
table names `crates/viewer/src/frame.rs:657` and `:704` and owns that
half. A lane taking this row **announces the crossing and strikes those
two table lines, or leaves the path alone and repairs only the gate
claim** — the two must not both land.

## The fix

Delete the gate claim. What survives is true and worth keeping: the
per-subject line is not `frame_status`'s to give, it needs a value
carrying several subjects, and
`work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry` owns the
fork. `frame.rs:704` carries the same pointer without the gate claim
and is the wording to match.

## What rides on it

`work/vnews/folded-moved-true-arm-covers-a-fold-that-did-not-move`
(the doc line half), `work/vnews/tone-doc-argues-from-a-site-that-now-reads-the-value`
(its two `frame.rs` members),
`work/vnews/a-dead-composition-sets-up-a-fixture-without-saying-so`,
and `work/vnews/hand-maintained-counts-in-frame-rs-prose-have-no-guard`.
Each is its own file with `rides_with:` naming this row, per
`work/README.md`; closing this one does not close them.
