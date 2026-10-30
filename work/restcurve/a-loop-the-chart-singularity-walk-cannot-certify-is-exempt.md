---
id: a-loop-the-chart-singularity-walk-cannot-certify-is-exempt
kind: issue
title: A loop the chart-singularity walk cannot certify a winding for is exempt from tier 3
status: open
opened: 2026-10-30
priority: P3
cost: M
---


## What

Where no closed form reads a loop's side of a chart singularity (a cone
loop, or a sphere loop with an edge that is not a circle arc), the
reading walks the loop through the chart
(`geom_brep::chart_singularity::walk_loop`, `chart_singularity.rs:835`),
certifying every step to turn by less than a sixteenth of a turn about
the surface axis. Where it cannot, `locate` answers `Unread::Loop`
(`chart_singularity.rs:668`) and tier 3's chart-singularity arm exempts
the face (`crates/topo/src/validate.rs:8223`; the not-yet-checked list
names it):

- a sample with no chart point away from every singular point;
- a step the refinement cannot certify within `WALK_BUDGET`
  (`chart_singularity.rs:779`) or its depth: a curve near the axis that
  the speed bound reads as possibly turning round it;
- a loop winding more than once (row:
  `geom-brep/tests/chart_singularity_locate.rs`
  `a_loop_winding_twice_and_an_unsettled_carrier_are_exempt`).

A step within reach of a singular point that lies ON the loop is read as
the detour round it and is not among these. Import refuses the same
reading (`unreadable`).

## Owed

A walk whose turn bound follows the carrier locally (as the sampled
distance's row owes), and a statement of what a loop winding twice is:
no face of this kernel, which tier 1 or 2 should refuse before tier 3
reads it. Coupled to
`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`,
whose shared walk this one can become.

Filed by the RESTCURVE implementer in PR 6054's fix pass (adjudication
A2, Q6).
