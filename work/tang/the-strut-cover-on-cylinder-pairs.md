---
id: the-strut-cover-on-cylinder-pairs
kind: issue
title: The strut source of the one-sided cover admits plane x cylinder and two cylinders; sphere x cylinder waits for a witness, and every union reaching it stops at a later door
status: open
opened: 2026-10-02
priority: P1
cost: M
---


## What

`tangency_certifies_side` (`crates/topo/src/boolean/mod.rs`) is the one
table for which tangencies along a curve give the one-sided cover a
global side. Its seam column admits plane × cylinder, two cylinders,
sphere × cylinder, and sphere or plane × torus. Its strut column (an
operand edge described `TangentIntersection`) admits plane × cylinder
and two cylinders.

PR 3849's first fix pass widened the strut column to the whole table.
Its delta review restored the old strut table and measured the effect:
zero outcomes changed over 3,992 tests. The reviewer's two probes, a
strut between two cylinder walls and a strut between a sphere and a
cylinder (the one-profile capsule's joint), refuse with and without the
widening. So the widening was unproven and unexercised, and it was
reverted (fix pass 2).

## The cylinder row has its witness

Two plates stacked with every flush finding declared, the lower one's
outline a big arc running into a small arc tangent to it (authored as
its tangent continuation, so the ruling between the two walls is
`TangentIntersection`): with the small arc inside the big circle,
outside it (an S), and under a plate cut back to the big arc alone.
On main each union refuses `CurvedPierceUnsupported` in both member
orders; with `(Cylinder, Cylinder)` in the strut column each builds,
valid at tier 3 and 3′, at its closed-form volume, at ε default, 1e-6
and 1e-12 (`crates/sweep/tests/strut_cover_on_cylinder_pairs.rs`).
The cut-back stack needs both directions the row certifies.

## Where the sphere × cylinder probes stop

The one-profile capsule (tube `z ∈ [0, 2]`, radius 0.5, half ball on
it, the joint authored as the side's tangent continuation), unioned with
a coaxial rod of its radius whose wall is declared a continuation of
the capsule's, in both member orders:

| Rod | Main | Strut column with the sphere row |
|---|---|---|
| `z ∈ [−1, 2]`, ending on the joint | `CurvedPierceUnsupported` | `Escalated { Coincidence(Sectors, Moot) }` (capsule first), `CurvedPairUnsupported { site: InteriorLoopGuard }` cylinder × sphere (rod first) |
| `z ∈ [−1, 2.25]`, into the half ball | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` (the join's lane dispatch) |
| `z ∈ [1, 3]`, over the half ball | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` |
| `z ∈ [2, 3]`, from the joint | `CurvedPierceUnsupported` | `CurvedBooleanUnsupported` |
| `z ∈ [−1, 1]`, short of the joint | builds | builds |

Undeclared, every capsule and stack pose refuses `UndeclaredCoincidence`
(`carrier_cyl_axis_parallel`) before any cover is read: the strut
column is consulted only for a pair the declaration door verified one
carrier (`DeclaredPairs::build`). `CurvedBooleanUnsupported` is
`join/cylinder-sphere-germ-pair-has-no-join-lane`'s door. The table is
pinned by `the_capsules_strut_waits_at_the_crossing_layer`, which goes
red if the sphere row is admitted.

## The work

The rows are true for a strut as for a seam: the table's doc argues each
one, and `the_table_matches_which_carriers_keep_to_one_side` measures
them. What is missing is a fixture whose outcome turns on them. Find
a union that needs the strut cover on sphere × cylinder once the doors
above move, and widen the strut column's sphere row with it as its
witness. The torus rows were not probed.
