---
id: the-region-legs-spell-a-circle-and-a-line-beside-curve3
kind: issue
title: The region legs spell a circle and a line beside Curve3
status: open
priority: P2
opened: 2026-10-25
cost: M
---

## What

The region skeleton's leg courses are hand-written carriers beside the
`geom::Curve3` variants they restate (`crates/topo/src/boolean/region.rs`):

- `Round` (`:681`) is a circle: `Round::at` (`:690`) evaluates
  `center + radius·(e1 cos s + e2 sin s)` by `sin_cos`, and
  `Round::angle` (`:697`) and its `Course::heading` (`:729`) are its
  inverse and tangent, where `Curve3::Circle` (`center`, `axis`,
  `radius`, `u_ref`) has `eval` and `deriv`.
- `Line` (`:737`) is `Curve3::Line` (`origin`, `dir`) under other names.

This predates PR 5674, which folded the doors' own copies into these
two; its review (S10) found the pair itself still outside `Curve3`.
Two spellings of one carrier's evaluation can differ in their last
bits, and every reading past them inherits whichever one it took.

## Owed

Let a leg's course carry its `Curve3` (and what a leg adds: its start,
its run direction), and read `eval`/`deriv` and one inverse through it.
Re-baseline any bit that moves.

## Moved (2026-10-26)

CHART folded into CHARTTAIL (Ev, in chat, 2026-10-26): CHART had closed
every P0 and P1 row, and its last rows are the chart boundary's tail.
Id and body unchanged.
