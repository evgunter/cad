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

**Both channels are typed, and the u channel's door is now picked by
the arm's own kind rather than by the reader.** The finding was right
that one crossing was wearing two doors; what makes it two crossings is
what the chart's `u` MEANS, and that varies by chart kind:

- on the ANGULAR kinds (cylinder, sphere, torus, cone) the `u`
  discrepancy is an ANGLE and the arm is metres per RADIAN at a `v`
  (`r`, `r·cos v`, `R + r·cos v`, `v·sin α`). That is the levered
  door's dimensional argument — a dimensionless quantity times a
  length lever — and it stays `Margin::levered`.
- on the PLANE and SPLINE kinds there is no angle: the discrepancy is a
  chart-PARAMETER span and the arm is `chart_stretch_sup`'s first
  component, a `SupSpeed` — the same crossing as the v channel, which
  is what this row was filed about. Those go through
  `Margin::metered_sup`.
- the **v** channel's is a parameter span whose units differ per kind
  (an angle on the polar charts, a length on a plane, cylinder or cone,
  a chart parameter on a spline), so it is a metres-per-parameter-unit
  crossing and takes the metric door. `v_meter` answers a `SupSpeed<T>`
  and the escape claim's safe direction is checked by the compiler.

The direction was wrong too, and that half is what `metered`'s doc
promised: both rates are SUP bounds on the spline arm while the door
promised an inf (PROPS' row, closed in the same PR).

`azimuth_arm` is now `chart_u_arm`, returning `ChartArm<T>` —
`Angular(T)` or `Rate(SupSpeed<T>)` — whose `meter` method is the one
place that says which door a first-channel gap reaches the band
through. Every consumer of the arm picks its door off that type:
`walk_loop`, `chart_boundary`'s closure, `validate_pcurves`,
`loop_closes` and `ChartBound::assembled`'s span check.

**What is left, and it is the class's next member, not a residue of
this row.** The ANGULAR kinds' arm is still a bare `T`: metres per
radian is not a rate per parameter unit, so the rate pair has no type
for it, and the same is true of `azimuth_lever`, `chart_windings` and
the cone arm `chart_arms_at` supplies. Whether the kernel wants a
second tagged pair for angular arms is a question this unit did not
answer and did not pretend to — filed as
`work/trim/angular-arms-are-an-untagged-lever-beside-a-typed-rate.md`.

`docs/predicate-dimension-audit.md`'s `pcurves.rs` row records the
doors and the reason for each.
