---
id: status-line-writers-bypass-the-ranking
kind: issue
title: Nineteen writers reach the status line without the frame's ranking, so it decides nothing they say
status: open
opened: 2026-09-04
refs: [camera-fold-clears-status-line, the-news-vocabulary-has-no-expiry, stale-file-citations-after-the-split]
---

## What this is

`frame::frame_status` ranks a frame's news — refusal, then every notice
the frame produced, then the batch's own verdict — and `frame::apply`
is the one door a `StatusUpdate` becomes the field through. **Nineteen
writers reach the field without asking the ranking.**

Each is the shape `camera-fold-clears-status-line` fixed at one site: a
writer with no way to say "I have nothing to add", whose sentence is
then kept or lost by whichever writer in the frame happens to run last.
`perform_batch` runs AFTER the panes have drawn
(`crates/viewer/src/app.rs`, the frame loop), so a message a pane wrote
this frame is erased by that frame's `StatusUpdate::Clear` before it is
ever painted — the defect `frame_status` was extracted to stop for tool
notices, still live for everything else.

## The arithmetic

`grep -rn "status = " crates/viewer/src/` matched 22 lines at this
branch's merge base. Two are `apply_status`'s own arms, which are the
door and not writers, leaving **20**. This branch removed two:
`app.rs`'s product-fault assignment, which became a badge, and `land`'s
own unconditional assignment, which became `frame::apply` +
`frame::fold_status`. **18 matched writers remain**, plus one the grep
could not match (below), for **19**.

**What the pattern cannot match**, restated from what it missed: a
**struct-literal field initializer** (`status:` with a colon, not
`status =`) — that is how `app.rs:580` was missed; a writer that
mutates the line through a helper; a `*self.status = …` split across
lines by rustfmt; and any writer reaching the field under another
binding.

## The hit list, classified

**News** — belongs on the line, but must reach it through
`frame_status`'s ranking:

- `crates/viewer/src/app.rs:697` — a δ that `DisplayTolerance` refused.
- `crates/viewer/src/app.rs:860` — the preferences store could not be
  written. The outcome of an act.
- `crates/viewer/src/pane/view.rs:93` — the δ field's text is not a
  number.
- `crates/viewer/src/pane/create.rs:162,167` — the mate tool's refusal,
  and its no-landed-evaluation arm.
- `crates/viewer/src/pane/create.rs:375` — add datum refused.
- `crates/viewer/src/pane/create.rs:600,601` — add profile: no frame
  picked, and the placement refusal.
- `crates/viewer/src/pane/create.rs:808` — extrude refused.
- `crates/viewer/src/pane/create.rs:1139,1171,1176,1217` — the blend
  tool's event wording, its two refusal arms, and the seated tools'
  shared one.
- `crates/viewer/src/pane/viewport.rs:172` — a cursor action the pick
  index refused.
- `crates/viewer/src/pane/viewport.rs:363` — the two picking paths
  disagree. Already a frame product (`frame::disagreement`); only its
  delivery bypasses the ranking.

**Standing facts** — still true after the frame ends, so per
`crates/viewer/src/frame.rs`'s header they want a badge, not the line:

- `crates/viewer/src/app.rs:629` — `pick index: {error}`. The cache
  holds a refused build (one attempt per landed generation and δ), so
  the picture on screen is stale for exactly as long as this stands.
- `crates/viewer/src/app.rs:674` — `scene: {error}`. Same lifetime.
- `crates/viewer/src/pane/viewport.rs:324` — `projection: {error}`.
  True on every frame until the camera moves, and re-written on every
  one of them while early-returning out of the paint.
- `crates/viewer/src/app.rs:580` — **the writer the grep missed.** The
  startup preferences notices, written as a struct-literal
  initializer. "Your preferences file has a key I do not understand"
  is a standing fact about the file; it sits on the news line, where
  the first acting batch of the session silently deletes it.

**A policy that reaches the field without the ranking**, which is
this item's own module being one of its subjects:

- `frame::fold_status`, applied at `crates/viewer/src/pane/viewport.rs`'s
  `land`. It answers in the right vocabulary and goes through
  `frame::apply`, so it is not an assignment — but it does not go
  through `frame_status`, so a camera refusal raised in a frame that
  also carries a clean acting op is overwritten by that batch's
  `Clear` before it is painted. Fixing it is the same work as the rest
  of this item and not a separate change.

## Why it was not swept with the fold

`crates/viewer/src/pane/` is CHROME-adjacent: CHROME's items cite
`crates/viewer/src/app.rs` (e.g. `work/chrome/drag-tick-has-three-homes.md`),
and the 1c split moved that code into `pane/*`, so CHROME owns items
over code now living in these files even though its citations still
name the old path (`stale-file-citations-after-the-split`). A
nineteen-site sweep landing there is a merge conflict bought for
nothing.

## What a fix looks like

Each news site pushes onto the frame's `notices` rather than assigning
— the door `Tools::reconcile` and `Tools::feed` already use, which puts
it in `frame_status`'s rank 2 and stops the same frame's batch verdict
from erasing it. **`pane` modules cannot reach `notices` today**:
`ViewerBehavior` carries `status` and not `notices`, so that field
moves with the sweep. Each standing fact gets a badge function in
`frame` beside `product_badge` and a toolbar read beside the existing
badges — see `four-badges-five-spellings` for what that family should
look like before four more members are added to it. The pane sites are
the bulk and are independent of each other, so this splits cleanly.
