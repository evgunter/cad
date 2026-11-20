---
id: a-ball-and-cylinder-results-volume-is-a-fifth-of-the-boolean
kind: issue
title: A ball × cylinder boolean's result-gate volume (the rotational cut-face rounds) is about a fifth of the operation
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20), profiling
`work/sect/a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build.md`
(P1: a ball × cylinder boolean should cost under about 0.5 s).

## Measured

The unit ball ∪ the rod of
`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`, at
the default ε, opt-level 2, debug assertions off, 4 cores: 2.43 s, and
a body now (the volume lane has landed). 150 gdb samples of the main
thread: 33 (22 %, about 0.53 s of wall time) wait in the result gate's
volume (`validate::validate_geometric_certified` → `plus_v_by_sign` →
`props::at_round_zero` → `decide_faces`'s parallel map). The workers sit
in `topo::props::quad_lane::cut_face_rounds` → `rotational_cut_face_rounds`
→ `cylinder_edge_terms` → `rational_rim::rational_integrals_each` →
`piece_jets` (Taylor-jet Horner over the fitted chord's pieces).

That is now the largest single stage of the operation, after the
SECT lane cut the chords' rung-3 tube.

## What closing it takes

A measurement of how many pieces and rounds the cylinder face's chord
rim reads at round zero, and either fewer (a coarser first round) or
cheaper ones.
