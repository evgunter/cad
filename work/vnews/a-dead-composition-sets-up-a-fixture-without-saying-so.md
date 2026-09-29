---
id: a-dead-composition-sets-up-a-fixture-without-saying-so
kind: issue
title: One of the two dead apply-over-fold_status compositions discloses that it is dead and the other does not
status: closed
opened: 2026-09-20
priority: P4
cost: E
rides_with: frame-rs-says-the-per-subject-line-is-a-question-for-ev
closed: 2026-09-24
---


Filed 2026-09-20 as the residue of
`work/vnews/a-fold-row-composes-a-producer-with-a-dead-door`, which
closed as a negative result on its own question. `work/README.md`:
*"give it its own file at the moment you disclose it"* — a residue
disclosed inside a closing row's prose reads as a record of work done
and dies with the row.

## The two rows, and the difference between them

`crates/viewer/src/frame.rs`'s test module holds two dead compositions
and they are the same one, `apply(status, fold_status(refused))`: no
production caller hands a `fold_status` `Show` to `apply` since #2026
routed a refused fold through `frame::deliver`.

- `a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it`
  (`crates/viewer/src/frame.rs:2351`, composition at `:2370`)
  **discloses it**, at `:2337-2349`: *"No production caller composes
  them any more … `pane::viewport`'s
  `landing_a_refused_fold_is_news_and_joins_the_frames_notices` is the
  row on that live path"*, plus an inline note at `:2367-2368` saying
  the producer is *"the nearest producer to hand rather than a live
  composition"*.
- `a_clean_fold_retires_the_camera_refusal_it_did_write`
  (`crates/viewer/src/frame.rs:2182`, composition at `:2188`)
  **does not**. Its only comment (`:2183-2186`) is about the defect the
  row closes, not about the door it uses to get there.

Nothing the second row asserts is false and its name does not
overclaim, which is why the parent closed rather than demanding a
rewrite. What is missing is one paragraph, mirroring its sibling's.

## Why it is not just tidiness

`pane/viewport.rs`'s
`landing_a_clean_fold_retires_the_camera_refusal_it_landed_before` is
the row on the live path for this exact property, and #2026's fix pass
put it there. A reader of `:2182` who does not know that is one edit
away from deleting the live-path row as a duplicate, or from extending
`:2182` as though it covered the live path. The disclosure is what
stops both.

## The population may go to one before this lands

`work/vnews/ranked-and-unranked-verdicts-are-one-type` would make
`frame::apply` take a ranked verdict rather than a policy's
`StatusUpdate`. Under that arm `:2188` and `:2370` stop compiling and
both rows are rewritten anyway. Under the row's other arm — `apply`
made private to `frame` — the module's own tests still reach it and
nothing here changes. **Re-derive against the tree before writing the
paragraph.**

## Home

VNEWS's: `crates/viewer/src/frame.rs`, which is serialized. It rides
`work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev`,
group A of `work/vnews/plan.md` §Order 7.

## Closed 2026-09-24 (`vnews/frame-rs-prose-pass`)

Re-derived first, as the row asked: `frame::apply` still takes a
`StatusUpdate` (`ranked-and-unranked-verdicts-are-one-type` has not
landed), so both compositions still compile and the paragraph is owed.

`a_clean_fold_retires_the_camera_refusal_it_did_write` now carries a
doc comment mirroring its sibling's: its first `apply` of a
`fold_status` `Show` is not a live composition — a refused fold reaches
the line through `deliver` and the ranking — and it is there only to
put a camera refusal on the line so the row can ask what the next clean
fold's `Expire` does to it. It names
`pane::viewport`'s `landing_a_clean_fold_retires_the_camera_refusal_it_landed_before`
as the live-path row and says the two are not duplicates and this one
does not cover that path — the two misreadings the row named.
