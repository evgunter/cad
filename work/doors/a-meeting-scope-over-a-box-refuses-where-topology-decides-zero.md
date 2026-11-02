---
id: a-meeting-scope-over-a-box-refuses-where-topology-decides-zero
kind: issue
title: A clearance between scopes that meet refuses box_dependent over a box where topology decides 0
status: closed
opened: 2026-11-02
priority: P2
cost: E
closed: 2026-11-02
---


Found by the correctness re-verify of PR 6292 (R1, sure, probed;
https://github.com/evgunter/cad/pull/6292#issuecomment-6574195890). A
regression that PR's fix pass introduced: a pin face asserted against
itself over `r ±0.05` (`≤ 0`, `≤ 0.01`) refuses `box_dependent`
("reaches [0, 0.2136]"); at `r ±1e-4`, `≤ 0` still refuses ([0, 4.2e-4]).
At the PR's first head these read `Within`, which is right: the topology
fixes the distance at 0 at every realization.

Cause: the meeting candidate's stations go through
`Enclosure::separation` (`topo/src/clearance.rs`), which evaluates one
station of one face twice as independent intervals (`S(u,v) − S(u,v)`,
the dependency problem). The refusal's "a narrower box can" is false,
and the drive's cue bisects to its depth floor for nothing.

Repair: a meeting candidate's `Within` belongs to the topology —
`meeting` is already known where the candidate is offered — so decide
it there, before the enclosure. Who hits it: a contact requirement
(`= 0`, `≤ small`) between scopes that meet, on a part whose geometry
varies with a parameter.

## Closed

PR 6354. A meeting candidate's station pair (the vertex both faces
share, or one vertex of the one face) is read as separation exactly 0
in `Enclosure::separation`, so an upper bound between scopes that meet
is `Within` over a box; every other pair still goes through the
interval enclosure. Sound as a universal proof because the meeting is
topological, fixed over the lane's box: the shared point exists at
every realization. Orchestrator's read (E).
