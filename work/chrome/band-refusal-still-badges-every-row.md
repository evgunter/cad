---
id: band-refusal-still-badges-every-row
kind: issue
title: MateFault::Band still badges every row in the cluster — the filed defect, surviving in one arm
status: open
opened: 2026-09-04
refs: [1769, 1463]
priority: P1
cost: E
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


## Measured 2026-09-22 (`chrome/rowstatus-exhaustive`) — the half that ships, and the one file that still blocks the other

The partner row `has-faults-cannot-red-on-a-new-rowstatus` is closed
on that branch: `tree::has_faults` is an exhaustive `match`, so the
variant this row wants can no longer land silently. That was the
prerequisite. **The variant itself did not land, and the reason is one
file.**

**Re-taken, because this row says to re-take it.** `grep -rn 'Failed
{ message }' crates/viewer/ | grep -v src/tree.rs` is **18 sites in 8
files** today, not the 19 in 9 the section above records — one suite
went away. The direction is unchanged and so is the conclusion: a
FIELD on `Failed` still costs more than a VARIANT.

**What a variant costs, measured rather than read.** A scratch fifth
`RowStatus` was added to `tree.rs` and the crate's tests built with
nothing else patched. Every site that reds, and only these:

- `crates/viewer/src/tree.rs` — `RowStatus::badge`, `tone`, `message`,
  `has_faults`. The lane's own ground.
- `crates/viewer/src/pane/features.rs` — the pane's deliberately
  exhaustive *does this row draw a badge* match, under its *"Exhaustive
  on purpose"* comment. One arm, mechanical. Ordinary shared ground
  since `work/README.md`'s 2026-09-20 ruling.
- `crates/viewer/src/frame.rs` — `badge_site`'s guard test
  `the_tree_still_has_exactly_the_three_states_this_policy_pairs_with`.
- *(Since PR 3090)* `crates/viewer/src/pane/features.rs` — the link
  decision described next is exhaustive too, so it reds as well: four
  places, the count `tree::has_faults`'s doc now carries.

**`features.rs` is not one mechanical arm, and the compiler is why
this reads as though it were.** Twelve lines below the exhaustive
match, inside the same `feature_row`, the pane asks the second
question — *does this row's message link anywhere* — as
`match &row.status { RowStatus::Poisoned { through, .. } => Some(*through), _ => None }`.
That wildcard does **not** red, so the scratch variant above never
surfaced it. What it decides silently is that a row in the new state
has no row to link to: its message, if it has one, is drawn with
`ui.weak` and the eye is sent nowhere — which is issue 1463's symptom,
the symptom this row exists for, reappearing in the fix for it. And it
is the SAME question `frame.rs` declines to answer below: does a
run-refusal state point somewhere? So the taker's fence spans two
decisions in `features.rs`, one of which the build will not remind
them of. (The wildcard is a class, not a one-off: filed as
`a-wildcard-match-decides-viewer-policy-in-five-places`.)

**The `frame.rs` half is not a mechanical re-spelling, and that is the
finding.** With the new arm added to every match above *including*
that test's own closure and variant list, the test still fails: it
asserts `left_to_the_tree == states`, and a fifth `RowStatus` makes
`states` 4 while the `ProductErrorKind` classes the policy leaves to
the tree stay 3. So `badge_site`'s doc claim — *"`RowStatus` has
exactly three non-`Ok` states … those same three states seen from the
gather"*, a ONE-FOR-ONE pairing — is what the variant breaks. Deciding
whether a run-refusal state pairs with a gather class or is
deliberately unpaired is a design call in `frame.rs`, not a
compile-error fix. Measured: after the mechanical arms, `cargo test -p
viewer --features app --lib -- the_tree_still_has_exactly` fails at
`frame.rs`'s assertion.

**Citations in the sections above, corrected by subject.**
`MateFault::Band` is declared at `crates/editor-core/src/mate.rs`'s
`enum MateFault` (line 752 today, with the `Band` arm at 796, not the
414-418 the first section cites), and `solve_document` / `solve_with_env` — where the refusal is
inserted against every `Node::Mate` and `Node::InstantiatePart` in
`doc.order()` — live in `crates/editor-core/src/mate/solve.rs`, not in
`mate.rs`. The behaviour the 2026-09-15 section describes is exactly
what that code does; only the addresses had moved.

**So what a taker needs is a fence spanning `frame.rs` and
`pane/features.rs`**, which is not a wider effort than this row
already expected — it is the same sentence the row has carried since
2026-09-15. `features.rs` stays on it not for the badge arm, which is
mechanical, but for the link decision twelve lines below it. The
`has_faults` half is done and no longer part of the cost.

## Taken 2026-09-22 (`chrome/badge-attribution`, PR 3090) — the mechanical half landed; the design call is a question

**Landed.** `pane/features.rs`'s link decision in `feature_row` is an
exhaustive `match` over `RowStatus` (`Failed` → its own cause, nowhere
further; `Ok`/`Unevaluated` → no line). A fifth status now fails to
compile at the link decision as well as at the badge match, so the
silent "links nowhere" answer this row named cannot be minted by
adding the variant.

**Not built, because it has several defensible answers with a
user-visible consequence: what does a row reached by
`MateFault::Band` badge?** What exists today, read off the tree
(corrected in the fix pass; the first draft of this section said no
document-level mark existed, which was wrong):

- A loud document mark already exists. `crates/viewer/src/session.rs`
  raises `AtRestBadge::Refused` (`Tone::Actionable`) for ANY gather
  fault on an assembly-shaped document, and every document a Band
  refusal reaches is one — the fault is recorded against instances.
- What that badge lacks is the band's WORDS: its message is
  `AssemblyError::product_refusal` over `ProductError::RootFailed
  { node }`, which reads *"product: root N failed to evaluate (ask
  `Evaluation::node_error` for the typed cause)"* — the gather error
  carries no cause, and `frame::badge_site` leaves `RootFailed` to the
  tree.

The options:

- **(a) Status quo.** Every reached row `FAILED`, `Actionable`, the
  payload's own words, beside a document badge that names a root and
  no cause. N+1 loud marks. `child_band_refusal_rows` pins the rows.
- **(b) Put the cause in the badge that already exists, then quiet the
  rows.** Render the root's `Evaluation::node_error` into the
  at-rest refusal (following `through` for a poisoned root, which
  `node_error` already does in one hop), so the one document mark
  says *"the mate solve could not build a band: …"*; and give the
  reached rows a run-refusal `RowStatus` (e.g. `RunRefused`) toned
  `Advisory`. One loud mark for one cause, at document level — which
  is where the cause lives (the document's tolerance). The badge half
  helps every `RootFailed`, not only Band. `badge_site`'s one-for-one
  pairing is restated (the new state pairs with no gather class) and
  its guard test with it; the link decision answers `None` for the
  new state.
- **(c) The same variant, toned `Actionable` on every row.** Honest
  scope wording ("the run refused, not this node"), no badge change,
  still N+1 loud marks.

Under (b) or (c): `tree::has_faults` must answer `true` (its match is
exhaustive, so it will ask), and `crates/viewer/GUI-DESIGN.md`'s GQ2
sentence ("those rows draw as downstream of the mate the fault names")
gains its Band half.

The lane's recommendation is (b). Its badge half touches
`session.rs`, which is live ground this wave (#3052, #2960, #2961),
which is one more reason it is asked rather than built.
