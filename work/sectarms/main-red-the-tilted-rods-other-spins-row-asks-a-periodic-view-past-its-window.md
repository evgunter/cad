---
id: main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window
kind: issue
title: Main red: the tilted rod's other-spins row asks a periodic chord's view past its window
status: open
opened: 2026-11-17
priority: P2
cost: M
---


Filed by ONEHOME's fork 275 step 3b, whose workspace suite found it red
on main.

**Red on main.** `crates/sweep/tests/germ_tilted_rod.rs:369`,
`other_spins_and_steeper_tilts_refuse_at_their_germs` (in the slow set,
about 20 minutes in a debug build), is red at the merge of PR 7048
(`0c2b761e91`), after PR 7052 put the periodic cubic (`Curve3::Periodic`)
on main. At ε 1e-9 two of its eight poses read off their pinned stops
(`OTHER_SPINS_STOPS`, `germ_tilted_rod.rs:306`):

- β 0.5, spin 1, first op: `Euler(RebasedCarrier { edge: EdgeKey(15v1),
  error: PeriodicView(Span { asked: (-0.36108014447909237,
  0.6389198555209077), window: (-2.0, -0.0) }) })`, where
  `"loop guard Torus × Cylinder"` is pinned;
- β 0.6, spin π/2: `"JoinPairsTied"` on ops 1, 2 and 4, where the loop
  guards are pinned.

**The view asked past its window.** The error is
`PeriodicNurbs3::clamped_view` (`crates/geom/src/curves/periodic_nurbs.rs:586`)
refusing a forward span one unit long that lies wholly outside the
reversed view's window `(−2, 0)`, raised through the rebased edge's
certification (`crates/topo/src/euler_kill.rs:1242`). An edge's
parameters on a periodic chord are named in a turn the view is not
seated in.

**It moves with the foot.** On ONEHOME's step 3b branch
(`onehome/span-foot`, the curve pair's point feet read over the span),
the same row reads only β 0.5, spin 0.5 off, with the same error
(`asked: (-0.45617739171656774, 0.5438226082834323)`, window
`(−2, 0)`), and the other two poses read their pins. Which pose reaches
the unseated span follows the foot a parameter is read from, so the
defect is the turn the parameters are named in, not any one pose.

**What would close it.** Seat the rebased edge's span in the view's
window before the view is asked (a period's shift, as main's seating
does elsewhere), or name the parameters in the window's turn where the
foot is read, and pin it on the three poses above.

**Seen again (2026-11-21, GERM's `germ/crossing-consumers` lane, U3b).** Red at ε 1e-9 on that lane's base, `germ/crossing-pose-cross` at `4303b022e8` (692 s), the same first cell as on the lane's head: `β 0.5, spin 0.5: Euler(RebasedCarrier { edge: EdgeKey(15v1), error: PeriodicView(Span { asked: (-0.456, 0.544), window: (-2.0, -0.0) }) })`.

## Red at every ε row, one class (GREEN U9, 2026-11-22)

(GREEN, `green/held-reds`) This row is the only live one naming
`sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`;
two lanes had cited it to GREEN's closed
`main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`.
The nightly of main `1942290ead` (run 44989234943) has it red at all
three ε, each time the view asked past its window:

- ε 1e-9: β 0.5, spin 0.5, op 1, `RebasedCarrier { edge: 15v1,
  PeriodicView(Span { asked: (-0.45617739171656774, 0.5438226082834323),
  window: (-2.0, -0.0) }) }` (the span-foot reading above, now on main);
- ε 1e-6: the same pose, op 1 the same edge (`asked: (-0.456179…,
  0.543820…)`), and op 5 edge `20v1` asked `(-0.543820…, 0.456179…)`
  against the window `(0, 2)`;
- ε 1e-12: β 0.5, spin 0.5, op 5 edge `20v1` (`asked: (-0.543822…,
  0.456177…)`, window `(0, 2)`), and β 0.5, spin 1, ops 1 and 5
  (edges `15v1` and `20v1`).

So the fix this row names greens the 1e-6 and 1e-12 rows too, and the
pin it asks for is per ε.

On current main (`5ea1ac0a70`, hosted, throwaway run 45051677551) it
is still red at the default ε with the ε 1e-9 message above, to the
digit.

## Seen in the crossed-cylinder row at 1e-12 (2026-11-25, SSIARITH)

`sweep::all verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly`
at ε 1e-12 stops on its second pose, two cylinders on skew axes
(radius 1 upright over z ∈ [0, 6], radius 0.8 over [−5, 5] turned 0.7
rad about x and moved to (0.9, 0, 3)): `skew axes, crossing walls/intersect:
ERR Euler(DescriptionUncertified { edge: Keyed(EdgeKey(17v1)), error:
PeriodicView(Span { asked: (-0.98875…, …) … }) })`, nightly of main
`6a680cb845` (run 45332993102). The view is asked past its window as
in the poses above, raised through an edge's description rather than a
rebase. On main `6003877ad8` the same row passes whole at 1e-12
(release, 361 s), so this pose no longer reaches it there; the merge
between was not named. Recorded as one more pose the class reached,
not as a live stop. — (SSIARITH orchestrator)

