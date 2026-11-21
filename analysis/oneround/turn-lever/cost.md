# What the cover read costs

Local, 4 cores, debug test profile. Every run is of the same tree pair:
- base: `origin/oneround/lever-sites-topo` merged with `origin/main`;
- head: `oneround/turn-lever` at 568dcc79 (the same merge plus this unit).
The runs were sequential on an otherwise idle box. The failing tests are the
same on both trees (5 in topo, 3 in sweep: main's and the base's known reds),
except that the seed-drawn square-wall fuzzer failed in one base run and in both
head runs.

| Suite | base wall | head wall | head / base |
|---|---|---|---|
| `cargo nextest run -p topo` (4338 tests) | 1668 s | 1771 s | 1.06 |
| `cargo nextest run -p sweep` (3825 tests) | 5336 s | 5541 s | 1.04 |
| topo's boolean rows, `-E 'test(/boolean/)'` (803 tests), two runs each | 126 s, 127 s | 140 s, 139 s | 1.10 |

Hosted CI is no reference here: its kernel job's per-crate times moved by
1.2–1.7× between runs with this unit's diff unchanged, and crates the diff does
not touch moved as much as the rest.

## How often a face's cover is read

This was counted over the boolean rows (803 tests), one line per cover the
`WedgeReach` cache misses:
- 366,329 covers computed, over 12,173 distinct (body, face) pairs per test:
  about 30 computations per face per test (median test 5).
- 4,046 `WedgeReach` instances.
- The worst test: 12,408 computations of one face, in
  `a_slit_tip_at_a_cone_apex_reads_its_full_turn::a_boolean_through_the_apex_at_the_slits_tip_builds`.

By where the `WedgeReach` was made:

| Where | Computations | Why it repeats |
|---|---|---|
| `cover::cone_may_hold_apex` → `pcurves::Lift::decided` → the lift's turns (`WedgeReach::unlifted`) | 143,968 | Every cone face's cover decides its lift, and the lift's singular turns read their faces' unlifted covers, once per lift. |
| `euler::certify_edge_spec` (free `wedge_reach`) | 130,866 | One fresh cache per certified edge, so both faces are re-read for every edge. |
| `euler::certify_rebased_run` (free `wedge_reach`) | 32,012 | As above, per rebased edge. |
| `boolean::ops::describe_edges` | 21,380 | Per edge. |
| `validate` tier 3 (one cache per pass) | 19,473 | Once per face per pass. |
| `boolean::combine::graft_solids_impl` | 13,116 | Per grafted edge. |
| the rest | under 4,200 each | |

The top two rows account for 75% of the computations. The second is
certification, which takes the bound from below only, so the cover it computes
is not read at all.
