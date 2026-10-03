---
id: a-settled-declared-coincidence-crosses-a-tight-volume-bound
kind: issue
title: A declared coincidence the door settles inside the band moves a correct result past a tight volume bound, and the backstop refuses it ResultVolumeImplausible
status: open
opened: 2026-10-02
priority: P2
cost: H
design: true
---


Found on PR 3844 (`reach/door-backstop`). The reviewers of that PR's
dual review ruled the shipped fix unsound, and it was taken out.

## What

Every volume-backstop bound is tight somewhere a correct result
reaches: ∩ ≤ B when B ⊂ A, ∪ ≤ A + B when the operands only touch,
∖ ≥ A − B when B ⊂ A. When the door glues a declared coincident pair
onto one carrier, the face it drops may stand off by anything inside
the band. The correct result's volume then moves by up to that gap over
the glued face. At a tight bound, that crosses it, and arm 1 (a
violation certified at the exact band, re-derived in interval
arithmetic) refuses `ResultVolumeImplausible`. The refusal text says
kernel defect, about a correct body. It is the safe direction.

## Measured

Fixture: `crates/topo/tests/door_backstop_settled_residue.rs`. A
`3 × 4.5 × 1` block, and a parallelepiped cornered on its top face by a
5° wedge angle with that face tilted by `θ` about the wedge's edge. The
pair is declared (`Rest` standing on the block, a continuation sunk
flush into it). Oracle: box arithmetic; the gap's volume is
`½·|θ|·sin² φ` (4.6e-12 m³ at 1.2ε).

| pose, θ | ∪ | ∩ | A ∖ B | B ∖ A |
|---|---|---|---|---|
| standing, +0.3…+1.2ε | refuses `vol(A ∪ B) ≤ vol(A) + vol(B)` (+1.74e-12 m³ at 1.2ε) | empty | 13.5 | wedge |
| standing, −0.3…−1.2ε | builds, −1.74e-12 | empty | 13.5 | wedge |
| standing, ±1.6…2ε | `RestZipUnsupported` | empty | 13.5 | wedge |
| sunk, +0.3…+1.2ε | 13.5 | builds, −1.74e-12 | builds | empty |
| sunk, −0.3…−1.2ε | 13.5 | refuses `vol(A ∩ B) ≤ vol(B)` | refuses `vol(A ∖ B) ≥ vol(A) − vol(B)` | empty |
| sunk, ±1.6…2ε | 13.5 | `JoinDesync` (`work/join/…join-desync`) | `JoinDesync` | empty |

At ε = 1e-12 the residue scales down to ~1e-15 m³. There the sunk ∩
still refuses, but the ∪ and A ∖ B crossings (13.5 m³ bodies) fall
under the interval's own rounding and build within the gap. The row
pins both readings.

## Why the shipped allowance was removed (PR 3844's dual review)

The first fix widened every margin by `band.escalate()` × Σ over
declared pairs of the smaller face's area. Both reviewers showed by
execution that this is a VOLUME, so it forgives a defect of that size
ANYWHERE in the body, not where the declared pair could move:

- The MAJ-1 fixture: a 2 m plate keeping a wrong 3 mm cube passes ∩
  and ∪ once the plate tops are declared. At ε = 1e-9 the pass/refuse
  edge is exactly ΔV = escalate × 4 m²; at 1e-6 it forgives a 3 cm
  cube.
- The rounded stack: its 8–9 detector declarations let a ∩ result
  thickened by δ = 1.4e-8 m pass, a 6.9 mm cube's volume at 1e-9.
- Duplicates: declaring one pair 100× (or 10⁶×) multiplied the
  allowance. The door accepts duplicates (`DeclaredPairs` collapses
  them; the sum did not).

A tightened volume still forgives a defect of its size elsewhere. That
is the magnitude-only posture the backstop's arm 1 exists to reject.

## What would close it (design)

The forgiveness has to be LOCAL to the declared face. A correct result
may stand off a bound only by what its glued face's displacement
encloses, and only there. Possible shapes, each to be weighed:

- Decide the bound per face region. Subtract the glued face's own flux
  contribution and its operand counterpart, and bound their difference
  by the band over that face. Every other region keeps the exact
  sign.
- Make the glue exact. Re-place the dropped face onto the kept carrier
  before the zip, so the result's volume carries no residue at all.
- Read the crossing in the refusal. Keep refusing, but name it a
  settled-residue crossing rather than a kernel defect.

Measure each against the MAJ-1 fixture, the rounded stack and the
duplicate-declaration attack above before choosing.
