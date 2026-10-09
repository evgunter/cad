---
id: projected-row-reads-its-chart-edge-through-the-box-not-its-eval
kind: issue
title: topo::pcurves::chart_edge reads a projected row through chart_box, because Pcurve::eval over a wide t encloses the image too loosely for far-cell membership; why is not established
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [4304]
---


Filed by the fix pass of PR 4304 (review item 13). The deviation was
disclosed in the PR body but had no row.

## What the code does

`topo::pcurves::chart_edge` gives a `Pcurve::Projected` row its
`ChartEdge::Envelope` image as the hull of `Pcurve::chart_box` over the
walked interval, with slack 0 (the `Pcurve::Projected(_)` arm, beside
the `Fitted`/`General` arm). The closed-form rows read `Pcurve::eval` at
the interval scalar's `t` enclosure instead (the natural extension).

## Why it deviates, as far as was measured

With the natural extension (`eval` at a wide `Interval` `t`), the far
cell of `crates/topo/tests/trim_3_chart_bound.rs` row t9 read the cell as
outside. Reading the box restored it. Why the extension's enclosure
misses the far cell was not investigated: it may be the `atan2`
branch's enclosure over a wide piece, or `eval`'s per-span hull of
several pieces.

## What a fix owes

Either the reason `eval`'s natural extension over a wide `t` is the
wrong reading for a projected row, stated at `chart_edge`, or an `eval`
whose enclosure is tight enough that the arm can read it as the others
do.
