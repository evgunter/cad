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

## Measured, with the status line in a row of its own (`chrome/status-line`, 2026-09-29)

The status line now has its own row under the toolbar's controls, and
the row scrolls sideways only. Its height is whatever the joined line
wraps to, so N strand sentences push the viewport down by N rows'
worth. Headless, through `app::tests`' `toolbar_with`, one 184-character
strand sentence (`node 12 carries a face name minted by node 7; …`)
repeated N times, in a 600-point-tall window:

| N | 400-pt window | 1000-pt window | 1920-pt window |
|---|---|---|---|
| 1 | 3 lines, 45 pt | 2 lines, 30 pt | 1 line, 15 pt |
| 3 | 9 lines, 135 pt | 4 lines, 60 pt | 2 lines, 30 pt |
| 8 | 24 lines, 360 pt | 9 lines, 135 pt | 5 lines, 75 pt |

At N = 8 in a 400-point window the toolbar ends at y = 431 of 600. The
choice is still open, and the options are the ones above: a count on the line
pointing at OFFER's post-click panel (not built yet), a count per arm
with the rows on hover, the first few rows and "and N more", or a
height bound on the status row so that it scrolls vertically too.
