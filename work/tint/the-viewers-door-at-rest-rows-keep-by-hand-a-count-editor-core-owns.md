---
id: the-viewers-door-at-rest-rows-keep-by-hand-a-count-editor-core-owns
kind: issue
title: The viewer's door_at_rest rows keep by hand a count of the composed die's unproven rows that editor-core's door test owns
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN's default-eps lane (`green/default-eps`, PR 6931,
2026-11-14), from its STYLE review (S1, S11).

## What

`crates/viewer/tests/door_at_rest.rs` holds `const DIE_UNPROVEN: usize`,
the number of `unproven-coincidence` findings the composed die
(`die_composed`) settles with. Both of its rows assert it
(`a_landings_door_reads_pending_until_its_own_answer_settles`,
`the_threaded_door_seam_answers_the_newest_landing`). The number is
derived by hand from `crates/editor-core/tests/coincidence_door.rs`
`the_door_proves_half_the_composed_dies_joints`, which pins, for the
same document, the joints the door proves (`proved`), the rows replayed
(`rows`) and the booleans' `Meet` rows (`met`): the viewer's count is
the unproven joints plus the `Meet` rows.

It has gone stale once. PR 6710 made the die's second joint prove, PR
6771 re-blessed editor-core's row (1 → 2 proved), and the viewer's copy
stayed 3 where main read 2. Both viewer rows were red on the nightly,
by 2026-11-11 at the latest, until PR 6931 set the copy to 2
(`work/coincide/main-red-two-door-at-rest-rows-on-the-2026-11-11-nightly.md`).
Both files are in the slow set and neither PR's diff spelled the
viewer rows, so no gate ran them. The next change to what the door
proves on that die does the same again.

## Wanted

The viewer rows assert what they are about (pending until the landing's
own answer settles; a stale answer is dropped; settled once) against a
count read from the run, not a literal: the settled report's
`unproven-coincidence` findings equal the door job's own findings for
that landing (`second.job.run(..)` is already in hand in the first row,
`findings` in the second), with a floor of one so the row cannot pass on
an empty report. Then `DIE_UNPROVEN` goes.

## Also: the editor-core row's name is stale

`the_door_proves_half_the_composed_dies_joints` proves both of
`die_composed`'s two joints and 21 of `die_composed_tour`'s 42 since PR
6771; "half" is true of the tour only. The viewer's doc cites the row by
that name. A rename touches the slow-set list in `.config/nextest.toml`
and that citation.
