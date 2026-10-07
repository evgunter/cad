---
id: a-roof-cross-valley-on-a-cube-edge-refuses-every-chord-arc
kind: issue
title: A roof-cross valley corner on a cube's edge refuses JoinDesync 'every chord arc separates a loose scaffolding pair' every op
status: review
opened: 2026-10-05
priority: P1
cost: M
branch: join/roof-cross-valley
pr: 4272
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

The class is a pinched face with pierce rings on both sides of the
chord, not the edge placement as such. The r2 sweeps reach it only at
edge placements: the `corner` placements and the `hex` sweep (648 runs)
do not refuse. Review `rv` placements reach it with the corner slid
along the edge or pushed generically off it.

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

## Built

The first wrong state is the role-order verdict at the first chord of
valley4's roof face (roof y's x > 0 slope). The scaffolding and the
matching are right, and so is the segment order.
- The face's outer loop is pinched at the valley corner.
- The cube's two edges pierce the face on either side of the chord, so
  each pierce point is a ring of the face.
- Every loose half on both arcs has its partner on one of those rings.
- The old rule read any partner off the arc as walled off. A ring is
  not walled off: the mef's `rehome_rings` moves it to the side that
  holds its segment's other end.

`join.rs` now ranks a run with `capture_rank`: `Clean`, `RingHeld`
(every partner off the arc lies on another ring of the face) or
`Separates`.
- The outer lane (`best_arc`) takes the better of its two arcs and
  prefers `Clean`, so a ring-held arc is taken only where neither arc
  is clean.
- The two forced-order lanes (`ring_order` and the across-edge lane)
  accept only a `Clean` run, which is the rule they had before.

Pins:
- `review_r2_vv_probes::a_roof_cross_valley_on_a_cube_edge_builds` (the
  witness);
- `join_pierce_runs_sweep::the_outer_lane_prefers_a_clean_arc_at_an_exact_tie`,
  which goes red when the outer lane takes a ring-held arc where a
  clean one exists.

## Measured

Main against head, release.

First pass, against main f7e17b0f:
- r2 `grid`, `psi` and `tilt` over 13 shapes, the `mnotch` set, the
  pierce, pinch and corner-pairs batteries, `join1_r1_reflex_battery`,
  and `rc_wide` shards 0, 3, 6 and 9 of 12;
- 174 lines move, all `JoinDesync` → SOUND: r2 grid 48, psi 72 and
  tilt 48 (every valley4 refusal), and mnotch 6;
- nothing else moves.

Fix pass, against main 517b81a2: valley4 `grid`, `psi`, `tilt` and
`rv`, and the corner-pairs battery.
- 246 lines move, all `JoinDesync` → SOUND: grid 48, psi 72, tilt 48,
  and `rv` 78 (`rv-in` 30, `rv-mix` 30, `rv-out` 18, all off the edge).
- The corner-pairs battery does not move (16 380 lines).
- No line goes SOUND→refusal or refusal→BAD.

valley4 `tilt` keeps 33 BAD lines, identical on main and head:
- 12 at eps 0: `StaleContactDeclaration` (8) and `UndeclaredContact`
  (4). These are D10 ground.
- 21 at 1e-6: `CensusEscalated`, filed under
  `near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`.
