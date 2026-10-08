---
id: a-nested-pairing-at-a-shared-vertex-refuses-shared-vertex-crossings
kind: issue
title: A vertex pair whose pairing nests in B's walk order, at a B vertex another crossing pair shares, refuses SharedVertexCrossings (102 pinch-battery lines)
status: review
opened: 2026-10-07
priority: P1
cost: M
refs: [a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another, a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch]
branch: join/nested-pairing-shared-vertex
pr: 4274
---


## What

Found while building
`a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another`.
It is pre-existing; that build does not move these lines.

`join_pierce_runs_sweep::pinch_runs_battery` (`notch343` against a
two-cube corner pinch) refuses `SharedVertexCrossings { operand: B }`
on 102 lines: every op at 34 poses with the pinch first (`ba`). There
the notch's corner is B's vertex, and both pinch cubes' vertices pair
with it. One of those vertex pairs crosses six times and pairs nested
in B's walk order (`insert::b_runs`). `insert::reconcile_pass` refuses
any nested plan at a shared vertex up front: turning a nested run to
clear the other pair's cuts would make it hold the rest of its own
plan. Example: `i=0 j=0 k=2` (`psi = 2.2`), pinned by
`join_pierce_runs_sweep::a_nested_pairing_at_a_shared_vertex_refuses_typed`.
With the notch first, the same poses build `SOUND`.

## Candidate route (unmeasured)

`insert::hang_in_turned` now places a run that a turned run of its own
plan holds: it mints at the turned run's copy. A nested plan already
mints a held run at its holder's copy (`Held`, `b_runs`). So the
refusal might lift if the reconcile reads a nested plan's runs as one
laminar family: turn only an outermost run, and nest the rest under the
holders the two readings give. That needs a depth above one and
holders from both readings, which nothing builds today.

## Built (branch `join/nested-pairing-shared-vertex`)

**The pin's pose, traced.** At `i=0 j=0 k=2`, pinch first, the
notch's corner `v` is B's, with an orbit of four entries. The other
pair cuts it with one In strut in entry 2. The six-crossing pair's
runs, as arcs of B's walk of its six germs, are:
- a fan from 1 to 4 (Out), which holds a strut from 2 to 3 (In);
- a fan from 5 to 0 (Out).

In `ba U` and `ba S` the outer fan holds the other pair's strut, and
turning it onto its complement (4 to 1) clears it. The complement
leaves its strut outside and holds the other fan. In `ba I` nothing
turns: the fans run 0 to 1 nested (In holding Out), clear of entry 2.
The only obstacle was the reconcile's up-front refusal.

**One laminar reading.** A run is the arc of its solid's walk between
its germs' positions, now recorded on `SideRun::walk`. Turning a run
keeps its ends and takes the complementary arc. So B's nesting and the
reconcile's turns are one family of arcs. Two arcs in it are disjoint,
nested, or each holds the other's ends; in the last case they cover the
walk.
- `insert::arc_holders` reads each run's innermost holder off the
  arcs. It refuses a cover, and an arc that holds one end of another
  (a crossing).
- `insert::held_by` turns the holder chain into `Held`: the depth, the
  innermost fan, and whether a strut holds the run directly.
- `b_runs` reads its holders through it at the plan.
- `insert::hang_at_shared` reads them again after the reconcile at
  every shared vertex, so a turn re-roots the family. This replaces
  `hang_in_turned`, `sibling_holds` and `sibling_holders`; the
  geometric count of cuts goes.
- `reconcile_pass` no longer refuses a plan nested in B's walk order
  up front.

No new design: `Held` already carried depth, and the mint already
hangs a run at its innermost fan's copy.

**Refusal left:** a turned run and a run of its plan that held it
cover the walk, `SharedVertexCrossings`. No battery line reaches it;
the unit test pins it.

## Measured (release, main `8fae267e` vs this branch)

- `pinch_runs_battery`: the 102 `ba` lines (34 poses × U, I, S) go
  `SharedVertexCrossings` → `SOUND`, with the oracle's volume and t2,
  t3′, the certificate and the operand check passing. Each one holds
  one vertex per cone at `v` on one key and meshes
  (`pierce_point_finding` over all 3 024 runs). The other 2 922 lines
  are byte-identical. Their 468 `SOUND` bodies on two keys are the
  same on main, and are the ones counted on
  `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys`.
