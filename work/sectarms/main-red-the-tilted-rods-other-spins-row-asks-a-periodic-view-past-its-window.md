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
