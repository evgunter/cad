---
id: a-settled-declared-coincidence-crosses-a-tight-volume-bound
kind: issue
title: A declared coincidence the door settles inside the band moves a correct result past a tight volume bound, and the backstop refuses it ResultVolumeImplausible
status: parked
opened: 2026-10-02
priority: P2
cost: H
design: true
blocked_on: [intent-stage4-is-built]
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

## Measured again on PR 3977 (the confirm reads each plane off its loops)

PR 3977 re-derives the backstop's confirming margin about a corner of
each body, with every planar face's flux taken off its own loops
(`quad_lane::planar_face_about`: the loop fanned from one of its
points, so the faces sum to a closed surface's volume). On this
fixture the glued face's loop points stand off its carrier inside the
band, and that standoff times the face's area moves the re-derived
volume by ~8e-11 m³ at ε = 1e-9 (the result ∪ reads
13.58715574282784 against the walk's 13.587155742749397), some 17× the
gap. Taken off the stored plane instead, the same face moved it by
−7.85e-11. The crossing the backstop is asked to confirm is smaller
than the representation's own ambiguity, so which crossing confirms
follows the standoff's sign:

| pose, θ | ∪ | ∩ | A ∖ B |
|---|---|---|---|
| standing, +1.2ε | refuses at 1e-9 and 1e-6, builds at 1e-12 (unchanged) | empty | 13.5 |
| sunk, −1.2ε and −2ε | 13.5 | builds within the gap at all three ε (main: refuses) | refuses at all three ε (main: builds at 1e-12) |

The PR 3844 attacks were re-run on that tree at 1e-9, 1e-6 and 1e-12:
the MAJ-1 plate refuses every planted component down to a 2.7e-17 m³
cube at 1 mm scale, with or without declarations; the rounded stack's
thickened ∩ passes at no δ; declarations, duplicated or not, no longer
reach the backstop (`gate_volume_backstop` takes none).

### The confirm also depends on the fan's anchor (PR 3977's last pass)

PR 3977's second delta review re-measured the result bodies of this
fixture in exact rationals of their stored data and found the confirm's
reading depends on WHICH loop point each plane is fanned from, not only
on the standoff. The glued block-top face's ring stands δ = 1.0e-10
(θ = ±1.2ε) to 1.7e-10 (±2ε) off its carrier at ε 1e-9, so its `A⃗`
has an off-normal part of order δ × its size, and `(anchor − c)·A⃗`
moves with the anchor: over the outer loop's corners by 0 or +1.57e-10
(in V), over the ring's by −3.90e-10 or +8.02e-11, against crossings of
±8.0e-11 and a gap of 4.6e-12. Anchoring the fan at the loop's second
point (same geometry, carrier and centre) flipped every refusal the row
then pinned. Two bodies storing one boundary from different first
points re-derive values up to `Σ δ·|A⃗|` apart (1.4e-9 m³ on the
result here).

So the row (`door_backstop_settled_residue`) now asserts only what the
geometry decides: every op that builds is within the gap of the box
arithmetic, and only an op whose intended result crosses may refuse,
naming that bound. Whether it refuses is not pinned. Any design for
this item has to read the bound at a resolution the representation
supports, which on this fixture is coarser than the gap.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the residue comes from the door bridging a declared pair inside the band; under D10 in-band refuses and only Zero glues, so the class is redefined at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## A curved instance inside the zero band (TANG, 2026-10-08)

The tube of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
at radius `R − k·zero`, unioned with `dome_on_the_cap()`, its discs
declared `Rest`, at `k = ¼` and `½`. The dome's disc overhangs the
tube's by less than the zero band, so this is the Zero glue that D10
keeps, not an in-band one. Both member orders refuse
`vol(A ∪ B) ≤ vol(A) + vol(B)` at every ε row. At ε 1e-9, `k = ½`: got
6.971041463505098 (order 0) and 6.971041464552296 (order 1), bound
6.971041462457901. The same offsets outward (`R + k·zero`) build in both
orders. Pinned by
`a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders`.
