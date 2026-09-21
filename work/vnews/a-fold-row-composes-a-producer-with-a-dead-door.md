---
id: a-fold-row-composes-a-producer-with-a-dead-door
kind: issue
title: Two frame.rs rows compose fold_status with apply over a composition no caller performs
status: open
opened: 2026-09-06
refs: [status-line-writers-bypass-the-ranking, ranked-and-unranked-verdicts-are-one-type, 2026]
priority: P3
cost: E
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

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`): a negative result on the question, and a rider on the residue

### The sweep reproduces

Re-derived by the stated rule — every row in
`crates/viewer/src/frame.rs` that composes a `*_status` producer with
`frame::apply`. **Five rows, six compositions, unchanged**, and the
two dead ones are still the two the table names:

| row | line | composition | live? |
|---|---|---|---|
| `a_clean_fold_keeps_a_message_it_did_not_write` (`:2157`) | `:2172` | `apply(status, fold_status(clean))` | yes |
| `a_clean_fold_retires_the_camera_refusal_it_did_write` (`:2182`) | `:2188` | `apply(status, fold_status(refused))` | **no** |
| " | `:2191` | `apply(status, fold_status(clean))` | yes |
| `a_cursor_that_has_not_moved_retires_nothing` (`:2224`) | `:2231`, `:2239` | `apply(status, cursor_status(step))` | yes |
| `a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it` (`:2351`) | `:2370` | `apply(status, fold_status(refused))` | **no** |
| `a_supersession_survives_the_accepted_edit_that_caused_it` (`:2475`) | `:2509-2513` | `apply(status, frame_status(…))` | yes |

`apply(status, StatusUpdate::…)` sites (`:2214`, `:2436`, `:2439`,
`:2445`) are not members: they hand over a hand-built verdict and
compose no producer.

### The question this row filed itself to ask is already answered in the tree

The row says the discriminator *"is a useful rule and nothing states it
anywhere"*. **That is false as of #2026's fix pass.**
`crates/viewer/src/frame.rs:2337-2349` states it, at the row that uses
the dead composition, in as many words: *"No production caller composes
them any more … `pane::viewport`'s
`landing_a_refused_fold_is_news_and_joins_the_frames_notices` is the
row on that live path. What survives here is `apply`'s contract … a
`Show` handed to `apply` overwrites whatever was held, whoever hands it
over"*, and the composition itself carries *"asserted through the
nearest producer to hand rather than a live composition — see the doc
above"* (`:2367-2368`).

So the general question — *may a unit row set up its fixture with a
composition production does not perform?* — has a worked answer at a
site: **yes, when the row's doc says the composition is dead and names
the row on the live path, and when the row's NAME does not claim the
composition is the behaviour.** Nothing on this slate has to decide it.

### And `ranked-and-unranked-verdicts-are-one-type` deletes the residue outright

Both dead compositions are `apply(…, fold_status(…))`. That row's
proposal makes `apply` take a ranked verdict rather than a policy's
`StatusUpdate`, at which point `:2188` and `:2370` **stop compiling**.
So this row must not be spent as a serialized `frame.rs` lane ahead of
it; taking that row first removes the subject.

### What is actually left, and where it goes

1. **A rider, in `frame.rs`.** `a_clean_fold_retires_the_camera_refusal_it_did_write`
   (`:2182`) is the one dead composition with no disclosure — its
   sibling at `:2351` has one and it does not. One paragraph mirroring
   `:2337-2349`. It rides any `frame.rs` lane; it is not a lane.
2. **The crate-wide clause, filed across the fence.** *"Worth answering
   once for the crate"* means a sentence in `crates/viewer/README.md`,
   which `work/vnews/program.md`'s `keep_out` puts in VDOC's territory
   and says is *"never fixed across that fence"*. Filed as
   `work/vdoc/a-fixture-may-compose-a-dead-door-and-nothing-says-when`.

**This row is therefore a negative result on its own question**, kept
open only to carry item 1. Closing it before item 1 lands would bury a
residue in prose, which `work/README.md` forbids.

