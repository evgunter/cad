---
id: a-gap-from-a-span-end-has-no-door-of-its-own
kind: issue
title: A parameter gap from a span's end has no door of its own, so span-end-gap-sites matches end names rather than the shape
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
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

## Done

**The door.** `geom_brep::curve_pair::Span::end_gap(t, end, meter)` is the one door for a parameter's gap from a span's end, in metres. `end_gap_widened` measures from an end moved out by a step.
- The gap is metered at a bound on the carrier's speed over the gap itself: `geom::carrier_speed::speed_between`, over `window_speed`.
- The caller names the bound:
  - `GapMeter::Clear` meters at the inf, so a decided `Positive` or `Negative` is sound;
  - `GapMeter::Within` meters at the sup, so a decided `Zero` is within the band of the end.
- The window bound has one home, `window_speed`: the speed at the window's middle ± the larger semi-axis times its reach, clamped. `turn_defect_speed_hi` and `turn_defect_speed_lo` now read it, bit for bit as before.
  - `splitting/containment`'s hand-spelled copy is in its `mod tests`. It is an oracle that reproduces the pre-turn-rule spelling bit for bit, so it keeps its own.

**Readers, measured.** Of the 93 production `Margin::metered*` calls, five meter a gap from a span's end:
- `pair_section::within_span` and `curve_face/plane_crossing`'s graze both hold a span, and both read the door now. The graze is the site the old regex missed.
- `trace::circle_crossings` (a loose circle's own turn, at an exact rate), `pcurve_cache::side_of` (a surface chart's coordinate against its domain) and `ring_path::span_readings` (its own slope meter) hold no carrier span. They stay as named sites.

**The gate.** It reads the call's whole first argument, brackets balanced, so a nested end (`t − (t0 − δ)`) reads, and an extent on either side (`t1 − t0`) does not. Its blind spot, that a non-`Span` site is found by its end's name, is stated at its head, with `ring_path` named as the one known reader that binds its gap to a `let` and slips it. With main's `plane_crossing.rs` restored, it flags `graze_reaches_span` (0 → 2).
