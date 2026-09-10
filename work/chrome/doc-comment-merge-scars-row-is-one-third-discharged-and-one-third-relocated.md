---
id: doc-comment-merge-scars-row-is-one-third-discharged-and-one-third-relocated
kind: issue
title: app-rs-doc-comment-merge-scars: one scar is fixed and the row's title asserts it is not, one moved out of app.rs into sketch.rs, and one is live where the row says
status: open
opened: 2026-09-10
---


Reached VIEW as a citation report from #2278's census — four `app.rs`
line numbers its diff had shifted. **That report was wrong, and
re-deriving it by subject rather than by delta is what produced this
row.** The numbers were never right on `main`: `perform_batch` sat at
`app.rs:907` before that diff as well as after, and `:1722` held an
unrelated comment about clipped rows in both trees. What the row
actually needs is not four new numbers.

`work/chrome/app-rs-doc-comment-merge-scars.md` is CHROME's, so this is
a report and not a change. Everything below was read at `0349ae49d`.

**The row already did the right thing about numbers** and says so at
`:15-16`: *"Line numbers are as of PR 1776's head; the function names
are the durable anchors."* So its citations drifting is the expected
cost of a convention it declared, not a defect. Its three bullets have
each moved differently since, and that is what has gone unrecorded.

## Bullet 3 is discharged, and the row's TITLE asserts otherwise

The title is *"Three doc-comment merge scars in viewer/src/app.rs leave
apply_status undocumented"*. Both halves of that bullet are fixed:

- `remember_theme`'s doc block opens at `crates/viewer/src/app.rs:999`
  with `/// Write the current theme choice to the preferences store.` —
  its own summary. The three lines describing `apply_status` that the
  row quotes are gone.
- `apply_status` at `:1042` **has a doc comment**, `:1027-1041`, and a
  substantial one: *"This application's door onto [`frame::apply`], for
  the verdict the ranking has ALREADY WEIGHED"*, with a second paragraph
  on why it is not the one place a `StatusUpdate` becomes the field.

`git log -S'Apply a policy verdict to the status line'` names
`35609ed29`, *"VIEW-3 adjudication: the vocabulary/driver split, and the
claims that were wrong"*. So a VIEW unit discharged a CHROME bullet in
passing and neither slate recorded it — which is the fence working in
one direction only.

## Bullet 1 is live but has left the file the row is about

`tip_mark`'s self-spliced summary is still there, verbatim, and it is
now at **`crates/viewer/src/sketch.rs:1015`**:

```
/// **How big the tip marks in a profile preview are**/// **How big the tip marks in a profile preview are**, in sketch-plane
```

`grep -c 'tip marks in a profile preview' crates/viewer/src/app.rs`
returns **0**. The module split moved it out, so a row titled *"three
doc-comment merge scars in viewer/src/app.rs"* covers two.

## Bullet 2 is live, exactly as described

`app.rs:895` is the stranded pre-batch summary
`/// Perform one operation and record what it refused.` sitting
immediately above `:896-897`'s real one, *"Perform one frame's whole
batch of operations, keeping the refusal worth showing."* The row's
reading of it holds: the paragraph below argues this is NOT one
assignment per op, so the stranded line says the opposite of what the
doc argues.

**Worth saying how nearly this was missed.** Reading a window that
started at `:896` shows a clean single summary and reads as discharged;
the defect is one line above it. The row survived a check that would
have closed it wrongly, and the reason is that a window is not a
subject either.

## Shapes

1. **Re-title and re-scope**: two scars, one in `app.rs` and one in
   `sketch.rs`, with the `apply_status` clause removed from the title.
2. **Record bullet 3's discharge and who did it** (`35609ed29`), so the
   row's remaining life is not read as three-thirds live.
3. Both scars are one-line edits; whether they ride with the re-scoping
   or wait is CHROME's call.

## Confidence

`sure` on all three readings — each subject found by name and read at
`0349ae49d`, and `grep -c` for bullet 1's absence from `app.rs` run
rather than inferred. `likely` that `35609ed29` is the commit that
discharged bullet 3, which is a `git log -S` attribution and not a diff
read.
