---
id: a-reflex-corner-that-is-a-graph-but-unread-in-its-cell-is-typed-local-where-main-typed-it-structural
kind: issue
title: A decided reflex corner that is a graph but is not read in its cell is typed Local where main typed it Structural: unmeasured on a body
status: open
opened: 2026-11-23
priority: P2
cost: E
---

## Finding

From lane `embed/tangent-corner` (PR 7320, 2026-11-23). **Unmeasured on
a body.**

Before that PR a curve's corner whose other edge was decided to leave
onto the wrong side (reflex) was `Unpresented::Structural`: no smaller
cell reads it, so the pair refused `Uncertified` at once. With the run
through a vertex (`graph_read`, `line_onward`):

- a reflex corner whose line is no graph with the curve over a column
  is still `Structural`;
- one that is a graph, but is not read in this cell (`line_onward`
  `Local`: the line ends inside the columns; or `clear` fails on the
  grown `R`), is `Local`.

So a pair whose reflex corner is a graph and is never read at any
depth now splits until `Budget(Depth)` where main refused
`Uncertified { Departure }` at the first cell: the same body refused,
later, with more cells spent and another finding named. The lane's
first report said "never worse than main"; that was too strong.

Not measured: whether any body does this, and what it costs. The tour
walk (85 stops) is identical to main's line for line, counts of
candidates and splits included, so no tour body does.

## What is needed

A body with a reflex curve corner that stays unread (a line that ends
inside the columns at every depth), its verdict and cell count on main
and with the run; then either the typing is kept and this row closes
with the numbers, or the corner is typed `Structural` where no smaller
cell can read it.
