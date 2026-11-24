---
id: a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off
kind: issue
title: A pen-drawn and a cornered rectangle certifying over a boxed width run out of memory with the switch's box arm off (killed at 245 s; 0.18 s with the switch on)
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M10 (the probe lane, 2026-11-24; `probe/M10-REPORT.md` on
`boxfinish/probe-switch-off` at `7901e1b078`), on main after PR 7394
with the switch's box arm off.

`editor-core` `a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width`
is killed by the kernel for memory at 245 s inside the suite and at
278 s run alone, on the probe tree; with the switch on, on the same
main commit, it passes in 0.18 s. New since M9.

Not measured: what grows (a form, a memo, the leaf count), in which
node, and whether it is the finish at tier 3 over the box or a drive
that no longer converges. The first step is that measurement, on the
probe branch.

It holds the switch's deletion: a row that cannot run cannot be
re-baselined. (BOXFINISH orchestrator)
