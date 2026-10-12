---
id: a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses
kind: issue
title: A thin-walled hollow ships while a slab as thin refuses: the door reads a sliver per shell, never per solid
status: open
opened: 2026-10-11
priority: P2
cost: M
refs: [a-result-that-is-only-a-sliver-shell-passes-the-door, 4636]
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
  `escalated` is folded back into `finished_body_refusal`, its one
  remaining caller.
- `AtRestBody::slivers` stays, narrowed to the solids it lists
  (`Vec<SolidKey>`). CLEAVE's operand orientation probe
  (`boolean/reduce.rs` `gate_sliver_orientation`, PR 4725) reads it, and
  the orientation probe is all check 7 is now. `PlusVVerdict::Sliver`
  drops its certificate, which only the retired refusal's diagnostic
  read. `InBandShell` is check 10's alone again, the in-band shell D10
  now defines.

On this branch, at ε 1e-9:
- **Below Kε.** The slab, the hollow and the fin refuse on the pair
  above.
- **10ε.** The slab and the hollow build. The fin refuses
  `SelfCheck(RingWinding)` at margin 9.99999985e-9, which rounds just
  inside the band.
- **12 to 21ε.** All three build, and `validate_pseudomanifold` over
  each result's own contacts passes.
- **The 1 mm cube ∪ plate** builds in both orders, census clean.
- **The sliver-only rows** now build, judged by their pairs:
  - d = 3e-8 cube ∩ piece and d = 3.05e-8: every pair is definite, so
    each is a legal needle.
  - d = 1e-8, both orders, and d = 3e-8 piece ∩ cube: the at-rest census
    escalates `pm_census_ee_span` (margins 1.4e-9 to 2.8e-9), an edge
    pair in band. The door runs no census, so they ship. The missing
    read is the census at the door. That is REACHHOLD's
    `boolean-door-runs-the-census-over-its-result`, and the evidence is
    on that row.

Tests (`topo/tests/sliver_shell_role.rs`):
- `a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it`. It
  replaces the slab test and the hollow test, which shared its fixtures.
  It covers the slab, the hollow and the fin, with each pair refusal's
  subject and margin asserted.
- `a_millimetre_cube_on_a_thin_plate_builds`.
- `a_result_that_is_only_a_needle_is_judged_by_its_pairs`. It replaces
  `a_result_that_is_only_a_sliver_refuses_in_band`, and pins the census
  verdicts above.

`missed_ray_side.rs`
`a_thin_slab_is_a_legal_operand_against_a_brick_far_away` keeps its
assertion. Its doc now names check 10 as the refusal's source.
