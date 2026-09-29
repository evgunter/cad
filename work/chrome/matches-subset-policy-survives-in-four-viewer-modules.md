---
id: matches-subset-policy-survives-in-four-viewer-modules
kind: issue
title: A matches! over an enum subset stands in for policy at 21 sites in eight more viewer modules, and none of them can red when the enum grows
status: closed
opened: 2026-09-22
priority: P2
cost: E
refs: [has-faults-cannot-red-on-a-new-rowstatus]
closed: 2026-09-24
branch: chrome/subset-policy
pr: 3140
---

## Finding

Filed by the `chrome/rowstatus-exhaustive` lane, from the sweep
`has-faults-cannot-red-on-a-new-rowstatus` asks for. That row's own
point is that the ORIGINATING sweep's criterion — *"a multi-arm
`matches!` over a subset of an EXTERNAL enum"* — had a word in it
doing no work. Swept for the shape instead: **a `matches!` over a
subset of any enum, standing in for a policy, where the excluded
states are a decision stated only in prose.**

`tree::has_faults` is fixed on that lane. **Twenty-one more sites, in
eight modules** under `crates/viewer/src`, are outside that lane's
fence. (The first draft of this row said *"Six more sites, in four
modules"* over a list of seven; the arithmetic was wrong and so was
the criterion that produced the list — see *The criterion, corrected*
below, which is what takes it to 21.)

