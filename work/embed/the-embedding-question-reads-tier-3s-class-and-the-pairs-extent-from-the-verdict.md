---
id: the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict
kind: unit
title: the embedding question reads tier 3's class and the pair's extent from the verdict, and decides neither
status: dispatched
opened: 2026-11-22
priority: P0
cost: H
branch: embed/verdict-read
---

## Ruling

Ev on PR 7456 (2026-11-22): "1 sounds good. 2 is also probably good,
though i'm less sure that there won't be some issue later. we can
proceed with it though". The text is `docs/DESIGN.md`
`[topology.tiers]`: where faces meet, the question reads from the
verdict what tier 3 decided there (an edge's class and turn, an end, a
pole or apex vertex) and decides none of it; over each cell it reads
the rate at which the two faces part, the faces' rates summed, levered
at how far the narrower of them runs from what they share, measured
across it, never at a cell of the question's own subdivision; where
tier 3 holds no fact, that reading is the only one. Three designers
agreed on it after five rounds (fork-log row 304; the PR body holds
their final texts and the worked cases).

It also builds the hand-over Ev approved on PR 6893, which nobody
built: the strict question takes tier 3's verdict as its input, and
`AtRestBody` keeps tier 3's per-entity facts.

## Spec

Stacked on `embed/remeter` (PR 7329), which levers the departure
margin at the faces' reach, recomputed per cell pair.

1. **The verdict keeps, per shared edge:** its class as check 4 decided
   it (a crease and its turn, a smooth seam, an end) and the pair's
   extent: how far the narrower of the two faces runs from the edge,
   measured across it. Per face, its pole or apex vertex by key (check
   6 places it). Per vertex two faces share alone, the same kind of
   extent from the vertex (`reach_from_vertex` takes the farther face's
   today). Each is read once and never recomputed per cell pair.
2. **The question takes the finished body** and never classes an edge.
   The class decides which arguments may run: the other face's own
   surface is a separator only across a crease; across an end no
   first-order separator certifies, the bisecting plane included, and
   the pair goes to the second-order arm; a smooth seam is proved
   through a separator both faces leave squarely. Where the question
   proves a sign at a crease it must agree with the turn the verdict
   holds; a mismatch refuses.
3. **The cell's reading is the two faces' rates off the separator,
   summed**, times that extent, against the band. Under the carrier arm
   one face contributes rate 0.
4. **Pins owed** (measured first on `embed/wedge-measure`): the curling
   sliver (a 1 mm cylinder face 1e-7 m wide ending 5e-12 m from a 1 m
   plane) and its flat twin (sine 5e-5) refuse; the cone that owns its
   apex, cut through its axis, holds; a crease read under the carrier
   arm and under the bisecting plane decides alike; the bodies whose
   verdict moves between PR 7329's reach and the narrower run are
   listed with each one's narrower face.
5. **Not this unit's:** whether check 4 itself takes the narrower
   face's extent, and whether its fold is the right meter (RESTFRONT:
   `check-4-levers-a-wedge-at-the-farther-faces-run-…`,
   `check-4s-fold-reads-a-crease-where-a-face-curves-back-…`). Until
   check 4 moves, the question can refuse a body check 4 passed (the
   flat strip): the right verdict at the wrong door, to be said in the
   PR. The run is measured across the edge, which HONEONE's
   `the-wedge-reach-counts-a-faces-run-along-its-edge` proposes; if
   that lands otherwise this unit says so on that row.

For the reviewer: the summed-rate bound assumes a separator whose
gradient has unit length (exact for a plane, close for a face's own
surface near the edge, unchecked for the edge-graph separator).

Seams: `validate.rs` (what tier 3's verdict keeps: TOPO, RESTFRONT),
`finish.rs` (EMBEDGATE: `AtRestBody`), `wedge_reach.rs` (HONEONE,
ONEROUND).

Tier: dual (the verdict's shape is broad and hard to change later).
Letter H.

- 2026-11-23 — **Two of PR 7329's rows are this unit's to turn** (both of its reviewers, delta re-reads): `injective::margin_reach_rows::the_arm_counts_a_strips_run_along_its_edge` pins today's over-read (the arm counts a strip's run along its edge), and `the_reach_alone_signs_an_edge_check_4_classes_smooth` pins the question signing an edge check 4 classes smooth. Each goes red when the lever is the narrower face's run across the edge and the class is read from the verdict; this unit re-pins them and lists them among its pins. (EMBED orchestrator)

