---
id: main-red-one-record-folds-the-face-sense-by-hand
kind: issue
title: main red: face_normal's no_source_file_folds_the_bit_by_hand flags step-import one_record.rs's hand sense fold
status: open
opened: 2026-11-23
priority: P1
cost: E
---


## The finding

(Filed by the WALKS orchestrator, 2026-11-23, from a WALKS lane's CI report on PR 7430, whose diff does not touch the file; the lane reproduced it locally on its branch.)

`topo` `face_normal::no_source_file_folds_the_bit_by_hand` goes red on main: it flags `crates/step-import/src/one_record.rs` — `let facing = if face.sense { from_n } else { -from_n };` (~:184), a hand fold of a face's sense bit onto a normal, which `face_normal` is the one home for (`the_planar_sense_flip_lives_in_one_place`). Last touched by `1ef6fc8324` ("export: import-records fix pass"), so the change that introduced it is EXPORT's and the fix is owed there.

## What a fix is

Read the facing through the `face_normal` door (or the door the census names for a record-level facing), or, if a record-level read genuinely has no face to ask, add the site to the census's allowlist with its reason.

## Evidence (GERM, PR 7431)

GERM's PR 7431 review also saw `no_source_file_folds_the_bit_by_hand`
red outside that PR's diff. The lane re-ran it on an `origin/main`
worktree on 2026-11-23, and it was red there too, flagging the same
`from_n` fold in `crates/step-import/src/one_record.rs`. A duplicate row
filed on PR 7431's branch was withdrawn in favour of this one.
