---
id: a-reshaping-that-shadows-a-kept-steps-piece-reports-nothing
kind: issue
title: A SetProgram that keeps a step but leaves its named role undrawn reports nothing: the name vanishes with no Maintenance row
status: open
opened: 2026-09-29
priority: P2
cost: M
---

## The finding

Found by AUTH-6 (`author/profile-reshape`), which made reshaping a
committed profile a one-click act in the viewer's profile editor.

`DocEdit::SetProgram` reports a name as stranded only when it spells a
step the new program DROPS: `stranded_steps`
(`crates/editor-core/src/edit.rs`) filters the document's name carriers
by `piece_steps().is_disjoint(dropped)`, and `dropped` is the old ids
`settle_step_ids` did not see kept. A name on a step the program KEEPS
is never reported.

But a kept step can stop drawing the role a name spells because of a
step inserted beside it. N1 (`crates/editor-core/src/names/README.md`,
"Undrawn pieces vanish rather than alias") states the mechanism: two
pieces on one carrier are drawn as one segment, which answers to the
earlier piece in authored order, and the later piece's locator
resolves `Vanished`. Its own example: in `at, toward(+x), fillet(r),
toward(+y), to (2, 2), line_to(0, 2), line_to(Start)` the far-end
step's `Leg` denotes nothing, because the fillet's `RunOut` holds the
segment. So a `SetProgram` that inserts a `fillet` before a kept
`line_to` whose `Leg` a blend or a derived frame names leaves that name
`Vanished`, with `Applied.maintenance` empty.

N1 treats a role that the current VALUES do not draw as transient ("until
the values draw it again"). This one is undrawn because of the program's
STRUCTURE, which is DM7's subject ("the edit that removes a name's
referent"); it does not come back until the structure changes again.

## Why it matters now

The viewer's edit door counts, on its Apply button and before the
click, the names the door will strand
(`crates/viewer/src/pane/profile.rs`, `apply_and_revert`, fed by
`DocSession::edit_profile_report`). It reads the door's own rows, so
this case reads "Apply" with no count while a named piece goes silent.
The editor cannot fix this by deriving it again: it would be a second
authority on which pieces a program draws.

## Unmeasured

No row pins this at the door yet. The first step is a row in
`crates/editor-core/tests/edit_set_program.rs`: insert a `fillet`
before a named, kept leg, and assert what `Applied.maintenance` holds.
Whether the fix is a new report row, a strand, or a ruling that N1's
vanish covers it is EDIT's call.

A pure reorder is, by reading, another trigger: moving a kept step so
that a neighbour's piece takes its segment needs no insert. AUTH-6's
correctness review found no valid instance, because the lattice pins a
`fillet` between `toward`/`to` neighbours, but did not rule one out.
