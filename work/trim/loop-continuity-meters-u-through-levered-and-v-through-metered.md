---
id: loop-continuity-meters-u-through-levered-and-v-through-metered
kind: issue
title: pcurve_loop_continuity crosses the same param-to-meters seam through two different Margin doors, u via levered and v via metered
status: closed
opened: 2026-09-12
closed: 2026-09-15
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


## Closed (RATE-PAIR, `rate-pair-in-geom-core`)

**What the u/v asymmetry becomes: two doors, and the reason is now in
the types rather than in the reader's head.** The two channels do not
cross the same seam, which is why one door was never going to serve
both:

- the **u** channel's discrepancy is an ANGLE on every chart kind the
  arm is exact for, and `azimuth_arm` is metres per RADIAN at a `v`
  (`r`, `r·cos v`, `R + r·cos v`, `v·sin α`). That is the levered
  door's dimensional argument — a dimensionless quantity times a length
  lever — and it stays `Margin::levered`.
- the **v** channel's is a parameter SPAN whose units differ per kind
  (an angle on the polar charts, a length on a plane or cylinder, a
  chart parameter on a spline), so it is a metres-per-parameter-unit
  crossing and takes the metric door.

What was genuinely wrong was not the asymmetry but the direction: both
rates are SUP bounds on the spline arm, and `metered`'s doc promised an
inf (PROPS' row, closed in the same PR). `v_meter` now answers a
`SupSpeed<T>` and the v-channel goes through `Margin::metered_sup`, so
the escape claim's safe direction is checked by the compiler.
`azimuth_arm` keeps its bare `T` and says why at its own header: an
arm per radian is not a rate per parameter unit, and its spline branch
takes the tag off `chart_stretch_sup` at the one place where the same
number is read as a lever.

`docs/predicate-dimension-audit.md`'s `pcurves.rs` row records the two
doors and the reason for each.
