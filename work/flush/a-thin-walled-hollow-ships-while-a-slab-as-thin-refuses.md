---
id: a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses
kind: issue
title: A thin-walled hollow ships while a slab as thin refuses: the door reads a sliver per shell, never per solid
status: closed
opened: 2026-10-11
priority: P2
cost: M
refs: [a-result-that-is-only-a-sliver-shell-passes-the-door, 4636]
closed: 2026-10-12
branch: flush/thinness-by-pairs
pr: 4783
---


## What

Found by PR 4636's review (MINOR-2).

D10 (Booleans) says a result holds no in-band shell. The boolean door
reads the sliver band two ways, and both are per shell:
- **check 10**, for each shell of a solid of several shells;
- **check 7**, for a solid's only shell (`AtRestBody::slivers`, PR 4636).

Nothing reads a solid of several shells whose check-7 total is in band
while each of its shells decides a role. That is a thin-walled hollow,
and it ships `Ok`. A slab just as thin, alone in its solid, refuses.

**Repro (ε = 1e-9).** In `crates/topo/tests/sliver_shell_role.rs`:
- `a_thin_walled_hollow_ships_while_a_slab_as_thin_refuses` pins today's
  behaviour. `subtract(unit brick, brick inset δ on every side)` at
  δ = 1e-8 builds one solid of two shells.
  - The outer shell and the cavity each read a definite role.
  - The wall's V/A is about δ/2 = 5e-9, in band.
  - It ships `Ok` on main and with PR 4636. The reviewer's probe shows
    δ = 1.9e-8 to 5e-8 also ship. δ ≤ 5e-9 refuses earlier, as a
    coincidence (`VertexOnFace`).
- `a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds`
  covers the slab: a unit brick ∩ a brick from z = 1 − t.
  - At t = 1e-8 it refuses `Escalated { ShellRole }`, with certified V/A
    [4.9999999e-9, 4.9999999e-9].
  - At t = 2.1e-8 it builds.

## The question

This is a design question: what does "a sliver" measure for a solid?
- **Per shell** (today). A hollow's wall is thin in no single shell's
  measure, so the hollow ships.
- **Per solid.** Check 7's certified total, read on every solid. A
  hollow whose wall is in band would refuse. The cost is zero extra
  walks, since check 7 already reads it, but what the refusal names is
  open: no single shell is the in-band one.
- **Per wall.** The in-band face pairs a thin wall is made of. That is
  D10's "no in-band pair", which no reading at the gate builds today.

## The fork (fork log row 113), converged after three rounds

The designer pair converged on **pairs only**. The boolean door judges
how thin a result is by its pairs of cells alone. The "shell" half of
D10 is check 10's undecided role in a solid of several shells. PR
4636's refusal of a solid's only shell, certified in band by V/A,
retires.
- **The hollow is right; the slab was wrongly refused.** Every wall
  refuses below Kε on a pair and builds above it, whatever shape it is
  part of.
- **V/A is not a measure of material.** On main today,
  `union(1 mm cube, 1 m² plate 1.5e-8 thick)` refuses through PR 4636's
  gate (V/A 8e-9), though every pair is definite.
- **The slab looked unusable as an operand only because of a measuring
  defect.** An op that places a piece disjoint from the slab casts rays
  that miss it, and `solid_contain.rs` `at_infinity_side` then reads the
  side from the solid's volume sign (margin V/A). Filed on CLEAVE as
  `a-missed-ray-reads-the-solids-volume-sign-for-its-side`.

The question is with Ev on an `[ev]` PR, because it re-words D10 and
retracts fork row 101's "the lump refuses `Escalated`". If Ev ratifies,
the unit is:
- remove `AtRestBody::slivers`, `PlusVVerdict::Sliver` and `gate`'s
  `escalated(kept.slivers())`;
- flip `a_slab_thinner_than_two_bands_refuses_and_a_thicker_one_builds`
  and `a_result_that_is_only_a_sliver_refuses_in_band` to build in
  [Kε, 2Kε), and add pair-refusal rows below Kε for the slab, the hollow
  and the fin;