- `four_pairs_battery` (5 333 lines), 307 moved, all from
  `SharedVertexCrossings`:
  - 182 go to `SOUND`, one vertex per cone on one key;
  - 120 go to `PinchConesOnSeparateKeys`, the parked D10 class's typed
    refusal (example `four i=0 j=6 t=0 k=0 ba S`);
  - 5 still refuse `SharedVertexCrossings`, now in the reconcile with
    the blocking pair named (`four i=2 j=1 t=1 k=2 ba I`).
- Byte-identical: `pierce_runs_battery` (4 536), `corner_pairs_battery`
  (16 380), `join1_r1_reflex_battery` (1 152), `rc_wide_battery` at
  shards 0, 13, 27, 41, 55, 69 and 83 of 84 (3 360).
  `three_pairs_whose_hang_leaves_the_point_on_two_keys_refuse_typed`
  (`three`, `tripod`) passes on both.
- refusal → `BAD`: 0. `SOUND` → refusal: 0.

## Fix pass (PR 4274's dual review)

**MAJOR (r1 M1): a strut chain left at a shared vertex reached the
In/Out-end invariant.** The pose is `wedge343×wedge330` with a cube
pinched on the posed corner, `ba` U and S, at 42 of 48 cube poses.
- **What the trace showed.** The reconcile turns both fans of the
  eight-crossing pair. That leaves its strut chain `[2,5] ⊃ [3,4]` at
  the shared vertex, and the cube's strut nests in both struts
  (`holds_whole`).
- **Why it broke.** `mint_plans` sorted held runs after every unheld
  one. So `[3,4]`, held only by a strut, minted after the cube's strut.
  The cube's strut then hung at the tip of `[2,5]`, the only holder
  hung so far, and that tip took both an In end and an Out end.
- **The arcs were right; the mint order was wrong.** A run that only
  struts hold mints at the plan's own vertex. So it now sorts among
  that vertex's struts by its geometric nesting depth, which already
  counts its own strut holders. A run that a fan holds keeps sorting
  after its holders, at the fan's copy.
- **The 84 lines now build `SOUND`**, each with one vertex per cone on
  one key, and each meshes.
- **The `mint_plans` comment (r1 S2) was false**, and is rewritten. The
  reconcile clears a holding fan of every other pair's cut, but a
  holding strut may nest another pair's strut (`held_cut` leaves nested
  struts to `holds_whole`).

**Pins.** `a_strut_chain_at_a_shared_vertex_nests_another_pairs_strut_and_depth_three_hangs`
covers two wedge poses:
- `t=0 a=0 k=0` is the M1 pose;
- `t=2 a=3 k=0` has the depth-3 chain fan ⊃ fan ⊃ strut ⊃ strut, with
  `by_strut` set.

Four mutants each turn it red: restoring the old sort key, and three
confined to `hang_at_shared` (`depth1`, `nostrut`, `outer`). r1's probe
is kept as `eight_crossings_with_a_pinched_cube_battery`.

**MINORs and style.**
- `arc_holders` returns which refusal fired (`ArcRefusal::Crossing`,
  `ArcRefusal::Cover`). `b_runs` maps a crossing to `PairingMismatch`.
  `hang_at_shared` maps a cover to `SharedVertexCrossings`, documented
  as a backstop no known pose reaches, and a crossing to
  `ClassificationInvariant`, since a turn keeps a run's ends.
- `NullPlan::walk_len` carries the walk's length, asserted equal to
  `2 * runs.len()` in `hang_at_shared`.
- A `debug_assert` checks that every run's holders nest.

**Re-measured** (release, main `7266fa33` vs this branch's head):

| battery | moved | moves |
|---|---|---|
| `pinch_runs_battery` | 102 | `SharedVertexCrossings` → `SOUND` |
| `four_pairs_battery` | 307 | → `SOUND` 182, → `PinchConesOnSeparateKeys` 120, → `SharedVertexCrossings` 5 |
| `eight_crossings_with_a_pinched_cube_battery` (r1's probe) | 276 | → `SOUND` 276, all one vertex per cone on one key |
| r2's `many_pairs`, 4 cubes | 61 | → `SOUND` 50, → `PinchConesOnSeparateKeys` 10, → `SharedVertexCrossings` 1 |
| r2's `many_pairs`, 5 cubes | 1 | → `SOUND` 1 |
| `corner_pairs_battery` | 0 | byte-identical |

→ `ClassificationInvariant`: 0. refusal → `BAD`: 0; the `BAD` lines
(24, 8 and 287) are byte-identical on main.
