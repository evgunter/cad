---
id: maintenance-net-keeps-an-undrawn-piece-strand-a-later-edit-redrew
kind: issue
title: MaintenanceNet keeps a SetProgram's undrawn-piece strand that a later edit of the same action draws again
status: open
opened: 2026-10-02
priority: P4
cost: M
---


## The finding

`MaintenanceNet` (`crates/editor-core/src/edit.rs`) keeps a
`Maintenance::Strand` when its carrier is live at the end of the action
and still holds the name, and a `StrandedAppearance` when the store
still holds the key. That was exact while every strand was permanent: a
deleted node or a dropped step id never comes back.

A `SetProgram` now also strands a name on a kept step's piece it stops
drawing (`undrawn_kept_pieces`), and that piece CAN come back: a later
edit of the same action that redraws it (a second `SetProgram`, or a
value edit that ends a `Zero` fit) leaves the name denoting its piece
again, while the net still reports the strand. No caller lands such an
action today — the viewer's path editor commits one `SetProgram` per
action (`DocSession::edit_profile_report`, `set_program_of`), and the
parameter door's action is a notation and a value edit — so this is
latent.

## The fix

The survival test for a strand whose cause was an undrawn piece has to
ask whether the end document's program still leaves that piece undrawn
— `ProfilePayload::drawn_pieces` under the end document's parameters —
rather than whether the carrier holds the name. That needs the row to
carry, or the net to re-derive, which piece it was about.
