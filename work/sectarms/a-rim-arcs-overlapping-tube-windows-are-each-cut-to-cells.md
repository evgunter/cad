---
id: a-rim-arcs-overlapping-tube-windows-are-each-cut-to-cells
kind: issue
title: A rim arc's overlapping tube windows are each cut to cells, so a many-span arc's first reading images each stretch of wall several times
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Filed by the `sectarms/rim-pad-cells` lane (the fork's defect list,
fork-log row 280).

## What

A fitted spline × implicit arc's cells
(`pair_section::spline_implicit::cells`, `Enclosure::first`) are read
window by window: each window of the chart tube is cut to the knot
rectangle and halved to cells a rung wide, the cells the partner's form
clears dropped. Consecutive windows are a span's hull padded by the
rung, so on an arc of many short spans they overlap, and the same
stretch of wall is imaged once per window that covers it. Each image is
one `NurbsBoxes::rect_box`, which is where the first reading's time
goes (measured below).

## Measured

Test profile (unoptimized), the carry rows' wall at `s = 1`, one arc,
first reading only:

| partner | spans' control points | cells kept | `rect_box` calls | first reading | the listing |
|---|---|---|---|---|---|
| ball about `(0, 1.1, −3)`, radius `3.5` | 173 | 858 | 2084 | 0.35 s | 1.8 s |
| ball about `(0, 1.1, −1)`, radius `1.6` | 332 | 1628 | 3953 | 0.78 s | 3.1 s |
| ball about `(0, 1.1, −0.5)`, radius `1.2` | 453 | 2162 | 5242 | 0.93 s | 4.6 s |

Two arcs each, the same wall and profile (measured on a quieter box
than the table above, so compare within these two lines):

| partner | cells kept an arc | first reading | the listing |
|---|---|---|---|
| rod of radius `0.09` along the wall | 64, 64 | 0.06 s | 0.38 s |
| the same rod turned 30° in the wall | 1674, 1475 | 1.83 s | 4.68 s |

The turned rod's arcs are fitted in many short spans, whose windows
overlap, and are read in twenty-five times the cells of the straight
rod's for arcs about as long.

About five cells are kept per span, where an arc a rung wide needs
about two per rung of its length. Before the cells were read to the
rung and cleared (two-rung cells, read against their own span), the
same three first readings took 0.12, 0.23 and 0.29 s.

## Remedy

Sweep the union of the windows once: drop a cell that lies wholly
inside a window already swept, whose kept cells cover every point of it
the partner's form does not clear. Two readers want a window's own
cells and need care: the witness's reach (`Enclosure::carried`, the
cells of a window whose span holds the witness's parameter) and the
runs a site's foot is read over (`CarriedComponent::runs`, a window's
cells hulled with its span).

Separately, `rect_box` reads its derivative boxes off the whole span
cell on every call
(`work/ssiedge/the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell.md`),
so every sub-rectangle of one span cell recomputes the same two boxes.

## Done when

A many-span arc's first reading images each stretch of its tube once,
and the three rows above cost no more than they did at two-rung cells.
