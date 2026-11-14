# EMBED — log

- 2026-11-14 — **Cut from CLEAR** at Ev's request in chat. CLEAR stood at 41 of 30 budget points.
  - **What moved:** 20 rows, by `git mv` with ids and bodies unchanged:
    - the at-rest gate unit and its f64 cavity switch;
    - the engine-reading rows (the curved corner, the presentations, the extents);
    - the soundness rows (the half-sphere, the graph separator, the sliver).
  - **What CLEAR kept:** the clearance engine's metric side.
  - **The history before the cut** is in `work/clear/log.md` (2026-11-12 to 2026-11-14): PRs 6880, 6885, 6886, 6892, 6893, 6894, 6901, 6902, 6905, 6907.
  - **The measuring harness** that those lanes used is `clear-gate-harness.patch`, in the CLEAR orchestrator's session scratchpad. The scratchpad does not persist. Re-derive the harness from the at-rest gate row's `## Measured` section: an example walking `demos/tour`'s `walk_tour`, asking `topo::clearance::embedded` per body.
  - **Handed back `ready`.** (CLEAR orchestrator)
