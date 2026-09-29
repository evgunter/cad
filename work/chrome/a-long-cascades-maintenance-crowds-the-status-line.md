---
id: a-long-cascades-maintenance-crowds-the-status-line
kind: issue
title: A long cascade's maintenance puts one long sentence per surviving row on the one status line
status: open
opened: 2026-09-25
priority: P3
cost: D
---


`frame::outcome_notices` (`crates/viewer/src/frame.rs`) gives every
surviving DM7 row of an action its own notice, and `frame_status`
joins a frame's notices onto the ONE status line with
`NOTICE_SEPARATOR`. Each row's sentence is `Display for Maintenance`'s,
and a strand's runs to about two hundred characters. A cascade delete
through a node that many surviving carriers name — the die's fillet
selections, a declared union's pairs — therefore puts N such sentences
on one line, where nothing counts, truncates or groups them.

What the line should say instead is a chrome decision: a counted
preamble per arm, the way `frame::Withdrawal` counts ("hide: 3 hides
were dropped — …"), is not directly available, because a strand's own
sentence writes `LIST_SEPARATOR` and a flat join of them could not be
split back into rows (`frame::maintenance_notice`'s doc). The per-row
list, with the `Rebind` it leads to, is what
`work/offer/cascade-delete-shows-the-strand-count.md` already owes as
the post-click panel; the line could then carry a count and point at
it.

Found while building the maintenance report
(`the-viewer-drops-every-dm7-rename-report`, PR #3196).
