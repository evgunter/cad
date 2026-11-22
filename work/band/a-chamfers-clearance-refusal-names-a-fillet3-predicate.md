---
id: a-chamfers-clearance-refusal-names-a-fillet3-predicate
kind: issue
title: A chamfer's FaceClearance and RingClearance refusals name a fillet3_* predicate id, so a chamfer's text says fillet
status: open
opened: 2026-11-17
priority: P3
cost: E
---


Found by the PR 7072 reviewers (bracket step 3, cut 3), pre-existing.

A chamfer's `FaceClearance` and `RingClearance` refusals render their `ClassifiedMargin` (`crates/sweep/src/blend/mod.rs`, `BlendError::write`, the `FaceClearance` and `RingClearance` arms), and the margin prints its predicate id, `fillet3_face_clearance` or `fillet3_ring_clearance`. So a chamfer's refusal text contains "fillet". `blend6_verb_vocab` holds that a chamfer refusal never says the other verb, but it checks only the arms it renders. Measured: the reviewers' follow probe on a cube chamfered at 0.6 found `fillet_in_chamfer=true` on every `FaceClearance` row.

Fix: render the margin without its family id under the chamfer, or name the family verb-neutrally. Then extend `blend6_verb_vocab` to these arms.
