---
id: the-torus-arm-at-a-finer-first-guess-is-measured-on-two-bodies
kind: issue
title: The torus arm took less work at 0.9 δ than at δ with the correction, on six faces of two bodies: a measurement owed on a wider torus corpus, not a factor
status: open
opened: 2026-11-23
priority: P4
cost: C
---


Filed by `a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta` (stage 1, 2026-11-23), on the orchestrator's ruling.

## What was measured

Over `mesh` and `sweep`'s meshing modules at ε `10⁻⁹` (release, one thread, nothing else running), the trimmed lane's 109 torus faces took 0.69 of today's time at δ with the correction and 0.53 at a 0.9 δ first guess. Every other chart took more at 0.9 δ than at δ (cone 0.56 against 0.52, cylinder 0.75 against 0.74, sphere 0.60 against 0.56). All six torus faces that take a round at δ are two bodies at δ `10⁻³`, `reach_probes::a_ring_stored_a_period_away_meshes_where_the_door_moves_it` and `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`; at 0.9 δ neither takes one.

`[mesh.sizing]` says "No safety factor", and on every other chart a finer first guess moved the corrected faces rather than removing them. So the arm sizes at δ.

## Done when

The same comparison is taken on a torus corpus of more than two bodies (spiric-bounded walls, rings at several placements, a few δ) and says whether the torus arm is the one chart where a finer first guess is less work. If it is, the answer is a stated first guess from that corpus's measurement, argued as `[mesh.sizing]`'s "slightly past it", not a factor.
