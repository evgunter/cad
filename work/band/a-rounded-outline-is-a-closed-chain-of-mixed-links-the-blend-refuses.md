---
id: a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses
kind: issue
title: A rounded outline's edges form a closed tangent-continuous chain of mixed links, which the blend refuses (junction carry-through)
status: open
opened: 2026-11-02
priority: P0
cost: H
---

Filed by the REFS orchestrator (2026-11-02), from the designers' measurement on `work/refs/select-all-edges-picks-edges-a-fillet-refuses.md`.

A plain rounded-rectangle plate, once its eight smooth cut edges are out of the request (the REFS unit leaves them alone), refuses filleting all its edges with "a closed chain's blend is not a torus … junction carry-through not implemented". Any rounded outline is a closed tangent-continuous chain whose links mix planes and cylinders. Its cuts' margins read ±7e-18, which is still a decided Zero.

Witness, committed: `crates/sweep/tests/pins_of_todays_blend_refusals.rs::pin_today_a_rounded_rectangle_filleted_all_round_refuses_its_closed_mixed_chain` (a [−1, 1] × [−0.6, 0.6] plate with corner radius 0.3; every edge requested leaves the eight struts and refuses `UnsupportedChain` on a cap outline). It pins today's refusal; this row's PR flips it to a build.

- 2026-11-02 — moved from BANDRULE to BAND by `git mv` when BANDRULE merged into BAND, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and BANDRULE had fallen far under it).
