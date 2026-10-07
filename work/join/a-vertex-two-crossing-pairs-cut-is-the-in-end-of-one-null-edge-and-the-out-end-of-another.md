---
id: a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another
kind: issue
title: A vertex two crossing pairs cut refuses ClassificationInvariant: the In end of one null edge and the Out end of another
status: open
opened: 2026-10-05
priority: P1
cost: M
branch: join/pinch-cones-split-at-insertion
---


## What

Found by PR 4050's review r1 (MINOR-3) and diagnosed in its fix pass.
The class is pre-existing on main.

The operand is a pinch: two cubes touching only at one corner, in
opposite octants of one frame, united undeclared. The 343° notch's
reflex corner sits at that point. The point then holds one vertex of
the notch and two of the pinch, so two vertex pairs cross there and
share a vertex. Insertion refuses `ClassificationInvariant` "a vertex at
a shared point is the In end of one null edge and the Out end of
another" (`insert::mint_directed`'s hung-ends check).

Measured over the pierce sweep's 84 directions × 6 turns
(`psi = 1.05 k + 0.1`), every op in both orders:
- **main `6e57858c`:** 149 runs at 42 poses, on plans that do not nest:
  42 each of notch-first U and S, and 23, 19 and 23 of pinch-first U,
  I and S. Example: `i=0 j=2 k=2`, notch first, U and S.
- **PR 4050:** 217 runs. The 68 more are notch-first U and S at poses
  that refused `PairingMismatch` on main. There the notch's vertex is
  shared and the pinch's cube nests its pairing in B.

The error is raised in A, at the vertex the two plans share. In those
68 runs A is the notch, whose runs PR 4050 does not change: they are
consecutive in A's walk, from the start on A's kept side. So the cause
is not the nested pairing.

## Candidate cause (unmeasured)

Each plan picks its pairing start on A's kept side only when it has
more than two survivors (`insert::pairing_start_turns`). A plan with
two survivors runs the way `run_degenerates` leaves it. At a shared
vertex, one plan's run can then take A's kept side and another's the
discarded side, so the shared vertex is the In end of one and the Out
end of the other.

## Pin

`join_pierce_runs_sweep::a_nested_pairing_at_a_shared_vertex_refuses_typed`
pins the notch-first U and S at `i=0 j=0 psi=2.2` (PR 4050). Flip that
row when this builds.

## Hold

The pinch is undeclared, so this is not on D10's held ground.

## Measured

The candidate cause above is false. Measured with
`join_pierce_runs_sweep::pinch_runs_battery` (the pierce sweep's 84
directions × 6 turns, every op, both orders), in release:
- **Reproduced.** 149 at `6e57858c`, split 42/42 notch-first U/S and
  23/19/23 pinch-first U/I/S. 217 at `6c617914`, split 76/76 and
  23/19/23. The example `i=0 j=2 k=2` refuses in notch-first U and S.
- **Traced.** In every one of the 217, `reconcile_pass` turns exactly
  one run. That run is always in the plan with more than two survivors,
  and never in the two-survivor plan. The two-survivor plan's run there
  is a strut (its sense is set by its facing, not its direction) or a
  fan already on the side that holds no other cut. That plan pairs from
  A's kept side (`pairing_start_turns`). One of its runs holds the other
  plan's cut, and the reconcile turns that run alone onto its
  complement. That flips its side against its own plan's other runs, so
  the shared vertex is the In end of one null edge and the Out end of
  another. At `i=0 j=2 k=2 ab U`, the four-survivor plan pairs Out
  1→3 and 3→0, and the two-survivor strut sits at entry 2. Run 0 turns
  to In; run 1 stays Out.
- **Re-pairing from the other start is not the fix.** As an experiment,
  every plan at a shared vertex paired from the side In the other
  operand (where both pinch cones' arcs are disjoint):
  - 45 of the 217 go SOUND (13 + 19 notch-first, 13 pinch-first U);
  - 133 go to `JoinDesync` and 20 to `PinchUncrossed`;
  - the 19 pinch-first I lines are unchanged (that op already pairs In);
  - 414 lines that build SOUND today (with no run turned) go to
    `JoinDesync`, `PinchUncrossed` or `ClassificationInvariant`.

  So each start builds a different subset, and the In start reaches
  the pinch crossing in `zip.rs`, which is held under PR 4057.
- Option 2 of PR 4059's question (one vertex per cone on one point key,
  so no vertex is shared) is the final state in front of Ev on PR
  4057. Options 1 (re-pair a plan whole) and 3 (refuse typed at the
  plan) wait on that ruling.

