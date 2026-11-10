---
id: the-rotational-lanes-entry-reads-a-many-span-chord-at-four-pieces-a-span
kind: issue
title: The rotational lane reads a projected chord's length and turn at four pieces a span before its rounds, now the larger part of a face's measurement
status: open
opened: 2026-11-10
priority: P3
cost: E
refs: [a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses]
---

## What

Filed by the cylinder-pair cost lane (branch
`spiricarm/cylinder-pair-cost`, 2026-11-10). Its fix spreads a round's
pieces over a projected carrier's spans
(`geom_brep::props::quad::rotational::span_pieces`), so a 625-span chord's
round 0 takes 27–120 ms where it took 1.2–3.1 s. What the lane reads of
each projected edge before its rounds (`edge_entry`) still reads
`PROJECTED_ENTRY_PIECES` = 4 pieces a span: the metric length
(`projected_metric_length`, `rational_rate_bound`) and the turn
(`framed_integrals`, `du` alone). On the fenced cylinder pair those reads
are now most of a face's call, 0.2–0.7 s against a 30–120 ms round
(release), and the tier 3 gate is 1.8–2.3 s of the operation.

Spreading them as the rounds are is not free: a coarser length bound
widens the map-residual and chart-term pads, and a coarser turn reads
wider in `branch_turns`, whose quarter-turn test refuses. PR 6509
(the sphere stage) rewrites the turn read (`entry_turn`, adaptive from
`PROJECTED_ENTRY_PIECES`), so this waits on it.
