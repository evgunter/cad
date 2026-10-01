---
id: a-split-circles-phase-edit-re-aims-its-piece-names-silently
kind: issue
title: A circle_split phase moved by a multiple of 2π/n keeps every piece name and reports nothing, while Piece(k) now draws the arc Piece(k+1) drew
status: open
opened: 2026-09-30
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
