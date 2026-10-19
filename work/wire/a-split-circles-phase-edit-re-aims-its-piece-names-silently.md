---
id: a-split-circles-phase-edit-re-aims-its-piece-names-silently
kind: issue
title: A circle_split phase moved by a multiple of 2π/n keeps every piece name and reports nothing, while Piece(k) now draws the arc Piece(k+1) drew
status: closed
opened: 2026-09-30
priority: P1
cost: M
closed: 2026-10-18
---


## The finding

Found by AUTH-6's correctness review (#3446), by reading; not measured.

N1 (`crates/editor-core/src/names/README.md`, "What cannot move it")
lists "a value edit" among the changes that do not move what a locator
denotes. A `circle_split`'s `phase` is a value: it has always been a
`SetParam`, and the viewer's profile editor keeps the step's id when it
moves (`crates/viewer/src/drafts.rs`, `ProfileEdit::ids`: the same verb
and piece count keep the id). `Piece(k)` of a `circle_split` is the arc
that starts at `phase + 2πk/n`. Move `phase` by exactly `2π/n` and the
circle draws the same geometry, but `Piece(k)` now sits on the arc
`Piece(k+1)` sat on before. A name on `Piece(k)` (a blend, a derived
frame, an appearance) keeps its spelling, resolves, and denotes a
different arc. Nothing is reported: the edit is a value edit, and
`Applied.maintenance` reports only strands.

At any other phase change the pieces rotate by less than one arc, and
"the same piece, moved" is the honest reading. At a multiple of `2π/n`
the same reading is a confident wrong answer: the geometry did not
change and the name moved to the neighbouring arc.

## Unmeasured

No row pins it. The first step is a row that names `Piece(0)` of a
`circle_split(n = 4)`, sets `phase += π/2`, and asserts which arc the
name resolves to and what the edit reported. Whether the fix is a
ruling that N1 means this (a piece is its index, not its place), a
report row, or something else is WIRE's call, with N1's text.

## FORK-CIRCLEPHASE (2026-10-18): converged; the N1 sentence is Ev's

Both designers, briefed with the problem only, rejected the row's premise: N1 says a name "denotes a construction role, not a point set", and the phase is a value N1 says cannot move a locator, so `Piece(k)` is the k-th arc from vertex k at `phase + 2πk/n`; a phase edit moves it continuously, and a shift by a whole number of arcs is that motion ending where a neighbour began (the same as rotating a chain's vertices). Not a re-aim; no report (a report at the coincidence would be a float equality over a continuous parameter). The row closes with a pinning row once N1 states the reading; the `[ev]` PR adds that sentence to N1's `circle_split` bullet. Design-fork log row 158.

Signed (WIRE orchestrator).

## Resolution

N1's `circle_split` bullet states the reading (Ev, PR 5278): `Piece(k)` is the arc from vertex `k`, at `phase + 2πk/n`, to vertex `k + 1`, and a phase edit moves every piece and renames none. `a_circle_splits_phase_edit_moves_its_pieces_and_renames_none` (`crates/editor-core/tests/edit_set_program.rs`) pins it: `n = 4`, phase `0 → π/2` through `SetParam`, a frame on `Piece(0)`'s wall still resolves, the wall spans `[π/2, π]` and its start strut stands at angle `π/2`, and the edit reports nothing. No fillet-on-a-seam row: the split's seams join one cylinder to itself, tangent-continuous, so this fixture has no crease for a blend to carve; not attempted.
