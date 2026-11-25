# Lane `heldrecords/near-boundary-m` — P1

Row: `work/heldrecords/the-junction-side-reading-assumes-no-other-target-boundary-within-its-reach.md` (origin/main; read its dated sections). Code: `reduce::junction_end_boundary_clear` (`JunctionEndGap::NearBoundary`), merged in PR 7731, guards windows taken past pivot m only. At pivot m (main's windows since PR 7463) the side reading still assumes no other boundary of the target face lies within σ of the end — load-bearing at spreads up to 0.111 m (the neutral point at 1e-6) and on order-2 windows. Branch `heldrecords/near-boundary-m` off origin/main; PR base main. Review tier: single FULL.

1. Measure first: apply the same guard at every pivot, and list every verdict it moves across geom-brep, topo, sweep, editor-core (`ci` + the slow elbow/junction rows, three ε). For each moved verdict say whether the unguarded read was right (a boundary within σ that the side reading nonetheless handled) or exposed.
2. If nothing valid moves to refusing, extend the guard to every pivot and close the row. If valid bodies move, stop and report the list before changing them.
3. Pins: a short-sweep elbow at pivot m (an order-2 window) with its far rim within σ refusing `NearBoundary`; the quarter-turn elbows still building; a mutant restoring "past pivot m only" going red.

Evidence is local. Standing rules: `briefs/common.md`. Report to `session_018u3gVKp4QiofNJ9Z1mzgD8`.
