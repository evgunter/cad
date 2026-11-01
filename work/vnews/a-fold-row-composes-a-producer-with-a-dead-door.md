---
id: a-fold-row-composes-a-producer-with-a-dead-door
kind: issue
title: Two frame.rs rows compose fold_status with apply over a composition no caller performs
status: closed
opened: 2026-09-06
refs: [ranked-and-unranked-verdicts-are-one-type, 2026]
priority: P3
cost: E
closed: 2026-09-20
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

### The sweep reproduces, under a rule the original left implicit

Re-derived by the stated rule — every row in
`crates/viewer/src/frame.rs` that composes a `*_status` producer with
`frame::apply`. **Five rows, unchanged. Seven CALL SITES**, which the
table above renders as *"six compositions"* because it collapses the
cursor pair into one line with a `×2`. Both numbers are right under
their own rule and neither was written down; this one counts `apply`
calls whose second argument is a `*_status(…)` producer, including the
one that reaches `apply` through a bound `update` rather than inline.
The two dead ones are still the two the table names:

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

### The question is APPLIED in the tree, not stated — and that is enough to close it

The row says the discriminator *"is a useful rule and nothing states it
anywhere"*. The precise correction, which an earlier draft of this
section overstated:

**`crates/viewer/src/frame.rs:2337-2349` does not state the rule.** It
DISCLOSES, for one row, that no production caller performs the
composition and names the row on the live path: *"No production caller
composes them any more … `pane::viewport`'s
`landing_a_refused_fold_is_news_and_joins_the_frames_notices` is the
row on that live path. What survives here is `apply`'s contract"*, with
`:2367-2368` adding *"asserted through the nearest producer to hand
rather than a live composition"*. **The NAME test — a row is wrong when
its name claims the composition is the behaviour — appears nowhere in
it.** So the row's sentence is right about the general rule and wrong
only in implying the tree offers nothing: it offers a worked
application without its rule.

That distinction is why the disposition splits the way it does, and why
`work/vdoc/a-fixture-may-compose-a-dead-door-and-nothing-says-when`
says, correctly, that the rule *"is stated nowhere general — only
inside the one doc comment that applies it"*. The two files agree.

**What closes this row is that neither half is a question this slate
has to answer.** The general rule is a `crates/viewer/README.md`
clause, which is VDOC's by a named `keep_out`. The one undisclosed
instance is a paragraph of prose. Neither needs a decision; both need
an owner, and both now have one.

### `ranked-and-unranked-verdicts-are-one-type` may delete the residue, on one of its two arms

Both dead compositions are `apply(…, fold_status(…))`, so what happens
to them is a TYPE question and turns on which arm that row takes:

- **`apply` takes the ranked verdict** — `fold_status` returns the
  policy's `StatusUpdate`, so `:2188` and `:2370` stop compiling. So do
  `:2172`, `:2191`, `:2231`, `:2239` and the production site
  `crates/viewer/src/pane/viewport.rs:473`, none of which is dead:
  **seven sites, not two**, which is a larger argument for taking that
  row first than this row's residue alone.
- **`apply` becomes private to `frame`** — the row's other stated arm.
  The module's own tests are inside `frame`, so every site here
  compiles unchanged and nothing about this row moves.

The first arm is the one this adjudication reads as live, because
`apply` has two production callers outside the module
(`crates/viewer/src/app.rs:1342`, `crates/viewer/src/pane/viewport.rs:473`)
and privacy is not available without the ranked type anyway. But it is
an argument, not a certainty, so the ordering rule is stated
conditionally: **this row must not be spent as a serialized `frame.rs`
lane ahead of that one**, because on the live arm taking it first is
work thrown away.

## Closed 2026-09-20 — a negative result, both residues re-homed

Each residue has its own file, filed in the same commit that closes
this one. `work/README.md` is explicit that a residue disclosed inside
a closing row's prose *"reads as a record of work done, not as an open
thread"* and dies with the row, so disclosing is not scheduling —
**give it its own file at the moment you disclose it**:

1. **The undisclosed instance** —
   `a_clean_fold_retires_the_camera_refusal_it_did_write`
   (`crates/viewer/src/frame.rs:2182`, composition at `:2188`) uses the
   dead composition and says nothing, where its sibling at `:2351`
   discloses. One paragraph. Filed as
   `work/vnews/a-dead-composition-sets-up-a-fixture-without-saying-so`,
   riding group A's carrier.
2. **The crate-wide rule** — *"worth answering once for the crate"*
   means a clause in `crates/viewer/README.md`, which
   `work/vnews/program.md`'s `keep_out` puts in VDOC's territory and
   says is *"never fixed across that fence"*. Filed as
   `work/vdoc/a-fixture-may-compose-a-dead-door-and-nothing-says-when`.

**Nothing in the tree changes because of this row, and that is the
result.** The sweep reproduces, the two dead compositions are real, and
the one row that does not disclose asserts nothing false and carries a
name that does not overclaim — which is the row's own discriminator,
applied to itself. What looked like a question for this slate was an
ownership question with two answers, and both are now files.

