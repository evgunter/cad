---
id: split-gate-torus-ring-fallback-has-no-door
kind: issue
title: The split gate's torus fallback for R <= r has no public door: revolve refuses the spindle torus
status: open
opened: 2026-10-03
priority: P3
cost: E
---


Found by PR 3982's dual review (both reviewers), and disclosed in that PR.

`splitting/classify.rs` `torus_window_reach` takes the closed form only
when `R > r` is decided (`split_gate_torus_ring`), because its proof
needs `R + r·cos v > 0`. Otherwise it falls back to the rule's sampled
box (`census::face_reach_in`). Nothing reaches that fallback: `revolve`
refuses a spindle torus (`UnsupportedToroid`, which both reviewers
executed with a lemon and an apple profile), and no other door mints a
torus with `R ≤ r`. A near-spindle fillet (`R − r = 0.02`) takes the
closed form, and it is sound (reviewer R2).

When a door can mint a spindle or horn torus, add the row: a cut clear
of such a face must not be admitted by the closed form. The fallback is
the sound direction. Until then the arm is untested, and the decision
is a guard on the proof's premise, not a measured path.

## Since PR 6336 (BOXES, 2026-11-02)

`torus_window_reach` and `split_gate_torus_ring` are gone: the split gate reads `census::face_reach_in`, whose torus window is the closed form behind a ring guard, falling to the whole tube when the ring is not decided. The no-door gap moved with it: PR 6336's review showed the guard carries weight (a spindle torus R = 0.3, r = 1 under-claims −0.6965 against −0.700 without it) and that no row held it. PR 6336's fix pass adds rows on a spindle description; whether this row's door question still stands after that is GAUGE's to judge. (BOXES orchestrator)
