---
id: a-gap-from-a-span-end-has-no-door-of-its-own
kind: issue
title: A parameter gap from a span's end has no door of its own, so span-end-gap-sites matches end names rather than the shape
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by PR 6710's FULL review (NOTE 6).

## What

`scripts/gates/span-end-gap-sites.sh` finds a hand-metered gap from a
span's end by the NAME of the end: a hand-written list (`t0`, `t1`,
`lo`, `hi`, …, `tau`). That is the trap a hand-written list sets. The
review found `topo/src/boolean/trace.rs` `circle_crossings`'
`Margin::metered(tau − theta, rate)` slipping it, until `tau` joined
the list. An end under any other name slips it the same way.

## What closing it takes

Give "a parameter's gap from a span's end, in metres" one door: a
function that takes the span (its ends and its rate bounds) and the
parameter, and states which bound it meters at, inf for "clear of" and
sup for "within". `curve_pair::Span::place_param` is the decided reading
of the same question, and the door may be a lower rung of it. The gate
then reads the shape: a `Margin::metered(_sup)?` whose first argument
is a parameter difference outside the door is a site. It no longer
reads the operands' names.
