---
id: cap-rim-refusal-order-follows-the-arena
kind: issue
title: replace_faces_offset: a cone chart whose rims leave whole-disc caps refuses as the rim edge's RechartFalsifies (no gap) on one frustum and ReanchorOffCarrier (with the gap) on its mirror
status: open
opened: 2026-10-01
priority: P2
cost: E
---


Found by BAND (`band/sweeps-build-one-wall-per-run`), filed from outside
SHELL's fence.

## What

A full revolve now builds a frustum's two discs whole (one face each,
no pole vertex and no meridian across them; `crates/sweep/README.md`,
"Walls: one per run"). Offsetting the frustum's cone chart
(`topo::replace_faces_offset`, both half-bands) by any `|d|` above the
rim tolerance still refuses — the moved rims leave the unmoved caps by
`|d|·sin α` — but WHICH refusal arrives now depends on the frustum:

- widening upward (`common::cone_nappe::opening_frustum`), both signs,
  and narrowing upward at `d > 0`: `ReanchorOffCarrier { gap }`, the gap
  `|d|·sin α` to 1e-15;
- narrowing upward (`mirror_frustum`) at `d < 0`: `Op { error:
  RechartFalsifies { edge, Surface1Residual, sample 0 } }` on a rim
  edge — the same contact, refused at the edge's re-chart onto the cap
  before any rim vertex is re-anchored, and carrying no gap.

When the caps were two half-discs every case reached the vertex
re-anchor first. The rows that measured the gap
(`shell6_nappe_home::the_per_chart_doors_reach_is_a_threshold_in_the_rim_tolerance`,
`…::the_apex_window_gate_fires_on_both_nappes_at_the_same_reach`,
`shell6_r1_probes::r1_e2e_hollow_both_frustums_from_the_consumers_seat`,
`sf2b_r2_probes::r2_per_chart_door_on_a_mirror_nappe_cone`) now accept
either through `common::cone_nappe::rim_refusal_gap` and check the gap
only where one is carried. `shell6_r2_probes`' own frustums refuse at
the re-chart on all four (nappe × sign) cases, so its
`r2p1_the_shipped_gap_row_is_blind_to_the_turn` — which measured the
spread of those four gaps — had nothing left to measure and was
retired; it returns with the gap.

## The question for SHELL

Whether the door should reach one refusal for one contact whatever
order it walks the rim in — the vertex re-anchor first, say, so the
caller always reads the gap — or whether the edge refusal is the
honest first word and should carry the gap too. Either way the five
rows above can return to asserting one variant.
