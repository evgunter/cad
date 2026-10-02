---
id: blind-d-pocket-subtract-refuses-with-join-internal-words
kind: issue
title: boolean: a blind D-profile pocket (block minus a D rod) refuses JoinDesync from the top face — an internal-desync payload for an ordinary pocket
status: open
opened: 2026-09-25
priority: P1
cost: H
---


## Measured

`Tol::witness()`, public doors. The block `[−1, 1]² × [0, 1]` (one
extruded square) minus a D-profile rod extruded for `1.0` from
`z = z0` — the D is `sweep::test_support::rod_chord_at(0.3)`'s: chord
`x = 0.3`, major arc of radius 0.5 about the origin (the D-rod's own
profile, `review_fillet_h7_r1_probes::d_rod`).

| tool | `z0` | `topo::subtract` |
|---|---|---|
| D rod | `0.5` (pocket from the TOP face) | `JoinDesync { what: "ring-run winding is degenerate (zero enclosed area)" }` |
| D rod | `−0.5` (pocket from the BOTTOM face) | `Join(SectionInvariant { what: "the all-planar join lane reached a conic run edge (the operand gate promises every carrier planar)" })` |
| disc r = 0.5 | either | builds |
| square 0.6 | either | builds |

So a blind round pocket and a blind square pocket build, and a blind D
pocket — the two combined — refuses from both sides, each time in an
internal payload's words rather than a frontier's. The top-face payload
is `crates/topo/src/boolean/join.rs`'s ring-run winding decision
(`bool_ring_run_winding` deciding `Zero`); the bottom-face payload is the
same site `work/contact/axis-coincident-lap-trips-the-planar-join-invariant`
carries for a different pose (evidence added there).

## Why it matters

A D-shaped blind pocket is an ordinary feature, and the ruled fillet
carves its concave creases as of `band/ruled-d-hole-ring-crease` (the
top end of each crease in the top cap's ring, the bottom end in the
pocket floor's outer cycle) — but nothing can build the body to fillet:
the extrude door makes only the THROUGH-hole, and the boolean refuses
the blind one.

## Found by

`band/ruled-d-hole-ring-crease`'s class sweep (a mixed crease, one end
in a ring and one in an outer cycle), probing for a fixture.