- rename the hollow row's test;
- close this row and re-open the gate half of
  `a-result-that-is-only-a-sliver-shell-passes-the-door`.

## Ruling

Ev ratified **pairs only** on PR 4690 (2026-10-12: "sounds good!"). D10
Booleans now defines an in-band pair and an in-band shell, and tier 3′
check 7 is an orientation probe, not a thinness gate, at every door.
This row is now the implementation unit listed above. Fork row 101's
"the lump refuses `Escalated`" is retracted by fork row 113.

## Measured

On main at `9578813e2`, ε = 1e-9, through the plain booleans. Thicknesses
are in ε; the band is (ε, Kε) with K = 10.
- **Slab** (unit brick ∩ a brick from z = 1 − t), **hollow** (unit brick
  ∖ a brick inset t) and **fin**. The fin is a unit brick ∪ a 0.2 m
  block, less a cutter that leaves t of the block standing 0.5 m above
  the top. It is cut, because the prism door builds no brick under 10ε.
  - At 3, 5, 6, 8 and 9.9ε each refuses on a pair, with margin t: the
    slab and the hollow `Coincidence(VertexOnFace)`, the fin
    `Containment { Loop(Boundary) }`.
  - At 10 to 19ε the slab refuses `ShellRole` through PR 4636's gate,
    with certified V/A t/2. The hollow and the fin build.
  - At 21ε all three build.
- **`union(1 mm cube, 1 m² plate 1.5e-8 thick)`, flush.** It refuses
  `ShellRole` (V/A 8.0e-9) in both orders. At 2.5e-8 it builds.
- **Slab ∪ far cube**, the slab 10 to 19ε thick, either order. It still
  refuses `ShellRole` after CLEAVE's PR 4725. That refusal is check 10's,
  not the gate's: the pieces sort cannot read the slab's role off its
  in-band V/A, so it leaves the slab in the far brick's solid. It is the
  reader of `check-10-reads-a-cavitys-role-from-its-volume-sign`, and
  that row has the evidence. It refuses the same way on this branch.
- **`brick (−1,2)³ ∖ slab(t)`**, 10 to 19ε. It refuses `ShellRole` on
  the cavity through check 10, on main and on this branch.
- **The sliver-only rows** (`a-result-that-is-only-a-sliver-shell-passes-the-door`):
  the `notch307` piece ∩ cube. At d = 1e-8 and 3e-8, either order, it
  refuses `ShellRole` through the gate. At 3.05e-8 it builds, and at
  −1e-8 it is empty.

## Fix

- `boolean/ops.rs` `gate` no longer refuses on `AtRestBody::slivers`.
  `escalated` stays, serving `finished_body_refusal` (check 10's
  in-band shells) alone.
- `AtRestBody::slivers` stays, narrowed to the solids it lists
  (`Vec<SolidKey>`). CLEAVE's operand orientation probe
  (`boolean/reduce.rs` `gate_sliver_orientation`, PR 4725) reads it, and
  the orientation probe is all check 7 is now. `PlusVVerdict::Sliver`
  drops its certificate, which only the retired refusal's diagnostic
  read. `InBandShell` is check 10's alone again, the in-band shell D10
  now defines.
- Docs that called `V/A` a thickness say what it is, a length: the
  volume backstop (`ops.rs`), the missed-ray side read
  (`solid_contain.rs`) and `BooleanDecision::ShellRole`
  (`refusal_routes.rs`). The census probe example
  (`sweep/examples/near_tangent_census_probe.rs`) names check 10 where
  it named the gate.

On this branch, ε 1e-9 and 1e-6:
- **Below Kε** (3, 6, 9.9ε) the slab, the hollow and the fin refuse on
  the pair above, at both ε.
- **At Kε itself** the pair's margin rounds either way: at 1e-9 all
  three build, census clean; at 1e-6 all three refuse on the pair, at
  margin 9.99999999995e-6, a hair under Kε.
