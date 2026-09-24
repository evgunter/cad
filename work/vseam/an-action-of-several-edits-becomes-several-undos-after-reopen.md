---
id: an-action-of-several-edits-becomes-several-undos-after-reopen
kind: issue
title: History::replayed commits each logged edit as its own state, so a several-edit action (duplicate, add-profile on a new frame) is several undos after save and reopen
status: open
opened: 2026-09-24
priority: P1
cost: D
---



## Finding

Found by AUTH-4's correctness reviewer (AUTHOR, PR 3052), filed by
AUTH-4's lane under implementer-discipline §6. Pre-existing; AUTH-4
added a second action it breaks.

**In session**, a user action that takes several edits is ONE history
state and therefore one undo: `DocSession::commit_run` applies the
edits in order and `record_action` hands the whole run to
`History::commit_group` (`crates/viewer/src/history.rs`, one `Entry`
whose `edits` is the run).

**After save and reopen it is not.** `History::replayed`
(`history.rs`, ~`:159`) seeds the history from the file's edit log and
calls `history.commit(entry.clone(), applied.doc)` once PER LOGGED
EDIT, so every edit becomes its own state. The file does not record
which edits were one action, so replay has nothing to group by.

Two actions this breaks today, both built on `commit_run` precisely so
that they would be one undo:

- **AUTH-3's add-profile on a new XY frame**
  (`DocSession::add_profile_on_new_xy`): a frame and the profile drawn
  on it. After reopen, one undo removes the profile and leaves an
  orphan frame the user never asked for on its own.
- **AUTH-4's duplicate** (`DocSession::add_duplicate`): a pattern and
  its two projections. After reopen, one undo removes the copy's
  projection only — and because a projection consumes the pattern from
  `Doc::roots`, the intermediate state draws ONE body where the user
  had two, which is the exact picture the gesture was built to avoid.

## What deciding it needs

The grouping is not in the file, so this cannot be fixed inside
`history.rs` alone. Either the persisted log carries the action
boundary (a change to `editor_core`'s `LoggedEdit` / persist format,
which is EDIT's ground), or the reopened history is allowed to differ
from the session's in undo granularity and the docs say so. Which one
is a design call; this row records that the in-session and reopened
histories disagree today, and names the two actions a user meets it
through.

**Measured, not argued** (AUTH-4 lane, through the session's own
`Save` / `Open` / `Undo` doors): box, duplicate → roots `[4, 5]`; save
and reopen → roots `[4, 5]`; ONE undo → roots `[4]`, one body drawn.
In session, the same undo returns to the box alone.
