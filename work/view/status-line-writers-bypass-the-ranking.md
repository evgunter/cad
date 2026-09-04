---
id: status-line-writers-bypass-the-ranking
kind: issue
title: Eighteen writers assign the status line directly, so the frame's ranking decides nothing they say
status: open
opened: 2026-09-04
refs: [camera-fold-clears-status-line]
---

## What this is

`frame::frame_status` ranks a frame's news — refusal, then every
notice the frame produced, then the batch's own verdict — and
`frame::apply` is the one door a `StatusUpdate` becomes the field
through. **Eighteen sites assign `status` without asking either** —
nineteen at this branch's merge base, one of which
`camera-fold-clears-status-line` removed by giving the product fault a
badge.

Each is the same shape as the camera fold that item fixed: a writer
with no way to say "I have nothing to add", whose sentence is then
kept or lost by whichever writer in the frame happens to run last.

Two of them run in `sync_scene`, BEFORE the frame's batch is
performed, so a clean acting batch's `StatusUpdate::Clear` erases them
exactly as `land` used to — the defect `frame_status` was extracted to
stop for tool notices, still live for everything else.

Censused with `grep -rn "status = " crates/viewer/src/`, excluding
`src/frame.rs` (where `apply`'s own arms live) and the two test-local
`let mut status` bindings in `pane/viewport.rs`'s rows, which are not
writers. Line numbers are as of the head of `view/status-lifetimes`.
**What that pattern cannot match**: a writer that mutates the line
through a helper, a `*self.status = …` split across lines by rustfmt,
and any writer that reaches the line through `ViewerBehavior`'s field
under another binding.

## The hit list, classified

**News** — belongs on the line, but must reach it through
`frame_status`'s ranking; today it does not, so a same-frame batch or
tool notice silently outranks it or is outranked by it:

- `crates/viewer/src/app.rs:697` — a δ that `DisplayTolerance` refused.
- `crates/viewer/src/app.rs:860` — the preferences store could not be
  written. A save was attempted and failed: the outcome of an act.
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
- `crates/viewer/src/app.rs:674` — `scene: {error}`. Same lifetime:
  nothing retries until the landed pair or the display state moves.
- `crates/viewer/src/pane/viewport.rs:324` — `projection: {error}`.
  True on every frame until the camera moves, and the arm re-writes it
  on every one of them while early-returning out of the paint.

## Why it was not swept with the fold

`crates/viewer/src/*` is shared with the CHROME program, which has
items newly unblocked in `create.rs` and `viewport.rs`; an
eighteen-site sweep landing beside them is a merge conflict bought for
nothing. The vocabulary the sweep needs now exists — `frame::apply`,
`frame::fold_status`, `frame::product_badge` — so the work is
mechanical per site once the territory is free.

## What a fix looks like

Each news site pushes onto the frame's `notices` rather than
assigning — the door `Tools::reconcile` and `Tools::feed` already use,
which puts it in `frame_status`'s rank 2 and stops the same frame's
batch verdict from erasing it. `pane` modules cannot reach `notices`
today (`ViewerBehavior` carries `status`, not `notices`), so that
field moves with the sweep. Each standing fact gets a badge function
in `frame` beside `product_badge` and a toolbar read beside the
at-rest, checks and budget badges. The pane sites are the bulk and are
independent of each other, so this splits cleanly.
