---
id: loop-continuity-meters-u-through-levered-and-v-through-metered
kind: issue
title: pcurve_loop_continuity crosses the same param-to-meters seam through two different Margin doors, u via levered and v via metered
status: open
opened: 2026-09-12
---


## Finding

`crates/topo/src/pcurves.rs`, `pcurve_loop_continuity`: the u-channel
gap goes through `Margin::levered(gap, azimuth_arm)` and the v-channel
gap through `Margin::metered(gap, v_meter)` — one predicate, one
crossing (chart units to meters), two doors with two dimensional
arguments (an angle times an arm; a span times a rate). Both arrive at
meters; the asymmetry is the reviewer's Q1 shape (two spellings of one
rule), and it hides that both channels' rates are SUP bounds on the
spline arm while `metered`'s doc promises an inf (PROPS' row
`metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup`).
Found by the SCALAR rate census, 2026-09-12; filed on TRIM as the
owner of `crates/topo/src/pcurves.rs`.
