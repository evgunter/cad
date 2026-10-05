---
id: a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another
kind: issue
title: A vertex two crossing pairs cut refuses ClassificationInvariant: the In end of one null edge and the Out end of another
status: open
opened: 2026-10-05
priority: P1
cost: M
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
