# Review probes, PR 7808 at 46186823d1

`probes.patch` is the diff to the head, also applied on this branch. Each mutant is an
env var, so one build serves them all. Each row ran as
`cargo nextest run --cargo-profile release --profile ci -p topo` then `-p sweep`
(7227 tests).

| env | mutant | red (topo + sweep) | PR 7799 probes |
|---|---|---|---|
| (none) | | 0 | 0 |
| PROBE_NOSECTV | section vertices NOT named on the carried path (the old flag) | **0** (also 0 attributable over the whole slow set, `--ignore-default-filter`) | n/a (was the default) |
| PROBE_NOSECT | section not named | 14 | 14 |
| PROBE_IGNREM | ignore removed members | 35 | 34 |
| PROBE_JOINTBAD | B's clone cells read unmapped in the joint body | 31 | 31 |
| PROBE_ANYREM | consume on any removal | 3 | 2 |
| PROBE_ANYINC | any pair error is structure | 1 | 1 |
| PROBE_GAP=face/edge/recut + PROBE_PANIC | red / silent consume | 3/3, 12/6, 56/54 | 3/3, 12/6, 56/54 |
| PROBE_GAP=face/edge/recut, no panic | red | 2, 6, 10 | |

Traces (`PROBE_DOORLOG`, `PROBE_UNREADLOG`, `PROBE_PRINT`) on
`a_pierce_held_in_a_curved_face`: 27 door calls, all vertex-in-edge, none
vertex-vertex; every one Zero. In six of them a second meeting merged into a
row `reduce::pend` already held. Two of those six kept the smaller margin
(e.g. kept 2.29143174e-13, dropped 2.29143577e-13). The stress row's 0.03°
`Unread` is `CensusEscalated Indeterminate` (enclosure [4.6e-13, 1.4e-3],
`curve_carrier_straddle`). Its 0.007° refusals are `CurvedPierceEscalated
WallRoots(Discriminant)` at a margin of 7.6e-9, in the band [1e-9, 1e-8].
