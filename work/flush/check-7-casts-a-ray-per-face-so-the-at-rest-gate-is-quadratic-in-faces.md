---
id: check-7-casts-a-ray-per-face-so-the-at-rest-gate-is-quadratic-in-faces
kind: issue
title: Check 7 reads every aim, one ray per face across every face, so the at-rest gate is quadratic in a body's faces
status: open
opened: 2026-10-20
refs: [a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass]
priority: P3
cost: M
---


Found by FLUSH's fin-crown unit (`flush/fin-crown-winding`), which made
check 7 read every aim that decides (D1 tier 3: definitely inside out
by any reading is invalid).

## Measured

`check_7_reads_every_aim_at_a_cost`
(`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`, ignored, release,
ε 1e-9) times `validate_geometric` on a prism over a regular `n`-gon:

| faces | first ray that decides | every aim |
|---|---|---|
| 18 | 1.6 ms | 2.5 ms |
| 66 | 1.7 ms | 12 ms |
| 258 | 6.6 ms | 231 ms |
| 1026 | 47 ms | 5.7 s |

A valid body's rays all read one side, so every face that offers an aim
casts one, and each ray asks every face of the selection
(`boolean::solid_contain::ray_crossings`: the plane hit, then
`point_in_face`) and measures the selection's reach from its aim twice
(`side_reader::beyond`, `selection_reach`). Both are linear in the
faces, so the gate is quadratic in them: about 5.5 ms a ray at 1026
faces. Extrapolated, a 10 000-face import would spend minutes in
check 7.

## What to find

A cheaper ray for the same reading. Two candidates, both on shared
ground (`solid_contain.rs` is INSIDE's and CLEAVE's):
- a per-selection box or ball per face, so that a ray skips a face it
  definitely misses by more than the band;
- the selection's reach taken once about a centre, rather than from
  each aim.

Either moves the aimed rays' recorded verdicts
(`side_reader::FAMILY`), and the ball also moves where each ray starts,
so a fix re-reads the batteries. The reading itself must not change:
every aim that decides is still read.
