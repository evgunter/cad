---
id: m6-sense-gate-recorded-residuals
kind: issue
title: M6-6 sense-gate residuals — the recorded classes outside the gate, with their flip conditions
status: open
opened: 2026-08-07
github: 226
refs: [223, 250]
---

## From GitHub issue 226

Opened 2026-08-07; 0 comments.

Tracking issue for #223's recorded residuals (Ev's ask — recorded AND scheduled). Each is pinned in-tree (the pin's doc names its flip condition; the pin fails the day the condition lands, forcing the honest update):

1. **Conic-trimmed cylinder walls slip both gates** (residual 4, the review's executed counterexample): cut_cylinder's ellipse-trimmed wall — single-wall flip green, whole-body inversion green with positive volume, export-flip-reimport green. Pin: `cut_cylinder_conic_trim_residual_stays_green`. **Flip condition: the ellipse-rim material-side encoding** (extend boundary_material_sign's rim vocabulary past circles). Sequencing: a rider on whichever unit next touches the rim-classification family, or its own S unit if the M6 exit walk wants it sooner.
2. **Rimless-band half-flip invisible** (V=0 Zero-exempt; the ball's single-encoding limit). Pinned as residual at #223. Flip condition: a shell-level orientation check or a second encoding channel — genuinely open design, sized with the next props unit.
3. **NURBS faces bit-free** (winding-derived quadrature — outside the four-kind scope by the ratified unit shape). Flip condition: a NURBS material-side encoding, naturally riding the NURBS-vocabulary growth (stage-1 recognition era).
4. **Arc-bounded planar caps check-6-exempt** (the pre-existing deferral; whole-body cases now caught via circle-rimmed curved walls — but a body with ONLY arc-bounded planar faces + conic-trimmed walls remains uncovered, per residual 1's counterexample). Flip condition: check 6's arc-bounded planar arm. **NEW FACT (VERBS-1031B, 2026-09-03): the class now has a PRODUCER, and that turns this residual from a coverage hole into an assigner/checker divergence.** `merge_faces::loop_winding` learned the arc-bounded winding arm, so `merge_coplanar_faces` now MINTS merged planar faces whose outer loop and rings ride circles (four per teapot cup) and ASSIGNS their outer/ring roles from the `bool_ring_run_winding` predicate — while check 6, the same predicate's third site, still skips exactly those loops. Roles are therefore assigned by a functional the validator cannot check. Measured rather than argued: under VERBS-1031B's MUT-2 (the bulge correction applied backwards) the cup's merge SUCCEEDS with every annulus inside out and `validate_geometric` stays `Ok(())`. The flip condition is unchanged and is now owned, with its refusal-surface cost, by `work/verbs/verbs-1031b-assigner-checker-divergence.md`.

These are walk material for M6's exit (carried items with named owners per the walk discipline); none blocks the walk itself.

## Home

`work/issues/`: the four residuals span `validate` check 6, the props sense gate and the NURBS vocabulary, and no open program's charter claims the set — VERBS cites only residual 1, as VERBS-CONE's known trap.
