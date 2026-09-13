---
id: drag-tick-row-cites-app-rs-for-a-finding-that-lives-in-forms-rs
kind: issue
title: drag-tick-has-three-homes cites app.rs throughout for a finding that lives entirely in forms.rs, and its own reproduction command now returns nothing
status: open
opened: 2026-09-10
---


Found by VIEW's `view/chooser-arm` lane (PR 2278) while sweeping every
open row in `work/` against the spans that unit deletes, and re-derived
independently by the VIEW orchestrator before filing. The subject is
CHROME's slate, so this is a report rather than a change.

**The finding in `drag-tick-has-three-homes` is intact. Its address is
not.** Nothing here says the row should close.

## What the row says

`work/chrome/drag-tick-has-three-homes.md:12-14`:

> "How fast does a field of this dimension move" is answered in **three
> places** in `crates/viewer/src/app.rs`, and in one of them the answers
> already disagree.

and then cites `app.rs` for all three homes, at `:1093-1099`, `:1056`,
`:1065`, `:1075`, `:1079` and `:1121-1166`.

## Where each of them actually is

Re-derived at `origin/main` (`104f1445b`) by finding each subject by
name, not by shifting a number:

| the row's home | the row's citation | where it is |
|---|---|---|
| `drag_tick(dimension)` | `app.rs:1093-1099` | `crates/viewer/src/forms.rs:391` |
| `FIELD_DRAG_SPEED` | `app.rs:1056` | `forms.rs:354` |
| `ANGLE_DRAG_SPEED` | `app.rs:1065` | `forms.rs:363` |
| `UNIT_DRAG_SPEED` | `app.rs:1075` | `forms.rs:373` |
| `COUNT_DRAG_SPEED` | `app.rs:1079` | `forms.rs:377` |
| `FieldWriting::of` | `app.rs:1121-1166` | `forms.rs:437`, in the `impl` at `:432` |

**`crates/viewer/src/app.rs` holds none of them.** The module split
moved the whole subject to `forms.rs` and the row was not re-pointed.

## Why this is worse than a set of wrong numbers

**The row's own reproduction command returns nothing.** Its third home
is *"the creation forms, by hand"*, evidenced as
`rg '_DRAG_SPEED,' crates/viewer/src/app.rs` — *"41 lines, of which
~30 are a hand-picked argument"*. That command now matches **zero**
lines. A reader who runs it concludes the finding is stale and closes
the row.

The call sites are all still there, in four files and not one:
`pane/create.rs` (22), `widgets.rs` (17), `forms.rs` (4),
`pane/properties.rs` (2) — 45 in total, by
`git grep -c '_DRAG_SPEED,' -- crates/viewer/src`. So the *"~30
hand-picked"* claim wants re-deriving too, and against a population the
row's framing does not currently describe: the hand-picking is spread
across the pane bodies and the widget vocabulary, which is a stronger
version of the same finding than "three places in one file".

## The coincidence that surfaced it, which is worth recording

`app.rs:1093-1099` and `:1056`/`:1065`/`:1075`/`:1079` on `main` land
inside `ViewerApp::deliver_status`'s doc comment — the exact region PR
2278 deletes. That is why a span-collision sweep found this row at all.
The overlap is arithmetic coincidence and nothing more: these citations
were already pointing at unrelated text before that PR existed, and the
numbers move again once it lands. **A sweep for one thing found a
different, larger thing, which is the argument for running span sweeps
over other programs' rows at all** even though the fix never crosses
the fence.

## Shapes

1. **Re-point the row by subject** — the six citations above, plus the
   reproduction command and the *"three places in `app.rs`"* framing,
   which is the sentence that has to change rather than a number.
2. **Re-derive the third home's population while re-pointing it**, since
   `~30 of 41 in app.rs` is now `45 across four files` and the row's
   argument gets stronger, not weaker.

## Confidence

`sure` on every location in the table and on the reproduction command
returning nothing; each was read at `104f1445b`. `likely` on the
41 → 45 reading, which is a `grep -c` of `_DRAG_SPEED,` and not a
count of hand-picked arguments — the row's own distinction between a
call-site argument and any other mention is what would settle it.
