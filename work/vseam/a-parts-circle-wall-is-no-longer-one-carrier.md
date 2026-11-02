---
id: a-parts-circle-wall-is-no-longer-one-carrier
kind: issue
title: A part instance's circle wall is no longer one carrier or one axis, so a coincidence across its halves does not reach through the part
status: open
opened: 2026-11-02
priority: P2
cost: M
---


## What happens

`editor-core`'s coincidence door names the walls an extrude or revolve
sweeps over a `circle` or `circle_split` loop as one carrier, by the
loop's first piece (`coincide/stated.rs::carrier`). Since PR 6340 it
does so only when it can read the step's loop in this document's
program (`on_one_carrier`, through `LoopProgram::carrier_radius`), so
that a `rectangle`'s four `Piece(k)` sides stay four carriers. A cell
the walk stopped at a part's instance for carries the part's names,
which this document does not hold, so the merge is withheld there
(`stated.rs::carrier`, the `!in_a_part(doc, cell)` guard, lines
290–293).

The cost: a part instance's `circle` or `circle_split` wall is no
longer one carrier, nor one axis (`stated::axis_of` reads the carrier
first). DR-394's "other half" liveness — the door proving a row on one
half of a cylinder wall from a statement about the other — no longer
reaches through a part: a mate on half 0 of a part's cylinder and a row
on half 1 are two cells. Before PR 6340 the merge was blind to the
loop's form and held there too, which was unsound once a rectangle
could name its sides `Piece(k)`. The withheld direction gives fewer
proofs, never a false one, and is pinned by
`coincidence_door::a_parts_wall_names_are_not_read_against_the_hosts_circle`.

## What would close it

Read the part's own loop through the instance: resolve the instance's
`DocRef` to the part document (the resolver the evaluation already
uses), find the sweeping node and its profile there, and ask the same
`carrier_radius` question of the step's loop. The row above then gains
a positive half: a real part whose wall is a circle merges, one whose
wall is a rectangle does not.

## Price

M: the door's carrier read gains a resolver path and a part fixture,
and the coincidence rows over parts (`atrest_carried_coincidence`,
`asm*`) are its regression set.
