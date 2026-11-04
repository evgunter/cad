---
id: a-sphere-class-arc-centre-just-across-the-axis-refuses-at-the-pcurve-mint
kind: issue
title: revolve: a sphere-class arc whose centre sits one band-zero across the axis builds a sphere and refuses at the pcurve mint (pcurve_map_residual), not as the ArcCenter sliver
status: open
opened: 2026-11-04
priority: P3
cost: E
---

(SPIRICARM, the fix pass of PR 6486: review 1's NOTE-2, pre-existing,
reproduced at the fix head.)

## What

`sweep::revolve::axis::classify_segment` (`crates/sweep/src/revolve/axis.rs`)
reads an arc's centre against the axis first (`axis_arc_center`,
`Margin::of(rc)`), and a `Zero` mints a sphere about the centre's foot
on the axis. The football's profile (`crates/sweep/tests/a_conical_ended_football_revolves.rs`'s
`Band`, two cones and an arc `|v| ≤ 1.2` of radius 1) with its centre
`c` off the axis, at the witness tolerance (band zero `1e-9`, escalate
`1e-8`), reads:

- `c = ±1e-13`: a sphere, which builds;
- `c = +1e-9`: `SliverAxisClearance { decision: ArcCenter }` (the margin
  rounds to `1.00000008e-9`, past the zero);
- `c = −1e-9`: decided `Zero`, a sphere minted, and the revolve then
  refuses at its own pcurve mint, `Op { Certification { Escalated {
  check: ChartResidual, predicate: "pcurve_map_residual" } } }`: the
  arc's points sit `1e-9` off the minted sphere;
- `c = −1e-8`: the `ArcCenter` sliver;
- `c = +1e-8`: decided off the axis, so the circle reaches it and a
  sheet with `R = 1e-8` is minted, which refuses at the certification
  `AnalyticRung3(NoOffsetBound { operand: Spindle })` (its lever
  `4|R|ρ` is not certified positive).

The `c = −1e-9` reading is the row: a centre the classifier puts ON the
axis is a sphere whose description the arc does not lie on to within
the band the next decision reads, so the refusal comes from the mint,
named after a pcurve residual, rather than from the classification as
the sliver it is. The `c = +1e-8` reading is the same seam one class
over (a sheet of `R` at the escalate).

## Repair shape

Either the sphere arm mints about the arc's own centre's foot with the
residual it leaves carried into the band the mint reads (so a `Zero`
centre always builds), or a `Zero` whose residual the mint cannot
clear is refused at the classification as the `ArcCenter` sliver.
Measure which first, on the ladder above at the three ε rows.
