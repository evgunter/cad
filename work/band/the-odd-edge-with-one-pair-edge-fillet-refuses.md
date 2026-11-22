---
id: the-odd-edge-with-one-pair-edge-fillet-refuses
kind: issue
title: A fillet naming a mixed corner's odd edge with one pair edge refuses
status: open
opened: 2026-11-17
priority: P3
cost: H
design: true
refs: [a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds]
---

Filed by the BAND orchestrator (2026-11-17) from the OCCT comparison Ev asked for on PR 7038 (comment 6937345056; `docs/renders/band-2026-11/occt-compare.md` on `band/occt-compare-renders` at 8394370cd0).

**Today.** A fillet naming a mixed corner's odd edge and one pair edge refuses, as the ratified rule says (`a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds.md`, "The odd edge and one pair edge": the pair band would have to stop where its edge runs smoothly into the cut-off arc). Pinned in `band_mixed_corner_odd_plus_one.rs` (the fillet rows still refusing).

**Elsewhere.** OCCT 8.0.1 builds it at single corners, with the two cylinders and a corner patch, and fails on the bracket. T-FLEX (Parasolid) documents a corner blend for fillets of opposite convexity (`roundinggroup7.png`, cited in the comment).

**Design: yes.** Building it changes the rule's decision for this request (a new end: a band stopping mid-run, closed by a patch), so it is a design fork for Ev before any unit. P3 until a consumer asks.
