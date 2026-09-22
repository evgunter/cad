---
id: band-refusal-still-badges-every-row
kind: issue
title: MateFault::Band still badges every row in the cluster — the filed defect, surviving in one arm
status: dispatched
opened: 2026-09-04
refs: [1769, 1463]
priority: P1
cost: E
branch: chrome/rowstatus-exhaustive
---

Found by CHROME's style lane on PR 1769, judging that PR's own
disclosed carve-out.

PR 1769 sends the eye to the offending mate for every `MateFault` arm
that names one. `MateFault::Band` names none
(`crates/editor-core/src/mate.rs:414-418`), so `blamed_mates` returns
empty for it (`crates/viewer/src/tree.rs:294`) and every row it reached
keeps its own `Failed`.

The PR defends that in three sentences: no band, no decisions, no mate
more at fault than another. **That is a good reason not to PICK a
mate. It is not a reason the filed symptom is acceptable.** A `Band`
refusal is a run-tolerance failure, so it fans out across the cluster
exactly like the arms that were fixed — which means for that one arm a
user still meets N identical FAILED badges with the eye sent nowhere,
which is issue 1463 verbatim.

Nothing is scheduled and no test covers the `Band` shape, so the
carve-out currently reads as closed rather than as the remaining half.

**What closing it probably looks like**, offered as a starting point
rather than a design: a row reached by a cause that names no single
node still knows it is one of many, so the honest badge names the
CLUSTER rather than the row — "one of N in a refused cluster" — which
sends the eye somewhere true without inventing a culprit. That is a
different shape from `Poisoned { through }` and may want its own
status, which is why it is filed rather than folded into 1769.

Signed: (CHROME orchestrator)

## Measured 2026-09-15 (`chrome/band-refusal-badging`) — the arm is covered now, and the proposed wording names the wrong set

A first pass covered the shape and corrected two of this row's
premises. The symptom stands; the remedy sketched above does not.

**The refusal is the RUN's, not a cluster's.**
`editor_core::mate::solve::solve_document` builds its band as the first
thing it does and, on failure, inserts one cloned `MateFault::Band`
against **every** `Node::Mate` and **every** `Node::InstantiatePart` in
`doc.order()` before it reads a single mate — before `has_mates`,
before `read_mates`, before any cluster exists. So the refusal reaches
instances in singleton clusters that no mate touches, and a document
with three clusters has all three refused. *"One of N in a refused
cluster"* would therefore name a strictly smaller set than the one the
user is looking at, and computing N from `clusters` — the route the
dispatch expected — would be computing the wrong number correctly.
Whatever wording closes this row has to say *the run*, not *this
cluster*.

**The arm is reachable, along exactly one path.** At a tolerance
`Band::linear` refuses, `eval::wire`'s own band door refuses most nodes
with `NodeErrorKind::Band` first and the authoring door refuses a
profile outright, so no document carrying geometry ever gets far enough
to show a `MateFault::Band`. What does: a document of instances and
mates read back at a tolerance that admits no band — which is what
`Tolerance::init_document_eps` commits when a saved document's own ε is
the pathological one. The badging complaint sits on that path and
nothing else.

**Covered now, for the first time.**
`crates/viewer/tests/tree_badges.rs` gains
`a_band_refusal_reaches_the_whole_document_and_blames_no_row` (with its
re-exec'd child `child_band_refusal_rows` — `Tolerance` commits once
per process and `tests/all.rs` is one binary, so the pathological
tolerance runs in a child, `crates/editor-core/tests/
wire_band_cause.rs`'s pattern). It pins three things: the refusal
reaches the unmated instance as surely as the mated pair; every reached
row's message is the payload's own rendering byte-identical, so no
cohort clause may be composed onto a failing row; and **no row is drawn
downstream of another**, so a lane closing this row by PICKING a
culprit reddens it. That last assertion is the decision PR 1769 made,
held by a test rather than by a comment.

**What still blocks the fix, and it is a fence, not a design gap.** The
honest badge is a state of its own — a row saying "the run refused,
not this node" — exactly as this row guessed. A new `RowStatus` variant
does not land inside CHROME's fence: `crates/viewer/src/pane/
features.rs` matches `RowStatus` exhaustively with no wildcard, so a
variant is a compile error there, and `crates/viewer/src/frame.rs`
asserts in prose that `RowStatus` *"has exactly three non-`Ok` states"*
— both VIEW's files while VIEW is live in this crate. Adding a FIELD to
`RowStatus::Failed` is no cheaper: a struct pattern that omits a field
is a hard error, and
`grep -rn 'Failed { message }' crates/viewer/ | grep -v src/tree.rs`
is **19 sites in 9 files** on this tree — eight test suites plus
`examples/r1_e2e.rs`. (An earlier draft of this paragraph said 17 in 8.
That is the count with `tests/tree_badges.rs`'s own two excluded —
i.e. the file the lane was editing — beside a file count that included
it. A measurement in prose with nothing re-taking it: the command above
is written out so the next reader re-takes it rather than trusting the
number, and it will drift as suites are added.) The direction of the
error is worth stating: the true cost is HIGHER, so it strengthens the
decision to decline rather than weakening it. So this wants a VIEW
handoff or a unit whose fence spans `pane/` — not a bigger effort, a
wider one.

`crates/viewer/src/tree.rs`'s module header and `blamed_mates` now say
all of the above at the code: the header's claim that *"every
`MateFault` arm but `Band` names its subject"* was false —
`PosesOfAnotherDocument` names none either — and the two shared one
arm and one comment, which is what let the live carve-out read as
settled. They have an arm each now, because one reaches rows and one
cannot.

## The fence is gone; the cost argument stands (2026-09-21, orchestrator)

The section above closes on *"a new `RowStatus` variant does not land
inside CHROME's fence … both VIEW's files while VIEW is live in this
crate"*. **That fence is retired** (`work/chrome/plan.md`, *Territory
— the carve-out is spent*): VIEW re-scoped on 2026-09-17 and
dispatches no new units, and `work/README.md`'s 2026-09-20 ruling
makes `pane/features.rs` and `frame.rs` ordinary shared ground. The
row's own sentence — *"this wants a VIEW handoff or a unit whose fence
spans `pane/`"* — is answered: a unit whose scope spans `pane/` is now
an ordinary CHROME dispatch.

**What does NOT change is the measurement.** The row's count of
`Failed { message }` sites is the reason to add a VARIANT rather than
a FIELD, and it is re-taken by the command the row writes out, not
trusted from its prose. Re-take it at dispatch.

## Cross-reference (added by the `chrome/empty-document-gate` lane)

The new status this row's closing shape proposes is the event
`has-faults-cannot-red-on-a-new-rowstatus` is about.
`crates/viewer/src/tree.rs`'s `has_faults` asks
`matches!(row.status, RowStatus::Failed { .. } | RowStatus::Poisoned
{ .. })`, so **a `RowStatus` variant added for a refused cluster is
silently not a fault**: every row carrying it answers `false` to
*"this document is not building"*, and no build says so.

Whoever takes this row should make that `matches!` exhaustive before
adding the variant, or take both rows together.