- **`bounds.rs`, `Verdict::of`** — `matches!(result,
  NodeResult::Failed(_))` over `NodeResult` (`Ok`, `Failed`,
  `Poisoned`). The strongest hit: it is `has_faults`'s defect exactly,
  one enum up. The doc directly above it argues the `Poisoned`
  exclusion at length (*"counting it would make one failure register
  as many"*) — so the policy is already written out, and only the
  construct is silent. A fourth `NodeResult` is silently not a
  failure, and the verdict every value-comparison is made on gets it
  wrong with nothing saying so. Cheapest of these to fix: the arms
  and their reasons are already there.
- **`session/refuse.rs`, the `NodeKindWanted::Frame` arm** —
  `matches!(held, Some(Node::Datum(Datum::Frame { .. } |
  Datum::FaceFrame { .. })))`, with a comment saying why both frame
  kinds count. The outer `match` over `NodeKindWanted` is exhaustive;
  the inner subset over `Datum` is not, so a new datum flavour a
  picker should accept is silently refused.
- **`session/refuse.rs`, the body predicate** — `matches!(payload,
  ValuePayload::Body(_) | ValuePayload::Boolean(BooleanValue::Body
  { .. }))`. A viewer policy (*does this value carry a body*) over a
  kernel enum; a new payload that carries one is silently bodiless.
- **`sketch.rs`, twice** — the same `Datum::Frame { .. } |
  Datum::FaceFrame { .. }` subset, asking which nodes a sketch may be
  drawn on. Two sites, and a third spelling of the same question as
  `refuse.rs`'s first hit, which is its own smaller finding.
- **`tools.rs`, `ToolKind::commits`, all six arms** — *did this tool's
  edit land*, asked of `SessionOp`. The outer `match` over `ToolKind`
  is exhaustive; every `SessionOp` subset inside it is not, so a new op
  a tool commits leaves the tool open after its own edit landed.

## The criterion, corrected — and it is the same correction twice

This row's first draft filed `tools.rs`'s **two multi-op arms** and
left the four single-op siblings **in the same `match`** unmentioned,
and did the same at `refuse.rs`. The word doing no work this time was
*multi-arm*.

`matches!(op, SessionOp::AddRevolve { .. })` inside
`ToolKind::commits` is not asking *is this value an `AddRevolve`*; it
is answering *did the revolve tool's edit land*, over an enum of ~40
ops, and it is silent in exactly the way its two-op neighbours are. A
single-variant pattern is IDENTITY only when the question is the
variant — `Bound::is_edge`, `Polarity::Dark`. When the question is a
policy the pattern stands in for, the alternation bar is incidental.
The row already knew this and had not said it: its own strongest hit,
`bounds.rs`'s `Verdict::of`, is single-arm.

So the criterion is: **a `matches!` whose scrutinee is an enum, where
the variants it does not name are a decision — not the answer to "is
this value that one thing".** Under it:

- `session/refuse.rs`, `admits` — **five** arms, not one. `Profile`,
  `Axis`, `SketchAxis` and `Plane` map a `NodeKindWanted` onto a
  `Node`/`Datum` subset exactly as `Frame` does.
- `tools.rs`, `commits` — **six** arms, not two.
- **`matetool.rs:501` + `session/select.rs:312` + `:331`, the
  `Resolution::Resolved` trio** — *is this pick still good*, spelled
  three times over one kernel enum: a lost pick, a live selection and
  an unresolved verdict. A `Resolution` variant that resolves
  differently is silently a lost pick AND not live AND not unresolved.
  Three spellings of one policy is the sharper half of this hit.
- **`sketch.rs:998` + `pane/profile.rs:182`, the `Transition { verb:
  None }` pair** — *is this refusal the benign not-yet-closed state
  rather than something a step's author did*, asked once of
  `ReplayErrorKind` and once of `PreviewError`. Two enums, one
  question, two spellings; a new kind meaning the same thing is
  coloured as a refusal in the pane and re-refused in the replay.
- **`frame.rs:572`, `acts`** — `!matches!(op, SessionOp::Hover(_))`,
  the same silence with the sign flipped: a new op is silently an
  ACTION on the document, and the doc argues at length why exactly one
  op is not. A complement is a subset.

## The population, so the negative result is a receipt

`grep -rn 'matches!' crates/viewer/src` is 54 lines, one of which is
the word inside `tree::has_faults`'s new doc: **53 sites**. Twelve are
inside `#[cfg(test)]` modules. Of the remaining **41**, this row files
**21** (the list above) and excludes **20**:

- **Identity** — the question IS the variant, and a new variant
  correctly answers no: `app.rs:1118`, `app.rs:1186`, `app.rs:2188`,
  `datums.rs:708` (a whole variant), `sketch.rs:1102`,
  `bounds.rs:227`, `pickcache.rs:490`, `props.rs:407`,
  `display.rs:398`, `pane/properties.rs:154`, `pane/viewport.rs:510`,
  `frame.rs:1037`, `frame.rs:2133`, `session.rs:2391`,
  `session.rs:2793`, `platform.rs:66`.
- **Not an enum** — `Ok(None)` / `Some(Ok(_))` shapes
  (`pane/create.rs:507`, `drafts.rs:659`) and the two macro bodies in
  `vocab.rs`, whose pattern is the caller's.

The twelve test-module sites are excluded on the ground this row
already used for `pane/viewport.rs`: an assertion naming the one state
it wants is not a policy over a set.

## What the sweep could not match

The pattern is textual and finds `matches!` only. It cannot see the
same policy written as an `if let` / `let … else` chain, as a `match`
with a `_ =>` wildcard, or as a helper predicate whose own body is the
subset (a `foo.is_bar()` reads as one token at the call site). A
second pass over every `matches!` in `crates/viewer/src` closed the
sub-gap that the first pattern required a `|` alternation, and a third
found no policy spelled as two `matches!` joined by `||`.

**The wildcard-`match` gap is swept and has its own row**:
`a-wildcard-match-decides-viewer-policy-in-five-places`, which carries
the 41-hit list and its disposition. It is a separate file, not a
section here, because closing THIS row by fixing the sites above would
otherwise close it with the gap still open — the shape
`work/meta/a-stated-sweep-blind-spot-is-never-swept` exists for. The
helper-predicate gap and the `if let` spelling are that row's
remaining half, with the one worked instance found so far recorded
there.

An earlier draft of this section declared the wildcard gap unsearched
on the ground that `-D clippy::wildcard_enum_match_arm` was the
instrument and running it was a unit of its own. That was wrong twice
over: `grep -rn '^\s*_ =>' crates/viewer/src` is 41 hits and a second
of work, and it turned up a live site inside `pane/features.rs` —
inside the very set this lane had just measured by hand.

## A shape for whoever opens that impl next

`tree::has_faults` is a free function over `&[TreeRow]` while `badge`,
`tone` and `message` are `RowStatus` methods, so the four readings of
one enum do not sit together and a fifth variant's author meets three
of them. `RowStatus::is_fault(&self)` beside the other three, with
`has_faults` left as the `rows.iter().any(…)` over it, is the tidier
shape.

**Declined on `chrome/rowstatus-exhaustive`, deliberately**: the
reviewer who raised it was explicitly unsure, it churns roughly eleven
call sites for a taste improvement, and it was raised on a PR that was
already green. Recorded here rather than acted on so the next lane
with that impl open for another reason can take it for free.

## Closed 2026-09-24 (`chrome/subset-policy`)

All 21 sites are exhaustive `match`es, plus `frame::retype_draft`,
which this row counted as identity and which is the same "which
refusals" question `creation_offer` asks over `Refusal`. Where one
decision was spelled in several places, it now has one home and the
places call it:

- the `admits` five and `sketch.rs`'s two frame subsets →
  `session::refuse::seat_kind`, behind `admits`, which `sketch::frames`
  and `sketch::frame_placement` now call;
- the `Resolution::Resolved` trio → `session::select::resolves`;
- the `Transition { verb: None }` pair → `sketch::PreviewError::unfinished`.
  The preview's retry asks it of the converted refusal rather than of
  `ReplayErrorKind`, so the two enums are one question asked once;
- `ToolKind::commits`' six → `commits` still matches every tool, and
  reads `tools::committed_by`, one exhaustive map from `SessionOp` to
  the tool it closes. A new tool and a new op each red; the first draft
  held only the op side, and review caught it;
- `sketch::committed` and `pane::properties`' profile check now ask
  `admits(…, NodeKindWanted::Profile)` rather than spelling the subset;
- `bounds::Verdict::of`, `refuse::is_one_body` and `frame::acts` are
  converted in place.

The census re-taken at the merge base (54 `matches!` lines: 22 fixed,
32 left, each with its reason) is in the PR body, along with the
second-pass sweep.

**The `RowStatus::is_fault` shape** recorded above was not taken. This
lane did not open `RowStatus`'s impl for another reason, and the ~11
call-site churn is unchanged. It stays a taste call for whoever does.
