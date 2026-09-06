---
id: viewer-preview-names-a-verb-by-its-variant-identifier
kind: issue
title: the profile preview names a Verb by its variant identifier, the class PR 2053 enforced
status: open
opened: 2026-09-06
refs: [2053]
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
`crates/viewer/src/pane/create.rs:581-585` renders that string into the
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
