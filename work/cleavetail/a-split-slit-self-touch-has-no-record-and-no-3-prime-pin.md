---
id: a-split-slit-self-touch-has-no-record-and-no-3-prime-pin
kind: issue
title: A split's slit self-touch has no coincidence record, and no row pins why a slit body passes 3′
status: open
opened: 2026-10-12
priority: P3
cost: E
---

Found by the keel design fork (`[ev]` PR 4795).

## What

1. The split's coincidence list (`reduce`, which records only vertices
   with two or more runs) holds nothing backing the self-touch along a
   slit's two coincident edges: the open slit PR 4622 landed, and the
   island ring the keel row builds. Whether the pair owes a record
   depends on reading D1 (ii), "every pair whose interiors meet is
   recorded" (PR 4547), against PR 3317's "the shared edge is the
   structural record". The census's reconstruction rule licenses the
   pair with no record when both bounds are structural.
2. No test pins that a slit-bearing body passes tier 3′ *for that
   reason*: 4622's pins run tier 3 only, and the split runs no tier 3′
   on its sides.

## Fix

Pin one row: a slit body (4622's one-end trench) passes 3′, with the
census's reconstruction rule named as the reason. If that reading is
wrong, the split mints the record (`SplitResult` in
`crates/topo/src/splitting/finish.rs` has no channel for records the
split mints today).

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
