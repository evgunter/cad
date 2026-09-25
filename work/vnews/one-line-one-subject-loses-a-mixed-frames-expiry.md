---
id: one-line-one-subject-loses-a-mixed-frames-expiry
kind: issue
title: One line holds one subject, so a mixed frame's rank-2 join loses the finer expiry
status: open
opened: 2026-09-05
refs: [2026]
priority: P3
cost: E
---


## What this is

`frame::frame_status`'s rank 2 joins a frame's notices into ONE
`Message`, and a `Message` carries one `Subject`. When the notices
disagree about their subject, `frame::joined_subject` falls back to
`Subject::Document` — which has no `Expire` issuer, so the joined line
is swept only by an act the document accepts and the finer expiry is
lost.

Concretely: a frame carrying a picking disagreement (`Subject::Cursor`,
which `frame::cursor_status` would retire on the next cursor move) and
a tool notice (`Subject::Document`) produces a line that survives every
cursor move until the next accepted edit.

## It IS reachable, as of #2026 (was: "why it is not reachable yet")

The heading this section carried was true when the item was filed and
is false now; it is corrected rather than left, because a resolving
heading reads as a checked one.

Every notice a frame produced was document-provoked, so the fallback
arm had never fired in production. `status-line-writers-bypass-the-
ranking` routes `crates/viewer/src/pane/viewport.rs`'s disagreement
into the frame's notices, and from that unit onward a frame that
carries a disagreement *and* a tool notice hits it.

The arm itself is asserted —
`frame_policy::a_joined_line_keeps_a_shared_subject_and_falls_back_when_they_differ`
pins both the fallback and the fact that the fallback survives a cursor
move — so the behaviour is stated, not silent. What is unresolved is
whether it is the behaviour anyone wants.

## The fork

1. **Keep the fallback.** A conservative line that says too much for
   too long is cheaper than one that deletes a clause a reader had not
   read. Cheapest, and the status quo.
2. **The line holds one message PER SUBJECT** rather than one message.
   Expiry then reaches exactly its own clause and nothing else, and the
   join happens at paint. It is the shape the vocabulary wants — but it
   contradicts rank 1's *"a refusal wins, alone"*, which is a
   statement about the whole line, so it is a design change and not a
   refactor. (*"Ratified"* was struck from this sentence on
   2026-09-20; see the adjudication section at the foot of this file.)
3. **Rank 2 refuses to join across subjects** and shows only the
   highest-ranked subject's notices. Needs a rank among subjects, which
   nothing has argued for.

## What #2026 actually landed, and the arm the item did not predict

The unit landed, and the reachability is wider than the sentence above
predicted. `crates/viewer/src/frame.rs`'s `joined_subject` now sees
five subjects on the notices list where it saw one:

| subject | who puts it there |
|---|---|
| `Document` | ten `tool_news` sites in `pane/create.rs`, `pick_refusal`, the two `Withdrawal` notices |
| `Display` | `unindexed_refusal` (`PICK_INDEX_SEAM`), `delta_refusal` and `delta_not_a_number` (`SCENE_SEAM`) |
| `Cursor` | `Disagreement::notice` — the arm this item named |
| `Camera` | **`fold_status`'s refused fold**, delivered through `frame::deliver` at `pane::viewport::land` |
| `Preferences` | `store_refusal` |

**The CAMERA arm is the one this item did not name, and it is the worse
one.** The item's example is Cursor + Document; a camera refusal and a
tool notice in one frame is a plainer composition than that — a drag
that dollies past a limit while a create-pane form refuses is one
gesture — and its consequence is sharper. The joined line comes out as
`Subject::Document`, and `fold_status`'s next `Expire(Camera)` on the
following clean fold then retires nothing, because the line is no
longer about the camera. So the camera refusal survives every
subsequent navigation until an act the document accepts sweeps it —
which is `camera-fold-clears-status-line`'s exact defect, reappearing
through the join rather than through `land`.

That is the defect `frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER`
(`crates/viewer/src/frame.rs`) exists to make assertable: `Camera` and
`Cursor` are on that list precisely because a message wearing either is
supposed to be retired by an event, and the join is what silently takes
them off it. `Document` has no issuer, so the fallback does not merely
coarsen the expiry — for those two subjects it removes it.

**Not fixed by #2026 and deliberately not**: the fix is the three-way
fork this item already states, and every arm of it is a design change
rather than a refactor. What that unit did was
turn the fallback from an unreachable arm into a live one and record it
here. `crates/viewer/src/frame.rs`'s `joined_subject` doc says the same
thing at the code, and points here.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### Nothing here waits on Ev, and one sentence in the source says otherwise

This row's arm 2 was argued against *"rank 1's ratified 'a refusal
wins, alone'"*. **Rank 1 is not ratified.** The finding, and every
search that establishes it, is in
`work/vnews/rank-one-discards-the-frames-other-news`'s adjudication
section, which is its canonical home — cited, not restated, because a
search copied into six rows is six places to re-derive when one of them
moves.

So **every arm of this fork is this program's to decide.** Two further
corrections to how the fork is framed:

- **Arm 1 is not a design change**; it is the status quo, and choosing
  it costs one sentence recording the choice. The row's later claim
  that *"every arm of it is a design change rather than a refactor"* is
  false of arm 1 on the row's own account of it.
- **`crates/viewer/src/frame.rs:656-658` claims this question is for
  Ev** — *"it needs a value that carries several subjects, which
  `work/view/one-line-one-subject-loses-a-mixed-frames-expiry.md` owns
  and which is a design question for Ev"*. That sentence was written by
  `30443532b` (2026-09-15, author Claude) and asserts a gate that does
  not exist. It is false twice over, because the path it names is also
  stale — this row moved to `work/vnews/` in the 2026-09-17 re-scope.
  The stale half is already filed as
  `work/vdoc/crates-cite-work-view-rows-that-moved-in-the-rescope`
  (its table names `crates/viewer/src/frame.rs:657` and `:704`); the
  **false-gate half is new and is this program's**, because `frame.rs`
  is this program's file and the claim is about this row.

### Arm 2 does not deliver `rank-one-discards-the-frames-other-news`

The plan groups the two rows as *"the discards … one conversation"*.
Checked against the tree, they are adjacent and independent, and arm 2
is not the other row's fix:

- `batch_status` spells a refusal `Subject::Document`
  (`crates/viewer/src/frame.rs:583-585`).
- `Withdrawal::superseded`'s notice is also `Subject::Document`,
  asserted at `crates/viewer/src/frame.rs:2502-2507`.

A line holding one message PER SUBJECT therefore still holds **one**
message for that pair, and the supersession is still dropped. What
arm 2 fixes is the `Camera`/`Cursor` loss this row is actually about.
The other row's fix is a rule about which rank-2 messages ride
alongside rank 1, which is a different edit at a different line.

They do interlock in one direction, and it is the ordering argument:
a rank-2 list that already has to carry a second message beside a
refusal is the shape arm 2 wants, so taking
`rank-one-discards-the-frames-other-news` first makes arm 2 cheaper,
while taking this row first does nothing for that one.

### Citations re-derived against the tree (2026-09-20)

- `joined_subject` is `crates/viewer/src/frame.rs:707-713`; its doc
  comment, including the five-subject census this row's table
  duplicates, is `:672-706`.
- `frame::SUBJECTS_WITH_AN_EXPIRY_ISSUER` is
  `crates/viewer/src/frame.rs:334`.
- `frame_policy::a_joined_line_keeps_a_shared_subject_and_falls_back_when_they_differ`
  is `crates/viewer/tests/frame_policy.rs:891`.
