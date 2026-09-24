---
id: viewer-preview-names-a-verb-by-its-variant-identifier
kind: issue
title: the profile preview names a Verb by its variant identifier, the class PR 2053 enforced
status: closed
opened: 2026-09-06
refs: [2347, 2053]
closed: 2026-09-19
---


Found by the style review of PR 2053 (`view/refusal-all`), whose whole
subject is this class.

## The instance

`crates/viewer/src/sketch.rs:663`, inside `impl Display for
PreviewError`:

    "loop {loop_} step {step}: {verb:?} is not well-typed there — the tip is {}",
    tip_state_words(*state)

`verb` is `profile::path::Verb`, a fieldless enum, so `{verb:?}` writes
a bare variant identifier (`LineTo`) into a sentence a person reads.
`crates/viewer/src/pane/create.rs:582-586` renders that string into the
create pane with `ui.colored_label`, so it is on screen, not a log.

The same `write!` names the tip's state through `tip_state_words`
(`crates/viewer/src/sketch.rs:826`) — prose — and the verb through
`Debug`. One sentence, two policies, adjacent.

## Why this is filed against 2053 rather than found by it

PR 2053 fixed four sites of exactly this shape (a value whose prose
rendering exists, reaching a user through `Debug`) and its sweep table
is scoped to *"a `Dimension` reaching user-visible text through
`Debug`, across `crates/viewer/src/`"*. That pattern's blind spot is
the type, not the tree: the defect class is *any* kind rendered as its
identifier, and this instance sits in the swept directory.

It was also already on the board.
`work/fix/verb-and-dimension-render-through-debug.md` (open, FIX's
slate) names this exact site under "`profile::path::Verb` has no
`Display`" — the item cites `sketch.rs:651`, which is this `write!`
before the file moved. Its second half, "`Dimension` renders through
`Debug` in four UI labels", lists the four sites 2053 fixed (at their
pre-split paths `app.rs:2832`, `app.rs:4339`, `app.rs:4362`,
`session.rs:750`). So the tracker held the complete hit list and the
sweep did not consult it.

## What to do with it

The fix `verb-and-dimension-render-through-debug` proposes is an
`impl Display for Verb` in `profile` (macro-generated declaration, so
the impl goes in the macro beside `Verb::ALL` or is written out) and a
forward at this site. The viewer half of that is this program's; the
`profile` half is not. Whoever takes either should also close out that
item's `Dimension` section, which PR 2053 completed without saying so.

## The FIX item's two halves, as they now stand (PR 2053)

Recorded here so the VIEW side points at the FIX side rather than
leaving a reader to re-derive it.

`work/fix/verb-and-dimension-render-through-debug.md` is **open** and
holds two halves. PR 2053 completed one of them without knowing the
item existed:

- **`Dimension` renders through `Debug` in four UI labels` — DONE by
  PR 2053.** The item lists them at pre-split paths `app.rs:2832`,
  `app.rs:4339`, `app.rs:4362` and `session.rs:750`; they are now
  `crates/viewer/src/pane/properties.rs` (the parameter header and the
  two dimension tags) and `crates/viewer/src/session/refuse.rs`
  (`Refusal::exists_wording`). All four forward to `Dimension`'s
  `Display`. The item's one stated check — that the `Display` reads
  correctly inside a parenthetical, where three of the four sit — holds:
  the sentence is *"parameter width already exists (length) — edit it
  instead?"*.
- **`profile::path::Verb` has no `Display`` — UNTOUCHED, and this row
  is its viewer half.** PR 2053 deliberately did not fix
  `crates/viewer/src/sketch.rs:663`. The fix FIX's item specifies is
  `impl Display for Verb` in `crates/profile`, which is not this
  program's territory, and forwarding the verb in prose from the viewer
  instead would mint a fourth spelling of the word list — the defect
  `work/issues/the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused.md`
  is about.

So `verb-and-dimension-render-through-debug` should not be closed on
its `Dimension` half alone, and whoever takes its `Verb` half takes
this row with it. Neither `work/fix/` nor that item was edited by
PR 2053: a unit branch does not file or close on another program's
slate (`docs/prompts/implementer-discipline.md`, §6).

## Closed 2026-09-19 — already discharged, and not by this program

Re-deriving this row's PREMISE at VNEWS' first dispatch: **there is no
`{verb:?}` anywhere in `crates/viewer/src`, and the fix this row
specifies has already landed.**

- `profile::path::Verb` has a `Display` — `crates/profile/src/path/program.rs`,
  inside the macro that also declares `Verb::ALL`, which is the shape
  `work/fix/verb-and-dimension-render-through-debug` proposed.
- `PreviewError`'s `Display` arm in `crates/viewer/src/sketch.rs` now
  reads `"loop {loop_} step {step}: {verb} is not well-typed there — the
  tip is {}"` — `{verb}`, forwarding to that `Display`, beside
  `tip_state_words(*state)`. The two policies the row objected to
  sharing one sentence are now one policy.

**Who closed it.** `work/fix/verb-and-dimension-render-through-debug`
(FIX, PR 2347, closed 2026-09-11) took both halves. That item's own
closing note records the consequence this row was never told about:
*"`prose_census.rs`'s `UNDECIDED` named `crates/viewer/src/sketch.rs` /
`PreviewError` / `verb`; that site is gone, so the entry is deleted."*
So the row was discharged eight days ago and travelled through VIEW's
re-scope into this program still reading as open.

**Two stale citations in this row, left rather than repointed**, per the
register's rule that a citation whose subject has moved is disclosed and
not shifted onto whatever now sits at the number: `sketch.rs:663` is not
the `write!` (the file moved again after the row was filed), and
`work/issues/the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused.md`
is now `work/census/…`, live on CENSUS' slate and untouched by this.

**What this cost, said plainly.** VNEWS' §Order correction of earlier
today said this row "needs `impl Display for Verb` in `crates/profile`,
which is PATHS' territory" and routed it accordingly. That was wrong:
the orchestrator re-derived the row's citation and not its premise,
which is the half of the register's rule that matters more, and is the
same rule handed to every lane dispatched this morning.

## Reference note (FIX's sweep, 2026-09-21)

`verb-and-dimension-render-through-debug` was dropped from this row's `refs` because the row closed with **FIX**, which left the tracker at sweep 18 — `work/fix/` is deleted and `docs/DOC-LEDGER.md` is its done-state of record. The finding is unchanged and still readable: `git show 6f0e04ce1534:work/fix/verb-and-dimension-render-through-debug.md`.
