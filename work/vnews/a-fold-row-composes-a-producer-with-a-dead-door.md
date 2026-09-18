---
id: a-fold-row-composes-a-producer-with-a-dead-door
kind: issue
title: Two frame.rs rows compose fold_status with apply over a composition no caller performs
status: open
opened: 2026-09-06
refs: [status-line-writers-bypass-the-ranking, ranked-and-unranked-verdicts-are-one-type, 2026]
---

Found by the class sweep #2026's style review asked for, after fixing
the first instance it named.

## The sweep, in full

**Every row in `crates/viewer/src/frame.rs` that composes a `*_status`
producer with `frame::apply`** — five rows, six compositions — and
whether each still mirrors a call some production path makes:

| Row | Composition | Live? |
|---|---|---|
| `a_clean_fold_keeps_a_message_it_did_not_write` | `apply(status, fold_status(clean))` | **yes** — `land` → `deliver` → `apply` for the retiring half |
| `a_clean_fold_retires_the_camera_refusal_it_did_write` | `apply(status, fold_status(refused))` | **no** | 
| " (second half) | `apply(status, fold_status(clean))` | **yes** |
| `a_cursor_that_has_not_moved_retires_nothing` | `apply(status, cursor_status(step))` ×2 | **yes** — `pane/viewport.rs:178` |
| `a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it` | `apply(status, fold_status(refused))` | **no** — fixed in #2026's fix pass, see below |
| `a_supersession_survives_the_accepted_edit_that_caused_it` | `apply(status, frame_status(...))` | **yes** — `perform_batch` → `apply_status` |

Two of the six are dead, and they are the same composition:
`apply(status, fold_status(refused))`. Since #2026 a refused fold
reaches the line through `frame::deliver`, which pushes it onto the
frame's notices; no caller hands a `fold_status` `Show` to `apply`.

## The one still standing

`a_clean_fold_retires_the_camera_refusal_it_did_write` uses the dead
composition to SET UP its fixture — it puts a camera refusal on the
line so the clean fold below has something to retire — and then asserts
the retirement, which is live. So:

- **Nothing it asserts is false.** `apply`'s contract is unchanged, and
  the row's subject (a clean fold retires the camera sentence) is a
  real property of the real path.
- **Its name does not overclaim**, unlike the row the review caught,
  whose name advertised an outranking the tree no longer performs.
- **What it loses is the end-to-end property.** The live path puts the
  refusal on the line through `frame_status`'s rank 2, and that step is
  where the retirement can break: `joined_subject` answers `Document`
  for a frame carrying two disagreeing notices, and an
  `Expire(Camera)` against a `Document` line retires nothing
  (`one-line-one-subject-loses-a-mixed-frames-expiry`). A row that
  hand-places the message cannot see that, and it is a `frame.rs` unit
  row, so arguably should not — `pane/viewport.rs`'s
  `landing_a_clean_fold_retires_the_camera_refusal_it_landed_before`
  is the row on the live path, and #2026's fix pass put it there.

## Why it is filed rather than fixed

The fix is one line (`apply(&mut status, frame_status(&[msg], &[], None))`,
or hand-building the `Message`) and the question it raises is not: **is
a unit row entitled to set up its fixture with a composition production
does not perform?** Sometimes obviously yes — the alternative is every
row dragging the whole frame loop in. The discriminator the review used
on the first instance was the NAME: a row is wrong when its name claims
the composition is the behaviour. That is a useful rule and nothing
states it anywhere.

Worth answering once for the crate rather than per row, because the
same shape will recur every time a door is added beside an existing one
— which `ranked-and-unranked-verdicts-are-one-type` proposes to stop
happening.

