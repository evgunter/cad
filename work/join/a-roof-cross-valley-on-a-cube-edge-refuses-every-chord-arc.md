---
id: a-roof-cross-valley-on-a-cube-edge-refuses-every-chord-arc
kind: issue
title: A roof-cross valley corner on a cube's edge refuses JoinDesync 'every chord arc separates a loose scaffolding pair' every op
status: open
opened: 2026-10-05
priority: P1
cost: M
---


## What

Found while measuring branch `join/six-crossing-pairing` against main.
The result is identical on both, so the defect is pre-existing.

The operand is review r2's `valley4`: the union of two roofs whose
ridges cross at `(0, 0, 1)`, so it has a 4-valent valley corner there.
That corner sits on a cube's edge. Every op in both orders refuses
`JoinDesync { what: "every chord arc separates a loose scaffolding pair" }`
at 28 poses, 168 runs:
- r2 `grid`: 48 of 5 040 valley4 runs, e.g. `valley4 edge i=10 j=0 psi=1.3`;
- r2 `psi`: 72 of 2 880, e.g. `valley4 edge i=6 j=0 psik=90`;
- r2 `tilt`: 48 of 2 592, e.g. `valley4 edge frame=1 eps=1e-2 ax=1`.

All of them are edge placements. The `corner` placements and the `hex`
sweep (648 runs) do not refuse.

Probe: `crates/sweep/tests/review_r2_vv_probes.rs` on branch
`join/reflex-corner-vertex-vertex-review-r2`, run as
`R2_SWEEP=grid R2_SHAPES=valley4`. Its oracle is the two roofs' convex
halves, signed.

## Why P1

The operand is itself a union with a 4-valent valley. That is verb
breadth beyond the everyday shapes, and the refusal is typed.

## Owed

Find the first wrong state. This is not the six-crossing class: a
nested pairing refused `PairingMismatch` before the join on main, and
these runs reach the join there.

## Possibly the same cause (PR 4050's review r2, NOTE-2)

A 359° wedge against the mirrored 345° notch (r2's `mnotch`) refuses the same
`JoinDesync` "every chord arc separates a loose scaffolding pair" at
six crossings with nested plans: 18 runs that were `PairingMismatch`
on main. Main gives the same refusal on 96 neighbouring poses, so the
class pre-exists there too. Whether it shares the valley's cause is
unsure. Probe: `crates/sweep/tests/review_sixx_r2_probes.rs` on branch
`join/six-crossing-pairing-review-r2`, `SX_SHAPES=w359 SX_OTHERS=mnotch`.
