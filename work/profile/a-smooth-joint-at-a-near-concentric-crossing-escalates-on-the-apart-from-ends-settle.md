---
id: a-smooth-joint-at-a-near-concentric-crossing-escalates-on-the-apart-from-ends-settle
kind: issue
title: A smooth joint sitting at a near-concentric crossing escalates on arc_span, because ApartFromEnds refuses to settle on the joint itself
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the third review of PR #5175, at head `8f7882102d`. That head made a fail-loud regression against main; nothing here is silent.

## The defect

`crates/profile/src/seg.rs` `end_in_stretch`, with `Settle::ApartFromEnds`, is called from `arc_arc`'s chord-band and secant readings. PR #5175 added the settle so that a cusp whose crossing reads in band cannot be settled by the shared joint. That rule closed a silent acceptance. It cannot tell a cusp from a smooth joint, though. When the joint `v` sits at one of the near-concentric carriers' crossings, the span reads that crossing in band, and the end nearest it is `v`, which is an end of the other segment. So the settle is refused and the pair escalates on `arc_span`. At a smooth joint, the discounted touch is the joint's own.

## Reproduction (f64; the Interval reading differs)

Arc 1 on the unit circle runs into the crossing. Arc 2, on the circle about `(d, 0)` with radius `1 − d + g`, runs out the other way. The joint is smooth, so the loop is valid.

| d | g | ε | head `8f7882102d` | main, `842e212ee8` |
|---|---|---|---|---|
| 17ε | 0.9ε | 1e-9 | escalates `arc_span` | validates |
| 17ε | 0.5ε, 0.9ε | 1e-6 | escalates | validates |
| 34ε | 0.9ε | 1e-6 | escalates | validates |

The valid "short/long" two-arc lens, with vertices at both crossings and d = 17–100ε, also escalates at 1e-9 and 1e-6. Main refused it `TangentialContact`, and `842e212ee8` validated it.

The reviewer's probes are in the orchestrator's scratchpad, `reviews/5175-probes.rs`; copy the rows when this is taken.

## Where to look

The settle needs to tell a joint whose touch is the joint's own (the arcs leave `v` on opposite sides of the tangency direction) from the cusp that runs back along the other arc. Keep `rejections::a_cusp_whose_crossing_one_arc_reads_in_band_escalates` red-capable. Also re-word the PR's claim that each kept contact is "a point within ε of both arcs": a near-full arc's end read can refuse at up to about 95Kε, which is `arc_span`'s documented compression. That refusal is never an acceptance.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
