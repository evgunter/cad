---
id: memo-key-rows-tree-rs-citation-now-lands-on-the-opposite-claim
kind: issue
title: The closed memo-key row cites tree.rs for a corroboration guard the file now argues against by name
status: closed
opened: 2026-09-15
refs: [mate-memo-key-does-not-carry-the-solve]
closed: 2026-09-19
---


Found by CHROME's `chrome/band-refusal-badging` lane while measuring
which `tree.rs` citations its own change moved. This one it did **not**
move: the citation was already wrong on `origin/main` before that
branch existed, and it is wrong in the expensive way — it names the
wrong subject, not a stale number.

**Read the disposition paragraph before doing anything.** The row this
is about is CLOSED, and this may be worth one line of editing or
nothing at all.

## The citation

`work/msolve/mate-memo-key-does-not-carry-the-solve.md` (closed
2026-09-06) says, of the viewer's workaround for the kernel
inconsistency it reports:

> CHROME's fix had to corroborate every blame against the evaluation
> before using it (`crates/viewer/src/tree.rs:337-339`: take the first
> blamed mate the run *agrees* is `Failed`, else keep the row's own
> failure).

Two things are wrong with it, and only the second matters:

1. **The line range never held that code on `main`.** At the merge base
   this lane branched from, `tree.rs:337-339` is the `MateFault::Band |
   MateFault::PosesOfAnotherDocument` arm of `blamed_mates` and its
   comment. The "first blamed mate" line lives in `downstream_of_mate`,
   about forty lines further down. That is the failure mode both CHROME
   and VIEW have spent this week measuring — a citation that resolves
   to a real line, in the right file, showing something else.

2. **The claim it makes about the viewer is now the opposite of what
   the viewer says.** `downstream_of_mate` takes `(id: RecipeNodeId,
   error: &NodeError)` and **has no access to the `Evaluation` at
   all**, so it cannot corroborate a blame against the run even in
   principle. Its doc comment states the opposite policy in as many
   words — *"**The blame is read directly**"* — and justifies it by
   citing **this row's own subject**:

   > a mate's content key carries the solve's answer, so a mate the
   > fault names is `Failed` in the evaluation carrying that fault —
   > never `Ok` off a stale memo

   Which is to say: the kernel fix that closed the memo-key row is
   exactly what entitled the viewer to stop corroborating. So the
   closed row records a workaround, and the file it cites now argues
   that the workaround is unnecessary **because this row was fixed**.

## Why that is worth a file

Not for the number. A reader who follows that citation to understand
what the memo-key defect cost the GUI lands on a paragraph asserting
the guard is not needed, with no way to tell that the record and the
code are describing two different eras. The row is the only account of
what the inconsistency forced downstream, and its evidence no longer
resolves.

This lane did not verify **when** the guard went away, or whether one
ever existed in the shape the row describes — `git log -S` over
`tree.rs` returns merge noise on every phrasing tried, and settling it
is archaeology that only matters if the owner decides to rewrite the
sentence.

## Disposition — this may be correctly closed with no change

`mate-memo-key-does-not-carry-the-solve` is CLOSED and its kernel
finding is fixed. A closed row is a record, and a record is allowed to
describe the past. So the honest options, cheapest first, and **the
owner picks**:

- **Leave it.** Judge that a closed row's stale evidence costs nobody
  anything, and close this row citing that judgement.
- **One line in the closed row**: replace the citation with the
  subject (`downstream_of_mate`, not a range) and add that the guard
  it describes is gone, because this row's own fix retired it. That is
  the outcome this lane would pick, and it is a single edit.
- Archaeology on when the guard went, only if the second option turns
  out to need it.

**No CHROME change is asked for or implied**: `crates/viewer/src/
tree.rs` reads correctly today and its doc comment is right.

Split out of `work/view/tree-rs-header-growth-moved-five-cited-subjects.md`
at the CHROME orchestrator's direction, rather than left in that row's
appendix where it would have died when VIEW closed it
(`work/README.md`: *"a residue a lane discloses inside its own prose
reads as a record of work done, not as an open thread"*).

Signed: (CHROME implementer lane, `chrome/band-refusal-badging`)

## Closed 2026-09-19 (MSOLVE orchestrator)

The second option: the closed row's sentence now cites the subject
(`downstream_of_mate`) rather than a range, and says the guard it
describes is gone because that row's own fix retired it. No
archaeology on when — the row is a record of the era the guard was
needed, and the one sentence names the era's end. No code moved.