- **10.5 to 21ε**: all three build at both ε, and `validate_pseudomanifold`
  over each result's own contacts passes. (A fin *placed* as a 10ε brick
  refuses `SelfCheck(RingWinding)` at margin 9.99999985e-9; the cut fin
  the tests use builds clean at 10ε, as both reviewers found.)
- **The 1 mm cube ∪ plate** builds in both orders, census clean.

**What now ships with an in-band pair.** The door judges pairs, but it
reads only the pairs its own build decides; a pair only the census reads
(an edge-edge span) is not read at the door. With the V/A gate gone,
results the gate refused for the wrong reason now ship, and some of them
hold such a pair:
- This lane's probe: the `notch307 nt e0 a3` piece ∩ cube at d = 1e-8
  (both orders) and 3e-8 (piece first): the at-rest census escalates
  `pm_census_ee_span` at margins 1.4e-9 to 2.8e-9.
- R2's review probe (`crates/topo/tests/review_4783_r2_probes.rs` on
  `flush/thinness-by-pairs-review-r2` @ ef12bed5), across a 512-op
  notch307 family (a0–a15, d from 5e-9 to 4e-8, both orders): 14 ops
  change. **11** now ship with `pm_census_ee_span` in band (a3 d1e-8,
  1.5e-8, 2e-8 and 2.5e-8 in both orders, a3 d3e-8 piece first, a2
  d3.05e-8 in both orders) and 3 ship clean. The same family already
  ships **44** such results on base.
- R1's review probe (the `review-sliver-probe` feature on
  `flush/thinness-by-pairs-review` @ a79ba445), at ε 1e-6:
  `rc_wide_battery` moves **34** lines from `ShellRole` to built
  (`OK BAD … operand=false`, census clean). 25 are the check-10 /
  pieces-sort class; 9 refuse `Containment` in the operand orientation
  probe.

No stop-gap: a census run only on results that `slivers()` flags would
refuse a needle and ship a fat body with the same in-band pair, which is
the V/A ruler retired. The gap is the door census,
`work/reachhold/boolean-door-runs-the-census-over-its-result.md`, which
holds this evidence; the gate half of
`a-result-that-is-only-a-sliver-shell-passes-the-door` is parked on it.

Tests (`topo/tests/sliver_shell_role.rs`):
- `a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it`, at
  ε 1e-9 and 1e-6. It replaces the slab test and the hollow test, which
  shared its fixtures, and covers the slab, the hollow and the fin: each
  pair refusal's subject and margin below Kε; Kε itself (builds clean,
  or refuses on a pair under Kε); 10.5 to 19ε building census-clean.
- `a_millimetre_cube_on_a_thin_plate_builds`.
- `a_needle_ships_an_in_band_edge_pair_until_the_door_runs_the_census`.
  It replaces `a_result_that_is_only_a_sliver_refuses_in_band` and pins
  the census verdicts above; a door census turns its in-band rows red.

`missed_ray_side.rs`'s slab ∪ far brick test is renamed
`a_thin_slab_is_a_legal_operand_against_a_brick_far_away_and_its_union_with_it_refuses`,
since it asserts a refusal. Its doc names check 10 as the source.

Filed: `work/cleave/a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face.md`
and `work/flush/an-inside-out-sliver-result-passes-its-own-door.md`.

## Closed

Built to Ev's ruling on PR 4690: the door judges thinness by pairs
alone. What is left lives on other rows:
- the in-band pairs only the census reads:
  `boolean-door-runs-the-census-over-its-result` (REACHHOLD), with
  `a-result-that-is-only-a-sliver-shell-passes-the-door` parked on it;
- a thin operand check 10 still refuses (`slab ∪ far`, `brick ∖ slab`):
  `check-10-reads-a-cavitys-role-from-its-volume-sign`;
- the operand orientation probe's planar-face dependence and silent
  skip: CLEAVE's row above;
- an inverted sliver result passing its own door: FLUSH's row above.
