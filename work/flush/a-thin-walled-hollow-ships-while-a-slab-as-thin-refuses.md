---
id: a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses
kind: issue
title: A thin-walled hollow ships while a slab as thin refuses: the door reads a sliver per shell, never per solid
status: dispatched
opened: 2026-10-11
priority: P2
cost: M
refs: [a-result-that-is-only-a-sliver-shell-passes-the-door, 4636]
branch: flush/thinness-by-pairs
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