## Measured (pinch unit, branch `join/pinch-one-vertex-per-cone-build`)

One vertex per cone is built as a split before the zips
(`zip::split_cones`). These 217 lines refuse earlier, at insertion
(`insert::mint_directed`'s hung-ends check), so the split does not reach
them. `pinch_runs_battery` is line-identical on main `f9bf3bca` and the
unit's head: all 217 still refuse `ClassificationInvariant` "a vertex at
a shared point is the In end of one null edge and the Out end of
another". Option 2 for this row would split the shared vertex per cone
at insertion, where the two plans meet. That is not what the unit built,
so the row stays open on its own question.

## 2026-10-07 — the ruling settles the fork

Ev's ruling on PR 4057 (one vertex per cone) is option 2 of PR 4059's
question, so the design question is closed: split the shared vertex per
cone at insertion, where the two plans meet. Flip
`a_nested_pairing_at_a_shared_vertex_refuses_typed` when the 217 build.

## Built (branch `join/pinch-cones-split-at-insertion`)

**Where the two plans met.** Traced at `i=0 j=2 k=2 ab U`. The
notch's corner `v` holds two plans:
- plan 0 has four survivors and Out runs `1→3` and `3→0`;
- plan 1 has two survivors and one strut at entry 2.

`reconcile_pass` turns `1→3`, which holds the strut, onto its
complement `3→1`. That complement holds plan 0's own run `3→0` whole,
but `3→0` still minted at `v`. So `v` was the Out end of plan 1's strut
and the In end of `3→0`.

**The vertex per cone.** `insert::hang_in_turned` runs once the
reconcile settles. At each shared vertex, a run that a turned run of its
own plan holds whole is marked `Held` by that fan. `mint_plans` then
mints it after the fan, at the fan's copy: the existing machinery for B's
nested pairings, now reached by A's runs too. As a result:
- the shared vertex keeps what lies outside the turned run, with the
  other plans' ends;
- the turned run's copy is the vertex its own plan's other runs hang on.

No vertex then takes ends from both sides. `zip::split_cones` leaves
one vertex per result cone, as before. Refusals, typed
`SharedVertexCrossings`:
- two turned runs of one plan;
- a run whose cuts a sibling holds only one of;
- a holder that is itself held.

No battery line reaches any of them.

**Why this route.** The other route nests the held plan's strut under
the four-survivor plan's unturned run, across plans. It builds the same
tree of vertices joined by null edges at `v`, only with `v`'s key on a
different node, so the final state is the same. This route reuses the
turn the reconcile already makes, and it changes nothing at any vertex
where no turned run holds another of its plan.

## Measured (release, PR 4207's head `fb8c7cbb` vs this branch)

- `pinch_runs_battery`: the 217 lines (`ab U` 76, `ab S` 76, `ba U`
  23, `ba I` 19, `ba S` 23) go `ClassificationInvariant` → `SOUND`.
  - Each one's volume equals the oracle's at the printed 9 decimals, and
    tier 2, tier 3′, the certificate and the operand check all pass.
  - Each one holds one vertex per cone at `v` on one point key (175 with
    two vertices there, 42 with one), and tessellates at δ = 0.05 and
    passes `check_mesh` (`pierce_point_finding`).
  - The other 2 807 lines are byte-identical.
- Byte-identical: `pierce_runs_battery` (4 536), `corner_pairs_battery`
  (16 380), `join1_r1_reflex_battery` and `j3r2_r1_reflex_battery`
  (1 152 each), and `rc_wide_battery` ×84 shards (40 320).
- SOUND → refusal 0, refusal → BAD 0.
- `a_nested_pairing_at_a_shared_vertex_refuses_typed`: `ab U` and `ab S`
  flip to `SOUND`. The `ba` runs still refuse `SharedVertexCrossings`,
  filed as `a-nested-pairing-at-a-shared-vertex-refuses-shared-vertex-crossings`.
